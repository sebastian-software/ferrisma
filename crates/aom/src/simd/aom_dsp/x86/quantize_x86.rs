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

// Translated by hand from upstream/aom/aom_dsp/x86/quantize_x86.h (ADR 0056,
// ADR 0058). Only the helpers that the translated high-bitdepth adaptive
// quantizers use are translated; the others stay untranslated until a kernel
// needs them. Each helper carries the smallest target feature that its own
// intrinsics need, which is SSE2 for all of them.
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn invert_sign_32_sse2(mut a: __m128i, sign: __m128i) -> __m128i {
    a = _mm_xor_si128(a, sign);
    _mm_sub_epi32(a, sign)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_non_zero_count(mask: __m128i) -> i32 {
    let mut mask0: __m128i;
    let mut mask1: __m128i;
    let non_zero_count: i32;
    mask0 = _mm_unpackhi_epi64(mask, mask);
    mask1 = _mm_max_epi16(mask0, mask);
    mask0 = _mm_shuffle_epi32::<1>(mask1);
    mask0 = _mm_max_epi16(mask0, mask1);
    mask1 = _mm_srli_epi32::<16>(mask0);
    mask0 = _mm_max_epi16(mask0, mask1);
    non_zero_count = _mm_extract_epi16::<0>(mask0) + 1;

    non_zero_count
}
