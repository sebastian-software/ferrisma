/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/decoder/decodetxb.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_get_iqmatrix(
        quant_params: *const CommonQuantParams,
        xd: *const macroblockd,
        plane: core::ffi::c_int,
        tx_size: TX_SIZE,
        tx_type: TX_TYPE,
    ) -> *const qm_val_t;
    fn av1_set_entropy_contexts(
        xd: *const MACROBLOCKD,
        pd: *mut macroblockd_plane,
        plane: core::ffi::c_int,
        plane_bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
        has_eob: core::ffi::c_int,
        aoff: core::ffi::c_int,
        loff: core::ffi::c_int,
    );
    fn av1_get_tx_scale(tx_size: TX_SIZE) -> core::ffi::c_int;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    static av1_scan_orders: [[SCAN_ORDER; 16]; 19];
    static av1_eob_group_start: [int16_t; 12];
    static av1_eob_offset_bits: [int16_t; 12];
    static mut av1_nz_map_ctx_offset: [*const int8_t; 19];
    fn od_ec_decode_bool_q15(
        dec: *mut od_ec_dec,
        f: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn od_ec_decode_cdf_q15(
        dec: *mut od_ec_dec,
        cdf: *const uint16_t,
        nsyms: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_read_tx_type(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        blk_row: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        tx_size: TX_SIZE,
        r: *mut aom_reader,
    );
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
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
pub const PLANE_TYPES: C2RustUnnamed_2 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_2 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_2 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_3 = 25;
pub const INTRA_MODES: C2RustUnnamed_3 = 13;
pub const INTER_MODE_END: C2RustUnnamed_3 = 25;
pub const INTER_MODE_START: C2RustUnnamed_3 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_3 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_3 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_3 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_3 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_3 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_3 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_3 = 13;
pub const DIR_MODE_END: C2RustUnnamed_3 = 9;
pub const DIR_MODE_START: C2RustUnnamed_3 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_3 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_3 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_3 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_3 = 25;
pub const NEW_NEWMV: C2RustUnnamed_3 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_3 = 23;
pub const NEW_NEARMV: C2RustUnnamed_3 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_3 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_3 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_3 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_3 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_3 = 17;
pub const NEWMV: C2RustUnnamed_3 = 16;
pub const GLOBALMV: C2RustUnnamed_3 = 15;
pub const NEARMV: C2RustUnnamed_3 = 14;
pub const NEARESTMV: C2RustUnnamed_3 = 13;
pub const PAETH_PRED: C2RustUnnamed_3 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_3 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_3 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_3 = 9;
pub const D67_PRED: C2RustUnnamed_3 = 8;
pub const D203_PRED: C2RustUnnamed_3 = 7;
pub const D157_PRED: C2RustUnnamed_3 = 6;
pub const D113_PRED: C2RustUnnamed_3 = 5;
pub const D135_PRED: C2RustUnnamed_3 = 4;
pub const D45_PRED: C2RustUnnamed_3 = 3;
pub const H_PRED: C2RustUnnamed_3 = 2;
pub const V_PRED: C2RustUnnamed_3 = 1;
pub const DC_PRED: C2RustUnnamed_3 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_4 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_4 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_4 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_4 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_4 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_4 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_4 = 9;
pub const UV_D67_PRED: C2RustUnnamed_4 = 8;
pub const UV_D203_PRED: C2RustUnnamed_4 = 7;
pub const UV_D157_PRED: C2RustUnnamed_4 = 6;
pub const UV_D113_PRED: C2RustUnnamed_4 = 5;
pub const UV_D135_PRED: C2RustUnnamed_4 = 4;
pub const UV_D45_PRED: C2RustUnnamed_4 = 3;
pub const UV_H_PRED: C2RustUnnamed_4 = 2;
pub const UV_V_PRED: C2RustUnnamed_4 = 1;
pub const UV_DC_PRED: C2RustUnnamed_4 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_5 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_5 = 7;
pub const BWD_REFS: C2RustUnnamed_5 = 3;
pub const FWD_REFS: C2RustUnnamed_5 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_5 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_5 = 8;
pub const REF_FRAMES: C2RustUnnamed_5 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_5 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_5 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_5 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_5 = 4;
pub const LAST3_FRAME: C2RustUnnamed_5 = 3;
pub const LAST2_FRAME: C2RustUnnamed_5 = 2;
pub const LAST_FRAME: C2RustUnnamed_5 = 1;
pub const INTRA_FRAME: C2RustUnnamed_5 = 0;
pub const NONE_FRAME: C2RustUnnamed_5 = -1;
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
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub c2rust_unnamed_0: C2RustUnnamed_14,
    pub c2rust_unnamed_1: C2RustUnnamed_12,
    pub c2rust_unnamed_2: C2RustUnnamed_10,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_6,
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
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
}
pub type pthread_mutex_t = libc::pthread_mutex_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_17,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
}
pub type MACROBLOCKD = macroblockd;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txb_ctx {
    pub txb_skip_ctx: core::ffi::c_int,
    pub dc_sign_ctx: core::ffi::c_int,
}
pub type AV1_COMMON = AV1Common;
pub type TXB_CTX = txb_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCAN_ORDER {
    pub scan: *const int16_t,
    pub iscan: *const int16_t,
}
pub type br_cdf_arr = *mut [aom_cdf_prob; 5];
pub type base_cdf_arr = *mut [aom_cdf_prob; 5];
pub type TX_CLASS = uint8_t;
pub const TX_CLASS_HORIZ: C2RustUnnamed_18 = 1;
pub const TX_CLASS_VERT: C2RustUnnamed_18 = 2;
pub const TX_CLASS_2D: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const TX_CLASSES: C2RustUnnamed_18 = 3;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_QM_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const TX_PAD_HOR_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const TX_PAD_HOR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const TX_PAD_TOP: core::ffi::c_int = 0 as core::ffi::c_int;
pub const TX_PAD_BOTTOM: core::ffi::c_int = 4 as core::ffi::c_int;
pub const TX_PAD_VER: core::ffi::c_int = TX_PAD_TOP + TX_PAD_BOTTOM;
pub const TX_PAD_END: core::ffi::c_int = 16 as core::ffi::c_int;
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
static mut txsize_log2_minus4: [int8_t; 19] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
];
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
#[inline]
unsafe extern "C-unwind" fn update_cdf(
    mut cdf: *mut aom_cdf_prob,
    mut val: int8_t,
    mut nsymbs: core::ffi::c_int,
) {
    if !(nsymbs < 17 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"update_cdf\0"))
                .as_ptr(),
            b"prob.h\0" as *const u8 as *const core::ffi::c_char,
            111 as core::ffi::c_int,
            b"nsymbs < 17\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
