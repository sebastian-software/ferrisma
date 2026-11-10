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

// Mechanically translated from upstream/aom/av1/encoder/interp_search.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_get_pred_context_switchable_interp(
        xd: *const MACROBLOCKD,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
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
    fn aom_highbd_sse_c(
        a8: *const uint8_t,
        a_stride: core::ffi::c_int,
        b8: *const uint8_t,
        b_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> int64_t;
    fn aom_sse_c(
        a: *const uint8_t,
        a_stride: core::ffi::c_int,
        b: *const uint8_t,
        b_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> int64_t;
    fn av1_enc_build_inter_predictor(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        ctx: *const BUFFER_SET,
        bsize: BLOCK_SIZE,
        plane_from: core::ffi::c_int,
        plane_to: core::ffi::c_int,
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
pub type InterpKernel = [int16_t; 8];
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
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub frame: C2RustUnnamed_0,
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
pub struct C2RustUnnamed_0 {
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
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
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
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const SHARP_SHARP: C2RustUnnamed_1 = 8;
pub const SHARP_SMOOTH: C2RustUnnamed_1 = 7;
pub const SHARP_REG: C2RustUnnamed_1 = 6;
pub const SMOOTH_SHARP: C2RustUnnamed_1 = 5;
pub const SMOOTH_SMOOTH: C2RustUnnamed_1 = 4;
pub const SMOOTH_REG: C2RustUnnamed_1 = 3;
pub const REG_SHARP: C2RustUnnamed_1 = 2;
pub const REG_SMOOTH: C2RustUnnamed_1 = 1;
pub const REG_REG: C2RustUnnamed_1 = 0;
pub type DUAL_FILTER_TYPE = uint8_t;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_2 = 25;
pub const INTRA_MODES: C2RustUnnamed_2 = 13;
pub const INTER_MODE_END: C2RustUnnamed_2 = 25;
pub const INTER_MODE_START: C2RustUnnamed_2 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_2 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_2 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_2 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_2 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_2 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_2 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_2 = 13;
pub const DIR_MODE_END: C2RustUnnamed_2 = 9;
pub const DIR_MODE_START: C2RustUnnamed_2 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_2 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_2 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_2 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_2 = 25;
pub const NEW_NEWMV: C2RustUnnamed_2 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_2 = 23;
pub const NEW_NEARMV: C2RustUnnamed_2 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_2 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_2 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_2 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_2 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_2 = 17;
pub const NEWMV: C2RustUnnamed_2 = 16;
pub const GLOBALMV: C2RustUnnamed_2 = 15;
pub const NEARMV: C2RustUnnamed_2 = 14;
pub const NEARESTMV: C2RustUnnamed_2 = 13;
pub const PAETH_PRED: C2RustUnnamed_2 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_2 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_2 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_2 = 9;
pub const D67_PRED: C2RustUnnamed_2 = 8;
pub const D203_PRED: C2RustUnnamed_2 = 7;
pub const D157_PRED: C2RustUnnamed_2 = 6;
pub const D113_PRED: C2RustUnnamed_2 = 5;
pub const D135_PRED: C2RustUnnamed_2 = 4;
pub const D45_PRED: C2RustUnnamed_2 = 3;
pub const H_PRED: C2RustUnnamed_2 = 2;
pub const V_PRED: C2RustUnnamed_2 = 1;
pub const DC_PRED: C2RustUnnamed_2 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_3 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_3 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_3 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_3 = 0;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_4 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_4 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_4 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_4 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_4 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_4 = 0;
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
pub type SUBPEL_SEARCH_TYPE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const INTERP_SKIP_LUMA_SKIP_CHROMA: C2RustUnnamed_6 = 3;
pub const INTERP_EVAL_LUMA_SKIP_CHROMA: C2RustUnnamed_6 = 2;
pub const INTERP_SKIP_LUMA_EVAL_CHROMA: C2RustUnnamed_6 = 1;
pub const INTERP_EVAL_LUMA_EVAL_CHROMA: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const INTERP_PRED_TYPE_ALL: C2RustUnnamed_7 = 4;
pub const INTERP_HORZ_EQ_VERT_EQ: C2RustUnnamed_7 = 3;
pub const INTERP_HORZ_NEQ_VERT_EQ: C2RustUnnamed_7 = 2;
pub const INTERP_HORZ_EQ_VERT_NEQ: C2RustUnnamed_7 = 1;
pub const INTERP_HORZ_NEQ_VERT_NEQ: C2RustUnnamed_7 = 0;
pub type INTERP_PRED_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilters {
    pub y_filter: uint16_t,
    pub x_filter: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_interpfilters {
    pub as_int: uint32_t,
    pub as_filters: InterpFilters,
}
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
    pub c2rust_unnamed: C2RustUnnamed_18,
    pub c2rust_unnamed_0: C2RustUnnamed_16,
    pub c2rust_unnamed_1: C2RustUnnamed_14,
    pub c2rust_unnamed_2: C2RustUnnamed_12,
    pub c2rust_unnamed_3: C2RustUnnamed_10,
    pub c2rust_unnamed_4: C2RustUnnamed_8,
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
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_17,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub c2rust_unnamed: C2RustUnnamed_19,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_20 = 4;
pub const AFFINE: C2RustUnnamed_20 = 3;
pub const ROTZOOM: C2RustUnnamed_20 = 2;
pub const TRANSLATION: C2RustUnnamed_20 = 1;
pub const IDENTITY: C2RustUnnamed_20 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUFFER_SET {
    pub plane: [*mut uint8_t; 3],
    pub stride: [core::ffi::c_int; 3],
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
pub type MODE = uint8_t;
pub type SEARCH_METHODS = uint8_t;
pub type WARP_SEARCH_METHOD = uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERPOLATION_FILTER_STATS {
    pub filters: int_interpfilters,
    pub mv: [int_mv; 2],
    pub ref_frames: [int8_t; 2],
    pub comp_type: COMPOUND_TYPE,
    pub compound_idx: core::ffi::c_int,
    pub rd: int64_t,
    pub pred_sse: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HandleInterModeArgs {
    pub above_pred_buf: [*mut uint8_t; 3],
    pub above_pred_stride: [core::ffi::c_int; 3],
    pub left_pred_buf: [*mut uint8_t; 3],
    pub left_pred_stride: [core::ffi::c_int; 3],
    pub single_newmv: *mut [int_mv; 8],
    pub single_newmv_rate: *mut [core::ffi::c_int; 8],
    pub single_newmv_valid: *mut [core::ffi::c_int; 8],
    pub modelled_rd: *mut [[int64_t; 8]; 3],
    pub ref_frame_cost: core::ffi::c_int,
    pub single_comp_cost: core::ffi::c_int,
    pub simple_rd: *mut [[int64_t; 8]; 3],
    pub skip_motion_mode: core::ffi::c_int,
    pub skip_ifs: bool,
    pub inter_intra_mode: *mut INTERINTRA_MODE,
    pub interp_filter_stats: [INTERPOLATION_FILTER_STATS; 128],
    pub start_mv_stack: [FULLPEL_MV; 6],
    pub ref_mv_idx_stack: [uint8_t; 6],
    pub start_mv_cnt: core::ffi::c_int,
    pub interp_filter_stats_idx: core::ffi::c_int,
    pub wedge_index: core::ffi::c_int,
    pub wedge_sign: core::ffi::c_int,
    pub diffwtd_index: core::ffi::c_int,
    pub cmp_mode: [core::ffi::c_int; 29],
    pub best_single_sse_in_refs: [core::ffi::c_uint; 8],
    pub best_pred_sse: core::ffi::c_uint,
}
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
pub const MODELRD_LEGACY: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const MODELRD_TYPES: C2RustUnnamed_21 = 2;
pub const MODELRD_CURVFIT: C2RustUnnamed_21 = 1;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << SUBPEL_BITS)
    - 1 as core::ffi::c_int;
pub const SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int) << SUBPEL_BITS;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const ALLOW_ALL_INTERP_FILT_MASK: core::ffi::c_int = 0x1ff as core::ffi::c_int;
static mut av1_interp_dual_filt_mask: [[uint16_t; 3]; 2] = [
    [
        ((1 as core::ffi::c_int) << REG_REG as core::ffi::c_int
            | (1 as core::ffi::c_int) << SMOOTH_REG as core::ffi::c_int
            | (1 as core::ffi::c_int) << SHARP_REG as core::ffi::c_int) as uint16_t,
        ((1 as core::ffi::c_int) << REG_SMOOTH as core::ffi::c_int
            | (1 as core::ffi::c_int) << SMOOTH_SMOOTH as core::ffi::c_int
            | (1 as core::ffi::c_int) << SHARP_SMOOTH as core::ffi::c_int) as uint16_t,
        ((1 as core::ffi::c_int) << REG_SHARP as core::ffi::c_int
            | (1 as core::ffi::c_int) << SMOOTH_SHARP as core::ffi::c_int
            | (1 as core::ffi::c_int) << SHARP_SHARP as core::ffi::c_int) as uint16_t,
    ],
    [
        ((1 as core::ffi::c_int) << REG_REG as core::ffi::c_int
            | (1 as core::ffi::c_int) << REG_SMOOTH as core::ffi::c_int
            | (1 as core::ffi::c_int) << REG_SHARP as core::ffi::c_int) as uint16_t,
        ((1 as core::ffi::c_int) << SMOOTH_REG as core::ffi::c_int
            | (1 as core::ffi::c_int) << SMOOTH_SMOOTH as core::ffi::c_int
            | (1 as core::ffi::c_int) << SMOOTH_SHARP as core::ffi::c_int) as uint16_t,
        ((1 as core::ffi::c_int) << SHARP_REG as core::ffi::c_int
            | (1 as core::ffi::c_int) << SHARP_SMOOTH as core::ffi::c_int
            | (1 as core::ffi::c_int) << SHARP_SHARP as core::ffi::c_int) as uint16_t,
    ],
];
#[inline]
unsafe extern "C-unwind" fn reset_interp_filter_allowed_mask(
    mut allow_interp_mask: *mut uint16_t,
    mut filt_type: DUAL_FILTER_TYPE,
) {
    let mut tmp: uint16_t = (!((1 as core::ffi::c_int) << filt_type as core::ffi::c_int)
        & 0xffff as core::ffi::c_int) as uint16_t;
    *allow_interp_mask = (*allow_interp_mask as core::ffi::c_int
        & (tmp as core::ffi::c_int & ALLOW_ALL_INTERP_FILT_MASK)) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn set_interp_filter_allowed_mask(
    mut allow_interp_mask: *mut uint16_t,
    mut filt_type: DUAL_FILTER_TYPE,
) {
    *allow_interp_mask = (*allow_interp_mask as core::ffi::c_int
        | (1 as core::ffi::c_int) << filt_type as core::ffi::c_int) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn get_interp_filter_allowed_mask(
    mut allow_interp_mask: uint16_t,
    mut filt_type: DUAL_FILTER_TYPE,
) -> uint8_t {
    return (allow_interp_mask as core::ffi::c_int >> filt_type as core::ffi::c_int
        & 1 as core::ffi::c_int) as uint8_t;
}
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
static mut mi_size_wide_log2: [uint8_t; 22] = [
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
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
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
#[inline]
unsafe extern "C-unwind" fn clamp_mv(mut mv: *mut MV, mut mv_limits: *const SubpelMvLimits) {
    (*mv).col = clamp(
        (*mv).col as core::ffi::c_int,
        (*mv_limits).col_min,
        (*mv_limits).col_max,
    ) as int16_t;
    (*mv).row = clamp(
        (*mv).row as core::ffi::c_int,
        (*mv_limits).row_min,
        (*mv_limits).row_max,
    ) as int16_t;
}
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
unsafe extern "C-unwind" fn have_newmv_in_inter_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAREST_NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEW_NEARESTMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_intrabc_block(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (*mbmi).use_intrabc() as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
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
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SWITCHABLE_INTERP_RATE_FACTOR: core::ffi::c_int = 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_chessboard_index(
    mut frame_index: core::ffi::c_int,
) -> core::ffi::c_int {
    return frame_index & 0x1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_frame_update_type(
    mut gf_group: *const GF_GROUP,
    mut gf_frame_index: core::ffi::c_int,
) -> FRAME_UPDATE_TYPE {
    return (*gf_group).update_type[gf_frame_index as usize];
}
#[inline]
unsafe extern "C-unwind" fn restore_dst_buf(
    mut xd: *mut MACROBLOCKD,
    dst: BUFFER_SET,
    num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        (*xd).plane[i as usize].dst.buf = dst.plane[i as usize];
        (*xd).plane[i as usize].dst.stride = dst.stride[i as usize];
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn swap_dst_buf(
    mut xd: *mut MACROBLOCKD,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut num_planes: core::ffi::c_int,
) {
    let mut buf0: *const BUFFER_SET = *dst_bufs.offset(0 as core::ffi::c_int as isize);
    let fresh0 = &raw mut *dst_bufs.offset(0 as core::ffi::c_int as isize);
    *fresh0 = *dst_bufs.offset(1 as core::ffi::c_int as isize);
    let fresh1 = &raw mut *dst_bufs.offset(1 as core::ffi::c_int as isize);
    *fresh1 = buf0;
    restore_dst_buf(xd, **dst_bufs.offset(0 as core::ffi::c_int as isize), num_planes);
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
pub const MAX_INTERP_FILTER_STATS: core::ffi::c_int = 128 as core::ffi::c_int;
static mut filter_sets: [int_interpfilters; 9] = [
    int_interpfilters {
        as_int: 0 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x10000 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x20000 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x1 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x10001 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x20001 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x2 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x10002 as core::ffi::c_int as uint32_t,
    },
    int_interpfilters {
        as_int: 0x20002 as core::ffi::c_int as uint32_t,
    },
];
pub const MODELRD_TYPE_INTERP_FILTER: core::ffi::c_int = 1 as core::ffi::c_int;
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
#[inline]
unsafe extern "C-unwind" fn clamp_mv_to_umv_border_sb(
    mut xd: *const MACROBLOCKD,
    mut src_mv: *const MV,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
) -> MV {
    let spel_left: core::ffi::c_int = AOM_INTERP_EXTEND + bw << SUBPEL_BITS;
    let spel_right: core::ffi::c_int = spel_left - SUBPEL_SHIFTS;
    let spel_top: core::ffi::c_int = AOM_INTERP_EXTEND + bh << SUBPEL_BITS;
    let spel_bottom: core::ffi::c_int = spel_top - SUBPEL_SHIFTS;
    let mut clamped_mv: MV = {
        let mut init = mv {
            row: ((*src_mv).row as core::ffi::c_int
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_y)) as int16_t,
            col: ((*src_mv).col as core::ffi::c_int
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_x)) as int16_t,
        };
        init
    };
    let mv_limits: SubpelMvLimits = {
        let mut init = SubpelMvLimits {
            col_min: (*xd).mb_to_left_edge
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_x) - spel_left,
            col_max: (*xd).mb_to_right_edge
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_x) + spel_right,
            row_min: (*xd).mb_to_top_edge
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_y) - spel_top,
            row_max: (*xd).mb_to_bottom_edge
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ss_y) + spel_bottom,
        };
        init
    };
    clamp_mv(&raw mut clamped_mv, &mv_limits);
    return clamped_mv;
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
#[inline]
unsafe extern "C-unwind" fn is_interp_filter_good_match(
    mut st: *const INTERPOLATION_FILTER_STATS,
    mi: *mut MB_MODE_INFO,
    mut skip_level: core::ffi::c_int,
) -> core::ffi::c_int {
    let is_comp: core::ffi::c_int = has_second_ref(mi) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 1 as core::ffi::c_int + is_comp {
        if (*st).ref_frames[i as usize] as core::ffi::c_int
            != (*mi).ref_frame[i as usize] as core::ffi::c_int
        {
            return INT_MAX;
        }
        i += 1;
    }
    if skip_level == 1 as core::ffi::c_int && is_comp != 0 {
        if (*st).comp_type as core::ffi::c_int
            != (*mi).interinter_comp.type_0 as core::ffi::c_int
        {
            return INT_MAX;
        }
        if (*st).compound_idx != (*mi).compound_idx() as core::ffi::c_int {
            return INT_MAX;
        }
    }
    let mut mv_diff: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 1 as core::ffi::c_int + is_comp {
        mv_diff
            += abs(
                (*st).mv[i as usize].as_mv.row as core::ffi::c_int
                    - (*mi).mv[i as usize].as_mv.row as core::ffi::c_int,
            )
                + abs(
                    (*st).mv[i as usize].as_mv.col as core::ffi::c_int
                        - (*mi).mv[i as usize].as_mv.col as core::ffi::c_int,
                );
        i += 1;
    }
    return mv_diff;
}
#[inline]
unsafe extern "C-unwind" fn save_interp_filter_search_stat(
    mbmi: *mut MB_MODE_INFO,
    mut rd: int64_t,
    mut pred_sse: core::ffi::c_uint,
    mut interp_filter_stats: *mut INTERPOLATION_FILTER_STATS,
    mut interp_filter_stats_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    if interp_filter_stats_idx < MAX_INTERP_FILTER_STATS {
        let mut stat: INTERPOLATION_FILTER_STATS = {
            let mut init = INTERPOLATION_FILTER_STATS {
                filters: (*mbmi).interp_filters,
                mv: [
                    (*mbmi).mv[0 as core::ffi::c_int as usize],
                    (*mbmi).mv[1 as core::ffi::c_int as usize],
                ],
                ref_frames: [
                    (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
                    (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
                ],
                comp_type: (*mbmi).interinter_comp.type_0,
                compound_idx: (*mbmi).compound_idx() as core::ffi::c_int,
                rd: rd,
                pred_sse: pred_sse,
            };
            init
        };
        *interp_filter_stats.offset(interp_filter_stats_idx as isize) = stat;
        interp_filter_stats_idx += 1;
    }
    return interp_filter_stats_idx;
}
#[inline]
unsafe extern "C-unwind" fn find_interp_filter_in_stats(
    mbmi: *mut MB_MODE_INFO,
    mut interp_filter_stats: *mut INTERPOLATION_FILTER_STATS,
    mut interp_filter_stats_idx: core::ffi::c_int,
    mut skip_level: core::ffi::c_int,
) -> core::ffi::c_int {
    let thr: [[core::ffi::c_int; 2]; 2] = [
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
        [3 as core::ffi::c_int, 7 as core::ffi::c_int],
    ];
    let is_comp: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let mut best: core::ffi::c_int = INT_MAX;
    let mut match_0: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < interp_filter_stats_idx {
        let mut st: *const INTERPOLATION_FILTER_STATS = &raw mut *interp_filter_stats
            .offset(j as isize) as *mut INTERPOLATION_FILTER_STATS;
        let mv_diff: core::ffi::c_int = is_interp_filter_good_match(st, mbmi, skip_level)
            as core::ffi::c_int;
        if mv_diff == 0 as core::ffi::c_int {
            match_0 = j;
            break;
        } else {
            if mv_diff < best
                && mv_diff
                    <= thr[(skip_level - 1 as core::ffi::c_int)
                        as usize][is_comp as usize]
            {
                best = mv_diff;
                match_0 = j;
            }
            j += 1;
        }
    }
    if match_0 != -(1 as core::ffi::c_int) {
        (*mbmi).interp_filters = (*interp_filter_stats.offset(match_0 as isize)).filters;
        return match_0;
    }
    return -(1 as core::ffi::c_int);
}
unsafe extern "C-unwind" fn find_interp_filter_match(
    mbmi: *mut MB_MODE_INFO,
    cpi: *const AV1_COMP,
    assign_filter: InterpFilter,
    need_search: core::ffi::c_int,
    mut interp_filter_stats: *mut INTERPOLATION_FILTER_STATS,
    mut interp_filter_stats_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut match_found_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
    if (*cpi).sf.interp_sf.use_interp_filter != 0 && need_search != 0 {
        match_found_idx = find_interp_filter_in_stats(
            mbmi,
            interp_filter_stats,
            interp_filter_stats_idx,
            (*cpi).sf.interp_sf.use_interp_filter,
        );
    }
    if need_search == 0 || match_found_idx == -(1 as core::ffi::c_int) {
        set_default_interp_filters(mbmi, assign_filter);
    }
    return match_found_idx;
}
#[inline]
unsafe extern "C-unwind" fn get_switchable_rate(
    x: *mut MACROBLOCK,
    filters: int_interpfilters,
    mut ctx: *const core::ffi::c_int,
    mut dual_filter: core::ffi::c_int,
) -> core::ffi::c_int {
    let filter0: InterpFilter = filters.as_filters.y_filter as InterpFilter;
    let mut inter_filter_cost: core::ffi::c_int = (*x)
        .mode_costs
        .switchable_interp_costs[*ctx.offset(0 as core::ffi::c_int as isize)
        as usize][filter0 as usize];
    if dual_filter != 0 {
        let filter1: InterpFilter = filters.as_filters.x_filter as InterpFilter;
        inter_filter_cost
            += (*x)
                .mode_costs
                .switchable_interp_costs[*ctx.offset(1 as core::ffi::c_int as isize)
                as usize][filter1 as usize];
    }
    return SWITCHABLE_INTERP_RATE_FACTOR * inter_filter_cost;
}
#[inline]
unsafe extern "C-unwind" fn interp_model_rd_eval(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
    mut rd_stats: *mut RD_STATS,
    mut is_skip_build_pred: core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut tmp_rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_init_rd_stats(&raw mut tmp_rd_stats);
    if is_skip_build_pred == 0 {
        let mi_row: core::ffi::c_int = (*xd).mi_row;
        let mi_col: core::ffi::c_int = (*xd).mi_col;
        av1_enc_build_inter_predictor(
            cm,
            xd,
            mi_row,
            mi_col,
            orig_dst,
            bsize,
            plane_from,
            plane_to,
        );
    }
    (model_rd_sb_fn[(if (*cpi).sf.rt_sf.use_simple_rd_model != 0 {
        MODELRD_LEGACY as core::ffi::c_int
    } else {
        MODELRD_TYPE_INTERP_FILTER
    }) as usize])
        .expect(
            "non-null function pointer",
        )(
        cpi,
        bsize,
        x,
        xd,
        plane_from,
        plane_to,
        &raw mut tmp_rd_stats.rate,
        &raw mut tmp_rd_stats.dist,
        &raw mut tmp_rd_stats.skip_txfm,
        &raw mut tmp_rd_stats.sse,
        0 as *mut core::ffi::c_int,
        0 as *mut int64_t,
        0 as *mut int64_t,
    );
    av1_merge_rd_stats(rd_stats, &raw mut tmp_rd_stats);
}
#[inline]
unsafe extern "C-unwind" fn interpolation_filter_rd(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    mut rd_stats_luma: *mut RD_STATS,
    mut rd_stats: *mut RD_STATS,
    switchable_rate: *mut core::ffi::c_int,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut filter_idx: core::ffi::c_int,
    mut switchable_ctx: *const core::ffi::c_int,
    skip_pred: core::ffi::c_int,
) -> int64_t {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut interp_search_flags: *const InterpSearchFlags = &(*cpi).interp_search_flags;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut this_rd_stats_luma: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut this_rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_init_rd_stats(&raw mut this_rd_stats_luma);
    this_rd_stats = *rd_stats_luma;
    let last_best: int_interpfilters = (*mbmi).interp_filters;
    (*mbmi).interp_filters = filter_sets[filter_idx as usize];
    let is_sharp: core::ffi::c_int = ((*mbmi).interp_filters.as_filters.x_filter
        as core::ffi::c_int == MULTITAP_SHARP as core::ffi::c_int
        || (*mbmi).interp_filters.as_filters.y_filter as core::ffi::c_int
            == MULTITAP_SHARP as core::ffi::c_int) as core::ffi::c_int;
    let mul: core::ffi::c_int = if is_sharp != 0
        && (*cpi).sf.interp_sf.use_more_sharp_interp != 0
    {
        90 as core::ffi::c_int
    } else {
        100 as core::ffi::c_int
    };
    let tmp_rs: core::ffi::c_int = get_switchable_rate(
        x,
        (*mbmi).interp_filters,
        switchable_ctx,
        (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut min_rd: int64_t = (tmp_rs as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    if min_rd * mul as int64_t / 100 as int64_t > *rd {
        (*mbmi).interp_filters = last_best;
        return 0 as int64_t;
    }
    let mut tmp_skip_pred: core::ffi::c_int = if skip_pred
        == (*interp_search_flags).default_interp_skip_flags
    {
        INTERP_SKIP_LUMA_SKIP_CHROMA as core::ffi::c_int
    } else {
        skip_pred
    };
    if tmp_skip_pred == INTERP_SKIP_LUMA_SKIP_CHROMA as core::ffi::c_int {
        this_rd_stats = *rd_stats;
    } else {
        if tmp_skip_pred == INTERP_EVAL_LUMA_EVAL_CHROMA as core::ffi::c_int
            || tmp_skip_pred == INTERP_EVAL_LUMA_SKIP_CHROMA as core::ffi::c_int
        {
            interp_model_rd_eval(
                x,
                cpi,
                bsize,
                orig_dst,
                AOM_PLANE_Y,
                AOM_PLANE_Y,
                &raw mut this_rd_stats_luma,
                0 as core::ffi::c_int,
            );
            this_rd_stats = this_rd_stats_luma;
        }
        if tmp_skip_pred == INTERP_EVAL_LUMA_EVAL_CHROMA as core::ffi::c_int
            || tmp_skip_pred == INTERP_SKIP_LUMA_EVAL_CHROMA as core::ffi::c_int
        {
            let mut plane: core::ffi::c_int = 1 as core::ffi::c_int;
            while plane < num_planes {
                let mut tmp_rd: int64_t = ((tmp_rs + this_rd_stats.rate) as int64_t
                    * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + this_rd_stats.dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                if tmp_rd * mul as int64_t / 100 as int64_t >= *rd {
                    (*mbmi).interp_filters = last_best;
                    return 0 as int64_t;
                }
                interp_model_rd_eval(
                    x,
                    cpi,
                    bsize,
                    orig_dst,
                    plane,
                    plane,
                    &raw mut this_rd_stats,
                    0 as core::ffi::c_int,
                );
                plane += 1;
            }
        }
    }
    let mut tmp_rd_0: int64_t = ((tmp_rs + this_rd_stats.rate) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + this_rd_stats.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    if (tmp_rd_0 * mul as int64_t / 100 as int64_t) < *rd {
        *rd = tmp_rd_0;
        *switchable_rate = tmp_rs;
        if skip_pred != (*interp_search_flags).default_interp_skip_flags {
            if skip_pred == INTERP_EVAL_LUMA_EVAL_CHROMA as core::ffi::c_int {
                *rd_stats_luma = this_rd_stats_luma;
                *rd_stats = this_rd_stats;
                (*x).recalc_luma_mc_data = 0 as core::ffi::c_int;
            } else if skip_pred == INTERP_SKIP_LUMA_EVAL_CHROMA as core::ffi::c_int {
                *rd_stats = this_rd_stats;
                (*x).recalc_luma_mc_data ^= 1 as core::ffi::c_int;
            }
            swap_dst_buf(xd, dst_bufs, num_planes);
        }
        return 1 as int64_t;
    }
    (*mbmi).interp_filters = last_best;
    return 0 as int64_t;
}
#[inline]
unsafe extern "C-unwind" fn is_pred_filter_search_allowed(
    cpi: *const AV1_COMP,
    mut xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut af: *mut int_interpfilters,
    mut lf: *mut int_interpfilters,
) -> INTERP_PRED_TYPE {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let bsl: core::ffi::c_int = mi_size_wide_log2[bsize as usize] as core::ffi::c_int;
    let mut is_horiz_eq: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut is_vert_eq: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mbmi.is_null() && is_inter_block(above_mbmi) != 0 {
        *af = (*above_mbmi).interp_filters;
    }
    if !left_mbmi.is_null() && is_inter_block(left_mbmi) != 0 {
        *lf = (*left_mbmi).interp_filters;
    }
    if (*af).as_filters.x_filter as core::ffi::c_int
        != INTERP_INVALID as core::ffi::c_int
    {
        is_horiz_eq = ((*af).as_filters.x_filter as core::ffi::c_int
            == (*lf).as_filters.x_filter as core::ffi::c_int) as core::ffi::c_int;
    }
    if (*af).as_filters.y_filter as core::ffi::c_int
        != INTERP_INVALID as core::ffi::c_int
    {
        is_vert_eq = ((*af).as_filters.y_filter as core::ffi::c_int
            == (*lf).as_filters.y_filter as core::ffi::c_int) as core::ffi::c_int;
    }
    let mut pred_filter_type: INTERP_PRED_TYPE = ((is_vert_eq << 1 as core::ffi::c_int)
        + is_horiz_eq) as INTERP_PRED_TYPE;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut pred_filter_enable: core::ffi::c_int = if (*cpi)
        .sf
        .interp_sf
        .cb_pred_filter_search != 0
    {
        (mi_row + mi_col >> bsl)
            + get_chessboard_index((*cm).current_frame.frame_number as core::ffi::c_int)
            & 0x1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    pred_filter_enable &= (is_horiz_eq != 0 || is_vert_eq != 0) as core::ffi::c_int;
    return (pred_filter_enable * pred_filter_type as core::ffi::c_int)
        as INTERP_PRED_TYPE;
}
unsafe extern "C-unwind" fn find_best_interp_rd_facade(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    mut rd_stats_y: *mut RD_STATS,
    mut rd_stats: *mut RD_STATS,
    switchable_rate: *mut core::ffi::c_int,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut switchable_ctx: *const core::ffi::c_int,
    skip_pred: core::ffi::c_int,
    mut allow_interp_mask: uint16_t,
    mut is_w4_or_h4: core::ffi::c_int,
) -> DUAL_FILTER_TYPE {
    let mut tmp_skip_pred: core::ffi::c_int = skip_pred;
    let mut best_filt_type: DUAL_FILTER_TYPE = REG_REG as core::ffi::c_int
        as DUAL_FILTER_TYPE;
    if allow_interp_mask as core::ffi::c_int == 0 as core::ffi::c_int {
        return best_filt_type;
    }
    tmp_skip_pred = if is_w4_or_h4 != 0 {
        (*cpi).interp_search_flags.default_interp_skip_flags
    } else {
        skip_pred
    };
    let mut filt_type: core::ffi::c_int = SHARP_SHARP as core::ffi::c_int;
    while filt_type >= REG_REG as core::ffi::c_int {
        let is_filter_allowed: core::ffi::c_int = get_interp_filter_allowed_mask(
            allow_interp_mask,
            filt_type as DUAL_FILTER_TYPE,
        ) as core::ffi::c_int;
        if is_filter_allowed != 0 {
            if interpolation_filter_rd(
                x,
                cpi,
                tile_data,
                bsize,
                orig_dst,
                rd,
                rd_stats_y,
                rd_stats,
                switchable_rate,
                dst_bufs,
                filt_type,
                switchable_ctx,
                tmp_skip_pred,
            ) != 0
            {
                best_filt_type = filt_type as DUAL_FILTER_TYPE;
            }
        }
        tmp_skip_pred = skip_pred;
        filt_type -= 1;
    }
    return best_filt_type;
}
#[inline]
unsafe extern "C-unwind" fn pred_dual_interp_filter_rd(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    mut rd_stats_y: *mut RD_STATS,
    mut rd_stats: *mut RD_STATS,
    switchable_rate: *mut core::ffi::c_int,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut switchable_ctx: *const core::ffi::c_int,
    skip_pred: core::ffi::c_int,
    mut pred_filt_type: INTERP_PRED_TYPE,
    mut af: *mut int_interpfilters,
    mut lf: *mut int_interpfilters,
) {
    let mut allowed_interp_mask: uint16_t = 0 as uint16_t;
    if pred_filt_type as core::ffi::c_int == INTERP_HORZ_EQ_VERT_NEQ as core::ffi::c_int
    {
        allowed_interp_mask = av1_interp_dual_filt_mask[(pred_filt_type
            as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize][(*af).as_filters.x_filter as usize];
    } else if pred_filt_type as core::ffi::c_int
        == INTERP_HORZ_NEQ_VERT_EQ as core::ffi::c_int
    {
        allowed_interp_mask = av1_interp_dual_filt_mask[(pred_filt_type
            as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize][(*af).as_filters.y_filter as usize];
    } else {
        let mut filt_type: core::ffi::c_int = (*af).as_filters.x_filter
            as core::ffi::c_int
            + (*af).as_filters.y_filter as core::ffi::c_int
                * SWITCHABLE_FILTERS as core::ffi::c_int;
        set_interp_filter_allowed_mask(
            &raw mut allowed_interp_mask,
            filt_type as DUAL_FILTER_TYPE,
        );
    }
    reset_interp_filter_allowed_mask(
        &raw mut allowed_interp_mask,
        REG_REG as core::ffi::c_int as DUAL_FILTER_TYPE,
    );
    find_best_interp_rd_facade(
        x,
        cpi,
        tile_data,
        bsize,
        orig_dst,
        rd,
        rd_stats_y,
        rd_stats,
        switchable_rate,
        dst_bufs,
        switchable_ctx,
        skip_pred,
        allowed_interp_mask,
        0 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn fast_dual_interp_filter_rd(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    mut rd_stats_y: *mut RD_STATS,
    mut rd_stats: *mut RD_STATS,
    switchable_rate: *mut core::ffi::c_int,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut switchable_ctx: *const core::ffi::c_int,
    skip_hor: core::ffi::c_int,
    skip_ver: core::ffi::c_int,
) {
    let mut interp_search_flags: *const InterpSearchFlags = &(*cpi).interp_search_flags;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut pred_filter_type: INTERP_PRED_TYPE = INTERP_HORZ_NEQ_VERT_NEQ
        as core::ffi::c_int as INTERP_PRED_TYPE;
    let mut af: int_interpfilters = av1_broadcast_interp_filter(INTERP_INVALID);
    let mut lf: int_interpfilters = af;
    if have_newmv_in_inter_mode((*mbmi).mode) == 0 {
        pred_filter_type = is_pred_filter_search_allowed(
            cpi,
            xd,
            bsize,
            &raw mut af,
            &raw mut lf,
        );
    }
    if pred_filter_type != 0 {
        pred_dual_interp_filter_rd(
            x,
            cpi,
            tile_data,
            bsize,
            orig_dst,
            rd,
            rd_stats_y,
            rd_stats,
            switchable_rate,
            dst_bufs,
            switchable_ctx,
            skip_hor & skip_ver,
            pred_filter_type,
            &raw mut af,
            &raw mut lf,
        );
    } else {
        let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
        let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
        let mut best_dual_mode: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut skip_pred: core::ffi::c_int = if bw <= 4 as core::ffi::c_int {
            (*interp_search_flags).default_interp_skip_flags
        } else {
            skip_hor
        };
        let mut i: core::ffi::c_int = SWITCHABLE_FILTERS as core::ffi::c_int
            - 1 as core::ffi::c_int;
        while i >= 1 as core::ffi::c_int {
            if interpolation_filter_rd(
                x,
                cpi,
                tile_data,
                bsize,
                orig_dst,
                rd,
                rd_stats_y,
                rd_stats,
                switchable_rate,
                dst_bufs,
                i,
                switchable_ctx,
                skip_pred,
            ) != 0
            {
                best_dual_mode = i;
            }
            skip_pred = skip_hor;
            i -= 1;
        }
        skip_pred = if bh <= 4 as core::ffi::c_int {
            (*interp_search_flags).default_interp_skip_flags
        } else {
            skip_ver
        };
        let mut i_0: core::ffi::c_int = best_dual_mode
            + SWITCHABLE_FILTERS as core::ffi::c_int * 2 as core::ffi::c_int;
        while i_0 >= best_dual_mode + SWITCHABLE_FILTERS as core::ffi::c_int {
            interpolation_filter_rd(
                x,
                cpi,
                tile_data,
                bsize,
                orig_dst,
                rd,
                rd_stats_y,
                rd_stats,
                switchable_rate,
                dst_bufs,
                i_0,
                switchable_ctx,
                skip_pred,
            );
            skip_pred = skip_ver;
            i_0 -= SWITCHABLE_FILTERS as core::ffi::c_int;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn find_best_non_dual_interp_filter(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    mut rd_stats_y: *mut RD_STATS,
    mut rd_stats: *mut RD_STATS,
    switchable_rate: *mut core::ffi::c_int,
    mut dst_bufs: *mut *const BUFFER_SET,
    mut switchable_ctx: *const core::ffi::c_int,
    skip_ver: core::ffi::c_int,
    skip_hor: core::ffi::c_int,
) {
    let mut interp_search_flags: *const InterpSearchFlags = &(*cpi).interp_search_flags;
    let mut i: int8_t = 0;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut interp_filter_search_mask: uint16_t = (*interp_search_flags)
        .interp_filter_search_mask;
    if (*cpi).sf.interp_sf.adaptive_interp_filter_search == 2 as core::ffi::c_int {
        let update_type: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        let ctx0: core::ffi::c_int = av1_get_pred_context_switchable_interp(
            xd,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let ctx1: core::ffi::c_int = av1_get_pred_context_switchable_interp(
            xd,
            1 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let mut use_actual_frame_probs: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut switchable_interp_p0: *const core::ffi::c_int = 0
            as *const core::ffi::c_int;
        let mut switchable_interp_p1: *const core::ffi::c_int = 0
            as *const core::ffi::c_int;
        if use_actual_frame_probs != 0 {
            switchable_interp_p0 = ((*(*cpi).ppi)
                .frame_probs
                .switchable_interp_probs[update_type as usize][ctx0 as usize])
                .as_mut_ptr();
            switchable_interp_p1 = ((*(*cpi).ppi)
                .frame_probs
                .switchable_interp_probs[update_type as usize][ctx1 as usize])
                .as_mut_ptr();
        }
        static mut thr: [core::ffi::c_int; 7] = [
            0 as core::ffi::c_int,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
            0 as core::ffi::c_int,
            8 as core::ffi::c_int,
        ];
        let thresh: core::ffi::c_int = thr[update_type as usize];
        i = 0 as int8_t;
        while (i as core::ffi::c_int) < SWITCHABLE_FILTERS as core::ffi::c_int {
            if *switchable_interp_p0.offset(i as isize) < thresh
                && *switchable_interp_p1.offset(i as isize) < thresh
            {
                let mut filt_type: DUAL_FILTER_TYPE = (i as core::ffi::c_int
                    + SWITCHABLE_FILTERS as core::ffi::c_int * i as core::ffi::c_int)
                    as DUAL_FILTER_TYPE;
                reset_interp_filter_allowed_mask(
                    &raw mut interp_filter_search_mask,
                    filt_type,
                );
            }
            if (*cpi).oxcf.algo_cfg.sharpness == 3 as core::ffi::c_int
                && i as core::ffi::c_int == EIGHTTAP_SMOOTH as core::ffi::c_int
            {
                let mut filt_type_0: DUAL_FILTER_TYPE = (i as core::ffi::c_int
                    + SWITCHABLE_FILTERS as core::ffi::c_int * i as core::ffi::c_int)
                    as DUAL_FILTER_TYPE;
                reset_interp_filter_allowed_mask(
                    &raw mut interp_filter_search_mask,
                    filt_type_0,
                );
            }
            i += 1;
        }
    }
    if skip_hor & skip_ver != (*interp_search_flags).default_interp_skip_flags {
        let mut pred_filter_type: INTERP_PRED_TYPE = INTERP_HORZ_NEQ_VERT_NEQ
            as core::ffi::c_int as INTERP_PRED_TYPE;
        let mut af: int_interpfilters = av1_broadcast_interp_filter(INTERP_INVALID);
        let mut lf: int_interpfilters = af;
        pred_filter_type = is_pred_filter_search_allowed(
            cpi,
            xd,
            bsize,
            &raw mut af,
            &raw mut lf,
        );
        if pred_filter_type != 0 {
            let mut filter_idx: core::ffi::c_int = SWITCHABLE as core::ffi::c_int
                * af.as_filters.x_filter as core::ffi::c_int;
            if (*cpi).sf.interp_sf.adaptive_interp_filter_search != 0
                && get_interp_filter_allowed_mask(
                    interp_filter_search_mask,
                    filter_idx as DUAL_FILTER_TYPE,
                ) == 0
            {
                return;
            }
            if filter_idx != 0 {
                interpolation_filter_rd(
                    x,
                    cpi,
                    tile_data,
                    bsize,
                    orig_dst,
                    rd,
                    rd_stats_y,
                    rd_stats,
                    switchable_rate,
                    dst_bufs,
                    filter_idx,
                    switchable_ctx,
                    skip_hor & skip_ver,
                );
            }
            return;
        }
    }
    if bsize as core::ffi::c_int == BLOCK_4X4 as core::ffi::c_int
        || block_size_wide[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
            && skip_ver == (*interp_search_flags).default_interp_skip_flags
        || block_size_high[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
            && skip_hor == (*interp_search_flags).default_interp_skip_flags
    {
        let mut skip_pred: core::ffi::c_int = skip_hor & skip_ver;
        let mut allowed_interp_mask: uint16_t = 0 as uint16_t;
        set_interp_filter_allowed_mask(
            &raw mut allowed_interp_mask,
            SHARP_SHARP as core::ffi::c_int as DUAL_FILTER_TYPE,
        );
        set_interp_filter_allowed_mask(
            &raw mut allowed_interp_mask,
            SMOOTH_SMOOTH as core::ffi::c_int as DUAL_FILTER_TYPE,
        );
        if (*cpi).sf.interp_sf.adaptive_interp_filter_search != 0 {
            allowed_interp_mask = (allowed_interp_mask as core::ffi::c_int
                & interp_filter_search_mask as core::ffi::c_int) as uint16_t;
        }
        find_best_interp_rd_facade(
            x,
            cpi,
            tile_data,
            bsize,
            orig_dst,
            rd,
            rd_stats_y,
            rd_stats,
            switchable_rate,
            dst_bufs,
            switchable_ctx,
            skip_pred,
            allowed_interp_mask,
            1 as core::ffi::c_int,
        );
    } else {
        let mut skip_pred_0: core::ffi::c_int = skip_hor & skip_ver;
        i = (SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int) as int8_t;
        while (i as core::ffi::c_int)
            < SWITCHABLE_FILTERS as core::ffi::c_int
                * SWITCHABLE_FILTERS as core::ffi::c_int
        {
            if !((*cpi).sf.interp_sf.adaptive_interp_filter_search != 0
                && get_interp_filter_allowed_mask(
                    interp_filter_search_mask,
                    i as DUAL_FILTER_TYPE,
                ) == 0)
            {
                interpolation_filter_rd(
                    x,
                    cpi,
                    tile_data,
                    bsize,
                    orig_dst,
                    rd,
                    rd_stats_y,
                    rd_stats,
                    switchable_rate,
                    dst_bufs,
                    i as core::ffi::c_int,
                    switchable_ctx,
                    skip_pred_0,
                );
                if (*cpi).sf.interp_sf.skip_sharp_interp_filter_search != 0
                    && skip_pred_0 != (*interp_search_flags).default_interp_skip_flags
                {
                    if (*mbmi).interp_filters.as_int
                        == filter_sets[SMOOTH_SMOOTH as core::ffi::c_int as usize].as_int
                    {
                        break;
                    }
                }
            }
            i = (i as core::ffi::c_int
                + (SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int))
                as int8_t;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn calc_interp_skip_pred_flag(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut skip_hor: *mut core::ffi::c_int,
    mut skip_ver: *mut core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + is_compound {
        let sf: *const scale_factors = get_ref_scale_factors_const(
            cm,
            (*mbmi).ref_frame[ref_0 as usize],
        ) as *const scale_factors;
        if av1_is_scaled(sf) != 0 {
            *skip_hor = 0 as core::ffi::c_int;
            *skip_ver = 0 as core::ffi::c_int;
            break;
        } else {
            let mv: MV = (*mbmi).mv[ref_0 as usize].as_mv;
            let mut skip_hor_plane: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut skip_ver_plane: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut plane_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane_idx
                < (if 1 as core::ffi::c_int > num_planes - 1 as core::ffi::c_int {
                    1 as core::ffi::c_int
                } else {
                    num_planes - 1 as core::ffi::c_int
                })
            {
                let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
                    .as_mut_ptr()
                    .offset(plane_idx as isize) as *mut macroblockd_plane;
                let bw: core::ffi::c_int = (*pd).width as core::ffi::c_int;
                let bh: core::ffi::c_int = (*pd).height as core::ffi::c_int;
                let mv_q4: MV = clamp_mv_to_umv_border_sb(
                    xd,
                    &mv,
                    bw,
                    bh,
                    (*pd).subsampling_x,
                    (*pd).subsampling_y,
                ) as MV;
                let sub_x: core::ffi::c_int = (mv_q4.col as core::ffi::c_int
                    & SUBPEL_MASK) << SCALE_EXTRA_BITS;
                let sub_y: core::ffi::c_int = (mv_q4.row as core::ffi::c_int
                    & SUBPEL_MASK) << SCALE_EXTRA_BITS;
                skip_hor_plane
                    |= ((sub_x == 0 as core::ffi::c_int) as core::ffi::c_int)
                        << plane_idx;
                skip_ver_plane
                    |= ((sub_y == 0 as core::ffi::c_int) as core::ffi::c_int)
                        << plane_idx;
                plane_idx += 1;
            }
            *skip_hor &= skip_hor_plane;
            *skip_ver &= skip_ver_plane;
            ref_0 += 1;
        }
    }
    if is_compound != 0
        && (*mbmi).compound_idx() as core::ffi::c_int == 1 as core::ffi::c_int
        && (*mbmi).interinter_comp.type_0 as core::ffi::c_int
            == COMPOUND_DIFFWTD as core::ffi::c_int
    {
        if *skip_hor == 0 as core::ffi::c_int && *skip_ver == 1 as core::ffi::c_int {
            *skip_ver = 0 as core::ffi::c_int;
        }
    }
    if (*cpi).sf.interp_sf.skip_model_rd_uv != 0 && num_planes > 1 as core::ffi::c_int {
        *skip_hor |= INTERP_EVAL_LUMA_SKIP_CHROMA as core::ffi::c_int;
        *skip_ver |= INTERP_EVAL_LUMA_SKIP_CHROMA as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_interpolation_filter_search(
    x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut tile_data: *const TileDataEnc,
    mut bsize: BLOCK_SIZE,
    tmp_dst: *const BUFFER_SET,
    orig_dst: *const BUFFER_SET,
    rd: *mut int64_t,
    switchable_rate: *mut core::ffi::c_int,
    mut skip_build_pred: *mut core::ffi::c_int,
    mut args: *mut HandleInterModeArgs,
    mut ref_best_rd: int64_t,
) -> int64_t {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut interp_search_flags: *const InterpSearchFlags = &(*cpi).interp_search_flags;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let need_search: core::ffi::c_int = av1_is_interp_needed(xd) as core::ffi::c_int;
    let ref_frame: core::ffi::c_int = (**((*xd).mi)
        .offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int;
    let skip_model_rd_uv: core::ffi::c_int = (*cpi).sf.interp_sf.skip_model_rd_uv;
    let mut rd_stats_luma: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut rd_stats: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_init_rd_stats(&raw mut rd_stats_luma);
    av1_init_rd_stats(&raw mut rd_stats);
    let mut match_found_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
    let assign_filter: InterpFilter = (*cm).features.interp_filter;
    match_found_idx = find_interp_filter_match(
        mbmi,
        cpi,
        assign_filter,
        need_search,
        ((*args).interp_filter_stats).as_mut_ptr(),
        (*args).interp_filter_stats_idx,
    );
    if match_found_idx != -(1 as core::ffi::c_int) {
        *rd = (*args).interp_filter_stats[match_found_idx as usize].rd;
        (*x).pred_sse[ref_frame as usize] = (*args)
            .interp_filter_stats[match_found_idx as usize]
            .pred_sse;
        *skip_build_pred = INTERP_EVAL_LUMA_EVAL_CHROMA as core::ffi::c_int;
        return 0 as int64_t;
    }
    let mut switchable_ctx: [core::ffi::c_int; 2] = [0; 2];
    switchable_ctx[0 as core::ffi::c_int as usize] = av1_get_pred_context_switchable_interp(
        xd,
        0 as core::ffi::c_int,
    );
    switchable_ctx[1 as core::ffi::c_int as usize] = av1_get_pred_context_switchable_interp(
        xd,
        1 as core::ffi::c_int,
    );
    *switchable_rate = get_switchable_rate(
        x,
        (*mbmi).interp_filters,
        switchable_ctx.as_mut_ptr() as *const core::ffi::c_int,
        (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int,
    );
    interp_model_rd_eval(
        x,
        cpi,
        bsize,
        orig_dst,
        AOM_PLANE_Y,
        AOM_PLANE_Y,
        &raw mut rd_stats_luma,
        *skip_build_pred,
    );
    if num_planes > 1 as core::ffi::c_int && skip_model_rd_uv == 0 {
        interp_model_rd_eval(
            x,
            cpi,
            bsize,
            orig_dst,
            AOM_PLANE_U,
            AOM_PLANE_V,
            &raw mut rd_stats,
            *skip_build_pred,
        );
    }
    *skip_build_pred = if num_planes > 1 as core::ffi::c_int && skip_model_rd_uv != 0 {
        INTERP_SKIP_LUMA_EVAL_CHROMA as core::ffi::c_int
    } else {
        INTERP_SKIP_LUMA_SKIP_CHROMA as core::ffi::c_int
    };
    av1_merge_rd_stats(&raw mut rd_stats, &raw mut rd_stats_luma);
    *rd = ((*switchable_rate + rd_stats.rate) as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + rd_stats.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    (*x).pred_sse[ref_frame as usize] = (rd_stats_luma.sse >> 4 as core::ffi::c_int)
        as core::ffi::c_uint;
    if assign_filter as core::ffi::c_int != SWITCHABLE as core::ffi::c_int
        || match_found_idx != -(1 as core::ffi::c_int)
    {
        return 0 as int64_t;
    }
    if need_search == 0 {
        let mut filters: int_interpfilters = av1_broadcast_interp_filter(
            EIGHTTAP_REGULAR,
        );
        return 0 as int64_t;
    }
    if !((*args).modelled_rd).is_null() {
        if has_second_ref(mbmi) != 0 {
            let ref_mv_idx: core::ffi::c_int = (*mbmi).ref_mv_idx() as core::ffi::c_int;
            let mut refs: *mut MV_REFERENCE_FRAME = ((*mbmi).ref_frame).as_mut_ptr();
            let mode0: core::ffi::c_int = compound_ref0_mode((*mbmi).mode)
                as core::ffi::c_int;
            let mode1: core::ffi::c_int = compound_ref1_mode((*mbmi).mode)
                as core::ffi::c_int;
            let mrd: int64_t = if (*((*args).modelled_rd)
                .offset(
                    mode0 as isize,
                ))[ref_mv_idx
                as usize][*refs.offset(0 as core::ffi::c_int as isize) as usize]
                < (*((*args).modelled_rd)
                    .offset(
                        mode1 as isize,
                    ))[ref_mv_idx
                    as usize][*refs.offset(1 as core::ffi::c_int as isize) as usize]
            {
                (*((*args).modelled_rd)
                    .offset(
                        mode0 as isize,
                    ))[ref_mv_idx
                    as usize][*refs.offset(0 as core::ffi::c_int as isize) as usize]
            } else {
                (*((*args).modelled_rd)
                    .offset(
                        mode1 as isize,
                    ))[ref_mv_idx
                    as usize][*refs.offset(1 as core::ffi::c_int as isize) as usize]
            };
            if *rd >> 1 as core::ffi::c_int > mrd && ref_best_rd < INT64_MAX as int64_t {
                return INT64_MAX as int64_t;
            }
        }
    }
    (*x).recalc_luma_mc_data = 0 as core::ffi::c_int;
    let mut skip_hor: core::ffi::c_int = (*interp_search_flags)
        .default_interp_skip_flags;
    let mut skip_ver: core::ffi::c_int = (*interp_search_flags)
        .default_interp_skip_flags;
    calc_interp_skip_pred_flag(x, cpi, &raw mut skip_hor, &raw mut skip_ver);
    restore_dst_buf(xd, *tmp_dst, num_planes);
    let mut dst_bufs: [*const BUFFER_SET; 2] = [tmp_dst, orig_dst];
    if (*(*cm).seq_params).enable_dual_filter != 0 {
        if (*cpi).sf.interp_sf.use_fast_interpolation_filter_search != 0 {
            fast_dual_interp_filter_rd(
                x,
                cpi,
                tile_data,
                bsize,
                orig_dst,
                rd,
                &raw mut rd_stats_luma,
                &raw mut rd_stats,
                switchable_rate,
                dst_bufs.as_mut_ptr(),
                switchable_ctx.as_mut_ptr() as *const core::ffi::c_int,
                skip_hor,
                skip_ver,
            );
        } else {
            let mut allowed_interp_mask: uint16_t = ALLOW_ALL_INTERP_FILT_MASK
                as uint16_t;
            reset_interp_filter_allowed_mask(
                &raw mut allowed_interp_mask,
                REG_REG as core::ffi::c_int as DUAL_FILTER_TYPE,
            );
            find_best_interp_rd_facade(
                x,
                cpi,
                tile_data,
                bsize,
                orig_dst,
                rd,
                &raw mut rd_stats_luma,
                &raw mut rd_stats,
                switchable_rate,
                dst_bufs.as_mut_ptr(),
                switchable_ctx.as_mut_ptr() as *const core::ffi::c_int,
                skip_hor & skip_ver,
                allowed_interp_mask,
                0 as core::ffi::c_int,
            );
        }
    } else {
        find_best_non_dual_interp_filter(
            x,
            cpi,
            tile_data,
            bsize,
            orig_dst,
            rd,
            &raw mut rd_stats_luma,
            &raw mut rd_stats,
            switchable_rate,
            dst_bufs.as_mut_ptr(),
            switchable_ctx.as_mut_ptr() as *const core::ffi::c_int,
            skip_ver,
            skip_hor,
        );
    }
    swap_dst_buf(xd, dst_bufs.as_mut_ptr(), num_planes);
    if (*x).recalc_luma_mc_data == 1 as core::ffi::c_int {
        let mi_row: core::ffi::c_int = (*xd).mi_row;
        let mi_col: core::ffi::c_int = (*xd).mi_col;
        av1_enc_build_inter_predictor(
            cm,
            xd,
            mi_row,
            mi_col,
            orig_dst,
            bsize,
            AOM_PLANE_Y,
            AOM_PLANE_Y,
        );
    }
    (*x).pred_sse[ref_frame as usize] = (rd_stats_luma.sse >> 4 as core::ffi::c_int)
        as core::ffi::c_uint;
    if (*cpi).sf.interp_sf.use_interp_filter != 0 {
        (*args).interp_filter_stats_idx = save_interp_filter_search_stat(
            mbmi,
            *rd,
            (*x).pred_sse[ref_frame as usize],
            ((*args).interp_filter_stats).as_mut_ptr(),
            (*args).interp_filter_stats_idx,
        );
    }
    return 0 as int64_t;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn log2(_: core::ffi::c_double) -> core::ffi::c_double;
}
