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

// Translated by hand from upstream/aom/av1/common/x86/av1_txfm_sse2.h
// (ADR 0056, ADR 0058). Only flip_buf_sse2, which the high-bitdepth inverse
// transforms in highbd_inv_txfm_sse4.c use, is translated so far; the 16-bit
// butterflies, loads, and stores of the low-bitdepth transforms are added by
// the translations that need them.
//
// Translation notes:
// - flip_buf_sse2 uses no intrinsic; it carries `sse2`, the smallest feature
//   for the header, because it moves 128-bit vectors.
// - Array parameters stay raw pointers (`out[i]` -> `*out.offset(i)`).

use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn flip_buf_sse2(in_: *mut __m128i, out: *mut __m128i, size: i32) {
    for i in 0..size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
    }
}
