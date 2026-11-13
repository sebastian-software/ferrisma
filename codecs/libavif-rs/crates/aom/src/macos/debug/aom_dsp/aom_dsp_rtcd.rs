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

// Mechanically translated from upstream/aom/aom_dsp/aom_dsp_rtcd.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_highbd_dc_128_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_left_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_top_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_h_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_paeth_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_quantize_b_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_32x32_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_32x32_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_32x32_adaptive_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_32x32_adaptive_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_64x64_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_64x64_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_64x64_adaptive_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_64x64_adaptive_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_adaptive_c(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_quantize_b_adaptive_neon(
        coeff_ptr: *const tran_low_t,
        n_coeffs: intptr_t,
        zbin_ptr: *const int16_t,
        round_ptr: *const int16_t,
        quant_ptr: *const int16_t,
        quant_shift_ptr: *const int16_t,
        qcoeff_ptr: *mut tran_low_t,
        dqcoeff_ptr: *mut tran_low_t,
        dequant_ptr: *const int16_t,
        eob_ptr: *mut uint16_t,
        scan: *const int16_t,
        iscan: *const int16_t,
    );
    fn aom_highbd_smooth_h_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_h_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_smooth_v_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_subtract_block_c(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
    fn aom_highbd_subtract_block_neon(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
    fn aom_highbd_v_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_32x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_4x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x64_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_64x64_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x16_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x32_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x4_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x4_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x8_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_8x8_neon(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_subtract_block_c(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
    fn aom_subtract_block_neon(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
    fn aom_lpf_vertical_14_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_14_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_14_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_14_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_14_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_14_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_8_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_8_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_8_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_8_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_8_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_8_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_6_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_6_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_6_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_6_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_6_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_6_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_4_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_4_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_4_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_4_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_4_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_4_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_14_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_14_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_14_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_14_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_14_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_14_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_8_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_8_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_8_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_8_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_8_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_8_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_6_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_6_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_6_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_6_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_6_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_6_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_4_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_4_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_4_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_4_dual_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_4_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_4_quad_neon(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_highbd_lpf_horizontal_4_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_dual_neon(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_dc_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x4_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x8_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x16_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x32_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x64_neon(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_quantize_b_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_neon(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_32x32_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_32x32_neon(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_64x64_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_64x64_neon(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_sse_c(a: *const uint8_t, a_stride: core::ffi::c_int, b: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_sse_neon(a: *const uint8_t, a_stride: core::ffi::c_int, b: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_highbd_sse_c(a8: *const uint8_t, a_stride: core::ffi::c_int, b8: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_highbd_sse_neon(a8: *const uint8_t, a_stride: core::ffi::c_int, b8: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_sum_squares_2d_i16_c(src: *const int16_t, stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_sum_squares_2d_i16_neon(src: *const int16_t, stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_sum_squares_i16_c(src: *const int16_t, N: uint32_t) -> uint64_t;
    fn aom_sum_squares_i16_neon(src: *const int16_t, N: uint32_t) -> uint64_t;
    fn aom_var_2d_u8_c(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u8_neon(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u16_c(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u16_neon(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_get_blk_sse_sum_c(data: *const int16_t, stride: core::ffi::c_int, bw: core::ffi::c_int, bh: core::ffi::c_int, x_sum: *mut core::ffi::c_int, x2_sum: *mut int64_t);
    fn aom_get_blk_sse_sum_neon(data: *const int16_t, stride: core::ffi::c_int, bw: core::ffi::c_int, bh: core::ffi::c_int, x_sum: *mut core::ffi::c_int, x2_sum: *mut int64_t);
    fn aom_mse_wxh_16bit_c(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_neon(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_16xh_16bit_c(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_16xh_16bit_neon(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_highbd_c(dst: *mut uint16_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_highbd_neon(dst: *mut uint16_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_sum_sse_2d_i16_c(src: *const int16_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, sum: *mut core::ffi::c_int) -> uint64_t;
    fn aom_sum_sse_2d_i16_neon(src: *const int16_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, sum: *mut core::ffi::c_int) -> uint64_t;
    fn aom_arm_cpu_caps() -> core::ffi::c_int;
}
pub type ptrdiff_t = isize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type intptr_t = isize;
pub type tran_low_t = int32_t;
pub type pthread_once_t = libc::pthread_once_t;
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
#[no_mangle]
pub static mut aom_dc_128_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_128_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_left_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_dc_top_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_get_blk_sse_sum: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_h_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_128_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_left_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_dc_top_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_h_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_14: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_4: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_6: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_8: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_horizontal_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_14: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_4: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_6: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_8: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_lpf_vertical_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_paeth_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b_32x32: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b_32x32_adaptive: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b_64x64: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b_64x64_adaptive: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_quantize_b_adaptive: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        intptr_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *const int16_t,
        *mut tran_low_t,
        *mut tran_low_t,
        *const int16_t,
        *mut uint16_t,
        *const int16_t,
        *const int16_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_h_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_smooth_v_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_sse: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t> = None;
#[no_mangle]
pub static mut aom_highbd_subtract_block: Option<
    unsafe extern "C-unwind" fn(
        core::ffi::c_int,
        core::ffi::c_int,
        *mut int16_t,
        ptrdiff_t,
        *const uint8_t,
        ptrdiff_t,
        *const uint8_t,
        ptrdiff_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_16x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_16x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_16x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_16x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_16x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_32x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_32x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_32x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_32x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_4x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_4x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_4x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_64x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_64x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_64x64: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_8x16: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_8x32: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_8x4: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_highbd_v_predictor_8x8: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_14: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_14_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_4_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_6: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_6_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_horizontal_8_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_14: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_14_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_4_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_6: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_6_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_lpf_vertical_8_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_mse_16xh_16bit: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_mse_wxh_16bit: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_mse_wxh_16bit_highbd: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_paeth_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_quantize_b: Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_quantize_b_32x32: Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_quantize_b_64x64: Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_h_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_smooth_v_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_sse: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t> = None;
#[no_mangle]
pub static mut aom_subtract_block: Option<
    unsafe extern "C-unwind" fn(
        core::ffi::c_int,
        core::ffi::c_int,
        *mut int16_t,
        ptrdiff_t,
        *const uint8_t,
        ptrdiff_t,
        *const uint8_t,
        ptrdiff_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut aom_sum_squares_2d_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_sum_squares_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_sum_sse_2d_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_v_predictor_16x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_16x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_16x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_16x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_16x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_32x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_32x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_32x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_32x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_4x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_4x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_4x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_64x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_64x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_64x64: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_8x16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_8x32: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_8x4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_v_predictor_8x8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()> = None;
#[no_mangle]
pub static mut aom_var_2d_u16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_var_2d_u8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
pub const HAS_NEON: core::ffi::c_int = (1 as core::ffi::c_int) << 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = aom_arm_cpu_caps();
    aom_dc_128_predictor_16x16 = Some(aom_dc_128_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_16x16 = Some(aom_dc_128_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x32 = Some(aom_dc_128_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_16x32 = Some(aom_dc_128_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x4 = Some(aom_dc_128_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_16x4 = Some(aom_dc_128_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x64 = Some(aom_dc_128_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_16x64 = Some(aom_dc_128_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x8 = Some(aom_dc_128_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_16x8 = Some(aom_dc_128_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x16 = Some(aom_dc_128_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_32x16 = Some(aom_dc_128_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x32 = Some(aom_dc_128_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_32x32 = Some(aom_dc_128_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x64 = Some(aom_dc_128_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_32x64 = Some(aom_dc_128_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x8 = Some(aom_dc_128_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_32x8 = Some(aom_dc_128_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_4x16 = Some(aom_dc_128_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_4x16 = Some(aom_dc_128_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_4x4 = Some(aom_dc_128_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_4x4 = Some(aom_dc_128_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_4x8 = Some(aom_dc_128_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_4x8 = Some(aom_dc_128_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x16 = Some(aom_dc_128_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_64x16 = Some(aom_dc_128_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x32 = Some(aom_dc_128_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_64x32 = Some(aom_dc_128_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x64 = Some(aom_dc_128_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_64x64 = Some(aom_dc_128_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x16 = Some(aom_dc_128_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_8x16 = Some(aom_dc_128_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x32 = Some(aom_dc_128_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_8x32 = Some(aom_dc_128_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x4 = Some(aom_dc_128_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_8x4 = Some(aom_dc_128_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x8 = Some(aom_dc_128_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_128_predictor_8x8 = Some(aom_dc_128_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x16 = Some(aom_dc_left_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_16x16 = Some(aom_dc_left_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x32 = Some(aom_dc_left_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_16x32 = Some(aom_dc_left_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x4 = Some(aom_dc_left_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_16x4 = Some(aom_dc_left_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x64 = Some(aom_dc_left_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_16x64 = Some(aom_dc_left_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x8 = Some(aom_dc_left_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_16x8 = Some(aom_dc_left_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x16 = Some(aom_dc_left_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_32x16 = Some(aom_dc_left_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x32 = Some(aom_dc_left_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_32x32 = Some(aom_dc_left_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x64 = Some(aom_dc_left_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_32x64 = Some(aom_dc_left_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x8 = Some(aom_dc_left_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_32x8 = Some(aom_dc_left_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_4x16 = Some(aom_dc_left_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_4x16 = Some(aom_dc_left_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_4x4 = Some(aom_dc_left_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_4x4 = Some(aom_dc_left_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_4x8 = Some(aom_dc_left_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_4x8 = Some(aom_dc_left_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x16 = Some(aom_dc_left_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_64x16 = Some(aom_dc_left_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x32 = Some(aom_dc_left_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_64x32 = Some(aom_dc_left_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x64 = Some(aom_dc_left_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_64x64 = Some(aom_dc_left_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x16 = Some(aom_dc_left_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_8x16 = Some(aom_dc_left_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x32 = Some(aom_dc_left_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_8x32 = Some(aom_dc_left_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x4 = Some(aom_dc_left_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_8x4 = Some(aom_dc_left_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x8 = Some(aom_dc_left_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_left_predictor_8x8 = Some(aom_dc_left_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x16 = Some(aom_dc_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_16x16 = Some(aom_dc_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x32 = Some(aom_dc_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_16x32 = Some(aom_dc_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x4 = Some(aom_dc_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_16x4 = Some(aom_dc_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x64 = Some(aom_dc_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_16x64 = Some(aom_dc_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x8 = Some(aom_dc_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_16x8 = Some(aom_dc_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x16 = Some(aom_dc_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_32x16 = Some(aom_dc_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x32 = Some(aom_dc_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_32x32 = Some(aom_dc_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x64 = Some(aom_dc_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_32x64 = Some(aom_dc_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x8 = Some(aom_dc_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_32x8 = Some(aom_dc_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_4x16 = Some(aom_dc_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_4x16 = Some(aom_dc_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_4x4 = Some(aom_dc_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_4x4 = Some(aom_dc_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_4x8 = Some(aom_dc_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_4x8 = Some(aom_dc_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x16 = Some(aom_dc_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_64x16 = Some(aom_dc_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x32 = Some(aom_dc_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_64x32 = Some(aom_dc_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x64 = Some(aom_dc_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_64x64 = Some(aom_dc_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x16 = Some(aom_dc_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_8x16 = Some(aom_dc_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x32 = Some(aom_dc_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_8x32 = Some(aom_dc_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x4 = Some(aom_dc_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_8x4 = Some(aom_dc_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x8 = Some(aom_dc_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_predictor_8x8 = Some(aom_dc_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x16 = Some(aom_dc_top_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_16x16 = Some(aom_dc_top_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x32 = Some(aom_dc_top_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_16x32 = Some(aom_dc_top_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x4 = Some(aom_dc_top_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_16x4 = Some(aom_dc_top_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x64 = Some(aom_dc_top_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_16x64 = Some(aom_dc_top_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x8 = Some(aom_dc_top_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_16x8 = Some(aom_dc_top_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x16 = Some(aom_dc_top_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_32x16 = Some(aom_dc_top_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x32 = Some(aom_dc_top_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_32x32 = Some(aom_dc_top_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x64 = Some(aom_dc_top_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_32x64 = Some(aom_dc_top_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x8 = Some(aom_dc_top_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_32x8 = Some(aom_dc_top_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_4x16 = Some(aom_dc_top_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_4x16 = Some(aom_dc_top_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_4x4 = Some(aom_dc_top_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_4x4 = Some(aom_dc_top_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_4x8 = Some(aom_dc_top_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_4x8 = Some(aom_dc_top_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x16 = Some(aom_dc_top_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_64x16 = Some(aom_dc_top_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x32 = Some(aom_dc_top_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_64x32 = Some(aom_dc_top_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x64 = Some(aom_dc_top_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_64x64 = Some(aom_dc_top_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x16 = Some(aom_dc_top_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_8x16 = Some(aom_dc_top_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x32 = Some(aom_dc_top_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_8x32 = Some(aom_dc_top_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x4 = Some(aom_dc_top_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_8x4 = Some(aom_dc_top_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x8 = Some(aom_dc_top_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_dc_top_predictor_8x8 = Some(aom_dc_top_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_get_blk_sse_sum = Some(aom_get_blk_sse_sum_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_get_blk_sse_sum = Some(aom_get_blk_sse_sum_neon as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()>;
    }
    aom_h_predictor_16x16 = Some(aom_h_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_16x16 = Some(aom_h_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x32 = Some(aom_h_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_16x32 = Some(aom_h_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x4 = Some(aom_h_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_16x4 = Some(aom_h_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x64 = Some(aom_h_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_16x64 = Some(aom_h_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x8 = Some(aom_h_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_16x8 = Some(aom_h_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x16 = Some(aom_h_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_32x16 = Some(aom_h_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x32 = Some(aom_h_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_32x32 = Some(aom_h_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x64 = Some(aom_h_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_32x64 = Some(aom_h_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x8 = Some(aom_h_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_32x8 = Some(aom_h_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_4x16 = Some(aom_h_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_4x16 = Some(aom_h_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_4x4 = Some(aom_h_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_4x4 = Some(aom_h_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_4x8 = Some(aom_h_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_4x8 = Some(aom_h_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x16 = Some(aom_h_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_64x16 = Some(aom_h_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x32 = Some(aom_h_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_64x32 = Some(aom_h_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x64 = Some(aom_h_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_64x64 = Some(aom_h_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x16 = Some(aom_h_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_8x16 = Some(aom_h_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x32 = Some(aom_h_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_8x32 = Some(aom_h_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x4 = Some(aom_h_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_8x4 = Some(aom_h_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x8 = Some(aom_h_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_h_predictor_8x8 = Some(aom_h_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_highbd_dc_128_predictor_16x16 = Some(
        aom_highbd_dc_128_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_16x16 = Some(
            aom_highbd_dc_128_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_16x32 = Some(
        aom_highbd_dc_128_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_16x32 = Some(
            aom_highbd_dc_128_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_16x4 = Some(
        aom_highbd_dc_128_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_16x4 = Some(
            aom_highbd_dc_128_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_16x64 = Some(
        aom_highbd_dc_128_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_16x64 = Some(
            aom_highbd_dc_128_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_16x8 = Some(
        aom_highbd_dc_128_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_16x8 = Some(
            aom_highbd_dc_128_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_32x16 = Some(
        aom_highbd_dc_128_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_32x16 = Some(
            aom_highbd_dc_128_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_32x32 = Some(
        aom_highbd_dc_128_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_32x32 = Some(
            aom_highbd_dc_128_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_32x64 = Some(
        aom_highbd_dc_128_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_32x64 = Some(
            aom_highbd_dc_128_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_32x8 = Some(
        aom_highbd_dc_128_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_32x8 = Some(
            aom_highbd_dc_128_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_4x16 = Some(
        aom_highbd_dc_128_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_4x16 = Some(
            aom_highbd_dc_128_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_4x4 = Some(
        aom_highbd_dc_128_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_4x4 = Some(
            aom_highbd_dc_128_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_4x8 = Some(
        aom_highbd_dc_128_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_4x8 = Some(
            aom_highbd_dc_128_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_64x16 = Some(
        aom_highbd_dc_128_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_64x16 = Some(
            aom_highbd_dc_128_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_64x32 = Some(
        aom_highbd_dc_128_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_64x32 = Some(
            aom_highbd_dc_128_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_64x64 = Some(
        aom_highbd_dc_128_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_64x64 = Some(
            aom_highbd_dc_128_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_8x16 = Some(
        aom_highbd_dc_128_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_8x16 = Some(
            aom_highbd_dc_128_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_8x32 = Some(
        aom_highbd_dc_128_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_8x32 = Some(
            aom_highbd_dc_128_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_8x4 = Some(
        aom_highbd_dc_128_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_8x4 = Some(
            aom_highbd_dc_128_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_128_predictor_8x8 = Some(
        aom_highbd_dc_128_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_128_predictor_8x8 = Some(
            aom_highbd_dc_128_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_16x16 = Some(
        aom_highbd_dc_left_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_16x16 = Some(
            aom_highbd_dc_left_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_16x32 = Some(
        aom_highbd_dc_left_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_16x32 = Some(
            aom_highbd_dc_left_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_16x4 = Some(
        aom_highbd_dc_left_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_16x4 = Some(
            aom_highbd_dc_left_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_16x64 = Some(
        aom_highbd_dc_left_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_16x64 = Some(
            aom_highbd_dc_left_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_16x8 = Some(
        aom_highbd_dc_left_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_16x8 = Some(
            aom_highbd_dc_left_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_32x16 = Some(
        aom_highbd_dc_left_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_32x16 = Some(
            aom_highbd_dc_left_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_32x32 = Some(
        aom_highbd_dc_left_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_32x32 = Some(
            aom_highbd_dc_left_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_32x64 = Some(
        aom_highbd_dc_left_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_32x64 = Some(
            aom_highbd_dc_left_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_32x8 = Some(
        aom_highbd_dc_left_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_32x8 = Some(
            aom_highbd_dc_left_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_4x16 = Some(
        aom_highbd_dc_left_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_4x16 = Some(
            aom_highbd_dc_left_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_4x4 = Some(
        aom_highbd_dc_left_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_4x4 = Some(
            aom_highbd_dc_left_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_4x8 = Some(
        aom_highbd_dc_left_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_4x8 = Some(
            aom_highbd_dc_left_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_64x16 = Some(
        aom_highbd_dc_left_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_64x16 = Some(
            aom_highbd_dc_left_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_64x32 = Some(
        aom_highbd_dc_left_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_64x32 = Some(
            aom_highbd_dc_left_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_64x64 = Some(
        aom_highbd_dc_left_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_64x64 = Some(
            aom_highbd_dc_left_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_8x16 = Some(
        aom_highbd_dc_left_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_8x16 = Some(
            aom_highbd_dc_left_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_8x32 = Some(
        aom_highbd_dc_left_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_8x32 = Some(
            aom_highbd_dc_left_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_8x4 = Some(
        aom_highbd_dc_left_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_8x4 = Some(
            aom_highbd_dc_left_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_left_predictor_8x8 = Some(
        aom_highbd_dc_left_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_left_predictor_8x8 = Some(
            aom_highbd_dc_left_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_16x16 = Some(
        aom_highbd_dc_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_16x16 = Some(
            aom_highbd_dc_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_16x32 = Some(
        aom_highbd_dc_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_16x32 = Some(
            aom_highbd_dc_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_16x4 = Some(
        aom_highbd_dc_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_16x4 = Some(
            aom_highbd_dc_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_16x64 = Some(
        aom_highbd_dc_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_16x64 = Some(
            aom_highbd_dc_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_16x8 = Some(
        aom_highbd_dc_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_16x8 = Some(
            aom_highbd_dc_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_32x16 = Some(
        aom_highbd_dc_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_32x16 = Some(
            aom_highbd_dc_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_32x32 = Some(
        aom_highbd_dc_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_32x32 = Some(
            aom_highbd_dc_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_32x64 = Some(
        aom_highbd_dc_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_32x64 = Some(
            aom_highbd_dc_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_32x8 = Some(
        aom_highbd_dc_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_32x8 = Some(
            aom_highbd_dc_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_4x16 = Some(
        aom_highbd_dc_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_4x16 = Some(
            aom_highbd_dc_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_4x4 = Some(
        aom_highbd_dc_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_4x4 = Some(
            aom_highbd_dc_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_4x8 = Some(
        aom_highbd_dc_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_4x8 = Some(
            aom_highbd_dc_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_64x16 = Some(
        aom_highbd_dc_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_64x16 = Some(
            aom_highbd_dc_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_64x32 = Some(
        aom_highbd_dc_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_64x32 = Some(
            aom_highbd_dc_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_64x64 = Some(
        aom_highbd_dc_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_64x64 = Some(
            aom_highbd_dc_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_8x16 = Some(
        aom_highbd_dc_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_8x16 = Some(
            aom_highbd_dc_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_8x32 = Some(
        aom_highbd_dc_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_8x32 = Some(
            aom_highbd_dc_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_8x4 = Some(
        aom_highbd_dc_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_8x4 = Some(
            aom_highbd_dc_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_predictor_8x8 = Some(
        aom_highbd_dc_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_predictor_8x8 = Some(
            aom_highbd_dc_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_16x16 = Some(
        aom_highbd_dc_top_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_16x16 = Some(
            aom_highbd_dc_top_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_16x32 = Some(
        aom_highbd_dc_top_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_16x32 = Some(
            aom_highbd_dc_top_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_16x4 = Some(
        aom_highbd_dc_top_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_16x4 = Some(
            aom_highbd_dc_top_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_16x64 = Some(
        aom_highbd_dc_top_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_16x64 = Some(
            aom_highbd_dc_top_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_16x8 = Some(
        aom_highbd_dc_top_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_16x8 = Some(
            aom_highbd_dc_top_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_32x16 = Some(
        aom_highbd_dc_top_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_32x16 = Some(
            aom_highbd_dc_top_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_32x32 = Some(
        aom_highbd_dc_top_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_32x32 = Some(
            aom_highbd_dc_top_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_32x64 = Some(
        aom_highbd_dc_top_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_32x64 = Some(
            aom_highbd_dc_top_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_32x8 = Some(
        aom_highbd_dc_top_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_32x8 = Some(
            aom_highbd_dc_top_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_4x16 = Some(
        aom_highbd_dc_top_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_4x16 = Some(
            aom_highbd_dc_top_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_4x4 = Some(
        aom_highbd_dc_top_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_4x4 = Some(
            aom_highbd_dc_top_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_4x8 = Some(
        aom_highbd_dc_top_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_4x8 = Some(
            aom_highbd_dc_top_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_64x16 = Some(
        aom_highbd_dc_top_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_64x16 = Some(
            aom_highbd_dc_top_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_64x32 = Some(
        aom_highbd_dc_top_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_64x32 = Some(
            aom_highbd_dc_top_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_64x64 = Some(
        aom_highbd_dc_top_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_64x64 = Some(
            aom_highbd_dc_top_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_8x16 = Some(
        aom_highbd_dc_top_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_8x16 = Some(
            aom_highbd_dc_top_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_8x32 = Some(
        aom_highbd_dc_top_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_8x32 = Some(
            aom_highbd_dc_top_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_8x4 = Some(
        aom_highbd_dc_top_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_8x4 = Some(
            aom_highbd_dc_top_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_dc_top_predictor_8x8 = Some(
        aom_highbd_dc_top_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_dc_top_predictor_8x8 = Some(
            aom_highbd_dc_top_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_16x16 = Some(
        aom_highbd_h_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_16x16 = Some(
            aom_highbd_h_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_16x32 = Some(
        aom_highbd_h_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_16x32 = Some(
            aom_highbd_h_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_16x4 = Some(
        aom_highbd_h_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_16x4 = Some(
            aom_highbd_h_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_16x64 = Some(
        aom_highbd_h_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_16x64 = Some(
            aom_highbd_h_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_16x8 = Some(
        aom_highbd_h_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_16x8 = Some(
            aom_highbd_h_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_32x16 = Some(
        aom_highbd_h_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_32x16 = Some(
            aom_highbd_h_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_32x32 = Some(
        aom_highbd_h_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_32x32 = Some(
            aom_highbd_h_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_32x64 = Some(
        aom_highbd_h_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_32x64 = Some(
            aom_highbd_h_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_32x8 = Some(
        aom_highbd_h_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_32x8 = Some(
            aom_highbd_h_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_4x16 = Some(
        aom_highbd_h_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_4x16 = Some(
            aom_highbd_h_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_4x4 = Some(
        aom_highbd_h_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_4x4 = Some(
            aom_highbd_h_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_4x8 = Some(
        aom_highbd_h_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_4x8 = Some(
            aom_highbd_h_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_64x16 = Some(
        aom_highbd_h_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_64x16 = Some(
            aom_highbd_h_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_64x32 = Some(
        aom_highbd_h_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_64x32 = Some(
            aom_highbd_h_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_64x64 = Some(
        aom_highbd_h_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_64x64 = Some(
            aom_highbd_h_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_8x16 = Some(
        aom_highbd_h_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_8x16 = Some(
            aom_highbd_h_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_8x32 = Some(
        aom_highbd_h_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_8x32 = Some(
            aom_highbd_h_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_8x4 = Some(
        aom_highbd_h_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_8x4 = Some(
            aom_highbd_h_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_h_predictor_8x8 = Some(
        aom_highbd_h_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_h_predictor_8x8 = Some(
            aom_highbd_h_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_lpf_horizontal_14 = Some(aom_highbd_lpf_horizontal_14_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_14 = Some(aom_highbd_lpf_horizontal_14_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_14_dual = Some(aom_highbd_lpf_horizontal_14_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_14_dual = Some(aom_highbd_lpf_horizontal_14_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_4 = Some(aom_highbd_lpf_horizontal_4_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_4 = Some(aom_highbd_lpf_horizontal_4_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_4_dual = Some(aom_highbd_lpf_horizontal_4_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_4_dual = Some(aom_highbd_lpf_horizontal_4_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_6 = Some(aom_highbd_lpf_horizontal_6_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_6 = Some(aom_highbd_lpf_horizontal_6_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_6_dual = Some(aom_highbd_lpf_horizontal_6_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_6_dual = Some(aom_highbd_lpf_horizontal_6_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_8 = Some(aom_highbd_lpf_horizontal_8_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_8 = Some(aom_highbd_lpf_horizontal_8_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_8_dual = Some(aom_highbd_lpf_horizontal_8_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_horizontal_8_dual = Some(aom_highbd_lpf_horizontal_8_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_14 = Some(aom_highbd_lpf_vertical_14_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_14 = Some(aom_highbd_lpf_vertical_14_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_14_dual = Some(aom_highbd_lpf_vertical_14_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_14_dual = Some(aom_highbd_lpf_vertical_14_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_4 = Some(aom_highbd_lpf_vertical_4_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_4 = Some(aom_highbd_lpf_vertical_4_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_4_dual = Some(aom_highbd_lpf_vertical_4_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_4_dual = Some(aom_highbd_lpf_vertical_4_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_6 = Some(aom_highbd_lpf_vertical_6_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_6 = Some(aom_highbd_lpf_vertical_6_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_6_dual = Some(aom_highbd_lpf_vertical_6_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_6_dual = Some(aom_highbd_lpf_vertical_6_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_8 = Some(aom_highbd_lpf_vertical_8_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_8 = Some(aom_highbd_lpf_vertical_8_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_8_dual = Some(aom_highbd_lpf_vertical_8_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_highbd_lpf_vertical_8_dual = Some(aom_highbd_lpf_vertical_8_dual_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_paeth_predictor_16x16 = Some(
        aom_highbd_paeth_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_16x16 = Some(
            aom_highbd_paeth_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_16x32 = Some(
        aom_highbd_paeth_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_16x32 = Some(
            aom_highbd_paeth_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_16x4 = Some(
        aom_highbd_paeth_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_16x4 = Some(
            aom_highbd_paeth_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_16x64 = Some(
        aom_highbd_paeth_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_16x64 = Some(
            aom_highbd_paeth_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_16x8 = Some(
        aom_highbd_paeth_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_16x8 = Some(
            aom_highbd_paeth_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_32x16 = Some(
        aom_highbd_paeth_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_32x16 = Some(
            aom_highbd_paeth_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_32x32 = Some(
        aom_highbd_paeth_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_32x32 = Some(
            aom_highbd_paeth_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_32x64 = Some(
        aom_highbd_paeth_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_32x64 = Some(
            aom_highbd_paeth_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_32x8 = Some(
        aom_highbd_paeth_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_32x8 = Some(
            aom_highbd_paeth_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_4x16 = Some(
        aom_highbd_paeth_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_4x16 = Some(
            aom_highbd_paeth_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_4x4 = Some(
        aom_highbd_paeth_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_4x4 = Some(
            aom_highbd_paeth_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_4x8 = Some(
        aom_highbd_paeth_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_4x8 = Some(
            aom_highbd_paeth_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_64x16 = Some(
        aom_highbd_paeth_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_64x16 = Some(
            aom_highbd_paeth_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_64x32 = Some(
        aom_highbd_paeth_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_64x32 = Some(
            aom_highbd_paeth_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_64x64 = Some(
        aom_highbd_paeth_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_64x64 = Some(
            aom_highbd_paeth_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_8x16 = Some(
        aom_highbd_paeth_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_8x16 = Some(
            aom_highbd_paeth_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_8x32 = Some(
        aom_highbd_paeth_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_8x32 = Some(
            aom_highbd_paeth_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_8x4 = Some(
        aom_highbd_paeth_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_8x4 = Some(
            aom_highbd_paeth_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_paeth_predictor_8x8 = Some(
        aom_highbd_paeth_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_paeth_predictor_8x8 = Some(
            aom_highbd_paeth_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b = Some(
        aom_highbd_quantize_b_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b = Some(
            aom_highbd_quantize_b_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b_32x32 = Some(
        aom_highbd_quantize_b_32x32_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b_32x32 = Some(
            aom_highbd_quantize_b_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b_32x32_adaptive = Some(
        aom_highbd_quantize_b_32x32_adaptive_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b_32x32_adaptive = Some(
            aom_highbd_quantize_b_32x32_adaptive_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b_64x64 = Some(
        aom_highbd_quantize_b_64x64_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b_64x64 = Some(
            aom_highbd_quantize_b_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b_64x64_adaptive = Some(
        aom_highbd_quantize_b_64x64_adaptive_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b_64x64_adaptive = Some(
            aom_highbd_quantize_b_64x64_adaptive_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_quantize_b_adaptive = Some(
        aom_highbd_quantize_b_adaptive_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                intptr_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *const int16_t,
                *mut tran_low_t,
                *mut tran_low_t,
                *const int16_t,
                *mut uint16_t,
                *const int16_t,
                *const int16_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_quantize_b_adaptive = Some(
            aom_highbd_quantize_b_adaptive_neon
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    intptr_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *const int16_t,
                    *mut tran_low_t,
                    *mut tran_low_t,
                    *const int16_t,
                    *mut uint16_t,
                    *const int16_t,
                    *const int16_t,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_16x16 = Some(
        aom_highbd_smooth_h_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_16x16 = Some(
            aom_highbd_smooth_h_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_16x32 = Some(
        aom_highbd_smooth_h_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_16x32 = Some(
            aom_highbd_smooth_h_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_16x4 = Some(
        aom_highbd_smooth_h_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_16x4 = Some(
            aom_highbd_smooth_h_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_16x64 = Some(
        aom_highbd_smooth_h_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_16x64 = Some(
            aom_highbd_smooth_h_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_16x8 = Some(
        aom_highbd_smooth_h_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_16x8 = Some(
            aom_highbd_smooth_h_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_32x16 = Some(
        aom_highbd_smooth_h_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_32x16 = Some(
            aom_highbd_smooth_h_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_32x32 = Some(
        aom_highbd_smooth_h_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_32x32 = Some(
            aom_highbd_smooth_h_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_32x64 = Some(
        aom_highbd_smooth_h_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_32x64 = Some(
            aom_highbd_smooth_h_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_32x8 = Some(
        aom_highbd_smooth_h_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_32x8 = Some(
            aom_highbd_smooth_h_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_4x16 = Some(
        aom_highbd_smooth_h_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_4x16 = Some(
            aom_highbd_smooth_h_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_4x4 = Some(
        aom_highbd_smooth_h_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_4x4 = Some(
            aom_highbd_smooth_h_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_4x8 = Some(
        aom_highbd_smooth_h_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_4x8 = Some(
            aom_highbd_smooth_h_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_64x16 = Some(
        aom_highbd_smooth_h_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_64x16 = Some(
            aom_highbd_smooth_h_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_64x32 = Some(
        aom_highbd_smooth_h_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_64x32 = Some(
            aom_highbd_smooth_h_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_64x64 = Some(
        aom_highbd_smooth_h_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_64x64 = Some(
            aom_highbd_smooth_h_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_8x16 = Some(
        aom_highbd_smooth_h_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_8x16 = Some(
            aom_highbd_smooth_h_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_8x32 = Some(
        aom_highbd_smooth_h_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_8x32 = Some(
            aom_highbd_smooth_h_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_8x4 = Some(
        aom_highbd_smooth_h_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_8x4 = Some(
            aom_highbd_smooth_h_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_h_predictor_8x8 = Some(
        aom_highbd_smooth_h_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_h_predictor_8x8 = Some(
            aom_highbd_smooth_h_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_16x16 = Some(
        aom_highbd_smooth_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_16x16 = Some(
            aom_highbd_smooth_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_16x32 = Some(
        aom_highbd_smooth_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_16x32 = Some(
            aom_highbd_smooth_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_16x4 = Some(
        aom_highbd_smooth_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_16x4 = Some(
            aom_highbd_smooth_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_16x64 = Some(
        aom_highbd_smooth_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_16x64 = Some(
            aom_highbd_smooth_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_16x8 = Some(
        aom_highbd_smooth_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_16x8 = Some(
            aom_highbd_smooth_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_32x16 = Some(
        aom_highbd_smooth_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_32x16 = Some(
            aom_highbd_smooth_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_32x32 = Some(
        aom_highbd_smooth_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_32x32 = Some(
            aom_highbd_smooth_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_32x64 = Some(
        aom_highbd_smooth_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_32x64 = Some(
            aom_highbd_smooth_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_32x8 = Some(
        aom_highbd_smooth_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_32x8 = Some(
            aom_highbd_smooth_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_4x16 = Some(
        aom_highbd_smooth_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_4x16 = Some(
            aom_highbd_smooth_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_4x4 = Some(
        aom_highbd_smooth_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_4x4 = Some(
            aom_highbd_smooth_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_4x8 = Some(
        aom_highbd_smooth_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_4x8 = Some(
            aom_highbd_smooth_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_64x16 = Some(
        aom_highbd_smooth_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_64x16 = Some(
            aom_highbd_smooth_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_64x32 = Some(
        aom_highbd_smooth_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_64x32 = Some(
            aom_highbd_smooth_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_64x64 = Some(
        aom_highbd_smooth_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_64x64 = Some(
            aom_highbd_smooth_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_8x16 = Some(
        aom_highbd_smooth_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_8x16 = Some(
            aom_highbd_smooth_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_8x32 = Some(
        aom_highbd_smooth_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_8x32 = Some(
            aom_highbd_smooth_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_8x4 = Some(
        aom_highbd_smooth_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_8x4 = Some(
            aom_highbd_smooth_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_predictor_8x8 = Some(
        aom_highbd_smooth_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_predictor_8x8 = Some(
            aom_highbd_smooth_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_16x16 = Some(
        aom_highbd_smooth_v_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_16x16 = Some(
            aom_highbd_smooth_v_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_16x32 = Some(
        aom_highbd_smooth_v_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_16x32 = Some(
            aom_highbd_smooth_v_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_16x4 = Some(
        aom_highbd_smooth_v_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_16x4 = Some(
            aom_highbd_smooth_v_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_16x64 = Some(
        aom_highbd_smooth_v_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_16x64 = Some(
            aom_highbd_smooth_v_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_16x8 = Some(
        aom_highbd_smooth_v_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_16x8 = Some(
            aom_highbd_smooth_v_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_32x16 = Some(
        aom_highbd_smooth_v_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_32x16 = Some(
            aom_highbd_smooth_v_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_32x32 = Some(
        aom_highbd_smooth_v_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_32x32 = Some(
            aom_highbd_smooth_v_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_32x64 = Some(
        aom_highbd_smooth_v_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_32x64 = Some(
            aom_highbd_smooth_v_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_32x8 = Some(
        aom_highbd_smooth_v_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_32x8 = Some(
            aom_highbd_smooth_v_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_4x16 = Some(
        aom_highbd_smooth_v_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_4x16 = Some(
            aom_highbd_smooth_v_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_4x4 = Some(
        aom_highbd_smooth_v_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_4x4 = Some(
            aom_highbd_smooth_v_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_4x8 = Some(
        aom_highbd_smooth_v_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_4x8 = Some(
            aom_highbd_smooth_v_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_64x16 = Some(
        aom_highbd_smooth_v_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_64x16 = Some(
            aom_highbd_smooth_v_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_64x32 = Some(
        aom_highbd_smooth_v_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_64x32 = Some(
            aom_highbd_smooth_v_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_64x64 = Some(
        aom_highbd_smooth_v_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_64x64 = Some(
            aom_highbd_smooth_v_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_8x16 = Some(
        aom_highbd_smooth_v_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_8x16 = Some(
            aom_highbd_smooth_v_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_8x32 = Some(
        aom_highbd_smooth_v_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_8x32 = Some(
            aom_highbd_smooth_v_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_8x4 = Some(
        aom_highbd_smooth_v_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_8x4 = Some(
            aom_highbd_smooth_v_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_smooth_v_predictor_8x8 = Some(
        aom_highbd_smooth_v_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_smooth_v_predictor_8x8 = Some(
            aom_highbd_smooth_v_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_sse = Some(aom_highbd_sse_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    if flags & HAS_NEON != 0 {
        aom_highbd_sse = Some(aom_highbd_sse_neon as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    }
    aom_highbd_subtract_block = Some(
        aom_highbd_subtract_block_c
            as unsafe extern "C-unwind" fn(
                core::ffi::c_int,
                core::ffi::c_int,
                *mut int16_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                core::ffi::c_int,
                core::ffi::c_int,
                *mut int16_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_subtract_block = Some(
            aom_highbd_subtract_block_neon
                as unsafe extern "C-unwind" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut int16_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut int16_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_16x16 = Some(
        aom_highbd_v_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_16x16 = Some(
            aom_highbd_v_predictor_16x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_16x32 = Some(
        aom_highbd_v_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_16x32 = Some(
            aom_highbd_v_predictor_16x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_16x4 = Some(
        aom_highbd_v_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_16x4 = Some(
            aom_highbd_v_predictor_16x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_16x64 = Some(
        aom_highbd_v_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_16x64 = Some(
            aom_highbd_v_predictor_16x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_16x8 = Some(
        aom_highbd_v_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_16x8 = Some(
            aom_highbd_v_predictor_16x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_32x16 = Some(
        aom_highbd_v_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_32x16 = Some(
            aom_highbd_v_predictor_32x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_32x32 = Some(
        aom_highbd_v_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_32x32 = Some(
            aom_highbd_v_predictor_32x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_32x64 = Some(
        aom_highbd_v_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_32x64 = Some(
            aom_highbd_v_predictor_32x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_32x8 = Some(
        aom_highbd_v_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_32x8 = Some(
            aom_highbd_v_predictor_32x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_4x16 = Some(
        aom_highbd_v_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_4x16 = Some(
            aom_highbd_v_predictor_4x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_4x4 = Some(
        aom_highbd_v_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_4x4 = Some(
            aom_highbd_v_predictor_4x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_4x8 = Some(
        aom_highbd_v_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_4x8 = Some(
            aom_highbd_v_predictor_4x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_64x16 = Some(
        aom_highbd_v_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_64x16 = Some(
            aom_highbd_v_predictor_64x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_64x32 = Some(
        aom_highbd_v_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_64x32 = Some(
            aom_highbd_v_predictor_64x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_64x64 = Some(
        aom_highbd_v_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_64x64 = Some(
            aom_highbd_v_predictor_64x64_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_8x16 = Some(
        aom_highbd_v_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_8x16 = Some(
            aom_highbd_v_predictor_8x16_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_8x32 = Some(
        aom_highbd_v_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_8x32 = Some(
            aom_highbd_v_predictor_8x32_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_8x4 = Some(
        aom_highbd_v_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_8x4 = Some(
            aom_highbd_v_predictor_8x4_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_highbd_v_predictor_8x8 = Some(
        aom_highbd_v_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_highbd_v_predictor_8x8 = Some(
            aom_highbd_v_predictor_8x8_neon
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    aom_lpf_horizontal_14 = Some(aom_lpf_horizontal_14_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_14 = Some(aom_lpf_horizontal_14_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_14_dual = Some(aom_lpf_horizontal_14_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_14_dual = Some(aom_lpf_horizontal_14_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_14_quad = Some(aom_lpf_horizontal_14_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_14_quad = Some(aom_lpf_horizontal_14_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4 = Some(aom_lpf_horizontal_4_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_4 = Some(aom_lpf_horizontal_4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4_dual = Some(aom_lpf_horizontal_4_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_4_dual = Some(aom_lpf_horizontal_4_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4_quad = Some(aom_lpf_horizontal_4_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_4_quad = Some(aom_lpf_horizontal_4_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6 = Some(aom_lpf_horizontal_6_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_6 = Some(aom_lpf_horizontal_6_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6_dual = Some(aom_lpf_horizontal_6_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_6_dual = Some(aom_lpf_horizontal_6_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6_quad = Some(aom_lpf_horizontal_6_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_6_quad = Some(aom_lpf_horizontal_6_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8 = Some(aom_lpf_horizontal_8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_8 = Some(aom_lpf_horizontal_8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8_dual = Some(aom_lpf_horizontal_8_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_8_dual = Some(aom_lpf_horizontal_8_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8_quad = Some(aom_lpf_horizontal_8_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_horizontal_8_quad = Some(aom_lpf_horizontal_8_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14 = Some(aom_lpf_vertical_14_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_14 = Some(aom_lpf_vertical_14_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14_dual = Some(aom_lpf_vertical_14_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_14_dual = Some(aom_lpf_vertical_14_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14_quad = Some(aom_lpf_vertical_14_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_14_quad = Some(aom_lpf_vertical_14_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4 = Some(aom_lpf_vertical_4_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_4 = Some(aom_lpf_vertical_4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4_dual = Some(aom_lpf_vertical_4_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_4_dual = Some(aom_lpf_vertical_4_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4_quad = Some(aom_lpf_vertical_4_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_4_quad = Some(aom_lpf_vertical_4_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6 = Some(aom_lpf_vertical_6_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_6 = Some(aom_lpf_vertical_6_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6_dual = Some(aom_lpf_vertical_6_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_6_dual = Some(aom_lpf_vertical_6_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6_quad = Some(aom_lpf_vertical_6_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_6_quad = Some(aom_lpf_vertical_6_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8 = Some(aom_lpf_vertical_8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_8 = Some(aom_lpf_vertical_8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8_dual = Some(aom_lpf_vertical_8_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_8_dual = Some(aom_lpf_vertical_8_dual_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8_quad = Some(aom_lpf_vertical_8_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_lpf_vertical_8_quad = Some(aom_lpf_vertical_8_quad_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_mse_16xh_16bit = Some(aom_mse_16xh_16bit_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_mse_16xh_16bit = Some(aom_mse_16xh_16bit_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_mse_wxh_16bit = Some(aom_mse_wxh_16bit_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_mse_wxh_16bit = Some(aom_mse_wxh_16bit_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_mse_wxh_16bit_highbd = Some(aom_mse_wxh_16bit_highbd_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_mse_wxh_16bit_highbd = Some(aom_mse_wxh_16bit_highbd_neon as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_paeth_predictor_16x16 = Some(aom_paeth_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_16x16 = Some(aom_paeth_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x32 = Some(aom_paeth_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_16x32 = Some(aom_paeth_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x4 = Some(aom_paeth_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_16x4 = Some(aom_paeth_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x64 = Some(aom_paeth_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_16x64 = Some(aom_paeth_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x8 = Some(aom_paeth_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_16x8 = Some(aom_paeth_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x16 = Some(aom_paeth_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_32x16 = Some(aom_paeth_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x32 = Some(aom_paeth_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_32x32 = Some(aom_paeth_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x64 = Some(aom_paeth_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_32x64 = Some(aom_paeth_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x8 = Some(aom_paeth_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_32x8 = Some(aom_paeth_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x16 = Some(aom_paeth_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_4x16 = Some(aom_paeth_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x4 = Some(aom_paeth_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_4x4 = Some(aom_paeth_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x8 = Some(aom_paeth_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_4x8 = Some(aom_paeth_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x16 = Some(aom_paeth_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_64x16 = Some(aom_paeth_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x32 = Some(aom_paeth_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_64x32 = Some(aom_paeth_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x64 = Some(aom_paeth_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_64x64 = Some(aom_paeth_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x16 = Some(aom_paeth_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_8x16 = Some(aom_paeth_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x32 = Some(aom_paeth_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_8x32 = Some(aom_paeth_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x4 = Some(aom_paeth_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_8x4 = Some(aom_paeth_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x8 = Some(aom_paeth_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_paeth_predictor_8x8 = Some(aom_paeth_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_quantize_b = Some(aom_quantize_b_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_quantize_b = Some(aom_quantize_b_neon as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_quantize_b_32x32 = Some(aom_quantize_b_32x32_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_quantize_b_32x32 = Some(aom_quantize_b_32x32_neon as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_quantize_b_64x64 = Some(aom_quantize_b_64x64_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_quantize_b_64x64 = Some(aom_quantize_b_64x64_neon as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_smooth_h_predictor_16x16 = Some(aom_smooth_h_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_16x16 = Some(aom_smooth_h_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x32 = Some(aom_smooth_h_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_16x32 = Some(aom_smooth_h_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x4 = Some(aom_smooth_h_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_16x4 = Some(aom_smooth_h_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x64 = Some(aom_smooth_h_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_16x64 = Some(aom_smooth_h_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x8 = Some(aom_smooth_h_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_16x8 = Some(aom_smooth_h_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x16 = Some(aom_smooth_h_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_32x16 = Some(aom_smooth_h_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x32 = Some(aom_smooth_h_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_32x32 = Some(aom_smooth_h_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x64 = Some(aom_smooth_h_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_32x64 = Some(aom_smooth_h_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x8 = Some(aom_smooth_h_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_32x8 = Some(aom_smooth_h_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x16 = Some(aom_smooth_h_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_4x16 = Some(aom_smooth_h_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x4 = Some(aom_smooth_h_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_4x4 = Some(aom_smooth_h_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x8 = Some(aom_smooth_h_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_4x8 = Some(aom_smooth_h_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x16 = Some(aom_smooth_h_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_64x16 = Some(aom_smooth_h_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x32 = Some(aom_smooth_h_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_64x32 = Some(aom_smooth_h_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x64 = Some(aom_smooth_h_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_64x64 = Some(aom_smooth_h_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x16 = Some(aom_smooth_h_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_8x16 = Some(aom_smooth_h_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x32 = Some(aom_smooth_h_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_8x32 = Some(aom_smooth_h_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x4 = Some(aom_smooth_h_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_8x4 = Some(aom_smooth_h_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x8 = Some(aom_smooth_h_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_h_predictor_8x8 = Some(aom_smooth_h_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x16 = Some(aom_smooth_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_16x16 = Some(aom_smooth_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x32 = Some(aom_smooth_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_16x32 = Some(aom_smooth_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x4 = Some(aom_smooth_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_16x4 = Some(aom_smooth_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x64 = Some(aom_smooth_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_16x64 = Some(aom_smooth_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x8 = Some(aom_smooth_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_16x8 = Some(aom_smooth_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x16 = Some(aom_smooth_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_32x16 = Some(aom_smooth_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x32 = Some(aom_smooth_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_32x32 = Some(aom_smooth_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x64 = Some(aom_smooth_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_32x64 = Some(aom_smooth_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x8 = Some(aom_smooth_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_32x8 = Some(aom_smooth_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x16 = Some(aom_smooth_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_4x16 = Some(aom_smooth_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x4 = Some(aom_smooth_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_4x4 = Some(aom_smooth_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x8 = Some(aom_smooth_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_4x8 = Some(aom_smooth_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x16 = Some(aom_smooth_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_64x16 = Some(aom_smooth_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x32 = Some(aom_smooth_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_64x32 = Some(aom_smooth_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x64 = Some(aom_smooth_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_64x64 = Some(aom_smooth_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x16 = Some(aom_smooth_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_8x16 = Some(aom_smooth_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x32 = Some(aom_smooth_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_8x32 = Some(aom_smooth_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x4 = Some(aom_smooth_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_8x4 = Some(aom_smooth_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x8 = Some(aom_smooth_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_predictor_8x8 = Some(aom_smooth_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x16 = Some(aom_smooth_v_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_16x16 = Some(aom_smooth_v_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x32 = Some(aom_smooth_v_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_16x32 = Some(aom_smooth_v_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x4 = Some(aom_smooth_v_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_16x4 = Some(aom_smooth_v_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x64 = Some(aom_smooth_v_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_16x64 = Some(aom_smooth_v_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x8 = Some(aom_smooth_v_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_16x8 = Some(aom_smooth_v_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x16 = Some(aom_smooth_v_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_32x16 = Some(aom_smooth_v_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x32 = Some(aom_smooth_v_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_32x32 = Some(aom_smooth_v_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x64 = Some(aom_smooth_v_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_32x64 = Some(aom_smooth_v_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x8 = Some(aom_smooth_v_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_32x8 = Some(aom_smooth_v_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x16 = Some(aom_smooth_v_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_4x16 = Some(aom_smooth_v_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x4 = Some(aom_smooth_v_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_4x4 = Some(aom_smooth_v_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x8 = Some(aom_smooth_v_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_4x8 = Some(aom_smooth_v_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x16 = Some(aom_smooth_v_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_64x16 = Some(aom_smooth_v_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x32 = Some(aom_smooth_v_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_64x32 = Some(aom_smooth_v_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x64 = Some(aom_smooth_v_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_64x64 = Some(aom_smooth_v_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x16 = Some(aom_smooth_v_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_8x16 = Some(aom_smooth_v_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x32 = Some(aom_smooth_v_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_8x32 = Some(aom_smooth_v_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x4 = Some(aom_smooth_v_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_8x4 = Some(aom_smooth_v_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x8 = Some(aom_smooth_v_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_smooth_v_predictor_8x8 = Some(aom_smooth_v_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_sse = Some(aom_sse_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    if flags & HAS_NEON != 0 {
        aom_sse = Some(aom_sse_neon as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    }
    aom_subtract_block = Some(
        aom_subtract_block_c
            as unsafe extern "C-unwind" fn(
                core::ffi::c_int,
                core::ffi::c_int,
                *mut int16_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                core::ffi::c_int,
                core::ffi::c_int,
                *mut int16_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
                *const uint8_t,
                ptrdiff_t,
            ) -> (),
        >;
    if flags & HAS_NEON != 0 {
        aom_subtract_block = Some(
            aom_subtract_block_neon
                as unsafe extern "C-unwind" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut int16_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut int16_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                ) -> (),
            >;
    }
    aom_sum_squares_2d_i16 = Some(aom_sum_squares_2d_i16_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_sum_squares_2d_i16 = Some(aom_sum_squares_2d_i16_neon as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_sum_squares_i16 = Some(aom_sum_squares_i16_c as unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_sum_squares_i16 = Some(aom_sum_squares_i16_neon as unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t>;
    }
    aom_sum_sse_2d_i16 = Some(aom_sum_sse_2d_i16_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_sum_sse_2d_i16 = Some(aom_sum_sse_2d_i16_neon as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t>;
    }
    aom_v_predictor_16x16 = Some(aom_v_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_16x16 = Some(aom_v_predictor_16x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x32 = Some(aom_v_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_16x32 = Some(aom_v_predictor_16x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x4 = Some(aom_v_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_16x4 = Some(aom_v_predictor_16x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x64 = Some(aom_v_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_16x64 = Some(aom_v_predictor_16x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x8 = Some(aom_v_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_16x8 = Some(aom_v_predictor_16x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x16 = Some(aom_v_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_32x16 = Some(aom_v_predictor_32x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x32 = Some(aom_v_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_32x32 = Some(aom_v_predictor_32x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x64 = Some(aom_v_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_32x64 = Some(aom_v_predictor_32x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x8 = Some(aom_v_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_32x8 = Some(aom_v_predictor_32x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_4x16 = Some(aom_v_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_4x16 = Some(aom_v_predictor_4x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_4x4 = Some(aom_v_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_4x4 = Some(aom_v_predictor_4x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_4x8 = Some(aom_v_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_4x8 = Some(aom_v_predictor_4x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x16 = Some(aom_v_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_64x16 = Some(aom_v_predictor_64x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x32 = Some(aom_v_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_64x32 = Some(aom_v_predictor_64x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x64 = Some(aom_v_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_64x64 = Some(aom_v_predictor_64x64_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x16 = Some(aom_v_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_8x16 = Some(aom_v_predictor_8x16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x32 = Some(aom_v_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_8x32 = Some(aom_v_predictor_8x32_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x4 = Some(aom_v_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_8x4 = Some(aom_v_predictor_8x4_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x8 = Some(aom_v_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_NEON != 0 {
        aom_v_predictor_8x8 = Some(aom_v_predictor_8x8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_var_2d_u16 = Some(aom_var_2d_u16_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_var_2d_u16 = Some(aom_var_2d_u16_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_var_2d_u8 = Some(aom_var_2d_u8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_NEON != 0 {
        aom_var_2d_u8 = Some(aom_var_2d_u8_neon as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
}
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func as Option<unsafe extern "C-unwind" fn() -> ()>);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dsp_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
