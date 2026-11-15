/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/variance_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the still-image functions of this file (aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and the
// static helpers they call are translated; the inter-prediction variance kernels are not
// (working decision O2, 2026-11-13, delegated, revisable by the owner).
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

// CONFIG_AV1_HIGHBITDEPTH
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_4x8: __m128i;
    let mut dst1_4x8: __m128i;
    let mut dst2_4x8: __m128i;
    let mut dst3_4x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src0_4x16: __m128i;
    let mut src1_4x16: __m128i;
    let mut src2_4x16: __m128i;
    let mut src3_4x16: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut src_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(4) {
        dst0_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 0) * dstride) as isize) as *const i32,
        ));
        dst1_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 1) * dstride) as isize) as *const i32,
        ));
        dst2_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 2) * dstride) as isize) as *const i32,
        ));
        dst3_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 3) * dstride) as isize) as *const i32,
        ));
        dst_16x8 = _mm_unpacklo_epi64(
            _mm_unpacklo_epi32(dst0_4x8, dst1_4x8),
            _mm_unpacklo_epi32(dst2_4x8, dst3_4x8),
        );
        dst_16x16 = _mm256_cvtepu8_epi16(dst_16x8);
        src0_4x16 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        src1_4x16 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        src2_4x16 = _mm_loadl_epi64(src.offset(((i + 2) * sstride) as isize) as *const __m128i);
        src3_4x16 = _mm_loadl_epi64(src.offset(((i + 3) * sstride) as isize) as *const __m128i);
        src0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(src0_4x16, src1_4x16));
        src1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(src2_4x16, src3_4x16));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        // r15 r14 r13------------r1 r0  - 16 bit
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit
        src_16x16 = _mm256_madd_epi16(sub_result, sub_result);
        // accumulation of result
        square_result = _mm256_add_epi32(square_result, src_16x16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

// Compute mse of four consecutive 4x4 blocks.
// In src buffer, each 4x4 block in a 32x32 filter block is stored sequentially.
// Hence src_blk_stride is same as block width. Whereas dst buffer is a frame
// buffer, thus dstride is a frame level stride.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_quad_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    src_blk_stride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_16x8: __m128i;
    let mut dst1_16x8: __m128i;
    let mut dst2_16x8: __m128i;
    let mut dst3_16x8: __m128i;
    let mut dst0_16x16: __m256i;
    let mut dst1_16x16: __m256i;
    let mut dst2_16x16: __m256i;
    let mut dst3_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result_0: __m256i;
    let mut sub_result_1: __m256i;
    let mut sub_result_2: __m256i;
    let mut sub_result_3: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = zeros;
    let mut src_temp: *mut u16 = src;
    for i in (0..h).step_by(4) {
        dst0_16x8 = _mm_loadu_si128(
            dst.offset(((i + 0) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst1_16x8 = _mm_loadu_si128(
            dst.offset(((i + 1) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst2_16x8 = _mm_loadu_si128(
            dst.offset(((i + 2) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst3_16x8 = _mm_loadu_si128(
            dst.offset(((i + 3) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        // row0 of 1st,2nd, 3rd and 4th 4x4 blocks- d00 d10 d20 d30
        dst0_16x16 = _mm256_cvtepu8_epi16(dst0_16x8);
        // row1 of 1st,2nd, 3rd and 4th 4x4 blocks - d01 d11 d21 d31
        dst1_16x16 = _mm256_cvtepu8_epi16(dst1_16x8);
        // row2 of 1st,2nd, 3rd and 4th 4x4 blocks - d02 d12 d22 d32
        dst2_16x16 = _mm256_cvtepu8_epi16(dst2_16x8);
        // row3 of 1st,2nd, 3rd and 4th 4x4 blocks - d03 d13 d23 d33
        dst3_16x16 = _mm256_cvtepu8_epi16(dst3_16x8);
        // All rows of 1st 4x4 block - r00 r01 r02 r03
        let mut src0_16x16: __m256i = _mm256_loadu_si256(src_temp as *const __m256i);
        // All rows of 2nd 4x4 block - r10 r11 r12 r13
        let mut src1_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset(src_blk_stride as isize) as *const __m256i);
        // All rows of 3rd 4x4 block - r20 r21 r22 r23
        let mut src2_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset((2 * src_blk_stride) as isize) as *const __m256i);
        // All rows of 4th 4x4 block - r30 r31 r32 r33
        let mut src3_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset((3 * src_blk_stride) as isize) as *const __m256i);
        // r00 r10 r02 r12
        let mut tmp0_16x16: __m256i = _mm256_unpacklo_epi64(src0_16x16, src1_16x16);
        // r01 r11 r03 r13
        let mut tmp1_16x16: __m256i = _mm256_unpackhi_epi64(src0_16x16, src1_16x16);
        // r20 r30 r22 r32
        let mut tmp2_16x16: __m256i = _mm256_unpacklo_epi64(src2_16x16, src3_16x16);
        // r21 r31 r23 r33
        let mut tmp3_16x16: __m256i = _mm256_unpackhi_epi64(src2_16x16, src3_16x16);
        // r00 r10 r20 r30
        src0_16x16 = _mm256_permute2f128_si256::<0x20>(tmp0_16x16, tmp2_16x16);
        // r01 r11 r21 r31
        src1_16x16 = _mm256_permute2f128_si256::<0x20>(tmp1_16x16, tmp3_16x16);
        // r02 r12 r22 r32
        src2_16x16 = _mm256_permute2f128_si256::<0x31>(tmp0_16x16, tmp2_16x16);
        // r03 r13 r23 r33
        src3_16x16 = _mm256_permute2f128_si256::<0x31>(tmp1_16x16, tmp3_16x16);
        // r15 r14 r13------------r1 r0  - 16 bit
        sub_result_0 = _mm256_abs_epi16(_mm256_sub_epi16(src0_16x16, dst0_16x16));
        sub_result_1 = _mm256_abs_epi16(_mm256_sub_epi16(src1_16x16, dst1_16x16));
        sub_result_2 = _mm256_abs_epi16(_mm256_sub_epi16(src2_16x16, dst2_16x16));
        sub_result_3 = _mm256_abs_epi16(_mm256_sub_epi16(src3_16x16, dst3_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0    - 32bit
        src0_16x16 = _mm256_madd_epi16(sub_result_0, sub_result_0);
        src1_16x16 = _mm256_madd_epi16(sub_result_1, sub_result_1);
        src2_16x16 = _mm256_madd_epi16(sub_result_2, sub_result_2);
        src3_16x16 = _mm256_madd_epi16(sub_result_3, sub_result_3);
        // accumulation of result
        src0_16x16 = _mm256_add_epi32(src0_16x16, src1_16x16);
        src2_16x16 = _mm256_add_epi32(src2_16x16, src3_16x16);
        let square_result_0: __m256i = _mm256_add_epi32(src0_16x16, src2_16x16);
        square_result = _mm256_add_epi32(square_result, square_result_0);
        src_temp = src_temp.offset(16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7  s6  s3  s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_8x8: __m128i;
    let mut dst1_8x8: __m128i;
    let mut dst3_16x8: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(2) {
        dst0_8x8 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        dst1_8x8 = _mm_loadl_epi64(dst.offset(((i + 1) * dstride) as isize) as *const __m128i);
        dst3_16x8 = _mm_unpacklo_epi64(dst0_8x8, dst1_8x8);
        dst_16x16 = _mm256_cvtepu8_epi16(dst3_16x8);
        src0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(src.offset((i * sstride) as isize)
            as *mut __m128i
            as *const __m128i));
        src1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(src.offset(((i + 1) * sstride) as isize)
                as *mut __m128i
                as *const __m128i));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        // r15 r14 r13 - - - r1 r0 - 16 bit
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit
        src_16x16 = _mm256_madd_epi16(sub_result, sub_result);
        // accumulation of result
        square_result = _mm256_add_epi32(square_result, src_16x16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

// Compute mse of two consecutive 8x8 blocks.
// In src buffer, each 8x8 block in a 64x64 filter block is stored sequentially.
// Hence src_blk_stride is same as block width. Whereas dst buffer is a frame
// buffer, thus dstride is a frame level stride.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_dual_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    src_blk_stride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_16x8: __m128i;
    let mut dst1_16x8: __m128i;
    let mut dst0_16x16: __m256i;
    let mut dst1_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result_0: __m256i;
    let mut sub_result_1: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = zeros;
    let mut src_temp: *mut u16 = src;
    for i in (0..h).step_by(2) {
        dst0_16x8 = _mm_loadu_si128(
            dst.offset(((i + 0) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst1_16x8 = _mm_loadu_si128(
            dst.offset(((i + 1) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        // row0 of 1st and 2nd 8x8 block - d00 d10
        dst0_16x16 = _mm256_cvtepu8_epi16(dst0_16x8);
        // row1 of 1st and 2nd 8x8 block - d01 d11
        dst1_16x16 = _mm256_cvtepu8_epi16(dst1_16x8);
        // 2 rows of 1st 8x8 block - r00 r01
        let mut src0_16x16: __m256i = _mm256_loadu_si256(src_temp as *const __m256i);
        // 2 rows of 2nd 8x8 block - r10 r11
        let mut src1_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset(src_blk_stride as isize) as *const __m256i);
        // r00 r10 - 128bit
        let mut tmp0_16x16: __m256i = _mm256_permute2f128_si256::<0x20>(src0_16x16, src1_16x16);
        // r01 r11 - 128bit
        let mut tmp1_16x16: __m256i = _mm256_permute2f128_si256::<0x31>(src0_16x16, src1_16x16);
        // r15 r14 r13------------r1 r0 - 16 bit
        sub_result_0 = _mm256_abs_epi16(_mm256_sub_epi16(tmp0_16x16, dst0_16x16));
        sub_result_1 = _mm256_abs_epi16(_mm256_sub_epi16(tmp1_16x16, dst1_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit each
        src0_16x16 = _mm256_madd_epi16(sub_result_0, sub_result_0);
        src1_16x16 = _mm256_madd_epi16(sub_result_1, sub_result_1);
        // accumulation of result
        src0_16x16 = _mm256_add_epi32(src0_16x16, src1_16x16);
        square_result = _mm256_add_epi32(square_result, src0_16x16);
        src_temp = src_temp.offset(16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    match w {
        4 => {
            return mse_4xh_16bit_avx2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_avx2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}

// Computes mse of two 8x8 or four 4x4 consecutive blocks. Luma plane uses 8x8
// block and Chroma uses 4x4 block. In src buffer, each block in a filter block
// is stored sequentially. Hence src_blk_stride is same as block width. Whereas
// dst buffer is a frame buffer, thus dstride is a frame level stride.
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    match w {
        4 => {
            return mse_4xh_quad_16bit_avx2(dst, dstride, src, w * h, h);
        }
        8 => {
            return mse_8xh_dual_16bit_avx2(dst, dstride, src, w * h, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}
