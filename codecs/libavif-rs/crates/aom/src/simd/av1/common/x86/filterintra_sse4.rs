/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/x86/filterintra_sse4.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse4.1.
//
// Translation notes:
// - `av1_filter_intra_taps` is the 16-byte aligned table that
//   av1/common/reconintra.c defines; the translated C exports it, and it is
//   read here through a raw pointer to that same object (it is a `static mut`
//   in the translation), so the aligned `xx_load_128` loads see the original
//   alignment.
// - `tx_size_wide` / `tx_size_high` (av1/common/common_data.h) are
//   header-local in the original and are transcribed below, as in
//   reconintra_neon.rs.
// - `(int8_t)` arguments of `_mm_insert_epi8` become `as i8 as i32`, the
//   C integer promotion of the `int` parameter; the instruction keeps the
//   low 8 bits either way.
// - `void *` parameters of the static helper become `*mut c_void` /
//   `*const c_void`.
use core::arch::x86_64::*;
use core::ffi::c_void;

use crate::av1__common__reconintra::av1_filter_intra_taps;
use crate::simd::aom_dsp__x86__synonyms::*;

/// `TX_SIZE` from av1/common/enums.h.
pub type TX_SIZE = u8;

// Transcribed from upstream/aom/av1/common/common_data.h (header-local).
// Transform block width in pixels
static tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];

// Transcribed from upstream/aom/av1/common/common_data.h (header-local).
// Transform block height in pixels
static tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

/// `av1_filter_intra_taps[mode][k]` as a pointer into the translated table.
#[inline]
unsafe fn filter_intra_taps(mode: i32, k: usize) -> *const c_void {
    let table = core::ptr::addr_of!(av1_filter_intra_taps.0);
    (*table)[mode as usize][k].as_ptr() as *const c_void
}

//------------------------------------------------------------------------------
// filter_intra_predictor_sse4_1

// This shuffle mask selects 32-bit blocks in the order 0, 1, 0, 1, which
// duplicates the first 8 bytes of a 128-bit vector into the second 8 bytes.
const DUPLICATE_FIRST_HALF: i32 = 0x44;

// Apply all filter taps to the given 7 packed 16-bit values, keeping the 8th
// at zero to preserve the sum.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn filter_4x2_sse4_1(dst: *mut u8, stride: isize, pixels: *const __m128i, taps_0_1: *const __m128i,
                            taps_2_3: *const __m128i, taps_4_5: *const __m128i, taps_6_7: *const __m128i) {
    let mul_0_01 = _mm_maddubs_epi16(*pixels, *taps_0_1);
    let mul_0_23 = _mm_maddubs_epi16(*pixels, *taps_2_3);
    // |output_half| contains 8 partial sums.
    let mut output_half = _mm_hadd_epi16(mul_0_01, mul_0_23);
    let mut output = _mm_hadd_epi16(output_half, output_half);
    let output_row0 = _mm_packus_epi16(xx_roundn_epi16_unsigned(output, 4),
                                       /* arbitrary pack arg */ output);
    xx_storel_32(dst as *mut c_void, output_row0);
    let mul_1_01 = _mm_maddubs_epi16(*pixels, *taps_4_5);
    let mul_1_23 = _mm_maddubs_epi16(*pixels, *taps_6_7);
    output_half = _mm_hadd_epi16(mul_1_01, mul_1_23);
    output = _mm_hadd_epi16(output_half, output_half);
    let output_row1 = _mm_packus_epi16(xx_roundn_epi16_unsigned(output, 4),
                                       /* arbitrary pack arg */ output);
    xx_storel_32(dst.offset(stride) as *mut c_void, output_row1);
}

