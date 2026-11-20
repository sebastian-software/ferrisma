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
    fn aom_highbd_12_variance4x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_12_variance4x4_sse4_1(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance4x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance4x4_sse4_1(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_8_variance4x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_8_variance4x4_sse4_1(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_mse16x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_highbd_10_mse16x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_highbd_10_variance128x128_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance128x128_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance128x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance128x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x128_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x128_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance64x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance32x8_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance16x8_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_highbd_10_variance8x8_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut uint32_t) -> core::ffi::c_uint;
    fn aom_get_var_sse_sum_16x16_dual_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse16x16: *mut uint32_t, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var16x16: *mut uint32_t);
    fn aom_get_var_sse_sum_16x16_dual_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse16x16: *mut uint32_t, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var16x16: *mut uint32_t);
    fn aom_get_var_sse_sum_16x16_dual_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse16x16: *mut uint32_t, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var16x16: *mut uint32_t);
    fn aom_get_var_sse_sum_8x8_quad_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse8x8: *mut uint32_t, sum8x8: *mut core::ffi::c_int, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var8x8: *mut uint32_t);
    fn aom_get_var_sse_sum_8x8_quad_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse8x8: *mut uint32_t, sum8x8: *mut core::ffi::c_int, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var8x8: *mut uint32_t);
    fn aom_get_var_sse_sum_8x8_quad_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse8x8: *mut uint32_t, sum8x8: *mut core::ffi::c_int, tot_sse: *mut core::ffi::c_uint, tot_sum: *mut core::ffi::c_int, var8x8: *mut uint32_t);
    fn aom_mse16x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse16x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse16x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse16x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse16x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse8x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse8x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse8x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_mse8x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, recon_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x64_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x8_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x32_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x4_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x4_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x128_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x128_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x128_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x64_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance128x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x128_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x128_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x128_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x64_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x32_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance64x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x64_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x64_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x64_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x32_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance32x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x32_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x32_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x32_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x16_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance16x8_avx2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x16_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x16_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance8x4_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x8_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x8_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x4_c(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_variance4x4_sse2(src_ptr: *const uint8_t, source_stride: core::ffi::c_int, ref_ptr: *const uint8_t, ref_stride: core::ffi::c_int, sse: *mut core::ffi::c_uint) -> core::ffi::c_uint;
    fn aom_highbd_hadamard_32x32_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_highbd_hadamard_32x32_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_highbd_hadamard_16x16_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_highbd_hadamard_16x16_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_highbd_hadamard_8x8_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_highbd_hadamard_8x8_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_lp_8x8_dual_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_8x8_dual_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_8x8_dual_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_16x16_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_16x16_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_16x16_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_8x8_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_lp_8x8_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut int16_t);
    fn aom_hadamard_32x32_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_32x32_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_32x32_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_16x16_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_16x16_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_16x16_avx2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_8x8_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_8x8_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_4x4_c(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_hadamard_4x4_sse2(src_diff: *const int16_t, src_stride: ptrdiff_t, coeff: *mut tran_low_t);
    fn aom_mse_wxh_16bit_highbd_c(dst: *mut uint16_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_highbd_sse2(dst: *mut uint16_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_highbd_avx2(dst: *mut uint16_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_16xh_16bit_c(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_16xh_16bit_sse2(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_16xh_16bit_avx2(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_c(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_sse2(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_mse_wxh_16bit_avx2(dst: *mut uint8_t, dstride: core::ffi::c_int, src: *mut uint16_t, sstride: core::ffi::c_int, w: core::ffi::c_int, h: core::ffi::c_int) -> uint64_t;
    fn aom_get_blk_sse_sum_c(data: *const int16_t, stride: core::ffi::c_int, bw: core::ffi::c_int, bh: core::ffi::c_int, x_sum: *mut core::ffi::c_int, x2_sum: *mut int64_t);
    fn aom_get_blk_sse_sum_sse2(data: *const int16_t, stride: core::ffi::c_int, bw: core::ffi::c_int, bh: core::ffi::c_int, x_sum: *mut core::ffi::c_int, x2_sum: *mut int64_t);
    fn aom_get_blk_sse_sum_avx2(data: *const int16_t, stride: core::ffi::c_int, bw: core::ffi::c_int, bh: core::ffi::c_int, x_sum: *mut core::ffi::c_int, x2_sum: *mut int64_t);
    fn aom_sum_sse_2d_i16_c(src: *const int16_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, sum: *mut core::ffi::c_int) -> uint64_t;
    fn aom_sum_sse_2d_i16_sse2(src: *const int16_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, sum: *mut core::ffi::c_int) -> uint64_t;
    fn aom_sum_sse_2d_i16_avx2(src: *const int16_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, sum: *mut core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u16_c(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u16_sse2(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u16_avx2(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u8_c(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u8_sse2(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_var_2d_u8_avx2(src: *mut uint8_t, src_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_sum_squares_i16_c(src: *const int16_t, N: uint32_t) -> uint64_t;
    fn aom_sum_squares_i16_sse2(src: *const int16_t, N: uint32_t) -> uint64_t;
    fn aom_sum_squares_2d_i16_c(src: *const int16_t, stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_sum_squares_2d_i16_sse2(src: *const int16_t, stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_sum_squares_2d_i16_avx2(src: *const int16_t, stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> uint64_t;
    fn aom_highbd_sse_c(a8: *const uint8_t, a_stride: core::ffi::c_int, b8: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_highbd_sse_sse4_1(a8: *const uint8_t, a_stride: core::ffi::c_int, b8: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_highbd_sse_avx2(a8: *const uint8_t, a_stride: core::ffi::c_int, b8: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_sse_c(a: *const uint8_t, a_stride: core::ffi::c_int, b: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_sse_sse4_1(a: *const uint8_t, a_stride: core::ffi::c_int, b: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_sse_avx2(a: *const uint8_t, a_stride: core::ffi::c_int, b: *const uint8_t, b_stride: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int) -> int64_t;
    fn aom_quantize_b_64x64_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_64x64_ssse3(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_64x64_avx2(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_32x32_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_32x32_avx2(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_c(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_sse2(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_quantize_b_avx2(coeff_ptr: *const tran_low_t, n_coeffs: intptr_t, zbin_ptr: *const int16_t, round_ptr: *const int16_t, quant_ptr: *const int16_t, quant_shift_ptr: *const int16_t, qcoeff_ptr: *mut tran_low_t, dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const int16_t, eob_ptr: *mut uint16_t, scan: *const int16_t, iscan: *const int16_t);
    fn aom_v_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_v_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_8x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_64x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_4x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_32x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_v_predictor_16x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_8x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_64x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_4x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_32x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_predictor_16x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_8x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_64x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_4x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_32x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_smooth_h_predictor_16x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_8x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_4x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x8_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x8_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x64_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x4_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x32_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x16_ssse3(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_paeth_predictor_16x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_h_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_top_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_left_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_8x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_64x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_4x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x64_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x32_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x16_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x16_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_32x16_avx2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x8_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x8_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x64_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x64_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x4_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x4_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x32_c(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_dc_128_predictor_16x32_sse2(dst: *mut uint8_t, y_stride: ptrdiff_t, above: *const uint8_t, left: *const uint8_t);
    fn aom_lpf_vertical_8_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_8_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_8_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_8_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_8_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_8_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_6_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_6_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_6_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_6_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_6_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_6_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_4_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_4_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_4_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_4_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_4_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_4_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_14_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_14_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_vertical_14_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_14_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_14_quad_avx2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_vertical_14_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_vertical_14_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_8_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_8_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_8_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_8_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_8_quad_avx2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_8_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_8_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_6_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_6_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_6_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_6_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_6_quad_avx2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_6_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_6_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_4_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_4_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_4_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_4_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_4_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_4_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_14_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_14_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t);
    fn aom_lpf_horizontal_14_quad_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_14_quad_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_14_quad_avx2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t);
    fn aom_lpf_horizontal_14_dual_c(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_lpf_horizontal_14_dual_sse2(s: *mut uint8_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t);
    fn aom_highbd_lpf_vertical_8_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_8_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_6_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_4_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_vertical_14_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_8_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_6_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_4_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit: *const uint8_t, limit: *const uint8_t, thresh: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_dual_c(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_dual_sse2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
    fn aom_highbd_lpf_horizontal_14_dual_avx2(s: *mut uint16_t, pitch: core::ffi::c_int, blimit0: *const uint8_t, limit0: *const uint8_t, thresh0: *const uint8_t, blimit1: *const uint8_t, limit1: *const uint8_t, thresh1: *const uint8_t, bd: core::ffi::c_int);
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
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type tran_low_t = int32_t;
pub type pthread_once_t = libc::pthread_once_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const PTHREAD_ONCE_INIT: core::ffi::c_int = 0 as core::ffi::c_int;
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
pub static mut aom_get_blk_sse_sum: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()> = None;
#[no_mangle]
pub static mut aom_get_var_sse_sum_16x16_dual: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()> = None;
#[no_mangle]
pub static mut aom_get_var_sse_sum_8x8_quad: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()> = None;
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
pub static mut aom_hadamard_16x16: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_32x32: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_4x4: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_8x8: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_lp_16x16: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_lp_8x8: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_hadamard_lp_8x8_dual: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_10_mse16x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance128x128: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance128x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance16x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance16x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance16x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance16x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance32x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance32x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance32x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance32x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance4x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance64x128: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance64x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance64x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance64x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance8x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance8x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_10_variance8x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_12_variance4x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_highbd_8_variance4x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint> = None;
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
pub static mut aom_highbd_hadamard_16x16: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_hadamard_32x32: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
#[no_mangle]
pub static mut aom_highbd_hadamard_8x8: Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()> = None;
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
pub static mut aom_mse16x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_mse16x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_mse8x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_mse8x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
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
#[inline]
unsafe extern "C-unwind" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as uint32_t;
    // XGETBV (0F 01 D0) through its intrinsic instead of the original's raw
    // opcode bytes (engine ADR-0044): the same instruction with ECX = 0,
    // returning EDX:EAX as one value as the original does.
    #[target_feature(enable = "xsave")]
    unsafe fn read_xcr(xcr: uint32_t) -> uint64_t {
        core::arch::x86_64::_xgetbv(xcr)
    }
    // SAFETY: XGETBV needs CR4.OSXSAVE, which CPUID.1:ECX bit 27 (OSXSAVE)
    // reports and the OS can set only on a CPU with XSAVE, the feature
    // `read_xcr` enables. The original calls xgetbv() only inside its branch
    // that requires CPUID.1:ECX bits 27 and 28; the callers keep that branch.
    return read_xcr(ecx);
}
#[no_mangle]
pub static mut aom_sum_squares_2d_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_sum_squares_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_sum_sse_2d_i16: Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t> = None;
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
pub static mut aom_var_2d_u16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_var_2d_u8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t> = None;
#[no_mangle]
pub static mut aom_variance128x128: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance128x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance16x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance16x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance16x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance16x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance16x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance32x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance32x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance32x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance32x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance4x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance4x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance4x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance64x128: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance64x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance64x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance64x64: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance8x16: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance8x32: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance8x4: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
#[no_mangle]
pub static mut aom_variance8x8: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint> = None;
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
    aom_dc_128_predictor_16x32 = Some(aom_dc_128_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_16x32 = Some(aom_dc_128_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x4 = Some(aom_dc_128_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_16x4 = Some(aom_dc_128_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x64 = Some(aom_dc_128_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_16x64 = Some(aom_dc_128_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_16x8 = Some(aom_dc_128_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_16x8 = Some(aom_dc_128_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x16 = Some(aom_dc_128_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_32x16 = Some(aom_dc_128_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_32x16 = Some(aom_dc_128_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x32 = Some(aom_dc_128_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_32x32 = Some(aom_dc_128_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x64 = Some(aom_dc_128_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_32x64 = Some(aom_dc_128_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_32x64 = Some(aom_dc_128_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_32x8 = Some(aom_dc_128_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_32x8 = Some(aom_dc_128_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_4x16 = Some(aom_dc_128_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_4x16 = Some(aom_dc_128_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_4x8 = Some(aom_dc_128_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_4x8 = Some(aom_dc_128_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x16 = Some(aom_dc_128_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_64x16 = Some(aom_dc_128_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_64x16 = Some(aom_dc_128_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x32 = Some(aom_dc_128_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_64x32 = Some(aom_dc_128_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_64x32 = Some(aom_dc_128_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_64x64 = Some(aom_dc_128_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_64x64 = Some(aom_dc_128_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_128_predictor_64x64 = Some(aom_dc_128_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x16 = Some(aom_dc_128_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_8x16 = Some(aom_dc_128_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x32 = Some(aom_dc_128_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_8x32 = Some(aom_dc_128_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_128_predictor_8x4 = Some(aom_dc_128_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_128_predictor_8x4 = Some(aom_dc_128_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x32 = Some(aom_dc_left_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_16x32 = Some(aom_dc_left_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x4 = Some(aom_dc_left_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_16x4 = Some(aom_dc_left_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x64 = Some(aom_dc_left_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_16x64 = Some(aom_dc_left_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_16x8 = Some(aom_dc_left_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_16x8 = Some(aom_dc_left_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x16 = Some(aom_dc_left_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_32x16 = Some(aom_dc_left_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_32x16 = Some(aom_dc_left_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x32 = Some(aom_dc_left_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_32x32 = Some(aom_dc_left_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x64 = Some(aom_dc_left_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_32x64 = Some(aom_dc_left_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_32x64 = Some(aom_dc_left_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_32x8 = Some(aom_dc_left_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_32x8 = Some(aom_dc_left_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_4x16 = Some(aom_dc_left_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_4x16 = Some(aom_dc_left_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_4x8 = Some(aom_dc_left_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_4x8 = Some(aom_dc_left_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x16 = Some(aom_dc_left_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_64x16 = Some(aom_dc_left_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_64x16 = Some(aom_dc_left_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x32 = Some(aom_dc_left_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_64x32 = Some(aom_dc_left_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_64x32 = Some(aom_dc_left_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_64x64 = Some(aom_dc_left_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_64x64 = Some(aom_dc_left_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_left_predictor_64x64 = Some(aom_dc_left_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x16 = Some(aom_dc_left_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_8x16 = Some(aom_dc_left_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x32 = Some(aom_dc_left_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_8x32 = Some(aom_dc_left_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_left_predictor_8x4 = Some(aom_dc_left_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_left_predictor_8x4 = Some(aom_dc_left_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x32 = Some(aom_dc_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_16x32 = Some(aom_dc_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x4 = Some(aom_dc_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_16x4 = Some(aom_dc_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x64 = Some(aom_dc_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_16x64 = Some(aom_dc_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_16x8 = Some(aom_dc_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_16x8 = Some(aom_dc_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x16 = Some(aom_dc_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_32x16 = Some(aom_dc_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_32x16 = Some(aom_dc_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x32 = Some(aom_dc_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_32x32 = Some(aom_dc_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x64 = Some(aom_dc_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_32x64 = Some(aom_dc_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_32x64 = Some(aom_dc_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_32x8 = Some(aom_dc_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_32x8 = Some(aom_dc_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_4x16 = Some(aom_dc_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_4x16 = Some(aom_dc_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_4x8 = Some(aom_dc_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_4x8 = Some(aom_dc_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x16 = Some(aom_dc_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_64x16 = Some(aom_dc_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_64x16 = Some(aom_dc_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x32 = Some(aom_dc_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_64x32 = Some(aom_dc_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_64x32 = Some(aom_dc_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_64x64 = Some(aom_dc_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_64x64 = Some(aom_dc_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_predictor_64x64 = Some(aom_dc_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x16 = Some(aom_dc_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_8x16 = Some(aom_dc_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x32 = Some(aom_dc_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_8x32 = Some(aom_dc_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_predictor_8x4 = Some(aom_dc_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_predictor_8x4 = Some(aom_dc_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x32 = Some(aom_dc_top_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_16x32 = Some(aom_dc_top_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x4 = Some(aom_dc_top_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_16x4 = Some(aom_dc_top_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x64 = Some(aom_dc_top_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_16x64 = Some(aom_dc_top_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_16x8 = Some(aom_dc_top_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_16x8 = Some(aom_dc_top_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x16 = Some(aom_dc_top_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_32x16 = Some(aom_dc_top_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_32x16 = Some(aom_dc_top_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x32 = Some(aom_dc_top_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_32x32 = Some(aom_dc_top_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x64 = Some(aom_dc_top_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_32x64 = Some(aom_dc_top_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_32x64 = Some(aom_dc_top_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_32x8 = Some(aom_dc_top_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_32x8 = Some(aom_dc_top_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_4x16 = Some(aom_dc_top_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_4x16 = Some(aom_dc_top_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_4x8 = Some(aom_dc_top_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_4x8 = Some(aom_dc_top_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x16 = Some(aom_dc_top_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_64x16 = Some(aom_dc_top_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_64x16 = Some(aom_dc_top_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x32 = Some(aom_dc_top_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_64x32 = Some(aom_dc_top_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_64x32 = Some(aom_dc_top_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_64x64 = Some(aom_dc_top_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_64x64 = Some(aom_dc_top_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_dc_top_predictor_64x64 = Some(aom_dc_top_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x16 = Some(aom_dc_top_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_8x16 = Some(aom_dc_top_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x32 = Some(aom_dc_top_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_8x32 = Some(aom_dc_top_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_dc_top_predictor_8x4 = Some(aom_dc_top_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_dc_top_predictor_8x4 = Some(aom_dc_top_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_get_blk_sse_sum = Some(aom_get_blk_sse_sum_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_get_blk_sse_sum = Some(aom_get_blk_sse_sum_sse2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_get_blk_sse_sum = Some(aom_get_blk_sse_sum_avx2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int, *mut int64_t) -> ()>;
    }
    aom_get_var_sse_sum_16x16_dual = Some(aom_get_var_sse_sum_16x16_dual_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_get_var_sse_sum_16x16_dual = Some(aom_get_var_sse_sum_16x16_dual_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_get_var_sse_sum_16x16_dual = Some(aom_get_var_sse_sum_16x16_dual_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    }
    aom_get_var_sse_sum_8x8_quad = Some(aom_get_var_sse_sum_8x8_quad_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_get_var_sse_sum_8x8_quad = Some(aom_get_var_sse_sum_8x8_quad_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_get_var_sse_sum_8x8_quad = Some(aom_get_var_sse_sum_8x8_quad_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t, *mut core::ffi::c_int, *mut core::ffi::c_uint, *mut core::ffi::c_int, *mut uint32_t) -> ()>;
    }
    aom_h_predictor_16x32 = Some(aom_h_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_16x32 = Some(aom_h_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x4 = Some(aom_h_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_16x4 = Some(aom_h_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x64 = Some(aom_h_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_16x64 = Some(aom_h_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_16x8 = Some(aom_h_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_16x8 = Some(aom_h_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x16 = Some(aom_h_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_32x16 = Some(aom_h_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x32 = Some(aom_h_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_h_predictor_32x32 = Some(aom_h_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x64 = Some(aom_h_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_32x64 = Some(aom_h_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_32x8 = Some(aom_h_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_32x8 = Some(aom_h_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_4x16 = Some(aom_h_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_4x16 = Some(aom_h_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_4x8 = Some(aom_h_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_4x8 = Some(aom_h_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x16 = Some(aom_h_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_64x16 = Some(aom_h_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x32 = Some(aom_h_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_64x32 = Some(aom_h_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_64x64 = Some(aom_h_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_64x64 = Some(aom_h_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x16 = Some(aom_h_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_8x16 = Some(aom_h_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x32 = Some(aom_h_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_8x32 = Some(aom_h_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_h_predictor_8x4 = Some(aom_h_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_h_predictor_8x4 = Some(aom_h_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_hadamard_16x16 = Some(aom_hadamard_16x16_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_16x16 = Some(aom_hadamard_16x16_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_hadamard_16x16 = Some(aom_hadamard_16x16_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_hadamard_32x32 = Some(aom_hadamard_32x32_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_32x32 = Some(aom_hadamard_32x32_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_hadamard_32x32 = Some(aom_hadamard_32x32_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_hadamard_4x4 = Some(aom_hadamard_4x4_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_4x4 = Some(aom_hadamard_4x4_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_hadamard_8x8 = Some(aom_hadamard_8x8_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_8x8 = Some(aom_hadamard_8x8_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_hadamard_lp_16x16 = Some(aom_hadamard_lp_16x16_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_lp_16x16 = Some(aom_hadamard_lp_16x16_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_hadamard_lp_16x16 = Some(aom_hadamard_lp_16x16_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    }
    aom_hadamard_lp_8x8 = Some(aom_hadamard_lp_8x8_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_lp_8x8 = Some(aom_hadamard_lp_8x8_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    }
    aom_hadamard_lp_8x8_dual = Some(aom_hadamard_lp_8x8_dual_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_hadamard_lp_8x8_dual = Some(aom_hadamard_lp_8x8_dual_sse2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_hadamard_lp_8x8_dual = Some(aom_hadamard_lp_8x8_dual_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut int16_t) -> ()>;
    }
    aom_highbd_10_mse16x16 = Some(aom_highbd_10_mse16x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_mse16x16 = Some(aom_highbd_10_mse16x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance128x128 = Some(aom_highbd_10_variance128x128_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance128x128 = Some(aom_highbd_10_variance128x128_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance128x64 = Some(aom_highbd_10_variance128x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance128x64 = Some(aom_highbd_10_variance128x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance16x16 = Some(aom_highbd_10_variance16x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance16x16 = Some(aom_highbd_10_variance16x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance16x32 = Some(aom_highbd_10_variance16x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance16x32 = Some(aom_highbd_10_variance16x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance16x64 = Some(aom_highbd_10_variance16x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance16x64 = Some(aom_highbd_10_variance16x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance16x8 = Some(aom_highbd_10_variance16x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance16x8 = Some(aom_highbd_10_variance16x8_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance32x16 = Some(aom_highbd_10_variance32x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance32x16 = Some(aom_highbd_10_variance32x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance32x32 = Some(aom_highbd_10_variance32x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance32x32 = Some(aom_highbd_10_variance32x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance32x64 = Some(aom_highbd_10_variance32x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance32x64 = Some(aom_highbd_10_variance32x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance32x8 = Some(aom_highbd_10_variance32x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance32x8 = Some(aom_highbd_10_variance32x8_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance4x4 = Some(aom_highbd_10_variance4x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_SSE4_1 != 0 {
        aom_highbd_10_variance4x4 = Some(aom_highbd_10_variance4x4_sse4_1 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance64x128 = Some(aom_highbd_10_variance64x128_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance64x128 = Some(aom_highbd_10_variance64x128_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance64x16 = Some(aom_highbd_10_variance64x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance64x16 = Some(aom_highbd_10_variance64x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance64x32 = Some(aom_highbd_10_variance64x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance64x32 = Some(aom_highbd_10_variance64x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance64x64 = Some(aom_highbd_10_variance64x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance64x64 = Some(aom_highbd_10_variance64x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance8x16 = Some(aom_highbd_10_variance8x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance8x16 = Some(aom_highbd_10_variance8x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance8x32 = Some(aom_highbd_10_variance8x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance8x32 = Some(aom_highbd_10_variance8x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_10_variance8x8 = Some(aom_highbd_10_variance8x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_10_variance8x8 = Some(aom_highbd_10_variance8x8_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_12_variance4x4 = Some(aom_highbd_12_variance4x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_SSE4_1 != 0 {
        aom_highbd_12_variance4x4 = Some(aom_highbd_12_variance4x4_sse4_1 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    }
    aom_highbd_8_variance4x4 = Some(aom_highbd_8_variance4x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
    if flags & HAS_SSE4_1 != 0 {
        aom_highbd_8_variance4x4 = Some(aom_highbd_8_variance4x4_sse4_1 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut uint32_t) -> core::ffi::c_uint>;
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
    aom_highbd_hadamard_16x16 = Some(aom_highbd_hadamard_16x16_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_hadamard_16x16 = Some(aom_highbd_hadamard_16x16_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_highbd_hadamard_32x32 = Some(aom_highbd_hadamard_32x32_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_hadamard_32x32 = Some(aom_highbd_hadamard_32x32_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_highbd_hadamard_8x8 = Some(aom_highbd_hadamard_8x8_c as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_highbd_hadamard_8x8 = Some(aom_highbd_hadamard_8x8_avx2 as unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const int16_t, ptrdiff_t, *mut tran_low_t) -> ()>;
    }
    aom_highbd_lpf_horizontal_14 = Some(aom_highbd_lpf_horizontal_14_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_14 = Some(aom_highbd_lpf_horizontal_14_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_14_dual = Some(aom_highbd_lpf_horizontal_14_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_14_dual = Some(aom_highbd_lpf_horizontal_14_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_horizontal_14_dual = Some(aom_highbd_lpf_horizontal_14_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_4 = Some(aom_highbd_lpf_horizontal_4_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_4 = Some(aom_highbd_lpf_horizontal_4_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_4_dual = Some(aom_highbd_lpf_horizontal_4_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_4_dual = Some(aom_highbd_lpf_horizontal_4_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_horizontal_4_dual = Some(aom_highbd_lpf_horizontal_4_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_6 = Some(aom_highbd_lpf_horizontal_6_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_6 = Some(aom_highbd_lpf_horizontal_6_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_6_dual = Some(aom_highbd_lpf_horizontal_6_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_6_dual = Some(aom_highbd_lpf_horizontal_6_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_8 = Some(aom_highbd_lpf_horizontal_8_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_8 = Some(aom_highbd_lpf_horizontal_8_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_horizontal_8_dual = Some(aom_highbd_lpf_horizontal_8_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_horizontal_8_dual = Some(aom_highbd_lpf_horizontal_8_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_horizontal_8_dual = Some(aom_highbd_lpf_horizontal_8_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_14 = Some(aom_highbd_lpf_vertical_14_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_14 = Some(aom_highbd_lpf_vertical_14_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_14_dual = Some(aom_highbd_lpf_vertical_14_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_14_dual = Some(aom_highbd_lpf_vertical_14_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_vertical_14_dual = Some(aom_highbd_lpf_vertical_14_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_4 = Some(aom_highbd_lpf_vertical_4_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_4 = Some(aom_highbd_lpf_vertical_4_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_4_dual = Some(aom_highbd_lpf_vertical_4_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_4_dual = Some(aom_highbd_lpf_vertical_4_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_vertical_4_dual = Some(aom_highbd_lpf_vertical_4_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_6 = Some(aom_highbd_lpf_vertical_6_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_6 = Some(aom_highbd_lpf_vertical_6_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_6_dual = Some(aom_highbd_lpf_vertical_6_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_6_dual = Some(aom_highbd_lpf_vertical_6_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_8 = Some(aom_highbd_lpf_vertical_8_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_8 = Some(aom_highbd_lpf_vertical_8_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    aom_highbd_lpf_vertical_8_dual = Some(aom_highbd_lpf_vertical_8_dual_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_highbd_lpf_vertical_8_dual = Some(aom_highbd_lpf_vertical_8_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_lpf_vertical_8_dual = Some(aom_highbd_lpf_vertical_8_dual_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
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
    aom_highbd_sse = Some(aom_highbd_sse_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    if flags & HAS_SSE4_1 != 0 {
        aom_highbd_sse = Some(aom_highbd_sse_sse4_1 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_highbd_sse = Some(aom_highbd_sse_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
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
    aom_lpf_horizontal_14 = Some(aom_lpf_horizontal_14_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_14 = Some(aom_lpf_horizontal_14_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_14_dual = Some(aom_lpf_horizontal_14_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_14_dual = Some(aom_lpf_horizontal_14_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_14_quad = Some(aom_lpf_horizontal_14_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_14_quad = Some(aom_lpf_horizontal_14_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_lpf_horizontal_14_quad = Some(aom_lpf_horizontal_14_quad_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4 = Some(aom_lpf_horizontal_4_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_4 = Some(aom_lpf_horizontal_4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4_dual = Some(aom_lpf_horizontal_4_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_4_dual = Some(aom_lpf_horizontal_4_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_4_quad = Some(aom_lpf_horizontal_4_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_4_quad = Some(aom_lpf_horizontal_4_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6 = Some(aom_lpf_horizontal_6_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_6 = Some(aom_lpf_horizontal_6_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6_dual = Some(aom_lpf_horizontal_6_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_6_dual = Some(aom_lpf_horizontal_6_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_6_quad = Some(aom_lpf_horizontal_6_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_6_quad = Some(aom_lpf_horizontal_6_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_lpf_horizontal_6_quad = Some(aom_lpf_horizontal_6_quad_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8 = Some(aom_lpf_horizontal_8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_8 = Some(aom_lpf_horizontal_8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8_dual = Some(aom_lpf_horizontal_8_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_8_dual = Some(aom_lpf_horizontal_8_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_horizontal_8_quad = Some(aom_lpf_horizontal_8_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_horizontal_8_quad = Some(aom_lpf_horizontal_8_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_lpf_horizontal_8_quad = Some(aom_lpf_horizontal_8_quad_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14 = Some(aom_lpf_vertical_14_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_14 = Some(aom_lpf_vertical_14_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14_dual = Some(aom_lpf_vertical_14_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_14_dual = Some(aom_lpf_vertical_14_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_14_quad = Some(aom_lpf_vertical_14_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_14_quad = Some(aom_lpf_vertical_14_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_lpf_vertical_14_quad = Some(aom_lpf_vertical_14_quad_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4 = Some(aom_lpf_vertical_4_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_4 = Some(aom_lpf_vertical_4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4_dual = Some(aom_lpf_vertical_4_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_4_dual = Some(aom_lpf_vertical_4_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_4_quad = Some(aom_lpf_vertical_4_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_4_quad = Some(aom_lpf_vertical_4_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6 = Some(aom_lpf_vertical_6_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_6 = Some(aom_lpf_vertical_6_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6_dual = Some(aom_lpf_vertical_6_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_6_dual = Some(aom_lpf_vertical_6_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_6_quad = Some(aom_lpf_vertical_6_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_6_quad = Some(aom_lpf_vertical_6_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8 = Some(aom_lpf_vertical_8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_8 = Some(aom_lpf_vertical_8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8_dual = Some(aom_lpf_vertical_8_dual_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_8_dual = Some(aom_lpf_vertical_8_dual_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_lpf_vertical_8_quad = Some(aom_lpf_vertical_8_quad_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_lpf_vertical_8_quad = Some(aom_lpf_vertical_8_quad_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_mse16x16 = Some(aom_mse16x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_mse16x16 = Some(aom_mse16x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_mse16x16 = Some(aom_mse16x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_mse16x8 = Some(aom_mse16x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_mse16x8 = Some(aom_mse16x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_mse8x16 = Some(aom_mse8x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_mse8x16 = Some(aom_mse8x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_mse8x8 = Some(aom_mse8x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_mse8x8 = Some(aom_mse8x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_mse_16xh_16bit = Some(aom_mse_16xh_16bit_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_mse_16xh_16bit = Some(aom_mse_16xh_16bit_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_mse_16xh_16bit = Some(aom_mse_16xh_16bit_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_mse_wxh_16bit = Some(aom_mse_wxh_16bit_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_mse_wxh_16bit = Some(aom_mse_wxh_16bit_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_mse_wxh_16bit = Some(aom_mse_wxh_16bit_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_mse_wxh_16bit_highbd = Some(aom_mse_wxh_16bit_highbd_c as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_mse_wxh_16bit_highbd = Some(aom_mse_wxh_16bit_highbd_sse2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_mse_wxh_16bit_highbd = Some(aom_mse_wxh_16bit_highbd_avx2 as unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *mut uint16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_paeth_predictor_16x16 = Some(aom_paeth_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_16x16 = Some(aom_paeth_predictor_16x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_16x16 = Some(aom_paeth_predictor_16x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x32 = Some(aom_paeth_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_16x32 = Some(aom_paeth_predictor_16x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_16x32 = Some(aom_paeth_predictor_16x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x4 = Some(aom_paeth_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_16x4 = Some(aom_paeth_predictor_16x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x64 = Some(aom_paeth_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_16x64 = Some(aom_paeth_predictor_16x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_16x64 = Some(aom_paeth_predictor_16x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_16x8 = Some(aom_paeth_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_16x8 = Some(aom_paeth_predictor_16x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_16x8 = Some(aom_paeth_predictor_16x8_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x16 = Some(aom_paeth_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_32x16 = Some(aom_paeth_predictor_32x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_32x16 = Some(aom_paeth_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x32 = Some(aom_paeth_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_32x32 = Some(aom_paeth_predictor_32x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_32x32 = Some(aom_paeth_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x64 = Some(aom_paeth_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_32x64 = Some(aom_paeth_predictor_32x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_32x64 = Some(aom_paeth_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_32x8 = Some(aom_paeth_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_32x8 = Some(aom_paeth_predictor_32x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x16 = Some(aom_paeth_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_4x16 = Some(aom_paeth_predictor_4x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x4 = Some(aom_paeth_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_4x4 = Some(aom_paeth_predictor_4x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_4x8 = Some(aom_paeth_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_4x8 = Some(aom_paeth_predictor_4x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x16 = Some(aom_paeth_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_64x16 = Some(aom_paeth_predictor_64x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_64x16 = Some(aom_paeth_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x32 = Some(aom_paeth_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_64x32 = Some(aom_paeth_predictor_64x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_64x32 = Some(aom_paeth_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_64x64 = Some(aom_paeth_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_64x64 = Some(aom_paeth_predictor_64x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_paeth_predictor_64x64 = Some(aom_paeth_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x16 = Some(aom_paeth_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_8x16 = Some(aom_paeth_predictor_8x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x32 = Some(aom_paeth_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_8x32 = Some(aom_paeth_predictor_8x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x4 = Some(aom_paeth_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_8x4 = Some(aom_paeth_predictor_8x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_paeth_predictor_8x8 = Some(aom_paeth_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_paeth_predictor_8x8 = Some(aom_paeth_predictor_8x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_quantize_b = Some(aom_quantize_b_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_quantize_b = Some(aom_quantize_b_sse2 as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_quantize_b = Some(aom_quantize_b_avx2 as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_quantize_b_32x32 = Some(aom_quantize_b_32x32_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_quantize_b_32x32 = Some(aom_quantize_b_32x32_avx2 as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_quantize_b_64x64 = Some(aom_quantize_b_64x64_c as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_quantize_b_64x64 = Some(aom_quantize_b_64x64_ssse3 as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_quantize_b_64x64 = Some(aom_quantize_b_64x64_avx2 as unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()) as Option<unsafe extern "C-unwind" fn(*const tran_low_t, intptr_t, *const int16_t, *const int16_t, *const int16_t, *const int16_t, *mut tran_low_t, *mut tran_low_t, *const int16_t, *mut uint16_t, *const int16_t, *const int16_t) -> ()>;
    }
    aom_smooth_h_predictor_16x16 = Some(aom_smooth_h_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_16x16 = Some(aom_smooth_h_predictor_16x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x32 = Some(aom_smooth_h_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_16x32 = Some(aom_smooth_h_predictor_16x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x4 = Some(aom_smooth_h_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_16x4 = Some(aom_smooth_h_predictor_16x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x64 = Some(aom_smooth_h_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_16x64 = Some(aom_smooth_h_predictor_16x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_16x8 = Some(aom_smooth_h_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_16x8 = Some(aom_smooth_h_predictor_16x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x16 = Some(aom_smooth_h_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_32x16 = Some(aom_smooth_h_predictor_32x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x32 = Some(aom_smooth_h_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_32x32 = Some(aom_smooth_h_predictor_32x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x64 = Some(aom_smooth_h_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_32x64 = Some(aom_smooth_h_predictor_32x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_32x8 = Some(aom_smooth_h_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_32x8 = Some(aom_smooth_h_predictor_32x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x16 = Some(aom_smooth_h_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_4x16 = Some(aom_smooth_h_predictor_4x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x4 = Some(aom_smooth_h_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_4x4 = Some(aom_smooth_h_predictor_4x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_4x8 = Some(aom_smooth_h_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_4x8 = Some(aom_smooth_h_predictor_4x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x16 = Some(aom_smooth_h_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_64x16 = Some(aom_smooth_h_predictor_64x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x32 = Some(aom_smooth_h_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_64x32 = Some(aom_smooth_h_predictor_64x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_64x64 = Some(aom_smooth_h_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_64x64 = Some(aom_smooth_h_predictor_64x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x16 = Some(aom_smooth_h_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_8x16 = Some(aom_smooth_h_predictor_8x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x32 = Some(aom_smooth_h_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_8x32 = Some(aom_smooth_h_predictor_8x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x4 = Some(aom_smooth_h_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_8x4 = Some(aom_smooth_h_predictor_8x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_h_predictor_8x8 = Some(aom_smooth_h_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_h_predictor_8x8 = Some(aom_smooth_h_predictor_8x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x16 = Some(aom_smooth_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_16x16 = Some(aom_smooth_predictor_16x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x32 = Some(aom_smooth_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_16x32 = Some(aom_smooth_predictor_16x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x4 = Some(aom_smooth_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_16x4 = Some(aom_smooth_predictor_16x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x64 = Some(aom_smooth_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_16x64 = Some(aom_smooth_predictor_16x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_16x8 = Some(aom_smooth_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_16x8 = Some(aom_smooth_predictor_16x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x16 = Some(aom_smooth_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_32x16 = Some(aom_smooth_predictor_32x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x32 = Some(aom_smooth_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_32x32 = Some(aom_smooth_predictor_32x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x64 = Some(aom_smooth_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_32x64 = Some(aom_smooth_predictor_32x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_32x8 = Some(aom_smooth_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_32x8 = Some(aom_smooth_predictor_32x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x16 = Some(aom_smooth_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_4x16 = Some(aom_smooth_predictor_4x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x4 = Some(aom_smooth_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_4x4 = Some(aom_smooth_predictor_4x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_4x8 = Some(aom_smooth_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_4x8 = Some(aom_smooth_predictor_4x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x16 = Some(aom_smooth_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_64x16 = Some(aom_smooth_predictor_64x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x32 = Some(aom_smooth_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_64x32 = Some(aom_smooth_predictor_64x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_64x64 = Some(aom_smooth_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_64x64 = Some(aom_smooth_predictor_64x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x16 = Some(aom_smooth_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_8x16 = Some(aom_smooth_predictor_8x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x32 = Some(aom_smooth_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_8x32 = Some(aom_smooth_predictor_8x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x4 = Some(aom_smooth_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_8x4 = Some(aom_smooth_predictor_8x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_predictor_8x8 = Some(aom_smooth_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_predictor_8x8 = Some(aom_smooth_predictor_8x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x16 = Some(aom_smooth_v_predictor_16x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_16x16 = Some(aom_smooth_v_predictor_16x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x32 = Some(aom_smooth_v_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_16x32 = Some(aom_smooth_v_predictor_16x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x4 = Some(aom_smooth_v_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_16x4 = Some(aom_smooth_v_predictor_16x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x64 = Some(aom_smooth_v_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_16x64 = Some(aom_smooth_v_predictor_16x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_16x8 = Some(aom_smooth_v_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_16x8 = Some(aom_smooth_v_predictor_16x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x16 = Some(aom_smooth_v_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_32x16 = Some(aom_smooth_v_predictor_32x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x32 = Some(aom_smooth_v_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_32x32 = Some(aom_smooth_v_predictor_32x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x64 = Some(aom_smooth_v_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_32x64 = Some(aom_smooth_v_predictor_32x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_32x8 = Some(aom_smooth_v_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_32x8 = Some(aom_smooth_v_predictor_32x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x16 = Some(aom_smooth_v_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_4x16 = Some(aom_smooth_v_predictor_4x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x4 = Some(aom_smooth_v_predictor_4x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_4x4 = Some(aom_smooth_v_predictor_4x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_4x8 = Some(aom_smooth_v_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_4x8 = Some(aom_smooth_v_predictor_4x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x16 = Some(aom_smooth_v_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_64x16 = Some(aom_smooth_v_predictor_64x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x32 = Some(aom_smooth_v_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_64x32 = Some(aom_smooth_v_predictor_64x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_64x64 = Some(aom_smooth_v_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_64x64 = Some(aom_smooth_v_predictor_64x64_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x16 = Some(aom_smooth_v_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_8x16 = Some(aom_smooth_v_predictor_8x16_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x32 = Some(aom_smooth_v_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_8x32 = Some(aom_smooth_v_predictor_8x32_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x4 = Some(aom_smooth_v_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_8x4 = Some(aom_smooth_v_predictor_8x4_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_smooth_v_predictor_8x8 = Some(aom_smooth_v_predictor_8x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSSE3 != 0 {
        aom_smooth_v_predictor_8x8 = Some(aom_smooth_v_predictor_8x8_ssse3 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_sse = Some(aom_sse_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    if flags & HAS_SSE4_1 != 0 {
        aom_sse = Some(aom_sse_sse4_1 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_sse = Some(aom_sse_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
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
    aom_sum_squares_2d_i16 = Some(aom_sum_squares_2d_i16_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_sum_squares_2d_i16 = Some(aom_sum_squares_2d_i16_sse2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_sum_squares_2d_i16 = Some(aom_sum_squares_2d_i16_avx2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_sum_squares_i16 = Some(aom_sum_squares_i16_c as unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_sum_squares_i16 = Some(aom_sum_squares_i16_sse2 as unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, uint32_t) -> uint64_t>;
    }
    aom_sum_sse_2d_i16 = Some(aom_sum_sse_2d_i16_c as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_sum_sse_2d_i16 = Some(aom_sum_sse_2d_i16_sse2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_sum_sse_2d_i16 = Some(aom_sum_sse_2d_i16_avx2 as unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*const int16_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_int) -> uint64_t>;
    }
    aom_v_predictor_16x32 = Some(aom_v_predictor_16x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_16x32 = Some(aom_v_predictor_16x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x4 = Some(aom_v_predictor_16x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_16x4 = Some(aom_v_predictor_16x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x64 = Some(aom_v_predictor_16x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_16x64 = Some(aom_v_predictor_16x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_16x8 = Some(aom_v_predictor_16x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_16x8 = Some(aom_v_predictor_16x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x16 = Some(aom_v_predictor_32x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_32x16 = Some(aom_v_predictor_32x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_32x16 = Some(aom_v_predictor_32x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x32 = Some(aom_v_predictor_32x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_32x32 = Some(aom_v_predictor_32x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x64 = Some(aom_v_predictor_32x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_32x64 = Some(aom_v_predictor_32x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_32x64 = Some(aom_v_predictor_32x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_32x8 = Some(aom_v_predictor_32x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_32x8 = Some(aom_v_predictor_32x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_4x16 = Some(aom_v_predictor_4x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_4x16 = Some(aom_v_predictor_4x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_4x8 = Some(aom_v_predictor_4x8_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_4x8 = Some(aom_v_predictor_4x8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x16 = Some(aom_v_predictor_64x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_64x16 = Some(aom_v_predictor_64x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_64x16 = Some(aom_v_predictor_64x16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x32 = Some(aom_v_predictor_64x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_64x32 = Some(aom_v_predictor_64x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_64x32 = Some(aom_v_predictor_64x32_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_64x64 = Some(aom_v_predictor_64x64_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_64x64 = Some(aom_v_predictor_64x64_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_v_predictor_64x64 = Some(aom_v_predictor_64x64_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x16 = Some(aom_v_predictor_8x16_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_8x16 = Some(aom_v_predictor_8x16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x32 = Some(aom_v_predictor_8x32_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_8x32 = Some(aom_v_predictor_8x32_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_v_predictor_8x4 = Some(aom_v_predictor_8x4_c as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    if flags & HAS_SSE2 != 0 {
        aom_v_predictor_8x4 = Some(aom_v_predictor_8x4_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
    }
    aom_var_2d_u16 = Some(aom_var_2d_u16_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_var_2d_u16 = Some(aom_var_2d_u16_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_var_2d_u16 = Some(aom_var_2d_u16_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_var_2d_u8 = Some(aom_var_2d_u8_c as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    if flags & HAS_SSE2 != 0 {
        aom_var_2d_u8 = Some(aom_var_2d_u8_sse2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_var_2d_u8 = Some(aom_var_2d_u8_avx2 as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    }
    aom_variance128x128 = Some(aom_variance128x128_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance128x128 = Some(aom_variance128x128_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance128x128 = Some(aom_variance128x128_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance128x64 = Some(aom_variance128x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance128x64 = Some(aom_variance128x64_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance128x64 = Some(aom_variance128x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance16x16 = Some(aom_variance16x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance16x16 = Some(aom_variance16x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance16x16 = Some(aom_variance16x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance16x32 = Some(aom_variance16x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance16x32 = Some(aom_variance16x32_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance16x32 = Some(aom_variance16x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance16x4 = Some(aom_variance16x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance16x4 = Some(aom_variance16x4_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance16x4 = Some(aom_variance16x4_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance16x64 = Some(aom_variance16x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance16x64 = Some(aom_variance16x64_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance16x64 = Some(aom_variance16x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance16x8 = Some(aom_variance16x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance16x8 = Some(aom_variance16x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance16x8 = Some(aom_variance16x8_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance32x16 = Some(aom_variance32x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance32x16 = Some(aom_variance32x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance32x16 = Some(aom_variance32x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance32x32 = Some(aom_variance32x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance32x32 = Some(aom_variance32x32_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance32x32 = Some(aom_variance32x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance32x64 = Some(aom_variance32x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance32x64 = Some(aom_variance32x64_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance32x64 = Some(aom_variance32x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance32x8 = Some(aom_variance32x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance32x8 = Some(aom_variance32x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance32x8 = Some(aom_variance32x8_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance4x16 = Some(aom_variance4x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance4x16 = Some(aom_variance4x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance4x4 = Some(aom_variance4x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance4x4 = Some(aom_variance4x4_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance4x8 = Some(aom_variance4x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance4x8 = Some(aom_variance4x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance64x128 = Some(aom_variance64x128_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance64x128 = Some(aom_variance64x128_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance64x128 = Some(aom_variance64x128_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance64x16 = Some(aom_variance64x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance64x16 = Some(aom_variance64x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance64x16 = Some(aom_variance64x16_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance64x32 = Some(aom_variance64x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance64x32 = Some(aom_variance64x32_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance64x32 = Some(aom_variance64x32_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance64x64 = Some(aom_variance64x64_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance64x64 = Some(aom_variance64x64_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    if flags & HAS_AVX2 != 0 {
        aom_variance64x64 = Some(aom_variance64x64_avx2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance8x16 = Some(aom_variance8x16_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance8x16 = Some(aom_variance8x16_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance8x32 = Some(aom_variance8x32_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance8x32 = Some(aom_variance8x32_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance8x4 = Some(aom_variance8x4_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance8x4 = Some(aom_variance8x4_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    }
    aom_variance8x8 = Some(aom_variance8x8_c as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
    if flags & HAS_SSE2 != 0 {
        aom_variance8x8 = Some(aom_variance8x8_sse2 as unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint) as Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, *mut core::ffi::c_uint) -> core::ffi::c_uint>;
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
