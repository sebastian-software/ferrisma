/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/encoder/tx_search.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type VmafModel;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_dc_quant_QTX(
        qindex: core::ffi::c_int,
        delta: core::ffi::c_int,
        bit_depth: aom_bit_depth_t,
    ) -> int16_t;
    fn av1_ac_quant_QTX(
        qindex: core::ffi::c_int,
        delta: core::ffi::c_int,
        bit_depth: aom_bit_depth_t,
    ) -> int16_t;
    fn av1_use_qmatrix(
        quant_params: *const CommonQuantParams,
        xd: *const macroblockd,
        segment_id: core::ffi::c_int,
    ) -> bool;
    static mut av1_block_error: Option<
        unsafe extern "C-unwind" fn(
            *const tran_low_t,
            *const tran_low_t,
            intptr_t,
            *mut int64_t,
        ) -> int64_t,
    >;
    fn av1_get_crc32c_value_c(
        crc_calculator: *mut core::ffi::c_void,
        p: *const uint8_t,
        length: size_t,
    ) -> uint32_t;
    fn av1_get_horver_correlation_full_c(
        diff: *const int16_t,
        stride: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        hcorr: *mut core::ffi::c_float,
        vcorr: *mut core::ffi::c_float,
    );
    static mut av1_highbd_block_error: Option<
        unsafe extern "C-unwind" fn(
            *const tran_low_t,
            *const tran_low_t,
            intptr_t,
            *mut int64_t,
            core::ffi::c_int,
        ) -> int64_t,
    >;
    fn av1_nn_fast_softmax_16_c(
        input_nodes: *const core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn av1_nn_predict_c(
        input_nodes: *const core::ffi::c_float,
        nn_config: *const NN_CONFIG,
        reduce_prec: core::ffi::c_int,
        output: *mut core::ffi::c_float,
    );
    fn cfl_store_tx(
        xd: *mut MACROBLOCKD,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
        tx_size: TX_SIZE,
        bsize: BLOCK_SIZE,
    );
    fn av1_predict_intra_block_facade(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        plane: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        blk_row: core::ffi::c_int,
        tx_size: TX_SIZE,
    );
    fn av1_fwd_txfm(
        src_diff: *const int16_t,
        coeff: *mut tran_low_t,
        diff_stride: core::ffi::c_int,
        txfm_param: *mut TxfmParam,
    );
    fn av1_get_tx_scale(tx_size: TX_SIZE) -> core::ffi::c_int;
    fn av1_inverse_transform_block(
        xd: *const MACROBLOCKD,
        dqcoeff: *const tran_low_t,
        plane: core::ffi::c_int,
        tx_type: TX_TYPE,
        tx_size: TX_SIZE,
        dst: *mut uint8_t,
        stride: core::ffi::c_int,
        eob: core::ffi::c_int,
        reduced_tx_set: core::ffi::c_int,
    );
    static av1_scan_orders: [[SCAN_ORDER; 16]; 19];
    fn av1_model_rd_from_var_lapndz(
        var: int64_t,
        n: core::ffi::c_uint,
        qstep: core::ffi::c_uint,
        rate: *mut core::ffi::c_int,
        dist: *mut int64_t,
    );
    fn av1_model_rd_curvfit(
        bsize: BLOCK_SIZE,
        sse_norm: core::ffi::c_double,
        xqr: core::ffi::c_double,
        rate_dist_f: *mut core::ffi::c_double,
    );
    fn av1_get_entropy_contexts(
        plane_bsize: BLOCK_SIZE,
        pd: *const macroblockd_plane,
        t_above: *mut ENTROPY_CONTEXT,
        t_left: *mut ENTROPY_CONTEXT,
    );
    fn aom_sse_odd_size(
        a: *const uint8_t,
        a_stride: core::ffi::c_int,
        b: *const uint8_t,
        b_stride: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    ) -> uint32_t;
    fn aom_highbd_sse_odd_size(
        a: *const uint8_t,
        a_stride: core::ffi::c_int,
        b: *const uint8_t,
        b_stride: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    ) -> uint64_t;
    fn av1_foreach_transformed_block_in_plane(
        xd: *const MACROBLOCKD,
        plane_bsize: BLOCK_SIZE,
        plane: core::ffi::c_int,
        visit: foreach_transformed_block_visitor,
        arg: *mut core::ffi::c_void,
    );
    fn av1_setup_xform(
        cm: *const AV1_COMMON,
        x: *mut MACROBLOCK,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        txfm_param: *mut TxfmParam,
    );
    fn av1_setup_quant(
        tx_size: TX_SIZE,
        use_optimize_b: core::ffi::c_int,
        xform_quant_idx: core::ffi::c_int,
        use_quant_b_adapt: core::ffi::c_int,
        qparam: *mut QUANT_PARAM,
    );
    fn av1_setup_qmatrix(
        quant_params: *const CommonQuantParams,
        xd: *const MACROBLOCKD,
        plane: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        qparam: *mut QUANT_PARAM,
    );
    fn av1_xform_dc_only(
        x: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        txfm_param: *mut TxfmParam,
        per_px_mean: int64_t,
    );
    fn av1_xform_quant(
        x: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        blk_row: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        plane_bsize: BLOCK_SIZE,
        txfm_param: *mut TxfmParam,
        qparam: *const QUANT_PARAM,
    );
    fn av1_xform(
        x: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        blk_row: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        plane_bsize: BLOCK_SIZE,
        txfm_param: *mut TxfmParam,
    );
    fn av1_quant(
        x: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        txfm_param: *mut TxfmParam,
        qparam: *const QUANT_PARAM,
    );
    fn av1_optimize_b(
        cpi: *const AV1_COMP,
        mb: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        txb_ctx: *const TXB_CTX,
        rate_cost: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_subtract_txb(
        x: *mut MACROBLOCK,
        plane: core::ffi::c_int,
        plane_bsize: BLOCK_SIZE,
        blk_col: core::ffi::c_int,
        blk_row: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        do_border_pad: bool,
    );
    fn av1_subtract_plane(
        x: *mut MACROBLOCK,
        plane_bsize: BLOCK_SIZE,
        plane: core::ffi::c_int,
        do_border_pad: bool,
    );
    fn aom_convolve_copy_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn aom_get_blk_sse_sum_c(
        data: *const int16_t,
        stride: core::ffi::c_int,
        bw: core::ffi::c_int,
        bh: core::ffi::c_int,
        x_sum: *mut core::ffi::c_int,
        x2_sum: *mut int64_t,
    );
    fn aom_highbd_convolve_copy_c(
        src: *const uint16_t,
        src_stride: ptrdiff_t,
        dst: *mut uint16_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn aom_highbd_sse_c(
        a8: *const uint8_t,
        a_stride: core::ffi::c_int,
        b8: *const uint8_t,
        b_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> int64_t;
    fn aom_satd_c(
        coeff: *const tran_low_t,
        length: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_sse_c(
        a: *const uint8_t,
        a_stride: core::ffi::c_int,
        b: *const uint8_t,
        b_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> int64_t;
    fn aom_sum_squares_2d_i16_c(
        src: *const int16_t,
        stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> uint64_t;
    fn aom_sum_sse_2d_i16_c(
        src: *const int16_t,
        src_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        sum: *mut core::ffi::c_int,
    ) -> uint64_t;
    fn av1_cost_coeffs_txb(
        x: *const MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        txb_ctx: *const TXB_CTX,
        reduced_tx_set_used: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_cost_coeffs_txb_laplacian(
        x: *const MACROBLOCK,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
        txb_ctx: *const TXB_CTX,
        reduced_tx_set_used: core::ffi::c_int,
        adjust_eob: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed = 4;
pub const TX_SIZES: C2RustUnnamed = 5;
pub const TX_SIZES_ALL: C2RustUnnamed = 19;
pub const TX_64X16: C2RustUnnamed = 18;
pub const TX_16X64: C2RustUnnamed = 17;
pub const TX_32X8: C2RustUnnamed = 16;
pub const TX_8X32: C2RustUnnamed = 15;
pub const TX_16X4: C2RustUnnamed = 14;
pub const TX_4X16: C2RustUnnamed = 13;
pub const TX_64X32: C2RustUnnamed = 12;
pub const TX_32X64: C2RustUnnamed = 11;
pub const TX_32X16: C2RustUnnamed = 10;
pub const TX_16X32: C2RustUnnamed = 9;
pub const TX_16X8: C2RustUnnamed = 8;
pub const TX_8X16: C2RustUnnamed = 7;
pub const TX_8X4: C2RustUnnamed = 6;
pub const TX_4X8: C2RustUnnamed = 5;
pub const TX_64X64: C2RustUnnamed = 4;
pub const TX_32X32: C2RustUnnamed = 3;
pub const TX_16X16: C2RustUnnamed = 2;
pub const TX_8X8: C2RustUnnamed = 1;
pub const TX_4X4: C2RustUnnamed = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_0 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_0 = 15;
pub const TX_TYPES: C2RustUnnamed_0 = 16;
pub const H_FLIPADST: C2RustUnnamed_0 = 15;
pub const V_FLIPADST: C2RustUnnamed_0 = 14;
pub const H_ADST: C2RustUnnamed_0 = 13;
pub const V_ADST: C2RustUnnamed_0 = 12;
pub const H_DCT: C2RustUnnamed_0 = 11;
pub const V_DCT: C2RustUnnamed_0 = 10;
pub const IDTX: C2RustUnnamed_0 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_0 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_0 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_0 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_0 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_0 = 4;
pub const ADST_ADST: C2RustUnnamed_0 = 3;
pub const DCT_ADST: C2RustUnnamed_0 = 2;
pub const ADST_DCT: C2RustUnnamed_0 = 1;
pub const DCT_DCT: C2RustUnnamed_0 = 0;
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const EXT_TX_SET_TYPES: C2RustUnnamed_1 = 6;
pub const EXT_TX_SET_ALL16: C2RustUnnamed_1 = 5;
pub const EXT_TX_SET_DTT9_IDTX_1DDCT: C2RustUnnamed_1 = 4;
pub const EXT_TX_SET_DTT4_IDTX_1DDCT: C2RustUnnamed_1 = 3;
pub const EXT_TX_SET_DTT4_IDTX: C2RustUnnamed_1 = 2;
pub const EXT_TX_SET_DCT_IDTX: C2RustUnnamed_1 = 1;
pub const EXT_TX_SET_DCTONLY: C2RustUnnamed_1 = 0;
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
pub type float_t = core::ffi::c_float;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut core::ffi::c_uchar,
    pub _r: core::ffi::c_int,
    pub _w: core::ffi::c_int,
    pub _flags: core::ffi::c_short,
    pub _file: core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: core::ffi::c_int,
    pub _cookie: *mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C-unwind" fn(*mut core::ffi::c_void, fpos_t, core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: core::ffi::c_int,
    pub _ubuf: [core::ffi::c_uchar; 3],
    pub _nbuf: [core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = libc::FILE;
pub type aom_color_primaries = core::ffi::c_uint;
pub const AOM_CICP_CP_RESERVED_23: aom_color_primaries = 23;
pub const AOM_CICP_CP_EBU_3213: aom_color_primaries = 22;
pub const AOM_CICP_CP_RESERVED_13: aom_color_primaries = 13;
pub const AOM_CICP_CP_SMPTE_432: aom_color_primaries = 12;
pub const AOM_CICP_CP_SMPTE_431: aom_color_primaries = 11;
pub const AOM_CICP_CP_XYZ: aom_color_primaries = 10;
pub const AOM_CICP_CP_BT_2020: aom_color_primaries = 9;
pub const AOM_CICP_CP_GENERIC_FILM: aom_color_primaries = 8;
pub const AOM_CICP_CP_SMPTE_240: aom_color_primaries = 7;
pub const AOM_CICP_CP_BT_601: aom_color_primaries = 6;
pub const AOM_CICP_CP_BT_470_B_G: aom_color_primaries = 5;
pub const AOM_CICP_CP_BT_470_M: aom_color_primaries = 4;
pub const AOM_CICP_CP_RESERVED_3: aom_color_primaries = 3;
pub const AOM_CICP_CP_UNSPECIFIED: aom_color_primaries = 2;
pub const AOM_CICP_CP_BT_709: aom_color_primaries = 1;
pub const AOM_CICP_CP_RESERVED_0: aom_color_primaries = 0;
pub type aom_color_primaries_t = aom_color_primaries;
pub type aom_transfer_characteristics = core::ffi::c_uint;
pub const AOM_CICP_TC_RESERVED_19: aom_transfer_characteristics = 19;
pub const AOM_CICP_TC_HLG: aom_transfer_characteristics = 18;
pub const AOM_CICP_TC_SMPTE_428: aom_transfer_characteristics = 17;
pub const AOM_CICP_TC_SMPTE_2084: aom_transfer_characteristics = 16;
pub const AOM_CICP_TC_BT_2020_12_BIT: aom_transfer_characteristics = 15;
pub const AOM_CICP_TC_BT_2020_10_BIT: aom_transfer_characteristics = 14;
pub const AOM_CICP_TC_SRGB: aom_transfer_characteristics = 13;
pub const AOM_CICP_TC_BT_1361: aom_transfer_characteristics = 12;
pub const AOM_CICP_TC_IEC_61966: aom_transfer_characteristics = 11;
pub const AOM_CICP_TC_LOG_100_SQRT10: aom_transfer_characteristics = 10;
pub const AOM_CICP_TC_LOG_100: aom_transfer_characteristics = 9;
pub const AOM_CICP_TC_LINEAR: aom_transfer_characteristics = 8;
pub const AOM_CICP_TC_SMPTE_240: aom_transfer_characteristics = 7;
pub const AOM_CICP_TC_BT_601: aom_transfer_characteristics = 6;
pub const AOM_CICP_TC_BT_470_B_G: aom_transfer_characteristics = 5;
pub const AOM_CICP_TC_BT_470_M: aom_transfer_characteristics = 4;
pub const AOM_CICP_TC_RESERVED_3: aom_transfer_characteristics = 3;
pub const AOM_CICP_TC_UNSPECIFIED: aom_transfer_characteristics = 2;
pub const AOM_CICP_TC_BT_709: aom_transfer_characteristics = 1;
pub const AOM_CICP_TC_RESERVED_0: aom_transfer_characteristics = 0;
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
pub type aom_matrix_coefficients = core::ffi::c_uint;
pub const AOM_CICP_MC_YCGCO_RO: aom_matrix_coefficients = 17;
pub const AOM_CICP_MC_YCGCO_RE: aom_matrix_coefficients = 16;
pub const AOM_CICP_MC_IPT_C2: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_RESERVED_15: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_ICTCP: aom_matrix_coefficients = 14;
pub const AOM_CICP_MC_CHROMAT_CL: aom_matrix_coefficients = 13;
pub const AOM_CICP_MC_CHROMAT_NCL: aom_matrix_coefficients = 12;
pub const AOM_CICP_MC_SMPTE_2085: aom_matrix_coefficients = 11;
pub const AOM_CICP_MC_BT_2020_CL: aom_matrix_coefficients = 10;
pub const AOM_CICP_MC_BT_2020_NCL: aom_matrix_coefficients = 9;
pub const AOM_CICP_MC_SMPTE_YCGCO: aom_matrix_coefficients = 8;
pub const AOM_CICP_MC_SMPTE_240: aom_matrix_coefficients = 7;
pub const AOM_CICP_MC_BT_601: aom_matrix_coefficients = 6;
pub const AOM_CICP_MC_BT_470_B_G: aom_matrix_coefficients = 5;
pub const AOM_CICP_MC_FCC: aom_matrix_coefficients = 4;
pub const AOM_CICP_MC_RESERVED_3: aom_matrix_coefficients = 3;
pub const AOM_CICP_MC_UNSPECIFIED: aom_matrix_coefficients = 2;
pub const AOM_CICP_MC_BT_709: aom_matrix_coefficients = 1;
pub const AOM_CICP_MC_IDENTITY: aom_matrix_coefficients = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_color_range_t = aom_color_range;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_metadata_insert_flags = core::ffi::c_uint;
pub const AOM_MIF_ANY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 18;
pub const AOM_MIF_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 17;
pub const AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 16;
pub const AOM_MIF_ANY_FRAME: aom_metadata_insert_flags = 2;
pub const AOM_MIF_KEY_FRAME: aom_metadata_insert_flags = 1;
pub const AOM_MIF_NON_KEY_FRAME: aom_metadata_insert_flags = 0;
pub type aom_metadata_insert_flags_t = aom_metadata_insert_flags;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_metadata_array {
    pub sz: size_t,
    pub metadata_array: *mut *mut aom_metadata_t,
}
pub type aom_metadata_t = aom_metadata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_metadata {
    pub type_0: uint32_t,
    pub payload: *mut uint8_t,
    pub sz: size_t,
    pub insert_flag: aom_metadata_insert_flags_t,
}
pub type aom_metadata_array_t = aom_metadata_array;
pub type aom_codec_err_t = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: aom_codec_err_t = 9;
pub const AOM_CODEC_INVALID_PARAM: aom_codec_err_t = 8;
pub const AOM_CODEC_CORRUPT_FRAME: aom_codec_err_t = 7;
pub const AOM_CODEC_UNSUP_FEATURE: aom_codec_err_t = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: aom_codec_err_t = 5;
pub const AOM_CODEC_INCAPABLE: aom_codec_err_t = 4;
pub const AOM_CODEC_ABI_MISMATCH: aom_codec_err_t = 3;
pub const AOM_CODEC_MEM_ERROR: aom_codec_err_t = 2;
pub const AOM_CODEC_ERROR: aom_codec_err_t = 1;
pub const AOM_CODEC_OK: aom_codec_err_t = 0;
pub type aom_codec_pts_t = int64_t;
pub type aom_fixed_buf_t = aom_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
}
pub type aom_rc_mode = core::ffi::c_uint;
pub const AOM_Q: aom_rc_mode = 3;
pub const AOM_CQ: aom_rc_mode = 2;
pub const AOM_CBR: aom_rc_mode = 1;
pub const AOM_VBR: aom_rc_mode = 0;
pub type aom_superres_mode = core::ffi::c_uint;
pub const AOM_SUPERRES_AUTO: aom_superres_mode = 4;
pub const AOM_SUPERRES_QTHRESH: aom_superres_mode = 3;
pub const AOM_SUPERRES_RANDOM: aom_superres_mode = 2;
pub const AOM_SUPERRES_FIXED: aom_superres_mode = 1;
pub const AOM_SUPERRES_NONE: aom_superres_mode = 0;
pub type aom_enc_pass = core::ffi::c_uint;
pub const AOM_RC_LAST_PASS: aom_enc_pass = 2;
pub const AOM_RC_THIRD_PASS: aom_enc_pass = 3;
pub const AOM_RC_SECOND_PASS: aom_enc_pass = 2;
pub const AOM_RC_FIRST_PASS: aom_enc_pass = 1;
pub const AOM_RC_ONE_PASS: aom_enc_pass = 0;
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_cx_pkt {
    pub kind: aom_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub frame: C2RustUnnamed_3,
    pub twopass_stats: aom_fixed_buf_t,
    pub firstpass_mb_stats: aom_fixed_buf_t,
    pub psnr: aom_psnr_pkt,
    pub raw: aom_fixed_buf_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_psnr_pkt {
    pub samples: [core::ffi::c_uint; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [core::ffi::c_double; 4],
    pub samples_hbd: [core::ffi::c_uint; 4],
    pub sse_hbd: [uint64_t; 4],
    pub psnr_hbd: [core::ffi::c_double; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
    pub pts: aom_codec_pts_t,
    pub duration: core::ffi::c_ulong,
    pub flags: aom_codec_frame_flags_t,
    pub partition_id: core::ffi::c_int,
    pub vis_frame_size: size_t,
}
pub type aom_codec_frame_flags_t = uint32_t;
pub type aom_codec_cx_pkt_kind = core::ffi::c_uint;
pub const AOM_CODEC_CUSTOM_PKT: aom_codec_cx_pkt_kind = 256;
pub const AOM_CODEC_PSNR_PKT: aom_codec_cx_pkt_kind = 3;
pub const AOM_CODEC_FPMB_STATS_PKT: aom_codec_cx_pkt_kind = 2;
pub const AOM_CODEC_STATS_PKT: aom_codec_cx_pkt_kind = 1;
pub const AOM_CODEC_CX_FRAME_PKT: aom_codec_cx_pkt_kind = 0;
pub type aom_enc_frame_flags_t = core::ffi::c_long;
pub type aom_release_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_get_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        size_t,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type aom_superblock_size = core::ffi::c_uint;
pub const AOM_SUPERBLOCK_SIZE_DYNAMIC: aom_superblock_size = 2;
pub const AOM_SUPERBLOCK_SIZE_128X128: aom_superblock_size = 1;
pub const AOM_SUPERBLOCK_SIZE_64X64: aom_superblock_size = 0;
pub type aom_superblock_size_t = aom_superblock_size;
pub type BITSTREAM_PROFILE = int8_t;
pub type BLOCK_SIZE = core::ffi::c_uchar;
pub const BLOCK_LARGEST: BLOCK_SIZE = 15;
pub const BLOCK_INVALID: BLOCK_SIZE = 255;
pub const BLOCK_SIZES: BLOCK_SIZE = 16;
pub const BLOCK_SIZES_ALL: BLOCK_SIZE = 22;
pub const BLOCK_64X16: BLOCK_SIZE = 21;
pub const BLOCK_16X64: BLOCK_SIZE = 20;
pub const BLOCK_32X8: BLOCK_SIZE = 19;
pub const BLOCK_8X32: BLOCK_SIZE = 18;
pub const BLOCK_16X4: BLOCK_SIZE = 17;
pub const BLOCK_4X16: BLOCK_SIZE = 16;
pub const BLOCK_128X128: BLOCK_SIZE = 15;
pub const BLOCK_128X64: BLOCK_SIZE = 14;
pub const BLOCK_64X128: BLOCK_SIZE = 13;
pub const BLOCK_64X64: BLOCK_SIZE = 12;
pub const BLOCK_64X32: BLOCK_SIZE = 11;
pub const BLOCK_32X64: BLOCK_SIZE = 10;
pub const BLOCK_32X32: BLOCK_SIZE = 9;
pub const BLOCK_32X16: BLOCK_SIZE = 8;
pub const BLOCK_16X32: BLOCK_SIZE = 7;
pub const BLOCK_16X16: BLOCK_SIZE = 6;
pub const BLOCK_16X8: BLOCK_SIZE = 5;
pub const BLOCK_8X16: BLOCK_SIZE = 4;
pub const BLOCK_8X8: BLOCK_SIZE = 3;
pub const BLOCK_8X4: BLOCK_SIZE = 2;
pub const BLOCK_4X8: BLOCK_SIZE = 1;
pub const BLOCK_4X4: BLOCK_SIZE = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const TX_MODES: C2RustUnnamed_4 = 3;
pub const TX_MODE_SELECT: C2RustUnnamed_4 = 2;
pub const TX_MODE_LARGEST: C2RustUnnamed_4 = 1;
pub const ONLY_4X4: C2RustUnnamed_4 = 0;
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_5 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_5 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_5 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_6 = 25;
pub const INTRA_MODES: C2RustUnnamed_6 = 13;
pub const INTER_MODE_END: C2RustUnnamed_6 = 25;
pub const INTER_MODE_START: C2RustUnnamed_6 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_6 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_6 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_6 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_6 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_6 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_6 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_6 = 13;
pub const DIR_MODE_END: C2RustUnnamed_6 = 9;
pub const DIR_MODE_START: C2RustUnnamed_6 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_6 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_6 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_6 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_6 = 25;
pub const NEW_NEWMV: C2RustUnnamed_6 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_6 = 23;
pub const NEW_NEARMV: C2RustUnnamed_6 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_6 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_6 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_6 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_6 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_6 = 17;
pub const NEWMV: C2RustUnnamed_6 = 16;
pub const GLOBALMV: C2RustUnnamed_6 = 15;
pub const NEARMV: C2RustUnnamed_6 = 14;
pub const NEARESTMV: C2RustUnnamed_6 = 13;
pub const PAETH_PRED: C2RustUnnamed_6 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_6 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_6 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_6 = 9;
pub const D67_PRED: C2RustUnnamed_6 = 8;
pub const D203_PRED: C2RustUnnamed_6 = 7;
pub const D157_PRED: C2RustUnnamed_6 = 6;
pub const D113_PRED: C2RustUnnamed_6 = 5;
pub const D135_PRED: C2RustUnnamed_6 = 4;
pub const D45_PRED: C2RustUnnamed_6 = 3;
pub const H_PRED: C2RustUnnamed_6 = 2;
pub const V_PRED: C2RustUnnamed_6 = 1;
pub const DC_PRED: C2RustUnnamed_6 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_7 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_7 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_7 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_7 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_7 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_7 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_7 = 9;
pub const UV_D67_PRED: C2RustUnnamed_7 = 8;
pub const UV_D203_PRED: C2RustUnnamed_7 = 7;
pub const UV_D157_PRED: C2RustUnnamed_7 = 6;
pub const UV_D113_PRED: C2RustUnnamed_7 = 5;
pub const UV_D135_PRED: C2RustUnnamed_7 = 4;
pub const UV_D45_PRED: C2RustUnnamed_7 = 3;
pub const UV_H_PRED: C2RustUnnamed_7 = 2;
pub const UV_V_PRED: C2RustUnnamed_7 = 1;
pub const UV_DC_PRED: C2RustUnnamed_7 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_8 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_8 = 7;
pub const BWD_REFS: C2RustUnnamed_8 = 3;
pub const FWD_REFS: C2RustUnnamed_8 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_8 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_8 = 8;
pub const REF_FRAMES: C2RustUnnamed_8 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_8 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_8 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_8 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_8 = 4;
pub const LAST3_FRAME: C2RustUnnamed_8 = 3;
pub const LAST2_FRAME: C2RustUnnamed_8 = 2;
pub const LAST_FRAME: C2RustUnnamed_8 = 1;
pub const INTRA_FRAME: C2RustUnnamed_8 = 0;
pub const NONE_FRAME: C2RustUnnamed_8 = -1;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_pyramid {
    pub mutex: pthread_mutex_t,
    pub max_levels: core::ffi::c_int,
    pub filled_levels: core::ffi::c_int,
    pub buffer_alloc: *mut uint8_t,
    pub layers: *mut PyramidLayer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyramidLayer {
    pub buffer: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub c2rust_unnamed: C2RustUnnamed_19,
    pub c2rust_unnamed_0: C2RustUnnamed_17,
    pub c2rust_unnamed_1: C2RustUnnamed_15,
    pub c2rust_unnamed_2: C2RustUnnamed_13,
    pub c2rust_unnamed_3: C2RustUnnamed_11,
    pub c2rust_unnamed_4: C2RustUnnamed_9,
    pub use_external_reference_buffers: core::ffi::c_int,
    pub store_buf_adr: [*mut uint8_t; 3],
    pub y_pyramid: *mut image_pyramid,
    pub corners: *mut corner_list,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: core::ffi::c_int,
    pub frame_size: size_t,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub bit_depth: core::ffi::c_uint,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub monochrome: uint8_t,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub color_range: aom_color_range_t,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub corrupted: core::ffi::c_int,
    pub flags: core::ffi::c_int,
    pub metadata: *mut aom_metadata_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub c2rust_unnamed: C2RustUnnamed_18,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub c2rust_unnamed: C2RustUnnamed_20,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation {
    pub enabled: uint8_t,
    pub update_map: uint8_t,
    pub update_data: uint8_t,
    pub temporal_update: uint8_t,
    pub feature_data: [[int16_t; 8]; 8],
    pub feature_mask: [core::ffi::c_uint; 8],
    pub last_active_segid: core::ffi::c_int,
    pub segid_preskip: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Common {
    pub current_frame: CurrentFrame,
    pub error: *mut aom_internal_error_info,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub superres_upscaled_width: core::ffi::c_int,
    pub superres_upscaled_height: core::ffi::c_int,
    pub superres_scale_denominator: uint8_t,
    pub buffer_removal_times: [uint32_t; 33],
    pub frame_presentation_time: uint32_t,
    pub prev_frame: *mut RefCntBuffer,
    pub cur_frame: *mut RefCntBuffer,
    pub remapped_ref_idx: [core::ffi::c_int; 8],
    pub sf_identity: scale_factors,
    pub ref_scale_factors: [scale_factors; 8],
    pub ref_frame_map: [*mut RefCntBuffer; 8],
    pub show_frame: core::ffi::c_int,
    pub showable_frame: core::ffi::c_int,
    pub show_existing_frame: core::ffi::c_int,
    pub features: FeatureFlags,
    pub mi_params: CommonModeInfoParams,
    pub quant_params: CommonQuantParams,
    pub seg: segmentation,
    pub last_frame_seg_map: *mut uint8_t,
    pub lf_info: loop_filter_info_n,
    pub lf: loopfilter,
    pub rst_info: [RestorationInfo; 3],
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut RestorationLineBuffers,
    pub rst_frame: YV12_BUFFER_CONFIG,
    pub cdef_info: CdefInfo,
    pub film_grain_params: aom_film_grain_t,
    pub delta_q_info: DeltaQInfo,
    pub global_motion: [WarpedMotionParams; 8],
    pub seq_params: *mut SequenceHeader,
    pub fc: *mut FRAME_CONTEXT,
    pub default_frame_context: *mut FRAME_CONTEXT,
    pub tiles: CommonTileParams,
    pub buffer_pool: *mut BufferPool,
    pub above_contexts: CommonContexts,
    pub current_frame_id: core::ffi::c_int,
    pub ref_frame_id: [core::ffi::c_int; 8],
    pub tpl_mvs: *mut TPL_MV_REF,
    pub tpl_mvs_mem_size: core::ffi::c_int,
    pub ref_frame_sign_bias: [core::ffi::c_int; 8],
    pub ref_frame_side: [int8_t; 8],
    pub temporal_layer_id: core::ffi::c_int,
    pub spatial_layer_id: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPL_MV_REF {
    pub mfmv0: int_mv,
    pub ref_frame_offset: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
    pub as_fullmv: FULLPEL_MV,
}
pub type FULLPEL_MV = fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type MV = mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv {
    pub row: int16_t,
    pub col: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonContexts {
    pub partition: *mut *mut PARTITION_CONTEXT,
    pub entropy: [*mut *mut ENTROPY_CONTEXT; 3],
    pub txfm: *mut *mut TXFM_CONTEXT,
    pub num_planes: core::ffi::c_int,
    pub num_tile_rows: core::ffi::c_int,
    pub num_mi_cols: core::ffi::c_int,
}
pub type ENTROPY_CONTEXT = core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferPool {
    pub pool_mutex: pthread_mutex_t,
    pub cb_priv: *mut core::ffi::c_void,
    pub get_fb_cb: aom_get_frame_buffer_cb_fn_t,
    pub release_fb_cb: aom_release_frame_buffer_cb_fn_t,
    pub frame_bufs: *mut RefCntBuffer,
    pub num_frame_bufs: uint8_t,
    pub int_frame_buffers: InternalFrameBufferList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBufferList {
    pub num_internal_frame_buffers: core::ffi::c_int,
    pub int_fb: *mut InternalFrameBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBuffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub in_use: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefCntBuffer {
    pub ref_count: core::ffi::c_int,
    pub order_hint: core::ffi::c_uint,
    pub ref_order_hints: [core::ffi::c_uint; 7],
    pub display_order_hint: core::ffi::c_uint,
    pub ref_display_order_hint: [core::ffi::c_uint; 7],
    pub pyramid_level: core::ffi::c_uint,
    pub base_qindex: core::ffi::c_int,
    pub mvs: *mut MV_REF,
    pub seg_map: *mut uint8_t,
    pub seg: segmentation,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub global_motion: [WarpedMotionParams; 8],
    pub showable_frame: core::ffi::c_int,
    pub film_grain_params_present: uint8_t,
    pub film_grain_params: aom_film_grain_t,
    pub raw_frame_buffer: aom_codec_frame_buffer_t,
    pub buf: YV12_BUFFER_CONFIG,
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub interp_filter_selected: [core::ffi::c_int; 4],
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
    pub frame_context: FRAME_CONTEXT,
    pub filter_level: [core::ffi::c_int; 2],
}
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub txb_skip_cdf: [[[aom_cdf_prob; 3]; 13]; 5],
    pub eob_extra_cdf: [[[[aom_cdf_prob; 3]; 9]; 2]; 5],
    pub dc_sign_cdf: [[[aom_cdf_prob; 3]; 3]; 2],
    pub eob_flag_cdf16: [[[aom_cdf_prob; 6]; 2]; 2],
    pub eob_flag_cdf32: [[[aom_cdf_prob; 7]; 2]; 2],
    pub eob_flag_cdf64: [[[aom_cdf_prob; 8]; 2]; 2],
    pub eob_flag_cdf128: [[[aom_cdf_prob; 9]; 2]; 2],
    pub eob_flag_cdf256: [[[aom_cdf_prob; 10]; 2]; 2],
    pub eob_flag_cdf512: [[[aom_cdf_prob; 11]; 2]; 2],
    pub eob_flag_cdf1024: [[[aom_cdf_prob; 12]; 2]; 2],
    pub coeff_base_eob_cdf: [[[[aom_cdf_prob; 4]; 4]; 2]; 5],
    pub coeff_base_cdf: [[[[aom_cdf_prob; 5]; 42]; 2]; 5],
    pub coeff_br_cdf: [[[[aom_cdf_prob; 5]; 21]; 2]; 5],
    pub newmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub zeromv_cdf: [[aom_cdf_prob; 3]; 2],
    pub refmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub drl_cdf: [[aom_cdf_prob; 3]; 3],
    pub inter_compound_mode_cdf: [[aom_cdf_prob; 9]; 8],
    pub compound_type_cdf: [[aom_cdf_prob; 3]; 22],
    pub wedge_idx_cdf: [[aom_cdf_prob; 17]; 22],
    pub interintra_cdf: [[aom_cdf_prob; 3]; 4],
    pub wedge_interintra_cdf: [[aom_cdf_prob; 3]; 22],
    pub interintra_mode_cdf: [[aom_cdf_prob; 5]; 4],
    pub motion_mode_cdf: [[aom_cdf_prob; 4]; 22],
    pub obmc_cdf: [[aom_cdf_prob; 3]; 22],
    pub palette_y_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_uv_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_y_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_uv_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_y_mode_cdf: [[[aom_cdf_prob; 3]; 3]; 7],
    pub palette_uv_mode_cdf: [[aom_cdf_prob; 3]; 2],
    pub comp_inter_cdf: [[aom_cdf_prob; 3]; 5],
    pub single_ref_cdf: [[[aom_cdf_prob; 3]; 6]; 3],
    pub comp_ref_type_cdf: [[aom_cdf_prob; 3]; 5],
    pub uni_comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_bwdref_cdf: [[[aom_cdf_prob; 3]; 2]; 3],
    pub txfm_partition_cdf: [[aom_cdf_prob; 3]; 21],
    pub compound_index_cdf: [[aom_cdf_prob; 3]; 6],
    pub comp_group_idx_cdf: [[aom_cdf_prob; 3]; 6],
    pub skip_mode_cdfs: [[aom_cdf_prob; 3]; 3],
    pub skip_txfm_cdfs: [[aom_cdf_prob; 3]; 3],
    pub intra_inter_cdf: [[aom_cdf_prob; 3]; 4],
    pub nmvc: nmv_context,
    pub ndvc: nmv_context,
    pub intrabc_cdf: [aom_cdf_prob; 3],
    pub seg: segmentation_probs,
    pub filter_intra_cdfs: [[aom_cdf_prob; 3]; 22],
    pub filter_intra_mode_cdf: [aom_cdf_prob; 6],
    pub switchable_restore_cdf: [aom_cdf_prob; 4],
    pub wiener_restore_cdf: [aom_cdf_prob; 3],
    pub sgrproj_restore_cdf: [aom_cdf_prob; 3],
    pub y_mode_cdf: [[aom_cdf_prob; 14]; 4],
    pub uv_mode_cdf: [[[aom_cdf_prob; 15]; 13]; 2],
    pub partition_cdf: [[aom_cdf_prob; 11]; 20],
    pub switchable_interp_cdf: [[aom_cdf_prob; 4]; 16],
    pub kf_y_cdf: [[[aom_cdf_prob; 14]; 5]; 5],
    pub angle_delta_cdf: [[aom_cdf_prob; 8]; 8],
    pub tx_size_cdf: [[[aom_cdf_prob; 4]; 3]; 4],
    pub delta_q_cdf: [aom_cdf_prob; 5],
    pub delta_lf_multi_cdf: [[aom_cdf_prob; 5]; 4],
    pub delta_lf_cdf: [aom_cdf_prob; 5],
    pub intra_ext_tx_cdf: [[[[aom_cdf_prob; 17]; 13]; 4]; 3],
    pub inter_ext_tx_cdf: [[[aom_cdf_prob; 17]; 4]; 4],
    pub cfl_sign_cdf: [aom_cdf_prob; 9],
    pub cfl_alpha_cdf: [[aom_cdf_prob; 17]; 6],
    pub initialized: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_context {
    pub joints_cdf: [aom_cdf_prob; 5],
    pub comps: [nmv_component; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_component {
    pub classes_cdf: [aom_cdf_prob; 12],
    pub class0_fp_cdf: [[aom_cdf_prob; 5]; 2],
    pub fp_cdf: [aom_cdf_prob; 5],
    pub sign_cdf: [aom_cdf_prob; 3],
    pub class0_hp_cdf: [aom_cdf_prob; 3],
    pub hp_cdf: [aom_cdf_prob; 3],
    pub class0_cdf: [aom_cdf_prob; 3],
    pub bits_cdf: [[aom_cdf_prob; 3]; 10],
}
pub type FRAME_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_t {
    pub apply_grain: core::ffi::c_int,
    pub update_parameters: core::ffi::c_int,
    pub scaling_points_y: [[core::ffi::c_int; 2]; 14],
    pub num_y_points: core::ffi::c_int,
    pub scaling_points_cb: [[core::ffi::c_int; 2]; 10],
    pub num_cb_points: core::ffi::c_int,
    pub scaling_points_cr: [[core::ffi::c_int; 2]; 10],
    pub num_cr_points: core::ffi::c_int,
    pub scaling_shift: core::ffi::c_int,
    pub ar_coeff_lag: core::ffi::c_int,
    pub ar_coeffs_y: [core::ffi::c_int; 24],
    pub ar_coeffs_cb: [core::ffi::c_int; 25],
    pub ar_coeffs_cr: [core::ffi::c_int; 25],
    pub ar_coeff_shift: core::ffi::c_int,
    pub cb_mult: core::ffi::c_int,
    pub cb_luma_mult: core::ffi::c_int,
    pub cb_offset: core::ffi::c_int,
    pub cr_mult: core::ffi::c_int,
    pub cr_luma_mult: core::ffi::c_int,
    pub cr_offset: core::ffi::c_int,
    pub overlap_flag: core::ffi::c_int,
    pub clip_to_restricted_range: core::ffi::c_int,
    pub bit_depth: core::ffi::c_uint,
    pub chroma_scaling_from_luma: core::ffi::c_int,
    pub grain_scale_shift: core::ffi::c_int,
    pub random_seed: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
pub type TransformationType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_REF {
    pub mv: int_mv,
    pub ref_frame: MV_REFERENCE_FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonTileParams {
    pub cols: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub max_width_sb: core::ffi::c_int,
    pub max_height_sb: core::ffi::c_int,
    pub min_inner_width: core::ffi::c_int,
    pub uniform_spacing: core::ffi::c_int,
    pub log2_cols: core::ffi::c_int,
    pub log2_rows: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub min_log2_cols: core::ffi::c_int,
    pub min_log2_rows: core::ffi::c_int,
    pub max_log2_cols: core::ffi::c_int,
    pub max_log2_rows: core::ffi::c_int,
    pub min_log2: core::ffi::c_int,
    pub col_start_sb: [core::ffi::c_int; 65],
    pub row_start_sb: [core::ffi::c_int; 65],
    pub large_scale: core::ffi::c_uint,
    pub single_tile_decoding: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceHeader {
    pub num_bits_width: core::ffi::c_int,
    pub num_bits_height: core::ffi::c_int,
    pub max_frame_width: core::ffi::c_int,
    pub max_frame_height: core::ffi::c_int,
    pub frame_id_numbers_present_flag: uint8_t,
    pub frame_id_length: core::ffi::c_int,
    pub delta_frame_id_length: core::ffi::c_int,
    pub sb_size: BLOCK_SIZE,
    pub mib_size: core::ffi::c_int,
    pub mib_size_log2: core::ffi::c_int,
    pub order_hint_info: OrderHintInfo,
    pub force_screen_content_tools: uint8_t,
    pub still_picture: uint8_t,
    pub reduced_still_picture_hdr: uint8_t,
    pub force_integer_mv: uint8_t,
    pub enable_filter_intra: uint8_t,
    pub enable_intra_edge_filter: uint8_t,
    pub enable_interintra_compound: uint8_t,
    pub enable_masked_compound: uint8_t,
    pub enable_dual_filter: uint8_t,
    pub enable_warped_motion: uint8_t,
    pub enable_superres: uint8_t,
    pub enable_cdef: uint8_t,
    pub enable_restoration: uint8_t,
    pub profile: BITSTREAM_PROFILE,
    pub bit_depth: aom_bit_depth_t,
    pub use_highbitdepth: uint8_t,
    pub monochrome: uint8_t,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub color_range: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub separate_uv_delta_q: uint8_t,
    pub film_grain_params_present: uint8_t,
    pub operating_points_cnt_minus_1: core::ffi::c_int,
    pub operating_point_idc: [core::ffi::c_int; 32],
    pub has_nonzero_operating_point_idc: bool,
    pub timing_info_present: core::ffi::c_int,
    pub timing_info: aom_timing_info_t,
    pub decoder_model_info_present_flag: uint8_t,
    pub decoder_model_info: aom_dec_model_info_t,
    pub display_model_info_present_flag: uint8_t,
    pub seq_level_idx: [AV1_LEVEL; 32],
    pub tier: [uint8_t; 32],
    pub op_params: [aom_dec_model_op_parameters_t; 33],
}
pub type aom_dec_model_op_parameters_t = aom_dec_model_op_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_op_parameters {
    pub decoder_model_param_present_flag: core::ffi::c_int,
    pub bitrate: int64_t,
    pub buffer_size: int64_t,
    pub decoder_buffer_delay: uint32_t,
    pub encoder_buffer_delay: uint32_t,
    pub low_delay_mode_flag: core::ffi::c_int,
    pub display_model_param_present_flag: core::ffi::c_int,
    pub initial_display_delay: core::ffi::c_int,
}
pub type aom_dec_model_info_t = aom_dec_model_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_info {
    pub num_units_in_decoding_tick: uint32_t,
    pub encoder_decoder_buffer_delay_length: core::ffi::c_int,
    pub buffer_removal_time_length: core::ffi::c_int,
    pub frame_presentation_time_length: core::ffi::c_int,
}
pub type aom_timing_info_t = aom_timing;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_timing {
    pub num_units_in_display_tick: uint32_t,
    pub time_scale: uint32_t,
    pub equal_picture_interval: core::ffi::c_int,
    pub num_ticks_per_picture: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OrderHintInfo {
    pub enable_order_hint: core::ffi::c_int,
    pub order_hint_bits_minus_1: core::ffi::c_int,
    pub enable_dist_wtd_comp: core::ffi::c_int,
    pub enable_ref_frame_mvs: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeltaQInfo {
    pub delta_q_present_flag: core::ffi::c_int,
    pub delta_q_res: core::ffi::c_int,
    pub delta_lf_present_flag: core::ffi::c_int,
    pub delta_lf_res: core::ffi::c_int,
    pub delta_lf_multi: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefInfo {
    pub colbuf: [*mut uint16_t; 3],
    pub linebuf: [*mut uint16_t; 3],
    pub srcbuf: *mut uint16_t,
    pub allocated_colbuf_size: [size_t; 3],
    pub allocated_linebuf_size: [size_t; 3],
    pub allocated_srcbuf_size: size_t,
    pub cdef_damping: core::ffi::c_int,
    pub nb_cdef_strengths: core::ffi::c_int,
    pub cdef_strengths: [core::ffi::c_int; 16],
    pub cdef_uv_strengths: [core::ffi::c_int; 16],
    pub cdef_bits: core::ffi::c_int,
    pub allocated_mi_rows: core::ffi::c_int,
    pub allocated_num_workers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationLineBuffers {
    pub tmp_save_above: [[uint16_t; 392]; 3],
    pub tmp_save_below: [[uint16_t; 392]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationInfo {
    pub frame_restoration_type: RestorationType,
    pub restoration_unit_size: core::ffi::c_int,
    pub num_rest_units: core::ffi::c_int,
    pub vert_units: core::ffi::c_int,
    pub horz_units: core::ffi::c_int,
    pub unit_info: *mut RestorationUnitInfo,
    pub boundaries: RestorationStripeBoundaries,
    pub optimized_lr: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationStripeBoundaries {
    pub stripe_boundary_above: *mut uint8_t,
    pub stripe_boundary_below: *mut uint8_t,
    pub stripe_boundary_stride: core::ffi::c_int,
    pub stripe_boundary_size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationUnitInfo {
    pub restoration_type: RestorationType,
    pub wiener_info: WienerInfo,
    pub sgrproj_info: SgrprojInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SgrprojInfo {
    pub ep: core::ffi::c_int,
    pub xqd: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WienerInfo {
    pub __aom_align_vfilter: [crate::platform::Align16; 0],

    pub vfilter: InterpKernel,
    pub __aom_align_hfilter: [crate::platform::Align16; 0],
    pub hfilter: InterpKernel,
}
pub type InterpKernel = [int16_t; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loopfilter {
    pub filter_level: [core::ffi::c_int; 2],
    pub filter_level_u: core::ffi::c_int,
    pub filter_level_v: core::ffi::c_int,
    pub backup_filter_level: [core::ffi::c_int; 2],
    pub backup_filter_level_u: core::ffi::c_int,
    pub backup_filter_level_v: core::ffi::c_int,
    pub sharpness_level: core::ffi::c_int,
    pub mode_ref_delta_enabled: uint8_t,
    pub mode_ref_delta_update: uint8_t,
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub lfthr: [loop_filter_thresh; 64],
    pub lvl: [[[[[uint8_t; 2]; 8]; 2]; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_thresh {
    pub __aom_align_mblim: [crate::platform::Align16; 0],

    pub mblim: [uint8_t; 16],
    pub __aom_align_lim: [crate::platform::Align16; 0],
    pub lim: [uint8_t; 16],
    pub __aom_align_hev_thr: [crate::platform::Align16; 0],
    pub hev_thr: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonQuantParams {
    pub base_qindex: core::ffi::c_int,
    pub sharpness: core::ffi::c_int,
    pub y_dc_delta_q: core::ffi::c_int,
    pub u_dc_delta_q: core::ffi::c_int,
    pub v_dc_delta_q: core::ffi::c_int,
    pub u_ac_delta_q: core::ffi::c_int,
    pub v_ac_delta_q: core::ffi::c_int,
    pub y_dequant_QTX: [[int16_t; 2]; 8],
    pub u_dequant_QTX: [[int16_t; 2]; 8],
    pub v_dequant_QTX: [[int16_t; 2]; 8],
    pub giqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub gqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub y_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub u_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub v_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub using_qmatrix: bool,
    pub qmatrix_level_y: core::ffi::c_int,
    pub qmatrix_level_u: core::ffi::c_int,
    pub qmatrix_level_v: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonModeInfoParams {
    pub mb_rows: core::ffi::c_int,
    pub mb_cols: core::ffi::c_int,
    pub MBs: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_alloc: *mut MB_MODE_INFO,
    pub mi_alloc_size: core::ffi::c_int,
    pub mi_alloc_stride: core::ffi::c_int,
    pub mi_alloc_bsize: BLOCK_SIZE,
    pub mi_grid_base: *mut *mut MB_MODE_INFO,
    pub mi_grid_size: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub tx_type_map: *mut TX_TYPE,
    pub free_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub setup_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub set_mb_mi: Option<
        unsafe extern "C-unwind" fn(
            *mut CommonModeInfoParams,
            core::ffi::c_int,
            core::ffi::c_int,
            BLOCK_SIZE,
        ) -> (),
    >,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub bsize: BLOCK_SIZE,
    pub partition: PARTITION_TYPE,
    pub mode: PREDICTION_MODE,
    pub uv_mode: UV_PREDICTION_MODE,
    pub current_qindex: core::ffi::c_int,
    pub mv: [int_mv; 2],
    pub ref_frame: [MV_REFERENCE_FRAME; 2],
    pub interp_filters: int_interpfilters,
    pub motion_mode: MOTION_MODE,
    pub num_proj_ref: uint8_t,
    pub overlappable_neighbors: uint8_t,
    pub wm_params: WarpedMotionParams,
    pub interintra_mode: INTERINTRA_MODE,
    pub interintra_wedge_index: int8_t,
    pub interinter_comp: INTERINTER_COMPOUND_DATA,
    pub angle_delta: [int8_t; 2],
    pub filter_intra_mode_info: FILTER_INTRA_MODE_INFO,
    pub cfl_alpha_signs: int8_t,
    pub cfl_alpha_idx: uint8_t,
    pub palette_mode_info: PALETTE_MODE_INFO,
    pub skip_txfm: uint8_t,
    pub tx_size: TX_SIZE,
    pub inter_tx_size: [TX_SIZE; 16],
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    #[bitfield(name = "segment_id", ty = "uint8_t", bits = "0..=2")]
    #[bitfield(name = "seg_id_predicted", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "ref_mv_idx", ty = "uint8_t", bits = "4..=5")]
    #[bitfield(name = "skip_mode", ty = "uint8_t", bits = "6..=6")]
    #[bitfield(name = "use_intrabc", ty = "uint8_t", bits = "7..=7")]
    #[bitfield(name = "comp_group_idx", ty = "uint8_t", bits = "8..=8")]
    #[bitfield(name = "compound_idx", ty = "uint8_t", bits = "9..=9")]
    #[bitfield(name = "use_wedge_interintra", ty = "uint8_t", bits = "10..=10")]
    #[bitfield(name = "cdef_strength", ty = "int8_t", bits = "11..=14")]
    pub segment_id_seg_id_predicted_ref_mv_idx_skip_mode_use_intrabc_comp_group_idx_compound_idx_use_wedge_interintra_cdef_strength: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTE_MODE_INFO {
    pub palette_colors: [uint16_t; 24],
    pub palette_size: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER_INTRA_MODE_INFO {
    pub filter_intra_mode: FILTER_INTRA_MODE,
    pub use_filter_intra: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERINTER_COMPOUND_DATA {
    pub seg_mask: *mut uint8_t,
    pub wedge_index: int8_t,
    pub wedge_sign: int8_t,
    pub mask_type: DIFFWTD_MASK_TYPE,
    pub type_0: COMPOUND_TYPE,
}
pub type DIFFWTD_MASK_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_interpfilters {
    pub as_int: uint32_t,
    pub as_filters: InterpFilters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilters {
    pub y_filter: uint16_t,
    pub x_filter: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureFlags {
    pub disable_cdf_update: bool,
    pub allow_high_precision_mv: bool,
    pub cur_frame_force_integer_mv: bool,
    pub allow_screen_content_tools: bool,
    pub allow_intrabc: bool,
    pub allow_warped_motion: bool,
    pub allow_ref_frame_mvs: bool,
    pub coded_lossless: bool,
    pub all_lossless: bool,
    pub reduced_tx_set_used: bool,
    pub error_resilient_mode: bool,
    pub switchable_motion_mode: bool,
    pub tx_mode: TX_MODE,
    pub interp_filter: InterpFilter,
    pub primary_ref_frame: core::ffi::c_int,
    pub byte_alignment: core::ffi::c_int,
    pub refresh_frame_context: REFRESH_FRAME_CONTEXT_MODE,
}
pub type REFRESH_FRAME_CONTEXT_MODE = uint8_t;
pub type InterpFilter = core::ffi::c_uchar;
pub const INTERP_INVALID: InterpFilter = 255;
pub const EXTRA_FILTERS: InterpFilter = 2;
pub const SWITCHABLE: InterpFilter = 4;
pub const SWITCHABLE_FILTERS: InterpFilter = 3;
pub const INTERP_FILTERS_ALL: InterpFilter = 5;
pub const MULTITAP_SHARP2: InterpFilter = 4;
pub const BILINEAR: InterpFilter = 3;
pub const MULTITAP_SHARP: InterpFilter = 2;
pub const EIGHTTAP_SMOOTH: InterpFilter = 1;
pub const EIGHTTAP_REGULAR: InterpFilter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CurrentFrame {
    pub frame_type: FRAME_TYPE,
    pub reference_mode: REFERENCE_MODE,
    pub order_hint: core::ffi::c_uint,
    pub display_order_hint: core::ffi::c_uint,
    pub pyramid_level: core::ffi::c_uint,
    pub frame_number: core::ffi::c_uint,
    pub skip_mode_info: SkipModeInfo,
    pub refresh_frame_flags: core::ffi::c_int,
    pub frame_refs_short_signaling: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkipModeInfo {
    pub skip_mode_allowed: core::ffi::c_int,
    pub skip_mode_flag: core::ffi::c_int,
    pub ref_frame_idx_0: core::ffi::c_int,
    pub ref_frame_idx_1: core::ffi::c_int,
}
pub type REFERENCE_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub is_chroma_ref: bool,
    pub plane: [macroblockd_plane; 3],
    pub tile: TileInfo,
    pub mi: *mut *mut MB_MODE_INFO,
    pub up_available: bool,
    pub left_available: bool,
    pub chroma_up_available: bool,
    pub chroma_left_available: bool,
    pub left_mbmi: *mut MB_MODE_INFO,
    pub above_mbmi: *mut MB_MODE_INFO,
    pub chroma_left_mbmi: *mut MB_MODE_INFO,
    pub chroma_above_mbmi: *mut MB_MODE_INFO,
    pub tx_type_map: *mut uint8_t,
    pub tx_type_map_stride: core::ffi::c_int,
    pub mb_to_left_edge: core::ffi::c_int,
    pub mb_to_right_edge: core::ffi::c_int,
    pub mb_to_top_edge: core::ffi::c_int,
    pub mb_to_bottom_edge: core::ffi::c_int,
    pub block_ref_scale_factors: [*const scale_factors; 2],
    pub cur_buf: *const YV12_BUFFER_CONFIG,
    pub above_entropy_context: [*mut ENTROPY_CONTEXT; 3],
    pub left_entropy_context: [[ENTROPY_CONTEXT; 32]; 3],
    pub above_partition_context: *mut PARTITION_CONTEXT,
    pub left_partition_context: [PARTITION_CONTEXT; 32],
    pub above_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context_buffer: [TXFM_CONTEXT; 32],
    pub wiener_info: [WienerInfo; 3],
    pub sgrproj_info: [SgrprojInfo; 3],
    pub width: uint8_t,
    pub height: uint8_t,
    pub ref_mv_stack: [[CANDIDATE_MV; 8]; 29],
    pub weight: [[uint16_t; 8]; 29],
    pub is_last_vertical_rect: bool,
    pub is_first_horizontal_rect: bool,
    pub neighbors_ref_counts: [uint8_t; 8],
    pub tile_ctx: *mut FRAME_CONTEXT,
    pub bd: core::ffi::c_int,
    pub qindex: [core::ffi::c_int; 8],
    pub lossless: [core::ffi::c_int; 8],
    pub current_base_qindex: core::ffi::c_int,
    pub cur_frame_force_integer_mv: core::ffi::c_int,
    pub error_info: *mut aom_internal_error_info,
    pub global_motion: *const WarpedMotionParams,
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    pub cdef_transmitted: [bool; 4],
    pub seg_mask: *mut uint8_t,
    pub cfl: CFL_CTX,
    pub color_index_map_offset: [uint16_t; 2],
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub tmp_upsample_pred: *mut uint8_t,
}
pub type CONV_BUF_TYPE = uint16_t;
pub type CFL_CTX = cfl_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfl_ctx {
    pub recon_buf_q3: [uint16_t; 1024],
    pub ac_buf_q3: [int16_t; 1024],
    pub dc_pred_is_cached: [bool; 2],
    pub use_dc_pred_cache: bool,
    pub dc_pred_cache: [[int16_t; 32]; 2],
    pub buf_height: core::ffi::c_int,
    pub buf_width: core::ffi::c_int,
    pub are_parameters_computed: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub store_y: core::ffi::c_int,
}
pub type CANDIDATE_MV = candidate_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct candidate_mv {
    pub this_mv: int_mv,
    pub comp_mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileInfo {
    pub mi_row_start: core::ffi::c_int,
    pub mi_row_end: core::ffi::c_int,
    pub mi_col_start: core::ffi::c_int,
    pub mi_col_end: core::ffi::c_int,
    pub tile_row: core::ffi::c_int,
    pub tile_col: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd_plane {
    pub plane_type: PLANE_TYPE,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub dst: buf_2d,
    pub pre: [buf_2d; 2],
    pub above_entropy_context: *mut ENTROPY_CONTEXT,
    pub left_entropy_context: *mut ENTROPY_CONTEXT,
    pub seg_dequant_QTX: [[int16_t; 2]; 8],
    pub color_index_map: *mut uint8_t,
    pub width: uint8_t,
    pub height: uint8_t,
    pub seg_iqmatrix: [[*mut qm_val_t; 19]; 8],
    pub seg_qmatrix: [[*mut qm_val_t; 19]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_2d {
    pub buf: *mut uint8_t,
    pub buf0: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type pthread_cond_t = libc::pthread_cond_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FullMvLimits {
    pub col_min: core::ffi::c_int,
    pub col_max: core::ffi::c_int,
    pub row_min: core::ffi::c_int,
    pub row_max: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpelMvLimits {
    pub col_min: core::ffi::c_int,
    pub col_max: core::ffi::c_int,
    pub row_min: core::ffi::c_int,
    pub row_max: core::ffi::c_int,
}
pub type SUBPEL_SEARCH_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCAN_ORDER {
    pub scan: *const int16_t,
    pub iscan: *const int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_STATS {
    pub rate: core::ffi::c_int,
    pub zero_rate: core::ffi::c_int,
    pub dist: int64_t,
    pub rdcost: int64_t,
    pub sse: int64_t,
    pub skip_txfm: uint8_t,
}
pub type MACROBLOCKD_PLANE = macroblockd_plane;
pub type MACROBLOCKD = macroblockd;
pub type foreach_transformed_block_visitor = Option<
    unsafe extern "C-unwind" fn(
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        BLOCK_SIZE,
        TX_SIZE,
        *mut core::ffi::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationTileLimits {
    pub h_start: core::ffi::c_int,
    pub h_end: core::ffi::c_int,
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
}
pub type rest_unit_visitor_t = Option<
    unsafe extern "C-unwind" fn(
        *const RestorationTileLimits,
        core::ffi::c_int,
        *mut core::ffi::c_void,
        *mut int32_t,
        *mut RestorationLineBuffers,
        *mut aom_internal_error_info,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterFrameCtxt {
    pub rsi: *const RestorationInfo,
    pub ss_x: core::ffi::c_int,
    pub ss_y: core::ffi::c_int,
    pub plane_w: core::ffi::c_int,
    pub plane_h: core::ffi::c_int,
    pub highbd: core::ffi::c_int,
    pub bit_depth: core::ffi::c_int,
    pub data8: *mut uint8_t,
    pub dst8: *mut uint8_t,
    pub data_stride: core::ffi::c_int,
    pub dst_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrStruct {
    pub on_rest_unit: rest_unit_visitor_t,
    pub ctxt: [FilterFrameCtxt; 3],
    pub frame: *mut YV12_BUFFER_CONFIG,
    pub dst: *mut YV12_BUFFER_CONFIG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrSyncData {
    pub mutex_: [*mut pthread_mutex_t; 3],
    pub cond_: [*mut pthread_cond_t; 3],
    pub cur_sb_col: [*mut core::ffi::c_int; 3],
    pub sync_range: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub num_planes: core::ffi::c_int,
    pub num_workers: core::ffi::c_int,
    pub job_mutex: *mut pthread_mutex_t,
    pub lrworkerdata: *mut LRWorkerData,
    pub job_queue: *mut AV1LrMTInfo,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub lr_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrMTInfo {
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
    pub lr_unit_row: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub sync_mode: core::ffi::c_int,
    pub v_copy_start: core::ffi::c_int,
    pub v_copy_end: core::ffi::c_int,
}
pub type LRWorkerData = LoopRestorationWorkerData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopRestorationWorkerData {
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut core::ffi::c_void,
    pub lr_ctxt: *mut core::ffi::c_void,
    pub do_extend_border: core::ffi::c_int,
    pub error_info: aom_internal_error_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblock {
    pub plane: [macroblock_plane; 3],
    pub e_mbd: MACROBLOCKD,
    pub mbmi_ext: MB_MODE_INFO_EXT,
    pub mbmi_ext_frame: *mut MB_MODE_INFO_EXT_FRAME,
    pub row_ctx: *mut FRAME_CONTEXT,
    pub tile_pb_ctx: *mut FRAME_CONTEXT,
    pub cb_coef_buff: *mut CB_COEFF_BUFFER,
    pub cb_offset: [uint16_t; 2],
    pub obmc_buffer: OBMCBuffer,
    pub palette_buffer: *mut PALETTE_BUFFER,
    pub comp_rd_buffer: CompoundTypeRdBuffers,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_pred_bufs: [*mut uint8_t; 2],
    pub upsample_pred: *mut uint8_t,
    pub qindex: core::ffi::c_int,
    pub delta_qindex: core::ffi::c_int,
    pub rdmult_delta_qindex: core::ffi::c_int,
    pub rdmult_cur_qindex: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub intra_sb_rdmult_modifier: core::ffi::c_int,
    pub rb: core::ffi::c_double,
    pub mb_energy: core::ffi::c_int,
    pub sb_energy_level: core::ffi::c_int,
    pub mode_costs: ModeCosts,
    pub mv_costs: *mut MvCosts,
    pub dv_costs: *mut IntraBCMVCosts,
    pub coeff_costs: CoeffCosts,
    pub errorperbit: core::ffi::c_int,
    pub sadperbit: core::ffi::c_int,
    pub seg_skip_block: core::ffi::c_int,
    pub actual_num_seg1_blocks: core::ffi::c_int,
    pub actual_num_seg2_blocks: core::ffi::c_int,
    pub cnt_zeromv: core::ffi::c_int,
    pub force_zeromv_skip_for_sb: core::ffi::c_int,
    pub force_zeromv_skip_for_blk: core::ffi::c_int,
    pub prev_segment_id: core::ffi::c_int,
    pub sb_enc: SuperBlockEnc,
    pub content_state_sb: CONTENT_STATE_SB,
    pub pred_mv_sad: [core::ffi::c_int; 8],
    pub best_pred_mv_sad: [core::ffi::c_int; 2],
    pub pred_mv0_sad: [core::ffi::c_int; 8],
    pub pred_mv1_sad: [core::ffi::c_int; 8],
    pub tpl_keep_ref_frame: [uint8_t; 8],
    pub warp_sample_info: [WARP_SAMPLE_INFO; 8],
    pub picked_ref_frames_mask: [core::ffi::c_int; 1024],
    pub nonrd_prune_ref_frame_search: core::ffi::c_int,
    pub part_search_info: PartitionSearchInfo,
    pub must_find_valid_partition: core::ffi::c_int,
    pub skip_mode: core::ffi::c_int,
    pub thresh_freq_fact: [[core::ffi::c_int; 169]; 22],
    pub winner_mode_stats: *mut WinnerModeStats,
    pub winner_mode_count: core::ffi::c_int,
    pub rd_model: TXFM_RD_MODEL,
    pub inter_modes_info: *mut inter_modes_info,
    pub compound_idx: uint8_t,
    pub comp_rd_stats: [COMP_RD_STATS; 64],
    pub comp_rd_stats_idx: core::ffi::c_int,
    pub recalc_luma_mc_data: core::ffi::c_int,
    pub intrabc_hash_info: IntraBCHashInfo,
    pub use_mb_mode_cache: core::ffi::c_int,
    pub mb_mode_cache: *const MB_MODE_INFO,
    pub pixel_gradient_info: *mut PixelLevelGradientInfo,
    pub is_sb_gradient_cached: [bool; 2],
    pub reuse_inter_pred: bool,
    pub max_mv_context: [core::ffi::c_uint; 8],
    pub mv_limits: FullMvLimits,
    pub search_site_cfg_buf: [search_site_config; 6],
    pub txfm_search_params: TxfmSearchParams,
    pub txfm_search_info: TxfmSearchInfo,
    pub color_sensitivity_sb: [uint8_t; 2],
    pub color_sensitivity_sb_g: [uint8_t; 2],
    pub color_sensitivity_sb_alt: [uint8_t; 2],
    pub color_sensitivity: [uint8_t; 2],
    pub min_dist_inter_uv: int64_t,
    pub color_palette_thresh: core::ffi::c_int,
    pub force_color_check_block_level: core::ffi::c_int,
    pub dqcoeff_buf: *mut tran_low_t,
    pub source_variance: core::ffi::c_uint,
    pub block_is_zero_sad: core::ffi::c_int,
    pub sb_me_partition: core::ffi::c_int,
    pub sb_me_block: core::ffi::c_int,
    pub sb_col_scroll: core::ffi::c_int,
    pub sb_row_scroll: core::ffi::c_int,
    pub sb_me_mv: int_mv,
    pub sb_force_fixed_part: core::ffi::c_int,
    pub pred_sse: [core::ffi::c_uint; 8],
    pub try_merge_partition: core::ffi::c_int,
    pub src_var_info_of_4x4_sub_blocks: *mut Block4x4VarInfo,
    pub last_set_offsets_loc: SetOffsetsLoc,
    pub palette_pixels: core::ffi::c_int,
    pub top_inter_tx_no_split_rd: [[int64_t; 4]; 4],
    pub sb_stats_cache: *mut SB_FIRST_PASS_STATS,
    pub sb_fp_stats: *mut SB_FIRST_PASS_STATS,
    pub top_comp_avg_est_rd: [int64_t; 5],
    pub pix_to_bottom_edge: core::ffi::c_int,
    pub pix_to_right_edge: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetOffsetsLoc {
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub bsize: BLOCK_SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Block4x4VarInfo {
    pub log_var: core::ffi::c_double,
    pub var: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmSearchInfo {
    pub skip_txfm: uint8_t,
    pub blk_skip: [uint8_t; 1024],
    pub tx_type_map_: [uint8_t; 1024],
    pub mb_rd_record: *mut MB_RD_RECORD,
    pub txb_split_count: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_RD_RECORD {
    pub mb_rd_info: [MB_RD_INFO; 8],
    pub index_start: core::ffi::c_int,
    pub num: core::ffi::c_int,
    pub crc_calculator: CRC32C,
}
pub type CRC32C = _CRC32C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CRC32C {
    pub table: [[uint32_t; 256]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_RD_INFO {
    pub tx_size: TX_SIZE,
    pub inter_tx_size: [TX_SIZE; 16],
    pub blk_skip: [uint8_t; 1024],
    pub tx_type_map: [uint8_t; 1024],
    pub rd_stats: RD_STATS,
    pub hash_value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmSearchParams {
    pub use_default_intra_tx_type: core::ffi::c_int,
    pub use_derived_intra_tx_type_set: core::ffi::c_int,
    pub default_inter_tx_type_prob_thresh: core::ffi::c_int,
    pub prune_2d_txfm_mode: core::ffi::c_int,
    pub coeff_opt_thresholds: [core::ffi::c_uint; 2],
    pub tx_domain_dist_threshold: core::ffi::c_uint,
    pub tx_size_search_method: TX_SIZE_SEARCH_METHOD,
    pub use_transform_domain_distortion: core::ffi::c_uint,
    pub skip_txfm_level: core::ffi::c_uint,
    pub tx_mode_search_type: TX_MODE,
    pub predict_dc_level: core::ffi::c_uint,
    pub use_qm_dist_metric: core::ffi::c_int,
    pub mode_eval_type: core::ffi::c_int,
    pub nn_prune_depths_for_intra_tx: TX_PRUNE_TYPE,
    pub enable_nn_prune_intra_tx_depths: bool,
}
pub type TX_PRUNE_TYPE = uint8_t;
pub type TX_SIZE_SEARCH_METHOD = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_site_config {
    pub site: [[search_site; 17]; 22],
    pub num_search_steps: core::ffi::c_int,
    pub searches_per_step: [core::ffi::c_int; 22],
    pub radius: [core::ffi::c_int; 22],
    pub stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_site {
    pub mv: FULLPEL_MV,
    pub offset: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixelLevelGradientInfo {
    pub abs_dx_abs_dy_sum: uint16_t,
    pub hist_bin_idx: int8_t,
    pub is_dx_zero: bool,
}
pub type IntraBCHashInfo = intrabc_hash_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intrabc_hash_info {
    pub hash_value_buffer: [*mut uint32_t; 2],
    pub intrabc_hash_table: hash_table,
    pub crc_calculator: CRC32C,
    pub crc_initialized: core::ffi::c_int,
}
pub type hash_table = _hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hash_table {
    pub p_lookup_table: *mut *mut Vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    pub size: size_t,
    pub capacity: size_t,
    pub element_size: size_t,
    pub data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMP_RD_STATS {
    pub rate: [int32_t; 4],
    pub dist: [int64_t; 4],
    pub model_rate: [int32_t; 4],
    pub model_dist: [int64_t; 4],
    pub comp_rs2: [core::ffi::c_int; 4],
    pub mv: [int_mv; 2],
    pub ref_frames: [MV_REFERENCE_FRAME; 2],
    pub mode: PREDICTION_MODE,
    pub filter: int_interpfilters,
    pub ref_mv_idx: core::ffi::c_int,
    pub is_global: [core::ffi::c_int; 2],
    pub interinter_comp: INTERINTER_COMPOUND_DATA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inter_modes_info {
    pub num: core::ffi::c_int,
    pub mbmi_arr: [MB_MODE_INFO; 1024],
    pub mode_rate_arr: [core::ffi::c_int; 1024],
    pub sse_arr: [int64_t; 1024],
    pub est_rd_arr: [int64_t; 1024],
    pub rd_idx_pair_arr: [RdIdxPair; 1024],
    pub rd_cost_arr: [RD_STATS; 1024],
    pub rd_cost_y_arr: [RD_STATS; 1024],
    pub rd_cost_uv_arr: [RD_STATS; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RdIdxPair {
    pub idx: core::ffi::c_int,
    pub rd: int64_t,
}
pub type TXFM_RD_MODEL = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WinnerModeStats {
    pub mbmi: MB_MODE_INFO,
    pub rd_cost: RD_STATS,
    pub rd: int64_t,
    pub rate_y: core::ffi::c_int,
    pub rate_uv: core::ffi::c_int,
    pub color_index_map: [uint8_t; 16384],
    pub mode_index: THR_MODES,
}
pub type THR_MODES = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionSearchInfo {
    pub quad_tree_idx: core::ffi::c_int,
    pub cnn_output_valid: core::ffi::c_int,
    pub cnn_buffer: [core::ffi::c_float; 1636],
    pub log_q: core::ffi::c_float,
    pub variance_low: [uint8_t; 105],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WARP_SAMPLE_INFO {
    pub num: core::ffi::c_int,
    pub pts: [core::ffi::c_int; 16],
    pub pts_inref: [core::ffi::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONTENT_STATE_SB {
    pub source_sad_nonrd: SOURCE_SAD,
    pub source_sad_rd: SOURCE_SAD,
    pub lighting_change: core::ffi::c_int,
    pub low_sumdiff: core::ffi::c_int,
}
pub type SOURCE_SAD = core::ffi::c_uint;
pub const kHighSad: SOURCE_SAD = 4;
pub const kMedSad: SOURCE_SAD = 3;
pub const kLowSad: SOURCE_SAD = 2;
pub const kVeryLowSad: SOURCE_SAD = 1;
pub const kZeroSad: SOURCE_SAD = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SuperBlockEnc {
    pub min_partition_size: BLOCK_SIZE,
    pub max_partition_size: BLOCK_SIZE,
    pub tpl_data_count: core::ffi::c_int,
    pub tpl_inter_cost: [int64_t; 64],
    pub tpl_intra_cost: [int64_t; 64],
    pub tpl_mv: [[int_mv; 7]; 64],
    pub tpl_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoeffCosts {
    pub coeff_costs: [[LV_MAP_COEFF_COST; 2]; 5],
    pub eob_costs: [[LV_MAP_EOB_COST; 2]; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LV_MAP_EOB_COST {
    pub eob_cost: [[core::ffi::c_int; 11]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LV_MAP_COEFF_COST {
    pub txb_skip_cost: [[core::ffi::c_int; 2]; 13],
    pub base_eob_cost: [[core::ffi::c_int; 3]; 4],
    pub base_cost: [[core::ffi::c_int; 8]; 42],
    pub eob_extra_cost: [[core::ffi::c_int; 2]; 9],
    pub dc_sign_cost: [[core::ffi::c_int; 2]; 3],
    pub lps_cost: [[core::ffi::c_int; 26]; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntraBCMVCosts {
    pub joint_mv: [core::ffi::c_int; 4],
    pub dv_costs_alloc: [[core::ffi::c_int; 32767]; 2],
    pub dv_costs: [*mut core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvCosts {
    pub nmv_joint_cost: [core::ffi::c_int; 4],
    pub nmv_cost_alloc: [[core::ffi::c_int; 32767]; 2],
    pub nmv_cost_hp_alloc: [[core::ffi::c_int; 32767]; 2],
    pub nmv_cost: [*mut core::ffi::c_int; 2],
    pub nmv_cost_hp: [*mut core::ffi::c_int; 2],
    pub mv_cost_stack: *mut *mut core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModeCosts {
    pub partition_cost: [[core::ffi::c_int; 10]; 20],
    pub mbmode_cost: [[core::ffi::c_int; 13]; 4],
    pub y_mode_costs: [[[core::ffi::c_int; 13]; 13]; 13],
    pub intra_uv_mode_cost: [[[core::ffi::c_int; 14]; 13]; 2],
    pub filter_intra_cost: [[core::ffi::c_int; 2]; 22],
    pub filter_intra_mode_cost: [core::ffi::c_int; 5],
    pub angle_delta_cost: [[core::ffi::c_int; 7]; 8],
    pub cfl_cost: [[[core::ffi::c_int; 16]; 2]; 8],
    pub intrabc_cost: [core::ffi::c_int; 2],
    pub palette_y_size_cost: [[core::ffi::c_int; 7]; 7],
    pub palette_uv_size_cost: [[core::ffi::c_int; 7]; 7],
    pub palette_y_color_cost: [[[core::ffi::c_int; 8]; 5]; 7],
    pub palette_uv_color_cost: [[[core::ffi::c_int; 8]; 5]; 7],
    pub palette_y_mode_cost: [[[core::ffi::c_int; 2]; 3]; 7],
    pub palette_uv_mode_cost: [[core::ffi::c_int; 2]; 2],
    pub skip_mode_cost: [[core::ffi::c_int; 2]; 3],
    pub newmv_mode_cost: [[core::ffi::c_int; 2]; 6],
    pub zeromv_mode_cost: [[core::ffi::c_int; 2]; 2],
    pub refmv_mode_cost: [[core::ffi::c_int; 2]; 6],
    pub drl_mode_cost0: [[core::ffi::c_int; 2]; 3],
    pub single_ref_cost: [[[core::ffi::c_int; 2]; 6]; 3],
    pub comp_inter_cost: [[core::ffi::c_int; 2]; 5],
    pub comp_ref_type_cost: [[core::ffi::c_int; 3]; 5],
    pub uni_comp_ref_cost: [[[core::ffi::c_int; 3]; 3]; 3],
    pub comp_ref_cost: [[[core::ffi::c_int; 2]; 3]; 3],
    pub comp_bwdref_cost: [[[core::ffi::c_int; 2]; 2]; 3],
    pub intra_inter_cost: [[core::ffi::c_int; 2]; 4],
    pub inter_compound_mode_cost: [[core::ffi::c_int; 8]; 8],
    pub compound_type_cost: [[core::ffi::c_int; 2]; 22],
    pub wedge_idx_cost: [[core::ffi::c_int; 16]; 22],
    pub interintra_cost: [[core::ffi::c_int; 2]; 4],
    pub wedge_interintra_cost: [[core::ffi::c_int; 2]; 22],
    pub interintra_mode_cost: [[core::ffi::c_int; 4]; 4],
    pub comp_idx_cost: [[core::ffi::c_int; 2]; 6],
    pub comp_group_idx_cost: [[core::ffi::c_int; 2]; 6],
    pub motion_mode_cost: [[core::ffi::c_int; 3]; 22],
    pub motion_mode_cost1: [[core::ffi::c_int; 2]; 22],
    pub switchable_interp_costs: [[core::ffi::c_int; 3]; 16],
    pub skip_txfm_cost: [[core::ffi::c_int; 2]; 3],
    pub tx_size_cost: [[[core::ffi::c_int; 5]; 3]; 4],
    pub txfm_partition_cost: [[core::ffi::c_int; 2]; 21],
    pub inter_tx_type_costs: [[[core::ffi::c_int; 16]; 4]; 4],
    pub intra_tx_type_costs: [[[[core::ffi::c_int; 16]; 13]; 4]; 3],
    pub switchable_restore_cost: [core::ffi::c_int; 3],
    pub wiener_restore_cost: [core::ffi::c_int; 2],
    pub sgrproj_restore_cost: [core::ffi::c_int; 2],
    pub tmp_pred_cost: [[core::ffi::c_int; 2]; 3],
    pub spatial_pred_cost: [[core::ffi::c_int; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompoundTypeRdBuffers {
    pub pred0: *mut uint8_t,
    pub pred1: *mut uint8_t,
    pub residual1: *mut int16_t,
    pub diff10: *mut int16_t,
    pub tmp_best_mask_buf: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTE_BUFFER {
    pub best_palette_color_map: [uint8_t; 4096],
    pub kmeans_data_buf: [int16_t; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBMCBuffer {
    pub wsrc: *mut int32_t,
    pub mask: *mut int32_t,
    pub above_pred: *mut uint8_t,
    pub left_pred: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CB_COEFF_BUFFER {
    pub tcoeff: [*mut tran_low_t; 3],
    pub eobs: [*mut uint16_t; 3],
    pub entropy_ctx: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_MODE_INFO_EXT_FRAME {
    pub ref_mv_stack: [CANDIDATE_MV; 4],
    pub weight: [uint16_t; 4],
    pub ref_mv_count: uint8_t,
    pub global_mvs: [int_mv; 8],
    pub mode_context: int16_t,
    pub cb_offset: [uint16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_MODE_INFO_EXT {
    pub ref_mv_stack: [[CANDIDATE_MV; 4]; 29],
    pub weight: [[uint16_t; 4]; 29],
    pub ref_mv_count: [uint8_t; 29],
    pub global_mvs: [int_mv; 8],
    pub mode_context: [int16_t; 29],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblock_plane {
    pub src_diff: *mut int16_t,
    pub dqcoeff: *mut tran_low_t,
    pub qcoeff: *mut tran_low_t,
    pub coeff: *mut tran_low_t,
    pub eobs: *mut uint16_t,
    pub txb_entropy_ctx: *mut uint8_t,
    pub src: buf_2d,
    pub quant_fp_QTX: *const int16_t,
    pub round_fp_QTX: *const int16_t,
    pub quant_QTX: *const int16_t,
    pub round_QTX: *const int16_t,
    pub quant_shift_QTX: *const int16_t,
    pub zbin_QTX: *const int16_t,
    pub dequant_QTX: *const int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_variance_vtable {
    pub sdf: aom_sad_fn_t,
    pub sdsf: aom_sad_fn_t,
    pub sdaf: aom_sad_avg_fn_t,
    pub vf: aom_variance_fn_t,
    pub svf: aom_subpixvariance_fn_t,
    pub svaf: aom_subp_avg_variance_fn_t,
    pub sdx4df: aom_sad_multi_d_fn_t,
    pub sdx3df: aom_sad_multi_d_fn_t,
    pub sdsx4df: aom_sad_multi_d_fn_t,
    pub msdf: aom_masked_sad_fn_t,
    pub msvf: aom_masked_subpixvariance_fn_t,
    pub osdf: aom_obmc_sad_fn_t,
    pub ovf: aom_obmc_variance_fn_t,
    pub osvf: aom_obmc_subpixvariance_fn_t,
}
pub type aom_obmc_subpixvariance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *const int32_t,
        *const int32_t,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
>;
pub type aom_obmc_variance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const int32_t,
        *const int32_t,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
>;
pub type aom_obmc_sad_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const int32_t,
        *const int32_t,
    ) -> core::ffi::c_uint,
>;
pub type aom_masked_subpixvariance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
>;
pub type aom_masked_sad_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_uint,
>;
pub type aom_sad_multi_d_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const *const uint8_t,
        core::ffi::c_int,
        *mut core::ffi::c_uint,
    ) -> (),
>;
pub type aom_subp_avg_variance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut core::ffi::c_uint,
        *const uint8_t,
    ) -> core::ffi::c_uint,
>;
pub type aom_subpixvariance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
>;
pub type aom_variance_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
>;
pub type aom_sad_avg_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
    ) -> core::ffi::c_uint,
>;
pub type aom_sad_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
    ) -> core::ffi::c_uint,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NN_CONFIG {
    pub num_inputs: core::ffi::c_int,
    pub num_outputs: core::ffi::c_int,
    pub num_hidden_layers: core::ffi::c_int,
    pub num_hidden_nodes: [core::ffi::c_int; 10],
    pub weights: [*const core::ffi::c_float; 11],
    pub bias: [*const core::ffi::c_float; 11],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorker {
    pub impl_: *mut AVxWorkerImpl,
    pub status_: AVxWorkerStatus,
    pub thread_name: *const core::ffi::c_char,
    pub hook: AVxWorkerHook,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
    pub had_error: core::ffi::c_int,
}
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
pub type aom_ext_part_model_t = *mut core::ffi::c_void;
pub type aom_ext_part_decision_mode = core::ffi::c_uint;
pub const AOM_EXT_PART_RECURSIVE: aom_ext_part_decision_mode = 1;
pub const AOM_EXT_PART_WHOLE_TREE: aom_ext_part_decision_mode = 0;
pub type aom_ext_part_decision_mode_t = aom_ext_part_decision_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_ext_part_config {
    pub superblock_size: core::ffi::c_int,
}
pub type aom_ext_part_config_t = aom_ext_part_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_before_none {
    pub f: [core::ffi::c_float; 17],
    pub f_part2: [core::ffi::c_float; 25],
}
pub type aom_partition_features_before_none_t = aom_partition_features_before_none;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_none {
    pub f: [core::ffi::c_float; 4],
    pub f_terminate: [core::ffi::c_float; 28],
}
pub type aom_partition_features_none_t = aom_partition_features_none;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_split {
    pub f_terminate: [core::ffi::c_float; 31],
    pub f_prune_rect: [core::ffi::c_float; 9],
}
pub type aom_partition_features_split_t = aom_partition_features_split;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_rect {
    pub f: [core::ffi::c_float; 10],
}
pub type aom_partition_features_rect_t = aom_partition_features_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_ab {
    pub f: [core::ffi::c_float; 18],
}
pub type aom_partition_features_ab_t = aom_partition_features_ab;
pub type AOM_EXT_PART_FEATURE_ID = core::ffi::c_uint;
pub const AOM_EXT_PART_FEATURE_AFTER_AB: AOM_EXT_PART_FEATURE_ID = 7;
pub const AOM_EXT_PART_FEATURE_AFTER_RECT: AOM_EXT_PART_FEATURE_ID = 6;
pub const AOM_EXT_PART_FEATURE_AFTER_SPLIT_PART2: AOM_EXT_PART_FEATURE_ID = 5;
pub const AOM_EXT_PART_FEATURE_AFTER_SPLIT: AOM_EXT_PART_FEATURE_ID = 4;
pub const AOM_EXT_PART_FEATURE_AFTER_NONE_PART2: AOM_EXT_PART_FEATURE_ID = 3;
pub const AOM_EXT_PART_FEATURE_AFTER_NONE: AOM_EXT_PART_FEATURE_ID = 2;
pub const AOM_EXT_PART_FEATURE_BEFORE_NONE_PART2: AOM_EXT_PART_FEATURE_ID = 1;
pub const AOM_EXT_PART_FEATURE_BEFORE_NONE: AOM_EXT_PART_FEATURE_ID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_tpl_features {
    pub available: core::ffi::c_int,
    pub tpl_unit_length: core::ffi::c_int,
    pub num_units: core::ffi::c_int,
    pub intra_cost: [int64_t; 64],
    pub inter_cost: [int64_t; 64],
    pub mc_dep_cost: [int64_t; 64],
}
pub type aom_sb_tpl_features_t = aom_sb_tpl_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_simple_motion_features {
    pub unit_length: core::ffi::c_int,
    pub num_units: core::ffi::c_int,
    pub block_sse: [core::ffi::c_int; 64],
    pub block_var: [core::ffi::c_int; 64],
}
pub type aom_sb_simple_motion_features_t = aom_sb_simple_motion_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_features {
    pub motion_features: aom_sb_simple_motion_features_t,
    pub tpl_features: aom_sb_tpl_features_t,
}
pub type aom_sb_features_t = aom_sb_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features {
    pub id: AOM_EXT_PART_FEATURE_ID,
    pub before_part_none: aom_partition_features_before_none_t,
    pub after_part_none: aom_partition_features_none_t,
    pub after_part_split: aom_partition_features_split_t,
    pub after_part_rect: aom_partition_features_rect_t,
    pub after_part_ab: aom_partition_features_ab_t,
    pub sb_features: aom_sb_features_t,
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub frame_width: core::ffi::c_int,
    pub frame_height: core::ffi::c_int,
    pub block_size: core::ffi::c_int,
    pub valid_partition_types: core::ffi::c_int,
    pub update_type: core::ffi::c_int,
    pub qindex: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub pyramid_level: core::ffi::c_int,
    pub has_above_block: core::ffi::c_int,
    pub above_block_width: core::ffi::c_int,
    pub above_block_height: core::ffi::c_int,
    pub has_left_block: core::ffi::c_int,
    pub left_block_width: core::ffi::c_int,
    pub left_block_height: core::ffi::c_int,
    pub block_sse: core::ffi::c_uint,
    pub block_var: core::ffi::c_uint,
    pub sub_block_sse: [core::ffi::c_uint; 4],
    pub sub_block_var: [core::ffi::c_uint; 4],
    pub horz_block_sse: [core::ffi::c_uint; 2],
    pub horz_block_var: [core::ffi::c_uint; 2],
    pub vert_block_sse: [core::ffi::c_uint; 2],
    pub vert_block_var: [core::ffi::c_uint; 2],
    pub tpl_intra_cost: int64_t,
    pub tpl_inter_cost: int64_t,
    pub tpl_mc_dep_cost: int64_t,
}
pub type aom_partition_features_t = aom_partition_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_decision {
    pub is_final_decision: core::ffi::c_int,
    pub num_nodes: core::ffi::c_int,
    pub partition_decision: [core::ffi::c_int; 2048],
    pub current_decision: core::ffi::c_int,
    pub terminate_partition_search: core::ffi::c_int,
    pub partition_none_allowed: core::ffi::c_int,
    pub partition_rect_allowed: [core::ffi::c_int; 2],
    pub do_rectangular_split: core::ffi::c_int,
    pub do_square_split: core::ffi::c_int,
    pub prune_rect_part: [core::ffi::c_int; 2],
    pub horza_partition_allowed: core::ffi::c_int,
    pub horzb_partition_allowed: core::ffi::c_int,
    pub verta_partition_allowed: core::ffi::c_int,
    pub vertb_partition_allowed: core::ffi::c_int,
    pub partition_horz4_allowed: core::ffi::c_int,
    pub partition_vert4_allowed: core::ffi::c_int,
}
pub type aom_partition_decision_t = aom_partition_decision;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_stats {
    pub rate: core::ffi::c_int,
    pub dist: int64_t,
    pub rdcost: int64_t,
}
pub type aom_partition_stats_t = aom_partition_stats;
pub type aom_ext_part_status = core::ffi::c_uint;
pub const AOM_EXT_PART_TEST: aom_ext_part_status = 2;
pub const AOM_EXT_PART_ERROR: aom_ext_part_status = 1;
pub const AOM_EXT_PART_OK: aom_ext_part_status = 0;
pub type aom_ext_part_status_t = aom_ext_part_status;
pub type aom_ext_part_create_model_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *const aom_ext_part_config_t,
        *mut aom_ext_part_model_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_send_features_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *const aom_partition_features_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_get_decision_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *mut aom_partition_decision_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_send_partition_stats_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *const aom_partition_stats_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_delete_model_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_ext_part_model_t) -> aom_ext_part_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_ext_part_funcs {
    pub create_model: aom_ext_part_create_model_fn_t,
    pub send_features: aom_ext_part_send_features_fn_t,
    pub get_partition_decision: aom_ext_part_get_decision_fn_t,
    pub send_partition_stats: aom_ext_part_send_partition_stats_fn_t,
    pub delete_model: aom_ext_part_delete_model_fn_t,
    pub decision_mode: aom_ext_part_decision_mode_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_ext_part_funcs_t = aom_ext_part_funcs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_pkt_list {
    pub cnt: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
    pub pkts: [aom_codec_cx_pkt; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefWorker {
    pub cm: *mut AV1_COMMON,
    pub xd: *mut MACROBLOCKD,
    pub colbuf: [*mut uint16_t; 3],
    pub srcbuf: *mut uint16_t,
    pub linebuf: [*mut uint16_t; 3],
    pub cdef_init_fb_row_fn: cdef_init_fb_row_t,
    pub do_extend_border: core::ffi::c_int,
    pub error_info: aom_internal_error_info,
}
pub type cdef_init_fb_row_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *const MACROBLOCKD,
        *mut CdefBlockInfo,
        *mut *mut uint16_t,
        *mut uint16_t,
        *mut AV1CdefSyncData,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cdef_row_mt: *mut AV1CdefRowSync,
    pub end_of_frame: core::ffi::c_int,
    pub fbr: core::ffi::c_int,
    pub fbc: core::ffi::c_int,
    pub cdef_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefRowSync {
    pub row_mutex_: *mut pthread_mutex_t,
    pub row_cond_: *mut pthread_cond_t,
    pub is_row_done: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefBlockInfo {
    pub src: *mut uint16_t,
    pub top_linebuf: [*mut uint16_t; 3],
    pub bot_linebuf: [*mut uint16_t; 3],
    pub dst: *mut uint8_t,
    pub dlist: [cdef_list; 256],
    pub xdec: core::ffi::c_int,
    pub ydec: core::ffi::c_int,
    pub mi_wide_l2: core::ffi::c_int,
    pub mi_high_l2: core::ffi::c_int,
    pub frame_boundary: [core::ffi::c_int; 4],
    pub damping: core::ffi::c_int,
    pub coeff_shift: core::ffi::c_int,
    pub level: core::ffi::c_int,
    pub sec_strength: core::ffi::c_int,
    pub cdef_count: core::ffi::c_int,
    pub dir: [[core::ffi::c_int; 16]; 16],
    pub var: [[core::ffi::c_int; 16]; 16],
    pub dst_stride: core::ffi::c_int,
    pub coffset: core::ffi::c_int,
    pub roffset: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdef_list {
    pub by: uint8_t,
    pub bx: uint8_t,
}
pub type AV1_COMMON = AV1Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_DEBLOCKING_PARAMETERS {
    pub filter_length: uint8_t,
    pub lfthr: *const loop_filter_thresh,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterWorkerData {
    pub frame_buffer: *mut YV12_BUFFER_CONFIG,
    pub cm: *mut AV1Common,
    pub planes: [macroblockd_plane; 3],
    pub xd: *mut MACROBLOCKD,
    pub params_buf: [AV1_DEBLOCKING_PARAMETERS; 32],
    pub tx_buf: [TX_SIZE; 32],
    pub error_info: aom_internal_error_info,
}
pub type LFWorkerData = LoopFilterWorkerData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LfSyncData {
    pub mutex_: [*mut pthread_mutex_t; 3],
    pub cond_: [*mut pthread_cond_t; 3],
    pub cur_sb_col: [*mut core::ffi::c_int; 3],
    pub sync_range: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub lfdata: *mut LFWorkerData,
    pub num_workers: core::ffi::c_int,
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut AV1LfMTInfo,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub lf_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LfMTInfo {
    pub mi_row: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub dir: core::ffi::c_int,
    pub lpf_opt_level: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_table_entry_t {
    pub params: aom_film_grain_t,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub next: *mut aom_film_grain_table_entry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_table_t {
    pub head: *mut aom_film_grain_table_entry_t,
    pub tail: *mut aom_film_grain_table_entry_t,
}
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const LOW_TXFM_RD: C2RustUnnamed_21 = 1;
pub const FULL_TXFM_RD: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const USE_LARGESTALL: C2RustUnnamed_22 = 2;
pub const USE_FAST_RD: C2RustUnnamed_22 = 1;
pub const USE_FULL_RD: C2RustUnnamed_22 = 0;
pub type MODE = uint8_t;
pub type SEARCH_METHODS = uint8_t;
pub type WARP_SEARCH_METHOD = uint8_t;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const TX_PRUNE_SPLIT: C2RustUnnamed_23 = 2;
pub const TX_PRUNE_LARGEST: C2RustUnnamed_23 = 1;
pub const TX_PRUNE_NONE: C2RustUnnamed_23 = 0;
pub type MACROBLOCK = macroblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type AOM_FULLPEL_MV = aom_fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type AOM_MV = aom_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub union aom_int_mv {
    pub as_int: uint32_t,
    pub as_mv: AOM_MV,
    pub as_fullmv: AOM_FULLPEL_MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AomTplBlockStats {
    pub row: int16_t,
    pub col: int16_t,
    pub srcrf_sse: int64_t,
    pub srcrf_dist: int64_t,
    pub recrf_sse: int64_t,
    pub recrf_dist: int64_t,
    pub intra_sse: int64_t,
    pub intra_dist: int64_t,
    pub cmp_recrf_dist: [int64_t; 2],
    pub mc_dep_rate: int64_t,
    pub mc_dep_dist: int64_t,
    pub pred_error: [int64_t; 7],
    pub intra_cost: int32_t,
    pub inter_cost: int32_t,
    pub srcrf_rate: int32_t,
    pub recrf_rate: int32_t,
    pub intra_rate: int32_t,
    pub cmp_recrf_rate: [int32_t; 2],
    pub mv: [aom_int_mv; 7],
    pub ref_frame_index: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AomTplFrameStats {
    pub frame_width: core::ffi::c_int,
    pub frame_height: core::ffi::c_int,
    pub num_blocks: core::ffi::c_int,
    pub block_stats_list: *mut AomTplBlockStats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AomTplGopStats {
    pub size: core::ffi::c_int,
    pub frame_stats_list: *mut AomTplFrameStats,
}
pub type aom_rc_type = core::ffi::c_uint;
pub const AOM_RC_GOP_QP_RDMULT: aom_rc_type = 7;
pub const AOM_RC_GOP_QP: aom_rc_type = 3;
pub const AOM_RC_RDMULT: aom_rc_type = 4;
pub const AOM_RC_GOP: aom_rc_type = 2;
pub const AOM_RC_QP: aom_rc_type = 1;
pub const AOM_RC_NONE: aom_rc_type = 0;
pub type aom_rc_type_t = aom_rc_type;
pub type aom_ext_rc_mode = core::ffi::c_uint;
pub const AOM_RC_CQ: aom_ext_rc_mode = 2;
pub const AOM_RC_VBR: aom_ext_rc_mode = 1;
pub const AOM_RC_QMODE: aom_ext_rc_mode = 0;
pub type aom_ext_rc_mode_t = aom_ext_rc_mode;
pub type aom_rc_frame_update_type = core::ffi::c_int;
pub const AOM_RC_INTNL_ARF_UPDATE: aom_rc_frame_update_type = 6;
pub const AOM_RC_INTNL_OVERLAY_UPDATE: aom_rc_frame_update_type = 5;
pub const AOM_RC_OVERLAY_UPDATE: aom_rc_frame_update_type = 4;
pub const AOM_RC_ARF_UPDATE: aom_rc_frame_update_type = 3;
pub const AOM_RC_GF_UPDATE: aom_rc_frame_update_type = 2;
pub const AOM_RC_LF_UPDATE: aom_rc_frame_update_type = 1;
pub const AOM_RC_KF_UPDATE: aom_rc_frame_update_type = 0;
pub const AOM_RC_INVALID_UPDATE_TYPE: aom_rc_frame_update_type = -1;
pub type aom_rc_frame_update_type_t = aom_rc_frame_update_type;
pub type aom_rc_ref_name = core::ffi::c_int;
pub const AOM_RC_ALTREF_FRAME: aom_rc_ref_name = 7;
pub const AOM_RC_ALTREF2_FRAME: aom_rc_ref_name = 6;
pub const AOM_RC_BWDREF_FRAME: aom_rc_ref_name = 5;
pub const AOM_RC_GOLDEN_FRAME: aom_rc_ref_name = 4;
pub const AOM_RC_LAST3_FRAME: aom_rc_ref_name = 3;
pub const AOM_RC_LAST2_FRAME: aom_rc_ref_name = 2;
pub const AOM_RC_LAST_FRAME: aom_rc_ref_name = 1;
pub const AOM_RC_INTRA_FRAME: aom_rc_ref_name = 0;
pub const AOM_RC_INVALID_REF_FRAME: aom_rc_ref_name = -1;
pub type aom_rc_ref_name_t = aom_rc_ref_name;
pub type aom_rc_model_t = *mut core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_parameters {
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
}
pub type aom_sb_params = aom_sb_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_encodeframe_decision {
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub use_delta_q: *mut core::ffi::c_int,
    pub sb_params_list: *mut aom_sb_params,
}
pub type aom_rc_encodeframe_decision_t = aom_rc_encodeframe_decision;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_encodeframe_info {
    pub frame_type: core::ffi::c_int,
    pub show_index: core::ffi::c_int,
    pub coding_index: core::ffi::c_int,
    pub gop_index: core::ffi::c_int,
    pub ref_frame_coding_indexes: [core::ffi::c_int; 8],
    pub ref_frame_valid_list: [core::ffi::c_int; 8],
    pub gop_size: core::ffi::c_int,
    pub use_alt_ref: core::ffi::c_int,
}
pub type aom_rc_encodeframe_info_t = aom_rc_encodeframe_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_encodeframe_result {
    pub bit_count: int64_t,
    pub actual_encoding_qindex: core::ffi::c_int,
}
pub type aom_rc_encodeframe_result_t = aom_rc_encodeframe_result;
pub type aom_rc_status = core::ffi::c_uint;
pub const AOM_RC_ERROR: aom_rc_status = 1;
pub const AOM_RC_OK: aom_rc_status = 0;
pub type aom_rc_status_t = aom_rc_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_frame_stats {
    pub frame: core::ffi::c_double,
    pub weight: core::ffi::c_double,
    pub intra_error: core::ffi::c_double,
    pub frame_avg_wavelet_energy: core::ffi::c_double,
    pub coded_error: core::ffi::c_double,
    pub sr_coded_error: core::ffi::c_double,
    pub pcnt_inter: core::ffi::c_double,
    pub pcnt_motion: core::ffi::c_double,
    pub pcnt_second_ref: core::ffi::c_double,
    pub pcnt_neutral: core::ffi::c_double,
    pub intra_skip_pct: core::ffi::c_double,
    pub inactive_zone_rows: core::ffi::c_double,
    pub inactive_zone_cols: core::ffi::c_double,
    pub MVr: core::ffi::c_double,
    pub mvr_abs: core::ffi::c_double,
    pub MVc: core::ffi::c_double,
    pub mvc_abs: core::ffi::c_double,
    pub MVrv: core::ffi::c_double,
    pub MVcv: core::ffi::c_double,
    pub mv_in_out_count: core::ffi::c_double,
    pub new_mv_count: core::ffi::c_double,
    pub duration: core::ffi::c_double,
    pub count: core::ffi::c_double,
    pub raw_error_stdev: core::ffi::c_double,
    pub is_flash: int64_t,
    pub noise_var: core::ffi::c_double,
    pub cor_coeff: core::ffi::c_double,
    pub log_intra_error: core::ffi::c_double,
    pub log_coded_error: core::ffi::c_double,
}
pub type aom_rc_frame_stats_t = aom_rc_frame_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_firstpass_stats {
    pub frame_stats: *mut aom_rc_frame_stats_t,
    pub num_frames: core::ffi::c_int,
}
pub type aom_rc_firstpass_stats_t = aom_rc_firstpass_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_config {
    pub frame_width: core::ffi::c_int,
    pub frame_height: core::ffi::c_int,
    pub show_frame_count: core::ffi::c_int,
    pub max_gf_interval: core::ffi::c_int,
    pub min_gf_interval: core::ffi::c_int,
    pub target_bitrate_kbps: core::ffi::c_int,
    pub frame_rate_num: core::ffi::c_int,
    pub frame_rate_den: core::ffi::c_int,
    pub rc_mode: aom_ext_rc_mode_t,
    pub overshoot_percent: core::ffi::c_int,
    pub undershoot_percent: core::ffi::c_int,
    pub min_base_q_index: core::ffi::c_int,
    pub max_base_q_index: core::ffi::c_int,
    pub base_qp: core::ffi::c_int,
    pub superblock_size: core::ffi::c_int,
}
pub type aom_rc_config_t = aom_rc_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_ref_frame {
    pub index: [core::ffi::c_int; 8],
    pub name: [aom_rc_ref_name_t; 8],
}
pub type aom_rc_ref_frame_t = aom_rc_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_single_ref_frame {
    pub index: core::ffi::c_int,
    pub name: aom_rc_ref_name_t,
}
pub type aom_rc_single_ref_frame_t = aom_rc_single_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_gop_frame {
    pub order_idx: core::ffi::c_int,
    pub coding_idx: core::ffi::c_int,
    pub display_idx: core::ffi::c_int,
    pub order_hint: core::ffi::c_int,
    pub global_order_idx: core::ffi::c_int,
    pub global_coding_idx: core::ffi::c_int,
    pub is_key_frame: core::ffi::c_int,
    pub update_type: aom_rc_frame_update_type_t,
    pub colocated_ref_idx: core::ffi::c_int,
    pub update_ref_idx: core::ffi::c_int,
    pub extra_refresh_frame_flags: core::ffi::c_int,
    pub ref_frame_list: aom_rc_ref_frame_t,
    pub layer_depth: core::ffi::c_int,
    pub primary_ref_frame: aom_rc_single_ref_frame_t,
}
pub type aom_rc_gop_frame_t = aom_rc_gop_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_gop_decision {
    pub show_frame_count: core::ffi::c_int,
    pub order_hint_offset: core::ffi::c_int,
    pub global_order_idx_offset: core::ffi::c_int,
    pub global_coding_idx_offset: core::ffi::c_int,
    pub is_scene_cut: core::ffi::c_int,
    pub base_q_ratio: core::ffi::c_double,
    pub gop_frame_list: *mut aom_rc_gop_frame_t,
    pub gop_frame_count: core::ffi::c_int,
}
pub type aom_rc_gop_decision_t = aom_rc_gop_decision;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_key_frame_decision {
    pub key_frame_show_index: core::ffi::c_int,
    pub key_frame_group_size: core::ffi::c_int,
}
pub type aom_rc_key_frame_decision_t = aom_rc_key_frame_decision;
pub type aom_rc_create_model_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *const aom_rc_config_t,
        *mut aom_rc_model_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_send_firstpass_stats_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_firstpass_stats_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_send_tpl_gop_stats_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t, *const AomTplGopStats) -> aom_rc_status_t,
>;
pub type aom_rc_get_encodeframe_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        core::ffi::c_int,
        *mut aom_rc_encodeframe_decision_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_update_encodeframe_result_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_encodeframe_result_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_get_key_frame_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *mut aom_rc_key_frame_decision_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_get_gop_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t, *mut aom_rc_gop_decision_t) -> aom_rc_status_t,
>;
pub type aom_rc_get_frame_rdmult_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_encodeframe_info_t,
        *mut core::ffi::c_int,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_delete_model_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t) -> aom_rc_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_funcs {
    pub rc_type: aom_rc_type_t,
    pub create_model: aom_rc_create_model_cb_fn_t,
    pub send_firstpass_stats: aom_rc_send_firstpass_stats_cb_fn_t,
    pub send_tpl_gop_stats: aom_rc_send_tpl_gop_stats_cb_fn_t,
    pub get_encodeframe_decision: aom_rc_get_encodeframe_decision_cb_fn_t,
    pub update_encodeframe_result: aom_rc_update_encodeframe_result_cb_fn_t,
    pub get_key_frame_decision: aom_rc_get_key_frame_decision_cb_fn_t,
    pub get_gop_decision: aom_rc_get_gop_decision_cb_fn_t,
    pub get_frame_rdmult: aom_rc_get_frame_rdmult_cb_fn_t,
    pub delete_model: aom_rc_delete_model_cb_fn_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_rc_funcs_t = aom_rc_funcs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_roi_map {
    pub enabled: uint8_t,
    pub roi_map: *mut core::ffi::c_uchar,
    pub rows: core::ffi::c_uint,
    pub cols: core::ffi::c_uint,
    pub delta_q: [core::ffi::c_int; 8],
    pub delta_lf: [core::ffi::c_int; 8],
    pub skip: [core::ffi::c_int; 8],
    pub ref_frame: [core::ffi::c_int; 8],
    pub delta_qp_enabled: core::ffi::c_int,
    pub reference_enabled: core::ffi::c_int,
    pub rdmult_delta_qp: core::ffi::c_int,
}
pub type aom_roi_map_t = aom_roi_map;
pub type aom_tune_content = core::ffi::c_uint;
pub const AOM_CONTENT_INVALID: aom_tune_content = 3;
pub const AOM_CONTENT_FILM: aom_tune_content = 2;
pub const AOM_CONTENT_SCREEN: aom_tune_content = 1;
pub const AOM_CONTENT_DEFAULT: aom_tune_content = 0;
pub type aom_screen_detection_mode = core::ffi::c_uint;
pub const AOM_SCREEN_DETECTION_ANTIALIASING_AWARE: aom_screen_detection_mode = 2;
pub const AOM_SCREEN_DETECTION_STANDARD: aom_screen_detection_mode = 1;
pub type aom_tune_metric = core::ffi::c_uint;
pub const AOM_TUNE_SSIMULACRA2: aom_tune_metric = 11;
pub const AOM_TUNE_IQ: aom_tune_metric = 10;
pub const AOM_TUNE_VMAF_SALIENCY_MAP: aom_tune_metric = 9;
pub const AOM_TUNE_BUTTERAUGLI: aom_tune_metric = 8;
pub const AOM_TUNE_VMAF_NEG_MAX_GAIN: aom_tune_metric = 7;
pub const AOM_TUNE_VMAF_MAX_GAIN: aom_tune_metric = 6;
pub const AOM_TUNE_VMAF_WITHOUT_PREPROCESSING: aom_tune_metric = 5;
pub const AOM_TUNE_VMAF_WITH_PREPROCESSING: aom_tune_metric = 4;
pub const AOM_TUNE_SSIM: aom_tune_metric = 1;
pub const AOM_TUNE_PSNR: aom_tune_metric = 0;
pub type aom_dist_metric = core::ffi::c_uint;
pub const AOM_DIST_METRIC_QM_PSNR: aom_dist_metric = 1;
pub const AOM_DIST_METRIC_PSNR: aom_dist_metric = 0;
pub type AOM_SVC_FRAME_DROP_MODE = core::ffi::c_uint;
pub const AOM_FULL_SUPERFRAME_DROP: AOM_SVC_FRAME_DROP_MODE = 1;
pub const AOM_LAYER_DROP: AOM_SVC_FRAME_DROP_MODE = 0;
pub type AV1LfSync = AV1LfSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TokenExtra {
    #[bitfield(name = "token", ty = "uint8_t", bits = "0..=2")]
    #[bitfield(name = "reserved", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "color_ctx", ty = "int8_t", bits = "4..=7")]
    pub token_reserved_color_ctx: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenList {
    pub start: *mut TokenExtra,
    pub count: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenInfo {
    pub tokens_allocated: core::ffi::c_uint,
    pub tile_tok: [[*mut TokenExtra; 64]; 64],
    pub tplist: [[*mut TokenList; 64]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_COMP {
    pub ppi: *mut AV1_PRIMARY,
    pub enc_quant_dequant_params: EncQuantDequantParams,
    pub td: ThreadData,
    pub counts: FRAME_COUNTS,
    pub mbmi_ext_info: MBMIExtFrameBufferInfo,
    pub coeff_buffer_base: *mut CB_COEFF_BUFFER,
    pub coeff_buffer_pool: CoeffBufferPool,
    pub common: AV1_COMMON,
    pub oxcf: AV1EncoderConfig,
    pub optimize_seg_arr: [TRELLIS_OPT_TYPE; 8],
    pub source: *mut YV12_BUFFER_CONFIG,
    pub last_source: *mut YV12_BUFFER_CONFIG,
    pub unscaled_source: *mut YV12_BUFFER_CONFIG,
    pub scaled_source: YV12_BUFFER_CONFIG,
    pub unscaled_last_source: *mut YV12_BUFFER_CONFIG,
    pub scaled_last_source: YV12_BUFFER_CONFIG,
    pub unfiltered_source: *mut YV12_BUFFER_CONFIG,
    pub orig_source: YV12_BUFFER_CONFIG,
    pub skip_tpl_setup_stats: core::ffi::c_int,
    pub cb_delta_rdmult_enabled: core::ffi::c_int,
    pub tpl_rdmult_scaling_factors: *mut core::ffi::c_double,
    pub tf_ctx: TemporalFilterCtx,
    pub cdef_search_ctx: *mut CdefSearchCtx,
    pub force_intpel_info: ForceIntegerMVInfo,
    pub scaled_ref_buf: [*mut RefCntBuffer; 7],
    pub last_show_frame_buf: *mut RefCntBuffer,
    pub refresh_frame: RefreshFrameInfo,
    pub rt_reduce_num_ref_buffers: core::ffi::c_int,
    pub ext_flags: ExternalFlags,
    pub last_frame_uf: YV12_BUFFER_CONFIG,
    pub trial_frame_rst: YV12_BUFFER_CONFIG,
    pub ambient_err: int64_t,
    pub rd: RD_OPT,
    pub coding_context: CODING_CONTEXT,
    pub gm_info: GlobalMotionInfo,
    pub winner_mode_params: WinnerModeParams,
    pub time_stamps: TimeStamps,
    pub rc: RATE_CONTROL,
    pub framerate: core::ffi::c_double,
    pub ref_frame_flags: core::ffi::c_int,
    pub speed: core::ffi::c_int,
    pub sf: SPEED_FEATURES,
    pub mv_search_params: MotionVectorSearchParams,
    pub all_one_sided_refs: core::ffi::c_int,
    pub enc_seg: EncSegmentationInfo,
    pub cyclic_refresh: *mut CYCLIC_REFRESH,
    pub active_map: ActiveMap,
    pub gf_frame_index: core::ffi::c_uchar,
    pub droppable: core::ffi::c_int,
    pub frame_info: FRAME_INFO,
    pub frame_index_set: FRAME_INDEX_SET,
    pub data_alloc_width: core::ffi::c_int,
    pub data_alloc_height: core::ffi::c_int,
    pub initial_mbs: core::ffi::c_int,
    pub frame_size_related_setup_done: bool,
    pub last_coded_width: core::ffi::c_int,
    pub last_coded_height: core::ffi::c_int,
    pub resize_pending_params: ResizePendingParams,
    pub tile_data: *mut TileDataEnc,
    pub allocated_tiles: core::ffi::c_int,
    pub token_info: TokenInfo,
    pub vaq_refresh: core::ffi::c_int,
    pub vbp_info: VarBasedPartitionInfo,
    pub num_frame_recode: core::ffi::c_int,
    pub frame_new_probs: [FrameProbInfo; 10],
    pub do_update_frame_probs_txtype: [core::ffi::c_int; 10],
    pub do_update_frame_probs_obmc: [core::ffi::c_int; 10],
    pub do_update_frame_probs_warp: [core::ffi::c_int; 10],
    pub do_update_frame_probs_interpfilter: [core::ffi::c_int; 10],
    pub new_framerate: core::ffi::c_double,
    pub do_update_vbr_bits_off_target_fast: core::ffi::c_int,
    pub mt_info: MultiThreadInfo,
    pub existing_fb_idx_to_show: core::ffi::c_int,
    pub intrabc_used: core::ffi::c_int,
    pub prune_ref_frame_mask: core::ffi::c_int,
    pub keep_single_ref_frame_mask: core::ffi::c_int,
    pub keep_comp_ref_frame_mask: core::ffi::c_int,
    pub lr_ctxt: AV1LrStruct,
    pub pick_lr_ctxt: AV1LrPickStruct,
    pub film_grain_table: *mut aom_film_grain_table_t,
    pub denoise_and_model: *mut aom_denoise_and_model_t,
    pub interp_search_flags: InterpSearchFlags,
    pub use_screen_content_tools: core::ffi::c_int,
    pub is_screen_content_type: core::ffi::c_int,
    pub frame_header_count: core::ffi::c_int,
    pub deltaq_used: core::ffi::c_int,
    pub ref_frame_dist_info: RefFrameDistanceInfo,
    pub ssim_rdmult_scaling_factors: *mut core::ffi::c_double,
    pub vmaf_info: TuneVMAFInfo,
    pub svc: SVC,
    pub compressor_stage: COMPRESSOR_STAGE,
    pub last_frame_type: FRAME_TYPE,
    pub num_tg: core::ffi::c_int,
    pub superres_mode: aom_superres_mode,
    pub firstpass_data: FirstPassData,
    pub noise_estimate: NOISE_ESTIMATE,
    pub consec_zero_mv: *mut uint8_t,
    pub consec_zero_mv_alloc_size: core::ffi::c_int,
    pub fp_block_size: BLOCK_SIZE,
    pub sb_counter: core::ffi::c_int,
    pub ext_part_controller: ExtPartController,
    pub mv_stats: MV_STATS,
    pub ref_refresh_index: core::ffi::c_int,
    pub refresh_idx_available: bool,
    pub ref_idx_to_skip: core::ffi::c_int,
    pub do_frame_data_update: bool,
    pub mb_weber_stats: *mut WeberStats,
    pub prep_rate_estimates: *mut core::ffi::c_int,
    pub ext_rate_distribution: *mut core::ffi::c_double,
    pub ext_rate_scale: core::ffi::c_double,
    pub weber_bsize: BLOCK_SIZE,
    pub norm_wiener_variance: int64_t,
    pub mb_delta_q: *mut core::ffi::c_int,
    pub is_dropped_frame: bool,
    pub twopass_frame: TWO_PASS_FRAME,
    pub second_pass_log_stream: *mut FILE,
    pub src_sad_blk_64x64: *mut uint64_t,
    pub src_sad_blk_alloc_size: core::ffi::c_int,
    pub rec_sse: uint64_t,
    pub use_ducky_encode: core::ffi::c_int,
    pub ducky_encode_info: DuckyEncodeInfo,
    pub frames_since_last_update: core::ffi::c_int,
    pub zeromv_skip_thresh_exit_part: [core::ffi::c_uint; 22],
    pub alloc_pyramid: bool,
    pub palette_pixel_num: core::ffi::c_int,
    pub scaled_last_source_available: core::ffi::c_int,
    pub roi: aom_roi_map_t,
    pub ext_ratectrl: AOM_EXT_RATECTRL,
    pub extrc_tpl_gop_stats: AomTplGopStats,
    pub do_border_pad: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AOM_EXT_RATECTRL {
    pub ready: core::ffi::c_int,
    pub ext_rdmult: core::ffi::c_int,
    pub use_delta_q: core::ffi::c_int,
    pub model: aom_rc_model_t,
    pub funcs: aom_rc_funcs_t,
    pub ratectrl_config: aom_rc_config_t,
    pub rc_firstpass_stats: aom_rc_firstpass_stats_t,
    pub sb_params_list: *mut aom_sb_params,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DuckyEncodeInfo {
    pub frame_info: DuckyEncodeFrameInfo,
    pub frame_result: DuckyEncodeFrameResult,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DuckyEncodeFrameResult {
    pub global_order_idx: core::ffi::c_int,
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub rate: core::ffi::c_int,
    pub dist: int64_t,
    pub psnr: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DuckyEncodeFrameInfo {
    pub qp_mode: DUCKY_ENCODE_FRAME_MODE,
    pub gop_mode: DUCKY_ENCODE_GOP_MODE,
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub superblock_encode_qindex: *mut core::ffi::c_int,
    pub superblock_encode_rdmult: *mut core::ffi::c_int,
    pub delta_q_enabled: core::ffi::c_int,
}
pub type DUCKY_ENCODE_GOP_MODE = uint8_t;
pub type DUCKY_ENCODE_FRAME_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWO_PASS_FRAME {
    pub stats_in: *const FIRSTPASS_STATS,
    pub this_frame: *const FIRSTPASS_STATS,
    pub mb_av_energy: core::ffi::c_double,
    pub fr_content_type: FRAME_CONTENT_TYPE,
    pub frame_avg_haar_energy: core::ffi::c_double,
}
pub type FRAME_CONTENT_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIRSTPASS_STATS {
    pub frame: core::ffi::c_double,
    pub weight: core::ffi::c_double,
    pub intra_error: core::ffi::c_double,
    pub frame_avg_wavelet_energy: core::ffi::c_double,
    pub coded_error: core::ffi::c_double,
    pub sr_coded_error: core::ffi::c_double,
    pub lt_coded_error: core::ffi::c_double,
    pub pcnt_inter: core::ffi::c_double,
    pub pcnt_motion: core::ffi::c_double,
    pub pcnt_second_ref: core::ffi::c_double,
    pub pcnt_neutral: core::ffi::c_double,
    pub intra_skip_pct: core::ffi::c_double,
    pub inactive_zone_rows: core::ffi::c_double,
    pub inactive_zone_cols: core::ffi::c_double,
    pub MVr: core::ffi::c_double,
    pub mvr_abs: core::ffi::c_double,
    pub MVc: core::ffi::c_double,
    pub mvc_abs: core::ffi::c_double,
    pub MVrv: core::ffi::c_double,
    pub MVcv: core::ffi::c_double,
    pub mv_in_out_count: core::ffi::c_double,
    pub new_mv_count: core::ffi::c_double,
    pub duration: core::ffi::c_double,
    pub count: core::ffi::c_double,
    pub raw_error_stdev: core::ffi::c_double,
    pub is_flash: int64_t,
    pub noise_var: core::ffi::c_double,
    pub cor_coeff: core::ffi::c_double,
    pub log_intra_error: core::ffi::c_double,
    pub log_coded_error: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeberStats {
    pub mb_wiener_variance: int64_t,
    pub src_variance: int64_t,
    pub rec_variance: int64_t,
    pub src_pix_max: int16_t,
    pub rec_pix_max: int16_t,
    pub distortion: int64_t,
    pub satd: int64_t,
    pub max_scale: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_STATS {
    pub high_prec: core::ffi::c_int,
    pub q: core::ffi::c_int,
    pub order: core::ffi::c_int,
    pub inter_count: core::ffi::c_int,
    pub intra_count: core::ffi::c_int,
    pub default_mvs: core::ffi::c_int,
    pub mv_joint_count: [core::ffi::c_int; 4],
    pub last_bit_zero: core::ffi::c_int,
    pub last_bit_nonzero: core::ffi::c_int,
    pub total_mv_rate: core::ffi::c_int,
    pub hp_total_mv_rate: core::ffi::c_int,
    pub lp_total_mv_rate: core::ffi::c_int,
    pub horz_text: core::ffi::c_int,
    pub vert_text: core::ffi::c_int,
    pub diag_text: core::ffi::c_int,
    pub valid: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtPartController {
    pub ready: core::ffi::c_int,
    pub test_mode: core::ffi::c_int,
    pub config: aom_ext_part_config_t,
    pub model: aom_ext_part_model_t,
    pub funcs: aom_ext_part_funcs_t,
}
pub type NOISE_ESTIMATE = noise_estimate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct noise_estimate {
    pub enabled: core::ffi::c_int,
    pub level: NOISE_LEVEL,
    pub value: core::ffi::c_int,
    pub thresh: core::ffi::c_int,
    pub adapt_thresh: core::ffi::c_int,
    pub count: core::ffi::c_int,
    pub last_w: core::ffi::c_int,
    pub last_h: core::ffi::c_int,
    pub num_frames_estimate: core::ffi::c_int,
}
pub type NOISE_LEVEL = noise_level;
pub type noise_level = core::ffi::c_uint;
pub const kHigh: noise_level = 3;
pub const kMedium: noise_level = 2;
pub const kLow: noise_level = 1;
pub const kLowLow: noise_level = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FirstPassData {
    pub mb_stats: *mut FRAME_STATS,
    pub raw_motion_err_list: *mut core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_STATS {
    pub intra_error: int64_t,
    pub frame_avg_wavelet_energy: int64_t,
    pub coded_error: int64_t,
    pub sr_coded_error: int64_t,
    pub lt_coded_error: int64_t,
    pub mv_count: core::ffi::c_int,
    pub inter_count: core::ffi::c_int,
    pub second_ref_count: core::ffi::c_int,
    pub neutral_count: core::ffi::c_double,
    pub intra_skip_count: core::ffi::c_int,
    pub image_data_start_row: core::ffi::c_int,
    pub new_mv_count: core::ffi::c_int,
    pub sum_in_vectors: core::ffi::c_int,
    pub sum_mvr: core::ffi::c_int,
    pub sum_mvc: core::ffi::c_int,
    pub sum_mvr_abs: core::ffi::c_int,
    pub sum_mvc_abs: core::ffi::c_int,
    pub sum_mvrs: int64_t,
    pub sum_mvcs: int64_t,
    pub intra_factor: core::ffi::c_double,
    pub brightness_factor: core::ffi::c_double,
}
pub type COMPRESSOR_STAGE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVC {
    pub spatial_layer_id: core::ffi::c_int,
    pub temporal_layer_id: core::ffi::c_int,
    pub number_spatial_layers: core::ffi::c_int,
    pub number_temporal_layers: core::ffi::c_int,
    pub prev_number_spatial_layers: core::ffi::c_int,
    pub prev_number_temporal_layers: core::ffi::c_int,
    pub use_flexible_mode: core::ffi::c_int,
    pub ksvc_fixed_mode: core::ffi::c_int,
    pub base_framerate: core::ffi::c_double,
    pub current_superframe: core::ffi::c_uint,
    pub skip_mvsearch_last: core::ffi::c_int,
    pub skip_mvsearch_gf: core::ffi::c_int,
    pub skip_mvsearch_altref: core::ffi::c_int,
    pub spatial_layer_fb: [core::ffi::c_int; 8],
    pub temporal_layer_fb: [core::ffi::c_int; 8],
    pub num_encoded_top_layer: core::ffi::c_int,
    pub first_layer_denoise: core::ffi::c_int,
    pub source_last_TL0: YV12_BUFFER_CONFIG,
    pub layer_context: *mut LAYER_CONTEXT,
    pub num_allocated_layers: core::ffi::c_int,
    pub downsample_filter_type: [InterpFilter; 4],
    pub downsample_filter_phase: [core::ffi::c_int; 4],
    pub force_zero_mode_spatial_ref: core::ffi::c_int,
    pub has_lower_quality_layer: core::ffi::c_int,
    pub framedrop_mode: AOM_SVC_FRAME_DROP_MODE,
    pub last_layer_dropped: [bool; 4],
    pub drop_spatial_layer: [bool; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LAYER_CONTEXT {
    pub rc: RATE_CONTROL,
    pub p_rc: PRIMARY_RATE_CONTROL,
    pub framerate_factor: core::ffi::c_int,
    pub layer_target_bitrate: int64_t,
    pub scaling_factor_num: core::ffi::c_int,
    pub scaling_factor_den: core::ffi::c_int,
    pub target_bandwidth: int64_t,
    pub spatial_layer_target_bandwidth: int64_t,
    pub framerate: core::ffi::c_double,
    pub avg_frame_size: core::ffi::c_int,
    pub max_q: core::ffi::c_int,
    pub min_q: core::ffi::c_int,
    pub frames_from_key_frame: core::ffi::c_int,
    pub sb_index: core::ffi::c_int,
    pub map: *mut int8_t,
    pub actual_num_seg1_blocks: core::ffi::c_int,
    pub actual_num_seg2_blocks: core::ffi::c_int,
    pub counter_encode_maxq_scene_change: core::ffi::c_int,
    pub speed: uint8_t,
    pub group_index: core::ffi::c_uchar,
    pub is_key_frame: core::ffi::c_int,
    pub max_mv_magnitude: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMARY_RATE_CONTROL {
    pub gf_group_bits: int64_t,
    pub kf_boost: core::ffi::c_int,
    pub gfu_boost: core::ffi::c_int,
    pub gfu_boost_average: core::ffi::c_int,
    pub gf_intervals: [core::ffi::c_int; 15],
    pub cur_gf_index: core::ffi::c_int,
    pub num_regions: core::ffi::c_int,
    pub regions: [REGIONS; 150],
    pub regions_offset: core::ffi::c_int,
    pub frames_till_regions_update: core::ffi::c_int,
    pub baseline_gf_interval: core::ffi::c_int,
    pub constrained_gf_group: core::ffi::c_int,
    pub this_key_frame_forced: core::ffi::c_int,
    pub next_key_frame_forced: core::ffi::c_int,
    pub starting_buffer_level: int64_t,
    pub optimal_buffer_level: int64_t,
    pub maximum_buffer_size: int64_t,
    pub arf_q: core::ffi::c_int,
    pub arf_boost_factor: float_t,
    pub base_layer_qp: core::ffi::c_int,
    pub num_stats_used_for_kf_boost: core::ffi::c_int,
    pub num_stats_used_for_gfu_boost: core::ffi::c_int,
    pub num_stats_required_for_gfu_boost: core::ffi::c_int,
    pub enable_scenecut_detection: core::ffi::c_int,
    pub use_arf_in_this_kf_group: core::ffi::c_int,
    pub ni_frames: core::ffi::c_int,
    pub tot_q: core::ffi::c_double,
    pub last_kf_qindex: core::ffi::c_int,
    pub avg_frame_qindex: [core::ffi::c_int; 4],
    pub active_best_quality: [core::ffi::c_int; 7],
    pub last_boosted_qindex: core::ffi::c_int,
    pub avg_q: core::ffi::c_double,
    pub last_q: [core::ffi::c_int; 4],
    pub rate_correction_factors: [core::ffi::c_double; 4],
    pub total_actual_bits: int64_t,
    pub total_target_bits: int64_t,
    pub buffer_level: int64_t,
    pub rate_error_estimate: core::ffi::c_int,
    pub vbr_bits_off_target: int64_t,
    pub vbr_bits_off_target_fast: int64_t,
    pub bits_off_target: int64_t,
    pub rolling_target_bits: core::ffi::c_int,
    pub rolling_actual_bits: core::ffi::c_int,
    pub q_history: [core::ffi::c_int; 1000],
}
pub type REGIONS = regions;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regions {
    pub start: core::ffi::c_int,
    pub last: core::ffi::c_int,
    pub avg_noise_var: core::ffi::c_double,
    pub avg_cor_coeff: core::ffi::c_double,
    pub avg_sr_fr_ratio: core::ffi::c_double,
    pub avg_intra_err: core::ffi::c_double,
    pub avg_coded_err: core::ffi::c_double,
    pub type_0: REGION_TYPES,
}
pub type REGION_TYPES = region_types;
pub type region_types = core::ffi::c_uint;
pub const BLENDING_REGION: region_types = 3;
pub const SCENECUT_REGION: region_types = 2;
pub const HIGH_VAR_REGION: region_types = 1;
pub const STABLE_REGION: region_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RATE_CONTROL {
    pub base_frame_target: core::ffi::c_int,
    pub this_frame_target: core::ffi::c_int,
    pub projected_frame_size: core::ffi::c_int,
    pub coefficient_size: core::ffi::c_int,
    pub sb64_target_rate: core::ffi::c_int,
    pub frames_since_golden: core::ffi::c_int,
    pub frames_till_gf_update_due: core::ffi::c_int,
    pub intervals_till_gf_calculate_due: core::ffi::c_int,
    pub min_gf_interval: core::ffi::c_int,
    pub max_gf_interval: core::ffi::c_int,
    pub static_scene_max_gf_interval: core::ffi::c_int,
    pub frames_to_key: core::ffi::c_int,
    pub frames_since_key: core::ffi::c_int,
    pub frames_to_fwd_kf: core::ffi::c_int,
    pub is_src_frame_alt_ref: core::ffi::c_int,
    pub sframe_due: core::ffi::c_int,
    pub high_source_sad: core::ffi::c_int,
    pub high_source_sad_lag: [core::ffi::c_int; 32],
    pub high_motion_content_screen_rtc: core::ffi::c_int,
    pub avg_source_sad: uint64_t,
    pub prev_avg_source_sad: uint64_t,
    pub frame_source_sad: uint64_t,
    pub frame_source_sad_lag: [uint64_t; 32],
    pub last_frame_low_source_sad: core::ffi::c_uint,
    pub frame_spatial_variance: uint64_t,
    pub static_since_last_scene_change: core::ffi::c_int,
    pub last_encoded_size_keyframe: core::ffi::c_int,
    pub last_target_size_keyframe: core::ffi::c_int,
    pub frames_since_scene_change: core::ffi::c_int,
    pub perc_spatial_flat_blocks: core::ffi::c_int,
    pub num_col_blscroll_last_tl0: core::ffi::c_int,
    pub num_row_blscroll_last_tl0: core::ffi::c_int,
    pub avg_frame_bandwidth: core::ffi::c_int,
    pub min_frame_bandwidth: core::ffi::c_int,
    pub max_frame_bandwidth: core::ffi::c_int,
    pub prev_avg_frame_bandwidth: core::ffi::c_int,
    pub ni_av_qi: core::ffi::c_int,
    pub ni_tot_qi: core::ffi::c_int,
    pub decimation_factor: core::ffi::c_int,
    pub decimation_count: core::ffi::c_int,
    pub prev_frame_is_dropped: core::ffi::c_int,
    pub drop_count_consec: core::ffi::c_int,
    pub max_consec_drop: core::ffi::c_int,
    pub force_max_q: core::ffi::c_int,
    pub postencode_drop: core::ffi::c_int,
    pub frame_number_encoded: core::ffi::c_uint,
    pub worst_quality: core::ffi::c_int,
    pub best_quality: core::ffi::c_int,
    pub rc_1_frame: core::ffi::c_int,
    pub rc_2_frame: core::ffi::c_int,
    pub q_1_frame: core::ffi::c_int,
    pub q_2_frame: core::ffi::c_int,
    pub active_worst_quality: core::ffi::c_int,
    pub avg_frame_low_motion: core::ffi::c_int,
    pub cnt_zeromv: core::ffi::c_int,
    pub percent_blocks_with_motion: core::ffi::c_int,
    pub percent_blocks_inactive: core::ffi::c_int,
    pub max_block_source_sad: uint64_t,
    pub resize_state: RESIZE_STATE,
    pub resize_avg_qp: core::ffi::c_int,
    pub resize_buffer_underflow: core::ffi::c_int,
    pub resize_count: core::ffi::c_int,
    pub rtc_external_ratectrl: core::ffi::c_int,
    pub frame_level_fast_extra_bits: core::ffi::c_int,
    pub frame_level_rate_correction_factors: [core::ffi::c_double; 4],
    pub frame_num_last_gf_refresh: core::ffi::c_int,
    pub prev_coded_width: core::ffi::c_int,
    pub prev_coded_height: core::ffi::c_int,
    pub bit_est_ratio: core::ffi::c_int,
    pub use_external_qp_one_pass: core::ffi::c_int,
}
pub type RESIZE_STATE = core::ffi::c_uint;
pub const ONE_HALF: RESIZE_STATE = 2;
pub const THREE_QUARTER: RESIZE_STATE = 1;
pub const ORIG: RESIZE_STATE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TuneVMAFInfo {
    pub rdmult_scaling_factors: *mut core::ffi::c_double,
    pub last_frame_ysse: [core::ffi::c_double; 6],
    pub last_frame_vmaf: [core::ffi::c_double; 6],
    pub last_frame_unsharp_amount: [core::ffi::c_double; 6],
    pub original_qindex: core::ffi::c_int,
    pub vmaf_model: *mut VmafModel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefFrameDistanceInfo {
    pub ref_relative_dist: [core::ffi::c_int; 7],
    pub nearest_past_ref: int8_t,
    pub nearest_future_ref: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpSearchFlags {
    pub default_interp_skip_flags: core::ffi::c_int,
    pub interp_filter_search_mask: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrPickStruct {
    pub rusi: [*mut RestUnitSearchInfo; 3],
    pub dgd_avg: *mut int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestUnitSearchInfo {
    pub wiener: WienerInfo,
    pub sgrproj: SgrprojInfo,
    pub best_rtype: [RestorationType; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiThreadInfo {
    pub num_workers: core::ffi::c_int,
    pub num_mod_workers: [core::ffi::c_int; 12],
    pub workers: *mut AVxWorker,
    pub tile_thr_data: *mut EncWorkerData,
    pub row_mt_enabled: bool,
    pub pack_bs_mt_enabled: bool,
    pub enc_row_mt: AV1EncRowMultiThreadInfo,
    pub intra_mt: AV1EncAllIntraMultiThreadInfo,
    pub tpl_row_mt: AV1TplRowMultiThreadInfo,
    pub lf_row_sync: AV1LfSync,
    pub lr_row_sync: AV1LrSync,
    pub pack_bs_sync: AV1EncPackBSSync,
    pub gm_sync: AV1GlobalMotionSync,
    pub tf_sync: AV1TemporalFilterSync,
    pub cdef_sync: AV1CdefSync,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub restore_state_buf: RestoreStateBuffers,
    pub pipeline_lpf_mt_with_enc: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestoreStateBuffers {
    pub cdef_srcbuf: *mut uint16_t,
    pub cdef_colbuf: [*mut uint16_t; 3],
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut RestorationLineBuffers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1TemporalFilterSync {
    pub mutex_: *mut pthread_mutex_t,
    pub next_tf_row: core::ffi::c_int,
    pub tf_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1GlobalMotionSync {
    pub job_info: GlobalMotionJobInfo,
    pub mutex_: *mut pthread_mutex_t,
    pub gm_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalMotionJobInfo {
    pub thread_id_to_dir: [int8_t; 64],
    pub early_exit: [int8_t; 2],
    pub next_frame_to_process: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1EncPackBSSync {
    pub mutex_: *mut pthread_mutex_t,
    pub pack_bs_tile_order: [PackBSTileOrder; 512],
    pub next_job_idx: core::ffi::c_int,
    pub pack_bs_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PackBSTileOrder {
    pub abs_sum_level: uint64_t,
    pub tile_idx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1TplRowMultiThreadInfo {
    pub tpl_mt_exit: bool,
    pub mutex_: *mut pthread_mutex_t,
    pub sync_read_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1TplRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
    pub sync_write_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1TplRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1TplRowMultiThreadSync {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub num_finished_cols: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1EncAllIntraMultiThreadInfo {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub intra_sync_read_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1EncRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sync_write_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1EncRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1EncRowMultiThreadSync {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub num_finished_cols: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub intrabc_extra_top_right_sb_delay: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub next_mi_row: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1EncRowMultiThreadInfo {
    pub allocated_tile_rows: core::ffi::c_int,
    pub allocated_tile_cols: core::ffi::c_int,
    pub allocated_rows: core::ffi::c_int,
    pub allocated_cols: core::ffi::c_int,
    pub thread_id_to_tile_id: [core::ffi::c_int; 64],
    pub num_tile_cols_done: *mut core::ffi::c_int,
    pub allocated_sb_rows: core::ffi::c_int,
    pub row_mt_exit: bool,
    pub firstpass_mt_exit: bool,
    pub mb_wiener_mt_exit: bool,
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub sync_read_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1EncRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
    pub sync_write_ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut AV1EncRowMultiThreadSync,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameProbInfo {
    pub obmc_probs: [[core::ffi::c_int; 22]; 7],
    pub warped_probs: [core::ffi::c_int; 7],
    pub tx_type_probs: [[[core::ffi::c_int; 16]; 19]; 7],
    pub switchable_interp_probs: [[[core::ffi::c_int; 3]; 16]; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarBasedPartitionInfo {
    pub thresholds: [int64_t; 5],
    pub threshold_minmax: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileDataEnc {
    pub tile_info: TileInfo,
    pub __aom_align_tctx: [crate::platform::Align16; 0],
    pub tctx: FRAME_CONTEXT,
    pub row_ctx: *mut FRAME_CONTEXT,
    pub abs_sum_level: uint64_t,
    pub allow_update_cdf: uint8_t,
    pub inter_mode_rd_models: [InterModeRdModel; 22],
    pub row_mt_sync: AV1EncRowMultiThreadSync,
    pub firstpass_top_mv: MV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterModeRdModel {
    pub ready: core::ffi::c_int,
    pub a: core::ffi::c_double,
    pub b: core::ffi::c_double,
    pub dist_mean: core::ffi::c_double,
    pub ld_mean: core::ffi::c_double,
    pub sse_mean: core::ffi::c_double,
    pub sse_sse_mean: core::ffi::c_double,
    pub sse_ld_mean: core::ffi::c_double,
    pub num: core::ffi::c_int,
    pub dist_sum: core::ffi::c_double,
    pub ld_sum: core::ffi::c_double,
    pub sse_sum: core::ffi::c_double,
    pub sse_sse_sum: core::ffi::c_double,
    pub sse_ld_sum: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResizePendingParams {
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_INDEX_SET {
    pub show_frame_count: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_INFO {
    pub frame_width: core::ffi::c_int,
    pub frame_height: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mb_rows: core::ffi::c_int,
    pub mb_cols: core::ffi::c_int,
    pub num_mbs: core::ffi::c_int,
    pub bit_depth: aom_bit_depth_t,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActiveMap {
    pub enabled: core::ffi::c_int,
    pub update: core::ffi::c_int,
    pub map: *mut core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CYCLIC_REFRESH {
    pub percent_refresh: core::ffi::c_int,
    pub percent_refresh_adjustment: core::ffi::c_int,
    pub max_qdelta_perc: core::ffi::c_int,
    pub sb_index: core::ffi::c_int,
    pub last_sb_index: core::ffi::c_int,
    pub time_for_refresh: core::ffi::c_int,
    pub target_num_seg_blocks: core::ffi::c_int,
    pub actual_num_seg1_blocks: core::ffi::c_int,
    pub actual_num_seg2_blocks: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub map: *mut int8_t,
    pub thresh_rate_sb: int64_t,
    pub thresh_dist_sb: int64_t,
    pub motion_thresh: int16_t,
    pub rate_ratio_qdelta: core::ffi::c_double,
    pub rate_ratio_qdelta_adjustment: core::ffi::c_double,
    pub rate_boost_fac: core::ffi::c_int,
    pub qindex_delta: [core::ffi::c_int; 3],
    pub apply_cyclic_refresh: core::ffi::c_int,
    pub skip_over4x4: core::ffi::c_int,
    pub counter_encode_maxq_scene_change: core::ffi::c_int,
    pub use_block_sad_scene_det: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncSegmentationInfo {
    pub map: *mut uint8_t,
    pub has_lossless_segment: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionVectorSearchParams {
    pub max_mv_magnitude: core::ffi::c_int,
    pub mv_step_param: core::ffi::c_int,
    pub find_fractional_mv_step: Option<fractional_mv_step_fp>,
    pub search_site_cfg: [[search_site_config; 6]; 3],
}
pub type fractional_mv_step_fp = unsafe extern "C-unwind" fn(
    *mut MACROBLOCKD,
    *const AV1_COMMON,
    *const SUBPEL_MOTION_SEARCH_PARAMS,
    MV,
    *const FULLPEL_MV_STATS,
    *mut MV,
    *mut core::ffi::c_int,
    *mut core::ffi::c_uint,
    *mut int_mv,
) -> core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FULLPEL_MV_STATS {
    pub err_cost: core::ffi::c_int,
    pub distortion: core::ffi::c_uint,
    pub sse: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SUBPEL_MOTION_SEARCH_PARAMS {
    pub allow_hp: core::ffi::c_int,
    pub cost_list: *const core::ffi::c_int,
    pub forced_stop: SUBPEL_FORCE_STOP,
    pub iters_per_step: core::ffi::c_int,
    pub mv_limits: SubpelMvLimits,
    pub mv_cost_params: MV_COST_PARAMS,
    pub var_params: SUBPEL_SEARCH_VAR_PARAMS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SUBPEL_SEARCH_VAR_PARAMS {
    pub vfp: *const aom_variance_fn_ptr_t,
    pub subpel_search_type: SUBPEL_SEARCH_TYPE,
    pub ms_buffers: MSBuffers,
    pub w: core::ffi::c_int,
    pub h: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MSBuffers {
    pub ref_0: *const buf_2d,
    pub src: *const buf_2d,
    pub second_pred: *const uint8_t,
    pub mask: *const uint8_t,
    pub mask_stride: core::ffi::c_int,
    pub inv_mask: core::ffi::c_int,
    pub wsrc: *const int32_t,
    pub obmc_mask: *const int32_t,
}
pub type aom_variance_fn_ptr_t = aom_variance_vtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_COST_PARAMS {
    pub ref_mv: *const MV,
    pub full_ref_mv: FULLPEL_MV,
    pub mv_cost_type: MV_COST_TYPE,
    pub mvjcost: *const core::ffi::c_int,
    pub mvcost: [*const core::ffi::c_int; 2],
    pub error_per_bit: core::ffi::c_int,
    pub sad_per_bit: core::ffi::c_int,
}
pub type MV_COST_TYPE = uint8_t;
pub type SUBPEL_FORCE_STOP = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEED_FEATURES {
    pub hl_sf: HIGH_LEVEL_SPEED_FEATURES,
    pub fp_sf: FIRST_PASS_SPEED_FEATURES,
    pub tpl_sf: TPL_SPEED_FEATURES,
    pub gm_sf: GLOBAL_MOTION_SPEED_FEATURES,
    pub part_sf: PARTITION_SPEED_FEATURES,
    pub mv_sf: MV_SPEED_FEATURES,
    pub inter_sf: INTER_MODE_SPEED_FEATURES,
    pub interp_sf: INTERP_FILTER_SPEED_FEATURES,
    pub intra_sf: INTRA_MODE_SPEED_FEATURES,
    pub tx_sf: TX_SPEED_FEATURES,
    pub rd_sf: RD_CALC_SPEED_FEATURES,
    pub winner_mode_sf: WINNER_MODE_SPEED_FEATURES,
    pub lpf_sf: LOOP_FILTER_SPEED_FEATURES,
    pub rt_sf: REAL_TIME_SPEED_FEATURES,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REAL_TIME_SPEED_FEATURES {
    pub check_intra_pred_nonrd: core::ffi::c_int,
    pub skip_intra_pred: core::ffi::c_int,
    pub estimate_motion_for_var_based_partition: core::ffi::c_int,
    pub nonrd_check_partition_merge_mode: core::ffi::c_int,
    pub nonrd_check_partition_split: core::ffi::c_int,
    pub mode_search_skip_flags: core::ffi::c_uint,
    pub nonrd_prune_ref_frame_search: core::ffi::c_int,
    pub use_nonrd_pick_mode: core::ffi::c_int,
    pub discount_color_cost: core::ffi::c_int,
    pub use_nonrd_altref_frame: core::ffi::c_int,
    pub use_comp_ref_nonrd: core::ffi::c_int,
    pub ref_frame_comp_nonrd: [core::ffi::c_int; 3],
    pub use_real_time_ref_set: core::ffi::c_int,
    pub short_circuit_low_temp_var: core::ffi::c_int,
    pub reuse_inter_pred_nonrd: core::ffi::c_int,
    pub num_inter_modes_for_tx_search: core::ffi::c_int,
    pub use_nonrd_filter_search: core::ffi::c_int,
    pub use_simple_rd_model: core::ffi::c_int,
    pub hybrid_intra_pickmode: core::ffi::c_int,
    pub prune_palette_search_nonrd: core::ffi::c_int,
    pub source_metrics_sb_nonrd: core::ffi::c_int,
    pub overshoot_detection_cbr: OVERSHOOT_DETECTION_CBR,
    pub check_scene_detection: core::ffi::c_int,
    pub rc_adjust_keyframe: core::ffi::c_int,
    pub rc_compute_spatial_var_sc_kf: core::ffi::c_int,
    pub prefer_large_partition_blocks: core::ffi::c_int,
    pub use_temporal_noise_estimate: core::ffi::c_int,
    pub fullpel_search_step_param: core::ffi::c_int,
    pub intra_y_mode_bsize_mask_nrd: [core::ffi::c_int; 16],
    pub prune_hv_pred_modes_using_src_sad: bool,
    pub nonrd_aggressive_skip: core::ffi::c_int,
    pub skip_cdef_sb: core::ffi::c_int,
    pub selective_cdf_update: core::ffi::c_int,
    pub rt_use_intrabc: core::ffi::c_int,
    pub rt_prune_intrabc_nonrd: core::ffi::c_int,
    pub rt_intrabc_miss_mode: core::ffi::c_int,
    pub force_only_last_ref: core::ffi::c_int,
    pub force_large_partition_blocks_intra: core::ffi::c_int,
    pub use_fast_fixed_part: core::ffi::c_int,
    pub increase_source_sad_thresh: core::ffi::c_int,
    pub skip_tx_no_split_var_based_partition: core::ffi::c_int,
    pub skip_newmv_mode_based_on_sse: core::ffi::c_int,
    pub gf_length_lvl: core::ffi::c_int,
    pub prune_inter_modes_with_golden_ref: core::ffi::c_int,
    pub prune_inter_modes_wrt_gf_arf_based_on_sad: core::ffi::c_int,
    pub prune_inter_modes_using_temp_var: core::ffi::c_int,
    pub reduce_mv_pel_precision_highmotion: core::ffi::c_int,
    pub reduce_mv_pel_precision_lowcomplex: core::ffi::c_int,
    pub prune_intra_mode_based_on_mv_range: BLOCK_SIZE,
    pub var_part_split_threshold_shift: core::ffi::c_int,
    pub var_part_based_on_qidx: core::ffi::c_int,
    pub gf_refresh_based_on_qp: core::ffi::c_int,
    pub use_rtc_tf: core::ffi::c_int,
    pub use_idtx_nonrd: core::ffi::c_int,
    pub prune_idtx_nonrd: core::ffi::c_int,
    pub dct_only_palette_nonrd: core::ffi::c_int,
    pub skip_lf_screen: core::ffi::c_int,
    pub thresh_active_maps_skip_lf_cdef: core::ffi::c_int,
    pub part_early_exit_zeromv: core::ffi::c_int,
    pub sse_early_term_inter_search: INTER_SEARCH_EARLY_TERM_IDX,
    pub sad_based_adp_altref_lag: core::ffi::c_int,
    pub partition_direct_merging: core::ffi::c_int,
    pub tx_size_level_based_on_qstep: core::ffi::c_int,
    pub vbp_prune_16x16_split_using_min_max_sub_blk_var: bool,
    pub screen_content_cdef_filter_qindex_thresh: core::ffi::c_int,
    pub prune_compoundmode_with_singlecompound_var: bool,
    pub frame_level_mode_cost_update: bool,
    pub prune_h_pred_using_best_mode_so_far: bool,
    pub enable_intra_mode_pruning_using_neighbors: bool,
    pub prune_intra_mode_using_best_sad_so_far: bool,
    pub check_only_zero_zeromv_on_large_blocks: bool,
    pub disable_cdf_update_non_reference_frame: bool,
    pub prune_compoundmode_with_singlemode_var: bool,
    pub skip_compound_based_on_var: bool,
    pub set_zeromv_skip_based_on_source_sad: core::ffi::c_int,
    pub use_adaptive_subpel_search: bool,
    pub enable_ref_short_signaling: bool,
    pub check_globalmv_on_single_ref: bool,
    pub increase_color_thresh_palette: bool,
    pub higher_thresh_scene_detection: core::ffi::c_int,
    pub skip_newmv_flat_blocks_screen: core::ffi::c_int,
    pub skip_encoding_non_reference_slide_change: core::ffi::c_int,
    pub rc_faster_convergence_static: core::ffi::c_int,
    pub skip_newmv_mode_sad_screen: core::ffi::c_int,
}
pub type INTER_SEARCH_EARLY_TERM_IDX = core::ffi::c_uint;
pub const EARLY_TERM_INDICES: INTER_SEARCH_EARLY_TERM_IDX = 5;
pub const EARLY_TERM_IDX_4: INTER_SEARCH_EARLY_TERM_IDX = 4;
pub const EARLY_TERM_IDX_3: INTER_SEARCH_EARLY_TERM_IDX = 3;
pub const EARLY_TERM_IDX_2: INTER_SEARCH_EARLY_TERM_IDX = 2;
pub const EARLY_TERM_IDX_1: INTER_SEARCH_EARLY_TERM_IDX = 1;
pub const EARLY_TERM_DISABLED: INTER_SEARCH_EARLY_TERM_IDX = 0;
pub type OVERSHOOT_DETECTION_CBR = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOOP_FILTER_SPEED_FEATURES {
    pub lpf_pick: LPF_PICK_METHOD,
    pub use_coarse_filter_level_search: core::ffi::c_int,
    pub adaptive_luma_loop_filter_skip: core::ffi::c_int,
    pub skip_loop_filter_using_filt_error: core::ffi::c_int,
    pub cdef_pick_method: CDEF_PICK_METHOD,
    pub zero_low_cdef_strengths: bool,
    pub adaptive_cdef_mode: core::ffi::c_int,
    pub dual_sgr_penalty_level: core::ffi::c_int,
    pub switchable_lr_with_bias_level: core::ffi::c_int,
    pub enable_sgr_ep_pruning: core::ffi::c_int,
    pub disable_loop_restoration_chroma: core::ffi::c_int,
    pub disable_loop_restoration_luma: core::ffi::c_int,
    pub min_lr_unit_size: core::ffi::c_int,
    pub max_lr_unit_size: core::ffi::c_int,
    pub prune_wiener_based_on_src_var: core::ffi::c_int,
    pub prune_sgr_based_on_wiener: core::ffi::c_int,
    pub reduce_wiener_window_size: core::ffi::c_int,
    pub disable_wiener_filter: bool,
    pub disable_sgr_filter: bool,
    pub disable_wiener_coeff_refine_search: bool,
    pub use_downsampled_wiener_stats: core::ffi::c_int,
}
pub type CDEF_PICK_METHOD = core::ffi::c_uint;
pub const CDEF_PICK_METHODS: CDEF_PICK_METHOD = 7;
pub const CDEF_PICK_FROM_Q: CDEF_PICK_METHOD = 6;
pub const CDEF_FAST_SEARCH_LVL5: CDEF_PICK_METHOD = 5;
pub const CDEF_FAST_SEARCH_LVL4: CDEF_PICK_METHOD = 4;
pub const CDEF_FAST_SEARCH_LVL3: CDEF_PICK_METHOD = 3;
pub const CDEF_FAST_SEARCH_LVL2: CDEF_PICK_METHOD = 2;
pub const CDEF_FAST_SEARCH_LVL1: CDEF_PICK_METHOD = 1;
pub const CDEF_FULL_SEARCH: CDEF_PICK_METHOD = 0;
pub type LPF_PICK_METHOD = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WINNER_MODE_SPEED_FEATURES {
    pub enable_winner_mode_for_coeff_opt: core::ffi::c_int,
    pub enable_winner_mode_for_tx_size_srch: core::ffi::c_int,
    pub tx_size_search_level: core::ffi::c_int,
    pub enable_winner_mode_for_use_tx_domain_dist: core::ffi::c_int,
    pub multi_winner_mode_type: MULTI_WINNER_MODE_TYPE,
    pub motion_mode_for_winner_cand: core::ffi::c_int,
    pub dc_blk_pred_level: core::ffi::c_int,
    pub winner_mode_ifs: core::ffi::c_int,
    pub prune_winner_mode_eval_level: core::ffi::c_int,
}
pub type MULTI_WINNER_MODE_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_CALC_SPEED_FEATURES {
    pub simple_model_rd_from_var: core::ffi::c_int,
    pub tx_domain_dist_level: core::ffi::c_int,
    pub tx_domain_dist_thres_level: core::ffi::c_int,
    pub optimize_coefficients: TRELLIS_OPT_TYPE,
    pub use_mb_rd_hash: core::ffi::c_int,
    pub perform_coeff_opt: core::ffi::c_int,
}
pub type TRELLIS_OPT_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TX_SPEED_FEATURES {
    pub inter_tx_size_search_init_depth_sqr: core::ffi::c_int,
    pub inter_tx_size_search_init_depth_rect: core::ffi::c_int,
    pub intra_tx_size_search_init_depth_sqr: core::ffi::c_int,
    pub intra_tx_size_search_init_depth_rect: core::ffi::c_int,
    pub tx_size_search_lgr_block: core::ffi::c_int,
    pub tx_type_search: TX_TYPE_SEARCH,
    pub txb_split_cap: core::ffi::c_int,
    pub adaptive_txb_search_level: core::ffi::c_int,
    pub model_based_prune_tx_search_level: core::ffi::c_int,
    pub refine_fast_tx_search_results: core::ffi::c_int,
    pub prune_tx_size_level: core::ffi::c_int,
    pub prune_intra_tx_depths_using_nn: bool,
    pub use_rd_based_breakout_for_intra_tx_search: bool,
    pub prune_inter_tx_split_rd_eval_lvl: core::ffi::c_int,
    pub use_chroma_trellis_rd_mult: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TX_TYPE_SEARCH {
    pub prune_2d_txfm_mode: TX_TYPE_PRUNE_MODE,
    pub fast_intra_tx_type_search: core::ffi::c_int,
    pub fast_inter_tx_type_prob_thresh: core::ffi::c_int,
    pub use_reduced_intra_txset: core::ffi::c_int,
    pub use_skip_flag_prediction: core::ffi::c_int,
    pub ml_tx_split_thresh: core::ffi::c_int,
    pub skip_tx_search: core::ffi::c_int,
    pub prune_tx_type_using_stats: core::ffi::c_int,
    pub prune_tx_type_est_rd: core::ffi::c_int,
    pub winner_mode_tx_type_pruning: core::ffi::c_int,
}
pub type TX_TYPE_PRUNE_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTRA_MODE_SPEED_FEATURES {
    pub intra_y_mode_mask: [core::ffi::c_int; 5],
    pub intra_uv_mode_mask: [core::ffi::c_int; 5],
    pub skip_intra_in_interframe: core::ffi::c_int,
    pub intra_pruning_with_hog: core::ffi::c_int,
    pub chroma_intra_pruning_with_hog: core::ffi::c_int,
    pub disable_smooth_intra: core::ffi::c_int,
    pub prune_smooth_intra_mode_for_chroma: bool,
    pub prune_filter_intra_level: core::ffi::c_int,
    pub prune_palette_search_level: core::ffi::c_int,
    pub prune_luma_palette_size_search_level: core::ffi::c_int,
    pub prune_chroma_modes_using_luma_winner: core::ffi::c_int,
    pub dv_cost_upd_level: INTERNAL_COST_UPDATE_TYPE,
    pub cfl_search_range: core::ffi::c_int,
    pub top_intra_model_count_allowed: core::ffi::c_int,
    pub adapt_top_model_rd_count_using_neighbors: core::ffi::c_int,
    pub prune_luma_odd_delta_angles_in_intra: core::ffi::c_int,
    pub early_term_chroma_palette_size_search: core::ffi::c_int,
    pub skip_filter_intra_in_inter_frames: core::ffi::c_int,
}
pub type INTERNAL_COST_UPDATE_TYPE = core::ffi::c_uint;
pub const INTERNAL_COST_UPD_SB: INTERNAL_COST_UPDATE_TYPE = 4;
pub const INTERNAL_COST_UPD_SBROW: INTERNAL_COST_UPDATE_TYPE = 3;
pub const INTERNAL_COST_UPD_SBROW_SET: INTERNAL_COST_UPDATE_TYPE = 2;
pub const INTERNAL_COST_UPD_TILE: INTERNAL_COST_UPDATE_TYPE = 1;
pub const INTERNAL_COST_UPD_OFF: INTERNAL_COST_UPDATE_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERP_FILTER_SPEED_FEATURES {
    pub use_fast_interpolation_filter_search: core::ffi::c_int,
    pub disable_dual_filter: core::ffi::c_int,
    pub use_interp_filter: core::ffi::c_int,
    pub skip_sharp_interp_filter_search: core::ffi::c_int,
    pub cb_pred_filter_search: core::ffi::c_int,
    pub adaptive_interp_filter_search: core::ffi::c_int,
    pub skip_interp_filter_search: core::ffi::c_int,
    pub use_more_sharp_interp: core::ffi::c_int,
    pub skip_model_rd_uv: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTER_MODE_SPEED_FEATURES {
    pub inter_mode_rd_model_estimation: core::ffi::c_int,
    pub txfm_rd_gate_level: [core::ffi::c_int; 3],
    pub reduce_inter_modes: core::ffi::c_int,
    pub adaptive_rd_thresh: core::ffi::c_int,
    pub prune_inter_modes_if_skippable: core::ffi::c_int,
    pub selective_ref_frame: core::ffi::c_int,
    pub prune_ref_frame_for_rect_partitions: core::ffi::c_int,
    pub alt_ref_search_fp: core::ffi::c_int,
    pub prune_single_ref: core::ffi::c_int,
    pub prune_comp_ref_frames: core::ffi::c_int,
    pub skip_newmv_in_drl: core::ffi::c_int,
    pub skip_repeated_ref_mv: core::ffi::c_int,
    pub perform_best_rd_based_gating_for_chroma: core::ffi::c_int,
    pub reuse_inter_intra_mode: core::ffi::c_int,
    pub prune_comp_type_by_model_rd: core::ffi::c_int,
    pub prune_comp_type_by_comp_avg: core::ffi::c_int,
    pub prune_comp_search_by_single_result: core::ffi::c_int,
    pub prune_mode_search_simple_translation: core::ffi::c_int,
    pub prune_compound_using_single_ref: core::ffi::c_int,
    pub prune_ext_comp_using_neighbors: core::ffi::c_int,
    pub skip_ext_comp_nearmv_mode: core::ffi::c_int,
    pub prune_comp_using_best_single_mode_ref: core::ffi::c_int,
    pub prune_nearest_near_mv_using_refmv_weight: core::ffi::c_int,
    pub prune_ref_mv_idx_search: core::ffi::c_int,
    pub disable_onesided_comp: core::ffi::c_int,
    pub prune_obmc_prob_thresh: core::ffi::c_int,
    pub prune_warped_prob_thresh: core::ffi::c_int,
    pub disable_interintra_wedge_var_thresh: core::ffi::c_uint,
    pub disable_interinter_wedge_var_thresh: core::ffi::c_uint,
    pub fast_interintra_wedge_search: core::ffi::c_int,
    pub fast_wedge_sign_estimate: core::ffi::c_int,
    pub disable_interinter_wedge_newmv_search: core::ffi::c_int,
    pub use_dist_wtd_comp_flag: DIST_WTD_COMP_FLAG,
    pub mv_cost_upd_level: INTERNAL_COST_UPDATE_TYPE,
    pub coeff_cost_upd_level: INTERNAL_COST_UPDATE_TYPE,
    pub mode_cost_upd_level: INTERNAL_COST_UPDATE_TYPE,
    pub prune_inter_modes_based_on_tpl: core::ffi::c_int,
    pub prune_nearmv_using_neighbors: PRUNE_NEARMV_LEVEL,
    pub model_based_post_interp_filter_breakout: core::ffi::c_int,
    pub reuse_compound_type_decision: core::ffi::c_int,
    pub disable_masked_comp: core::ffi::c_int,
    pub enable_fast_compound_mode_search: core::ffi::c_int,
    pub reuse_mask_search_results: core::ffi::c_int,
    pub enable_fast_wedge_mask_search: core::ffi::c_int,
    pub inter_mode_txfm_breakout: core::ffi::c_int,
    pub limit_inter_mode_cands: core::ffi::c_int,
    pub limit_txfm_eval_per_mode: core::ffi::c_int,
    pub extra_prune_warped: core::ffi::c_int,
    pub skip_arf_compound: core::ffi::c_int,
    pub bias_warp_mode_rd_scale_pct: core::ffi::c_float,
    pub bias_obmc_mode_rd_scale_pct: core::ffi::c_float,
    pub bias_gm_mode_rd_scale_pct: core::ffi::c_float,
    pub skip_cmp_using_top_cmp_avg_est_rd_lvl: core::ffi::c_int,
    pub skip_interinter_wedge_search_based_on_mse: core::ffi::c_int,
    pub enable_comp_wedge_search_using_model_rd: core::ffi::c_int,
}
pub type PRUNE_NEARMV_LEVEL = uint8_t;
pub type DIST_WTD_COMP_FLAG = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_SPEED_FEATURES {
    pub search_method: SEARCH_METHODS,
    pub use_bsize_dependent_search_method: core::ffi::c_int,
    pub auto_mv_step_size: core::ffi::c_int,
    pub subpel_search_method: SUBPEL_SEARCH_METHOD,
    pub subpel_iters_per_step: core::ffi::c_int,
    pub subpel_force_stop: SUBPEL_FORCE_STOP,
    pub simple_motion_subpel_force_stop: SUBPEL_FORCE_STOP,
    pub use_accurate_subpel_search: SUBPEL_SEARCH_TYPE,
    pub exhaustive_searches_thresh: core::ffi::c_int,
    pub mesh_patterns: [MESH_PATTERN; 4],
    pub intrabc_mesh_patterns: [MESH_PATTERN; 4],
    pub reduce_search_range: core::ffi::c_int,
    pub prune_mesh_search: PRUNE_MESH_SEARCH_LEVEL,
    pub use_fullpel_costlist: core::ffi::c_int,
    pub obmc_full_pixel_search_level: core::ffi::c_int,
    pub full_pixel_search_level: core::ffi::c_int,
    pub use_intrabc: core::ffi::c_int,
    pub prune_intrabc_candidate_block_hash_search: core::ffi::c_int,
    pub intrabc_search_level: core::ffi::c_int,
    pub hash_max_8x8_intrabc_blocks: core::ffi::c_int,
    pub use_downsampled_sad: core::ffi::c_int,
    pub disable_extensive_joint_motion_search: core::ffi::c_int,
    pub disable_second_mv: core::ffi::c_int,
    pub skip_fullpel_search_using_startmv_refmv: core::ffi::c_int,
    pub warp_search_method: WARP_SEARCH_METHOD,
    pub warp_search_iters: core::ffi::c_int,
}
pub type PRUNE_MESH_SEARCH_LEVEL = core::ffi::c_uint;
pub const PRUNE_MESH_SEARCH_LVL_2: PRUNE_MESH_SEARCH_LEVEL = 2;
pub const PRUNE_MESH_SEARCH_LVL_1: PRUNE_MESH_SEARCH_LEVEL = 1;
pub const PRUNE_MESH_SEARCH_DISABLED: PRUNE_MESH_SEARCH_LEVEL = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MESH_PATTERN {
    pub range: core::ffi::c_int,
    pub interval: core::ffi::c_int,
}
pub type SUBPEL_SEARCH_METHOD = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARTITION_SPEED_FEATURES {
    pub partition_search_type: PARTITION_SEARCH_TYPE,
    pub fixed_partition_size: BLOCK_SIZE,
    pub prune_ext_partition_types_search_level: core::ffi::c_int,
    pub prune_part4_search: core::ffi::c_int,
    pub ml_prune_partition: core::ffi::c_int,
    pub ml_early_term_after_part_split_level: core::ffi::c_int,
    pub less_rectangular_check_level: core::ffi::c_int,
    pub use_square_partition_only_threshold: BLOCK_SIZE,
    pub auto_max_partition_based_on_simple_motion: MAX_PART_PRED_MODE,
    pub default_min_partition_size: BLOCK_SIZE,
    pub default_max_partition_size: BLOCK_SIZE,
    pub adjust_var_based_rd_partitioning: core::ffi::c_int,
    pub partition_search_breakout_dist_thr: int64_t,
    pub partition_search_breakout_rate_thr: core::ffi::c_int,
    pub ml_partition_search_breakout_thresh: [core::ffi::c_float; 5],
    pub ml_partition_search_breakout_model_index: core::ffi::c_int,
    pub ml_4_partition_search_level_index: core::ffi::c_int,
    pub simple_motion_search_prune_agg: core::ffi::c_int,
    pub simple_motion_search_prune_rect: core::ffi::c_int,
    pub simple_motion_search_split: core::ffi::c_int,
    pub simple_motion_search_early_term_none: core::ffi::c_int,
    pub simple_motion_search_reduce_search_steps: core::ffi::c_int,
    pub max_intra_bsize: BLOCK_SIZE,
    pub intra_cnn_based_part_prune_level: core::ffi::c_int,
    pub ext_partition_eval_thresh: BLOCK_SIZE,
    pub ext_part_eval_based_on_cur_best: core::ffi::c_int,
    pub rect_partition_eval_thresh: core::ffi::c_int,
    pub prune_ext_part_using_split_info: core::ffi::c_int,
    pub prune_rectangular_split_based_on_qidx: core::ffi::c_int,
    pub prune_rect_part_using_4x4_var_deviation: bool,
    pub prune_rect_part_using_none_pred_mode: bool,
    pub early_term_after_none_split: core::ffi::c_int,
    pub ml_predict_breakout_level: core::ffi::c_int,
    pub prune_sub_8x8_partition_level: core::ffi::c_int,
    pub simple_motion_search_rect_split: core::ffi::c_int,
    pub reuse_prev_rd_results_for_part_ab: core::ffi::c_int,
    pub reuse_best_prediction_for_part_ab: core::ffi::c_int,
    pub use_best_rd_for_pruning: core::ffi::c_int,
    pub skip_non_sq_part_based_on_none: core::ffi::c_int,
    pub disable_8x8_part_based_on_qidx: core::ffi::c_int,
    pub prune_h_or_v_4part_using_sms_info: bool,
    pub split_partition_penalty_level: core::ffi::c_int,
}
pub type MAX_PART_PRED_MODE = uint8_t;
pub type PARTITION_SEARCH_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBAL_MOTION_SPEED_FEATURES {
    pub gm_search_type: GM_SEARCH_TYPE,
    pub prune_ref_frame_for_gm_search: core::ffi::c_int,
    pub prune_zero_mv_with_sse: core::ffi::c_int,
    pub disable_gm_search_based_on_stats: core::ffi::c_int,
    pub downsample_level: core::ffi::c_int,
    pub num_refinement_steps: core::ffi::c_int,
    pub gm_erroradv_tr_level: core::ffi::c_int,
}
pub type GM_SEARCH_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPL_SPEED_FEATURES {
    pub gop_length_decision_method: core::ffi::c_int,
    pub prune_intra_modes: core::ffi::c_int,
    pub reduce_first_step_size: core::ffi::c_int,
    pub skip_alike_starting_mv: core::ffi::c_int,
    pub subpel_force_stop: SUBPEL_FORCE_STOP,
    pub search_method: SEARCH_METHODS,
    pub prune_starting_mv: core::ffi::c_int,
    pub prune_ref_frames_in_tpl: core::ffi::c_int,
    pub allow_compound_pred: core::ffi::c_int,
    pub use_y_only_rate_distortion: core::ffi::c_int,
    pub use_sad_for_mode_decision: core::ffi::c_int,
    pub reduce_num_frames: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIRST_PASS_SPEED_FEATURES {
    pub reduce_mv_step_param: core::ffi::c_int,
    pub skip_motion_search_threshold: core::ffi::c_int,
    pub disable_recon: core::ffi::c_int,
    pub skip_zeromv_motion_search: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HIGH_LEVEL_SPEED_FEATURES {
    pub frame_parameter_update: core::ffi::c_int,
    pub recode_loop: RECODE_LOOP_TYPE,
    pub recode_tolerance: core::ffi::c_int,
    pub high_precision_mv_usage: MV_PREC_LOGIC,
    pub static_segmentation: core::ffi::c_int,
    pub superres_auto_search_type: SUPERRES_AUTO_SEARCH_TYPE,
    pub disable_extra_sc_testing: core::ffi::c_int,
    pub second_alt_ref_filtering: core::ffi::c_int,
    pub adjust_num_frames_for_arf_filtering: core::ffi::c_int,
    pub accurate_bit_estimate: core::ffi::c_int,
    pub weight_calc_level_in_tf: core::ffi::c_int,
    pub allow_sub_blk_me_in_tf: core::ffi::c_int,
    pub ref_frame_mvs_lvl: core::ffi::c_int,
    pub screen_detection_mode2_fast_detection: core::ffi::c_int,
    pub weighted_chroma_distortion: core::ffi::c_int,
}
pub type SUPERRES_AUTO_SEARCH_TYPE = uint8_t;
pub type MV_PREC_LOGIC = uint8_t;
pub type RECODE_LOOP_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimeStamps {
    pub prev_ts_start: int64_t,
    pub prev_ts_end: int64_t,
    pub first_ts_start: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WinnerModeParams {
    pub coeff_opt_thresholds: [[core::ffi::c_uint; 2]; 3],
    pub tx_size_search_methods: [TX_SIZE_SEARCH_METHOD; 3],
    pub use_transform_domain_distortion: [core::ffi::c_uint; 3],
    pub tx_domain_dist_threshold: [core::ffi::c_uint; 3],
    pub skip_txfm_level: [core::ffi::c_uint; 3],
    pub predict_dc_level: [core::ffi::c_uint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalMotionInfo {
    pub search_done: bool,
    pub ref_buf: [*mut YV12_BUFFER_CONFIG; 8],
    pub num_ref_frames: [core::ffi::c_int; 2],
    pub reference_frames: [[FrameDistPair; 7]; 2],
    pub segment_map_w: core::ffi::c_int,
    pub segment_map_h: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameDistPair {
    pub distance: core::ffi::c_int,
    pub frame: MV_REFERENCE_FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODING_CONTEXT {
    pub lf: loopfilter,
    pub cdef_info: CdefInfo,
    pub copy_buffer: YV12_BUFFER_CONFIG,
    pub rc: RATE_CONTROL,
    pub mv_stats: MV_STATS,
    pub frame_number: core::ffi::c_uint,
    pub frame_index_set: FRAME_INDEX_SET,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_OPT {
    pub thresh_mult: [core::ffi::c_int; 169],
    pub threshes: [[[core::ffi::c_int; 169]; 22]; 8],
    pub RDMULT: core::ffi::c_int,
    pub r0: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalFlags {
    pub ref_frame_flags: core::ffi::c_int,
    pub refresh_frame: ExtRefreshFrameFlagsInfo,
    pub refresh_frame_context: bool,
    pub refresh_frame_context_pending: bool,
    pub use_ref_frame_mvs: bool,
    pub use_error_resilient: bool,
    pub use_s_frame: bool,
    pub use_primary_ref_none: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtRefreshFrameFlagsInfo {
    pub last_frame: bool,
    pub golden_frame: bool,
    pub bwd_ref_frame: bool,
    pub alt2_ref_frame: bool,
    pub alt_ref_frame: bool,
    pub update_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefreshFrameInfo {
    pub golden_frame: bool,
    pub bwd_ref_frame: bool,
    pub alt_ref_frame: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ForceIntegerMVInfo {
    pub cs_rate_array: [core::ffi::c_double; 32],
    pub rate_index: core::ffi::c_int,
    pub rate_size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefSearchCtx {
    pub ref_0: *const YV12_BUFFER_CONFIG,
    pub mi_params: *mut CommonModeInfoParams,
    pub plane: [macroblockd_plane; 3],
    pub copy_fn: copy_fn_t,
    pub compute_cdef_dist_fn: compute_cdef_dist_t,
    pub total_strengths: core::ffi::c_int,
    pub coeff_shift: core::ffi::c_int,
    pub damping: core::ffi::c_int,
    pub pick_method: core::ffi::c_int,
    pub num_planes: core::ffi::c_int,
    pub mi_wide_l2: [core::ffi::c_int; 3],
    pub mi_high_l2: [core::ffi::c_int; 3],
    pub xdec: [core::ffi::c_int; 3],
    pub ydec: [core::ffi::c_int; 3],
    pub bsize: [core::ffi::c_int; 3],
    pub nvfb: core::ffi::c_int,
    pub nhfb: core::ffi::c_int,
    pub mse: [*mut [uint64_t; 64]; 2],
    pub sb_index: *mut core::ffi::c_int,
    pub sb_count: core::ffi::c_int,
    pub use_highbitdepth: bool,
}
pub type compute_cdef_dist_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *mut uint16_t,
        *mut cdef_list,
        core::ffi::c_int,
        BLOCK_SIZE,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> uint64_t,
>;
pub type copy_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TemporalFilterCtx {
    pub frames: [*mut YV12_BUFFER_CONFIG; 48],
    pub num_frames: core::ffi::c_int,
    pub output_frame: *mut YV12_BUFFER_CONFIG,
    pub filter_frame_idx: core::ffi::c_int,
    pub compute_frame_diff: core::ffi::c_int,
    pub sf: scale_factors,
    pub noise_levels: [core::ffi::c_double; 3],
    pub num_pels: core::ffi::c_int,
    pub mb_rows: core::ffi::c_int,
    pub mb_cols: core::ffi::c_int,
    pub is_highbitdepth: core::ffi::c_int,
    pub q_factor: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1EncoderConfig {
    pub input_cfg: InputCfg,
    pub frm_dim_cfg: FrameDimensionCfg,
    pub algo_cfg: AlgoCfg,
    pub kf_cfg: KeyFrameCfg,
    pub rc_cfg: RateControlCfg,
    pub q_cfg: QuantizationCfg,
    pub resize_cfg: ResizeCfg,
    pub superres_cfg: SuperResCfg,
    pub twopass_stats_in: aom_fixed_buf_t,
    pub tool_cfg: ToolCfg,
    pub gf_cfg: GFConfig,
    pub tile_cfg: TileConfig,
    pub tune_cfg: TuneCfg,
    pub color_cfg: ColorCfg,
    pub dec_model_cfg: DecoderModelCfg,
    pub ref_frm_cfg: RefFrameCfg,
    pub unit_test_cfg: UnitTestCfg,
    pub motion_mode_cfg: MotionModeCfg,
    pub intra_mode_cfg: IntraModeCfg,
    pub txfm_cfg: TxfmSizeTypeCfg,
    pub comp_type_cfg: CompoundTypeCfg,
    pub part_cfg: PartitionCfg,
    pub cost_upd_freq: CostUpdateFreq,
    pub noise_level: core::ffi::c_float,
    pub noise_block_size: core::ffi::c_int,
    pub enable_dnl_denoising: core::ffi::c_int,
    pub tier_mask: core::ffi::c_uint,
    pub border_in_pixels: core::ffi::c_int,
    pub max_threads: core::ffi::c_int,
    pub speed: core::ffi::c_int,
    pub enable_low_complexity_decode: core::ffi::c_uint,
    pub target_seq_level_idx: [AV1_LEVEL; 32],
    pub profile: BITSTREAM_PROFILE,
    pub pass: aom_enc_pass,
    pub passes: core::ffi::c_int,
    pub two_pass_output: *const core::ffi::c_char,
    pub second_pass_log: *const core::ffi::c_char,
    pub mode: MODE,
    pub row_mt: bool,
    pub fp_mt: bool,
    pub use_highbitdepth: bool,
    pub save_as_annexb: bool,
    pub partition_info_path: *const core::ffi::c_char,
    pub enable_rate_guide_deltaq: core::ffi::c_uint,
    pub rate_distribution_info: *const core::ffi::c_char,
    pub strict_level_conformance: core::ffi::c_int,
    pub kf_max_pyr_height: core::ffi::c_int,
    pub sb_qp_sweep: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CostUpdateFreq {
    pub coeff: COST_UPDATE_TYPE,
    pub mode: COST_UPDATE_TYPE,
    pub mv: COST_UPDATE_TYPE,
    pub dv: COST_UPDATE_TYPE,
}
pub type COST_UPDATE_TYPE = core::ffi::c_uint;
pub const NUM_COST_UPDATE_TYPES: COST_UPDATE_TYPE = 4;
pub const COST_UPD_OFF: COST_UPDATE_TYPE = 3;
pub const COST_UPD_TILE: COST_UPDATE_TYPE = 2;
pub const COST_UPD_SBROW: COST_UPDATE_TYPE = 1;
pub const COST_UPD_SB: COST_UPDATE_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionCfg {
    pub enable_rect_partitions: bool,
    pub enable_ab_partitions: bool,
    pub enable_1to4_partitions: bool,
    pub min_partition_size: BLOCK_SIZE,
    pub max_partition_size: BLOCK_SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompoundTypeCfg {
    pub enable_dist_wtd_comp: bool,
    pub enable_masked_comp: bool,
    pub enable_smooth_interintra: bool,
    pub enable_diff_wtd_comp: bool,
    pub enable_interinter_wedge: bool,
    pub enable_interintra_wedge: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmSizeTypeCfg {
    pub enable_tx64: bool,
    pub enable_flip_idtx: bool,
    pub enable_rect_tx: bool,
    pub reduced_tx_type_set: bool,
    pub use_intra_dct_only: bool,
    pub use_inter_dct_only: bool,
    pub use_intra_default_tx_only: bool,
    pub enable_tx_size_search: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntraModeCfg {
    pub enable_intra_edge_filter: bool,
    pub enable_filter_intra: bool,
    pub enable_smooth_intra: bool,
    pub enable_paeth_intra: bool,
    pub enable_cfl_intra: bool,
    pub enable_directional_intra: bool,
    pub enable_diagonal_intra: bool,
    pub enable_angle_delta: bool,
    pub auto_intra_tools_off: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModeCfg {
    pub enable_warped_motion: bool,
    pub allow_warped_motion: bool,
    pub enable_obmc: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnitTestCfg {
    pub motion_vector_unit_test: core::ffi::c_uint,
    pub sb_multipass_unit_test: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefFrameCfg {
    pub max_reference_frames: core::ffi::c_uint,
    pub enable_reduced_reference_set: bool,
    pub enable_onesided_comp: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderModelCfg {
    pub timing_info: aom_timing_info_t,
    pub num_units_in_decoding_tick: uint32_t,
    pub decoder_model_info_present_flag: bool,
    pub display_model_info_present_flag: bool,
    pub timing_info_present: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorCfg {
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub color_range: aom_color_range_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TuneCfg {
    pub vmaf_model_path: *const core::ffi::c_char,
    pub film_grain_table_filename: *const core::ffi::c_char,
    pub tuning: aom_tune_metric,
    pub content: aom_tune_content,
    pub film_grain_test_vector: core::ffi::c_int,
    pub dist_metric: aom_dist_metric,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileConfig {
    pub num_tile_groups: core::ffi::c_uint,
    pub mtu: core::ffi::c_uint,
    pub tile_columns: core::ffi::c_int,
    pub tile_rows: core::ffi::c_int,
    pub tile_width_count: core::ffi::c_int,
    pub tile_height_count: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub enable_large_scale_tile: bool,
    pub enable_single_tile_decoding: bool,
    pub enable_ext_tile_debug: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GFConfig {
    pub lag_in_frames: core::ffi::c_int,
    pub min_gf_interval: core::ffi::c_int,
    pub max_gf_interval: core::ffi::c_int,
    pub gf_min_pyr_height: core::ffi::c_int,
    pub gf_max_pyr_height: core::ffi::c_int,
    pub enable_auto_arf: bool,
    pub enable_auto_brf: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ToolCfg {
    pub bit_depth: aom_bit_depth_t,
    pub superblock_size: aom_superblock_size_t,
    pub enable_deltalf_mode: bool,
    pub cdef_control: CDEF_CONTROL,
    pub enable_restoration: bool,
    pub force_video_mode: bool,
    pub error_resilient_mode: bool,
    pub frame_parallel_decoding_mode: bool,
    pub enable_monochrome: bool,
    pub full_still_picture_hdr: bool,
    pub enable_dual_filter: bool,
    pub enable_order_hint: bool,
    pub ref_frame_mvs_present: bool,
    pub enable_ref_frame_mvs: bool,
    pub enable_interintra_comp: bool,
    pub enable_global_motion: bool,
    pub enable_palette: bool,
}
pub type CDEF_CONTROL = core::ffi::c_uint;
pub const CDEF_ADAPTIVE: CDEF_CONTROL = 3;
pub const CDEF_REFERENCE: CDEF_CONTROL = 2;
pub const CDEF_ALL: CDEF_CONTROL = 1;
pub const CDEF_NONE: CDEF_CONTROL = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SuperResCfg {
    pub superres_qthresh: core::ffi::c_int,
    pub superres_kf_qthresh: core::ffi::c_int,
    pub superres_scale_denominator: uint8_t,
    pub superres_kf_scale_denominator: uint8_t,
    pub superres_mode: aom_superres_mode,
    pub enable_superres: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResizeCfg {
    pub resize_mode: RESIZE_MODE,
    pub resize_scale_denominator: uint8_t,
    pub resize_kf_scale_denominator: uint8_t,
}
pub type RESIZE_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuantizationCfg {
    pub use_fixed_qp_offsets: core::ffi::c_int,
    pub qm_minlevel: core::ffi::c_int,
    pub qm_maxlevel: core::ffi::c_int,
    pub quant_b_adapt: core::ffi::c_int,
    pub aq_mode: AQ_MODE,
    pub deltaq_mode: DELTAQ_MODE,
    pub deltaq_strength: core::ffi::c_uint,
    pub enable_chroma_deltaq: bool,
    pub enable_hdr_deltaq: bool,
    pub using_qm: bool,
}
pub type DELTAQ_MODE = uint8_t;
pub type AQ_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RateControlCfg {
    pub starting_buffer_level_ms: int64_t,
    pub optimal_buffer_level_ms: int64_t,
    pub maximum_buffer_size_ms: int64_t,
    pub target_bandwidth: int64_t,
    pub vbr_corpus_complexity_lap: core::ffi::c_uint,
    pub max_intra_bitrate_pct: core::ffi::c_uint,
    pub max_inter_bitrate_pct: core::ffi::c_uint,
    pub gf_cbr_boost_pct: core::ffi::c_uint,
    pub min_cr: core::ffi::c_uint,
    pub drop_frames_water_mark: core::ffi::c_int,
    pub under_shoot_pct: core::ffi::c_int,
    pub over_shoot_pct: core::ffi::c_int,
    pub worst_allowed_q: core::ffi::c_int,
    pub best_allowed_q: core::ffi::c_int,
    pub cq_level: core::ffi::c_int,
    pub mode: aom_rc_mode,
    pub vbrbias: core::ffi::c_int,
    pub vbrmin_section: core::ffi::c_int,
    pub vbrmax_section: core::ffi::c_int,
    pub max_consec_drop_ms: core::ffi::c_int,
    pub force_max_q: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyFrameCfg {
    pub key_freq_min: core::ffi::c_int,
    pub key_freq_max: core::ffi::c_int,
    pub enable_keyframe_filtering: core::ffi::c_int,
    pub sframe_dist: core::ffi::c_int,
    pub sframe_mode: core::ffi::c_int,
    pub auto_key: bool,
    pub fwd_kf_dist: core::ffi::c_int,
    pub fwd_kf_enabled: bool,
    pub enable_sframe: bool,
    pub enable_intrabc: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlgoCfg {
    pub sharpness: core::ffi::c_int,
    pub enable_adaptive_sharpness: bool,
    pub disable_trellis_quant: core::ffi::c_int,
    pub arnr_max_frames: core::ffi::c_int,
    pub arnr_strength: core::ffi::c_int,
    pub cdf_update_mode: uint8_t,
    pub enable_tpl_model: bool,
    pub enable_overlay: bool,
    pub loopfilter_control: LOOPFILTER_CONTROL,
    pub skip_postproc_filtering: bool,
    pub mode_ref_delta_enabled: core::ffi::c_int,
    pub screen_detection_mode: aom_screen_detection_mode,
}
pub type LOOPFILTER_CONTROL = core::ffi::c_uint;
pub const LOOPFILTER_SELECTIVELY: LOOPFILTER_CONTROL = 3;
pub const LOOPFILTER_REFERENCE: LOOPFILTER_CONTROL = 2;
pub const LOOPFILTER_ALL: LOOPFILTER_CONTROL = 1;
pub const LOOPFILTER_NONE: LOOPFILTER_CONTROL = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameDimensionCfg {
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub forced_max_frame_width: core::ffi::c_int,
    pub forced_max_frame_height: core::ffi::c_int,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputCfg {
    pub init_framerate: core::ffi::c_double,
    pub input_bit_depth: core::ffi::c_uint,
    pub limit: core::ffi::c_uint,
    pub chroma_subsampling_x: core::ffi::c_uint,
    pub chroma_subsampling_y: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoeffBufferPool {
    pub tcoeff: *mut tran_low_t,
    pub eobs: *mut uint16_t,
    pub entropy_ctx: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBMIExtFrameBufferInfo {
    pub frame_base: *mut MB_MODE_INFO_EXT_FRAME,
    pub alloc_size: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_COUNTS {
    pub switchable_interp: [[core::ffi::c_uint; 3]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadData {
    pub mb: MACROBLOCK,
    pub mv_costs_alloc: *mut MvCosts,
    pub dv_costs_alloc: *mut IntraBCMVCosts,
    pub rd_counts: RD_COUNTS,
    pub counts: *mut FRAME_COUNTS,
    pub shared_coeff_buf: PC_TREE_SHARED_BUFFERS,
    pub sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    pub sms_root: *mut SIMPLE_MOTION_DATA_TREE,
    pub hash_value_buffer: [*mut uint32_t; 2],
    pub obmc_buffer: OBMCBuffer,
    pub palette_buffer: *mut PALETTE_BUFFER,
    pub comp_rd_buffer: CompoundTypeRdBuffers,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub abs_sum_level: uint64_t,
    pub tmp_pred_bufs: [*mut uint8_t; 2],
    pub upsample_pred: *mut uint8_t,
    pub wiener_tmp_pred_buf: *mut uint8_t,
    pub intrabc_used: core::ffi::c_int,
    pub deltaq_used: core::ffi::c_int,
    pub coefficient_size: core::ffi::c_int,
    pub max_mv_magnitude: core::ffi::c_int,
    pub interp_filter_selected: [core::ffi::c_int; 4],
    pub tctx: *mut FRAME_CONTEXT,
    pub vt64x64: *mut VP64x64,
    pub num_64x64_blocks: int32_t,
    pub firstpass_ctx: *mut PICK_MODE_CONTEXT,
    pub tf_data: TemporalFilterData,
    pub tpl_tmp_buffers: TplBuffers,
    pub tpl_txfm_stats: TplTxfmStats,
    pub gm_data: GlobalMotionData,
    pub pixel_gradient_info: *mut PixelLevelGradientInfo,
    pub src_var_info_of_4x4_sub_blocks: *mut Block4x4VarInfo,
    pub pc_root: *mut PC_TREE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PC_TREE {
    pub partitioning: PARTITION_TYPE,
    pub block_size: BLOCK_SIZE,
    pub none: *mut PICK_MODE_CONTEXT,
    pub horizontal: [*mut PICK_MODE_CONTEXT; 2],
    pub vertical: [*mut PICK_MODE_CONTEXT; 2],
    pub horizontala: [*mut PICK_MODE_CONTEXT; 3],
    pub horizontalb: [*mut PICK_MODE_CONTEXT; 3],
    pub verticala: [*mut PICK_MODE_CONTEXT; 3],
    pub verticalb: [*mut PICK_MODE_CONTEXT; 3],
    pub horizontal4: [*mut PICK_MODE_CONTEXT; 4],
    pub vertical4: [*mut PICK_MODE_CONTEXT; 4],
    pub split: [*mut PC_TREE; 4],
    pub index: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PICK_MODE_CONTEXT {
    pub mic: MB_MODE_INFO,
    pub mbmi_ext_best: MB_MODE_INFO_EXT_FRAME,
    pub color_index_map: [*mut uint8_t; 2],
    pub coeff: [*mut tran_low_t; 3],
    pub qcoeff: [*mut tran_low_t; 3],
    pub dqcoeff: [*mut tran_low_t; 3],
    pub eobs: [*mut uint16_t; 3],
    pub txb_entropy_ctx: [*mut uint8_t; 3],
    pub tx_type_map: *mut uint8_t,
    pub num_4x4_blk: core::ffi::c_int,
    pub skippable: core::ffi::c_int,
    pub rd_stats: RD_STATS,
    pub rd_mode_is_ready: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalMotionData {
    pub motion_models: [MotionModel; 1],
    pub segment_map: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TplTxfmStats {
    pub ready: core::ffi::c_int,
    pub abs_coeff_sum: [core::ffi::c_double; 256],
    pub abs_coeff_mean: [core::ffi::c_double; 256],
    pub txfm_block_count: core::ffi::c_int,
    pub coeff_num: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TplBuffers {
    pub predictor8: *mut uint8_t,
    pub src_diff: *mut int16_t,
    pub coeff: *mut tran_low_t,
    pub qcoeff: *mut tran_low_t,
    pub dqcoeff: *mut tran_low_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TemporalFilterData {
    pub diff: FRAME_DIFF,
    pub tmp_mbmi: *mut MB_MODE_INFO,
    pub accum: *mut uint32_t,
    pub count: *mut uint16_t,
    pub pred: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_DIFF {
    pub sum: int64_t,
    pub sse: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP64x64 {
    pub part_variances: VPVariance,
    pub split: [VP32x32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP32x32 {
    pub part_variances: VPVariance,
    pub split: [VP16x16; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP16x16 {
    pub part_variances: VPVariance,
    pub split: [VP8x8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8x8 {
    pub part_variances: VPVariance,
    pub split: [VP4x4; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP4x4 {
    pub part_variances: VPVariance,
    pub split: [VPartVar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPartVar {
    pub sum_square_error: uint32_t,
    pub sum_error: int32_t,
    pub log2_count: core::ffi::c_int,
    pub variance: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPVariance {
    pub none: VPartVar,
    pub horz: [VPartVar; 2],
    pub vert: [VPartVar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIMPLE_MOTION_DATA_TREE {
    pub block_size: BLOCK_SIZE,
    pub partitioning: PARTITION_TYPE,
    pub split: [*mut SIMPLE_MOTION_DATA_TREE; 4],
    pub start_mvs: [FULLPEL_MV; 8],
    pub sms_none_feat: [core::ffi::c_uint; 2],
    pub sms_rect_feat: [core::ffi::c_uint; 8],
    pub sms_none_valid: core::ffi::c_int,
    pub sms_rect_valid: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PC_TREE_SHARED_BUFFERS {
    pub coeff_buf: [*mut tran_low_t; 3],
    pub qcoeff_buf: [*mut tran_low_t; 3],
    pub dqcoeff_buf: [*mut tran_low_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_COUNTS {
    pub compound_ref_used_flag: core::ffi::c_int,
    pub skip_mode_used_flag: core::ffi::c_int,
    pub tx_type_used: [[core::ffi::c_int; 16]; 19],
    pub obmc_used: [[core::ffi::c_int; 2]; 22],
    pub warped_used: [core::ffi::c_int; 2],
    pub newmv_or_intra_blocks: core::ffi::c_int,
    pub seg_tmp_pred_cost: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncQuantDequantParams {
    pub quants: QUANTS,
    pub dequants: Dequants,
    pub prev_deltaq_params: DeltaQuantParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeltaQuantParams {
    pub y_dc_delta_q: core::ffi::c_int,
    pub u_dc_delta_q: core::ffi::c_int,
    pub u_ac_delta_q: core::ffi::c_int,
    pub v_dc_delta_q: core::ffi::c_int,
    pub v_ac_delta_q: core::ffi::c_int,
    pub sharpness: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dequants {
    pub __aom_align_y_dequant_QTX: [crate::platform::Align16; 0],

    pub y_dequant_QTX: [[int16_t; 8]; 256],
    pub __aom_align_u_dequant_QTX: [crate::platform::Align16; 0],
    pub u_dequant_QTX: [[int16_t; 8]; 256],
    pub __aom_align_v_dequant_QTX: [crate::platform::Align16; 0],
    pub v_dequant_QTX: [[int16_t; 8]; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUANTS {
    pub __aom_align_y_quant: [crate::platform::Align16; 0],

    pub y_quant: [[int16_t; 8]; 256],
    pub __aom_align_y_quant_shift: [crate::platform::Align16; 0],
    pub y_quant_shift: [[int16_t; 8]; 256],
    pub __aom_align_y_zbin: [crate::platform::Align16; 0],
    pub y_zbin: [[int16_t; 8]; 256],
    pub __aom_align_y_round: [crate::platform::Align16; 0],
    pub y_round: [[int16_t; 8]; 256],
    pub __aom_align_y_quant_fp: [crate::platform::Align16; 0],
    pub y_quant_fp: [[int16_t; 8]; 256],
    pub __aom_align_u_quant_fp: [crate::platform::Align16; 0],
    pub u_quant_fp: [[int16_t; 8]; 256],
    pub __aom_align_v_quant_fp: [crate::platform::Align16; 0],
    pub v_quant_fp: [[int16_t; 8]; 256],
    pub __aom_align_y_round_fp: [crate::platform::Align16; 0],
    pub y_round_fp: [[int16_t; 8]; 256],
    pub __aom_align_u_round_fp: [crate::platform::Align16; 0],
    pub u_round_fp: [[int16_t; 8]; 256],
    pub __aom_align_v_round_fp: [crate::platform::Align16; 0],
    pub v_round_fp: [[int16_t; 8]; 256],
    pub __aom_align_u_quant: [crate::platform::Align16; 0],
    pub u_quant: [[int16_t; 8]; 256],
    pub __aom_align_v_quant: [crate::platform::Align16; 0],
    pub v_quant: [[int16_t; 8]; 256],
    pub __aom_align_u_quant_shift: [crate::platform::Align16; 0],
    pub u_quant_shift: [[int16_t; 8]; 256],
    pub __aom_align_v_quant_shift: [crate::platform::Align16; 0],
    pub v_quant_shift: [[int16_t; 8]; 256],
    pub __aom_align_u_zbin: [crate::platform::Align16; 0],
    pub u_zbin: [[int16_t; 8]; 256],
    pub __aom_align_v_zbin: [crate::platform::Align16; 0],
    pub v_zbin: [[int16_t; 8]; 256],
    pub __aom_align_u_round: [crate::platform::Align16; 0],
    pub u_round: [[int16_t; 8]; 256],
    pub __aom_align_v_round: [crate::platform::Align16; 0],
    pub v_round: [[int16_t; 8]; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_PRIMARY {
    pub parallel_cpi: [*mut AV1_COMP; 4],
    pub parallel_frames_data: [AV1_COMP_DATA; 3],
    pub ref_frame_map_copy: [*mut RefCntBuffer; 8],
    pub ts_start_last_show_frame: int64_t,
    pub ts_end_last_show_frame: int64_t,
    pub num_fp_contexts: core::ffi::c_int,
    pub filter_level: [core::ffi::c_int; 2],
    pub filter_level_u: core::ffi::c_int,
    pub filter_level_v: core::ffi::c_int,
    pub cpi: *mut AV1_COMP,
    pub cpi_lap: *mut AV1_COMP,
    pub lookahead: *mut lookahead_ctx,
    pub seq_params_locked: core::ffi::c_int,
    pub output_pkt_list: *mut aom_codec_pkt_list,
    pub internal_altref_allowed: core::ffi::c_int,
    pub show_existing_alt_ref: core::ffi::c_int,
    pub gf_group: GF_GROUP,
    pub gf_state: GF_STATE,
    pub lap_enabled: core::ffi::c_int,
    pub level_params: AV1LevelParams,
    pub b_calculate_psnr: core::ffi::c_int,
    pub frames_left: core::ffi::c_int,
    pub twopass: TWO_PASS,
    pub p_rc: PRIMARY_RATE_CONTROL,
    pub tf_info: TEMPORAL_FILTER_INFO,
    pub seq_params: SequenceHeader,
    pub use_svc: core::ffi::c_int,
    pub buffer_removal_time_present: bool,
    pub number_temporal_layers: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
    pub error: aom_internal_error_info,
    pub fn_ptr: [aom_variance_fn_ptr_t; 22],
    pub tpl_sb_rdmult_scaling_factors: *mut core::ffi::c_double,
    pub tpl_data: TplParams,
    pub mv_stats: MV_STATS,
    pub fb_of_context_type: [core::ffi::c_int; 8],
    pub p_mt_info: PrimaryMultiThreadInfo,
    pub frame_probs: FrameProbInfo,
    pub valid_gm_model_found: [core::ffi::c_int; 7],
    pub rtc_ref: RTC_REF,
    pub intra_row_mt_sync: AV1EncRowMultiThreadSync,
    pub b_freeze_internal_state: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RTC_REF {
    pub reference: [core::ffi::c_int; 7],
    pub ref_idx: [core::ffi::c_int; 7],
    pub refresh: [core::ffi::c_int; 8],
    pub set_ref_frame_config: core::ffi::c_int,
    pub non_reference_frame: core::ffi::c_int,
    pub ref_frame_comp: [core::ffi::c_int; 3],
    pub gld_idx_1layer: core::ffi::c_int,
    pub buffer_time_index: [core::ffi::c_uint; 8],
    pub buffer_spatial_layer: [core::ffi::c_uchar; 8],
    pub reference_was_previous_frame: bool,
    pub bias_recovery_frame: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrimaryMultiThreadInfo {
    pub num_workers: core::ffi::c_int,
    pub num_mod_workers: [core::ffi::c_int; 12],
    pub workers: *mut AVxWorker,
    pub tile_thr_data: *mut EncWorkerData,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub p_workers: [*mut AVxWorker; 4],
    pub p_num_workers: core::ffi::c_int,
    pub prev_num_enc_workers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TplParams {
    pub ready: core::ffi::c_int,
    pub tpl_stats_block_mis_log2: uint8_t,
    pub tpl_bsize_1d: uint8_t,
    pub tpl_stats_buffer: [TplDepFrame; 105],
    pub tpl_stats_pool: [*mut TplDepStats; 48],
    pub txfm_stats_list: *mut TplTxfmStats,
    pub tpl_rec_pool: [YV12_BUFFER_CONFIG; 48],
    pub tpl_frame: *mut TplDepFrame,
    pub sf: scale_factors,
    pub frame_idx: core::ffi::c_int,
    pub src_ref_frame: [*const YV12_BUFFER_CONFIG; 7],
    pub ref_frame: [*const YV12_BUFFER_CONFIG; 7],
    pub prev_gop_arf_src: YV12_BUFFER_CONFIG,
    pub prev_gop_arf_tpl_recon: YV12_BUFFER_CONFIG,
    pub prev_gop_arf_disp_order: int64_t,
    pub tpl_mt_sync: AV1TplRowMultiThreadSync,
    pub border_in_pixels: core::ffi::c_int,
    pub r0_adjust_factor: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TplDepFrame {
    pub is_valid: uint8_t,
    pub tpl_stats_ptr: *mut TplDepStats,
    pub gf_picture: *const YV12_BUFFER_CONFIG,
    pub rec_picture: *mut YV12_BUFFER_CONFIG,
    pub ref_map_index: [core::ffi::c_int; 8],
    pub stride: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub base_rdmult: core::ffi::c_int,
    pub frame_display_index: uint32_t,
    pub use_pred_sad: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TplDepStats {
    pub srcrf_sse: int64_t,
    pub srcrf_dist: int64_t,
    pub recrf_sse: int64_t,
    pub recrf_dist: int64_t,
    pub intra_sse: int64_t,
    pub intra_dist: int64_t,
    pub cmp_recrf_dist: [int64_t; 2],
    pub mc_dep_rate: int64_t,
    pub mc_dep_dist: int64_t,
    pub pred_error: [int64_t; 7],
    pub intra_cost: int32_t,
    pub inter_cost: int32_t,
    pub srcrf_rate: int32_t,
    pub recrf_rate: int32_t,
    pub intra_rate: int32_t,
    pub cmp_recrf_rate: [int32_t; 2],
    pub mv: [int_mv; 7],
    pub ref_frame_index: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEMPORAL_FILTER_INFO {
    pub is_temporal_filter_on: core::ffi::c_int,
    pub tf_buf: [YV12_BUFFER_CONFIG; 2],
    pub tf_buf_second_arf: YV12_BUFFER_CONFIG,
    pub frame_diff: [FRAME_DIFF; 2],
    pub tf_buf_gf_index: [core::ffi::c_int; 2],
    pub tf_buf_display_index_offset: [core::ffi::c_int; 2],
    pub tf_buf_valid: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWO_PASS {
    pub section_intra_rating: core::ffi::c_uint,
    pub frame_stats_arr: [*mut FIRSTPASS_STATS; 49],
    pub frame_stats_next_idx: core::ffi::c_int,
    pub stats_buf_ctx: *mut STATS_BUFFER_CTX,
    pub firstpass_info: FIRSTPASS_INFO,
    pub first_pass_done: core::ffi::c_int,
    pub bits_left: int64_t,
    pub modified_error_min: core::ffi::c_double,
    pub modified_error_max: core::ffi::c_double,
    pub modified_error_left: core::ffi::c_double,
    pub kf_group_bits: int64_t,
    pub kf_group_error_left: core::ffi::c_double,
    pub bpm_factor: core::ffi::c_double,
    pub rolling_arf_group_target_bits: core::ffi::c_int,
    pub rolling_arf_group_actual_bits: core::ffi::c_int,
    pub sr_update_lag: core::ffi::c_int,
    pub kf_zeromotion_pct: core::ffi::c_int,
    pub last_kfgroup_zeromotion_pct: core::ffi::c_int,
    pub extend_minq: core::ffi::c_int,
    pub extend_maxq: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIRSTPASS_INFO {
    pub static_stats_buf: [FIRSTPASS_STATS; 49],
    pub stats_buf: *mut FIRSTPASS_STATS,
    pub stats_buf_size: core::ffi::c_int,
    pub start_index: core::ffi::c_int,
    pub stats_count: core::ffi::c_int,
    pub cur_index: core::ffi::c_int,
    pub future_stats_count: core::ffi::c_int,
    pub past_stats_count: core::ffi::c_int,
    pub total_stats: FIRSTPASS_STATS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATS_BUFFER_CTX {
    pub stats_in_start: *mut FIRSTPASS_STATS,
    pub stats_in_end: *mut FIRSTPASS_STATS,
    pub stats_in_buf_end: *mut FIRSTPASS_STATS,
    pub total_stats: *mut FIRSTPASS_STATS,
    pub total_left_stats: *mut FIRSTPASS_STATS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LevelParams {
    pub target_seq_level_idx: [AV1_LEVEL; 32],
    pub keep_level_stats: uint32_t,
    pub level_info: [*mut AV1LevelInfo; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LevelInfo {
    pub level_stats: AV1LevelStats,
    pub level_spec: AV1LevelSpec,
    pub frame_window_buffer: FrameWindowBuffer,
    pub decoder_models: [DECODER_MODEL; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODER_MODEL {
    pub status: DECODER_MODEL_STATUS,
    pub mode: DECODER_MODEL_MODE,
    pub is_low_delay_mode: bool,
    pub level: AV1_LEVEL,
    pub encoder_buffer_delay: core::ffi::c_int,
    pub decoder_buffer_delay: core::ffi::c_int,
    pub num_ticks_per_picture: core::ffi::c_int,
    pub initial_display_delay: core::ffi::c_int,
    pub decode_rate: int64_t,
    pub display_clock_tick: core::ffi::c_double,
    pub current_time: core::ffi::c_double,
    pub initial_presentation_delay: core::ffi::c_double,
    pub bit_rate: core::ffi::c_double,
    pub num_frame: core::ffi::c_int,
    pub num_decoded_frame: core::ffi::c_int,
    pub num_shown_frame: core::ffi::c_int,
    pub vbi: [core::ffi::c_int; 8],
    pub frame_buffer_pool: [FRAME_BUFFER; 10],
    pub dfg_interval_queue: DFG_INTERVAL_QUEUE,
    pub first_bit_arrival_time: core::ffi::c_double,
    pub last_bit_arrival_time: core::ffi::c_double,
    pub coded_bits: size_t,
    pub removal_time: core::ffi::c_double,
    pub presentation_time: core::ffi::c_double,
    pub decode_samples: core::ffi::c_int,
    pub display_samples: core::ffi::c_int,
    pub max_display_rate: core::ffi::c_double,
    pub max_decode_rate: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DFG_INTERVAL_QUEUE {
    pub head: core::ffi::c_int,
    pub size: core::ffi::c_int,
    pub total_interval: core::ffi::c_double,
    pub buf: [DFG_INTERVAL; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DFG_INTERVAL {
    pub first_bit_arrival_time: core::ffi::c_double,
    pub last_bit_arrival_time: core::ffi::c_double,
    pub removal_time: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAME_BUFFER {
    pub decoder_ref_count: core::ffi::c_int,
    pub player_ref_count: core::ffi::c_int,
    pub display_index: core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub presentation_time: core::ffi::c_double,
}
pub type DECODER_MODEL_MODE = uint8_t;
pub type DECODER_MODEL_STATUS = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameWindowBuffer {
    pub buf: [FrameRecord; 256],
    pub num: core::ffi::c_int,
    pub start: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameRecord {
    pub ts_start: int64_t,
    pub ts_end: int64_t,
    pub encoded_size_in_bytes: size_t,
    pub pic_size: core::ffi::c_int,
    pub frame_header_count: core::ffi::c_int,
    pub tiles: core::ffi::c_int,
    pub show_frame: core::ffi::c_int,
    pub show_existing_frame: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LevelSpec {
    pub level: AV1_LEVEL,
    pub max_picture_size: core::ffi::c_int,
    pub max_h_size: core::ffi::c_int,
    pub max_v_size: core::ffi::c_int,
    pub max_header_rate: core::ffi::c_int,
    pub max_tile_rate: core::ffi::c_int,
    pub max_tiles: core::ffi::c_int,
    pub max_tile_cols: core::ffi::c_int,
    pub max_display_rate: int64_t,
    pub max_decode_rate: int64_t,
    pub main_mbps: core::ffi::c_double,
    pub high_mbps: core::ffi::c_double,
    pub main_cr: core::ffi::c_double,
    pub high_cr: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LevelStats {
    pub max_bitrate: core::ffi::c_int,
    pub max_tile_size: core::ffi::c_int,
    pub max_superres_tile_width: core::ffi::c_int,
    pub min_cropped_tile_width: core::ffi::c_int,
    pub min_cropped_tile_height: core::ffi::c_int,
    pub tile_width_is_valid: core::ffi::c_int,
    pub min_frame_width: core::ffi::c_int,
    pub min_frame_height: core::ffi::c_int,
    pub total_compressed_size: core::ffi::c_double,
    pub total_time_encoded: core::ffi::c_double,
    pub min_cr: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GF_STATE {
    pub arf_gf_boost_lst: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GF_GROUP {
    pub update_type: [FRAME_UPDATE_TYPE; 250],
    pub arf_src_offset: [core::ffi::c_uchar; 250],
    pub cur_frame_idx: [core::ffi::c_uchar; 250],
    pub layer_depth: [core::ffi::c_int; 250],
    pub arf_boost: [core::ffi::c_int; 250],
    pub max_layer_depth: core::ffi::c_int,
    pub max_layer_depth_allowed: core::ffi::c_int,
    pub q_val: [core::ffi::c_int; 250],
    pub rdmult_val: [core::ffi::c_int; 250],
    pub bit_allocation: [core::ffi::c_int; 250],
    pub frame_type: [FRAME_TYPE; 250],
    pub refbuf_state: [REFBUF_STATE; 250],
    pub arf_index: core::ffi::c_int,
    pub size: core::ffi::c_int,
    pub src_offset: [core::ffi::c_int; 250],
    pub display_idx: [core::ffi::c_int; 250],
    pub ref_frame_list: [[int8_t; 8]; 250],
    pub update_ref_idx: [core::ffi::c_int; 250],
    pub primary_ref_idx: [core::ffi::c_int; 250],
    pub frame_parallel_level: [core::ffi::c_int; 250],
    pub is_frame_non_ref: [bool; 250],
    pub is_frame_dropped: [bool; 250],
    pub skip_frame_refresh: [[core::ffi::c_int; 8]; 250],
    pub skip_frame_as_ref: [core::ffi::c_int; 250],
    pub is_sframe_due: bool,
    pub use_ext_ref_frame_map: [core::ffi::c_int; 250],
}
pub type REFBUF_STATE = uint8_t;
pub type FRAME_UPDATE_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookahead_ctx {
    pub max_sz: core::ffi::c_int,
    pub write_idx: core::ffi::c_int,
    pub read_ctxs: [read_ctx; 2],
    pub buf: *mut lookahead_entry,
    pub push_frame_count: core::ffi::c_int,
    pub max_pre_frames: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookahead_entry {
    pub img: YV12_BUFFER_CONFIG,
    pub ts_start: int64_t,
    pub ts_end: int64_t,
    pub display_idx: core::ffi::c_int,
    pub flags: aom_enc_frame_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_ctx {
    pub sz: core::ffi::c_int,
    pub read_idx: core::ffi::c_int,
    pub pop_sz: core::ffi::c_int,
    pub valid: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_COMP_DATA {
    pub cx_data: *mut core::ffi::c_uchar,
    pub cx_data_sz: size_t,
    pub frame_size: size_t,
    pub lib_flags: core::ffi::c_uint,
    pub ts_frame_start: int64_t,
    pub ts_frame_end: int64_t,
    pub flush: core::ffi::c_int,
    pub timestamp_ratio: *const aom_rational64_t,
    pub pop_lookahead: core::ffi::c_int,
}
pub type aom_rational64_t = aom_rational64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational64 {
    pub num: int64_t,
    pub den: core::ffi::c_int,
}
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const DRY_RUN_COSTCOEFFS: C2RustUnnamed_24 = 2;
pub const DRY_RUN_NORMAL: C2RustUnnamed_24 = 1;
pub const OUTPUT_ENABLED: C2RustUnnamed_24 = 0;
pub type RUN_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUANT_PARAM {
    pub log_scale: core::ffi::c_int,
    pub tx_size: TX_SIZE,
    pub qmatrix: *const qm_val_t,
    pub iqmatrix: *const qm_val_t,
    pub use_quant_b_adapt: core::ffi::c_int,
    pub use_optimize_b: core::ffi::c_int,
    pub xform_quant_idx: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txb_ctx {
    pub txb_skip_ctx: core::ffi::c_int,
    pub dc_sign_ctx: core::ffi::c_int,
}
pub type TXB_CTX = txb_ctx;
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const AV1_XFORM_QUANT_TYPES: C2RustUnnamed_25 = 4;
pub const AV1_XFORM_QUANT_SKIP_QUANT: C2RustUnnamed_25 = 3;
pub const AV1_XFORM_QUANT_DC: C2RustUnnamed_25 = 2;
pub const AV1_XFORM_QUANT_B: C2RustUnnamed_25 = 1;
pub const AV1_XFORM_QUANT_FP: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = core::ffi::c_uint;
pub const NO_ESTIMATE_YRD_TRELLIS_OPT: C2RustUnnamed_26 = 3;
pub const FINAL_PASS_TRELLIS_OPT: C2RustUnnamed_26 = 2;
pub const FULL_TRELLIS_OPT: C2RustUnnamed_26 = 1;
pub const NO_TRELLIS_OPT: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const TX_TYPE_PRUNE_5: C2RustUnnamed_27 = 5;
pub const TX_TYPE_PRUNE_4: C2RustUnnamed_27 = 4;
pub const TX_TYPE_PRUNE_3: C2RustUnnamed_27 = 3;
pub const TX_TYPE_PRUNE_2: C2RustUnnamed_27 = 2;
pub const TX_TYPE_PRUNE_1: C2RustUnnamed_27 = 1;
pub const TX_TYPE_PRUNE_0: C2RustUnnamed_27 = 0;
pub type model_rd_for_sb_type = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMP,
        BLOCK_SIZE,
        *mut MACROBLOCK,
        *mut MACROBLOCKD,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut core::ffi::c_int,
        *mut int64_t,
        *mut uint8_t,
        *mut int64_t,
        *mut core::ffi::c_int,
        *mut int64_t,
        *mut int64_t,
    ) -> (),
>;
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const FTXS_USE_TRANSFORM_DOMAIN: C2RustUnnamed_28 = 4;
pub const FTXS_DISABLE_TRELLIS_OPT: C2RustUnnamed_28 = 2;
pub const FTXS_DCT_AND_1D_DCT_ONLY: C2RustUnnamed_28 = 1;
pub const FTXS_NONE: C2RustUnnamed_28 = 0;
pub type FAST_TX_SEARCH_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rdcost_block_args {
    pub cpi: *const AV1_COMP,
    pub x: *mut MACROBLOCK,
    pub t_above: [ENTROPY_CONTEXT; 32],
    pub t_left: [ENTROPY_CONTEXT; 32],
    pub rd_stats: RD_STATS,
    pub current_rd: int64_t,
    pub best_rd: int64_t,
    pub exit_early: core::ffi::c_int,
    pub incomplete_exit: core::ffi::c_int,
    pub ftxs_mode: FAST_TX_SEARCH_MODE,
    pub skip_trellis: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxCandidateInfo {
    pub rd: int64_t,
    pub txb_entropy_ctx: core::ffi::c_int,
    pub tx_type: TX_TYPE,
}
static mut c2rust_aom_float_table_0: [core::ffi::c_float; 14] = [
    0.00549f32,
    0.01306f32,
    0.02039f32,
    0.02747f32,
    0.03406f32,
    0.04065f32,
    0.04724f32,
    0.05383f32,
    0.06067f32,
    0.06799f32,
    0.07605f32,
    0.08533f32,
    0.09778f32,
    0.11780f32,
];
static mut c2rust_aom_float_table_1: [core::ffi::c_float; 14] = [
    0.00037f32,
    0.00183f32,
    0.00525f32,
    0.01038f32,
    0.01697f32,
    0.02502f32,
    0.03381f32,
    0.04333f32,
    0.05286f32,
    0.06287f32,
    0.07434f32,
    0.08850f32,
    0.10803f32,
    0.14124f32,
];
static mut c2rust_aom_float_table_2: [core::ffi::c_float; 10] = [
    0.01404f32,
    0.02000f32,
    0.04211f32,
    0.05164f32,
    0.05798f32,
    0.06335f32,
    0.06897f32,
    0.07629f32,
    0.08875f32,
    0.11169f32,
];
static mut c2rust_aom_float_table_3: [core::ffi::c_float; 14] = [
    0.00183f32,
    0.00745f32,
    0.01428f32,
    0.02185f32,
    0.02966f32,
    0.03723f32,
    0.04456f32,
    0.05188f32,
    0.05920f32,
    0.06702f32,
    0.07605f32,
    0.08704f32,
    0.10168f32,
    0.12585f32,
];
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT64_MAX: core::ffi::c_ulonglong = 18446744073709551615
    as core::ffi::c_ulonglong;
static mut c2rust_aom_float_table_4: [core::ffi::c_float; 14] = [
    0.00085f32,
    0.00476f32,
    0.01135f32,
    0.01892f32,
    0.02698f32,
    0.03528f32,
    0.04358f32,
    0.05164f32,
    0.05994f32,
    0.06848f32,
    0.07849f32,
    0.09021f32,
    0.10583f32,
    0.13123f32,
];
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_QM_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_VARTX_DEPTH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_TX_DEPTH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_TX_SIZE_LOG2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_TX_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_TX_SIZE_LOG2;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
static mut mi_size_wide: [uint8_t; 22] = [
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
];
static mut mi_size_high: [uint8_t; 22] = [
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
static mut block_size_wide: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
];
static mut block_size_high: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
];
static mut num_pels_log2_lookup: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    6 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    9 as core::ffi::c_int as uint8_t,
    9 as core::ffi::c_int as uint8_t,
    10 as core::ffi::c_int as uint8_t,
    11 as core::ffi::c_int as uint8_t,
    11 as core::ffi::c_int as uint8_t,
    12 as core::ffi::c_int as uint8_t,
    13 as core::ffi::c_int as uint8_t,
    13 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
    6 as core::ffi::c_int as uint8_t,
    6 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    10 as core::ffi::c_int as uint8_t,
    10 as core::ffi::c_int as uint8_t,
];
static mut max_txsize_lookup: [TX_SIZE; 22] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
];
static mut max_txsize_rect_lookup: [TX_SIZE; 22] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X32 as core::ffi::c_int as TX_SIZE,
    TX_32X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X64 as core::ffi::c_int as TX_SIZE,
    TX_64X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_4X16 as core::ffi::c_int as TX_SIZE,
    TX_16X4 as core::ffi::c_int as TX_SIZE,
    TX_8X32 as core::ffi::c_int as TX_SIZE,
    TX_32X8 as core::ffi::c_int as TX_SIZE,
    TX_16X64 as core::ffi::c_int as TX_SIZE,
    TX_64X16 as core::ffi::c_int as TX_SIZE,
];
static mut sub_tx_size_map: [TX_SIZE; 19] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X32 as core::ffi::c_int as TX_SIZE,
    TX_32X16 as core::ffi::c_int as TX_SIZE,
];
static mut tx_size_wide: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
];
static mut tx_size_high: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    4 as core::ffi::c_int,
    32 as core::ffi::c_int,
    8 as core::ffi::c_int,
    64 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
static mut tx_size_wide_unit: [core::ffi::c_int; 19] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    1 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
static mut tx_size_high_unit: [core::ffi::c_int; 19] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    1 as core::ffi::c_int,
    8 as core::ffi::c_int,
    2 as core::ffi::c_int,
    16 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut tx_size_wide_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
];
static mut tx_size_high_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    3 as core::ffi::c_int,
    6 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut tx_size_2d: [core::ffi::c_int; 20] = [
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
    256 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    4096 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    128 as core::ffi::c_int,
    128 as core::ffi::c_int,
    512 as core::ffi::c_int,
    512 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    64 as core::ffi::c_int,
    64 as core::ffi::c_int,
    256 as core::ffi::c_int,
    256 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    0,
];
static mut txsize_to_bsize: [BLOCK_SIZE; 19] = [
    BLOCK_4X4,
    BLOCK_8X8,
    BLOCK_16X16,
    BLOCK_32X32,
    BLOCK_64X64,
    BLOCK_4X8,
    BLOCK_8X4,
    BLOCK_8X16,
    BLOCK_16X8,
    BLOCK_16X32,
    BLOCK_32X16,
    BLOCK_32X64,
    BLOCK_64X32,
    BLOCK_4X16,
    BLOCK_16X4,
    BLOCK_8X32,
    BLOCK_32X8,
    BLOCK_16X64,
    BLOCK_64X16,
];
static mut txsize_sqr_map: [TX_SIZE; 19] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
];
static mut txsize_sqr_up_map: [TX_SIZE; 19] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
];
static mut tx_mode_to_biggest_tx_size: [TX_SIZE; 3] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
];
pub const COEFF_CONTEXT_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const COEFF_CONTEXT_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << COEFF_CONTEXT_BITS) - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn combine_entropy_contexts(
    mut a: ENTROPY_CONTEXT,
    mut b: ENTROPY_CONTEXT,
) -> core::ffi::c_int {
    return (a as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        + (b as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_entropy_context(
    mut tx_size: TX_SIZE,
    mut a: *const ENTROPY_CONTEXT,
    mut l: *const ENTROPY_CONTEXT,
) -> core::ffi::c_int {
    let mut above_ec: ENTROPY_CONTEXT = 0 as ENTROPY_CONTEXT;
    let mut left_ec: ENTROPY_CONTEXT = 0 as ENTROPY_CONTEXT;
    match tx_size as core::ffi::c_int {
        0 => {
            above_ec = (*a.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*l.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        5 => {
            above_ec = (*a.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        6 => {
            above_ec = (*(a as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*l.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        7 => {
            above_ec = (*(a as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        8 => {
            above_ec = (*(a as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        9 => {
            above_ec = (*(a as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        10 => {
            above_ec = (*(a as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        1 => {
            above_ec = (*(a as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        2 => {
            above_ec = (*(a as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        3 => {
            above_ec = (*(a as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        4 => {
            above_ec = (*(a as *const uint64_t)
                | *(a.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t)
                | *(l.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        11 => {
            above_ec = (*(a as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t)
                | *(l.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        12 => {
            above_ec = (*(a as *const uint64_t)
                | *(a.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        13 => {
            above_ec = (*a.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        14 => {
            above_ec = (*(a as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*l.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int) as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        15 => {
            above_ec = (*(a as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        16 => {
            above_ec = (*(a as *const uint64_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint16_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        17 => {
            above_ec = (*(a as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint64_t)
                | *(l.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
        }
        18 => {
            above_ec = (*(a as *const uint64_t)
                | *(a.offset(8 as core::ffi::c_int as isize) as *const uint64_t) != 0)
                as core::ffi::c_int as ENTROPY_CONTEXT;
            left_ec = (*(l as *const uint32_t) != 0) as core::ffi::c_int
                as ENTROPY_CONTEXT;
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid transform size.\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"get_entropy_context\0"))
                        .as_ptr(),
                    b"entropy.h\0" as *const u8 as *const core::ffi::c_char,
                    168 as core::ffi::c_int,
                    b"0 && \"Invalid transform size.\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    return combine_entropy_contexts(above_ec, left_ec);
}
#[inline]
unsafe extern "C-unwind" fn get_txsize_entropy_ctx(mut txsize: TX_SIZE) -> TX_SIZE {
    return (txsize_sqr_map[txsize as usize] as core::ffi::c_int
        + txsize_sqr_up_map[txsize as usize] as core::ffi::c_int + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int) as TX_SIZE;
}
pub const USE_B_QUANT_NO_TRELLIS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
static mut fimode_to_intradir: [PREDICTION_MODE; 5] = [
    DC_PRED as core::ffi::c_int as PREDICTION_MODE,
    V_PRED as core::ffi::c_int as PREDICTION_MODE,
    H_PRED as core::ffi::c_int as PREDICTION_MODE,
    D157_PRED as core::ffi::c_int as PREDICTION_MODE,
    DC_PRED as core::ffi::c_int as PREDICTION_MODE,
];
#[inline]
unsafe extern "C-unwind" fn is_intrabc_block(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (*mbmi).use_intrabc() as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_uv_mode(mut mode: UV_PREDICTION_MODE) -> PREDICTION_MODE {
    if !((mode as core::ffi::c_int) < UV_INTRA_MODES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"get_uv_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            350 as core::ffi::c_int,
            b"mode < UV_INTRA_MODES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    static mut uv2y: [PREDICTION_MODE; 16] = [
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        V_PRED as core::ffi::c_int as PREDICTION_MODE,
        H_PRED as core::ffi::c_int as PREDICTION_MODE,
        D45_PRED as core::ffi::c_int as PREDICTION_MODE,
        D135_PRED as core::ffi::c_int as PREDICTION_MODE,
        D113_PRED as core::ffi::c_int as PREDICTION_MODE,
        D157_PRED as core::ffi::c_int as PREDICTION_MODE,
        D203_PRED as core::ffi::c_int as PREDICTION_MODE,
        D67_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_V_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_H_PRED as core::ffi::c_int as PREDICTION_MODE,
        PAETH_PRED as core::ffi::c_int as PREDICTION_MODE,
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        INTRA_INVALID as core::ffi::c_int as PREDICTION_MODE,
        INTRA_INVALID as core::ffi::c_int as PREDICTION_MODE,
    ];
    return uv2y[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_inter_block(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return (is_intrabc_block(mbmi) != 0
        || (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn intra_mode_to_tx_type(
    mut mbmi: *const MB_MODE_INFO,
    mut plane_type: PLANE_TYPE,
) -> TX_TYPE {
    static mut _intra_mode_to_tx_type: [TX_TYPE; 13] = [
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
    ];
    let mode: PREDICTION_MODE = (if plane_type as core::ffi::c_int
        == PLANE_TYPE_Y as core::ffi::c_int
    {
        (*mbmi).mode as core::ffi::c_int
    } else {
        get_uv_mode((*mbmi).uv_mode) as core::ffi::c_int
    }) as PREDICTION_MODE;
    if !((mode as core::ffi::c_int) < INTRA_MODES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"intra_mode_to_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1026 as core::ffi::c_int,
            b"mode < INTRA_MODES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return _intra_mode_to_tx_type[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_rect_tx(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    return (tx_size as core::ffi::c_int >= TX_SIZES as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn block_signals_txsize(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int)
        as core::ffi::c_int;
}
static mut av1_ext_tx_used: [[core::ffi::c_int; 16]; 6] = [
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
];
static mut av1_derived_intra_tx_used_flag: [uint16_t; 13] = [
    0x209 as core::ffi::c_int as uint16_t,
    0x403 as core::ffi::c_int as uint16_t,
    0x805 as core::ffi::c_int as uint16_t,
    0x20f as core::ffi::c_int as uint16_t,
    0x9 as core::ffi::c_int as uint16_t,
    0x9 as core::ffi::c_int as uint16_t,
    0x9 as core::ffi::c_int as uint16_t,
    0x805 as core::ffi::c_int as uint16_t,
    0x403 as core::ffi::c_int as uint16_t,
    0x205 as core::ffi::c_int as uint16_t,
    0x403 as core::ffi::c_int as uint16_t,
    0x805 as core::ffi::c_int as uint16_t,
    0x209 as core::ffi::c_int as uint16_t,
];
static mut av1_reduced_intra_tx_used_flag: [uint16_t; 13] = [
    0x80f as core::ffi::c_int as uint16_t,
    0x40f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0x20f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0x40f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0x40f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0x40f as core::ffi::c_int as uint16_t,
    0x80f as core::ffi::c_int as uint16_t,
    0xc0e as core::ffi::c_int as uint16_t,
];
static mut av1_ext_tx_used_flag: [uint16_t; 6] = [
    0x1 as core::ffi::c_int as uint16_t,
    0x201 as core::ffi::c_int as uint16_t,
    0x20f as core::ffi::c_int as uint16_t,
    0xe0f as core::ffi::c_int as uint16_t,
    0xfff as core::ffi::c_int as uint16_t,
    0xffff as core::ffi::c_int as uint16_t,
];
static mut av1_ext_tx_set_lookup: [[TxSetType; 2]; 2] = [
    [
        EXT_TX_SET_DTT4_IDTX_1DDCT as core::ffi::c_int as TxSetType,
        EXT_TX_SET_DTT4_IDTX as core::ffi::c_int as TxSetType,
    ],
    [
        EXT_TX_SET_ALL16 as core::ffi::c_int as TxSetType,
        EXT_TX_SET_DTT9_IDTX_1DDCT as core::ffi::c_int as TxSetType,
    ],
];
#[inline]
unsafe extern "C-unwind" fn av1_get_ext_tx_set_type(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> TxSetType {
    let tx_size_sqr_up: TX_SIZE = txsize_sqr_up_map[tx_size as usize];
    if tx_size_sqr_up as core::ffi::c_int > TX_32X32 as core::ffi::c_int {
        return EXT_TX_SET_DCTONLY as core::ffi::c_int as TxSetType;
    }
    if tx_size_sqr_up as core::ffi::c_int == TX_32X32 as core::ffi::c_int {
        return (if is_inter != 0 {
            EXT_TX_SET_DCT_IDTX as core::ffi::c_int
        } else {
            EXT_TX_SET_DCTONLY as core::ffi::c_int
        }) as TxSetType;
    }
    if use_reduced_set != 0 {
        return (if is_inter != 0 {
            EXT_TX_SET_DCT_IDTX as core::ffi::c_int
        } else {
            EXT_TX_SET_DTT4_IDTX as core::ffi::c_int
        }) as TxSetType;
    }
    let tx_size_sqr: TX_SIZE = txsize_sqr_map[tx_size as usize];
    return av1_ext_tx_set_lookup[is_inter
        as usize][(tx_size_sqr as core::ffi::c_int == TX_16X16 as core::ffi::c_int)
        as core::ffi::c_int as usize];
}
#[inline]
unsafe extern "C-unwind" fn tx_size_from_tx_mode(
    mut bsize: BLOCK_SIZE,
    mut tx_mode: TX_MODE,
) -> TX_SIZE {
    let largest_tx_size: TX_SIZE = tx_mode_to_biggest_tx_size[tx_mode as usize];
    let max_rect_tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    if bsize as core::ffi::c_int == BLOCK_4X4 as core::ffi::c_int {
        return (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
            < largest_tx_size as core::ffi::c_int
        {
            max_txsize_lookup[bsize as usize] as core::ffi::c_int
        } else {
            largest_tx_size as core::ffi::c_int
        }) as TX_SIZE;
    }
    if txsize_sqr_map[max_rect_tx_size as usize] as core::ffi::c_int
        <= largest_tx_size as core::ffi::c_int
    {
        return max_rect_tx_size
    } else {
        return largest_tx_size
    };
}
#[inline]
unsafe extern "C-unwind" fn get_default_tx_type(
    mut plane_type: PLANE_TYPE,
    mut xd: *const MACROBLOCKD,
    mut tx_size: TX_SIZE,
    mut use_screen_content_tools: core::ffi::c_int,
) -> TX_TYPE {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if is_inter_block(mbmi) != 0
        || plane_type as core::ffi::c_int != PLANE_TYPE_Y as core::ffi::c_int
        || (*xd).lossless[(*mbmi).segment_id() as usize] != 0
        || tx_size as core::ffi::c_int >= TX_32X32 as core::ffi::c_int
        || use_screen_content_tools != 0
    {
        return DCT_DCT as core::ffi::c_int as TX_TYPE;
    }
    return intra_mode_to_tx_type(mbmi, plane_type);
}
#[inline]
unsafe extern "C-unwind" fn get_plane_block_size(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1194 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_x >= 0 as core::ffi::c_int && subsampling_x < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1195 as core::ffi::c_int,
            b"subsampling_x >= 0 && subsampling_x < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_y >= 0 as core::ffi::c_int && subsampling_y < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1196 as core::ffi::c_int,
            b"subsampling_y >= 0 && subsampling_y < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return av1_ss_size_lookup[bsize
        as usize][subsampling_x as usize][subsampling_y as usize];
}
#[inline]
unsafe extern "C-unwind" fn av1_get_txb_size_index(
    mut bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
) -> core::ffi::c_int {
    static mut tw_w_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
    ];
    static mut tw_h_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
    ];
    static mut stride_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
    ];
    let index: core::ffi::c_int = ((blk_row
        >> tw_h_log2_table[bsize as usize] as core::ffi::c_int)
        << stride_log2_table[bsize as usize] as core::ffi::c_int)
        + (blk_col >> tw_w_log2_table[bsize as usize] as core::ffi::c_int);
    if !(index < 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_get_txb_size_index\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1225 as core::ffi::c_int,
            b"index < INTER_TX_SIZE_BUF_LEN\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return index;
}
#[inline]
unsafe extern "C-unwind" fn update_txk_array(
    xd: *mut MACROBLOCKD,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut tx_type: TX_TYPE,
) {
    let stride: core::ffi::c_int = (*xd).tx_type_map_stride;
    *((*xd).tx_type_map).offset((blk_row * stride + blk_col) as isize) = tx_type
        as uint8_t;
    let txw: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txh: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    if txw == tx_size_wide_unit[TX_64X64 as core::ffi::c_int as usize]
        || txh == tx_size_high_unit[TX_64X64 as core::ffi::c_int as usize]
    {
        let tx_unit: core::ffi::c_int = tx_size_wide_unit[TX_16X16 as core::ffi::c_int
            as usize];
        let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
        while idy < txh {
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < txw {
                *((*xd).tx_type_map)
                    .offset(((blk_row + idy) * stride + blk_col + idx) as isize) = tx_type
                    as uint8_t;
                idx += tx_unit;
            }
            idy += tx_unit;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn av1_get_tx_type(
    mut xd: *const MACROBLOCKD,
    mut plane_type: PLANE_TYPE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut reduced_tx_set: core::ffi::c_int,
) -> TX_TYPE {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0
        || txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            > TX_32X32 as core::ffi::c_int
    {
        return DCT_DCT as core::ffi::c_int as TX_TYPE;
    }
    let mut tx_type: TX_TYPE = 0;
    if plane_type as core::ffi::c_int == PLANE_TYPE_Y as core::ffi::c_int {
        tx_type = *((*xd).tx_type_map)
            .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize) as TX_TYPE;
    } else {
        if is_inter_block(mbmi) != 0 {
            let pd: *const macroblockd_plane = &*((*xd).plane)
                .as_ptr()
                .offset(plane_type as isize) as *const macroblockd_plane;
            blk_row <<= (*pd).subsampling_y;
            blk_col <<= (*pd).subsampling_x;
            tx_type = *((*xd).tx_type_map)
                .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize)
                as TX_TYPE;
        } else {
            tx_type = intra_mode_to_tx_type(
                mbmi,
                PLANE_TYPE_UV as core::ffi::c_int as PLANE_TYPE,
            );
        }
        let tx_set_type: TxSetType = av1_get_ext_tx_set_type(
            tx_size,
            is_inter_block(mbmi),
            reduced_tx_set,
        ) as TxSetType;
        if av1_ext_tx_used[tx_set_type as usize][tx_type as usize] == 0 {
            tx_type = DCT_DCT as core::ffi::c_int as TX_TYPE;
        }
    }
    if !((tx_type as core::ffi::c_int) < TX_TYPES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_get_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1311 as core::ffi::c_int,
            b"tx_type < TX_TYPES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (av1_ext_tx_used[av1_get_ext_tx_set_type(
        tx_size,
        is_inter_block(mbmi),
        reduced_tx_set,
    ) as usize][tx_type as usize] == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_get_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1313 as core::ffi::c_int,
            b"av1_ext_tx_used[av1_get_ext_tx_set_type(tx_size, is_inter_block(mbmi), reduced_tx_set)][tx_type]\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return tx_type;
}
#[inline]
unsafe extern "C-unwind" fn bsize_to_tx_size_cat(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"bsize_to_tx_size_cat\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1350 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    static mut bsize_to_tx_size_depth_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
    ];
    let depth: core::ffi::c_int = bsize_to_tx_size_depth_table[bsize as usize]
        as core::ffi::c_int;
    if !(depth
        <= TX_SIZES as core::ffi::c_int
            - (TX_4X4 as core::ffi::c_int + 1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"bsize_to_tx_size_cat\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1355 as core::ffi::c_int,
            b"depth <= MAX_TX_CATS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return depth - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_adjusted_tx_size(mut tx_size: TX_SIZE) -> TX_SIZE {
    match tx_size as core::ffi::c_int {
        4 | 12 | 11 => return TX_32X32 as core::ffi::c_int as TX_SIZE,
        18 => return TX_32X16 as core::ffi::c_int as TX_SIZE,
        17 => return TX_16X32 as core::ffi::c_int as TX_SIZE,
        _ => return tx_size,
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_max_uv_txsize(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> TX_SIZE {
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        subsampling_x,
        subsampling_y,
    ) as BLOCK_SIZE;
    if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_get_max_uv_txsize\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1381 as core::ffi::c_int,
            b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let uv_tx: TX_SIZE = max_txsize_rect_lookup[plane_bsize as usize];
    return av1_get_adjusted_tx_size(uv_tx);
}
#[inline]
unsafe extern "C-unwind" fn av1_get_tx_size(
    mut plane: core::ffi::c_int,
    mut xd: *const MACROBLOCKD,
) -> TX_SIZE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        return TX_4X4 as core::ffi::c_int as TX_SIZE;
    }
    if plane == 0 as core::ffi::c_int {
        return (*mbmi).tx_size;
    }
    let mut pd: *const MACROBLOCKD_PLANE = &*((*xd).plane)
        .as_ptr()
        .offset(plane as isize) as *const MACROBLOCKD_PLANE;
    return av1_get_max_uv_txsize(
        (*mbmi).bsize,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
    );
}
#[inline]
unsafe extern "C-unwind" fn get_vartx_max_txsize(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*xd)
        .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
        as usize] != 0
    {
        return TX_4X4 as core::ffi::c_int;
    }
    let max_txsize: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    if plane == 0 as core::ffi::c_int {
        return max_txsize as core::ffi::c_int;
    }
    return av1_get_adjusted_tx_size(max_txsize) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_plane_type(mut plane: core::ffi::c_int) -> PLANE_TYPE {
    return (if plane == 0 as core::ffi::c_int {
        PLANE_TYPE_Y as core::ffi::c_int
    } else {
        PLANE_TYPE_UV as core::ffi::c_int
    }) as PLANE_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_max_eob(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    if tx_size as core::ffi::c_int == TX_64X64 as core::ffi::c_int
        || tx_size as core::ffi::c_int == TX_64X32 as core::ffi::c_int
        || tx_size as core::ffi::c_int == TX_32X64 as core::ffi::c_int
    {
        return 1024 as core::ffi::c_int;
    }
    if tx_size as core::ffi::c_int == TX_16X64 as core::ffi::c_int
        || tx_size as core::ffi::c_int == TX_64X16 as core::ffi::c_int
    {
        return 512 as core::ffi::c_int;
    }
    return tx_size_2d[tx_size as usize];
}
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
#[inline]
unsafe extern "C-unwind" fn max_block_wide(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"max_block_wide\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1569 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut max_blocks_wide: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    if (*xd).mb_to_right_edge < 0 as core::ffi::c_int {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(plane as isize) as *const macroblockd_plane;
        max_blocks_wide
            += (*xd).mb_to_right_edge >> 3 as core::ffi::c_int + (*pd).subsampling_x;
    }
    return max_blocks_wide >> MI_SIZE_LOG2;
}
#[inline]
unsafe extern "C-unwind" fn max_block_high(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut max_blocks_high: core::ffi::c_int = block_size_high[bsize as usize]
        as core::ffi::c_int;
    if (*xd).mb_to_bottom_edge < 0 as core::ffi::c_int {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(plane as isize) as *const macroblockd_plane;
        max_blocks_high
            += (*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int + (*pd).subsampling_y;
    }
    return max_blocks_high >> MI_SIZE_LOG2;
}
#[inline]
unsafe extern "C-unwind" fn txfm_partition_update(
    mut above_ctx: *mut TXFM_CONTEXT,
    mut left_ctx: *mut TXFM_CONTEXT,
    mut tx_size: TX_SIZE,
    mut txb_size: TX_SIZE,
) {
    let mut bsize: BLOCK_SIZE = txsize_to_bsize[txb_size as usize];
    let mut bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mut txw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let mut txh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < bh {
        *left_ctx.offset(i as isize) = txh as TXFM_CONTEXT;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < bw {
        *above_ctx.offset(i as isize) = txw as TXFM_CONTEXT;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_sqr_tx_size(mut tx_dim: core::ffi::c_int) -> TX_SIZE {
    match tx_dim {
        128 | 64 => return TX_64X64 as core::ffi::c_int as TX_SIZE,
        32 => return TX_32X32 as core::ffi::c_int as TX_SIZE,
        16 => return TX_16X16 as core::ffi::c_int as TX_SIZE,
        8 => return TX_8X8 as core::ffi::c_int as TX_SIZE,
        _ => return TX_4X4 as core::ffi::c_int as TX_SIZE,
    };
}
#[inline]
unsafe extern "C-unwind" fn txfm_partition_context(
    above_ctx: *const TXFM_CONTEXT,
    left_ctx: *const TXFM_CONTEXT,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    let txw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let txh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    let above: core::ffi::c_int = ((*above_ctx as core::ffi::c_int)
        < txw as core::ffi::c_int) as core::ffi::c_int;
    let left: core::ffi::c_int = ((*left_ctx as core::ffi::c_int)
        < txh as core::ffi::c_int) as core::ffi::c_int;
    let mut category: core::ffi::c_int = (TX_SIZES as core::ffi::c_int
        - TX_8X8 as core::ffi::c_int) * 6 as core::ffi::c_int - 3 as core::ffi::c_int;
    if tx_size as core::ffi::c_int <= TX_4X4 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut max_tx_size: TX_SIZE = get_sqr_tx_size(
        if block_size_wide[bsize as usize] as core::ffi::c_int
            > block_size_high[bsize as usize] as core::ffi::c_int
        {
            block_size_wide[bsize as usize] as core::ffi::c_int
        } else {
            block_size_high[bsize as usize] as core::ffi::c_int
        },
    );
    if max_tx_size as core::ffi::c_int >= TX_8X8 as core::ffi::c_int {
        category = (txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            != max_tx_size as core::ffi::c_int
            && max_tx_size as core::ffi::c_int > TX_8X8 as core::ffi::c_int)
            as core::ffi::c_int
            + (TX_SIZES as core::ffi::c_int - 1 as core::ffi::c_int
                - max_tx_size as core::ffi::c_int) * 2 as core::ffi::c_int;
    }
    if !(category
        != (TX_SIZES as core::ffi::c_int - TX_8X8 as core::ffi::c_int)
            * 6 as core::ffi::c_int - 3 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"txfm_partition_context\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1769 as core::ffi::c_int,
            b"category != TXFM_PARTITION_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return category * 3 as core::ffi::c_int + above + left;
}
static mut c2rust_aom_float_table_5: [core::ffi::c_float; 14] = [
    0.00037f32,
    0.00232f32,
    0.00671f32,
    0.01257f32,
    0.01965f32,
    0.02722f32,
    0.03552f32,
    0.04382f32,
    0.05237f32,
    0.06189f32,
    0.07336f32,
    0.08728f32,
    0.10730f32,
    0.14221f32,
];
pub const RD_RECORD_BUFFER_LEN: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_rect_tx_allowed_bsize(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    static mut LUT: [core::ffi::c_char; 22] = [
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
    ];
    return LUT[bsize as usize] as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn tx_size_to_depth(
    mut tx_size: TX_SIZE,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut ctx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    let mut depth: core::ffi::c_int = 0 as core::ffi::c_int;
    while tx_size as core::ffi::c_int != ctx_size as core::ffi::c_int {
        depth += 1;
        ctx_size = sub_tx_size_map[ctx_size as usize];
        if !(depth <= 2 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"tx_size_to_depth\0"))
                    .as_ptr(),
                b"block.h\0" as *const u8 as *const core::ffi::c_char,
                1527 as core::ffi::c_int,
                b"depth <= MAX_TX_DEPTH\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    return depth;
}
pub const MAX_TX_SCALE: core::ffi::c_int = 1 as core::ffi::c_int;
static mut c2rust_aom_float_table_6: [core::ffi::c_float; 14] = [
    0.00061f32,
    0.00330f32,
    0.00818f32,
    0.01453f32,
    0.02185f32,
    0.02966f32,
    0.03772f32,
    0.04578f32,
    0.05383f32,
    0.06262f32,
    0.07288f32,
    0.08582f32,
    0.10339f32,
    0.13464f32,
];
#[inline]
unsafe extern "C-unwind" fn get_default_scan(
    mut tx_size: TX_SIZE,
    mut tx_type: TX_TYPE,
) -> *const SCAN_ORDER {
    return &*(*av1_scan_orders.as_ptr().offset(tx_size as isize))
        .as_ptr()
        .offset(tx_type as isize) as *const SCAN_ORDER;
}
#[inline]
unsafe extern "C-unwind" fn get_scan(
    mut tx_size: TX_SIZE,
    mut tx_type: TX_TYPE,
) -> *const SCAN_ORDER {
    return get_default_scan(tx_size, tx_type);
}
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_init_rd_stats(mut rd_stats: *mut RD_STATS) {
    (*rd_stats).rate = 0 as core::ffi::c_int;
    (*rd_stats).dist = 0 as int64_t;
    (*rd_stats).rdcost = 0 as int64_t;
    (*rd_stats).sse = 0 as int64_t;
    (*rd_stats).skip_txfm = 1 as uint8_t;
    (*rd_stats).zero_rate = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_invalid_rd_stats(mut rd_stats: *mut RD_STATS) {
    (*rd_stats).rate = INT_MAX;
    (*rd_stats).dist = INT64_MAX as int64_t;
    (*rd_stats).rdcost = INT64_MAX as int64_t;
    (*rd_stats).sse = INT64_MAX as int64_t;
    (*rd_stats).skip_txfm = 0 as uint8_t;
    (*rd_stats).zero_rate = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_merge_rd_stats(
    mut rd_stats_dst: *mut RD_STATS,
    mut rd_stats_src: *const RD_STATS,
) {
    if (*rd_stats_dst).rate == INT_MAX || (*rd_stats_src).rate == INT_MAX {
        av1_invalid_rd_stats(rd_stats_dst);
        return;
    }
    (*rd_stats_dst).rate = (if ((*rd_stats_dst).rate as int64_t
        + (*rd_stats_src).rate as int64_t) < 2147483647 as int64_t
    {
        (*rd_stats_dst).rate as int64_t + (*rd_stats_src).rate as int64_t
    } else {
        2147483647 as int64_t
    }) as core::ffi::c_int;
    if (*rd_stats_dst).zero_rate == 0 {
        (*rd_stats_dst).zero_rate = (*rd_stats_src).zero_rate;
    }
    (*rd_stats_dst).dist += (*rd_stats_src).dist;
    if (*rd_stats_dst).sse < INT64_MAX as int64_t
        && (*rd_stats_src).sse < INT64_MAX as int64_t
    {
        (*rd_stats_dst).sse += (*rd_stats_src).sse;
    }
    (*rd_stats_dst).skip_txfm = ((*rd_stats_dst).skip_txfm as core::ffi::c_int
        & (*rd_stats_src).skip_txfm as core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_merge_rd_stats_weighted(
    mut rd_stats_dst: *mut RD_STATS,
    mut rd_stats_src: *const RD_STATS,
) {
    if (*rd_stats_dst).rate == INT_MAX || (*rd_stats_src).rate == INT_MAX {
        av1_invalid_rd_stats(rd_stats_dst);
        return;
    }
    (*rd_stats_dst).rate = (if ((*rd_stats_dst).rate as int64_t
        + (*rd_stats_src).rate as int64_t) < 2147483647 as int64_t
    {
        (*rd_stats_dst).rate as int64_t + (*rd_stats_src).rate as int64_t
    } else {
        2147483647 as int64_t
    }) as core::ffi::c_int;
    if (*rd_stats_dst).zero_rate == 0 {
        (*rd_stats_dst).zero_rate = (*rd_stats_src).zero_rate;
    }
    (*rd_stats_dst).dist += (*rd_stats_src).dist * 15 as int64_t / 16 as int64_t;
    if (*rd_stats_dst).sse < INT64_MAX as int64_t
        && (*rd_stats_src).sse < INT64_MAX as int64_t
    {
        (*rd_stats_dst).sse += (*rd_stats_src).sse * 15 as int64_t / 16 as int64_t;
    }
    (*rd_stats_dst).skip_txfm = ((*rd_stats_dst).skip_txfm as core::ffi::c_int
        & (*rd_stats_src).skip_txfm as core::ffi::c_int) as uint8_t;
}
pub const MAX_TX_SIZE_UNIT: core::ffi::c_int = 16 as core::ffi::c_int;
static mut kTxbCtxSigns: [int8_t; 3] = [
    0 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    1 as core::ffi::c_int as int8_t,
];
static mut kTxbCtxDcSignContexts: [int8_t; 65] = [
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
];
static mut kTxbCtxSkipContexts: [[uint8_t; 5]; 5] = [
    [
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
    ],
    [
        2 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
    ],
    [
        2 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
    ],
    [
        2 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
    ],
    [
        3 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
    ],
];
unsafe extern "C-unwind" fn get_txb_ctx_general(
    plane_bsize: BLOCK_SIZE,
    tx_size: TX_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    let txb_w_unit: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_h_unit: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mut dc_sign: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let sign: core::ffi::c_uint = (*a.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"get_txb_ctx_general\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                320 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_w_unit) {
            break;
        }
    }
    k = 0 as core::ffi::c_int;
    loop {
        let sign_0: core::ffi::c_uint = (*l.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign_0 <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"get_txb_ctx_general\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                327 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign_0 as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_h_unit) {
            break;
        }
    }
    (*txb_ctx).dc_sign_ctx = kTxbCtxDcSignContexts[(dc_sign
        + 2 as core::ffi::c_int * MAX_TX_SIZE_UNIT) as usize] as core::ffi::c_int;
    if plane == 0 as core::ffi::c_int {
        if plane_bsize as core::ffi::c_int
            == txsize_to_bsize[tx_size as usize] as core::ffi::c_int
        {
            (*txb_ctx).txb_skip_ctx = 0 as core::ffi::c_int;
        } else {
            let mut top: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut left: core::ffi::c_int = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            loop {
                top |= *a.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_w_unit) {
                    break;
                }
            }
            top &= COEFF_CONTEXT_MASK;
            top = if top < 4 as core::ffi::c_int { top } else { 4 as core::ffi::c_int };
            k = 0 as core::ffi::c_int;
            loop {
                left |= *l.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_h_unit) {
                    break;
                }
            }
            left &= COEFF_CONTEXT_MASK;
            left = if left < 4 as core::ffi::c_int {
                left
            } else {
                4 as core::ffi::c_int
            };
            (*txb_ctx).txb_skip_ctx = kTxbCtxSkipContexts[top as usize][left as usize]
                as core::ffi::c_int;
        }
    } else {
        let ctx_base: core::ffi::c_int = get_entropy_context(tx_size, a, l)
            as core::ffi::c_int;
        let ctx_offset: core::ffi::c_int = if num_pels_log2_lookup[plane_bsize as usize]
            as core::ffi::c_int
            > num_pels_log2_lookup[txsize_to_bsize[tx_size as usize] as usize]
                as core::ffi::c_int
        {
            10 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        };
        (*txb_ctx).txb_skip_ctx = ctx_base + ctx_offset;
    };
}
unsafe extern "C-unwind" fn get_txb_ctx_4x4(
    plane_bsize: BLOCK_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    let tx_size: TX_SIZE = TX_4X4 as core::ffi::c_int as TX_SIZE;
    let txb_w_unit: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_h_unit: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mut dc_sign: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let sign: core::ffi::c_uint = (*a.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"get_txb_ctx_4x4\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                431 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_w_unit) {
            break;
        }
    }
    k = 0 as core::ffi::c_int;
    loop {
        let sign_0: core::ffi::c_uint = (*l.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign_0 <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"get_txb_ctx_4x4\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                431 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign_0 as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_h_unit) {
            break;
        }
    }
    (*txb_ctx).dc_sign_ctx = kTxbCtxDcSignContexts[(dc_sign
        + 2 as core::ffi::c_int * MAX_TX_SIZE_UNIT) as usize] as core::ffi::c_int;
    if plane == 0 as core::ffi::c_int {
        if plane_bsize as core::ffi::c_int
            == txsize_to_bsize[tx_size as usize] as core::ffi::c_int
        {
            (*txb_ctx).txb_skip_ctx = 0 as core::ffi::c_int;
        } else {
            let mut top: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut left: core::ffi::c_int = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            loop {
                top |= *a.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_w_unit) {
                    break;
                }
            }
            top &= COEFF_CONTEXT_MASK;
            top = if top < 4 as core::ffi::c_int { top } else { 4 as core::ffi::c_int };
            k = 0 as core::ffi::c_int;
            loop {
                left |= *l.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_h_unit) {
                    break;
                }
            }
            left &= COEFF_CONTEXT_MASK;
            left = if left < 4 as core::ffi::c_int {
                left
            } else {
                4 as core::ffi::c_int
            };
            (*txb_ctx).txb_skip_ctx = kTxbCtxSkipContexts[top as usize][left as usize]
                as core::ffi::c_int;
        }
    } else {
        let ctx_base: core::ffi::c_int = get_entropy_context(tx_size, a, l)
            as core::ffi::c_int;
        let ctx_offset: core::ffi::c_int = if num_pels_log2_lookup[plane_bsize as usize]
            as core::ffi::c_int
            > num_pels_log2_lookup[txsize_to_bsize[tx_size as usize] as usize]
                as core::ffi::c_int
        {
            10 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        };
        (*txb_ctx).txb_skip_ctx = ctx_base + ctx_offset;
    };
}
unsafe extern "C-unwind" fn get_txb_ctx_8x8(
    plane_bsize: BLOCK_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    let tx_size: TX_SIZE = TX_8X8 as core::ffi::c_int as TX_SIZE;
    let txb_w_unit: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_h_unit: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mut dc_sign: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let sign: core::ffi::c_uint = (*a.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"get_txb_ctx_8x8\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                432 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_w_unit) {
            break;
        }
    }
    k = 0 as core::ffi::c_int;
    loop {
        let sign_0: core::ffi::c_uint = (*l.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign_0 <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"get_txb_ctx_8x8\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                432 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign_0 as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_h_unit) {
            break;
        }
    }
    (*txb_ctx).dc_sign_ctx = kTxbCtxDcSignContexts[(dc_sign
        + 2 as core::ffi::c_int * MAX_TX_SIZE_UNIT) as usize] as core::ffi::c_int;
    if plane == 0 as core::ffi::c_int {
        if plane_bsize as core::ffi::c_int
            == txsize_to_bsize[tx_size as usize] as core::ffi::c_int
        {
            (*txb_ctx).txb_skip_ctx = 0 as core::ffi::c_int;
        } else {
            let mut top: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut left: core::ffi::c_int = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            loop {
                top |= *a.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_w_unit) {
                    break;
                }
            }
            top &= COEFF_CONTEXT_MASK;
            top = if top < 4 as core::ffi::c_int { top } else { 4 as core::ffi::c_int };
            k = 0 as core::ffi::c_int;
            loop {
                left |= *l.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_h_unit) {
                    break;
                }
            }
            left &= COEFF_CONTEXT_MASK;
            left = if left < 4 as core::ffi::c_int {
                left
            } else {
                4 as core::ffi::c_int
            };
            (*txb_ctx).txb_skip_ctx = kTxbCtxSkipContexts[top as usize][left as usize]
                as core::ffi::c_int;
        }
    } else {
        let ctx_base: core::ffi::c_int = get_entropy_context(tx_size, a, l)
            as core::ffi::c_int;
        let ctx_offset: core::ffi::c_int = if num_pels_log2_lookup[plane_bsize as usize]
            as core::ffi::c_int
            > num_pels_log2_lookup[txsize_to_bsize[tx_size as usize] as usize]
                as core::ffi::c_int
        {
            10 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        };
        (*txb_ctx).txb_skip_ctx = ctx_base + ctx_offset;
    };
}
unsafe extern "C-unwind" fn get_txb_ctx_16x16(
    plane_bsize: BLOCK_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    let tx_size: TX_SIZE = TX_16X16 as core::ffi::c_int as TX_SIZE;
    let txb_w_unit: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_h_unit: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mut dc_sign: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let sign: core::ffi::c_uint = (*a.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"get_txb_ctx_16x16\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                433 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_w_unit) {
            break;
        }
    }
    k = 0 as core::ffi::c_int;
    loop {
        let sign_0: core::ffi::c_uint = (*l.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign_0 <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"get_txb_ctx_16x16\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                433 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign_0 as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_h_unit) {
            break;
        }
    }
    (*txb_ctx).dc_sign_ctx = kTxbCtxDcSignContexts[(dc_sign
        + 2 as core::ffi::c_int * MAX_TX_SIZE_UNIT) as usize] as core::ffi::c_int;
    if plane == 0 as core::ffi::c_int {
        if plane_bsize as core::ffi::c_int
            == txsize_to_bsize[tx_size as usize] as core::ffi::c_int
        {
            (*txb_ctx).txb_skip_ctx = 0 as core::ffi::c_int;
        } else {
            let mut top: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut left: core::ffi::c_int = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            loop {
                top |= *a.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_w_unit) {
                    break;
                }
            }
            top &= COEFF_CONTEXT_MASK;
            top = if top < 4 as core::ffi::c_int { top } else { 4 as core::ffi::c_int };
            k = 0 as core::ffi::c_int;
            loop {
                left |= *l.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_h_unit) {
                    break;
                }
            }
            left &= COEFF_CONTEXT_MASK;
            left = if left < 4 as core::ffi::c_int {
                left
            } else {
                4 as core::ffi::c_int
            };
            (*txb_ctx).txb_skip_ctx = kTxbCtxSkipContexts[top as usize][left as usize]
                as core::ffi::c_int;
        }
    } else {
        let ctx_base: core::ffi::c_int = get_entropy_context(tx_size, a, l)
            as core::ffi::c_int;
        let ctx_offset: core::ffi::c_int = if num_pels_log2_lookup[plane_bsize as usize]
            as core::ffi::c_int
            > num_pels_log2_lookup[txsize_to_bsize[tx_size as usize] as usize]
                as core::ffi::c_int
        {
            10 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        };
        (*txb_ctx).txb_skip_ctx = ctx_base + ctx_offset;
    };
}
unsafe extern "C-unwind" fn get_txb_ctx_32x32(
    plane_bsize: BLOCK_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    let tx_size: TX_SIZE = TX_32X32 as core::ffi::c_int as TX_SIZE;
    let txb_w_unit: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_h_unit: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mut dc_sign: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let sign: core::ffi::c_uint = (*a.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"get_txb_ctx_32x32\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                434 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_w_unit) {
            break;
        }
    }
    k = 0 as core::ffi::c_int;
    loop {
        let sign_0: core::ffi::c_uint = (*l.offset(k as isize) as uint8_t
            as core::ffi::c_int >> COEFF_CONTEXT_BITS) as core::ffi::c_uint;
        if !(sign_0 <= 2 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"get_txb_ctx_32x32\0"))
                    .as_ptr(),
                b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
                434 as core::ffi::c_int,
                b"sign <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        dc_sign += kTxbCtxSigns[sign_0 as usize] as core::ffi::c_int;
        k += 1;
        if !(k < txb_h_unit) {
            break;
        }
    }
    (*txb_ctx).dc_sign_ctx = kTxbCtxDcSignContexts[(dc_sign
        + 2 as core::ffi::c_int * MAX_TX_SIZE_UNIT) as usize] as core::ffi::c_int;
    if plane == 0 as core::ffi::c_int {
        if plane_bsize as core::ffi::c_int
            == txsize_to_bsize[tx_size as usize] as core::ffi::c_int
        {
            (*txb_ctx).txb_skip_ctx = 0 as core::ffi::c_int;
        } else {
            let mut top: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut left: core::ffi::c_int = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            loop {
                top |= *a.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_w_unit) {
                    break;
                }
            }
            top &= COEFF_CONTEXT_MASK;
            top = if top < 4 as core::ffi::c_int { top } else { 4 as core::ffi::c_int };
            k = 0 as core::ffi::c_int;
            loop {
                left |= *l.offset(k as isize) as core::ffi::c_int;
                k += 1;
                if !(k < txb_h_unit) {
                    break;
                }
            }
            left &= COEFF_CONTEXT_MASK;
            left = if left < 4 as core::ffi::c_int {
                left
            } else {
                4 as core::ffi::c_int
            };
            (*txb_ctx).txb_skip_ctx = kTxbCtxSkipContexts[top as usize][left as usize]
                as core::ffi::c_int;
        }
    } else {
        let ctx_base: core::ffi::c_int = get_entropy_context(tx_size, a, l)
            as core::ffi::c_int;
        let ctx_offset: core::ffi::c_int = if num_pels_log2_lookup[plane_bsize as usize]
            as core::ffi::c_int
            > num_pels_log2_lookup[txsize_to_bsize[tx_size as usize] as usize]
                as core::ffi::c_int
        {
            10 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        };
        (*txb_ctx).txb_skip_ctx = ctx_base + ctx_offset;
    };
}
#[inline]
unsafe extern "C-unwind" fn get_txb_ctx(
    plane_bsize: BLOCK_SIZE,
    tx_size: TX_SIZE,
    plane: core::ffi::c_int,
    a: *const ENTROPY_CONTEXT,
    l: *const ENTROPY_CONTEXT,
    txb_ctx: *mut TXB_CTX,
) {
    match tx_size as core::ffi::c_int {
        0 => {
            get_txb_ctx_4x4(plane_bsize, plane, a, l, txb_ctx);
        }
        1 => {
            get_txb_ctx_8x8(plane_bsize, plane, a, l, txb_ctx);
        }
        2 => {
            get_txb_ctx_16x16(plane_bsize, plane, a, l, txb_ctx);
        }
        3 => {
            get_txb_ctx_32x32(plane_bsize, plane, a, l, txb_ctx);
        }
        _ => {
            get_txb_ctx_general(plane_bsize, tx_size, plane, a, l, txb_ctx);
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_set_txb_context(
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut a: *mut ENTROPY_CONTEXT,
    mut l: *mut ENTROPY_CONTEXT,
) {
    let ctx: uint8_t = *((*x).plane[plane as usize].txb_entropy_ctx)
        .offset(block as isize);
    memset(
        a as *mut core::ffi::c_void,
        ctx as core::ffi::c_int,
        (tx_size_wide_unit[tx_size as usize] as size_t)
            .wrapping_mul(::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t),
    );
    memset(
        l as *mut core::ffi::c_void,
        ctx as core::ffi::c_int,
        (tx_size_high_unit[tx_size as usize] as size_t)
            .wrapping_mul(::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t),
    );
}
#[inline]
unsafe extern "C-unwind" fn is_trellis_used(
    mut optimize_b: TRELLIS_OPT_TYPE,
    mut dry_run: RUN_TYPE,
) -> core::ffi::c_int {
    if optimize_b as core::ffi::c_int == NO_TRELLIS_OPT as core::ffi::c_int {
        return false_0;
    }
    if optimize_b as core::ffi::c_int == FINAL_PASS_TRELLIS_OPT as core::ffi::c_int
        && dry_run as core::ffi::c_int != OUTPUT_ENABLED as core::ffi::c_int
    {
        return false_0;
    }
    return true_0;
}
static mut dc_coeff_scale: [uint16_t; 19] = [
    1024 as core::ffi::c_int as uint16_t,
    2048 as core::ffi::c_int as uint16_t,
    4096 as core::ffi::c_int as uint16_t,
    4096 as core::ffi::c_int as uint16_t,
    0 as core::ffi::c_int as uint16_t,
    1448 as core::ffi::c_int as uint16_t,
    1448 as core::ffi::c_int as uint16_t,
    2896 as core::ffi::c_int as uint16_t,
    2896 as core::ffi::c_int as uint16_t,
    2896 as core::ffi::c_int as uint16_t,
    2896 as core::ffi::c_int as uint16_t,
    0 as core::ffi::c_int as uint16_t,
    0 as core::ffi::c_int as uint16_t,
    2048 as core::ffi::c_int as uint16_t,
    2048 as core::ffi::c_int as uint16_t,
    4096 as core::ffi::c_int as uint16_t,
    4096 as core::ffi::c_int as uint16_t,
    0 as core::ffi::c_int as uint16_t,
    0 as core::ffi::c_int as uint16_t,
];
#[inline]
unsafe extern "C-unwind" fn get_frame_update_type(
    mut gf_group: *const GF_GROUP,
    mut gf_frame_index: core::ffi::c_int,
) -> FRAME_UPDATE_TYPE {
    return (*gf_group).update_type[gf_frame_index as usize];
}
#[inline]
unsafe extern "C-unwind" fn av1_get_skip_txfm_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_skip_txfm: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).skip_txfm as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_skip_txfm: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).skip_txfm as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return above_skip_txfm + left_skip_txfm;
}
#[inline]
unsafe extern "C-unwind" fn get_tx_size_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let max_tx_size: TX_SIZE = max_txsize_rect_lookup[(*mbmi).bsize as usize];
    let max_tx_wide: core::ffi::c_int = tx_size_wide[max_tx_size as usize];
    let max_tx_high: core::ffi::c_int = tx_size_high[max_tx_size as usize];
    let has_above: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let has_left: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    let mut above: core::ffi::c_int = (*((*xd).above_txfm_context)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int >= max_tx_wide)
        as core::ffi::c_int;
    let mut left: core::ffi::c_int = (*((*xd).left_txfm_context)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int >= max_tx_high)
        as core::ffi::c_int;
    if has_above != 0 {
        if is_inter_block(above_mbmi) != 0 {
            above = (block_size_wide[(*above_mbmi).bsize as usize] as core::ffi::c_int
                >= max_tx_wide) as core::ffi::c_int;
        }
    }
    if has_left != 0 {
        if is_inter_block(left_mbmi) != 0 {
            left = (block_size_high[(*left_mbmi).bsize as usize] as core::ffi::c_int
                >= max_tx_high) as core::ffi::c_int;
        }
    }
    if has_above != 0 && has_left != 0 {
        return above + left
    } else if has_above != 0 {
        return above
    } else if has_left != 0 {
        return left
    } else {
        return 0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn get_visible_dimensions(
    mut x: *const MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut blk_col: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut cols: core::ffi::c_int,
    mut rows: core::ffi::c_int,
    mut clip_dims: bool,
    mut visible_cols: *mut core::ffi::c_int,
    mut visible_rows: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    if (*x).pix_to_bottom_edge >= 0 as core::ffi::c_int
        && (*x).pix_to_right_edge >= 0 as core::ffi::c_int || !clip_dims
    {
        if !visible_cols.is_null() && !visible_rows.is_null() {
            *visible_rows = rows;
            *visible_cols = cols;
        }
        return 0 as core::ffi::c_int;
    }
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let pd: *const macroblockd_plane = &*((*xd).plane).as_ptr().offset(plane as isize)
        as *const macroblockd_plane;
    let mut valid_cols: core::ffi::c_int = 0;
    let mut valid_rows: core::ffi::c_int = 0;
    if (*x).pix_to_bottom_edge >= 0 as core::ffi::c_int {
        valid_rows = rows;
    } else {
        let block_height: core::ffi::c_int = block_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        let block_rows: core::ffi::c_int = -(-(*x).pix_to_bottom_edge
            + ((1 as core::ffi::c_int) << (*pd).subsampling_y >> 1 as core::ffi::c_int)
            >> (*pd).subsampling_y) + block_height;
        valid_rows = clamp(
            block_rows - (blk_row << MI_SIZE_LOG2),
            0 as core::ffi::c_int,
            rows,
        );
    }
    if (*x).pix_to_right_edge >= 0 as core::ffi::c_int {
        valid_cols = cols;
    } else {
        let block_width: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let block_cols: core::ffi::c_int = -(-(*x).pix_to_right_edge
            + ((1 as core::ffi::c_int) << (*pd).subsampling_x >> 1 as core::ffi::c_int)
            >> (*pd).subsampling_x) + block_width;
        valid_cols = clamp(
            block_cols - (blk_col << MI_SIZE_LOG2),
            0 as core::ffi::c_int,
            cols,
        );
    }
    if !visible_cols.is_null() && !visible_rows.is_null() {
        *visible_cols = valid_cols;
        *visible_rows = valid_rows;
    }
    return (valid_cols < cols || valid_rows < rows) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn bsize_to_num_blk(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    let mut num_blk: core::ffi::c_int = (1 as core::ffi::c_int)
        << num_pels_log2_lookup[bsize as usize] as core::ffi::c_int
            - 2 as core::ffi::c_int * MI_SIZE_LOG2;
    return num_blk;
}
pub const MODELRD_TYPE_TX_SEARCH_PRUNE: core::ffi::c_int = 1 as core::ffi::c_int;
unsafe extern "C-unwind" fn calculate_sse(
    xd: *mut MACROBLOCKD,
    mut p: *const macroblock_plane,
    mut pd: *mut macroblockd_plane,
    bw: core::ffi::c_int,
    bh: core::ffi::c_int,
) -> int64_t {
    let mut sse: int64_t = 0 as int64_t;
    let shift: core::ffi::c_int = (*xd).bd - 8 as core::ffi::c_int;
    if is_cur_buf_hbd(xd) != 0 {
        sse = aom_highbd_sse_c(
            (*p).src.buf,
            (*p).src.stride,
            (*pd).dst.buf,
            (*pd).dst.stride,
            bw,
            bh,
        );
    } else {
        sse = aom_sse_c(
            (*p).src.buf,
            (*p).src.stride,
            (*pd).dst.buf,
            (*pd).dst.stride,
            bw,
            bh,
        );
    }
    sse = sse
        + ((1 as core::ffi::c_int) << shift * 2 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as int64_t >> shift * 2 as core::ffi::c_int;
    return sse;
}
unsafe extern "C-unwind" fn pixel_dist_visible_only(
    cpi: *const AV1_COMP,
    mut x: *const MACROBLOCK,
    mut src: *const uint8_t,
    src_stride: core::ffi::c_int,
    mut dst: *const uint8_t,
    dst_stride: core::ffi::c_int,
    tx_bsize: BLOCK_SIZE,
    mut txb_rows: core::ffi::c_int,
    mut txb_cols: core::ffi::c_int,
    mut visible_rows: core::ffi::c_int,
    mut visible_cols: core::ffi::c_int,
) -> core::ffi::c_uint {
    if visible_rows == 0 as core::ffi::c_int || visible_cols == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_uint;
    }
    let mut sse: core::ffi::c_uint = 0;
    if txb_rows == visible_rows && txb_cols == visible_cols {
        ((*(*cpi).ppi).fn_ptr[tx_bsize as usize].vf)
            .expect(
                "non-null function pointer",
            )(src, src_stride, dst, dst_stride, &raw mut sse);
        return sse;
    }
    let mut xd: *const MACROBLOCKD = &(*x).e_mbd;
    if is_cur_buf_hbd(xd) != 0 {
        let mut sse64: uint64_t = 0;
        if visible_rows % 4 as core::ffi::c_int == 0
            && visible_cols % 4 as core::ffi::c_int == 0
        {
            sse64 = aom_highbd_sse_c(
                src,
                src_stride,
                dst,
                dst_stride,
                visible_cols,
                visible_rows,
            ) as uint64_t;
        } else {
            sse64 = aom_highbd_sse_odd_size(
                src,
                src_stride,
                dst,
                dst_stride,
                visible_cols,
                visible_rows,
            );
        }
        return (sse64
            .wrapping_add(
                ((1 as core::ffi::c_int)
                    << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as uint64_t,
            ) >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int)
            as core::ffi::c_uint;
    }
    if visible_rows % 4 as core::ffi::c_int == 0
        && visible_cols % 4 as core::ffi::c_int == 0
    {
        sse = aom_sse_c(src, src_stride, dst, dst_stride, visible_cols, visible_rows)
            as core::ffi::c_uint;
    } else {
        sse = aom_sse_odd_size(
            src,
            src_stride,
            dst,
            dst_stride,
            visible_cols,
            visible_rows,
        ) as core::ffi::c_uint;
    }
    return sse;
}
#[inline]
unsafe extern "C-unwind" fn model_rd_from_sse(
    cpi: *const AV1_COMP,
    x: *const MACROBLOCK,
    mut plane_bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut sse: int64_t,
    mut num_samples: core::ffi::c_int,
    mut rate: *mut core::ffi::c_int,
    mut dist: *mut int64_t,
) {
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let p: *const macroblock_plane = &*((*x).plane).as_ptr().offset(plane as isize)
        as *const macroblock_plane;
    let dequant_shift: core::ffi::c_int = if is_cur_buf_hbd(xd) != 0 {
        (*xd).bd - 5 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    if (*cpi).sf.rd_sf.simple_model_rd_from_var != 0 {
        let square_error: int64_t = sse;
        let mut quantizer: core::ffi::c_int = *((*p).dequant_QTX)
            .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int >> dequant_shift;
        if quantizer < 120 as core::ffi::c_int {
            *rate = (if (square_error * (280 as core::ffi::c_int - quantizer) as int64_t
                >> 16 as core::ffi::c_int - 9 as core::ffi::c_int)
                < 2147483647 as int64_t
            {
                square_error * (280 as core::ffi::c_int - quantizer) as int64_t
                    >> 16 as core::ffi::c_int - 9 as core::ffi::c_int
            } else {
                2147483647 as int64_t
            }) as core::ffi::c_int;
        } else {
            *rate = 0 as core::ffi::c_int;
        }
        if !(*rate >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"model_rd_from_sse\0"))
                    .as_ptr(),
                b"model_rd.h\0" as *const u8 as *const core::ffi::c_char,
                150 as core::ffi::c_int,
                b"*rate >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        *dist = square_error * quantizer as int64_t >> 8 as core::ffi::c_int;
    } else {
        av1_model_rd_from_var_lapndz(
            sse,
            num_pels_log2_lookup[plane_bsize as usize] as core::ffi::c_uint,
            (*((*p).dequant_QTX).offset(1 as core::ffi::c_int as isize)
                as core::ffi::c_int >> dequant_shift) as core::ffi::c_uint,
            rate,
            dist,
        );
    }
    *dist <<= 4 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn model_rd_with_curvfit(
    cpi: *const AV1_COMP,
    x: *const MACROBLOCK,
    mut plane_bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut sse: int64_t,
    mut num_samples: core::ffi::c_int,
    mut rate: *mut core::ffi::c_int,
    mut dist: *mut int64_t,
) {
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let p: *const macroblock_plane = &*((*x).plane).as_ptr().offset(plane as isize)
        as *const macroblock_plane;
    let dequant_shift: core::ffi::c_int = if is_cur_buf_hbd(xd) != 0 {
        (*xd).bd - 5 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    let qstep: core::ffi::c_int = if *((*p).dequant_QTX)
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int >> dequant_shift
        > 1 as core::ffi::c_int
    {
        *((*p).dequant_QTX).offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            >> dequant_shift
    } else {
        1 as core::ffi::c_int
    };
    if sse == 0 as int64_t {
        if !rate.is_null() {
            *rate = 0 as core::ffi::c_int;
        }
        if !dist.is_null() {
            *dist = 0 as int64_t;
        }
        return;
    }
    let sse_norm: core::ffi::c_double = sse as core::ffi::c_double
        / num_samples as core::ffi::c_double;
    let qstepsqr: core::ffi::c_double = qstep as core::ffi::c_double
        * qstep as core::ffi::c_double;
    let xqr: core::ffi::c_double = log2(sse_norm / qstepsqr) as core::ffi::c_double;
    let mut rate_dist_f: [core::ffi::c_double; 2] = [0.; 2];
    av1_model_rd_curvfit(plane_bsize, sse_norm, xqr, rate_dist_f.as_mut_ptr());
    let dist_f: core::ffi::c_double = rate_dist_f[1 as core::ffi::c_int as usize]
        * sse_norm;
    let mut rate_i: core::ffi::c_int = ((if 0.0f64
        > rate_dist_f[0 as core::ffi::c_int as usize]
            * num_samples as core::ffi::c_double
    {
        0.0f64
    } else {
        rate_dist_f[0 as core::ffi::c_int as usize] * num_samples as core::ffi::c_double
    }) + 0.5f64) as core::ffi::c_int;
    let mut dist_i: int64_t = ((if 0.0f64 > dist_f * num_samples as core::ffi::c_double {
        0.0f64
    } else {
        dist_f * num_samples as core::ffi::c_double
    }) + 0.5f64) as int64_t;
    if rate_i == 0 as core::ffi::c_int {
        dist_i = sse << 4 as core::ffi::c_int;
    } else if (rate_i as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + dist_i * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
        >= (0 as core::ffi::c_int as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (sse << 4 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
    {
        rate_i = 0 as core::ffi::c_int;
        dist_i = sse << 4 as core::ffi::c_int;
    }
    if !rate.is_null() {
        *rate = rate_i;
    }
    if !dist.is_null() {
        *dist = dist_i;
    }
}
#[inline]
unsafe extern "C-unwind" fn model_rd_for_sb(
    cpi: *const AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
    mut out_rate_sum: *mut core::ffi::c_int,
    mut out_dist_sum: *mut int64_t,
    mut skip_txfm_sb: *mut uint8_t,
    mut skip_sse_sb: *mut int64_t,
    mut plane_rate: *mut core::ffi::c_int,
    mut plane_sse: *mut int64_t,
    mut plane_dist: *mut int64_t,
) {
    let mut plane: core::ffi::c_int = 0;
    let ref_0: core::ffi::c_int = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int;
    let mut rate_sum: int64_t = 0 as int64_t;
    let mut dist_sum: int64_t = 0 as int64_t;
    let mut total_sse: int64_t = 0 as int64_t;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"model_rd_for_sb\0"))
                .as_ptr(),
            b"model_rd.h\0" as *const u8 as *const core::ffi::c_char,
            219 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    plane = plane_from;
    while plane <= plane_to {
        if plane != 0 && !(*xd).is_chroma_ref {
            break;
        }
        let p: *mut macroblock_plane = &raw mut *((*x).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblock_plane;
        let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblockd_plane;
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"model_rd_for_sb\0"))
                    .as_ptr(),
                b"model_rd.h\0" as *const u8 as *const core::ffi::c_char,
                227 as core::ffi::c_int,
                b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let bw: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let bh: core::ffi::c_int = block_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        let mut sse: int64_t = 0;
        let mut rate: core::ffi::c_int = 0;
        let mut dist: int64_t = 0;
        sse = calculate_sse(xd, p, pd, bw, bh);
        model_rd_from_sse(
            cpi,
            x,
            plane_bsize,
            plane,
            sse,
            bw * bh,
            &raw mut rate,
            &raw mut dist,
        );
        if plane == 0 as core::ffi::c_int {
            (*x).pred_sse[ref_0 as usize] = (if sse
                < (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as int64_t
            {
                sse
            } else {
                (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as int64_t
            }) as core::ffi::c_uint;
        }
        total_sse += sse;
        rate_sum += rate as int64_t;
        dist_sum += dist;
        if !plane_rate.is_null() {
            *plane_rate.offset(plane as isize) = rate;
        }
        if !plane_sse.is_null() {
            *plane_sse.offset(plane as isize) = sse;
        }
        if !plane_dist.is_null() {
            *plane_dist.offset(plane as isize) = dist;
        }
        if !(rate_sum >= 0 as int64_t) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"model_rd_for_sb\0"))
                    .as_ptr(),
                b"model_rd.h\0" as *const u8 as *const core::ffi::c_char,
                246 as core::ffi::c_int,
                b"rate_sum >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        plane += 1;
    }
    if !skip_txfm_sb.is_null() {
        *skip_txfm_sb = (total_sse == 0 as int64_t) as core::ffi::c_int as uint8_t;
    }
    if !skip_sse_sb.is_null() {
        *skip_sse_sb = total_sse << 4 as core::ffi::c_int;
    }
    rate_sum = if rate_sum < 2147483647 as int64_t {
        rate_sum
    } else {
        2147483647 as int64_t
    };
    *out_rate_sum = rate_sum as core::ffi::c_int;
    *out_dist_sum = dist_sum;
}
#[inline]
unsafe extern "C-unwind" fn model_rd_for_sb_with_curvfit(
    cpi: *const AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
    mut out_rate_sum: *mut core::ffi::c_int,
    mut out_dist_sum: *mut int64_t,
    mut skip_txfm_sb: *mut uint8_t,
    mut skip_sse_sb: *mut int64_t,
    mut plane_rate: *mut core::ffi::c_int,
    mut plane_sse: *mut int64_t,
    mut plane_dist: *mut int64_t,
) {
    let ref_0: core::ffi::c_int = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int;
    let mut rate_sum: int64_t = 0 as int64_t;
    let mut dist_sum: int64_t = 0 as int64_t;
    let mut total_sse: int64_t = 0 as int64_t;
    let mut plane: core::ffi::c_int = plane_from;
    while plane <= plane_to {
        if plane != 0 && !(*xd).is_chroma_ref {
            break;
        }
        let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblockd_plane;
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        let mut dist: int64_t = 0;
        let mut sse: int64_t = 0;
        let mut rate: core::ffi::c_int = 0;
        let mut bw: core::ffi::c_int = 0;
        let mut bh: core::ffi::c_int = 0;
        let p: *const macroblock_plane = &raw mut *((*x).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblock_plane;
        let block_width: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let block_height: core::ffi::c_int = block_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        get_visible_dimensions(
            x,
            plane,
            plane_bsize,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            block_width,
            block_height,
            true_0 != 0,
            &raw mut bw,
            &raw mut bh,
        );
        sse = pixel_dist_visible_only(
            cpi,
            x,
            (*p).src.buf,
            (*p).src.stride,
            (*pd).dst.buf,
            (*pd).dst.stride,
            plane_bsize,
            block_height,
            block_width,
            bh,
            bw,
        ) as int64_t;
        model_rd_with_curvfit(
            cpi,
            x,
            plane_bsize,
            plane,
            sse,
            bw * bh,
            &raw mut rate,
            &raw mut dist,
        );
        if plane == 0 as core::ffi::c_int {
            (*x).pred_sse[ref_0 as usize] = (if sse
                < (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as int64_t
            {
                sse
            } else {
                (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as int64_t
            }) as core::ffi::c_uint;
        }
        total_sse += sse;
        rate_sum += rate as int64_t;
        dist_sum += dist;
        if !plane_rate.is_null() {
            *plane_rate.offset(plane as isize) = rate;
        }
        if !plane_sse.is_null() {
            *plane_sse.offset(plane as isize) = sse;
        }
        if !plane_dist.is_null() {
            *plane_dist.offset(plane as isize) = dist;
        }
        plane += 1;
    }
    if !skip_txfm_sb.is_null() {
        *skip_txfm_sb = (rate_sum == 0 as int64_t) as core::ffi::c_int as uint8_t;
    }
    if !skip_sse_sb.is_null() {
        *skip_sse_sb = total_sse << 4 as core::ffi::c_int;
    }
    *out_rate_sum = rate_sum as core::ffi::c_int;
    *out_dist_sum = dist_sum;
}
static mut model_rd_sb_fn: [model_rd_for_sb_type; 2] = unsafe {
    [
        Some(
            model_rd_for_sb
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMP,
                    BLOCK_SIZE,
                    *mut MACROBLOCK,
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut core::ffi::c_int,
                    *mut int64_t,
                    *mut uint8_t,
                    *mut int64_t,
                    *mut core::ffi::c_int,
                    *mut int64_t,
                    *mut int64_t,
                ) -> (),
        ),
        Some(
            model_rd_for_sb_with_curvfit
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMP,
                    BLOCK_SIZE,
                    *mut MACROBLOCK,
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *mut core::ffi::c_int,
                    *mut int64_t,
                    *mut uint8_t,
                    *mut int64_t,
                    *mut core::ffi::c_int,
                    *mut int64_t,
                    *mut int64_t,
                ) -> (),
        ),
    ]
};
static mut c2rust_aom_float_table_7: [core::ffi::c_float; 14] = [
    0.00232f32,
    0.00671f32,
    0.01257f32,
    0.01941f32,
    0.02673f32,
    0.03430f32,
    0.04211f32,
    0.04968f32,
    0.05750f32,
    0.06580f32,
    0.07507f32,
    0.08655f32,
    0.10242f32,
    0.12878f32,
];
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_sort_fi32_16(
    mut k: *mut core::ffi::c_float,
    mut v: *mut int32_t,
) {
    let maxf: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let minf: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    let mini: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf;
    *k.offset(1 as core::ffi::c_int as isize) = minf;
    *v.offset(0 as core::ffi::c_int as isize) = maxi as int32_t;
    *v.offset(1 as core::ffi::c_int as isize) = mini as int32_t;
    let maxf_0: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let minf_0: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_0: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    let mini_0: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_0;
    *k.offset(3 as core::ffi::c_int as isize) = minf_0;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_0 as int32_t;
    *v.offset(3 as core::ffi::c_int as isize) = mini_0 as int32_t;
    let maxf_1: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_1: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_1: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_1: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_1;
    *k.offset(5 as core::ffi::c_int as isize) = minf_1;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_1 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_1 as int32_t;
    let maxf_2: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_2: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_2: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_2: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_2;
    *k.offset(7 as core::ffi::c_int as isize) = minf_2;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_2 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_2 as int32_t;
    let maxf_3: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let minf_3: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let maxi_3: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    let mini_3: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    *k.offset(8 as core::ffi::c_int as isize) = maxf_3;
    *k.offset(9 as core::ffi::c_int as isize) = minf_3;
    *v.offset(8 as core::ffi::c_int as isize) = maxi_3 as int32_t;
    *v.offset(9 as core::ffi::c_int as isize) = mini_3 as int32_t;
    let maxf_4: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let minf_4: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let maxi_4: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    let mini_4: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    *k.offset(10 as core::ffi::c_int as isize) = maxf_4;
    *k.offset(11 as core::ffi::c_int as isize) = minf_4;
    *v.offset(10 as core::ffi::c_int as isize) = maxi_4 as int32_t;
    *v.offset(11 as core::ffi::c_int as isize) = mini_4 as int32_t;
    let maxf_5: core::ffi::c_float = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let minf_5: core::ffi::c_float = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let maxi_5: core::ffi::c_int = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    let mini_5: core::ffi::c_int = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    *k.offset(12 as core::ffi::c_int as isize) = maxf_5;
    *k.offset(13 as core::ffi::c_int as isize) = minf_5;
    *v.offset(12 as core::ffi::c_int as isize) = maxi_5 as int32_t;
    *v.offset(13 as core::ffi::c_int as isize) = mini_5 as int32_t;
    let maxf_6: core::ffi::c_float = if *k.offset(14 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(15 as core::ffi::c_int as isize)
    };
    let minf_6: core::ffi::c_float = if *k.offset(14 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(15 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let maxi_6: core::ffi::c_int = if *k.offset(14 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(15 as core::ffi::c_int as isize)
    };
    let mini_6: core::ffi::c_int = if *k.offset(14 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(15 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    *k.offset(14 as core::ffi::c_int as isize) = maxf_6;
    *k.offset(15 as core::ffi::c_int as isize) = minf_6;
    *v.offset(14 as core::ffi::c_int as isize) = maxi_6 as int32_t;
    *v.offset(15 as core::ffi::c_int as isize) = mini_6 as int32_t;
    let maxf_7: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let minf_7: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi_7: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    let mini_7: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf_7;
    *k.offset(2 as core::ffi::c_int as isize) = minf_7;
    *v.offset(0 as core::ffi::c_int as isize) = maxi_7 as int32_t;
    *v.offset(2 as core::ffi::c_int as isize) = mini_7 as int32_t;
    let maxf_8: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let minf_8: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_8: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    let mini_8: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_8;
    *k.offset(3 as core::ffi::c_int as isize) = minf_8;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_8 as int32_t;
    *v.offset(3 as core::ffi::c_int as isize) = mini_8 as int32_t;
    let maxf_9: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_9: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_9: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_9: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_9;
    *k.offset(6 as core::ffi::c_int as isize) = minf_9;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_9 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_9 as int32_t;
    let maxf_10: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_10: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_10: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_10: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_10;
    *k.offset(7 as core::ffi::c_int as isize) = minf_10;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_10 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_10 as int32_t;
    let maxf_11: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_11: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let maxi_11: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_11: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    *k.offset(8 as core::ffi::c_int as isize) = maxf_11;
    *k.offset(10 as core::ffi::c_int as isize) = minf_11;
    *v.offset(8 as core::ffi::c_int as isize) = maxi_11 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_11 as int32_t;
    let maxf_12: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let minf_12: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let maxi_12: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    let mini_12: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    *k.offset(9 as core::ffi::c_int as isize) = maxf_12;
    *k.offset(11 as core::ffi::c_int as isize) = minf_12;
    *v.offset(9 as core::ffi::c_int as isize) = maxi_12 as int32_t;
    *v.offset(11 as core::ffi::c_int as isize) = mini_12 as int32_t;
    let maxf_13: core::ffi::c_float = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_13: core::ffi::c_float = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let maxi_13: core::ffi::c_int = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_13: core::ffi::c_int = if *k.offset(12 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    *k.offset(12 as core::ffi::c_int as isize) = maxf_13;
    *k.offset(14 as core::ffi::c_int as isize) = minf_13;
    *v.offset(12 as core::ffi::c_int as isize) = maxi_13 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_13 as int32_t;
    let maxf_14: core::ffi::c_float = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(15 as core::ffi::c_int as isize)
    };
    let minf_14: core::ffi::c_float = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(15 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let maxi_14: core::ffi::c_int = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(15 as core::ffi::c_int as isize)
    };
    let mini_14: core::ffi::c_int = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(15 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    *k.offset(13 as core::ffi::c_int as isize) = maxf_14;
    *k.offset(15 as core::ffi::c_int as isize) = minf_14;
    *v.offset(13 as core::ffi::c_int as isize) = maxi_14 as int32_t;
    *v.offset(15 as core::ffi::c_int as isize) = mini_14 as int32_t;
    let maxf_15: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let minf_15: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_15: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    let mini_15: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_15;
    *k.offset(2 as core::ffi::c_int as isize) = minf_15;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_15 as int32_t;
    *v.offset(2 as core::ffi::c_int as isize) = mini_15 as int32_t;
    let maxf_16: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_16: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_16: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_16: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_16;
    *k.offset(6 as core::ffi::c_int as isize) = minf_16;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_16 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_16 as int32_t;
    let maxf_17: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_17: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi_17: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_17: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf_17;
    *k.offset(4 as core::ffi::c_int as isize) = minf_17;
    *v.offset(0 as core::ffi::c_int as isize) = maxi_17 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_17 as int32_t;
    let maxf_18: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_18: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_18: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_18: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_18;
    *k.offset(7 as core::ffi::c_int as isize) = minf_18;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_18 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_18 as int32_t;
    let maxf_19: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_19: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let maxi_19: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_19: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    *k.offset(9 as core::ffi::c_int as isize) = maxf_19;
    *k.offset(10 as core::ffi::c_int as isize) = minf_19;
    *v.offset(9 as core::ffi::c_int as isize) = maxi_19 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_19 as int32_t;
    let maxf_20: core::ffi::c_float = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_20: core::ffi::c_float = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let maxi_20: core::ffi::c_int = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_20: core::ffi::c_int = if *k.offset(13 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    *k.offset(13 as core::ffi::c_int as isize) = maxf_20;
    *k.offset(14 as core::ffi::c_int as isize) = minf_20;
    *v.offset(13 as core::ffi::c_int as isize) = maxi_20 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_20 as int32_t;
    let maxf_21: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_21: core::ffi::c_float = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let maxi_21: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_21: core::ffi::c_int = if *k.offset(8 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    *k.offset(8 as core::ffi::c_int as isize) = maxf_21;
    *k.offset(12 as core::ffi::c_int as isize) = minf_21;
    *v.offset(8 as core::ffi::c_int as isize) = maxi_21 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_21 as int32_t;
    let maxf_22: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(15 as core::ffi::c_int as isize)
    };
    let minf_22: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(15 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let maxi_22: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(15 as core::ffi::c_int as isize)
    };
    let mini_22: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(15 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    *k.offset(11 as core::ffi::c_int as isize) = maxf_22;
    *k.offset(15 as core::ffi::c_int as isize) = minf_22;
    *v.offset(11 as core::ffi::c_int as isize) = maxi_22 as int32_t;
    *v.offset(15 as core::ffi::c_int as isize) = mini_22 as int32_t;
    let maxf_23: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_23: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_23: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_23: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_23;
    *k.offset(5 as core::ffi::c_int as isize) = minf_23;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_23 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_23 as int32_t;
    let maxf_24: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_24: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_24: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_24: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_24;
    *k.offset(6 as core::ffi::c_int as isize) = minf_24;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_24 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_24 as int32_t;
    let maxf_25: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let minf_25: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let maxi_25: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    let mini_25: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    *k.offset(9 as core::ffi::c_int as isize) = maxf_25;
    *k.offset(13 as core::ffi::c_int as isize) = minf_25;
    *v.offset(9 as core::ffi::c_int as isize) = maxi_25 as int32_t;
    *v.offset(13 as core::ffi::c_int as isize) = mini_25 as int32_t;
    let maxf_26: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_26: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let maxi_26: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_26: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    *k.offset(10 as core::ffi::c_int as isize) = maxf_26;
    *k.offset(14 as core::ffi::c_int as isize) = minf_26;
    *v.offset(10 as core::ffi::c_int as isize) = maxi_26 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_26 as int32_t;
    let maxf_27: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_27: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi_27: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_27: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf_27;
    *k.offset(8 as core::ffi::c_int as isize) = minf_27;
    *v.offset(0 as core::ffi::c_int as isize) = maxi_27 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_27 as int32_t;
    let maxf_28: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(15 as core::ffi::c_int as isize)
    };
    let minf_28: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *k.offset(15 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_28: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(15 as core::ffi::c_int as isize)
    };
    let mini_28: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(15 as core::ffi::c_int as isize)
    {
        *v.offset(15 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_28;
    *k.offset(15 as core::ffi::c_int as isize) = minf_28;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_28 as int32_t;
    *v.offset(15 as core::ffi::c_int as isize) = mini_28 as int32_t;
    let maxf_29: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_29: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_29: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_29: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_29;
    *k.offset(4 as core::ffi::c_int as isize) = minf_29;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_29 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_29 as int32_t;
    let maxf_30: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_30: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_30: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_30: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_30;
    *k.offset(6 as core::ffi::c_int as isize) = minf_30;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_30 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_30 as int32_t;
    let maxf_31: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_31: core::ffi::c_float = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let maxi_31: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_31: core::ffi::c_int = if *k.offset(9 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    *k.offset(9 as core::ffi::c_int as isize) = maxf_31;
    *k.offset(12 as core::ffi::c_int as isize) = minf_31;
    *v.offset(9 as core::ffi::c_int as isize) = maxi_31 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_31 as int32_t;
    let maxf_32: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_32: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let maxi_32: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_32: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    *k.offset(11 as core::ffi::c_int as isize) = maxf_32;
    *k.offset(14 as core::ffi::c_int as isize) = minf_32;
    *v.offset(11 as core::ffi::c_int as isize) = maxi_32 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_32 as int32_t;
    let maxf_33: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_33: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_33: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_33: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_33;
    *k.offset(4 as core::ffi::c_int as isize) = minf_33;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_33 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_33 as int32_t;
    let maxf_34: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_34: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_34: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_34: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_34;
    *k.offset(5 as core::ffi::c_int as isize) = minf_34;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_34 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_34 as int32_t;
    let maxf_35: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_35: core::ffi::c_float = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let maxi_35: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_35: core::ffi::c_int = if *k.offset(10 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    *k.offset(10 as core::ffi::c_int as isize) = maxf_35;
    *k.offset(12 as core::ffi::c_int as isize) = minf_35;
    *v.offset(10 as core::ffi::c_int as isize) = maxi_35 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_35 as int32_t;
    let maxf_36: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let minf_36: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let maxi_36: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    let mini_36: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    *k.offset(11 as core::ffi::c_int as isize) = maxf_36;
    *k.offset(13 as core::ffi::c_int as isize) = minf_36;
    *v.offset(11 as core::ffi::c_int as isize) = maxi_36 as int32_t;
    *v.offset(13 as core::ffi::c_int as isize) = mini_36 as int32_t;
    let maxf_37: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let minf_37: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_37: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    let mini_37: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_37;
    *k.offset(9 as core::ffi::c_int as isize) = minf_37;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_37 as int32_t;
    *v.offset(9 as core::ffi::c_int as isize) = mini_37 as int32_t;
    let maxf_38: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_38: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_38: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_38: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_38;
    *k.offset(14 as core::ffi::c_int as isize) = minf_38;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_38 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_38 as int32_t;
    let maxf_39: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_39: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_39: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_39: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_39;
    *k.offset(4 as core::ffi::c_int as isize) = minf_39;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_39 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_39 as int32_t;
    let maxf_40: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_40: core::ffi::c_float = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let maxi_40: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_40: core::ffi::c_int = if *k.offset(11 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    *k.offset(11 as core::ffi::c_int as isize) = maxf_40;
    *k.offset(12 as core::ffi::c_int as isize) = minf_40;
    *v.offset(11 as core::ffi::c_int as isize) = maxi_40 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_40 as int32_t;
    let maxf_41: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_41: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_41: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_41: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_41;
    *k.offset(8 as core::ffi::c_int as isize) = minf_41;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_41 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_41 as int32_t;
    let maxf_42: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_42: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_42: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_42: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_42;
    *k.offset(10 as core::ffi::c_int as isize) = minf_42;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_42 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_42 as int32_t;
    let maxf_43: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let minf_43: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_43: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    let mini_43: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_43;
    *k.offset(13 as core::ffi::c_int as isize) = minf_43;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_43 as int32_t;
    *v.offset(13 as core::ffi::c_int as isize) = mini_43 as int32_t;
    let maxf_44: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(14 as core::ffi::c_int as isize)
    };
    let minf_44: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *k.offset(14 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_44: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(14 as core::ffi::c_int as isize)
    };
    let mini_44: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(14 as core::ffi::c_int as isize)
    {
        *v.offset(14 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_44;
    *k.offset(14 as core::ffi::c_int as isize) = minf_44;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_44 as int32_t;
    *v.offset(14 as core::ffi::c_int as isize) = mini_44 as int32_t;
    let maxf_45: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let minf_45: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_45: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    let mini_45: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_45;
    *k.offset(11 as core::ffi::c_int as isize) = minf_45;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_45 as int32_t;
    *v.offset(11 as core::ffi::c_int as isize) = mini_45 as int32_t;
    let maxf_46: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_46: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_46: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_46: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_46;
    *k.offset(8 as core::ffi::c_int as isize) = minf_46;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_46 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_46 as int32_t;
    let maxf_47: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_47: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_47: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_47: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_47;
    *k.offset(12 as core::ffi::c_int as isize) = minf_47;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_47 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_47 as int32_t;
    let maxf_48: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(13 as core::ffi::c_int as isize)
    };
    let minf_48: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *k.offset(13 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_48: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(13 as core::ffi::c_int as isize)
    };
    let mini_48: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(13 as core::ffi::c_int as isize)
    {
        *v.offset(13 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_48;
    *k.offset(13 as core::ffi::c_int as isize) = minf_48;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_48 as int32_t;
    *v.offset(13 as core::ffi::c_int as isize) = mini_48 as int32_t;
    let maxf_49: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_49: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_49: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_49: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_49;
    *k.offset(10 as core::ffi::c_int as isize) = minf_49;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_49 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_49 as int32_t;
    let maxf_50: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_50: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_50: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_50: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_50;
    *k.offset(12 as core::ffi::c_int as isize) = minf_50;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_50 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_50 as int32_t;
    let maxf_51: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let minf_51: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_51: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    let mini_51: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_51;
    *k.offset(9 as core::ffi::c_int as isize) = minf_51;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_51 as int32_t;
    *v.offset(9 as core::ffi::c_int as isize) = mini_51 as int32_t;
    let maxf_52: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_52: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_52: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_52: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_52;
    *k.offset(12 as core::ffi::c_int as isize) = minf_52;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_52 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_52 as int32_t;
    let maxf_53: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_53: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_53: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_53: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_53;
    *k.offset(8 as core::ffi::c_int as isize) = minf_53;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_53 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_53 as int32_t;
    let maxf_54: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(12 as core::ffi::c_int as isize)
    };
    let minf_54: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *k.offset(12 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_54: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(12 as core::ffi::c_int as isize)
    };
    let mini_54: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(12 as core::ffi::c_int as isize)
    {
        *v.offset(12 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_54;
    *k.offset(12 as core::ffi::c_int as isize) = minf_54;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_54 as int32_t;
    *v.offset(12 as core::ffi::c_int as isize) = mini_54 as int32_t;
    let maxf_55: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let minf_55: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_55: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    let mini_55: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_55;
    *k.offset(9 as core::ffi::c_int as isize) = minf_55;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_55 as int32_t;
    *v.offset(9 as core::ffi::c_int as isize) = mini_55 as int32_t;
    let maxf_56: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_56: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_56: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_56: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_56;
    *k.offset(10 as core::ffi::c_int as isize) = minf_56;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_56 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_56 as int32_t;
    let maxf_57: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_57: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_57: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_57: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_57;
    *k.offset(8 as core::ffi::c_int as isize) = minf_57;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_57 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_57 as int32_t;
    let maxf_58: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(11 as core::ffi::c_int as isize)
    };
    let minf_58: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *k.offset(11 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_58: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(11 as core::ffi::c_int as isize)
    };
    let mini_58: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(11 as core::ffi::c_int as isize)
    {
        *v.offset(11 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_58;
    *k.offset(11 as core::ffi::c_int as isize) = minf_58;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_58 as int32_t;
    *v.offset(11 as core::ffi::c_int as isize) = mini_58 as int32_t;
    let maxf_59: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_59: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_59: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_59: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_59;
    *k.offset(8 as core::ffi::c_int as isize) = minf_59;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_59 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_59 as int32_t;
    let maxf_60: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(10 as core::ffi::c_int as isize)
    };
    let minf_60: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *k.offset(10 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_60: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(10 as core::ffi::c_int as isize)
    };
    let mini_60: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(10 as core::ffi::c_int as isize)
    {
        *v.offset(10 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_60;
    *k.offset(10 as core::ffi::c_int as isize) = minf_60;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_60 as int32_t;
    *v.offset(10 as core::ffi::c_int as isize) = mini_60 as int32_t;
    let maxf_61: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_61: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_61: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_61: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_61;
    *k.offset(8 as core::ffi::c_int as isize) = minf_61;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_61 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_61 as int32_t;
    let maxf_62: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(9 as core::ffi::c_int as isize)
    };
    let minf_62: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *k.offset(9 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_62: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(9 as core::ffi::c_int as isize)
    };
    let mini_62: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(9 as core::ffi::c_int as isize)
    {
        *v.offset(9 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_62;
    *k.offset(9 as core::ffi::c_int as isize) = minf_62;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_62 as int32_t;
    *v.offset(9 as core::ffi::c_int as isize) = mini_62 as int32_t;
    let maxf_63: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(8 as core::ffi::c_int as isize)
    };
    let minf_63: core::ffi::c_float = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *k.offset(8 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let maxi_63: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(8 as core::ffi::c_int as isize)
    };
    let mini_63: core::ffi::c_int = if *k.offset(7 as core::ffi::c_int as isize)
        >= *k.offset(8 as core::ffi::c_int as isize)
    {
        *v.offset(8 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    *k.offset(7 as core::ffi::c_int as isize) = maxf_63;
    *k.offset(8 as core::ffi::c_int as isize) = minf_63;
    *v.offset(7 as core::ffi::c_int as isize) = maxi_63 as int32_t;
    *v.offset(8 as core::ffi::c_int as isize) = mini_63 as int32_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_sort_fi32_8(
    mut k: *mut core::ffi::c_float,
    mut v: *mut int32_t,
) {
    let maxf: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let minf: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    let mini: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(1 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf;
    *k.offset(1 as core::ffi::c_int as isize) = minf;
    *v.offset(0 as core::ffi::c_int as isize) = maxi as int32_t;
    *v.offset(1 as core::ffi::c_int as isize) = mini as int32_t;
    let maxf_0: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let minf_0: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_0: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    let mini_0: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_0;
    *k.offset(3 as core::ffi::c_int as isize) = minf_0;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_0 as int32_t;
    *v.offset(3 as core::ffi::c_int as isize) = mini_0 as int32_t;
    let maxf_1: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_1: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_1: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_1: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_1;
    *k.offset(5 as core::ffi::c_int as isize) = minf_1;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_1 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_1 as int32_t;
    let maxf_2: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_2: core::ffi::c_float = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let maxi_2: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_2: core::ffi::c_int = if *k.offset(6 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    *k.offset(6 as core::ffi::c_int as isize) = maxf_2;
    *k.offset(7 as core::ffi::c_int as isize) = minf_2;
    *v.offset(6 as core::ffi::c_int as isize) = maxi_2 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_2 as int32_t;
    let maxf_3: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let minf_3: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi_3: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    let mini_3: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf_3;
    *k.offset(2 as core::ffi::c_int as isize) = minf_3;
    *v.offset(0 as core::ffi::c_int as isize) = maxi_3 as int32_t;
    *v.offset(2 as core::ffi::c_int as isize) = mini_3 as int32_t;
    let maxf_4: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let minf_4: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_4: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    let mini_4: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(3 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_4;
    *k.offset(3 as core::ffi::c_int as isize) = minf_4;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_4 as int32_t;
    *v.offset(3 as core::ffi::c_int as isize) = mini_4 as int32_t;
    let maxf_5: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_5: core::ffi::c_float = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let maxi_5: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_5: core::ffi::c_int = if *k.offset(4 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    *k.offset(4 as core::ffi::c_int as isize) = maxf_5;
    *k.offset(6 as core::ffi::c_int as isize) = minf_5;
    *v.offset(4 as core::ffi::c_int as isize) = maxi_5 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_5 as int32_t;
    let maxf_6: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_6: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_6: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_6: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_6;
    *k.offset(7 as core::ffi::c_int as isize) = minf_6;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_6 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_6 as int32_t;
    let maxf_7: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let minf_7: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_7: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    let mini_7: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(2 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_7;
    *k.offset(2 as core::ffi::c_int as isize) = minf_7;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_7 as int32_t;
    *v.offset(2 as core::ffi::c_int as isize) = mini_7 as int32_t;
    let maxf_8: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_8: core::ffi::c_float = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let maxi_8: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_8: core::ffi::c_int = if *k.offset(5 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    *k.offset(5 as core::ffi::c_int as isize) = maxf_8;
    *k.offset(6 as core::ffi::c_int as isize) = minf_8;
    *v.offset(5 as core::ffi::c_int as isize) = maxi_8 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_8 as int32_t;
    let maxf_9: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(0 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_9: core::ffi::c_float = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(0 as core::ffi::c_int as isize)
    };
    let maxi_9: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(0 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_9: core::ffi::c_int = if *k.offset(0 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(0 as core::ffi::c_int as isize)
    };
    *k.offset(0 as core::ffi::c_int as isize) = maxf_9;
    *k.offset(4 as core::ffi::c_int as isize) = minf_9;
    *v.offset(0 as core::ffi::c_int as isize) = maxi_9 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_9 as int32_t;
    let maxf_10: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(7 as core::ffi::c_int as isize)
    };
    let minf_10: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *k.offset(7 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_10: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(7 as core::ffi::c_int as isize)
    };
    let mini_10: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(7 as core::ffi::c_int as isize)
    {
        *v.offset(7 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_10;
    *k.offset(7 as core::ffi::c_int as isize) = minf_10;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_10 as int32_t;
    *v.offset(7 as core::ffi::c_int as isize) = mini_10 as int32_t;
    let maxf_11: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_11: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_11: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_11: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_11;
    *k.offset(5 as core::ffi::c_int as isize) = minf_11;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_11 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_11 as int32_t;
    let maxf_12: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_12: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_12: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_12: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_12;
    *k.offset(6 as core::ffi::c_int as isize) = minf_12;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_12 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_12 as int32_t;
    let maxf_13: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(1 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_13: core::ffi::c_float = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(1 as core::ffi::c_int as isize)
    };
    let maxi_13: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(1 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_13: core::ffi::c_int = if *k.offset(1 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(1 as core::ffi::c_int as isize)
    };
    *k.offset(1 as core::ffi::c_int as isize) = maxf_13;
    *k.offset(4 as core::ffi::c_int as isize) = minf_13;
    *v.offset(1 as core::ffi::c_int as isize) = maxi_13 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_13 as int32_t;
    let maxf_14: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(6 as core::ffi::c_int as isize)
    };
    let minf_14: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *k.offset(6 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_14: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(6 as core::ffi::c_int as isize)
    };
    let mini_14: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(6 as core::ffi::c_int as isize)
    {
        *v.offset(6 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_14;
    *k.offset(6 as core::ffi::c_int as isize) = minf_14;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_14 as int32_t;
    *v.offset(6 as core::ffi::c_int as isize) = mini_14 as int32_t;
    let maxf_15: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(2 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_15: core::ffi::c_float = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(2 as core::ffi::c_int as isize)
    };
    let maxi_15: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(2 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_15: core::ffi::c_int = if *k.offset(2 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(2 as core::ffi::c_int as isize)
    };
    *k.offset(2 as core::ffi::c_int as isize) = maxf_15;
    *k.offset(4 as core::ffi::c_int as isize) = minf_15;
    *v.offset(2 as core::ffi::c_int as isize) = maxi_15 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_15 as int32_t;
    let maxf_16: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(5 as core::ffi::c_int as isize)
    };
    let minf_16: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *k.offset(5 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_16: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(5 as core::ffi::c_int as isize)
    };
    let mini_16: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(5 as core::ffi::c_int as isize)
    {
        *v.offset(5 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_16;
    *k.offset(5 as core::ffi::c_int as isize) = minf_16;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_16 as int32_t;
    *v.offset(5 as core::ffi::c_int as isize) = mini_16 as int32_t;
    let maxf_17: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(3 as core::ffi::c_int as isize)
    } else {
        *k.offset(4 as core::ffi::c_int as isize)
    };
    let minf_17: core::ffi::c_float = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *k.offset(4 as core::ffi::c_int as isize)
    } else {
        *k.offset(3 as core::ffi::c_int as isize)
    };
    let maxi_17: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(3 as core::ffi::c_int as isize)
    } else {
        *v.offset(4 as core::ffi::c_int as isize)
    };
    let mini_17: core::ffi::c_int = if *k.offset(3 as core::ffi::c_int as isize)
        >= *k.offset(4 as core::ffi::c_int as isize)
    {
        *v.offset(4 as core::ffi::c_int as isize)
    } else {
        *v.offset(3 as core::ffi::c_int as isize)
    };
    *k.offset(3 as core::ffi::c_int as isize) = maxf_17;
    *k.offset(4 as core::ffi::c_int as isize) = minf_17;
    *v.offset(3 as core::ffi::c_int as isize) = maxi_17 as int32_t;
    *v.offset(4 as core::ffi::c_int as isize) = mini_17 as int32_t;
}
static mut av1_tx_type_nn_weights_4x4_hor_layer0: [core::ffi::c_float; 32] = [
    -1.64947f32,
    -1.54497f32,
    -1.62832f32,
    -0.17774f32,
    -2.89498f32,
    -0.72498f32,
    0.72036f32,
    0.17996f32,
    1.20000f32,
    -0.27654f32,
    0.77396f32,
    1.21684f32,
    -1.75909f32,
    -0.51272f32,
    -1.25923f32,
    0.35005f32,
    -0.04257f32,
    -0.23389f32,
    -0.41841f32,
    -0.08229f32,
    0.09503f32,
    2.73144f32,
    -0.16875f32,
    -0.23482f32,
    0.02194f32,
    -0.26427f32,
    0.28049f32,
    0.21260f32,
    1.35792f32,
    0.27733f32,
    0.88660f32,
    -0.68304f32,
];
static mut av1_tx_type_nn_bias_4x4_hor_layer0: [core::ffi::c_float; 8] = [
    1.38742f32,
    0.59540f32,
    -1.37622f32,
    1.92114f32,
    0.00000f32,
    -0.38998f32,
    -0.32726f32,
    -0.15650f32,
];
static mut av1_tx_type_nn_weights_4x4_hor_layer1: [core::ffi::c_float; 32] = [
    1.65254f32,
    1.00915f32,
    -0.89318f32,
    -2.05142f32,
    -0.23235f32,
    0.96781f32,
    -0.37145f32,
    -0.21056f32,
    1.13891f32,
    0.38675f32,
    0.87739f32,
    -1.42697f32,
    0.48015f32,
    0.61883f32,
    -0.03979f32,
    0.11487f32,
    0.48042f32,
    0.45200f32,
    -0.23242f32,
    0.75166f32,
    0.55458f32,
    0.39452f32,
    -0.35285f32,
    1.59120f32,
    -1.49221f32,
    -0.48349f32,
    -0.64692f32,
    1.49297f32,
    -0.26782f32,
    -0.65416f32,
    -0.10648f32,
    0.05568f32,
];
static mut av1_tx_type_nn_bias_4x4_hor_layer1: [core::ffi::c_float; 4] = [
    4.07177f32,
    3.26961f32,
    0.58083f32,
    1.21199f32,
];
static mut av1_tx_type_nnconfig_4x4_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_4x4_ver_layer0: [core::ffi::c_float; 32] = [
    -0.02032f32,
    2.61610f32,
    0.02098f32,
    -0.30217f32,
    0.12637f32,
    0.11017f32,
    -3.01996f32,
    0.35144f32,
    1.93776f32,
    -0.20463f32,
    1.64102f32,
    -1.41986f32,
    -3.66717f32,
    -0.51655f32,
    0.43910f32,
    0.37778f32,
    -1.02634f32,
    0.85337f32,
    -0.69753f32,
    1.00206f32,
    2.11784f32,
    1.89427f32,
    1.92919f32,
    0.43201f32,
    -1.67358f32,
    -1.67035f32,
    -1.54623f32,
    0.16714f32,
    -0.06589f32,
    -0.28142f32,
    -0.33118f32,
    1.72227f32,
];
static mut av1_tx_type_nn_bias_4x4_ver_layer0: [core::ffi::c_float; 8] = [
    -0.33685f32,
    0.22025f32,
    0.28140f32,
    0.56138f32,
    0.93489f32,
    -1.77048f32,
    1.34989f32,
    -0.93747f32,
];
static mut av1_tx_type_nn_weights_4x4_ver_layer1: [core::ffi::c_float; 32] = [
    -1.39506f32,
    -1.06271f32,
    -1.10886f32,
    -1.69719f32,
    0.19699f32,
    -2.39850f32,
    -1.26457f32,
    0.75328f32,
    -1.26005f32,
    -0.82738f32,
    -0.12015f32,
    -1.02702f32,
    1.40828f32,
    -2.37739f32,
    -0.65639f32,
    -0.71992f32,
    -0.90453f32,
    -1.12510f32,
    -2.41362f32,
    -1.16061f32,
    -1.85577f32,
    -0.99165f32,
    -1.91366f32,
    0.16785f32,
    0.34776f32,
    0.58154f32,
    -0.18217f32,
    -0.29257f32,
    -0.86315f32,
    -0.53336f32,
    0.30320f32,
    -1.32331f32,
];
static mut av1_tx_type_nn_bias_4x4_ver_layer1: [core::ffi::c_float; 4] = [
    -1.31519f32,
    -3.26321f32,
    1.71794f32,
    -1.90778f32,
];
static mut av1_tx_type_nnconfig_4x4_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_4x8_hor_layer0: [core::ffi::c_float; 32] = [
    0.00218f32,
    -0.41880f32,
    -0.61215f32,
    -0.92588f32,
    0.54291f32,
    -0.10898f32,
    0.70691f32,
    0.46819f32,
    -1.61598f32,
    -0.08834f32,
    -0.96839f32,
    1.18489f32,
    -0.45171f32,
    -0.65445f32,
    -0.32179f32,
    -0.10399f32,
    1.04379f32,
    0.91895f32,
    0.85589f32,
    0.08267f32,
    1.35388f32,
    -2.03096f32,
    0.08168f32,
    -0.06372f32,
    -0.26732f32,
    -0.48262f32,
    -0.08682f32,
    2.44071f32,
    -1.35896f32,
    -1.17121f32,
    1.68866f32,
    0.10357f32,
];
static mut av1_tx_type_nn_bias_4x8_hor_layer0: [core::ffi::c_float; 8] = [
    2.93391f32,
    0.66831f32,
    -0.21419f32,
    0.00000f32,
    -0.72878f32,
    0.15127f32,
    -1.46755f32,
    0.16658f32,
];
static mut av1_tx_type_nn_weights_4x8_hor_layer1: [core::ffi::c_float; 32] = [
    -1.52077f32,
    -1.06243f32,
    0.35319f32,
    -0.49207f32,
    0.54524f32,
    0.44271f32,
    1.37117f32,
    -0.38957f32,
    -1.28889f32,
    -0.57133f32,
    0.04658f32,
    0.62278f32,
    0.37984f32,
    0.33247f32,
    1.65547f32,
    -0.56806f32,
    -1.38645f32,
    -0.76258f32,
    0.67926f32,
    0.08783f32,
    -0.01443f32,
    0.34950f32,
    1.45812f32,
    -0.51332f32,
    -1.41331f32,
    -0.16453f32,
    0.05755f32,
    0.31405f32,
    -0.50191f32,
    0.18219f32,
    1.83664f32,
    -0.75276f32,
];
static mut av1_tx_type_nn_bias_4x8_hor_layer1: [core::ffi::c_float; 4] = [
    -1.17455f32,
    -2.26089f32,
    -1.79863f32,
    -2.26333f32,
];
static mut av1_tx_type_nnconfig_4x8_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_4x8_ver_layer0: [core::ffi::c_float; 128] = [
    -0.00952f32,
    -0.98858f32,
    -0.93181f32,
    1.39594f32,
    0.96559f32,
    0.18162f32,
    -0.76064f32,
    -0.06066f32,
    0.07907f32,
    -0.09365f32,
    -0.21313f32,
    -0.02187f32,
    -2.61707f32,
    -2.68702f32,
    -0.10982f32,
    0.18559f32,
    1.17049f32,
    1.11387f32,
    1.12697f32,
    1.05804f32,
    1.12764f32,
    1.06318f32,
    1.12052f32,
    0.17406f32,
    1.83157f32,
    0.19362f32,
    0.46910f32,
    0.39608f32,
    0.33342f32,
    0.40083f32,
    0.27645f32,
    1.06864f32,
    -4.06645f32,
    -0.38775f32,
    -0.11070f32,
    0.03781f32,
    -0.09141f32,
    0.06185f32,
    -0.04852f32,
    0.20163f32,
    0.16784f32,
    0.16641f32,
    -0.50941f32,
    -0.61087f32,
    2.07008f32,
    -0.82381f32,
    -0.85558f32,
    0.05528f32,
    -0.10535f32,
    -2.81150f32,
    0.67038f32,
    0.43643f32,
    0.49062f32,
    -0.04465f32,
    0.90438f32,
    0.00977f32,
    0.46272f32,
    1.59751f32,
    0.95234f32,
    0.35086f32,
    0.85624f32,
    0.73149f32,
    1.67779f32,
    -2.21511f32,
    -1.24746f32,
    -1.09014f32,
    -0.92441f32,
    -1.22591f32,
    -1.06961f32,
    -0.95897f32,
    -1.24956f32,
    0.73797f32,
    1.23275f32,
    -0.60064f32,
    -0.07851f32,
    0.14397f32,
    0.22110f32,
    -0.04422f32,
    0.14350f32,
    0.75926f32,
    0.35032f32,
    0.48104f32,
    2.81408f32,
    0.34662f32,
    0.42090f32,
    0.35521f32,
    -1.36804f32,
    -0.14974f32,
    -0.47696f32,
    -0.07892f32,
    0.36910f32,
    0.32299f32,
    0.23916f32,
    0.06032f32,
    -0.17844f32,
    -0.17558f32,
    -1.42746f32,
    -0.55828f32,
    -1.00418f32,
    -0.64823f32,
    -0.73654f32,
    -0.85197f32,
    -1.50989f32,
    1.69385f32,
    -0.04973f32,
    -0.09273f32,
    1.04249f32,
    0.79235f32,
    1.13229f32,
    0.99617f32,
    0.03851f32,
    0.56334f32,
    0.90795f32,
    1.08296f32,
    0.58519f32,
    1.74765f32,
    0.63971f32,
    1.35951f32,
    0.07803f32,
    -0.05127f32,
    0.26514f32,
    -0.84629f32,
    -0.66343f32,
    -2.10630f32,
    0.11017f32,
    2.18528f32,
    -0.21958f32,
    0.05970f32,
];
static mut av1_tx_type_nn_bias_4x8_ver_layer0: [core::ffi::c_float; 16] = [
    0.04205f32,
    0.22260f32,
    -1.03870f32,
    -1.19568f32,
    0.44283f32,
    0.01143f32,
    0.00235f32,
    4.26772f32,
    0.44364f32,
    -0.33199f32,
    -0.39076f32,
    -0.35129f32,
    0.08288f32,
    0.18195f32,
    -0.79890f32,
    0.10047f32,
];
static mut av1_tx_type_nn_weights_4x8_ver_layer1: [core::ffi::c_float; 64] = [
    -0.38193f32,
    -0.12095f32,
    1.57802f32,
    0.34932f32,
    -0.47333f32,
    -0.12304f32,
    -0.01736f32,
    -2.52445f32,
    0.18983f32,
    -0.64707f32,
    -0.60889f32,
    -0.53750f32,
    0.91666f32,
    -0.62823f32,
    -0.13377f32,
    -0.43594f32,
    -0.38618f32,
    -0.01328f32,
    0.97457f32,
    1.48589f32,
    -1.03238f32,
    -0.33459f32,
    -0.35108f32,
    -2.42417f32,
    0.60229f32,
    0.06824f32,
    -0.75495f32,
    0.26902f32,
    0.65311f32,
    -0.23887f32,
    -0.44604f32,
    -0.55800f32,
    -0.33842f32,
    0.04259f32,
    -0.59589f32,
    0.49738f32,
    -0.62301f32,
    -0.30896f32,
    -0.29602f32,
    -2.57052f32,
    2.00943f32,
    -0.66490f32,
    -0.76312f32,
    0.28256f32,
    1.06311f32,
    -0.38364f32,
    -0.63508f32,
    -0.57609f32,
    -0.88765f32,
    -1.04403f32,
    -0.46531f32,
    0.34084f32,
    -1.20498f32,
    -0.68352f32,
    -0.72251f32,
    -2.63242f32,
    -0.68736f32,
    -0.37904f32,
    -1.32371f32,
    0.47288f32,
    1.51904f32,
    0.78372f32,
    -1.01830f32,
    -1.01848f32,
];
static mut av1_tx_type_nn_bias_4x8_ver_layer1: [core::ffi::c_float; 4] = [
    -1.45955f32,
    -2.08949f32,
    -1.24813f32,
    -1.55368f32,
];
static mut av1_tx_type_nnconfig_4x8_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x4_hor_layer0: [core::ffi::c_float; 128] = [
    -0.22492f32,
    0.13341f32,
    -4.03243f32,
    -0.64015f32,
    0.02783f32,
    0.60466f32,
    -0.13335f32,
    0.16828f32,
    0.12336f32,
    0.52904f32,
    1.18455f32,
    -0.32425f32,
    0.13052f32,
    0.93810f32,
    -3.71165f32,
    0.02990f32,
    -4.63558f32,
    0.05666f32,
    0.03524f32,
    -0.07449f32,
    -0.44006f32,
    -0.33215f32,
    -0.33713f32,
    0.08097f32,
    0.60873f32,
    0.29582f32,
    0.21696f32,
    -0.78729f32,
    -0.16757f32,
    -0.26567f32,
    -0.00720f32,
    -1.11226f32,
    1.58189f32,
    1.58463f32,
    1.48536f32,
    1.54374f32,
    1.60069f32,
    1.46125f32,
    1.53932f32,
    0.05974f32,
    -1.82192f32,
    0.47043f32,
    0.38090f32,
    0.20833f32,
    -0.05637f32,
    0.05183f32,
    0.01323f32,
    -0.25662f32,
    0.78634f32,
    -0.55069f32,
    -0.02975f32,
    -1.29294f32,
    -0.77192f32,
    -2.34299f32,
    -1.28074f32,
    0.77894f32,
    -1.69740f32,
    -1.66032f32,
    -1.44323f32,
    -1.55063f32,
    -1.50845f32,
    -1.23690f32,
    -1.80663f32,
    0.75079f32,
    2.32551f32,
    0.05878f32,
    0.80438f32,
    0.88584f32,
    0.69153f32,
    0.89060f32,
    0.73660f32,
    0.87259f32,
    -0.00745f32,
    -1.30044f32,
    -0.59430f32,
    2.07270f32,
    1.03307f32,
    -0.84697f32,
    -1.19393f32,
    0.17549f32,
    -0.24978f32,
    -3.67234f32,
    0.20781f32,
    -0.53946f32,
    -0.05068f32,
    0.88274f32,
    1.30371f32,
    0.10288f32,
    0.07585f32,
    0.12259f32,
    -0.30815f32,
    0.25437f32,
    -2.82096f32,
    -2.69482f32,
    0.02370f32,
    0.12500f32,
    -0.21019f32,
    -0.49220f32,
    0.03638f32,
    -0.29795f32,
    0.28645f32,
    -0.48432f32,
    -0.38584f32,
    -0.32148f32,
    -0.47197f32,
    0.32437f32,
    0.32528f32,
    -0.19437f32,
    0.30383f32,
    -0.31879f32,
    0.26359f32,
    -0.12164f32,
    -0.43647f32,
    -0.08288f32,
    -0.33438f32,
    -0.63608f32,
    -0.46647f32,
    -0.46574f32,
    0.47806f32,
    -0.49012f32,
    -1.51234f32,
    -1.13502f32,
    -1.20470f32,
    -1.02913f32,
    -1.09182f32,
    -0.93921f32,
    -1.85523f32,
    0.92532f32,
];
static mut av1_tx_type_nn_bias_8x4_hor_layer0: [core::ffi::c_float; 16] = [
    0.36631f32,
    0.02901f32,
    0.64305f32,
    1.53074f32,
    -1.40229f32,
    0.03852f32,
    -0.05043f32,
    0.89632f32,
    -1.23312f32,
    0.07036f32,
    0.17070f32,
    0.56250f32,
    -0.28958f32,
    -0.32869f32,
    -0.01704f32,
    0.68171f32,
];
static mut av1_tx_type_nn_weights_8x4_hor_layer1: [core::ffi::c_float; 64] = [
    -0.49441f32,
    -0.31960f32,
    -0.84946f32,
    -0.85800f32,
    -2.37767f32,
    0.81373f32,
    -0.73172f32,
    -0.69337f32,
    0.88807f32,
    -0.49242f32,
    -0.44717f32,
    -0.11436f32,
    0.09978f32,
    0.15393f32,
    0.17083f32,
    1.44850f32,
    -0.20582f32,
    -0.04906f32,
    0.42990f32,
    -0.61939f32,
    -1.09692f32,
    -1.14885f32,
    -1.36879f32,
    -1.30828f32,
    -0.59558f32,
    -0.30903f32,
    -0.08906f32,
    0.06953f32,
    0.15383f32,
    -0.04193f32,
    -0.54858f32,
    1.82676f32,
    -0.22411f32,
    0.05264f32,
    -0.45848f32,
    -0.72985f32,
    0.87553f32,
    0.04116f32,
    -1.29774f32,
    -2.63018f32,
    1.09089f32,
    -0.36048f32,
    -0.16725f32,
    0.11627f32,
    0.49918f32,
    0.07539f32,
    0.00763f32,
    0.73706f32,
    0.87800f32,
    0.57049f32,
    0.60969f32,
    1.02779f32,
    1.53339f32,
    -0.35915f32,
    0.06410f32,
    1.44582f32,
    0.09698f32,
    0.71888f32,
    0.60594f32,
    0.84103f32,
    -0.50440f32,
    -0.38825f32,
    0.15626f32,
    -1.10654f32,
];
static mut av1_tx_type_nn_bias_8x4_hor_layer1: [core::ffi::c_float; 4] = [
    -0.92861f32,
    -1.45151f32,
    -1.33588f32,
    -4.33853f32,
];
static mut av1_tx_type_nnconfig_8x4_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x4_ver_layer0: [core::ffi::c_float; 32] = [
    -1.10946f32,
    1.86574f32,
    -1.59343f32,
    0.27018f32,
    -1.70676f32,
    -0.73982f32,
    -0.19021f32,
    -1.94208f32,
    -2.29759f32,
    -1.44402f32,
    0.28700f32,
    -1.18340f32,
    -1.50158f32,
    -0.44175f32,
    -1.36831f32,
    1.00374f32,
    2.59312f32,
    0.50291f32,
    -0.71042f32,
    -0.12238f32,
    -0.15901f32,
    -0.22807f32,
    -0.67376f32,
    -0.30215f32,
    0.54407f32,
    -0.45538f32,
    1.18262f32,
    2.28687f32,
    1.66212f32,
    1.70826f32,
    1.55182f32,
    0.12230f32,
];
static mut av1_tx_type_nn_bias_8x4_ver_layer0: [core::ffi::c_float; 8] = [
    0.10943f32,
    2.09789f32,
    2.16578f32,
    0.15766f32,
    -0.42461f32,
    0.00000f32,
    1.22090f32,
    -1.28717f32,
];
static mut av1_tx_type_nn_weights_8x4_ver_layer1: [core::ffi::c_float; 32] = [
    1.20426f32,
    -1.23237f32,
    2.41053f32,
    -0.72488f32,
    1.25249f32,
    0.18018f32,
    -0.09586f32,
    2.17901f32,
    0.15364f32,
    1.21535f32,
    -0.38263f32,
    -0.74309f32,
    0.50551f32,
    -0.54208f32,
    0.59139f32,
    1.16095f32,
    0.55919f32,
    -0.60183f32,
    1.18949f32,
    1.60787f32,
    0.54002f32,
    -0.10712f32,
    -0.16153f32,
    0.16207f32,
    -0.32338f32,
    2.68712f32,
    -2.83483f32,
    -0.27086f32,
    -1.15005f32,
    -0.39311f32,
    1.51236f32,
    -1.68973f32,
];
static mut av1_tx_type_nn_bias_8x4_ver_layer1: [core::ffi::c_float; 4] = [
    1.81013f32,
    1.10517f32,
    2.90059f32,
    0.95391f32,
];
static mut av1_tx_type_nnconfig_8x4_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x8_hor_layer0: [core::ffi::c_float; 128] = [
    -0.85529f32,
    0.37619f32,
    0.12754f32,
    0.08622f32,
    0.45278f32,
    0.54929f32,
    1.60651f32,
    -0.62654f32,
    -0.54929f32,
    -0.10131f32,
    -0.17569f32,
    0.13948f32,
    0.31695f32,
    -0.05616f32,
    0.20483f32,
    -0.36448f32,
    2.27203f32,
    -0.33087f32,
    0.47679f32,
    0.86888f32,
    0.39370f32,
    0.46239f32,
    0.01113f32,
    1.50327f32,
    -1.48226f32,
    -1.69621f32,
    -1.49777f32,
    -1.38885f32,
    -1.37753f32,
    -1.22681f32,
    -1.70576f32,
    0.51329f32,
    -1.65662f32,
    1.74197f32,
    -0.13579f32,
    -0.13133f32,
    -0.58396f32,
    -0.55510f32,
    -1.10709f32,
    -2.34975f32,
    0.22445f32,
    -0.56491f32,
    -0.83432f32,
    0.13492f32,
    1.32147f32,
    2.85285f32,
    0.13819f32,
    0.03792f32,
    -1.30792f32,
    0.04155f32,
    -0.70644f32,
    -0.43430f32,
    -0.16212f32,
    -0.86945f32,
    -1.16976f32,
    1.68339f32,
    0.29540f32,
    0.01137f32,
    -0.25335f32,
    -0.16856f32,
    0.12028f32,
    0.05207f32,
    0.39357f32,
    -0.01545f32,
    -0.21980f32,
    -1.94091f32,
    -1.01315f32,
    -0.68270f32,
    -0.40590f32,
    -0.67111f32,
    2.08283f32,
    0.19291f32,
    -4.81426f32,
    -0.65044f32,
    -0.24598f32,
    0.06371f32,
    -0.10272f32,
    -0.14502f32,
    -0.06821f32,
    0.45202f32,
    0.21091f32,
    -0.80864f32,
    0.39255f32,
    1.79189f32,
    1.80453f32,
    1.10484f32,
    1.17608f32,
    0.96901f32,
    -0.35871f32,
    -0.94311f32,
    0.63147f32,
    2.95157f32,
    0.45917f32,
    -0.42849f32,
    -0.55643f32,
    -0.06097f32,
    3.49299f32,
    -0.50972f32,
    0.11075f32,
    -0.08405f32,
    -0.09274f32,
    -0.22694f32,
    -0.42426f32,
    0.48632f32,
    -1.61074f32,
    1.82998f32,
    0.37623f32,
    -1.20330f32,
    -0.01142f32,
    -1.33307f32,
    -0.27492f32,
    -2.23621f32,
    1.38846f32,
    1.42085f32,
    1.42568f32,
    1.36152f32,
    1.46910f32,
    1.27473f32,
    1.34752f32,
    0.12753f32,
    -1.08197f32,
    -1.08280f32,
    -0.79489f32,
    -1.12338f32,
    -1.06795f32,
    -0.87857f32,
    -0.99892f32,
    1.09823f32,
];
static mut av1_tx_type_nn_bias_8x8_hor_layer0: [core::ffi::c_float; 16] = [
    -0.49232f32,
    -0.29685f32,
    -1.44020f32,
    1.10940f32,
    1.16452f32,
    -0.34862f32,
    -0.38761f32,
    -0.36243f32,
    0.21776f32,
    0.28234f32,
    2.34269f32,
    -0.04104f32,
    -0.26319f32,
    2.65579f32,
    -1.30137f32,
    -0.01487f32,
];
static mut av1_tx_type_nn_weights_8x8_hor_layer1: [core::ffi::c_float; 64] = [
    -0.38058f32,
    -0.41295f32,
    -1.26884f32,
    -0.75560f32,
    -1.57450f32,
    0.56072f32,
    -1.42322f32,
    -0.29106f32,
    0.07228f32,
    0.04391f32,
    1.61388f32,
    -0.03055f32,
    0.81637f32,
    2.06045f32,
    0.27119f32,
    -0.48328f32,
    -0.45528f32,
    -0.60534f32,
    -1.61209f32,
    -0.78157f32,
    -1.65034f32,
    0.60958f32,
    -1.30523f32,
    0.25143f32,
    0.11398f32,
    0.37860f32,
    1.54829f32,
    0.02309f32,
    0.67288f32,
    2.11447f32,
    0.44845f32,
    -0.70406f32,
    -0.67897f32,
    -0.38759f32,
    -1.30383f32,
    -1.22646f32,
    -1.54571f32,
    0.60552f32,
    -1.52565f32,
    0.11469f32,
    0.17344f32,
    0.08622f32,
    1.57906f32,
    -0.00909f32,
    0.81634f32,
    2.04909f32,
    1.26466f32,
    -1.45741f32,
    -0.75229f32,
    0.06200f32,
    -1.05835f32,
    -0.66257f32,
    -1.73766f32,
    0.99923f32,
    -1.87082f32,
    0.14580f32,
    0.49525f32,
    0.46839f32,
    1.32203f32,
    0.33923f32,
    0.97001f32,
    2.38584f32,
    1.58811f32,
    0.06161f32,
];
static mut av1_tx_type_nn_bias_8x8_hor_layer1: [core::ffi::c_float; 4] = [
    1.70385f32,
    1.82373f32,
    1.78496f32,
    1.80826f32,
];
static mut av1_tx_type_nnconfig_8x8_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x8_ver_layer0: [core::ffi::c_float; 128] = [
    -0.67016f32,
    -1.72366f32,
    -1.86576f32,
    -1.50962f32,
    -1.70419f32,
    -1.73964f32,
    -1.84615f32,
    2.09681f32,
    -0.05081f32,
    -0.61030f32,
    2.02541f32,
    0.60222f32,
    0.99936f32,
    2.02114f32,
    -0.53893f32,
    -0.23757f32,
    0.73566f32,
    0.25443f32,
    0.00132f32,
    -0.74036f32,
    -0.75351f32,
    -0.76964f32,
    -1.71007f32,
    -0.15770f32,
    1.60982f32,
    2.17638f32,
    0.90681f32,
    0.64973f32,
    0.85914f32,
    0.58786f32,
    -1.46228f32,
    0.05187f32,
    1.18804f32,
    0.30850f32,
    0.29512f32,
    0.40526f32,
    0.37635f32,
    0.32311f32,
    0.37471f32,
    1.12346f32,
    3.41856f32,
    -0.36653f32,
    0.42537f32,
    -0.19240f32,
    0.00155f32,
    0.30826f32,
    -0.02116f32,
    -0.53435f32,
    -0.34829f32,
    -0.52466f32,
    -0.11521f32,
    -0.29163f32,
    -2.05689f32,
    -2.87372f32,
    -0.62626f32,
    0.09585f32,
    -0.75257f32,
    0.10057f32,
    1.43474f32,
    0.89450f32,
    0.75900f32,
    1.11147f32,
    1.00558f32,
    0.25886f32,
    2.22095f32,
    -0.17926f32,
    0.57161f32,
    0.39546f32,
    0.47846f32,
    0.40452f32,
    0.54298f32,
    0.45814f32,
    -3.62788f32,
    -3.02374f32,
    0.03716f32,
    -0.13937f32,
    -0.09415f32,
    -0.12463f32,
    0.05682f32,
    0.03672f32,
    1.20746f32,
    1.25003f32,
    1.27071f32,
    1.31883f32,
    1.27473f32,
    1.34943f32,
    1.23158f32,
    0.09039f32,
    0.19388f32,
    0.63420f32,
    2.79612f32,
    0.93803f32,
    -0.11323f32,
    -0.02027f32,
    0.41286f32,
    -0.05979f32,
    -3.80705f32,
    -0.52451f32,
    -0.77098f32,
    -0.68132f32,
    -0.65559f32,
    -0.60975f32,
    -1.26165f32,
    0.25582f32,
    0.05346f32,
    0.61403f32,
    0.32140f32,
    -2.39831f32,
    -1.42355f32,
    1.30541f32,
    1.02361f32,
    0.12930f32,
    -1.61469f32,
    -0.77036f32,
    -0.59144f32,
    1.27769f32,
    1.52068f32,
    0.82137f32,
    1.83159f32,
    -0.66626f32,
    -0.69806f32,
    -1.00564f32,
    -0.85995f32,
    -0.90889f32,
    -0.84412f32,
    -0.85712f32,
    -1.29848f32,
    0.39308f32,
];
static mut av1_tx_type_nn_bias_8x8_ver_layer0: [core::ffi::c_float; 16] = [
    -0.14868f32,
    -0.48343f32,
    3.94416f32,
    -0.78037f32,
    -1.33789f32,
    -0.60611f32,
    0.51793f32,
    0.44030f32,
    -0.71563f32,
    0.22561f32,
    -1.19083f32,
    -0.46149f32,
    0.83015f32,
    0.06024f32,
    1.17180f32,
    0.65122f32,
];
static mut av1_tx_type_nn_weights_8x8_ver_layer1: [core::ffi::c_float; 64] = [
    -1.42711f32,
    -0.21683f32,
    2.12061f32,
    0.20489f32,
    -0.50228f32,
    -0.24770f32,
    0.23391f32,
    1.03470f32,
    -0.44847f32,
    -0.63225f32,
    -0.21583f32,
    -0.06467f32,
    -0.21892f32,
    -0.07786f32,
    1.43322f32,
    0.00280f32,
    -1.53057f32,
    -0.18912f32,
    1.95333f32,
    0.31151f32,
    -2.07601f32,
    0.06776f32,
    0.25529f32,
    0.94800f32,
    -1.11453f32,
    -0.20594f32,
    -0.13281f32,
    0.01485f32,
    0.17650f32,
    -0.07955f32,
    1.43734f32,
    -0.23193f32,
    -2.06463f32,
    -0.21238f32,
    2.13707f32,
    0.30351f32,
    0.27594f32,
    -0.36245f32,
    0.19539f32,
    0.91045f32,
    -0.24068f32,
    -0.37616f32,
    0.88792f32,
    0.02947f32,
    -0.16903f32,
    -0.04932f32,
    1.51293f32,
    -0.95967f32,
    -1.62903f32,
    0.05326f32,
    2.30703f32,
    0.64445f32,
    -1.09464f32,
    -0.16623f32,
    1.00240f32,
    0.07548f32,
    -0.50406f32,
    0.63854f32,
    1.02340f32,
    0.49833f32,
    0.13671f32,
    0.26722f32,
    2.09516f32,
    -0.41305f32,
];
static mut av1_tx_type_nn_bias_8x8_ver_layer1: [core::ffi::c_float; 4] = [
    2.14067f32,
    2.76699f32,
    2.04233f32,
    1.34803f32,
];
static mut av1_tx_type_nnconfig_8x8_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x16_hor_layer0: [core::ffi::c_float; 128] = [
    -1.61872f32,
    -1.58520f32,
    -1.41236f32,
    -1.53255f32,
    -1.59794f32,
    -1.25769f32,
    -1.90043f32,
    0.73431f32,
    1.10135f32,
    0.47054f32,
    0.43230f32,
    -0.43009f32,
    -0.09135f32,
    -0.07289f32,
    -0.38785f32,
    1.23775f32,
    -0.35312f32,
    0.73789f32,
    0.88864f32,
    0.75957f32,
    0.62579f32,
    0.46974f32,
    0.21851f32,
    1.63821f32,
    -2.27289f32,
    -0.68522f32,
    -0.69814f32,
    -0.84368f32,
    -0.91320f32,
    -0.63055f32,
    -1.03296f32,
    0.55778f32,
    -0.00071f32,
    1.27539f32,
    1.60068f32,
    1.40975f32,
    0.97372f32,
    0.92843f32,
    1.90853f32,
    0.12626f32,
    1.71953f32,
    1.41978f32,
    -0.12234f32,
    -1.27058f32,
    0.76207f32,
    0.02495f32,
    -0.67038f32,
    -0.05255f32,
    1.72923f32,
    1.47630f32,
    1.47058f32,
    1.47614f32,
    1.49354f32,
    1.66131f32,
    1.50801f32,
    0.17145f32,
    -2.30947f32,
    -2.10850f32,
    -1.25636f32,
    -0.24900f32,
    0.72602f32,
    1.26572f32,
    0.97865f32,
    -0.65466f32,
    1.31129f32,
    0.26916f32,
    0.12139f32,
    -0.12761f32,
    -0.39143f32,
    -0.28134f32,
    0.06584f32,
    2.24418f32,
    0.22516f32,
    0.05011f32,
    -0.01671f32,
    -0.29476f32,
    -0.40326f32,
    0.21138f32,
    -0.11573f32,
    -0.31154f32,
    -0.36828f32,
    0.03694f32,
    -0.07172f32,
    -0.63419f32,
    -3.14351f32,
    -1.23125f32,
    0.65311f32,
    -0.11406f32,
    1.97287f32,
    -0.10422f32,
    0.83896f32,
    0.85033f32,
    0.49724f32,
    0.80482f32,
    0.51454f32,
    1.06447f32,
    0.76693f32,
    0.72599f32,
    -0.78573f32,
    -0.53950f32,
    0.40894f32,
    0.00086f32,
    0.10784f32,
    -0.70498f32,
    1.16395f32,
    1.14597f32,
    1.13496f32,
    1.12177f32,
    1.02100f32,
    -1.37574f32,
    -2.97144f32,
    0.33899f32,
    0.42013f32,
    0.86327f32,
    2.31983f32,
    2.04008f32,
    0.95503f32,
    0.15081f32,
    0.11530f32,
    -0.02574f32,
    -4.77119f32,
    0.13257f32,
    -0.01704f32,
    -0.23087f32,
    -0.00825f32,
    0.07029f32,
    -0.28136f32,
    0.42556f32,
];
static mut av1_tx_type_nn_bias_8x16_hor_layer0: [core::ffi::c_float; 16] = [
    0.93617f32,
    -0.24000f32,
    -1.26821f32,
    0.78780f32,
    0.13690f32,
    -0.21948f32,
    -1.45162f32,
    0.44584f32,
    -1.92582f32,
    -0.23169f32,
    0.56004f32,
    -1.19937f32,
    1.81560f32,
    -1.02643f32,
    -0.81690f32,
    0.08302f32,
];
static mut av1_tx_type_nn_weights_8x16_hor_layer1: [core::ffi::c_float; 64] = [
    0.06696f32,
    -0.11538f32,
    -1.42029f32,
    0.32965f32,
    0.81046f32,
    0.01146f32,
    1.20945f32,
    -0.16899f32,
    0.53224f32,
    -0.40232f32,
    0.01786f32,
    -0.73242f32,
    1.29750f32,
    1.95185f32,
    0.70143f32,
    1.43287f32,
    0.76220f32,
    0.79937f32,
    -1.79011f32,
    -1.15178f32,
    0.42526f32,
    -0.67519f32,
    0.77267f32,
    -0.30697f32,
    2.46004f32,
    -0.49828f32,
    0.02875f32,
    1.09972f32,
    1.47662f32,
    0.61719f32,
    0.61417f32,
    -0.12363f32,
    2.53048f32,
    0.00418f32,
    -1.38964f32,
    0.88117f32,
    0.39239f32,
    -0.19347f32,
    -2.58600f32,
    -0.33715f32,
    1.09323f32,
    -0.32127f32,
    0.02456f32,
    -0.19125f32,
    1.12728f32,
    0.66502f32,
    0.34296f32,
    1.14897f32,
    0.29967f32,
    1.19209f32,
    0.22108f32,
    -0.11975f32,
    1.49776f32,
    -1.34624f32,
    -2.58478f32,
    -1.34632f32,
    1.53207f32,
    0.45634f32,
    -1.48476f32,
    0.17489f32,
    0.71790f32,
    -2.12086f32,
    -1.21778f32,
    -1.31243f32,
];
static mut av1_tx_type_nn_bias_8x16_hor_layer1: [core::ffi::c_float; 4] = [
    0.83359f32,
    1.06875f32,
    1.77645f32,
    1.49570f32,
];
static mut av1_tx_type_nnconfig_8x16_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x16_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x16_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x16_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x16_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_8x16_ver_layer0: [core::ffi::c_float; 128] = [
    0.32858f32,
    -1.28887f32,
    0.25632f32,
    -0.05262f32,
    2.69203f32,
    -0.07004f32,
    1.37337f32,
    -0.05725f32,
    -0.05659f32,
    0.05592f32,
    0.01039f32,
    -0.29343f32,
    1.58628f32,
    -0.30003f32,
    -3.43118f32,
    0.00272f32,
    1.70928f32,
    -0.76348f32,
    0.05889f32,
    -0.03263f32,
    -0.07724f32,
    0.03523f32,
    -0.19890f32,
    1.18005f32,
    -0.03605f32,
    -0.20530f32,
    -4.00733f32,
    0.10210f32,
    -0.05368f32,
    -0.17650f32,
    -0.15317f32,
    0.06499f32,
    0.56705f32,
    1.04341f32,
    0.62890f32,
    0.73451f32,
    -0.22199f32,
    0.86659f32,
    0.78443f32,
    -0.61664f32,
    -0.50606f32,
    0.30247f32,
    0.14455f32,
    0.39276f32,
    0.49203f32,
    0.65019f32,
    0.12269f32,
    1.64080f32,
    1.68289f32,
    1.42694f32,
    1.60825f32,
    1.58501f32,
    1.47252f32,
    1.62589f32,
    1.48218f32,
    0.17726f32,
    -0.04884f32,
    0.35376f32,
    -0.04796f32,
    0.32589f32,
    0.35087f32,
    0.35258f32,
    -0.46103f32,
    -0.31176f32,
    -0.05203f32,
    0.07247f32,
    -0.26756f32,
    0.22019f32,
    0.03412f32,
    0.33773f32,
    0.29811f32,
    -0.11140f32,
    0.12831f32,
    -0.44673f32,
    -0.09858f32,
    0.07889f32,
    0.15137f32,
    0.00347f32,
    -0.23394f32,
    0.08886f32,
    -0.31201f32,
    -0.79912f32,
    -0.51092f32,
    0.14123f32,
    -1.09599f32,
    -4.26020f32,
    -0.68675f32,
    -0.02842f32,
    -1.54538f32,
    -1.28977f32,
    -1.30558f32,
    -1.21074f32,
    -1.37142f32,
    -1.14743f32,
    -1.85397f32,
    0.82985f32,
    -0.30681f32,
    0.04494f32,
    -0.24023f32,
    -4.18053f32,
    -0.16096f32,
    -0.55492f32,
    -0.27882f32,
    0.05829f32,
    -0.41224f32,
    -2.52088f32,
    -0.56162f32,
    -1.04547f32,
    -1.70685f32,
    -0.28842f32,
    -1.43673f32,
    -0.01468f32,
    -3.20585f32,
    -0.69120f32,
    -0.43931f32,
    -0.46270f32,
    -0.65885f32,
    -0.55884f32,
    -0.75138f32,
    0.36381f32,
    -5.70858f32,
    -0.14548f32,
    -0.15745f32,
    -0.11812f32,
    -0.07605f32,
    -0.07693f32,
    -0.12236f32,
    0.16075f32,
];
static mut av1_tx_type_nn_bias_8x16_ver_layer0: [core::ffi::c_float; 16] = [
    -0.35385f32,
    0.30491f32,
    -0.90011f32,
    0.42941f32,
    1.20928f32,
    -0.88331f32,
    -1.48818f32,
    -0.34785f32,
    -0.32668f32,
    -0.22695f32,
    0.89188f32,
    0.65521f32,
    0.57598f32,
    0.99819f32,
    0.75175f32,
    0.17044f32,
];
static mut av1_tx_type_nn_weights_8x16_ver_layer1: [core::ffi::c_float; 64] = [
    -0.62913f32,
    -0.34304f32,
    0.42963f32,
    -0.17440f32,
    -1.44092f32,
    0.69142f32,
    -1.36067f32,
    0.52211f32,
    0.44658f32,
    -0.26501f32,
    -0.41657f32,
    0.34428f32,
    -0.34390f32,
    -0.58567f32,
    -0.84097f32,
    -1.96311f32,
    -0.37215f32,
    -0.22250f32,
    -1.23811f32,
    -0.07247f32,
    -0.81731f32,
    0.58755f32,
    -1.30559f32,
    0.39551f32,
    0.41743f32,
    -0.09940f32,
    -0.33230f32,
    0.14458f32,
    -0.25139f32,
    -0.54517f32,
    0.13469f32,
    -0.38157f32,
    -0.39109f32,
    -0.18205f32,
    0.06834f32,
    -0.08395f32,
    -0.92187f32,
    0.56724f32,
    1.44381f32,
    0.53226f32,
    -0.22356f32,
    0.12285f32,
    -0.29418f32,
    -1.86749f32,
    -0.22372f32,
    -0.60204f32,
    -0.87746f32,
    -1.16936f32,
    0.56884f32,
    0.62641f32,
    -0.11823f32,
    1.00395f32,
    1.64794f32,
    -0.64535f32,
    2.29322f32,
    -0.23397f32,
    0.17251f32,
    -0.35927f32,
    0.65631f32,
    -0.26812f32,
    0.80128f32,
    0.85748f32,
    0.47404f32,
    2.20547f32,
];
static mut av1_tx_type_nn_bias_8x16_ver_layer1: [core::ffi::c_float; 4] = [
    -0.44080f32,
    -1.67455f32,
    -1.46332f32,
    -6.13206f32,
];
static mut av1_tx_type_nnconfig_8x16_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x16_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_8x16_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x16_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_8x16_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_16x8_hor_layer0: [core::ffi::c_float; 128] = [
    0.02600f32,
    0.09786f32,
    -1.05107f32,
    -0.35594f32,
    -0.15658f32,
    2.99828f32,
    -0.07106f32,
    -0.10101f32,
    -0.14412f32,
    -0.83790f32,
    -0.19434f32,
    2.28368f32,
    1.91727f32,
    -0.00956f32,
    -0.90640f32,
    0.09174f32,
    1.58895f32,
    1.38945f32,
    1.49431f32,
    1.51381f32,
    1.44803f32,
    1.53544f32,
    1.44694f32,
    0.17753f32,
    1.69735f32,
    -0.78652f32,
    0.31092f32,
    -0.23736f32,
    0.02231f32,
    -0.09884f32,
    -0.00493f32,
    1.21189f32,
    -1.94382f32,
    -0.34629f32,
    -0.58309f32,
    0.72291f32,
    -0.30056f32,
    0.90660f32,
    -0.57495f32,
    3.07809f32,
    0.73644f32,
    1.43050f32,
    1.34356f32,
    -0.66554f32,
    0.50102f32,
    -0.64305f32,
    0.42044f32,
    -1.66165f32,
    -0.05733f32,
    -2.51402f32,
    -1.01067f32,
    -0.33390f32,
    -0.32986f32,
    -0.92431f32,
    1.86281f32,
    -0.07290f32,
    -0.26290f32,
    -0.68941f32,
    1.81156f32,
    0.66125f32,
    -2.09974f32,
    0.17032f32,
    -0.67461f32,
    -0.00876f32,
    -1.50154f32,
    1.17153f32,
    1.00377f32,
    0.33022f32,
    0.74689f32,
    0.42878f32,
    0.61725f32,
    -0.83967f32,
    0.09467f32,
    -0.39892f32,
    0.33863f32,
    0.10656f32,
    -0.09249f32,
    -0.39757f32,
    0.48481f32,
    -0.35162f32,
    1.47014f32,
    1.67827f32,
    -1.84051f32,
    0.16291f32,
    -0.50135f32,
    -2.29911f32,
    -0.42217f32,
    -0.13358f32,
    1.45899f32,
    -0.14743f32,
    -0.02763f32,
    -0.28003f32,
    -0.01364f32,
    0.21014f32,
    -0.29026f32,
    -0.20198f32,
    1.38782f32,
    0.56731f32,
    0.27489f32,
    0.43227f32,
    0.41326f32,
    0.42721f32,
    0.87720f32,
    -1.90067f32,
    -5.04951f32,
    -0.17638f32,
    -0.58119f32,
    -0.08954f32,
    -0.13692f32,
    -0.12325f32,
    -0.38548f32,
    0.66462f32,
    -1.42377f32,
    -1.21917f32,
    -1.38193f32,
    -1.36539f32,
    -1.39378f32,
    -1.19629f32,
    -1.59812f32,
    0.28689f32,
    0.32394f32,
    0.52128f32,
    0.01013f32,
    -0.28948f32,
    -0.26293f32,
    -0.44331f32,
    -0.36570f32,
    -0.50757f32,
];
static mut av1_tx_type_nn_bias_16x8_hor_layer0: [core::ffi::c_float; 16] = [
    -0.08696f32,
    -0.22110f32,
    -1.43604f32,
    -1.00451f32,
    -1.51029f32,
    0.63736f32,
    0.45260f32,
    0.16229f32,
    4.01393f32,
    -0.21748f32,
    0.36411f32,
    -0.08764f32,
    -0.12329f32,
    0.08986f32,
    1.08117f32,
    -0.00220f32,
];
static mut av1_tx_type_nn_weights_16x8_hor_layer1: [core::ffi::c_float; 64] = [
    0.55824f32,
    -0.14648f32,
    0.81947f32,
    -0.45867f32,
    -1.86078f32,
    -0.17291f32,
    0.34849f32,
    0.15153f32,
    1.75625f32,
    -0.25760f32,
    0.72015f32,
    -0.30059f32,
    -0.57975f32,
    0.07609f32,
    -0.02036f32,
    0.07912f32,
    0.57080f32,
    -0.13792f32,
    0.74184f32,
    -0.87669f32,
    -1.87572f32,
    -0.27270f32,
    0.39751f32,
    0.19652f32,
    2.03514f32,
    -0.32944f32,
    0.76251f32,
    0.04399f32,
    -0.63175f32,
    0.37420f32,
    0.08309f32,
    0.04466f32,
    0.60255f32,
    -0.12820f32,
    1.66065f32,
    -0.59496f32,
    -1.94794f32,
    -0.14847f32,
    0.39424f32,
    0.16273f32,
    1.80587f32,
    0.41197f32,
    0.74691f32,
    -0.21217f32,
    -0.63173f32,
    0.09510f32,
    -0.35538f32,
    -0.04407f32,
    0.92847f32,
    0.20141f32,
    1.68680f32,
    -0.56528f32,
    -2.26960f32,
    0.12978f32,
    0.73748f32,
    0.42438f32,
    2.00673f32,
    -0.40189f32,
    0.95423f32,
    0.23234f32,
    -0.80953f32,
    0.65814f32,
    0.49444f32,
    -0.23347f32,
];
static mut av1_tx_type_nn_bias_16x8_hor_layer1: [core::ffi::c_float; 4] = [
    3.57175f32,
    2.42612f32,
    3.31259f32,
    2.08287f32,
];
static mut av1_tx_type_nnconfig_16x8_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x8_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x8_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_16x8_ver_layer0: [core::ffi::c_float; 128] = [
    0.46633f32,
    1.55328f32,
    -0.11230f32,
    -0.29571f32,
    0.18814f32,
    -1.52430f32,
    -2.34660f32,
    0.08644f32,
    -1.97718f32,
    -1.29140f32,
    -1.12262f32,
    -1.12985f32,
    -1.25911f32,
    -0.96506f32,
    -1.57129f32,
    0.96021f32,
    1.34192f32,
    1.28623f32,
    1.21655f32,
    1.28758f32,
    1.25482f32,
    1.30195f32,
    1.19190f32,
    0.09310f32,
    0.52072f32,
    0.91487f32,
    1.24100f32,
    1.61236f32,
    1.72166f32,
    2.20750f32,
    1.62379f32,
    -1.43936f32,
    0.50665f32,
    0.40213f32,
    0.66502f32,
    -1.66699f32,
    -3.07618f32,
    0.05877f32,
    0.60987f32,
    -0.09995f32,
    -0.10916f32,
    0.48049f32,
    0.23812f32,
    0.39847f32,
    -0.21682f32,
    -0.63455f32,
    0.33453f32,
    -0.67939f32,
    -4.14355f32,
    -0.62756f32,
    -0.22502f32,
    -0.17215f32,
    0.01062f32,
    0.27049f32,
    -0.10748f32,
    0.30945f32,
    2.72445f32,
    -0.89181f32,
    -0.06800f32,
    0.20595f32,
    -0.73385f32,
    0.04071f32,
    -1.30294f32,
    1.83507f32,
    0.92570f32,
    0.69609f32,
    0.76285f32,
    0.69892f32,
    0.76409f32,
    0.63104f32,
    0.73397f32,
    1.09575f32,
    -0.20129f32,
    -0.24022f32,
    -0.24599f32,
    -0.59107f32,
    -0.88755f32,
    -0.68987f32,
    -0.75495f32,
    -1.31002f32,
    -1.30237f32,
    -0.94093f32,
    -2.15678f32,
    -1.49303f32,
    -1.17498f32,
    -1.39952f32,
    -0.91270f32,
    -0.05587f32,
    1.02381f32,
    -0.75580f32,
    -0.65263f32,
    -0.78996f32,
    -0.71075f32,
    -0.71018f32,
    -0.70350f32,
    -1.26196f32,
    2.34208f32,
    -0.53611f32,
    0.19752f32,
    -0.16842f32,
    -0.24828f32,
    0.21857f32,
    0.08222f32,
    -2.55894f32,
    -1.75702f32,
    0.11394f32,
    1.03083f32,
    0.79972f32,
    -1.54112f32,
    -1.82341f32,
    -0.57597f32,
    -0.02077f32,
    -0.39616f32,
    -0.00995f32,
    -0.12809f32,
    0.01188f32,
    -0.25117f32,
    0.09202f32,
    0.09336f32,
    -0.05614f32,
    -0.30039f32,
    0.25834f32,
    1.19944f32,
    1.22533f32,
    0.92330f32,
    0.75967f32,
    -0.81945f32,
    -0.41647f32,
];
static mut av1_tx_type_nn_bias_16x8_ver_layer0: [core::ffi::c_float; 16] = [
    0.17841f32,
    0.67315f32,
    -1.24450f32,
    3.13859f32,
    0.16203f32,
    -0.14992f32,
    0.29553f32,
    -1.15567f32,
    -0.71421f32,
    1.15977f32,
    1.14585f32,
    3.02460f32,
    -0.04510f32,
    0.48000f32,
    -0.09354f32,
    -0.42422f32,
];
static mut av1_tx_type_nn_weights_16x8_ver_layer1: [core::ffi::c_float; 64] = [
    0.29912f32,
    -0.10009f32,
    -1.11478f32,
    1.76812f32,
    -0.27719f32,
    0.52148f32,
    0.17622f32,
    -1.17116f32,
    0.73397f32,
    -0.69279f32,
    -0.11080f32,
    1.53751f32,
    -1.42003f32,
    0.14731f32,
    0.13592f32,
    -0.04883f32,
    0.39186f32,
    -0.13655f32,
    -0.43994f32,
    1.82759f32,
    -0.25601f32,
    -0.15018f32,
    0.51920f32,
    -1.56070f32,
    0.31683f32,
    -0.79367f32,
    -0.02904f32,
    1.28637f32,
    -1.15203f32,
    0.26627f32,
    0.42828f32,
    -0.24258f32,
    0.38647f32,
    -0.83352f32,
    0.32553f32,
    2.09522f32,
    -0.26822f32,
    -0.42191f32,
    0.32825f32,
    -1.30748f32,
    1.50551f32,
    -0.52669f32,
    0.20045f32,
    1.69318f32,
    -1.47839f32,
    0.30802f32,
    -0.07290f32,
    -0.28106f32,
    0.68192f32,
    -0.15522f32,
    1.12579f32,
    2.21921f32,
    0.09720f32,
    -0.50265f32,
    0.83165f32,
    -1.31721f32,
    0.72422f32,
    -1.24952f32,
    0.61653f32,
    2.04117f32,
    -1.42406f32,
    0.52568f32,
    -0.46180f32,
    -0.00873f32,
];
static mut av1_tx_type_nn_bias_16x8_ver_layer1: [core::ffi::c_float; 4] = [
    3.34981f32,
    3.74710f32,
    1.38339f32,
    0.45176f32,
];
static mut av1_tx_type_nnconfig_16x8_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x8_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x8_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_16x16_layer0: [core::ffi::c_float; 128] = [
    1.26592f32,
    1.36313f32,
    1.30956f32,
    1.29926f32,
    1.48816f32,
    1.68851f32,
    1.32000f32,
    0.13321f32,
    -0.22477f32,
    -0.88906f32,
    -0.19622f32,
    1.69605f32,
    1.22180f32,
    -1.57771f32,
    -1.15765f32,
    0.05710f32,
    -1.13355f32,
    -0.85486f32,
    -0.99971f32,
    -0.91571f32,
    -1.06031f32,
    -0.77952f32,
    -1.15723f32,
    1.17809f32,
    1.35602f32,
    -0.05243f32,
    -0.37596f32,
    0.26108f32,
    0.17611f32,
    -0.10323f32,
    0.77279f32,
    -0.48911f32,
    -0.79308f32,
    0.55112f32,
    0.43918f32,
    0.27872f32,
    0.28714f32,
    0.45830f32,
    1.05689f32,
    0.03705f32,
    -2.49975f32,
    -0.01940f32,
    0.05709f32,
    0.07942f32,
    -0.13290f32,
    -0.10359f32,
    0.00143f32,
    0.37303f32,
    0.96470f32,
    0.53293f32,
    1.14459f32,
    0.89185f32,
    0.43378f32,
    0.47764f32,
    0.90924f32,
    0.15279f32,
    -0.15361f32,
    0.02949f32,
    0.42240f32,
    0.68143f32,
    0.89588f32,
    0.73754f32,
    0.10974f32,
    1.57755f32,
    -0.39870f32,
    -0.32914f32,
    0.35638f32,
    0.34991f32,
    -0.00003f32,
    -0.23373f32,
    0.29630f32,
    -0.76699f32,
    -0.01356f32,
    0.04234f32,
    0.84253f32,
    1.92078f32,
    0.93160f32,
    0.71993f32,
    0.71604f32,
    0.76455f32,
    -1.59782f32,
    0.32332f32,
    1.11628f32,
    0.33062f32,
    -0.03728f32,
    -0.05710f32,
    0.80447f32,
    -0.14719f32,
    1.34658f32,
    -0.05718f32,
    0.64015f32,
    0.21926f32,
    0.41653f32,
    0.12720f32,
    0.54092f32,
    1.39411f32,
    1.81819f32,
    -0.24513f32,
    0.00955f32,
    0.38011f32,
    -0.57787f32,
    -0.41759f32,
    0.68834f32,
    -0.31783f32,
    -0.40607f32,
    -0.10107f32,
    -0.79374f32,
    0.75599f32,
    -0.16282f32,
    -0.14490f32,
    -0.20783f32,
    -0.55019f32,
    -0.13793f32,
    -0.22293f32,
    0.18305f32,
    0.12445f32,
    0.56830f32,
    0.24567f32,
    0.09278f32,
    0.70803f32,
    0.35803f32,
    -1.52676f32,
    -0.89624f32,
    0.77665f32,
    0.19877f32,
    0.77175f32,
    0.50355f32,
    0.08592f32,
];
static mut av1_tx_type_nn_bias_16x16_layer0: [core::ffi::c_float; 16] = [
    -1.31834f32,
    0.14346f32,
    -0.10062f32,
    0.84489f32,
    0.95617f32,
    -0.06720f32,
    -0.68502f32,
    -0.91442f32,
    -0.31932f32,
    0.25276f32,
    -0.15138f32,
    -1.57661f32,
    -0.14062f32,
    -0.42120f32,
    0.94573f32,
    -0.09287f32,
];
static mut av1_tx_type_nn_weights_16x16_layer1: [core::ffi::c_float; 64] = [
    -1.80333f32,
    -1.06353f32,
    0.55139f32,
    0.74644f32,
    0.13747f32,
    -0.93018f32,
    -0.10286f32,
    0.67133f32,
    0.24460f32,
    1.44583f32,
    0.02173f32,
    0.26037f32,
    -0.73687f32,
    0.19566f32,
    0.61846f32,
    -0.58601f32,
    -1.03196f32,
    -0.74415f32,
    0.30041f32,
    -0.41967f32,
    1.08740f32,
    0.96224f32,
    -0.59139f32,
    0.03813f32,
    0.05403f32,
    1.33427f32,
    -0.54375f32,
    -1.92181f32,
    0.54704f32,
    0.13608f32,
    0.22151f32,
    -0.38076f32,
    1.18390f32,
    -0.77508f32,
    -1.84283f32,
    1.00894f32,
    0.62318f32,
    -0.15296f32,
    1.27600f32,
    0.22822f32,
    0.12751f32,
    0.93910f32,
    -0.28502f32,
    0.53912f32,
    -0.96889f32,
    0.10182f32,
    0.81508f32,
    -0.43028f32,
    2.67386f32,
    0.52204f32,
    0.49820f32,
    -0.41711f32,
    1.05038f32,
    1.12192f32,
    0.74349f32,
    -0.75417f32,
    -0.03718f32,
    -0.35769f32,
    0.89651f32,
    0.63236f32,
    0.54215f32,
    -0.07894f32,
    0.48274f32,
    1.08829f32,
];
static mut av1_tx_type_nn_bias_16x16_layer1: [core::ffi::c_float; 4] = [
    0.81986f32,
    1.26865f32,
    0.11118f32,
    2.48404f32,
];
static mut av1_tx_type_nnconfig_16x16: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_4x16_hor_layer0: [core::ffi::c_float; 32] = [
    0.36539f32,
    0.25667f32,
    0.01491f32,
    -0.21959f32,
    2.55105f32,
    0.17615f32,
    1.79884f32,
    1.65936f32,
    -0.44363f32,
    0.00706f32,
    -0.68004f32,
    -0.64360f32,
    1.75760f32,
    1.91906f32,
    1.47682f32,
    0.09650f32,
    -3.59244f32,
    -0.35004f32,
    0.93295f32,
    0.25806f32,
    -0.08154f32,
    0.79332f32,
    0.79535f32,
    1.09467f32,
    1.57855f32,
    -0.51359f32,
    0.90553f32,
    -1.67744f32,
    -1.74563f32,
    -0.88830f32,
    -1.77603f32,
    2.15935f32,
];
static mut av1_tx_type_nn_bias_4x16_hor_layer0: [core::ffi::c_float; 8] = [
    -0.36435f32,
    -2.22731f32,
    -0.00837f32,
    -1.34546f32,
    0.62806f32,
    -0.20675f32,
    4.91940f32,
    -0.56079f32,
];
static mut av1_tx_type_nn_weights_4x16_hor_layer1: [core::ffi::c_float; 32] = [
    -0.57191f32,
    -1.46418f32,
    0.67331f32,
    -1.15027f32,
    0.46288f32,
    0.81251f32,
    2.51768f32,
    -0.27147f32,
    0.00761f32,
    -2.15214f32,
    -0.69650f32,
    -0.50808f32,
    0.92832f32,
    0.45668f32,
    2.34201f32,
    -0.52941f32,
    0.51008f32,
    -1.55496f32,
    -0.01371f32,
    -0.12356f32,
    0.66624f32,
    0.88043f32,
    2.64862f32,
    -1.28024f32,
    -0.17578f32,
    -1.80034f32,
    -0.32217f32,
    0.89519f32,
    1.28413f32,
    -0.30326f32,
    2.45329f32,
    -0.83335f32,
];
static mut av1_tx_type_nn_bias_4x16_hor_layer1: [core::ffi::c_float; 4] = [
    2.33198f32,
    3.36245f32,
    1.62603f32,
    2.91056f32,
];
static mut av1_tx_type_nnconfig_4x16_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x16_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x16_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x16_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x16_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_4x16_ver_layer0: [core::ffi::c_float; 128] = [
    1.61392f32,
    1.41239f32,
    1.47646f32,
    1.47325f32,
    1.46110f32,
    1.49208f32,
    1.49414f32,
    0.12835f32,
    -0.76986f32,
    0.07087f32,
    -0.24572f32,
    -0.93168f32,
    3.07935f32,
    -0.18183f32,
    -0.09831f32,
    -0.07703f32,
    -0.03222f32,
    -0.25473f32,
    -0.06090f32,
    2.93713f32,
    -0.38711f32,
    -0.12884f32,
    -0.18329f32,
    -0.06262f32,
    -0.00327f32,
    -0.02930f32,
    -0.01641f32,
    -0.00622f32,
    -0.03305f32,
    -4.07069f32,
    -2.76643f32,
    0.04413f32,
    -1.03176f32,
    -0.19217f32,
    -0.44980f32,
    -2.48615f32,
    -2.58112f32,
    -0.87695f32,
    0.16187f32,
    -0.04891f32,
    -0.06854f32,
    1.08104f32,
    0.75245f32,
    1.49302f32,
    0.63363f32,
    1.45715f32,
    0.92574f32,
    1.72029f32,
    0.33326f32,
    3.86646f32,
    0.04422f32,
    0.41019f32,
    0.36212f32,
    0.56600f32,
    -1.01552f32,
    0.05128f32,
    0.40454f32,
    -1.05100f32,
    -0.47461f32,
    -1.33168f32,
    -0.46145f32,
    -1.36870f32,
    -0.88838f32,
    -1.05358f32,
    -0.18537f32,
    -0.34357f32,
    -0.03698f32,
    0.68905f32,
    0.41010f32,
    0.31223f32,
    -0.43382f32,
    -0.74715f32,
    2.03366f32,
    -0.30419f32,
    0.45747f32,
    0.09526f32,
    0.31678f32,
    0.22915f32,
    0.21832f32,
    1.26385f32,
    -0.06814f32,
    -0.71417f32,
    -1.18947f32,
    0.03762f32,
    0.10936f32,
    2.97396f32,
    -0.42638f32,
    -0.03123f32,
    -5.49756f32,
    -0.17029f32,
    -0.11323f32,
    0.05173f32,
    -0.44274f32,
    -0.15738f32,
    0.11311f32,
    0.43872f32,
    0.16837f32,
    -0.52849f32,
    2.90050f32,
    -0.54735f32,
    -0.29591f32,
    1.24030f32,
    0.21696f32,
    -0.04443f32,
    -1.60877f32,
    -1.36365f32,
    -1.27432f32,
    -1.52060f32,
    -1.34397f32,
    -1.13371f32,
    -1.87554f32,
    0.80123f32,
    0.42820f32,
    -0.14157f32,
    -2.73963f32,
    -0.68040f32,
    -0.35236f32,
    0.14490f32,
    2.23477f32,
    0.01370f32,
    -0.20426f32,
    -1.51411f32,
    -0.72293f32,
    0.64516f32,
    0.97638f32,
    0.32616f32,
    -0.27975f32,
    -0.01149f32,
];
static mut av1_tx_type_nn_bias_4x16_ver_layer0: [core::ffi::c_float; 16] = [
    -1.37863f32,
    -0.05763f32,
    -0.07041f32,
    0.15306f32,
    0.96026f32,
    -1.42105f32,
    -0.55822f32,
    1.04845f32,
    -0.17662f32,
    -1.25345f32,
    -0.11927f32,
    0.49845f32,
    -0.32530f32,
    0.73483f32,
    0.08322f32,
    -0.23890f32,
];
static mut av1_tx_type_nn_weights_4x16_ver_layer1: [core::ffi::c_float; 64] = [
    0.27194f32,
    0.50607f32,
    0.49229f32,
    -0.48192f32,
    0.15667f32,
    -1.38891f32,
    0.38102f32,
    -0.58825f32,
    -0.07337f32,
    -0.52909f32,
    0.36975f32,
    0.28710f32,
    0.34992f32,
    -0.73630f32,
    0.30386f32,
    -0.58822f32,
    0.36127f32,
    0.57950f32,
    0.55878f32,
    -0.42796f32,
    0.19967f32,
    -1.45517f32,
    0.42529f32,
    -0.54630f32,
    -0.38169f32,
    -0.84899f32,
    0.41622f32,
    0.46935f32,
    0.39077f32,
    -0.75448f32,
    0.31698f32,
    -0.76187f32,
    0.97765f32,
    0.57052f32,
    0.55825f32,
    -0.54273f32,
    0.20466f32,
    -1.46347f32,
    0.41813f32,
    -0.55019f32,
    -0.19948f32,
    -0.57982f32,
    0.41206f32,
    0.32373f32,
    0.38537f32,
    -1.11657f32,
    0.32887f32,
    -0.76911f32,
    1.12259f32,
    0.72163f32,
    0.82603f32,
    0.37786f32,
    0.34976f32,
    -1.86642f32,
    0.59961f32,
    -0.16329f32,
    -0.36631f32,
    -0.56814f32,
    0.60410f32,
    0.53158f32,
    0.56389f32,
    -0.70508f32,
    0.51009f32,
    -0.56513f32,
];
static mut av1_tx_type_nn_bias_4x16_ver_layer1: [core::ffi::c_float; 4] = [
    4.60896f32,
    4.53551f32,
    4.53124f32,
    4.27435f32,
];
static mut av1_tx_type_nnconfig_4x16_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x16_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_4x16_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x16_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_4x16_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_16x4_hor_layer0: [core::ffi::c_float; 128] = [
    1.45347f32,
    -0.15743f32,
    0.44236f32,
    0.25808f32,
    0.33944f32,
    0.38678f32,
    0.24428f32,
    1.67287f32,
    0.09539f32,
    -0.42940f32,
    -0.31507f32,
    -0.00154f32,
    -2.98755f32,
    -2.27744f32,
    -0.49183f32,
    0.09333f32,
    -0.99026f32,
    -0.22157f32,
    0.53701f32,
    0.60447f32,
    0.15686f32,
    -0.04646f32,
    0.26341f32,
    2.12361f32,
    0.27090f32,
    -1.14716f32,
    -0.64146f32,
    -0.91604f32,
    -0.75335f32,
    -0.60056f32,
    -1.25084f32,
    1.68473f32,
    -3.24075f32,
    -4.03867f32,
    -2.07877f32,
    -0.02347f32,
    0.00333f32,
    -0.01259f32,
    -0.00465f32,
    0.02526f32,
    0.36286f32,
    -0.10324f32,
    2.12780f32,
    -0.74584f32,
    -1.05052f32,
    1.78467f32,
    -0.55065f32,
    -0.03326f32,
    2.46781f32,
    1.18349f32,
    0.96015f32,
    1.01696f32,
    1.10584f32,
    1.07263f32,
    1.11531f32,
    -1.06413f32,
    0.32389f32,
    -1.87360f32,
    -0.14435f32,
    1.77926f32,
    1.09966f32,
    -0.12680f32,
    -0.61386f32,
    -0.09724f32,
    -0.33095f32,
    1.12122f32,
    1.00791f32,
    1.52416f32,
    1.35004f32,
    1.32657f32,
    0.60950f32,
    -1.13538f32,
    -0.38654f32,
    0.06473f32,
    2.10669f32,
    0.27734f32,
    -0.38359f32,
    -1.91455f32,
    -1.22676f32,
    0.05786f32,
    0.97432f32,
    2.19967f32,
    0.50457f32,
    0.78976f32,
    0.95183f32,
    -0.32414f32,
    0.49437f32,
    -0.04506f32,
    0.18993f32,
    -0.07971f32,
    0.23889f32,
    -0.09872f32,
    -0.66036f32,
    0.05377f32,
    2.69638f32,
    -0.08259f32,
    -0.69210f32,
    -1.08296f32,
    -1.96504f32,
    -2.31947f32,
    -0.80161f32,
    -0.80456f32,
    -1.35556f32,
    -0.05323f32,
    -4.42658f32,
    -0.30732f32,
    -0.12043f32,
    0.11126f32,
    0.10771f32,
    -0.14956f32,
    -0.02218f32,
    0.41016f32,
    1.16599f32,
    1.14629f32,
    1.12881f32,
    1.18676f32,
    1.24677f32,
    1.28695f32,
    1.11270f32,
    0.08233f32,
    1.75440f32,
    0.49228f32,
    -0.34858f32,
    -0.17032f32,
    0.29288f32,
    0.47175f32,
    0.19055f32,
    -1.56413f32,
];
static mut av1_tx_type_nn_bias_16x4_hor_layer0: [core::ffi::c_float; 16] = [
    -1.71227f32,
    0.47291f32,
    -0.97536f32,
    -0.66216f32,
    0.11729f32,
    -0.21451f32,
    2.75281f32,
    0.04318f32,
    2.03965f32,
    0.14618f32,
    -0.70483f32,
    -0.24517f32,
    1.14048f32,
    0.33308f32,
    -1.10886f32,
    0.41184f32,
];
static mut av1_tx_type_nn_weights_16x4_hor_layer1: [core::ffi::c_float; 64] = [
    -1.17079f32,
    0.19096f32,
    -1.05753f32,
    -0.30803f32,
    -1.21680f32,
    -0.67255f32,
    1.60115f32,
    0.05972f32,
    1.44759f32,
    -0.04068f32,
    -0.26331f32,
    0.31400f32,
    0.96923f32,
    0.33443f32,
    -0.77215f32,
    -0.91316f32,
    -1.78928f32,
    0.21483f32,
    -1.24008f32,
    -0.46190f32,
    -0.12127f32,
    -0.62144f32,
    1.37593f32,
    0.08373f32,
    1.56215f32,
    0.00279f32,
    -0.14556f32,
    0.38710f32,
    0.96228f32,
    0.66433f32,
    -0.51798f32,
    -0.80738f32,
    -0.18539f32,
    0.19377f32,
    -1.03090f32,
    -1.51044f32,
    -0.59485f32,
    -0.62589f32,
    1.90742f32,
    0.09078f32,
    1.49113f32,
    0.00205f32,
    -0.15918f32,
    0.40827f32,
    1.08553f32,
    0.43431f32,
    0.33519f32,
    -1.12669f32,
    -1.10274f32,
    0.80004f32,
    -1.83599f32,
    -0.53134f32,
    2.00515f32,
    -0.32670f32,
    1.37124f32,
    0.51136f32,
    1.62563f32,
    0.24787f32,
    0.31757f32,
    0.81751f32,
    1.57262f32,
    0.83214f32,
    1.04661f32,
    -0.43819f32,
];
static mut av1_tx_type_nn_bias_16x4_hor_layer1: [core::ffi::c_float; 4] = [
    2.32575f32,
    2.75703f32,
    1.12304f32,
    2.15567f32,
];
static mut av1_tx_type_nnconfig_16x4_hor: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x4_hor_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x4_hor_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nn_weights_16x4_ver_layer0: [core::ffi::c_float; 32] = [
    0.26047f32,
    0.99930f32,
    1.16484f32,
    -0.28196f32,
    -2.67483f32,
    -0.21456f32,
    -0.16854f32,
    0.46375f32,
    1.47951f32,
    1.13735f32,
    1.12356f32,
    0.27385f32,
    0.50978f32,
    2.09967f32,
    -1.47386f32,
    0.01950f32,
    -0.06362f32,
    0.26014f32,
    1.04544f32,
    -0.03099f32,
    0.07478f32,
    -0.39701f32,
    0.05545f32,
    2.73633f32,
    -0.56305f32,
    -0.02208f32,
    -0.44517f32,
    -0.00897f32,
    -0.17967f32,
    -0.96622f32,
    0.42635f32,
    -1.04784f32,
];
static mut av1_tx_type_nn_bias_16x4_ver_layer0: [core::ffi::c_float; 8] = [
    -0.52088f32,
    0.52844f32,
    -1.03655f32,
    -0.30974f32,
    2.59952f32,
    -1.93604f32,
    0.00000f32,
    2.51787f32,
];
static mut av1_tx_type_nn_weights_16x4_ver_layer1: [core::ffi::c_float; 32] = [
    0.10916f32,
    -0.21219f32,
    -0.51340f32,
    0.69161f32,
    1.45988f32,
    -1.36942f32,
    -0.40899f32,
    1.05136f32,
    -0.08486f32,
    0.10008f32,
    -0.55304f32,
    0.88012f32,
    1.61177f32,
    -1.64507f32,
    0.63428f32,
    1.15130f32,
    -0.17287f32,
    -0.18592f32,
    -0.01143f32,
    0.88293f32,
    1.73326f32,
    -1.63624f32,
    0.09359f32,
    1.18393f32,
    0.26531f32,
    0.22378f32,
    0.15170f32,
    1.06965f32,
    1.26814f32,
    -1.93873f32,
    -0.00768f32,
    1.58309f32,
];
static mut av1_tx_type_nn_bias_16x4_ver_layer1: [core::ffi::c_float; 4] = [
    2.34713f32,
    1.68667f32,
    1.25488f32,
    1.69812f32,
];
static mut av1_tx_type_nnconfig_16x4_ver: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 4 as core::ffi::c_int,
            num_outputs: 4 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [8 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_weights_16x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x4_ver_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_type_nn_bias_16x4_ver_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_type_nnconfig_map_hor: [*const NN_CONFIG; 19] = unsafe {
    [
        &raw const av1_tx_type_nnconfig_4x4_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x8_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x16 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_4x8_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x4_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x16_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x8_hor as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_4x16_hor as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x4_hor as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
    ]
};
static mut av1_tx_type_nnconfig_map_ver: [*const NN_CONFIG; 19] = unsafe {
    [
        &raw const av1_tx_type_nnconfig_4x4_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x8_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x16 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_4x8_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x4_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_8x16_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x8_ver as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_4x16_ver as *const NN_CONFIG,
        &raw const av1_tx_type_nnconfig_16x4_ver as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
        0 as *const NN_CONFIG,
    ]
};
static mut av1_tx_split_nn_weights_4x8_layer0: [core::ffi::c_float; 128] = [
    0.068650f32,
    -0.732073f32,
    -0.040361f32,
    0.322550f32,
    -0.021123f32,
    0.212518f32,
    -0.350546f32,
    0.435987f32,
    -0.111756f32,
    -0.401568f32,
    0.069548f32,
    -0.313000f32,
    0.073918f32,
    -0.373805f32,
    -0.775810f32,
    -0.124753f32,
    0.181094f32,
    -0.602641f32,
    -0.026219f32,
    -0.350112f32,
    0.020599f32,
    -0.311752f32,
    -0.476482f32,
    -0.669465f32,
    -0.310921f32,
    0.348869f32,
    -0.115984f32,
    0.154250f32,
    0.200485f32,
    -0.016689f32,
    0.020392f32,
    0.413810f32,
    0.634064f32,
    -0.627530f32,
    0.399178f32,
    -0.012284f32,
    0.472030f32,
    0.091087f32,
    -0.706100f32,
    -0.447944f32,
    -0.274226f32,
    0.445656f32,
    0.309339f32,
    0.505522f32,
    0.038496f32,
    -0.152809f32,
    0.408684f32,
    -0.068151f32,
    0.271612f32,
    0.353233f32,
    -0.150365f32,
    0.075212f32,
    -0.035096f32,
    0.346615f32,
    0.124382f32,
    0.477072f32,
    0.216288f32,
    0.070548f32,
    -0.106362f32,
    0.681613f32,
    -0.145502f32,
    -0.218631f32,
    -0.099248f32,
    -0.001983f32,
    -0.196819f32,
    -0.969045f32,
    0.063009f32,
    -0.123053f32,
    0.104875f32,
    -0.137581f32,
    -0.282933f32,
    -0.003624f32,
    -0.315659f32,
    -0.333523f32,
    -0.503000f32,
    -0.100063f32,
    -0.536711f32,
    -0.059978f32,
    -0.670248f32,
    -0.353762f32,
    0.181109f32,
    0.289715f32,
    -0.071206f32,
    0.261141f32,
    0.052796f32,
    -0.114554f32,
    -0.139214f32,
    -0.261380f32,
    0.075984f32,
    -0.647925f32,
    -0.099528f32,
    -0.677814f32,
    0.015712f32,
    -0.389385f32,
    -0.095622f32,
    -0.165117f32,
    -0.109454f32,
    -0.175240f32,
    -0.393914f32,
    0.212330f32,
    0.037822f32,
    0.248280f32,
    0.180197f32,
    0.110493f32,
    -0.525727f32,
    -0.092329f32,
    -0.524029f32,
    -0.407364f32,
    -0.542373f32,
    -0.435626f32,
    -0.912194f32,
    0.062794f32,
    0.160433f32,
    0.741485f32,
    -0.103659f32,
    -0.119327f32,
    -0.055275f32,
    0.334358f32,
    0.014713f32,
    0.046327f32,
    0.831114f32,
    -0.576682f32,
    0.354369f32,
    -0.082088f32,
    0.452331f32,
    0.039730f32,
    -0.792429f32,
    -0.385862f32,
];
static mut av1_tx_split_nn_bias_4x8_layer0: [core::ffi::c_float; 16] = [
    0.238621f32,
    2.186830f32,
    1.383035f32,
    -0.867139f32,
    1.257119f32,
    -0.351571f32,
    -0.240650f32,
    -0.971692f32,
    2.744843f32,
    1.116991f32,
    0.139062f32,
    -0.165332f32,
    0.262171f32,
    -1.598153f32,
    -1.427340f32,
    -1.602306f32,
];
static mut av1_tx_split_nn_weights_4x8_layer1: [core::ffi::c_float; 16] = [
    -0.367134f32,
    1.373058f32,
    -0.897039f32,
    -0.326819f32,
    -0.734030f32,
    -0.290413f32,
    -0.501249f32,
    0.505321f32,
    -0.537692f32,
    -0.767893f32,
    0.268697f32,
    0.278987f32,
    0.085082f32,
    0.614986f32,
    0.847904f32,
    0.637578f32,
];
static mut av1_tx_split_nn_bias_4x8_layer1: [core::ffi::c_float; 1] = [0.20586078f32];
static mut av1_tx_split_nnconfig_4x8: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_4x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_4x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_4x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_4x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_8x8_layer0: [core::ffi::c_float; 144] = [
    0.177983f32,
    -0.938386f32,
    -0.074460f32,
    -0.221843f32,
    -0.073182f32,
    -0.295155f32,
    -0.098202f32,
    -0.279510f32,
    0.001054f32,
    -0.119319f32,
    -1.835282f32,
    -0.581507f32,
    -1.222222f32,
    -1.049006f32,
    -0.807508f32,
    -0.454252f32,
    -0.774879f32,
    -0.180607f32,
    -0.886976f32,
    -0.231971f32,
    -0.824677f32,
    -0.351872f32,
    -1.323819f32,
    0.235378f32,
    0.015331f32,
    -0.341818f32,
    0.145549f32,
    -0.348362f32,
    0.147647f32,
    -0.323400f32,
    0.047558f32,
    -0.553025f32,
    -0.295485f32,
    -0.330368f32,
    -0.530605f32,
    -0.407516f32,
    0.447740f32,
    0.782381f32,
    -0.179164f32,
    -0.584675f32,
    -0.052645f32,
    0.038656f32,
    -0.096783f32,
    0.038342f32,
    -0.170762f32,
    -0.405844f32,
    -0.552665f32,
    -0.509866f32,
    0.757204f32,
    -1.296465f32,
    0.631015f32,
    0.009265f32,
    0.646192f32,
    0.044523f32,
    0.653161f32,
    0.033820f32,
    0.849639f32,
    -0.068555f32,
    -1.036085f32,
    -0.511652f32,
    0.104693f32,
    -1.458690f32,
    0.286051f32,
    -0.089800f32,
    0.381564f32,
    -0.302640f32,
    0.304465f32,
    -0.268706f32,
    0.432603f32,
    -0.117914f32,
    -2.070031f32,
    -0.565696f32,
    -0.073027f32,
    -1.783570f32,
    -0.318144f32,
    -0.320990f32,
    -0.343966f32,
    -0.140996f32,
    -0.322977f32,
    -0.232147f32,
    -0.373210f32,
    -0.158266f32,
    -1.922305f32,
    -0.634373f32,
    0.101894f32,
    -0.221847f32,
    0.018412f32,
    -0.423887f32,
    -0.266684f32,
    -0.444930f32,
    -0.196237f32,
    0.106638f32,
    -0.065834f32,
    -0.538401f32,
    -0.280772f32,
    -0.620348f32,
    1.089957f32,
    -0.799928f32,
    0.504112f32,
    -0.165763f32,
    0.578741f32,
    -0.172653f32,
    0.547316f32,
    -0.143484f32,
    0.717220f32,
    -0.297190f32,
    -1.237854f32,
    -0.074819f32,
    -0.977304f32,
    -0.484092f32,
    -0.646427f32,
    -0.451443f32,
    -0.612126f32,
    -0.224475f32,
    -0.731608f32,
    -0.257077f32,
    -0.665857f32,
    -0.346742f32,
    -1.216372f32,
    0.227267f32,
    0.231249f32,
    -1.693073f32,
    -0.035899f32,
    0.380845f32,
    -0.058476f32,
    0.409405f32,
    -0.066679f32,
    0.406731f32,
    -0.068501f32,
    0.396748f32,
    0.639462f32,
    0.150834f32,
    -0.418659f32,
    -1.421931f32,
    0.101889f32,
    0.083573f32,
    0.129746f32,
    0.134460f32,
    0.081185f32,
    0.127420f32,
    0.083664f32,
    0.051096f32,
    1.361688f32,
    0.386093f32,
];
static mut av1_tx_split_nn_bias_8x8_layer0: [core::ffi::c_float; 12] = [
    4.280443f32,
    2.218902f32,
    -0.256953f32,
    3.161431f32,
    2.082548f32,
    2.506052f32,
    2.563224f32,
    1.421976f32,
    -1.627813f32,
    -1.436085f32,
    2.297265f32,
    1.500469f32,
];
static mut av1_tx_split_nn_weights_8x8_layer1: [core::ffi::c_float; 12] = [
    1.178833f32,
    -0.428527f32,
    -0.078737f32,
    0.381434f32,
    -0.466895f32,
    -0.901745f32,
    -0.766968f32,
    -0.356663f32,
    0.450146f32,
    0.509370f32,
    -0.356604f32,
    -0.443506f32,
];
static mut av1_tx_split_nn_bias_8x8_layer1: [core::ffi::c_float; 1] = [-0.156294f32];
static mut av1_tx_split_nnconfig_8x8: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 12 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [12 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_8x16_layer0: [core::ffi::c_float; 512] = [
    0.374660f32,
    0.218905f32,
    -0.139779f32,
    0.212141f32,
    0.056517f32,
    0.051114f32,
    0.042860f32,
    -0.273258f32,
    -0.340809f32,
    0.138983f32,
    -0.216996f32,
    -0.241519f32,
    -0.123244f32,
    0.078577f32,
    -0.472273f32,
    -0.194201f32,
    0.125056f32,
    0.239761f32,
    -0.332782f32,
    0.174782f32,
    -0.211400f32,
    -0.129795f32,
    0.062195f32,
    0.113176f32,
    -0.008869f32,
    0.140764f32,
    0.059833f32,
    0.163826f32,
    0.359293f32,
    -0.109797f32,
    -0.022091f32,
    -0.059536f32,
    -0.188226f32,
    0.179709f32,
    0.031386f32,
    0.164790f32,
    0.214364f32,
    0.198555f32,
    0.152262f32,
    -0.242980f32,
    0.319367f32,
    -0.136902f32,
    0.046524f32,
    -0.043591f32,
    0.342178f32,
    -0.011757f32,
    -0.014286f32,
    0.072871f32,
    -0.278314f32,
    -0.345303f32,
    -0.252103f32,
    -0.107154f32,
    -0.235101f32,
    -0.106739f32,
    -0.120865f32,
    -0.160042f32,
    0.240028f32,
    0.112902f32,
    -0.141587f32,
    -0.703012f32,
    -0.136591f32,
    0.318993f32,
    -0.154417f32,
    -0.054668f32,
    0.192870f32,
    0.176166f32,
    -0.029965f32,
    0.266942f32,
    -0.178384f32,
    0.038680f32,
    0.134403f32,
    -0.002426f32,
    0.534825f32,
    -0.070923f32,
    0.413281f32,
    0.418148f32,
    0.093729f32,
    0.016454f32,
    0.305358f32,
    -0.040512f32,
    0.069904f32,
    -0.227588f32,
    -0.362220f32,
    -0.031604f32,
    -0.394901f32,
    0.071506f32,
    -0.342833f32,
    -0.142550f32,
    -0.164005f32,
    0.182600f32,
    0.213062f32,
    0.076805f32,
    0.278758f32,
    0.125613f32,
    -0.035552f32,
    0.040971f32,
    0.182785f32,
    -0.227961f32,
    -0.105413f32,
    -0.074949f32,
    -0.084629f32,
    -0.254767f32,
    0.114657f32,
    0.047121f32,
    0.195902f32,
    0.264759f32,
    0.017799f32,
    0.210230f32,
    0.150749f32,
    -0.142142f32,
    0.182494f32,
    -0.142415f32,
    -0.259782f32,
    -0.114830f32,
    -0.198826f32,
    0.000061f32,
    -0.375668f32,
    -0.276656f32,
    -0.373202f32,
    0.210298f32,
    0.422680f32,
    0.066960f32,
    0.351106f32,
    -0.209034f32,
    0.367195f32,
    -0.110274f32,
    0.115573f32,
    -0.066642f32,
    -0.389673f32,
    -0.260447f32,
    0.056949f32,
    -0.180425f32,
    0.069922f32,
    -0.153506f32,
    -0.097053f32,
    -0.111757f32,
    0.094069f32,
    0.144837f32,
    -0.052984f32,
    -0.506681f32,
    -0.034474f32,
    0.279057f32,
    -0.105025f32,
    0.006656f32,
    -0.125017f32,
    -0.114096f32,
    0.103153f32,
    -0.117402f32,
    -0.359472f32,
    0.072534f32,
    0.110291f32,
    0.003088f32,
    -0.456897f32,
    0.038331f32,
    -0.322298f32,
    0.113942f32,
    -0.119916f32,
    -0.194392f32,
    0.093167f32,
    0.193459f32,
    0.074671f32,
    0.033602f32,
    0.004440f32,
    -0.179578f32,
    -0.036637f32,
    -0.216172f32,
    -0.296530f32,
    -0.318992f32,
    0.319160f32,
    -0.066218f32,
    0.291246f32,
    0.181292f32,
    0.089914f32,
    0.025273f32,
    0.303128f32,
    0.019063f32,
    0.078545f32,
    -0.396919f32,
    0.014065f32,
    -0.122121f32,
    0.037107f32,
    -0.151886f32,
    -0.299392f32,
    -0.172207f32,
    -0.124571f32,
    -0.232553f32,
    0.102970f32,
    -0.225040f32,
    0.061059f32,
    -0.258188f32,
    -0.469871f32,
    -0.099607f32,
    -0.061524f32,
    -0.213700f32,
    0.070237f32,
    -0.289134f32,
    -0.238225f32,
    0.256403f32,
    -0.119344f32,
    0.067782f32,
    -0.398983f32,
    -0.123975f32,
    -0.200205f32,
    -0.047038f32,
    0.026569f32,
    0.031037f32,
    0.094302f32,
    -0.101239f32,
    0.433307f32,
    -0.303612f32,
    0.088537f32,
    -0.164436f32,
    0.202471f32,
    -0.048592f32,
    -0.251904f32,
    0.122577f32,
    -0.309874f32,
    -0.263405f32,
    -0.292503f32,
    0.216589f32,
    0.035378f32,
    0.136599f32,
    -0.145844f32,
    -0.018211f32,
    0.174084f32,
    -0.449941f32,
    -0.001428f32,
    0.064134f32,
    0.039652f32,
    0.111083f32,
    -0.246076f32,
    -0.204733f32,
    0.056559f32,
    -0.000123f32,
    0.104049f32,
    0.138512f32,
    -0.128309f32,
    0.087855f32,
    0.232784f32,
    0.247138f32,
    0.162766f32,
    0.154829f32,
    0.313605f32,
    -0.164115f32,
    -0.050844f32,
    0.156549f32,
    0.185279f32,
    -0.238962f32,
    -0.308281f32,
    -0.179592f32,
    -0.193262f32,
    0.201670f32,
    -0.203399f32,
    -0.096831f32,
    -0.127867f32,
    0.310674f32,
    -0.008181f32,
    0.004078f32,
    -0.211038f32,
    -0.193480f32,
    -0.185639f32,
    -0.150202f32,
    -0.204858f32,
    -0.240758f32,
    0.114268f32,
    -0.032535f32,
    -0.052403f32,
    -0.234333f32,
    -0.064072f32,
    -0.208444f32,
    -0.352853f32,
    -0.224001f32,
    -0.156330f32,
    0.215436f32,
    0.171846f32,
    0.291849f32,
    0.108832f32,
    0.046991f32,
    -0.127801f32,
    0.032485f32,
    0.141493f32,
    0.123319f32,
    -0.057250f32,
    0.315346f32,
    -0.061317f32,
    -0.465086f32,
    -0.130179f32,
    -0.217841f32,
    -0.239089f32,
    -0.073251f32,
    -0.327718f32,
    0.054905f32,
    -0.283169f32,
    -0.028900f32,
    0.071450f32,
    0.270072f32,
    0.248891f32,
    0.088052f32,
    0.253319f32,
    0.122808f32,
    0.175490f32,
    -0.147805f32,
    0.089169f32,
    -0.045457f32,
    -0.330788f32,
    0.099791f32,
    -0.137376f32,
    -0.195977f32,
    -0.350942f32,
    -0.284930f32,
    -0.559037f32,
    0.030504f32,
    0.162554f32,
    -0.199100f32,
    -0.050453f32,
    -0.131320f32,
    -0.077863f32,
    -0.066253f32,
    -0.379723f32,
    -0.424047f32,
    -0.081182f32,
    -0.252261f32,
    -0.102815f32,
    0.058240f32,
    -0.182036f32,
    0.176772f32,
    -0.070823f32,
    0.216054f32,
    -0.211533f32,
    -0.232992f32,
    0.279346f32,
    0.117984f32,
    0.236674f32,
    0.126625f32,
    -0.046220f32,
    0.044919f32,
    0.278492f32,
    0.083944f32,
    0.180512f32,
    0.217994f32,
    0.401170f32,
    -0.064417f32,
    0.011636f32,
    -0.139597f32,
    -0.050020f32,
    -0.268438f32,
    -0.032803f32,
    0.024908f32,
    -0.085713f32,
    -0.012984f32,
    -0.055192f32,
    -0.338657f32,
    0.045826f32,
    -0.312849f32,
    -0.023393f32,
    -0.168800f32,
    -0.030886f32,
    -0.131816f32,
    -0.253542f32,
    -0.104812f32,
    -0.354389f32,
    0.169464f32,
    0.094151f32,
    -0.217122f32,
    -0.456397f32,
    0.211478f32,
    0.219232f32,
    -0.155519f32,
    -0.353700f32,
    -0.264759f32,
    -0.034709f32,
    0.034409f32,
    -0.148639f32,
    -0.132850f32,
    -0.216791f32,
    -0.118492f32,
    0.173721f32,
    -0.144181f32,
    0.335028f32,
    0.176439f32,
    0.105980f32,
    0.169390f32,
    0.155615f32,
    -0.040618f32,
    -0.176029f32,
    0.155569f32,
    -0.184833f32,
    -0.171099f32,
    -0.178663f32,
    -0.032051f32,
    -0.434334f32,
    0.092238f32,
    -0.263103f32,
    0.061804f32,
    -0.172957f32,
    0.005962f32,
    -0.100176f32,
    0.125898f32,
    0.048092f32,
    -0.088141f32,
    0.247196f32,
    -0.221601f32,
    -0.114474f32,
    -0.124410f32,
    -0.156393f32,
    -0.181782f32,
    -0.083562f32,
    0.034937f32,
    0.403401f32,
    -0.046200f32,
    0.322259f32,
    0.219678f32,
    0.109850f32,
    0.051837f32,
    0.196861f32,
    -0.019118f32,
    0.248818f32,
    -0.137567f32,
    0.127862f32,
    0.052293f32,
    0.298726f32,
    0.275788f32,
    0.015344f32,
    0.058714f32,
    0.283691f32,
    -0.053794f32,
    -0.123270f32,
    -0.227761f32,
    -0.141744f32,
    -0.268515f32,
    -0.007189f32,
    -0.242117f32,
    -0.252396f32,
    -0.069017f32,
    0.034803f32,
    -0.003388f32,
    -0.262577f32,
    0.062115f32,
    -0.298393f32,
    0.215415f32,
    -0.153615f32,
    0.289902f32,
    0.085886f32,
    -0.504290f32,
    0.077178f32,
    0.150861f32,
    -0.228848f32,
    -0.261020f32,
    0.198204f32,
    0.162113f32,
    0.346418f32,
    -0.286950f32,
    0.354756f32,
    -0.226419f32,
    0.024720f32,
    0.208037f32,
    0.107286f32,
    -0.110849f32,
    0.104415f32,
    -0.207725f32,
    0.063932f32,
    -0.037748f32,
    -0.167037f32,
    -0.068282f32,
    0.320815f32,
    -0.051884f32,
    0.099989f32,
    -0.078388f32,
    0.127071f32,
    0.046675f32,
    -0.336571f32,
    -0.273080f32,
    0.264694f32,
    -0.007352f32,
    -0.093828f32,
    0.094773f32,
    -0.144434f32,
    0.091795f32,
    -0.031615f32,
    0.056914f32,
    0.064673f32,
    -0.136669f32,
    0.344734f32,
    0.225926f32,
    0.283451f32,
    -0.068354f32,
    0.030572f32,
    0.180784f32,
    -0.378047f32,
    -0.092962f32,
    -0.083291f32,
    0.038970f32,
    0.052094f32,
    -0.017932f32,
    0.216302f32,
    -0.184396f32,
    0.079888f32,
    0.210406f32,
    -0.020627f32,
    0.244744f32,
    0.336972f32,
    -0.182914f32,
    -0.220976f32,
    -0.304225f32,
    -0.330974f32,
    -0.370868f32,
    -0.084935f32,
    -0.136489f32,
    -0.210082f32,
    -0.188088f32,
    -0.408768f32,
    0.184693f32,
];
static mut av1_tx_split_nn_bias_8x16_layer0: [core::ffi::c_float; 64] = [
    -0.274107f32,
    0.445751f32,
    0.234359f32,
    0.291593f32,
    0.163298f32,
    0.183707f32,
    -0.548839f32,
    -0.190779f32,
    -0.163346f32,
    -0.669028f32,
    0.399209f32,
    -0.354974f32,
    0.000000f32,
    -0.254630f32,
    0.220149f32,
    0.371104f32,
    0.789759f32,
    0.270300f32,
    0.195126f32,
    -0.206958f32,
    0.917708f32,
    -0.256232f32,
    1.131933f32,
    1.178944f32,
    0.461270f32,
    0.246169f32,
    -0.818614f32,
    -0.111986f32,
    0.759355f32,
    0.154889f32,
    0.470299f32,
    -1.025250f32,
    0.678678f32,
    0.959346f32,
    -0.164105f32,
    0.544079f32,
    -0.448733f32,
    0.649221f32,
    -0.536672f32,
    0.962758f32,
    -0.256427f32,
    0.808664f32,
    -0.118694f32,
    0.684873f32,
    -0.015635f32,
    -0.046469f32,
    0.075481f32,
    0.412647f32,
    0.454456f32,
    -0.107169f32,
    0.775235f32,
    -0.261629f32,
    -1.194849f32,
    0.010093f32,
    -0.231289f32,
    0.658286f32,
    -0.769320f32,
    0.564545f32,
    0.482962f32,
    -0.131378f32,
    -0.255844f32,
    -0.078400f32,
    0.476752f32,
    0.643001f32,
];
static mut av1_tx_split_nn_weights_8x16_layer1: [core::ffi::c_float; 64] = [
    -0.145065f32,
    -0.145101f32,
    0.174786f32,
    0.196692f32,
    0.102025f32,
    -0.087735f32,
    0.386353f32,
    -0.660539f32,
    -0.183940f32,
    0.490045f32,
    -0.276404f32,
    -0.145669f32,
    0.209846f32,
    -0.085574f32,
    -0.156821f32,
    -0.377450f32,
    -0.950010f32,
    0.450709f32,
    -0.108545f32,
    -0.261181f32,
    1.435606f32,
    -0.176621f32,
    -1.158548f32,
    2.035680f32,
    0.218069f32,
    -0.138629f32,
    0.305958f32,
    -0.277194f32,
    -0.602468f32,
    0.203873f32,
    0.120720f32,
    0.216095f32,
    -0.434502f32,
    -0.579746f32,
    -0.239450f32,
    0.755529f32,
    0.545643f32,
    0.232091f32,
    0.330169f32,
    0.988136f32,
    -0.070465f32,
    -0.345584f32,
    -0.162455f32,
    -0.617064f32,
    0.123881f32,
    -0.201098f32,
    0.222756f32,
    0.112932f32,
    0.048647f32,
    -0.147890f32,
    0.394584f32,
    -0.262148f32,
    0.280564f32,
    -0.195432f32,
    -0.047515f32,
    1.133410f32,
    0.255415f32,
    -0.299032f32,
    -0.397807f32,
    -0.153246f32,
    -0.256734f32,
    0.177370f32,
    0.213522f32,
    -0.530158f32,
];
static mut av1_tx_split_nn_bias_8x16_layer1: [core::ffi::c_float; 1] = [0.14910713f32];
static mut av1_tx_split_nnconfig_8x16: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [64 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_16x16_layer0: [core::ffi::c_float; 288] = [
    -0.177215f32,
    -0.297166f32,
    0.299924f32,
    0.207878f32,
    0.216871f32,
    0.173264f32,
    0.295464f32,
    0.048395f32,
    0.154731f32,
    0.305880f32,
    0.056787f32,
    -0.166617f32,
    0.115653f32,
    -0.529477f32,
    -0.073995f32,
    -0.211746f32,
    -0.018169f32,
    0.000788f32,
    -0.024940f32,
    -0.007055f32,
    0.001392f32,
    0.021678f32,
    -1.594600f32,
    -0.099593f32,
    0.332930f32,
    0.103574f32,
    0.158249f32,
    0.182601f32,
    0.332665f32,
    0.226207f32,
    -0.139566f32,
    0.185531f32,
    0.099074f32,
    -0.185654f32,
    -0.203121f32,
    -0.285678f32,
    -0.313453f32,
    -0.294452f32,
    -0.143707f32,
    -0.031265f32,
    -0.453030f32,
    -0.061874f32,
    -0.066150f32,
    -0.099058f32,
    -0.458879f32,
    0.127544f32,
    0.338314f32,
    -0.161350f32,
    0.030091f32,
    -0.075528f32,
    0.004320f32,
    0.353690f32,
    -0.013480f32,
    -0.420402f32,
    -0.004659f32,
    -0.329401f32,
    -0.001745f32,
    0.227384f32,
    -0.055183f32,
    0.121405f32,
    0.160340f32,
    0.143603f32,
    -0.221813f32,
    0.079107f32,
    -0.657639f32,
    -0.084348f32,
    -0.303414f32,
    0.046774f32,
    -0.367679f32,
    0.060005f32,
    0.168645f32,
    0.084421f32,
    -0.133625f32,
    0.301375f32,
    0.079412f32,
    -0.419303f32,
    0.017235f32,
    0.068637f32,
    0.018384f32,
    -0.428325f32,
    -0.019753f32,
    0.149444f32,
    -0.474836f32,
    -0.287162f32,
    0.198083f32,
    0.028292f32,
    -0.299092f32,
    -0.005849f32,
    -0.256245f32,
    0.233277f32,
    -0.217561f32,
    -0.264003f32,
    0.269411f32,
    0.207032f32,
    -0.339411f32,
    -0.198431f32,
    -0.028521f32,
    0.158076f32,
    0.177116f32,
    0.345702f32,
    -0.145132f32,
    0.064623f32,
    -0.090867f32,
    0.288816f32,
    -0.263198f32,
    -0.071028f32,
    -0.044546f32,
    0.380017f32,
    -0.014100f32,
    -0.271192f32,
    -0.318559f32,
    0.129015f32,
    -0.050314f32,
    -0.093355f32,
    -0.578498f32,
    0.099090f32,
    -0.133080f32,
    -0.029975f32,
    -0.059828f32,
    -0.157765f32,
    -0.321153f32,
    -0.343671f32,
    -0.242959f32,
    0.128304f32,
    0.017170f32,
    0.072787f32,
    -0.475838f32,
    -0.003806f32,
    -0.068615f32,
    0.150556f32,
    -0.159903f32,
    -0.416513f32,
    0.218794f32,
    -0.290456f32,
    -0.084569f32,
    -0.170014f32,
    -0.044414f32,
    -0.153069f32,
    -0.077329f32,
    -0.089747f32,
    -0.096526f32,
    0.537952f32,
    0.134725f32,
    -0.006469f32,
    -0.323335f32,
    -0.168183f32,
    -0.107163f32,
    -0.139954f32,
    0.011286f32,
    -0.021712f32,
    -0.513992f32,
    0.259135f32,
    -0.319808f32,
    0.077811f32,
    0.104613f32,
    0.370571f32,
    0.185244f32,
    0.065530f32,
    -0.091098f32,
    -0.573741f32,
    0.111934f32,
    0.437417f32,
    -0.123691f32,
    0.220641f32,
    -0.024783f32,
    -0.149460f32,
    -0.354185f32,
    -0.134127f32,
    0.038015f32,
    -0.380596f32,
    0.250980f32,
    0.142208f32,
    0.135170f32,
    -0.131129f32,
    -0.357556f32,
    -0.530945f32,
    0.159672f32,
    -0.147025f32,
    -0.377829f32,
    -0.504508f32,
    -0.492870f32,
    0.020753f32,
    0.142818f32,
    0.025172f32,
    0.086140f32,
    0.091283f32,
    0.087491f32,
    -0.186415f32,
    0.177785f32,
    -0.195121f32,
    -1.191148f32,
    -0.477102f32,
    0.023371f32,
    0.227004f32,
    -0.023502f32,
    -0.242913f32,
    -0.074398f32,
    -0.153480f32,
    0.162900f32,
    0.415509f32,
    -0.162565f32,
    -0.131709f32,
    -0.258852f32,
    -0.252027f32,
    -0.080845f32,
    -0.330274f32,
    0.021874f32,
    0.232398f32,
    0.069277f32,
    0.220567f32,
    -0.024237f32,
    -0.366771f32,
    0.081673f32,
    -0.429906f32,
    -0.302170f32,
    0.061045f32,
    0.352777f32,
    -0.230376f32,
    0.408153f32,
    0.064758f32,
    0.142051f32,
    0.007219f32,
    0.622878f32,
    0.212577f32,
    0.036489f32,
    0.081150f32,
    -0.284767f32,
    0.107763f32,
    -0.529786f32,
    -0.072190f32,
    -0.300421f32,
    -0.287959f32,
    -0.568900f32,
    0.011547f32,
    -0.131696f32,
    -0.356854f32,
    -0.587962f32,
    -0.026598f32,
    0.405829f32,
    0.057565f32,
    0.414265f32,
    -0.159155f32,
    0.221456f32,
    0.146314f32,
    0.265776f32,
    -0.006516f32,
    0.473978f32,
    -0.186431f32,
    0.288672f32,
    -0.060437f32,
    0.083380f32,
    -0.205641f32,
    0.360016f32,
    0.222041f32,
    0.420011f32,
    0.024579f32,
    0.377546f32,
    0.250380f32,
    -0.069900f32,
    0.296743f32,
    0.073532f32,
    -0.243225f32,
    -0.374987f32,
    -0.387288f32,
    -0.237255f32,
    -0.287013f32,
    0.417831f32,
    -0.252988f32,
    -0.257652f32,
    -0.066775f32,
    -0.253926f32,
    0.057841f32,
    0.346133f32,
    -0.157797f32,
    -0.406028f32,
    -0.286893f32,
    0.274507f32,
    -0.452561f32,
    0.143381f32,
    -0.097755f32,
    0.021242f32,
    0.034561f32,
    0.044115f32,
    0.004065f32,
    0.066729f32,
    0.043558f32,
    0.102991f32,
    -0.477574f32,
];
static mut av1_tx_split_nn_bias_16x16_layer0: [core::ffi::c_float; 24] = [
    -0.479033f32,
    1.467402f32,
    -0.366291f32,
    0.372511f32,
    0.715322f32,
    -0.605500f32,
    0.176848f32,
    0.032318f32,
    0.237429f32,
    -0.046047f32,
    0.452082f32,
    0.451805f32,
    -0.822845f32,
    0.636762f32,
    -0.057350f32,
    1.163978f32,
    0.728287f32,
    0.603654f32,
    -0.245519f32,
    -0.893569f32,
    -1.428185f32,
    0.808870f32,
    -0.076159f32,
    1.231976f32,
];
static mut av1_tx_split_nn_weights_16x16_layer1: [core::ffi::c_float; 24] = [
    -0.176161f32,
    1.670188f32,
    -0.180755f32,
    -0.321326f32,
    0.249728f32,
    -0.170504f32,
    -0.538432f32,
    0.033893f32,
    0.149842f32,
    0.404140f32,
    -0.377812f32,
    0.338838f32,
    -0.176091f32,
    0.249844f32,
    -0.362533f32,
    1.412460f32,
    0.196862f32,
    0.278194f32,
    -0.140444f32,
    0.297746f32,
    0.172533f32,
    0.116470f32,
    -0.151656f32,
    -0.603250f32,
];
static mut av1_tx_split_nn_bias_16x16_layer1: [core::ffi::c_float; 1] = [0.184803f32];
static mut av1_tx_split_nnconfig_16x16: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 12 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [24 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_32x32_layer0: [core::ffi::c_float; 384] = [
    -0.439303f32,
    0.004813f32,
    -0.365052f32,
    -0.116868f32,
    -0.356716f32,
    -0.196537f32,
    -0.196770f32,
    -0.076096f32,
    0.357004f32,
    -0.044909f32,
    -0.112910f32,
    -0.129081f32,
    0.156725f32,
    -0.386346f32,
    0.038971f32,
    0.160696f32,
    0.204923f32,
    -0.384333f32,
    -0.319546f32,
    0.028179f32,
    -0.250524f32,
    -0.289669f32,
    -0.284138f32,
    -0.258963f32,
    -0.180854f32,
    -0.000807f32,
    -0.029620f32,
    -0.353134f32,
    0.212408f32,
    0.141414f32,
    0.303016f32,
    0.098066f32,
    0.482455f32,
    0.036069f32,
    -0.166279f32,
    0.210119f32,
    -0.086337f32,
    -0.023550f32,
    -0.250796f32,
    -0.183945f32,
    -0.393856f32,
    0.170608f32,
    -0.306403f32,
    0.026318f32,
    -0.277296f32,
    0.092684f32,
    -0.033584f32,
    -0.018371f32,
    -0.025043f32,
    -0.257659f32,
    -0.139163f32,
    -0.206949f32,
    -0.190105f32,
    0.028053f32,
    0.361851f32,
    -0.364726f32,
    -0.096771f32,
    -0.184166f32,
    -0.433228f32,
    -0.182191f32,
    -0.097051f32,
    0.259172f32,
    0.016432f32,
    0.259358f32,
    0.145059f32,
    0.037196f32,
    0.091581f32,
    -0.219644f32,
    0.140384f32,
    -0.446837f32,
    -0.234531f32,
    0.149508f32,
    -0.083429f32,
    0.186189f32,
    -0.099890f32,
    -0.111277f32,
    0.495214f32,
    0.085053f32,
    -0.266613f32,
    -0.051366f32,
    0.148593f32,
    0.111875f32,
    0.077787f32,
    -0.371653f32,
    -0.146157f32,
    -0.229235f32,
    0.076203f32,
    0.488975f32,
    0.096771f32,
    -0.009483f32,
    0.192985f32,
    0.246273f32,
    -0.192671f32,
    -0.557890f32,
    -0.292650f32,
    -0.088907f32,
    -0.106892f32,
    -0.329659f32,
    0.012105f32,
    -0.359326f32,
    0.170723f32,
    -0.004357f32,
    0.171593f32,
    -0.478768f32,
    -0.236016f32,
    -0.035077f32,
    0.133731f32,
    0.137962f32,
    -0.397926f32,
    -0.155164f32,
    -0.276709f32,
    -0.186602f32,
    -0.258301f32,
    0.036965f32,
    -0.649359f32,
    0.127605f32,
    0.097930f32,
    0.182775f32,
    -0.313324f32,
    0.053349f32,
    0.204203f32,
    -0.222948f32,
    -0.059008f32,
    -0.049759f32,
    -0.056848f32,
    0.087497f32,
    -0.039987f32,
    -0.055042f32,
    -0.041623f32,
    -0.078424f32,
    -0.317291f32,
    -0.191398f32,
    0.632147f32,
    0.221825f32,
    0.268394f32,
    -0.096357f32,
    0.442545f32,
    -0.007117f32,
    -0.036125f32,
    0.000525f32,
    0.088092f32,
    -0.203653f32,
    0.086925f32,
    0.439141f32,
    0.329889f32,
    -0.370050f32,
    -0.194306f32,
    -0.207430f32,
    0.132779f32,
    -0.217614f32,
    -0.039444f32,
    -0.053019f32,
    -0.260725f32,
    -0.116563f32,
    -0.271048f32,
    0.283737f32,
    -0.007300f32,
    0.062257f32,
    -0.347865f32,
    -0.296767f32,
    -0.359123f32,
    0.230459f32,
    -0.189117f32,
    -0.087622f32,
    -0.561091f32,
    0.184182f32,
    -0.044980f32,
    0.012643f32,
    0.241672f32,
    0.050272f32,
    -0.204851f32,
    -0.159285f32,
    -0.064081f32,
    -0.118666f32,
    -0.269471f32,
    0.231668f32,
    0.135749f32,
    -0.131162f32,
    0.062760f32,
    0.100949f32,
    0.074967f32,
    -0.056918f32,
    0.251707f32,
    0.034098f32,
    0.341290f32,
    -0.105027f32,
    0.313246f32,
    -0.092679f32,
    -0.014632f32,
    -0.390967f32,
    0.136881f32,
    -0.241554f32,
    0.097674f32,
    0.110832f32,
    -0.390245f32,
    0.017654f32,
    -0.506222f32,
    0.065252f32,
    0.244834f32,
    -0.171352f32,
    -0.331702f32,
    0.111043f32,
    0.125217f32,
    -0.058116f32,
    -0.382595f32,
    -0.052545f32,
    0.114261f32,
    -0.493617f32,
    0.243984f32,
    -0.171053f32,
    0.165009f32,
    -0.063020f32,
    0.096502f32,
    0.341339f32,
    -0.013443f32,
    0.056372f32,
    0.339284f32,
    0.398376f32,
    0.389409f32,
    0.257252f32,
    0.517368f32,
    0.078856f32,
    0.087716f32,
    -0.171092f32,
    0.227461f32,
    0.125307f32,
    -0.054423f32,
    -0.143161f32,
    0.224041f32,
    -0.086477f32,
    -0.092548f32,
    0.072392f32,
    -0.061608f32,
    0.258347f32,
    0.147033f32,
    -0.478244f32,
    -0.204869f32,
    0.038552f32,
    -0.144563f32,
    0.224087f32,
    -0.296705f32,
    0.153889f32,
    -0.064624f32,
    0.085265f32,
    -0.103826f32,
    0.127971f32,
    0.019965f32,
    0.111937f32,
    -0.074187f32,
    -0.029518f32,
    -0.127305f32,
    -0.012210f32,
    0.042714f32,
    0.070052f32,
    -0.202360f32,
    0.348144f32,
    -0.132097f32,
    -0.209585f32,
    -0.248286f32,
    -0.065774f32,
    -0.089482f32,
    -0.133226f32,
    0.325430f32,
    -0.013468f32,
    -0.406090f32,
    -0.144936f32,
    0.208620f32,
    0.343445f32,
    -0.059639f32,
    0.114857f32,
    -0.069431f32,
    -0.218725f32,
    0.190575f32,
    -0.368101f32,
    0.030030f32,
    0.062815f32,
    -0.239369f32,
    -0.537852f32,
    0.022487f32,
    0.023038f32,
    0.190788f32,
    0.040123f32,
    -0.004304f32,
    0.060749f32,
    -0.108929f32,
    0.136796f32,
    -0.542875f32,
    -0.227074f32,
    -0.182244f32,
    0.082559f32,
    0.019149f32,
    0.178854f32,
    0.120284f32,
    0.009070f32,
    0.068268f32,
    -0.544822f32,
    0.120536f32,
    0.354028f32,
    -0.119890f32,
    -0.122055f32,
    -0.405335f32,
    0.122341f32,
    -0.304412f32,
    0.062405f32,
    -0.302568f32,
    -0.276505f32,
    -0.120915f32,
    -0.221841f32,
    0.282007f32,
    -0.253971f32,
    0.059517f32,
    -0.144976f32,
    0.149391f32,
    -0.047355f32,
    -0.167742f32,
    -0.392333f32,
    -0.041132f32,
    0.342135f32,
    0.017485f32,
    0.021038f32,
    -0.023728f32,
    -0.192181f32,
    -0.103996f32,
    0.092873f32,
    -0.114365f32,
    -0.397732f32,
    -0.065421f32,
    0.053084f32,
    0.035201f32,
    0.053019f32,
    -0.105377f32,
    -0.039500f32,
    0.131904f32,
    -0.123911f32,
    -0.390328f32,
    -0.125198f32,
    -0.000126f32,
    0.014864f32,
    -0.220187f32,
    0.084056f32,
    -0.492155f32,
    -0.164979f32,
    0.133592f32,
    0.121519f32,
    -0.240813f32,
    0.186680f32,
    0.118673f32,
    0.235006f32,
    -0.239894f32,
    -0.185759f32,
    -0.336992f32,
    0.209620f32,
    -0.298845f32,
    0.127803f32,
    -0.083992f32,
    0.194340f32,
    -0.245378f32,
    0.212308f32,
    0.142512f32,
    -0.163324f32,
    0.383495f32,
    0.291065f32,
    0.286620f32,
    -0.239957f32,
    0.225127f32,
    -0.174424f32,
    0.297231f32,
    -0.045434f32,
    0.156444f32,
    -0.184273f32,
    -0.204567f32,
    0.202551f32,
    0.370019f32,
    -0.073910f32,
    0.344897f32,
    0.063100f32,
    0.338547f32,
    -0.099145f32,
    0.391863f32,
    -0.214244f32,
    -0.241734f32,
    -0.281851f32,
    -0.035133f32,
    -0.153157f32,
];
static mut av1_tx_split_nn_bias_32x32_layer0: [core::ffi::c_float; 32] = [
    0.143343f32,
    -0.021982f32,
    -0.314939f32,
    0.170867f32,
    -0.081248f32,
    0.125758f32,
    -0.355762f32,
    0.279798f32,
    1.027712f32,
    -0.434660f32,
    1.072005f32,
    0.668893f32,
    -0.031216f32,
    -0.528650f32,
    0.328349f32,
    0.543645f32,
    -0.188810f32,
    0.221110f32,
    -1.638637f32,
    0.058045f32,
    -1.731105f32,
    -0.444284f32,
    0.513693f32,
    0.890025f32,
    0.160288f32,
    0.393312f32,
    0.332856f32,
    -0.080767f32,
    0.299822f32,
    0.235876f32,
    0.254942f32,
    -0.017796f32,
];
static mut av1_tx_split_nn_weights_32x32_layer1: [core::ffi::c_float; 32] = [
    -0.090326f32,
    -0.267553f32,
    -0.026071f32,
    0.100912f32,
    0.279137f32,
    0.079064f32,
    -0.074885f32,
    0.053804f32,
    0.736810f32,
    -0.031693f32,
    -0.970514f32,
    0.174069f32,
    0.095940f32,
    -0.065047f32,
    0.052911f32,
    0.176728f32,
    -0.058274f32,
    0.148364f32,
    -0.162210f32,
    0.093875f32,
    -0.367663f32,
    0.020876f32,
    0.137280f32,
    -1.099116f32,
    0.146854f32,
    0.075590f32,
    0.228534f32,
    0.141993f32,
    0.072143f32,
    0.101421f32,
    -0.068547f32,
    -0.154148f32,
];
static mut av1_tx_split_nn_bias_32x32_layer1: [core::ffi::c_float; 1] = [0.316622f32];
static mut av1_tx_split_nnconfig_32x32: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 12 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [32 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_32x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_32x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_32x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_32x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_64x64_layer0: [core::ffi::c_float; 384] = [
    -0.006828f32,
    0.149944f32,
    -0.017614f32,
    -0.044599f32,
    -0.024517f32,
    0.507698f32,
    0.001039f32,
    0.037164f32,
    0.015091f32,
    -0.306620f32,
    -0.162047f32,
    -0.369440f32,
    0.396310f32,
    0.087121f32,
    0.208609f32,
    -0.083068f32,
    0.493774f32,
    0.217682f32,
    0.377393f32,
    0.172879f32,
    0.397422f32,
    0.078919f32,
    0.741350f32,
    0.064169f32,
    -0.099989f32,
    -0.192983f32,
    -0.278230f32,
    -0.310048f32,
    -0.439965f32,
    -0.226698f32,
    -0.436596f32,
    -0.007551f32,
    -0.396721f32,
    0.153570f32,
    -0.190838f32,
    -0.071869f32,
    0.048799f32,
    -0.301301f32,
    -0.005015f32,
    0.500480f32,
    -0.030622f32,
    -0.559095f32,
    -0.032634f32,
    -0.054160f32,
    -0.056979f32,
    -0.456545f32,
    0.306536f32,
    -0.411323f32,
    -0.005366f32,
    -0.069496f32,
    0.019990f32,
    0.327931f32,
    -0.002516f32,
    0.393190f32,
    0.001759f32,
    0.035093f32,
    -0.030302f32,
    -0.528984f32,
    0.174781f32,
    0.241462f32,
    -0.415427f32,
    -0.164502f32,
    0.143065f32,
    -0.122595f32,
    0.082049f32,
    -0.143346f32,
    0.055642f32,
    -0.124701f32,
    0.004050f32,
    -0.216235f32,
    -2.681730f32,
    0.101658f32,
    0.381239f32,
    0.465936f32,
    0.331154f32,
    0.301708f32,
    -0.360171f32,
    0.054886f32,
    -0.118658f32,
    0.287921f32,
    0.277859f32,
    0.203784f32,
    0.247809f32,
    0.656924f32,
    -0.354628f32,
    0.315081f32,
    0.105108f32,
    -0.510179f32,
    0.059267f32,
    0.061386f32,
    0.076423f32,
    0.347119f32,
    0.100134f32,
    0.028402f32,
    -0.118621f32,
    -0.238689f32,
    0.080141f32,
    -0.138863f32,
    0.009009f32,
    -0.100526f32,
    -0.138875f32,
    0.066992f32,
    0.005949f32,
    0.564336f32,
    0.046994f32,
    0.004655f32,
    0.366047f32,
    0.014695f32,
    -0.146928f32,
    -0.024665f32,
    -0.440357f32,
    -0.109395f32,
    0.527231f32,
    -0.020925f32,
    -0.227236f32,
    -0.068141f32,
    0.282009f32,
    0.040192f32,
    -0.267100f32,
    0.229228f32,
    0.133861f32,
    0.338706f32,
    -0.030178f32,
    -0.040919f32,
    -0.026343f32,
    -0.330338f32,
    -0.066931f32,
    -0.110580f32,
    -0.072056f32,
    0.599457f32,
    -0.020738f32,
    0.169200f32,
    0.836240f32,
    -0.157548f32,
    0.386273f32,
    0.002404f32,
    0.329410f32,
    -0.007020f32,
    0.351705f32,
    -0.041259f32,
    0.388861f32,
    0.003899f32,
    0.582627f32,
    0.023572f32,
    0.409912f32,
    -0.158472f32,
    0.536383f32,
    0.525093f32,
    0.604247f32,
    0.439159f32,
    0.692832f32,
    0.046272f32,
    0.590367f32,
    -0.082166f32,
    0.262357f32,
    0.478671f32,
    0.031935f32,
    0.042675f32,
    0.120002f32,
    0.398616f32,
    -0.078967f32,
    0.227986f32,
    -0.044679f32,
    0.151061f32,
    -0.085564f32,
    0.220205f32,
    -0.265606f32,
    -0.203623f32,
    0.204719f32,
    -0.125922f32,
    0.038544f32,
    -0.269379f32,
    0.025866f32,
    0.109967f32,
    0.019064f32,
    -0.237297f32,
    -0.309746f32,
    -0.329118f32,
    -0.278368f32,
    -0.063859f32,
    0.278496f32,
    0.018620f32,
    0.209971f32,
    0.296250f32,
    0.142850f32,
    0.288689f32,
    0.137084f32,
    0.130517f32,
    0.128171f32,
    -0.155396f32,
    -0.008449f32,
    -0.099845f32,
    0.173455f32,
    -0.059909f32,
    -0.147318f32,
    0.102851f32,
    -0.251389f32,
    -0.001448f32,
    0.103907f32,
    0.297273f32,
    -0.027846f32,
    0.028260f32,
    -0.382601f32,
    0.346695f32,
    -0.601641f32,
    0.162366f32,
    -0.477495f32,
    -0.042731f32,
    -0.387871f32,
    -0.051791f32,
    -0.401498f32,
    -0.048446f32,
    -0.456270f32,
    -0.062287f32,
    0.493919f32,
    0.003008f32,
    0.099917f32,
    -0.358525f32,
    -0.094903f32,
    -0.022811f32,
    -0.062259f32,
    0.019455f32,
    -0.050644f32,
    0.020041f32,
    -0.132912f32,
    -0.061578f32,
    -3.083691f32,
    -0.014961f32,
    -0.129115f32,
    -0.710559f32,
    0.157213f32,
    -0.844037f32,
    -0.121991f32,
    -0.943386f32,
    -0.231269f32,
    -0.003462f32,
    0.331478f32,
    -0.132703f32,
    -1.285993f32,
    -0.120957f32,
    -0.373755f32,
    -0.322609f32,
    0.309059f32,
    -0.131523f32,
    -0.118334f32,
    -0.063805f32,
    -0.104251f32,
    0.012166f32,
    -0.094699f32,
    -0.283753f32,
    0.128168f32,
    -0.526929f32,
    -0.050331f32,
    0.186153f32,
    0.005913f32,
    -0.221236f32,
    0.036363f32,
    0.160909f32,
    -0.001342f32,
    -0.382749f32,
    0.037820f32,
    0.281689f32,
    -0.024275f32,
    0.028854f32,
    0.318291f32,
    0.318526f32,
    0.035778f32,
    0.034031f32,
    0.189663f32,
    -0.293367f32,
    0.082022f32,
    0.127923f32,
    0.078866f32,
    -0.081361f32,
    -0.268117f32,
    0.246675f32,
    0.248605f32,
    -0.215479f32,
    -0.073084f32,
    0.496140f32,
    -0.067327f32,
    0.396237f32,
    -0.120739f32,
    0.033752f32,
    -0.044120f32,
    -0.218941f32,
    -0.028078f32,
    0.195132f32,
    -0.040400f32,
    0.281604f32,
    -0.100471f32,
    0.415207f32,
    -0.258503f32,
    -0.429749f32,
    0.150569f32,
    -0.010859f32,
    0.136448f32,
    0.026589f32,
    0.148466f32,
    0.110764f32,
    0.380967f32,
    0.009177f32,
    0.103075f32,
    0.116417f32,
    0.226273f32,
    -0.327746f32,
    0.169346f32,
    0.284553f32,
    -0.094986f32,
    0.312745f32,
    -0.147840f32,
    0.025062f32,
    -0.494482f32,
    0.112388f32,
    -0.213962f32,
    0.107050f32,
    -0.433371f32,
    -0.096276f32,
    -0.244835f32,
    -0.003518f32,
    -0.459148f32,
    -0.145080f32,
    0.017150f32,
    0.042846f32,
    -0.237479f32,
    0.104746f32,
    0.158677f32,
    0.358937f32,
    0.099921f32,
    0.277109f32,
    0.012410f32,
    -0.062897f32,
    0.116130f32,
    0.255309f32,
    0.341628f32,
    0.145002f32,
    -0.429344f32,
    -0.016433f32,
    -0.068985f32,
    0.285194f32,
    -0.286719f32,
    -0.018298f32,
    -0.179369f32,
    -0.194655f32,
    -0.165380f32,
    0.026071f32,
    -0.428268f32,
    -0.379929f32,
    -0.727543f32,
    0.179610f32,
    -0.963979f32,
    -0.042026f32,
    -0.616202f32,
    0.133401f32,
    -0.784966f32,
    0.061205f32,
    -0.713357f32,
    0.129795f32,
    0.120512f32,
    -0.339545f32,
    0.353557f32,
    0.114906f32,
    -0.329813f32,
    -0.209987f32,
    0.085410f32,
    0.214313f32,
    -0.122082f32,
    0.335770f32,
    -0.020937f32,
    0.202456f32,
    0.289023f32,
    -0.421186f32,
    0.337905f32,
    0.407663f32,
    0.132771f32,
    0.071734f32,
    0.213914f32,
    0.128595f32,
    0.302659f32,
    -0.209501f32,
    0.217756f32,
    0.253079f32,
    -0.089505f32,
    -0.205614f32,
];
static mut av1_tx_split_nn_bias_64x64_layer0: [core::ffi::c_float; 32] = [
    0.296914f32,
    -1.826816f32,
    0.346130f32,
    0.969520f32,
    -0.528154f32,
    1.175862f32,
    -0.075985f32,
    -0.097323f32,
    -0.233059f32,
    0.004846f32,
    0.401279f32,
    -2.272435f32,
    0.086257f32,
    0.414162f32,
    -0.194786f32,
    -0.233887f32,
    -0.113215f32,
    -2.453546f32,
    0.861214f32,
    0.298361f32,
    0.267397f32,
    -0.158557f32,
    -0.119911f32,
    -0.098134f32,
    -0.339263f32,
    0.385871f32,
    -0.678123f32,
    0.263218f32,
    0.251611f32,
    -1.155773f32,
    -0.365437f32,
    0.229255f32,
];
static mut av1_tx_split_nn_weights_64x64_layer1: [core::ffi::c_float; 32] = [
    0.502104f32,
    -0.708023f32,
    0.419648f32,
    1.583418f32,
    0.419355f32,
    -1.462981f32,
    -0.439623f32,
    0.405691f32,
    0.823257f32,
    0.061654f32,
    0.750875f32,
    0.775031f32,
    -0.387909f32,
    0.447385f32,
    0.284690f32,
    0.353262f32,
    -0.224347f32,
    0.832864f32,
    -1.708491f32,
    -1.042447f32,
    -0.272829f32,
    0.540640f32,
    0.310509f32,
    0.723745f32,
    0.245592f32,
    -0.218417f32,
    -0.597987f32,
    -0.362301f32,
    0.702217f32,
    -0.692614f32,
    0.207812f32,
    0.513560f32,
];
static mut av1_tx_split_nn_bias_64x64_layer1: [core::ffi::c_float; 1] = [-0.2307045f32];
static mut av1_tx_split_nnconfig_64x64: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 12 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [32 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_64x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_64x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_64x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_64x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_4x16_layer0: [core::ffi::c_float; 128] = [
    -1.344184f32,
    -1.454625f32,
    -0.703110f32,
    -0.140570f32,
    -0.841536f32,
    -0.068131f32,
    -2.128968f32,
    -0.655518f32,
    0.432180f32,
    0.879752f32,
    -0.222211f32,
    0.061615f32,
    -0.230969f32,
    0.569496f32,
    1.424188f32,
    0.598063f32,
    -0.436005f32,
    -0.737606f32,
    -0.137875f32,
    -0.085730f32,
    -0.076512f32,
    -0.583101f32,
    -0.937377f32,
    -0.203556f32,
    -0.215797f32,
    -0.015361f32,
    -0.124098f32,
    -0.411917f32,
    0.340441f32,
    -0.331752f32,
    -0.472607f32,
    -0.097714f32,
    -0.930572f32,
    -1.354713f32,
    -0.550724f32,
    0.176212f32,
    -0.636060f32,
    0.183271f32,
    -0.610212f32,
    0.345895f32,
    -1.100906f32,
    -1.605713f32,
    0.111888f32,
    -0.140937f32,
    0.063013f32,
    -0.013315f32,
    -0.273472f32,
    -0.255870f32,
    1.200328f32,
    0.274002f32,
    1.005776f32,
    0.322392f32,
    1.222373f32,
    0.158227f32,
    0.408810f32,
    0.145022f32,
    0.139842f32,
    -1.249412f32,
    0.286672f32,
    -0.635699f32,
    0.312562f32,
    -0.495606f32,
    -1.117034f32,
    -0.085107f32,
    -0.097484f32,
    -0.341521f32,
    -0.132199f32,
    -0.863055f32,
    0.217579f32,
    -1.161425f32,
    -0.302087f32,
    -1.357271f32,
    -0.520724f32,
    -1.211069f32,
    -1.048729f32,
    -0.333087f32,
    -1.171527f32,
    -0.280824f32,
    -2.057684f32,
    -0.228755f32,
    0.606278f32,
    0.101198f32,
    -0.314847f32,
    -1.303255f32,
    -0.294964f32,
    1.301923f32,
    0.041712f32,
    0.077593f32,
    -1.152746f32,
    0.495315f32,
    -0.751566f32,
    0.230249f32,
    -0.840661f32,
    0.100731f32,
    1.346269f32,
    0.649898f32,
    -1.432258f32,
    -0.456710f32,
    -1.018123f32,
    -0.348559f32,
    -1.225226f32,
    -0.170717f32,
    -0.354072f32,
    0.068292f32,
    -0.234168f32,
    0.277503f32,
    0.179134f32,
    0.907420f32,
    0.354626f32,
    -0.627210f32,
    0.905779f32,
    0.512612f32,
    0.161190f32,
    -0.843177f32,
    0.014953f32,
    -0.354983f32,
    0.011116f32,
    -0.429598f32,
    -1.017138f32,
    -0.211432f32,
    0.941840f32,
    -0.281747f32,
    0.957776f32,
    -0.541914f32,
    1.041880f32,
    -0.433580f32,
    -1.416451f32,
    -0.166467f32,
];
static mut av1_tx_split_nn_bias_4x16_layer0: [core::ffi::c_float; 16] = [
    3.086118f32,
    -3.235095f32,
    4.830956f32,
    -0.165706f32,
    0.955031f32,
    4.055783f32,
    -0.311489f32,
    4.660205f32,
    -0.576277f32,
    -0.248111f32,
    -0.790519f32,
    -1.686412f32,
    -1.191704f32,
    -3.800073f32,
    4.121552f32,
    -1.399397f32,
];
static mut av1_tx_split_nn_weights_4x16_layer1: [core::ffi::c_float; 16] = [
    -0.758677f32,
    0.388776f32,
    0.439906f32,
    0.011390f32,
    -0.084319f32,
    -0.667969f32,
    -0.467316f32,
    -0.875491f32,
    -0.160668f32,
    0.805292f32,
    0.114393f32,
    -0.549682f32,
    0.462109f32,
    0.343315f32,
    1.092593f32,
    0.483152f32,
];
static mut av1_tx_split_nn_bias_4x16_layer1: [core::ffi::c_float; 1] = [0.8205083f32];
static mut av1_tx_split_nnconfig_4x16: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_4x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_4x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_4x16_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_4x16_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_16x32_layer0: [core::ffi::c_float; 256] = [
    0.180713f32,
    0.033211f32,
    0.607561f32,
    0.138642f32,
    0.637204f32,
    -0.000940f32,
    0.012630f32,
    0.358109f32,
    0.022238f32,
    0.190418f32,
    0.079088f32,
    0.065925f32,
    0.038242f32,
    0.162380f32,
    -0.122728f32,
    0.379382f32,
    -0.303283f32,
    -0.327550f32,
    0.029120f32,
    -0.284553f32,
    0.269588f32,
    -0.309805f32,
    -0.241036f32,
    -0.161103f32,
    -0.304887f32,
    0.239843f32,
    -0.149146f32,
    0.311234f32,
    -0.073640f32,
    -0.132718f32,
    0.178901f32,
    0.474712f32,
    0.020280f32,
    0.063685f32,
    -0.609170f32,
    -0.013658f32,
    -0.338074f32,
    0.250429f32,
    0.082978f32,
    -0.186315f32,
    -0.788959f32,
    0.039859f32,
    -0.426461f32,
    -0.001524f32,
    -0.447211f32,
    0.378102f32,
    0.315617f32,
    0.017428f32,
    0.745494f32,
    -0.219024f32,
    0.512836f32,
    0.200522f32,
    0.680449f32,
    0.313686f32,
    -0.412569f32,
    -0.132927f32,
    0.631120f32,
    0.042735f32,
    0.336153f32,
    0.044772f32,
    0.432606f32,
    0.175681f32,
    -0.634411f32,
    -0.073509f32,
    -0.040643f32,
    -0.559260f32,
    -0.104034f32,
    -0.570495f32,
    -0.247365f32,
    0.063256f32,
    -0.582021f32,
    -0.492585f32,
    -0.194955f32,
    -0.207934f32,
    -0.506627f32,
    0.021743f32,
    -0.416518f32,
    0.320876f32,
    0.115889f32,
    0.149399f32,
    -0.229376f32,
    0.095505f32,
    0.115191f32,
    -0.471921f32,
    0.113068f32,
    0.343684f32,
    -0.036831f32,
    0.021240f32,
    0.295112f32,
    0.031166f32,
    0.448201f32,
    -0.132241f32,
    0.164032f32,
    0.355572f32,
    0.072154f32,
    0.017335f32,
    -0.046113f32,
    0.178719f32,
    -0.026881f32,
    -0.242590f32,
    0.055073f32,
    -0.012958f32,
    0.077904f32,
    0.351356f32,
    0.107655f32,
    0.260568f32,
    -0.080052f32,
    -0.197553f32,
    0.085763f32,
    0.263416f32,
    -0.327741f32,
    0.158855f32,
    0.056899f32,
    -0.162121f32,
    0.339518f32,
    -0.571204f32,
    0.264966f32,
    -0.252214f32,
    -0.202560f32,
    -0.134213f32,
    -0.330188f32,
    0.009470f32,
    -0.468376f32,
    -0.065240f32,
    -0.307957f32,
    0.116479f32,
    -0.222238f32,
    -0.458716f32,
    0.186493f32,
    -0.391415f32,
    0.118649f32,
    -0.104653f32,
    -0.259958f32,
    -0.332081f32,
    -0.403785f32,
    -0.050147f32,
    -0.573511f32,
    0.177117f32,
    -0.598358f32,
    0.164947f32,
    -0.119694f32,
    -0.058520f32,
    0.203829f32,
    -0.267404f32,
    -0.048202f32,
    -0.600006f32,
    0.181594f32,
    -0.731805f32,
    0.146417f32,
    -0.687148f32,
    -1.210525f32,
    -0.450101f32,
    -0.620635f32,
    0.208825f32,
    -0.611357f32,
    0.112202f32,
    -0.309468f32,
    -0.323545f32,
    0.357770f32,
    0.308061f32,
    0.553199f32,
    0.049012f32,
    0.530093f32,
    -0.208597f32,
    0.607882f32,
    -0.058120f32,
    -0.527634f32,
    0.018136f32,
    0.060753f32,
    0.118894f32,
    0.175649f32,
    0.014731f32,
    0.428318f32,
    -0.106465f32,
    -0.119077f32,
    0.080179f32,
    0.524997f32,
    0.368286f32,
    0.528286f32,
    0.213659f32,
    0.639286f32,
    0.195079f32,
    -0.049815f32,
    -0.092008f32,
    -0.302958f32,
    0.298149f32,
    -0.173870f32,
    -0.145205f32,
    -0.233589f32,
    -0.303368f32,
    0.141275f32,
    0.325622f32,
    -0.115293f32,
    0.155188f32,
    0.047225f32,
    0.231050f32,
    -0.167447f32,
    0.349754f32,
    0.295544f32,
    -0.319466f32,
    0.095144f32,
    0.174612f32,
    -0.194652f32,
    0.305915f32,
    -0.239008f32,
    -0.037453f32,
    0.280696f32,
    0.125850f32,
    0.749196f32,
    -0.101919f32,
    0.791808f32,
    -0.236811f32,
    0.064157f32,
    0.032865f32,
    -0.225911f32,
    0.350384f32,
    0.723183f32,
    -0.103992f32,
    0.483085f32,
    -0.123992f32,
    0.602138f32,
    0.023895f32,
    -0.692601f32,
    -0.118387f32,
    0.162527f32,
    0.145178f32,
    -0.184702f32,
    -0.017753f32,
    -0.159436f32,
    0.124105f32,
    -0.131067f32,
    0.310275f32,
    0.151499f32,
    0.138924f32,
    0.537459f32,
    0.263212f32,
    0.615896f32,
    0.281255f32,
    0.021293f32,
    -0.473459f32,
    0.210145f32,
    -0.056682f32,
    0.063658f32,
    0.377254f32,
    -0.314410f32,
    -0.183487f32,
    0.300384f32,
    0.328471f32,
    0.164694f32,
    -0.159272f32,
    -0.160942f32,
    -0.502861f32,
    -0.129147f32,
    0.045916f32,
    -0.606865f32,
    -0.101378f32,
];
static mut av1_tx_split_nn_bias_16x32_layer0: [core::ffi::c_float; 32] = [
    0.051664f32,
    -0.212487f32,
    -0.077596f32,
    -0.818467f32,
    0.638475f32,
    -0.759937f32,
    0.157198f32,
    0.989640f32,
    1.586035f32,
    0.431144f32,
    0.041605f32,
    0.543085f32,
    0.498379f32,
    0.320504f32,
    0.134233f32,
    0.670979f32,
    -0.105562f32,
    -1.574879f32,
    1.261812f32,
    -0.287530f32,
    -1.610592f32,
    0.730899f32,
    -0.894240f32,
    -0.657790f32,
    0.270806f32,
    -0.181708f32,
    0.298578f32,
    0.817240f32,
    -0.221508f32,
    -0.201771f32,
    -0.294389f32,
    1.456413f32,
];
static mut av1_tx_split_nn_weights_16x32_layer1: [core::ffi::c_float; 32] = [
    1.208914f32,
    0.324728f32,
    0.383352f32,
    -0.874321f32,
    0.172565f32,
    -0.580927f32,
    -0.432927f32,
    0.433698f32,
    -0.801935f32,
    0.672028f32,
    0.563493f32,
    0.260077f32,
    -0.200557f32,
    -0.121638f32,
    0.530735f32,
    -0.525196f32,
    0.281799f32,
    0.624204f32,
    -0.662775f32,
    -0.230887f32,
    0.980989f32,
    0.223437f32,
    -0.790591f32,
    0.600724f32,
    -0.273445f32,
    0.427635f32,
    -0.501641f32,
    -0.878390f32,
    0.234731f32,
    -0.172550f32,
    0.418904f32,
    1.792187f32,
];
static mut av1_tx_split_nn_bias_16x32_layer1: [core::ffi::c_float; 1] = [-0.29233751f32];
static mut av1_tx_split_nnconfig_16x32: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [32 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_32x64_layer0: [core::ffi::c_float; 256] = [
    0.031614f32,
    -0.110926f32,
    0.052418f32,
    -0.702506f32,
    0.045708f32,
    0.238329f32,
    -0.021806f32,
    -0.208128f32,
    0.509745f32,
    -0.293891f32,
    0.277788f32,
    0.113937f32,
    0.741576f32,
    0.062848f32,
    0.351878f32,
    0.212532f32,
    0.385842f32,
    0.081517f32,
    0.398502f32,
    -0.015156f32,
    0.242616f32,
    0.214619f32,
    -0.182678f32,
    -0.170546f32,
    0.110605f32,
    -0.236749f32,
    -0.023831f32,
    -0.285243f32,
    0.147156f32,
    -0.257639f32,
    0.341355f32,
    -0.571641f32,
    -0.721797f32,
    0.139588f32,
    -0.518494f32,
    -0.206526f32,
    -0.570560f32,
    -0.184295f32,
    0.110271f32,
    0.210292f32,
    -0.109132f32,
    -0.001080f32,
    0.129251f32,
    -0.204230f32,
    -0.396312f32,
    -0.183024f32,
    0.421243f32,
    -0.013154f32,
    0.222627f32,
    0.169826f32,
    0.226037f32,
    0.218153f32,
    -0.343528f32,
    0.274906f32,
    -0.156632f32,
    0.250261f32,
    -0.484020f32,
    0.019909f32,
    -0.349575f32,
    -0.286643f32,
    -0.507396f32,
    0.202446f32,
    -0.154110f32,
    -0.292644f32,
    0.122666f32,
    0.306963f32,
    0.424895f32,
    0.005579f32,
    0.494094f32,
    -0.079551f32,
    0.473740f32,
    0.352414f32,
    -0.356917f32,
    0.264331f32,
    -0.554487f32,
    0.119978f32,
    0.012291f32,
    -0.141641f32,
    -0.254714f32,
    -0.213723f32,
    -0.116701f32,
    -0.011267f32,
    0.190025f32,
    -0.118501f32,
    0.305151f32,
    -0.316782f32,
    -0.220801f32,
    -0.308420f32,
    -0.324285f32,
    0.421329f32,
    -0.177066f32,
    -0.055114f32,
    0.229698f32,
    -0.199523f32,
    0.054278f32,
    0.365020f32,
    -0.060586f32,
    -0.300618f32,
    0.157563f32,
    -0.064338f32,
    -0.005711f32,
    -0.176991f32,
    -0.424502f32,
    -0.111914f32,
    0.092608f32,
    0.126621f32,
    0.078547f32,
    0.148008f32,
    0.024221f32,
    0.124599f32,
    0.001343f32,
    0.059402f32,
    0.453753f32,
    0.047102f32,
    0.242544f32,
    0.055735f32,
    -0.067451f32,
    -0.170061f32,
    -0.170469f32,
    -0.232173f32,
    0.214908f32,
    0.248889f32,
    0.544348f32,
    -0.084566f32,
    0.402478f32,
    0.298031f32,
    0.099038f32,
    -0.238019f32,
    -0.475085f32,
    -0.070042f32,
    -0.754955f32,
    -0.049095f32,
    -0.783801f32,
    -0.099857f32,
    -0.582008f32,
    -0.055194f32,
    -0.103655f32,
    0.143689f32,
    0.100219f32,
    0.293934f32,
    0.099271f32,
    -0.036320f32,
    0.356626f32,
    -0.261445f32,
    0.879544f32,
    0.000878f32,
    0.532920f32,
    -0.093918f32,
    0.508867f32,
    -0.040215f32,
    -0.789042f32,
    -0.145380f32,
    -0.090040f32,
    -0.066636f32,
    0.015212f32,
    0.352989f32,
    -0.058831f32,
    -0.164588f32,
    0.039890f32,
    0.122861f32,
    0.222508f32,
    0.061217f32,
    0.466487f32,
    0.022666f32,
    0.423777f32,
    -0.002200f32,
    -0.656835f32,
    -0.099760f32,
    -0.520606f32,
    0.303204f32,
    -0.563620f32,
    -0.160922f32,
    -0.243203f32,
    0.313354f32,
    -0.336516f32,
    -0.206764f32,
    -0.236040f32,
    0.325899f32,
    -0.418748f32,
    0.163205f32,
    -0.476242f32,
    -0.121928f32,
    0.139178f32,
    -0.157193f32,
    -0.531766f32,
    -0.180202f32,
    -0.485254f32,
    0.187703f32,
    -0.440072f32,
    0.137854f32,
    0.029139f32,
    0.109530f32,
    -0.078475f32,
    -0.360618f32,
    -0.334672f32,
    -0.350890f32,
    -0.403976f32,
    0.180336f32,
    -0.304542f32,
    0.005123f32,
    0.413995f32,
    0.314639f32,
    0.342648f32,
    -0.293264f32,
    0.358135f32,
    -0.180425f32,
    -0.369530f32,
    -0.048413f32,
    0.498366f32,
    0.121875f32,
    0.270948f32,
    -0.187966f32,
    0.342503f32,
    0.174420f32,
    -0.352105f32,
    0.088080f32,
    0.008277f32,
    0.020275f32,
    -0.002381f32,
    0.504389f32,
    -0.018832f32,
    -0.366047f32,
    -0.090947f32,
    -0.168150f32,
    0.016184f32,
    -0.328914f32,
    0.089579f32,
    -0.017349f32,
    0.005844f32,
    -0.005010f32,
    -1.857514f32,
    -0.282426f32,
    0.010177f32,
    -0.214727f32,
    -0.182529f32,
    0.156943f32,
    -0.162032f32,
    -0.472654f32,
    0.069432f32,
    0.016901f32,
    -0.767905f32,
    0.137129f32,
    -0.411463f32,
    0.049056f32,
    -0.431657f32,
    -0.037641f32,
    0.785500f32,
    0.046225f32,
    0.195831f32,
    0.245204f32,
    0.368614f32,
    0.212261f32,
    0.440626f32,
    -0.158048f32,
    -0.461031f32,
    -0.146280f32,
];
static mut av1_tx_split_nn_bias_32x64_layer0: [core::ffi::c_float; 32] = [
    0.490777f32,
    -1.894238f32,
    0.621333f32,
    -0.076756f32,
    0.286298f32,
    0.286375f32,
    -0.126431f32,
    -0.350034f32,
    -1.017572f32,
    0.620125f32,
    0.408128f32,
    0.238756f32,
    -0.060728f32,
    0.210912f32,
    0.043124f32,
    0.445649f32,
    0.907025f32,
    0.360272f32,
    1.083101f32,
    -0.068952f32,
    1.062348f32,
    0.396354f32,
    0.280075f32,
    0.501732f32,
    0.328422f32,
    0.066241f32,
    0.474697f32,
    0.126313f32,
    0.741206f32,
    0.314796f32,
    0.552712f32,
    0.299410f32,
];
static mut av1_tx_split_nn_weights_32x64_layer1: [core::ffi::c_float; 32] = [
    1.033823f32,
    0.603439f32,
    0.304591f32,
    -0.279940f32,
    -0.780909f32,
    -0.132801f32,
    0.154059f32,
    0.662014f32,
    -0.718368f32,
    0.198733f32,
    0.039766f32,
    -0.208516f32,
    -0.104909f32,
    -0.394209f32,
    0.081617f32,
    0.365041f32,
    -0.874960f32,
    -0.063315f32,
    -1.189897f32,
    0.337225f32,
    0.410893f32,
    0.307519f32,
    0.221323f32,
    0.233895f32,
    0.469536f32,
    0.438557f32,
    0.280144f32,
    0.422423f32,
    -1.394513f32,
    0.781900f32,
    0.352981f32,
    0.111265f32,
];
static mut av1_tx_split_nn_bias_32x64_layer1: [core::ffi::c_float; 1] = [-0.18160765f32];
static mut av1_tx_split_nnconfig_32x64: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [32 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_32x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_32x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_32x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_32x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_8x32_layer0: [core::ffi::c_float; 192] = [
    -0.687846f32,
    0.121404f32,
    -0.372905f32,
    0.126770f32,
    -0.103298f32,
    -0.101650f32,
    -0.148490f32,
    -0.271740f32,
    0.682915f32,
    -0.079765f32,
    0.634347f32,
    -0.151503f32,
    0.287692f32,
    -0.079072f32,
    -0.236948f32,
    0.065064f32,
    0.713383f32,
    0.397123f32,
    0.553621f32,
    0.368529f32,
    0.767663f32,
    -0.046601f32,
    -0.392402f32,
    -0.294822f32,
    -0.292325f32,
    -0.010573f32,
    -0.837945f32,
    0.050113f32,
    -0.811360f32,
    0.199162f32,
    0.150832f32,
    0.011602f32,
    0.369694f32,
    -0.225876f32,
    0.234113f32,
    -0.269808f32,
    0.303805f32,
    -0.190281f32,
    -0.451136f32,
    0.209755f32,
    -0.308894f32,
    0.326956f32,
    0.313591f32,
    0.089923f32,
    -0.095754f32,
    0.390981f32,
    0.467366f32,
    0.169670f32,
    0.853322f32,
    0.054055f32,
    0.830319f32,
    -0.121918f32,
    0.262019f32,
    -0.093526f32,
    0.385558f32,
    0.419174f32,
    0.040198f32,
    -0.347030f32,
    -0.450492f32,
    -0.106764f32,
    0.487502f32,
    -0.204188f32,
    0.430374f32,
    -0.116388f32,
    0.236407f32,
    -0.157376f32,
    0.732294f32,
    -0.651387f32,
    0.347446f32,
    0.342575f32,
    0.048406f32,
    0.187657f32,
    0.434899f32,
    -0.447782f32,
    0.032728f32,
    -0.071168f32,
    -0.255327f32,
    0.104174f32,
    0.095689f32,
    -0.431743f32,
    0.725694f32,
    0.031797f32,
    0.523171f32,
    0.061801f32,
    0.469804f32,
    -0.071068f32,
    -0.059024f32,
    -0.211937f32,
    0.392134f32,
    -0.321490f32,
    0.366060f32,
    -0.427798f32,
    0.166771f32,
    0.299652f32,
    0.044660f32,
    0.205142f32,
    0.039133f32,
    -0.051835f32,
    -0.465475f32,
    0.216976f32,
    -0.341156f32,
    0.095358f32,
    0.230807f32,
    0.201674f32,
    0.279266f32,
    -0.713534f32,
    -0.091690f32,
    -0.569708f32,
    -0.119001f32,
    0.252160f32,
    -1.544578f32,
    -0.284477f32,
    0.555348f32,
    0.226471f32,
    0.347690f32,
    0.034365f32,
    0.770835f32,
    -0.241859f32,
    -0.130241f32,
    0.292936f32,
    0.396622f32,
    -0.417916f32,
    0.492224f32,
    0.125517f32,
    0.344824f32,
    0.232172f32,
    -0.432106f32,
    -0.278745f32,
    0.035069f32,
    -0.307247f32,
    -0.120760f32,
    0.170950f32,
    0.433601f32,
    0.044286f32,
    0.141463f32,
    -0.041382f32,
    0.529346f32,
    0.010868f32,
    -0.323674f32,
    0.185205f32,
    0.623459f32,
    0.232842f32,
    -0.406693f32,
    -0.142944f32,
    0.222988f32,
    0.343634f32,
    0.065401f32,
    0.002621f32,
    0.805335f32,
    -0.426926f32,
    0.279181f32,
    0.131364f32,
    0.192339f32,
    -0.402391f32,
    0.544120f32,
    -0.060618f32,
    0.467780f32,
    0.165224f32,
    -0.373131f32,
    0.002427f32,
    0.688064f32,
    0.322317f32,
    0.259713f32,
    0.130583f32,
    0.185032f32,
    -0.189111f32,
    -0.067821f32,
    0.010875f32,
    0.644724f32,
    -0.179291f32,
    0.463222f32,
    0.155230f32,
    0.721384f32,
    -0.046019f32,
    0.438501f32,
    0.440027f32,
    -0.462090f32,
    -0.002039f32,
    -0.468026f32,
    -0.008890f32,
    -0.328530f32,
    0.370102f32,
    0.482531f32,
    0.043471f32,
    -0.469732f32,
    -0.532663f32,
    0.122081f32,
    -0.379659f32,
    0.037219f32,
    -0.519913f32,
    -0.128975f32,
    -0.404365f32,
];
static mut av1_tx_split_nn_bias_8x32_layer0: [core::ffi::c_float; 24] = [
    -1.198965f32,
    0.395204f32,
    -0.408627f32,
    -0.021654f32,
    -0.658355f32,
    0.154525f32,
    -0.288354f32,
    1.207574f32,
    0.411608f32,
    0.964678f32,
    -1.176893f32,
    1.059006f32,
    -0.472969f32,
    2.087975f32,
    1.065536f32,
    0.595569f32,
    0.197907f32,
    -0.349938f32,
    1.013651f32,
    -0.931093f32,
    -0.973595f32,
    -0.459094f32,
    -1.253062f32,
    1.624782f32,
];
static mut av1_tx_split_nn_weights_8x32_layer1: [core::ffi::c_float; 24] = [
    0.815787f32,
    -0.393465f32,
    -0.483427f32,
    -0.565592f32,
    0.493494f32,
    0.430229f32,
    -0.507073f32,
    -0.251379f32,
    -0.353418f32,
    -0.495445f32,
    0.820029f32,
    0.649146f32,
    -0.487383f32,
    1.844503f32,
    0.480324f32,
    -0.982705f32,
    -0.501446f32,
    -0.220584f32,
    0.334299f32,
    0.802238f32,
    0.805838f32,
    -0.487848f32,
    0.300772f32,
    -1.232857f32,
];
static mut av1_tx_split_nn_bias_8x32_layer1: [core::ffi::c_float; 1] = [0.13435879f32];
static mut av1_tx_split_nnconfig_8x32: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [24 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_8x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x32_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_8x32_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nn_weights_16x64_layer0: [core::ffi::c_float; 128] = [
    -0.378223f32,
    -0.124216f32,
    -0.514089f32,
    -0.110117f32,
    -0.585801f32,
    -0.094838f32,
    -0.455385f32,
    -0.220254f32,
    -0.504568f32,
    -0.082351f32,
    -0.476420f32,
    -0.253993f32,
    -0.454709f32,
    -0.059461f32,
    0.210313f32,
    -0.155683f32,
    0.192968f32,
    -0.127804f32,
    0.471996f32,
    0.253377f32,
    0.472625f32,
    0.485322f32,
    0.150560f32,
    0.164868f32,
    -0.475587f32,
    0.447559f32,
    -0.455759f32,
    -0.306665f32,
    -0.194866f32,
    -0.283716f32,
    -0.243897f32,
    0.293020f32,
    -0.308298f32,
    -0.191904f32,
    -0.468568f32,
    0.014053f32,
    -0.618848f32,
    0.096273f32,
    -0.444586f32,
    0.347750f32,
    -0.280643f32,
    -0.062872f32,
    0.118661f32,
    0.540099f32,
    0.104141f32,
    -0.279300f32,
    -0.098721f32,
    -0.173427f32,
    -0.984558f32,
    -0.424559f32,
    -0.411928f32,
    -0.120875f32,
    -0.488999f32,
    -0.050716f32,
    -0.523103f32,
    0.093620f32,
    -0.930396f32,
    -0.431997f32,
    -1.163297f32,
    0.190384f32,
    -0.422581f32,
    -0.005354f32,
    0.450552f32,
    0.369210f32,
    0.562484f32,
    0.679922f32,
    0.282099f32,
    -0.039075f32,
    0.404196f32,
    0.006371f32,
    0.069679f32,
    -0.196160f32,
    -0.213675f32,
    0.275187f32,
    -0.104235f32,
    -0.193090f32,
    0.003116f32,
    -0.252454f32,
    -0.094591f32,
    0.210439f32,
    -0.137070f32,
    0.145043f32,
    0.024558f32,
    0.121718f32,
    0.010138f32,
    0.301651f32,
    -0.377990f32,
    0.444414f32,
    0.001845f32,
    -0.095334f32,
    0.550259f32,
    0.087603f32,
    0.792492f32,
    -0.044584f32,
    0.641706f32,
    -0.328458f32,
    -0.447791f32,
    0.135376f32,
    0.356385f32,
    0.135748f32,
    0.310370f32,
    0.293757f32,
    -0.062000f32,
    -0.056368f32,
    0.343930f32,
    0.312039f32,
    0.370763f32,
    0.452381f32,
    -0.023630f32,
    -0.185909f32,
    0.422277f32,
    -0.006306f32,
    0.045166f32,
    0.423359f32,
    -0.157735f32,
    -0.084901f32,
    0.219527f32,
    -0.209510f32,
    0.575057f32,
    0.249276f32,
    0.069267f32,
    0.233898f32,
    -0.229392f32,
    0.117197f32,
    -0.038551f32,
    0.293976f32,
    0.101996f32,
    0.120878f32,
];
static mut av1_tx_split_nn_bias_16x64_layer0: [core::ffi::c_float; 16] = [
    1.036995f32,
    0.160249f32,
    0.100264f32,
    0.694881f32,
    0.694677f32,
    0.128379f32,
    -0.843405f32,
    -0.405515f32,
    0.104139f32,
    0.182980f32,
    -0.025472f32,
    0.901067f32,
    -0.299866f32,
    -0.103079f32,
    -0.190352f32,
    -0.048121f32,
];
static mut av1_tx_split_nn_weights_16x64_layer1: [core::ffi::c_float; 16] = [
    -1.778868f32,
    0.174690f32,
    0.211991f32,
    0.712138f32,
    0.589352f32,
    0.466652f32,
    1.029146f32,
    -0.490044f32,
    0.483015f32,
    0.600215f32,
    -0.577776f32,
    -0.755546f32,
    0.348337f32,
    -0.205082f32,
    0.347129f32,
    -0.322277f32,
];
static mut av1_tx_split_nn_bias_16x64_layer1: [core::ffi::c_float; 1] = [0.04230947f32];
static mut av1_tx_split_nnconfig_16x64: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: 8 as core::ffi::c_int,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: 1 as core::ffi::c_int,
            num_hidden_nodes: [16 as core::ffi::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_weights_16x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x64_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_tx_split_nn_bias_16x64_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_tx_split_nnconfig_map: [*const NN_CONFIG; 19] = unsafe {
    [
        0 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_8x8 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_16x16 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_32x32 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_64x64 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_4x8 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_4x8 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_8x16 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_8x16 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_16x32 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_16x32 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_32x64 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_32x64 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_4x16 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_4x16 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_8x32 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_8x32 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_16x64 as *const NN_CONFIG,
        &raw const av1_tx_split_nnconfig_16x64 as *const NN_CONFIG,
    ]
};
pub const NUM_INTRA_TX_SPLIT_FEATURES: core::ffi::c_int = 14 as core::ffi::c_int;
pub const NUM_INTRA_TX_SPLIT_HIDDEN_LAYERS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const NUM_INTRA_TX_SPLIT_HIDDEN_NODES: core::ffi::c_int = 16 as core::ffi::c_int;
static mut av1_intra_tx_split_8x8_mean: [core::ffi::c_float; 14] = [
    0.110706f32,
    18.901518f32,
    0.250436f32,
    13.483487f32,
    0.118141f32,
    14.318728f32,
    0.028409f32,
    14.257664f32,
    0.045839f32,
    15.143358f32,
    9.702971f32,
    14.300809f32,
    6.018646f32,
    3.682534f32,
];
static mut av1_intra_tx_split_8x8_std: [core::ffi::c_float; 14] = [
    13.750575f32,
    13.440116f32,
    14.334330f32,
    12.236641f32,
    18.415247f32,
    12.733355f32,
    18.309339f32,
    12.858130f32,
    23.465142f32,
    13.447014f32,
    8.625048f32,
    10.456774f32,
    1.185447f32,
    1.810423f32,
];
static mut av1_intra_tx_split_nn_weights_8x8_layer0: [core::ffi::c_float; 224] = [
    -0.156142f32,
    -0.753623f32,
    0.026883f32,
    0.039188f32,
    -0.035310f32,
    0.106140f32,
    0.051622f32,
    0.077838f32,
    0.101632f32,
    0.107278f32,
    0.232200f32,
    0.269083f32,
    0.048966f32,
    -1.553293f32,
    -0.113983f32,
    -0.151248f32,
    -0.067369f32,
    0.787292f32,
    0.076651f32,
    -0.802634f32,
    0.266414f32,
    1.107563f32,
    -0.068848f32,
    -0.956468f32,
    -0.074920f32,
    -0.192258f32,
    0.006207f32,
    0.176196f32,
    -0.493442f32,
    0.152290f32,
    -0.208874f32,
    -0.014658f32,
    0.297385f32,
    -0.351695f32,
    0.246295f32,
    -0.178519f32,
    -0.204191f32,
    0.049663f32,
    -0.330343f32,
    -0.299754f32,
    0.246215f32,
    -0.014558f32,
    -0.117611f32,
    0.206445f32,
    0.045840f32,
    -0.047563f32,
    -0.049679f32,
    0.406892f32,
    -0.052307f32,
    -1.513404f32,
    0.166166f32,
    0.520760f32,
    -0.143320f32,
    -0.593928f32,
    -0.010533f32,
    0.250752f32,
    0.076738f32,
    0.537512f32,
    -0.082619f32,
    -1.534031f32,
    0.047109f32,
    0.634247f32,
    -0.089730f32,
    0.545534f32,
    -0.022742f32,
    -0.779047f32,
    -0.606358f32,
    -0.199145f32,
    -0.051269f32,
    0.248784f32,
    0.327545f32,
    -0.851751f32,
    0.071739f32,
    0.035975f32,
    0.387781f32,
    -0.136427f32,
    -0.284436f32,
    0.578449f32,
    -0.198276f32,
    0.579950f32,
    0.600111f32,
    -0.370164f32,
    -0.215297f32,
    0.517342f32,
    0.200061f32,
    -2.507660f32,
    -0.030851f32,
    0.227315f32,
    -0.078289f32,
    0.276052f32,
    -0.050281f32,
    0.251481f32,
    -0.139318f32,
    0.281175f32,
    0.226524f32,
    0.058968f32,
    0.197436f32,
    0.517294f32,
    -0.105914f32,
    -1.599567f32,
    0.064985f32,
    0.043209f32,
    -0.280038f32,
    0.126874f32,
    0.330387f32,
    -0.014407f32,
    0.031241f32,
    0.237801f32,
    0.948959f32,
    -0.253791f32,
    -0.022622f32,
    -0.061430f32,
    0.265852f32,
    0.750823f32,
    0.086606f32,
    0.853527f32,
    -0.180971f32,
    -1.255744f32,
    -0.152979f32,
    -1.022198f32,
    -0.044708f32,
    0.506424f32,
    -0.501968f32,
    -0.416863f32,
    -0.012688f32,
    0.193523f32,
    -0.093698f32,
    0.430875f32,
    0.007379f32,
    0.019278f32,
    0.080890f32,
    0.462755f32,
    -0.054326f32,
    -0.157611f32,
    -0.004851f32,
    -1.275676f32,
    -0.060528f32,
    -0.508170f32,
    0.195429f32,
    -0.023534f32,
    0.355211f32,
    0.983561f32,
    -0.122036f32,
    -0.911948f32,
    -0.172280f32,
    -1.135245f32,
    -0.043211f32,
    0.576456f32,
    -0.075247f32,
    0.429734f32,
    -0.246309f32,
    -0.355575f32,
    -0.048809f32,
    0.217113f32,
    0.078385f32,
    0.720341f32,
    0.007070f32,
    0.144617f32,
    -0.167642f32,
    0.303056f32,
    -0.031425f32,
    0.123448f32,
    -0.320530f32,
    0.164070f32,
    -0.497849f32,
    -0.233918f32,
    -0.032123f32,
    0.084983f32,
    0.312216f32,
    0.062609f32,
    -0.389815f32,
    0.237593f32,
    0.000157f32,
    -0.642068f32,
    0.167898f32,
    0.495234f32,
    -0.083493f32,
    -0.555971f32,
    0.124437f32,
    0.381125f32,
    -0.459219f32,
    0.047924f32,
    -0.138222f32,
    -2.232816f32,
    0.127585f32,
    -0.102420f32,
    0.131598f32,
    0.036837f32,
    -0.163055f32,
    -0.067429f32,
    -0.078521f32,
    -0.055666f32,
    1.387057f32,
    0.400154f32,
    -0.003355f32,
    -0.073627f32,
    -0.305098f32,
    -0.413383f32,
    -0.008266f32,
    -0.038329f32,
    0.209808f32,
    0.375777f32,
    0.037274f32,
    -0.050226f32,
    -0.100576f32,
    0.237441f32,
    0.237854f32,
    0.828296f32,
    0.001149f32,
    -0.093964f32,
    0.214051f32,
    -0.031486f32,
    -0.561307f32,
    0.014540f32,
    0.169357f32,
    0.323202f32,
    -0.395334f32,
    -0.038941f32,
    0.476800f32,
    -0.213122f32,
    -0.287521f32,
    -0.420717f32,
    -0.054142f32,
    -0.102266f32,
];
static mut av1_intra_tx_split_nn_bias_8x8_layer0: [core::ffi::c_float; 16] = [
    -1.150850f32,
    -0.236404f32,
    0.184554f32,
    -0.904162f32,
    -0.949979f32,
    0.427016f32,
    -0.546867f32,
    -0.611094f32,
    -0.676570f32,
    -0.208959f32,
    -0.286384f32,
    0.562238f32,
    0.434197f32,
    -0.746518f32,
    0.123085f32,
    -0.549836f32,
];
static mut av1_intra_tx_split_nn_weights_8x8_layer1: [core::ffi::c_float; 16] = [
    0.749814f32,
    0.598172f32,
    0.375611f32,
    0.751612f32,
    0.947538f32,
    -0.282228f32,
    -1.457522f32,
    -1.092290f32,
    0.738657f32,
    0.575779f32,
    0.514823f32,
    -0.560616f32,
    -0.491619f32,
    -1.482014f32,
    0.524625f32,
    -0.533590f32,
];
static mut av1_intra_tx_split_nn_bias_8x8_layer1: [core::ffi::c_float; 1] = [
    -0.488888f32,
];
static mut av1_intra_tx_split_nnconfig_8x8: NN_CONFIG = unsafe {
    {
        let mut init = NN_CONFIG {
            num_inputs: NUM_INTRA_TX_SPLIT_FEATURES,
            num_outputs: 1 as core::ffi::c_int,
            num_hidden_layers: NUM_INTRA_TX_SPLIT_HIDDEN_LAYERS,
            num_hidden_nodes: [
                NUM_INTRA_TX_SPLIT_HIDDEN_NODES,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            weights: [
                core::ptr::addr_of!(av1_intra_tx_split_nn_weights_8x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_intra_tx_split_nn_weights_8x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
            bias: [
                core::ptr::addr_of!(av1_intra_tx_split_nn_bias_8x8_layer0).cast::<core::ffi::c_float>(),
                core::ptr::addr_of!(av1_intra_tx_split_nn_bias_8x8_layer1).cast::<core::ffi::c_float>(),
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
                0 as *const core::ffi::c_float,
            ],
        };
        init
    }
};
static mut av1_intra_tx_prune_nn_thresh_8x8: [core::ffi::c_float; 2] = [
    -0.405465f32,
    0.405465f32,
];
#[inline]
unsafe extern "C-unwind" fn tx_size_cost(
    x: *const MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    if !(bsize as core::ffi::c_int
        == (**((*x).e_mbd.mi).offset(0 as core::ffi::c_int as isize)).bsize
            as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"tx_size_cost\0"))
                .as_ptr(),
            b"tx_search.h\0" as *const u8 as *const core::ffi::c_char,
            38 as core::ffi::c_int,
            b"bsize == x->e_mbd.mi[0]->bsize\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*x).txfm_search_params.tx_mode_search_type as core::ffi::c_int
        != TX_MODE_SELECT as core::ffi::c_int || block_signals_txsize(bsize) == 0
    {
        return 0 as core::ffi::c_int;
    }
    let tx_size_cat: int32_t = bsize_to_tx_size_cat(bsize) as int32_t;
    let depth: core::ffi::c_int = tx_size_to_depth(tx_size, bsize) as core::ffi::c_int;
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let tx_size_ctx: core::ffi::c_int = get_tx_size_context(xd) as core::ffi::c_int;
    return (*x)
        .mode_costs
        .tx_size_cost[tx_size_cat as usize][tx_size_ctx as usize][depth as usize];
}
static mut c2rust_aom_float_table_8: [core::ffi::c_float; 14] = [
    0.00110f32,
    0.00525f32,
    0.01208f32,
    0.01990f32,
    0.02795f32,
    0.03601f32,
    0.04358f32,
    0.05115f32,
    0.05896f32,
    0.06702f32,
    0.07629f32,
    0.08752f32,
    0.10217f32,
    0.12610f32,
];
pub const PROB_THRESH_OFFSET_TX_TYPE: core::ffi::c_int = 100 as core::ffi::c_int;
static mut skip_pred_threshold: [[uint32_t; 22]; 3] = [
    [
        64 as core::ffi::c_int as uint32_t,
        64 as core::ffi::c_int as uint32_t,
        64 as core::ffi::c_int as uint32_t,
        70 as core::ffi::c_int as uint32_t,
        60 as core::ffi::c_int as uint32_t,
        60 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        64 as core::ffi::c_int as uint32_t,
        64 as core::ffi::c_int as uint32_t,
        70 as core::ffi::c_int as uint32_t,
        70 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
    ],
    [
        88 as core::ffi::c_int as uint32_t,
        88 as core::ffi::c_int as uint32_t,
        88 as core::ffi::c_int as uint32_t,
        86 as core::ffi::c_int as uint32_t,
        87 as core::ffi::c_int as uint32_t,
        87 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        88 as core::ffi::c_int as uint32_t,
        88 as core::ffi::c_int as uint32_t,
        86 as core::ffi::c_int as uint32_t,
        86 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
        68 as core::ffi::c_int as uint32_t,
    ],
    [
        90 as core::ffi::c_int as uint32_t,
        93 as core::ffi::c_int as uint32_t,
        93 as core::ffi::c_int as uint32_t,
        90 as core::ffi::c_int as uint32_t,
        93 as core::ffi::c_int as uint32_t,
        93 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        90 as core::ffi::c_int as uint32_t,
        90 as core::ffi::c_int as uint32_t,
        90 as core::ffi::c_int as uint32_t,
        90 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
        74 as core::ffi::c_int as uint32_t,
    ],
];
static mut max_predict_sf_tx_size: [TX_SIZE; 22] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_4X16 as core::ffi::c_int as TX_SIZE,
    TX_16X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
];
static mut sqrt_tx_pixels_2d: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    23 as core::ffi::c_int,
    23 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    23 as core::ffi::c_int,
    23 as core::ffi::c_int,
];
static mut num_inter_tx_no_split_cand: [core::ffi::c_int; 2] = [
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
#[inline]
unsafe extern "C-unwind" fn get_block_residue_hash(
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
) -> uint32_t {
    let rows: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let cols: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let mut diff: *const int16_t = (*x).plane[0 as core::ffi::c_int as usize].src_diff;
    let hash: uint32_t = av1_get_crc32c_value_c(
        &raw mut (*(*x).txfm_search_info.mb_rd_record).crc_calculator as *mut CRC32C
            as *mut core::ffi::c_void,
        diff as *mut uint8_t,
        (2 as core::ffi::c_int * rows * cols) as size_t,
    ) as uint32_t;
    return (hash << 5 as core::ffi::c_int).wrapping_add(bsize as uint32_t);
}
#[inline]
unsafe extern "C-unwind" fn find_mb_rd_info(
    mb_rd_record: *const MB_RD_RECORD,
    ref_best_rd: int64_t,
    hash: uint32_t,
) -> int32_t {
    let mut match_index: int32_t = -(1 as int32_t);
    if ref_best_rd != INT64_MAX as int64_t {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*mb_rd_record).num {
            let index: core::ffi::c_int = ((*mb_rd_record).index_start + i)
                % RD_RECORD_BUFFER_LEN;
            if (*mb_rd_record).mb_rd_info[index as usize].hash_value == hash {
                match_index = index as int32_t;
                break;
            } else {
                i += 1;
            }
        }
    }
    return match_index;
}
#[inline]
unsafe extern "C-unwind" fn fetch_mb_rd_info(
    mut n4: core::ffi::c_int,
    mb_rd_info: *const MB_RD_INFO,
    rd_stats: *mut RD_STATS,
    x: *mut MACROBLOCK,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).tx_size = (*mb_rd_info).tx_size;
    memcpy(
        ((*mbmi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mb_rd_info).inter_tx_size).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
    );
    memcpy(
        (*xd).tx_type_map as *mut core::ffi::c_void,
        ((*mb_rd_info).tx_type_map).as_ptr() as *const core::ffi::c_void,
        (n4 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    *rd_stats = (*mb_rd_info).rd_stats;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_pixel_diff_dist(
    mut x: *const MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    plane_bsize: BLOCK_SIZE,
    tx_bsize: BLOCK_SIZE,
    mut block_mse_q8: *mut core::ffi::c_uint,
) -> int64_t {
    let mut visible_rows: core::ffi::c_int = 0;
    let mut visible_cols: core::ffi::c_int = 0;
    let txb_cols: core::ffi::c_int = block_size_wide[tx_bsize as usize]
        as core::ffi::c_int;
    let txb_rows: core::ffi::c_int = block_size_high[tx_bsize as usize]
        as core::ffi::c_int;
    get_visible_dimensions(
        x,
        plane,
        plane_bsize,
        blk_col,
        blk_row,
        txb_cols,
        txb_rows,
        true_0 != 0,
        &raw mut visible_cols,
        &raw mut visible_rows,
    );
    let mut sse: uint64_t = 0 as uint64_t;
    if visible_cols > 0 as core::ffi::c_int && visible_rows > 0 as core::ffi::c_int {
        let diff_stride: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let mut diff: *const int16_t = (*x).plane[plane as usize].src_diff;
        diff = diff.offset((blk_row * diff_stride + blk_col << MI_SIZE_LOG2) as isize);
        sse = aom_sum_squares_2d_i16_c(diff, diff_stride, visible_cols, visible_rows);
        if !block_mse_q8.is_null() {
            *block_mse_q8 = (256 as uint64_t)
                .wrapping_mul(sse)
                .wrapping_div((visible_cols * visible_rows) as uint64_t)
                as core::ffi::c_uint;
        }
    } else if !block_mse_q8.is_null() {
        *block_mse_q8 = 0 as core::ffi::c_uint;
    }
    return sse as int64_t;
}
#[inline]
unsafe extern "C-unwind" fn pixel_diff_stats(
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    plane_bsize: BLOCK_SIZE,
    tx_bsize: BLOCK_SIZE,
    mut block_mse_q8: *mut core::ffi::c_uint,
    mut per_px_mean: *mut int64_t,
    mut block_var: *mut uint64_t,
) -> int64_t {
    let mut visible_rows: core::ffi::c_int = 0;
    let mut visible_cols: core::ffi::c_int = 0;
    let txb_cols: core::ffi::c_int = block_size_wide[tx_bsize as usize]
        as core::ffi::c_int;
    let txb_rows: core::ffi::c_int = block_size_high[tx_bsize as usize]
        as core::ffi::c_int;
    get_visible_dimensions(
        x,
        plane,
        plane_bsize,
        blk_col,
        blk_row,
        txb_cols,
        txb_rows,
        true_0 != 0,
        &raw mut visible_cols,
        &raw mut visible_rows,
    );
    let mut sse: uint64_t = 0 as uint64_t;
    if visible_cols > 0 as core::ffi::c_int && visible_rows > 0 as core::ffi::c_int {
        let diff_stride: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let mut diff: *const int16_t = (*x).plane[plane as usize].src_diff;
        diff = diff.offset((blk_row * diff_stride + blk_col << MI_SIZE_LOG2) as isize);
        let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
        sse = aom_sum_sse_2d_i16_c(
            diff,
            diff_stride,
            visible_cols,
            visible_rows,
            &raw mut sum,
        );
        let mut norm_factor: core::ffi::c_double = 1.0f64
            / (visible_cols * visible_rows) as core::ffi::c_double;
        let mut sign_sum: core::ffi::c_int = if sum > 0 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            -(1 as core::ffi::c_int)
        };
        *per_px_mean = ((norm_factor * abs(sum) as core::ffi::c_double) as int64_t)
            << 7 as core::ffi::c_int;
        *per_px_mean = sign_sum as int64_t * *per_px_mean;
        *block_mse_q8 = (norm_factor
            * (256 as uint64_t).wrapping_mul(sse) as core::ffi::c_double)
            as core::ffi::c_uint;
        *block_var = sse
            .wrapping_sub(
                (norm_factor * sum as core::ffi::c_double * sum as core::ffi::c_double)
                    as uint64_t,
            );
    } else {
        *block_mse_q8 = 0 as core::ffi::c_uint;
        *block_var = 0 as uint64_t;
        *per_px_mean = 0 as int64_t;
    }
    return sse as int64_t;
}
unsafe extern "C-unwind" fn predict_skip_txfm(
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut dist: *mut int64_t,
    mut reduced_tx_set: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let mut xd: *const MACROBLOCKD = &raw mut (*x).e_mbd;
    let dc_q: int16_t = av1_dc_quant_QTX(
        (*x).qindex,
        0 as core::ffi::c_int,
        (*xd).bd as aom_bit_depth_t,
    ) as int16_t;
    *dist = av1_pixel_diff_dist(
        x,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bsize,
        bsize,
        0 as *mut core::ffi::c_uint,
    );
    let mse: int64_t = *dist / bw as int64_t / bh as int64_t;
    let normalized_dc_q: int16_t = (dc_q as core::ffi::c_int >> 3 as core::ffi::c_int)
        as int16_t;
    let mse_thresh: int64_t = normalized_dc_q as int64_t * normalized_dc_q as int64_t
        / 8 as int64_t;
    let mut pred_err: int64_t = if (*txfm_params).skip_txfm_level
        >= 2 as core::ffi::c_uint
    {
        *dist
    } else {
        mse
    };
    if pred_err > mse_thresh {
        return 0 as core::ffi::c_int
    } else if (*txfm_params).skip_txfm_level >= 2 as core::ffi::c_uint {
        return 1 as core::ffi::c_int
    }
    let max_tx_size: core::ffi::c_int = max_predict_sf_tx_size[bsize as usize]
        as core::ffi::c_int;
    let tx_h: core::ffi::c_int = tx_size_high[max_tx_size as usize];
    let tx_w: core::ffi::c_int = tx_size_wide[max_tx_size as usize];
    let mut coefs: crate::platform::Aligned32<[tran_low_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut param: TxfmParam = txfm_param {
        tx_type: 0,
        tx_size: 0,
        lossless: 0,
        bd: 0,
        is_hbd: 0,
        tx_set_type: 0,
        eob: 0,
    };
    param.tx_type = DCT_DCT as core::ffi::c_int as TX_TYPE;
    param.tx_size = max_tx_size as TX_SIZE;
    param.bd = (*xd).bd;
    param.is_hbd = is_cur_buf_hbd(xd);
    param.lossless = 0 as core::ffi::c_int;
    param.tx_set_type = av1_get_ext_tx_set_type(
        param.tx_size,
        is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)),
        reduced_tx_set,
    );
    let bd_idx: core::ffi::c_int = if (*xd).bd == 8 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else if (*xd).bd == 10 as core::ffi::c_int {
        1 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    let max_qcoef_thresh: uint32_t = skip_pred_threshold[bd_idx
        as usize][bsize as usize];
    let mut src_diff: *const int16_t = (*x)
        .plane[0 as core::ffi::c_int as usize]
        .src_diff;
    let n_coeff: core::ffi::c_int = tx_w * tx_h;
    let ac_q: int16_t = av1_ac_quant_QTX(
        (*x).qindex,
        0 as core::ffi::c_int,
        (*xd).bd as aom_bit_depth_t,
    ) as int16_t;
    let dc_thresh: uint32_t = max_qcoef_thresh.wrapping_mul(dc_q as uint32_t);
    let ac_thresh: uint32_t = max_qcoef_thresh.wrapping_mul(ac_q as uint32_t);
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < bh {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < bw {
            av1_fwd_txfm(
                src_diff.offset(col as isize),
                coefs.0.as_mut_ptr(),
                bw,
                &raw mut param,
            );
            let dc_coef: uint32_t = (abs(coefs.0[0 as core::ffi::c_int as usize])
                as uint32_t) << 7 as core::ffi::c_int;
            if dc_coef >= dc_thresh {
                return 0 as core::ffi::c_int;
            }
            let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
            while i < n_coeff {
                let ac_coef: uint32_t = (abs(coefs.0[i as usize]) as uint32_t)
                    << 7 as core::ffi::c_int;
                if ac_coef >= ac_thresh {
                    return 0 as core::ffi::c_int;
                }
                i += 1;
            }
            col += tx_w;
        }
        src_diff = src_diff.offset((tx_h * bw) as isize);
        row += tx_h;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_skip_txfm(
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut dist: int64_t,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let n4: core::ffi::c_int = bsize_to_num_blk(bsize) as core::ffi::c_int;
    let tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    memset(
        (*xd).tx_type_map as *mut core::ffi::c_void,
        DCT_DCT as core::ffi::c_int,
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(n4 as size_t),
    );
    memset(
        ((*mbmi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
        tx_size as core::ffi::c_int,
        ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
    );
    (*mbmi).tx_size = tx_size;
    (*rd_stats).skip_txfm = 1 as uint8_t;
    if is_cur_buf_hbd(xd) != 0 {
        dist = dist
            + ((1 as core::ffi::c_int)
                << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                >> 1 as core::ffi::c_int) as int64_t
            >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int;
    }
    (*rd_stats).sse = dist << 4 as core::ffi::c_int;
    (*rd_stats).dist = (*rd_stats).sse;
    let mut ctxa: [ENTROPY_CONTEXT; 32] = [0; 32];
    let mut ctxl: [ENTROPY_CONTEXT; 32] = [0; 32];
    av1_get_entropy_contexts(
        bsize,
        &raw mut *((*xd).plane).as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        ctxa.as_mut_ptr(),
        ctxl.as_mut_ptr(),
    );
    let mut ta: *mut ENTROPY_CONTEXT = ctxa.as_mut_ptr();
    let mut tl: *mut ENTROPY_CONTEXT = ctxl.as_mut_ptr();
    let txs_ctx: TX_SIZE = get_txsize_entropy_ctx(tx_size) as TX_SIZE;
    let mut txb_ctx: TXB_CTX = txb_ctx {
        txb_skip_ctx: 0,
        dc_sign_ctx: 0,
    };
    get_txb_ctx(bsize, tx_size, 0 as core::ffi::c_int, ta, tl, &raw mut txb_ctx);
    let zero_blk_rate: core::ffi::c_int = (*x)
        .coeff_costs
        .coeff_costs[txs_ctx as usize][PLANE_TYPE_Y as core::ffi::c_int as usize]
        .txb_skip_cost[txb_ctx.txb_skip_ctx as usize][1 as core::ffi::c_int as usize];
    (*rd_stats).rate = zero_blk_rate
        * (block_size_wide[bsize as usize] as core::ffi::c_int
            >> tx_size_wide_log2[tx_size as usize])
        * (block_size_high[bsize as usize] as core::ffi::c_int
            >> tx_size_high_log2[tx_size as usize]);
}
#[inline]
unsafe extern "C-unwind" fn save_mb_rd_info(
    mut n4: core::ffi::c_int,
    mut hash: uint32_t,
    x: *const MACROBLOCK,
    rd_stats: *const RD_STATS,
    mut mb_rd_record: *mut MB_RD_RECORD,
) {
    let mut index: core::ffi::c_int = 0;
    if (*mb_rd_record).num < RD_RECORD_BUFFER_LEN {
        index = ((*mb_rd_record).index_start + (*mb_rd_record).num)
            % RD_RECORD_BUFFER_LEN;
        (*mb_rd_record).num += 1;
    } else {
        index = (*mb_rd_record).index_start;
        (*mb_rd_record).index_start = ((*mb_rd_record).index_start
            + 1 as core::ffi::c_int) % RD_RECORD_BUFFER_LEN;
    }
    let mb_rd_info: *mut MB_RD_INFO = &raw mut *((*mb_rd_record).mb_rd_info)
        .as_mut_ptr()
        .offset(index as isize) as *mut MB_RD_INFO;
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mb_rd_info).hash_value = hash;
    (*mb_rd_info).tx_size = (*mbmi).tx_size;
    memcpy(
        ((*mb_rd_info).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mbmi).inter_tx_size).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
    );
    memcpy(
        ((*mb_rd_info).tx_type_map).as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).tx_type_map as *const core::ffi::c_void,
        (n4 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    (*mb_rd_info).rd_stats = *rd_stats;
}
#[inline]
unsafe extern "C-unwind" fn push_inter_block_tx_no_split_rd(
    mut x: *mut MACROBLOCK,
    mut mbmi: *const MB_MODE_INFO,
    mut tmp_rd: int64_t,
    mut blk_idx: core::ffi::c_int,
    mut prune_inter_tx_split_rd_eval_lvl: core::ffi::c_int,
) {
    if !(blk_idx
        < (1 as core::ffi::c_int)
            << (7 as core::ffi::c_int - 6 as core::ffi::c_int) * 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"push_inter_block_tx_no_split_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            321 as core::ffi::c_int,
            b"blk_idx < MAX_TX_BLOCKS_IN_MAX_SB\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if prune_inter_tx_split_rd_eval_lvl == 0 {
        return;
    }
    if blk_idx == -(1 as core::ffi::c_int) || tmp_rd == INT64_MAX as int64_t {
        return;
    }
    if (*mbmi).skip_mode() as core::ffi::c_int != 0 as core::ffi::c_int
        || is_intrabc_block(mbmi) != 0
    {
        return;
    }
    if !(prune_inter_tx_split_rd_eval_lvl <= 2 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"push_inter_block_tx_no_split_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            329 as core::ffi::c_int,
            b"prune_inter_tx_split_rd_eval_lvl <= 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let num_top_cand: core::ffi::c_int = num_inter_tx_no_split_cand[(prune_inter_tx_split_rd_eval_lvl
        - 1 as core::ffi::c_int) as usize];
    if !(num_top_cand <= 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"push_inter_block_tx_no_split_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            332 as core::ffi::c_int,
            b"num_top_cand <= TOP_INTER_TX_NO_SPLIT_COUNT\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_top_cand {
        if tmp_rd < (*x).top_inter_tx_no_split_rd[blk_idx as usize][i as usize] {
            let mut j: core::ffi::c_int = num_top_cand - 1 as core::ffi::c_int;
            while j > i {
                (*x).top_inter_tx_no_split_rd[blk_idx as usize][j as usize] = (*x)
                    .top_inter_tx_no_split_rd[blk_idx
                    as usize][(j - 1 as core::ffi::c_int) as usize];
                j -= 1;
            }
            (*x).top_inter_tx_no_split_rd[blk_idx as usize][i as usize] = tmp_rd;
            break;
        } else {
            i += 1;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn prune_tx_split_eval_using_no_split_rd(
    mut x: *const MACROBLOCK,
    mut mbmi: *const MB_MODE_INFO,
    mut tmp_rd: int64_t,
    mut blk_idx: core::ffi::c_int,
    mut prune_inter_tx_split_rd_eval_lvl: core::ffi::c_int,
) -> bool {
    if prune_inter_tx_split_rd_eval_lvl == 0 {
        return false_0 != 0;
    }
    if blk_idx == -(1 as core::ffi::c_int) || tmp_rd == INT64_MAX as int64_t {
        return false_0 != 0;
    }
    if (*mbmi).skip_mode() as core::ffi::c_int != 0 as core::ffi::c_int
        || is_intrabc_block(mbmi) != 0
    {
        return false_0 != 0;
    }
    if !(prune_inter_tx_split_rd_eval_lvl <= 2 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 38],
                [core::ffi::c_char; 38],
            >(*b"prune_tx_split_eval_using_no_split_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            358 as core::ffi::c_int,
            b"prune_inter_tx_split_rd_eval_lvl <= 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let num_top_cand: core::ffi::c_int = num_inter_tx_no_split_cand[(prune_inter_tx_split_rd_eval_lvl
        - 1 as core::ffi::c_int) as usize];
    if !(num_top_cand <= 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 38],
                [core::ffi::c_char; 38],
            >(*b"prune_tx_split_eval_using_no_split_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            361 as core::ffi::c_int,
            b"num_top_cand <= TOP_INTER_TX_NO_SPLIT_COUNT\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*x)
        .top_inter_tx_no_split_rd[blk_idx
        as usize][(num_top_cand - 1 as core::ffi::c_int) as usize]
        == INT64_MAX as int64_t
    {
        return false_0 != 0;
    }
    if tmp_rd
        > (*x)
            .top_inter_tx_no_split_rd[blk_idx
            as usize][(num_top_cand - 1 as core::ffi::c_int) as usize]
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn get_search_init_depth(
    mut mi_width: core::ffi::c_int,
    mut mi_height: core::ffi::c_int,
    mut is_inter: core::ffi::c_int,
    mut sf: *const SPEED_FEATURES,
    mut tx_size_search_method: core::ffi::c_int,
) -> core::ffi::c_int {
    if tx_size_search_method == USE_LARGESTALL as core::ffi::c_int {
        return MAX_VARTX_DEPTH;
    }
    if (*sf).tx_sf.tx_size_search_lgr_block != 0 {
        if mi_width
            > mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
            || mi_height
                > mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int
        {
            return MAX_VARTX_DEPTH;
        }
    }
    if is_inter != 0 {
        return if mi_height != mi_width {
            (*sf).tx_sf.inter_tx_size_search_init_depth_rect
        } else {
            (*sf).tx_sf.inter_tx_size_search_init_depth_sqr
        }
    } else {
        return if mi_height != mi_width {
            (*sf).tx_sf.intra_tx_size_search_init_depth_rect
        } else {
            (*sf).tx_sf.intra_tx_size_search_init_depth_sqr
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn inverse_transform_block_facade(
    x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut eob: core::ffi::c_int,
    mut reduced_tx_set: core::ffi::c_int,
) {
    if eob == 0 {
        return;
    }
    let p: *mut macroblock_plane = &raw mut *((*x).plane).as_mut_ptr().offset(plane as isize)
        as *mut macroblock_plane;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut dqcoeff: *mut tran_low_t = ((*p).dqcoeff)
        .offset((block << 4 as core::ffi::c_int) as isize);
    let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
    let tx_size: TX_SIZE = av1_get_tx_size(plane, xd) as TX_SIZE;
    let tx_type: TX_TYPE = av1_get_tx_type(
        xd,
        plane_type,
        blk_row,
        blk_col,
        tx_size,
        reduced_tx_set,
    ) as TX_TYPE;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let dst_stride: core::ffi::c_int = (*pd).dst.stride;
    let mut dst: *mut uint8_t = &raw mut *((*pd).dst.buf)
        .offset((blk_row * dst_stride + blk_col << MI_SIZE_LOG2) as isize)
        as *mut uint8_t;
    av1_inverse_transform_block(
        xd,
        dqcoeff,
        plane,
        tx_type,
        tx_size,
        dst,
        dst_stride,
        eob,
        reduced_tx_set,
    );
}
#[inline]
unsafe extern "C-unwind" fn recon_intra(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    txb_ctx: *const TXB_CTX,
    mut skip_trellis: core::ffi::c_int,
    mut best_tx_type: TX_TYPE,
    mut do_quant: core::ffi::c_int,
    mut rate_cost: *mut core::ffi::c_int,
    mut best_eob: uint16_t,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    if is_inter == 0 && best_eob as core::ffi::c_int != 0
        && (blk_row + tx_size_high_unit[tx_size as usize]
            < mi_size_high[plane_bsize as usize] as core::ffi::c_int
            || blk_col + tx_size_wide_unit[tx_size as usize]
                < mi_size_wide[plane_bsize as usize] as core::ffi::c_int)
    {
        if do_quant != 0 {
            let mut txfm_param_intra: TxfmParam = txfm_param {
                tx_type: 0,
                tx_size: 0,
                lossless: 0,
                bd: 0,
                is_hbd: 0,
                tx_set_type: 0,
                eob: 0,
            };
            let mut quant_param_intra: QUANT_PARAM = QUANT_PARAM {
                log_scale: 0,
                tx_size: 0,
                qmatrix: 0 as *const qm_val_t,
                iqmatrix: 0 as *const qm_val_t,
                use_quant_b_adapt: 0,
                use_optimize_b: 0,
                xform_quant_idx: 0,
            };
            av1_setup_xform(cm, x, tx_size, best_tx_type, &raw mut txfm_param_intra);
            av1_setup_quant(
                tx_size,
                (skip_trellis == 0) as core::ffi::c_int,
                if skip_trellis != 0 {
                    if USE_B_QUANT_NO_TRELLIS != 0 {
                        AV1_XFORM_QUANT_B as core::ffi::c_int
                    } else {
                        AV1_XFORM_QUANT_FP as core::ffi::c_int
                    }
                } else {
                    AV1_XFORM_QUANT_FP as core::ffi::c_int
                },
                (*cpi).oxcf.q_cfg.quant_b_adapt,
                &raw mut quant_param_intra,
            );
            av1_setup_qmatrix(
                &(*cm).quant_params,
                xd,
                plane,
                tx_size,
                best_tx_type,
                &raw mut quant_param_intra,
            );
            av1_xform_quant(
                x,
                plane,
                block,
                blk_row,
                blk_col,
                plane_bsize,
                &raw mut txfm_param_intra,
                &raw mut quant_param_intra,
            );
            if quant_param_intra.use_optimize_b != 0 {
                av1_optimize_b(
                    cpi as *const AV1_COMP,
                    x,
                    plane,
                    block,
                    tx_size,
                    best_tx_type,
                    txb_ctx,
                    rate_cost,
                );
            }
        }
        inverse_transform_block_facade(
            x,
            plane,
            block,
            blk_row,
            blk_col,
            *((*x).plane[plane as usize].eobs).offset(block as isize)
                as core::ffi::c_int,
            (*cm).features.reduced_tx_set_used as core::ffi::c_int,
        );
        if plane == 0 as core::ffi::c_int
            && *((*x).plane[plane as usize].eobs).offset(block as isize)
                as core::ffi::c_int == 0 as core::ffi::c_int
            && best_tx_type as core::ffi::c_int != DCT_DCT as core::ffi::c_int
        {
            update_txk_array(
                xd,
                blk_row,
                blk_col,
                tx_size,
                DCT_DCT as core::ffi::c_int as TX_TYPE,
            );
        }
    }
}
unsafe extern "C-unwind" fn pixel_dist(
    cpi: *const AV1_COMP,
    mut x: *const MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut src: *const uint8_t,
    src_stride: core::ffi::c_int,
    mut dst: *const uint8_t,
    dst_stride: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    plane_bsize: BLOCK_SIZE,
    tx_bsize: BLOCK_SIZE,
) -> core::ffi::c_uint {
    let mut txb_rows: core::ffi::c_int = 0;
    let mut txb_cols: core::ffi::c_int = 0;
    let mut visible_rows: core::ffi::c_int = 0;
    let mut visible_cols: core::ffi::c_int = 0;
    txb_cols = block_size_wide[tx_bsize as usize] as core::ffi::c_int;
    txb_rows = block_size_high[tx_bsize as usize] as core::ffi::c_int;
    get_visible_dimensions(
        x,
        plane,
        plane_bsize,
        blk_col,
        blk_row,
        txb_cols,
        txb_rows,
        true_0 != 0,
        &raw mut visible_cols,
        &raw mut visible_rows,
    );
    if !(visible_rows > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"pixel_dist\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            994 as core::ffi::c_int,
            b"visible_rows > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(visible_cols > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"pixel_dist\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            995 as core::ffi::c_int,
            b"visible_cols > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut sse: core::ffi::c_uint = pixel_dist_visible_only(
        cpi,
        x,
        src,
        src_stride,
        dst,
        dst_stride,
        tx_bsize,
        txb_rows,
        txb_cols,
        visible_rows,
        visible_cols,
    );
    return sse;
}
#[inline]
unsafe extern "C-unwind" fn dist_block_px_domain(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> int64_t {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let p: *const macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblock_plane;
    let eob: uint16_t = *((*p).eobs).offset(block as isize);
    let tx_bsize: BLOCK_SIZE = txsize_to_bsize[tx_size as usize];
    let bsw: core::ffi::c_int = block_size_wide[tx_bsize as usize] as core::ffi::c_int;
    let bsh: core::ffi::c_int = block_size_high[tx_bsize as usize] as core::ffi::c_int;
    let src_stride: core::ffi::c_int = (*x).plane[plane as usize].src.stride;
    let dst_stride: core::ffi::c_int = (*xd).plane[plane as usize].dst.stride;
    let src_idx: core::ffi::c_int = blk_row * src_stride + blk_col << MI_SIZE_LOG2;
    let dst_idx: core::ffi::c_int = blk_row * dst_stride + blk_col << MI_SIZE_LOG2;
    let mut src: *const uint8_t = &raw mut *((*((*x).plane)
        .as_mut_ptr()
        .offset(plane as isize))
        .src
        .buf)
        .offset(src_idx as isize) as *mut uint8_t;
    let mut dst: *const uint8_t = &raw mut *((*((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize))
        .dst
        .buf)
        .offset(dst_idx as isize) as *mut uint8_t;
    let mut dqcoeff: *const tran_low_t = ((*p).dqcoeff)
        .offset((block << 4 as core::ffi::c_int) as isize);
    if cpi.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"dist_block_px_domain\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1023 as core::ffi::c_int,
            b"cpi != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tx_size_wide_log2[0 as core::ffi::c_int as usize]
        == tx_size_high_log2[0 as core::ffi::c_int as usize]) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"dist_block_px_domain\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1024 as core::ffi::c_int,
            b"tx_size_wide_log2[0] == tx_size_high_log2[0]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut recon: *mut uint8_t = 0 as *mut uint8_t;
    let mut recon16: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    if is_cur_buf_hbd(xd) != 0 {
        recon = (recon16.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int)
            as *mut uint8_t;
        aom_highbd_convolve_copy_c(
            ((dst as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            dst_stride as ptrdiff_t,
            ((recon as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            MAX_TX_SIZE as ptrdiff_t,
            bsw,
            bsh,
        );
    } else {
        recon = recon16.0.as_mut_ptr() as *mut uint8_t;
        aom_convolve_copy_c(
            dst,
            dst_stride as ptrdiff_t,
            recon,
            MAX_TX_SIZE as ptrdiff_t,
            bsw,
            bsh,
        );
    }
    let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
    let mut tx_type: TX_TYPE = av1_get_tx_type(
        xd,
        plane_type,
        blk_row,
        blk_col,
        tx_size,
        (*cpi).common.features.reduced_tx_set_used as core::ffi::c_int,
    );
    av1_inverse_transform_block(
        xd,
        dqcoeff,
        plane,
        tx_type,
        tx_size,
        recon,
        MAX_TX_SIZE,
        eob as core::ffi::c_int,
        (*cpi).common.features.reduced_tx_set_used as core::ffi::c_int,
    );
    return (16 as core::ffi::c_uint)
        .wrapping_mul(
            pixel_dist(
                cpi,
                x,
                plane,
                src,
                src_stride,
                recon,
                MAX_TX_SIZE,
                blk_row,
                blk_col,
                plane_bsize,
                tx_bsize,
            ),
        ) as int64_t;
}
static mut prune_factors: [core::ffi::c_int; 5] = [
    200 as core::ffi::c_int,
    200 as core::ffi::c_int,
    120 as core::ffi::c_int,
    80 as core::ffi::c_int,
    40 as core::ffi::c_int,
];
static mut mul_factors: [core::ffi::c_int; 5] = [
    80 as core::ffi::c_int,
    80 as core::ffi::c_int,
    70 as core::ffi::c_int,
    50 as core::ffi::c_int,
    30 as core::ffi::c_int,
];
#[inline]
unsafe extern "C-unwind" fn sort_rd(
    mut rds: *mut int64_t,
    mut txk: *mut core::ffi::c_int,
    mut len: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    i = 1 as core::ffi::c_int;
    while i <= len - 1 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < i {
            if *rds.offset(j as isize) > *rds.offset(i as isize) {
                let mut temprd: int64_t = 0;
                let mut tempi: core::ffi::c_int = 0;
                temprd = *rds.offset(i as isize);
                tempi = *txk.offset(i as isize);
                k = i;
                while k > j {
                    *rds.offset(k as isize) = *rds
                        .offset((k - 1 as core::ffi::c_int) as isize);
                    *txk.offset(k as isize) = *txk
                        .offset((k - 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                *rds.offset(j as isize) = temprd;
                *txk.offset(j as isize) = tempi;
                break;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn av1_block_error_qm(
    mut coeff: *const tran_low_t,
    mut dqcoeff: *const tran_low_t,
    mut block_size: intptr_t,
    mut qmatrix: *const qm_val_t,
    mut scan: *const int16_t,
    mut ssz: *mut int64_t,
    mut bd: core::ffi::c_int,
) -> int64_t {
    let mut i: core::ffi::c_int = 0;
    let mut error: int64_t = 0 as int64_t;
    let mut sqcoeff: int64_t = 0 as int64_t;
    let mut shift: core::ffi::c_int = 2 as core::ffi::c_int
        * (bd - 8 as core::ffi::c_int);
    let mut rounding: core::ffi::c_int = (1 as core::ffi::c_int) << shift
        >> 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while (i as intptr_t) < block_size {
        let mut weight: int64_t = *qmatrix.offset(*scan.offset(i as isize) as isize)
            as int64_t;
        let mut dd: int64_t = (*coeff.offset(i as isize) - *dqcoeff.offset(i as isize))
            as int64_t;
        dd *= weight;
        let mut cc: int64_t = *coeff.offset(i as isize) as int64_t;
        cc *= weight;
        error
            += dd * dd
                + ((1 as core::ffi::c_int)
                    << 2 as core::ffi::c_int * AOM_QM_BITS - 1 as core::ffi::c_int)
                    as int64_t >> 2 as core::ffi::c_int * AOM_QM_BITS;
        sqcoeff
            += cc * cc
                + ((1 as core::ffi::c_int)
                    << 2 as core::ffi::c_int * AOM_QM_BITS - 1 as core::ffi::c_int)
                    as int64_t >> 2 as core::ffi::c_int * AOM_QM_BITS;
        i += 1;
    }
    error = error + rounding as int64_t >> shift;
    sqcoeff = sqcoeff + rounding as int64_t >> shift;
    *ssz = sqcoeff;
    return error;
}
#[inline]
unsafe extern "C-unwind" fn dist_block_tx_domain(
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut qmatrix: *const qm_val_t,
    mut scan: *const int16_t,
    mut out_dist: *mut int64_t,
    mut out_sse: *mut int64_t,
) {
    let p: *const macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblock_plane;
    let buffer_length: core::ffi::c_int = av1_get_max_eob(tx_size) as core::ffi::c_int;
    let mut this_sse: int64_t = 0;
    let mut shift: core::ffi::c_int = (MAX_TX_SCALE - av1_get_tx_scale(tx_size))
        * 2 as core::ffi::c_int;
    let block_offset: core::ffi::c_int = block << 4 as core::ffi::c_int;
    let coeff: *mut tran_low_t = ((*p).coeff).offset(block_offset as isize);
    let dqcoeff: *mut tran_low_t = ((*p).dqcoeff).offset(block_offset as isize);
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if is_cur_buf_hbd(xd) != 0 {
        if qmatrix.is_null() || (*x).txfm_search_params.use_qm_dist_metric == 0 {
            *out_dist = av1_highbd_block_error
                .expect(
                    "non-null function pointer",
                )(coeff, dqcoeff, buffer_length as intptr_t, &raw mut this_sse, (*xd).bd);
        } else {
            *out_dist = av1_block_error_qm(
                coeff,
                dqcoeff,
                buffer_length as intptr_t,
                qmatrix,
                scan,
                &raw mut this_sse,
                (*xd).bd,
            );
        }
    } else if qmatrix.is_null() || (*x).txfm_search_params.use_qm_dist_metric == 0 {
        *out_dist = av1_block_error
            .expect(
                "non-null function pointer",
            )(coeff, dqcoeff, buffer_length as intptr_t, &raw mut this_sse);
    } else {
        *out_dist = av1_block_error_qm(
            coeff,
            dqcoeff,
            buffer_length as intptr_t,
            qmatrix,
            scan,
            &raw mut this_sse,
            8 as core::ffi::c_int,
        );
    }
    *out_dist = if shift < 0 as core::ffi::c_int {
        *out_dist << -shift
    } else {
        *out_dist >> shift
    };
    *out_sse = if shift < 0 as core::ffi::c_int {
        this_sse << -shift
    } else {
        this_sse >> shift
    };
}
unsafe extern "C-unwind" fn prune_txk_type_separ(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut txk_map: *mut core::ffi::c_int,
    mut allowed_tx_mask: int16_t,
    mut prune_factor: core::ffi::c_int,
    txb_ctx: *const TXB_CTX,
    mut reduced_tx_set_used: core::ffi::c_int,
    mut ref_best_rd: int64_t,
    mut num_sel: core::ffi::c_int,
) -> uint16_t {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut idx: core::ffi::c_int = 0;
    let mut rds_v: [int64_t; 4] = [0; 4];
    let mut rds_h: [int64_t; 4] = [0; 4];
    let mut idx_v: [core::ffi::c_int; 4] = [
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ];
    let mut idx_h: [core::ffi::c_int; 4] = [
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ];
    let mut skip_v: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let mut skip_h: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let idx_map: [core::ffi::c_int; 16] = [
        DCT_DCT as core::ffi::c_int,
        DCT_ADST as core::ffi::c_int,
        DCT_FLIPADST as core::ffi::c_int,
        V_DCT as core::ffi::c_int,
        ADST_DCT as core::ffi::c_int,
        ADST_ADST as core::ffi::c_int,
        ADST_FLIPADST as core::ffi::c_int,
        V_ADST as core::ffi::c_int,
        FLIPADST_DCT as core::ffi::c_int,
        FLIPADST_ADST as core::ffi::c_int,
        FLIPADST_FLIPADST as core::ffi::c_int,
        V_FLIPADST as core::ffi::c_int,
        H_DCT as core::ffi::c_int,
        H_ADST as core::ffi::c_int,
        H_FLIPADST as core::ffi::c_int,
        IDTX as core::ffi::c_int,
    ];
    let sel_pattern_v: [core::ffi::c_int; 16] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        2 as core::ffi::c_int,
        0 as core::ffi::c_int,
        3 as core::ffi::c_int,
        1 as core::ffi::c_int,
        3 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ];
    let sel_pattern_h: [core::ffi::c_int; 16] = [
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        0 as core::ffi::c_int,
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        0 as core::ffi::c_int,
        3 as core::ffi::c_int,
        1 as core::ffi::c_int,
        3 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ];
    let mut quant_param: QUANT_PARAM = QUANT_PARAM {
        log_scale: 0,
        tx_size: 0,
        qmatrix: 0 as *const qm_val_t,
        iqmatrix: 0 as *const qm_val_t,
        use_quant_b_adapt: 0,
        use_optimize_b: 0,
        xform_quant_idx: 0,
    };
    let mut txfm_param: TxfmParam = txfm_param {
        tx_type: 0,
        tx_size: 0,
        lossless: 0,
        bd: 0,
        is_hbd: 0,
        tx_set_type: 0,
        eob: 0,
    };
    av1_setup_xform(
        cm,
        x,
        tx_size,
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        &raw mut txfm_param,
    );
    av1_setup_quant(
        tx_size,
        1 as core::ffi::c_int,
        AV1_XFORM_QUANT_B as core::ffi::c_int,
        (*cpi).oxcf.q_cfg.quant_b_adapt,
        &raw mut quant_param,
    );
    let mut tx_type: core::ffi::c_int = 0;
    if !(txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
        <= TX_16X16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"prune_txk_type_separ\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1195 as core::ffi::c_int,
            b"txsize_sqr_up_map[tx_size] <= TX_16X16\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    txfm_param.tx_set_type = EXT_TX_SET_ALL16 as core::ffi::c_int as TxSetType;
    let mut rate_cost: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dist: int64_t = 0 as int64_t;
    let mut sse: int64_t = 0 as int64_t;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        tx_type = idx_map[idx as usize];
        txfm_param.tx_type = tx_type as TX_TYPE;
        av1_setup_qmatrix(
            &(*cm).quant_params,
            xd,
            plane,
            tx_size,
            tx_type as TX_TYPE,
            &raw mut quant_param,
        );
        av1_xform_quant(
            x,
            plane,
            block,
            blk_row,
            blk_col,
            plane_bsize,
            &raw mut txfm_param,
            &raw mut quant_param,
        );
        let scan_order: *const SCAN_ORDER = get_scan(
            txfm_param.tx_size,
            txfm_param.tx_type,
        ) as *const SCAN_ORDER;
        dist_block_tx_domain(
            x,
            plane,
            block,
            tx_size,
            quant_param.qmatrix,
            (*scan_order).scan,
            &raw mut dist,
            &raw mut sse,
        );
        rate_cost = av1_cost_coeffs_txb_laplacian(
            x,
            plane,
            block,
            tx_size,
            tx_type as TX_TYPE,
            txb_ctx,
            reduced_tx_set_used,
            0 as core::ffi::c_int,
        );
        rds_h[idx as usize] = (rate_cost as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if rds_h[idx as usize] - (rds_h[idx as usize] >> 2 as core::ffi::c_int)
            > ref_best_rd
        {
            skip_h[idx as usize] = 1 as core::ffi::c_int;
        }
        idx += 1;
    }
    sort_rd(rds_h.as_mut_ptr(), idx_h.as_mut_ptr(), 4 as core::ffi::c_int);
    idx = 1 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        if rds_h[idx as usize] as core::ffi::c_double
            > rds_h[0 as core::ffi::c_int as usize] as core::ffi::c_double * 1.2f64
        {
            skip_h[idx_h[idx as usize] as usize] = 1 as core::ffi::c_int;
        }
        idx += 1;
    }
    if skip_h[idx_h[0 as core::ffi::c_int as usize] as usize] != 0 {
        return 0xffff as core::ffi::c_int as uint16_t;
    }
    rds_v[0 as core::ffi::c_int as usize] = rds_h[0 as core::ffi::c_int as usize];
    let mut start_v: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut end_v: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut idx_map_v: *const core::ffi::c_int = idx_map
        .as_ptr()
        .offset(idx_h[0 as core::ffi::c_int as usize] as isize);
    idx = start_v;
    while idx < end_v {
        tx_type = *idx_map_v
            .offset((idx_v[idx as usize] * 4 as core::ffi::c_int) as isize);
        txfm_param.tx_type = tx_type as TX_TYPE;
        av1_setup_qmatrix(
            &(*cm).quant_params,
            xd,
            plane,
            tx_size,
            tx_type as TX_TYPE,
            &raw mut quant_param,
        );
        av1_xform_quant(
            x,
            plane,
            block,
            blk_row,
            blk_col,
            plane_bsize,
            &raw mut txfm_param,
            &raw mut quant_param,
        );
        let scan_order_0: *const SCAN_ORDER = get_scan(
            txfm_param.tx_size,
            txfm_param.tx_type,
        ) as *const SCAN_ORDER;
        dist_block_tx_domain(
            x,
            plane,
            block,
            tx_size,
            quant_param.qmatrix,
            (*scan_order_0).scan,
            &raw mut dist,
            &raw mut sse,
        );
        rate_cost = av1_cost_coeffs_txb_laplacian(
            x,
            plane,
            block,
            tx_size,
            tx_type as TX_TYPE,
            txb_ctx,
            reduced_tx_set_used,
            0 as core::ffi::c_int,
        );
        rds_v[idx as usize] = (rate_cost as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if rds_v[idx as usize] - (rds_v[idx as usize] >> 2 as core::ffi::c_int)
            > ref_best_rd
        {
            skip_v[idx as usize] = 1 as core::ffi::c_int;
        }
        idx += 1;
    }
    sort_rd(rds_v.as_mut_ptr(), idx_v.as_mut_ptr(), 4 as core::ffi::c_int);
    idx = 1 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        if rds_v[idx as usize] as core::ffi::c_double
            > rds_v[0 as core::ffi::c_int as usize] as core::ffi::c_double * 1.2f64
        {
            skip_v[idx_v[idx as usize] as usize] = 1 as core::ffi::c_int;
        }
        idx += 1;
    }
    let mut i_v: core::ffi::c_int = 0;
    let mut i_h: core::ffi::c_int = 0;
    let mut rds: [int64_t; 16] = [0; 16];
    let mut num_cand: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut last: core::ffi::c_int = TX_TYPES as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 16 as core::ffi::c_int {
        i_v = sel_pattern_v[i as usize];
        i_h = sel_pattern_h[i as usize];
        tx_type = idx_map[(idx_v[i_v as usize] * 4 as core::ffi::c_int
            + idx_h[i_h as usize]) as usize];
        if allowed_tx_mask as core::ffi::c_int & (1 as core::ffi::c_int) << tx_type == 0
            || skip_h[idx_h[i_h as usize] as usize] != 0
            || skip_v[idx_v[i_v as usize] as usize] != 0
        {
            *txk_map.offset(last as isize) = tx_type;
            last -= 1;
        } else {
            *txk_map.offset(num_cand as isize) = tx_type;
            rds[num_cand as usize] = rds_v[i_v as usize] + rds_h[i_h as usize];
            if rds[num_cand as usize] == 0 as int64_t {
                rds[num_cand as usize] = 1 as int64_t;
            }
            num_cand += 1;
        }
        i += 1;
    }
    sort_rd(rds.as_mut_ptr(), txk_map as *mut core::ffi::c_int, num_cand);
    let mut prune: uint16_t = !((1 as core::ffi::c_int)
        << *txk_map.offset(0 as core::ffi::c_int as isize)) as uint16_t;
    num_sel = if num_sel < num_cand { num_sel } else { num_cand };
    let mut i_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while i_0 < num_sel {
        let mut factor: int64_t = 1800 as int64_t
            * (rds[i_0 as usize] - rds[0 as core::ffi::c_int as usize])
            / rds[0 as core::ffi::c_int as usize];
        if !(factor < prune_factor as int64_t) {
            break;
        }
        prune = (prune as core::ffi::c_int
            & !((1 as core::ffi::c_int) << *txk_map.offset(i_0 as isize))) as uint16_t;
        i_0 += 1;
    }
    return prune;
}
unsafe extern "C-unwind" fn prune_txk_type(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut txk_map: *mut core::ffi::c_int,
    mut allowed_tx_mask: uint16_t,
    mut prune_factor: core::ffi::c_int,
    txb_ctx: *const TXB_CTX,
    mut reduced_tx_set_used: core::ffi::c_int,
) -> uint16_t {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut tx_type: core::ffi::c_int = 0;
    let mut rds: [int64_t; 16] = [0; 16];
    let mut num_cand: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut last: core::ffi::c_int = TX_TYPES as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut txfm_param: TxfmParam = txfm_param {
        tx_type: 0,
        tx_size: 0,
        lossless: 0,
        bd: 0,
        is_hbd: 0,
        tx_set_type: 0,
        eob: 0,
    };
    let mut quant_param: QUANT_PARAM = QUANT_PARAM {
        log_scale: 0,
        tx_size: 0,
        qmatrix: 0 as *const qm_val_t,
        iqmatrix: 0 as *const qm_val_t,
        use_quant_b_adapt: 0,
        use_optimize_b: 0,
        xform_quant_idx: 0,
    };
    av1_setup_xform(
        cm,
        x,
        tx_size,
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        &raw mut txfm_param,
    );
    av1_setup_quant(
        tx_size,
        1 as core::ffi::c_int,
        AV1_XFORM_QUANT_B as core::ffi::c_int,
        (*cpi).oxcf.q_cfg.quant_b_adapt,
        &raw mut quant_param,
    );
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < TX_TYPES as core::ffi::c_int {
        tx_type = idx;
        let mut rate_cost: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut dist: int64_t = 0 as int64_t;
        let mut sse: int64_t = 0 as int64_t;
        if allowed_tx_mask as core::ffi::c_int & (1 as core::ffi::c_int) << tx_type == 0
        {
            *txk_map.offset(last as isize) = tx_type;
            last -= 1;
        } else {
            txfm_param.tx_type = tx_type as TX_TYPE;
            av1_setup_qmatrix(
                &(*cm).quant_params,
                xd,
                plane,
                tx_size,
                tx_type as TX_TYPE,
                &raw mut quant_param,
            );
            av1_xform_quant(
                x,
                plane,
                block,
                blk_row,
                blk_col,
                plane_bsize,
                &raw mut txfm_param,
                &raw mut quant_param,
            );
            rate_cost = av1_cost_coeffs_txb_laplacian(
                x,
                plane,
                block,
                tx_size,
                tx_type as TX_TYPE,
                txb_ctx,
                reduced_tx_set_used,
                0 as core::ffi::c_int,
            );
            let scan_order: *const SCAN_ORDER = get_scan(
                txfm_param.tx_size,
                txfm_param.tx_type,
            ) as *const SCAN_ORDER;
            dist_block_tx_domain(
                x,
                plane,
                block,
                tx_size,
                quant_param.qmatrix,
                (*scan_order).scan,
                &raw mut dist,
                &raw mut sse,
            );
            *txk_map.offset(num_cand as isize) = tx_type;
            rds[num_cand as usize] = (rate_cost as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            if rds[num_cand as usize] == 0 as int64_t {
                rds[num_cand as usize] = 1 as int64_t;
            }
            num_cand += 1;
        }
        idx += 1;
    }
    if num_cand == 0 as core::ffi::c_int {
        return 0xffff as core::ffi::c_int as uint16_t;
    }
    sort_rd(rds.as_mut_ptr(), txk_map as *mut core::ffi::c_int, num_cand);
    let mut prune: uint16_t = !((1 as core::ffi::c_int)
        << *txk_map.offset(0 as core::ffi::c_int as isize)) as uint16_t;
    let mut factor: int64_t = 0 as int64_t;
    let mut idx_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while idx_0 < num_cand {
        factor = 1000 as int64_t
            * (rds[idx_0 as usize] - rds[0 as core::ffi::c_int as usize])
            / rds[0 as core::ffi::c_int as usize];
        if !(factor < prune_factor as int64_t) {
            break;
        }
        prune = (prune as core::ffi::c_int
            & !((1 as core::ffi::c_int) << *txk_map.offset(idx_0 as isize))) as uint16_t;
        idx_0 += 1;
    }
    return prune;
}
static mut prune_2D_adaptive_thresholds: [*const core::ffi::c_float; 19] = unsafe {
    [
        core::ptr::addr_of!(c2rust_aom_float_table_0).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_1).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_2).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_3).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_4).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_5).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_6).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_7).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        core::ptr::addr_of!(c2rust_aom_float_table_8).cast::<core::ffi::c_float>() as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
        0 as *const core::ffi::c_float,
    ]
};
#[inline]
unsafe extern "C-unwind" fn get_adaptive_thresholds(
    mut tx_size: TX_SIZE,
    mut tx_set_type: TxSetType,
    mut prune_2d_txfm_mode: TX_TYPE_PRUNE_MODE,
) -> core::ffi::c_float {
    let prune_aggr_table: [[core::ffi::c_int; 2]; 5] = [
        [4 as core::ffi::c_int, 1 as core::ffi::c_int],
        [6 as core::ffi::c_int, 3 as core::ffi::c_int],
        [9 as core::ffi::c_int, 6 as core::ffi::c_int],
        [9 as core::ffi::c_int, 6 as core::ffi::c_int],
        [12 as core::ffi::c_int, 9 as core::ffi::c_int],
    ];
    let mut pruning_aggressiveness: core::ffi::c_int = 0 as core::ffi::c_int;
    if tx_set_type as core::ffi::c_int == EXT_TX_SET_ALL16 as core::ffi::c_int {
        pruning_aggressiveness = prune_aggr_table[(prune_2d_txfm_mode as core::ffi::c_int
            - TX_TYPE_PRUNE_1 as core::ffi::c_int)
            as usize][0 as core::ffi::c_int as usize];
    } else if tx_set_type as core::ffi::c_int
        == EXT_TX_SET_DTT9_IDTX_1DDCT as core::ffi::c_int
    {
        pruning_aggressiveness = prune_aggr_table[(prune_2d_txfm_mode as core::ffi::c_int
            - TX_TYPE_PRUNE_1 as core::ffi::c_int)
            as usize][1 as core::ffi::c_int as usize];
    }
    return *(prune_2D_adaptive_thresholds[tx_size as usize])
        .offset(pruning_aggressiveness as isize);
}
#[inline]
unsafe extern "C-unwind" fn get_energy_distribution_finer(
    mut diff: *const int16_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut hordist: *mut core::ffi::c_float,
    mut verdist: *mut core::ffi::c_float,
) {
    let mut esq: [core::ffi::c_uint; 256] = [0; 256];
    let w_shift: core::ffi::c_int = if bw <= 8 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let h_shift: core::ffi::c_int = if bh <= 8 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let esq_w: core::ffi::c_int = bw >> w_shift;
    let esq_h: core::ffi::c_int = bh >> h_shift;
    let esq_sz: core::ffi::c_int = esq_w * esq_h;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    memset(
        esq.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (esq_sz as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as size_t),
    );
    if w_shift != 0 {
        i = 0 as core::ffi::c_int;
        while i < bh {
            let mut cur_esq_row: *mut core::ffi::c_uint = esq
                .as_mut_ptr()
                .offset(((i >> h_shift) * esq_w) as isize);
            let mut cur_diff_row: *const int16_t = diff.offset((i * stride) as isize);
            j = 0 as core::ffi::c_int;
            while j < bw {
                let fresh0 = &raw mut *cur_esq_row
                    .offset((j >> 1 as core::ffi::c_int) as isize);
                *fresh0 = (*fresh0)
                    .wrapping_add(
                        (*cur_diff_row.offset(j as isize) as core::ffi::c_int
                            * *cur_diff_row.offset(j as isize) as core::ffi::c_int
                            + *cur_diff_row.offset((j + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int
                                * *cur_diff_row.offset((j + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int) as core::ffi::c_uint,
                    );
                j += 2 as core::ffi::c_int;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < bh {
            let mut cur_esq_row_0: *mut core::ffi::c_uint = esq
                .as_mut_ptr()
                .offset(((i >> h_shift) * esq_w) as isize);
            let mut cur_diff_row_0: *const int16_t = diff.offset((i * stride) as isize);
            j = 0 as core::ffi::c_int;
            while j < bw {
                let fresh1 = &raw mut *cur_esq_row_0.offset(j as isize);
                *fresh1 = (*fresh1)
                    .wrapping_add(
                        (*cur_diff_row_0.offset(j as isize) as core::ffi::c_int
                            * *cur_diff_row_0.offset(j as isize) as core::ffi::c_int)
                            as core::ffi::c_uint,
                    );
                j += 1;
            }
            i += 1;
        }
    }
    let mut total: uint64_t = 0 as uint64_t;
    i = 0 as core::ffi::c_int;
    while i < esq_sz {
        total = total.wrapping_add(esq[i as usize] as uint64_t);
        i += 1;
    }
    if total == 0 as uint64_t {
        let mut hor_val: core::ffi::c_float = 1.0f32 / esq_w as core::ffi::c_float;
        j = 0 as core::ffi::c_int;
        while j < esq_w - 1 as core::ffi::c_int {
            *hordist.offset(j as isize) = hor_val;
            j += 1;
        }
        let mut ver_val: core::ffi::c_float = 1.0f32 / esq_h as core::ffi::c_float;
        i = 0 as core::ffi::c_int;
        while i < esq_h - 1 as core::ffi::c_int {
            *verdist.offset(i as isize) = ver_val;
            i += 1;
        }
        return;
    }
    let e_recip: core::ffi::c_float = 1.0f32 / total as core::ffi::c_float;
    memset(
        hordist as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((esq_w - 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    );
    memset(
        verdist as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((esq_h - 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    );
    let mut cur_esq_row_1: *const core::ffi::c_uint = 0 as *const core::ffi::c_uint;
    i = 0 as core::ffi::c_int;
    while i < esq_h - 1 as core::ffi::c_int {
        cur_esq_row_1 = esq.as_mut_ptr().offset((i * esq_w) as isize);
        j = 0 as core::ffi::c_int;
        while j < esq_w - 1 as core::ffi::c_int {
            *hordist.offset(j as isize)
                += *cur_esq_row_1.offset(j as isize) as core::ffi::c_float;
            *verdist.offset(i as isize)
                += *cur_esq_row_1.offset(j as isize) as core::ffi::c_float;
            j += 1;
        }
        *verdist.offset(i as isize)
            += *cur_esq_row_1.offset(j as isize) as core::ffi::c_float;
        i += 1;
    }
    cur_esq_row_1 = esq.as_mut_ptr().offset((i * esq_w) as isize);
    j = 0 as core::ffi::c_int;
    while j < esq_w - 1 as core::ffi::c_int {
        *hordist.offset(j as isize)
            += *cur_esq_row_1.offset(j as isize) as core::ffi::c_float;
        j += 1;
    }
    j = 0 as core::ffi::c_int;
    while j < esq_w - 1 as core::ffi::c_int {
        *hordist.offset(j as isize) *= e_recip;
        j += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < esq_h - 1 as core::ffi::c_int {
        *verdist.offset(i as isize) *= e_recip;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn check_bit_mask(
    mut mask: uint16_t,
    mut val: core::ffi::c_int,
) -> bool {
    return mask as core::ffi::c_int & (1 as core::ffi::c_int) << val != 0;
}
#[inline]
unsafe extern "C-unwind" fn set_bit_mask(mut mask: *mut uint16_t, mut val: core::ffi::c_int) {
    *mask = (*mask as core::ffi::c_int | (1 as core::ffi::c_int) << val) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn unset_bit_mask(mut mask: *mut uint16_t, mut val: core::ffi::c_int) {
    *mask = (*mask as core::ffi::c_int & !((1 as core::ffi::c_int) << val)) as uint16_t;
}
unsafe extern "C-unwind" fn prune_tx_2D(
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_set_type: TxSetType,
    mut prune_2d_txfm_mode: TX_TYPE_PRUNE_MODE,
    mut txk_map: *mut core::ffi::c_int,
    mut allowed_tx_mask: *mut uint16_t,
) {
    static mut tx_type_table_2D: [core::ffi::c_int; 16] = [
        DCT_DCT as core::ffi::c_int,
        DCT_ADST as core::ffi::c_int,
        DCT_FLIPADST as core::ffi::c_int,
        V_DCT as core::ffi::c_int,
        ADST_DCT as core::ffi::c_int,
        ADST_ADST as core::ffi::c_int,
        ADST_FLIPADST as core::ffi::c_int,
        V_ADST as core::ffi::c_int,
        FLIPADST_DCT as core::ffi::c_int,
        FLIPADST_ADST as core::ffi::c_int,
        FLIPADST_FLIPADST as core::ffi::c_int,
        V_FLIPADST as core::ffi::c_int,
        H_DCT as core::ffi::c_int,
        H_ADST as core::ffi::c_int,
        H_FLIPADST as core::ffi::c_int,
        IDTX as core::ffi::c_int,
    ];
    if tx_set_type as core::ffi::c_int != EXT_TX_SET_ALL16 as core::ffi::c_int
        && tx_set_type as core::ffi::c_int
            != EXT_TX_SET_DTT9_IDTX_1DDCT as core::ffi::c_int
    {
        return;
    }
    let mut nn_config_hor: *const NN_CONFIG = av1_tx_type_nnconfig_map_hor[tx_size
        as usize];
    let mut nn_config_ver: *const NN_CONFIG = av1_tx_type_nnconfig_map_ver[tx_size
        as usize];
    if nn_config_hor.is_null() || nn_config_ver.is_null() {
        return;
    }
    let mut hfeatures: [core::ffi::c_float; 16] = [0.; 16];
    let mut vfeatures: [core::ffi::c_float; 16] = [0.; 16];
    let mut hscores: [core::ffi::c_float; 4] = [0.; 4];
    let mut vscores: [core::ffi::c_float; 4] = [0.; 4];
    let mut scores_2D_raw: [core::ffi::c_float; 16] = [0.; 16];
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    let hfeatures_num: core::ffi::c_int = if bw <= 8 as core::ffi::c_int {
        bw
    } else {
        bw / 2 as core::ffi::c_int
    };
    let vfeatures_num: core::ffi::c_int = if bh <= 8 as core::ffi::c_int {
        bh
    } else {
        bh / 2 as core::ffi::c_int
    };
    if !(hfeatures_num <= 16 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"prune_tx_2D\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1556 as core::ffi::c_int,
            b"hfeatures_num <= 16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(vfeatures_num <= 16 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"prune_tx_2D\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1557 as core::ffi::c_int,
            b"vfeatures_num <= 16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let p: *const macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblock_plane;
    let diff_stride: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut diff: *const int16_t = ((*p).src_diff)
        .offset((4 as core::ffi::c_int * blk_row * diff_stride) as isize)
        .offset((4 as core::ffi::c_int * blk_col) as isize);
    get_energy_distribution_finer(
        diff,
        diff_stride,
        bw,
        bh,
        hfeatures.as_mut_ptr(),
        vfeatures.as_mut_ptr(),
    );
    av1_get_horver_correlation_full_c(
        diff,
        diff_stride,
        bw,
        bh,
        &raw mut *hfeatures
            .as_mut_ptr()
            .offset((hfeatures_num - 1 as core::ffi::c_int) as isize),
        &raw mut *vfeatures
            .as_mut_ptr()
            .offset((vfeatures_num - 1 as core::ffi::c_int) as isize),
    );
    av1_nn_predict_c(
        hfeatures.as_mut_ptr(),
        nn_config_hor,
        1 as core::ffi::c_int,
        hscores.as_mut_ptr(),
    );
    av1_nn_predict_c(
        vfeatures.as_mut_ptr(),
        nn_config_ver,
        1 as core::ffi::c_int,
        vscores.as_mut_ptr(),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        let mut cur_scores_2D: *mut core::ffi::c_float = scores_2D_raw
            .as_mut_ptr()
            .offset((i * 4 as core::ffi::c_int) as isize);
        *cur_scores_2D.offset(0 as core::ffi::c_int as isize) = vscores[i as usize]
            * hscores[0 as core::ffi::c_int as usize];
        *cur_scores_2D.offset(1 as core::ffi::c_int as isize) = vscores[i as usize]
            * hscores[1 as core::ffi::c_int as usize];
        *cur_scores_2D.offset(2 as core::ffi::c_int as isize) = vscores[i as usize]
            * hscores[2 as core::ffi::c_int as usize];
        *cur_scores_2D.offset(3 as core::ffi::c_int as isize) = vscores[i as usize]
            * hscores[3 as core::ffi::c_int as usize];
        i += 1;
    }
    if !(TX_TYPES as core::ffi::c_int == 16 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"prune_tx_2D\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1585 as core::ffi::c_int,
            b"TX_TYPES == 16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_nn_fast_softmax_16_c(scores_2D_raw.as_mut_ptr(), scores_2D_raw.as_mut_ptr());
    let score_thresh: core::ffi::c_float = get_adaptive_thresholds(
        tx_size,
        tx_set_type,
        prune_2d_txfm_mode,
    ) as core::ffi::c_float;
    let mut max_score_i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_score: core::ffi::c_float = 0.0f32;
    let mut allow_bitmask: uint16_t = 0 as uint16_t;
    let mut sum_score: core::ffi::c_float = 0.0f32;
    let mut allow_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tx_type_allowed: [core::ffi::c_int; 16] = [
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
        TX_TYPE_INVALID as core::ffi::c_int,
    ];
    let mut scores_2D: [core::ffi::c_float; 16] = [
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
        -(1 as core::ffi::c_int) as core::ffi::c_float,
    ];
    let mut tx_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while tx_idx < TX_TYPES as core::ffi::c_int {
        let allow_tx_type: core::ffi::c_int = check_bit_mask(
            *allowed_tx_mask,
            tx_type_table_2D[tx_idx as usize],
        ) as core::ffi::c_int;
        if !(allow_tx_type == 0) {
            if scores_2D_raw[tx_idx as usize] > max_score {
                max_score = scores_2D_raw[tx_idx as usize];
                max_score_i = tx_idx;
            }
            if scores_2D_raw[tx_idx as usize] >= score_thresh {
                set_bit_mask(&raw mut allow_bitmask, tx_type_table_2D[tx_idx as usize]);
                sum_score += scores_2D_raw[tx_idx as usize];
                scores_2D[allow_count as usize] = scores_2D_raw[tx_idx as usize];
                tx_type_allowed[allow_count as usize] = tx_type_table_2D[tx_idx
                    as usize];
                allow_count += 1 as core::ffi::c_int;
            }
        }
        tx_idx += 1;
    }
    if !check_bit_mask(allow_bitmask, tx_type_table_2D[max_score_i as usize]) {
        set_bit_mask(&raw mut allow_bitmask, tx_type_table_2D[max_score_i as usize]);
        memcpy(
            txk_map as *mut core::ffi::c_void,
            core::ptr::addr_of!(tx_type_table_2D).cast::<core::ffi::c_int>() as *const core::ffi::c_void,
            ::core::mem::size_of::<[core::ffi::c_int; 16]>() as size_t,
        );
        *allowed_tx_mask = allow_bitmask;
        return;
    }
    if allow_count <= 8 as core::ffi::c_int {
        av1_sort_fi32_8(
            scores_2D.as_mut_ptr(),
            tx_type_allowed.as_mut_ptr() as *mut int32_t,
        );
    } else {
        av1_sort_fi32_16(
            scores_2D.as_mut_ptr(),
            tx_type_allowed.as_mut_ptr() as *mut int32_t,
        );
    }
    if prune_2d_txfm_mode as core::ffi::c_int >= TX_TYPE_PRUNE_4 as core::ffi::c_int {
        let mut temp_score: core::ffi::c_float = 0.0f32;
        let mut score_ratio: core::ffi::c_float = 0.0f32;
        let mut tx_idx_0: core::ffi::c_int = 0;
        let mut tx_count: core::ffi::c_int = 0 as core::ffi::c_int;
        let inv_sum_score: core::ffi::c_float = 100 as core::ffi::c_int
            as core::ffi::c_float / sum_score;
        tx_idx_0 = 0 as core::ffi::c_int;
        while tx_idx_0 < allow_count {
            if score_ratio as core::ffi::c_double > 30.0f64
                && tx_count >= 2 as core::ffi::c_int
            {
                break;
            }
            if !check_bit_mask(allow_bitmask, tx_type_allowed[tx_idx_0 as usize])
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"prune_tx_2D\0"))
                        .as_ptr(),
                    b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1664 as core::ffi::c_int,
                    b"check_bit_mask(allow_bitmask, tx_type_allowed[tx_idx])\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            temp_score += scores_2D[tx_idx_0 as usize];
            score_ratio = temp_score * inv_sum_score;
            tx_count += 1;
            tx_idx_0 += 1;
        }
        while tx_idx_0 < allow_count {
            unset_bit_mask(&raw mut allow_bitmask, tx_type_allowed[tx_idx_0 as usize]);
            tx_idx_0 += 1;
        }
    }
    memcpy(
        txk_map as *mut core::ffi::c_void,
        tx_type_allowed.as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_int; 16]>() as size_t,
    );
    *allowed_tx_mask = allow_bitmask;
}
unsafe extern "C-unwind" fn get_dev(
    mut mean: core::ffi::c_float,
    mut x2_sum: core::ffi::c_double,
    mut num: core::ffi::c_int,
) -> core::ffi::c_float {
    let e_x2: core::ffi::c_float = (x2_sum / num as core::ffi::c_double)
        as core::ffi::c_float;
    let diff: core::ffi::c_float = e_x2 - mean * mean;
    let dev: core::ffi::c_float = if diff > 0 as core::ffi::c_int as core::ffi::c_float {
        sqrtf(diff) as core::ffi::c_float
    } else {
        0 as core::ffi::c_int as core::ffi::c_float
    };
    return dev;
}
#[inline]
unsafe extern "C-unwind" fn get_mean_dev_features(
    mut data: *const int16_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut features: *mut core::ffi::c_float,
) -> core::ffi::c_int {
    let data_ptr: *const int16_t = &*data.offset(0 as core::ffi::c_int as isize)
        as *const int16_t;
    let subh: core::ffi::c_int = if bh >= bw { bh >> 1 as core::ffi::c_int } else { bh };
    let subw: core::ffi::c_int = if bw >= bh { bw >> 1 as core::ffi::c_int } else { bw };
    let num: core::ffi::c_int = bw * bh;
    let sub_num: core::ffi::c_int = subw * subh;
    let mut feature_idx: core::ffi::c_int = 2 as core::ffi::c_int;
    let mut total_x_sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut total_x2_sum: int64_t = 0 as int64_t;
    let mut num_sub_blks: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mean2_sum: core::ffi::c_double = 0.0f64;
    let mut dev_sum: core::ffi::c_float = 0.0f32;
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < bh {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < bw {
            let mut x_sum: core::ffi::c_int = 0;
            let mut x2_sum: int64_t = 0;
            aom_get_blk_sse_sum_c(
                data_ptr.offset((row * stride) as isize).offset(col as isize),
                stride,
                subw,
                subh,
                &raw mut x_sum,
                &raw mut x2_sum,
            );
            total_x_sum += x_sum;
            total_x2_sum += x2_sum;
            let mean: core::ffi::c_float = x_sum as core::ffi::c_float
                / sub_num as core::ffi::c_float;
            let dev: core::ffi::c_float = get_dev(
                mean,
                x2_sum as core::ffi::c_double,
                sub_num,
            ) as core::ffi::c_float;
            let fresh2 = feature_idx;
            feature_idx = feature_idx + 1;
            *features.offset(fresh2 as isize) = mean;
            let fresh3 = feature_idx;
            feature_idx = feature_idx + 1;
            *features.offset(fresh3 as isize) = dev;
            mean2_sum += (mean * mean) as core::ffi::c_double;
            dev_sum += dev;
            num_sub_blks += 1;
            col += subw;
        }
        row += subh;
    }
    let lvl0_mean: core::ffi::c_float = total_x_sum as core::ffi::c_float
        / num as core::ffi::c_float;
    *features.offset(0 as core::ffi::c_int as isize) = lvl0_mean;
    *features.offset(1 as core::ffi::c_int as isize) = get_dev(
        lvl0_mean,
        total_x2_sum as core::ffi::c_double,
        num,
    );
    let fresh4 = feature_idx;
    feature_idx = feature_idx + 1;
    *features.offset(fresh4 as isize) = get_dev(lvl0_mean, mean2_sum, num_sub_blks);
    let fresh5 = feature_idx;
    feature_idx = feature_idx + 1;
    *features.offset(fresh5 as isize) = dev_sum / num_sub_blks as core::ffi::c_float;
    return feature_idx;
}
unsafe extern "C-unwind" fn ml_predict_tx_split(
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    let mut nn_config: *const NN_CONFIG = av1_tx_split_nnconfig_map[tx_size as usize];
    if nn_config.is_null() {
        return -(1 as core::ffi::c_int);
    }
    let diff_stride: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut diff: *const int16_t = ((*x).plane[0 as core::ffi::c_int as usize].src_diff)
        .offset((4 as core::ffi::c_int * blk_row * diff_stride) as isize)
        .offset((4 as core::ffi::c_int * blk_col) as isize);
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut features: [core::ffi::c_float; 64] = [
        0.0f32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    get_mean_dev_features(diff, diff_stride, bw, bh, features.as_mut_ptr());
    let mut score: core::ffi::c_float = 0.0f32;
    av1_nn_predict_c(
        features.as_mut_ptr(),
        nn_config,
        1 as core::ffi::c_int,
        &raw mut score,
    );
    let mut int_score: core::ffi::c_int = (score
        * 10000 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_int;
    return clamp(int_score, -(80000 as core::ffi::c_int), 80000 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn get_tx_mask(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    txb_ctx: *const TXB_CTX,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut ref_best_rd: int64_t,
    mut allowed_txk_types: *mut TX_TYPE,
    mut txk_map: *mut core::ffi::c_int,
) -> uint16_t {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let fast_tx_search: core::ffi::c_int = ftxs_mode as core::ffi::c_int
        & FTXS_DCT_AND_1D_DCT_ONLY as core::ffi::c_int as core::ffi::c_int;
    let mut txk_allowed: TX_TYPE = TX_TYPES as core::ffi::c_int as TX_TYPE;
    let update_type: FRAME_UPDATE_TYPE = get_frame_update_type(
        &raw mut (*(*cpi).ppi).gf_group,
        (*cpi).gf_frame_index as core::ffi::c_int,
    ) as FRAME_UPDATE_TYPE;
    let mut use_actual_frame_probs: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut tx_type_probs: *const core::ffi::c_int = 0 as *const core::ffi::c_int;
    if use_actual_frame_probs != 0 {
        tx_type_probs = ((*(*cpi).ppi)
            .frame_probs
            .tx_type_probs[update_type as usize][tx_size as usize])
            .as_mut_ptr();
    }
    if is_inter == 0 && (*txfm_params).use_default_intra_tx_type != 0
        || is_inter != 0
            && (*txfm_params).default_inter_tx_type_prob_thresh == 0 as core::ffi::c_int
    {
        txk_allowed = get_default_tx_type(
            0 as PLANE_TYPE,
            xd,
            tx_size,
            (*cpi).use_screen_content_tools,
        );
    } else if is_inter != 0
        && (*txfm_params).default_inter_tx_type_prob_thresh != INT_MAX
    {
        if *tx_type_probs.offset(DCT_DCT as core::ffi::c_int as isize)
            > (*txfm_params).default_inter_tx_type_prob_thresh
        {
            txk_allowed = DCT_DCT as core::ffi::c_int as TX_TYPE;
        } else {
            let mut force_tx_type: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut max_prob: core::ffi::c_int = 0 as core::ffi::c_int;
            let tx_type_prob_threshold: core::ffi::c_int = (*txfm_params)
                .default_inter_tx_type_prob_thresh + PROB_THRESH_OFFSET_TX_TYPE;
            let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
            while i < TX_TYPES as core::ffi::c_int {
                if *tx_type_probs.offset(i as isize) > max_prob {
                    max_prob = *tx_type_probs.offset(i as isize);
                    force_tx_type = i;
                }
                i += 1;
            }
            if max_prob > tx_type_prob_threshold {
                txk_allowed = force_tx_type as TX_TYPE;
            } else if (*x).rd_model as core::ffi::c_int
                == LOW_TXFM_RD as core::ffi::c_int
            {
                if plane == 0 as core::ffi::c_int {
                    txk_allowed = DCT_DCT as core::ffi::c_int as TX_TYPE;
                }
            }
        }
    } else if (*x).rd_model as core::ffi::c_int == LOW_TXFM_RD as core::ffi::c_int {
        if plane == 0 as core::ffi::c_int {
            txk_allowed = DCT_DCT as core::ffi::c_int as TX_TYPE;
        }
    }
    let tx_set_type: TxSetType = av1_get_ext_tx_set_type(
        tx_size,
        is_inter,
        (*cm).features.reduced_tx_set_used as core::ffi::c_int,
    ) as TxSetType;
    let mut uv_tx_type: TX_TYPE = DCT_DCT as core::ffi::c_int as TX_TYPE;
    if plane != 0 {
        txk_allowed = av1_get_tx_type(
            xd,
            get_plane_type(plane),
            blk_row,
            blk_col,
            tx_size,
            (*cm).features.reduced_tx_set_used as core::ffi::c_int,
        );
        uv_tx_type = txk_allowed;
    }
    let mut intra_dir: PREDICTION_MODE = (if (*mbmi)
        .filter_intra_mode_info
        .use_filter_intra as core::ffi::c_int != 0
    {
        fimode_to_intradir[(*mbmi).filter_intra_mode_info.filter_intra_mode as usize]
            as core::ffi::c_int
    } else {
        (*mbmi).mode as core::ffi::c_int
    }) as PREDICTION_MODE;
    let mut ext_tx_used_flag: uint16_t = (if (*cpi)
        .sf
        .tx_sf
        .tx_type_search
        .use_reduced_intra_txset != 0 as core::ffi::c_int
        && tx_set_type as core::ffi::c_int
            == EXT_TX_SET_DTT4_IDTX_1DDCT as core::ffi::c_int
    {
        av1_reduced_intra_tx_used_flag[intra_dir as usize] as core::ffi::c_int
    } else {
        av1_ext_tx_used_flag[tx_set_type as usize] as core::ffi::c_int
    }) as uint16_t;
    if (*cpi).sf.tx_sf.tx_type_search.use_reduced_intra_txset == 2 as core::ffi::c_int {
        ext_tx_used_flag = (ext_tx_used_flag as core::ffi::c_int
            & av1_derived_intra_tx_used_flag[intra_dir as usize] as core::ffi::c_int)
            as uint16_t;
    }
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0
        || txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            > TX_32X32 as core::ffi::c_int
        || ext_tx_used_flag as core::ffi::c_int == 0x1 as core::ffi::c_int
        || is_inter != 0
            && (*cpi).oxcf.txfm_cfg.use_inter_dct_only as core::ffi::c_int != 0
        || is_inter == 0
            && (*cpi).oxcf.txfm_cfg.use_intra_dct_only as core::ffi::c_int != 0
    {
        txk_allowed = DCT_DCT as core::ffi::c_int as TX_TYPE;
    }
    if (*cpi).oxcf.txfm_cfg.enable_flip_idtx as core::ffi::c_int == 0 as core::ffi::c_int
    {
        ext_tx_used_flag = (ext_tx_used_flag as core::ffi::c_int
            & DCT_ADST_TX_MASK as core::ffi::c_int) as uint16_t;
    }
    let mut allowed_tx_mask: uint16_t = 0 as uint16_t;
    if (txk_allowed as core::ffi::c_int) < TX_TYPES as core::ffi::c_int {
        allowed_tx_mask = ((1 as core::ffi::c_int) << txk_allowed as core::ffi::c_int)
            as uint16_t;
        allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
            & ext_tx_used_flag as core::ffi::c_int) as uint16_t;
    } else if fast_tx_search != 0 {
        allowed_tx_mask = 0xc01 as uint16_t;
        allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
            & ext_tx_used_flag as core::ffi::c_int) as uint16_t;
    } else if is_inter == 0 && (*txfm_params).use_derived_intra_tx_type_set != 0 {
        allowed_tx_mask = av1_derived_intra_tx_used_flag[intra_dir as usize];
        allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
            & ext_tx_used_flag as core::ffi::c_int) as uint16_t;
    } else {
        if !(plane == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"get_tx_mask\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                1866 as core::ffi::c_int,
                b"plane == 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        allowed_tx_mask = ext_tx_used_flag;
        let mut num_allowed: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut i_0: core::ffi::c_int = 0;
        if (*cpi).sf.tx_sf.tx_type_search.prune_tx_type_using_stats != 0 {
            static mut thresh_arr: [[core::ffi::c_int; 7]; 2] = [
                [
                    10 as core::ffi::c_int,
                    15 as core::ffi::c_int,
                    15 as core::ffi::c_int,
                    10 as core::ffi::c_int,
                    15 as core::ffi::c_int,
                    15 as core::ffi::c_int,
                    15 as core::ffi::c_int,
                ],
                [
                    10 as core::ffi::c_int,
                    17 as core::ffi::c_int,
                    17 as core::ffi::c_int,
                    10 as core::ffi::c_int,
                    17 as core::ffi::c_int,
                    17 as core::ffi::c_int,
                    17 as core::ffi::c_int,
                ],
            ];
            let thresh: core::ffi::c_int = thresh_arr[((*cpi)
                .sf
                .tx_sf
                .tx_type_search
                .prune_tx_type_using_stats - 1 as core::ffi::c_int)
                as usize][update_type as usize];
            let mut prune: uint16_t = 0 as uint16_t;
            let mut max_prob_0: core::ffi::c_int = -(1 as core::ffi::c_int);
            let mut max_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            i_0 = 0 as core::ffi::c_int;
            while i_0 < TX_TYPES as core::ffi::c_int {
                if *tx_type_probs.offset(i_0 as isize) > max_prob_0
                    && allowed_tx_mask as core::ffi::c_int
                        & (1 as core::ffi::c_int) << i_0 != 0
                {
                    max_prob_0 = *tx_type_probs.offset(i_0 as isize);
                    max_idx = i_0;
                }
                if *tx_type_probs.offset(i_0 as isize) < thresh {
                    prune = (prune as core::ffi::c_int | (1 as core::ffi::c_int) << i_0)
                        as uint16_t;
                }
                i_0 += 1;
            }
            if prune as core::ffi::c_int >> max_idx & 0x1 as core::ffi::c_int != 0 {
                prune = (prune as core::ffi::c_int
                    & !((1 as core::ffi::c_int) << max_idx)) as uint16_t;
            }
            allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
                & !(prune as core::ffi::c_int)) as uint16_t;
        }
        i_0 = 0 as core::ffi::c_int;
        while i_0 < TX_TYPES as core::ffi::c_int {
            if allowed_tx_mask as core::ffi::c_int & (1 as core::ffi::c_int) << i_0 != 0
            {
                num_allowed += 1;
            }
            i_0 += 1;
        }
        if !(num_allowed > 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"get_tx_mask\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                1893 as core::ffi::c_int,
                b"num_allowed > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if num_allowed > 2 as core::ffi::c_int
            && (*cpi).sf.tx_sf.tx_type_search.prune_tx_type_est_rd != 0
        {
            let mut pf: core::ffi::c_int = prune_factors[(*txfm_params)
                .prune_2d_txfm_mode as usize];
            let mut mf: core::ffi::c_int = mul_factors[(*txfm_params).prune_2d_txfm_mode
                as usize];
            if num_allowed <= 7 as core::ffi::c_int {
                let prune_0: uint16_t = prune_txk_type(
                    cpi,
                    x,
                    plane,
                    block,
                    tx_size,
                    blk_row,
                    blk_col,
                    plane_bsize,
                    txk_map,
                    allowed_tx_mask,
                    pf,
                    txb_ctx,
                    (*cm).features.reduced_tx_set_used as core::ffi::c_int,
                ) as uint16_t;
                allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
                    & !(prune_0 as core::ffi::c_int)) as uint16_t;
            } else {
                let num_sel: core::ffi::c_int = (num_allowed * mf
                    + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
                let prune_1: uint16_t = prune_txk_type_separ(
                    cpi,
                    x,
                    plane,
                    block,
                    tx_size,
                    blk_row,
                    blk_col,
                    plane_bsize,
                    txk_map,
                    allowed_tx_mask as int16_t,
                    pf,
                    txb_ctx,
                    (*cm).features.reduced_tx_set_used as core::ffi::c_int,
                    ref_best_rd,
                    num_sel,
                ) as uint16_t;
                allowed_tx_mask = (allowed_tx_mask as core::ffi::c_int
                    & !(prune_1 as core::ffi::c_int)) as uint16_t;
            }
        } else {
            if !(num_allowed > 0 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"get_tx_mask\0"))
                        .as_ptr(),
                    b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1914 as core::ffi::c_int,
                    b"num_allowed > 0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut allowed_tx_count: core::ffi::c_int = if (*txfm_params)
                .prune_2d_txfm_mode >= TX_TYPE_PRUNE_4 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                5 as core::ffi::c_int
            };
            if (*txfm_params).prune_2d_txfm_mode >= TX_TYPE_PRUNE_1 as core::ffi::c_int
                && is_inter != 0 && num_allowed > allowed_tx_count
            {
                prune_tx_2D(
                    x,
                    plane_bsize,
                    tx_size,
                    blk_row,
                    blk_col,
                    tx_set_type,
                    (*txfm_params).prune_2d_txfm_mode as TX_TYPE_PRUNE_MODE,
                    txk_map,
                    &raw mut allowed_tx_mask,
                );
            }
        }
    }
    if allowed_tx_mask as core::ffi::c_int == 0 as core::ffi::c_int {
        txk_allowed = (if plane != 0 {
            uv_tx_type as core::ffi::c_int
        } else {
            DCT_DCT as core::ffi::c_int
        }) as TX_TYPE;
        allowed_tx_mask = ((1 as core::ffi::c_int) << txk_allowed as core::ffi::c_int)
            as uint16_t;
    }
    if !(!((txk_allowed as core::ffi::c_int) < TX_TYPES as core::ffi::c_int)
        || allowed_tx_mask as core::ffi::c_int
            == (1 as core::ffi::c_int) << txk_allowed as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"get_tx_mask\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            1932 as core::ffi::c_int,
            b"IMPLIES(txk_allowed < TX_TYPES, allowed_tx_mask == 1 << txk_allowed)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *allowed_txk_types = txk_allowed;
    return allowed_tx_mask;
}
#[inline]
unsafe extern "C-unwind" fn cost_coeffs(
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    tx_type: TX_TYPE,
    txb_ctx: *const TXB_CTX,
    mut reduced_tx_set_used: core::ffi::c_int,
) -> core::ffi::c_int {
    let cost: core::ffi::c_int = av1_cost_coeffs_txb(
        x,
        plane,
        block,
        tx_size,
        tx_type,
        txb_ctx,
        reduced_tx_set_used,
    ) as core::ffi::c_int;
    return cost;
}
unsafe extern "C-unwind" fn skip_trellis_opt_based_on_satd(
    mut x: *mut MACROBLOCK,
    mut quant_param: *mut QUANT_PARAM,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut quant_b_adapt: core::ffi::c_int,
    mut qstep: core::ffi::c_int,
    mut coeff_opt_satd_threshold: core::ffi::c_uint,
    mut skip_trellis: core::ffi::c_int,
    mut dc_only_blk: core::ffi::c_int,
) -> core::ffi::c_int {
    if skip_trellis != 0 || coeff_opt_satd_threshold == UINT_MAX {
        return skip_trellis;
    }
    let p: *const macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblock_plane;
    let block_offset: core::ffi::c_int = block << 4 as core::ffi::c_int;
    let coeff_ptr: *mut tran_low_t = ((*p).coeff).offset(block_offset as isize);
    let n_coeffs: core::ffi::c_int = av1_get_max_eob(tx_size) as core::ffi::c_int;
    let shift: core::ffi::c_int = MAX_TX_SCALE
        - av1_get_tx_scale(tx_size) as core::ffi::c_int;
    let mut satd: core::ffi::c_int = if dc_only_blk != 0 {
        abs(*coeff_ptr.offset(0 as core::ffi::c_int as isize))
    } else {
        aom_satd_c(coeff_ptr, n_coeffs)
    };
    satd = if shift < 0 as core::ffi::c_int { satd << -shift } else { satd >> shift };
    satd >>= (*x).e_mbd.bd - 8 as core::ffi::c_int;
    let skip_block_trellis: core::ffi::c_int = (satd as uint64_t
        > (coeff_opt_satd_threshold as uint64_t)
            .wrapping_mul(qstep as uint64_t)
            .wrapping_mul(sqrt_tx_pixels_2d[tx_size as usize] as uint64_t))
        as core::ffi::c_int;
    av1_setup_quant(
        tx_size,
        (skip_block_trellis == 0) as core::ffi::c_int,
        if skip_block_trellis != 0 {
            if USE_B_QUANT_NO_TRELLIS != 0 {
                AV1_XFORM_QUANT_B as core::ffi::c_int
            } else {
                AV1_XFORM_QUANT_FP as core::ffi::c_int
            }
        } else {
            AV1_XFORM_QUANT_FP as core::ffi::c_int
        },
        quant_b_adapt,
        quant_param,
    );
    return skip_block_trellis;
}
#[inline]
unsafe extern "C-unwind" fn predict_dc_only_block(
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut best_rd_stats: *mut RD_STATS,
    mut block_sse: *mut int64_t,
    mut block_mse_q8: *mut core::ffi::c_uint,
    mut per_px_mean: *mut int64_t,
    mut dc_only_blk: *mut core::ffi::c_int,
) {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let dequant_shift: core::ffi::c_int = if is_cur_buf_hbd(xd) != 0 {
        (*xd).bd - 5 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    let qstep: core::ffi::c_int = *((*x).plane[plane as usize].dequant_QTX)
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int >> dequant_shift;
    let mut block_var: uint64_t = UINT64_MAX as uint64_t;
    let dc_qstep: core::ffi::c_int = *((*x).plane[plane as usize].dequant_QTX)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        >> 3 as core::ffi::c_int;
    *block_sse = pixel_diff_stats(
        x,
        plane,
        blk_row,
        blk_col,
        plane_bsize,
        txsize_to_bsize[tx_size as usize],
        block_mse_q8,
        per_px_mean,
        &raw mut block_var,
    );
    if !(*block_mse_q8
        != (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"predict_dc_only_block\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2012 as core::ffi::c_int,
            b"(*block_mse_q8) != UINT_MAX\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut var_threshold: uint64_t = (1.8f64 * qstep as core::ffi::c_double
        * qstep as core::ffi::c_double) as uint64_t;
    if is_cur_buf_hbd(xd) != 0 {
        block_var = block_var
            .wrapping_add(
                ((1 as core::ffi::c_int)
                    << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as uint64_t,
            ) >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int;
    }
    if block_var >= var_threshold {
        return;
    }
    let predict_dc_level: core::ffi::c_uint = (*x).txfm_search_params.predict_dc_level;
    if !(predict_dc_level != 0 as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"predict_dc_only_block\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2019 as core::ffi::c_int,
            b"predict_dc_level != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (llabs(*per_px_mean) * dc_coeff_scale[tx_size as usize] as core::ffi::c_longlong)
        < (dc_qstep << 12 as core::ffi::c_int) as core::ffi::c_longlong
    {
        (*best_rd_stats).skip_txfm = 1 as uint8_t;
        *((*x).plane[plane as usize].eobs).offset(block as isize) = 0 as uint16_t;
        if is_cur_buf_hbd(xd) != 0 {
            *block_sse = *block_sse
                + ((1 as core::ffi::c_int)
                    << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t
                >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int;
        }
        (*best_rd_stats).dist = *block_sse << 4 as core::ffi::c_int;
        (*best_rd_stats).sse = (*best_rd_stats).dist;
        let mut ctxa: [ENTROPY_CONTEXT; 32] = [0; 32];
        let mut ctxl: [ENTROPY_CONTEXT; 32] = [0; 32];
        av1_get_entropy_contexts(
            plane_bsize,
            &raw mut *((*xd).plane).as_mut_ptr().offset(plane as isize),
            ctxa.as_mut_ptr(),
            ctxl.as_mut_ptr(),
        );
        let mut ta: *mut ENTROPY_CONTEXT = ctxa.as_mut_ptr();
        let mut tl: *mut ENTROPY_CONTEXT = ctxl.as_mut_ptr();
        let txs_ctx: TX_SIZE = get_txsize_entropy_ctx(tx_size) as TX_SIZE;
        let mut txb_ctx_tmp: TXB_CTX = txb_ctx {
            txb_skip_ctx: 0,
            dc_sign_ctx: 0,
        };
        let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
        get_txb_ctx(plane_bsize, tx_size, plane, ta, tl, &raw mut txb_ctx_tmp);
        let zero_blk_rate: core::ffi::c_int = (*x)
            .coeff_costs
            .coeff_costs[txs_ctx as usize][plane_type as usize]
            .txb_skip_cost[txb_ctx_tmp.txb_skip_ctx
            as usize][1 as core::ffi::c_int as usize];
        (*best_rd_stats).rate = zero_blk_rate;
        (*best_rd_stats).rdcost = ((*best_rd_stats).rate as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*best_rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        *((*x).plane[plane as usize].txb_entropy_ctx).offset(block as isize) = 0
            as uint8_t;
    } else if predict_dc_level > 1 as core::ffi::c_uint {
        if plane == 0 as core::ffi::c_int
            || plane > 0 as core::ffi::c_int && is_inter_block(mbmi) != 0
        {
            *dc_only_blk = 1 as core::ffi::c_int;
        }
    }
}
unsafe extern "C-unwind" fn search_tx_type(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    txb_ctx: *const TXB_CTX,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut ref_best_rd: int64_t,
    mut best_rd_stats: *mut RD_STATS,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let mut best_rd: int64_t = INT64_MAX as int64_t;
    let mut best_eob: uint16_t = 0 as uint16_t;
    let mut best_tx_type: TX_TYPE = DCT_DCT as core::ffi::c_int as TX_TYPE;
    let mut rate_cost: core::ffi::c_int = 0 as core::ffi::c_int;
    let p: *mut macroblock_plane = &raw mut *((*x).plane).as_mut_ptr().offset(plane as isize)
        as *mut macroblock_plane;
    let mut orig_dqcoeff: *mut tran_low_t = (*p).dqcoeff;
    let mut best_dqcoeff: *mut tran_low_t = (*x).dqcoeff_buf;
    let tx_type_map_idx: core::ffi::c_int = if plane != 0 {
        0 as core::ffi::c_int
    } else {
        blk_row * (*xd).tx_type_map_stride + blk_col
    };
    av1_invalid_rd_stats(best_rd_stats);
    let mut skip_trellis: core::ffi::c_int = (is_trellis_used(
        (*cpi)
            .optimize_seg_arr[(**((*xd).mi).offset(0 as core::ffi::c_int as isize))
            .segment_id() as usize],
        DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
    ) == 0) as core::ffi::c_int;
    let mut best_txb_ctx: uint8_t = 0 as uint8_t;
    let mut txk_allowed: TX_TYPE = TX_TYPES as core::ffi::c_int as TX_TYPE;
    let mut txk_map: [core::ffi::c_int; 16] = [
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        5 as core::ffi::c_int,
        6 as core::ffi::c_int,
        7 as core::ffi::c_int,
        8 as core::ffi::c_int,
        9 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        12 as core::ffi::c_int,
        13 as core::ffi::c_int,
        14 as core::ffi::c_int,
        15 as core::ffi::c_int,
    ];
    let dequant_shift: core::ffi::c_int = if is_cur_buf_hbd(xd) != 0 {
        (*xd).bd - 5 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    let qstep: core::ffi::c_int = *((*x).plane[plane as usize].dequant_QTX)
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int >> dequant_shift;
    let txw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let txh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    let mut block_sse: int64_t = 0;
    let mut block_mse_q8: core::ffi::c_uint = 0;
    let mut dc_only_blk: core::ffi::c_int = 0 as core::ffi::c_int;
    let predict_dc_block: bool = (*txfm_params).predict_dc_level
        >= 1 as core::ffi::c_uint && txw as core::ffi::c_int != 64 as core::ffi::c_int
        && txh as core::ffi::c_int != 64 as core::ffi::c_int;
    let mut per_px_mean: int64_t = INT64_MAX as int64_t;
    let mut is_border_block: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*cpi).do_border_pad {
        is_border_block = get_visible_dimensions(
            x,
            plane,
            plane_bsize,
            blk_col,
            blk_row,
            txw as core::ffi::c_int,
            txh as core::ffi::c_int,
            true_0 != 0,
            0 as *mut core::ffi::c_int,
            0 as *mut core::ffi::c_int,
        );
        if is_border_block != 0 {
            av1_subtract_txb(
                x,
                plane,
                plane_bsize,
                blk_col,
                blk_row,
                tx_size,
                best_tx_type,
                (*cpi).do_border_pad,
            );
        }
    }
    if predict_dc_block {
        predict_dc_only_block(
            x,
            plane,
            plane_bsize,
            tx_size,
            block,
            blk_row,
            blk_col,
            best_rd_stats,
            &raw mut block_sse,
            &raw mut block_mse_q8,
            &raw mut per_px_mean,
            &raw mut dc_only_blk,
        );
        if (*best_rd_stats).skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int {
            let tx_type: TX_TYPE = DCT_DCT as core::ffi::c_int as TX_TYPE;
            if plane == 0 as core::ffi::c_int {
                *((*xd).tx_type_map).offset(tx_type_map_idx as isize) = tx_type
                    as uint8_t;
            }
            return;
        }
    } else {
        block_sse = av1_pixel_diff_dist(
            x,
            plane,
            blk_row,
            blk_col,
            plane_bsize,
            txsize_to_bsize[tx_size as usize],
            &raw mut block_mse_q8,
        );
        if !(block_mse_q8
            != (2147483647 as core::ffi::c_uint)
                .wrapping_mul(2 as core::ffi::c_uint)
                .wrapping_add(1 as core::ffi::c_uint)) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"search_tx_type\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                2128 as core::ffi::c_int,
                b"block_mse_q8 != UINT_MAX\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    let mut tx_mask: uint16_t = 0;
    if dc_only_blk != 0
        || (*cpi).sf.rt_sf.dct_only_palette_nonrd == 1 as core::ffi::c_int
    {
        tx_mask = ((1 as core::ffi::c_int) << DCT_DCT as core::ffi::c_int) as uint16_t;
    } else {
        tx_mask = get_tx_mask(
            cpi,
            x,
            plane,
            block,
            blk_row,
            blk_col,
            plane_bsize,
            tx_size,
            txb_ctx,
            ftxs_mode,
            ref_best_rd,
            &raw mut txk_allowed,
            txk_map.as_mut_ptr(),
        );
    }
    let allowed_tx_mask: uint16_t = tx_mask;
    if is_cur_buf_hbd(xd) != 0 {
        block_sse = block_sse
            + ((1 as core::ffi::c_int)
                << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                >> 1 as core::ffi::c_int) as int64_t
            >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int;
        block_mse_q8 = block_mse_q8
            .wrapping_add(
                ((1 as core::ffi::c_int)
                    << ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as core::ffi::c_uint,
            ) >> ((*xd).bd - 8 as core::ffi::c_int) * 2 as core::ffi::c_int;
    }
    block_sse *= 16 as int64_t;
    let perform_block_coeff_opt: core::ffi::c_int = (block_mse_q8 as uint64_t
        <= ((*txfm_params).coeff_opt_thresholds[0 as core::ffi::c_int as usize]
            as uint64_t)
            .wrapping_mul(qstep as uint64_t)
            .wrapping_mul(qstep as uint64_t)) as core::ffi::c_int;
    skip_trellis |= (perform_block_coeff_opt == 0) as core::ffi::c_int;
    let mut use_transform_domain_distortion: core::ffi::c_int = ((*txfm_params)
        .use_transform_domain_distortion > 0 as core::ffi::c_uint
        && block_mse_q8 >= (*txfm_params).tx_domain_dist_threshold
        && txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            != TX_64X64 as core::ffi::c_int && dc_only_blk == 0) as core::ffi::c_int;
    let mut calc_pixel_domain_distortion_final: core::ffi::c_int = ((*txfm_params)
        .use_transform_domain_distortion == 1 as core::ffi::c_uint
        && use_transform_domain_distortion != 0
        && (*x).rd_model as core::ffi::c_int != LOW_TXFM_RD as core::ffi::c_int)
        as core::ffi::c_int;
    if calc_pixel_domain_distortion_final != 0
        && ((txk_allowed as core::ffi::c_int) < TX_TYPES as core::ffi::c_int
            || allowed_tx_mask as core::ffi::c_int == 0x1 as core::ffi::c_int)
    {
        use_transform_domain_distortion = 0 as core::ffi::c_int;
        calc_pixel_domain_distortion_final = use_transform_domain_distortion;
    }
    let mut eobs_ptr: *const uint16_t = (*x).plane[plane as usize].eobs;
    let mut txfm_param: TxfmParam = txfm_param {
        tx_type: 0,
        tx_size: 0,
        lossless: 0,
        bd: 0,
        is_hbd: 0,
        tx_set_type: 0,
        eob: 0,
    };
    let mut quant_param: QUANT_PARAM = QUANT_PARAM {
        log_scale: 0,
        tx_size: 0,
        qmatrix: 0 as *const qm_val_t,
        iqmatrix: 0 as *const qm_val_t,
        use_quant_b_adapt: 0,
        use_optimize_b: 0,
        xform_quant_idx: 0,
    };
    let mut skip_trellis_based_on_satd: [core::ffi::c_int; 16] = [0
        as core::ffi::c_int; 16];
    av1_setup_xform(
        cm,
        x,
        tx_size,
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        &raw mut txfm_param,
    );
    av1_setup_quant(
        tx_size,
        (skip_trellis == 0) as core::ffi::c_int,
        if skip_trellis != 0 {
            if USE_B_QUANT_NO_TRELLIS != 0 {
                AV1_XFORM_QUANT_B as core::ffi::c_int
            } else {
                AV1_XFORM_QUANT_FP as core::ffi::c_int
            }
        } else {
            AV1_XFORM_QUANT_FP as core::ffi::c_int
        },
        (*cpi).oxcf.q_cfg.quant_b_adapt,
        &raw mut quant_param,
    );
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < TX_TYPES as core::ffi::c_int {
        let tx_type_0: TX_TYPE = txk_map[idx as usize] as TX_TYPE;
        if !(tx_type_0 as core::ffi::c_int == TX_TYPE_INVALID as core::ffi::c_int
            || !check_bit_mask(allowed_tx_mask, tx_type_0 as core::ffi::c_int))
        {
            txfm_param.tx_type = tx_type_0;
            if av1_use_qmatrix(
                &(*cm).quant_params,
                xd,
                (*mbmi).segment_id() as core::ffi::c_int,
            ) {
                av1_setup_qmatrix(
                    &(*cm).quant_params,
                    xd,
                    plane,
                    tx_size,
                    tx_type_0,
                    &raw mut quant_param,
                );
            }
            if plane == 0 as core::ffi::c_int {
                *((*xd).tx_type_map).offset(tx_type_map_idx as isize) = tx_type_0
                    as uint8_t;
            }
            let mut this_rd_stats: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            av1_invalid_rd_stats(&raw mut this_rd_stats);
            if is_border_block != 0 {
                av1_subtract_txb(
                    x,
                    plane,
                    plane_bsize,
                    blk_col,
                    blk_row,
                    tx_size,
                    tx_type_0,
                    (*cpi).do_border_pad,
                );
            }
            if dc_only_blk == 0 {
                av1_xform(
                    x,
                    plane,
                    block,
                    blk_row,
                    blk_col,
                    plane_bsize,
                    &raw mut txfm_param,
                );
            } else {
                av1_xform_dc_only(x, plane, block, &raw mut txfm_param, per_px_mean);
            }
            skip_trellis_based_on_satd[tx_type_0 as usize] = skip_trellis_opt_based_on_satd(
                x,
                &raw mut quant_param,
                plane,
                block,
                tx_size,
                (*cpi).oxcf.q_cfg.quant_b_adapt,
                qstep,
                (*txfm_params).coeff_opt_thresholds[1 as core::ffi::c_int as usize],
                skip_trellis,
                dc_only_blk,
            );
            av1_quant(x, plane, block, &raw mut txfm_param, &raw mut quant_param);
            if quant_param.use_optimize_b != 0 {
                av1_optimize_b(
                    cpi as *const AV1_COMP,
                    x,
                    plane,
                    block,
                    tx_size,
                    tx_type_0,
                    txb_ctx,
                    &raw mut rate_cost,
                );
            } else {
                rate_cost = cost_coeffs(
                    x,
                    plane,
                    block,
                    tx_size,
                    tx_type_0,
                    txb_ctx,
                    (*cm).features.reduced_tx_set_used as core::ffi::c_int,
                );
            }
            if !((rate_cost as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                    as int64_t > best_rd)
            {
                if *eobs_ptr.offset(block as isize) as core::ffi::c_int
                    == 0 as core::ffi::c_int
                {
                    this_rd_stats.sse = block_sse;
                    this_rd_stats.dist = this_rd_stats.sse;
                } else if dc_only_blk != 0 {
                    this_rd_stats.sse = block_sse;
                    this_rd_stats.dist = dist_block_px_domain(
                        cpi,
                        x,
                        plane,
                        plane_bsize,
                        block,
                        blk_row,
                        blk_col,
                        tx_size,
                    );
                } else if use_transform_domain_distortion != 0 {
                    let scan_order: *const SCAN_ORDER = get_scan(
                        txfm_param.tx_size,
                        txfm_param.tx_type,
                    ) as *const SCAN_ORDER;
                    dist_block_tx_domain(
                        x,
                        plane,
                        block,
                        tx_size,
                        quant_param.qmatrix,
                        (*scan_order).scan,
                        &raw mut this_rd_stats.dist,
                        &raw mut this_rd_stats.sse,
                    );
                } else {
                    let mut sse_diff: int64_t = INT64_MAX as int64_t;
                    let high_energy_thresh: int64_t = 128 as core::ffi::c_int as int64_t
                        * 128 as int64_t * tx_size_2d[tx_size as usize] as int64_t;
                    let is_high_energy: core::ffi::c_int = (block_sse
                        >= high_energy_thresh) as core::ffi::c_int;
                    if tx_size as core::ffi::c_int == TX_64X64 as core::ffi::c_int
                        || is_high_energy != 0
                    {
                        let scan_order_0: *const SCAN_ORDER = get_scan(
                            txfm_param.tx_size,
                            txfm_param.tx_type,
                        ) as *const SCAN_ORDER;
                        dist_block_tx_domain(
                            x,
                            plane,
                            block,
                            tx_size,
                            quant_param.qmatrix,
                            (*scan_order_0).scan,
                            &raw mut this_rd_stats.dist,
                            &raw mut this_rd_stats.sse,
                        );
                        sse_diff = block_sse - this_rd_stats.sse;
                    }
                    if tx_size as core::ffi::c_int != TX_64X64 as core::ffi::c_int
                        || is_high_energy == 0
                        || (sse_diff * 2 as int64_t) < this_rd_stats.sse
                    {
                        let tx_domain_dist: int64_t = this_rd_stats.dist;
                        this_rd_stats.dist = dist_block_px_domain(
                            cpi,
                            x,
                            plane,
                            plane_bsize,
                            block,
                            blk_row,
                            blk_col,
                            tx_size,
                        );
                        if is_high_energy != 0 && this_rd_stats.dist < tx_domain_dist {
                            this_rd_stats.dist = tx_domain_dist;
                        }
                    } else {
                        if !(sse_diff < 9223372036854775807 as int64_t)
                            as core::ffi::c_int as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 15],
                                    [core::ffi::c_char; 15],
                                >(*b"search_tx_type\0"))
                                    .as_ptr(),
                                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                                2285 as core::ffi::c_int,
                                b"sse_diff < INT64_MAX\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        this_rd_stats.dist += sse_diff;
                    }
                    this_rd_stats.sse = block_sse;
                }
                this_rd_stats.rate = rate_cost;
                let rd: int64_t = (this_rd_stats.rate as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + this_rd_stats.dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                if rd < best_rd {
                    best_rd = rd;
                    *best_rd_stats = this_rd_stats;
                    best_tx_type = tx_type_0;
                    best_txb_ctx = *((*x).plane[plane as usize].txb_entropy_ctx)
                        .offset(block as isize);
                    best_eob = *((*x).plane[plane as usize].eobs).offset(block as isize);
                    let tmp_dqcoeff: *mut tran_low_t = best_dqcoeff;
                    best_dqcoeff = (*p).dqcoeff;
                    (*p).dqcoeff = tmp_dqcoeff;
                }
                if (*cpi).sf.tx_sf.adaptive_txb_search_level != 0 {
                    if best_rd - (best_rd >> (*cpi).sf.tx_sf.adaptive_txb_search_level)
                        > ref_best_rd
                    {
                        break;
                    }
                }
                if (*cpi).sf.tx_sf.tx_type_search.skip_tx_search != 0 && best_eob == 0 {
                    break;
                }
            }
        }
        idx += 1;
    }
    if !(best_rd != 9223372036854775807 as int64_t) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"search_tx_type\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2364 as core::ffi::c_int,
            b"best_rd != INT64_MAX\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*best_rd_stats).skip_txfm = (best_eob as core::ffi::c_int == 0 as core::ffi::c_int)
        as core::ffi::c_int as uint8_t;
    if plane == 0 as core::ffi::c_int {
        update_txk_array(xd, blk_row, blk_col, tx_size, best_tx_type);
    }
    *((*x).plane[plane as usize].txb_entropy_ctx).offset(block as isize) = best_txb_ctx;
    *((*x).plane[plane as usize].eobs).offset(block as isize) = best_eob;
    skip_trellis = skip_trellis_based_on_satd[best_tx_type as usize];
    (*p).dqcoeff = best_dqcoeff;
    if calc_pixel_domain_distortion_final != 0 && best_eob as core::ffi::c_int != 0 {
        (*best_rd_stats).dist = dist_block_px_domain(
            cpi,
            x,
            plane,
            plane_bsize,
            block,
            blk_row,
            blk_col,
            tx_size,
        );
        (*best_rd_stats).sse = block_sse;
    }
    if is_border_block != 0 {
        av1_subtract_txb(
            x,
            plane,
            plane_bsize,
            blk_col,
            blk_row,
            tx_size,
            best_tx_type,
            (*cpi).do_border_pad,
        );
    }
    recon_intra(
        cpi,
        x,
        plane,
        block,
        blk_row,
        blk_col,
        plane_bsize,
        tx_size,
        txb_ctx,
        skip_trellis,
        best_tx_type,
        0 as core::ffi::c_int,
        &raw mut rate_cost,
        best_eob,
    );
    (*p).dqcoeff = orig_dqcoeff;
}
#[inline]
unsafe extern "C-unwind" fn tx_type_rd(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut tx_size: TX_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut plane_bsize: core::ffi::c_int,
    mut txb_ctx: *mut TXB_CTX,
    mut rd_stats: *mut RD_STATS,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut ref_rdcost: int64_t,
) {
    if (is_inter_block(*((*x).e_mbd.mi).offset(0 as core::ffi::c_int as isize)) == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"tx_type_rd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2401 as core::ffi::c_int,
            b"is_inter_block(x->e_mbd.mi[0])\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut this_rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    search_tx_type(
        cpi,
        x,
        0 as core::ffi::c_int,
        block,
        blk_row,
        blk_col,
        plane_bsize as BLOCK_SIZE,
        tx_size,
        txb_ctx,
        ftxs_mode,
        ref_rdcost,
        &raw mut this_rd_stats,
    );
    av1_merge_rd_stats(rd_stats, &raw mut this_rd_stats);
}
#[inline]
unsafe extern "C-unwind" fn try_tx_block_no_split(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut ta: *const ENTROPY_CONTEXT,
    mut tl: *const ENTROPY_CONTEXT,
    mut txfm_partition_ctx: core::ffi::c_int,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut no_split: *mut TxCandidateInfo,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblock_plane;
    let pta: *const ENTROPY_CONTEXT = ta.offset(blk_col as isize);
    let ptl: *const ENTROPY_CONTEXT = tl.offset(blk_row as isize);
    let txs_ctx: TX_SIZE = get_txsize_entropy_ctx(tx_size) as TX_SIZE;
    let mut txb_ctx: TXB_CTX = txb_ctx {
        txb_skip_ctx: 0,
        dc_sign_ctx: 0,
    };
    get_txb_ctx(plane_bsize, tx_size, 0 as core::ffi::c_int, pta, ptl, &raw mut txb_ctx);
    let zero_blk_rate: core::ffi::c_int = (*x)
        .coeff_costs
        .coeff_costs[txs_ctx as usize][PLANE_TYPE_Y as core::ffi::c_int as usize]
        .txb_skip_cost[txb_ctx.txb_skip_ctx as usize][1 as core::ffi::c_int as usize];
    (*rd_stats).zero_rate = zero_blk_rate;
    let index: core::ffi::c_int = av1_get_txb_size_index(plane_bsize, blk_row, blk_col)
        as core::ffi::c_int;
    (*mbmi).inter_tx_size[index as usize] = tx_size;
    tx_type_rd(
        cpi,
        x,
        tx_size,
        blk_row,
        blk_col,
        block,
        plane_bsize as core::ffi::c_int,
        &raw mut txb_ctx,
        rd_stats,
        ftxs_mode,
        ref_best_rd,
    );
    if !((*rd_stats).rate < 2147483647 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"try_tx_block_no_split\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2430 as core::ffi::c_int,
            b"rd_stats->rate < INT_MAX\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let pick_skip_txfm: core::ffi::c_int = ((*xd).lossless[(*mbmi).segment_id() as usize]
        == 0
        && ((*rd_stats).skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int
            || ((*rd_stats).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
                >= (zero_blk_rate as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*rd_stats).sse
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t))
        as core::ffi::c_int;
    if pick_skip_txfm != 0 {
        (*rd_stats).rate = zero_blk_rate;
        (*rd_stats).dist = (*rd_stats).sse;
        *((*p).eobs).offset(block as isize) = 0 as uint16_t;
        update_txk_array(
            xd,
            blk_row,
            blk_col,
            tx_size,
            DCT_DCT as core::ffi::c_int as TX_TYPE,
        );
    }
    (*rd_stats).skip_txfm = pick_skip_txfm as uint8_t;
    if tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int
        && depth < MAX_VARTX_DEPTH
    {
        (*rd_stats).rate
            += (*x)
                .mode_costs
                .txfm_partition_cost[txfm_partition_ctx
                as usize][0 as core::ffi::c_int as usize];
    }
    (*no_split).rd = ((*rd_stats).rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    (*no_split).txb_entropy_ctx = *((*p).txb_entropy_ctx).offset(block as isize)
        as core::ffi::c_int;
    (*no_split).tx_type = *((*xd).tx_type_map)
        .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize) as TX_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn try_tx_block_split(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut ta: *mut ENTROPY_CONTEXT,
    mut tl: *mut ENTROPY_CONTEXT,
    mut tx_above: *mut TXFM_CONTEXT,
    mut tx_left: *mut TXFM_CONTEXT,
    mut txfm_partition_ctx: core::ffi::c_int,
    mut no_split_rd: int64_t,
    mut ref_best_rd: int64_t,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut split_rd_stats: *mut RD_STATS,
) {
    if !((tx_size as core::ffi::c_int) < TX_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"try_tx_block_split\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2463 as core::ffi::c_int,
            b"tx_size < TX_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let max_blocks_high: core::ffi::c_int = max_block_high(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let txb_width: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txb_height: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
    let sub_txb_width: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
    let sub_txb_height: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
    let sub_step: core::ffi::c_int = sub_txb_width * sub_txb_height;
    let nblks: core::ffi::c_int = txb_height / sub_txb_height
        * (txb_width / sub_txb_width);
    if !(nblks > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"try_tx_block_split\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2475 as core::ffi::c_int,
            b"nblks > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_init_rd_stats(split_rd_stats);
    (*split_rd_stats).rate = (*x)
        .mode_costs
        .txfm_partition_cost[txfm_partition_ctx
        as usize][1 as core::ffi::c_int as usize];
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut blk_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < txb_height {
        let offsetr: core::ffi::c_int = blk_row + r;
        if offsetr >= max_blocks_high {
            break;
        }
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < txb_width {
            if !(blk_idx < 4 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"try_tx_block_split\0"))
                        .as_ptr(),
                    b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                    2484 as core::ffi::c_int,
                    b"blk_idx < 4\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let offsetc: core::ffi::c_int = blk_col + c;
            if !(offsetc >= max_blocks_wide) {
                let mut this_rd_stats: RD_STATS = RD_STATS {
                    rate: 0,
                    zero_rate: 0,
                    dist: 0,
                    rdcost: 0,
                    sse: 0,
                    skip_txfm: 0,
                };
                let mut this_cost_valid: core::ffi::c_int = 1 as core::ffi::c_int;
                select_tx_block(
                    cpi,
                    x,
                    offsetr,
                    offsetc,
                    block,
                    sub_txs,
                    depth + 1 as core::ffi::c_int,
                    plane_bsize,
                    ta,
                    tl,
                    tx_above,
                    tx_left,
                    &raw mut this_rd_stats,
                    no_split_rd / nblks as int64_t,
                    ref_best_rd - (*split_rd_stats).rdcost,
                    &raw mut this_cost_valid,
                    ftxs_mode,
                    -(1 as core::ffi::c_int),
                );
                if this_cost_valid == 0 {
                    (*split_rd_stats).rdcost = INT64_MAX as int64_t;
                    return;
                }
                av1_merge_rd_stats(split_rd_stats, &raw mut this_rd_stats);
                (*split_rd_stats).rdcost = ((*split_rd_stats).rate as int64_t
                    * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*split_rd_stats).dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                if (*split_rd_stats).rdcost > ref_best_rd {
                    (*split_rd_stats).rdcost = INT64_MAX as int64_t;
                    return;
                }
                block += sub_step;
            }
            c += sub_txb_width;
            blk_idx += 1;
        }
        r += sub_txb_height;
    }
}
unsafe extern "C-unwind" fn get_var(
    mut mean: core::ffi::c_float,
    mut x2_sum: core::ffi::c_double,
    mut num: core::ffi::c_int,
) -> core::ffi::c_float {
    let e_x2: core::ffi::c_float = (x2_sum / num as core::ffi::c_double)
        as core::ffi::c_float;
    let diff: core::ffi::c_float = e_x2 - mean * mean;
    return diff;
}
#[inline]
unsafe extern "C-unwind" fn get_blk_var_dev(
    mut data: *const int16_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut dev_of_mean: *mut core::ffi::c_float,
    mut var_of_vars: *mut core::ffi::c_float,
) {
    let data_ptr: *const int16_t = &*data.offset(0 as core::ffi::c_int as isize)
        as *const int16_t;
    let subh: core::ffi::c_int = if bh >= bw { bh >> 1 as core::ffi::c_int } else { bh };
    let subw: core::ffi::c_int = if bw >= bh { bw >> 1 as core::ffi::c_int } else { bw };
    let num: core::ffi::c_int = bw * bh;
    let sub_num: core::ffi::c_int = subw * subh;
    let mut total_x_sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut total_x2_sum: int64_t = 0 as int64_t;
    let mut blk_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut var_sum: core::ffi::c_float = 0.0f32;
    let mut mean_sum: core::ffi::c_float = 0.0f32;
    let mut var2_sum: core::ffi::c_double = 0.0f64;
    let mut mean2_sum: core::ffi::c_double = 0.0f64;
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < bh {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < bw {
            let mut x_sum: core::ffi::c_int = 0;
            let mut x2_sum: int64_t = 0;
            aom_get_blk_sse_sum_c(
                data_ptr.offset((row * stride) as isize).offset(col as isize),
                stride,
                subw,
                subh,
                &raw mut x_sum,
                &raw mut x2_sum,
            );
            total_x_sum += x_sum;
            total_x2_sum += x2_sum;
            let mean: core::ffi::c_float = x_sum as core::ffi::c_float
                / sub_num as core::ffi::c_float;
            let var: core::ffi::c_float = get_var(
                mean,
                x2_sum as core::ffi::c_double,
                sub_num,
            ) as core::ffi::c_float;
            mean_sum += mean;
            mean2_sum += (mean * mean) as core::ffi::c_double;
            var_sum += var;
            var2_sum += (var * var) as core::ffi::c_double;
            blk_idx += 1;
            col += subw;
        }
        row += subh;
    }
    let lvl0_mean: core::ffi::c_float = total_x_sum as core::ffi::c_float
        / num as core::ffi::c_float;
    let block_var: core::ffi::c_float = get_var(
        lvl0_mean,
        total_x2_sum as core::ffi::c_double,
        num,
    ) as core::ffi::c_float;
    mean_sum += lvl0_mean;
    mean2_sum += (lvl0_mean * lvl0_mean) as core::ffi::c_double;
    var_sum += block_var;
    var2_sum += (block_var * block_var) as core::ffi::c_double;
    let av_mean: core::ffi::c_float = mean_sum
        / 5 as core::ffi::c_int as core::ffi::c_float;
    if blk_idx > 1 as core::ffi::c_int {
        *dev_of_mean = get_dev(av_mean, mean2_sum, blk_idx + 1 as core::ffi::c_int);
        let mean_var: core::ffi::c_float = var_sum
            / (blk_idx + 1 as core::ffi::c_int) as core::ffi::c_float;
        *var_of_vars = get_var(mean_var, var2_sum, blk_idx + 1 as core::ffi::c_int);
    }
}
unsafe extern "C-unwind" fn prune_tx_split_no_split(
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut try_no_split: *mut core::ffi::c_int,
    mut try_split: *mut core::ffi::c_int,
    mut pruning_level: core::ffi::c_int,
) {
    let diff_stride: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut diff: *const int16_t = ((*x).plane[0 as core::ffi::c_int as usize].src_diff)
        .offset((4 as core::ffi::c_int * blk_row * diff_stride) as isize)
        .offset((4 as core::ffi::c_int * blk_col) as isize);
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut dev_of_means: core::ffi::c_float = 0.0f32;
    let mut var_of_vars: core::ffi::c_float = 0.0f32;
    get_blk_var_dev(diff, diff_stride, bw, bh, &raw mut dev_of_means, &raw mut var_of_vars);
    let dc_q: core::ffi::c_int = *((*x)
        .plane[0 as core::ffi::c_int as usize]
        .dequant_QTX)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        >> 3 as core::ffi::c_int;
    let ac_q: core::ffi::c_int = *((*x)
        .plane[0 as core::ffi::c_int as usize]
        .dequant_QTX)
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        >> 3 as core::ffi::c_int;
    let no_split_thresh_scales: [core::ffi::c_int; 4] = [
        0 as core::ffi::c_int,
        24 as core::ffi::c_int,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    ];
    let no_split_thresh_scale: core::ffi::c_int = no_split_thresh_scales[pruning_level
        as usize];
    let split_thresh_scales: [core::ffi::c_int; 4] = [
        0 as core::ffi::c_int,
        24 as core::ffi::c_int,
        10 as core::ffi::c_int,
        8 as core::ffi::c_int,
    ];
    let split_thresh_scale: core::ffi::c_int = split_thresh_scales[pruning_level
        as usize];
    if dev_of_means <= dc_q as core::ffi::c_float
        && split_thresh_scale as core::ffi::c_float * var_of_vars
            <= (ac_q * ac_q) as core::ffi::c_float
    {
        *try_split = 0 as core::ffi::c_int;
    }
    if dev_of_means > (no_split_thresh_scale * dc_q) as core::ffi::c_float
        && var_of_vars > (no_split_thresh_scale * ac_q * ac_q) as core::ffi::c_float
    {
        *try_no_split = 0 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn select_tx_block(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut ta: *mut ENTROPY_CONTEXT,
    mut tl: *mut ENTROPY_CONTEXT,
    mut tx_above: *mut TXFM_CONTEXT,
    mut tx_left: *mut TXFM_CONTEXT,
    mut rd_stats: *mut RD_STATS,
    mut prev_level_rd: int64_t,
    mut ref_best_rd: int64_t,
    mut is_cost_valid: *mut core::ffi::c_int,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
    mut blk_idx: core::ffi::c_int,
) {
    if !((tx_size as core::ffi::c_int) < TX_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"select_tx_block\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2610 as core::ffi::c_int,
            b"tx_size < TX_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_init_rd_stats(rd_stats);
    if ref_best_rd < 0 as int64_t {
        *is_cost_valid = 0 as core::ffi::c_int;
        return;
    }
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if !(blk_row < max_block_high(xd, plane_bsize, 0 as core::ffi::c_int)
        && blk_col < max_block_wide(xd, plane_bsize, 0 as core::ffi::c_int))
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"select_tx_block\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2619 as core::ffi::c_int,
            b"blk_row < max_block_high(xd, plane_bsize, 0) && blk_col < max_block_wide(xd, plane_bsize, 0)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let ctx: core::ffi::c_int = txfm_partition_context(
        tx_above.offset(blk_col as isize),
        tx_left.offset(blk_row as isize),
        (*mbmi).bsize,
        tx_size,
    ) as core::ffi::c_int;
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblock_plane;
    let mut try_no_split: core::ffi::c_int = (((*cpi).oxcf.txfm_cfg.enable_tx64
        as core::ffi::c_int != 0
        || txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            != TX_64X64 as core::ffi::c_int)
        && ((*cpi).oxcf.txfm_cfg.enable_rect_tx as core::ffi::c_int != 0
            || tx_size_wide[tx_size as usize] == tx_size_high[tx_size as usize]))
        as core::ffi::c_int;
    let mut try_split: core::ffi::c_int = (tx_size as core::ffi::c_int
        > TX_4X4 as core::ffi::c_int && depth < MAX_VARTX_DEPTH) as core::ffi::c_int;
    let mut no_split: TxCandidateInfo = {
        let mut init = TxCandidateInfo {
            rd: INT64_MAX as int64_t,
            txb_entropy_ctx: 0 as core::ffi::c_int,
            tx_type: TX_TYPES as core::ffi::c_int as TX_TYPE,
        };
        init
    };
    if tx_size as core::ffi::c_int != TX_4X4 as core::ffi::c_int
        && try_split == 1 as core::ffi::c_int && try_no_split == 1 as core::ffi::c_int
        && (*cpi).sf.tx_sf.prune_tx_size_level > 0 as core::ffi::c_int
    {
        prune_tx_split_no_split(
            x,
            plane_bsize,
            blk_row,
            blk_col,
            tx_size,
            &raw mut try_no_split,
            &raw mut try_split,
            (*cpi).sf.tx_sf.prune_tx_size_level,
        );
    }
    if (*cpi).sf.rt_sf.skip_tx_no_split_var_based_partition != 0 {
        if (*x).try_merge_partition != 0 && try_split != 0
            && *((*p).eobs).offset(block as isize) as core::ffi::c_int != 0
        {
            try_no_split = 0 as core::ffi::c_int;
        }
    }
    if try_no_split != 0 {
        try_tx_block_no_split(
            cpi,
            x,
            blk_row,
            blk_col,
            block,
            tx_size,
            depth,
            plane_bsize,
            ta,
            tl,
            ctx,
            rd_stats,
            ref_best_rd,
            ftxs_mode,
            &raw mut no_split,
        );
        push_inter_block_tx_no_split_rd(
            x,
            mbmi,
            no_split.rd,
            blk_idx,
            (*cpi).sf.tx_sf.prune_inter_tx_split_rd_eval_lvl,
        );
        let search_level: core::ffi::c_int = (*cpi).sf.tx_sf.adaptive_txb_search_level;
        if search_level != 0 {
            if no_split.rd - (no_split.rd >> 1 as core::ffi::c_int + search_level)
                > ref_best_rd
            {
                *is_cost_valid = 0 as core::ffi::c_int;
                return;
            }
            if no_split.rd - (no_split.rd >> 2 as core::ffi::c_int + search_level)
                > prev_level_rd
            {
                try_split = 0 as core::ffi::c_int;
            }
        }
        if (*cpi).sf.tx_sf.txb_split_cap != 0 {
            if *((*p).eobs).offset(block as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                try_split = 0 as core::ffi::c_int;
            }
        }
        if prune_tx_split_eval_using_no_split_rd(
            x,
            mbmi,
            no_split.rd,
            blk_idx,
            (*cpi).sf.tx_sf.prune_inter_tx_split_rd_eval_lvl,
        ) {
            try_split = 0 as core::ffi::c_int;
        }
    }
    if (*x).e_mbd.bd == 8 as core::ffi::c_int && try_split != 0
        && !(ref_best_rd == INT64_MAX as int64_t && no_split.rd == INT64_MAX as int64_t)
    {
        let threshold: core::ffi::c_int = (*cpi)
            .sf
            .tx_sf
            .tx_type_search
            .ml_tx_split_thresh;
        if threshold >= 0 as core::ffi::c_int {
            let split_score: core::ffi::c_int = ml_predict_tx_split(
                x,
                plane_bsize,
                blk_row,
                blk_col,
                tx_size,
            ) as core::ffi::c_int;
            if split_score < -threshold {
                try_split = 0 as core::ffi::c_int;
            }
        }
    }
    let mut split_rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    split_rd_stats.rdcost = INT64_MAX as int64_t;
    if try_split != 0 {
        try_tx_block_split(
            cpi,
            x,
            blk_row,
            blk_col,
            block,
            tx_size,
            depth,
            plane_bsize,
            ta,
            tl,
            tx_above,
            tx_left,
            ctx,
            no_split.rd,
            if no_split.rd < ref_best_rd { no_split.rd } else { ref_best_rd },
            ftxs_mode,
            &raw mut split_rd_stats,
        );
    }
    if no_split.rd < split_rd_stats.rdcost {
        let mut pta: *mut ENTROPY_CONTEXT = ta.offset(blk_col as isize);
        let mut ptl: *mut ENTROPY_CONTEXT = tl.offset(blk_row as isize);
        *((*p).txb_entropy_ctx).offset(block as isize) = no_split.txb_entropy_ctx
            as uint8_t;
        av1_set_txb_context(x, 0 as core::ffi::c_int, block, tx_size, pta, ptl);
        txfm_partition_update(
            tx_above.offset(blk_col as isize),
            tx_left.offset(blk_row as isize),
            tx_size,
            tx_size,
        );
        let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
        while idy < tx_size_high_unit[tx_size as usize] {
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < tx_size_wide_unit[tx_size as usize] {
                let index: core::ffi::c_int = av1_get_txb_size_index(
                    plane_bsize,
                    blk_row + idy,
                    blk_col + idx,
                ) as core::ffi::c_int;
                (*mbmi).inter_tx_size[index as usize] = tx_size;
                idx += 1;
            }
            idy += 1;
        }
        (*mbmi).tx_size = tx_size;
        update_txk_array(xd, blk_row, blk_col, tx_size, no_split.tx_type);
    } else {
        *rd_stats = split_rd_stats;
        if split_rd_stats.rdcost == INT64_MAX as int64_t {
            *is_cost_valid = 0 as core::ffi::c_int;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn choose_largest_tx_size(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut bs: BLOCK_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    (*mbmi).tx_size = tx_size_from_tx_mode(bs, (*txfm_params).tx_mode_search_type);
    if !(*cpi).oxcf.txfm_cfg.enable_tx64
        && (*cpi).oxcf.txfm_cfg.enable_rect_tx as core::ffi::c_int != 0
    {
        static mut tx_size_max_32: [TX_SIZE; 19] = [
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_4X8 as core::ffi::c_int as TX_SIZE,
            TX_8X4 as core::ffi::c_int as TX_SIZE,
            TX_8X16 as core::ffi::c_int as TX_SIZE,
            TX_16X8 as core::ffi::c_int as TX_SIZE,
            TX_16X32 as core::ffi::c_int as TX_SIZE,
            TX_32X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_4X16 as core::ffi::c_int as TX_SIZE,
            TX_16X4 as core::ffi::c_int as TX_SIZE,
            TX_8X32 as core::ffi::c_int as TX_SIZE,
            TX_32X8 as core::ffi::c_int as TX_SIZE,
            TX_16X32 as core::ffi::c_int as TX_SIZE,
            TX_32X16 as core::ffi::c_int as TX_SIZE,
        ];
        (*mbmi).tx_size = tx_size_max_32[(*mbmi).tx_size as usize];
    } else if (*cpi).oxcf.txfm_cfg.enable_tx64 as core::ffi::c_int != 0
        && !(*cpi).oxcf.txfm_cfg.enable_rect_tx
    {
        static mut tx_size_max_square: [TX_SIZE; 19] = [
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_64X64 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
        ];
        (*mbmi).tx_size = tx_size_max_square[(*mbmi).tx_size as usize];
    } else if !(*cpi).oxcf.txfm_cfg.enable_tx64 && !(*cpi).oxcf.txfm_cfg.enable_rect_tx {
        static mut tx_size_max_32_square: [TX_SIZE; 19] = [
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_32X32 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_4X4 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_8X8 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
            TX_16X16 as core::ffi::c_int as TX_SIZE,
        ];
        (*mbmi).tx_size = tx_size_max_32_square[(*mbmi).tx_size as usize];
    }
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let no_skip_txfm_rate: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let skip_txfm_rate: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let skip_txfm_rd: int64_t = if is_inter_block(mbmi) != 0 {
        (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                as int64_t
    } else {
        INT64_MAX as int64_t
    };
    let no_skip_txfm_rd: int64_t = (no_skip_txfm_rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    av1_txfm_rd_in_plane(
        x,
        cpi,
        rd_stats,
        ref_best_rd,
        if no_skip_txfm_rd < skip_txfm_rd { no_skip_txfm_rd } else { skip_txfm_rd },
        AOM_PLANE_Y,
        bs,
        (*mbmi).tx_size,
        FTXS_NONE as core::ffi::c_int as FAST_TX_SEARCH_MODE,
    );
}
#[inline]
unsafe extern "C-unwind" fn choose_smallest_tx_size(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut bs: BLOCK_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
    av1_txfm_rd_in_plane(
        x,
        cpi,
        rd_stats,
        ref_best_rd,
        0 as int64_t,
        0 as core::ffi::c_int,
        bs,
        (*mbmi).tx_size,
        FTXS_NONE as core::ffi::c_int as FAST_TX_SEARCH_MODE,
    );
}
unsafe extern "C-unwind" fn ml_predict_intra_tx_depth_prune(
    mut x: *mut MACROBLOCK,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) {
    let xd: *const MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0
        || txsize_to_bsize[tx_size as usize] as core::ffi::c_int
            != bsize as core::ffi::c_int || (*xd).bd != 8 as core::ffi::c_int
    {
        return;
    }
    if tx_size as core::ffi::c_int != TX_8X8 as core::ffi::c_int {
        return;
    }
    let nn_config: *const NN_CONFIG = &raw const av1_intra_tx_split_nnconfig_8x8;
    let intra_tx_prune_thresh: *const core::ffi::c_float = core::ptr::addr_of!(av1_intra_tx_prune_nn_thresh_8x8).cast::<core::ffi::c_float>();
    let mut features: [core::ffi::c_float; 14] = [
        0.0f32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let diff_stride: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut diff: *const int16_t = ((*x).plane[0 as core::ffi::c_int as usize].src_diff)
        .offset((MI_SIZE * blk_row * diff_stride) as isize)
        .offset((MI_SIZE * blk_col) as isize);
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut feature_idx: core::ffi::c_int = get_mean_dev_features(
        diff,
        diff_stride,
        bw,
        bh,
        features.as_mut_ptr(),
    );
    let fresh6 = feature_idx;
    feature_idx = feature_idx + 1;
    features[fresh6 as usize] = log1pf((*x).source_variance as core::ffi::c_float);
    let dc_q: core::ffi::c_int = av1_dc_quant_QTX(
        (*x).qindex,
        0 as core::ffi::c_int,
        (*xd).bd as aom_bit_depth_t,
    ) as core::ffi::c_int >> (*xd).bd - 8 as core::ffi::c_int;
    let log_dc_q_square: core::ffi::c_float = log1pf(
        (dc_q * dc_q) as core::ffi::c_float / 256.0f32,
    ) as core::ffi::c_float;
    let fresh7 = feature_idx;
    feature_idx = feature_idx + 1;
    features[fresh7 as usize] = log_dc_q_square;
    if !(feature_idx == 14 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"ml_predict_intra_tx_depth_prune\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2874 as core::ffi::c_int,
            b"feature_idx == NUM_INTRA_TX_SPLIT_FEATURES\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < NUM_INTRA_TX_SPLIT_FEATURES {
        features[i as usize] = (features[i as usize]
            - av1_intra_tx_split_8x8_mean[i as usize])
            / av1_intra_tx_split_8x8_std[i as usize];
        i += 1;
    }
    let mut score: core::ffi::c_float = 0.;
    av1_nn_predict_c(
        features.as_mut_ptr(),
        nn_config,
        1 as core::ffi::c_int,
        &raw mut score,
    );
    let txfm_params: *mut TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if score <= *intra_tx_prune_thresh.offset(0 as core::ffi::c_int as isize) {
        (*txfm_params).nn_prune_depths_for_intra_tx = TX_PRUNE_SPLIT as core::ffi::c_int
            as TX_PRUNE_TYPE;
    } else if score > *intra_tx_prune_thresh.offset(1 as core::ffi::c_int as isize) {
        (*txfm_params).nn_prune_depths_for_intra_tx = TX_PRUNE_LARGEST
            as core::ffi::c_int as TX_PRUNE_TYPE;
    }
}
unsafe extern "C-unwind" fn uniform_txfm_yrd(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut bs: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
) -> int64_t {
    if !(is_rect_tx(tx_size) == 0 || is_rect_tx_allowed_bsize(bs) != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"uniform_txfm_yrd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            2912 as core::ffi::c_int,
            b"IMPLIES(is_rect_tx(tx_size), is_rect_tx_allowed_bsize(bs))\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let tx_select: core::ffi::c_int = ((*txfm_params).tx_mode_search_type
        as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int
        && block_signals_txsize((*mbmi).bsize) != 0) as core::ffi::c_int;
    let mut tx_size_rate: core::ffi::c_int = 0 as core::ffi::c_int;
    if tx_select != 0 {
        let ctx: core::ffi::c_int = txfm_partition_context(
            (*xd).above_txfm_context,
            (*xd).left_txfm_context,
            (*mbmi).bsize,
            tx_size,
        ) as core::ffi::c_int;
        tx_size_rate = if is_inter != 0 {
            (*mode_costs)
                .txfm_partition_cost[ctx as usize][0 as core::ffi::c_int as usize]
        } else {
            tx_size_cost(x, bs, tx_size)
        };
    }
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let no_skip_txfm_rate: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let skip_txfm_rate: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let skip_txfm_rd: int64_t = if is_inter != 0 {
        (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                as int64_t
    } else {
        INT64_MAX as int64_t
    };
    let no_this_rd: int64_t = ((no_skip_txfm_rate + tx_size_rate) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    (*mbmi).tx_size = tx_size;
    av1_txfm_rd_in_plane(
        x,
        cpi,
        rd_stats,
        ref_best_rd,
        if no_this_rd < skip_txfm_rd { no_this_rd } else { skip_txfm_rd },
        AOM_PLANE_Y,
        bs,
        tx_size,
        ftxs_mode,
    );
    if (*rd_stats).rate == INT_MAX {
        return INT64_MAX as int64_t;
    }
    let mut rd: int64_t = 0;
    if (*rd_stats).skip_txfm as core::ffi::c_int != 0 && is_inter != 0 {
        rd = (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    } else {
        rd = (((*rd_stats).rate + no_skip_txfm_rate + tx_size_rate) as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        (*rd_stats).rate += tx_size_rate;
    }
    if is_inter != 0 && (*rd_stats).skip_txfm == 0
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
    {
        let mut temp_skip_txfm_rd: int64_t = (skip_txfm_rate as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if temp_skip_txfm_rd <= rd {
            rd = temp_skip_txfm_rd;
            (*rd_stats).rate = 0 as core::ffi::c_int;
            (*rd_stats).dist = (*rd_stats).sse;
            (*rd_stats).skip_txfm = 1 as uint8_t;
        }
    }
    return rd;
}
#[inline]
unsafe extern "C-unwind" fn choose_tx_size_type_from_rd(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut bs: BLOCK_SIZE,
) {
    av1_invalid_rd_stats(rd_stats);
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let txfm_params: *mut TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let max_rect_tx_size: TX_SIZE = max_txsize_rect_lookup[bs as usize];
    let tx_select: core::ffi::c_int = ((*txfm_params).tx_mode_search_type
        as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int) as core::ffi::c_int;
    let mut start_tx: core::ffi::c_int = 0;
    let mut init_depth: core::ffi::c_int = 0;
    if tx_select != 0 {
        start_tx = max_rect_tx_size as core::ffi::c_int;
        init_depth = get_search_init_depth(
            mi_size_wide[bs as usize] as core::ffi::c_int,
            mi_size_high[bs as usize] as core::ffi::c_int,
            is_inter_block(mbmi),
            &(*cpi).sf,
            (*txfm_params).tx_size_search_method as core::ffi::c_int,
        );
        if init_depth == MAX_TX_DEPTH && !(*cpi).oxcf.txfm_cfg.enable_tx64
            && txsize_sqr_up_map[start_tx as usize] as core::ffi::c_int
                == TX_64X64 as core::ffi::c_int
        {
            start_tx = sub_tx_size_map[start_tx as usize] as core::ffi::c_int;
        }
    } else {
        let chosen_tx_size: TX_SIZE = tx_size_from_tx_mode(
            bs,
            (*txfm_params).tx_mode_search_type,
        ) as TX_SIZE;
        start_tx = chosen_tx_size as core::ffi::c_int;
        init_depth = MAX_TX_DEPTH;
    }
    let mut best_txk_type_map: [uint8_t; 1024] = [0; 1024];
    let mut best_tx_size: TX_SIZE = max_rect_tx_size;
    let mut best_rd: int64_t = INT64_MAX as int64_t;
    let num_blks: core::ffi::c_int = bsize_to_num_blk(bs) as core::ffi::c_int;
    (*x).rd_model = FULL_TXFM_RD as core::ffi::c_int as TXFM_RD_MODEL;
    let mut rd: [int64_t; 3] = [INT64_MAX, INT64_MAX, INT64_MAX];
    let mut tx_size: core::ffi::c_int = start_tx;
    let mut depth: core::ffi::c_int = init_depth;
    while depth <= MAX_TX_DEPTH {
        if !(!(*cpi).oxcf.txfm_cfg.enable_tx64
            && txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
                == TX_64X64 as core::ffi::c_int
            || !(*cpi).oxcf.txfm_cfg.enable_rect_tx
                && tx_size_wide[tx_size as usize] != tx_size_high[tx_size as usize])
        {
            if (*txfm_params).nn_prune_depths_for_intra_tx as core::ffi::c_int
                == TX_PRUNE_SPLIT as core::ffi::c_int
            {
                break;
            }
            (*txfm_params).enable_nn_prune_intra_tx_depths = (*cpi)
                .sf
                .tx_sf
                .prune_intra_tx_depths_using_nn as core::ffi::c_int != 0
                && tx_size == start_tx;
            let mut this_rd_stats: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            let rd_thresh: int64_t = if (*cpi)
                .sf
                .tx_sf
                .use_rd_based_breakout_for_intra_tx_search as core::ffi::c_int != 0
            {
                if ref_best_rd < best_rd { ref_best_rd } else { best_rd }
            } else {
                ref_best_rd
            };
            rd[depth as usize] = uniform_txfm_yrd(
                cpi,
                x,
                &raw mut this_rd_stats,
                rd_thresh,
                bs,
                tx_size as TX_SIZE,
                FTXS_NONE as core::ffi::c_int as FAST_TX_SEARCH_MODE,
            );
            if rd[depth as usize] < best_rd {
                memcpy(
                    best_txk_type_map.as_mut_ptr() as *mut core::ffi::c_void,
                    (*xd).tx_type_map as *const core::ffi::c_void,
                    (num_blks as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
                best_tx_size = tx_size as TX_SIZE;
                best_rd = rd[depth as usize];
                *rd_stats = this_rd_stats;
            }
            if tx_size == TX_4X4 as core::ffi::c_int {
                break;
            }
            if depth > init_depth && depth != MAX_TX_DEPTH
                && (*x).source_variance < 256 as core::ffi::c_uint
            {
                if rd[(depth - 1 as core::ffi::c_int) as usize] != INT64_MAX as int64_t
                    && rd[depth as usize] > rd[(depth - 1 as core::ffi::c_int) as usize]
                {
                    break;
                }
            }
        }
        depth += 1;
        tx_size = sub_tx_size_map[tx_size as usize] as core::ffi::c_int;
    }
    if (*rd_stats).rate != INT_MAX {
        (*mbmi).tx_size = best_tx_size;
        memcpy(
            (*xd).tx_type_map as *mut core::ffi::c_void,
            best_txk_type_map.as_mut_ptr() as *const core::ffi::c_void,
            (num_blks as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    }
    (*txfm_params).enable_nn_prune_intra_tx_depths = false_0 != 0;
    (*txfm_params).nn_prune_depths_for_intra_tx = TX_PRUNE_NONE as core::ffi::c_int
        as TX_PRUNE_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn block_rd_txfm(
    mut plane: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut arg: *mut core::ffi::c_void,
) {
    let mut args: *mut rdcost_block_args = arg as *mut rdcost_block_args;
    if (*args).exit_early != 0 {
        (*args).incomplete_exit = 1 as core::ffi::c_int;
        return;
    }
    let x: *mut MACROBLOCK = (*args).x;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let is_inter: core::ffi::c_int = is_inter_block(
        *((*xd).mi).offset(0 as core::ffi::c_int as isize),
    ) as core::ffi::c_int;
    let mut cpi: *const AV1_COMP = (*args).cpi;
    let mut a: *mut ENTROPY_CONTEXT = ((*args).t_above)
        .as_mut_ptr()
        .offset(blk_col as isize);
    let mut l: *mut ENTROPY_CONTEXT = ((*args).t_left)
        .as_mut_ptr()
        .offset(blk_row as isize);
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut this_rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_init_rd_stats(&raw mut this_rd_stats);
    if is_inter == 0 {
        av1_predict_intra_block_facade(cm, xd, plane, blk_col, blk_row, tx_size);
        av1_subtract_txb(
            x,
            plane,
            plane_bsize,
            blk_col,
            blk_row,
            tx_size,
            DCT_DCT as core::ffi::c_int as TX_TYPE,
            false_0 != 0,
        );
        let txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
        if (*txfm_params).enable_nn_prune_intra_tx_depths {
            ml_predict_intra_tx_depth_prune(x, blk_row, blk_col, plane_bsize, tx_size);
            if (*txfm_params).nn_prune_depths_for_intra_tx as core::ffi::c_int
                == TX_PRUNE_LARGEST as core::ffi::c_int
            {
                av1_invalid_rd_stats(&raw mut (*args).rd_stats);
                (*args).exit_early = 1 as core::ffi::c_int;
                return;
            }
        }
    }
    let mut txb_ctx: TXB_CTX = txb_ctx {
        txb_skip_ctx: 0,
        dc_sign_ctx: 0,
    };
    get_txb_ctx(plane_bsize, tx_size, plane, a, l, &raw mut txb_ctx);
    search_tx_type(
        cpi,
        x,
        plane,
        block,
        blk_row,
        blk_col,
        plane_bsize,
        tx_size,
        &raw mut txb_ctx,
        (*args).ftxs_mode,
        (*args).best_rd - (*args).current_rd,
        &raw mut this_rd_stats,
    );
    if plane == AOM_PLANE_Y && (*xd).cfl.store_y != 0 {
        if !(is_inter == 0
            || (plane_bsize as core::ffi::c_int) < BLOCK_8X8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"block_rd_txfm\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                3113 as core::ffi::c_int,
                b"!is_inter || plane_bsize < BLOCK_8X8\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        cfl_store_tx(xd, blk_row, blk_col, tx_size, plane_bsize);
    }
    av1_set_txb_context(x, plane, block, tx_size, a, l);
    let mut rd: int64_t = 0;
    if is_inter != 0 {
        let no_skip_txfm_rd: int64_t = (this_rd_stats.rate as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + this_rd_stats.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        let skip_txfm_rd: int64_t = (0 as core::ffi::c_int as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + this_rd_stats.sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        rd = if no_skip_txfm_rd < skip_txfm_rd { no_skip_txfm_rd } else { skip_txfm_rd };
        this_rd_stats.skip_txfm = (this_rd_stats.skip_txfm as core::ffi::c_int
            & (*((*x).plane[plane as usize].eobs).offset(block as isize) == 0)
                as core::ffi::c_int) as uint8_t;
    } else {
        rd = (this_rd_stats.rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + this_rd_stats.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        this_rd_stats.skip_txfm = 0 as uint8_t;
    }
    av1_merge_rd_stats(&raw mut (*args).rd_stats, &raw mut this_rd_stats);
    (*args).current_rd += rd;
    if (*args).current_rd > (*args).best_rd {
        (*args).exit_early = 1 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_estimate_txfm_yrd(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut bs: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) -> int64_t {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let tx_select: core::ffi::c_int = ((*txfm_params).tx_mode_search_type
        as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int
        && block_signals_txsize((*mbmi).bsize) != 0) as core::ffi::c_int;
    let mut tx_size_rate: core::ffi::c_int = 0 as core::ffi::c_int;
    if tx_select != 0 {
        let ctx: core::ffi::c_int = txfm_partition_context(
            (*xd).above_txfm_context,
            (*xd).left_txfm_context,
            (*mbmi).bsize,
            tx_size,
        ) as core::ffi::c_int;
        tx_size_rate = (*mode_costs)
            .txfm_partition_cost[ctx as usize][0 as core::ffi::c_int as usize];
    }
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let no_skip_txfm_rate: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let skip_txfm_rate: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let skip_txfm_rd: int64_t = (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let no_this_rd: int64_t = ((no_skip_txfm_rate + tx_size_rate) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    (*mbmi).tx_size = tx_size;
    let txw_unit: uint8_t = tx_size_wide_unit[tx_size as usize] as uint8_t;
    let txh_unit: uint8_t = tx_size_high_unit[tx_size as usize] as uint8_t;
    let step: core::ffi::c_int = txw_unit as core::ffi::c_int
        * txh_unit as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(xd, bs, 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let max_blocks_high: core::ffi::c_int = max_block_high(xd, bs, 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut args: rdcost_block_args = rdcost_block_args {
        cpi: 0 as *const AV1_COMP,
        x: 0 as *mut MACROBLOCK,
        t_above: [0; 32],
        t_left: [0; 32],
        rd_stats: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        current_rd: 0,
        best_rd: 0,
        exit_early: 0,
        incomplete_exit: 0,
        ftxs_mode: 0,
        skip_trellis: 0,
    };
    memset(
        &raw mut args as *mut rdcost_block_args as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<rdcost_block_args>() as size_t,
    );
    args.x = x;
    args.cpi = cpi;
    args.best_rd = ref_best_rd;
    args.current_rd = if no_this_rd < skip_txfm_rd { no_this_rd } else { skip_txfm_rd };
    av1_init_rd_stats(&raw mut args.rd_stats);
    av1_get_entropy_contexts(
        bs,
        &raw mut *((*xd).plane).as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        (args.t_above).as_mut_ptr(),
        (args.t_left).as_mut_ptr(),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut blk_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while blk_row < max_blocks_high && args.incomplete_exit == 0 {
        let mut blk_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while blk_col < max_blocks_wide {
            let mut this_rd_stats: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            av1_init_rd_stats(&raw mut this_rd_stats);
            if args.exit_early != 0 {
                args.incomplete_exit = 1 as core::ffi::c_int;
                break;
            } else {
                let mut a: *mut ENTROPY_CONTEXT = (args.t_above)
                    .as_mut_ptr()
                    .offset(blk_col as isize);
                let mut l: *mut ENTROPY_CONTEXT = (args.t_left)
                    .as_mut_ptr()
                    .offset(blk_row as isize);
                let mut txb_ctx: TXB_CTX = txb_ctx {
                    txb_skip_ctx: 0,
                    dc_sign_ctx: 0,
                };
                get_txb_ctx(bs, tx_size, 0 as core::ffi::c_int, a, l, &raw mut txb_ctx);
                let mut txfm_param: TxfmParam = txfm_param {
                    tx_type: 0,
                    tx_size: 0,
                    lossless: 0,
                    bd: 0,
                    is_hbd: 0,
                    tx_set_type: 0,
                    eob: 0,
                };
                let mut quant_param: QUANT_PARAM = QUANT_PARAM {
                    log_scale: 0,
                    tx_size: 0,
                    qmatrix: 0 as *const qm_val_t,
                    iqmatrix: 0 as *const qm_val_t,
                    use_quant_b_adapt: 0,
                    use_optimize_b: 0,
                    xform_quant_idx: 0,
                };
                av1_setup_xform(
                    &(*cpi).common,
                    x,
                    tx_size,
                    DCT_DCT as core::ffi::c_int as TX_TYPE,
                    &raw mut txfm_param,
                );
                av1_setup_quant(
                    tx_size,
                    0 as core::ffi::c_int,
                    AV1_XFORM_QUANT_B as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    &raw mut quant_param,
                );
                av1_subtract_txb(
                    x,
                    PLANE_TYPE_Y as core::ffi::c_int,
                    bs,
                    blk_col,
                    blk_row,
                    tx_size,
                    DCT_DCT as core::ffi::c_int as TX_TYPE,
                    (*cpi).do_border_pad,
                );
                av1_xform(
                    x,
                    0 as core::ffi::c_int,
                    i,
                    blk_row,
                    blk_col,
                    bs,
                    &raw mut txfm_param,
                );
                av1_quant(
                    x,
                    0 as core::ffi::c_int,
                    i,
                    &raw mut txfm_param,
                    &raw mut quant_param,
                );
                this_rd_stats.rate = cost_coeffs(
                    x,
                    0 as core::ffi::c_int,
                    i,
                    tx_size,
                    txfm_param.tx_type,
                    &raw mut txb_ctx,
                    0 as core::ffi::c_int,
                );
                let scan_order: *const SCAN_ORDER = get_scan(
                    txfm_param.tx_size,
                    txfm_param.tx_type,
                ) as *const SCAN_ORDER;
                dist_block_tx_domain(
                    x,
                    0 as core::ffi::c_int,
                    i,
                    tx_size,
                    quant_param.qmatrix,
                    (*scan_order).scan,
                    &raw mut this_rd_stats.dist,
                    &raw mut this_rd_stats.sse,
                );
                let no_skip_txfm_rd: int64_t = (this_rd_stats.rate as int64_t
                    * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + this_rd_stats.dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                let skip_rd: int64_t = (0 as core::ffi::c_int as int64_t
                    * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + this_rd_stats.sse
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                this_rd_stats.skip_txfm = (this_rd_stats.skip_txfm as core::ffi::c_int
                    & (*((*x).plane[0 as core::ffi::c_int as usize].eobs)
                        .offset(i as isize) == 0) as core::ffi::c_int) as uint8_t;
                av1_merge_rd_stats(&raw mut args.rd_stats, &raw mut this_rd_stats);
                args.current_rd
                    += if no_skip_txfm_rd < skip_rd { no_skip_txfm_rd } else { skip_rd };
                if args.current_rd > ref_best_rd {
                    args.exit_early = 1 as core::ffi::c_int;
                    break;
                } else {
                    av1_set_txb_context(x, 0 as core::ffi::c_int, i, tx_size, a, l);
                    i += step;
                    blk_col += txw_unit as core::ffi::c_int;
                }
            }
        }
        blk_row += txh_unit as core::ffi::c_int;
    }
    if args.incomplete_exit != 0 {
        av1_invalid_rd_stats(&raw mut args.rd_stats);
    }
    *rd_stats = args.rd_stats;
    if (*rd_stats).rate == INT_MAX {
        return INT64_MAX as int64_t;
    }
    let mut rd: int64_t = 0;
    if (*rd_stats).skip_txfm as core::ffi::c_int != 0 && is_inter != 0 {
        rd = (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    } else {
        rd = (((*rd_stats).rate + no_skip_txfm_rate + tx_size_rate) as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        (*rd_stats).rate += tx_size_rate;
    }
    if is_inter != 0 && (*rd_stats).skip_txfm == 0
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
    {
        let mut temp_skip_txfm_rd: int64_t = (skip_txfm_rate as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if temp_skip_txfm_rd <= rd {
            rd = temp_skip_txfm_rd;
            (*rd_stats).rate = 0 as core::ffi::c_int;
            (*rd_stats).dist = (*rd_stats).sse;
            (*rd_stats).skip_txfm = 1 as uint8_t;
        }
    }
    return rd;
}
#[inline]
unsafe extern "C-unwind" fn tx_block_yrd(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut plane_bsize: BLOCK_SIZE,
    mut depth: core::ffi::c_int,
    mut above_ctx: *mut ENTROPY_CONTEXT,
    mut left_ctx: *mut ENTROPY_CONTEXT,
    mut tx_above: *mut TXFM_CONTEXT,
    mut tx_left: *mut TXFM_CONTEXT,
    mut ref_best_rd: int64_t,
    mut rd_stats: *mut RD_STATS,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
) {
    if !((tx_size as core::ffi::c_int) < TX_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"tx_block_yrd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3279 as core::ffi::c_int,
            b"tx_size < TX_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (is_inter_block(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"tx_block_yrd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3282 as core::ffi::c_int,
            b"is_inter_block(mbmi)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_blocks_high: core::ffi::c_int = max_block_high(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    let plane_tx_size: TX_SIZE = (*mbmi)
        .inter_tx_size[av1_get_txb_size_index(plane_bsize, blk_row, blk_col) as usize];
    let ctx: core::ffi::c_int = txfm_partition_context(
        tx_above.offset(blk_col as isize),
        tx_left.offset(blk_row as isize),
        (*mbmi).bsize,
        tx_size,
    ) as core::ffi::c_int;
    av1_init_rd_stats(rd_stats);
    if tx_size as core::ffi::c_int == plane_tx_size as core::ffi::c_int {
        let mut ta: *mut ENTROPY_CONTEXT = above_ctx.offset(blk_col as isize);
        let mut tl: *mut ENTROPY_CONTEXT = left_ctx.offset(blk_row as isize);
        let txs_ctx: TX_SIZE = get_txsize_entropy_ctx(tx_size) as TX_SIZE;
        let mut txb_ctx: TXB_CTX = txb_ctx {
            txb_skip_ctx: 0,
            dc_sign_ctx: 0,
        };
        get_txb_ctx(plane_bsize, tx_size, 0 as core::ffi::c_int, ta, tl, &raw mut txb_ctx);
        let zero_blk_rate: core::ffi::c_int = (*x)
            .coeff_costs
            .coeff_costs[txs_ctx
                as usize][get_plane_type(0 as core::ffi::c_int) as usize]
            .txb_skip_cost[txb_ctx.txb_skip_ctx
            as usize][1 as core::ffi::c_int as usize];
        (*rd_stats).zero_rate = zero_blk_rate;
        tx_type_rd(
            cpi,
            x,
            tx_size,
            blk_row,
            blk_col,
            block,
            plane_bsize as core::ffi::c_int,
            &raw mut txb_ctx,
            rd_stats,
            ftxs_mode,
            ref_best_rd,
        );
        if ((*rd_stats).rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
            >= (zero_blk_rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
            || (*rd_stats).skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int
        {
            (*rd_stats).rate = zero_blk_rate;
            (*rd_stats).dist = (*rd_stats).sse;
            (*rd_stats).skip_txfm = 1 as uint8_t;
            *((*x).plane[0 as core::ffi::c_int as usize].eobs).offset(block as isize) = 0
                as uint16_t;
            *((*x).plane[0 as core::ffi::c_int as usize].txb_entropy_ctx)
                .offset(block as isize) = 0 as uint8_t;
            update_txk_array(
                xd,
                blk_row,
                blk_col,
                tx_size,
                DCT_DCT as core::ffi::c_int as TX_TYPE,
            );
        } else {
            (*rd_stats).skip_txfm = 0 as uint8_t;
        }
        if tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int
            && depth < MAX_VARTX_DEPTH
        {
            (*rd_stats).rate
                += (*x)
                    .mode_costs
                    .txfm_partition_cost[ctx as usize][0 as core::ffi::c_int as usize];
        }
        av1_set_txb_context(x, 0 as core::ffi::c_int, block, tx_size, ta, tl);
        txfm_partition_update(
            tx_above.offset(blk_col as isize),
            tx_left.offset(blk_row as isize),
            tx_size,
            tx_size,
        );
    } else {
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        let txb_width: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let txb_height: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        let step: core::ffi::c_int = txb_height * txb_width;
        let row_end: core::ffi::c_int = if tx_size_high_unit[tx_size as usize]
            < max_blocks_high - blk_row
        {
            tx_size_high_unit[tx_size as usize]
        } else {
            max_blocks_high - blk_row
        };
        let col_end: core::ffi::c_int = if tx_size_wide_unit[tx_size as usize]
            < max_blocks_wide - blk_col
        {
            tx_size_wide_unit[tx_size as usize]
        } else {
            max_blocks_wide - blk_col
        };
        let mut pn_rd_stats: RD_STATS = RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        };
        let mut this_rd: int64_t = 0 as int64_t;
        if !(txb_width > 0 as core::ffi::c_int && txb_height > 0 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"tx_block_yrd\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                3335 as core::ffi::c_int,
                b"txb_width > 0 && txb_height > 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < row_end {
            let offsetr: core::ffi::c_int = blk_row + row;
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < col_end {
                let offsetc: core::ffi::c_int = blk_col + col;
                av1_init_rd_stats(&raw mut pn_rd_stats);
                tx_block_yrd(
                    cpi,
                    x,
                    offsetr,
                    offsetc,
                    block,
                    sub_txs,
                    plane_bsize,
                    depth + 1 as core::ffi::c_int,
                    above_ctx,
                    left_ctx,
                    tx_above,
                    tx_left,
                    ref_best_rd - this_rd,
                    &raw mut pn_rd_stats,
                    ftxs_mode,
                );
                if pn_rd_stats.rate == INT_MAX {
                    av1_invalid_rd_stats(rd_stats);
                    return;
                }
                av1_merge_rd_stats(rd_stats, &raw mut pn_rd_stats);
                this_rd
                    += (pn_rd_stats.rate as int64_t * (*x).rdmult as int64_t
                        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) as int64_t
                        >> 9 as core::ffi::c_int)
                        + pn_rd_stats.dist
                            * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                block += step;
                col += txb_width;
            }
            row += txb_height;
        }
        if tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int
            && depth < MAX_VARTX_DEPTH
        {
            (*rd_stats).rate
                += (*x)
                    .mode_costs
                    .txfm_partition_cost[ctx as usize][1 as core::ffi::c_int as usize];
        }
    };
}
unsafe extern "C-unwind" fn inter_block_yrd(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
) -> core::ffi::c_int {
    if ref_best_rd < 0 as int64_t {
        av1_invalid_rd_stats(rd_stats);
        return 0 as core::ffi::c_int;
    }
    av1_init_rd_stats(rd_stats);
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    let mi_width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let max_tx_size: TX_SIZE = get_vartx_max_txsize(xd, bsize, 0 as core::ffi::c_int)
        as TX_SIZE;
    let bh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
    let bw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
    let step: core::ffi::c_int = bw * bh;
    let init_depth: core::ffi::c_int = get_search_init_depth(
        mi_width,
        mi_height,
        1 as core::ffi::c_int,
        &(*cpi).sf,
        (*txfm_params).tx_size_search_method as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut ctxa: [ENTROPY_CONTEXT; 32] = [0; 32];
    let mut ctxl: [ENTROPY_CONTEXT; 32] = [0; 32];
    let mut tx_above: [TXFM_CONTEXT; 32] = [0; 32];
    let mut tx_left: [TXFM_CONTEXT; 32] = [0; 32];
    av1_get_entropy_contexts(bsize, pd, ctxa.as_mut_ptr(), ctxl.as_mut_ptr());
    memcpy(
        tx_above.as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).above_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        tx_left.as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).left_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
    let mut this_rd: int64_t = 0 as int64_t;
    let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut block: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy < mi_height {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < mi_width {
            let mut pn_rd_stats: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            av1_init_rd_stats(&raw mut pn_rd_stats);
            tx_block_yrd(
                cpi,
                x,
                idy,
                idx,
                block,
                max_tx_size,
                bsize,
                init_depth,
                ctxa.as_mut_ptr(),
                ctxl.as_mut_ptr(),
                tx_above.as_mut_ptr(),
                tx_left.as_mut_ptr(),
                ref_best_rd - this_rd,
                &raw mut pn_rd_stats,
                ftxs_mode,
            );
            if pn_rd_stats.rate == INT_MAX {
                av1_invalid_rd_stats(rd_stats);
                return 0 as core::ffi::c_int;
            }
            av1_merge_rd_stats(rd_stats, &raw mut pn_rd_stats);
            this_rd
                += if ((pn_rd_stats.rate as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + pn_rd_stats.dist
                        * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as int64_t)
                    < (pn_rd_stats.zero_rate as int64_t * (*x).rdmult as int64_t
                        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) as int64_t
                        >> 9 as core::ffi::c_int)
                        + pn_rd_stats.sse
                            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                                as int64_t
                {
                    (pn_rd_stats.rate as int64_t * (*x).rdmult as int64_t
                        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) as int64_t
                        >> 9 as core::ffi::c_int)
                        + pn_rd_stats.dist
                            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                                as int64_t
                } else {
                    (pn_rd_stats.zero_rate as int64_t * (*x).rdmult as int64_t
                        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) as int64_t
                        >> 9 as core::ffi::c_int)
                        + pn_rd_stats.sse
                            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                                as int64_t
                };
            block += step;
            idx += bw;
        }
        idy += bh;
    }
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let no_skip_txfm_rate: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let skip_txfm_rate: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let skip_txfm_rd: int64_t = (skip_txfm_rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    this_rd = (((*rd_stats).rate + no_skip_txfm_rate) as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    if skip_txfm_rd < this_rd {
        this_rd = skip_txfm_rd;
        (*rd_stats).rate = 0 as core::ffi::c_int;
        (*rd_stats).dist = (*rd_stats).sse;
        (*rd_stats).skip_txfm = 1 as uint8_t;
    }
    let is_cost_valid: core::ffi::c_int = (this_rd > ref_best_rd) as core::ffi::c_int;
    if is_cost_valid == 0 {
        av1_invalid_rd_stats(rd_stats);
    }
    return is_cost_valid;
}
unsafe extern "C-unwind" fn select_tx_size_and_type(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
) -> int64_t {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if (is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"select_tx_size_and_type\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3444 as core::ffi::c_int,
            b"is_inter_block(xd->mi[0])\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"select_tx_size_and_type\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3445 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let fast_tx_search: core::ffi::c_int = ((*txfm_params).tx_size_search_method
        as core::ffi::c_int > USE_FULL_RD as core::ffi::c_int) as core::ffi::c_int;
    let mut rd_thresh: int64_t = ref_best_rd;
    if rd_thresh == 0 as int64_t {
        av1_invalid_rd_stats(rd_stats);
        return INT64_MAX as int64_t;
    }
    if fast_tx_search != 0 && rd_thresh < INT64_MAX as int64_t {
        if INT64_MAX as int64_t - rd_thresh > rd_thresh >> 3 as core::ffi::c_int {
            rd_thresh += rd_thresh >> 3 as core::ffi::c_int;
        }
    }
    if !(rd_thresh > 0 as int64_t) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"select_tx_size_and_type\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3455 as core::ffi::c_int,
            b"rd_thresh > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ftxs_mode: FAST_TX_SEARCH_MODE = (if fast_tx_search != 0 {
        FTXS_DCT_AND_1D_DCT_ONLY as core::ffi::c_int
    } else {
        FTXS_NONE as core::ffi::c_int
    }) as FAST_TX_SEARCH_MODE;
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"select_tx_size_and_type\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3459 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mi_width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut ctxa: [ENTROPY_CONTEXT; 32] = [0; 32];
    let mut ctxl: [ENTROPY_CONTEXT; 32] = [0; 32];
    let mut tx_above: [TXFM_CONTEXT; 32] = [0; 32];
    let mut tx_left: [TXFM_CONTEXT; 32] = [0; 32];
    av1_get_entropy_contexts(bsize, pd, ctxa.as_mut_ptr(), ctxl.as_mut_ptr());
    memcpy(
        tx_above.as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).above_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        tx_left.as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).left_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
    let init_depth: core::ffi::c_int = get_search_init_depth(
        mi_width,
        mi_height,
        1 as core::ffi::c_int,
        &(*cpi).sf,
        (*txfm_params).tx_size_search_method as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    let bh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
    let bw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
    let step: core::ffi::c_int = bw * bh;
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let no_skip_txfm_cost: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let skip_txfm_cost: core::ffi::c_int = (*x)
        .mode_costs
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let mut skip_txfm_rd: int64_t = (skip_txfm_cost as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let mut no_skip_txfm_rd: int64_t = (no_skip_txfm_cost as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let mut block: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut blk_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    av1_init_rd_stats(rd_stats);
    let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy < max_block_high(xd, bsize, 0 as core::ffi::c_int) {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < max_block_wide(xd, bsize, 0 as core::ffi::c_int) {
            let best_rd_sofar: int64_t = if rd_thresh == INT64_MAX as int64_t {
                INT64_MAX as int64_t
            } else {
                rd_thresh
                    - (if skip_txfm_rd < no_skip_txfm_rd {
                        skip_txfm_rd
                    } else {
                        no_skip_txfm_rd
                    })
            };
            let mut is_cost_valid: core::ffi::c_int = 1 as core::ffi::c_int;
            let mut pn_rd_stats: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            select_tx_block(
                cpi,
                x,
                idy,
                idx,
                block,
                max_tx_size,
                init_depth,
                bsize,
                ctxa.as_mut_ptr(),
                ctxl.as_mut_ptr(),
                tx_above.as_mut_ptr(),
                tx_left.as_mut_ptr(),
                &raw mut pn_rd_stats,
                INT64_MAX as int64_t,
                best_rd_sofar,
                &raw mut is_cost_valid,
                ftxs_mode,
                blk_idx,
            );
            blk_idx += 1;
            if is_cost_valid == 0 || pn_rd_stats.rate == INT_MAX {
                av1_invalid_rd_stats(rd_stats);
                return INT64_MAX as int64_t;
            }
            av1_merge_rd_stats(rd_stats, &raw mut pn_rd_stats);
            skip_txfm_rd = (skip_txfm_cost as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            no_skip_txfm_rd = (((*rd_stats).rate + no_skip_txfm_cost) as int64_t
                * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            block += step;
            idx += bw;
        }
        idy += bh;
    }
    if (*rd_stats).rate == INT_MAX {
        return INT64_MAX as int64_t;
    }
    (*rd_stats).skip_txfm = (skip_txfm_rd <= no_skip_txfm_rd) as core::ffi::c_int
        as uint8_t;
    if fast_tx_search != 0 && (*cpi).sf.tx_sf.refine_fast_tx_search_results != 0 {
        if inter_block_yrd(
            cpi,
            x,
            rd_stats,
            bsize,
            ref_best_rd,
            FTXS_NONE as core::ffi::c_int as FAST_TX_SEARCH_MODE,
        ) == 0
        {
            return INT64_MAX as int64_t;
        }
    }
    let mut final_rd: int64_t = 0;
    if (*rd_stats).skip_txfm != 0 {
        final_rd = (skip_txfm_cost as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    } else {
        final_rd = (((*rd_stats).rate + no_skip_txfm_cost) as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if (*xd)
            .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
            as usize] == 0
        {
            final_rd = if final_rd
                < (skip_txfm_cost as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*rd_stats).sse
                        * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as int64_t
            {
                final_rd
            } else {
                (skip_txfm_cost as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*rd_stats).sse
                        * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as int64_t
            };
        }
    }
    return final_rd;
}
#[inline]
unsafe extern "C-unwind" fn model_based_tx_search_prune(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
) -> core::ffi::c_int {
    let level: core::ffi::c_int = (*cpi).sf.tx_sf.model_based_prune_tx_search_level;
    if !(level >= 0 as core::ffi::c_int && level <= 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"model_based_tx_search_prune\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3542 as core::ffi::c_int,
            b"level >= 0 && level <= 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut model_rate: core::ffi::c_int = 0;
    let mut model_dist: int64_t = 0;
    let mut model_skip: uint8_t = 0;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    (model_rd_sb_fn[MODELRD_TYPE_TX_SEARCH_PRUNE as usize])
        .expect(
            "non-null function pointer",
        )(
        cpi,
        bsize,
        x,
        xd,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        &raw mut model_rate,
        &raw mut model_dist,
        &raw mut model_skip,
        0 as *mut int64_t,
        0 as *mut core::ffi::c_int,
        0 as *mut int64_t,
        0 as *mut int64_t,
    );
    if model_skip != 0 {
        return 0 as core::ffi::c_int;
    }
    let model_rd: int64_t = (model_rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + model_dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    static mut prune_factor_by8: [core::ffi::c_int; 2] = [
        3 as core::ffi::c_int,
        5 as core::ffi::c_int,
    ];
    let factor: core::ffi::c_int = prune_factor_by8[(level - 1 as core::ffi::c_int)
        as usize];
    return (model_rd * factor as int64_t >> 3 as core::ffi::c_int > ref_best_rd)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_pick_recursive_tx_size_type_yrd(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if (is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_pick_recursive_tx_size_type_yrd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3564 as core::ffi::c_int,
            b"is_inter_block(xd->mi[0])\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_invalid_rd_stats(rd_stats);
    if (*cpi).sf.tx_sf.model_based_prune_tx_search_level != 0
        && ref_best_rd != INT64_MAX as int64_t
    {
        if model_based_tx_search_prune(cpi, x, bsize, ref_best_rd) != 0 {
            return;
        }
    }
    let mut hash: uint32_t = 0 as uint32_t;
    let mut mb_rd_record: *mut MB_RD_RECORD = 0 as *mut MB_RD_RECORD;
    let mi_row: core::ffi::c_int = (*x).e_mbd.mi_row;
    let mi_col: core::ffi::c_int = (*x).e_mbd.mi_col;
    let within_border: core::ffi::c_int = (mi_row >= (*xd).tile.mi_row_start
        && (mi_row + mi_size_high[bsize as usize] as core::ffi::c_int)
            < (*xd).tile.mi_row_end && mi_col >= (*xd).tile.mi_col_start
        && (mi_col + mi_size_wide[bsize as usize] as core::ffi::c_int)
            < (*xd).tile.mi_col_end) as core::ffi::c_int;
    let is_mb_rd_hash_enabled: core::ffi::c_int = (within_border != 0
        && (*cpi).sf.rd_sf.use_mb_rd_hash != 0) as core::ffi::c_int;
    let n4: core::ffi::c_int = bsize_to_num_blk(bsize) as core::ffi::c_int;
    if is_mb_rd_hash_enabled != 0 {
        hash = get_block_residue_hash(x, bsize);
        mb_rd_record = (*x).txfm_search_info.mb_rd_record;
        let match_index: core::ffi::c_int = find_mb_rd_info(
            mb_rd_record,
            ref_best_rd,
            hash,
        ) as core::ffi::c_int;
        if match_index != -(1 as core::ffi::c_int) {
            let mut mb_rd_info: *mut MB_RD_INFO = &raw mut *((*mb_rd_record).mb_rd_info)
                .as_mut_ptr()
                .offset(match_index as isize) as *mut MB_RD_INFO;
            fetch_mb_rd_info(n4, mb_rd_info, rd_stats, x);
            return;
        }
    }
    let mut dist: int64_t = 0;
    if (*txfm_params).skip_txfm_level != 0
        && predict_skip_txfm(
            x,
            bsize,
            &raw mut dist,
            (*cpi).common.features.reduced_tx_set_used as core::ffi::c_int,
        ) != 0
    {
        set_skip_txfm(x, rd_stats, bsize, dist);
        if is_mb_rd_hash_enabled != 0 {
            save_mb_rd_info(n4, hash, x, rd_stats, mb_rd_record);
        }
        return;
    }
    let rd: int64_t = select_tx_size_and_type(cpi, x, rd_stats, bsize, ref_best_rd)
        as int64_t;
    if rd == INT64_MAX as int64_t {
        if !(ref_best_rd != 9223372036854775807 as int64_t) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 36],
                    [core::ffi::c_char; 36],
                >(*b"av1_pick_recursive_tx_size_type_yrd\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                3622 as core::ffi::c_int,
                b"ref_best_rd != INT64_MAX\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_invalid_rd_stats(rd_stats);
        return;
    }
    if is_mb_rd_hash_enabled != 0 {
        if mb_rd_record.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 36],
                    [core::ffi::c_char; 36],
                >(*b"av1_pick_recursive_tx_size_type_yrd\0"))
                    .as_ptr(),
                b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
                3629 as core::ffi::c_int,
                b"mb_rd_record != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        save_mb_rd_info(n4, hash, x, rd_stats, mb_rd_record);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_pick_uniform_tx_size_type_yrd(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bs: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut tx_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if !(bs as core::ffi::c_int == (*mbmi).bsize as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_pick_uniform_tx_size_type_yrd\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3640 as core::ffi::c_int,
            b"bs == mbmi->bsize\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    av1_init_rd_stats(rd_stats);
    let mut hash: uint32_t = 0 as uint32_t;
    let mut mb_rd_record: *mut MB_RD_RECORD = 0 as *mut MB_RD_RECORD;
    let num_blks: core::ffi::c_int = bsize_to_num_blk(bs) as core::ffi::c_int;
    if is_inter != 0 && (*cpi).sf.rd_sf.use_mb_rd_hash != 0 {
        let within_border: core::ffi::c_int = (mi_row >= (*xd).tile.mi_row_start
            && (mi_row + mi_size_high[bs as usize] as core::ffi::c_int)
                < (*xd).tile.mi_row_end && mi_col >= (*xd).tile.mi_col_start
            && (mi_col + mi_size_wide[bs as usize] as core::ffi::c_int)
                < (*xd).tile.mi_col_end) as core::ffi::c_int;
        if within_border != 0 {
            hash = get_block_residue_hash(x, bs);
            mb_rd_record = (*x).txfm_search_info.mb_rd_record;
            let match_index: core::ffi::c_int = find_mb_rd_info(
                mb_rd_record,
                ref_best_rd,
                hash,
            ) as core::ffi::c_int;
            if match_index != -(1 as core::ffi::c_int) {
                let mut mb_rd_info: *mut MB_RD_INFO = &raw mut *((*mb_rd_record).mb_rd_info)
                    .as_mut_ptr()
                    .offset(match_index as isize) as *mut MB_RD_INFO;
                fetch_mb_rd_info(num_blks, mb_rd_info, rd_stats, x);
                return;
            }
        }
    }
    let mut dist: int64_t = 0;
    if (*tx_params).skip_txfm_level != 0 && is_inter != 0
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
        && predict_skip_txfm(
            x,
            bs,
            &raw mut dist,
            (*cpi).common.features.reduced_tx_set_used as core::ffi::c_int,
        ) != 0
    {
        set_skip_txfm(x, rd_stats, bs, dist);
        if !mb_rd_record.is_null() {
            save_mb_rd_info(num_blks, hash, x, rd_stats, mb_rd_record);
        }
        return;
    }
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        choose_smallest_tx_size(cpi, x, rd_stats, ref_best_rd, bs);
    } else if (*tx_params).tx_size_search_method as core::ffi::c_int
        == USE_LARGESTALL as core::ffi::c_int
    {
        choose_largest_tx_size(cpi, x, rd_stats, ref_best_rd, bs);
    } else {
        choose_tx_size_type_from_rd(cpi, x, rd_stats, ref_best_rd, bs);
    }
    if !mb_rd_record.is_null() {
        save_mb_rd_info(num_blks, hash, x, rd_stats, mb_rd_record);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_txfm_uvrd(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ref_best_rd: int64_t,
) -> core::ffi::c_int {
    av1_init_rd_stats(rd_stats);
    if ref_best_rd < 0 as int64_t {
        return 0 as core::ffi::c_int;
    }
    if !(*x).e_mbd.is_chroma_ref {
        return 1 as core::ffi::c_int;
    }
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_U as isize) as *mut macroblockd_plane;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let mut this_rd: int64_t = 0 as int64_t;
    let mut skip_txfm_rd: int64_t = 0 as int64_t;
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
    ) as BLOCK_SIZE;
    if is_inter != 0 {
        let mut plane: core::ffi::c_int = 1 as core::ffi::c_int;
        while plane < MAX_MB_PLANE {
            av1_subtract_plane(x, plane_bsize, plane, (*cpi).do_border_pad);
            plane += 1;
        }
    }
    let uv_tx_size: TX_SIZE = av1_get_tx_size(AOM_PLANE_U, xd) as TX_SIZE;
    let mut is_cost_valid: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut plane_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while plane_0 < MAX_MB_PLANE {
        let mut this_rd_stats: RD_STATS = RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        };
        let mut chroma_ref_best_rd: int64_t = ref_best_rd;
        if (*cpi).sf.inter_sf.perform_best_rd_based_gating_for_chroma != 0
            && is_inter != 0 && chroma_ref_best_rd != INT64_MAX as int64_t
        {
            chroma_ref_best_rd = ref_best_rd
                - (if this_rd < skip_txfm_rd { this_rd } else { skip_txfm_rd });
        }
        av1_txfm_rd_in_plane(
            x,
            cpi,
            &raw mut this_rd_stats,
            chroma_ref_best_rd,
            0 as int64_t,
            plane_0,
            plane_bsize,
            uv_tx_size,
            FTXS_NONE as core::ffi::c_int as FAST_TX_SEARCH_MODE,
        );
        if this_rd_stats.rate == INT_MAX {
            is_cost_valid = 0 as core::ffi::c_int;
            break;
        } else {
            av1_merge_rd_stats(rd_stats, &raw mut this_rd_stats);
            this_rd = ((*rd_stats).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            skip_txfm_rd = (0 as core::ffi::c_int as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            if (if this_rd < skip_txfm_rd { this_rd } else { skip_txfm_rd })
                > ref_best_rd
            {
                is_cost_valid = 0 as core::ffi::c_int;
                break;
            } else {
                plane_0 += 1;
            }
        }
    }
    if is_cost_valid == 0 {
        av1_invalid_rd_stats(rd_stats);
    }
    return is_cost_valid;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_txfm_rd_in_plane(
    mut x: *mut MACROBLOCK,
    mut cpi: *const AV1_COMP,
    mut rd_stats: *mut RD_STATS,
    mut ref_best_rd: int64_t,
    mut current_rd: int64_t,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut ftxs_mode: FAST_TX_SEARCH_MODE,
) {
    if !(!(plane == 0 as core::ffi::c_int)
        || (**((*x).e_mbd.mi).offset(0 as core::ffi::c_int as isize)).tx_size
            as core::ffi::c_int == tx_size as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_txfm_rd_in_plane\0"))
                .as_ptr(),
            b"tx_search.c\0" as *const u8 as *const core::ffi::c_char,
            3761 as core::ffi::c_int,
            b"IMPLIES(plane == 0, x->e_mbd.mi[0]->tx_size == tx_size)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(*cpi).oxcf.txfm_cfg.enable_tx64
        && txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            == TX_64X64 as core::ffi::c_int
    {
        av1_invalid_rd_stats(rd_stats);
        return;
    }
    if current_rd > ref_best_rd {
        av1_invalid_rd_stats(rd_stats);
        return;
    }
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let mut args: rdcost_block_args = rdcost_block_args {
        cpi: 0 as *const AV1_COMP,
        x: 0 as *mut MACROBLOCK,
        t_above: [0; 32],
        t_left: [0; 32],
        rd_stats: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        current_rd: 0,
        best_rd: 0,
        exit_early: 0,
        incomplete_exit: 0,
        ftxs_mode: 0,
        skip_trellis: 0,
    };
    memset(
        &raw mut args as *mut rdcost_block_args as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<rdcost_block_args>() as size_t,
    );
    args.x = x;
    args.cpi = cpi;
    args.best_rd = ref_best_rd;
    args.current_rd = current_rd;
    args.ftxs_mode = ftxs_mode;
    args.skip_trellis = 0 as core::ffi::c_int;
    av1_init_rd_stats(&raw mut args.rd_stats);
    av1_get_entropy_contexts(
        plane_bsize,
        pd,
        (args.t_above).as_mut_ptr(),
        (args.t_left).as_mut_ptr(),
    );
    av1_foreach_transformed_block_in_plane(
        xd,
        plane_bsize,
        plane,
        Some(
            block_rd_txfm
                as unsafe extern "C-unwind" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    BLOCK_SIZE,
                    TX_SIZE,
                    *mut core::ffi::c_void,
                ) -> (),
        ),
        &raw mut args as *mut rdcost_block_args as *mut core::ffi::c_void,
    );
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let invalid_rd: core::ffi::c_int = if is_inter != 0 {
        args.incomplete_exit
    } else {
        args.exit_early
    };
    if invalid_rd != 0 {
        av1_invalid_rd_stats(rd_stats);
    } else {
        *rd_stats = args.rd_stats;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_txfm_search(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut rd_stats: *mut RD_STATS,
    mut rd_stats_y: *mut RD_STATS,
    mut rd_stats_uv: *mut RD_STATS,
    mut mode_rate: core::ffi::c_int,
    mut ref_best_rd: int64_t,
) -> core::ffi::c_int {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_params: *mut TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let skip_txfm_cost: [core::ffi::c_int; 2] = [
        (*x)
            .mode_costs
            .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize],
        (*x).mode_costs.skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize],
    ];
    let min_header_rate: int64_t = (mode_rate
        + (if skip_txfm_cost[0 as core::ffi::c_int as usize]
            < skip_txfm_cost[1 as core::ffi::c_int as usize]
        {
            skip_txfm_cost[0 as core::ffi::c_int as usize]
        } else {
            skip_txfm_cost[1 as core::ffi::c_int as usize]
        })) as int64_t;
    let min_header_rd_possible: int64_t = (min_header_rate * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    if min_header_rd_possible > ref_best_rd {
        av1_invalid_rd_stats(rd_stats_y);
        return 0 as core::ffi::c_int;
    }
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mode_rd: int64_t = (mode_rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let rd_thresh: int64_t = if ref_best_rd == INT64_MAX as int64_t {
        INT64_MAX as int64_t
    } else {
        ref_best_rd - mode_rd
    };
    av1_init_rd_stats(rd_stats);
    av1_init_rd_stats(rd_stats_y);
    (*rd_stats).rate = mode_rate;
    av1_subtract_plane(x, bsize, PLANE_TYPE_Y as core::ffi::c_int, (*cpi).do_border_pad);
    if (*txfm_params).tx_mode_search_type as core::ffi::c_int
        == TX_MODE_SELECT as core::ffi::c_int
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
    {
        av1_pick_recursive_tx_size_type_yrd(cpi, x, rd_stats_y, bsize, rd_thresh);
    } else {
        av1_pick_uniform_tx_size_type_yrd(cpi, x, rd_stats_y, bsize, rd_thresh);
        memset(
            ((*mbmi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
            (*mbmi).tx_size as core::ffi::c_int,
            ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
        );
    }
    if (*rd_stats_y).rate == INT_MAX {
        return 0 as core::ffi::c_int;
    }
    av1_merge_rd_stats(rd_stats, rd_stats_y);
    let non_skip_txfm_rdcosty: int64_t = (((*rd_stats).rate
        + skip_txfm_cost[0 as core::ffi::c_int as usize]) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    let skip_txfm_rdcosty: int64_t = ((mode_rate
        + skip_txfm_cost[1 as core::ffi::c_int as usize]) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    let min_rdcosty: int64_t = if non_skip_txfm_rdcosty < skip_txfm_rdcosty {
        non_skip_txfm_rdcosty
    } else {
        skip_txfm_rdcosty
    };
    if min_rdcosty > ref_best_rd {
        return 0 as core::ffi::c_int;
    }
    av1_init_rd_stats(rd_stats_uv);
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if num_planes > 1 as core::ffi::c_int {
        let mut ref_best_chroma_rd: int64_t = ref_best_rd;
        if (*cpi).sf.inter_sf.perform_best_rd_based_gating_for_chroma != 0
            && ref_best_chroma_rd != INT64_MAX as int64_t
        {
            ref_best_chroma_rd = ref_best_chroma_rd
                - (if non_skip_txfm_rdcosty < skip_txfm_rdcosty {
                    non_skip_txfm_rdcosty
                } else {
                    skip_txfm_rdcosty
                });
        }
        let is_cost_valid_uv: core::ffi::c_int = av1_txfm_uvrd(
            cpi,
            x,
            rd_stats_uv,
            bsize,
            ref_best_chroma_rd,
        ) as core::ffi::c_int;
        if is_cost_valid_uv == 0 {
            return 0 as core::ffi::c_int;
        }
        if (*cpi).sf.hl_sf.weighted_chroma_distortion != 0 {
            av1_merge_rd_stats_weighted(rd_stats, rd_stats_uv);
        } else {
            av1_merge_rd_stats(rd_stats, rd_stats_uv);
        }
    }
    let mut choose_skip_txfm: core::ffi::c_int = (*rd_stats).skip_txfm
        as core::ffi::c_int;
    if choose_skip_txfm == 0 && (*xd).lossless[(*mbmi).segment_id() as usize] == 0 {
        let rdcost_no_skip_txfm: int64_t = (((*rd_stats_y).rate + (*rd_stats_uv).rate
            + skip_txfm_cost[0 as core::ffi::c_int as usize]) as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        let rdcost_skip_txfm: int64_t = (skip_txfm_cost[1 as core::ffi::c_int as usize]
            as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (*rd_stats).sse * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if rdcost_no_skip_txfm >= rdcost_skip_txfm {
            choose_skip_txfm = 1 as core::ffi::c_int;
        }
    }
    if choose_skip_txfm != 0 {
        (*rd_stats_y).rate = 0 as core::ffi::c_int;
        (*rd_stats_uv).rate = 0 as core::ffi::c_int;
        (*rd_stats).rate = mode_rate + skip_txfm_cost[1 as core::ffi::c_int as usize];
        (*rd_stats).dist = (*rd_stats).sse;
        (*rd_stats_y).dist = (*rd_stats_y).sse;
        (*rd_stats_uv).dist = (*rd_stats_uv).sse;
        (*mbmi).skip_txfm = 1 as uint8_t;
        if (*rd_stats).skip_txfm != 0 {
            let tmprd: int64_t = ((*rd_stats).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*rd_stats).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            if tmprd > ref_best_rd {
                return 0 as core::ffi::c_int;
            }
        }
    } else {
        (*rd_stats).rate += skip_txfm_cost[0 as core::ffi::c_int as usize];
        (*mbmi).skip_txfm = 0 as uint8_t;
    }
    return 1 as core::ffi::c_int;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn llabs(_: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn log2(_: core::ffi::c_double) -> core::ffi::c_double;
    fn log1pf(_: core::ffi::c_float) -> core::ffi::c_float;
    fn sqrtf(_: core::ffi::c_float) -> core::ffi::c_float;
}
