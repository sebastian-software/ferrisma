// libjpeg-turbo 3.2.1, simd/arm/jdcolext-neon.c:
//
// /*
//  * YCbCr-to-RGB/RGB565 Color Conversion (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2020, 2025, D. R. Commander.
//  *
//  * This software is provided 'as-is', without any express or implied
//  * warranty.  In no event will the authors be held liable for any damages
//  * arising from the use of this software.
//  *
//  * Permission is granted to anyone to use this software for any purpose,
//  * including commercial applications, and to alter it and redistribute it
//  * freely, subject to the following restrictions:
//  *
//  * 1. The origin of this software must not be misrepresented; you must not
//  *    claim that you wrote the original software. If you use this software
//  *    in a product, an acknowledgment in the product documentation would be
//  *    appreciated but is not required.
//  * 2. Altered source versions must be plainly marked as such, and must not be
//  *    misrepresented as being the original software.
//  * 3. This notice may not be removed or altered from any source distribution.
//  */
//! Translation of `simd/arm/jdcolext-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! Not translated: `rgb565`, because its RGB565 branch serves RGB565 output, which is not translated (ADR 0001).
//!
//! `jdcolor-neon.c` includes this file once per RGB pixel layout, with
//! `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, `RGB_PIXELSIZE` and possibly
//! `RGB_ALPHA` defined, and with the function renamed. The kernel is a
//! function generic over those values, and `jdcolor_neon.rs` names the
//! instances as C does; `RGB_ALPHA` is -1 where C leaves it undefined, as in
//! `jdcolext.rs`. Every instance has an `RGB_PIXELSIZE` of 3 or 4, so the
//! `#elif RGB_PIXELSIZE == 3` branch is the `else` of `RGB_PIXELSIZE == 4`.
//!
//! The kernel takes its rows as slices; a pointer increment is a reslice, and
//! [`input`] and [`output`] make the arrays that the loads and stores of
//! `neon` take. The input rows extend to the end of their storage, which
//! covers upstream's over-reads into the `alloc_sarray` padding; the stores
//! stay within `output_width`, as upstream's notes require. Rust names the
//! vectors of a Neon multi-vector type as fields, not as the array `val`, so
//! C's assignments to `val[RGB_RED]` and the others become [`rgb_val`] and
//! [`rgba_val`], which return the vectors in the pixel layout. The
//! fall-through `switch` of the tail is a chain of `if`s, one per `case`, as
//! `cols_remaining` is below 8 there.

#![forbid(unsafe_code)]

use core::arch::aarch64::{
    int16x4_t, int16x8_t, int32x4_t, uint8x8_t, uint8x8x3_t, uint8x8x4_t, uint8x16_t, uint8x16x3_t,
    uint8x16x4_t, vaddw_u8, vcombine_s16, vcombine_u8, vdup_n_u8, vdupq_n_s16, vdupq_n_u8,
    vget_high_s16, vget_high_u8, vget_low_s16, vget_low_u8, vmlsl_lane_s16, vmull_lane_s16,
    vqmovun_s16, vqrdmulhq_lane_s16, vreinterpretq_s16_u16, vreinterpretq_u16_s16, vrshrn_n_s32,
    vshlq_n_s16,
};

use crate::jdcolor_neon::jsimd_ycc_rgb_convert_neon_consts;
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS, JSAMPROWS_MUT};
use crate::neon::{
    vld1_s16, vld1_u8, vld1q_u8, vst3_lane_u8, vst3_u8, vst3q_u8, vst4_lane_u8, vst4_u8, vst4q_u8,
};

/// The `N` samples from `inptr` on that a load reads. Not an upstream item.
///
/// # Panics
///
/// When the row's storage ends before them, where C would read past the
/// allocation.
#[inline]
#[must_use]
pub fn input<const N: usize>(inptr: &[JSAMPLE]) -> &[JSAMPLE; N] {
    inptr
        .first_chunk()
        .expect("input row shorter than the load reads")
}

/// The `N` samples from `outptr + offset` on that a store writes. Not an
/// upstream item.
///
/// # Panics
///
/// When the row ends before them, where C would write past the row.
#[inline]
pub fn output<const N: usize>(outptr: &mut [JSAMPLE], offset: usize) -> &mut [JSAMPLE; N] {
    outptr[offset..]
        .first_chunk_mut()
        .expect("output row shorter than the store writes")
}

