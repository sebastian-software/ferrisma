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

// Mechanically translated from upstream/aom/av1/common/av1_rtcd.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use core::arch::asm;
extern "C-unwind" {
    fn av1_block_error_c(
        coeff: *const tran_low_t,
        dqcoeff: *const tran_low_t,
        block_size: intptr_t,
        ssz: *mut int64_t,
    ) -> int64_t;
    fn av1_block_error_avx2(
        coeff: *const tran_low_t,
        dqcoeff: *const tran_low_t,
        block_size: intptr_t,
        ssz: *mut int64_t,
    ) -> int64_t;
    fn av1_block_error_lp_c(
        coeff: *const int16_t,
        dqcoeff: *const int16_t,
        block_size: intptr_t,
    ) -> int64_t;
    fn av1_block_error_lp_avx2(
        coeff: *const int16_t,
        dqcoeff: *const int16_t,
        block_size: intptr_t,
    ) -> int64_t;
    fn av1_filter_intra_edge_c(
        p: *mut uint8_t,
        sz: core::ffi::c_int,
        strength: core::ffi::c_int,
    );
    fn av1_filter_intra_edge_sse4_1(
        p: *mut uint8_t,
        sz: core::ffi::c_int,
        strength: core::ffi::c_int,
    );
    fn av1_filter_intra_predictor_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        tx_size: TX_SIZE,
        above: *const uint8_t,
        left: *const uint8_t,
        mode: core::ffi::c_int,
    );
    fn av1_filter_intra_predictor_sse4_1(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        tx_size: TX_SIZE,
        above: *const uint8_t,
        left: *const uint8_t,
        mode: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x16_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x16_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x16_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x32_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x32_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x4_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x4_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x64_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x64_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x8_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x8_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_16x8_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x16_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x16_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x32_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x32_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x32_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x64_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x64_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x8_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_32x8_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x16_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x16_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x4_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x4_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x8_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_4x8_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x16_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x16_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x32_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x32_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x64_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x64_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_64x64_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x16_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x16_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x16_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x32_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x32_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x4_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x4_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x8_c(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x8_sse4_1(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_fwd_txfm2d_8x8_avx2(
        input: *const int16_t,
        output: *mut int32_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_block_error_c(
        coeff: *const tran_low_t,
        dqcoeff: *const tran_low_t,
        block_size: intptr_t,
        ssz: *mut int64_t,
        bd: core::ffi::c_int,
    ) -> int64_t;
    fn av1_highbd_block_error_avx2(
        coeff: *const tran_low_t,
        dqcoeff: *const tran_low_t,
        block_size: intptr_t,
        ssz: *mut int64_t,
        bd: core::ffi::c_int,
    ) -> int64_t;
    fn av1_highbd_dr_prediction_z1_c(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_above: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_dr_prediction_z1_avx2(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_above: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_dr_prediction_z2_c(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_above: core::ffi::c_int,
        upsample_left: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_dr_prediction_z2_avx2(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_above: core::ffi::c_int,
        upsample_left: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_dr_prediction_z3_c(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_left: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_dr_prediction_z3_avx2(
        dst: *mut uint16_t,
        stride: ptrdiff_t,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        above: *const uint16_t,
        left: *const uint16_t,
        upsample_left: core::ffi::c_int,
        dx: core::ffi::c_int,
        dy: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_filter_intra_edge_c(
        p: *mut uint16_t,
        sz: core::ffi::c_int,
        strength: core::ffi::c_int,
    );
    fn av1_highbd_filter_intra_edge_sse4_1(
        p: *mut uint16_t,
        sz: core::ffi::c_int,
        strength: core::ffi::c_int,
    );
    fn av1_highbd_inv_txfm_add_c(
        input: *const tran_low_t,
        dest: *mut uint8_t,
        stride: core::ffi::c_int,
        txfm_param: *const TxfmParam,
    );
    fn av1_highbd_inv_txfm_add_sse4_1(
        input: *const tran_low_t,
        dest: *mut uint8_t,
        stride: core::ffi::c_int,
        txfm_param: *const TxfmParam,
    );
    fn av1_highbd_inv_txfm_add_avx2(
        input: *const tran_low_t,
        dest: *mut uint8_t,
        stride: core::ffi::c_int,
        txfm_param: *const TxfmParam,
    );
    fn av1_highbd_quantize_fp_c(
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
        log_scale: core::ffi::c_int,
    );
    fn av1_highbd_quantize_fp_avx2(
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
        log_scale: core::ffi::c_int,
    );
    fn av1_highbd_upsample_intra_edge_c(
        p: *mut uint16_t,
        sz: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_upsample_intra_edge_sse4_1(
        p: *mut uint16_t,
        sz: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_upsample_intra_edge_c(p: *mut uint8_t, sz: core::ffi::c_int);
    fn av1_upsample_intra_edge_sse4_1(p: *mut uint8_t, sz: core::ffi::c_int);
    fn cdef_copy_rect8_16bit_to_16bit_c(
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint16_t,
        sstride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn cdef_copy_rect8_16bit_to_16bit_avx2(
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint16_t,
        sstride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn cdef_copy_rect8_8bit_to_16bit_c(
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint8_t,
        sstride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn cdef_copy_rect8_8bit_to_16bit_avx2(
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint8_t,
        sstride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn cdef_filter_16_0_c(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_0_avx2(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_1_c(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_1_avx2(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_2_c(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_2_avx2(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_3_c(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_16_3_avx2(
        dst16: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_0_c(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_0_avx2(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_1_c(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_1_avx2(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_2_c(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_2_avx2(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_3_c(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_filter_8_3_avx2(
        dst8: *mut core::ffi::c_void,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        pri_strength: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        dir: core::ffi::c_int,
        pri_damping: core::ffi::c_int,
        sec_damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
        block_width: core::ffi::c_int,
        block_height: core::ffi::c_int,
    );
    fn cdef_find_dir_c(
        img: *const uint16_t,
        stride: core::ffi::c_int,
        var: *mut int32_t,
        coeff_shift: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn cdef_find_dir_avx2(
        img: *const uint16_t,
        stride: core::ffi::c_int,
        var: *mut int32_t,
        coeff_shift: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn cdef_find_dir_dual_c(
        img1: *const uint16_t,
        img2: *const uint16_t,
        stride: core::ffi::c_int,
        var1: *mut int32_t,
        var2: *mut int32_t,
        coeff_shift: core::ffi::c_int,
        out1: *mut core::ffi::c_int,
        out2: *mut core::ffi::c_int,
    );
    fn cdef_find_dir_dual_avx2(
        img1: *const uint16_t,
        img2: *const uint16_t,
        stride: core::ffi::c_int,
        var1: *mut int32_t,
        var2: *mut int32_t,
        coeff_shift: core::ffi::c_int,
        out1: *mut core::ffi::c_int,
        out2: *mut core::ffi::c_int,
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type tran_low_t = int32_t;
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
pub type TxSetType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txfm_param {
    pub tx_type: TX_TYPE,
    pub tx_size: TX_SIZE,
    pub lossless: core::ffi::c_int,
    pub bd: core::ffi::c_int,
    pub is_hbd: core::ffi::c_int,
    pub tx_set_type: TxSetType,
    pub eob: core::ffi::c_int,
}
pub type TxfmParam = txfm_param;
pub type pthread_once_t = libc::pthread_once_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}
pub const PTHREAD_ONCE_INIT: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut av1_block_error: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        *const tran_low_t,
        intptr_t,
        *mut int64_t,
    ) -> int64_t,
> = None;
#[no_mangle]
pub static mut av1_block_error_lp: Option<
    unsafe extern "C-unwind" fn(*const int16_t, *const int16_t, intptr_t) -> int64_t,
> = None;
#[no_mangle]
pub static mut av1_filter_intra_edge: Option<
    unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut av1_filter_intra_predictor: Option<
    unsafe extern "C-unwind" fn(
        *mut uint8_t,
        ptrdiff_t,
        TX_SIZE,
        *const uint8_t,
        *const uint8_t,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_16x16: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_16x32: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_16x4: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_16x64: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_16x8: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_32x16: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_32x32: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_32x64: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_32x8: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_4x16: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_4x4: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_4x8: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_64x16: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_64x32: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_64x64: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_8x16: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_8x32: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_8x4: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_fwd_txfm2d_8x8: Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut int32_t,
        core::ffi::c_int,
        TX_TYPE,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_block_error: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        *const tran_low_t,
        intptr_t,
        *mut int64_t,
        core::ffi::c_int,
    ) -> int64_t,
> = None;
#[no_mangle]
pub static mut av1_highbd_dr_prediction_z1: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_dr_prediction_z2: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_dr_prediction_z3: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_filter_intra_edge: Option<
    unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_inv_txfm_add: Option<
    unsafe extern "C-unwind" fn(
        *const tran_low_t,
        *mut uint8_t,
        core::ffi::c_int,
        *const TxfmParam,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_quantize_fp: Option<
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
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut av1_highbd_upsample_intra_edge: Option<
    unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut av1_upsample_intra_edge: Option<
    unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut cdef_copy_rect8_16bit_to_16bit: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_copy_rect8_8bit_to_16bit: Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_16_0: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_16_1: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_16_2: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_16_3: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_8_0: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_8_1: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_8_2: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_filter_8_3: Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut cdef_find_dir: Option<
    unsafe extern "C-unwind" fn(
        *const uint16_t,
        core::ffi::c_int,
        *mut int32_t,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut cdef_find_dir_dual: Option<
    unsafe extern "C-unwind" fn(
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
        *mut int32_t,
        *mut int32_t,
        core::ffi::c_int,
        *mut core::ffi::c_int,
        *mut core::ffi::c_int,
    ) -> (),
> = None;
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = x86_simd_caps();
    av1_block_error = Some(
        av1_block_error_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *const tran_low_t,
                intptr_t,
                *mut int64_t,
            ) -> int64_t,
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *const tran_low_t,
                intptr_t,
                *mut int64_t,
            ) -> int64_t,
        >;
    if flags & HAS_AVX2 != 0 {
        av1_block_error = Some(
            av1_block_error_avx2
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *const tran_low_t,
                    intptr_t,
                    *mut int64_t,
                ) -> int64_t,
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *const tran_low_t,
                    intptr_t,
                    *mut int64_t,
                ) -> int64_t,
            >;
    }
    av1_block_error_lp = Some(
        av1_block_error_lp_c
            as unsafe extern "C-unwind" fn(*const int16_t, *const int16_t, intptr_t) -> int64_t,
    )
        as Option<
            unsafe extern "C-unwind" fn(*const int16_t, *const int16_t, intptr_t) -> int64_t,
        >;
    if flags & HAS_AVX2 != 0 {
        av1_block_error_lp = Some(
            av1_block_error_lp_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *const int16_t,
                    intptr_t,
                ) -> int64_t,
        )
            as Option<
                unsafe extern "C-unwind" fn(*const int16_t, *const int16_t, intptr_t) -> int64_t,
            >;
    }
    av1_filter_intra_edge = Some(
        av1_filter_intra_edge_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_filter_intra_edge = Some(
            av1_filter_intra_edge_sse4_1
                as unsafe extern "C-unwind" fn(
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_filter_intra_predictor = Some(
        av1_filter_intra_predictor_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                TX_SIZE,
                *const uint8_t,
                *const uint8_t,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                TX_SIZE,
                *const uint8_t,
                *const uint8_t,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_filter_intra_predictor = Some(
            av1_filter_intra_predictor_sse4_1
                as unsafe extern "C-unwind" fn(
                    *mut uint8_t,
                    ptrdiff_t,
                    TX_SIZE,
                    *const uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint8_t,
                    ptrdiff_t,
                    TX_SIZE,
                    *const uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_16x16 = Some(
        av1_fwd_txfm2d_16x16_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_16x16 = Some(
            av1_fwd_txfm2d_16x16_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_16x16 = Some(
            av1_fwd_txfm2d_16x16_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_16x32 = Some(
        av1_fwd_txfm2d_16x32_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_16x32 = Some(
            av1_fwd_txfm2d_16x32_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_16x4 = Some(
        av1_fwd_txfm2d_16x4_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_16x4 = Some(
            av1_fwd_txfm2d_16x4_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_16x64 = Some(
        av1_fwd_txfm2d_16x64_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_16x64 = Some(
            av1_fwd_txfm2d_16x64_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_16x8 = Some(
        av1_fwd_txfm2d_16x8_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_16x8 = Some(
            av1_fwd_txfm2d_16x8_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_16x8 = Some(
            av1_fwd_txfm2d_16x8_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_32x16 = Some(
        av1_fwd_txfm2d_32x16_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_32x16 = Some(
            av1_fwd_txfm2d_32x16_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_32x32 = Some(
        av1_fwd_txfm2d_32x32_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_32x32 = Some(
            av1_fwd_txfm2d_32x32_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_32x32 = Some(
            av1_fwd_txfm2d_32x32_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_32x64 = Some(
        av1_fwd_txfm2d_32x64_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_32x64 = Some(
            av1_fwd_txfm2d_32x64_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_32x8 = Some(
        av1_fwd_txfm2d_32x8_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_32x8 = Some(
            av1_fwd_txfm2d_32x8_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_4x16 = Some(
        av1_fwd_txfm2d_4x16_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_4x16 = Some(
            av1_fwd_txfm2d_4x16_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_4x4 = Some(
        av1_fwd_txfm2d_4x4_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_4x4 = Some(
            av1_fwd_txfm2d_4x4_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_4x8 = Some(
        av1_fwd_txfm2d_4x8_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_4x8 = Some(
            av1_fwd_txfm2d_4x8_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_64x16 = Some(
        av1_fwd_txfm2d_64x16_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_64x16 = Some(
            av1_fwd_txfm2d_64x16_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_64x32 = Some(
        av1_fwd_txfm2d_64x32_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_64x32 = Some(
            av1_fwd_txfm2d_64x32_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_64x64 = Some(
        av1_fwd_txfm2d_64x64_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_64x64 = Some(
            av1_fwd_txfm2d_64x64_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_64x64 = Some(
            av1_fwd_txfm2d_64x64_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_8x16 = Some(
        av1_fwd_txfm2d_8x16_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_8x16 = Some(
            av1_fwd_txfm2d_8x16_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_8x16 = Some(
            av1_fwd_txfm2d_8x16_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_8x32 = Some(
        av1_fwd_txfm2d_8x32_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_8x32 = Some(
            av1_fwd_txfm2d_8x32_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_8x4 = Some(
        av1_fwd_txfm2d_8x4_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_8x4 = Some(
            av1_fwd_txfm2d_8x4_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_fwd_txfm2d_8x8 = Some(
        av1_fwd_txfm2d_8x8_c
            as unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const int16_t,
                *mut int32_t,
                core::ffi::c_int,
                TX_TYPE,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_fwd_txfm2d_8x8 = Some(
            av1_fwd_txfm2d_8x8_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_fwd_txfm2d_8x8 = Some(
            av1_fwd_txfm2d_8x8_avx2
                as unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const int16_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    TX_TYPE,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_block_error = Some(
        av1_highbd_block_error_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *const tran_low_t,
                intptr_t,
                *mut int64_t,
                core::ffi::c_int,
            ) -> int64_t,
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *const tran_low_t,
                intptr_t,
                *mut int64_t,
                core::ffi::c_int,
            ) -> int64_t,
        >;
    if flags & HAS_AVX2 != 0 {
        av1_highbd_block_error = Some(
            av1_highbd_block_error_avx2
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *const tran_low_t,
                    intptr_t,
                    *mut int64_t,
                    core::ffi::c_int,
                ) -> int64_t,
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *const tran_low_t,
                    intptr_t,
                    *mut int64_t,
                    core::ffi::c_int,
                ) -> int64_t,
            >;
    }
    av1_highbd_dr_prediction_z1 = Some(
        av1_highbd_dr_prediction_z1_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        av1_highbd_dr_prediction_z1 = Some(
            av1_highbd_dr_prediction_z1_avx2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_dr_prediction_z2 = Some(
        av1_highbd_dr_prediction_z2_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        av1_highbd_dr_prediction_z2 = Some(
            av1_highbd_dr_prediction_z2_avx2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_dr_prediction_z3 = Some(
        av1_highbd_dr_prediction_z3_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        av1_highbd_dr_prediction_z3 = Some(
            av1_highbd_dr_prediction_z3_avx2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    ptrdiff_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_filter_intra_edge = Some(
        av1_highbd_filter_intra_edge_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_highbd_filter_intra_edge = Some(
            av1_highbd_filter_intra_edge_sse4_1
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_inv_txfm_add = Some(
        av1_highbd_inv_txfm_add_c
            as unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *mut uint8_t,
                core::ffi::c_int,
                *const TxfmParam,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const tran_low_t,
                *mut uint8_t,
                core::ffi::c_int,
                *const TxfmParam,
            ) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_highbd_inv_txfm_add = Some(
            av1_highbd_inv_txfm_add_sse4_1
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *const TxfmParam,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *const TxfmParam,
                ) -> (),
            >;
    }
    if flags & HAS_AVX2 != 0 {
        av1_highbd_inv_txfm_add = Some(
            av1_highbd_inv_txfm_add_avx2
                as unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *const TxfmParam,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const tran_low_t,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *const TxfmParam,
                ) -> (),
            >;
    }
    av1_highbd_quantize_fp = Some(
        av1_highbd_quantize_fp_c
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
                core::ffi::c_int,
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
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        av1_highbd_quantize_fp = Some(
            av1_highbd_quantize_fp_avx2
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
                    core::ffi::c_int,
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
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_highbd_upsample_intra_edge = Some(
        av1_highbd_upsample_intra_edge_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
        >;
    if flags & HAS_SSE4_1 != 0 {
        av1_highbd_upsample_intra_edge = Some(
            av1_highbd_upsample_intra_edge_sse4_1
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    av1_upsample_intra_edge = Some(
        av1_upsample_intra_edge_c
            as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> ()>;
    if flags & HAS_SSE4_1 != 0 {
        av1_upsample_intra_edge = Some(
            av1_upsample_intra_edge_sse4_1
                as unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> (),
        ) as Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> ()>;
    }
    cdef_copy_rect8_16bit_to_16bit = Some(
        cdef_copy_rect8_16bit_to_16bit_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_copy_rect8_16bit_to_16bit = Some(
            cdef_copy_rect8_16bit_to_16bit_avx2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_copy_rect8_8bit_to_16bit = Some(
        cdef_copy_rect8_8bit_to_16bit_c
            as unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut uint16_t,
                core::ffi::c_int,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_copy_rect8_8bit_to_16bit = Some(
            cdef_copy_rect8_8bit_to_16bit_avx2
                as unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut uint16_t,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_16_0 = Some(
        cdef_filter_16_0_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_16_0 = Some(
            cdef_filter_16_0_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_16_1 = Some(
        cdef_filter_16_1_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_16_1 = Some(
            cdef_filter_16_1_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_16_2 = Some(
        cdef_filter_16_2_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_16_2 = Some(
            cdef_filter_16_2_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_16_3 = Some(
        cdef_filter_16_3_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_16_3 = Some(
            cdef_filter_16_3_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_8_0 = Some(
        cdef_filter_8_0_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_8_0 = Some(
            cdef_filter_8_0_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_8_1 = Some(
        cdef_filter_8_1_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_8_1 = Some(
            cdef_filter_8_1_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_8_2 = Some(
        cdef_filter_8_2_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_8_2 = Some(
            cdef_filter_8_2_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_filter_8_3 = Some(
        cdef_filter_8_3_c
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                core::ffi::c_int,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_filter_8_3 = Some(
            cdef_filter_8_3_avx2
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    cdef_find_dir = Some(
        cdef_find_dir_c
            as unsafe extern "C-unwind" fn(
                *const uint16_t,
                core::ffi::c_int,
                *mut int32_t,
                core::ffi::c_int,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const uint16_t,
                core::ffi::c_int,
                *mut int32_t,
                core::ffi::c_int,
            ) -> core::ffi::c_int,
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_find_dir = Some(
            cdef_find_dir_avx2
                as unsafe extern "C-unwind" fn(
                    *const uint16_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const uint16_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
            >;
    }
    cdef_find_dir_dual = Some(
        cdef_find_dir_dual_c
            as unsafe extern "C-unwind" fn(
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                *mut int32_t,
                *mut int32_t,
                core::ffi::c_int,
                *mut core::ffi::c_int,
                *mut core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                *mut int32_t,
                *mut int32_t,
                core::ffi::c_int,
                *mut core::ffi::c_int,
                *mut core::ffi::c_int,
            ) -> (),
        >;
    if flags & HAS_AVX2 != 0 {
        cdef_find_dir_dual = Some(
            cdef_find_dir_dual_avx2
                as unsafe extern "C-unwind" fn(
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut core::ffi::c_int,
                    *mut core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *const uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut core::ffi::c_int,
                    *mut core::ffi::c_int,
                ) -> (),
            >;
    }
}
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
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func);
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
