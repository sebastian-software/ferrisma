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

// Mechanically translated from upstream/aom/av1/decoder/decodemv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_left_block_mode(left_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_above_block_mode(above_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_merge_corrupted_flag(
        corrupted: *mut core::ffi::c_int,
        value: core::ffi::c_int,
    );
    fn av1_copy_frame_mvs(
        cm: *const AV1_COMMON,
        mi: *const MB_MODE_INFO,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        x_mis: core::ffi::c_int,
        y_mis: core::ffi::c_int,
    );
    fn av1_find_mv_refs(
        cm: *const AV1_COMMON,
        xd: *const MACROBLOCKD,
        mi: *mut MB_MODE_INFO,
        ref_frame: MV_REFERENCE_FRAME,
        ref_mv_count: *mut uint8_t,
        ref_mv_stack: *mut [CANDIDATE_MV; 8],
        ref_mv_weight: *mut [uint16_t; 8],
        mv_ref_list: *mut [int_mv; 2],
        global_mvs: *mut int_mv,
        mode_context: *mut int16_t,
    );
    fn av1_find_best_ref_mvs(
        allow_hp: core::ffi::c_int,
        mvlist: *mut int_mv,
        nearest_mv: *mut int_mv,
        near_mv: *mut int_mv,
        is_integer: core::ffi::c_int,
    );
    fn av1_selectSamples(
        mv: *mut MV,
        pts: *mut core::ffi::c_int,
        pts_inref: *mut core::ffi::c_int,
        len: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    ) -> uint8_t;
    fn av1_findSamples(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        pts: *mut core::ffi::c_int,
        pts_inref: *mut core::ffi::c_int,
    ) -> uint8_t;
    fn av1_get_pred_context_switchable_interp(
        xd: *const MACROBLOCKD,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_palette_cache(
        xd: *const MACROBLOCKD,
        plane: core::ffi::c_int,
        cache: *mut uint16_t,
    ) -> core::ffi::c_int;
    fn av1_get_intra_inter_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_reference_mode_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_comp_reference_type_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_bwdref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_bwdref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p3(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p4(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p5(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p6(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_find_projection(
        np: core::ffi::c_int,
        pts1: *const core::ffi::c_int,
        pts2: *const core::ffi::c_int,
        bsize: BLOCK_SIZE,
        mvy: core::ffi::c_int,
        mvx: core::ffi::c_int,
        wm_params: *mut WarpedMotionParams,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    static av1_wedge_params_lookup: [wedge_params_type; 22];
    fn av1_count_overlappable_neighbors(cm: *const AV1_COMMON, xd: *mut MACROBLOCKD);
    fn od_ec_decode_bool_q15(
        dec: *mut od_ec_dec,
        f: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn od_ec_decode_cdf_q15(
        dec: *mut od_ec_dec,
        cdf: *const uint16_t,
        nsyms: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
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
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
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
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const COMP_REFERENCE_TYPES: C2RustUnnamed_2 = 2;
pub const BIDIR_COMP_REFERENCE: C2RustUnnamed_2 = 1;
pub const UNIDIR_COMP_REFERENCE: C2RustUnnamed_2 = 0;
pub type COMP_REFERENCE_TYPE = uint8_t;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_3 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_3 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_3 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const CFL_SIGNS: C2RustUnnamed_4 = 3;
pub const CFL_SIGN_POS: C2RustUnnamed_4 = 2;
pub const CFL_SIGN_NEG: C2RustUnnamed_4 = 1;
pub const CFL_SIGN_ZERO: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const CFL_ALLOWED_TYPES: C2RustUnnamed_5 = 2;
pub const CFL_ALLOWED: C2RustUnnamed_5 = 1;
pub const CFL_DISALLOWED: C2RustUnnamed_5 = 0;
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const PALETTE_SIZES: C2RustUnnamed_6 = 7;
pub const EIGHT_COLORS: C2RustUnnamed_6 = 6;
pub const SEVEN_COLORS: C2RustUnnamed_6 = 5;
pub const SIX_COLORS: C2RustUnnamed_6 = 4;
pub const FIVE_COLORS: C2RustUnnamed_6 = 3;
pub const FOUR_COLORS: C2RustUnnamed_6 = 2;
pub const THREE_COLORS: C2RustUnnamed_6 = 1;
pub const TWO_COLORS: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_7 = 25;
pub const INTRA_MODES: C2RustUnnamed_7 = 13;
pub const INTER_MODE_END: C2RustUnnamed_7 = 25;
pub const INTER_MODE_START: C2RustUnnamed_7 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_7 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_7 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_7 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_7 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_7 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_7 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_7 = 13;
pub const DIR_MODE_END: C2RustUnnamed_7 = 9;
pub const DIR_MODE_START: C2RustUnnamed_7 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_7 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_7 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_7 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_7 = 25;
pub const NEW_NEWMV: C2RustUnnamed_7 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_7 = 23;
pub const NEW_NEARMV: C2RustUnnamed_7 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_7 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_7 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_7 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_7 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_7 = 17;
pub const NEWMV: C2RustUnnamed_7 = 16;
pub const GLOBALMV: C2RustUnnamed_7 = 15;
pub const NEARMV: C2RustUnnamed_7 = 14;
pub const NEARESTMV: C2RustUnnamed_7 = 13;
pub const PAETH_PRED: C2RustUnnamed_7 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_7 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_7 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_7 = 9;
pub const D67_PRED: C2RustUnnamed_7 = 8;
pub const D203_PRED: C2RustUnnamed_7 = 7;
pub const D157_PRED: C2RustUnnamed_7 = 6;
pub const D113_PRED: C2RustUnnamed_7 = 5;
pub const D135_PRED: C2RustUnnamed_7 = 4;
pub const D45_PRED: C2RustUnnamed_7 = 3;
pub const H_PRED: C2RustUnnamed_7 = 2;
pub const V_PRED: C2RustUnnamed_7 = 1;
pub const DC_PRED: C2RustUnnamed_7 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_8 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_8 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_8 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_8 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_8 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_8 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_8 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_8 = 9;
pub const UV_D67_PRED: C2RustUnnamed_8 = 8;
pub const UV_D203_PRED: C2RustUnnamed_8 = 7;
pub const UV_D157_PRED: C2RustUnnamed_8 = 6;
pub const UV_D113_PRED: C2RustUnnamed_8 = 5;
pub const UV_D135_PRED: C2RustUnnamed_8 = 4;
pub const UV_D45_PRED: C2RustUnnamed_8 = 3;
pub const UV_H_PRED: C2RustUnnamed_8 = 2;
pub const UV_V_PRED: C2RustUnnamed_8 = 1;
pub const UV_DC_PRED: C2RustUnnamed_8 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_9 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_9 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_9 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_9 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_9 = 0;
pub type MOTION_MODE = uint8_t;
pub type C2RustUnnamed_10 = core::ffi::c_uint;
pub const INTERINTRA_MODES: C2RustUnnamed_10 = 4;
pub const II_SMOOTH_PRED: C2RustUnnamed_10 = 3;
pub const II_H_PRED: C2RustUnnamed_10 = 2;
pub const II_V_PRED: C2RustUnnamed_10 = 1;
pub const II_DC_PRED: C2RustUnnamed_10 = 0;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_11 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_11 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_11 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_11 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_11 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_11 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const FILTER_INTRA_MODES: C2RustUnnamed_12 = 5;
pub const FILTER_PAETH_PRED: C2RustUnnamed_12 = 4;
pub const FILTER_D157_PRED: C2RustUnnamed_12 = 3;
pub const FILTER_H_PRED: C2RustUnnamed_12 = 2;
pub const FILTER_V_PRED: C2RustUnnamed_12 = 1;
pub const FILTER_DC_PRED: C2RustUnnamed_12 = 0;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_13 = 7;
pub const BWD_REFS: C2RustUnnamed_13 = 3;
pub const FWD_REFS: C2RustUnnamed_13 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_13 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_13 = 8;
pub const REF_FRAMES: C2RustUnnamed_13 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_13 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_13 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_13 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_13 = 4;
pub const LAST3_FRAME: C2RustUnnamed_13 = 3;
pub const LAST2_FRAME: C2RustUnnamed_13 = 2;
pub const LAST_FRAME: C2RustUnnamed_13 = 1;
pub const INTRA_FRAME: C2RustUnnamed_13 = 0;
pub const NONE_FRAME: C2RustUnnamed_13 = -1;
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const UNIDIR_COMP_REFS: C2RustUnnamed_14 = 4;
pub const TOTAL_UNIDIR_COMP_REFS: C2RustUnnamed_14 = 9;
pub const ALTREF2_ALTREF_FRAMES: C2RustUnnamed_14 = 8;
pub const BWDREF_ALTREF2_FRAMES: C2RustUnnamed_14 = 7;
pub const LAST3_GOLDEN_FRAMES: C2RustUnnamed_14 = 6;
pub const LAST2_GOLDEN_FRAMES: C2RustUnnamed_14 = 5;
pub const LAST2_LAST3_FRAMES: C2RustUnnamed_14 = 4;
pub const BWDREF_ALTREF_FRAMES: C2RustUnnamed_14 = 3;
pub const LAST_GOLDEN_FRAMES: C2RustUnnamed_14 = 2;
pub const LAST_LAST3_FRAMES: C2RustUnnamed_14 = 1;
pub const LAST_LAST2_FRAMES: C2RustUnnamed_14 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_25,
    pub c2rust_unnamed_0: C2RustUnnamed_23,
    pub c2rust_unnamed_1: C2RustUnnamed_21,
    pub c2rust_unnamed_2: C2RustUnnamed_19,
    pub c2rust_unnamed_3: C2RustUnnamed_17,
    pub c2rust_unnamed_4: C2RustUnnamed_15,
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
pub union C2RustUnnamed_15 {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub c2rust_unnamed: C2RustUnnamed_18,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub c2rust_unnamed: C2RustUnnamed_20,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_22,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_24,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type aom_cdf_prob = uint16_t;
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_27 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_27 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_27 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_27 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_27 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_27 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_27 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_27 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_27 = 0;
pub type SEG_LVL_FEATURES = uint8_t;
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
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_28 = 4;
pub const AFFINE: C2RustUnnamed_28 = 3;
pub const ROTZOOM: C2RustUnnamed_28 = 2;
pub const TRANSLATION: C2RustUnnamed_28 = 1;
pub const IDENTITY: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const MV_JOINT_HNZVNZ: C2RustUnnamed_29 = 3;
pub const MV_JOINT_HZVNZ: C2RustUnnamed_29 = 2;
pub const MV_JOINT_HNZVZ: C2RustUnnamed_29 = 1;
pub const MV_JOINT_ZERO: C2RustUnnamed_29 = 0;
pub type MV_JOINT_TYPE = uint8_t;
pub type C2RustUnnamed_30 = core::ffi::c_uint;
pub const MV_CLASS_10: C2RustUnnamed_30 = 10;
pub const MV_CLASS_9: C2RustUnnamed_30 = 9;
pub const MV_CLASS_8: C2RustUnnamed_30 = 8;
pub const MV_CLASS_7: C2RustUnnamed_30 = 7;
pub const MV_CLASS_6: C2RustUnnamed_30 = 6;
pub const MV_CLASS_5: C2RustUnnamed_30 = 5;
pub const MV_CLASS_4: C2RustUnnamed_30 = 4;
pub const MV_CLASS_3: C2RustUnnamed_30 = 3;
pub const MV_CLASS_2: C2RustUnnamed_30 = 2;
pub const MV_CLASS_1: C2RustUnnamed_30 = 1;
pub const MV_CLASS_0: C2RustUnnamed_30 = 0;
pub type C2RustUnnamed_31 = core::ffi::c_int;
pub const MV_SUBPEL_HIGH_PRECISION: C2RustUnnamed_31 = 1;
pub const MV_SUBPEL_LOW_PRECISION: C2RustUnnamed_31 = 0;
pub const MV_SUBPEL_NONE: C2RustUnnamed_31 = -1;
pub type MvSubpelPrecision = int8_t;
pub type C2RustUnnamed_32 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_32 = 4;
pub const S_FRAME: C2RustUnnamed_32 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_32 = 2;
pub const INTER_FRAME: C2RustUnnamed_32 = 1;
pub const KEY_FRAME: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CB_BUFFER {
    pub __aom_align_dqcoeff: [crate::platform::Align32; 0],

    pub dqcoeff: [[tran_low_t; 16384]; 3],
    pub eob_data: [[eob_info; 1024]; 3],
    pub __aom_align_color_index_map: [crate::platform::Align16; 0],
    pub color_index_map: [[uint8_t; 16384]; 2],
}
pub type MACROBLOCKD = macroblockd;
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
pub type C2RustUnnamed_33 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_33 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_33 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_33 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_33 = 0;
pub type WedgeDirectionType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wedge_code_type {
    pub direction: WedgeDirectionType,
    pub x_offset: core::ffi::c_int,
    pub y_offset: core::ffi::c_int,
}
pub type wedge_masks_type = [uint32_t; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wedge_params_type {
    pub wedge_types: core::ffi::c_int,
    pub codebook: *const wedge_code_type,
    pub signflip: *const uint8_t,
    pub masks: *const wedge_masks_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Decoder {
    pub dcb: DecoderCodingBlock,
    pub __aom_align_common: [crate::platform::Align32; 0],
    pub common: AV1_COMMON,
    pub lf_worker: AVxWorker,
    pub lf_row_sync: AV1LfSync,
    pub lr_row_sync: AV1LrSync,
    pub lr_ctxt: AV1LrStruct,
    pub cdef_sync: AV1CdefSync,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub tile_workers: *mut AVxWorker,
    pub num_workers: core::ffi::c_int,
    pub thread_data: *mut DecWorkerData,
    pub td: ThreadData,
    pub tile_data: *mut TileDataDec,
    pub allocated_tiles: core::ffi::c_int,
    pub tile_buffers: [[TileBufferDec; 64]; 64],
    pub tile_mt_info: AV1DecTileMT,
    pub output_all_layers: core::ffi::c_int,
    pub output_frames: [*mut RefCntBuffer; 4],
    pub num_output_frames: size_t,
    pub decoding_first_frame: core::ffi::c_int,
    pub allow_lowbitdepth: core::ffi::c_int,
    pub max_threads: core::ffi::c_int,
    pub inv_tile_order: core::ffi::c_int,
    pub need_resync: core::ffi::c_int,
    pub reset_decoder_state: core::ffi::c_int,
    pub tile_size_bytes: core::ffi::c_int,
    pub tile_col_size_bytes: core::ffi::c_int,
    pub dec_tile_row: core::ffi::c_int,
    pub dec_tile_col: core::ffi::c_int,
    pub sequence_header_ready: core::ffi::c_int,
    pub sequence_header_changed: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub current_operating_point: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub seen_frame_header: core::ffi::c_int,
    pub next_start_tile: core::ffi::c_int,
    pub camera_frame_header_ready: core::ffi::c_int,
    pub frame_header_size: size_t,
    pub obu_size_hdr: DataBuffer,
    pub output_frame_width_in_tiles_minus_1: core::ffi::c_int,
    pub output_frame_height_in_tiles_minus_1: core::ffi::c_int,
    pub tile_count_minus_1: core::ffi::c_int,
    pub coded_tile_data_size: uint32_t,
    pub ext_tile_debug: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub ext_refs: EXTERNAL_REFERENCES,
    pub tile_list_outbuf: YV12_BUFFER_CONFIG,
    pub cb_buffer_base: *mut CB_BUFFER,
    pub cb_buffer_alloc_size: core::ffi::c_int,
    pub allocated_row_mt_sync_rows: core::ffi::c_int,
    pub row_mt_mutex_: *mut pthread_mutex_t,
    pub row_mt_cond_: *mut pthread_cond_t,
    pub frame_row_mt_info: AV1DecRowMTInfo,
    pub metadata: *mut aom_metadata_array_t,
    pub context_update_tile_id: core::ffi::c_int,
    pub skip_loop_filter: core::ffi::c_int,
    pub skip_film_grain: core::ffi::c_int,
    pub is_annexb: core::ffi::c_int,
    pub valid_for_referencing: [core::ffi::c_int; 8],
    pub is_fwd_kf_present: core::ffi::c_int,
    pub is_arf_frame_present: core::ffi::c_int,
    pub num_tile_groups: core::ffi::c_int,
    pub sframe_info: aom_s_frame_info,
    pub seq_params: SequenceHeader,
    pub buffer_removal_time_present: bool,
    pub error: aom_internal_error_info,
    pub number_temporal_layers: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_s_frame_info {
    pub is_s_frame: core::ffi::c_int,
    pub is_s_frame_at_altref: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTInfo {
    pub tile_rows_start: core::ffi::c_int,
    pub tile_rows_end: core::ffi::c_int,
    pub tile_cols_start: core::ffi::c_int,
    pub tile_cols_end: core::ffi::c_int,
    pub start_tile: core::ffi::c_int,
    pub end_tile: core::ffi::c_int,
    pub mi_rows_to_decode: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub row_mt_exit: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXTERNAL_REFERENCES {
    pub refs: [YV12_BUFFER_CONFIG; 128],
    pub num: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DataBuffer {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type AV1DecTileMT = AV1DecTileMTData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecTileMTData {
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut TileJobsDec,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub alloc_tile_rows: core::ffi::c_int,
    pub alloc_tile_cols: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileJobsDec {
    pub tile_buffer: *mut TileBufferDec,
    pub tile_data: *mut TileDataDec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileDataDec {
    pub tile_info: TileInfo,
    pub bit_reader: aom_reader,
    pub __aom_align_tctx: [crate::platform::Align16; 0],
    pub tctx: FRAME_CONTEXT,
    pub dec_row_mt_sync: AV1DecRowMTSync,
}
pub type AV1DecRowMTSync = AV1DecRowMTSyncData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub allocated_sb_rows: core::ffi::c_int,
    pub cur_sb_col: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub intrabc_extra_top_right_sb_delay: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_dec {
    pub buf: *const core::ffi::c_uchar,
    pub tell_offs: int32_t,
    pub end: *const core::ffi::c_uchar,
    pub bptr: *const core::ffi::c_uchar,
    pub dif: od_ec_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
}
pub type od_ec_window = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileBufferDec {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadData {
    pub dcb: DecoderCodingBlock,
    pub cb_buffer_base: CB_BUFFER,
    pub bit_reader: *mut aom_reader,
    pub mc_buf: [*mut uint8_t; 2],
    pub seg_mask: *mut uint8_t,
    pub mc_buf_size: int32_t,
    pub mc_buf_use_highbd: core::ffi::c_int,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub read_coeffs_tx_intra_block_visit: decode_block_visitor_fn_t,
    pub predict_and_recon_intra_block_visit: decode_block_visitor_fn_t,
    pub read_coeffs_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub inverse_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub predict_inter_block_visit: predict_inter_block_visitor_fn_t,
    pub cfl_store_inter_block_visit: cfl_store_inter_block_visitor_fn_t,
}
pub type cfl_store_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
>;
pub type predict_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut DecoderCodingBlock, BLOCK_SIZE) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderCodingBlock {
    pub __aom_align_xd: [crate::platform::Align32; 0],

    pub xd: MACROBLOCKD,
    pub corrupted: core::ffi::c_int,
    pub mc_buf: [*mut uint8_t; 2],
    pub dqcoeff_block: [*mut tran_low_t; 3],
    pub cb_offset: [uint16_t; 3],
    pub eob_data: [*mut eob_info; 3],
    pub txb_offset: [uint16_t; 3],
    pub ref_mv_count: [uint8_t; 29],
}
pub type decode_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *mut DecoderCodingBlock,
        *mut aom_reader,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        TX_SIZE,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecWorkerData {
    pub td: *mut ThreadData,
    pub data_end: *const uint8_t,
    pub error_info: aom_internal_error_info,
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
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1LfSync = AV1LfSyncData;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT8_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C-unwind" fn aom_ceil_log2(mut n: core::ffi::c_int) -> core::ffi::c_int {
    if n < 2 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    return get_msb((n - 1 as core::ffi::c_int) as core::ffi::c_uint)
        + 1 as core::ffi::c_int;
}
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MAX_MIB_MASK: core::ffi::c_int = MAX_MIB_SIZE - 1 as core::ffi::c_int;
pub const PALETTE_MAX_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE_LOG2: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_ANGLE_DELTA: core::ffi::c_int = 3 as core::ffi::c_int;
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NEWMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << GLOBALMV_OFFSET)
    - 1 as core::ffi::c_int;
pub const GLOBALMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << REFMV_OFFSET - GLOBALMV_OFFSET) - 1 as core::ffi::c_int;
pub const REFMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 8 as core::ffi::c_int - REFMV_OFFSET) - 1 as core::ffi::c_int;
pub const DELTA_Q_SMALL: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DELTA_LF_SMALL: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REF_CAT_LEVEL: core::ffi::c_int = 640 as core::ffi::c_int;
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
static mut size_group_lookup: [uint8_t; 22] = [
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
    3 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
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
static mut intra_mode_context: [core::ffi::c_int; 13] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
#[inline]
unsafe extern "C-unwind" fn update_cdf(
    mut cdf: *mut aom_cdf_prob,
    mut val: int8_t,
    mut nsymbs: core::ffi::c_int,
) {
    let count: core::ffi::c_int = *cdf.offset(nsymbs as isize) as core::ffi::c_int;
    let rate: core::ffi::c_int = 4 as core::ffi::c_int + (count >> 4 as core::ffi::c_int)
        + (nsymbs > 3 as core::ffi::c_int) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        if i < val as core::ffi::c_int {
            let fresh0 = &raw mut *cdf.offset(i as isize);
            *fresh0 = (*fresh0 as core::ffi::c_int
                + (CDF_PROB_TOP - *cdf.offset(i as isize) as core::ffi::c_int >> rate))
                as aom_cdf_prob;
        } else {
            let fresh1 = &raw mut *cdf.offset(i as isize);
            *fresh1 = (*fresh1 as core::ffi::c_int
                - (*cdf.offset(i as isize) as core::ffi::c_int >> rate)) as aom_cdf_prob;
        }
        i += 1;
        if !(i < nsymbs - 1 as core::ffi::c_int) {
            break;
        }
    }
    let fresh2 = &raw mut *cdf.offset(nsymbs as isize);
    *fresh2 = (*fresh2 as core::ffi::c_int
        + (count < 32 as core::ffi::c_int) as core::ffi::c_int) as aom_cdf_prob;
}
pub const MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn segfeature_active(
    mut seg: *const segmentation,
    mut segment_id: uint8_t,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return ((*seg).enabled as core::ffi::c_int != 0
        && (*seg).feature_mask[segment_id as usize]
            & ((1 as core::ffi::c_int) << feature_id as core::ffi::c_int)
                as core::ffi::c_uint != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_segdata(
    mut seg: *const segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return (*seg).feature_data[segment_id as usize][feature_id as usize]
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_segment_id(
    mut segment_ids: *mut uint8_t,
    mut mi_offset: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
    mut mi_stride: core::ffi::c_int,
    mut segment_id: uint8_t,
) {
    segment_ids = segment_ids.offset(mi_offset as isize);
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_mis {
        memset(
            &raw mut *segment_ids.offset((y * mi_stride) as isize) as *mut uint8_t
                as *mut core::ffi::c_void,
            segment_id as core::ffi::c_int,
            (x_mis as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        y += 1;
    }
}
pub const MAXQ: core::ffi::c_int = 255 as core::ffi::c_int;
static mut kZeroMv: MV = {
    let mut init = mv {
        row: 0 as int16_t,
        col: 0 as int16_t,
    };
    init
};
#[inline]
unsafe extern "C-unwind" fn get_mv_from_fullmv(mut full_mv: *const FULLPEL_MV) -> MV {
    let subpel_mv: MV = {
        let mut init = mv {
            row: ((*full_mv).row as core::ffi::c_int * 8 as core::ffi::c_int) as int16_t,
            col: ((*full_mv).col as core::ffi::c_int * 8 as core::ffi::c_int) as int16_t,
        };
        init
    };
    return subpel_mv;
}
#[inline]
unsafe extern "C-unwind" fn convert_fullmv_to_mv(mut mv: *mut int_mv) {
    (*mv).as_mv = get_mv_from_fullmv(&raw mut (*mv).as_fullmv);
}
pub const WARPEDMODEL_PREC_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const GM_TRANS_ONLY_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn block_center_x(
    mut mi_col: core::ffi::c_int,
    mut bs: BLOCK_SIZE,
) -> core::ffi::c_int {
    let bw: core::ffi::c_int = block_size_wide[bs as usize] as core::ffi::c_int;
    return mi_col * MI_SIZE + bw / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn block_center_y(
    mut mi_row: core::ffi::c_int,
    mut bs: BLOCK_SIZE,
) -> core::ffi::c_int {
    let bh: core::ffi::c_int = block_size_high[bs as usize] as core::ffi::c_int;
    return mi_row * MI_SIZE + bh / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn convert_to_trans_prec(
    mut allow_hp: core::ffi::c_int,
    mut coor: core::ffi::c_int,
) -> core::ffi::c_int {
    if allow_hp != 0 {
        return if coor < 0 as core::ffi::c_int {
            -(-coor
                + ((1 as core::ffi::c_int)
                    << 16 as core::ffi::c_int - 3 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 16 as core::ffi::c_int - 3 as core::ffi::c_int)
        } else {
            coor
                + ((1 as core::ffi::c_int)
                    << 16 as core::ffi::c_int - 3 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 16 as core::ffi::c_int - 3 as core::ffi::c_int
        }
    } else {
        return (if coor < 0 as core::ffi::c_int {
            -(-coor
                + ((1 as core::ffi::c_int)
                    << 16 as core::ffi::c_int - 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 16 as core::ffi::c_int - 2 as core::ffi::c_int)
        } else {
            coor
                + ((1 as core::ffi::c_int)
                    << 16 as core::ffi::c_int - 2 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 16 as core::ffi::c_int - 2 as core::ffi::c_int
        }) * 2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn integer_mv_precision(mut mv: *mut MV) {
    let mut mod_0: core::ffi::c_int = (*mv).row as core::ffi::c_int
        % 8 as core::ffi::c_int;
    if mod_0 != 0 as core::ffi::c_int {
        (*mv).row = ((*mv).row as core::ffi::c_int - mod_0) as int16_t;
        if abs(mod_0) > 4 as core::ffi::c_int {
            if mod_0 > 0 as core::ffi::c_int {
                (*mv).row = ((*mv).row as core::ffi::c_int + 8 as core::ffi::c_int)
                    as int16_t;
            } else {
                (*mv).row = ((*mv).row as core::ffi::c_int - 8 as core::ffi::c_int)
                    as int16_t;
            }
        }
    }
    mod_0 = (*mv).col as core::ffi::c_int % 8 as core::ffi::c_int;
    if mod_0 != 0 as core::ffi::c_int {
        (*mv).col = ((*mv).col as core::ffi::c_int - mod_0) as int16_t;
        if abs(mod_0) > 4 as core::ffi::c_int {
            if mod_0 > 0 as core::ffi::c_int {
                (*mv).col = ((*mv).col as core::ffi::c_int + 8 as core::ffi::c_int)
                    as int16_t;
            } else {
                (*mv).col = ((*mv).col as core::ffi::c_int - 8 as core::ffi::c_int)
                    as int16_t;
            }
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn gm_get_motion_vector(
    mut gm: *const WarpedMotionParams,
    mut allow_hp: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mi_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut is_integer: core::ffi::c_int,
) -> int_mv {
    let mut res: int_mv = int_mv { as_int: 0 };
    if (*gm).wmtype as core::ffi::c_int == IDENTITY as core::ffi::c_int {
        res.as_int = 0 as uint32_t;
        return res;
    }
    let mut mat: *const int32_t = ((*gm).wmmat).as_ptr();
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut tx: core::ffi::c_int = 0;
    let mut ty: core::ffi::c_int = 0;
    if (*gm).wmtype as core::ffi::c_int == TRANSLATION as core::ffi::c_int {
        res.as_mv.row = ((*gm).wmmat[0 as core::ffi::c_int as usize]
            >> GM_TRANS_ONLY_PREC_DIFF) as int16_t;
        res.as_mv.col = ((*gm).wmmat[1 as core::ffi::c_int as usize]
            >> GM_TRANS_ONLY_PREC_DIFF) as int16_t;
        if is_integer != 0 {
            integer_mv_precision(&raw mut res.as_mv);
        }
        return res;
    }
    x = block_center_x(mi_col, bsize);
    y = block_center_y(mi_row, bsize);
    (*gm).wmtype as core::ffi::c_int == ROTZOOM as core::ffi::c_int;
    let xc: core::ffi::c_int = (*mat.offset(2 as core::ffi::c_int as isize)
        - ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)) * x
        + *mat.offset(3 as core::ffi::c_int as isize) * y
        + *mat.offset(0 as core::ffi::c_int as isize);
    let yc: core::ffi::c_int = *mat.offset(4 as core::ffi::c_int as isize) * x
        + (*mat.offset(5 as core::ffi::c_int as isize)
            - ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)) * y
        + *mat.offset(1 as core::ffi::c_int as isize);
    tx = convert_to_trans_prec(allow_hp, xc);
    ty = convert_to_trans_prec(allow_hp, yc);
    res.as_mv.row = ty as int16_t;
    res.as_mv.col = tx as int16_t;
    if is_integer != 0 {
        integer_mv_precision(&raw mut res.as_mv);
    }
    return res;
}
#[inline]
unsafe extern "C-unwind" fn mv_joint_vertical(mut type_0: MV_JOINT_TYPE) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == MV_JOINT_HZVNZ as core::ffi::c_int
        || type_0 as core::ffi::c_int == MV_JOINT_HNZVNZ as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn mv_joint_horizontal(mut type_0: MV_JOINT_TYPE) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == MV_JOINT_HNZVZ as core::ffi::c_int
        || type_0 as core::ffi::c_int == MV_JOINT_HNZVNZ as core::ffi::c_int)
        as core::ffi::c_int;
}
pub const CLASS0_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const CLASS0_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << CLASS0_BITS;
pub const MV_IN_USE_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const MV_UPP: core::ffi::c_int = (1 as core::ffi::c_int) << MV_IN_USE_BITS;
pub const MV_LOW: core::ffi::c_int = -((1 as core::ffi::c_int) << MV_IN_USE_BITS);
#[inline]
unsafe extern "C-unwind" fn av1_broadcast_interp_filter(
    mut filter: InterpFilter,
) -> int_interpfilters {
    let mut filters: int_interpfilters = int_interpfilters { as_int: 0 };
    filters.as_filters.x_filter = filter as uint16_t;
    filters.as_filters.y_filter = filter as uint16_t;
    return filters;
}
#[inline]
unsafe extern "C-unwind" fn av1_unswitchable_filter(mut filter: InterpFilter) -> InterpFilter {
    return (if filter as core::ffi::c_int == SWITCHABLE as core::ffi::c_int {
        EIGHTTAP_REGULAR as core::ffi::c_int
    } else {
        filter as core::ffi::c_int
    }) as InterpFilter;
}
static mut av1_ext_tx_inv: [[core::ffi::c_int; 16]; 6] = [
    [
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
        0 as core::ffi::c_int,
    ],
    [
        9 as core::ffi::c_int,
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
        9 as core::ffi::c_int,
        0 as core::ffi::c_int,
        3 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
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
        9 as core::ffi::c_int,
        0 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        3 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
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
        9 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
        5 as core::ffi::c_int,
        3 as core::ffi::c_int,
        6 as core::ffi::c_int,
        7 as core::ffi::c_int,
        8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        9 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        12 as core::ffi::c_int,
        13 as core::ffi::c_int,
        14 as core::ffi::c_int,
        15 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
        5 as core::ffi::c_int,
        3 as core::ffi::c_int,
        6 as core::ffi::c_int,
        7 as core::ffi::c_int,
        8 as core::ffi::c_int,
    ],
];
pub const REF_SCALE_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const REF_NO_SCALE: core::ffi::c_int = (1 as core::ffi::c_int) << REF_SCALE_SHIFT;
pub const REF_INVALID_SCALE: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn av1_is_valid_scale(
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    return ((*sf).x_scale_fp != REF_INVALID_SCALE
        && (*sf).y_scale_fp != REF_INVALID_SCALE) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_scaled(mut sf: *const scale_factors) -> core::ffi::c_int {
    return (av1_is_valid_scale(sf) != 0
        && ((*sf).x_scale_fp != REF_NO_SCALE || (*sf).y_scale_fp != REF_NO_SCALE))
        as core::ffi::c_int;
}
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_PALETTE_BLOCK_WIDTH: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_PALETTE_BLOCK_HEIGHT: core::ffi::c_int = 64 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_comp_ref_allowed(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return ((if (block_size_wide[bsize as usize] as core::ffi::c_int)
        < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_inter_mode(mut mode: PREDICTION_MODE) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_inter_compound_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= COMP_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < COMP_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn compound_ref0_mode(mut mode: PREDICTION_MODE) -> PREDICTION_MODE {
    static mut lut: [PREDICTION_MODE; 25] = [
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
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
    ];
    return lut[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn compound_ref1_mode(mut mode: PREDICTION_MODE) -> PREDICTION_MODE {
    static mut lut: [PREDICTION_MODE; 25] = [
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
    ];
    return lut[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn have_nearmv_in_inter_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int == NEARMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAR_NEARMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_masked_compound_type(
    mut type_0: COMPOUND_TYPE,
) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == COMPOUND_WEDGE as core::ffi::c_int
        || type_0 as core::ffi::c_int == COMPOUND_DIFFWTD as core::ffi::c_int)
        as core::ffi::c_int;
}
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
unsafe extern "C-unwind" fn has_second_ref(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn comp_ref0(mut ref_idx: core::ffi::c_int) -> MV_REFERENCE_FRAME {
    static mut lut: [MV_REFERENCE_FRAME; 9] = [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ];
    return lut[ref_idx as usize];
}
#[inline]
unsafe extern "C-unwind" fn comp_ref1(mut ref_idx: core::ffi::c_int) -> MV_REFERENCE_FRAME {
    static mut lut: [MV_REFERENCE_FRAME; 9] = [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ];
    return lut[ref_idx as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_global_mv_block(
    mbmi: *const MB_MODE_INFO,
    mut type_0: TransformationType,
) -> core::ffi::c_int {
    let mode: PREDICTION_MODE = (*mbmi).mode;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let block_size_allowed: core::ffi::c_int = ((if (block_size_wide[bsize as usize]
        as core::ffi::c_int) < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
    return ((mode as core::ffi::c_int == GLOBALMV as core::ffi::c_int
        || mode as core::ffi::c_int == GLOBAL_GLOBALMV as core::ffi::c_int)
        && type_0 as core::ffi::c_int > TRANSLATION as core::ffi::c_int
        && block_size_allowed != 0) as core::ffi::c_int;
}
static mut av1_num_ext_tx_set: [core::ffi::c_int; 6] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    7 as core::ffi::c_int,
    12 as core::ffi::c_int,
    16 as core::ffi::c_int,
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
static mut ext_tx_set_index: [[core::ffi::c_int; 6]; 2] = [
    [
        0 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        -(1 as core::ffi::c_int),
    ],
    [
        0 as core::ffi::c_int,
        3 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        -(1 as core::ffi::c_int),
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
];
#[inline]
unsafe extern "C-unwind" fn get_ext_tx_set(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> core::ffi::c_int {
    let set_type: TxSetType = av1_get_ext_tx_set_type(tx_size, is_inter, use_reduced_set)
        as TxSetType;
    return ext_tx_set_index[is_inter as usize][set_type as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_ext_tx_types(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> core::ffi::c_int {
    let set_type: core::ffi::c_int = av1_get_ext_tx_set_type(
        tx_size,
        is_inter,
        use_reduced_set,
    ) as core::ffi::c_int;
    return av1_num_ext_tx_set[set_type as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_plane_block_size(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    return av1_ss_size_lookup[bsize
        as usize][subsampling_x as usize][subsampling_y as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_bsize(bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_mode(
    mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= SINGLE_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < SINGLE_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_ref(
    mut rf: *const MV_REFERENCE_FRAME,
) -> core::ffi::c_int {
    return (*rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            <= INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (is_interintra_allowed_bsize((*mbmi).bsize) != 0
        && is_interintra_allowed_mode((*mbmi).mode) != 0
        && is_interintra_allowed_ref(((*mbmi).ref_frame).as_ptr()) != 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_motion_variation_allowed_bsize(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    return ((if (block_size_wide[bsize as usize] as core::ffi::c_int)
        < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_motion_variation_allowed_compound(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (has_second_ref(mbmi) == 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn check_num_overlappable_neighbors(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).overlappable_neighbors as core::ffi::c_int != 0 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn motion_mode_allowed(
    mut gm_params: *const WarpedMotionParams,
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
    mut allow_warped_motion: core::ffi::c_int,
) -> MOTION_MODE {
    if check_num_overlappable_neighbors(mbmi) == 0 {
        return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    }
    if (*xd).cur_frame_force_integer_mv == 0 as core::ffi::c_int {
        let gm_type: TransformationType = (*gm_params
            .offset((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize))
            .wmtype;
        if is_global_mv_block(mbmi, gm_type) != 0 {
            return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
        }
    }
    if is_motion_variation_allowed_bsize((*mbmi).bsize) != 0
        && is_inter_mode((*mbmi).mode) != 0
        && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            != INTRA_FRAME as core::ffi::c_int
        && is_motion_variation_allowed_compound(mbmi) != 0
    {
        if (*mbmi).num_proj_ref as core::ffi::c_int >= 1 as core::ffi::c_int
            && allow_warped_motion != 0 && (*xd).cur_frame_force_integer_mv == 0
            && av1_is_scaled(
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
            ) == 0
        {
            return WARPED_CAUSAL as core::ffi::c_int as MOTION_MODE;
        }
        return OBMC_CAUSAL as core::ffi::c_int as MOTION_MODE;
    }
    return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_palette(
    mut allow_screen_content_tools: core::ffi::c_int,
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    return (allow_screen_content_tools != 0
        && block_size_wide[sb_type as usize] as core::ffi::c_int
            <= MAX_PALETTE_BLOCK_WIDTH
        && block_size_high[sb_type as usize] as core::ffi::c_int
            <= MAX_PALETTE_BLOCK_HEIGHT
        && sb_type as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_nontrans_global_motion(
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    let mut ref_0: core::ffi::c_int = 0;
    if (*mbmi).mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
        && (*mbmi).mode as core::ffi::c_int != GLOBAL_GLOBALMV as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (if (mi_size_wide[(*mbmi).bsize as usize] as core::ffi::c_int)
        < mi_size_high[(*mbmi).bsize as usize] as core::ffi::c_int
    {
        mi_size_wide[(*mbmi).bsize as usize] as core::ffi::c_int
    } else {
        mi_size_high[(*mbmi).bsize as usize] as core::ffi::c_int
    }) < 2 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    ref_0 = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + has_second_ref(mbmi) {
        if (*((*xd).global_motion).offset((*mbmi).ref_frame[ref_0 as usize] as isize))
            .wmtype as core::ffi::c_int == TRANSLATION as core::ffi::c_int
        {
            return 0 as core::ffi::c_int;
        }
        ref_0 += 1;
    }
    return 1 as core::ffi::c_int;
}
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MAX_LOOP_FILTER: core::ffi::c_int = 63 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn frame_is_intra_only(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame_map_idx(
    cm: *const AV1_COMMON,
    ref_frame: MV_REFERENCE_FRAME,
) -> core::ffi::c_int {
    return if ref_frame as core::ffi::c_int >= LAST_FRAME as core::ffi::c_int
        && ref_frame as core::ffi::c_int <= EXTREF_FRAME as core::ffi::c_int
    {
        (*cm)
            .remapped_ref_idx[(ref_frame as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize]
    } else {
        INVALID_IDX
    };
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame_buf(
    cm: *const AV1_COMMON,
    ref_frame: MV_REFERENCE_FRAME,
) -> *mut RefCntBuffer {
    let map_idx: core::ffi::c_int = get_ref_frame_map_idx(cm, ref_frame)
        as core::ffi::c_int;
    return if map_idx != INVALID_IDX {
        (*cm).ref_frame_map[map_idx as usize]
    } else {
        0 as *mut RefCntBuffer
    };
}
#[inline]
unsafe extern "C-unwind" fn get_ref_scale_factors_const(
    cm: *const AV1_COMMON,
    ref_frame: MV_REFERENCE_FRAME,
) -> *const scale_factors {
    let map_idx: core::ffi::c_int = get_ref_frame_map_idx(cm, ref_frame)
        as core::ffi::c_int;
    return if map_idx != INVALID_IDX {
        &*((*cm).ref_scale_factors).as_ptr().offset(map_idx as isize)
            as *const scale_factors
    } else {
        0 as *const scale_factors
    };
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
unsafe extern "C-unwind" fn get_y_mode_cdf(
    mut tile_ctx: *mut FRAME_CONTEXT,
    mut above_mi: *const MB_MODE_INFO,
    mut left_mi: *const MB_MODE_INFO,
) -> *mut aom_cdf_prob {
    let above: PREDICTION_MODE = av1_above_block_mode(above_mi) as PREDICTION_MODE;
    let left: PREDICTION_MODE = av1_left_block_mode(left_mi) as PREDICTION_MODE;
    let above_ctx: core::ffi::c_int = intra_mode_context[above as usize];
    let left_ctx: core::ffi::c_int = intra_mode_context[left as usize];
    return ((*tile_ctx).kf_y_cdf[above_ctx as usize][left_ctx as usize]).as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn get_mi_grid_idx(
    mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    return mi_row * (*mi_params).mi_stride + mi_col;
}
#[inline]
unsafe extern "C-unwind" fn is_cfl_allowed(mut xd: *const MACROBLOCKD) -> CFL_ALLOWED_TYPE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        let ssx: core::ffi::c_int = (*xd).plane[AOM_PLANE_U as usize].subsampling_x;
        let ssy: core::ffi::c_int = (*xd).plane[AOM_PLANE_U as usize].subsampling_y;
        let plane_bsize: core::ffi::c_int = get_plane_block_size(bsize, ssx, ssy)
            as core::ffi::c_int;
        return (plane_bsize == BLOCK_4X4 as core::ffi::c_int) as core::ffi::c_int
            as CFL_ALLOWED_TYPE;
    }
    return (block_size_wide[bsize as usize] as core::ffi::c_int <= 32 as core::ffi::c_int
        && block_size_high[bsize as usize] as core::ffi::c_int <= 32 as core::ffi::c_int)
        as core::ffi::c_int as CFL_ALLOWED_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn store_cfl_required(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
) -> CFL_ALLOWED_TYPE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    if (*(*cm).seq_params).monochrome != 0 {
        return CFL_DISALLOWED as core::ffi::c_int as CFL_ALLOWED_TYPE;
    }
    if !(*xd).is_chroma_ref {
        return CFL_ALLOWED as core::ffi::c_int as CFL_ALLOWED_TYPE;
    }
    return (is_inter_block(mbmi) == 0
        && (*mbmi).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int)
        as core::ffi::c_int as CFL_ALLOWED_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn get_relative_dist(
    mut oh: *const OrderHintInfo,
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*oh).enable_order_hint == 0 {
        return 0 as core::ffi::c_int;
    }
    let bits: core::ffi::c_int = (*oh).order_hint_bits_minus_1 + 1 as core::ffi::c_int;
    let mut diff: core::ffi::c_int = a - b;
    let m: core::ffi::c_int = (1 as core::ffi::c_int) << bits - 1 as core::ffi::c_int;
    diff = (diff & m - 1 as core::ffi::c_int) - (diff & m);
    return diff;
}
#[inline]
unsafe extern "C-unwind" fn lower_mv_precision(
    mut mv: *mut MV,
    mut allow_hp: core::ffi::c_int,
    mut is_integer: core::ffi::c_int,
) {
    if is_integer != 0 {
        integer_mv_precision(mv);
    } else if allow_hp == 0 {
        if (*mv).row as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
            (*mv).row = ((*mv).row as core::ffi::c_int
                + if (*mv).row as core::ffi::c_int > 0 as core::ffi::c_int {
                    -(1 as core::ffi::c_int)
                } else {
                    1 as core::ffi::c_int
                }) as int16_t;
        }
        if (*mv).col as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
            (*mv).col = ((*mv).col as core::ffi::c_int
                + if (*mv).col as core::ffi::c_int > 0 as core::ffi::c_int {
                    -(1 as core::ffi::c_int)
                } else {
                    1 as core::ffi::c_int
                }) as int16_t;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn get_uni_comp_ref_idx(rf: *const MV_REFERENCE_FRAME) -> int8_t {
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        <= INTRA_FRAME as core::ffi::c_int
    {
        return -(1 as core::ffi::c_int) as int8_t;
    }
    if (*rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        < BWDREF_FRAME as core::ffi::c_int
        && *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            >= BWDREF_FRAME as core::ffi::c_int
    {
        return -(1 as core::ffi::c_int) as int8_t;
    }
    let mut ref_idx: int8_t = 0 as int8_t;
    while (ref_idx as core::ffi::c_int) < TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int {
        if *rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == comp_ref0(ref_idx as core::ffi::c_int) as core::ffi::c_int
            && *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == comp_ref1(ref_idx as core::ffi::c_int) as core::ffi::c_int
        {
            return ref_idx;
        }
        ref_idx += 1;
    }
    return -(1 as core::ffi::c_int) as int8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_ref_frame_type(rf: *const MV_REFERENCE_FRAME) -> int8_t {
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
    {
        let uni_comp_ref_idx: int8_t = get_uni_comp_ref_idx(rf) as int8_t;
        if uni_comp_ref_idx as core::ffi::c_int >= 0 as core::ffi::c_int {
            return (REF_FRAMES as core::ffi::c_int
                + FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                + uni_comp_ref_idx as core::ffi::c_int) as int8_t
        } else {
            return (REF_FRAMES as core::ffi::c_int
                + (*rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    - LAST_FRAME as core::ffi::c_int)
                + (*rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    - BWDREF_FRAME as core::ffi::c_int) * FWD_REFS as core::ffi::c_int)
                as int8_t
        }
    }
    return *rf.offset(0 as core::ffi::c_int as isize);
}
static mut compound_mode_ctx_map: [[uint16_t; 5]; 3] = [
    [
        0 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
    ],
    [
        1 as core::ffi::c_int as uint16_t,
        2 as core::ffi::c_int as uint16_t,
        3 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
    ],
    [
        4 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
        5 as core::ffi::c_int as uint16_t,
        6 as core::ffi::c_int as uint16_t,
        7 as core::ffi::c_int as uint16_t,
    ],
];
#[inline]
unsafe extern "C-unwind" fn av1_mode_context_analyzer(
    mode_context: *const int16_t,
    rf: *const MV_REFERENCE_FRAME,
) -> int16_t {
    let ref_frame: int8_t = av1_ref_frame_type(rf) as int8_t;
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        <= INTRA_FRAME as core::ffi::c_int
    {
        return *mode_context.offset(ref_frame as isize);
    }
    let newmv_ctx: int16_t = (*mode_context.offset(ref_frame as isize)
        as core::ffi::c_int & NEWMV_CTX_MASK) as int16_t;
    let refmv_ctx: int16_t = (*mode_context.offset(ref_frame as isize)
        as core::ffi::c_int >> REFMV_OFFSET & REFMV_CTX_MASK) as int16_t;
    let comp_ctx: int16_t = compound_mode_ctx_map[(refmv_ctx as core::ffi::c_int
        >> 1 as core::ffi::c_int)
        as usize][(if (newmv_ctx as core::ffi::c_int)
        < 5 as core::ffi::c_int - 1 as core::ffi::c_int
    {
        newmv_ctx as core::ffi::c_int
    } else {
        5 as core::ffi::c_int - 1 as core::ffi::c_int
    }) as usize] as int16_t;
    return comp_ctx;
}
#[inline]
unsafe extern "C-unwind" fn av1_drl_ctx(
    mut ref_mv_weight: *const uint16_t,
    mut ref_idx: core::ffi::c_int,
) -> uint8_t {
    if *ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int >= REF_CAT_LEVEL
        && *ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int >= REF_CAT_LEVEL
    {
        return 0 as uint8_t;
    }
    if *ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int >= REF_CAT_LEVEL
        && (*ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < REF_CAT_LEVEL
    {
        return 1 as uint8_t;
    }
    if (*ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int) < REF_CAT_LEVEL
        && (*ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < REF_CAT_LEVEL
    {
        return 2 as uint8_t;
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_collect_neighbors_ref_counts(xd: *mut MACROBLOCKD) {
    memset(
        &raw mut (*xd).neighbors_ref_counts as *mut [uint8_t; 8] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
    let ref_counts: *mut uint8_t = ((*xd).neighbors_ref_counts).as_mut_ptr();
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_in_image: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let left_in_image: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    if above_in_image != 0 && is_inter_block(above_mbmi) != 0 {
        let fresh3 = &raw mut *ref_counts
            .offset((*above_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        if has_second_ref(above_mbmi) != 0 {
            let fresh4 = &raw mut *ref_counts
                .offset(
                    (*above_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize,
                );
            *fresh4 = (*fresh4).wrapping_add(1);
        }
    }
    if left_in_image != 0 && is_inter_block(left_mbmi) != 0 {
        let fresh5 = &raw mut *ref_counts
            .offset((*left_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        if has_second_ref(left_mbmi) != 0 {
            let fresh6 = &raw mut *ref_counts
                .offset((*left_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize);
            *fresh6 = (*fresh6).wrapping_add(1);
        }
    }
}
pub const INTRABC_DELAY_PIXELS: core::ffi::c_int = 256 as core::ffi::c_int;
pub const INTRABC_DELAY_SB64: core::ffi::c_int = INTRABC_DELAY_PIXELS
    / 64 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_find_ref_dv(
    mut ref_dv: *mut int_mv,
    tile: *const TileInfo,
    mut mib_size: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
) {
    if mi_row - mib_size < (*tile).mi_row_start {
        (*ref_dv).as_fullmv.row = 0 as int16_t;
        (*ref_dv).as_fullmv.col = (-MI_SIZE * mib_size - INTRABC_DELAY_PIXELS)
            as int16_t;
    } else {
        (*ref_dv).as_fullmv.row = (-MI_SIZE * mib_size) as int16_t;
        (*ref_dv).as_fullmv.col = 0 as int16_t;
    }
    convert_fullmv_to_mv(ref_dv);
}
#[inline]
unsafe extern "C-unwind" fn av1_is_dv_valid(
    dv: MV,
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mib_size_log2: core::ffi::c_int,
) -> core::ffi::c_int {
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let SCALE_PX_TO_MV: core::ffi::c_int = 8 as core::ffi::c_int;
    if dv.row as core::ffi::c_int & SCALE_PX_TO_MV - 1 as core::ffi::c_int != 0
        || dv.col as core::ffi::c_int & SCALE_PX_TO_MV - 1 as core::ffi::c_int != 0
    {
        return 0 as core::ffi::c_int;
    }
    let tile: *const TileInfo = &(*xd).tile;
    let src_top_edge: core::ffi::c_int = mi_row * MI_SIZE * SCALE_PX_TO_MV
        + dv.row as core::ffi::c_int;
    let tile_top_edge: core::ffi::c_int = (*tile).mi_row_start * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_top_edge < tile_top_edge {
        return 0 as core::ffi::c_int;
    }
    let src_left_edge: core::ffi::c_int = mi_col * MI_SIZE * SCALE_PX_TO_MV
        + dv.col as core::ffi::c_int;
    let tile_left_edge: core::ffi::c_int = (*tile).mi_col_start * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_left_edge < tile_left_edge {
        return 0 as core::ffi::c_int;
    }
    let src_bottom_edge: core::ffi::c_int = (mi_row * MI_SIZE + bh) * SCALE_PX_TO_MV
        + dv.row as core::ffi::c_int;
    let tile_bottom_edge: core::ffi::c_int = (*tile).mi_row_end * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_bottom_edge > tile_bottom_edge {
        return 0 as core::ffi::c_int;
    }
    let src_right_edge: core::ffi::c_int = (mi_col * MI_SIZE + bw) * SCALE_PX_TO_MV
        + dv.col as core::ffi::c_int;
    let tile_right_edge: core::ffi::c_int = (*tile).mi_col_end * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_right_edge > tile_right_edge {
        return 0 as core::ffi::c_int;
    }
    if (*xd).is_chroma_ref as core::ffi::c_int != 0
        && av1_num_planes(cm) > 1 as core::ffi::c_int
    {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(1 as core::ffi::c_int as isize) as *const macroblockd_plane;
        if bw < 8 as core::ffi::c_int && (*pd).subsampling_x != 0 {
            if src_left_edge < tile_left_edge + 4 as core::ffi::c_int * SCALE_PX_TO_MV {
                return 0 as core::ffi::c_int;
            }
        }
        if bh < 8 as core::ffi::c_int && (*pd).subsampling_y != 0 {
            if src_top_edge < tile_top_edge + 4 as core::ffi::c_int * SCALE_PX_TO_MV {
                return 0 as core::ffi::c_int;
            }
        }
    }
    let max_mib_size: core::ffi::c_int = (1 as core::ffi::c_int) << mib_size_log2;
    let active_sb_row: core::ffi::c_int = mi_row >> mib_size_log2;
    let active_sb64_col: core::ffi::c_int = mi_col * MI_SIZE >> 6 as core::ffi::c_int;
    let sb_size: core::ffi::c_int = max_mib_size * MI_SIZE;
    let src_sb_row: core::ffi::c_int = ((src_bottom_edge >> 3 as core::ffi::c_int)
        - 1 as core::ffi::c_int) / sb_size;
    let src_sb64_col: core::ffi::c_int = (src_right_edge >> 3 as core::ffi::c_int)
        - 1 as core::ffi::c_int >> 6 as core::ffi::c_int;
    let total_sb64_per_row: core::ffi::c_int = ((*tile).mi_col_end - (*tile).mi_col_start
        - 1 as core::ffi::c_int >> 4 as core::ffi::c_int) + 1 as core::ffi::c_int;
    let active_sb64: core::ffi::c_int = active_sb_row * total_sb64_per_row
        + active_sb64_col;
    let src_sb64: core::ffi::c_int = src_sb_row * total_sb64_per_row + src_sb64_col;
    if src_sb64 >= active_sb64 - INTRABC_DELAY_SB64 {
        return 0 as core::ffi::c_int;
    }
    let gradient: core::ffi::c_int = 1 as core::ffi::c_int + INTRABC_DELAY_SB64
        + (sb_size > 64 as core::ffi::c_int) as core::ffi::c_int;
    let wf_offset: core::ffi::c_int = gradient * (active_sb_row - src_sb_row);
    if src_sb_row > active_sb_row
        || src_sb64_col >= active_sb64_col - INTRABC_DELAY_SB64 + wf_offset
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_segment_id(
    mi_params: *const CommonModeInfoParams,
    mut segment_ids: *const uint8_t,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> uint8_t {
    let mi_offset: core::ffi::c_int = mi_row * (*mi_params).mi_cols + mi_col;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let xmis: core::ffi::c_int = if (*mi_params).mi_cols - mi_col < bw {
        (*mi_params).mi_cols - mi_col
    } else {
        bw
    };
    let ymis: core::ffi::c_int = if (*mi_params).mi_rows - mi_row < bh {
        (*mi_params).mi_rows - mi_row
    } else {
        bh
    };
    let seg_stride: core::ffi::c_int = (*mi_params).mi_cols;
    let mut segment_id: uint8_t = MAX_SEGMENTS as uint8_t;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < ymis {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < xmis {
            segment_id = (if (segment_id as core::ffi::c_int)
                < *segment_ids.offset((mi_offset + y * seg_stride + x) as isize)
                    as core::ffi::c_int
            {
                segment_id as core::ffi::c_int
            } else {
                *segment_ids.offset((mi_offset + y * seg_stride + x) as isize)
                    as core::ffi::c_int
            }) as uint8_t;
            x += 1;
        }
        y += 1;
    }
    return segment_id;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_spatial_seg_pred(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    mut cdf_index: *mut core::ffi::c_int,
    mut skip_over4x4: core::ffi::c_int,
) -> uint8_t {
    let step_size: core::ffi::c_int = if skip_over4x4 != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut prev_ul: uint8_t = UINT8_MAX as uint8_t;
    let mut prev_l: uint8_t = UINT8_MAX as uint8_t;
    let mut prev_u: uint8_t = UINT8_MAX as uint8_t;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mut seg_map: *const uint8_t = (*(*cm).cur_frame).seg_map;
    if (*xd).up_available as core::ffi::c_int != 0
        && (*xd).left_available as core::ffi::c_int != 0
    {
        prev_ul = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - step_size,
            mi_col - step_size,
        );
    }
    if (*xd).up_available {
        prev_u = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - step_size,
            mi_col - 0 as core::ffi::c_int,
        );
    }
    if (*xd).left_available {
        prev_l = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - 0 as core::ffi::c_int,
            mi_col - step_size,
        );
    }
    if prev_ul as core::ffi::c_int == UINT8_MAX {
        *cdf_index = 0 as core::ffi::c_int;
    } else if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int
        && prev_ul as core::ffi::c_int == prev_l as core::ffi::c_int
    {
        *cdf_index = 2 as core::ffi::c_int;
    } else if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int
        || prev_ul as core::ffi::c_int == prev_l as core::ffi::c_int
        || prev_u as core::ffi::c_int == prev_l as core::ffi::c_int
    {
        *cdf_index = 1 as core::ffi::c_int;
    } else {
        *cdf_index = 0 as core::ffi::c_int;
    }
    if prev_u as core::ffi::c_int == UINT8_MAX {
        return (if prev_l as core::ffi::c_int == UINT8_MAX {
            0 as core::ffi::c_int
        } else {
            prev_l as core::ffi::c_int
        }) as uint8_t;
    }
    if prev_l as core::ffi::c_int == UINT8_MAX {
        return prev_u;
    }
    return (if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int {
        prev_u as core::ffi::c_int
    } else {
        prev_l as core::ffi::c_int
    }) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_context_seg_id(mut xd: *const MACROBLOCKD) -> uint8_t {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_sip: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).seg_id_predicted() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_sip: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).seg_id_predicted() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return (above_sip + left_sip) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn get_comp_index_context(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bck_buf: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
    );
    let fwd_buf: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
    );
    let mut bck_frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut fwd_frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut cur_frame_index: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    if !bck_buf.is_null() {
        bck_frame_index = (*bck_buf).order_hint as core::ffi::c_int;
    }
    if !fwd_buf.is_null() {
        fwd_frame_index = (*fwd_buf).order_hint as core::ffi::c_int;
    }
    let mut fwd: core::ffi::c_int = abs(
        get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            fwd_frame_index,
            cur_frame_index,
        ),
    );
    let mut bck: core::ffi::c_int = abs(
        get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            cur_frame_index,
            bck_frame_index,
        ),
    );
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut above_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut left_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let offset: core::ffi::c_int = (fwd == bck) as core::ffi::c_int;
    if !above_mi.is_null() {
        if has_second_ref(above_mi) != 0 {
            above_ctx = (*above_mi).compound_idx() as core::ffi::c_int;
        } else if (*above_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            above_ctx = 1 as core::ffi::c_int;
        }
    }
    if !left_mi.is_null() {
        if has_second_ref(left_mi) != 0 {
            left_ctx = (*left_mi).compound_idx() as core::ffi::c_int;
        } else if (*left_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            left_ctx = 1 as core::ffi::c_int;
        }
    }
    return above_ctx + left_ctx + 3 as core::ffi::c_int * offset;
}
#[inline]
unsafe extern "C-unwind" fn get_comp_group_idx_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut above_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut left_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mi.is_null() {
        if has_second_ref(above_mi) != 0 {
            above_ctx = (*above_mi).comp_group_idx() as core::ffi::c_int;
        } else if (*above_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            above_ctx = 3 as core::ffi::c_int;
        }
    }
    if !left_mi.is_null() {
        if has_second_ref(left_mi) != 0 {
            left_ctx = (*left_mi).comp_group_idx() as core::ffi::c_int;
        } else if (*left_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            left_ctx = 3 as core::ffi::c_int;
        }
    }
    return if (5 as core::ffi::c_int) < above_ctx + left_ctx {
        5 as core::ffi::c_int
    } else {
        above_ctx + left_ctx
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_skip_mode_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_skip_mode: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).skip_mode() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_skip_mode: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).skip_mode() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return above_skip_mode + left_skip_mode;
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
unsafe extern "C-unwind" fn av1_get_palette_bsize_ctx(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    return num_pels_log2_lookup[bsize as usize] as core::ffi::c_int
        - num_pels_log2_lookup[BLOCK_8X8 as core::ffi::c_int as usize]
            as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_palette_mode_ctx(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mi.is_null() {
        ctx
            += ((*above_mi)
                .palette_mode_info
                .palette_size[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    if !left_mi.is_null() {
        ctx
            += ((*left_mi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize]
                as core::ffi::c_int > 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    return ctx;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p2(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p2(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_bwdref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_bwdref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_bwdref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_bwdref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_bwdref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_bwdref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p1(xd)
        as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p2(xd)
        as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p3(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p3(xd)
        as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p4(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p4(xd)
        as usize][3 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p5(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p5(xd)
        as usize][4 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p6(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p6(xd)
        as usize][5 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_interinter_compound_used(
    mut type_0: COMPOUND_TYPE,
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    let comp_allowed: core::ffi::c_int = is_comp_ref_allowed(sb_type)
        as core::ffi::c_int;
    match type_0 as core::ffi::c_int {
        0 | 1 | 3 => return comp_allowed,
        2 => {
            return (comp_allowed != 0
                && av1_wedge_params_lookup[sb_type as usize].wedge_types
                    > 0 as core::ffi::c_int) as core::ffi::c_int;
        }
        _ => return 0 as core::ffi::c_int,
    };
}
#[inline]
unsafe extern "C-unwind" fn is_any_masked_compound_used(
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut comp_type: COMPOUND_TYPE = 0;
    let mut i: core::ffi::c_int = 0;
    if is_comp_ref_allowed(sb_type) == 0 {
        return 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < COMPOUND_TYPES as core::ffi::c_int {
        comp_type = i as COMPOUND_TYPE;
        if is_masked_compound_type(comp_type) != 0
            && is_interinter_compound_used(comp_type, sb_type) != 0
        {
            return 1 as core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_wedge_used(mut sb_type: BLOCK_SIZE) -> core::ffi::c_int {
    return (av1_wedge_params_lookup[sb_type as usize].wedge_types
        > 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_default_interp_filters(
    mbmi: *mut MB_MODE_INFO,
    mut frame_interp_filter: InterpFilter,
) {
    (*mbmi).interp_filters = av1_broadcast_interp_filter(
        av1_unswitchable_filter(frame_interp_filter),
    );
}
#[inline]
unsafe extern "C-unwind" fn av1_is_interp_needed(xd: *const MACROBLOCKD) -> core::ffi::c_int {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*mbmi).skip_mode() != 0 {
        return 0 as core::ffi::c_int;
    }
    if (*mbmi).motion_mode as core::ffi::c_int == WARPED_CAUSAL as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if is_nontrans_global_motion(xd, *((*xd).mi).offset(0 as core::ffi::c_int as isize))
        != 0
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_is_directional_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= V_PRED as core::ffi::c_int
        && mode as core::ffi::c_int <= D67_PRED as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_use_angle_delta(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_filter_intra_allowed_bsize(
    cm: *const AV1_COMMON,
    mut bs: BLOCK_SIZE,
) -> core::ffi::c_int {
    if (*(*cm).seq_params).enable_filter_intra == 0
        || bs as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    return (block_size_wide[bs as usize] as core::ffi::c_int <= 32 as core::ffi::c_int
        && block_size_high[bs as usize] as core::ffi::c_int <= 32 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_filter_intra_allowed(
    cm: *const AV1_COMMON,
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int
        && (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == 0 as core::ffi::c_int
        && av1_filter_intra_allowed_bsize(cm, (*mbmi).bsize) != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_(
    mut r: *mut aom_reader,
    mut prob: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut p: core::ffi::c_int = 0x7fffff as core::ffi::c_int
        - (prob << 15 as core::ffi::c_int) + prob >> 8 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = od_ec_decode_bool_q15(
        &raw mut (*r).ec,
        p as core::ffi::c_uint,
    );
    return bit;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_bit_(mut r: *mut aom_reader) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    ret = aom_read_(r, 128 as core::ffi::c_int);
    return ret;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_literal_(
    mut r: *mut aom_reader,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut literal: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        literal |= aom_read_bit_(r) << bit;
        bit -= 1;
    }
    return literal;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_cdf_(
    mut r: *mut aom_reader,
    mut cdf: *const aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut symb: core::ffi::c_int = 0;
    symb = od_ec_decode_cdf_q15(&raw mut (*r).ec, cdf as *const uint16_t, nsymbs);
    return symb;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_symbol_(
    mut r: *mut aom_reader,
    mut cdf: *mut aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    ret = aom_read_cdf_(r, cdf, nsymbs);
    if (*r).allow_update_cdf != 0 {
        update_cdf(cdf, ret as int8_t, nsymbs);
    }
    return ret;
}
unsafe extern "C-unwind" fn read_intra_mode(
    mut r: *mut aom_reader,
    mut cdf: *mut aom_cdf_prob,
) -> PREDICTION_MODE {
    return aom_read_symbol_(r, cdf, INTRA_MODES as core::ffi::c_int) as PREDICTION_MODE;
}
unsafe extern "C-unwind" fn read_cdef(
    mut cm: *mut AV1_COMMON,
    mut r: *mut aom_reader,
    xd: *mut MACROBLOCKD,
) {
    let skip_txfm: core::ffi::c_int = (**((*xd).mi)
        .offset(0 as core::ffi::c_int as isize))
        .skip_txfm as core::ffi::c_int;
    if (*cm).features.coded_lossless {
        return;
    }
    if (*cm).features.allow_intrabc {
        return;
    }
    let sb_mask: core::ffi::c_int = (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let mi_row_in_sb: core::ffi::c_int = (*xd).mi_row & sb_mask;
    let mi_col_in_sb: core::ffi::c_int = (*xd).mi_col & sb_mask;
    if mi_row_in_sb == 0 as core::ffi::c_int && mi_col_in_sb == 0 as core::ffi::c_int {
        (*xd).cdef_transmitted[3 as core::ffi::c_int as usize] = false_0 != 0;
        (*xd).cdef_transmitted[2 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[3 as core::ffi::c_int as usize];
        (*xd).cdef_transmitted[1 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[2 as core::ffi::c_int as usize];
        (*xd).cdef_transmitted[0 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[1 as core::ffi::c_int as usize];
    }
    let cdef_size: core::ffi::c_int = (1 as core::ffi::c_int)
        << 6 as core::ffi::c_int - MI_SIZE_LOG2;
    let index_mask: core::ffi::c_int = cdef_size;
    let cdef_unit_row_in_sb: core::ffi::c_int = ((*xd).mi_row & index_mask
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let cdef_unit_col_in_sb: core::ffi::c_int = ((*xd).mi_col & index_mask
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let index: core::ffi::c_int = if (*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_128X128 as core::ffi::c_int
    {
        cdef_unit_col_in_sb + 2 as core::ffi::c_int * cdef_unit_row_in_sb
    } else {
        0 as core::ffi::c_int
    };
    if !(*xd).cdef_transmitted[index as usize] && skip_txfm == 0 {
        let first_block_mask: core::ffi::c_int = !(cdef_size - 1 as core::ffi::c_int);
        let mi_params: *mut CommonModeInfoParams = &raw mut (*cm).mi_params;
        let grid_idx: core::ffi::c_int = get_mi_grid_idx(
            mi_params,
            (*xd).mi_row & first_block_mask,
            (*xd).mi_col & first_block_mask,
        ) as core::ffi::c_int;
        let mbmi: *mut MB_MODE_INFO = *((*mi_params).mi_grid_base)
            .offset(grid_idx as isize);
        (*mbmi)
            .set_cdef_strength(
                aom_read_literal_(r, (*cm).cdef_info.cdef_bits) as int8_t as int8_t,
            );
        (*xd).cdef_transmitted[index as usize] = true_0 != 0;
    }
}
unsafe extern "C-unwind" fn read_delta_qindex(
    mut cm: *mut AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut r: *mut aom_reader,
    mbmi: *mut MB_MODE_INFO,
) -> core::ffi::c_int {
    let mut sign: core::ffi::c_int = 0;
    let mut abs_0: core::ffi::c_int = 0;
    let mut reduced_delta_qindex: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bsize: BLOCK_SIZE = (*mbmi).bsize;
    let b_col: core::ffi::c_int = (*xd).mi_col
        & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let b_row: core::ffi::c_int = (*xd).mi_row
        & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let read_delta_q_flag: core::ffi::c_int = (b_col == 0 as core::ffi::c_int
        && b_row == 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if (bsize as core::ffi::c_int != (*(*cm).seq_params).sb_size as core::ffi::c_int
        || (*mbmi).skip_txfm as core::ffi::c_int == 0 as core::ffi::c_int)
        && read_delta_q_flag != 0
    {
        abs_0 = aom_read_symbol_(
            r,
            ((*ec_ctx).delta_q_cdf).as_mut_ptr(),
            3 as core::ffi::c_int + 1 as core::ffi::c_int,
        );
        let smallval: core::ffi::c_int = (abs_0 < DELTA_Q_SMALL) as core::ffi::c_int;
        if smallval == 0 {
            let rem_bits: core::ffi::c_int = aom_read_literal_(r, 3 as core::ffi::c_int)
                as core::ffi::c_int + 1 as core::ffi::c_int;
            let thr: core::ffi::c_int = ((1 as core::ffi::c_int) << rem_bits)
                + 1 as core::ffi::c_int;
            abs_0 = aom_read_literal_(r, rem_bits) + thr;
        }
        if abs_0 != 0 {
            sign = aom_read_bit_(r);
        } else {
            sign = 1 as core::ffi::c_int;
        }
        reduced_delta_qindex = if sign != 0 { -abs_0 } else { abs_0 };
    }
    return reduced_delta_qindex;
}
unsafe extern "C-unwind" fn read_delta_lflevel(
    cm: *const AV1_COMMON,
    mut r: *mut aom_reader,
    cdf: *mut aom_cdf_prob,
    mbmi: *const MB_MODE_INFO,
    mut mi_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut reduced_delta_lflevel: core::ffi::c_int = 0 as core::ffi::c_int;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let b_col: core::ffi::c_int = mi_col
        & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let b_row: core::ffi::c_int = mi_row
        & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let read_delta_lf_flag: core::ffi::c_int = (b_col == 0 as core::ffi::c_int
        && b_row == 0 as core::ffi::c_int) as core::ffi::c_int;
    if (bsize as core::ffi::c_int != (*(*cm).seq_params).sb_size as core::ffi::c_int
        || (*mbmi).skip_txfm as core::ffi::c_int == 0 as core::ffi::c_int)
        && read_delta_lf_flag != 0
    {
        let mut abs_0: core::ffi::c_int = aom_read_symbol_(
            r,
            cdf,
            3 as core::ffi::c_int + 1 as core::ffi::c_int,
        );
        let smallval: core::ffi::c_int = (abs_0 < DELTA_LF_SMALL) as core::ffi::c_int;
        if smallval == 0 {
            let rem_bits: core::ffi::c_int = aom_read_literal_(r, 3 as core::ffi::c_int)
                as core::ffi::c_int + 1 as core::ffi::c_int;
            let thr: core::ffi::c_int = ((1 as core::ffi::c_int) << rem_bits)
                + 1 as core::ffi::c_int;
            abs_0 = aom_read_literal_(r, rem_bits) + thr;
        }
        let sign: core::ffi::c_int = if abs_0 != 0 {
            aom_read_bit_(r) as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        reduced_delta_lflevel = if sign != 0 { -abs_0 } else { abs_0 };
    }
    return reduced_delta_lflevel;
}
unsafe extern "C-unwind" fn read_intra_mode_uv(
    mut ec_ctx: *mut FRAME_CONTEXT,
    mut r: *mut aom_reader,
    mut cfl_allowed: CFL_ALLOWED_TYPE,
    mut y_mode: PREDICTION_MODE,
) -> UV_PREDICTION_MODE {
    let uv_mode: UV_PREDICTION_MODE = aom_read_symbol_(
        r,
        ((*ec_ctx).uv_mode_cdf[cfl_allowed as usize][y_mode as usize]).as_mut_ptr(),
        UV_INTRA_MODES as core::ffi::c_int - (cfl_allowed == 0) as core::ffi::c_int,
    ) as UV_PREDICTION_MODE;
    return uv_mode;
}
unsafe extern "C-unwind" fn read_cfl_alphas(
    ec_ctx: *mut FRAME_CONTEXT,
    mut r: *mut aom_reader,
    mut signs_out: *mut int8_t,
) -> uint8_t {
    let joint_sign: int8_t = aom_read_symbol_(
        r,
        ((*ec_ctx).cfl_sign_cdf).as_mut_ptr(),
        CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
            - 1 as core::ffi::c_int,
    ) as int8_t;
    let mut idx: uint8_t = 0 as uint8_t;
    if (joint_sign as core::ffi::c_int + 1 as core::ffi::c_int) * 11 as core::ffi::c_int
        >> 5 as core::ffi::c_int != CFL_SIGN_ZERO as core::ffi::c_int
    {
        let mut cdf_u: *mut aom_cdf_prob = ((*ec_ctx)
            .cfl_alpha_cdf[(joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int) as usize])
            .as_mut_ptr();
        idx = ((aom_read_symbol_(
            r,
            cdf_u,
            (1 as core::ffi::c_int) << 4 as core::ffi::c_int,
        ) as uint8_t as core::ffi::c_int) << CFL_ALPHABET_SIZE_LOG2) as uint8_t;
    }
    if joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
        - CFL_SIGNS as core::ffi::c_int
            * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
        != CFL_SIGN_ZERO as core::ffi::c_int
    {
        let mut cdf_v: *mut aom_cdf_prob = ((*ec_ctx)
            .cfl_alpha_cdf[((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int
                * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 11 as core::ffi::c_int >> 5 as core::ffi::c_int))
            * CFL_SIGNS as core::ffi::c_int
            + ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
            - CFL_SIGNS as core::ffi::c_int) as usize])
            .as_mut_ptr();
        idx = (idx as core::ffi::c_int
            + aom_read_symbol_(
                r,
                cdf_v,
                (1 as core::ffi::c_int) << 4 as core::ffi::c_int,
            ) as uint8_t as core::ffi::c_int) as uint8_t;
    }
    *signs_out = joint_sign;
    return idx;
}
unsafe extern "C-unwind" fn read_interintra_mode(
    mut xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
    mut size_group: core::ffi::c_int,
) -> INTERINTRA_MODE {
    let ii_mode: INTERINTRA_MODE = aom_read_symbol_(
        r,
        ((*(*xd).tile_ctx).interintra_mode_cdf[size_group as usize]).as_mut_ptr(),
        INTERINTRA_MODES as core::ffi::c_int,
    ) as INTERINTRA_MODE;
    return ii_mode;
}
unsafe extern "C-unwind" fn read_inter_mode(
    mut ec_ctx: *mut FRAME_CONTEXT,
    mut r: *mut aom_reader,
    mut ctx: int16_t,
) -> PREDICTION_MODE {
    let mut mode_ctx: int16_t = (ctx as core::ffi::c_int & NEWMV_CTX_MASK) as int16_t;
    let mut is_newmv: core::ffi::c_int = 0;
    let mut is_zeromv: core::ffi::c_int = 0;
    let mut is_refmv: core::ffi::c_int = 0;
    is_newmv = (aom_read_symbol_(
        r,
        ((*ec_ctx).newmv_cdf[mode_ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) == 0 as core::ffi::c_int) as core::ffi::c_int;
    if is_newmv != 0 {
        return NEWMV as core::ffi::c_int as PREDICTION_MODE;
    }
    mode_ctx = (ctx as core::ffi::c_int >> GLOBALMV_OFFSET & GLOBALMV_CTX_MASK)
        as int16_t;
    is_zeromv = (aom_read_symbol_(
        r,
        ((*ec_ctx).zeromv_cdf[mode_ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) == 0 as core::ffi::c_int) as core::ffi::c_int;
    if is_zeromv != 0 {
        return GLOBALMV as core::ffi::c_int as PREDICTION_MODE;
    }
    mode_ctx = (ctx as core::ffi::c_int >> REFMV_OFFSET & REFMV_CTX_MASK) as int16_t;
    is_refmv = (aom_read_symbol_(
        r,
        ((*ec_ctx).refmv_cdf[mode_ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) == 0 as core::ffi::c_int) as core::ffi::c_int;
    if is_refmv != 0 {
        return NEARESTMV as core::ffi::c_int as PREDICTION_MODE
    } else {
        return NEARMV as core::ffi::c_int as PREDICTION_MODE
    };
}
unsafe extern "C-unwind" fn read_drl_idx(
    mut ec_ctx: *mut FRAME_CONTEXT,
    mut dcb: *mut DecoderCodingBlock,
    mut mbmi: *mut MB_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mut ref_frame_type: uint8_t = av1_ref_frame_type(
        ((*mbmi).ref_frame).as_mut_ptr(),
    ) as uint8_t;
    (*mbmi).set_ref_mv_idx(0 as uint8_t as uint8_t);
    if (*mbmi).mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        || (*mbmi).mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
    {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < 2 as core::ffi::c_int {
            if (*dcb).ref_mv_count[ref_frame_type as usize] as core::ffi::c_int
                > idx + 1 as core::ffi::c_int
            {
                let mut drl_ctx: uint8_t = av1_drl_ctx(
                    ((*xd).weight[ref_frame_type as usize]).as_mut_ptr(),
                    idx,
                );
                let mut drl_idx: core::ffi::c_int = aom_read_symbol_(
                    r,
                    ((*ec_ctx).drl_cdf[drl_ctx as usize]).as_mut_ptr(),
                    2 as core::ffi::c_int,
                );
                (*mbmi).set_ref_mv_idx((idx + drl_idx) as uint8_t as uint8_t);
                if drl_idx == 0 {
                    return;
                }
            }
            idx += 1;
        }
    }
    if have_nearmv_in_inter_mode((*mbmi).mode) != 0 {
        let mut idx_0: core::ffi::c_int = 1 as core::ffi::c_int;
        while idx_0 < 3 as core::ffi::c_int {
            if (*dcb).ref_mv_count[ref_frame_type as usize] as core::ffi::c_int
                > idx_0 + 1 as core::ffi::c_int
            {
                let mut drl_ctx_0: uint8_t = av1_drl_ctx(
                    ((*xd).weight[ref_frame_type as usize]).as_mut_ptr(),
                    idx_0,
                );
                let mut drl_idx_0: core::ffi::c_int = aom_read_symbol_(
                    r,
                    ((*ec_ctx).drl_cdf[drl_ctx_0 as usize]).as_mut_ptr(),
                    2 as core::ffi::c_int,
                );
                (*mbmi)
                    .set_ref_mv_idx(
                        (idx_0 + drl_idx_0 - 1 as core::ffi::c_int) as uint8_t as uint8_t,
                    );
                if drl_idx_0 == 0 {
                    return;
                }
            }
            idx_0 += 1;
        }
    }
}
unsafe extern "C-unwind" fn read_motion_mode(
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut mbmi: *mut MB_MODE_INFO,
    mut r: *mut aom_reader,
) -> MOTION_MODE {
    if (*cm).features.switchable_motion_mode as core::ffi::c_int == 0 as core::ffi::c_int
    {
        return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    }
    if (*mbmi).skip_mode() != 0 {
        return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    }
    let last_motion_mode_allowed: MOTION_MODE = motion_mode_allowed(
        (*xd).global_motion,
        xd,
        mbmi,
        (*cm).features.allow_warped_motion as core::ffi::c_int,
    ) as MOTION_MODE;
    let mut motion_mode: core::ffi::c_int = 0;
    if last_motion_mode_allowed as core::ffi::c_int
        == SIMPLE_TRANSLATION as core::ffi::c_int
    {
        return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    }
    if last_motion_mode_allowed as core::ffi::c_int == OBMC_CAUSAL as core::ffi::c_int {
        motion_mode = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).obmc_cdf[(*mbmi).bsize as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        return (SIMPLE_TRANSLATION as core::ffi::c_int + motion_mode) as MOTION_MODE;
    } else {
        motion_mode = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).motion_mode_cdf[(*mbmi).bsize as usize]).as_mut_ptr(),
            MOTION_MODES as core::ffi::c_int,
        );
        return (SIMPLE_TRANSLATION as core::ffi::c_int + motion_mode) as MOTION_MODE;
    };
}
unsafe extern "C-unwind" fn read_inter_compound_mode(
    mut xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
    mut ctx: int16_t,
) -> PREDICTION_MODE {
    let mode: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*(*xd).tile_ctx).inter_compound_mode_cdf[ctx as usize]).as_mut_ptr(),
        1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
            - NEAREST_NEARESTMV as core::ffi::c_int,
    ) as core::ffi::c_int;
    return (NEAREST_NEARESTMV as core::ffi::c_int + mode) as PREDICTION_MODE;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_neg_deinterleave(
    mut diff: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut max: core::ffi::c_int,
) -> core::ffi::c_int {
    if ref_0 == 0 {
        return diff;
    }
    if ref_0 >= max - 1 as core::ffi::c_int {
        return max - diff - 1 as core::ffi::c_int;
    }
    if 2 as core::ffi::c_int * ref_0 < max {
        if diff <= 2 as core::ffi::c_int * ref_0 {
            if diff & 1 as core::ffi::c_int != 0 {
                return ref_0 + (diff + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            } else {
                return ref_0 - (diff >> 1 as core::ffi::c_int)
            }
        }
        return diff;
    } else {
        if diff <= 2 as core::ffi::c_int * (max - ref_0 - 1 as core::ffi::c_int) {
            if diff & 1 as core::ffi::c_int != 0 {
                return ref_0 + (diff + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            } else {
                return ref_0 - (diff >> 1 as core::ffi::c_int)
            }
        }
        return max - (diff + 1 as core::ffi::c_int);
    };
}
unsafe extern "C-unwind" fn read_segment_id(
    cm: *mut AV1_COMMON,
    xd: *const MACROBLOCKD,
    mut r: *mut aom_reader,
    mut skip: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cdf_num: core::ffi::c_int = 0;
    let pred: uint8_t = av1_get_spatial_seg_pred(
        cm,
        xd,
        &raw mut cdf_num,
        0 as core::ffi::c_int,
    ) as uint8_t;
    if skip != 0 {
        return pred as core::ffi::c_int;
    }
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let seg: *mut segmentation = &raw mut (*cm).seg;
    let segp: *mut segmentation_probs = &raw mut (*ec_ctx).seg;
    let mut pred_cdf: *mut aom_cdf_prob = ((*segp)
        .spatial_pred_seg_cdf[cdf_num as usize])
        .as_mut_ptr();
    let coded_id: core::ffi::c_int = aom_read_symbol_(r, pred_cdf, 8 as core::ffi::c_int)
        as core::ffi::c_int;
    let segment_id: core::ffi::c_int = av1_neg_deinterleave(
        coded_id,
        pred as core::ffi::c_int,
        (*seg).last_active_segid + 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if segment_id < 0 as core::ffi::c_int || segment_id > (*seg).last_active_segid {
        aom_internal_error(
            (*xd).error_info as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Corrupted segment_ids\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    return segment_id;
}
unsafe extern "C-unwind" fn dec_get_segment_id(
    mut cm: *const AV1_COMMON,
    mut segment_ids: *const uint8_t,
    mut mi_offset: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut segment_id: core::ffi::c_int = INT_MAX;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_mis {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < x_mis {
            segment_id = if segment_id
                < *segment_ids
                    .offset((mi_offset + y * (*cm).mi_params.mi_cols + x) as isize)
                    as core::ffi::c_int
            {
                segment_id
            } else {
                *segment_ids
                    .offset((mi_offset + y * (*cm).mi_params.mi_cols + x) as isize)
                    as core::ffi::c_int
            };
            x += 1;
        }
        y += 1;
    }
    return segment_id;
}
unsafe extern "C-unwind" fn read_intra_segment_id(
    cm: *mut AV1_COMMON,
    xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut r: *mut aom_reader,
    mut skip: core::ffi::c_int,
) -> core::ffi::c_int {
    let seg: *mut segmentation = &raw mut (*cm).seg;
    if (*seg).enabled == 0 {
        return 0 as core::ffi::c_int;
    }
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mi_stride: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let mi_offset: core::ffi::c_int = mi_row * mi_stride + mi_col;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let x_mis: core::ffi::c_int = if (*mi_params).mi_cols - mi_col < bw {
        (*mi_params).mi_cols - mi_col
    } else {
        bw
    };
    let y_mis: core::ffi::c_int = if (*mi_params).mi_rows - mi_row < bh {
        (*mi_params).mi_rows - mi_row
    } else {
        bh
    };
    let segment_id: core::ffi::c_int = read_segment_id(cm, xd, r, skip)
        as core::ffi::c_int;
    set_segment_id(
        (*(*cm).cur_frame).seg_map,
        mi_offset,
        x_mis,
        y_mis,
        mi_stride,
        segment_id as uint8_t,
    );
    return segment_id;
}
unsafe extern "C-unwind" fn copy_segment_id(
    mi_params: *const CommonModeInfoParams,
    mut last_segment_ids: *const uint8_t,
    mut current_segment_ids: *mut uint8_t,
    mut mi_offset: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) {
    let stride: core::ffi::c_int = (*mi_params).mi_cols;
    if !last_segment_ids.is_null() {
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < y_mis {
            memcpy(
                &raw mut *current_segment_ids.offset((mi_offset + y * stride) as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                &*last_segment_ids.offset((mi_offset + y * stride) as isize)
                    as *const uint8_t as *const core::ffi::c_void,
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(x_mis as size_t),
            );
            y += 1;
        }
    } else {
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < y_mis {
            memset(
                &raw mut *current_segment_ids.offset((mi_offset + y_0 * stride) as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(x_mis as size_t),
            );
            y_0 += 1;
        }
    };
}
unsafe extern "C-unwind" fn get_predicted_segment_id(
    cm: *mut AV1_COMMON,
    mut mi_offset: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) -> core::ffi::c_int {
    return if !((*cm).last_frame_seg_map).is_null() {
        dec_get_segment_id(cm, (*cm).last_frame_seg_map, mi_offset, x_mis, y_mis)
    } else {
        0 as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn read_inter_segment_id(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut preskip: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    let seg: *mut segmentation = &raw mut (*cm).seg;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mi_offset: core::ffi::c_int = mi_row * (*mi_params).mi_cols + mi_col;
    let bw: core::ffi::c_int = mi_size_wide[(*mbmi).bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[(*mbmi).bsize as usize] as core::ffi::c_int;
    let x_mis: core::ffi::c_int = if (*mi_params).mi_cols - mi_col < bw {
        (*mi_params).mi_cols - mi_col
    } else {
        bw
    };
    let y_mis: core::ffi::c_int = if (*mi_params).mi_rows - mi_row < bh {
        (*mi_params).mi_rows - mi_row
    } else {
        bh
    };
    if (*seg).enabled == 0 {
        return 0 as core::ffi::c_int;
    }
    if (*seg).update_map == 0 {
        copy_segment_id(
            mi_params,
            (*cm).last_frame_seg_map,
            (*(*cm).cur_frame).seg_map,
            mi_offset,
            x_mis,
            y_mis,
        );
        return get_predicted_segment_id(cm, mi_offset, x_mis, y_mis);
    }
    let mut segment_id: uint8_t = 0;
    let mi_stride: core::ffi::c_int = (*cm).mi_params.mi_cols;
    if preskip != 0 {
        if (*seg).segid_preskip == 0 {
            return 0 as core::ffi::c_int;
        }
    } else if (*mbmi).skip_txfm != 0 {
        if (*seg).temporal_update != 0 {
            (*mbmi).set_seg_id_predicted(0 as uint8_t as uint8_t);
        }
        segment_id = read_segment_id(cm, xd, r, 1 as core::ffi::c_int) as uint8_t;
        set_segment_id(
            (*(*cm).cur_frame).seg_map,
            mi_offset,
            x_mis,
            y_mis,
            mi_stride,
            segment_id,
        );
        return segment_id as core::ffi::c_int;
    }
    if (*seg).temporal_update != 0 {
        let ctx: uint8_t = av1_get_pred_context_seg_id(xd) as uint8_t;
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        let segp: *mut segmentation_probs = &raw mut (*ec_ctx).seg;
        let mut pred_cdf: *mut aom_cdf_prob = ((*segp).pred_cdf[ctx as usize])
            .as_mut_ptr();
        (*mbmi)
            .set_seg_id_predicted(
                aom_read_symbol_(r, pred_cdf, 2 as core::ffi::c_int) as uint8_t
                    as uint8_t,
            );
        if (*mbmi).seg_id_predicted() != 0 {
            segment_id = get_predicted_segment_id(cm, mi_offset, x_mis, y_mis)
                as uint8_t;
        } else {
            segment_id = read_segment_id(cm, xd, r, 0 as core::ffi::c_int) as uint8_t;
        }
    } else {
        segment_id = read_segment_id(cm, xd, r, 0 as core::ffi::c_int) as uint8_t;
    }
    set_segment_id(
        (*(*cm).cur_frame).seg_map,
        mi_offset,
        x_mis,
        y_mis,
        mi_stride,
        segment_id,
    );
    return segment_id as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_skip_mode(
    mut cm: *mut AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut segment_id: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    if (*cm).current_frame.skip_mode_info.skip_mode_flag == 0 {
        return 0 as core::ffi::c_int;
    }
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        return 0 as core::ffi::c_int;
    }
    if is_comp_ref_allowed((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize)
        == 0
    {
        return 0 as core::ffi::c_int;
    }
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        || segfeature_active(
            &raw mut (*cm).seg,
            segment_id as uint8_t,
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        return 0 as core::ffi::c_int;
    }
    let ctx: core::ffi::c_int = av1_get_skip_mode_context(xd) as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let skip_mode: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*ec_ctx).skip_mode_cdfs[ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) as core::ffi::c_int;
    return skip_mode;
}
unsafe extern "C-unwind" fn read_skip_txfm(
    mut cm: *mut AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut segment_id: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        return 1 as core::ffi::c_int
    } else {
        let ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        let skip_txfm: core::ffi::c_int = aom_read_symbol_(
            r,
            ((*ec_ctx).skip_txfm_cdfs[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as core::ffi::c_int;
        return skip_txfm;
    };
}
unsafe extern "C-unwind" fn merge_colors(
    mut colors: *mut uint16_t,
    mut cached_colors: *mut uint16_t,
    mut n_colors: core::ffi::c_int,
    mut n_cached_colors: core::ffi::c_int,
) {
    if n_cached_colors == 0 as core::ffi::c_int {
        return;
    }
    let mut cache_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut trans_idx: core::ffi::c_int = n_cached_colors;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_colors {
        if cache_idx < n_cached_colors
            && (trans_idx >= n_colors
                || *cached_colors.offset(cache_idx as isize) as core::ffi::c_int
                    <= *colors.offset(trans_idx as isize) as core::ffi::c_int)
        {
            let fresh9 = cache_idx;
            cache_idx = cache_idx + 1;
            *colors.offset(i as isize) = *cached_colors.offset(fresh9 as isize);
        } else {
            let fresh10 = trans_idx;
            trans_idx = trans_idx + 1;
            *colors.offset(i as isize) = *colors.offset(fresh10 as isize);
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn read_palette_colors_y(
    xd: *mut MACROBLOCKD,
    mut bit_depth: core::ffi::c_int,
    pmi: *mut PALETTE_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let mut color_cache: [uint16_t; 16] = [0; 16];
    let mut cached_colors: [uint16_t; 8] = [0; 8];
    let n_cache: core::ffi::c_int = av1_get_palette_cache(
        xd,
        0 as core::ffi::c_int,
        color_cache.as_mut_ptr(),
    ) as core::ffi::c_int;
    let n: core::ffi::c_int = (*pmi).palette_size[0 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_cache && idx < n {
        if aom_read_bit_(r) != 0 {
            let fresh11 = idx;
            idx = idx + 1;
            cached_colors[fresh11 as usize] = color_cache[i as usize];
        }
        i += 1;
    }
    if idx < n {
        let n_cached_colors: core::ffi::c_int = idx;
        let fresh12 = idx;
        idx = idx + 1;
        (*pmi).palette_colors[fresh12 as usize] = aom_read_literal_(r, bit_depth)
            as uint16_t;
        if idx < n {
            let min_bits: core::ffi::c_int = bit_depth - 3 as core::ffi::c_int;
            let mut bits: core::ffi::c_int = min_bits
                + aom_read_literal_(r, 2 as core::ffi::c_int);
            let mut range: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
                - (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                    as core::ffi::c_int - 1 as core::ffi::c_int;
            while idx < n {
                let delta: core::ffi::c_int = aom_read_literal_(r, bits)
                    as core::ffi::c_int + 1 as core::ffi::c_int;
                (*pmi).palette_colors[idx as usize] = clamp(
                    (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                        as core::ffi::c_int + delta,
                    0 as core::ffi::c_int,
                    ((1 as core::ffi::c_int) << bit_depth) - 1 as core::ffi::c_int,
                ) as uint16_t;
                range
                    -= (*pmi).palette_colors[idx as usize] as core::ffi::c_int
                        - (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                            as core::ffi::c_int;
                bits = if bits < aom_ceil_log2(range) {
                    bits
                } else {
                    aom_ceil_log2(range)
                };
                idx += 1;
            }
        }
        merge_colors(
            ((*pmi).palette_colors).as_mut_ptr(),
            cached_colors.as_mut_ptr(),
            n,
            n_cached_colors,
        );
    } else {
        memcpy(
            ((*pmi).palette_colors).as_mut_ptr() as *mut core::ffi::c_void,
            cached_colors.as_mut_ptr() as *const core::ffi::c_void,
            (n as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    };
}
unsafe extern "C-unwind" fn read_palette_colors_uv(
    xd: *mut MACROBLOCKD,
    mut bit_depth: core::ffi::c_int,
    pmi: *mut PALETTE_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let n: core::ffi::c_int = (*pmi).palette_size[1 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let mut color_cache: [uint16_t; 16] = [0; 16];
    let mut cached_colors: [uint16_t; 8] = [0; 8];
    let n_cache: core::ffi::c_int = av1_get_palette_cache(
        xd,
        1 as core::ffi::c_int,
        color_cache.as_mut_ptr(),
    ) as core::ffi::c_int;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_cache && idx < n {
        if aom_read_bit_(r) != 0 {
            let fresh7 = idx;
            idx = idx + 1;
            cached_colors[fresh7 as usize] = color_cache[i as usize];
        }
        i += 1;
    }
    if idx < n {
        let n_cached_colors: core::ffi::c_int = idx;
        idx += PALETTE_MAX_SIZE;
        let fresh8 = idx;
        idx = idx + 1;
        (*pmi).palette_colors[fresh8 as usize] = aom_read_literal_(r, bit_depth)
            as uint16_t;
        if idx < PALETTE_MAX_SIZE + n {
            let min_bits: core::ffi::c_int = bit_depth - 3 as core::ffi::c_int;
            let mut bits: core::ffi::c_int = min_bits
                + aom_read_literal_(r, 2 as core::ffi::c_int);
            let mut range: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
                - (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                    as core::ffi::c_int;
            while idx < PALETTE_MAX_SIZE + n {
                let delta: core::ffi::c_int = aom_read_literal_(r, bits)
                    as core::ffi::c_int;
                (*pmi).palette_colors[idx as usize] = clamp(
                    (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                        as core::ffi::c_int + delta,
                    0 as core::ffi::c_int,
                    ((1 as core::ffi::c_int) << bit_depth) - 1 as core::ffi::c_int,
                ) as uint16_t;
                range
                    -= (*pmi).palette_colors[idx as usize] as core::ffi::c_int
                        - (*pmi).palette_colors[(idx - 1 as core::ffi::c_int) as usize]
                            as core::ffi::c_int;
                bits = if bits < aom_ceil_log2(range) {
                    bits
                } else {
                    aom_ceil_log2(range)
                };
                idx += 1;
            }
        }
        merge_colors(
            ((*pmi).palette_colors).as_mut_ptr().offset(PALETTE_MAX_SIZE as isize),
            cached_colors.as_mut_ptr(),
            n,
            n_cached_colors,
        );
    } else {
        memcpy(
            ((*pmi).palette_colors).as_mut_ptr().offset(PALETTE_MAX_SIZE as isize)
                as *mut core::ffi::c_void,
            cached_colors.as_mut_ptr() as *const core::ffi::c_void,
            (n as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    }
    if aom_read_bit_(r) != 0 {
        let min_bits_v: core::ffi::c_int = bit_depth - 4 as core::ffi::c_int;
        let max_val: core::ffi::c_int = (1 as core::ffi::c_int) << bit_depth;
        let mut bits_0: core::ffi::c_int = min_bits_v
            + aom_read_literal_(r, 2 as core::ffi::c_int);
        (*pmi).palette_colors[(2 as core::ffi::c_int * PALETTE_MAX_SIZE) as usize] = aom_read_literal_(
            r,
            bit_depth,
        ) as uint16_t;
        let mut i_0: core::ffi::c_int = 1 as core::ffi::c_int;
        while i_0 < n {
            let mut delta_0: core::ffi::c_int = aom_read_literal_(r, bits_0);
            if delta_0 != 0 && aom_read_bit_(r) != 0 {
                delta_0 = -delta_0;
            }
            let mut val: core::ffi::c_int = (*pmi)
                .palette_colors[(2 as core::ffi::c_int * PALETTE_MAX_SIZE + i_0
                - 1 as core::ffi::c_int) as usize] as core::ffi::c_int + delta_0;
            if val < 0 as core::ffi::c_int {
                val += max_val;
            }
            if val >= max_val {
                val -= max_val;
            }
            (*pmi)
                .palette_colors[(2 as core::ffi::c_int * PALETTE_MAX_SIZE + i_0)
                as usize] = val as uint16_t;
            i_0 += 1;
        }
    } else {
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < n {
            (*pmi)
                .palette_colors[(2 as core::ffi::c_int * PALETTE_MAX_SIZE + i_1)
                as usize] = aom_read_literal_(r, bit_depth) as uint16_t;
            i_1 += 1;
        }
    };
}
unsafe extern "C-unwind" fn read_palette_mode_info(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let pmi: *mut PALETTE_MODE_INFO = &raw mut (*mbmi).palette_mode_info;
    let bsize_ctx: core::ffi::c_int = av1_get_palette_bsize_ctx(bsize)
        as core::ffi::c_int;
    if (*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        let palette_mode_ctx: core::ffi::c_int = av1_get_palette_mode_ctx(xd)
            as core::ffi::c_int;
        let modev: core::ffi::c_int = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx)
                .palette_y_mode_cdf[bsize_ctx as usize][palette_mode_ctx as usize])
                .as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if modev != 0 {
            (*pmi).palette_size[0 as core::ffi::c_int as usize] = (aom_read_symbol_(
                r,
                ((*(*xd).tile_ctx).palette_y_size_cdf[bsize_ctx as usize]).as_mut_ptr(),
                PALETTE_SIZES as core::ffi::c_int,
            ) + 2 as core::ffi::c_int) as uint8_t;
            read_palette_colors_y(
                xd,
                (*(*cm).seq_params).bit_depth as core::ffi::c_int,
                pmi,
                r,
            );
        }
    }
    if num_planes > 1 as core::ffi::c_int
        && (*mbmi).uv_mode as core::ffi::c_int == UV_DC_PRED as core::ffi::c_int
        && (*xd).is_chroma_ref as core::ffi::c_int != 0
    {
        let palette_uv_mode_ctx: core::ffi::c_int = ((*pmi)
            .palette_size[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int) as core::ffi::c_int;
        let modev_0: core::ffi::c_int = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).palette_uv_mode_cdf[palette_uv_mode_ctx as usize])
                .as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if modev_0 != 0 {
            (*pmi).palette_size[1 as core::ffi::c_int as usize] = (aom_read_symbol_(
                r,
                ((*(*xd).tile_ctx).palette_uv_size_cdf[bsize_ctx as usize]).as_mut_ptr(),
                PALETTE_SIZES as core::ffi::c_int,
            ) + 2 as core::ffi::c_int) as uint8_t;
            read_palette_colors_uv(
                xd,
                (*(*cm).seq_params).bit_depth as core::ffi::c_int,
                pmi,
                r,
            );
        }
    }
}
unsafe extern "C-unwind" fn read_angle_delta(
    mut r: *mut aom_reader,
    mut cdf: *mut aom_cdf_prob,
) -> core::ffi::c_int {
    let sym: core::ffi::c_int = aom_read_symbol_(
        r,
        cdf,
        2 as core::ffi::c_int * 3 as core::ffi::c_int + 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    return sym - MAX_ANGLE_DELTA;
}
unsafe extern "C-unwind" fn read_filter_intra_mode_info(
    cm: *const AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
) {
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut filter_intra_mode_info: *mut FILTER_INTRA_MODE_INFO = &raw mut (*mbmi)
        .filter_intra_mode_info;
    if av1_filter_intra_allowed(cm, mbmi) != 0 {
        (*filter_intra_mode_info).use_filter_intra = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).filter_intra_cdfs[(*mbmi).bsize as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as uint8_t;
        if (*filter_intra_mode_info).use_filter_intra != 0 {
            (*filter_intra_mode_info).filter_intra_mode = aom_read_symbol_(
                r,
                ((*(*xd).tile_ctx).filter_intra_mode_cdf).as_mut_ptr(),
                FILTER_INTRA_MODES as core::ffi::c_int,
            ) as FILTER_INTRA_MODE;
        }
    } else {
        (*filter_intra_mode_info).use_filter_intra = 0 as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_tx_type(
    cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut r: *mut aom_reader,
) {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut tx_type: *mut uint8_t = &raw mut *((*xd).tx_type_map)
        .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize) as *mut uint8_t;
    *tx_type = DCT_DCT as core::ffi::c_int as uint8_t;
    if (*mbmi).skip_txfm as core::ffi::c_int != 0
        || segfeature_active(
            &(*cm).seg,
            (*mbmi).segment_id(),
            SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        return;
    }
    let qindex: core::ffi::c_int = (*xd).qindex[(*mbmi).segment_id() as usize];
    if qindex == 0 as core::ffi::c_int {
        return;
    }
    let inter_block: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    if get_ext_tx_types(
        tx_size,
        inter_block,
        (*cm).features.reduced_tx_set_used as core::ffi::c_int,
    ) > 1 as core::ffi::c_int
    {
        let tx_set_type: TxSetType = av1_get_ext_tx_set_type(
            tx_size,
            inter_block,
            (*cm).features.reduced_tx_set_used as core::ffi::c_int,
        ) as TxSetType;
        let eset: core::ffi::c_int = get_ext_tx_set(
            tx_size,
            inter_block,
            (*cm).features.reduced_tx_set_used as core::ffi::c_int,
        ) as core::ffi::c_int;
        let square_tx_size: TX_SIZE = txsize_sqr_map[tx_size as usize];
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        if inter_block != 0 {
            *tx_type = av1_ext_tx_inv[tx_set_type
                as usize][aom_read_symbol_(
                r,
                ((*ec_ctx).inter_ext_tx_cdf[eset as usize][square_tx_size as usize])
                    .as_mut_ptr(),
                av1_num_ext_tx_set[tx_set_type as usize],
            ) as usize] as uint8_t;
        } else {
            let intra_mode: PREDICTION_MODE = (if (*mbmi)
                .filter_intra_mode_info
                .use_filter_intra as core::ffi::c_int != 0
            {
                fimode_to_intradir[(*mbmi).filter_intra_mode_info.filter_intra_mode
                    as usize] as core::ffi::c_int
            } else {
                (*mbmi).mode as core::ffi::c_int
            }) as PREDICTION_MODE;
            *tx_type = av1_ext_tx_inv[tx_set_type
                as usize][aom_read_symbol_(
                r,
                ((*ec_ctx)
                    .intra_ext_tx_cdf[eset
                    as usize][square_tx_size as usize][intra_mode as usize])
                    .as_mut_ptr(),
                av1_num_ext_tx_set[tx_set_type as usize],
            ) as usize] as uint8_t;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn assign_dv(
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut mv: *mut int_mv,
    mut ref_mv: *const int_mv,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    read_mv(
        r,
        &raw mut (*mv).as_mv,
        &(*ref_mv).as_mv,
        &raw mut (*ec_ctx).ndvc,
        MV_SUBPEL_NONE as core::ffi::c_int as MvSubpelPrecision,
    );
    (*mv).as_mv.col = (((*mv).as_mv.col as core::ffi::c_int >> 3 as core::ffi::c_int)
        * 8 as core::ffi::c_int) as int16_t;
    (*mv).as_mv.row = (((*mv).as_mv.row as core::ffi::c_int >> 3 as core::ffi::c_int)
        * 8 as core::ffi::c_int) as int16_t;
    let mut valid: core::ffi::c_int = (is_mv_valid(&raw mut (*mv).as_mv) != 0
        && av1_is_dv_valid(
            (*mv).as_mv,
            cm,
            xd,
            mi_row,
            mi_col,
            bsize,
            (*(*cm).seq_params).mib_size_log2,
        ) != 0) as core::ffi::c_int;
    return valid;
}
unsafe extern "C-unwind" fn read_intrabc_info(
    cm: *mut AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut r: *mut aom_reader,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    (*mbmi)
        .set_use_intrabc(
            aom_read_symbol_(
                r,
                ((*ec_ctx).intrabc_cdf).as_mut_ptr(),
                2 as core::ffi::c_int,
            ) as uint8_t as uint8_t,
        );
    if (*mbmi).use_intrabc() != 0 {
        let mut bsize: BLOCK_SIZE = (*mbmi).bsize;
        (*mbmi).mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
        (*mbmi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
        (*mbmi).interp_filters = av1_broadcast_interp_filter(BILINEAR);
        (*mbmi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
        let mut inter_mode_ctx: [int16_t; 29] = [0; 29];
        let mut ref_mvs: [[int_mv; 2]; 1] = [[int_mv { as_int: 0 }; 2]; 1];
        av1_find_mv_refs(
            cm,
            xd,
            mbmi,
            INTRA_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ((*dcb).ref_mv_count).as_mut_ptr(),
            ((*xd).ref_mv_stack).as_mut_ptr(),
            ((*xd).weight).as_mut_ptr(),
            ref_mvs.as_mut_ptr(),
            0 as *mut int_mv,
            inter_mode_ctx.as_mut_ptr(),
        );
        let mut nearestmv: int_mv = int_mv { as_int: 0 };
        let mut nearmv: int_mv = int_mv { as_int: 0 };
        av1_find_best_ref_mvs(
            0 as core::ffi::c_int,
            (ref_mvs[INTRA_FRAME as core::ffi::c_int as usize]).as_mut_ptr(),
            &raw mut nearestmv,
            &raw mut nearmv,
            0 as core::ffi::c_int,
        );
        let mut dv_ref: int_mv = if nearestmv.as_int == 0 as uint32_t {
            nearmv
        } else {
            nearestmv
        };
        if dv_ref.as_int == 0 as uint32_t {
            av1_find_ref_dv(
                &raw mut dv_ref,
                &raw mut (*xd).tile,
                (*(*cm).seq_params).mib_size,
                (*xd).mi_row,
            );
        }
        let mut valid_dv: core::ffi::c_int = (dv_ref.as_mv.col as core::ffi::c_int
            & 7 as core::ffi::c_int == 0 as core::ffi::c_int
            && dv_ref.as_mv.row as core::ffi::c_int & 7 as core::ffi::c_int
                == 0 as core::ffi::c_int) as core::ffi::c_int;
        dv_ref.as_mv.col = ((dv_ref.as_mv.col as core::ffi::c_int
            >> 3 as core::ffi::c_int) * 8 as core::ffi::c_int) as int16_t;
        dv_ref.as_mv.row = ((dv_ref.as_mv.row as core::ffi::c_int
            >> 3 as core::ffi::c_int) * 8 as core::ffi::c_int) as int16_t;
        valid_dv = (valid_dv != 0
            && assign_dv(
                cm,
                xd,
                &raw mut *((*mbmi).mv).as_mut_ptr().offset(0 as core::ffi::c_int as isize),
                &raw mut dv_ref,
                (*xd).mi_row,
                (*xd).mi_col,
                bsize,
                r,
            ) != 0) as core::ffi::c_int;
        if valid_dv == 0 {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid intrabc dv\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C-unwind" fn read_delta_q_params(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
) {
    let delta_q_info: *mut DeltaQInfo = &raw mut (*cm).delta_q_info;
    if (*delta_q_info).delta_q_present_flag != 0 {
        let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
        (*xd).current_base_qindex
            += read_delta_qindex(cm, xd, r, mbmi) * (*delta_q_info).delta_q_res;
        (*xd).current_base_qindex = clamp(
            (*xd).current_base_qindex,
            1 as core::ffi::c_int,
            MAXQ,
        );
        let ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        if (*delta_q_info).delta_lf_present_flag != 0 {
            let mi_row: core::ffi::c_int = (*xd).mi_row;
            let mi_col: core::ffi::c_int = (*xd).mi_col;
            if (*delta_q_info).delta_lf_multi != 0 {
                let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm)
                    > 1 as core::ffi::c_int
                {
                    FRAME_LF_COUNT
                } else {
                    FRAME_LF_COUNT - 2 as core::ffi::c_int
                };
                let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
                while lf_id < frame_lf_count {
                    let tmp_lvl: core::ffi::c_int = (*xd).delta_lf[lf_id as usize]
                        as core::ffi::c_int
                        + read_delta_lflevel(
                            cm,
                            r,
                            ((*ec_ctx).delta_lf_multi_cdf[lf_id as usize]).as_mut_ptr(),
                            mbmi,
                            mi_col,
                            mi_row,
                        ) as core::ffi::c_int * (*delta_q_info).delta_lf_res;
                    (*xd).delta_lf[lf_id as usize] = clamp(
                        tmp_lvl,
                        -MAX_LOOP_FILTER,
                        MAX_LOOP_FILTER,
                    ) as int8_t;
                    (*mbmi).delta_lf[lf_id as usize] = (*xd).delta_lf[lf_id as usize];
                    lf_id += 1;
                }
            } else {
                let tmp_lvl_0: core::ffi::c_int = (*xd).delta_lf_from_base
                    as core::ffi::c_int
                    + read_delta_lflevel(
                        cm,
                        r,
                        ((*ec_ctx).delta_lf_cdf).as_mut_ptr(),
                        mbmi,
                        mi_col,
                        mi_row,
                    ) as core::ffi::c_int * (*delta_q_info).delta_lf_res;
                (*xd).delta_lf_from_base = clamp(
                    tmp_lvl_0,
                    -MAX_LOOP_FILTER,
                    MAX_LOOP_FILTER,
                ) as int8_t;
                (*mbmi).delta_lf_from_base = (*xd).delta_lf_from_base;
            }
        }
    }
}
unsafe extern "C-unwind" fn read_intra_frame_mode_info(
    cm: *mut AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut r: *mut aom_reader,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let mut left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let seg: *mut segmentation = &raw mut (*cm).seg;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if (*seg).segid_preskip != 0 {
        (*mbmi)
            .set_segment_id(
                read_intra_segment_id(cm, xd, bsize, r, 0 as core::ffi::c_int) as uint8_t
                    as uint8_t,
            );
    }
    (*mbmi).skip_txfm = read_skip_txfm(
        cm,
        xd,
        (*mbmi).segment_id() as core::ffi::c_int,
        r,
    ) as uint8_t;
    if (*seg).segid_preskip == 0 {
        (*mbmi)
            .set_segment_id(
                read_intra_segment_id(
                    cm,
                    xd,
                    bsize,
                    r,
                    (*mbmi).skip_txfm as core::ffi::c_int,
                ) as uint8_t as uint8_t,
            );
    }
    read_cdef(cm, r, xd);
    read_delta_q_params(cm, xd, r);
    (*mbmi).current_qindex = (*xd).current_base_qindex;
    (*mbmi).ref_frame[0 as core::ffi::c_int as usize] = INTRA_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    (*mbmi).ref_frame[1 as core::ffi::c_int as usize] = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize] = 0
        as uint8_t;
    (*mbmi).palette_mode_info.palette_size[1 as core::ffi::c_int as usize] = 0
        as uint8_t;
    (*mbmi).filter_intra_mode_info.use_filter_intra = 0 as uint8_t;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*xd).tile.tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    if av1_allow_intrabc(cm) != 0 {
        read_intrabc_info(cm, dcb, r);
        if is_intrabc_block(mbmi) != 0 {
            return;
        }
    }
    (*mbmi).mode = read_intra_mode(r, get_y_mode_cdf(ec_ctx, above_mi, left_mi));
    let use_angle_delta: core::ffi::c_int = av1_use_angle_delta(bsize)
        as core::ffi::c_int;
    (*mbmi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize] = (if use_angle_delta
        != 0 && av1_is_directional_mode((*mbmi).mode) != 0
    {
        read_angle_delta(
            r,
            ((*ec_ctx)
                .angle_delta_cdf[((*mbmi).mode as core::ffi::c_int
                - V_PRED as core::ffi::c_int) as usize])
                .as_mut_ptr(),
        )
    } else {
        0 as core::ffi::c_int
    }) as int8_t;
    if (*(*cm).seq_params).monochrome == 0
        && (*xd).is_chroma_ref as core::ffi::c_int != 0
    {
        (*mbmi).uv_mode = read_intra_mode_uv(
            ec_ctx,
            r,
            is_cfl_allowed(xd),
            (*mbmi).mode,
        );
        if (*mbmi).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int {
            (*mbmi).cfl_alpha_idx = read_cfl_alphas(
                ec_ctx,
                r,
                &raw mut (*mbmi).cfl_alpha_signs,
            );
        }
        let intra_mode: PREDICTION_MODE = get_uv_mode((*mbmi).uv_mode)
            as PREDICTION_MODE;
        (*mbmi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize] = (if use_angle_delta
            != 0 && av1_is_directional_mode(intra_mode) != 0
        {
            read_angle_delta(
                r,
                ((*ec_ctx)
                    .angle_delta_cdf[(intra_mode as core::ffi::c_int
                    - V_PRED as core::ffi::c_int) as usize])
                    .as_mut_ptr(),
            )
        } else {
            0 as core::ffi::c_int
        }) as int8_t;
    } else {
        (*mbmi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
    }
    (*xd).cfl.store_y = store_cfl_required(cm, xd) as core::ffi::c_int;
    if av1_allow_palette(
        (*cm).features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) != 0
    {
        read_palette_mode_info(cm, xd, r);
    }
    read_filter_intra_mode_info(cm, xd, r);
}
unsafe extern "C-unwind" fn read_mv_component(
    mut r: *mut aom_reader,
    mut mvcomp: *mut nmv_component,
    mut use_subpel: core::ffi::c_int,
    mut usehp: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut mag: core::ffi::c_int = 0;
    let mut d: core::ffi::c_int = 0;
    let mut fr: core::ffi::c_int = 0;
    let mut hp: core::ffi::c_int = 0;
    let sign: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*mvcomp).sign_cdf).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mv_class: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*mvcomp).classes_cdf).as_mut_ptr(),
        11 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let class0: core::ffi::c_int = (mv_class == MV_CLASS_0 as core::ffi::c_int)
        as core::ffi::c_int;
    if class0 != 0 {
        d = aom_read_symbol_(
            r,
            ((*mvcomp).class0_cdf).as_mut_ptr(),
            (1 as core::ffi::c_int) << 1 as core::ffi::c_int,
        );
        mag = 0 as core::ffi::c_int;
    } else {
        let n: core::ffi::c_int = mv_class + CLASS0_BITS - 1 as core::ffi::c_int;
        d = 0 as core::ffi::c_int;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < n {
            d
                |= aom_read_symbol_(
                    r,
                    ((*mvcomp).bits_cdf[i as usize]).as_mut_ptr(),
                    2 as core::ffi::c_int,
                ) << i;
            i += 1;
        }
        mag = CLASS0_SIZE << mv_class + 2 as core::ffi::c_int;
    }
    if use_subpel != 0 {
        fr = aom_read_symbol_(
            r,
            if class0 != 0 {
                ((*mvcomp).class0_fp_cdf[d as usize]).as_mut_ptr()
            } else {
                ((*mvcomp).fp_cdf).as_mut_ptr()
            },
            4 as core::ffi::c_int,
        );
        hp = if usehp != 0 {
            aom_read_symbol_(
                r,
                if class0 != 0 {
                    ((*mvcomp).class0_hp_cdf).as_mut_ptr()
                } else {
                    ((*mvcomp).hp_cdf).as_mut_ptr()
                },
                2 as core::ffi::c_int,
            )
        } else {
            1 as core::ffi::c_int
        };
    } else {
        fr = 3 as core::ffi::c_int;
        hp = 1 as core::ffi::c_int;
    }
    mag
        += (d << 3 as core::ffi::c_int | fr << 1 as core::ffi::c_int | hp)
            + 1 as core::ffi::c_int;
    return if sign != 0 { -mag } else { mag };
}
#[inline]
unsafe extern "C-unwind" fn read_mv(
    mut r: *mut aom_reader,
    mut mv: *mut MV,
    mut ref_0: *const MV,
    mut ctx: *mut nmv_context,
    mut precision: MvSubpelPrecision,
) {
    let mut diff: MV = kZeroMv;
    let joint_type: MV_JOINT_TYPE = aom_read_symbol_(
        r,
        ((*ctx).joints_cdf).as_mut_ptr(),
        4 as core::ffi::c_int,
    ) as MV_JOINT_TYPE;
    if mv_joint_vertical(joint_type) != 0 {
        diff.row = read_mv_component(
            r,
            &raw mut *((*ctx).comps).as_mut_ptr().offset(0 as core::ffi::c_int as isize),
            (precision as core::ffi::c_int > MV_SUBPEL_NONE as core::ffi::c_int)
                as core::ffi::c_int,
            (precision as core::ffi::c_int > MV_SUBPEL_LOW_PRECISION as core::ffi::c_int)
                as core::ffi::c_int,
        ) as int16_t;
    }
    if mv_joint_horizontal(joint_type) != 0 {
        diff.col = read_mv_component(
            r,
            &raw mut *((*ctx).comps).as_mut_ptr().offset(1 as core::ffi::c_int as isize),
            (precision as core::ffi::c_int > MV_SUBPEL_NONE as core::ffi::c_int)
                as core::ffi::c_int,
            (precision as core::ffi::c_int > MV_SUBPEL_LOW_PRECISION as core::ffi::c_int)
                as core::ffi::c_int,
        ) as int16_t;
    }
    (*mv).row = ((*ref_0).row as core::ffi::c_int + diff.row as core::ffi::c_int)
        as int16_t;
    (*mv).col = ((*ref_0).col as core::ffi::c_int + diff.col as core::ffi::c_int)
        as int16_t;
}
unsafe extern "C-unwind" fn read_block_reference_mode(
    mut cm: *mut AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut r: *mut aom_reader,
) -> REFERENCE_MODE {
    if is_comp_ref_allowed((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize)
        == 0
    {
        return SINGLE_REFERENCE as core::ffi::c_int as REFERENCE_MODE;
    }
    if (*cm).current_frame.reference_mode as core::ffi::c_int
        == REFERENCE_MODE_SELECT as core::ffi::c_int
    {
        let ctx: core::ffi::c_int = av1_get_reference_mode_context(xd)
            as core::ffi::c_int;
        let mode: REFERENCE_MODE = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).comp_inter_cdf[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as REFERENCE_MODE;
        return mode;
    } else {
        return (*cm).current_frame.reference_mode
    };
}
unsafe extern "C-unwind" fn read_comp_reference_type(
    mut xd: *const MACROBLOCKD,
    mut r: *mut aom_reader,
) -> COMP_REFERENCE_TYPE {
    let ctx: core::ffi::c_int = av1_get_comp_reference_type_context(xd)
        as core::ffi::c_int;
    let comp_ref_type: COMP_REFERENCE_TYPE = aom_read_symbol_(
        r,
        ((*(*xd).tile_ctx).comp_ref_type_cdf[ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) as COMP_REFERENCE_TYPE;
    return comp_ref_type;
}
unsafe extern "C-unwind" fn set_ref_frames_for_skip_mode(
    cm: *mut AV1_COMMON,
    mut ref_frame: *mut MV_REFERENCE_FRAME,
) {
    *ref_frame.offset(0 as core::ffi::c_int as isize) = (LAST_FRAME as core::ffi::c_int
        + (*cm).current_frame.skip_mode_info.ref_frame_idx_0) as MV_REFERENCE_FRAME;
    *ref_frame.offset(1 as core::ffi::c_int as isize) = (LAST_FRAME as core::ffi::c_int
        + (*cm).current_frame.skip_mode_info.ref_frame_idx_1) as MV_REFERENCE_FRAME;
}
unsafe extern "C-unwind" fn read_ref_frames(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
    mut segment_id: core::ffi::c_int,
    mut ref_frame: *mut MV_REFERENCE_FRAME,
) {
    if (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).skip_mode() != 0 {
        set_ref_frames_for_skip_mode(cm, ref_frame);
        return;
    }
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        *ref_frame.offset(0 as core::ffi::c_int as isize) = get_segdata(
            &raw mut (*cm).seg,
            segment_id,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) as MV_REFERENCE_FRAME;
        *ref_frame.offset(1 as core::ffi::c_int as isize) = NONE_FRAME
            as core::ffi::c_int as MV_REFERENCE_FRAME;
    } else if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        || segfeature_active(
            &raw mut (*cm).seg,
            segment_id as uint8_t,
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        *ref_frame.offset(0 as core::ffi::c_int as isize) = LAST_FRAME
            as core::ffi::c_int as MV_REFERENCE_FRAME;
        *ref_frame.offset(1 as core::ffi::c_int as isize) = NONE_FRAME
            as core::ffi::c_int as MV_REFERENCE_FRAME;
    } else {
        let mode: REFERENCE_MODE = read_block_reference_mode(cm, xd, r)
            as REFERENCE_MODE;
        if mode as core::ffi::c_int == COMPOUND_REFERENCE as core::ffi::c_int {
            let comp_ref_type: COMP_REFERENCE_TYPE = read_comp_reference_type(xd, r)
                as COMP_REFERENCE_TYPE;
            if comp_ref_type as core::ffi::c_int
                == UNIDIR_COMP_REFERENCE as core::ffi::c_int
            {
                let bit: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_uni_comp_ref_p(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                if bit != 0 {
                    *ref_frame.offset(0 as core::ffi::c_int as isize) = BWDREF_FRAME
                        as core::ffi::c_int as MV_REFERENCE_FRAME;
                    *ref_frame.offset(1 as core::ffi::c_int as isize) = ALTREF_FRAME
                        as core::ffi::c_int as MV_REFERENCE_FRAME;
                } else {
                    let bit1: core::ffi::c_int = aom_read_symbol_(
                        r,
                        av1_get_pred_cdf_uni_comp_ref_p1(xd),
                        2 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    if bit1 != 0 {
                        let bit2: core::ffi::c_int = aom_read_symbol_(
                            r,
                            av1_get_pred_cdf_uni_comp_ref_p2(xd),
                            2 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        if bit2 != 0 {
                            *ref_frame.offset(0 as core::ffi::c_int as isize) = LAST_FRAME
                                as core::ffi::c_int as MV_REFERENCE_FRAME;
                            *ref_frame.offset(1 as core::ffi::c_int as isize) = GOLDEN_FRAME
                                as core::ffi::c_int as MV_REFERENCE_FRAME;
                        } else {
                            *ref_frame.offset(0 as core::ffi::c_int as isize) = LAST_FRAME
                                as core::ffi::c_int as MV_REFERENCE_FRAME;
                            *ref_frame.offset(1 as core::ffi::c_int as isize) = LAST3_FRAME
                                as core::ffi::c_int as MV_REFERENCE_FRAME;
                        }
                    } else {
                        *ref_frame.offset(0 as core::ffi::c_int as isize) = LAST_FRAME
                            as core::ffi::c_int as MV_REFERENCE_FRAME;
                        *ref_frame.offset(1 as core::ffi::c_int as isize) = LAST2_FRAME
                            as core::ffi::c_int as MV_REFERENCE_FRAME;
                    }
                }
                return;
            }
            let idx: core::ffi::c_int = 1 as core::ffi::c_int;
            let bit_0: core::ffi::c_int = aom_read_symbol_(
                r,
                av1_get_pred_cdf_comp_ref_p(xd),
                2 as core::ffi::c_int,
            ) as core::ffi::c_int;
            if bit_0 == 0 {
                let bit1_0: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_comp_ref_p1(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *ref_frame.offset((idx == 0) as core::ffi::c_int as isize) = (if bit1_0
                    != 0
                {
                    LAST2_FRAME as core::ffi::c_int
                } else {
                    LAST_FRAME as core::ffi::c_int
                }) as MV_REFERENCE_FRAME;
            } else {
                let bit2_0: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_comp_ref_p2(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *ref_frame.offset((idx == 0) as core::ffi::c_int as isize) = (if bit2_0
                    != 0
                {
                    GOLDEN_FRAME as core::ffi::c_int
                } else {
                    LAST3_FRAME as core::ffi::c_int
                }) as MV_REFERENCE_FRAME;
            }
            let bit_bwd: core::ffi::c_int = aom_read_symbol_(
                r,
                av1_get_pred_cdf_comp_bwdref_p(xd),
                2 as core::ffi::c_int,
            ) as core::ffi::c_int;
            if bit_bwd == 0 {
                let bit1_bwd: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_comp_bwdref_p1(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *ref_frame.offset(idx as isize) = (if bit1_bwd != 0 {
                    ALTREF2_FRAME as core::ffi::c_int
                } else {
                    BWDREF_FRAME as core::ffi::c_int
                }) as MV_REFERENCE_FRAME;
            } else {
                *ref_frame.offset(idx as isize) = ALTREF_FRAME as core::ffi::c_int
                    as MV_REFERENCE_FRAME;
            }
        } else if mode as core::ffi::c_int == SINGLE_REFERENCE as core::ffi::c_int {
            let bit0: core::ffi::c_int = aom_read_symbol_(
                r,
                av1_get_pred_cdf_single_ref_p1(xd),
                2 as core::ffi::c_int,
            ) as core::ffi::c_int;
            if bit0 != 0 {
                let bit1_1: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_single_ref_p2(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                if bit1_1 == 0 {
                    let bit5: core::ffi::c_int = aom_read_symbol_(
                        r,
                        av1_get_pred_cdf_single_ref_p6(xd),
                        2 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    *ref_frame.offset(0 as core::ffi::c_int as isize) = (if bit5 != 0 {
                        ALTREF2_FRAME as core::ffi::c_int
                    } else {
                        BWDREF_FRAME as core::ffi::c_int
                    }) as MV_REFERENCE_FRAME;
                } else {
                    *ref_frame.offset(0 as core::ffi::c_int as isize) = ALTREF_FRAME
                        as core::ffi::c_int as MV_REFERENCE_FRAME;
                }
            } else {
                let bit2_1: core::ffi::c_int = aom_read_symbol_(
                    r,
                    av1_get_pred_cdf_single_ref_p3(xd),
                    2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                if bit2_1 != 0 {
                    let bit4: core::ffi::c_int = aom_read_symbol_(
                        r,
                        av1_get_pred_cdf_single_ref_p5(xd),
                        2 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    *ref_frame.offset(0 as core::ffi::c_int as isize) = (if bit4 != 0 {
                        GOLDEN_FRAME as core::ffi::c_int
                    } else {
                        LAST3_FRAME as core::ffi::c_int
                    }) as MV_REFERENCE_FRAME;
                } else {
                    let bit3: core::ffi::c_int = aom_read_symbol_(
                        r,
                        av1_get_pred_cdf_single_ref_p4(xd),
                        2 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    *ref_frame.offset(0 as core::ffi::c_int as isize) = (if bit3 != 0 {
                        LAST2_FRAME as core::ffi::c_int
                    } else {
                        LAST_FRAME as core::ffi::c_int
                    }) as MV_REFERENCE_FRAME;
                }
            }
            *ref_frame.offset(1 as core::ffi::c_int as isize) = NONE_FRAME
                as core::ffi::c_int as MV_REFERENCE_FRAME;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn read_mb_interp_filter(
    xd: *const MACROBLOCKD,
    mut interp_filter: InterpFilter,
    mut enable_dual_filter: bool,
    mbmi: *mut MB_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if av1_is_interp_needed(xd) == 0 {
        set_default_interp_filters(mbmi, interp_filter);
        return;
    }
    if interp_filter as core::ffi::c_int != SWITCHABLE as core::ffi::c_int {
        (*mbmi).interp_filters = av1_broadcast_interp_filter(interp_filter);
    } else {
        let mut ref0_filter: [InterpFilter; 2] = [EIGHTTAP_REGULAR, EIGHTTAP_REGULAR];
        let mut dir: core::ffi::c_int = 0 as core::ffi::c_int;
        while dir < 2 as core::ffi::c_int {
            let ctx: core::ffi::c_int = av1_get_pred_context_switchable_interp(xd, dir)
                as core::ffi::c_int;
            ref0_filter[dir as usize] = aom_read_symbol_(
                r,
                ((*ec_ctx).switchable_interp_cdf[ctx as usize]).as_mut_ptr(),
                SWITCHABLE_FILTERS as core::ffi::c_int,
            ) as InterpFilter;
            if !enable_dual_filter {
                ref0_filter[1 as core::ffi::c_int as usize] = ref0_filter[0
                    as core::ffi::c_int as usize];
                break;
            } else {
                dir += 1;
            }
        }
        (*mbmi).interp_filters.as_filters.x_filter = ref0_filter[1 as core::ffi::c_int
            as usize] as uint16_t;
        (*mbmi).interp_filters.as_filters.y_filter = ref0_filter[0 as core::ffi::c_int
            as usize] as uint16_t;
    };
}
unsafe extern "C-unwind" fn read_intra_block_mode_info(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mbmi: *mut MB_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let use_angle_delta: core::ffi::c_int = av1_use_angle_delta(bsize)
        as core::ffi::c_int;
    (*mbmi).ref_frame[0 as core::ffi::c_int as usize] = INTRA_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    (*mbmi).ref_frame[1 as core::ffi::c_int as usize] = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    (*mbmi).mode = read_intra_mode(
        r,
        ((*ec_ctx).y_mode_cdf[size_group_lookup[bsize as usize] as usize]).as_mut_ptr(),
    );
    (*mbmi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize] = (if use_angle_delta
        != 0 && av1_is_directional_mode((*mbmi).mode) != 0
    {
        read_angle_delta(
            r,
            ((*ec_ctx)
                .angle_delta_cdf[((*mbmi).mode as core::ffi::c_int
                - V_PRED as core::ffi::c_int) as usize])
                .as_mut_ptr(),
        )
    } else {
        0 as core::ffi::c_int
    }) as int8_t;
    if (*(*cm).seq_params).monochrome == 0
        && (*xd).is_chroma_ref as core::ffi::c_int != 0
    {
        (*mbmi).uv_mode = read_intra_mode_uv(
            ec_ctx,
            r,
            is_cfl_allowed(xd),
            (*mbmi).mode,
        );
        if (*mbmi).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int {
            (*mbmi).cfl_alpha_idx = read_cfl_alphas(
                (*xd).tile_ctx,
                r,
                &raw mut (*mbmi).cfl_alpha_signs,
            );
        }
        let intra_mode: PREDICTION_MODE = get_uv_mode((*mbmi).uv_mode)
            as PREDICTION_MODE;
        (*mbmi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize] = (if use_angle_delta
            != 0 && av1_is_directional_mode(intra_mode) != 0
        {
            read_angle_delta(
                r,
                ((*ec_ctx)
                    .angle_delta_cdf[(intra_mode as core::ffi::c_int
                    - V_PRED as core::ffi::c_int) as usize])
                    .as_mut_ptr(),
            )
        } else {
            0 as core::ffi::c_int
        }) as int8_t;
    } else {
        (*mbmi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
    }
    (*xd).cfl.store_y = store_cfl_required(cm, xd) as core::ffi::c_int;
    (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize] = 0
        as uint8_t;
    (*mbmi).palette_mode_info.palette_size[1 as core::ffi::c_int as usize] = 0
        as uint8_t;
    if av1_allow_palette(
        (*cm).features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) != 0
    {
        read_palette_mode_info(cm, xd, r);
    }
    read_filter_intra_mode_info(cm, xd, r);
}
#[inline]
unsafe extern "C-unwind" fn is_mv_valid(mut mv: *const MV) -> core::ffi::c_int {
    return ((*mv).row as core::ffi::c_int > MV_LOW
        && ((*mv).row as core::ffi::c_int) < MV_UPP
        && (*mv).col as core::ffi::c_int > MV_LOW
        && ((*mv).col as core::ffi::c_int) < MV_UPP) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn assign_mv(
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut mode: PREDICTION_MODE,
    mut ref_frame: *mut MV_REFERENCE_FRAME,
    mut mv: *mut int_mv,
    mut ref_mv: *mut int_mv,
    mut nearest_mv: *mut int_mv,
    mut near_mv: *mut int_mv,
    mut is_compound: core::ffi::c_int,
    mut allow_hp: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut bsize: BLOCK_SIZE = (*mbmi).bsize;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    if (*features).cur_frame_force_integer_mv {
        allow_hp = MV_SUBPEL_NONE as core::ffi::c_int;
    }
    match mode as core::ffi::c_int {
        16 => {
            let nmvc: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            read_mv(
                r,
                &raw mut (*mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                &raw mut (*ref_mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                nmvc,
                allow_hp as MvSubpelPrecision,
            );
        }
        13 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*nearest_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
        }
        14 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*near_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
        }
        15 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (gm_get_motion_vector(
                &raw mut *((*cm).global_motion)
                    .as_mut_ptr()
                    .offset(*ref_frame.offset(0 as core::ffi::c_int as isize) as isize),
                (*features).allow_high_precision_mv as core::ffi::c_int,
                bsize,
                (*xd).mi_col,
                (*xd).mi_row,
                (*features).cur_frame_force_integer_mv as core::ffi::c_int,
            ))
                .as_int;
        }
        24 => {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < 2 as core::ffi::c_int {
                let nmvc_0: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
                read_mv(
                    r,
                    &raw mut (*mv.offset(i as isize)).as_mv,
                    &raw mut (*ref_mv.offset(i as isize)).as_mv,
                    nmvc_0,
                    allow_hp as MvSubpelPrecision,
                );
                i += 1;
            }
        }
        17 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*nearest_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
            (*mv.offset(1 as core::ffi::c_int as isize)).as_int = (*nearest_mv
                .offset(1 as core::ffi::c_int as isize))
                .as_int;
        }
        18 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*near_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
            (*mv.offset(1 as core::ffi::c_int as isize)).as_int = (*near_mv
                .offset(1 as core::ffi::c_int as isize))
                .as_int;
        }
        20 => {
            let nmvc_1: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            read_mv(
                r,
                &raw mut (*mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                &raw mut (*ref_mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                nmvc_1,
                allow_hp as MvSubpelPrecision,
            );
            (*mv.offset(1 as core::ffi::c_int as isize)).as_int = (*nearest_mv
                .offset(1 as core::ffi::c_int as isize))
                .as_int;
        }
        19 => {
            let nmvc_2: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*nearest_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
            read_mv(
                r,
                &raw mut (*mv.offset(1 as core::ffi::c_int as isize)).as_mv,
                &raw mut (*ref_mv.offset(1 as core::ffi::c_int as isize)).as_mv,
                nmvc_2,
                allow_hp as MvSubpelPrecision,
            );
        }
        21 => {
            let nmvc_3: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (*near_mv
                .offset(0 as core::ffi::c_int as isize))
                .as_int;
            read_mv(
                r,
                &raw mut (*mv.offset(1 as core::ffi::c_int as isize)).as_mv,
                &raw mut (*ref_mv.offset(1 as core::ffi::c_int as isize)).as_mv,
                nmvc_3,
                allow_hp as MvSubpelPrecision,
            );
        }
        22 => {
            let nmvc_4: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            read_mv(
                r,
                &raw mut (*mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                &raw mut (*ref_mv.offset(0 as core::ffi::c_int as isize)).as_mv,
                nmvc_4,
                allow_hp as MvSubpelPrecision,
            );
            (*mv.offset(1 as core::ffi::c_int as isize)).as_int = (*near_mv
                .offset(1 as core::ffi::c_int as isize))
                .as_int;
        }
        23 => {
            (*mv.offset(0 as core::ffi::c_int as isize)).as_int = (gm_get_motion_vector(
                &raw mut *((*cm).global_motion)
                    .as_mut_ptr()
                    .offset(*ref_frame.offset(0 as core::ffi::c_int as isize) as isize),
                (*features).allow_high_precision_mv as core::ffi::c_int,
                bsize,
                (*xd).mi_col,
                (*xd).mi_row,
                (*features).cur_frame_force_integer_mv as core::ffi::c_int,
            ))
                .as_int;
            (*mv.offset(1 as core::ffi::c_int as isize)).as_int = (gm_get_motion_vector(
                &raw mut *((*cm).global_motion)
                    .as_mut_ptr()
                    .offset(*ref_frame.offset(1 as core::ffi::c_int as isize) as isize),
                (*features).allow_high_precision_mv as core::ffi::c_int,
                bsize,
                (*xd).mi_col,
                (*xd).mi_row,
                (*features).cur_frame_force_integer_mv as core::ffi::c_int,
            ))
                .as_int;
        }
        _ => return 0 as core::ffi::c_int,
    }
    let mut ret: core::ffi::c_int = is_mv_valid(
        &raw mut (*mv.offset(0 as core::ffi::c_int as isize)).as_mv,
    );
    if is_compound != 0 {
        ret = (ret != 0
            && is_mv_valid(&raw mut (*mv.offset(1 as core::ffi::c_int as isize)).as_mv) != 0)
            as core::ffi::c_int;
    }
    return ret;
}
unsafe extern "C-unwind" fn read_is_inter_block(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut segment_id: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        let frame: core::ffi::c_int = get_segdata(
            &raw mut (*cm).seg,
            segment_id,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) as core::ffi::c_int;
        if frame < LAST_FRAME as core::ffi::c_int {
            return 0 as core::ffi::c_int;
        }
        return (frame != INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
    }
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        return 1 as core::ffi::c_int;
    }
    let ctx: core::ffi::c_int = av1_get_intra_inter_context(xd) as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let is_inter: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*ec_ctx).intra_inter_cdf[ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    ) as core::ffi::c_int;
    return is_inter;
}
unsafe extern "C-unwind" fn read_inter_block_mode_info(
    pbi: *mut AV1Decoder,
    mut dcb: *mut DecoderCodingBlock,
    mbmi: *mut MB_MODE_INFO,
    mut r: *mut aom_reader,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let allow_hp: core::ffi::c_int = (*features).allow_high_precision_mv
        as core::ffi::c_int;
    let mut nearestmv: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
    let mut nearmv: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
    let mut ref_mvs: [[int_mv; 2]; 29] = [
        [
            int_mv {
                as_int: 0 as core::ffi::c_int as uint32_t,
            },
            int_mv { as_int: 0 },
        ],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
        [int_mv { as_int: 0 }; 2],
    ];
    let mut inter_mode_ctx: [int16_t; 29] = [0; 29];
    let mut pts: [core::ffi::c_int; 16] = [0; 16];
    let mut pts_inref: [core::ffi::c_int; 16] = [0; 16];
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    (*mbmi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
    (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize] = 0
        as uint8_t;
    (*mbmi).palette_mode_info.palette_size[1 as core::ffi::c_int as usize] = 0
        as uint8_t;
    av1_collect_neighbors_ref_counts(xd);
    read_ref_frames(
        cm,
        xd,
        r,
        (*mbmi).segment_id() as core::ffi::c_int,
        ((*mbmi).ref_frame).as_mut_ptr(),
    );
    let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let ref_frame: MV_REFERENCE_FRAME = av1_ref_frame_type(
        ((*mbmi).ref_frame).as_mut_ptr(),
    ) as MV_REFERENCE_FRAME;
    av1_find_mv_refs(
        cm,
        xd,
        mbmi,
        ref_frame,
        ((*dcb).ref_mv_count).as_mut_ptr(),
        ((*xd).ref_mv_stack).as_mut_ptr(),
        ((*xd).weight).as_mut_ptr(),
        ref_mvs.as_mut_ptr(),
        0 as *mut int_mv,
        inter_mode_ctx.as_mut_ptr(),
    );
    (*mbmi).set_ref_mv_idx(0 as uint8_t as uint8_t);
    if (*mbmi).skip_mode() != 0 {
        (*mbmi).mode = NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE;
    } else if segfeature_active(
        &raw mut (*cm).seg,
        (*mbmi).segment_id(),
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        || segfeature_active(
            &raw mut (*cm).seg,
            (*mbmi).segment_id(),
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        (*mbmi).mode = GLOBALMV as core::ffi::c_int as PREDICTION_MODE;
    } else {
        let mode_ctx: core::ffi::c_int = av1_mode_context_analyzer(
            inter_mode_ctx.as_mut_ptr(),
            ((*mbmi).ref_frame).as_mut_ptr(),
        ) as core::ffi::c_int;
        if is_compound != 0 {
            (*mbmi).mode = read_inter_compound_mode(xd, r, mode_ctx as int16_t);
        } else {
            (*mbmi).mode = read_inter_mode(ec_ctx, r, mode_ctx as int16_t);
        }
        if (*mbmi).mode as core::ffi::c_int == NEWMV as core::ffi::c_int
            || (*mbmi).mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
            || have_nearmv_in_inter_mode((*mbmi).mode) != 0
        {
            read_drl_idx(ec_ctx, dcb, mbmi, r);
        }
    }
    if is_compound != is_inter_compound_mode((*mbmi).mode) {
        aom_internal_error(
            (*xd).error_info as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Prediction mode %d invalid with ref frame %d %d\0" as *const u8
                as *const core::ffi::c_char,
            (*mbmi).mode as core::ffi::c_int,
            (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int,
            (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int,
        );
    }
    if is_compound == 0
        && (*mbmi).mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
    {
        av1_find_best_ref_mvs(
            allow_hp,
            (ref_mvs[(*mbmi).ref_frame[0 as core::ffi::c_int as usize] as usize])
                .as_mut_ptr(),
            &raw mut *nearestmv.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
            &raw mut *nearmv.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
            (*features).cur_frame_force_integer_mv as core::ffi::c_int,
        );
    }
    if is_compound != 0
        && (*mbmi).mode as core::ffi::c_int != GLOBAL_GLOBALMV as core::ffi::c_int
    {
        let ref_mv_idx: core::ffi::c_int = (*mbmi).ref_mv_idx() as core::ffi::c_int
            + 1 as core::ffi::c_int;
        nearestmv[0 as core::ffi::c_int as usize] = (*xd)
            .ref_mv_stack[ref_frame as usize][0 as core::ffi::c_int as usize]
            .this_mv;
        nearestmv[1 as core::ffi::c_int as usize] = (*xd)
            .ref_mv_stack[ref_frame as usize][0 as core::ffi::c_int as usize]
            .comp_mv;
        nearmv[0 as core::ffi::c_int as usize] = (*xd)
            .ref_mv_stack[ref_frame as usize][ref_mv_idx as usize]
            .this_mv;
        nearmv[1 as core::ffi::c_int as usize] = (*xd)
            .ref_mv_stack[ref_frame as usize][ref_mv_idx as usize]
            .comp_mv;
        lower_mv_precision(
            &raw mut (*nearestmv.as_mut_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
            allow_hp,
            (*features).cur_frame_force_integer_mv as core::ffi::c_int,
        );
        lower_mv_precision(
            &raw mut (*nearestmv.as_mut_ptr().offset(1 as core::ffi::c_int as isize)).as_mv,
            allow_hp,
            (*features).cur_frame_force_integer_mv as core::ffi::c_int,
        );
        lower_mv_precision(
            &raw mut (*nearmv.as_mut_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
            allow_hp,
            (*features).cur_frame_force_integer_mv as core::ffi::c_int,
        );
        lower_mv_precision(
            &raw mut (*nearmv.as_mut_ptr().offset(1 as core::ffi::c_int as isize)).as_mv,
            allow_hp,
            (*features).cur_frame_force_integer_mv as core::ffi::c_int,
        );
    } else if (*mbmi).ref_mv_idx() as core::ffi::c_int > 0 as core::ffi::c_int
        && (*mbmi).mode as core::ffi::c_int == NEARMV as core::ffi::c_int
    {
        nearmv[0 as core::ffi::c_int as usize] = (*xd)
            .ref_mv_stack[(*mbmi).ref_frame[0 as core::ffi::c_int as usize]
                as usize][(1 as core::ffi::c_int
                + (*mbmi).ref_mv_idx() as core::ffi::c_int) as usize]
            .this_mv;
    }
    let mut ref_mv: [int_mv; 2] = [
        nearestmv[0 as core::ffi::c_int as usize],
        nearestmv[1 as core::ffi::c_int as usize],
    ];
    if is_compound != 0 {
        let mut ref_mv_idx_0: core::ffi::c_int = (*mbmi).ref_mv_idx()
            as core::ffi::c_int;
        if (*mbmi).mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
            || (*mbmi).mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int
        {
            ref_mv_idx_0 = 1 as core::ffi::c_int
                + (*mbmi).ref_mv_idx() as core::ffi::c_int;
        }
        if compound_ref0_mode((*mbmi).mode) as core::ffi::c_int
            == NEWMV as core::ffi::c_int
        {
            ref_mv[0 as core::ffi::c_int as usize] = (*xd)
                .ref_mv_stack[ref_frame as usize][ref_mv_idx_0 as usize]
                .this_mv;
        }
        if compound_ref1_mode((*mbmi).mode) as core::ffi::c_int
            == NEWMV as core::ffi::c_int
        {
            ref_mv[1 as core::ffi::c_int as usize] = (*xd)
                .ref_mv_stack[ref_frame as usize][ref_mv_idx_0 as usize]
                .comp_mv;
        }
    } else if (*mbmi).mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
        if (*dcb).ref_mv_count[ref_frame as usize] as core::ffi::c_int
            > 1 as core::ffi::c_int
        {
            ref_mv[0 as core::ffi::c_int as usize] = (*xd)
                .ref_mv_stack[ref_frame as usize][(*mbmi).ref_mv_idx() as usize]
                .this_mv;
        }
    }
    (*mbmi).skip_mode() != 0;
    let mv_corrupted_flag: core::ffi::c_int = (assign_mv(
        cm,
        xd,
        (*mbmi).mode,
        ((*mbmi).ref_frame).as_mut_ptr(),
        ((*mbmi).mv).as_mut_ptr(),
        ref_mv.as_mut_ptr(),
        nearestmv.as_mut_ptr(),
        nearmv.as_mut_ptr(),
        is_compound,
        allow_hp,
        r,
    ) == 0) as core::ffi::c_int;
    aom_merge_corrupted_flag(&raw mut (*dcb).corrupted, mv_corrupted_flag);
    (*mbmi).set_use_wedge_interintra(0 as uint8_t as uint8_t);
    if (*(*cm).seq_params).enable_interintra_compound as core::ffi::c_int != 0
        && (*mbmi).skip_mode() == 0 && is_interintra_allowed(mbmi) != 0
    {
        let bsize_group: core::ffi::c_int = size_group_lookup[bsize as usize]
            as core::ffi::c_int;
        let interintra: core::ffi::c_int = aom_read_symbol_(
            r,
            ((*ec_ctx).interintra_cdf[bsize_group as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if interintra != 0 {
            let interintra_mode: INTERINTRA_MODE = read_interintra_mode(
                xd,
                r,
                bsize_group,
            ) as INTERINTRA_MODE;
            (*mbmi).ref_frame[1 as core::ffi::c_int as usize] = INTRA_FRAME
                as core::ffi::c_int as MV_REFERENCE_FRAME;
            (*mbmi).interintra_mode = interintra_mode;
            (*mbmi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize] = 0 as int8_t;
            (*mbmi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize] = 0
                as int8_t;
            (*mbmi).filter_intra_mode_info.use_filter_intra = 0 as uint8_t;
            if av1_is_wedge_used(bsize) != 0 {
                (*mbmi)
                    .set_use_wedge_interintra(
                        aom_read_symbol_(
                            r,
                            ((*ec_ctx).wedge_interintra_cdf[bsize as usize])
                                .as_mut_ptr(),
                            2 as core::ffi::c_int,
                        ) as uint8_t as uint8_t,
                    );
                if (*mbmi).use_wedge_interintra() != 0 {
                    (*mbmi).interintra_wedge_index = aom_read_symbol_(
                        r,
                        ((*ec_ctx).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                        16 as core::ffi::c_int,
                    ) as int8_t;
                }
            }
        }
    }
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + has_second_ref(mbmi) {
        let frame: MV_REFERENCE_FRAME = (*mbmi).ref_frame[ref_0 as usize];
        (*xd).block_ref_scale_factors[ref_0 as usize] = get_ref_scale_factors_const(
            cm,
            frame,
        );
        ref_0 += 1;
    }
    (*mbmi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    if is_motion_variation_allowed_bsize((*mbmi).bsize) != 0 && (*mbmi).skip_mode() == 0
        && has_second_ref(mbmi) == 0
    {
        (*mbmi).num_proj_ref = av1_findSamples(
            cm,
            xd,
            pts.as_mut_ptr(),
            pts_inref.as_mut_ptr(),
        );
    }
    av1_count_overlappable_neighbors(cm, xd);
    if (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        != INTRA_FRAME as core::ffi::c_int
    {
        (*mbmi).motion_mode = read_motion_mode(cm, xd, mbmi, r);
    }
    (*mbmi).set_comp_group_idx(0 as uint8_t as uint8_t);
    (*mbmi).set_compound_idx(1 as uint8_t as uint8_t);
    (*mbmi).interinter_comp.type_0 = COMPOUND_AVERAGE as core::ffi::c_int
        as COMPOUND_TYPE;
    if has_second_ref(mbmi) != 0 && (*mbmi).skip_mode() == 0 {
        let masked_compound_used: core::ffi::c_int = (is_any_masked_compound_used(bsize)
            != 0 && (*(*cm).seq_params).enable_masked_compound as core::ffi::c_int != 0)
            as core::ffi::c_int;
        if masked_compound_used != 0 {
            let ctx_comp_group_idx: core::ffi::c_int = get_comp_group_idx_context(xd)
                as core::ffi::c_int;
            (*mbmi)
                .set_comp_group_idx(
                    aom_read_symbol_(
                        r,
                        ((*ec_ctx).comp_group_idx_cdf[ctx_comp_group_idx as usize])
                            .as_mut_ptr(),
                        2 as core::ffi::c_int,
                    ) as uint8_t as uint8_t,
                );
        }
        if (*mbmi).comp_group_idx() as core::ffi::c_int == 0 as core::ffi::c_int {
            if (*(*cm).seq_params).order_hint_info.enable_dist_wtd_comp != 0 {
                let comp_index_ctx: core::ffi::c_int = get_comp_index_context(cm, xd)
                    as core::ffi::c_int;
                (*mbmi)
                    .set_compound_idx(
                        aom_read_symbol_(
                            r,
                            ((*ec_ctx).compound_index_cdf[comp_index_ctx as usize])
                                .as_mut_ptr(),
                            2 as core::ffi::c_int,
                        ) as uint8_t as uint8_t,
                    );
                (*mbmi).interinter_comp.type_0 = (if (*mbmi).compound_idx()
                    as core::ffi::c_int != 0
                {
                    COMPOUND_AVERAGE as core::ffi::c_int
                } else {
                    COMPOUND_DISTWTD as core::ffi::c_int
                }) as COMPOUND_TYPE;
            } else {
                (*mbmi).set_compound_idx(1 as uint8_t as uint8_t);
                (*mbmi).interinter_comp.type_0 = COMPOUND_AVERAGE as core::ffi::c_int
                    as COMPOUND_TYPE;
            }
        } else {
            if is_interinter_compound_used(
                COMPOUND_WEDGE as core::ffi::c_int as COMPOUND_TYPE,
                bsize,
            ) != 0
            {
                (*mbmi).interinter_comp.type_0 = (COMPOUND_WEDGE as core::ffi::c_int
                    + aom_read_symbol_(
                        r,
                        ((*ec_ctx).compound_type_cdf[bsize as usize]).as_mut_ptr(),
                        MASKED_COMPOUND_TYPES as core::ffi::c_int,
                    )) as COMPOUND_TYPE;
            } else {
                (*mbmi).interinter_comp.type_0 = COMPOUND_DIFFWTD as core::ffi::c_int
                    as COMPOUND_TYPE;
            }
            if (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                == COMPOUND_WEDGE as core::ffi::c_int
            {
                (*mbmi).interinter_comp.wedge_index = aom_read_symbol_(
                    r,
                    ((*ec_ctx).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                    16 as core::ffi::c_int,
                ) as int8_t;
                (*mbmi).interinter_comp.wedge_sign = aom_read_bit_(r) as int8_t;
            } else {
                (*mbmi).interinter_comp.mask_type = aom_read_literal_(
                    r,
                    1 as core::ffi::c_int,
                ) as DIFFWTD_MASK_TYPE;
            }
        }
    }
    read_mb_interp_filter(
        xd,
        (*features).interp_filter,
        (*(*cm).seq_params).enable_dual_filter != 0,
        mbmi,
        r,
    );
    if (*mbmi).motion_mode as core::ffi::c_int == WARPED_CAUSAL as core::ffi::c_int {
        let mi_row: core::ffi::c_int = (*xd).mi_row;
        let mi_col: core::ffi::c_int = (*xd).mi_col;
        (*mbmi).wm_params.wmtype = AFFINE as core::ffi::c_int as TransformationType;
        (*mbmi).wm_params.invalid = 0 as int8_t;
        if (*mbmi).num_proj_ref as core::ffi::c_int > 1 as core::ffi::c_int {
            (*mbmi).num_proj_ref = av1_selectSamples(
                &raw mut (*((*mbmi).mv).as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                    .as_mv,
                pts.as_mut_ptr(),
                pts_inref.as_mut_ptr(),
                (*mbmi).num_proj_ref as core::ffi::c_int,
                bsize,
            );
        }
        if av1_find_projection(
            (*mbmi).num_proj_ref as core::ffi::c_int,
            pts.as_mut_ptr(),
            pts_inref.as_mut_ptr(),
            bsize,
            (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv.row as core::ffi::c_int,
            (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv.col as core::ffi::c_int,
            &raw mut (*mbmi).wm_params,
            mi_row,
            mi_col,
        ) != 0
        {
            (*mbmi).wm_params.invalid = 1 as int8_t;
        }
    }
    (*xd).cfl.store_y = store_cfl_required(cm, xd) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_inter_frame_mode_info(
    pbi: *mut AV1Decoder,
    mut dcb: *mut DecoderCodingBlock,
    mut r: *mut aom_reader,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut inter_block: core::ffi::c_int = 1 as core::ffi::c_int;
    (*mbmi).mv[0 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
    (*mbmi).mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
    (*mbmi)
        .set_segment_id(
            read_inter_segment_id(cm, xd, 1 as core::ffi::c_int, r) as uint8_t as uint8_t,
        );
    (*mbmi)
        .set_skip_mode(
            read_skip_mode(cm, xd, (*mbmi).segment_id() as core::ffi::c_int, r)
                as uint8_t as uint8_t,
        );
    if (*mbmi).skip_mode() != 0 {
        (*mbmi).skip_txfm = 1 as uint8_t;
    } else {
        (*mbmi).skip_txfm = read_skip_txfm(
            cm,
            xd,
            (*mbmi).segment_id() as core::ffi::c_int,
            r,
        ) as uint8_t;
    }
    if (*cm).seg.segid_preskip == 0 {
        (*mbmi)
            .set_segment_id(
                read_inter_segment_id(cm, xd, 0 as core::ffi::c_int, r) as uint8_t
                    as uint8_t,
            );
    }
    read_cdef(cm, r, xd);
    read_delta_q_params(cm, xd, r);
    if (*mbmi).skip_mode() == 0 {
        inter_block = read_is_inter_block(
            cm,
            xd,
            (*mbmi).segment_id() as core::ffi::c_int,
            r,
        );
    }
    (*mbmi).current_qindex = (*xd).current_base_qindex;
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*xd).tile.tile_row as isize))
        .offset((*xd).mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset(((*xd).mi_row & MAX_MIB_MASK) as isize);
    if inter_block != 0 {
        read_inter_block_mode_info(pbi, dcb, mbmi, r);
    } else {
        read_intra_block_mode_info(cm, xd, mbmi, r);
    };
}
unsafe extern "C-unwind" fn intra_copy_frame_mvs(
    cm: *mut AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) {
    let frame_mvs_stride: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    let mut frame_mvs: *mut MV_REF = ((*(*cm).cur_frame).mvs)
        .offset(((mi_row >> 1 as core::ffi::c_int) * frame_mvs_stride) as isize)
        .offset((mi_col >> 1 as core::ffi::c_int) as isize);
    x_mis = x_mis
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    y_mis = y_mis
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    let mut h: core::ffi::c_int = 0 as core::ffi::c_int;
    while h < y_mis {
        let mut mv: *mut MV_REF = frame_mvs;
        let mut w: core::ffi::c_int = 0 as core::ffi::c_int;
        while w < x_mis {
            (*mv).ref_frame = NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
            mv = mv.offset(1);
            w += 1;
        }
        frame_mvs = frame_mvs.offset(frame_mvs_stride as isize);
        h += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_mode_info(
    pbi: *mut AV1Decoder,
    mut dcb: *mut DecoderCodingBlock,
    mut r: *mut aom_reader,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mi).set_use_intrabc(0 as uint8_t as uint8_t);
    if frame_is_intra_only(cm) != 0 {
        read_intra_frame_mode_info(cm, dcb, r);
        if (*(*cm).seq_params).order_hint_info.enable_ref_frame_mvs != 0 {
            intra_copy_frame_mvs(cm, (*xd).mi_row, (*xd).mi_col, x_mis, y_mis);
        }
    } else {
        read_inter_frame_mode_info(pbi, dcb, r);
        if (*(*cm).seq_params).order_hint_info.enable_ref_frame_mvs != 0 {
            av1_copy_frame_mvs(cm, mi, (*xd).mi_row, (*xd).mi_col, x_mis, y_mis);
        }
    };
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
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
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_aom_ceil_log2(n: core::ffi::c_int) -> core::ffi::c_int {
    aom_ceil_log2(n)
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_av1_is_dv_valid(
    dv: MV,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    bsize: BLOCK_SIZE,
    mib_size_log2: core::ffi::c_int,
) -> core::ffi::c_int {
    av1_is_dv_valid(dv, cm, xd, mi_row, mi_col, bsize, mib_size_log2)
}