/// C's `x.val[RGB_RED] = red; x.val[RGB_GREEN] = green;
/// x.val[RGB_BLUE] = blue;` of a three-vector type: the vectors in the pixel
/// layout of the parameters, from which the caller builds the type. Not an
/// upstream item.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's subscripts by the layout constants"
)]
#[inline]
#[must_use]
pub fn rgb_val<const RGB_RED: i32, const RGB_GREEN: i32, const RGB_BLUE: i32, T: Copy>(
    red: T,
    green: T,
    blue: T,
) -> [T; 3] {
    let mut val = [red; 3];
    val[RGB_RED as usize] = red;
    val[RGB_GREEN as usize] = green;
    val[RGB_BLUE as usize] = blue;
    val
}

/// C's `x.val[RGB_RED] = red; x.val[RGB_GREEN] = green;
/// x.val[RGB_BLUE] = blue; x.val[RGB_ALPHA] = alpha;` of a four-vector type;
/// see [`rgb_val`]. Not an upstream item.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's subscripts by the layout constants"
)]
#[inline]
#[must_use]
pub fn rgba_val<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    T: Copy,
>(
    red: T,
    green: T,
    blue: T,
    alpha: T,
) -> [T; 4] {
    let mut val = [red; 4];
    val[RGB_RED as usize] = red;
    val[RGB_GREEN as usize] = green;
    val[RGB_BLUE as usize] = blue;
    val[RGB_ALPHA as usize] = alpha;
    val
}

/* YCbCr -> RGB conversion is defined by the following equations:
 *    R = Y                        + 1.40200 * (Cr - 128)
 *    G = Y - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128)
 *    B = Y + 1.77200 * (Cb - 128)
 *
 * Scaled integer constants are used to avoid floating-point arithmetic:
 *    0.3441467 = 11277 * 2^-15
 *    0.7141418 = 23401 * 2^-15
 *    1.4020386 = 22971 * 2^-14
 *    1.7720337 = 29033 * 2^-14
 * These constants are defined in jdcolor-neon.c.
 *
 * To ensure correct results, rounding is used when descaling.
 */

/* Notes on safe memory access for YCbCr -> RGB conversion routines:
 *
 * Input memory buffers can be safely overread up to the next multiple of
 * ALIGN_SIZE bytes, since they are always allocated by alloc_sarray() in
 * jmemmgr.c.
 *
 * The output buffer cannot safely be written beyond output_width, since
 * output_buf points to a possibly unpadded row in the decompressed image
 * buffer allocated by the calling program.
 */

// upstream: simd/arm/jdcolext-neon.c jsimd_ycc_rgb_convert_neon
/// Converts `num_rows` rows of `output_width` pixels of `input_buf` from row
/// `input_row` on from YCbCr to RGB in the pixel layout of the parameters.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
///
/// # Panics
///
/// When a row is shorter than C reads or writes.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C's int cols_remaining = output_width"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "C's conversions of the row index and the pixel size"
)]
#[expect(clippy::similar_names, reason = "C variable names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[target_feature(enable = "neon")]
pub fn jsimd_ycc_rgb_convert_neon<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_ALPHA: i32,
    const RGB_PIXELSIZE: i32,
