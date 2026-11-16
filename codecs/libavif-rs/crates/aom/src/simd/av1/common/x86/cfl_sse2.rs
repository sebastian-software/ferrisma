/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/common/x86/cfl_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The size wrappers and RTCD getters that the CFL_* macros of av1/common/cfl.h generate are
// expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`); the
// wrappers keep the C ABI because the getters return them as C function pointers.
//
// Translation notes:
// - Every function carries `sse2`, the flag upstream compiles the file with.
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
use core::arch::x86_64::*;

// av1/common/blockd.h
pub const CFL_BUF_LINE: i32 = 32;
pub const CFL_BUF_LINE_I128: i32 = CFL_BUF_LINE >> 3;
pub const CFL_BUF_LINE_I256: i32 = CFL_BUF_LINE >> 4;
// av1/common/enums.h
const TX_SIZES_ALL: i32 = 19;

// av1/common/cfl.h and config/av1_rtcd.h: the function types the RTCD getters return.
pub type cfl_subsample_lbd_fn = Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut u16)>;
pub type cfl_subsample_hbd_fn = Option<unsafe extern "C-unwind" fn(*const u16, i32, *mut u16)>;
pub type cfl_subtract_average_fn = Option<unsafe extern "C-unwind" fn(*const u16, *mut i16)>;
pub type cfl_predict_lbd_fn = Option<unsafe extern "C-unwind" fn(*const i16, *mut u8, i32, i32)>;
pub type cfl_predict_hbd_fn =
    Option<unsafe extern "C-unwind" fn(*const i16, *mut u16, i32, i32, i32)>;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fill_sum_epi32(mut l0: __m128i) -> __m128i {
    l0 = _mm_add_epi32(
        l0,
        _mm_shuffle_epi32::<{ ((1) << 6) | ((0) << 4) | ((3) << 2) | (2) }>(l0),
    );
    return _mm_add_epi32(
        l0,
        _mm_shuffle_epi32::<{ ((2) << 6) | ((3) << 4) | ((0) << 2) | (1) }>(l0),
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn subtract_average_sse2(
    src_ptr: *const u16,
    dst_ptr: *mut i16,
    width: i32,
    height: i32,
    round_offset: i32,
    num_pel_log2: i32,
) {
    let zeros: __m128i = _mm_setzero_si128();
    let round_offset_epi32: __m128i = _mm_set1_epi32(round_offset);
    let mut src: *const __m128i = src_ptr as *mut __m128i as *const __m128i;
    let mut end: *const __m128i = src.offset((height * CFL_BUF_LINE_I128) as isize);
    let step: i32 = CFL_BUF_LINE_I128 * (1 + (width == 8) as i32 + 3 * (width == 4) as i32);
    let mut sum: __m128i = zeros;
    loop {
        let mut l0: __m128i;
        if width == 4 {
            l0 = _mm_add_epi16(
                _mm_loadl_epi64(src),
                _mm_loadl_epi64(src.offset(CFL_BUF_LINE_I128 as isize)),
            );
            let mut l1: __m128i = _mm_add_epi16(
                _mm_loadl_epi64(src.offset((2 * CFL_BUF_LINE_I128) as isize)),
                _mm_loadl_epi64(src.offset((3 * CFL_BUF_LINE_I128) as isize)),
            );
            sum = _mm_add_epi32(
                sum,
                _mm_add_epi32(_mm_unpacklo_epi16(l0, zeros), _mm_unpacklo_epi16(l1, zeros)),
            );
        } else {
            if width == 8 {
                l0 = _mm_add_epi16(
                    _mm_loadu_si128(src),
                    _mm_loadu_si128(src.offset(CFL_BUF_LINE_I128 as isize)),
                );
            } else {
                l0 = _mm_add_epi16(_mm_loadu_si128(src), _mm_loadu_si128(src.offset(1)));
            }
            sum = _mm_add_epi32(
                sum,
                _mm_add_epi32(_mm_unpacklo_epi16(l0, zeros), _mm_unpackhi_epi16(l0, zeros)),
            );
            if width == 32 {
                l0 = _mm_add_epi16(
                    _mm_loadu_si128(src.offset(2)),
                    _mm_loadu_si128(src.offset(3)),
                );
                sum = _mm_add_epi32(
                    sum,
                    _mm_add_epi32(_mm_unpacklo_epi16(l0, zeros), _mm_unpackhi_epi16(l0, zeros)),
                );
            }
        }
        src = src.offset(step as isize);
        if !(src < end) {
            break;
        }
    }
    sum = fill_sum_epi32(sum);
    let mut avg_epi16: __m128i = _mm_srl_epi32(
        _mm_add_epi32(sum, round_offset_epi32),
        _mm_cvtsi32_si128(num_pel_log2),
    );
    avg_epi16 = _mm_packs_epi32(avg_epi16, avg_epi16);
    src = src_ptr as *mut __m128i as *const __m128i;
    let mut dst: *mut __m128i = dst_ptr as *mut __m128i;
    loop {
        if width == 4 {
            _mm_storel_epi64(dst, _mm_sub_epi16(_mm_loadl_epi64(src), avg_epi16));
        } else {
            _mm_storeu_si128(dst, _mm_sub_epi16(_mm_loadu_si128(src), avg_epi16));
            if width > 8 {
                _mm_storeu_si128(
                    dst.offset(1),
                    _mm_sub_epi16(_mm_loadu_si128(src.offset(1)), avg_epi16),
                );
                if width == 32 {
                    _mm_storeu_si128(
                        dst.offset(2),
                        _mm_sub_epi16(_mm_loadu_si128(src.offset(2)), avg_epi16),
                    );
                    _mm_storeu_si128(
                        dst.offset(3),
                        _mm_sub_epi16(_mm_loadu_si128(src.offset(3)), avg_epi16),
                    );
                }
            }
        }
        src = src.offset(CFL_BUF_LINE_I128 as isize);
        dst = dst.offset(CFL_BUF_LINE_I128 as isize);
        if !(src < end) {
            break;
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x4_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 4, 4, 8, 4);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x8_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 4, 8, 16, 5);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x16_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 4, 16, 32, 6);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x4_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 8, 4, 16, 5);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x8_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 8, 8, 32, 6);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x16_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 8, 16, 64, 7);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x32_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 8, 32, 128, 8);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x4_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 16, 4, 32, 6);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x8_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 16, 8, 64, 7);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x16_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 16, 16, 128, 8);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x32_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 16, 32, 256, 9);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x8_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 32, 8, 128, 8);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x16_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 32, 16, 256, 9);
}

#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x32_sse2(src: *const u16, dst: *mut i16) {
    subtract_average_sse2(src, dst, 32, 32, 512, 10);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn cfl_get_subtract_average_fn_sse2(
    tx_size: u8,
) -> cfl_subtract_average_fn {
    let mut sub_avg: [cfl_subtract_average_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subtract_average_4x4_sse2),
        Some(cfl_subtract_average_8x8_sse2),
        Some(cfl_subtract_average_16x16_sse2),
        Some(cfl_subtract_average_32x32_sse2),
        None,
        Some(cfl_subtract_average_4x8_sse2),
        Some(cfl_subtract_average_8x4_sse2),
        Some(cfl_subtract_average_8x16_sse2),
        Some(cfl_subtract_average_16x8_sse2),
        Some(cfl_subtract_average_16x32_sse2),
        Some(cfl_subtract_average_32x16_sse2),
        None,
        None,
        Some(cfl_subtract_average_4x16_sse2),
        Some(cfl_subtract_average_16x4_sse2),
        Some(cfl_subtract_average_8x32_sse2),
        Some(cfl_subtract_average_32x8_sse2),
        None,
        None,
    ];
    return sub_avg[(tx_size as i32 % TX_SIZES_ALL) as usize];
}
