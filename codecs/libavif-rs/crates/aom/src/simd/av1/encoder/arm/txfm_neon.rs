/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/encoder/arm/txfm_neon.h (ADR 0056).
#![allow(dead_code)]

#[inline(always)]
pub unsafe fn ud_adjust_input_and_stride(ud_flip: i32, input: *mut *const i16, stride: *mut i32,
                                         out_size: i32) {
    if ud_flip != 0 {
        *input = (*input).offset(((out_size - 1) * *stride) as isize);
        *stride = -*stride;
    }
}