>(
    output_width: JDIMENSION,
    input_buf: &[JSAMPROWS<'_>],
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    mut num_rows: i32,
) {
    let consts: int16x4_t = vld1_s16(&jsimd_ycc_rgb_convert_neon_consts);
    let neg_128: int16x8_t = vdupq_n_s16(-128);

    let mut out_row = 0;
    loop {
        num_rows -= 1;
        if num_rows < 0 {
            break;
        }
        /* Pointers to Y, Cb, and Cr data */
        let mut inptr0 = input_buf[0].row(input_row as usize);
        let mut inptr1 = input_buf[1].row(input_row as usize);
        let mut inptr2 = input_buf[2].row(input_row as usize);
        input_row += 1;
        let mut outptr = output_buf.row_mut(out_row);
        out_row += 1;
        let mut cols_remaining = output_width as i32;
        while cols_remaining >= 16 {
            let y: uint8x16_t = vld1q_u8(input(inptr0));
            let cb: uint8x16_t = vld1q_u8(input(inptr1));
            let cr: uint8x16_t = vld1q_u8(input(inptr2));
            /* Subtract 128 from Cb and Cr. */
            let cr_128_l: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), vget_low_u8(cr)));
            let cr_128_h: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), vget_high_u8(cr)));
            let cb_128_l: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), vget_low_u8(cb)));
            let cb_128_h: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), vget_high_u8(cb)));
            /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
            let mut g_sub_y_ll: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128_l), consts);
            let mut g_sub_y_lh: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128_l), consts);
            let mut g_sub_y_hl: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128_h), consts);
            let mut g_sub_y_hh: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128_h), consts);
            g_sub_y_ll = vmlsl_lane_s16::<1>(g_sub_y_ll, vget_low_s16(cr_128_l), consts);
            g_sub_y_lh = vmlsl_lane_s16::<1>(g_sub_y_lh, vget_high_s16(cr_128_l), consts);
            g_sub_y_hl = vmlsl_lane_s16::<1>(g_sub_y_hl, vget_low_s16(cr_128_h), consts);
            g_sub_y_hh = vmlsl_lane_s16::<1>(g_sub_y_hh, vget_high_s16(cr_128_h), consts);
            /* Descale G components: shift right 15, round, and narrow to 16-bit. */
            let g_sub_y_l: int16x8_t = vcombine_s16(
                vrshrn_n_s32::<15>(g_sub_y_ll),
                vrshrn_n_s32::<15>(g_sub_y_lh),
            );
            let g_sub_y_h: int16x8_t = vcombine_s16(
                vrshrn_n_s32::<15>(g_sub_y_hl),
                vrshrn_n_s32::<15>(g_sub_y_hh),
            );
            /* Compute R-Y: 1.40200 * (Cr - 128) */
            let r_sub_y_l: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128_l), consts);
            let r_sub_y_h: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128_h), consts);
            /* Compute B-Y: 1.77200 * (Cb - 128) */
            let b_sub_y_l: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128_l), consts);
            let b_sub_y_h: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128_h), consts);
            /* Add Y. */
            let r_l: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y_l), vget_low_u8(y)));
            let r_h: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y_h), vget_high_u8(y)));
            let b_l: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y_l), vget_low_u8(y)));
            let b_h: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y_h), vget_high_u8(y)));
            let g_l: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y_l), vget_low_u8(y)));
            let g_h: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y_h), vget_high_u8(y)));

            if RGB_PIXELSIZE == 4 {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgba = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                    vcombine_u8(vqmovun_s16(r_l), vqmovun_s16(r_h)),
                    vcombine_u8(vqmovun_s16(g_l), vqmovun_s16(g_h)),
                    vcombine_u8(vqmovun_s16(b_l), vqmovun_s16(b_h)),
                    /* Set alpha channel to opaque (0xFF). */
                    vdupq_n_u8(0xFF),
                );
                let rgba = uint8x16x4_t(rgba[0], rgba[1], rgba[2], rgba[3]);
                /* Store RGBA pixel data to memory. */
                vst4q_u8(output(outptr, 0), rgba);
            } else {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgb = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(
                    vcombine_u8(vqmovun_s16(r_l), vqmovun_s16(r_h)),
                    vcombine_u8(vqmovun_s16(g_l), vqmovun_s16(g_h)),
                    vcombine_u8(vqmovun_s16(b_l), vqmovun_s16(b_h)),
                );
                let rgb = uint8x16x3_t(rgb[0], rgb[1], rgb[2]);
                /* Store RGB pixel data to memory. */
                vst3q_u8(output(outptr, 0), rgb);
            }

            /* Increment pointers. */
            inptr0 = &inptr0[16..];
            inptr1 = &inptr1[16..];
            inptr2 = &inptr2[16..];
            outptr = &mut outptr[RGB_PIXELSIZE as usize * 16..];
            cols_remaining -= 16;
        }

        if cols_remaining >= 8 {
            let y: uint8x8_t = vld1_u8(input(inptr0));
            let cb: uint8x8_t = vld1_u8(input(inptr1));
            let cr: uint8x8_t = vld1_u8(input(inptr2));
            /* Subtract 128 from Cb and Cr. */
            let cr_128: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
            let cb_128: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
            /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
            let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
            let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
            g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
            g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
            /* Descale G components: shift right 15, round, and narrow to 16-bit. */
            let g_sub_y: int16x8_t =
                vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
            /* Compute R-Y: 1.40200 * (Cr - 128) */
            let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
            /* Compute B-Y: 1.77200 * (Cb - 128) */
            let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
            /* Add Y. */
            let r: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y));
            let b: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y));
            let g: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y));

            if RGB_PIXELSIZE == 4 {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgba = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                    vqmovun_s16(r),
                    vqmovun_s16(g),
                    vqmovun_s16(b),
                    /* Set alpha channel to opaque (0xFF). */
                    vdup_n_u8(0xFF),
                );
                let rgba = uint8x8x4_t(rgba[0], rgba[1], rgba[2], rgba[3]);
                /* Store RGBA pixel data to memory. */
                vst4_u8(output(outptr, 0), rgba);
            } else {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgb = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(
                    vqmovun_s16(r),
                    vqmovun_s16(g),
                    vqmovun_s16(b),
                );
                let rgb = uint8x8x3_t(rgb[0], rgb[1], rgb[2]);
                /* Store RGB pixel data to memory. */
                vst3_u8(output(outptr, 0), rgb);
            }

            /* Increment pointers. */
            inptr0 = &inptr0[8..];
            inptr1 = &inptr1[8..];
            inptr2 = &inptr2[8..];
            outptr = &mut outptr[RGB_PIXELSIZE as usize * 8..];
            cols_remaining -= 8;
        }

        /* Handle the tail elements. */
        if cols_remaining > 0 {
            let y: uint8x8_t = vld1_u8(input(inptr0));
            let cb: uint8x8_t = vld1_u8(input(inptr1));
            let cr: uint8x8_t = vld1_u8(input(inptr2));
            /* Subtract 128 from Cb and Cr. */
            let cr_128: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cr));
            let cb_128: int16x8_t =
                vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(neg_128), cb));
            /* Compute G-Y: - 0.34414 * (Cb - 128) - 0.71414 * (Cr - 128) */
            let mut g_sub_y_l: int32x4_t = vmull_lane_s16::<0>(vget_low_s16(cb_128), consts);
            let mut g_sub_y_h: int32x4_t = vmull_lane_s16::<0>(vget_high_s16(cb_128), consts);
            g_sub_y_l = vmlsl_lane_s16::<1>(g_sub_y_l, vget_low_s16(cr_128), consts);
            g_sub_y_h = vmlsl_lane_s16::<1>(g_sub_y_h, vget_high_s16(cr_128), consts);
            /* Descale G components: shift right 15, round, and narrow to 16-bit. */
            let g_sub_y: int16x8_t =
                vcombine_s16(vrshrn_n_s32::<15>(g_sub_y_l), vrshrn_n_s32::<15>(g_sub_y_h));
            /* Compute R-Y: 1.40200 * (Cr - 128) */
            let r_sub_y: int16x8_t = vqrdmulhq_lane_s16::<2>(vshlq_n_s16::<1>(cr_128), consts);
            /* Compute B-Y: 1.77200 * (Cb - 128) */
            let b_sub_y: int16x8_t = vqrdmulhq_lane_s16::<3>(vshlq_n_s16::<1>(cb_128), consts);
            /* Add Y. */
            let r: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(r_sub_y), y));
            let b: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(b_sub_y), y));
            let g: int16x8_t = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(g_sub_y), y));

            if RGB_PIXELSIZE == 4 {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgba = rgba_val::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_ALPHA, _>(
                    vqmovun_s16(r),
                    vqmovun_s16(g),
                    vqmovun_s16(b),
                    /* Set alpha channel to opaque (0xFF). */
                    vdup_n_u8(0xFF),
                );
                let rgba = uint8x8x4_t(rgba[0], rgba[1], rgba[2], rgba[3]);
                /* Store RGBA pixel data to memory. */
                if cols_remaining >= 7 {
                    vst4_lane_u8::<6>(output(outptr, 6 * RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 6 {
                    vst4_lane_u8::<5>(output(outptr, 5 * RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 5 {
                    vst4_lane_u8::<4>(output(outptr, 4 * RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 4 {
                    vst4_lane_u8::<3>(output(outptr, 3 * RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 3 {
                    vst4_lane_u8::<2>(output(outptr, 2 * RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 2 {
                    vst4_lane_u8::<1>(output(outptr, RGB_PIXELSIZE as usize), rgba);
                }
                if cols_remaining >= 1 {
                    vst4_lane_u8::<0>(output(outptr, 0), rgba);
                }
            } else {
                /* Convert each component to unsigned and narrow, clamping to [0-255]. */
                let rgb = rgb_val::<RGB_RED, RGB_GREEN, RGB_BLUE, _>(
                    vqmovun_s16(r),
                    vqmovun_s16(g),
                    vqmovun_s16(b),
                );
                let rgb = uint8x8x3_t(rgb[0], rgb[1], rgb[2]);
                /* Store RGB pixel data to memory. */
                if cols_remaining >= 7 {
                    vst3_lane_u8::<6>(output(outptr, 6 * RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 6 {
                    vst3_lane_u8::<5>(output(outptr, 5 * RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 5 {
                    vst3_lane_u8::<4>(output(outptr, 4 * RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 4 {
                    vst3_lane_u8::<3>(output(outptr, 3 * RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 3 {
                    vst3_lane_u8::<2>(output(outptr, 2 * RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 2 {
                    vst3_lane_u8::<1>(output(outptr, RGB_PIXELSIZE as usize), rgb);
                }
                if cols_remaining >= 1 {
                    vst3_lane_u8::<0>(output(outptr, 0), rgb);
                }
            }
        }
    }
}