// 4xH transform sizes are given special treatment because xx_loadl_64 goes out
// of bounds and every block involves the left column. This implementation
// loads TL from the top row for the first block, so it is not
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn filter_4xh(mut dest: *mut u8, stride: isize, top_ptr: *const u8, left_ptr: *const u8, mode: i32,
                     height: i32) {
    let taps_0_1 = xx_load_128(filter_intra_taps(mode, 0));
    let taps_2_3 = xx_load_128(filter_intra_taps(mode, 2));
    let taps_4_5 = xx_load_128(filter_intra_taps(mode, 4));
    let taps_6_7 = xx_load_128(filter_intra_taps(mode, 6));
    let top = xx_loadl_32(top_ptr.offset(-1) as *const c_void);
    let mut pixels = _mm_insert_epi8::<4>(top, *top_ptr.add(3) as i8 as i32);
    let mut left = if height == 4 {
        xx_loadl_32(left_ptr as *const c_void)
    } else {
        xx_loadl_64(left_ptr as *const c_void)
    };
    left = _mm_slli_si128::<5>(left);

    // Relative pixels: top[-1], top[0], top[1], top[2], top[3], left[0], left[1],
    // left[2], left[3], left[4], left[5], left[6], left[7]
    pixels = _mm_or_si128(left, pixels);

    // Duplicate first 8 bytes.
    pixels = _mm_shuffle_epi32::<DUPLICATE_FIRST_HALF>(pixels);
    filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
    dest = dest.offset(stride); // Move to y = 1.
    pixels = xx_loadl_32(dest as *const c_void);

    // Relative pixels: top[0], top[1], top[2], top[3], empty, left[-2], left[-1],
    // left[0], left[1], ...
    pixels = _mm_or_si128(left, pixels);

    // This mask rearranges bytes in the order: 6, 0, 1, 2, 3, 7, 8, 15. The last
    // byte is an unused value, which shall be multiplied by 0 when we apply the
    // filter.
    let kInsertTopLeftFirstMask: i64 = 0x0F08070302010006;

    // Insert left[-1] in front as TL and put left[0] and left[1] at the end.
    let pixel_order1 = _mm_set1_epi64x(kInsertTopLeftFirstMask);
    pixels = _mm_shuffle_epi8(pixels, pixel_order1);
    dest = dest.offset(stride); // Move to y = 2.
    filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
    dest = dest.offset(stride); // Move to y = 3.

    // Compute the middle 8 rows before using common code for the final 4 rows.
    // Because the common code below this block assumes that
    if height == 16 {
        // This shift allows us to use pixel_order2 twice after shifting by 2 later.
        left = _mm_slli_si128::<1>(left);
        pixels = xx_loadl_32(dest as *const c_void);

        // Relative pixels: top[0], top[1], top[2], top[3], empty, empty, left[-4],
        // left[-3], left[-2], left[-1], left[0], left[1], left[2], left[3]
        pixels = _mm_or_si128(left, pixels);

        // This mask rearranges bytes in the order: 9, 0, 1, 2, 3, 7, 8, 15. The
        // last byte is an unused value, as above. The top-left was shifted to
        // position nine to keep two empty spaces after the top pixels.
        let kInsertTopLeftSecondMask: i64 = 0x0F0B0A0302010009;

        // Insert (relative) left[-1] in front as TL and put left[0] and left[1] at
        // the end.
        let pixel_order2 = _mm_set1_epi64x(kInsertTopLeftSecondMask);
        pixels = _mm_shuffle_epi8(pixels, pixel_order2);
        dest = dest.offset(stride); // Move to y = 4.

        // First 4x2 in the if body.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);

        // Clear all but final pixel in the first 8 of left column.
        let mut keep_top_left = _mm_srli_si128::<13>(left);
        dest = dest.offset(stride); // Move to y = 5.
        pixels = xx_loadl_32(dest as *const c_void);
        left = _mm_srli_si128::<2>(left);

        // Relative pixels: top[0], top[1], top[2], top[3], left[-6],
        // left[-5], left[-4], left[-3], left[-2], left[-1], left[0], left[1]
        pixels = _mm_or_si128(left, pixels);
        left = xx_loadl_64(left_ptr.add(8) as *const c_void);

        pixels = _mm_shuffle_epi8(pixels, pixel_order2);
        dest = dest.offset(stride); // Move to y = 6.

        // Second 4x2 in the if body.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);

        // Position TL value so we can use pixel_order1.
        keep_top_left = _mm_slli_si128::<6>(keep_top_left);
        dest = dest.offset(stride); // Move to y = 7.
        pixels = xx_loadl_32(dest as *const c_void);
        left = _mm_slli_si128::<7>(left);
        left = _mm_or_si128(left, keep_top_left);

        // Relative pixels: top[0], top[1], top[2], top[3], empty, empty,
        // left[-1], left[0], left[1], left[2], left[3], ...
        pixels = _mm_or_si128(left, pixels);
        pixels = _mm_shuffle_epi8(pixels, pixel_order1);
        dest = dest.offset(stride); // Move to y = 8.

        // Third 4x2 in the if body.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
        dest = dest.offset(stride); // Move to y = 9.

        // Prepare final inputs.
        pixels = xx_loadl_32(dest as *const c_void);
        left = _mm_srli_si128::<2>(left);

        // Relative pixels: top[0], top[1], top[2], top[3], left[-3], left[-2]
        // left[-1], left[0], left[1], left[2], left[3], ...
        pixels = _mm_or_si128(left, pixels);
        pixels = _mm_shuffle_epi8(pixels, pixel_order1);
        dest = dest.offset(stride); // Move to y = 10.

        // Fourth 4x2 in the if body.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
        dest = dest.offset(stride); // Move to y = 11.
    }

    // In both the 8 and 16 case, we assume that the left vector has the next TL
    // at position 8.
    if height > 4 {
        // Erase prior left pixels by shifting TL to position 0.
        left = _mm_srli_si128::<8>(left);
        left = _mm_slli_si128::<6>(left);
        pixels = xx_loadl_32(dest as *const c_void);

        // Relative pixels: top[0], top[1], top[2], top[3], empty, empty,
        // left[-1], left[0], left[1], left[2], left[3], ...
        pixels = _mm_or_si128(left, pixels);
        pixels = _mm_shuffle_epi8(pixels, pixel_order1);
        dest = dest.offset(stride); // Move to y = 12 or 4.

        // First of final two 4x2 blocks.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
        dest = dest.offset(stride); // Move to y = 13 or 5.
        pixels = xx_loadl_32(dest as *const c_void);
        left = _mm_srli_si128::<2>(left);

        // Relative pixels: top[0], top[1], top[2], top[3], left[-3], left[-2]
        // left[-1], left[0], left[1], left[2], left[3], ...
        pixels = _mm_or_si128(left, pixels);
        pixels = _mm_shuffle_epi8(pixels, pixel_order1);
        dest = dest.offset(stride); // Move to y = 14 or 6.

        // Last of final two 4x2 blocks.
        filter_4x2_sse4_1(dest, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn filter_intra_predictor_sse4_1(dest: *mut c_void, stride: isize, top_row: *const c_void,
                                        left_column: *const c_void, mode: i32, width: i32, height: i32) {
    let top_ptr = top_row as *const u8;
    let left_ptr = left_column as *const u8;
    let mut dst = dest as *mut u8;
    if width == 4 {
        filter_4xh(dst, stride, top_ptr, left_ptr, mode, height);
        return;
    }

    // There is one set of 7 taps for each of the 4x2 output pixels.
    let taps_0_1 = xx_load_128(filter_intra_taps(mode, 0));
    let taps_2_3 = xx_load_128(filter_intra_taps(mode, 2));
    let taps_4_5 = xx_load_128(filter_intra_taps(mode, 4));
    let taps_6_7 = xx_load_128(filter_intra_taps(mode, 6));

    // This mask rearranges bytes in the order: 0, 1, 2, 3, 4, 8, 9, 15. The 15 at
    // the end is an unused value, which shall be multiplied by 0 when we apply
    // the filter.
    let kCondenseLeftMask: i64 = 0x0F09080403020100;

    // Takes the "left section" and puts it right after p0-p4.
    let pixel_order1 = _mm_set1_epi64x(kCondenseLeftMask);

    // This mask rearranges bytes in the order: 8, 0, 1, 2, 3, 9, 10, 15. The last
    // byte is unused as above.
    let kInsertTopLeftMask: i64 = 0x0F0A090302010008;

    // Shuffles the "top left" from the left section, to the front. Used when
    // grabbing data from left_column and not top_row.
    let pixel_order2 = _mm_set1_epi64x(kInsertTopLeftMask);

    // This first pass takes care of the cases where the top left pixel comes from
    // top_row.
    let mut pixels = xx_loadl_64(top_ptr.offset(-1) as *const c_void);
    let mut left = _mm_slli_si128::<8>(xx_loadl_32(left_column));
    pixels = _mm_or_si128(pixels, left);

    // Two sets of the same pixels to multiply with two sets of taps.
    pixels = _mm_shuffle_epi8(pixels, pixel_order1);
    filter_4x2_sse4_1(dst, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
    left = _mm_srli_si128::<1>(left);

    // Load
    pixels = xx_loadl_32(dst.offset(stride) as *const c_void);

    // Because of the above shift, this OR 'invades' the final of the first 8
    // bytes of |pixels|. This is acceptable because the 8th filter tap is always
    // a padded 0.
    pixels = _mm_or_si128(pixels, left);
    pixels = _mm_shuffle_epi8(pixels, pixel_order2);
    let stride2: isize = stride << 1;
    let stride4: isize = stride << 2;
    filter_4x2_sse4_1(dst.offset(stride2), stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
    dst = dst.add(4);
    let mut x = 3;
    while x < width - 4 {
        pixels = xx_loadl_32(top_ptr.offset(x as isize) as *const c_void);
        pixels = _mm_insert_epi8::<4>(pixels, *top_ptr.offset((x + 4) as isize) as i8 as i32);
        pixels = _mm_insert_epi8::<5>(pixels, *dst.offset(-1) as i8 as i32);
        pixels = _mm_insert_epi8::<6>(pixels, *dst.offset(stride - 1) as i8 as i32);

        // Duplicate bottom half into upper half.
        pixels = _mm_shuffle_epi32::<DUPLICATE_FIRST_HALF>(pixels);
        filter_4x2_sse4_1(dst, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
        pixels = xx_loadl_32(dst.offset(stride - 1) as *const c_void);
        pixels = _mm_insert_epi8::<4>(pixels, *dst.offset(stride + 3) as i8 as i32);
        pixels = _mm_insert_epi8::<5>(pixels, *dst.offset(stride2 - 1) as i8 as i32);
        pixels = _mm_insert_epi8::<6>(pixels, *dst.offset(stride + stride2 - 1) as i8 as i32);

        // Duplicate bottom half into upper half.
        pixels = _mm_shuffle_epi32::<DUPLICATE_FIRST_HALF>(pixels);
        filter_4x2_sse4_1(dst.offset(stride2), stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
        dst = dst.add(4);
        x += 4;
    }

    // Now we handle heights that reference previous blocks rather than top_row.
    let mut y = 4;
    while y < height {
        // Leftmost 4x4 block for this height.
        dst = dst.offset(-(width as isize));
        dst = dst.offset(stride4);

        // Top Left is not available by offset in these leftmost blocks.
        pixels = xx_loadl_32(dst.offset(-stride) as *const c_void);
        left = _mm_slli_si128::<8>(xx_loadl_32(left_ptr.offset((y - 1) as isize) as *const c_void));
        left = _mm_insert_epi8::<12>(left, *left_ptr.offset((y + 3) as isize) as i8 as i32);
        pixels = _mm_or_si128(pixels, left);
        pixels = _mm_shuffle_epi8(pixels, pixel_order2);
        filter_4x2_sse4_1(dst, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);

        // The bytes shifted into positions 6 and 7 will be ignored by the shuffle.
        left = _mm_srli_si128::<2>(left);
        pixels = xx_loadl_32(dst.offset(stride) as *const c_void);
        pixels = _mm_or_si128(pixels, left);
        pixels = _mm_shuffle_epi8(pixels, pixel_order2);
        filter_4x2_sse4_1(dst.offset(stride2), stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);

        dst = dst.add(4);

        // Remaining 4x4 blocks for this height.
        let mut x = 4;
        while x < width {
            pixels = xx_loadl_32(dst.offset(-stride - 1) as *const c_void);
            pixels = _mm_insert_epi8::<4>(pixels, *dst.offset(-stride + 3) as i8 as i32);
            pixels = _mm_insert_epi8::<5>(pixels, *dst.offset(-1) as i8 as i32);
            pixels = _mm_insert_epi8::<6>(pixels, *dst.offset(stride - 1) as i8 as i32);

            // Duplicate bottom half into upper half.
            pixels = _mm_shuffle_epi32::<DUPLICATE_FIRST_HALF>(pixels);
            filter_4x2_sse4_1(dst, stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
            pixels = xx_loadl_32(dst.offset(stride - 1) as *const c_void);
            pixels = _mm_insert_epi8::<4>(pixels, *dst.offset(stride + 3) as i8 as i32);
            pixels = _mm_insert_epi8::<5>(pixels, *dst.offset(stride2 - 1) as i8 as i32);
            pixels = _mm_insert_epi8::<6>(pixels, *dst.offset(stride2 + stride - 1) as i8 as i32);

            // Duplicate bottom half into upper half.
            pixels = _mm_shuffle_epi32::<DUPLICATE_FIRST_HALF>(pixels);
            filter_4x2_sse4_1(dst.offset(stride2), stride, &pixels, &taps_0_1, &taps_2_3, &taps_4_5, &taps_6_7);
            dst = dst.add(4);
            x += 4;
        }
        y += 4;
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_filter_intra_predictor_sse4_1(dst: *mut u8, stride: isize, tx_size: TX_SIZE,
                                                                  above: *const u8, left: *const u8, mode: i32) {
    let bw = tx_size_wide[tx_size as usize];
    let bh = tx_size_high[tx_size as usize];
    filter_intra_predictor_sse4_1(dst as *mut c_void, stride, above as *const c_void, left as *const c_void, mode,
                                  bw, bh);
}
