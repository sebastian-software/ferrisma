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
extern "C-unwind" {
    fn av1_block_error_c(
        coeff: *const tran_low_t,
        dqcoeff: *const tran_low_t,
        block_size: intptr_t,
        ssz: *mut int64_t,
    ) -> int64_t;
    fn av1_block_error_neon(
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
    fn av1_block_error_lp_neon(
        coeff: *const int16_t,
        dqcoeff: *const int16_t,
        block_size: intptr_t,
    ) -> int64_t;
    fn av1_filter_intra_edge_c(
        p: *mut uint8_t,
        sz: core::ffi::c_int,
        strength: core::ffi::c_int,
    );
    fn av1_filter_intra_edge_neon(
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
    fn av1_filter_intra_predictor_neon(
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
    fn av1_fwd_txfm2d_16x16_neon(
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
    fn av1_fwd_txfm2d_16x32_neon(
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
    fn av1_fwd_txfm2d_16x4_neon(
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
    fn av1_fwd_txfm2d_16x64_neon(
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
    fn av1_fwd_txfm2d_16x8_neon(
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
    fn av1_fwd_txfm2d_32x16_neon(
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
    fn av1_fwd_txfm2d_32x32_neon(
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
    fn av1_fwd_txfm2d_32x64_neon(
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
    fn av1_fwd_txfm2d_32x8_neon(
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
    fn av1_fwd_txfm2d_4x16_neon(
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
    fn av1_fwd_txfm2d_4x4_neon(
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
    fn av1_fwd_txfm2d_4x8_neon(
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
    fn av1_fwd_txfm2d_64x16_neon(
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
    fn av1_fwd_txfm2d_64x32_neon(
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
    fn av1_fwd_txfm2d_64x64_neon(
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
    fn av1_fwd_txfm2d_8x16_neon(
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
    fn av1_fwd_txfm2d_8x32_neon(
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
    fn av1_fwd_txfm2d_8x4_neon(
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
    fn av1_fwd_txfm2d_8x8_neon(
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
    fn av1_highbd_block_error_neon(
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
    fn av1_highbd_dr_prediction_z1_neon(
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
    fn av1_highbd_dr_prediction_z2_neon(
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
    fn av1_highbd_dr_prediction_z3_neon(
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
    fn av1_highbd_filter_intra_edge_neon(
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
    fn av1_highbd_inv_txfm_add_neon(
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
    fn av1_highbd_quantize_fp_neon(
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
    fn av1_highbd_upsample_intra_edge_neon(
        p: *mut uint16_t,
        sz: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_upsample_intra_edge_c(p: *mut uint8_t, sz: core::ffi::c_int);
    fn av1_upsample_intra_edge_neon(p: *mut uint8_t, sz: core::ffi::c_int);
    fn cdef_copy_rect8_16bit_to_16bit_c(
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint16_t,
        sstride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn cdef_copy_rect8_16bit_to_16bit_neon(
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
    fn cdef_copy_rect8_8bit_to_16bit_neon(
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
    fn cdef_filter_16_0_neon(
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
    fn cdef_filter_16_1_neon(
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
    fn cdef_filter_16_2_neon(
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
    fn cdef_filter_16_3_neon(
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
    fn cdef_filter_8_0_neon(
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
    fn cdef_filter_8_1_neon(
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
    fn cdef_filter_8_2_neon(
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
    fn cdef_filter_8_3_neon(
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
    fn cdef_find_dir_neon(
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
    fn cdef_find_dir_dual_neon(
        img1: *const uint16_t,
        img2: *const uint16_t,
        stride: core::ffi::c_int,
        var1: *mut int32_t,
        var2: *mut int32_t,
        coeff_shift: core::ffi::c_int,
        out1: *mut core::ffi::c_int,
        out2: *mut core::ffi::c_int,
    );
    fn aom_arm_cpu_caps() -> core::ffi::c_int;
}
pub type ptrdiff_t = isize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
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
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
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
pub const HAS_NEON: core::ffi::c_int = (1 as core::ffi::c_int) << 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = aom_arm_cpu_caps();
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
    if flags & HAS_NEON != 0 {
        av1_block_error = Some(
            av1_block_error_neon
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
    if flags & HAS_NEON != 0 {
        av1_block_error_lp = Some(
            av1_block_error_lp_neon
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
    if flags & HAS_NEON != 0 {
        av1_filter_intra_edge = Some(
            av1_filter_intra_edge_neon
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
    if flags & HAS_NEON != 0 {
        av1_filter_intra_predictor = Some(
            av1_filter_intra_predictor_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_16x16 = Some(
            av1_fwd_txfm2d_16x16_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_16x32 = Some(
            av1_fwd_txfm2d_16x32_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_16x4 = Some(
            av1_fwd_txfm2d_16x4_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_16x64 = Some(
            av1_fwd_txfm2d_16x64_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_16x8 = Some(
            av1_fwd_txfm2d_16x8_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_32x16 = Some(
            av1_fwd_txfm2d_32x16_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_32x32 = Some(
            av1_fwd_txfm2d_32x32_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_32x64 = Some(
            av1_fwd_txfm2d_32x64_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_32x8 = Some(
            av1_fwd_txfm2d_32x8_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_4x16 = Some(
            av1_fwd_txfm2d_4x16_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_4x4 = Some(
            av1_fwd_txfm2d_4x4_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_4x8 = Some(
            av1_fwd_txfm2d_4x8_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_64x16 = Some(
            av1_fwd_txfm2d_64x16_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_64x32 = Some(
            av1_fwd_txfm2d_64x32_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_64x64 = Some(
            av1_fwd_txfm2d_64x64_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_8x16 = Some(
            av1_fwd_txfm2d_8x16_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_8x32 = Some(
            av1_fwd_txfm2d_8x32_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_8x4 = Some(
            av1_fwd_txfm2d_8x4_neon
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
    if flags & HAS_NEON != 0 {
        av1_fwd_txfm2d_8x8 = Some(
            av1_fwd_txfm2d_8x8_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_block_error = Some(
            av1_highbd_block_error_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_dr_prediction_z1 = Some(
            av1_highbd_dr_prediction_z1_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_dr_prediction_z2 = Some(
            av1_highbd_dr_prediction_z2_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_dr_prediction_z3 = Some(
            av1_highbd_dr_prediction_z3_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_filter_intra_edge = Some(
            av1_highbd_filter_intra_edge_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_inv_txfm_add = Some(
            av1_highbd_inv_txfm_add_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_quantize_fp = Some(
            av1_highbd_quantize_fp_neon
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
    if flags & HAS_NEON != 0 {
        av1_highbd_upsample_intra_edge = Some(
            av1_highbd_upsample_intra_edge_neon
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
    if flags & HAS_NEON != 0 {
        av1_upsample_intra_edge = Some(
            av1_upsample_intra_edge_neon
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
    if flags & HAS_NEON != 0 {
        cdef_copy_rect8_16bit_to_16bit = Some(
            cdef_copy_rect8_16bit_to_16bit_neon
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
    if flags & HAS_NEON != 0 {
        cdef_copy_rect8_8bit_to_16bit = Some(
            cdef_copy_rect8_8bit_to_16bit_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_16_0 = Some(
            cdef_filter_16_0_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_16_1 = Some(
            cdef_filter_16_1_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_16_2 = Some(
            cdef_filter_16_2_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_16_3 = Some(
            cdef_filter_16_3_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_8_0 = Some(
            cdef_filter_8_0_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_8_1 = Some(
            cdef_filter_8_1_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_8_2 = Some(
            cdef_filter_8_2_neon
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
    if flags & HAS_NEON != 0 {
        cdef_filter_8_3 = Some(
            cdef_filter_8_3_neon
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
    if flags & HAS_NEON != 0 {
        cdef_find_dir = Some(
            cdef_find_dir_neon
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
    if flags & HAS_NEON != 0 {
        cdef_find_dir_dual = Some(
            cdef_find_dir_dual_neon
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
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func as Option<unsafe extern "C-unwind" fn() -> ()>);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
