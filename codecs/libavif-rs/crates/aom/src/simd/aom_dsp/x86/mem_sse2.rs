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

// Translated from upstream/aom/aom_dsp/x86/mem_sse2.h (ADR 0056, ADR 0058).
// Only the functions the translated x86 kernels call are translated; they were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
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

use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn loadu_int32(src: *const core::ffi::c_void) -> i32 {
    let mut v: i32 = 0;
    core::ptr::copy_nonoverlapping(
        src as *const u8,
        &raw mut v as *mut u8,
        core::mem::size_of::<i32>(),
    );
    return v;
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn loadu_int64(src: *const core::ffi::c_void) -> i64 {
    let mut v: i64 = 0;
    core::ptr::copy_nonoverlapping(
        src as *const u8,
        &raw mut v as *mut u8,
        core::mem::size_of::<i64>(),
    );
    return v;
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn loadh_epi64(src: *const core::ffi::c_void, s: __m128i) -> __m128i {
    return _mm_castps_si128(_mm_castpd_ps(_mm_loadh_pd(
        _mm_castps_pd(_mm_castsi128_ps(s)),
        &core::ptr::read_unaligned(src as *const u64 as *const f64),
    )));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_8bit_4x4_to_1_reg_sse2(
    src: *const core::ffi::c_void,
    byte_stride: i32,
) -> __m128i {
    return _mm_setr_epi32(
        loadu_int32((src as *mut i8).offset((0 * byte_stride) as isize) as *const core::ffi::c_void),
        loadu_int32((src as *mut i8).offset((1 * byte_stride) as isize) as *const core::ffi::c_void),
        loadu_int32((src as *mut i8).offset((2 * byte_stride) as isize) as *const core::ffi::c_void),
        loadu_int32((src as *mut i8).offset((3 * byte_stride) as isize) as *const core::ffi::c_void),
    );
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_8bit_8x2_to_1_reg_sse2(
    src: *const core::ffi::c_void,
    byte_stride: i32,
) -> __m128i {
    let mut dst: __m128i;
    dst = _mm_loadl_epi64(
        (src as *mut i8).offset((0 * byte_stride) as isize) as *mut __m128i as *const __m128i,
    );
    dst = loadh_epi64(
        (src as *mut i8).offset((1 * byte_stride) as isize) as *const core::ffi::c_void,
        dst,
    );
    return dst;
}

// AOM_AOM_DSP_X86_MEM_SSE2_H_