pub const SIG_COEF_CONTEXTS_2D: core::ffi::c_int = 26 as core::ffi::c_int;
pub const NUM_BASE_LEVELS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const BR_CDF_SIZE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const COEFF_BASE_RANGE: core::ffi::c_int = 4 as core::ffi::c_int
    * (BR_CDF_SIZE - 1 as core::ffi::c_int);
pub const COEFF_CONTEXT_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const COEFF_CONTEXT_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << COEFF_CONTEXT_BITS) - 1 as core::ffi::c_int;
pub const MAX_BASE_BR_RANGE: core::ffi::c_int = COEFF_BASE_RANGE + NUM_BASE_LEVELS
    + 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_get_adjusted_tx_size(mut tx_size: TX_SIZE) -> TX_SIZE {
    match tx_size as core::ffi::c_int {
        4 | 12 | 11 => return TX_32X32 as core::ffi::c_int as TX_SIZE,
        18 => return TX_32X16 as core::ffi::c_int as TX_SIZE,
        17 => return TX_16X32 as core::ffi::c_int as TX_SIZE,
        _ => return tx_size,
    };
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
static mut tx_type_to_class: [TX_CLASS; 16] = [
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_2D as core::ffi::c_int as TX_CLASS,
    TX_CLASS_VERT as core::ffi::c_int as TX_CLASS,
    TX_CLASS_HORIZ as core::ffi::c_int as TX_CLASS,
    TX_CLASS_VERT as core::ffi::c_int as TX_CLASS,
    TX_CLASS_HORIZ as core::ffi::c_int as TX_CLASS,
    TX_CLASS_VERT as core::ffi::c_int as TX_CLASS,
    TX_CLASS_HORIZ as core::ffi::c_int as TX_CLASS,
];
#[inline]
unsafe extern "C-unwind" fn get_txb_bhl(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    tx_size = av1_get_adjusted_tx_size(tx_size);
    return tx_size_high_log2[tx_size as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_txb_wide(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    tx_size = av1_get_adjusted_tx_size(tx_size);
    return tx_size_wide[tx_size as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_txb_high(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    tx_size = av1_get_adjusted_tx_size(tx_size);
    return tx_size_high[tx_size as usize];
}
#[inline]
unsafe extern "C-unwind" fn set_levels(
    levels_buf: *mut uint8_t,
    height: core::ffi::c_int,
) -> *mut uint8_t {
    return levels_buf.offset((TX_PAD_TOP * (height + TX_PAD_HOR)) as isize);
}
#[inline]
unsafe extern "C-unwind" fn get_padded_idx(
    idx: core::ffi::c_int,
    bhl: core::ffi::c_int,
) -> core::ffi::c_int {
    return idx + (idx >> bhl << TX_PAD_HOR_LOG2);
}
#[inline]
unsafe extern "C-unwind" fn get_br_ctx_2d(
    levels: *const uint8_t,
    c: core::ffi::c_int,
    bhl: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(c > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"get_br_ctx_2d\0"))
                .as_ptr(),
            b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
            76 as core::ffi::c_int,
            b"c > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let col: core::ffi::c_int = c >> bhl;
    let row: core::ffi::c_int = c - (col << bhl);
    let stride: core::ffi::c_int = ((1 as core::ffi::c_int) << bhl) + TX_PAD_HOR;
    let pos: core::ffi::c_int = col * stride + row;
    let mut mag: core::ffi::c_int = (if (*levels
        .offset((pos + 1 as core::ffi::c_int) as isize) as core::ffi::c_int)
        < 4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
            + 2 as core::ffi::c_int + 1 as core::ffi::c_int
    {
        *levels.offset((pos + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
    } else {
        4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
            + 2 as core::ffi::c_int + 1 as core::ffi::c_int
    })
        + (if (*levels.offset((pos + stride) as isize) as core::ffi::c_int)
            < 4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
                + 2 as core::ffi::c_int + 1 as core::ffi::c_int
        {
            *levels.offset((pos + stride) as isize) as core::ffi::c_int
        } else {
            4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
                + 2 as core::ffi::c_int + 1 as core::ffi::c_int
        })
        + (if (*levels.offset((pos + 1 as core::ffi::c_int + stride) as isize)
            as core::ffi::c_int)
            < 4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
                + 2 as core::ffi::c_int + 1 as core::ffi::c_int
        {
            *levels.offset((pos + 1 as core::ffi::c_int + stride) as isize)
                as core::ffi::c_int
        } else {
            4 as core::ffi::c_int * (4 as core::ffi::c_int - 1 as core::ffi::c_int)
                + 2 as core::ffi::c_int + 1 as core::ffi::c_int
        });
    mag = if (mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        < 6 as core::ffi::c_int
    {
        mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
    } else {
        6 as core::ffi::c_int
    };
    if row | col < 2 as core::ffi::c_int {
        return mag + 7 as core::ffi::c_int;
    }
    return mag + 14 as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C-unwind" fn get_br_ctx_eob(
    c: core::ffi::c_int,
    bhl: core::ffi::c_int,
    tx_class: TX_CLASS,
) -> core::ffi::c_int {
    let col: core::ffi::c_int = c >> bhl;
    let row: core::ffi::c_int = c - (col << bhl);
    if c == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if tx_class as core::ffi::c_int == TX_CLASS_2D as core::ffi::c_int
        && row < 2 as core::ffi::c_int && col < 2 as core::ffi::c_int
        || tx_class as core::ffi::c_int == TX_CLASS_HORIZ as core::ffi::c_int
            && col == 0 as core::ffi::c_int
        || tx_class as core::ffi::c_int == TX_CLASS_VERT as core::ffi::c_int
            && row == 0 as core::ffi::c_int
    {
        return 7 as core::ffi::c_int;
    }
    return 14 as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C-unwind" fn get_br_ctx(
    levels: *const uint8_t,
    c: core::ffi::c_int,
    bhl: core::ffi::c_int,
    tx_class: TX_CLASS,
) -> core::ffi::c_int {
    let col: core::ffi::c_int = c >> bhl;
    let row: core::ffi::c_int = c - (col << bhl);
    let stride: core::ffi::c_int = ((1 as core::ffi::c_int) << bhl) + TX_PAD_HOR;
    let pos: core::ffi::c_int = col * stride + row;
    let mut mag: core::ffi::c_int = *levels
        .offset((pos + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
    mag += *levels.offset((pos + stride) as isize) as core::ffi::c_int;
    match tx_class as core::ffi::c_int {
        0 => {
            mag
                += *levels.offset((pos + stride + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int;
            mag = if (mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
                < 6 as core::ffi::c_int
            {
                mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
            } else {
                6 as core::ffi::c_int
            };
            if c == 0 as core::ffi::c_int {
                return mag;
            }
            if row < 2 as core::ffi::c_int && col < 2 as core::ffi::c_int {
                return mag + 7 as core::ffi::c_int;
            }
        }
        1 => {
            mag
                += *levels.offset((pos + (stride << 1 as core::ffi::c_int)) as isize)
                    as core::ffi::c_int;
            mag = if (mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
                < 6 as core::ffi::c_int
            {
                mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
            } else {
                6 as core::ffi::c_int
            };
            if c == 0 as core::ffi::c_int {
                return mag;
            }
            if col == 0 as core::ffi::c_int {
                return mag + 7 as core::ffi::c_int;
            }
        }
        2 => {
            mag
                += *levels.offset((pos + 2 as core::ffi::c_int) as isize)
                    as core::ffi::c_int;
            mag = if (mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
                < 6 as core::ffi::c_int
            {
                mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
            } else {
                6 as core::ffi::c_int
            };
            if c == 0 as core::ffi::c_int {
                return mag;
            }
            if row == 0 as core::ffi::c_int {
                return mag + 7 as core::ffi::c_int;
            }
        }
        _ => {}
    }
    return mag + 14 as core::ffi::c_int;
}
static mut clip_max3: [uint8_t; 256] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
];
#[inline(always)]
unsafe extern "C-unwind" fn get_nz_mag(
    levels: *const uint8_t,
    bhl: core::ffi::c_int,
    tx_class: TX_CLASS,
) -> core::ffi::c_int {
    let mut mag: core::ffi::c_int = 0;
    mag = clip_max3[*levels
        .offset((((1 as core::ffi::c_int) << bhl) + TX_PAD_HOR) as isize) as usize]
        as core::ffi::c_int;
    mag
        += clip_max3[*levels.offset(1 as core::ffi::c_int as isize) as usize]
            as core::ffi::c_int;
    if tx_class as core::ffi::c_int == TX_CLASS_2D as core::ffi::c_int {
        mag
            += clip_max3[*levels
                .offset(
                    (((1 as core::ffi::c_int) << bhl) + TX_PAD_HOR
                        + 1 as core::ffi::c_int) as isize,
                ) as usize] as core::ffi::c_int;
        mag
            += clip_max3[*levels
                .offset(
                    (((2 as core::ffi::c_int) << bhl)
                        + ((2 as core::ffi::c_int) << TX_PAD_HOR_LOG2)) as isize,
                ) as usize] as core::ffi::c_int;
        mag
            += clip_max3[*levels.offset(2 as core::ffi::c_int as isize) as usize]
                as core::ffi::c_int;
    } else if tx_class as core::ffi::c_int == TX_CLASS_VERT as core::ffi::c_int {
        mag
            += clip_max3[*levels.offset(2 as core::ffi::c_int as isize) as usize]
                as core::ffi::c_int;
        mag
            += clip_max3[*levels.offset(3 as core::ffi::c_int as isize) as usize]
                as core::ffi::c_int;
        mag
            += clip_max3[*levels.offset(4 as core::ffi::c_int as isize) as usize]
                as core::ffi::c_int;
    } else {
        mag
            += clip_max3[*levels
                .offset(
                    (((2 as core::ffi::c_int) << bhl)
                        + ((2 as core::ffi::c_int) << TX_PAD_HOR_LOG2)) as isize,
                ) as usize] as core::ffi::c_int;
        mag
            += clip_max3[*levels
                .offset(
                    (((3 as core::ffi::c_int) << bhl)
                        + ((3 as core::ffi::c_int) << TX_PAD_HOR_LOG2)) as isize,
                ) as usize] as core::ffi::c_int;
        mag
            += clip_max3[*levels
                .offset(
                    (((4 as core::ffi::c_int) << bhl)
                        + ((4 as core::ffi::c_int) << TX_PAD_HOR_LOG2)) as isize,
                ) as usize] as core::ffi::c_int;
    }
    return mag;
}
pub const NZ_MAP_CTX_0: core::ffi::c_int = SIG_COEF_CONTEXTS_2D;
pub const NZ_MAP_CTX_5: core::ffi::c_int = NZ_MAP_CTX_0 + 5 as core::ffi::c_int;
pub const NZ_MAP_CTX_10: core::ffi::c_int = NZ_MAP_CTX_0 + 10 as core::ffi::c_int;
static mut nz_map_ctx_offset_1d: [core::ffi::c_int; 32] = [
    NZ_MAP_CTX_0,
    NZ_MAP_CTX_5,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
];
#[inline(always)]
unsafe extern "C-unwind" fn get_nz_map_ctx_from_stats(
    stats: core::ffi::c_int,
    coeff_idx: core::ffi::c_int,
    bhl: core::ffi::c_int,
    tx_size: TX_SIZE,
    tx_class: TX_CLASS,
) -> core::ffi::c_int {
    if tx_class as core::ffi::c_int | coeff_idx == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut ctx: core::ffi::c_int = stats + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    ctx = if ctx < 4 as core::ffi::c_int { ctx } else { 4 as core::ffi::c_int };
    match tx_class as core::ffi::c_int {
        0 => {
            return ctx
                + *(av1_nz_map_ctx_offset[tx_size as usize]).offset(coeff_idx as isize)
                    as core::ffi::c_int;
        }
        1 => {
            let col: core::ffi::c_int = coeff_idx >> bhl;
            return ctx + nz_map_ctx_offset_1d[col as usize];
        }
        2 => {
            let col_0: core::ffi::c_int = coeff_idx >> bhl;
            let row: core::ffi::c_int = coeff_idx - (col_0 << bhl);
            return ctx + nz_map_ctx_offset_1d[row as usize];
        }
        _ => {}
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_lower_levels_ctx_eob(
    mut bhl: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut scan_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    if scan_idx == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if scan_idx <= (width << bhl) / 8 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    if scan_idx <= (width << bhl) / 4 as core::ffi::c_int {
        return 2 as core::ffi::c_int;
    }
    return 3 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_lower_levels_ctx_2d(
    mut levels: *const uint8_t,
    mut coeff_idx: core::ffi::c_int,
    mut bhl: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    if !(coeff_idx > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"get_lower_levels_ctx_2d\0"))
                .as_ptr(),
            b"txb_common.h\0" as *const u8 as *const core::ffi::c_char,
            238 as core::ffi::c_int,
            b"coeff_idx > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut mag: core::ffi::c_int = 0;
    levels = levels.offset(get_padded_idx(coeff_idx, bhl) as isize);
    mag = if (*levels
        .offset((((1 as core::ffi::c_int) << bhl) + 4 as core::ffi::c_int) as isize)
        as core::ffi::c_int) < 3 as core::ffi::c_int
    {
        *levels
            .offset((((1 as core::ffi::c_int) << bhl) + 4 as core::ffi::c_int) as isize)
            as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    mag
        += if (*levels.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
            < 3 as core::ffi::c_int
        {
            *levels.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
    mag
        += if (*levels
            .offset(
                (((1 as core::ffi::c_int) << bhl) + 4 as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            ) as core::ffi::c_int) < 3 as core::ffi::c_int
        {
            *levels
                .offset(
                    (((1 as core::ffi::c_int) << bhl) + 4 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
    mag
        += if (*levels
            .offset(
                (((2 as core::ffi::c_int) << bhl)
                    + ((2 as core::ffi::c_int) << 2 as core::ffi::c_int)) as isize,
            ) as core::ffi::c_int) < 3 as core::ffi::c_int
        {
            *levels
                .offset(
                    (((2 as core::ffi::c_int) << bhl)
                        + ((2 as core::ffi::c_int) << 2 as core::ffi::c_int)) as isize,
                ) as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
    mag
        += if (*levels.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int)
            < 3 as core::ffi::c_int
        {
            *levels.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
    let ctx: core::ffi::c_int = if (mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        < 4 as core::ffi::c_int
    {
        mag + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
    } else {
        4 as core::ffi::c_int
    };
    return ctx
        + *(av1_nz_map_ctx_offset[tx_size as usize]).offset(coeff_idx as isize)
            as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C-unwind" fn get_lower_levels_ctx(
    mut levels: *const uint8_t,
    mut coeff_idx: core::ffi::c_int,
    mut bhl: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut tx_class: TX_CLASS,
) -> core::ffi::c_int {
    let stats: core::ffi::c_int = get_nz_mag(
        levels.offset(get_padded_idx(coeff_idx, bhl) as isize),
        bhl,
        tx_class,
    ) as core::ffi::c_int;
    return get_nz_map_ctx_from_stats(stats, coeff_idx, bhl, tx_size, tx_class);
}
#[inline]
unsafe extern "C-unwind" fn set_dc_sign(
    mut cul_level: *mut core::ffi::c_int,
    mut dc_val: core::ffi::c_int,
) {
    if dc_val < 0 as core::ffi::c_int {
        *cul_level |= (1 as core::ffi::c_int) << COEFF_CONTEXT_BITS;
    } else if dc_val > 0 as core::ffi::c_int {
        *cul_level += (2 as core::ffi::c_int) << COEFF_CONTEXT_BITS;
    }
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
unsafe extern "C-unwind" fn aom_read_cdf_(
    mut r: *mut aom_reader,
    mut cdf: *const aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut symb: core::ffi::c_int = 0;
    if cdf.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_read_cdf_\0"))
                .as_ptr(),
            b"bitreader.h\0" as *const u8 as *const core::ffi::c_char,
            171 as core::ffi::c_int,
            b"cdf != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C-unwind" fn read_golomb(
    mut xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    let mut x: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut length: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i == 0 {
        i = aom_read_bit_(r);
        length += 1;
        if length > 20 as core::ffi::c_int {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid length in read_golomb\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    i = 0 as core::ffi::c_int;
    while i < length - 1 as core::ffi::c_int {
        x <<= 1 as core::ffi::c_int;
        x += aom_read_bit_(r);
        i += 1;
    }
    return x - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn rec_eob_pos(
    eob_token: core::ffi::c_int,
    extra: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut eob: core::ffi::c_int = av1_eob_group_start[eob_token as usize]
        as core::ffi::c_int;
    if eob > 2 as core::ffi::c_int {
        eob += extra;
    }
    return eob;
}
#[inline]
unsafe extern "C-unwind" fn get_dqv(
    mut dequant: *const int16_t,
    mut coeff_idx: core::ffi::c_int,
    mut iqmatrix: *const qm_val_t,
) -> core::ffi::c_int {
    let mut dqv: core::ffi::c_int = *dequant
        .offset((coeff_idx != 0) as core::ffi::c_int as isize) as core::ffi::c_int;
    if !iqmatrix.is_null() {
        dqv = *iqmatrix.offset(coeff_idx as isize) as core::ffi::c_int * dqv
            + ((1 as core::ffi::c_int) << AOM_QM_BITS - 1 as core::ffi::c_int)
            >> AOM_QM_BITS;
    }
    return dqv;
}
#[inline]
unsafe extern "C-unwind" fn read_coeffs_reverse_2d(
    mut r: *mut aom_reader,
    mut tx_size: TX_SIZE,
    mut start_si: core::ffi::c_int,
    mut end_si: core::ffi::c_int,
    mut scan: *const int16_t,
    mut bhl: core::ffi::c_int,
    mut levels: *mut uint8_t,
    mut base_cdf: base_cdf_arr,
    mut br_cdf: br_cdf_arr,
) {
    let mut c: core::ffi::c_int = end_si;
    while c >= start_si {
        let pos: core::ffi::c_int = *scan.offset(c as isize) as core::ffi::c_int;
        let coeff_ctx: core::ffi::c_int = get_lower_levels_ctx_2d(
            levels,
            pos,
            bhl,
            tx_size,
        ) as core::ffi::c_int;
        let nsymbs: core::ffi::c_int = 4 as core::ffi::c_int;
        let mut level: core::ffi::c_int = aom_read_symbol_(
            r,
            (*base_cdf.offset(coeff_ctx as isize)).as_mut_ptr(),
            nsymbs,
        );
        if level > NUM_BASE_LEVELS {
            let br_ctx: core::ffi::c_int = get_br_ctx_2d(levels, pos, bhl)
                as core::ffi::c_int;
            let mut cdf: *mut aom_cdf_prob = (*br_cdf.offset(br_ctx as isize))
                .as_mut_ptr();
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < COEFF_BASE_RANGE {
                let k: core::ffi::c_int = aom_read_symbol_(r, cdf, 4 as core::ffi::c_int)
                    as core::ffi::c_int;
                level += k;
                if k < BR_CDF_SIZE - 1 as core::ffi::c_int {
                    break;
                }
                idx += BR_CDF_SIZE - 1 as core::ffi::c_int;
            }
        }
        *levels.offset(get_padded_idx(pos, bhl) as isize) = level as uint8_t;
        c -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn read_coeffs_reverse(
    mut r: *mut aom_reader,
    mut tx_size: TX_SIZE,
    mut tx_class: TX_CLASS,
    mut start_si: core::ffi::c_int,
    mut end_si: core::ffi::c_int,
    mut scan: *const int16_t,
    mut bhl: core::ffi::c_int,
    mut levels: *mut uint8_t,
    mut base_cdf: base_cdf_arr,
    mut br_cdf: br_cdf_arr,
) {
    let mut c: core::ffi::c_int = end_si;
    while c >= start_si {
        let pos: core::ffi::c_int = *scan.offset(c as isize) as core::ffi::c_int;
        let coeff_ctx: core::ffi::c_int = get_lower_levels_ctx(
            levels,
            pos,
            bhl,
            tx_size,
            tx_class,
        ) as core::ffi::c_int;
        let nsymbs: core::ffi::c_int = 4 as core::ffi::c_int;
        let mut level: core::ffi::c_int = aom_read_symbol_(
            r,
            (*base_cdf.offset(coeff_ctx as isize)).as_mut_ptr(),
            nsymbs,
        );
        if level > NUM_BASE_LEVELS {
            let br_ctx: core::ffi::c_int = get_br_ctx(levels, pos, bhl, tx_class)
                as core::ffi::c_int;
            let mut cdf: *mut aom_cdf_prob = (*br_cdf.offset(br_ctx as isize))
                .as_mut_ptr();
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < COEFF_BASE_RANGE {
                let k: core::ffi::c_int = aom_read_symbol_(r, cdf, 4 as core::ffi::c_int)
                    as core::ffi::c_int;
                level += k;
                if k < BR_CDF_SIZE - 1 as core::ffi::c_int {
                    break;
                }
                idx += BR_CDF_SIZE - 1 as core::ffi::c_int;
            }
        }
        *levels.offset(get_padded_idx(pos, bhl) as isize) = level as uint8_t;
        c -= 1;
    }
}
unsafe extern "C-unwind" fn read_coeffs_txb(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    blk_row: core::ffi::c_int,
    blk_col: core::ffi::c_int,
    plane: core::ffi::c_int,
    txb_ctx: *const TXB_CTX,
    tx_size: TX_SIZE,
) -> uint8_t {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let max_value: int32_t = ((1 as int32_t) << 7 as core::ffi::c_int + (*xd).bd)
        - 1 as int32_t;
    let min_value: int32_t = -((1 as int32_t) << 7 as core::ffi::c_int + (*xd).bd);
    let txs_ctx: TX_SIZE = get_txsize_entropy_ctx(tx_size) as TX_SIZE;
    let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let dequant: *const int16_t = ((*pd).seg_dequant_QTX[(*mbmi).segment_id() as usize])
        .as_mut_ptr();
    let tcoeffs: *mut tran_low_t = ((*dcb).dqcoeff_block[plane as usize])
        .offset((*dcb).cb_offset[plane as usize] as core::ffi::c_int as isize);
    let shift: core::ffi::c_int = av1_get_tx_scale(tx_size) as core::ffi::c_int;
    let bhl: core::ffi::c_int = get_txb_bhl(tx_size) as core::ffi::c_int;
    let width: core::ffi::c_int = get_txb_wide(tx_size) as core::ffi::c_int;
    let height: core::ffi::c_int = get_txb_high(tx_size) as core::ffi::c_int;
    let mut cul_level: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dc_val: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut levels_buf: [uint8_t; 1312] = [0; 1312];
    let levels: *mut uint8_t = set_levels(levels_buf.as_mut_ptr(), height)
        as *mut uint8_t;
    let all_zero: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*ec_ctx).txb_skip_cdf[txs_ctx as usize][(*txb_ctx).txb_skip_ctx as usize])
            .as_mut_ptr(),
        2 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut eob_data: *mut eob_info = ((*dcb).eob_data[plane as usize])
        .offset((*dcb).txb_offset[plane as usize] as core::ffi::c_int as isize);
    let eob: *mut uint16_t = &raw mut (*eob_data).eob;
    let max_scan_line: *mut uint16_t = &raw mut (*eob_data).max_scan_line;
    *max_scan_line = 0 as uint16_t;
    *eob = 0 as uint16_t;
    if all_zero != 0 {
        *max_scan_line = 0 as uint16_t;
        if plane == 0 as core::ffi::c_int {
            *((*xd).tx_type_map)
                .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize) = DCT_DCT
                as core::ffi::c_int as uint8_t;
        }
        return 0 as uint8_t;
    }
    if plane == AOM_PLANE_Y {
        av1_read_tx_type(cm, xd, blk_row, blk_col, tx_size, r);
    }
    let tx_type: TX_TYPE = av1_get_tx_type(
        xd,
        plane_type,
        blk_row,
        blk_col,
        tx_size,
        (*cm).features.reduced_tx_set_used as core::ffi::c_int,
    ) as TX_TYPE;
    let tx_class: TX_CLASS = tx_type_to_class[tx_type as usize];
    let mut iqmatrix: *const qm_val_t = av1_get_iqmatrix(
        &(*cm).quant_params,
        xd,
        plane,
        tx_size,
        tx_type,
    );
    let scan_order: *const SCAN_ORDER = get_scan(tx_size, tx_type) as *const SCAN_ORDER;
    let scan: *const int16_t = (*scan_order).scan;
    let mut eob_extra: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut eob_pt: core::ffi::c_int = 1 as core::ffi::c_int;
    let eob_multi_size: core::ffi::c_int = txsize_log2_minus4[tx_size as usize]
        as core::ffi::c_int;
    let eob_multi_ctx: core::ffi::c_int = if tx_class as core::ffi::c_int
        == TX_CLASS_2D as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    match eob_multi_size {
        0 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf16[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                5 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        1 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf32[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                6 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        2 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf64[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                7 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        3 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf128[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                8 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        4 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf256[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                9 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        5 => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf512[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                10 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
        6 | _ => {
            eob_pt = aom_read_symbol_(
                r,
                ((*ec_ctx).eob_flag_cdf1024[plane_type as usize][eob_multi_ctx as usize])
                    .as_mut_ptr(),
                11 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
        }
    }
    let eob_offset_bits: core::ffi::c_int = av1_eob_offset_bits[eob_pt as usize]
        as core::ffi::c_int;
    if eob_offset_bits > 0 as core::ffi::c_int {
        let eob_ctx: core::ffi::c_int = eob_pt - 3 as core::ffi::c_int;
        let mut bit: core::ffi::c_int = aom_read_symbol_(
            r,
            ((*ec_ctx)
                .eob_extra_cdf[txs_ctx as usize][plane_type as usize][eob_ctx as usize])
                .as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if bit != 0 {
            eob_extra
                += (1 as core::ffi::c_int) << eob_offset_bits - 1 as core::ffi::c_int;
        }
        let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
        while i < eob_offset_bits {
            bit = aom_read_bit_(r);
            if bit != 0 {
                eob_extra
                    += (1 as core::ffi::c_int)
                        << eob_offset_bits - 1 as core::ffi::c_int - i;
            }
            i += 1;
        }
    }
    *eob = rec_eob_pos(eob_pt, eob_extra) as uint16_t;
    if *eob as core::ffi::c_int > 1 as core::ffi::c_int {
        memset(
            levels_buf.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul(
                    ((height + TX_PAD_HOR) * (width + TX_PAD_VER) + TX_PAD_END) as size_t,
                ),
        );
    }
    let c: core::ffi::c_int = *eob as core::ffi::c_int - 1 as core::ffi::c_int;
    let pos: core::ffi::c_int = *scan.offset(c as isize) as core::ffi::c_int;
    let coeff_ctx: core::ffi::c_int = get_lower_levels_ctx_eob(bhl, width, c)
        as core::ffi::c_int;
    let nsymbs: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut cdf: *mut aom_cdf_prob = ((*ec_ctx)
        .coeff_base_eob_cdf[txs_ctx as usize][plane_type as usize][coeff_ctx as usize])
        .as_mut_ptr();
    let mut level: core::ffi::c_int = aom_read_symbol_(r, cdf, nsymbs)
        + 1 as core::ffi::c_int;
    if level > NUM_BASE_LEVELS {
        let br_ctx: core::ffi::c_int = get_br_ctx_eob(pos, bhl, tx_class)
            as core::ffi::c_int;
        cdf = ((*ec_ctx)
            .coeff_br_cdf[(if (txs_ctx as core::ffi::c_int)
            < TX_32X32 as core::ffi::c_int
        {
            txs_ctx as core::ffi::c_int
        } else {
            TX_32X32 as core::ffi::c_int
        }) as usize][plane_type as usize][br_ctx as usize])
            .as_mut_ptr();
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < COEFF_BASE_RANGE {
            let k: core::ffi::c_int = aom_read_symbol_(r, cdf, 4 as core::ffi::c_int)
                as core::ffi::c_int;
            level += k;
            if k < BR_CDF_SIZE - 1 as core::ffi::c_int {
                break;
            }
            idx += BR_CDF_SIZE - 1 as core::ffi::c_int;
        }
    }
    *levels.offset(get_padded_idx(pos, bhl) as isize) = level as uint8_t;
    if *eob as core::ffi::c_int > 1 as core::ffi::c_int {
        let mut base_cdf: base_cdf_arr = ((*ec_ctx)
            .coeff_base_cdf[txs_ctx as usize][plane_type as usize])
            .as_mut_ptr() as base_cdf_arr;
        let mut br_cdf: br_cdf_arr = ((*ec_ctx)
            .coeff_br_cdf[(if (txs_ctx as core::ffi::c_int)
            < TX_32X32 as core::ffi::c_int
        {
            txs_ctx as core::ffi::c_int
        } else {
            TX_32X32 as core::ffi::c_int
        }) as usize][plane_type as usize])
            .as_mut_ptr() as br_cdf_arr;
        if tx_class as core::ffi::c_int == TX_CLASS_2D as core::ffi::c_int {
            read_coeffs_reverse_2d(
                r,
                tx_size,
                1 as core::ffi::c_int,
                *eob as core::ffi::c_int - 1 as core::ffi::c_int - 1 as core::ffi::c_int,
                scan,
                bhl,
                levels,
                base_cdf,
                br_cdf,
            );
            read_coeffs_reverse(
                r,
                tx_size,
                tx_class,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                scan,
                bhl,
                levels,
                base_cdf,
                br_cdf,
            );
        } else {
            read_coeffs_reverse(
                r,
                tx_size,
                tx_class,
                0 as core::ffi::c_int,
                *eob as core::ffi::c_int - 1 as core::ffi::c_int - 1 as core::ffi::c_int,
                scan,
                bhl,
                levels,
                base_cdf,
                br_cdf,
            );
        }
    }
    let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_0 < *eob as core::ffi::c_int {
        let pos_0: core::ffi::c_int = *scan.offset(c_0 as isize) as core::ffi::c_int;
        let mut sign: uint8_t = 0;
        let mut level_0: tran_low_t = *levels.offset(get_padded_idx(pos_0, bhl) as isize)
            as tran_low_t;
        if level_0 != 0 {
            *max_scan_line = (if *max_scan_line as core::ffi::c_int > pos_0 {
                *max_scan_line as core::ffi::c_int
            } else {
                pos_0
            }) as uint16_t;
            if c_0 == 0 as core::ffi::c_int {
                let dc_sign_ctx: core::ffi::c_int = (*txb_ctx).dc_sign_ctx;
                sign = aom_read_symbol_(
                    r,
                    ((*ec_ctx).dc_sign_cdf[plane_type as usize][dc_sign_ctx as usize])
                        .as_mut_ptr(),
                    2 as core::ffi::c_int,
                ) as uint8_t;
            } else {
                sign = aom_read_bit_(r) as uint8_t;
            }
            if level_0 >= MAX_BASE_BR_RANGE as tran_low_t {
                level_0 = (level_0 as core::ffi::c_int + read_golomb(xd, r))
                    as tran_low_t;
            }
            if c_0 == 0 as core::ffi::c_int {
                dc_val = (if sign as core::ffi::c_int != 0 { -level_0 } else { level_0 })
                    as core::ffi::c_int;
            }
            level_0 = (level_0 as core::ffi::c_int & 0xfffff as core::ffi::c_int)
                as tran_low_t;
            cul_level += level_0 as core::ffi::c_int;
            let mut dq_coeff: tran_low_t = 0;
            dq_coeff = (level_0 as int64_t
                * get_dqv(
                    dequant,
                    *scan.offset(c_0 as isize) as core::ffi::c_int,
                    iqmatrix,
                ) as int64_t & 0xffffff as int64_t) as tran_low_t;
            dq_coeff = dq_coeff >> shift;
            if sign != 0 {
                dq_coeff = -dq_coeff;
            }
            *tcoeffs.offset(pos_0 as isize) = clamp(
                dq_coeff as core::ffi::c_int,
                min_value as core::ffi::c_int,
                max_value as core::ffi::c_int,
            ) as tran_low_t;
        }
        c_0 += 1;
    }
    cul_level = if (((1 as core::ffi::c_int) << 3 as core::ffi::c_int)
        - 1 as core::ffi::c_int) < cul_level
    {
        ((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int
    } else {
        cul_level
    };
    set_dc_sign(&raw mut cul_level, dc_val);
    return cul_level as uint8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_coeffs_txb(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    plane: core::ffi::c_int,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    tx_size: TX_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_read_coeffs_txb\0"))
                .as_ptr(),
            b"decodetxb.c\0" as *const u8 as *const core::ffi::c_char,
            336 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
    ) as BLOCK_SIZE;
    let mut txb_ctx: TXB_CTX = txb_ctx {
        txb_skip_ctx: 0,
        dc_sign_ctx: 0,
    };
    get_txb_ctx(
        plane_bsize,
        tx_size,
        plane,
        ((*pd).above_entropy_context).offset(col as isize),
        ((*pd).left_entropy_context).offset(row as isize),
        &raw mut txb_ctx,
    );
    if ((*(*dcb).xd.error_info).setjmp == 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_read_coeffs_txb\0"))
                .as_ptr(),
            b"decodetxb.c\0" as *const u8 as *const core::ffi::c_char,
            344 as core::ffi::c_int,
            b"dcb->xd.error_info->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cul_level: uint8_t = read_coeffs_txb(
        cm,
        dcb,
        r,
        row,
        col,
        plane,
        &raw mut txb_ctx,
        tx_size,
    ) as uint8_t;
    av1_set_entropy_contexts(
        xd,
        pd,
        plane,
        plane_bsize,
        tx_size,
        cul_level as core::ffi::c_int,
        col,
        row,
    );
    if is_inter_block(mbmi) != 0 {
        let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
        let tx_type: TX_TYPE = av1_get_tx_type(
            xd,
            plane_type,
            row,
            col,
            tx_size,
            (*cm).features.reduced_tx_set_used as core::ffi::c_int,
        ) as TX_TYPE;
        if plane == 0 as core::ffi::c_int {
            let txw: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
            let txh: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
            if txw == tx_size_wide_unit[TX_64X64 as core::ffi::c_int as usize]
                || txh == tx_size_high_unit[TX_64X64 as core::ffi::c_int as usize]
            {
                let tx_unit: core::ffi::c_int = tx_size_wide_unit[TX_16X16
                    as core::ffi::c_int as usize];
                let stride: core::ffi::c_int = (*xd).tx_type_map_stride;
                let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
                while idy < txh {
                    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
                    while idx < txw {
                        *((*xd).tx_type_map)
                            .offset(((row + idy) * stride + col + idx) as isize) = tx_type
                            as uint8_t;
                        idx += tx_unit;
                    }
                    idy += tx_unit;
                }
            }
        }
    }
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_get_default_scan(
    tx_size: TX_SIZE,
    tx_type: TX_TYPE,
) -> *const SCAN_ORDER {
    get_default_scan(tx_size, tx_type)
}
