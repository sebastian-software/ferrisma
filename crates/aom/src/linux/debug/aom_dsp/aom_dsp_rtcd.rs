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
use core::arch::asm;
extern "C-unwind" {
    fn aom_highbd_dc_128_predictor_16x16_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_dc_128_predictor_16x16_sse2(
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
    fn aom_highbd_dc_128_predictor_16x32_sse2(
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
    fn aom_highbd_dc_128_predictor_16x8_sse2(
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
    fn aom_highbd_dc_128_predictor_32x16_sse2(
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
    fn aom_highbd_dc_128_predictor_32x32_sse2(
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
    fn aom_highbd_dc_128_predictor_4x4_sse2(
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
    fn aom_highbd_dc_128_predictor_4x8_sse2(
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
    fn aom_highbd_dc_128_predictor_8x16_sse2(
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
    fn aom_highbd_dc_128_predictor_8x4_sse2(
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
    fn aom_highbd_dc_128_predictor_8x8_sse2(
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
    fn aom_highbd_dc_left_predictor_16x16_sse2(
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
    fn aom_highbd_dc_left_predictor_16x32_sse2(
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
    fn aom_highbd_dc_left_predictor_16x8_sse2(
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
    fn aom_highbd_dc_left_predictor_32x16_sse2(
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
    fn aom_highbd_dc_left_predictor_32x32_sse2(
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
    fn aom_highbd_dc_left_predictor_4x4_sse2(
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
    fn aom_highbd_dc_left_predictor_4x8_sse2(
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
    fn aom_highbd_dc_left_predictor_8x16_sse2(
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
    fn aom_highbd_dc_left_predictor_8x4_sse2(
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
    fn aom_highbd_dc_left_predictor_8x8_sse2(
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
    fn aom_highbd_dc_predictor_16x32_sse2(
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
    fn aom_highbd_dc_predictor_16x8_sse2(
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
    fn aom_highbd_dc_predictor_32x16_sse2(
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
    fn aom_highbd_dc_predictor_4x8_sse2(
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
    fn aom_highbd_dc_predictor_8x16_sse2(
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
    fn aom_highbd_dc_predictor_8x4_sse2(
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
    fn aom_highbd_dc_top_predictor_16x16_sse2(
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
    fn aom_highbd_dc_top_predictor_16x32_sse2(
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
    fn aom_highbd_dc_top_predictor_16x8_sse2(
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
    fn aom_highbd_dc_top_predictor_32x16_sse2(
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
    fn aom_highbd_dc_top_predictor_32x32_sse2(
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
    fn aom_highbd_dc_top_predictor_4x4_sse2(
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
    fn aom_highbd_dc_top_predictor_4x8_sse2(
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
    fn aom_highbd_dc_top_predictor_8x16_sse2(
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
    fn aom_highbd_dc_top_predictor_8x4_sse2(
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
    fn aom_highbd_dc_top_predictor_8x8_sse2(
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
    fn aom_highbd_h_predictor_16x16_sse2(
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
    fn aom_highbd_h_predictor_16x32_sse2(
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
    fn aom_highbd_h_predictor_16x8_sse2(
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
    fn aom_highbd_h_predictor_32x16_sse2(
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
    fn aom_highbd_h_predictor_32x32_sse2(
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
    fn aom_highbd_h_predictor_4x4_sse2(
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
    fn aom_highbd_h_predictor_4x8_sse2(
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
    fn aom_highbd_h_predictor_8x16_sse2(
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
    fn aom_highbd_h_predictor_8x4_sse2(
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
    fn aom_highbd_h_predictor_8x8_sse2(
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
    fn aom_highbd_quantize_b_avx2(
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
    fn aom_highbd_quantize_b_32x32_avx2(
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
    fn aom_highbd_quantize_b_32x32_adaptive_avx2(
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
    fn aom_highbd_quantize_b_64x64_avx2(
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
    fn aom_highbd_quantize_b_64x64_adaptive_sse2(
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
    fn aom_highbd_quantize_b_adaptive_avx2(
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
    fn aom_highbd_subtract_block_sse2(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
    fn aom_highbd_v_predictor_16x32_c(
        dst: *mut uint16_t,
        y_stride: ptrdiff_t,
        above: *const uint16_t,
        left: *const uint16_t,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_v_predictor_16x32_sse2(
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
    fn aom_highbd_v_predictor_16x8_sse2(
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
    fn aom_highbd_v_predictor_32x16_sse2(
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
    fn aom_highbd_v_predictor_4x8_sse2(
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
    fn aom_highbd_v_predictor_8x16_sse2(
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
    fn aom_highbd_v_predictor_8x4_sse2(
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
    fn aom_subtract_block_avx2(
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
        diff_ptr: *mut int16_t,
        diff_stride: ptrdiff_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        pred_ptr: *const uint8_t,
        pred_stride: ptrdiff_t,
    );
}
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type tran_low_t = int32_t;
pub type pthread_once_t = libc::pthread_once_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const PTHREAD_ONCE_INIT: core::ffi::c_int = 0 as core::ffi::c_int;
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
#[inline]
unsafe extern "C-unwind" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as uint32_t;
    let mut eax: uint32_t = 0;
    let mut edx: uint32_t = 0;
    asm!(
        ".byte 0x0f, 0x01, 0xd0\n", "\n", lateout("ax") eax, lateout("dx") edx,
        inlateout("cx") ecx => _, options(preserves_flags, att_syntax)
    );
    return (edx as uint64_t) << 32 as core::ffi::c_int | eax as uint64_t;
}
pub const HAS_MMX: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const HAS_SSE: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const HAS_SSE2: core::ffi::c_int = 0x4 as core::ffi::c_int;
pub const HAS_SSE3: core::ffi::c_int = 0x8 as core::ffi::c_int;
pub const HAS_SSSE3: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const HAS_SSE4_1: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const HAS_AVX: core::ffi::c_int = 0x40 as core::ffi::c_int;
pub const HAS_AVX2: core::ffi::c_int = 0x80 as core::ffi::c_int;
pub const HAS_SSE4_2: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const HAS_AVX512: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const MMX_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 23 as core::ffi::c_int;
pub const SSE_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 25 as core::ffi::c_int;
pub const SSE2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 26 as core::ffi::c_int;
pub const SSE3_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 0 as core::ffi::c_int;
pub const SSSE3_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 9 as core::ffi::c_int;
pub const SSE4_1_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 19 as core::ffi::c_int;
pub const SSE4_2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 20 as core::ffi::c_int;
pub const AVX_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 27 as core::ffi::c_int | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int;
pub const AVX2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 5 as core::ffi::c_int;
pub const AVX512_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 16 as core::ffi::c_int | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int;
pub const AVX512_DL_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 1 as core::ffi::c_int | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn x86_simd_caps() -> core::ffi::c_int {
    let mut flags: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut mask: core::ffi::c_uint = !(0 as core::ffi::c_uint);
    let mut max_cpuid_val: core::ffi::c_uint = 0;
    let mut reg_eax: core::ffi::c_uint = 0;
    let mut reg_ebx: core::ffi::c_uint = 0;
    let mut reg_ecx: core::ffi::c_uint = 0;
    let mut reg_edx: core::ffi::c_uint = 0;
    let mut env: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    env = getenv(b"AOM_SIMD_CAPS\0" as *const u8 as *const core::ffi::c_char);
    if !env.is_null() && *env as core::ffi::c_int != 0 {
        return strtol(env, 0 as *mut *mut core::ffi::c_char, 0 as core::ffi::c_int)
            as core::ffi::c_int;
    }
    env = getenv(b"AOM_SIMD_CAPS_MASK\0" as *const u8 as *const core::ffi::c_char);
    if !env.is_null() && *env as core::ffi::c_int != 0 {
        mask = strtoul(env, 0 as *mut *mut core::ffi::c_char, 0 as core::ffi::c_int)
            as core::ffi::c_uint;
    }
    let registers = core::arch::x86_64::__cpuid_count(0, 0); max_cpuid_val = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
    if max_cpuid_val < 1 as core::ffi::c_uint {
        return 0 as core::ffi::c_int;
    }
    let registers = core::arch::x86_64::__cpuid_count(1, 0); reg_eax = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 23 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 23 as core::ffi::c_int
        {
            HAS_MMX
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 25 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 25 as core::ffi::c_int
        {
            HAS_SSE
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 26 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 26 as core::ffi::c_int
        {
            HAS_SSE2
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 0 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 0 as core::ffi::c_int
        {
            HAS_SSE3
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
        {
            HAS_SSSE3
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 19 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 19 as core::ffi::c_int
        {
            HAS_SSE4_1
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 20 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 20 as core::ffi::c_int
        {
            HAS_SSE4_2
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    if reg_ecx
        & ((1 as core::ffi::c_uint) << 27 as core::ffi::c_int
            | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int)
        == (1 as core::ffi::c_uint) << 27 as core::ffi::c_int
            | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
    {
        if xgetbv() & 0x6 as uint64_t == 0x6 as uint64_t {
            flags |= HAS_AVX as core::ffi::c_uint;
            if max_cpuid_val >= 7 as core::ffi::c_uint {
                let registers = core::arch::x86_64::__cpuid_count(7, 0); reg_eax = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
                flags
                    |= (if reg_ebx & (1 as core::ffi::c_uint) << 5 as core::ffi::c_int
                        == (1 as core::ffi::c_uint) << 5 as core::ffi::c_int
                    {
                        HAS_AVX2
                    } else {
                        0 as core::ffi::c_int
                    }) as core::ffi::c_uint;
                if xgetbv() & 0xe6 as uint64_t == 0xe6 as uint64_t {
                    flags
                        |= (if reg_ebx
                            & ((1 as core::ffi::c_uint) << 16 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int)
                            == (1 as core::ffi::c_uint) << 16 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int
                            && reg_ecx
                                & ((1 as core::ffi::c_uint) << 1 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int)
                                == (1 as core::ffi::c_uint) << 1 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int
                        {
                            HAS_AVX512
                        } else {
                            0 as core::ffi::c_int
                        }) as core::ffi::c_uint;
                }
            }
        }
    }
    return (flags & mask) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = x86_simd_caps();
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_16x16 = Some(
            aom_highbd_dc_128_predictor_16x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_16x32 = Some(
            aom_highbd_dc_128_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_16x8 = Some(
            aom_highbd_dc_128_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_32x16 = Some(
            aom_highbd_dc_128_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_32x32 = Some(
            aom_highbd_dc_128_predictor_32x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_4x4 = Some(
            aom_highbd_dc_128_predictor_4x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_4x8 = Some(
            aom_highbd_dc_128_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_8x16 = Some(
            aom_highbd_dc_128_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_8x4 = Some(
            aom_highbd_dc_128_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_128_predictor_8x8 = Some(
            aom_highbd_dc_128_predictor_8x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_16x16 = Some(
            aom_highbd_dc_left_predictor_16x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_16x32 = Some(
            aom_highbd_dc_left_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_16x8 = Some(
            aom_highbd_dc_left_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_32x16 = Some(
            aom_highbd_dc_left_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_32x32 = Some(
            aom_highbd_dc_left_predictor_32x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_4x4 = Some(
            aom_highbd_dc_left_predictor_4x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_4x8 = Some(
            aom_highbd_dc_left_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_8x16 = Some(
            aom_highbd_dc_left_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_8x4 = Some(
            aom_highbd_dc_left_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_left_predictor_8x8 = Some(
            aom_highbd_dc_left_predictor_8x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_16x32 = Some(
            aom_highbd_dc_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_16x8 = Some(
            aom_highbd_dc_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_32x16 = Some(
            aom_highbd_dc_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_4x8 = Some(
            aom_highbd_dc_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_8x16 = Some(
            aom_highbd_dc_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_predictor_8x4 = Some(
            aom_highbd_dc_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_16x16 = Some(
            aom_highbd_dc_top_predictor_16x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_16x32 = Some(
            aom_highbd_dc_top_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_16x8 = Some(
            aom_highbd_dc_top_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_32x16 = Some(
            aom_highbd_dc_top_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_32x32 = Some(
            aom_highbd_dc_top_predictor_32x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_4x4 = Some(
            aom_highbd_dc_top_predictor_4x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_4x8 = Some(
            aom_highbd_dc_top_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_8x16 = Some(
            aom_highbd_dc_top_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_8x4 = Some(
            aom_highbd_dc_top_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_dc_top_predictor_8x8 = Some(
            aom_highbd_dc_top_predictor_8x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_16x16 = Some(
            aom_highbd_h_predictor_16x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_16x32 = Some(
            aom_highbd_h_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_16x8 = Some(
            aom_highbd_h_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_32x16 = Some(
            aom_highbd_h_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_32x32 = Some(
            aom_highbd_h_predictor_32x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_4x4 = Some(
            aom_highbd_h_predictor_4x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_4x8 = Some(
            aom_highbd_h_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_8x16 = Some(
            aom_highbd_h_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_8x4 = Some(
            aom_highbd_h_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_h_predictor_8x8 = Some(
            aom_highbd_h_predictor_8x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_AVX2 != 0 {
        aom_highbd_quantize_b = Some(
            aom_highbd_quantize_b_avx2
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
    if flags & HAS_AVX2 != 0 {
        aom_highbd_quantize_b_32x32 = Some(
            aom_highbd_quantize_b_32x32_avx2
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
    if flags & HAS_AVX2 != 0 {
        aom_highbd_quantize_b_32x32_adaptive = Some(
            aom_highbd_quantize_b_32x32_adaptive_avx2
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
    if flags & HAS_AVX2 != 0 {
        aom_highbd_quantize_b_64x64 = Some(
            aom_highbd_quantize_b_64x64_avx2
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_quantize_b_64x64_adaptive = Some(
            aom_highbd_quantize_b_64x64_adaptive_sse2
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
    if flags & HAS_AVX2 != 0 {
        aom_highbd_quantize_b_adaptive = Some(
            aom_highbd_quantize_b_adaptive_avx2
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_subtract_block = Some(
            aom_highbd_subtract_block_sse2
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_16x32 = Some(
            aom_highbd_v_predictor_16x32_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_16x8 = Some(
            aom_highbd_v_predictor_16x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_32x16 = Some(
            aom_highbd_v_predictor_32x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_4x8 = Some(
            aom_highbd_v_predictor_4x8_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_8x16 = Some(
            aom_highbd_v_predictor_8x16_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_SSE2 != 0 {
        aom_highbd_v_predictor_8x4 = Some(
            aom_highbd_v_predictor_8x4_sse2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                ) -> (),
        )
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
    if flags & HAS_AVX2 != 0 {
        aom_subtract_block = Some(
            aom_subtract_block_avx2
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
}
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dsp_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn strtol(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn strtoul(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_ulong;
    fn getenv(__name: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
