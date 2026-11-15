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

// Translated from upstream/aom/aom_dsp/x86/highbd_loopfilter_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
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

use crate::simd::aom_dsp__x86__highbd_loopfilter_sse2::*;
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_horizontal_14_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_vertical_14_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_horizontal_4_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_horizontal_8_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_vertical_4_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_dual_avx2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    aom_highbd_lpf_vertical_8_dual_sse2(
        s, p, blimit0, limit0, thresh0, blimit1, limit1, thresh1, bd,
    );
}
