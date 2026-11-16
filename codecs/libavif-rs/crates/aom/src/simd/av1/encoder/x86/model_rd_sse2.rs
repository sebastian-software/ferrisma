/*
 * Copyright (c) 2026, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/model_rd_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
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

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn av1_interp_cubic_rate_dist_sse2(
    p1: *const f64,
    p2: *const f64,
    x: f64,
    rate_dist_f: *mut f64,
) {
    let half: __m128d = _mm_set1_pd(0.5);
    let two: __m128d = _mm_set1_pd(2.0);
    let three: __m128d = _mm_set1_pd(3.0);
    let four: __m128d = _mm_set1_pd(4.0);
    let five: __m128d = _mm_set1_pd(5.0);
    let reg_x: __m128d = _mm_set1_pd(x);
    let reg_p0: __m128d = _mm_set_pd(*p2, *p1);
    let reg_p1: __m128d = _mm_set_pd(*p2.offset(1), *p1.offset(1));
    let reg_p2: __m128d = _mm_set_pd(*p2.offset(2), *p1.offset(2));
    let reg_p3: __m128d = _mm_set_pd(*p2.offset(3), *p1.offset(3));
    // To ensure that results are bit-identical to the C code, we need to perform
    // exactly the same sequence of operations here as in the C code.
    // reg_res_0 = x * (3.0 * (p[1] - p[2]) + p[3] - p[0])
    let mut reg_res_0: __m128d = _mm_sub_pd(reg_p1, reg_p2);
    reg_res_0 = _mm_mul_pd(three, reg_res_0);
    reg_res_0 = _mm_add_pd(reg_res_0, reg_p3);
    reg_res_0 = _mm_sub_pd(reg_res_0, reg_p0);
    reg_res_0 = _mm_mul_pd(reg_x, reg_res_0);
    // reg_res_1 = 2.0 * p[0] - 5.0 * p[1] + 4.0 * p[2]- p[3]
    let regp0_x_2: __m128d = _mm_mul_pd(two, reg_p0);
    let regp1_x_5: __m128d = _mm_mul_pd(five, reg_p1);
    let regp2_x_4: __m128d = _mm_mul_pd(four, reg_p2);
    let mut reg_res_1: __m128d = _mm_sub_pd(regp0_x_2, regp1_x_5);
    reg_res_1 = _mm_add_pd(reg_res_1, regp2_x_4);
    reg_res_1 = _mm_sub_pd(reg_res_1, reg_p3);
    // reg_res_2 = x * (reg_res_1 + reg_res_0)
    let mut reg_res_2: __m128d = _mm_add_pd(reg_res_1, reg_res_0);
    reg_res_2 = _mm_mul_pd(reg_x, reg_res_2);
    // reg_res_3 = p[2] - p[0] + reg_res_2
    let mut reg_res_3: __m128d = _mm_sub_pd(reg_p2, reg_p0);
    reg_res_3 = _mm_add_pd(reg_res_3, reg_res_2);
    // reg_res_4 = p[1] + 0.5 * x * reg_res_3
    let mut reg_res_4: __m128d = _mm_mul_pd(_mm_mul_pd(half, reg_x), reg_res_3);
    reg_res_4 = _mm_add_pd(reg_p1, reg_res_4);
    _mm_storeu_pd(rate_dist_f, reg_res_4);
}
