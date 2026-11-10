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

// Mechanically translated from upstream/aom/av1/encoder/encodeframe_utils.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
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
    fn av1_get_qindex(
        seg: *const segmentation,
        segment_id: core::ffi::c_int,
        base_qindex: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_left_block_mode(left_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_above_block_mode(above_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_copy_frame_mvs(
        cm: *const AV1_COMMON,
        mi: *const MB_MODE_INFO,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        x_mis: core::ffi::c_int,
        y_mis: core::ffi::c_int,
    );
    fn av1_cyclic_refresh_update_segment(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        rate: int64_t,
        dist: int64_t,
        skip: core::ffi::c_int,
        dry_run: RUN_TYPE,
    );
    fn av1_init_plane_quantizers(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        segment_id: core::ffi::c_int,
        do_update: core::ffi::c_int,
    );
    fn av1_compute_rd_mult(
        qindex: core::ffi::c_int,
        bit_depth: aom_bit_depth_t,
        update_type: FRAME_UPDATE_TYPE,
        layer_depth: core::ffi::c_int,
        boost_index: core::ffi::c_int,
        frame_type: FRAME_TYPE,
        use_fixed_qp_offsets: core::ffi::c_int,
        is_stat_consumption_stage_0: core::ffi::c_int,
        tuning: aom_tune_metric,
        mode: MODE,
    ) -> core::ffi::c_int;
    fn av1_fill_mode_rates(
        cm: *mut AV1_COMMON,
        mode_costs: *mut ModeCosts,
        fc: *mut FRAME_CONTEXT,
    );
    fn av1_fill_coeff_costs(
        coeff_costs: *mut CoeffCosts,
        fc: *mut FRAME_CONTEXT,
        num_planes: core::ffi::c_int,
    );
    fn av1_fill_mv_costs(
        nmvc: *const nmv_context,
        integer_mv: core::ffi::c_int,
        usehp: core::ffi::c_int,
        mv_costs: *mut MvCosts,
    );
    fn av1_fill_dv_costs(ndvc: *const nmv_context, dv_costs: *mut IntraBCMVCosts);
    fn av1_get_adaptive_rdmult(
        cpi: *const AV1_COMP,
        beta: core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn av1_get_deltaq_offset(
        bit_depth: aom_bit_depth_t,
        qindex: core::ffi::c_int,
        beta: core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn av1_tpl_ptr_pos(
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        stride: core::ffi::c_int,
        right_shift: uint8_t,
    ) -> core::ffi::c_int;
    fn av1_tpl_stats_ready(
        tpl_data: *const TplParams,
        gf_frame_index: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_neg_interleave(
        x: core::ffi::c_int,
        ref_0: core::ffi::c_int,
        max: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_pred_context_switchable_interp(
        xd: *const MACROBLOCKD,
        dir: core::ffi::c_int,
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
pub type jmp_buf = [core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_1 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_1 = 15;
pub const TX_TYPES: C2RustUnnamed_1 = 16;
pub const H_FLIPADST: C2RustUnnamed_1 = 15;
pub const V_FLIPADST: C2RustUnnamed_1 = 14;
pub const H_ADST: C2RustUnnamed_1 = 13;
pub const V_ADST: C2RustUnnamed_1 = 12;
pub const H_DCT: C2RustUnnamed_1 = 11;
pub const V_DCT: C2RustUnnamed_1 = 10;
pub const IDTX: C2RustUnnamed_1 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_1 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_1 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_1 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_1 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_1 = 4;
pub const ADST_ADST: C2RustUnnamed_1 = 3;
pub const DCT_ADST: C2RustUnnamed_1 = 2;
pub const ADST_DCT: C2RustUnnamed_1 = 1;
pub const DCT_DCT: C2RustUnnamed_1 = 0;
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
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const PARTITION_INVALID: C2RustUnnamed_2 = 255;
pub const PARTITION_TYPES: C2RustUnnamed_2 = 4;
pub const EXT_PARTITION_TYPES: C2RustUnnamed_2 = 10;
pub const PARTITION_VERT_4: C2RustUnnamed_2 = 9;
pub const PARTITION_HORZ_4: C2RustUnnamed_2 = 8;
pub const PARTITION_VERT_B: C2RustUnnamed_2 = 7;
pub const PARTITION_VERT_A: C2RustUnnamed_2 = 6;
pub const PARTITION_HORZ_B: C2RustUnnamed_2 = 5;
pub const PARTITION_HORZ_A: C2RustUnnamed_2 = 4;
pub const PARTITION_SPLIT: C2RustUnnamed_2 = 3;
pub const PARTITION_VERT: C2RustUnnamed_2 = 2;
pub const PARTITION_HORZ: C2RustUnnamed_2 = 1;
pub const PARTITION_NONE: C2RustUnnamed_2 = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
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
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const PALETTE_SIZES: C2RustUnnamed_5 = 7;
pub const EIGHT_COLORS: C2RustUnnamed_5 = 6;
pub const SEVEN_COLORS: C2RustUnnamed_5 = 5;
pub const SIX_COLORS: C2RustUnnamed_5 = 4;
pub const FIVE_COLORS: C2RustUnnamed_5 = 3;
pub const FOUR_COLORS: C2RustUnnamed_5 = 2;
pub const THREE_COLORS: C2RustUnnamed_5 = 1;
pub const TWO_COLORS: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const PALETTE_COLORS: C2RustUnnamed_6 = 8;
pub const PALETTE_COLOR_EIGHT: C2RustUnnamed_6 = 7;
pub const PALETTE_COLOR_SEVEN: C2RustUnnamed_6 = 6;
pub const PALETTE_COLOR_SIX: C2RustUnnamed_6 = 5;
pub const PALETTE_COLOR_FIVE: C2RustUnnamed_6 = 4;
pub const PALETTE_COLOR_FOUR: C2RustUnnamed_6 = 3;
pub const PALETTE_COLOR_THREE: C2RustUnnamed_6 = 2;
pub const PALETTE_COLOR_TWO: C2RustUnnamed_6 = 1;
pub const PALETTE_COLOR_ONE: C2RustUnnamed_6 = 0;
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
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
pub type float_t = core::ffi::c_float;
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
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_27 = 4;
pub const S_FRAME: C2RustUnnamed_27 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_27 = 2;
pub const INTER_FRAME: C2RustUnnamed_27 = 1;
pub const KEY_FRAME: C2RustUnnamed_27 = 0;
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
pub struct SB_FIRST_PASS_STATS {
    pub x_ctx: RD_SEARCH_MACROBLOCK_CONTEXT,
    pub rd_count: RD_COUNTS,
    pub split_count: core::ffi::c_int,
    pub fc: FRAME_COUNTS,
    pub inter_mode_rd_models: [InterModeRdModel; 22],
    pub thresh_freq_fact: [[core::ffi::c_int; 169]; 22],
    pub current_qindex: core::ffi::c_int,
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
pub struct FRAME_COUNTS {
    pub switchable_interp: [[core::ffi::c_uint; 3]; 16],
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
pub struct RD_SEARCH_MACROBLOCK_CONTEXT {
    pub a: [ENTROPY_CONTEXT; 96],
    pub l: [ENTROPY_CONTEXT; 96],
    pub sa: [PARTITION_CONTEXT; 32],
    pub sl: [PARTITION_CONTEXT; 32],
    pub p_ta: *mut TXFM_CONTEXT,
    pub p_tl: *mut TXFM_CONTEXT,
    pub ta: [TXFM_CONTEXT; 32],
    pub tl: [TXFM_CONTEXT; 32],
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
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_28 = 2;
pub const REALTIME: C2RustUnnamed_28 = 1;
pub const GOOD: C2RustUnnamed_28 = 0;
pub type MODE = uint8_t;
pub type SEARCH_METHODS = uint8_t;
pub type WARP_SEARCH_METHOD = uint8_t;
pub type MACROBLOCK = macroblock;
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
pub type RUN_TYPE = uint8_t;
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_29 = 2;
pub const LAP_STAGE: C2RustUnnamed_29 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_29 = 0;
pub type C2RustUnnamed_30 = core::ffi::c_uint;
pub const FRAME_UPDATE_TYPES: C2RustUnnamed_30 = 7;
pub const INTNL_ARF_UPDATE: C2RustUnnamed_30 = 6;
pub const INTNL_OVERLAY_UPDATE: C2RustUnnamed_30 = 5;
pub const OVERLAY_UPDATE: C2RustUnnamed_30 = 4;
pub const ARF_UPDATE: C2RustUnnamed_30 = 3;
pub const GF_UPDATE: C2RustUnnamed_30 = 2;
pub const LF_UPDATE: C2RustUnnamed_30 = 1;
pub const KF_UPDATE: C2RustUnnamed_30 = 0;
pub type C2RustUnnamed_31 = core::ffi::c_uint;
pub const AQ_MODE_COUNT: C2RustUnnamed_31 = 4;
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_31 = 3;
pub const COMPLEXITY_AQ: C2RustUnnamed_31 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_31 = 1;
pub const NO_AQ: C2RustUnnamed_31 = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT8_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_SB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2;
pub const MAX_SB_SQUARE: core::ffi::c_int = MAX_SB_SIZE * MAX_SB_SIZE;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MAX_MIB_MASK: core::ffi::c_int = MAX_MIB_SIZE - 1 as core::ffi::c_int;
pub const PALETTE_MIN_SIZE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SQR_BLOCK_SIZES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PARTITION_PLOFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const PARTITION_BLOCK_SIZES: core::ffi::c_int = 5 as core::ffi::c_int;
pub const PARTITION_CONTEXTS: core::ffi::c_int = PARTITION_BLOCK_SIZES
    * PARTITION_PLOFFSET;
pub const CFL_ALPHABET_SIZE_LOG2: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE: core::ffi::c_int = (1 as core::ffi::c_int)
    << CFL_ALPHABET_SIZE_LOG2;
pub const MAX_ANGLE_DELTA: core::ffi::c_int = 3 as core::ffi::c_int;
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NEWMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << GLOBALMV_OFFSET)
    - 1 as core::ffi::c_int;
pub const GLOBALMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << REFMV_OFFSET - GLOBALMV_OFFSET) - 1 as core::ffi::c_int;
pub const REFMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 8 as core::ffi::c_int - REFMV_OFFSET) - 1 as core::ffi::c_int;
pub const USABLE_REF_MV_STACK_SIZE: core::ffi::c_int = 4 as core::ffi::c_int;
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
static mut subsize_lookup: [[BLOCK_SIZE; 6]; 10] = [
    [BLOCK_4X4, BLOCK_8X8, BLOCK_16X16, BLOCK_32X32, BLOCK_64X64, BLOCK_128X128],
    [BLOCK_INVALID, BLOCK_8X4, BLOCK_16X8, BLOCK_32X16, BLOCK_64X32, BLOCK_128X64],
    [BLOCK_INVALID, BLOCK_4X8, BLOCK_8X16, BLOCK_16X32, BLOCK_32X64, BLOCK_64X128],
    [BLOCK_INVALID, BLOCK_4X4, BLOCK_8X8, BLOCK_16X16, BLOCK_32X32, BLOCK_64X64],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8, BLOCK_32X16, BLOCK_64X32, BLOCK_128X64],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8, BLOCK_32X16, BLOCK_64X32, BLOCK_128X64],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16, BLOCK_16X32, BLOCK_32X64, BLOCK_64X128],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16, BLOCK_16X32, BLOCK_32X64, BLOCK_64X128],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X4, BLOCK_32X8, BLOCK_64X16, BLOCK_INVALID],
    [BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X16, BLOCK_8X32, BLOCK_16X64, BLOCK_INVALID],
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
pub const MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
#[inline]
unsafe extern "C-unwind" fn av1_extract_interp_filter(
    mut filters: int_interpfilters,
    mut dir: core::ffi::c_int,
) -> InterpFilter {
    return (if dir != 0 {
        filters.as_filters.x_filter as core::ffi::c_int
    } else {
        filters.as_filters.y_filter as core::ffi::c_int
    }) as InterpFilter;
}
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_PALETTE_BLOCK_WIDTH: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_PALETTE_BLOCK_HEIGHT: core::ffi::c_int = 64 as core::ffi::c_int;
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
    if !((::core::mem::size_of::<[MV_REFERENCE_FRAME; 9]>() as usize)
        .wrapping_div(::core::mem::size_of::<MV_REFERENCE_FRAME>() as usize)
        as core::ffi::c_int == TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"comp_ref0\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            397 as core::ffi::c_int,
            b"NELEMENTS(lut) == TOTAL_UNIDIR_COMP_REFS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
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
    if !((::core::mem::size_of::<[MV_REFERENCE_FRAME; 9]>() as usize)
        .wrapping_div(::core::mem::size_of::<MV_REFERENCE_FRAME>() as usize)
        as core::ffi::c_int == TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"comp_ref1\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            413 as core::ffi::c_int,
            b"NELEMENTS(lut) == TOTAL_UNIDIR_COMP_REFS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return lut[ref_idx as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_sqr_bsize_idx(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    match bsize as core::ffi::c_int {
        0 => return 0 as core::ffi::c_int,
        3 => return 1 as core::ffi::c_int,
        6 => return 2 as core::ffi::c_int,
        9 => return 3 as core::ffi::c_int,
        12 => return 4 as core::ffi::c_int,
        15 => return 5 as core::ffi::c_int,
        _ => return SQR_BLOCK_SIZES,
    };
}
#[inline]
unsafe extern "C-unwind" fn get_partition_subsize(
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
) -> BLOCK_SIZE {
    if partition as core::ffi::c_int == PARTITION_INVALID as core::ffi::c_int {
        return BLOCK_INVALID
    } else {
        let sqr_bsize_idx: core::ffi::c_int = get_sqr_bsize_idx(bsize)
            as core::ffi::c_int;
        return (if sqr_bsize_idx >= SQR_BLOCK_SIZES {
            BLOCK_INVALID as core::ffi::c_int
        } else {
            subsize_lookup[partition as usize][sqr_bsize_idx as usize]
                as core::ffi::c_int
        }) as BLOCK_SIZE;
    };
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
unsafe extern "C-unwind" fn av1_allow_palette(
    mut allow_screen_content_tools: core::ffi::c_int,
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    if !((sb_type as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_allow_palette\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1505 as core::ffi::c_int,
            b"sb_type < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return (allow_screen_content_tools != 0
        && block_size_wide[sb_type as usize] as core::ffi::c_int
            <= MAX_PALETTE_BLOCK_WIDTH
        && block_size_high[sb_type as usize] as core::ffi::c_int
            <= MAX_PALETTE_BLOCK_HEIGHT
        && sb_type as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn frame_is_intra_only(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_alloc_mi_idx(
    mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let mi_alloc_size_1d: core::ffi::c_int = mi_size_wide[(*mi_params).mi_alloc_bsize
        as usize] as core::ffi::c_int;
    let mi_alloc_row: core::ffi::c_int = mi_row / mi_alloc_size_1d;
    let mi_alloc_col: core::ffi::c_int = mi_col / mi_alloc_size_1d;
    return mi_alloc_row * (*mi_params).mi_alloc_stride + mi_alloc_col;
}
#[inline]
unsafe extern "C-unwind" fn get_partition(
    cm: *const AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> PARTITION_TYPE {
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    if mi_row >= (*mi_params).mi_rows || mi_col >= (*mi_params).mi_cols {
        return PARTITION_INVALID as core::ffi::c_int as PARTITION_TYPE;
    }
    let offset: core::ffi::c_int = mi_row * (*mi_params).mi_stride + mi_col;
    let mut mi: *mut *mut MB_MODE_INFO = ((*mi_params).mi_grid_base)
        .offset(offset as isize);
    let subsize: BLOCK_SIZE = (**mi.offset(0 as core::ffi::c_int as isize)).bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"get_partition\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1786 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if subsize as core::ffi::c_int == bsize as core::ffi::c_int {
        return PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
    }
    let bhigh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let bwide: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let sshigh: core::ffi::c_int = mi_size_high[subsize as usize] as core::ffi::c_int;
    let sswide: core::ffi::c_int = mi_size_wide[subsize as usize] as core::ffi::c_int;
    if bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int
        && (mi_row + bwide / 2 as core::ffi::c_int) < (*mi_params).mi_rows
        && (mi_col + bhigh / 2 as core::ffi::c_int) < (*mi_params).mi_cols
    {
        let mbmi_right: *const MB_MODE_INFO = *mi
            .offset((bwide / 2 as core::ffi::c_int) as isize);
        let mbmi_below: *const MB_MODE_INFO = *mi
            .offset((bhigh / 2 as core::ffi::c_int * (*mi_params).mi_stride) as isize);
        if sswide == bwide {
            if sshigh * 4 as core::ffi::c_int == bhigh {
                return PARTITION_HORZ_4 as core::ffi::c_int as PARTITION_TYPE;
            }
            if !(sshigh * 2 as core::ffi::c_int == bhigh) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"get_partition\0"))
                        .as_ptr(),
                    b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                    1807 as core::ffi::c_int,
                    b"sshigh * 2 == bhigh\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if (*mbmi_below).bsize as core::ffi::c_int == subsize as core::ffi::c_int {
                return PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE
            } else {
                return PARTITION_HORZ_B as core::ffi::c_int as PARTITION_TYPE
            }
        } else if sshigh == bhigh {
            if sswide * 4 as core::ffi::c_int == bwide {
                return PARTITION_VERT_4 as core::ffi::c_int as PARTITION_TYPE;
            }
            if !(sswide * 2 as core::ffi::c_int == bwide) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"get_partition\0"))
                        .as_ptr(),
                    b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                    1818 as core::ffi::c_int,
                    b"sswide * 2 == bwide\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if (*mbmi_right).bsize as core::ffi::c_int == subsize as core::ffi::c_int {
                return PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE
            } else {
                return PARTITION_VERT_B as core::ffi::c_int as PARTITION_TYPE
            }
        } else {
            if sswide * 2 as core::ffi::c_int != bwide
                || sshigh * 2 as core::ffi::c_int != bhigh
            {
                return PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
            }
            if mi_size_wide[(*mbmi_below).bsize as usize] as core::ffi::c_int == bwide {
                return PARTITION_HORZ_A as core::ffi::c_int as PARTITION_TYPE;
            }
            if mi_size_high[(*mbmi_right).bsize as usize] as core::ffi::c_int == bhigh {
                return PARTITION_VERT_A as core::ffi::c_int as PARTITION_TYPE;
            }
            return PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
        }
    }
    let vert_split: core::ffi::c_int = (sswide < bwide) as core::ffi::c_int;
    let horz_split: core::ffi::c_int = (sshigh < bhigh) as core::ffi::c_int;
    let split_idx: core::ffi::c_int = vert_split << 1 as core::ffi::c_int | horz_split;
    if !(split_idx != 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"get_partition\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1843 as core::ffi::c_int,
            b"split_idx != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    static mut base_partitions: [PARTITION_TYPE; 4] = [
        PARTITION_INVALID as core::ffi::c_int as PARTITION_TYPE,
        PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
        PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    ];
    return base_partitions[split_idx as usize];
}
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
            if !((REF_FRAMES as core::ffi::c_int
                + FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                + uni_comp_ref_idx as core::ffi::c_int)
                < REF_FRAMES as core::ffi::c_int
                    + (FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                        + TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int))
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"av1_ref_frame_type\0"))
                        .as_ptr(),
                    b"mvref_common.h\0" as *const u8 as *const core::ffi::c_char,
                    118 as core::ffi::c_int,
                    b"(REF_FRAMES + FWD_REFS * BWD_REFS + uni_comp_ref_idx) < MODE_CTX_REF_FRAMES\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            return (REF_FRAMES as core::ffi::c_int
                + FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                + uni_comp_ref_idx as core::ffi::c_int) as int8_t;
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
pub const MAX_LAG_BUFFERS: core::ffi::c_int = 48 as core::ffi::c_int;
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_set_error_per_bit(
    mut errorperbit: *mut core::ffi::c_int,
    mut rdmult: core::ffi::c_int,
) {
    *errorperbit = if rdmult >> 6 as core::ffi::c_int > 1 as core::ffi::c_int {
        rdmult >> 6 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn convert_length_to_bsize(
    mut length: core::ffi::c_int,
) -> BLOCK_SIZE {
    match length {
        64 => return BLOCK_64X64,
        32 => return BLOCK_32X32,
        16 => return BLOCK_16X16,
        8 => return BLOCK_8X8,
        4 => return BLOCK_4X4,
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid block size for tpl model\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"convert_length_to_bsize\0"))
                        .as_ptr(),
                    b"tpl_model.h\0" as *const u8 as *const core::ffi::c_char,
                    51 as core::ffi::c_int,
                    b"0 && \"Invalid block size for tpl model\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            return BLOCK_16X16;
        }
    };
}
pub const MAX_TPL_FRAME_IDX: core::ffi::c_int = 2 as core::ffi::c_int * MAX_LAG_BUFFERS;
pub const TPL_DEP_COST_SCALE_LOG2: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_stat_generation_stage(cpi: *const AV1_COMP) -> core::ffi::c_int {
    if !(!((*cpi).compressor_stage as core::ffi::c_int == LAP_STAGE as core::ffi::c_int)
        || (*cpi).oxcf.pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            && (*(*cpi).ppi).lap_enabled != 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"is_stat_generation_stage\0"))
                .as_ptr(),
            b"encoder.h\0" as *const u8 as *const core::ffi::c_char,
            4142 as core::ffi::c_int,
            b"IMPLIES(cpi->compressor_stage == LAP_STAGE, cpi->oxcf.pass == AOM_RC_ONE_PASS && cpi->ppi->lap_enabled)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        == AOM_RC_FIRST_PASS as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).compressor_stage as core::ffi::c_int == LAP_STAGE as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_stat_consumption_stage_twopass(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        >= AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_stat_consumption_stage(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    return (is_stat_consumption_stage_twopass(cpi) != 0
        || (*cpi).oxcf.pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            && (*cpi).compressor_stage as core::ffi::c_int
                == ENCODE_STAGE as core::ffi::c_int && (*(*cpi).ppi).lap_enabled != 0
            && (*cpi).oxcf.mode as core::ffi::c_int != REALTIME as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn find_partition_size(
    mut bsize: BLOCK_SIZE,
    mut rows_left: core::ffi::c_int,
    mut cols_left: core::ffi::c_int,
    mut bh: *mut core::ffi::c_int,
    mut bw: *mut core::ffi::c_int,
) -> BLOCK_SIZE {
    let mut int_size: core::ffi::c_int = bsize as core::ffi::c_int;
    if rows_left <= 0 as core::ffi::c_int || cols_left <= 0 as core::ffi::c_int {
        return (if (bsize as core::ffi::c_int) < BLOCK_8X8 as core::ffi::c_int {
            bsize as core::ffi::c_int
        } else {
            BLOCK_8X8 as core::ffi::c_int
        }) as BLOCK_SIZE
    } else {
        while int_size > 0 as core::ffi::c_int {
            *bh = mi_size_high[int_size as usize] as core::ffi::c_int;
            *bw = mi_size_wide[int_size as usize] as core::ffi::c_int;
            if *bh <= rows_left && *bw <= cols_left {
                break;
            }
            int_size -= 3 as core::ffi::c_int;
        }
    }
    return int_size as BLOCK_SIZE;
}
#[inline]
unsafe extern "C-unwind" fn is_frame_tpl_eligible(
    gf_group: *const GF_GROUP,
    mut index: uint8_t,
) -> core::ffi::c_int {
    let update_type: FRAME_UPDATE_TYPE = (*gf_group).update_type[index as usize];
    return (update_type as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == KF_UPDATE as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_frame_update_type(
    mut gf_group: *const GF_GROUP,
    mut gf_frame_index: core::ffi::c_int,
) -> FRAME_UPDATE_TYPE {
    return (*gf_group).update_type[gf_frame_index as usize];
}
#[inline]
unsafe extern "C-unwind" fn av1_pixels_to_mi(mut pixels: core::ffi::c_int) -> core::ffi::c_int {
    return (pixels
        + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int))
        >> MI_SIZE_LOG2;
}
#[inline]
unsafe extern "C-unwind" fn is_cfl_allowed(mut xd: *const MACROBLOCKD) -> CFL_ALLOWED_TYPE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"is_cfl_allowed\0"))
                .as_ptr(),
            b"cfl.h\0" as *const u8 as *const core::ffi::c_char,
            22 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
    if !((segment_id as core::ffi::c_int) < 8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"get_segment_id\0"))
                .as_ptr(),
            b"pred_common.h\0" as *const u8 as *const core::ffi::c_char,
            44 as core::ffi::c_int,
            b"segment_id < MAX_SEGMENTS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
    if !(!(prev_ul as core::ffi::c_int != 255 as core::ffi::c_int)
        || prev_u as core::ffi::c_int != 255 as core::ffi::c_int
            && prev_l as core::ffi::c_int != 255 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_spatial_seg_pred\0"))
                .as_ptr(),
            b"pred_common.h\0" as *const u8 as *const core::ffi::c_char,
            73 as core::ffi::c_int,
            b"IMPLIES(prev_ul != UINT8_MAX, prev_u != UINT8_MAX && prev_l != UINT8_MAX)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C-unwind" fn av1_get_palette_bsize_ctx(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_get_palette_bsize_ctx\0"))
                .as_ptr(),
            b"pred_common.h\0" as *const u8 as *const core::ffi::c_char,
            193 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C-unwind" fn coded_to_superres_mi(
    mut mi_col: core::ffi::c_int,
    mut denom: core::ffi::c_int,
) -> core::ffi::c_int {
    return (mi_col * denom + SCALE_NUMERATOR / 2 as core::ffi::c_int) / SCALE_NUMERATOR;
}
#[inline]
unsafe extern "C-unwind" fn set_rdmult(
    cpi: *const AV1_COMP,
    x: *const MACROBLOCK,
    mut segment_id: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let mut quant_params: *const CommonQuantParams = &(*cm).quant_params;
    let bit_depth: aom_bit_depth_t = (*(*cm).seq_params).bit_depth;
    let update_type: FRAME_UPDATE_TYPE = (*(*cpi).ppi)
        .gf_group
        .update_type[(*cpi).gf_frame_index as usize];
    let frame_type: FRAME_TYPE = (*cm).current_frame.frame_type;
    let boost_index: core::ffi::c_int = if (15 as core::ffi::c_int)
        < (*(*cpi).ppi).p_rc.gfu_boost / 100 as core::ffi::c_int
    {
        15 as core::ffi::c_int
    } else {
        (*(*cpi).ppi).p_rc.gfu_boost / 100 as core::ffi::c_int
    };
    let layer_depth: core::ffi::c_int = if (*gf_group)
        .layer_depth[(*cpi).gf_frame_index as usize] < 6 as core::ffi::c_int
    {
        (*gf_group).layer_depth[(*cpi).gf_frame_index as usize]
    } else {
        6 as core::ffi::c_int
    };
    let mut qindex: core::ffi::c_int = 0;
    if segment_id >= 0 as core::ffi::c_int {
        qindex = av1_get_qindex(&(*cm).seg, segment_id, (*cm).quant_params.base_qindex);
    } else {
        qindex = (*quant_params).base_qindex + (*x).rdmult_delta_qindex
            + (*quant_params).y_dc_delta_q;
    }
    return av1_compute_rd_mult(
        qindex,
        bit_depth,
        update_type,
        layer_depth,
        boost_index,
        frame_type,
        (*cpi).oxcf.q_cfg.use_fixed_qp_offsets,
        is_stat_consumption_stage(cpi),
        (*cpi).oxcf.tune_cfg.tuning,
        (*cpi).oxcf.mode,
    );
}
#[inline]
unsafe extern "C-unwind" fn read_one_frame_stats(
    mut p: *const TWO_PASS,
    mut frm: core::ffi::c_int,
) -> *const FIRSTPASS_STATS {
    if !(frm >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"read_one_frame_stats\0"))
                .as_ptr(),
            b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
            331 as core::ffi::c_int,
            b"frm >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if frm < 0 as core::ffi::c_int
        || ((*(*p).stats_buf_ctx).stats_in_start).offset(frm as isize)
            > (*(*p).stats_buf_ctx).stats_in_end
    {
        return 0 as *const FIRSTPASS_STATS;
    }
    return &raw mut *((*(*p).stats_buf_ctx).stats_in_start).offset(frm as isize)
        as *mut FIRSTPASS_STATS;
}
pub const AM_SEGMENT_ID_INACTIVE: core::ffi::c_int = 7 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_ssim_rdmult(
    cpi: *const AV1_COMP,
    mut errorperbit: *mut core::ffi::c_int,
    bsize: BLOCK_SIZE,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    rdmult: *mut core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let bsize_base: BLOCK_SIZE = BLOCK_16X16;
    let num_mi_w: core::ffi::c_int = mi_size_wide[bsize_base as usize]
        as core::ffi::c_int;
    let num_mi_h: core::ffi::c_int = mi_size_high[bsize_base as usize]
        as core::ffi::c_int;
    let num_cols: core::ffi::c_int = ((*cm).mi_params.mi_cols + num_mi_w
        - 1 as core::ffi::c_int) / num_mi_w;
    let num_rows: core::ffi::c_int = ((*cm).mi_params.mi_rows + num_mi_h
        - 1 as core::ffi::c_int) / num_mi_h;
    let num_bcols: core::ffi::c_int = (mi_size_wide[bsize as usize] as core::ffi::c_int
        + num_mi_w - 1 as core::ffi::c_int) / num_mi_w;
    let num_brows: core::ffi::c_int = (mi_size_high[bsize as usize] as core::ffi::c_int
        + num_mi_h - 1 as core::ffi::c_int) / num_mi_h;
    let mut row: core::ffi::c_int = 0;
    let mut col: core::ffi::c_int = 0;
    let mut num_of_mi: core::ffi::c_double = 0.0f64;
    let mut geom_mean_of_scale: core::ffi::c_double = 1.0f64;
    if !(bsize_base as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_set_ssim_rdmult\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            50 as core::ffi::c_int,
            b"bsize_base >= BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
        == AOM_TUNE_SSIM as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
            == AOM_TUNE_IQ as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
            == AOM_TUNE_SSIMULACRA2 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_set_ssim_rdmult\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            53 as core::ffi::c_int,
            b"cpi->oxcf.tune_cfg.tuning == AOM_TUNE_SSIM || cpi->oxcf.tune_cfg.tuning == AOM_TUNE_IQ || cpi->oxcf.tune_cfg.tuning == AOM_TUNE_SSIMULACRA2\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    row = mi_row / num_mi_w;
    while row < num_rows && row < mi_row / num_mi_w + num_brows {
        col = mi_col / num_mi_h;
        while col < num_cols && col < mi_col / num_mi_h + num_bcols {
            let index: core::ffi::c_int = row * num_cols + col;
            if !(*((*cpi).ssim_rdmult_scaling_factors).offset(index as isize) != 0.0f64)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"av1_set_ssim_rdmult\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    60 as core::ffi::c_int,
                    b"cpi->ssim_rdmult_scaling_factors[index] != 0.0\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            geom_mean_of_scale
                *= *((*cpi).ssim_rdmult_scaling_factors).offset(index as isize);
            num_of_mi += 1.0f64;
            col += 1;
        }
        row += 1;
    }
    geom_mean_of_scale = pow(geom_mean_of_scale, 1.0f64 / num_of_mi);
    *rdmult = (*rdmult as core::ffi::c_double * geom_mean_of_scale + 0.5f64)
        as core::ffi::c_int;
    *rdmult = if *rdmult > 0 as core::ffi::c_int {
        *rdmult
    } else {
        0 as core::ffi::c_int
    };
    av1_set_error_per_bit(errorperbit, *rdmult);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_cb_rdmult(
    cpi: *const AV1_COMP,
    x: *mut MACROBLOCK,
    bsize: BLOCK_SIZE,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_get_cb_rdmult\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            98 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let tpl_idx: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let mut deltaq_rdmult: core::ffi::c_int = set_rdmult(
        cpi,
        x,
        -(1 as core::ffi::c_int),
    );
    if av1_tpl_stats_ready(&raw mut (*(*cpi).ppi).tpl_data, tpl_idx) == 0 {
        return deltaq_rdmult;
    }
    if (*cm).superres_scale_denominator as core::ffi::c_int != SCALE_NUMERATOR {
        return deltaq_rdmult;
    }
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int != NO_AQ as core::ffi::c_int {
        return deltaq_rdmult;
    }
    if (*x).rb == 0 as core::ffi::c_int as core::ffi::c_double {
        return deltaq_rdmult;
    }
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let mut tpl_frame: *mut TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(tpl_idx as isize) as *mut TplDepFrame;
    let mut tpl_stats: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let mi_wide: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_high: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut tpl_stride: core::ffi::c_int = (*tpl_frame).stride;
    let mut intra_cost_base: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut mc_dep_cost_base: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut cbcmp_base: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*tpl_data).tpl_stats_block_mis_log2 as core::ffi::c_int;
    let mut row: core::ffi::c_int = mi_row;
    while row < mi_row + mi_high {
        let mut col: core::ffi::c_int = mi_col;
        while col < mi_col + mi_wide {
            if !(row >= (*cm).mi_params.mi_rows || col >= (*cm).mi_params.mi_cols) {
                let mut this_stats: *mut TplDepStats = &raw mut *tpl_stats
                    .offset(
                        (av1_tpl_ptr_pos
                            as unsafe extern "C-unwind" fn(
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                uint8_t,
                            ) -> core::ffi::c_int)(
                            row,
                            col,
                            tpl_stride,
                            (*tpl_data).tpl_stats_block_mis_log2,
                        ) as isize,
                    ) as *mut TplDepStats;
                let mut cbcmp: core::ffi::c_double = (*this_stats).srcrf_dist
                    as core::ffi::c_double;
                let mut mc_dep_delta: int64_t = ((*this_stats).mc_dep_rate
                    * (*tpl_frame).base_rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*this_stats).mc_dep_dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                let mut dist_scaled: core::ffi::c_double = ((*this_stats).recrf_dist
                    << RDDIV_BITS) as core::ffi::c_double;
                intra_cost_base += log(dist_scaled) * cbcmp;
                mc_dep_cost_base
                    += log(
                        3 as core::ffi::c_int as core::ffi::c_double * dist_scaled
                            + mc_dep_delta as core::ffi::c_double,
                    ) * cbcmp;
                cbcmp_base += cbcmp;
            }
            col += step;
        }
        row += step;
    }
    if cbcmp_base == 0 as core::ffi::c_int as core::ffi::c_double {
        return deltaq_rdmult;
    }
    let mut rk: core::ffi::c_double = exp(
        (intra_cost_base - mc_dep_cost_base) / cbcmp_base,
    );
    deltaq_rdmult = (deltaq_rdmult as core::ffi::c_double * (rk / (*x).rb))
        as core::ffi::c_int;
    return if deltaq_rdmult > 1 as core::ffi::c_int {
        deltaq_rdmult
    } else {
        1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn update_filter_type_count(
    mut counts: *mut FRAME_COUNTS,
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
) {
    let mut dir: core::ffi::c_int = 0;
    dir = 0 as core::ffi::c_int;
    while dir < 2 as core::ffi::c_int {
        let ctx: core::ffi::c_int = av1_get_pred_context_switchable_interp(xd, dir)
            as core::ffi::c_int;
        let mut filter: InterpFilter = av1_extract_interp_filter(
            (*mbmi).interp_filters,
            dir,
        );
        if !((filter as core::ffi::c_int) < SWITCHABLE_FILTERS as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"update_filter_type_count\0"))
                    .as_ptr(),
                b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                156 as core::ffi::c_int,
                b"filter < SWITCHABLE_FILTERS\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*counts).switchable_interp[ctx as usize][filter as usize] = ((*counts)
            .switchable_interp[ctx as usize][filter as usize])
            .wrapping_add(1);
        dir += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn copy_mbmi_ext_frame_to_mbmi_ext(
    mut mbmi_ext: *mut MB_MODE_INFO_EXT,
    mbmi_ext_best: *const MB_MODE_INFO_EXT_FRAME,
    mut ref_frame_type: uint8_t,
) {
    memcpy(
        ((*mbmi_ext).ref_mv_stack[ref_frame_type as usize]).as_mut_ptr()
            as *mut core::ffi::c_void,
        ((*mbmi_ext_best).ref_mv_stack).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[CANDIDATE_MV; 4]>() as size_t,
    );
    memcpy(
        ((*mbmi_ext).weight[ref_frame_type as usize]).as_mut_ptr()
            as *mut core::ffi::c_void,
        ((*mbmi_ext_best).weight).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as size_t,
    );
    (*mbmi_ext).mode_context[ref_frame_type as usize] = (*mbmi_ext_best).mode_context;
    (*mbmi_ext).ref_mv_count[ref_frame_type as usize] = (*mbmi_ext_best).ref_mv_count;
    memcpy(
        ((*mbmi_ext).global_mvs).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mbmi_ext_best).global_mvs).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[int_mv; 8]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_update_state(
    cpi: *const AV1_COMP,
    mut td: *mut ThreadData,
    ctx: *const PICK_MODE_CONTEXT,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut dry_run: RUN_TYPE,
) {
    let mut i: core::ffi::c_int = 0;
    let mut x_idx: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let cm: *const AV1_COMMON = &(*cpi).common;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let p: *mut macroblock_plane = ((*x).plane).as_mut_ptr();
    let pd: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
    let mi: *const MB_MODE_INFO = &(*ctx).mic;
    if ctx.is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_ERROR,
            b"ctx is NULL in av1_update_state: %d %d %d %d %d %d \n\0" as *const u8
                as *const core::ffi::c_char,
            (*cm).current_frame.frame_type as core::ffi::c_int,
            (*cm).width,
            (*cm).height,
            mi_col,
            mi_row,
            bsize as core::ffi::c_int,
        );
    }
    if mi.is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_ERROR,
            b"mi is NULL in av1_update_state: %d %d %d %d %d %d \n\0" as *const u8
                as *const core::ffi::c_char,
            (*cm).current_frame.frame_type as core::ffi::c_int,
            (*cm).width,
            (*cm).height,
            mi_col,
            mi_row,
            bsize as core::ffi::c_int,
        );
    }
    let mi_addr: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let seg: *const segmentation = &(*cm).seg;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"av1_update_state\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            202 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*mi).bsize as core::ffi::c_int == bsize as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"av1_update_state\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            203 as core::ffi::c_int,
            b"mi->bsize == bsize\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mis: core::ffi::c_int = (*mi_params).mi_stride;
    let mi_width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    *mi_addr = *mi;
    copy_mbmi_ext_frame_to_mbmi_ext(
        &raw mut (*x).mbmi_ext,
        &(*ctx).mbmi_ext_best,
        av1_ref_frame_type(((*ctx).mic.ref_frame).as_ptr()) as uint8_t,
    );
    (*txfm_info).skip_txfm = (*ctx).rd_stats.skip_txfm;
    (*xd).tx_type_map = (*ctx).tx_type_map;
    (*xd).tx_type_map_stride = mi_size_wide[bsize as usize] as core::ffi::c_int;
    if dry_run == 0 {
        let grid_idx: core::ffi::c_int = get_mi_grid_idx(mi_params, mi_row, mi_col)
            as core::ffi::c_int;
        let tx_type_map: *mut uint8_t = ((*mi_params).tx_type_map)
            .offset(grid_idx as isize);
        let mi_stride: core::ffi::c_int = (*mi_params).mi_stride;
        let mut blk_row: core::ffi::c_int = 0 as core::ffi::c_int;
        while blk_row < bh {
            memcpy(
                tx_type_map.offset((blk_row * mi_stride) as isize)
                    as *mut core::ffi::c_void,
                ((*xd).tx_type_map).offset((blk_row * (*xd).tx_type_map_stride) as isize)
                    as *const core::ffi::c_void,
                (bw as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
            blk_row += 1;
        }
        (*xd).tx_type_map = tx_type_map;
        (*xd).tx_type_map_stride = mi_stride;
    }
    if (*seg).enabled != 0 {
        if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
            == COMPLEXITY_AQ as core::ffi::c_int
            || (*cpi).roi.enabled as core::ffi::c_int != 0
        {
            let map: *const uint8_t = if (*seg).update_map as core::ffi::c_int != 0 {
                (*cpi).enc_seg.map
            } else {
                (*cm).last_frame_seg_map
            };
            (*mi_addr)
                .set_segment_id(
                    (if !map.is_null() {
                        get_segment_id(mi_params, map, bsize, mi_row, mi_col)
                            as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint8_t as uint8_t,
                );
        }
        if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
            == CYCLIC_REFRESH_AQ as core::ffi::c_int && (*cpi).roi.enabled == 0
            && (*mi_addr).segment_id() as core::ffi::c_int != AM_SEGMENT_ID_INACTIVE
            && (*cpi).rc.rtc_external_ratectrl == 0
        {
            av1_cyclic_refresh_update_segment(
                cpi as *const AV1_COMP,
                x,
                mi_row,
                mi_col,
                bsize,
                (*ctx).rd_stats.rate as int64_t,
                (*ctx).rd_stats.dist,
                (*txfm_info).skip_txfm as core::ffi::c_int,
                dry_run,
            );
        }
        if (*mi_addr).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int
            && is_cfl_allowed(xd) == 0
        {
            (*mi_addr).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
        }
        if dry_run == 0 && (*mi_addr).skip_txfm == 0 {
            let mut cdf_num: core::ffi::c_int = 0;
            let spatial_pred: uint8_t = av1_get_spatial_seg_pred(
                cm,
                xd,
                &raw mut cdf_num,
                (*(*cpi).cyclic_refresh).skip_over4x4,
            ) as uint8_t;
            let coded_id: uint8_t = av1_neg_interleave(
                (*mi_addr).segment_id() as core::ffi::c_int,
                spatial_pred as core::ffi::c_int,
                (*seg).last_active_segid + 1 as core::ffi::c_int,
            ) as uint8_t;
            let mut spatial_cost: int64_t = (*x)
                .mode_costs
                .spatial_pred_cost[cdf_num as usize][coded_id as usize] as int64_t;
            (*td).rd_counts.seg_tmp_pred_cost[0 as core::ffi::c_int as usize] = ((*td)
                .rd_counts
                .seg_tmp_pred_cost[0 as core::ffi::c_int as usize])
                .wrapping_add(spatial_cost as uint64_t);
            let pred_segment_id: core::ffi::c_int = if !((*cm).last_frame_seg_map)
                .is_null()
            {
                get_segment_id(
                    mi_params,
                    (*cm).last_frame_seg_map,
                    bsize,
                    mi_row,
                    mi_col,
                ) as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            };
            let use_tmp_pred: core::ffi::c_int = (pred_segment_id
                == (*mi_addr).segment_id() as core::ffi::c_int) as core::ffi::c_int;
            let tmp_pred_ctx: uint8_t = av1_get_pred_context_seg_id(xd) as uint8_t;
            (*td).rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize] = ((*td)
                .rd_counts
                .seg_tmp_pred_cost[1 as core::ffi::c_int as usize])
                .wrapping_add(
                    (*x)
                        .mode_costs
                        .tmp_pred_cost[tmp_pred_ctx as usize][use_tmp_pred as usize]
                        as uint64_t,
                );
            if use_tmp_pred == 0 {
                (*td).rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize] = ((*td)
                    .rd_counts
                    .seg_tmp_pred_cost[1 as core::ffi::c_int as usize])
                    .wrapping_add(spatial_cost as uint64_t);
            }
        }
    }
    if dry_run == 0 && frame_is_intra_only(cm) == 0 {
        let mv: MV = (*mi).mv[0 as core::ffi::c_int as usize].as_mv;
        if is_inter_block(mi) != 0
            && (*mi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == LAST_FRAME as core::ffi::c_int
            && abs(mv.row as core::ffi::c_int) < 8 as core::ffi::c_int
            && abs(mv.col as core::ffi::c_int) < 8 as core::ffi::c_int
        {
            let ymis: core::ffi::c_int = if (*cm).mi_params.mi_rows - mi_row < bh {
                (*cm).mi_params.mi_rows - mi_row
            } else {
                bh
            };
            let mut mi_y: core::ffi::c_int = 0 as core::ffi::c_int;
            while mi_y < ymis {
                (*x).cnt_zeromv += bw << 1 as core::ffi::c_int;
                mi_y += 2 as core::ffi::c_int;
            }
        }
    }
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        let fresh3 = &raw mut (*p.offset(i as isize)).coeff;
        *fresh3 = (*ctx).coeff[i as usize];
        let fresh4 = &raw mut (*p.offset(i as isize)).qcoeff;
        *fresh4 = (*ctx).qcoeff[i as usize];
        let fresh5 = &raw mut (*p.offset(i as isize)).dqcoeff;
        *fresh5 = (*ctx).dqcoeff[i as usize];
        let fresh6 = &raw mut (*p.offset(i as isize)).eobs;
        *fresh6 = (*ctx).eobs[i as usize];
        let fresh7 = &raw mut (*p.offset(i as isize)).txb_entropy_ctx;
        *fresh7 = (*ctx).txb_entropy_ctx[i as usize];
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let fresh8 = &raw mut (*pd.offset(i as isize)).color_index_map;
        *fresh8 = (*ctx).color_index_map[i as usize];
        i += 1;
    }
    let cols: core::ffi::c_int = if ((*xd).mb_to_right_edge
        >> 3 as core::ffi::c_int + 2 as core::ffi::c_int) + mi_width < mi_width
    {
        ((*xd).mb_to_right_edge >> 3 as core::ffi::c_int + 2 as core::ffi::c_int)
            + mi_width
    } else {
        mi_width
    };
    let rows: core::ffi::c_int = if ((*xd).mb_to_bottom_edge
        >> 3 as core::ffi::c_int + 2 as core::ffi::c_int) + mi_height < mi_height
    {
        ((*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int + 2 as core::ffi::c_int)
            + mi_height
    } else {
        mi_height
    };
    y = 0 as core::ffi::c_int;
    while y < rows {
        x_idx = 0 as core::ffi::c_int;
        while x_idx < cols {
            let fresh9 = &raw mut *((*xd).mi).offset((x_idx + y * mis) as isize);
            *fresh9 = mi_addr;
            x_idx += 1;
        }
        y += 1;
    }
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int != 0
        || (*cpi).roi.enabled as core::ffi::c_int != 0
            && (*cpi).roi.delta_qp_enabled != 0
    {
        av1_init_plane_quantizers(
            cpi as *const AV1_COMP,
            x,
            (*mi_addr).segment_id() as core::ffi::c_int,
            0 as core::ffi::c_int,
        );
    }
    if dry_run != 0 {
        return;
    }
    if frame_is_intra_only(cm) == 0 {
        if is_inter_block(mi) != 0
            && (*cm).features.interp_filter as core::ffi::c_int
                == SWITCHABLE as core::ffi::c_int
        {
            update_filter_type_count((*td).counts, xd, mi_addr);
        }
    }
    let x_mis: core::ffi::c_int = if bw < (*mi_params).mi_cols - mi_col {
        bw
    } else {
        (*mi_params).mi_cols - mi_col
    };
    let y_mis: core::ffi::c_int = if bh < (*mi_params).mi_rows - mi_row {
        bh
    } else {
        (*mi_params).mi_rows - mi_row
    };
    if (*(*cm).seq_params).order_hint_info.enable_ref_frame_mvs != 0 {
        av1_copy_frame_mvs(cm, mi, mi_row, mi_col, x_mis, y_mis);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_update_inter_mode_stats(
    mut fc: *mut FRAME_CONTEXT,
    mut counts: *mut FRAME_COUNTS,
    mut mode: PREDICTION_MODE,
    mut mode_context: int16_t,
) {
    let mut mode_ctx: int16_t = (mode_context as core::ffi::c_int & NEWMV_CTX_MASK)
        as int16_t;
    if mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
        update_cdf(
            ((*fc).newmv_cdf[mode_ctx as usize]).as_mut_ptr(),
            0 as int8_t,
            2 as core::ffi::c_int,
        );
        return;
    }
    update_cdf(
        ((*fc).newmv_cdf[mode_ctx as usize]).as_mut_ptr(),
        1 as int8_t,
        2 as core::ffi::c_int,
    );
    mode_ctx = (mode_context as core::ffi::c_int >> GLOBALMV_OFFSET & GLOBALMV_CTX_MASK)
        as int16_t;
    if mode as core::ffi::c_int == GLOBALMV as core::ffi::c_int {
        update_cdf(
            ((*fc).zeromv_cdf[mode_ctx as usize]).as_mut_ptr(),
            0 as int8_t,
            2 as core::ffi::c_int,
        );
        return;
    }
    update_cdf(
        ((*fc).zeromv_cdf[mode_ctx as usize]).as_mut_ptr(),
        1 as int8_t,
        2 as core::ffi::c_int,
    );
    mode_ctx = (mode_context as core::ffi::c_int >> REFMV_OFFSET & REFMV_CTX_MASK)
        as int16_t;
    update_cdf(
        ((*fc).refmv_cdf[mode_ctx as usize]).as_mut_ptr(),
        (mode as core::ffi::c_int != NEARESTMV as core::ffi::c_int) as core::ffi::c_int
            as int8_t,
        2 as core::ffi::c_int,
    );
}
unsafe extern "C-unwind" fn update_palette_cdf(
    mut xd: *mut MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mut counts: *mut FRAME_COUNTS,
) {
    let mut fc: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let pmi: *const PALETTE_MODE_INFO = &(*mbmi).palette_mode_info;
    let palette_bsize_ctx: core::ffi::c_int = av1_get_palette_bsize_ctx(bsize)
        as core::ffi::c_int;
    if (*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        let n: core::ffi::c_int = (*pmi).palette_size[0 as core::ffi::c_int as usize]
            as core::ffi::c_int;
        let palette_mode_ctx: core::ffi::c_int = av1_get_palette_mode_ctx(xd)
            as core::ffi::c_int;
        update_cdf(
            ((*fc)
                .palette_y_mode_cdf[palette_bsize_ctx
                as usize][palette_mode_ctx as usize])
                .as_mut_ptr(),
            (n > 0 as core::ffi::c_int) as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int,
        );
        if n > 0 as core::ffi::c_int {
            update_cdf(
                ((*fc).palette_y_size_cdf[palette_bsize_ctx as usize]).as_mut_ptr(),
                (n - PALETTE_MIN_SIZE) as int8_t,
                PALETTE_SIZES as core::ffi::c_int,
            );
        }
    }
    if (*mbmi).uv_mode as core::ffi::c_int == UV_DC_PRED as core::ffi::c_int {
        let n_0: core::ffi::c_int = (*pmi).palette_size[1 as core::ffi::c_int as usize]
            as core::ffi::c_int;
        let palette_uv_mode_ctx: core::ffi::c_int = ((*pmi)
            .palette_size[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int) as core::ffi::c_int;
        update_cdf(
            ((*fc).palette_uv_mode_cdf[palette_uv_mode_ctx as usize]).as_mut_ptr(),
            (n_0 > 0 as core::ffi::c_int) as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int,
        );
        if n_0 > 0 as core::ffi::c_int {
            update_cdf(
                ((*fc).palette_uv_size_cdf[palette_bsize_ctx as usize]).as_mut_ptr(),
                (n_0 - PALETTE_MIN_SIZE) as int8_t,
                PALETTE_SIZES as core::ffi::c_int,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_sum_intra_stats(
    cm: *const AV1_COMMON,
    mut counts: *mut FRAME_COUNTS,
    mut xd: *mut MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mut above_mi: *const MB_MODE_INFO,
    mut left_mi: *const MB_MODE_INFO,
    intraonly: core::ffi::c_int,
) {
    let mut fc: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let y_mode: PREDICTION_MODE = (*mbmi).mode;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if intraonly != 0 {
        update_cdf(
            get_y_mode_cdf(fc, above_mi, left_mi),
            y_mode as int8_t,
            INTRA_MODES as core::ffi::c_int,
        );
    } else {
        update_cdf(
            ((*fc).y_mode_cdf[size_group_lookup[bsize as usize] as usize]).as_mut_ptr(),
            y_mode as int8_t,
            INTRA_MODES as core::ffi::c_int,
        );
    }
    if av1_filter_intra_allowed(cm, mbmi) != 0 {
        let use_filter_intra_mode: core::ffi::c_int = (*mbmi)
            .filter_intra_mode_info
            .use_filter_intra as core::ffi::c_int;
        update_cdf(
            ((*fc).filter_intra_cdfs[(*mbmi).bsize as usize]).as_mut_ptr(),
            use_filter_intra_mode as int8_t,
            2 as core::ffi::c_int,
        );
        if use_filter_intra_mode != 0 {
            update_cdf(
                ((*fc).filter_intra_mode_cdf).as_mut_ptr(),
                (*mbmi).filter_intra_mode_info.filter_intra_mode as int8_t,
                FILTER_INTRA_MODES as core::ffi::c_int,
            );
        }
    }
    if av1_is_directional_mode((*mbmi).mode) != 0 && av1_use_angle_delta(bsize) != 0 {
        update_cdf(
            ((*fc)
                .angle_delta_cdf[((*mbmi).mode as core::ffi::c_int
                - V_PRED as core::ffi::c_int) as usize])
                .as_mut_ptr(),
            ((*mbmi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize]
                as core::ffi::c_int + MAX_ANGLE_DELTA) as int8_t,
            2 as core::ffi::c_int * MAX_ANGLE_DELTA + 1 as core::ffi::c_int,
        );
    }
    if !(*xd).is_chroma_ref {
        return;
    }
    let uv_mode: UV_PREDICTION_MODE = (*mbmi).uv_mode;
    let cfl_allowed: CFL_ALLOWED_TYPE = is_cfl_allowed(xd) as CFL_ALLOWED_TYPE;
    update_cdf(
        ((*fc).uv_mode_cdf[cfl_allowed as usize][y_mode as usize]).as_mut_ptr(),
        uv_mode as int8_t,
        UV_INTRA_MODES as core::ffi::c_int - (cfl_allowed == 0) as core::ffi::c_int,
    );
    if uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int {
        let joint_sign: int8_t = (*mbmi).cfl_alpha_signs;
        let idx: uint8_t = (*mbmi).cfl_alpha_idx;
        update_cdf(
            ((*fc).cfl_sign_cdf).as_mut_ptr(),
            joint_sign,
            CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
                - 1 as core::ffi::c_int,
        );
        if (joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
            * 11 as core::ffi::c_int >> 5 as core::ffi::c_int
            != CFL_SIGN_ZERO as core::ffi::c_int
        {
            let mut cdf_u: *mut aom_cdf_prob = ((*fc)
                .cfl_alpha_cdf[(joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
                - CFL_SIGNS as core::ffi::c_int) as usize])
                .as_mut_ptr();
            update_cdf(
                cdf_u,
                (idx as core::ffi::c_int >> CFL_ALPHABET_SIZE_LOG2) as int8_t,
                CFL_ALPHABET_SIZE,
            );
        }
        if joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int
                * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
            != CFL_SIGN_ZERO as core::ffi::c_int
        {
            let mut cdf_v: *mut aom_cdf_prob = ((*fc)
                .cfl_alpha_cdf[((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
                - CFL_SIGNS as core::ffi::c_int
                    * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                        * 11 as core::ffi::c_int >> 5 as core::ffi::c_int))
                * CFL_SIGNS as core::ffi::c_int
                + ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
                - CFL_SIGNS as core::ffi::c_int) as usize])
                .as_mut_ptr();
            update_cdf(
                cdf_v,
                (idx as core::ffi::c_int & CFL_ALPHABET_SIZE - 1 as core::ffi::c_int)
                    as int8_t,
                CFL_ALPHABET_SIZE,
            );
        }
    }
    let intra_mode: PREDICTION_MODE = get_uv_mode(uv_mode) as PREDICTION_MODE;
    if av1_is_directional_mode(intra_mode) != 0 && av1_use_angle_delta(bsize) != 0 {
        update_cdf(
            ((*fc)
                .angle_delta_cdf[(intra_mode as core::ffi::c_int
                - V_PRED as core::ffi::c_int) as usize])
                .as_mut_ptr(),
            ((*mbmi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize]
                as core::ffi::c_int + MAX_ANGLE_DELTA) as int8_t,
            2 as core::ffi::c_int * MAX_ANGLE_DELTA + 1 as core::ffi::c_int,
        );
    }
    if av1_allow_palette(
        (*cm).features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) != 0
    {
        update_palette_cdf(xd, mbmi, counts);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_restore_context(
    mut x: *mut MACROBLOCK,
    mut ctx: *const RD_SEARCH_MACROBLOCK_CONTEXT,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    num_planes: core::ffi::c_int,
) {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut p: core::ffi::c_int = 0;
    let num_4x4_blocks_wide: core::ffi::c_int = mi_size_wide[bsize as usize]
        as core::ffi::c_int;
    let num_4x4_blocks_high: core::ffi::c_int = mi_size_high[bsize as usize]
        as core::ffi::c_int;
    let mut mi_width: core::ffi::c_int = mi_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut mi_height: core::ffi::c_int = mi_size_high[bsize as usize]
        as core::ffi::c_int;
    p = 0 as core::ffi::c_int;
    while p < num_planes {
        let mut tx_col: core::ffi::c_int = mi_col;
        let mut tx_row: core::ffi::c_int = mi_row & MAX_MIB_MASK;
        memcpy(
            ((*xd).above_entropy_context[p as usize])
                .offset((tx_col >> (*xd).plane[p as usize].subsampling_x) as isize)
                as *mut core::ffi::c_void,
            ((*ctx).a).as_ptr().offset((num_4x4_blocks_wide * p) as isize)
                as *const core::ffi::c_void,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(num_4x4_blocks_wide as size_t)
                >> (*xd).plane[p as usize].subsampling_x,
        );
        memcpy(
            ((*xd).left_entropy_context[p as usize])
                .as_mut_ptr()
                .offset((tx_row >> (*xd).plane[p as usize].subsampling_y) as isize)
                as *mut core::ffi::c_void,
            ((*ctx).l).as_ptr().offset((num_4x4_blocks_high * p) as isize)
                as *const core::ffi::c_void,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(num_4x4_blocks_high as size_t)
                >> (*xd).plane[p as usize].subsampling_y,
        );
        p += 1;
    }
    memcpy(
        ((*xd).above_partition_context).offset(mi_col as isize)
            as *mut core::ffi::c_void,
        ((*ctx).sa).as_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<PARTITION_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        ((*xd).left_partition_context)
            .as_mut_ptr()
            .offset((mi_row & MAX_MIB_MASK) as isize) as *mut core::ffi::c_void,
        ((*ctx).sl).as_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<PARTITION_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
    (*xd).above_txfm_context = (*ctx).p_ta;
    (*xd).left_txfm_context = (*ctx).p_tl;
    memcpy(
        (*xd).above_txfm_context as *mut core::ffi::c_void,
        ((*ctx).ta).as_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        (*xd).left_txfm_context as *mut core::ffi::c_void,
        ((*ctx).tl).as_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_save_context(
    mut x: *const MACROBLOCK,
    mut ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    num_planes: core::ffi::c_int,
) {
    let mut xd: *const MACROBLOCKD = &(*x).e_mbd;
    let mut p: core::ffi::c_int = 0;
    let mut mi_width: core::ffi::c_int = mi_size_wide[bsize as usize]
        as core::ffi::c_int;
    let mut mi_height: core::ffi::c_int = mi_size_high[bsize as usize]
        as core::ffi::c_int;
    p = 0 as core::ffi::c_int;
    while p < num_planes {
        let mut tx_col: core::ffi::c_int = mi_col;
        let mut tx_row: core::ffi::c_int = mi_row & MAX_MIB_MASK;
        memcpy(
            ((*ctx).a).as_mut_ptr().offset((mi_width * p) as isize)
                as *mut core::ffi::c_void,
            ((*xd).above_entropy_context[p as usize])
                .offset((tx_col >> (*xd).plane[p as usize].subsampling_x) as isize)
                as *const core::ffi::c_void,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(mi_width as size_t)
                >> (*xd).plane[p as usize].subsampling_x,
        );
        memcpy(
            ((*ctx).l).as_mut_ptr().offset((mi_height * p) as isize)
                as *mut core::ffi::c_void,
            ((*xd).left_entropy_context[p as usize])
                .as_ptr()
                .offset((tx_row >> (*xd).plane[p as usize].subsampling_y) as isize)
                as *const core::ffi::c_void,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(mi_height as size_t)
                >> (*xd).plane[p as usize].subsampling_y,
        );
        p += 1;
    }
    memcpy(
        ((*ctx).sa).as_mut_ptr() as *mut core::ffi::c_void,
        ((*xd).above_partition_context).offset(mi_col as isize)
            as *const core::ffi::c_void,
        (::core::mem::size_of::<PARTITION_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        ((*ctx).sl).as_mut_ptr() as *mut core::ffi::c_void,
        ((*xd).left_partition_context).as_ptr().offset((mi_row & MAX_MIB_MASK) as isize)
            as *const core::ffi::c_void,
        (::core::mem::size_of::<PARTITION_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
    memcpy(
        ((*ctx).ta).as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).above_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_width as size_t),
    );
    memcpy(
        ((*ctx).tl).as_mut_ptr() as *mut core::ffi::c_void,
        (*xd).left_txfm_context as *const core::ffi::c_void,
        (::core::mem::size_of::<TXFM_CONTEXT>() as size_t)
            .wrapping_mul(mi_height as size_t),
    );
    (*ctx).p_ta = (*xd).above_txfm_context;
    (*ctx).p_tl = (*xd).left_txfm_context;
}
unsafe extern "C-unwind" fn set_partial_sb_partition(
    cm: *const AV1_COMMON,
    mut mi: *mut MB_MODE_INFO,
    mut bh_in: core::ffi::c_int,
    mut bw_in: core::ffi::c_int,
    mut mi_rows_remaining: core::ffi::c_int,
    mut mi_cols_remaining: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mib: *mut *mut MB_MODE_INFO,
) {
    let mut bh: core::ffi::c_int = bh_in;
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < (*(*cm).seq_params).mib_size {
        let mut bw: core::ffi::c_int = bw_in;
        c = 0 as core::ffi::c_int;
        while c < (*(*cm).seq_params).mib_size {
            let grid_index: core::ffi::c_int = get_mi_grid_idx(&(*cm).mi_params, r, c)
                as core::ffi::c_int;
            let mi_index: core::ffi::c_int = get_alloc_mi_idx(&(*cm).mi_params, r, c)
                as core::ffi::c_int;
            let fresh11 = &raw mut *mib.offset(grid_index as isize);
            *fresh11 = mi.offset(mi_index as isize);
            (**mib.offset(grid_index as isize)).bsize = find_partition_size(
                bsize,
                mi_rows_remaining - r,
                mi_cols_remaining - c,
                &raw mut bh,
                &raw mut bw,
            );
            c += bw;
        }
        r += bh;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_fixed_partitioning(
    mut cpi: *mut AV1_COMP,
    tile: *const TileInfo,
    mut mib: *mut *mut MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let mi_rows_remaining: core::ffi::c_int = (*tile).mi_row_end - mi_row;
    let mi_cols_remaining: core::ffi::c_int = (*tile).mi_col_end - mi_col;
    let mi_upper_left: *mut MB_MODE_INFO = ((*mi_params).mi_alloc)
        .offset(get_alloc_mi_idx(mi_params, mi_row, mi_col) as isize);
    let mut bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    if !(bsize as core::ffi::c_int >= (*mi_params).mi_alloc_bsize as core::ffi::c_int
        && !(b"Attempted to use bsize < mi_params->mi_alloc_bsize\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_set_fixed_partitioning\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            648 as core::ffi::c_int,
            b"bsize >= mi_params->mi_alloc_bsize && \"Attempted to use bsize < mi_params->mi_alloc_bsize\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(mi_rows_remaining > 0 as core::ffi::c_int
        && mi_cols_remaining > 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_set_fixed_partitioning\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            649 as core::ffi::c_int,
            b"(mi_rows_remaining > 0) && (mi_cols_remaining > 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if mi_cols_remaining >= (*(*cm).seq_params).mib_size
        && mi_rows_remaining >= (*(*cm).seq_params).mib_size
    {
        let mut block_row: core::ffi::c_int = 0 as core::ffi::c_int;
        while block_row < (*(*cm).seq_params).mib_size {
            let mut block_col: core::ffi::c_int = 0 as core::ffi::c_int;
            while block_col < (*(*cm).seq_params).mib_size {
                let grid_index: core::ffi::c_int = get_mi_grid_idx(
                    mi_params,
                    block_row,
                    block_col,
                ) as core::ffi::c_int;
                let mi_index: core::ffi::c_int = get_alloc_mi_idx(
                    mi_params,
                    block_row,
                    block_col,
                ) as core::ffi::c_int;
                let fresh10 = &raw mut *mib.offset(grid_index as isize);
                *fresh10 = mi_upper_left.offset(mi_index as isize);
                (**mib.offset(grid_index as isize)).bsize = bsize;
                block_col += bw;
            }
            block_row += bh;
        }
    } else {
        set_partial_sb_partition(
            cm,
            mi_upper_left,
            bh,
            bw,
            mi_rows_remaining,
            mi_cols_remaining,
            bsize,
            mib,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_is_leaf_split_partition(
    mut cm: *mut AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let bs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bs / 2 as core::ffi::c_int;
    if !(bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_is_leaf_split_partition\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            675 as core::ffi::c_int,
            b"bsize >= BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let subsize: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        let mut x_idx: core::ffi::c_int = (i & 1 as core::ffi::c_int) * hbs;
        let mut y_idx: core::ffi::c_int = (i >> 1 as core::ffi::c_int) * hbs;
        if mi_row + y_idx >= (*cm).mi_params.mi_rows
            || mi_col + x_idx >= (*cm).mi_params.mi_cols
        {
            return 0 as core::ffi::c_int;
        }
        if get_partition(cm, mi_row + y_idx, mi_col + x_idx, subsize) as core::ffi::c_int
            != PARTITION_NONE as core::ffi::c_int
            && subsize as core::ffi::c_int != BLOCK_8X8 as core::ffi::c_int
        {
            return 0 as core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_rdmult_delta(
    mut cpi: *mut AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut orig_rdmult: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_get_rdmult_delta\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            698 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let tpl_idx: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let block_mis_log2: uint8_t = (*tpl_data).tpl_stats_block_mis_log2;
    let mut intra_cost: int64_t = 0 as int64_t;
    let mut mc_dep_cost: int64_t = 0 as int64_t;
    let mi_wide: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_high: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut tpl_frame: *mut TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(tpl_idx as isize) as *mut TplDepFrame;
    let mut tpl_stats: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let mut tpl_stride: core::ffi::c_int = (*tpl_frame).stride;
    if av1_tpl_stats_ready(
        &raw mut (*(*cpi).ppi).tpl_data,
        (*cpi).gf_frame_index as core::ffi::c_int,
    ) == 0
    {
        return orig_rdmult;
    }
    if is_frame_tpl_eligible(gf_group, (*cpi).gf_frame_index as uint8_t) == 0 {
        return orig_rdmult;
    }
    let mut mi_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_col_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_col_end_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col + mi_wide,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_cols_sr: core::ffi::c_int = av1_pixels_to_mi((*cm).superres_upscaled_width)
        as core::ffi::c_int;
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << block_mis_log2 as core::ffi::c_int;
    let row_step: core::ffi::c_int = step;
    let col_step_sr: core::ffi::c_int = coded_to_superres_mi(
        step,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut row: core::ffi::c_int = mi_row;
    while row < mi_row + mi_high {
        let mut col: core::ffi::c_int = mi_col_sr;
        while col < mi_col_end_sr {
            if !(row >= (*cm).mi_params.mi_rows || col >= mi_cols_sr) {
                let mut this_stats: *mut TplDepStats = &raw mut *tpl_stats
                    .offset(
                        (av1_tpl_ptr_pos
                            as unsafe extern "C-unwind" fn(
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                uint8_t,
                            ) -> core::ffi::c_int)(row, col, tpl_stride, block_mis_log2)
                            as isize,
                    ) as *mut TplDepStats;
                let mut mc_dep_delta: int64_t = ((*this_stats).mc_dep_rate
                    * (*tpl_frame).base_rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*this_stats).mc_dep_dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                intra_cost += (*this_stats).recrf_dist << RDDIV_BITS;
                mc_dep_cost += ((*this_stats).recrf_dist << RDDIV_BITS) + mc_dep_delta;
                mi_count += 1;
            }
            col += col_step_sr;
        }
        row += row_step;
    }
    if !(mi_count
        <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) / 16 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                / 16 as core::ffi::c_int)) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_get_rdmult_delta\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            745 as core::ffi::c_int,
            b"mi_count <= MAX_TPL_BLK_IN_SB * MAX_TPL_BLK_IN_SB\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut beta: core::ffi::c_double = 1.0f64;
    if mc_dep_cost > 0 as int64_t && intra_cost > 0 as int64_t {
        let r0: core::ffi::c_double = (*cpi).rd.r0;
        let rk: core::ffi::c_double = intra_cost as core::ffi::c_double
            / mc_dep_cost as core::ffi::c_double;
        beta = r0 / rk;
    }
    let mut rdmult: core::ffi::c_int = av1_get_adaptive_rdmult(cpi, beta);
    rdmult = if rdmult < orig_rdmult * 3 as core::ffi::c_int / 2 as core::ffi::c_int {
        rdmult
    } else {
        orig_rdmult * 3 as core::ffi::c_int / 2 as core::ffi::c_int
    };
    rdmult = if rdmult > orig_rdmult * 1 as core::ffi::c_int / 2 as core::ffi::c_int {
        rdmult
    } else {
        orig_rdmult * 1 as core::ffi::c_int / 2 as core::ffi::c_int
    };
    rdmult = if 1 as core::ffi::c_int > rdmult { 1 as core::ffi::c_int } else { rdmult };
    return rdmult;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_active_h_edge(
    mut cpi: *const AV1_COMP,
    mut mi_row: core::ffi::c_int,
    mut mi_step: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut top_edge: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bottom_edge: core::ffi::c_int = (*cpi).common.mi_params.mi_rows;
    let mut is_active_h_edge: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_stat_consumption_stage_twopass(cpi) != 0 {
        let cm: *const AV1_COMMON = &(*cpi).common;
        let this_frame_stats: *const FIRSTPASS_STATS = read_one_frame_stats(
            &raw mut (*(*cpi).ppi).twopass,
            (*cm).current_frame.display_order_hint as core::ffi::c_int,
        ) as *const FIRSTPASS_STATS;
        if this_frame_stats.is_null() {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        top_edge
            += ((*this_frame_stats).inactive_zone_rows
                * 4 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int;
        bottom_edge
            -= ((*this_frame_stats).inactive_zone_rows
                * 4 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int;
        bottom_edge = if top_edge > bottom_edge { top_edge } else { bottom_edge };
    }
    if top_edge >= mi_row && top_edge < mi_row + mi_step
        || bottom_edge >= mi_row && bottom_edge < mi_row + mi_step
    {
        is_active_h_edge = 1 as core::ffi::c_int;
    }
    return is_active_h_edge;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_active_v_edge(
    mut cpi: *const AV1_COMP,
    mut mi_col: core::ffi::c_int,
    mut mi_step: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut left_edge: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut right_edge: core::ffi::c_int = (*cpi).common.mi_params.mi_cols;
    let mut is_active_v_edge: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_stat_consumption_stage_twopass(cpi) != 0 {
        let cm: *const AV1_COMMON = &(*cpi).common;
        let this_frame_stats: *const FIRSTPASS_STATS = read_one_frame_stats(
            &raw mut (*(*cpi).ppi).twopass,
            (*cm).current_frame.display_order_hint as core::ffi::c_int,
        ) as *const FIRSTPASS_STATS;
        if this_frame_stats.is_null() {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        left_edge
            += ((*this_frame_stats).inactive_zone_cols
                * 4 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int;
        right_edge
            -= ((*this_frame_stats).inactive_zone_cols
                * 4 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int;
        right_edge = if left_edge > right_edge { left_edge } else { right_edge };
    }
    if left_edge >= mi_col && left_edge < mi_col + mi_step
        || right_edge >= mi_col && right_edge < mi_col + mi_step
    {
        is_active_v_edge = 1 as core::ffi::c_int;
    }
    return is_active_v_edge;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_tpl_stats_sb(
    mut cpi: *mut AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut sb_enc: *mut SuperBlockEnc,
) {
    (*sb_enc).tpl_data_count = 0 as core::ffi::c_int;
    if !(*cpi).oxcf.algo_cfg.enable_tpl_model {
        return;
    }
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        return;
    }
    let update_type: FRAME_UPDATE_TYPE = get_frame_update_type(
        &raw mut (*(*cpi).ppi).gf_group,
        (*cpi).gf_frame_index as core::ffi::c_int,
    ) as FRAME_UPDATE_TYPE;
    if update_type as core::ffi::c_int == INTNL_OVERLAY_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == OVERLAY_UPDATE as core::ffi::c_int
    {
        return;
    }
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_get_tpl_stats_sb\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            835 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let gf_group_index: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    if av1_tpl_stats_ready(tpl_data, gf_group_index) == 0 {
        return;
    }
    let mi_wide: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_high: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut tpl_frame: *mut TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(gf_group_index as isize) as *mut TplDepFrame;
    let mut tpl_stats: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let mut tpl_stride: core::ffi::c_int = (*tpl_frame).stride;
    let mut mi_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_col_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_col_end_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col + mi_wide,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_cols_sr: core::ffi::c_int = av1_pixels_to_mi((*cm).superres_upscaled_width)
        as core::ffi::c_int;
    let tpl_bsize: BLOCK_SIZE = convert_length_to_bsize(
        (*tpl_data).tpl_bsize_1d as core::ffi::c_int,
    ) as BLOCK_SIZE;
    if !(mi_size_wide[tpl_bsize as usize] as core::ffi::c_int
        == mi_size_high[tpl_bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_get_tpl_stats_sb\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            861 as core::ffi::c_int,
            b"mi_size_wide[tpl_bsize] == mi_size_high[tpl_bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let row_step: core::ffi::c_int = mi_size_high[tpl_bsize as usize]
        as core::ffi::c_int;
    let col_step_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_size_wide[tpl_bsize as usize] as core::ffi::c_int,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    (*sb_enc).tpl_stride = (mi_col_end_sr - mi_col_sr) / col_step_sr;
    let mut row: core::ffi::c_int = mi_row;
    while row < mi_row + mi_high {
        let mut col: core::ffi::c_int = mi_col_sr;
        while col < mi_col_end_sr {
            if !(count
                < ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    / 16 as core::ffi::c_int
                    * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                        / 16 as core::ffi::c_int)) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"av1_get_tpl_stats_sb\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    872 as core::ffi::c_int,
                    b"count < MAX_TPL_BLK_IN_SB * MAX_TPL_BLK_IN_SB\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if row >= (*cm).mi_params.mi_rows || col >= mi_cols_sr {
                (*sb_enc).tpl_inter_cost[count as usize] = INT64_MAX as int64_t;
                (*sb_enc).tpl_intra_cost[count as usize] = INT64_MAX as int64_t;
                let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
                    (*sb_enc).tpl_mv[count as usize][i as usize].as_int = INVALID_MV
                        as uint32_t;
                    i += 1;
                }
                count += 1;
            } else {
                let mut this_stats: *mut TplDepStats = &raw mut *tpl_stats
                    .offset(
                        (av1_tpl_ptr_pos
                            as unsafe extern "C-unwind" fn(
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                uint8_t,
                            ) -> core::ffi::c_int)(
                            row,
                            col,
                            tpl_stride,
                            (*tpl_data).tpl_stats_block_mis_log2,
                        ) as isize,
                    ) as *mut TplDepStats;
                (*sb_enc).tpl_inter_cost[count as usize] = ((*this_stats).inter_cost
                    << TPL_DEP_COST_SCALE_LOG2) as int64_t;
                (*sb_enc).tpl_intra_cost[count as usize] = ((*this_stats).intra_cost
                    << TPL_DEP_COST_SCALE_LOG2) as int64_t;
                memcpy(
                    ((*sb_enc).tpl_mv[count as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    ((*this_stats).mv).as_mut_ptr() as *const core::ffi::c_void,
                    ::core::mem::size_of::<[int_mv; 7]>() as size_t,
                );
                mi_count += 1;
                count += 1;
            }
            col += col_step_sr;
        }
        row += row_step;
    }
    if !(mi_count
        <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) / 16 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                / 16 as core::ffi::c_int)) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_get_tpl_stats_sb\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            896 as core::ffi::c_int,
            b"mi_count <= MAX_TPL_BLK_IN_SB * MAX_TPL_BLK_IN_SB\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    (*sb_enc).tpl_data_count = mi_count;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_q_for_deltaq_objective(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut delta_dist: *mut int64_t,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"av1_get_q_for_deltaq_objective\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            909 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let tpl_idx: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let block_mis_log2: uint8_t = (*tpl_data).tpl_stats_block_mis_log2;
    let mut intra_cost: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut mc_dep_reg: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut mc_dep_cost: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut cbcmp_base: core::ffi::c_double = 1 as core::ffi::c_int
        as core::ffi::c_double;
    let mut srcrf_dist: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut srcrf_sse: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut srcrf_rate: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mi_wide: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_high: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let base_qindex: core::ffi::c_int = (*cm).quant_params.base_qindex;
    if tpl_idx >= MAX_TPL_FRAME_IDX {
        return base_qindex;
    }
    let mut tpl_frame: *mut TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(tpl_idx as isize) as *mut TplDepFrame;
    let mut tpl_stats: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let mut tpl_stride: core::ffi::c_int = (*tpl_frame).stride;
    if (*tpl_frame).is_valid == 0 {
        return base_qindex;
    }
    let mut mi_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_col_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_col_end_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col + mi_wide,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_cols_sr: core::ffi::c_int = av1_pixels_to_mi((*cm).superres_upscaled_width)
        as core::ffi::c_int;
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << block_mis_log2 as core::ffi::c_int;
    let row_step: core::ffi::c_int = step;
    let col_step_sr: core::ffi::c_int = coded_to_superres_mi(
        step,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut row: core::ffi::c_int = mi_row;
    while row < mi_row + mi_high {
        let mut col: core::ffi::c_int = mi_col_sr;
        while col < mi_col_end_sr {
            if !(row >= (*cm).mi_params.mi_rows || col >= mi_cols_sr) {
                let mut this_stats: *mut TplDepStats = &raw mut *tpl_stats
                    .offset(
                        (av1_tpl_ptr_pos
                            as unsafe extern "C-unwind" fn(
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                uint8_t,
                            ) -> core::ffi::c_int)(row, col, tpl_stride, block_mis_log2)
                            as isize,
                    ) as *mut TplDepStats;
                let mut cbcmp: core::ffi::c_double = (*this_stats).srcrf_dist
                    as core::ffi::c_double;
                let mut mc_dep_delta: int64_t = ((*this_stats).mc_dep_rate
                    * (*tpl_frame).base_rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*this_stats).mc_dep_dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                let mut dist_scaled: core::ffi::c_double = ((*this_stats).recrf_dist
                    << RDDIV_BITS) as core::ffi::c_double;
                intra_cost += log(dist_scaled) * cbcmp;
                mc_dep_cost
                    += log(dist_scaled + mc_dep_delta as core::ffi::c_double) * cbcmp;
                mc_dep_reg
                    += log(
                        3 as core::ffi::c_int as core::ffi::c_double * dist_scaled
                            + mc_dep_delta as core::ffi::c_double,
                    ) * cbcmp;
                srcrf_dist
                    += ((*this_stats).srcrf_dist << RDDIV_BITS) as core::ffi::c_double;
                srcrf_sse
                    += ((*this_stats).srcrf_sse << RDDIV_BITS) as core::ffi::c_double;
                srcrf_rate
                    += ((*this_stats).srcrf_rate << TPL_DEP_COST_SCALE_LOG2)
                        as core::ffi::c_double;
                mi_count += 1;
                cbcmp_base += cbcmp;
            }
            col += col_step_sr;
        }
        row += row_step;
    }
    if !(mi_count
        <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) / 16 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                / 16 as core::ffi::c_int)) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"av1_get_q_for_deltaq_objective\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            965 as core::ffi::c_int,
            b"mi_count <= MAX_TPL_BLK_IN_SB * MAX_TPL_BLK_IN_SB\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut beta: core::ffi::c_double = 1.0f64;
    let mut rk: core::ffi::c_double = 0.;
    if mc_dep_cost > 0 as core::ffi::c_int as core::ffi::c_double
        && intra_cost > 0 as core::ffi::c_int as core::ffi::c_double
    {
        let r0: core::ffi::c_double = (*cpi).rd.r0;
        rk = exp((intra_cost - mc_dep_cost) / cbcmp_base);
        (*td).mb.rb = exp((intra_cost - mc_dep_reg) / cbcmp_base);
        beta = r0 / rk;
        if !(beta > 0.0f64) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 31],
                    [core::ffi::c_char; 31],
                >(*b"av1_get_q_for_deltaq_objective\0"))
                    .as_ptr(),
                b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                975 as core::ffi::c_int,
                b"beta > 0.0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    } else {
        return base_qindex
    }
    offset = av1_get_deltaq_offset((*(*cm).seq_params).bit_depth, base_qindex, beta);
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    offset = if offset
        < (*delta_q_info).delta_q_res * 9 as core::ffi::c_int - 1 as core::ffi::c_int
    {
        offset
    } else {
        (*delta_q_info).delta_q_res * 9 as core::ffi::c_int - 1 as core::ffi::c_int
    };
    offset = if offset
        > -(*delta_q_info).delta_q_res * 9 as core::ffi::c_int + 1 as core::ffi::c_int
    {
        offset
    } else {
        -(*delta_q_info).delta_q_res * 9 as core::ffi::c_int + 1 as core::ffi::c_int
    };
    let mut qindex: core::ffi::c_int = (*cm).quant_params.base_qindex + offset;
    qindex = if qindex < 255 as core::ffi::c_int {
        qindex
    } else {
        255 as core::ffi::c_int
    };
    qindex = if qindex > 0 as core::ffi::c_int { qindex } else { 0 as core::ffi::c_int };
    let mut frm_qstep: core::ffi::c_int = av1_dc_quant_QTX(
        base_qindex,
        0 as core::ffi::c_int,
        (*(*cm).seq_params).bit_depth,
    ) as core::ffi::c_int;
    let mut sbs_qstep: core::ffi::c_int = av1_dc_quant_QTX(
        base_qindex,
        offset,
        (*(*cm).seq_params).bit_depth,
    ) as core::ffi::c_int;
    if !delta_dist.is_null() {
        let mut sbs_dist: core::ffi::c_double = srcrf_dist
            * pow(
                sbs_qstep as core::ffi::c_double / frm_qstep as core::ffi::c_double,
                2.0f64,
            );
        let mut sbs_rate: core::ffi::c_double = srcrf_rate
            * (frm_qstep as core::ffi::c_double / sbs_qstep as core::ffi::c_double);
        sbs_dist = if sbs_dist < srcrf_sse { sbs_dist } else { srcrf_sse };
        *delta_dist = ((sbs_dist - srcrf_dist) / rk) as int64_t;
        *delta_dist
            += ((4 as core::ffi::c_int * 256 as core::ffi::c_int) as int64_t
                * (*tpl_frame).base_rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                    as int64_t;
        *delta_dist
            += ((sbs_rate - srcrf_rate) as int64_t * (*tpl_frame).base_rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                    as int64_t;
    }
    return qindex;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_q_for_hdr(
    cpi: *mut AV1_COMP,
    x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    if !((*(*cm).seq_params).bit_depth as core::ffi::c_uint
        == AOM_BITS_10 as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_get_q_for_hdr\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            1015 as core::ffi::c_int,
            b"cm->seq_params->bit_depth == AOM_BITS_10\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return (*cm).quant_params.base_qindex;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_simple_motion_tree_partition(
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut bsize: BLOCK_SIZE,
) {
    if sms_tree.is_null() {
        return;
    }
    (*sms_tree).partitioning = PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
    if bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int {
        let mut subsize: BLOCK_SIZE = get_partition_subsize(
            bsize,
            PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
        );
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < 4 as core::ffi::c_int {
            av1_reset_simple_motion_tree_partition(
                (*sms_tree).split[idx as usize],
                subsize,
            );
            idx += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_update_picked_ref_frames_mask(
    x: *mut MACROBLOCK,
    mut ref_type: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mib_size: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    if !(mi_size_wide[bsize as usize] as core::ffi::c_int
        == mi_size_high[bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_update_picked_ref_frames_mask\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            1063 as core::ffi::c_int,
            b"mi_size_wide[bsize] == mi_size_high[bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let sb_size_mask: core::ffi::c_int = mib_size - 1 as core::ffi::c_int;
    let mi_row_in_sb: core::ffi::c_int = mi_row & sb_size_mask;
    let mi_col_in_sb: core::ffi::c_int = mi_col & sb_size_mask;
    let mi_size: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mut i: core::ffi::c_int = mi_row_in_sb;
    while i < mi_row_in_sb + mi_size {
        let mut j: core::ffi::c_int = mi_col_in_sb;
        while j < mi_col_in_sb + mi_size {
            (*x).picked_ref_frames_mask[(i * 32 as core::ffi::c_int + j) as usize]
                |= (1 as core::ffi::c_int) << ref_type;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn avg_cdf_symbol(
    mut cdf_ptr_left: *mut aom_cdf_prob,
    mut cdf_ptr_tr: *mut aom_cdf_prob,
    mut num_cdfs: core::ffi::c_int,
    mut cdf_stride: core::ffi::c_int,
    mut nsymbs: core::ffi::c_int,
    mut wt_left: core::ffi::c_int,
    mut wt_tr: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_cdfs {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j <= nsymbs {
            *cdf_ptr_left.offset((i * cdf_stride + j) as isize) = ((*cdf_ptr_left
                .offset((i * cdf_stride + j) as isize) as core::ffi::c_int * wt_left
                + *cdf_ptr_tr.offset((i * cdf_stride + j) as isize) as core::ffi::c_int
                    * wt_tr + (wt_left + wt_tr) / 2 as core::ffi::c_int)
                / (wt_left + wt_tr)) as aom_cdf_prob;
            if !(*cdf_ptr_left.offset((i * cdf_stride + j) as isize) as core::ffi::c_int
                >= 0 as core::ffi::c_int
                && (*cdf_ptr_left.offset((i * cdf_stride + j) as isize)
                    as core::ffi::c_int)
                    < (1 as core::ffi::c_int) << 15 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 15],
                        [core::ffi::c_char; 15],
                    >(*b"avg_cdf_symbol\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    1086 as core::ffi::c_int,
                    b"cdf_ptr_left[i * cdf_stride + j] >= 0 && cdf_ptr_left[i * cdf_stride + j] < CDF_PROB_TOP\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn avg_nmv(
    mut nmv_left: *mut nmv_context,
    mut nmv_tr: *mut nmv_context,
    mut wt_left: core::ffi::c_int,
    mut wt_tr: core::ffi::c_int,
) {
    let mut cdf_ptr_left: *mut aom_cdf_prob = ((*nmv_left).joints_cdf).as_mut_ptr();
    let mut cdf_ptr_tr: *mut aom_cdf_prob = ((*nmv_tr).joints_cdf).as_mut_ptr();
    let mut array_size: core::ffi::c_int = (::core::mem::size_of::<[aom_cdf_prob; 5]>()
        as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs: core::ffi::c_int = array_size
        / (4 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left,
        cdf_ptr_tr,
        num_cdfs,
        4 as core::ffi::c_int + 1 as core::ffi::c_int,
        4 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let mut cdf_ptr_left_0: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .classes_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_0: *mut aom_cdf_prob = ((*nmv_tr)
            .comps[i as usize]
            .classes_cdf)
            .as_mut_ptr();
        let mut array_size_0: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 12],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_0: core::ffi::c_int = array_size_0
            / (11 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_0,
            cdf_ptr_tr_0,
            num_cdfs_0,
            11 as core::ffi::c_int + 1 as core::ffi::c_int,
            11 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_1: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .class0_fp_cdf)
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut cdf_ptr_tr_1: *mut aom_cdf_prob = ((*nmv_tr)
            .comps[i as usize]
            .class0_fp_cdf)
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut array_size_1: core::ffi::c_int = (::core::mem::size_of::<
            [[aom_cdf_prob; 5]; 2],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_1: core::ffi::c_int = array_size_1
            / (4 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_1,
            cdf_ptr_tr_1,
            num_cdfs_1,
            4 as core::ffi::c_int + 1 as core::ffi::c_int,
            4 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_2: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .fp_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_2: *mut aom_cdf_prob = ((*nmv_tr).comps[i as usize].fp_cdf)
            .as_mut_ptr();
        let mut array_size_2: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 5],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_2: core::ffi::c_int = array_size_2
            / (4 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_2,
            cdf_ptr_tr_2,
            num_cdfs_2,
            4 as core::ffi::c_int + 1 as core::ffi::c_int,
            4 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_3: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .sign_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_3: *mut aom_cdf_prob = ((*nmv_tr).comps[i as usize].sign_cdf)
            .as_mut_ptr();
        let mut array_size_3: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 3],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_3: core::ffi::c_int = array_size_3
            / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_3,
            cdf_ptr_tr_3,
            num_cdfs_3,
            2 as core::ffi::c_int + 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_4: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .class0_hp_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_4: *mut aom_cdf_prob = ((*nmv_tr)
            .comps[i as usize]
            .class0_hp_cdf)
            .as_mut_ptr();
        let mut array_size_4: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 3],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_4: core::ffi::c_int = array_size_4
            / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_4,
            cdf_ptr_tr_4,
            num_cdfs_4,
            2 as core::ffi::c_int + 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_5: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .hp_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_5: *mut aom_cdf_prob = ((*nmv_tr).comps[i as usize].hp_cdf)
            .as_mut_ptr();
        let mut array_size_5: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 3],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_5: core::ffi::c_int = array_size_5
            / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_5,
            cdf_ptr_tr_5,
            num_cdfs_5,
            2 as core::ffi::c_int + 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_6: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .class0_cdf)
            .as_mut_ptr();
        let mut cdf_ptr_tr_6: *mut aom_cdf_prob = ((*nmv_tr)
            .comps[i as usize]
            .class0_cdf)
            .as_mut_ptr();
        let mut array_size_6: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 3],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_6: core::ffi::c_int = array_size_6
            / (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_6,
            cdf_ptr_tr_6,
            num_cdfs_6,
            ((1 as core::ffi::c_int) << 1 as core::ffi::c_int) + 1 as core::ffi::c_int,
            (1 as core::ffi::c_int) << 1 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_7: *mut aom_cdf_prob = ((*nmv_left)
            .comps[i as usize]
            .bits_cdf)
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut cdf_ptr_tr_7: *mut aom_cdf_prob = ((*nmv_tr).comps[i as usize].bits_cdf)
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut array_size_7: core::ffi::c_int = (::core::mem::size_of::<
            [[aom_cdf_prob; 3]; 10],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_7: core::ffi::c_int = array_size_7
            / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_7,
            cdf_ptr_tr_7,
            num_cdfs_7,
            2 as core::ffi::c_int + 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_avg_cdf_symbols(
    mut ctx_left: *mut FRAME_CONTEXT,
    mut ctx_tr: *mut FRAME_CONTEXT,
    mut wt_left: core::ffi::c_int,
    mut wt_tr: core::ffi::c_int,
) {
    let mut cdf_ptr_left: *mut aom_cdf_prob = ((*ctx_left).txb_skip_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr: *mut aom_cdf_prob = ((*ctx_tr).txb_skip_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 13]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs: core::ffi::c_int = array_size
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left,
        cdf_ptr_tr,
        num_cdfs,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_0: *mut aom_cdf_prob = ((*ctx_left).eob_extra_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_0: *mut aom_cdf_prob = ((*ctx_tr).eob_extra_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_0: core::ffi::c_int = (::core::mem::size_of::<
        [[[[aom_cdf_prob; 3]; 9]; 2]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_0: core::ffi::c_int = array_size_0
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_0,
        cdf_ptr_tr_0,
        num_cdfs_0,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_1: *mut aom_cdf_prob = ((*ctx_left).dc_sign_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_1: *mut aom_cdf_prob = ((*ctx_tr).dc_sign_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_1: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 3]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_1: core::ffi::c_int = array_size_1
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_1,
        cdf_ptr_tr_1,
        num_cdfs_1,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_2: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf16).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_2: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf16).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_2: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 6]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_2: core::ffi::c_int = array_size_2
        / (5 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_2,
        cdf_ptr_tr_2,
        num_cdfs_2,
        5 as core::ffi::c_int + 1 as core::ffi::c_int,
        5 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_3: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf32).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_3: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf32).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_3: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 7]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_3: core::ffi::c_int = array_size_3
        / (6 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_3,
        cdf_ptr_tr_3,
        num_cdfs_3,
        6 as core::ffi::c_int + 1 as core::ffi::c_int,
        6 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_4: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf64).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_4: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf64).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_4: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 8]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_4: core::ffi::c_int = array_size_4
        / (7 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_4,
        cdf_ptr_tr_4,
        num_cdfs_4,
        7 as core::ffi::c_int + 1 as core::ffi::c_int,
        7 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_5: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf128)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_5: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf128).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_5: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 9]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_5: core::ffi::c_int = array_size_5
        / (8 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_5,
        cdf_ptr_tr_5,
        num_cdfs_5,
        8 as core::ffi::c_int + 1 as core::ffi::c_int,
        8 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_6: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf256)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_6: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf256).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_6: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 10]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_6: core::ffi::c_int = array_size_6
        / (9 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_6,
        cdf_ptr_tr_6,
        num_cdfs_6,
        9 as core::ffi::c_int + 1 as core::ffi::c_int,
        9 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_7: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf512)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_7: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf512).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_7: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 11]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_7: core::ffi::c_int = array_size_7
        / (10 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_7,
        cdf_ptr_tr_7,
        num_cdfs_7,
        10 as core::ffi::c_int + 1 as core::ffi::c_int,
        10 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_8: *mut aom_cdf_prob = ((*ctx_left).eob_flag_cdf1024)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_8: *mut aom_cdf_prob = ((*ctx_tr).eob_flag_cdf1024).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_8: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 12]; 2]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_8: core::ffi::c_int = array_size_8
        / (11 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_8,
        cdf_ptr_tr_8,
        num_cdfs_8,
        11 as core::ffi::c_int + 1 as core::ffi::c_int,
        11 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_9: *mut aom_cdf_prob = ((*ctx_left).coeff_base_eob_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_9: *mut aom_cdf_prob = ((*ctx_tr).coeff_base_eob_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_9: core::ffi::c_int = (::core::mem::size_of::<
        [[[[aom_cdf_prob; 4]; 4]; 2]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_9: core::ffi::c_int = array_size_9
        / (3 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_9,
        cdf_ptr_tr_9,
        num_cdfs_9,
        3 as core::ffi::c_int + 1 as core::ffi::c_int,
        3 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_10: *mut aom_cdf_prob = ((*ctx_left).coeff_base_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_10: *mut aom_cdf_prob = ((*ctx_tr).coeff_base_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_10: core::ffi::c_int = (::core::mem::size_of::<
        [[[[aom_cdf_prob; 5]; 42]; 2]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_10: core::ffi::c_int = array_size_10
        / (4 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_10,
        cdf_ptr_tr_10,
        num_cdfs_10,
        4 as core::ffi::c_int + 1 as core::ffi::c_int,
        4 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_11: *mut aom_cdf_prob = ((*ctx_left).coeff_br_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_11: *mut aom_cdf_prob = ((*ctx_tr).coeff_br_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_11: core::ffi::c_int = (::core::mem::size_of::<
        [[[[aom_cdf_prob; 5]; 21]; 2]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_11: core::ffi::c_int = array_size_11
        / (4 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_11,
        cdf_ptr_tr_11,
        num_cdfs_11,
        4 as core::ffi::c_int + 1 as core::ffi::c_int,
        4 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_12: *mut aom_cdf_prob = ((*ctx_left).newmv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_12: *mut aom_cdf_prob = ((*ctx_tr).newmv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_12: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_12: core::ffi::c_int = array_size_12
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_12,
        cdf_ptr_tr_12,
        num_cdfs_12,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_13: *mut aom_cdf_prob = ((*ctx_left).zeromv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_13: *mut aom_cdf_prob = ((*ctx_tr).zeromv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_13: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_13: core::ffi::c_int = array_size_13
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_13,
        cdf_ptr_tr_13,
        num_cdfs_13,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_14: *mut aom_cdf_prob = ((*ctx_left).refmv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_14: *mut aom_cdf_prob = ((*ctx_tr).refmv_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_14: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_14: core::ffi::c_int = array_size_14
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_14,
        cdf_ptr_tr_14,
        num_cdfs_14,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_15: *mut aom_cdf_prob = ((*ctx_left).drl_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_15: *mut aom_cdf_prob = ((*ctx_tr).drl_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_15: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_15: core::ffi::c_int = array_size_15
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_15,
        cdf_ptr_tr_15,
        num_cdfs_15,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_16: *mut aom_cdf_prob = ((*ctx_left).inter_compound_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_16: *mut aom_cdf_prob = ((*ctx_tr).inter_compound_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_16: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 9]; 8],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_16: core::ffi::c_int = array_size_16
        / (1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
            - NEAREST_NEARESTMV as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_16,
        cdf_ptr_tr_16,
        num_cdfs_16,
        1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
            - NEAREST_NEARESTMV as core::ffi::c_int + 1 as core::ffi::c_int,
        1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
            - NEAREST_NEARESTMV as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_17: *mut aom_cdf_prob = ((*ctx_left).compound_type_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_17: *mut aom_cdf_prob = ((*ctx_tr).compound_type_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_17: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_17: core::ffi::c_int = array_size_17
        / (MASKED_COMPOUND_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_17,
        cdf_ptr_tr_17,
        num_cdfs_17,
        MASKED_COMPOUND_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        MASKED_COMPOUND_TYPES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_18: *mut aom_cdf_prob = ((*ctx_left).wedge_idx_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_18: *mut aom_cdf_prob = ((*ctx_tr).wedge_idx_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_18: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 17]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_18: core::ffi::c_int = array_size_18
        / (16 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_18,
        cdf_ptr_tr_18,
        num_cdfs_18,
        16 as core::ffi::c_int + 1 as core::ffi::c_int,
        16 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_19: *mut aom_cdf_prob = ((*ctx_left).interintra_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_19: *mut aom_cdf_prob = ((*ctx_tr).interintra_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_19: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_19: core::ffi::c_int = array_size_19
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_19,
        cdf_ptr_tr_19,
        num_cdfs_19,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_20: *mut aom_cdf_prob = ((*ctx_left).wedge_interintra_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_20: *mut aom_cdf_prob = ((*ctx_tr).wedge_interintra_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_20: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_20: core::ffi::c_int = array_size_20
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_20,
        cdf_ptr_tr_20,
        num_cdfs_20,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_21: *mut aom_cdf_prob = ((*ctx_left).interintra_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_21: *mut aom_cdf_prob = ((*ctx_tr).interintra_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_21: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 5]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_21: core::ffi::c_int = array_size_21
        / (INTERINTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_21,
        cdf_ptr_tr_21,
        num_cdfs_21,
        INTERINTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        INTERINTRA_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_22: *mut aom_cdf_prob = ((*ctx_left).motion_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_22: *mut aom_cdf_prob = ((*ctx_tr).motion_mode_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_22: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_22: core::ffi::c_int = array_size_22
        / (MOTION_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_22,
        cdf_ptr_tr_22,
        num_cdfs_22,
        MOTION_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        MOTION_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_23: *mut aom_cdf_prob = ((*ctx_left).obmc_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_23: *mut aom_cdf_prob = ((*ctx_tr).obmc_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_23: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_23: core::ffi::c_int = array_size_23
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_23,
        cdf_ptr_tr_23,
        num_cdfs_23,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_24: *mut aom_cdf_prob = ((*ctx_left).palette_y_size_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_24: *mut aom_cdf_prob = ((*ctx_tr).palette_y_size_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_24: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 8]; 7],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_24: core::ffi::c_int = array_size_24
        / (PALETTE_SIZES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_24,
        cdf_ptr_tr_24,
        num_cdfs_24,
        PALETTE_SIZES as core::ffi::c_int + 1 as core::ffi::c_int,
        PALETTE_SIZES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_25: *mut aom_cdf_prob = ((*ctx_left).palette_uv_size_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_25: *mut aom_cdf_prob = ((*ctx_tr).palette_uv_size_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_25: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 8]; 7],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_25: core::ffi::c_int = array_size_25
        / (PALETTE_SIZES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_25,
        cdf_ptr_tr_25,
        num_cdfs_25,
        PALETTE_SIZES as core::ffi::c_int + 1 as core::ffi::c_int,
        PALETTE_SIZES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < PALETTE_SIZES as core::ffi::c_int {
        let mut nsymbs: core::ffi::c_int = j + PALETTE_MIN_SIZE;
        let mut cdf_ptr_left_26: *mut aom_cdf_prob = ((*ctx_left)
            .palette_y_color_index_cdf[j as usize])
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut cdf_ptr_tr_26: *mut aom_cdf_prob = ((*ctx_tr)
            .palette_y_color_index_cdf[j as usize])
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut array_size_26: core::ffi::c_int = (::core::mem::size_of::<
            [[aom_cdf_prob; 9]; 5],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_26: core::ffi::c_int = array_size_26
            / (PALETTE_COLORS as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_26,
            cdf_ptr_tr_26,
            num_cdfs_26,
            PALETTE_COLORS as core::ffi::c_int + 1 as core::ffi::c_int,
            nsymbs,
            wt_left,
            wt_tr,
        );
        let mut cdf_ptr_left_27: *mut aom_cdf_prob = ((*ctx_left)
            .palette_uv_color_index_cdf[j as usize])
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut cdf_ptr_tr_27: *mut aom_cdf_prob = ((*ctx_tr)
            .palette_uv_color_index_cdf[j as usize])
            .as_mut_ptr() as *mut aom_cdf_prob;
        let mut array_size_27: core::ffi::c_int = (::core::mem::size_of::<
            [[aom_cdf_prob; 9]; 5],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_27: core::ffi::c_int = array_size_27
            / (PALETTE_COLORS as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_27,
            cdf_ptr_tr_27,
            num_cdfs_27,
            PALETTE_COLORS as core::ffi::c_int + 1 as core::ffi::c_int,
            nsymbs,
            wt_left,
            wt_tr,
        );
        j += 1;
    }
    let mut cdf_ptr_left_28: *mut aom_cdf_prob = ((*ctx_left).palette_y_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_28: *mut aom_cdf_prob = ((*ctx_tr).palette_y_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_28: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 3]; 7],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_28: core::ffi::c_int = array_size_28
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_28,
        cdf_ptr_tr_28,
        num_cdfs_28,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_29: *mut aom_cdf_prob = ((*ctx_left).palette_uv_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_29: *mut aom_cdf_prob = ((*ctx_tr).palette_uv_mode_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_29: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 2],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_29: core::ffi::c_int = array_size_29
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_29,
        cdf_ptr_tr_29,
        num_cdfs_29,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_30: *mut aom_cdf_prob = ((*ctx_left).comp_inter_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_30: *mut aom_cdf_prob = ((*ctx_tr).comp_inter_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_30: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_30: core::ffi::c_int = array_size_30
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_30,
        cdf_ptr_tr_30,
        num_cdfs_30,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_31: *mut aom_cdf_prob = ((*ctx_left).single_ref_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_31: *mut aom_cdf_prob = ((*ctx_tr).single_ref_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_31: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 6]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_31: core::ffi::c_int = array_size_31
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_31,
        cdf_ptr_tr_31,
        num_cdfs_31,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_32: *mut aom_cdf_prob = ((*ctx_left).comp_ref_type_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_32: *mut aom_cdf_prob = ((*ctx_tr).comp_ref_type_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_32: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_32: core::ffi::c_int = array_size_32
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_32,
        cdf_ptr_tr_32,
        num_cdfs_32,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_33: *mut aom_cdf_prob = ((*ctx_left).uni_comp_ref_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_33: *mut aom_cdf_prob = ((*ctx_tr).uni_comp_ref_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_33: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_33: core::ffi::c_int = array_size_33
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_33,
        cdf_ptr_tr_33,
        num_cdfs_33,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_34: *mut aom_cdf_prob = ((*ctx_left).comp_ref_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_34: *mut aom_cdf_prob = ((*ctx_tr).comp_ref_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_34: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_34: core::ffi::c_int = array_size_34
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_34,
        cdf_ptr_tr_34,
        num_cdfs_34,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_35: *mut aom_cdf_prob = ((*ctx_left).comp_bwdref_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_35: *mut aom_cdf_prob = ((*ctx_tr).comp_bwdref_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_35: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 3]; 2]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_35: core::ffi::c_int = array_size_35
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_35,
        cdf_ptr_tr_35,
        num_cdfs_35,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_36: *mut aom_cdf_prob = ((*ctx_left).txfm_partition_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_36: *mut aom_cdf_prob = ((*ctx_tr).txfm_partition_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_36: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 21],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_36: core::ffi::c_int = array_size_36
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_36,
        cdf_ptr_tr_36,
        num_cdfs_36,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_37: *mut aom_cdf_prob = ((*ctx_left).compound_index_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_37: *mut aom_cdf_prob = ((*ctx_tr).compound_index_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_37: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_37: core::ffi::c_int = array_size_37
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_37,
        cdf_ptr_tr_37,
        num_cdfs_37,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_38: *mut aom_cdf_prob = ((*ctx_left).comp_group_idx_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_38: *mut aom_cdf_prob = ((*ctx_tr).comp_group_idx_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_38: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_38: core::ffi::c_int = array_size_38
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_38,
        cdf_ptr_tr_38,
        num_cdfs_38,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_39: *mut aom_cdf_prob = ((*ctx_left).skip_mode_cdfs)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_39: *mut aom_cdf_prob = ((*ctx_tr).skip_mode_cdfs).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_39: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_39: core::ffi::c_int = array_size_39
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_39,
        cdf_ptr_tr_39,
        num_cdfs_39,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_40: *mut aom_cdf_prob = ((*ctx_left).skip_txfm_cdfs)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_40: *mut aom_cdf_prob = ((*ctx_tr).skip_txfm_cdfs).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_40: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_40: core::ffi::c_int = array_size_40
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_40,
        cdf_ptr_tr_40,
        num_cdfs_40,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_41: *mut aom_cdf_prob = ((*ctx_left).intra_inter_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_41: *mut aom_cdf_prob = ((*ctx_tr).intra_inter_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_41: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_41: core::ffi::c_int = array_size_41
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_41,
        cdf_ptr_tr_41,
        num_cdfs_41,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    avg_nmv(&raw mut (*ctx_left).nmvc, &raw mut (*ctx_tr).nmvc, wt_left, wt_tr);
    avg_nmv(&raw mut (*ctx_left).ndvc, &raw mut (*ctx_tr).ndvc, wt_left, wt_tr);
    let mut cdf_ptr_left_42: *mut aom_cdf_prob = ((*ctx_left).intrabc_cdf).as_mut_ptr();
    let mut cdf_ptr_tr_42: *mut aom_cdf_prob = ((*ctx_tr).intrabc_cdf).as_mut_ptr();
    let mut array_size_42: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_42: core::ffi::c_int = array_size_42
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_42,
        cdf_ptr_tr_42,
        num_cdfs_42,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_43: *mut aom_cdf_prob = ((*ctx_left).seg.pred_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_43: *mut aom_cdf_prob = ((*ctx_tr).seg.pred_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_43: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_43: core::ffi::c_int = array_size_43
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_43,
        cdf_ptr_tr_43,
        num_cdfs_43,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_44: *mut aom_cdf_prob = ((*ctx_left).seg.spatial_pred_seg_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_44: *mut aom_cdf_prob = ((*ctx_tr).seg.spatial_pred_seg_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_44: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 9]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_44: core::ffi::c_int = array_size_44
        / (8 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_44,
        cdf_ptr_tr_44,
        num_cdfs_44,
        8 as core::ffi::c_int + 1 as core::ffi::c_int,
        8 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_45: *mut aom_cdf_prob = ((*ctx_left).filter_intra_cdfs)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_45: *mut aom_cdf_prob = ((*ctx_tr).filter_intra_cdfs).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_45: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 3]; 22],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_45: core::ffi::c_int = array_size_45
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_45,
        cdf_ptr_tr_45,
        num_cdfs_45,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_46: *mut aom_cdf_prob = ((*ctx_left).filter_intra_mode_cdf)
        .as_mut_ptr();
    let mut cdf_ptr_tr_46: *mut aom_cdf_prob = ((*ctx_tr).filter_intra_mode_cdf)
        .as_mut_ptr();
    let mut array_size_46: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_46: core::ffi::c_int = array_size_46
        / (FILTER_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_46,
        cdf_ptr_tr_46,
        num_cdfs_46,
        FILTER_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        FILTER_INTRA_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_47: *mut aom_cdf_prob = ((*ctx_left).switchable_restore_cdf)
        .as_mut_ptr();
    let mut cdf_ptr_tr_47: *mut aom_cdf_prob = ((*ctx_tr).switchable_restore_cdf)
        .as_mut_ptr();
    let mut array_size_47: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_47: core::ffi::c_int = array_size_47
        / (RESTORE_SWITCHABLE_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_47,
        cdf_ptr_tr_47,
        num_cdfs_47,
        RESTORE_SWITCHABLE_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        RESTORE_SWITCHABLE_TYPES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_48: *mut aom_cdf_prob = ((*ctx_left).wiener_restore_cdf)
        .as_mut_ptr();
    let mut cdf_ptr_tr_48: *mut aom_cdf_prob = ((*ctx_tr).wiener_restore_cdf)
        .as_mut_ptr();
    let mut array_size_48: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_48: core::ffi::c_int = array_size_48
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_48,
        cdf_ptr_tr_48,
        num_cdfs_48,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_49: *mut aom_cdf_prob = ((*ctx_left).sgrproj_restore_cdf)
        .as_mut_ptr();
    let mut cdf_ptr_tr_49: *mut aom_cdf_prob = ((*ctx_tr).sgrproj_restore_cdf)
        .as_mut_ptr();
    let mut array_size_49: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_49: core::ffi::c_int = array_size_49
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_49,
        cdf_ptr_tr_49,
        num_cdfs_49,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_50: *mut aom_cdf_prob = ((*ctx_left).y_mode_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_50: *mut aom_cdf_prob = ((*ctx_tr).y_mode_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_50: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 14]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_50: core::ffi::c_int = array_size_50
        / (INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_50,
        cdf_ptr_tr_50,
        num_cdfs_50,
        INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        INTRA_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_51: *mut aom_cdf_prob = ((*ctx_left)
        .uv_mode_cdf[0 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_51: *mut aom_cdf_prob = ((*ctx_tr)
        .uv_mode_cdf[0 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_51: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 15]; 13],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_51: core::ffi::c_int = array_size_51
        / (UV_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_51,
        cdf_ptr_tr_51,
        num_cdfs_51,
        UV_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        UV_INTRA_MODES as core::ffi::c_int - 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_52: *mut aom_cdf_prob = ((*ctx_left)
        .uv_mode_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_52: *mut aom_cdf_prob = ((*ctx_tr)
        .uv_mode_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_52: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 15]; 13],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_52: core::ffi::c_int = array_size_52
        / (UV_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_52,
        cdf_ptr_tr_52,
        num_cdfs_52,
        UV_INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        UV_INTRA_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < PARTITION_CONTEXTS {
        if i < 4 as core::ffi::c_int {
            let mut cdf_ptr_left_53: *mut aom_cdf_prob = ((*ctx_left)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut cdf_ptr_tr_53: *mut aom_cdf_prob = ((*ctx_tr)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut array_size_53: core::ffi::c_int = (::core::mem::size_of::<
                [aom_cdf_prob; 11],
            >() as core::ffi::c_int as usize)
                .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
                as core::ffi::c_int;
            let mut num_cdfs_53: core::ffi::c_int = array_size_53
                / (10 as core::ffi::c_int + 1 as core::ffi::c_int);
            avg_cdf_symbol(
                cdf_ptr_left_53,
                cdf_ptr_tr_53,
                num_cdfs_53,
                10 as core::ffi::c_int + 1 as core::ffi::c_int,
                4 as core::ffi::c_int,
                wt_left,
                wt_tr,
            );
        } else if i < 16 as core::ffi::c_int {
            let mut cdf_ptr_left_54: *mut aom_cdf_prob = ((*ctx_left)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut cdf_ptr_tr_54: *mut aom_cdf_prob = ((*ctx_tr)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut array_size_54: core::ffi::c_int = (::core::mem::size_of::<
                [aom_cdf_prob; 11],
            >() as core::ffi::c_int as usize)
                .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
                as core::ffi::c_int;
            let mut num_cdfs_54: core::ffi::c_int = array_size_54
                / (10 as core::ffi::c_int + 1 as core::ffi::c_int);
            avg_cdf_symbol(
                cdf_ptr_left_54,
                cdf_ptr_tr_54,
                num_cdfs_54,
                10 as core::ffi::c_int + 1 as core::ffi::c_int,
                10 as core::ffi::c_int,
                wt_left,
                wt_tr,
            );
        } else {
            let mut cdf_ptr_left_55: *mut aom_cdf_prob = ((*ctx_left)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut cdf_ptr_tr_55: *mut aom_cdf_prob = ((*ctx_tr)
                .partition_cdf[i as usize])
                .as_mut_ptr();
            let mut array_size_55: core::ffi::c_int = (::core::mem::size_of::<
                [aom_cdf_prob; 11],
            >() as core::ffi::c_int as usize)
                .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
                as core::ffi::c_int;
            let mut num_cdfs_55: core::ffi::c_int = array_size_55
                / (10 as core::ffi::c_int + 1 as core::ffi::c_int);
            avg_cdf_symbol(
                cdf_ptr_left_55,
                cdf_ptr_tr_55,
                num_cdfs_55,
                10 as core::ffi::c_int + 1 as core::ffi::c_int,
                8 as core::ffi::c_int,
                wt_left,
                wt_tr,
            );
        }
        i += 1;
    }
    let mut cdf_ptr_left_56: *mut aom_cdf_prob = ((*ctx_left).switchable_interp_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_56: *mut aom_cdf_prob = ((*ctx_tr).switchable_interp_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_56: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 16],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_56: core::ffi::c_int = array_size_56
        / (SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_56,
        cdf_ptr_tr_56,
        num_cdfs_56,
        SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int,
        SWITCHABLE_FILTERS as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_57: *mut aom_cdf_prob = ((*ctx_left).kf_y_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_57: *mut aom_cdf_prob = ((*ctx_tr).kf_y_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_57: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 14]; 5]; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_57: core::ffi::c_int = array_size_57
        / (INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_57,
        cdf_ptr_tr_57,
        num_cdfs_57,
        INTRA_MODES as core::ffi::c_int + 1 as core::ffi::c_int,
        INTRA_MODES as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_58: *mut aom_cdf_prob = ((*ctx_left).angle_delta_cdf)
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_58: *mut aom_cdf_prob = ((*ctx_tr).angle_delta_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_58: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 8]; 8],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_58: core::ffi::c_int = array_size_58
        / (2 as core::ffi::c_int * 3 as core::ffi::c_int + 1 as core::ffi::c_int
            + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_58,
        cdf_ptr_tr_58,
        num_cdfs_58,
        2 as core::ffi::c_int * 3 as core::ffi::c_int + 1 as core::ffi::c_int
            + 1 as core::ffi::c_int,
        2 as core::ffi::c_int * 3 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_59: *mut aom_cdf_prob = ((*ctx_left)
        .tx_size_cdf[0 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_59: *mut aom_cdf_prob = ((*ctx_tr)
        .tx_size_cdf[0 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_59: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_59: core::ffi::c_int = array_size_59
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_59,
        cdf_ptr_tr_59,
        num_cdfs_59,
        2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_60: *mut aom_cdf_prob = ((*ctx_left)
        .tx_size_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_60: *mut aom_cdf_prob = ((*ctx_tr)
        .tx_size_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_60: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_60: core::ffi::c_int = array_size_60
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_60,
        cdf_ptr_tr_60,
        num_cdfs_60,
        2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_61: *mut aom_cdf_prob = ((*ctx_left)
        .tx_size_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_61: *mut aom_cdf_prob = ((*ctx_tr)
        .tx_size_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_61: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_61: core::ffi::c_int = array_size_61
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_61,
        cdf_ptr_tr_61,
        num_cdfs_61,
        2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_62: *mut aom_cdf_prob = ((*ctx_left)
        .tx_size_cdf[3 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_62: *mut aom_cdf_prob = ((*ctx_tr)
        .tx_size_cdf[3 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_62: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 4]; 3],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_62: core::ffi::c_int = array_size_62
        / (2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_62,
        cdf_ptr_tr_62,
        num_cdfs_62,
        2 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_63: *mut aom_cdf_prob = ((*ctx_left).delta_q_cdf).as_mut_ptr();
    let mut cdf_ptr_tr_63: *mut aom_cdf_prob = ((*ctx_tr).delta_q_cdf).as_mut_ptr();
    let mut array_size_63: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_63: core::ffi::c_int = array_size_63
        / (3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_63,
        cdf_ptr_tr_63,
        num_cdfs_63,
        3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        3 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_64: *mut aom_cdf_prob = ((*ctx_left).delta_lf_cdf).as_mut_ptr();
    let mut cdf_ptr_tr_64: *mut aom_cdf_prob = ((*ctx_tr).delta_lf_cdf).as_mut_ptr();
    let mut array_size_64: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 5],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_64: core::ffi::c_int = array_size_64
        / (3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_64,
        cdf_ptr_tr_64,
        num_cdfs_64,
        3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        3 as core::ffi::c_int + 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < FRAME_LF_COUNT {
        let mut cdf_ptr_left_65: *mut aom_cdf_prob = ((*ctx_left)
            .delta_lf_multi_cdf[i_0 as usize])
            .as_mut_ptr();
        let mut cdf_ptr_tr_65: *mut aom_cdf_prob = ((*ctx_tr)
            .delta_lf_multi_cdf[i_0 as usize])
            .as_mut_ptr();
        let mut array_size_65: core::ffi::c_int = (::core::mem::size_of::<
            [aom_cdf_prob; 5],
        >() as core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
            as core::ffi::c_int;
        let mut num_cdfs_65: core::ffi::c_int = array_size_65
            / (3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int);
        avg_cdf_symbol(
            cdf_ptr_left_65,
            cdf_ptr_tr_65,
            num_cdfs_65,
            3 as core::ffi::c_int + 1 as core::ffi::c_int + 1 as core::ffi::c_int,
            3 as core::ffi::c_int + 1 as core::ffi::c_int,
            wt_left,
            wt_tr,
        );
        i_0 += 1;
    }
    let mut cdf_ptr_left_66: *mut aom_cdf_prob = ((*ctx_left)
        .intra_ext_tx_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_66: *mut aom_cdf_prob = ((*ctx_tr)
        .intra_ext_tx_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_66: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 17]; 13]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_66: core::ffi::c_int = array_size_66
        / (TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_66,
        cdf_ptr_tr_66,
        num_cdfs_66,
        TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        7 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_67: *mut aom_cdf_prob = ((*ctx_left)
        .intra_ext_tx_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_67: *mut aom_cdf_prob = ((*ctx_tr)
        .intra_ext_tx_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_67: core::ffi::c_int = (::core::mem::size_of::<
        [[[aom_cdf_prob; 17]; 13]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_67: core::ffi::c_int = array_size_67
        / (TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_67,
        cdf_ptr_tr_67,
        num_cdfs_67,
        TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        5 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_68: *mut aom_cdf_prob = ((*ctx_left)
        .inter_ext_tx_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_68: *mut aom_cdf_prob = ((*ctx_tr)
        .inter_ext_tx_cdf[1 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_68: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 17]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_68: core::ffi::c_int = array_size_68
        / (TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_68,
        cdf_ptr_tr_68,
        num_cdfs_68,
        TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        16 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_69: *mut aom_cdf_prob = ((*ctx_left)
        .inter_ext_tx_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_69: *mut aom_cdf_prob = ((*ctx_tr)
        .inter_ext_tx_cdf[2 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_69: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 17]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_69: core::ffi::c_int = array_size_69
        / (TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_69,
        cdf_ptr_tr_69,
        num_cdfs_69,
        TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        12 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_70: *mut aom_cdf_prob = ((*ctx_left)
        .inter_ext_tx_cdf[3 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_70: *mut aom_cdf_prob = ((*ctx_tr)
        .inter_ext_tx_cdf[3 as core::ffi::c_int as usize])
        .as_mut_ptr() as *mut aom_cdf_prob;
    let mut array_size_70: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 17]; 4],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_70: core::ffi::c_int = array_size_70
        / (TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_70,
        cdf_ptr_tr_70,
        num_cdfs_70,
        TX_TYPES as core::ffi::c_int + 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_71: *mut aom_cdf_prob = ((*ctx_left).cfl_sign_cdf).as_mut_ptr();
    let mut cdf_ptr_tr_71: *mut aom_cdf_prob = ((*ctx_tr).cfl_sign_cdf).as_mut_ptr();
    let mut array_size_71: core::ffi::c_int = (::core::mem::size_of::<
        [aom_cdf_prob; 9],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_71: core::ffi::c_int = array_size_71
        / (CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
            - 1 as core::ffi::c_int + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_71,
        cdf_ptr_tr_71,
        num_cdfs_71,
        CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
            - 1 as core::ffi::c_int + 1 as core::ffi::c_int,
        CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
            - 1 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
    let mut cdf_ptr_left_72: *mut aom_cdf_prob = ((*ctx_left).cfl_alpha_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut cdf_ptr_tr_72: *mut aom_cdf_prob = ((*ctx_tr).cfl_alpha_cdf).as_mut_ptr()
        as *mut aom_cdf_prob;
    let mut array_size_72: core::ffi::c_int = (::core::mem::size_of::<
        [[aom_cdf_prob; 17]; 6],
    >() as core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<aom_cdf_prob>() as usize)
        as core::ffi::c_int;
    let mut num_cdfs_72: core::ffi::c_int = array_size_72
        / (((1 as core::ffi::c_int) << 4 as core::ffi::c_int) + 1 as core::ffi::c_int);
    avg_cdf_symbol(
        cdf_ptr_left_72,
        cdf_ptr_tr_72,
        num_cdfs_72,
        ((1 as core::ffi::c_int) << 4 as core::ffi::c_int) + 1 as core::ffi::c_int,
        (1 as core::ffi::c_int) << 4 as core::ffi::c_int,
        wt_left,
        wt_tr,
    );
}
unsafe extern "C-unwind" fn check_neighbor_blocks(
    mut mi: *mut *mut MB_MODE_INFO,
    mut mi_stride: core::ffi::c_int,
    tile_info: *const TileInfo,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut is_above_low_motion: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut is_left_low_motion: core::ffi::c_int = 1 as core::ffi::c_int;
    let thr: core::ffi::c_int = 24 as core::ffi::c_int;
    if mi_row > (*tile_info).mi_row_start {
        let mut above_mbmi: *const MB_MODE_INFO = *mi.offset(-mi_stride as isize);
        let above_mv: int_mv = (*above_mbmi).mv[0 as core::ffi::c_int as usize];
        if (*above_mbmi).mode as core::ffi::c_int >= INTRA_MODE_END as core::ffi::c_int
            && (abs(above_mv.as_mv.row as core::ffi::c_int) > thr
                || abs(above_mv.as_mv.col as core::ffi::c_int) > thr)
        {
            is_above_low_motion = 0 as core::ffi::c_int;
        }
    }
    if mi_col > (*tile_info).mi_col_start {
        let mut left_mbmi: *const MB_MODE_INFO = *mi
            .offset(-(1 as core::ffi::c_int) as isize);
        let left_mv: int_mv = (*left_mbmi).mv[0 as core::ffi::c_int as usize];
        if (*left_mbmi).mode as core::ffi::c_int >= INTRA_MODE_END as core::ffi::c_int
            && (abs(left_mv.as_mv.row as core::ffi::c_int) > thr
                || abs(left_mv.as_mv.col as core::ffi::c_int) > thr)
        {
            is_left_low_motion = 0 as core::ffi::c_int;
        }
    }
    return (is_above_low_motion != 0 && is_left_low_motion != 0) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn fast_detect_non_zero_motion(
    mut cpi: *mut AV1_COMP,
    mut src_y: *const uint8_t,
    mut src_ystride: core::ffi::c_int,
    mut last_src_y: *const uint8_t,
    mut last_src_ystride: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let bsize: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let mut blk_sad: core::ffi::c_uint = INT_MAX as core::ffi::c_uint;
    if !((*cpi).src_sad_blk_64x64).is_null() {
        let sb_size_by_mb: core::ffi::c_int = if bsize as core::ffi::c_int
            == BLOCK_128X128 as core::ffi::c_int
        {
            (*(*cm).seq_params).mib_size >> 1 as core::ffi::c_int
        } else {
            (*(*cm).seq_params).mib_size
        };
        let sb_cols: core::ffi::c_int = ((*cm).mi_params.mi_cols + sb_size_by_mb
            - 1 as core::ffi::c_int) / sb_size_by_mb;
        let sbi_col: core::ffi::c_int = mi_col / sb_size_by_mb;
        let sbi_row: core::ffi::c_int = mi_row / sb_size_by_mb;
        blk_sad = *((*cpi).src_sad_blk_64x64)
            .offset((sbi_col + sbi_row * sb_cols) as isize) as core::ffi::c_uint;
    } else {
        blk_sad = ((*(*cpi).ppi).fn_ptr[bsize as usize].sdf)
            .expect(
                "non-null function pointer",
            )(src_y, src_ystride, last_src_y, last_src_ystride);
    }
    let search_pos: [*const uint8_t; 4] = [
        last_src_y.offset(-(last_src_ystride as isize)),
        last_src_y.offset(-(1 as core::ffi::c_int as isize)),
        last_src_y.offset(1 as core::ffi::c_int as isize),
        last_src_y.offset(last_src_ystride as isize),
    ];
    let mut sad_arr: [core::ffi::c_uint; 4] = [0; 4];
    ((*(*cpi).ppi).fn_ptr[bsize as usize].sdx4df)
        .expect(
            "non-null function pointer",
        )(
        src_y,
        src_ystride,
        search_pos.as_ptr(),
        last_src_ystride,
        sad_arr.as_mut_ptr(),
    );
    blk_sad = blk_sad.wrapping_mul(5 as core::ffi::c_uint) >> 3 as core::ffi::c_int;
    return (blk_sad < sad_arr[0 as core::ffi::c_int as usize]
        && blk_sad < sad_arr[1 as core::ffi::c_int as usize]
        && blk_sad < sad_arr[2 as core::ffi::c_int as usize]
        && blk_sad < sad_arr[3 as core::ffi::c_int as usize]) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_source_content_sb(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut tile_data: *mut TileDataEnc,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    if (*(*cpi).last_source).c2rust_unnamed.c2rust_unnamed.y_width
        != (*(*cpi).source).c2rust_unnamed.c2rust_unnamed.y_width
        || (*(*cpi).last_source).c2rust_unnamed_0.c2rust_unnamed.y_height
            != (*(*cpi).source).c2rust_unnamed_0.c2rust_unnamed.y_height
    {
        return;
    }
    if (*(*x).e_mbd.cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        return;
    }
    let mut tmp_sse: core::ffi::c_uint = 0;
    let mut tmp_variance: core::ffi::c_uint = 0;
    let bsize: BLOCK_SIZE = (*(*cpi).common.seq_params).sb_size;
    let mut src_y: *mut uint8_t = (*(*cpi).source)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .y_buffer;
    let src_ystride: core::ffi::c_int = (*(*cpi).source)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    let src_offset: core::ffi::c_int = src_ystride * (mi_row << 2 as core::ffi::c_int)
        + (mi_col << 2 as core::ffi::c_int);
    let mut last_src_y: *mut uint8_t = (*(*cpi).last_source)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .y_buffer;
    let last_src_ystride: core::ffi::c_int = (*(*cpi).last_source)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    let last_src_offset: core::ffi::c_int = last_src_ystride
        * (mi_row << 2 as core::ffi::c_int) + (mi_col << 2 as core::ffi::c_int);
    let mut avg_source_sse_threshold_verylow: uint64_t = 10000 as uint64_t;
    let mut avg_source_sse_threshold_low: [uint64_t; 2] = [
        100000 as core::ffi::c_int as uint64_t,
        36000 as core::ffi::c_int as uint64_t,
    ];
    let mut avg_source_sse_threshold_high: uint64_t = 1000000 as uint64_t;
    if (*cpi).sf.rt_sf.increase_source_sad_thresh != 0 {
        avg_source_sse_threshold_high = avg_source_sse_threshold_high
            << 1 as core::ffi::c_int;
        avg_source_sse_threshold_low[0 as core::ffi::c_int as usize] = avg_source_sse_threshold_low[0
            as core::ffi::c_int as usize] << 1 as core::ffi::c_int;
        avg_source_sse_threshold_verylow = avg_source_sse_threshold_verylow
            << 1 as core::ffi::c_int;
    }
    let mut sum_sq_thresh: uint64_t = 10000 as uint64_t;
    src_y = src_y.offset(src_offset as isize);
    last_src_y = last_src_y.offset(last_src_offset as isize);
    tmp_variance = ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
        .expect(
            "non-null function pointer",
        )(src_y, src_ystride, last_src_y, last_src_ystride, &raw mut tmp_sse);
    if (tmp_sse as uint64_t)
        < avg_source_sse_threshold_low[1 as core::ffi::c_int as usize]
    {
        (*x).content_state_sb.source_sad_rd = kLowSad;
    }
    if tmp_sse == 0 as core::ffi::c_uint {
        (*x).content_state_sb.source_sad_nonrd = kZeroSad;
        return;
    }
    if (tmp_sse as uint64_t) < avg_source_sse_threshold_verylow {
        (*x).content_state_sb.source_sad_nonrd = kVeryLowSad;
    } else if (tmp_sse as uint64_t)
        < avg_source_sse_threshold_low[0 as core::ffi::c_int as usize]
    {
        (*x).content_state_sb.source_sad_nonrd = kLowSad;
    } else if tmp_sse as uint64_t > avg_source_sse_threshold_high {
        (*x).content_state_sb.source_sad_nonrd = kHighSad;
    }
    if tmp_variance < tmp_sse >> 1 as core::ffi::c_int
        && tmp_sse.wrapping_sub(tmp_variance) as uint64_t > sum_sq_thresh
    {
        (*x).content_state_sb.lighting_change = 1 as core::ffi::c_int;
    }
    if (tmp_sse.wrapping_sub(tmp_variance) as uint64_t)
        < sum_sq_thresh >> 1 as core::ffi::c_int
    {
        (*x).content_state_sb.low_sumdiff = 1 as core::ffi::c_int;
    }
    if tmp_sse as uint64_t
        > avg_source_sse_threshold_high.wrapping_mul(7 as uint64_t)
            >> 3 as core::ffi::c_int && (*x).content_state_sb.lighting_change == 0
        && (*x).content_state_sb.low_sumdiff == 0
    {
        (*x).sb_force_fixed_part = 0 as core::ffi::c_int;
    }
    if (*cpi).sf.rt_sf.use_rtc_tf == 0 || (*cpi).rc.high_source_sad != 0
        || (*cpi).rc.frame_source_sad > 20000 as uint64_t
        || (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int
    {
        return;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let nmean2: core::ffi::c_uint = tmp_sse.wrapping_sub(tmp_variance);
    let ac_q_step: core::ffi::c_int = av1_ac_quant_QTX(
        (*cm).quant_params.base_qindex,
        0 as core::ffi::c_int,
        (*(*cm).seq_params).bit_depth,
    ) as core::ffi::c_int;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let avg_q_step: core::ffi::c_int = av1_ac_quant_QTX(
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize],
        0 as core::ffi::c_int,
        (*(*cm).seq_params).bit_depth,
    ) as core::ffi::c_int;
    let threshold: core::ffi::c_uint = (if (*cpi).sf.rt_sf.use_rtc_tf
        == 1 as core::ffi::c_int
    {
        clamp(avg_q_step, 250 as core::ffi::c_int, 1000 as core::ffi::c_int) * ac_q_step
    } else {
        250 as core::ffi::c_int * ac_q_step
    }) as core::ffi::c_uint;
    if tmp_variance <= threshold && nmean2 <= 15 as core::ffi::c_uint {
        let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
            .offset(get_mi_grid_idx(&raw mut (*cm).mi_params, mi_row, mi_col) as isize);
        let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
        let is_neighbor_blocks_low_motion: core::ffi::c_int = check_neighbor_blocks(
            mi,
            (*cm).mi_params.mi_stride,
            tile_info,
            mi_row,
            mi_col,
        ) as core::ffi::c_int;
        if is_neighbor_blocks_low_motion == 0 {
            return;
        }
        let is_this_blk_low_motion: core::ffi::c_int = fast_detect_non_zero_motion(
            cpi,
            src_y,
            src_ystride,
            last_src_y,
            last_src_ystride,
            mi_row,
            mi_col,
        ) as core::ffi::c_int;
        if is_this_blk_low_motion == 0 {
            return;
        }
        let shift_x: [core::ffi::c_int; 2] = [
            0 as core::ffi::c_int,
            (*(*cpi).source).subsampling_x,
        ];
        let shift_y: [core::ffi::c_int; 2] = [
            0 as core::ffi::c_int,
            (*(*cpi).source).subsampling_y,
        ];
        let h: uint8_t = block_size_high[bsize as usize];
        let w: uint8_t = block_size_wide[bsize as usize];
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < av1_num_planes(cm) {
            let mut src: *mut uint8_t = (*(*cpi).source)
                .c2rust_unnamed_4
                .buffers[plane as usize];
            let src_stride: core::ffi::c_int = (*(*cpi).source)
                .c2rust_unnamed_3
                .strides[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize];
            let mut last_src: *mut uint8_t = (*(*cpi).last_source)
                .c2rust_unnamed_4
                .buffers[plane as usize];
            let last_src_stride: core::ffi::c_int = (*(*cpi).last_source)
                .c2rust_unnamed_3
                .strides[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize];
            src = src
                .offset(
                    (src_stride
                        * (mi_row
                            << 2 as core::ffi::c_int
                                - shift_y[(plane != 0 as core::ffi::c_int)
                                    as core::ffi::c_int as usize])
                        + (mi_col
                            << 2 as core::ffi::c_int
                                - shift_x[(plane != 0 as core::ffi::c_int)
                                    as core::ffi::c_int as usize])) as isize,
                );
            last_src = last_src
                .offset(
                    (last_src_stride
                        * (mi_row
                            << 2 as core::ffi::c_int
                                - shift_y[(plane != 0 as core::ffi::c_int)
                                    as core::ffi::c_int as usize])
                        + (mi_col
                            << 2 as core::ffi::c_int
                                - shift_x[(plane != 0 as core::ffi::c_int)
                                    as core::ffi::c_int as usize])) as isize,
                );
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i
                < h as core::ffi::c_int
                    >> shift_y[(plane != 0 as core::ffi::c_int) as core::ffi::c_int
                        as usize]
            {
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j
                    < w as core::ffi::c_int
                        >> shift_x[(plane != 0 as core::ffi::c_int) as core::ffi::c_int
                            as usize]
                {
                    *src.offset(j as isize) = (*last_src.offset(j as isize)
                        as core::ffi::c_int + *src.offset(j as isize) as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as uint8_t;
                    j += 1;
                }
                src = src.offset(src_stride as isize);
                last_src = last_src.offset(last_src_stride as isize);
                i += 1;
            }
            plane += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_mbmi(
    mi_params: *mut CommonModeInfoParams,
    mut sb_size: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let sb_size_mi: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int;
    let mi_alloc_size_1d: core::ffi::c_int = mi_size_wide[(*mi_params).mi_alloc_bsize
        as usize] as core::ffi::c_int;
    let sb_size_alloc_mi: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int / mi_alloc_size_1d;
    if !((*mi_params).mi_alloc_stride % sb_size_alloc_mi == 0 as core::ffi::c_int
        && !(b"mi is not allocated as a multiple of sb!\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"av1_reset_mbmi\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            1451 as core::ffi::c_int,
            b"mi_params->mi_alloc_stride % sb_size_alloc_mi == 0 && \"mi is not allocated as a multiple of sb!\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*mi_params).mi_stride % sb_size_mi == 0 as core::ffi::c_int
        && !(b"mi_grid_base is not allocated as a multiple of sb!\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"av1_reset_mbmi\0"))
                .as_ptr(),
            b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
            1453 as core::ffi::c_int,
            b"mi_params->mi_stride % sb_size_mi == 0 && \"mi_grid_base is not allocated as a multiple of sb!\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mi_rows: core::ffi::c_int = mi_size_high[sb_size as usize] as core::ffi::c_int;
    let mut cur_mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while cur_mi_row < mi_rows {
        if !(get_mi_grid_idx(mi_params, 0 as core::ffi::c_int, mi_col + mi_alloc_size_1d)
            < (*mi_params).mi_stride) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"av1_reset_mbmi\0"))
                    .as_ptr(),
                b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                1458 as core::ffi::c_int,
                b"get_mi_grid_idx(mi_params, 0, mi_col + mi_alloc_size_1d) < mi_params->mi_stride\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mi_grid_idx: core::ffi::c_int = get_mi_grid_idx(
            mi_params,
            mi_row + cur_mi_row,
            mi_col,
        ) as core::ffi::c_int;
        let alloc_mi_idx: core::ffi::c_int = get_alloc_mi_idx(
            mi_params,
            mi_row + cur_mi_row,
            mi_col,
        ) as core::ffi::c_int;
        memset(
            &raw mut *((*mi_params).mi_grid_base).offset(mi_grid_idx as isize)
                as *mut *mut MB_MODE_INFO as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (sb_size_mi as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut MB_MODE_INFO>() as size_t),
        );
        memset(
            &raw mut *((*mi_params).tx_type_map).offset(mi_grid_idx as isize) as *mut TX_TYPE
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (sb_size_mi as size_t)
                .wrapping_mul(::core::mem::size_of::<TX_TYPE>() as size_t),
        );
        if cur_mi_row % mi_alloc_size_1d == 0 as core::ffi::c_int {
            memset(
                &raw mut *((*mi_params).mi_alloc).offset(alloc_mi_idx as isize)
                    as *mut MB_MODE_INFO as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (sb_size_alloc_mi as size_t)
                    .wrapping_mul(::core::mem::size_of::<MB_MODE_INFO>() as size_t),
            );
        }
        cur_mi_row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_backup_sb_state(
    mut sb_fp_stats: *mut SB_FIRST_PASS_STATS,
    mut cpi: *const AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *const TileDataEnc,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mut x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut tile_info: *const TileInfo = &(*tile_data).tile_info;
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    av1_save_context(x, &raw mut (*sb_fp_stats).x_ctx, mi_row, mi_col, sb_size, num_planes);
    (*sb_fp_stats).rd_count = (*td).rd_counts;
    (*sb_fp_stats).split_count = (*x).txfm_search_info.txb_split_count
        as core::ffi::c_int;
    (*sb_fp_stats).fc = *(*td).counts;
    if (*cpi).sf.inter_sf.inter_mode_rd_model_estimation == 1 as core::ffi::c_int {
        memcpy(
            ((*sb_fp_stats).inter_mode_rd_models).as_mut_ptr() as *mut core::ffi::c_void,
            ((*tile_data).inter_mode_rd_models).as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[InterModeRdModel; 22]>() as size_t,
        );
    }
    memcpy(
        ((*sb_fp_stats).thresh_freq_fact).as_mut_ptr() as *mut core::ffi::c_void,
        ((*x).thresh_freq_fact).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[core::ffi::c_int; 169]; 22]>() as size_t,
    );
    let alloc_mi_idx: core::ffi::c_int = get_alloc_mi_idx(
        &(*cm).mi_params,
        mi_row,
        mi_col,
    ) as core::ffi::c_int;
    (*sb_fp_stats).current_qindex = (*((*cm).mi_params.mi_alloc)
        .offset(alloc_mi_idx as isize))
        .current_qindex;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_restore_sb_state(
    mut sb_fp_stats: *const SB_FIRST_PASS_STATS,
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mut x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    av1_restore_context(x, &(*sb_fp_stats).x_ctx, mi_row, mi_col, sb_size, num_planes);
    (*td).rd_counts = (*sb_fp_stats).rd_count;
    (*x).txfm_search_info.txb_split_count = (*sb_fp_stats).split_count
        as core::ffi::c_uint;
    *(*td).counts = (*sb_fp_stats).fc;
    if (*cpi).sf.inter_sf.inter_mode_rd_model_estimation == 1 as core::ffi::c_int {
        memcpy(
            ((*tile_data).inter_mode_rd_models).as_mut_ptr() as *mut core::ffi::c_void,
            ((*sb_fp_stats).inter_mode_rd_models).as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[InterModeRdModel; 22]>() as size_t,
        );
    }
    memcpy(
        ((*x).thresh_freq_fact).as_mut_ptr() as *mut core::ffi::c_void,
        ((*sb_fp_stats).thresh_freq_fact).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[core::ffi::c_int; 169]; 22]>() as size_t,
    );
    let alloc_mi_idx: core::ffi::c_int = get_alloc_mi_idx(
        &(*cm).mi_params,
        mi_row,
        mi_col,
    ) as core::ffi::c_int;
    (*((*cm).mi_params.mi_alloc).offset(alloc_mi_idx as isize)).current_qindex = (*sb_fp_stats)
        .current_qindex;
}
unsafe extern "C-unwind" fn skip_cost_update(
    mut seq_params: *const SequenceHeader,
    tile_info: *const TileInfo,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    mut upd_level: INTERNAL_COST_UPDATE_TYPE,
) -> core::ffi::c_int {
    if upd_level as core::ffi::c_uint
        == INTERNAL_COST_UPD_SB as core::ffi::c_int as core::ffi::c_uint
    {
        return 0 as core::ffi::c_int;
    }
    if upd_level as core::ffi::c_uint
        == INTERNAL_COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
    {
        return 1 as core::ffi::c_int;
    }
    if mi_col != (*tile_info).mi_col_start {
        return 1 as core::ffi::c_int;
    }
    if upd_level as core::ffi::c_uint
        == INTERNAL_COST_UPD_SBROW_SET as core::ffi::c_int as core::ffi::c_uint
    {
        let mib_size_log2: core::ffi::c_int = (*seq_params).mib_size_log2;
        let sb_row: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
            >> mib_size_log2;
        let sb_size: core::ffi::c_int = (*seq_params).mib_size * MI_SIZE;
        let tile_height: core::ffi::c_int = ((*tile_info).mi_row_end
            - (*tile_info).mi_row_start) * MI_SIZE;
        let sb_size_update_freq_map: [core::ffi::c_int; 2] = [
            2 as core::ffi::c_int,
            4 as core::ffi::c_int,
        ];
        let update_freq_sb_rows: core::ffi::c_int = sb_size_update_freq_map[(sb_size
            != MAX_SB_SIZE) as core::ffi::c_int as usize];
        let update_freq_num_rows: core::ffi::c_int = sb_size * update_freq_sb_rows;
        let num_updates_per_tile: core::ffi::c_int = (tile_height + update_freq_num_rows
            - 1 as core::ffi::c_int) / update_freq_num_rows;
        let num_rows_update_per_tile: core::ffi::c_int = num_updates_per_tile * sb_size;
        let num_sb_rows_per_update: core::ffi::c_int = (tile_height
            + num_rows_update_per_tile - 1 as core::ffi::c_int)
            / num_rows_update_per_tile;
        if sb_row % num_sb_rows_per_update != 0 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn skip_mv_cost_update(
    mut cpi: *mut AV1_COMP,
    tile_info: *const TileInfo,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    if frame_is_intra_only(cm) != 0 {
        return 1 as core::ffi::c_int;
    }
    return skip_cost_update(
        (*cm).seq_params,
        tile_info,
        mi_row,
        mi_col,
        (*cpi).sf.inter_sf.mv_cost_upd_level,
    );
}
unsafe extern "C-unwind" fn skip_dv_cost_update(
    mut cpi: *mut AV1_COMP,
    tile_info: *const TileInfo,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    if av1_allow_intrabc(cm) == 0 || is_stat_generation_stage(cpi) != 0 {
        return 1 as core::ffi::c_int;
    }
    return skip_cost_update(
        (*cm).seq_params,
        tile_info,
        mi_row,
        mi_col,
        (*cpi).sf.intra_sf.dv_cost_upd_level,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_cost_upd_freq(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    tile_info: *const TileInfo,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if (*cm).features.disable_cdf_update {
        return;
    }
    let mut current_block_4: u64;
    match (*cpi).sf.inter_sf.coeff_cost_upd_level as core::ffi::c_uint {
        0 | 1 => {
            current_block_4 = 0;
        }
        2 => {
            current_block_4 = 1;
        }
        3 => {
            current_block_4 = 1;
        }
        4 => {
            current_block_4 = 2;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_set_cost_upd_freq\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    1645 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            current_block_4 = 0;
        }
    }
    match current_block_4 {
        1 => {
            current_block_4 = 2;
        }
        _ => {}
    }
    match current_block_4 {
        2 => {
            if !(skip_cost_update(
                (*cm).seq_params,
                tile_info,
                mi_row,
                mi_col,
                (*cpi).sf.inter_sf.coeff_cost_upd_level,
            ) != 0)
            {
                av1_fill_coeff_costs(&raw mut (*x).coeff_costs, (*xd).tile_ctx, num_planes);
            }
        }
        _ => {}
    }
    let mut current_block_7: u64;
    match (*cpi).sf.inter_sf.mode_cost_upd_level as core::ffi::c_uint {
        0 | 1 => {
            current_block_7 = 3;
        }
        2 => {
            current_block_7 = 4;
        }
        3 => {
            current_block_7 = 4;
        }
        4 => {
            current_block_7 = 5;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_set_cost_upd_freq\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    1660 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            current_block_7 = 3;
        }
    }
    match current_block_7 {
        4 => {
            current_block_7 = 5;
        }
        _ => {}
    }
    match current_block_7 {
        5 => {
            if !(skip_cost_update(
                (*cm).seq_params,
                tile_info,
                mi_row,
                mi_col,
                (*cpi).sf.inter_sf.mode_cost_upd_level,
            ) != 0)
            {
                av1_fill_mode_rates(cm, &raw mut (*x).mode_costs, (*xd).tile_ctx);
            }
        }
        _ => {}
    }
    let mut current_block_10: u64;
    match (*cpi).sf.inter_sf.mv_cost_upd_level as core::ffi::c_uint {
        0 | 1 => {
            current_block_10 = 6;
        }
        2 => {
            current_block_10 = 7;
        }
        3 => {
            current_block_10 = 7;
        }
        4 => {
            current_block_10 = 8;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_set_cost_upd_freq\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    1676 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            current_block_10 = 6;
        }
    }
    match current_block_10 {
        7 => {
            current_block_10 = 8;
        }
        _ => {}
    }
    match current_block_10 {
        8 => {
            if !(skip_mv_cost_update(cpi, tile_info, mi_row, mi_col) != 0) {
                av1_fill_mv_costs(
                    &raw mut (*(*xd).tile_ctx).nmvc,
                    (*cm).features.cur_frame_force_integer_mv as core::ffi::c_int,
                    (*cm).features.allow_high_precision_mv as core::ffi::c_int,
                    (*x).mv_costs,
                );
            }
        }
        _ => {}
    }
    let mut current_block_13: u64;
    match (*cpi).sf.intra_sf.dv_cost_upd_level as core::ffi::c_uint {
        0 | 1 => {
            current_block_13 = 9;
        }
        2 => {
            current_block_13 = 10;
        }
        3 => {
            current_block_13 = 10;
        }
        4 => {
            current_block_13 = 11;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_set_cost_upd_freq\0"))
                        .as_ptr(),
                    b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                    1690 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            current_block_13 = 9;
        }
    }
    match current_block_13 {
        10 => {
            current_block_13 = 11;
        }
        _ => {}
    }
    match current_block_13 {
        11 => {
            if !(skip_dv_cost_update(cpi, tile_info, mi_row, mi_col) != 0) {
                av1_fill_dv_costs(&raw mut (*(*xd).tile_ctx).ndvc, (*x).dv_costs);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dealloc_src_diff_buf(
    mut mb: *mut macroblock,
    mut num_planes: core::ffi::c_int,
) {
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        aom_free((*mb).plane[plane as usize].src_diff as *mut core::ffi::c_void);
        (*mb).plane[plane as usize].src_diff = 0 as *mut int16_t;
        plane += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_src_diff_buf(
    mut cm: *const AV1Common,
    mut mb: *mut macroblock,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm as *const AV1_COMMON)
        as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if !((*mb).plane[plane as usize].src_diff).is_null() as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"av1_alloc_src_diff_buf\0"))
                    .as_ptr(),
                b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                1705 as core::ffi::c_int,
                b"!mb->plane[plane].src_diff\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        plane += 1;
    }
    let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane_0 < num_planes {
        let subsampling_xy: core::ffi::c_int = if plane_0 != 0 {
            (*(*cm).seq_params).subsampling_x + (*(*cm).seq_params).subsampling_y
        } else {
            0 as core::ffi::c_int
        };
        let sb_size: core::ffi::c_int = MAX_SB_SQUARE >> subsampling_xy;
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"av1_alloc_src_diff_buf\0"))
                    .as_ptr(),
                b"encodeframe_utils.c\0" as *const u8 as *const core::ffi::c_char,
                1715 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*mb).plane[plane_0 as usize].src_diff = aom_memalign(
            32 as size_t,
            (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(sb_size as size_t),
        ) as *mut int16_t;
        if ((*mb).plane[plane_0 as usize].src_diff).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate mb->plane[plane].src_diff at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/encodeframe_utils.c\0" as *const u8
                    as *const core::ffi::c_char,
                1715 as core::ffi::c_int,
            );
        }
        plane_0 += 1;
    }
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;

extern "C" {
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
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn exp(_: core::ffi::c_double) -> core::ffi::c_double;
    fn log(_: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(_: core::ffi::c_double, _: core::ffi::c_double) -> core::ffi::c_double;
}
