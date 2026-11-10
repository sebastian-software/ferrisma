/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

/*!\defgroup gf_group_algo Golden Frame Group
 * \ingroup high_level_algo
 * Algorithms regarding determining the length of GF groups and defining GF
 * group structures.
 * @{
 */
/*! @} - end defgroup gf_group_algo */

// Mechanically translated from upstream/aom/av1/encoder/pass2_strategy.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_lookahead_peek(
        ctx: *mut lookahead_ctx,
        index: core::ffi::c_int,
        stage: COMPRESSOR_STAGE,
    ) -> *mut lookahead_entry;
    fn av1_lookahead_depth(
        ctx: *mut lookahead_ctx,
        stage: COMPRESSOR_STAGE,
    ) -> core::ffi::c_int;
    fn av1_lookahead_pop_sz(
        ctx: *mut lookahead_ctx,
        stage: COMPRESSOR_STAGE,
    ) -> core::ffi::c_int;
    fn av1_convert_qindex_to_q(
        qindex: core::ffi::c_int,
        bit_depth: aom_bit_depth_t,
    ) -> core::ffi::c_double;
    fn av1_rc_set_frame_target(
        cpi: *mut AV1_COMP,
        target: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn av1_calc_pframe_target_size_one_pass_vbr(
        cpi: *const AV1_COMP,
        frame_update_type: FRAME_UPDATE_TYPE,
    ) -> core::ffi::c_int;
    fn av1_calc_iframe_target_size_one_pass_vbr(
        cpi: *const AV1_COMP,
    ) -> core::ffi::c_int;
    fn av1_calc_pframe_target_size_one_pass_cbr(
        cpi: *const AV1_COMP,
        frame_update_type: FRAME_UPDATE_TYPE,
    ) -> core::ffi::c_int;
    fn av1_calc_iframe_target_size_one_pass_cbr(
        cpi: *const AV1_COMP,
    ) -> core::ffi::c_int;
    fn av1_rc_scene_detection_onepass_rt(
        cpi: *mut AV1_COMP,
        frame_input: *const EncodeFrameInput,
    );
    fn av1_firstpass_info_peek(
        firstpass_info: *const FIRSTPASS_INFO,
        offset_from_cur: core::ffi::c_int,
    ) -> *const FIRSTPASS_STATS;
    fn av1_firstpass_info_future_count(
        firstpass_info: *const FIRSTPASS_INFO,
        offset_from_cur: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_accumulate_stats(
        section: *mut FIRSTPASS_STATS,
        frame: *const FIRSTPASS_STATS,
    );
    fn av1_extrc_send_firstpass_stats(
        ext_ratectrl: *mut AOM_EXT_RATECTRL,
        first_pass_info: *const FIRSTPASS_INFO,
    ) -> aom_codec_err_t;
    fn av1_extrc_get_key_frame_decision(
        ext_ratectrl: *mut AOM_EXT_RATECTRL,
        key_frame_decision: *mut aom_rc_key_frame_decision_t,
    ) -> aom_codec_err_t;
    fn av1_get_max_bitrate_for_level(
        level_index: AV1_LEVEL,
        tier: core::ffi::c_int,
        profile: BITSTREAM_PROFILE,
    ) -> core::ffi::c_double;
    fn av1_tf_info_reset(tf_info: *mut TEMPORAL_FILTER_INFO);
    fn av1_tf_info_filtering(
        tf_info: *mut TEMPORAL_FILTER_INFO,
        cpi: *mut AV1_COMP,
        gf_group: *const GF_GROUP,
    );
    fn av1_tpl_setup_stats(
        cpi: *mut AV1_COMP,
        gop_eval: core::ffi::c_int,
        frame_params: *const EncodeFrameParams,
    ) -> core::ffi::c_int;
    fn av1_tpl_preload_rc_estimate(
        cpi: *mut AV1_COMP,
        frame_params: *const EncodeFrameParams,
    );
    fn av1_new_framerate(cpi: *mut AV1_COMP, framerate: core::ffi::c_double);
    fn av1_gop_setup_structure(cpi: *mut AV1_COMP, is_final_pass: core::ffi::c_int);
    fn is_forced_keyframe_pending(
        lookahead: *mut lookahead_ctx,
        up_to_index: core::ffi::c_int,
        compressor_stage: COMPRESSOR_STAGE,
    ) -> core::ffi::c_int;
}
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
pub type size_t = usize;
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_7,
    pub c2rust_unnamed_1: C2RustUnnamed_5,
    pub c2rust_unnamed_2: C2RustUnnamed_3,
    pub c2rust_unnamed_3: C2RustUnnamed_1,
    pub c2rust_unnamed_4: C2RustUnnamed,
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
pub type aom_metadata_array_t = aom_metadata_array;
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
pub type aom_metadata_insert_flags_t = aom_metadata_insert_flags;
pub type aom_metadata_insert_flags = core::ffi::c_uint;
pub const AOM_MIF_ANY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 18;
pub const AOM_MIF_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 17;
pub const AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 16;
pub const AOM_MIF_ANY_FRAME: aom_metadata_insert_flags = 2;
pub const AOM_MIF_KEY_FRAME: aom_metadata_insert_flags = 1;
pub const AOM_MIF_NON_KEY_FRAME: aom_metadata_insert_flags = 0;
pub type aom_color_range_t = aom_color_range;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
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
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
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
pub type aom_color_primaries_t = aom_color_primaries;
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
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
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
    pub data: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub frame: C2RustUnnamed_12,
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
pub struct C2RustUnnamed_12 {
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
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed_13 = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed_13 = 4;
pub const TX_SIZES: C2RustUnnamed_13 = 5;
pub const TX_SIZES_ALL: C2RustUnnamed_13 = 19;
pub const TX_64X16: C2RustUnnamed_13 = 18;
pub const TX_16X64: C2RustUnnamed_13 = 17;
pub const TX_32X8: C2RustUnnamed_13 = 16;
pub const TX_8X32: C2RustUnnamed_13 = 15;
pub const TX_16X4: C2RustUnnamed_13 = 14;
pub const TX_4X16: C2RustUnnamed_13 = 13;
pub const TX_64X32: C2RustUnnamed_13 = 12;
pub const TX_32X64: C2RustUnnamed_13 = 11;
pub const TX_32X16: C2RustUnnamed_13 = 10;
pub const TX_16X32: C2RustUnnamed_13 = 9;
pub const TX_16X8: C2RustUnnamed_13 = 8;
pub const TX_8X16: C2RustUnnamed_13 = 7;
pub const TX_8X4: C2RustUnnamed_13 = 6;
pub const TX_4X8: C2RustUnnamed_13 = 5;
pub const TX_64X64: C2RustUnnamed_13 = 4;
pub const TX_32X32: C2RustUnnamed_13 = 3;
pub const TX_16X16: C2RustUnnamed_13 = 2;
pub const TX_8X8: C2RustUnnamed_13 = 1;
pub const TX_4X4: C2RustUnnamed_13 = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_14 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_14 = 15;
pub const TX_TYPES: C2RustUnnamed_14 = 16;
pub const H_FLIPADST: C2RustUnnamed_14 = 15;
pub const V_FLIPADST: C2RustUnnamed_14 = 14;
pub const H_ADST: C2RustUnnamed_14 = 13;
pub const V_ADST: C2RustUnnamed_14 = 12;
pub const H_DCT: C2RustUnnamed_14 = 11;
pub const V_DCT: C2RustUnnamed_14 = 10;
pub const IDTX: C2RustUnnamed_14 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_14 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_14 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_14 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_14 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_14 = 4;
pub const ADST_ADST: C2RustUnnamed_14 = 3;
pub const DCT_ADST: C2RustUnnamed_14 = 2;
pub const ADST_DCT: C2RustUnnamed_14 = 1;
pub const DCT_DCT: C2RustUnnamed_14 = 0;
pub type TX_TYPE = uint8_t;
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
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const SEQ_LEVEL_KEEP_STATS: C2RustUnnamed_15 = 32;
pub const SEQ_LEVEL_MAX: C2RustUnnamed_15 = 31;
pub const SEQ_LEVELS: C2RustUnnamed_15 = 28;
pub const SEQ_LEVEL_8_3: C2RustUnnamed_15 = 27;
pub const SEQ_LEVEL_8_2: C2RustUnnamed_15 = 26;
pub const SEQ_LEVEL_8_1: C2RustUnnamed_15 = 25;
pub const SEQ_LEVEL_8_0: C2RustUnnamed_15 = 24;
pub const SEQ_LEVEL_7_3: C2RustUnnamed_15 = 23;
pub const SEQ_LEVEL_7_2: C2RustUnnamed_15 = 22;
pub const SEQ_LEVEL_7_1: C2RustUnnamed_15 = 21;
pub const SEQ_LEVEL_7_0: C2RustUnnamed_15 = 20;
pub const SEQ_LEVEL_6_3: C2RustUnnamed_15 = 19;
pub const SEQ_LEVEL_6_2: C2RustUnnamed_15 = 18;
pub const SEQ_LEVEL_6_1: C2RustUnnamed_15 = 17;
pub const SEQ_LEVEL_6_0: C2RustUnnamed_15 = 16;
pub const SEQ_LEVEL_5_3: C2RustUnnamed_15 = 15;
pub const SEQ_LEVEL_5_2: C2RustUnnamed_15 = 14;
pub const SEQ_LEVEL_5_1: C2RustUnnamed_15 = 13;
pub const SEQ_LEVEL_5_0: C2RustUnnamed_15 = 12;
pub const SEQ_LEVEL_4_3: C2RustUnnamed_15 = 11;
pub const SEQ_LEVEL_4_2: C2RustUnnamed_15 = 10;
pub const SEQ_LEVEL_4_1: C2RustUnnamed_15 = 9;
pub const SEQ_LEVEL_4_0: C2RustUnnamed_15 = 8;
pub const SEQ_LEVEL_3_3: C2RustUnnamed_15 = 7;
pub const SEQ_LEVEL_3_2: C2RustUnnamed_15 = 6;
pub const SEQ_LEVEL_3_1: C2RustUnnamed_15 = 5;
pub const SEQ_LEVEL_3_0: C2RustUnnamed_15 = 4;
pub const SEQ_LEVEL_2_3: C2RustUnnamed_15 = 3;
pub const SEQ_LEVEL_2_2: C2RustUnnamed_15 = 2;
pub const SEQ_LEVEL_2_1: C2RustUnnamed_15 = 1;
pub const SEQ_LEVEL_2_0: C2RustUnnamed_15 = 0;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
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
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_16 = 4;
pub const S_FRAME: C2RustUnnamed_16 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_16 = 2;
pub const INTER_FRAME: C2RustUnnamed_16 = 1;
pub const KEY_FRAME: C2RustUnnamed_16 = 0;
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
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_17 = 2;
pub const REALTIME: C2RustUnnamed_17 = 1;
pub const GOOD: C2RustUnnamed_17 = 0;
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
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_18 = 2;
pub const LAP_STAGE: C2RustUnnamed_18 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const FRAME_UPDATE_TYPES: C2RustUnnamed_19 = 7;
pub const INTNL_ARF_UPDATE: C2RustUnnamed_19 = 6;
pub const INTNL_OVERLAY_UPDATE: C2RustUnnamed_19 = 5;
pub const OVERLAY_UPDATE: C2RustUnnamed_19 = 4;
pub const ARF_UPDATE: C2RustUnnamed_19 = 3;
pub const GF_UPDATE: C2RustUnnamed_19 = 2;
pub const LF_UPDATE: C2RustUnnamed_19 = 1;
pub const KF_UPDATE: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodeFrameInput {
    pub source: *mut YV12_BUFFER_CONFIG,
    pub last_source: *mut YV12_BUFFER_CONFIG,
    pub ts_duration: int64_t,
}
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const FRAME_CONTENT_TYPES: C2RustUnnamed_20 = 2;
pub const FC_GRAPHICS_ANIMATION: C2RustUnnamed_20 = 1;
pub const FC_NORMAL: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodeFrameParams {
    pub error_resilient_mode: core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub primary_ref_frame: core::ffi::c_int,
    pub order_offset: core::ffi::c_int,
    pub show_frame: core::ffi::c_int,
    pub refresh_frame_flags: core::ffi::c_int,
    pub show_existing_frame: core::ffi::c_int,
    pub existing_fb_idx_to_show: core::ffi::c_int,
    pub ref_frame_flags: core::ffi::c_int,
    pub remapped_ref_idx: [core::ffi::c_int; 8],
    pub refresh_frame: RefreshFrameInfo,
    pub speed: core::ffi::c_int,
}
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const FRAMEFLAGS_ERROR_RESILIENT: C2RustUnnamed_21 = 64;
pub const FRAMEFLAGS_SWITCH: C2RustUnnamed_21 = 32;
pub const FRAMEFLAGS_INTRAONLY: C2RustUnnamed_21 = 16;
pub const FRAMEFLAGS_ALTREF: C2RustUnnamed_21 = 8;
pub const FRAMEFLAGS_BWDREF: C2RustUnnamed_21 = 4;
pub const FRAMEFLAGS_GOLDEN: C2RustUnnamed_21 = 2;
pub const FRAMEFLAGS_KEY: C2RustUnnamed_21 = 1;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const RESIZE_MODES: C2RustUnnamed_22 = 4;
pub const RESIZE_DYNAMIC: C2RustUnnamed_22 = 3;
pub const RESIZE_RANDOM: C2RustUnnamed_22 = 2;
pub const RESIZE_FIXED: C2RustUnnamed_22 = 1;
pub const RESIZE_NONE: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const ENABLE_SCENECUT_MODE_2: C2RustUnnamed_23 = 2;
pub const ENABLE_SCENECUT_MODE_1: C2RustUnnamed_23 = 1;
pub const DISABLE_SCENECUT: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const DUCKY_ENCODE_GOP_MODE_RCL: C2RustUnnamed_24 = 1;
pub const DUCKY_ENCODE_GOP_MODE_NONE: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GF_GROUP_STATS {
    pub gf_group_err: core::ffi::c_double,
    pub gf_group_raw_error: core::ffi::c_double,
    pub gf_group_skip_pct: core::ffi::c_double,
    pub gf_group_inactive_zone_rows: core::ffi::c_double,
    pub mv_ratio_accumulator: core::ffi::c_double,
    pub decay_accumulator: core::ffi::c_double,
    pub zero_motion_accumulator: core::ffi::c_double,
    pub loop_decay_rate: core::ffi::c_double,
    pub last_loop_decay_rate: core::ffi::c_double,
    pub this_frame_mv_in_out: core::ffi::c_double,
    pub mv_in_out_accumulator: core::ffi::c_double,
    pub abs_mv_in_out_accumulator: core::ffi::c_double,
    pub avg_sr_coded_error: core::ffi::c_double,
    pub avg_pcnt_second_ref: core::ffi::c_double,
    pub avg_new_mv_count: core::ffi::c_double,
    pub avg_wavelet_energy: core::ffi::c_double,
    pub avg_raw_err_stdev: core::ffi::c_double,
    pub non_zero_stdev_count: core::ffi::c_int,
}
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT64_MAX: core::ffi::c_ulonglong = 18446744073709551615
    as core::ffi::c_ulonglong;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn fclamp(
    mut value: core::ffi::c_double,
    mut low: core::ffi::c_double,
    mut high: core::ffi::c_double,
) -> core::ffi::c_double {
    return if value < low { low } else if value > high { high } else { value };
}
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn frame_is_intra_only(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
pub const BPER_MB_NORMBITS: core::ffi::c_int = 9 as core::ffi::c_int;
pub const STATIC_KF_GROUP_THRESH: core::ffi::c_int = 99 as core::ffi::c_int;
pub const STATIC_KF_GROUP_FLOAT_THRESH: core::ffi::c_double = 0.99f64;
pub const MAX_STATIC_GF_GROUP_LENGTH: core::ffi::c_int = 250 as core::ffi::c_int;
pub const MIN_GF_INTERVAL: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_GF_LENGTH_LAP: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_NUM_GF_INTERVALS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const MAX_ARF_LAYERS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MIN_ZERO_MOTION: core::ffi::c_double = 0.95f64;
pub const MAX_SR_CODED_ERROR: core::ffi::c_int = 40 as core::ffi::c_int;
pub const MAX_RAW_ERR_VAR: core::ffi::c_int = 2000 as core::ffi::c_int;
pub const FC_ANIMATION_THRESH: core::ffi::c_double = 0.15f64;
#[inline]
unsafe extern "C-unwind" fn is_fp_wavelet_energy_invalid(
    mut fp_stats: *const FIRSTPASS_STATS,
) -> core::ffi::c_int {
    return ((*fp_stats).frame_avg_wavelet_energy
        < 0 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_in_operating_point(
    mut operating_point: core::ffi::c_int,
    mut temporal_layer_id: core::ffi::c_int,
    mut spatial_layer_id: core::ffi::c_int,
) -> core::ffi::c_int {
    if operating_point == 0 {
        return 1 as core::ffi::c_int;
    }
    return (operating_point >> temporal_layer_id & 1 as core::ffi::c_int != 0
        && operating_point >> spatial_layer_id + 8 as core::ffi::c_int
            & 1 as core::ffi::c_int != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_use_tpl_for_extrc(mut ext_rc: *const AOM_EXT_RATECTRL) -> bool {
    return (*ext_rc).ready != 0 && ((*ext_rc).funcs.send_tpl_gop_stats).is_some();
}
pub const SCENE_CUT_KEY_TEST_INTERVAL: core::ffi::c_int = 16 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_lossless_requested(
    rc_cfg: *const RateControlCfg,
) -> core::ffi::c_int {
    return ((*rc_cfg).best_allowed_q == 0 as core::ffi::c_int
        && (*rc_cfg).worst_allowed_q == 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn frame_is_kf_gf_arf(mut cpi: *const AV1_COMP) -> core::ffi::c_int {
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let update_type: FRAME_UPDATE_TYPE = (*gf_group)
        .update_type[(*cpi).gf_frame_index as usize];
    return (frame_is_intra_only(&(*cpi).common) != 0
        || update_type as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int)
        as core::ffi::c_int;
}
pub const ALT_MIN_LAG: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_altref_enabled(
    mut lag_in_frames: core::ffi::c_int,
    mut enable_auto_arf: bool,
) -> core::ffi::c_int {
    return (lag_in_frames >= ALT_MIN_LAG && enable_auto_arf as core::ffi::c_int != 0)
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
unsafe extern "C-unwind" fn has_no_stats_stage(cpi: *const AV1_COMP) -> core::ffi::c_int {
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && ((*(*cpi).ppi).lap_enabled == 0
            || (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int))
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_one_pass_rt_lag_params(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).oxcf.gf_cfg.lag_in_frames > 0 as core::ffi::c_int
        && (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_encode_for_extrc(mut ext_rc: *const AOM_EXT_RATECTRL) -> bool {
    return (*ext_rc).ready != 0
        && (*ext_rc).funcs.rc_type as core::ffi::c_uint
            & AOM_RC_QP as core::ffi::c_int as core::ffi::c_uint
            != 0 as core::ffi::c_uint
        && ((*ext_rc).funcs.get_encodeframe_decision).is_some();
}
pub const MAX_GFUBOOST_FACTOR: core::ffi::c_double = 10.0f64;
#[inline]
unsafe extern "C-unwind" fn get_frame_update_type(
    mut gf_group: *const GF_GROUP,
    mut gf_frame_index: core::ffi::c_int,
) -> FRAME_UPDATE_TYPE {
    return (*gf_group).update_type[gf_frame_index as usize];
}
pub const NORMAL_BOOST: core::ffi::c_int = 100 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_get_gfu_boost_projection_factor(
    mut min_factor: core::ffi::c_double,
    mut max_factor: core::ffi::c_double,
    mut frame_count: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut factor: core::ffi::c_double = sqrt(frame_count as core::ffi::c_double);
    factor = if factor < max_factor { factor } else { max_factor };
    factor = if factor > min_factor { factor } else { min_factor };
    factor = 200.0f64 + 10.0f64 * factor;
    return factor;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_kf_boost_projection_factor(
    mut frame_count: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut factor: core::ffi::c_double = sqrt(frame_count as core::ffi::c_double);
    factor = if factor < 10.0f64 { factor } else { 10.0f64 };
    factor = if factor > 4.0f64 { factor } else { 4.0f64 };
    factor = 75.0f64 + 14.0f64 * factor;
    return factor;
}
pub const DEFAULT_KF_BOOST: core::ffi::c_int = 2300 as core::ffi::c_int;
pub const DEFAULT_GF_BOOST: core::ffi::c_int = 2000 as core::ffi::c_int;
pub const MIN_ACTIVE_AREA: core::ffi::c_double = 0.5f64;
pub const MAX_ACTIVE_AREA: core::ffi::c_double = 1.0f64;
unsafe extern "C-unwind" fn calculate_active_area(
    mut frame_info: *const FRAME_INFO,
    mut this_frame: *const FIRSTPASS_STATS,
) -> core::ffi::c_double {
    let active_pct: core::ffi::c_double = 1.0f64
        - ((*this_frame).intra_skip_pct / 2 as core::ffi::c_int as core::ffi::c_double
            + (*this_frame).inactive_zone_rows
                * 2 as core::ffi::c_int as core::ffi::c_double
                / (*frame_info).mb_rows as core::ffi::c_double);
    return fclamp(active_pct, MIN_ACTIVE_AREA, MAX_ACTIVE_AREA);
}
pub const ACT_AREA_CORRECTION: core::ffi::c_double = 0.5f64;
unsafe extern "C-unwind" fn calculate_modified_err_new(
    mut frame_info: *const FRAME_INFO,
    mut total_stats: *const FIRSTPASS_STATS,
    mut this_stats: *const FIRSTPASS_STATS,
    mut vbrbias: core::ffi::c_int,
    mut modified_error_min: core::ffi::c_double,
    mut modified_error_max: core::ffi::c_double,
) -> core::ffi::c_double {
    if total_stats.is_null() {
        return 0 as core::ffi::c_int as core::ffi::c_double;
    }
    let av_weight: core::ffi::c_double = (*total_stats).weight / (*total_stats).count;
    let av_err: core::ffi::c_double = (*total_stats).coded_error * av_weight
        / (*total_stats).count;
    let mut modified_error: core::ffi::c_double = av_err
        * pow(
            (*this_stats).coded_error * (*this_stats).weight
                / (if av_err < 0 as core::ffi::c_int as core::ffi::c_double {
                    av_err - 0.000001f64
                } else {
                    av_err + 0.000001f64
                }),
            vbrbias as core::ffi::c_double / 100.0f64,
        );
    modified_error
        *= pow(calculate_active_area(frame_info, this_stats), ACT_AREA_CORRECTION);
    return fclamp(modified_error, modified_error_min, modified_error_max);
}
unsafe extern "C-unwind" fn calculate_modified_err(
    mut frame_info: *const FRAME_INFO,
    mut twopass: *const TWO_PASS,
    mut oxcf: *const AV1EncoderConfig,
    mut this_frame: *const FIRSTPASS_STATS,
) -> core::ffi::c_double {
    let mut total_stats: *const FIRSTPASS_STATS = (*(*twopass).stats_buf_ctx)
        .total_stats;
    return calculate_modified_err_new(
        frame_info,
        total_stats,
        this_frame,
        (*oxcf).rc_cfg.vbrbias,
        (*twopass).modified_error_min,
        (*twopass).modified_error_max,
    );
}
unsafe extern "C-unwind" fn reset_fpf_position(
    mut p_frame: *mut TWO_PASS_FRAME,
    mut position: *const FIRSTPASS_STATS,
) {
    (*p_frame).stats_in = position;
}
unsafe extern "C-unwind" fn input_stats(
    mut p: *mut TWO_PASS,
    mut p_frame: *mut TWO_PASS_FRAME,
    mut fps: *mut FIRSTPASS_STATS,
) -> core::ffi::c_int {
    if (*p_frame).stats_in
        >= (*(*p).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS
    {
        return EOF;
    }
    *fps = *(*p_frame).stats_in;
    (*p_frame).stats_in = ((*p_frame).stats_in).offset(1);
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn input_stats_lap(
    mut p: *mut TWO_PASS,
    mut p_frame: *mut TWO_PASS_FRAME,
    mut fps: *mut FIRSTPASS_STATS,
) -> core::ffi::c_int {
    if (*p_frame).stats_in
        >= (*(*p).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS
    {
        return EOF;
    }
    *fps = *(*p_frame).stats_in;
    memmove(
        (*p).frame_stats_arr[0 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
        (*p).frame_stats_arr[1 as core::ffi::c_int as usize] as *const core::ffi::c_void,
        ((((*(*p).stats_buf_ctx).stats_in_end).offset_from((*p_frame).stats_in)
            as core::ffi::c_long - 1 as core::ffi::c_long) as size_t)
            .wrapping_mul(::core::mem::size_of::<FIRSTPASS_STATS>() as size_t),
    );
    (*(*p).stats_buf_ctx).stats_in_end = ((*(*p).stats_buf_ctx).stats_in_end).offset(-1);
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_frame_stats(
    mut p: *const TWO_PASS,
    mut p_frame: *const TWO_PASS_FRAME,
    mut offset: core::ffi::c_int,
) -> *const FIRSTPASS_STATS {
    if offset >= 0 as core::ffi::c_int
        && ((*p_frame).stats_in).offset(offset as isize)
            >= (*(*p).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS
        || offset < 0 as core::ffi::c_int
            && ((*p_frame).stats_in).offset(offset as isize)
                < (*(*p).stats_buf_ctx).stats_in_start as *const FIRSTPASS_STATS
    {
        return 0 as *const FIRSTPASS_STATS;
    }
    return &*((*p_frame).stats_in).offset(offset as isize) as *const FIRSTPASS_STATS;
}
unsafe extern "C-unwind" fn frame_max_bits(
    mut rc: *const RATE_CONTROL,
    mut oxcf: *const AV1EncoderConfig,
) -> core::ffi::c_int {
    let mut max_bits: int64_t = (*rc).avg_frame_bandwidth as int64_t
        * (*oxcf).rc_cfg.vbrmax_section as int64_t / 100 as int64_t;
    if max_bits < 0 as int64_t {
        max_bits = 0 as int64_t;
    } else if max_bits > (*rc).max_frame_bandwidth as int64_t {
        max_bits = (*rc).max_frame_bandwidth as int64_t;
    }
    return max_bits as core::ffi::c_int;
}
static mut q_pow_term: [core::ffi::c_double; 9] = [
    0.65f64,
    0.70f64,
    0.75f64,
    0.80f64,
    0.85f64,
    0.90f64,
    0.95f64,
    0.95f64,
    0.95f64,
];
pub const ERR_DIVISOR: core::ffi::c_double = 96.0f64;
unsafe extern "C-unwind" fn calc_correction_factor(
    mut err_per_mb: core::ffi::c_double,
    mut q: core::ffi::c_int,
    mut inactive_zone: core::ffi::c_double,
    mut lower_qindex_on_static_frame: bool,
) -> core::ffi::c_double {
    let mut error_term: core::ffi::c_double = err_per_mb / ERR_DIVISOR;
    if lower_qindex_on_static_frame {
        error_term = error_term
            * (1 as core::ffi::c_int as core::ffi::c_double
                - (if 0.6f64 < inactive_zone { 0.6f64 } else { inactive_zone }));
    }
    let index: core::ffi::c_int = q >> 5 as core::ffi::c_int;
    let power_term: core::ffi::c_double = q_pow_term[index as usize]
        + (q_pow_term[(index + 1 as core::ffi::c_int) as usize]
            - q_pow_term[index as usize])
            * (q % 32 as core::ffi::c_int) as core::ffi::c_double / 32.0f64;
    return fclamp(pow(error_term, power_term), 0.05f64, 5.0f64);
}
unsafe extern "C-unwind" fn twopass_update_bpm_factor(
    mut cpi: *mut AV1_COMP,
    mut rate_err_tol: core::ffi::c_int,
) {
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut damp_fac: core::ffi::c_double = if 5.0f64
        > rate_err_tol as core::ffi::c_double / 10.0f64
    {
        5.0f64
    } else {
        rate_err_tol as core::ffi::c_double / 10.0f64
    };
    let mut rate_err_factor: core::ffi::c_double = 1.0f64;
    let mut err_estimate: core::ffi::c_int = (*p_rc).rate_error_estimate;
    let mut bits_left: int64_t = (*twopass).bits_left;
    let mut total_actual_bits: int64_t = (*p_rc).total_actual_bits;
    let mut bits_off_target: int64_t = (*p_rc).vbr_bits_off_target;
    let mut rolling_arf_group_actual_bits: core::ffi::c_double = (*twopass)
        .rolling_arf_group_actual_bits as core::ffi::c_double;
    let mut rolling_arf_group_target_bits: core::ffi::c_double = (*twopass)
        .rolling_arf_group_target_bits as core::ffi::c_double;
    let mut adj_limit_cap: core::ffi::c_double = 0.;
    if rate_err_tol != 0 {
        let initial_adj_limit: core::ffi::c_double = 0.0626f64;
        let abs_err_tol: core::ffi::c_double = 0.6f64
            * rate_err_tol as core::ffi::c_double;
        let step_size: core::ffi::c_double = (0.2f64 - initial_adj_limit)
            / (rate_err_tol as core::ffi::c_double - abs_err_tol);
        let abs_err_till_now: core::ffi::c_double = 100 as core::ffi::c_int
            as core::ffi::c_double
            * fabs(
                bits_off_target as core::ffi::c_double
                    / (if total_actual_bits > bits_left {
                        total_actual_bits
                    } else {
                        bits_left
                    }) as core::ffi::c_double,
            ) as core::ffi::c_double;
        adj_limit_cap = initial_adj_limit;
        if abs_err_till_now >= abs_err_tol {
            adj_limit_cap = if 0.2f64
                < initial_adj_limit + (abs_err_till_now - abs_err_tol) * step_size
            {
                0.2f64
            } else {
                initial_adj_limit + (abs_err_till_now - abs_err_tol) * step_size
            };
        }
    } else {
        adj_limit_cap = 0.2f64;
    }
    let adj_limit: core::ffi::c_double = if adj_limit_cap
        > (100 as core::ffi::c_int - rate_err_tol) as core::ffi::c_double / 200.0f64
    {
        adj_limit_cap
    } else {
        (100 as core::ffi::c_int - rate_err_tol) as core::ffi::c_double / 200.0f64
    };
    let min_fac: core::ffi::c_double = 1.0f64 - adj_limit;
    let max_fac: core::ffi::c_double = 1.0f64 + adj_limit;
    if (*p_rc).bits_off_target != 0 && total_actual_bits > 0 as int64_t {
        if (*(*cpi).ppi).lap_enabled != 0 {
            rate_err_factor = rolling_arf_group_actual_bits
                / (if rolling_arf_group_target_bits
                    < 0 as core::ffi::c_int as core::ffi::c_double
                {
                    rolling_arf_group_target_bits - 0.000001f64
                } else {
                    rolling_arf_group_target_bits + 0.000001f64
                });
        } else {
            rate_err_factor = 1.0f64
                - bits_off_target as core::ffi::c_double
                    / (if total_actual_bits > bits_left {
                        total_actual_bits
                    } else {
                        bits_left
                    }) as core::ffi::c_double;
        }
        if (*twopass).bpm_factor != 1.0f64 || (*(*cpi).ppi).lap_enabled != 0 {
            rate_err_factor = 1.0f64 + (rate_err_factor - 1.0f64) / damp_fac;
        }
        rate_err_factor = if min_fac
            > (if max_fac < rate_err_factor { max_fac } else { rate_err_factor })
        {
            min_fac
        } else if max_fac < rate_err_factor {
            max_fac
        } else {
            rate_err_factor
        };
    }
    if rate_err_factor < 1.0f64 && err_estimate >= 0 as core::ffi::c_int
        || rate_err_factor > 1.0f64 && err_estimate <= 0 as core::ffi::c_int
    {
        (*twopass).bpm_factor *= rate_err_factor;
        if rate_err_tol >= 100 as core::ffi::c_int {
            (*twopass).bpm_factor = if min_fac
                > (if max_fac < (*twopass).bpm_factor {
                    max_fac
                } else {
                    (*twopass).bpm_factor
                })
            {
                min_fac
            } else if max_fac < (*twopass).bpm_factor {
                max_fac
            } else {
                (*twopass).bpm_factor
            };
        } else {
            (*twopass).bpm_factor = if 0.1f64
                > (if 10.0f64 < (*twopass).bpm_factor {
                    10.0f64
                } else {
                    (*twopass).bpm_factor
                })
            {
                0.1f64
            } else if 10.0f64 < (*twopass).bpm_factor {
                10.0f64
            } else {
                (*twopass).bpm_factor
            };
        }
    }
}
unsafe extern "C-unwind" fn qbpm_enumerator(
    mut rate_err_tol: core::ffi::c_int,
    mut use_smaller_enumerator: bool,
) -> core::ffi::c_int {
    return (if use_smaller_enumerator as core::ffi::c_int != 0 {
        1050000 as core::ffi::c_int
    } else {
        1125750 as core::ffi::c_int
    })
        + 300000 as core::ffi::c_int
            * (if (75 as core::ffi::c_int)
                < (if rate_err_tol - 25 as core::ffi::c_int > 0 as core::ffi::c_int {
                    rate_err_tol - 25 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                })
            {
                75 as core::ffi::c_int
            } else {
                (if rate_err_tol - 25 as core::ffi::c_int > 0 as core::ffi::c_int {
                    rate_err_tol - 25 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                })
            }) / 75 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn find_qindex_by_rate_with_correction(
    mut desired_bits_per_mb: uint64_t,
    mut bit_depth: aom_bit_depth_t,
    mut error_per_mb: core::ffi::c_double,
    mut lower_qindex_on_static_frame: bool,
    mut group_weight_factor: core::ffi::c_double,
    mut rate_err_tol: core::ffi::c_int,
    mut best_qindex: core::ffi::c_int,
    mut worst_qindex: core::ffi::c_int,
    mut use_smaller_enumerator: bool,
    mut inactive_zone: core::ffi::c_double,
) -> core::ffi::c_int {
    let mut low: core::ffi::c_int = best_qindex;
    let mut high: core::ffi::c_int = worst_qindex;
    while low < high {
        let mid: core::ffi::c_int = low + high >> 1 as core::ffi::c_int;
        let mid_factor: core::ffi::c_double = calc_correction_factor(
            error_per_mb,
            mid,
            inactive_zone,
            lower_qindex_on_static_frame,
        ) as core::ffi::c_double;
        let q: core::ffi::c_double = av1_convert_qindex_to_q(mid, bit_depth)
            as core::ffi::c_double;
        let enumerator: core::ffi::c_int = qbpm_enumerator(
            rate_err_tol,
            use_smaller_enumerator,
        ) as core::ffi::c_int;
        let mid_bits_per_mb: uint64_t = (enumerator as core::ffi::c_double * mid_factor
            * group_weight_factor / q) as uint64_t;
        if mid_bits_per_mb > desired_bits_per_mb {
            low = mid + 1 as core::ffi::c_int;
        } else {
            high = mid;
        }
    }
    return low;
}
unsafe extern "C-unwind" fn get_twopass_worst_quality(
    mut cpi: *mut AV1_COMP,
    av_frame_err: core::ffi::c_double,
    mut inactive_zone: core::ffi::c_double,
    mut av_target_bandwidth: core::ffi::c_int,
) -> core::ffi::c_int {
    let rc: *const RATE_CONTROL = &raw mut (*cpi).rc;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    inactive_zone = fclamp(inactive_zone, 0.0f64, 0.9999f64);
    let use_smaller_enumerator: bool = if (*(*cpi).ppi).p_rc.total_actual_bits
        == 0 as int64_t
    {
        true_0
    } else {
        false_0
    } != 0;
    if av_target_bandwidth <= 0 as core::ffi::c_int {
        return (*rc).worst_quality
    } else {
        let num_mbs: core::ffi::c_int = if (*oxcf).resize_cfg.resize_mode
            as core::ffi::c_int != RESIZE_NONE as core::ffi::c_int
        {
            (*cpi).initial_mbs
        } else {
            (*cpi).common.mi_params.MBs
        };
        let active_mbs: core::ffi::c_int = if 1 as core::ffi::c_int
            > num_mbs
                - (num_mbs as core::ffi::c_double * inactive_zone) as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            num_mbs
                - (num_mbs as core::ffi::c_double * inactive_zone) as core::ffi::c_int
        };
        let av_err_per_mb: core::ffi::c_double = av_frame_err / (1.0f64 - inactive_zone);
        let target_norm_bits_per_mb: uint64_t = ((av_target_bandwidth as uint64_t)
            << BPER_MB_NORMBITS)
            .wrapping_div(active_mbs as uint64_t);
        let mut rate_err_tol: core::ffi::c_int = if (*rc_cfg).under_shoot_pct
            < (*rc_cfg).over_shoot_pct
        {
            (*rc_cfg).under_shoot_pct
        } else {
            (*rc_cfg).over_shoot_pct
        };
        twopass_update_bpm_factor(cpi, rate_err_tol);
        let mut baseline_gf_interval: core::ffi::c_int = (*(*cpi).ppi)
            .p_rc
            .baseline_gf_interval - 1 as core::ffi::c_int;
        let mut cur_arf_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
            &raw mut (*(*cpi).ppi).twopass.firstpass_info,
            baseline_gf_interval,
        );
        let mut lower_qindex_on_static_frame: bool = false_0 != 0;
        if !cur_arf_stats.is_null() {
            lower_qindex_on_static_frame = (*cur_arf_stats).coded_error
                < 2 as core::ffi::c_int as core::ffi::c_double * av_err_per_mb;
        }
        let mut q: core::ffi::c_int = find_qindex_by_rate_with_correction(
            target_norm_bits_per_mb,
            (*(*cpi).common.seq_params).bit_depth,
            av_err_per_mb,
            lower_qindex_on_static_frame,
            (*(*cpi).ppi).twopass.bpm_factor,
            rate_err_tol,
            (*rc).best_quality,
            (*rc).worst_quality,
            use_smaller_enumerator,
            inactive_zone,
        );
        if (*rc_cfg).mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        {
            q = if q > (*rc_cfg).cq_level { q } else { (*rc_cfg).cq_level };
        }
        return q;
    };
}
pub const INTRA_PART: core::ffi::c_double = 0.005f64;
pub const LOW_SR_DIFF_TRHESH: core::ffi::c_double = 0.01f64;
pub const NCOUNT_FRAME_II_THRESH: core::ffi::c_double = 5.0f64;
pub const LOW_CODED_ERR_PER_MB: core::ffi::c_double = 0.01f64;
unsafe extern "C-unwind" fn get_sr_decay_rate(
    mut frame: *const FIRSTPASS_STATS,
) -> core::ffi::c_double {
    let mut sr_diff: core::ffi::c_double = (*frame).sr_coded_error
        - (*frame).coded_error;
    let mut sr_decay: core::ffi::c_double = 1.0f64;
    let mut modified_pct_inter: core::ffi::c_double = 0.;
    let mut modified_pcnt_intra: core::ffi::c_double = 0.;
    modified_pct_inter = (*frame).pcnt_inter;
    if (*frame).coded_error > LOW_CODED_ERR_PER_MB
        && (*frame).intra_error
            / (if (*frame).coded_error < 0 as core::ffi::c_int as core::ffi::c_double {
                (*frame).coded_error - 0.000001f64
            } else {
                (*frame).coded_error + 0.000001f64
            }) < NCOUNT_FRAME_II_THRESH
    {
        modified_pct_inter = (*frame).pcnt_inter - (*frame).pcnt_neutral;
    }
    modified_pcnt_intra = 100 as core::ffi::c_int as core::ffi::c_double
        * (1.0f64 - modified_pct_inter);
    if sr_diff > LOW_SR_DIFF_TRHESH {
        let mut sr_diff_part: core::ffi::c_double = sr_diff * 0.25f64
            / (*frame).intra_error;
        sr_decay = 1.0f64 - sr_diff_part - INTRA_PART * modified_pcnt_intra;
    }
    return if sr_decay > 0.75f64 { sr_decay } else { 0.75f64 };
}
unsafe extern "C-unwind" fn get_zero_motion_factor(
    mut frame: *const FIRSTPASS_STATS,
) -> core::ffi::c_double {
    let zero_motion_pct: core::ffi::c_double = (*frame).pcnt_inter
        - (*frame).pcnt_motion;
    let mut sr_decay: core::ffi::c_double = get_sr_decay_rate(frame);
    return if sr_decay < zero_motion_pct { sr_decay } else { zero_motion_pct };
}
pub const DEFAULT_ZM_FACTOR: core::ffi::c_double = 0.5f64;
unsafe extern "C-unwind" fn get_prediction_decay_rate(
    mut frame_stats: *const FIRSTPASS_STATS,
) -> core::ffi::c_double {
    let sr_decay_rate: core::ffi::c_double = get_sr_decay_rate(frame_stats)
        as core::ffi::c_double;
    let mut zero_motion_factor: core::ffi::c_double = DEFAULT_ZM_FACTOR
        * ((*frame_stats).pcnt_inter - (*frame_stats).pcnt_motion);
    if zero_motion_factor > 1.0f64 {
        zero_motion_factor = 1.0f64;
    } else if zero_motion_factor < 0.0f64 {
        zero_motion_factor = 0.0f64;
    }
    return if zero_motion_factor
        > sr_decay_rate + (1.0f64 - sr_decay_rate) * zero_motion_factor
    {
        zero_motion_factor
    } else {
        sr_decay_rate + (1.0f64 - sr_decay_rate) * zero_motion_factor
    };
}
unsafe extern "C-unwind" fn detect_transition_to_still(
    mut firstpass_info: *const FIRSTPASS_INFO,
    mut next_stats_index: core::ffi::c_int,
    min_gf_interval: core::ffi::c_int,
    frame_interval: core::ffi::c_int,
    still_interval: core::ffi::c_int,
    loop_decay_rate: core::ffi::c_double,
    last_decay_rate: core::ffi::c_double,
) -> core::ffi::c_int {
    if frame_interval > min_gf_interval && loop_decay_rate >= 0.999f64
        && last_decay_rate < 0.9f64
    {
        let mut stats_left: core::ffi::c_int = av1_firstpass_info_future_count(
            firstpass_info,
            next_stats_index,
        );
        if stats_left >= still_interval {
            let mut j: core::ffi::c_int = 0;
            j = 0 as core::ffi::c_int;
            while j < still_interval {
                let mut stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
                    firstpass_info,
                    next_stats_index + j,
                );
                if (*stats).pcnt_inter - (*stats).pcnt_motion < 0.999f64 {
                    break;
                }
                j += 1;
            }
            return (j == still_interval) as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn detect_flash(
    mut twopass: *const TWO_PASS,
    mut twopass_frame: *const TWO_PASS_FRAME,
    offset: core::ffi::c_int,
) -> core::ffi::c_int {
    let next_frame: *const FIRSTPASS_STATS = read_frame_stats(
        twopass,
        twopass_frame,
        offset,
    ) as *const FIRSTPASS_STATS;
    return (!next_frame.is_null()
        && (*next_frame).pcnt_second_ref > (*next_frame).pcnt_inter
        && (*next_frame).pcnt_second_ref >= 0.5f64) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn accumulate_frame_motion_stats(
    mut stats: *const FIRSTPASS_STATS,
    mut gf_stats: *mut GF_GROUP_STATS,
    mut f_w: core::ffi::c_double,
    mut f_h: core::ffi::c_double,
) {
    let pct: core::ffi::c_double = (*stats).pcnt_motion;
    (*gf_stats).this_frame_mv_in_out = (*stats).mv_in_out_count * pct;
    (*gf_stats).mv_in_out_accumulator += (*gf_stats).this_frame_mv_in_out;
    (*gf_stats).abs_mv_in_out_accumulator += fabs((*gf_stats).this_frame_mv_in_out);
    if pct > 0.05f64 {
        let mvr_ratio: core::ffi::c_double = fabs((*stats).mvr_abs)
            as core::ffi::c_double
            / (if fabs((*stats).MVr) < 0 as core::ffi::c_int as core::ffi::c_double {
                fabs((*stats).MVr) as core::ffi::c_double - 0.000001f64
            } else {
                fabs((*stats).MVr) as core::ffi::c_double + 0.000001f64
            });
        let mvc_ratio: core::ffi::c_double = fabs((*stats).mvc_abs)
            as core::ffi::c_double
            / (if fabs((*stats).MVc) < 0 as core::ffi::c_int as core::ffi::c_double {
                fabs((*stats).MVc) as core::ffi::c_double - 0.000001f64
            } else {
                fabs((*stats).MVc) as core::ffi::c_double + 0.000001f64
            });
        (*gf_stats).mv_ratio_accumulator
            += pct
                * (if mvr_ratio < (*stats).mvr_abs * f_h {
                    mvr_ratio
                } else {
                    (*stats).mvr_abs * f_h
                });
        (*gf_stats).mv_ratio_accumulator
            += pct
                * (if mvc_ratio < (*stats).mvc_abs * f_w {
                    mvc_ratio
                } else {
                    (*stats).mvc_abs * f_w
                });
    }
}
unsafe extern "C-unwind" fn accumulate_this_frame_stats(
    mut stats: *const FIRSTPASS_STATS,
    mod_frame_err: core::ffi::c_double,
    mut gf_stats: *mut GF_GROUP_STATS,
) {
    (*gf_stats).gf_group_err += mod_frame_err;
    (*gf_stats).gf_group_raw_error += (*stats).coded_error;
    (*gf_stats).gf_group_skip_pct += (*stats).intra_skip_pct;
    (*gf_stats).gf_group_inactive_zone_rows += (*stats).inactive_zone_rows;
}
unsafe extern "C-unwind" fn accumulate_next_frame_stats(
    mut stats: *const FIRSTPASS_STATS,
    flash_detected: core::ffi::c_int,
    frames_since_key: core::ffi::c_int,
    cur_idx: core::ffi::c_int,
    mut gf_stats: *mut GF_GROUP_STATS,
    mut f_w: core::ffi::c_int,
    mut f_h: core::ffi::c_int,
) {
    accumulate_frame_motion_stats(
        stats,
        gf_stats,
        f_w as core::ffi::c_double,
        f_h as core::ffi::c_double,
    );
    (*gf_stats).avg_sr_coded_error += (*stats).sr_coded_error;
    (*gf_stats).avg_pcnt_second_ref += (*stats).pcnt_second_ref;
    (*gf_stats).avg_new_mv_count += (*stats).new_mv_count;
    (*gf_stats).avg_wavelet_energy += (*stats).frame_avg_wavelet_energy;
    if fabs((*stats).raw_error_stdev) > 0.000001f64 {
        (*gf_stats).non_zero_stdev_count += 1;
        (*gf_stats).avg_raw_err_stdev += (*stats).raw_error_stdev;
    }
    if flash_detected == 0 {
        (*gf_stats).last_loop_decay_rate = (*gf_stats).loop_decay_rate;
        (*gf_stats).loop_decay_rate = get_prediction_decay_rate(stats);
        (*gf_stats).decay_accumulator = (*gf_stats).decay_accumulator
            * (*gf_stats).loop_decay_rate;
        if frames_since_key + cur_idx - 1 as core::ffi::c_int > 1 as core::ffi::c_int {
            (*gf_stats).zero_motion_accumulator = if (*gf_stats).zero_motion_accumulator
                < get_zero_motion_factor(stats)
            {
                (*gf_stats).zero_motion_accumulator
            } else {
                get_zero_motion_factor(stats)
            };
        }
    }
}
unsafe extern "C-unwind" fn average_gf_stats(
    total_frame: core::ffi::c_int,
    mut gf_stats: *mut GF_GROUP_STATS,
) {
    if total_frame != 0 {
        (*gf_stats).avg_sr_coded_error /= total_frame as core::ffi::c_double;
        (*gf_stats).avg_pcnt_second_ref /= total_frame as core::ffi::c_double;
        (*gf_stats).avg_new_mv_count /= total_frame as core::ffi::c_double;
        (*gf_stats).avg_wavelet_energy /= total_frame as core::ffi::c_double;
    }
    if (*gf_stats).non_zero_stdev_count != 0 {
        (*gf_stats).avg_raw_err_stdev
            /= (*gf_stats).non_zero_stdev_count as core::ffi::c_double;
    }
}
pub const BOOST_FACTOR: core::ffi::c_double = 12.5f64;
unsafe extern "C-unwind" fn baseline_err_per_mb(
    mut frame_info: *const FRAME_INFO,
) -> core::ffi::c_double {
    let mut screen_area: core::ffi::c_uint = ((*frame_info).frame_height
        * (*frame_info).frame_width) as core::ffi::c_uint;
    if screen_area
        <= (640 as core::ffi::c_int * 360 as core::ffi::c_int) as core::ffi::c_uint
    {
        return 500.0f64
    } else {
        return 1000.0f64
    };
}
unsafe extern "C-unwind" fn calc_frame_boost(
    mut p_rc: *const PRIMARY_RATE_CONTROL,
    mut frame_info: *const FRAME_INFO,
    mut this_frame: *const FIRSTPASS_STATS,
    mut this_frame_mv_in_out: core::ffi::c_double,
    mut max_boost: core::ffi::c_double,
    scale_max_boost: bool,
) -> core::ffi::c_double {
    let mut frame_boost: core::ffi::c_double = 0.;
    let lq: core::ffi::c_double = av1_convert_qindex_to_q(
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize],
        (*frame_info).bit_depth,
    ) as core::ffi::c_double;
    let boost_q_correction: core::ffi::c_double = if 0.5f64 + lq * 0.015f64 < 1.5f64 {
        0.5f64 + lq * 0.015f64
    } else {
        1.5f64
    };
    let active_area: core::ffi::c_double = calculate_active_area(frame_info, this_frame)
        as core::ffi::c_double;
    frame_boost = (if baseline_err_per_mb(frame_info) * active_area
        > (*this_frame).intra_error * active_area
    {
        baseline_err_per_mb(frame_info) * active_area
    } else {
        (*this_frame).intra_error * active_area
    })
        / (if (*this_frame).coded_error < 0 as core::ffi::c_int as core::ffi::c_double {
            (*this_frame).coded_error - 0.000001f64
        } else {
            (*this_frame).coded_error + 0.000001f64
        });
    frame_boost = frame_boost * BOOST_FACTOR * boost_q_correction;
    if this_frame_mv_in_out > 0.0f64 {
        frame_boost += frame_boost * (this_frame_mv_in_out * 2.0f64);
        if scale_max_boost {
            max_boost += max_boost * (this_frame_mv_in_out * 2.0f64);
        }
    } else {
        frame_boost += frame_boost * (this_frame_mv_in_out / 2.0f64);
    }
    return if frame_boost < max_boost * boost_q_correction {
        frame_boost
    } else {
        max_boost * boost_q_correction
    };
}
unsafe extern "C-unwind" fn calc_kf_frame_boost(
    mut p_rc: *const PRIMARY_RATE_CONTROL,
    mut frame_info: *const FRAME_INFO,
    mut this_frame: *const FIRSTPASS_STATS,
    mut sr_accumulator: *mut core::ffi::c_double,
    mut max_boost: core::ffi::c_double,
) -> core::ffi::c_double {
    let mut frame_boost: core::ffi::c_double = 0.;
    let lq: core::ffi::c_double = av1_convert_qindex_to_q(
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize],
        (*frame_info).bit_depth,
    ) as core::ffi::c_double;
    let boost_q_correction: core::ffi::c_double = if 0.50f64 + lq * 0.015f64 < 2.00f64 {
        0.50f64 + lq * 0.015f64
    } else {
        2.00f64
    };
    let active_area: core::ffi::c_double = calculate_active_area(frame_info, this_frame)
        as core::ffi::c_double;
    frame_boost = (if baseline_err_per_mb(frame_info) * active_area
        > (*this_frame).intra_error * active_area
    {
        baseline_err_per_mb(frame_info) * active_area
    } else {
        (*this_frame).intra_error * active_area
    })
        / (if ((*this_frame).coded_error + *sr_accumulator) * active_area
            < 0 as core::ffi::c_int as core::ffi::c_double
        {
            ((*this_frame).coded_error + *sr_accumulator) * active_area - 0.000001f64
        } else {
            ((*this_frame).coded_error + *sr_accumulator) * active_area + 0.000001f64
        });
    *sr_accumulator += (*this_frame).sr_coded_error - (*this_frame).coded_error;
    *sr_accumulator = if 0.0f64 > *sr_accumulator { 0.0f64 } else { *sr_accumulator };
    frame_boost = (frame_boost + 40.0f64) * boost_q_correction;
    return if frame_boost < max_boost * boost_q_correction {
        frame_boost
    } else {
        max_boost * boost_q_correction
    };
}
unsafe extern "C-unwind" fn get_projected_gfu_boost(
    mut p_rc: *const PRIMARY_RATE_CONTROL,
    mut gfu_boost: core::ffi::c_int,
    mut frames_to_project: core::ffi::c_int,
    mut num_stats_used_for_gfu_boost: core::ffi::c_int,
) -> core::ffi::c_int {
    if num_stats_used_for_gfu_boost >= frames_to_project {
        return gfu_boost;
    }
    let mut min_boost_factor: core::ffi::c_double = sqrt(
        (*p_rc).baseline_gf_interval as core::ffi::c_double,
    );
    let mut tpl_factor: core::ffi::c_double = av1_get_gfu_boost_projection_factor(
        min_boost_factor,
        MAX_GFUBOOST_FACTOR,
        frames_to_project,
    );
    let mut tpl_factor_num_stats: core::ffi::c_double = av1_get_gfu_boost_projection_factor(
        min_boost_factor,
        MAX_GFUBOOST_FACTOR,
        num_stats_used_for_gfu_boost,
    );
    let mut projected_gfu_boost: core::ffi::c_int = rint(
        tpl_factor * gfu_boost as core::ffi::c_double / tpl_factor_num_stats,
    ) as core::ffi::c_int;
    return projected_gfu_boost;
}
pub const GF_MAX_BOOST: core::ffi::c_double = 90.0f64;
pub const GF_MIN_BOOST: core::ffi::c_int = 50 as core::ffi::c_int;
pub const MIN_DECAY_FACTOR: core::ffi::c_double = 0.01f64;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_arf_boost(
    mut twopass: *const TWO_PASS,
    mut twopass_frame: *const TWO_PASS_FRAME,
    mut p_rc: *const PRIMARY_RATE_CONTROL,
    mut frame_info: *mut FRAME_INFO,
    mut offset: core::ffi::c_int,
    mut f_frames: core::ffi::c_int,
    mut b_frames: core::ffi::c_int,
    mut num_fpstats_used: *mut core::ffi::c_int,
    mut num_fpstats_required: *mut core::ffi::c_int,
    mut project_gfu_boost: core::ffi::c_int,
    scale_max_boost: bool,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut gf_stats: GF_GROUP_STATS = GF_GROUP_STATS {
        gf_group_err: 0.,
        gf_group_raw_error: 0.,
        gf_group_skip_pct: 0.,
        gf_group_inactive_zone_rows: 0.,
        mv_ratio_accumulator: 0.,
        decay_accumulator: 0.,
        zero_motion_accumulator: 0.,
        loop_decay_rate: 0.,
        last_loop_decay_rate: 0.,
        this_frame_mv_in_out: 0.,
        mv_in_out_accumulator: 0.,
        abs_mv_in_out_accumulator: 0.,
        avg_sr_coded_error: 0.,
        avg_pcnt_second_ref: 0.,
        avg_new_mv_count: 0.,
        avg_wavelet_energy: 0.,
        avg_raw_err_stdev: 0.,
        non_zero_stdev_count: 0,
    };
    init_gf_stats(&raw mut gf_stats);
    let mut boost_score: core::ffi::c_double = NORMAL_BOOST as core::ffi::c_double;
    let mut arf_boost: core::ffi::c_int = 0;
    let mut flash_detected: core::ffi::c_int = 0 as core::ffi::c_int;
    if !num_fpstats_used.is_null() {
        *num_fpstats_used = 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < f_frames {
        let mut this_frame: *const FIRSTPASS_STATS = read_frame_stats(
            twopass,
            twopass_frame,
            i + offset,
        );
        if this_frame.is_null() {
            break;
        }
        accumulate_frame_motion_stats(
            this_frame,
            &raw mut gf_stats,
            (*frame_info).frame_width as core::ffi::c_double,
            (*frame_info).frame_height as core::ffi::c_double,
        );
        flash_detected = (detect_flash(twopass, twopass_frame, i + offset) != 0
            || detect_flash(twopass, twopass_frame, i + offset + 1 as core::ffi::c_int)
                != 0) as core::ffi::c_int;
        if flash_detected == 0 {
            gf_stats.decay_accumulator *= get_prediction_decay_rate(this_frame);
            gf_stats.decay_accumulator = if gf_stats.decay_accumulator < MIN_DECAY_FACTOR
            {
                MIN_DECAY_FACTOR
            } else {
                gf_stats.decay_accumulator
            };
        }
        boost_score
            += gf_stats.decay_accumulator
                * calc_frame_boost(
                    p_rc,
                    frame_info,
                    this_frame,
                    gf_stats.this_frame_mv_in_out,
                    GF_MAX_BOOST,
                    scale_max_boost,
                );
        if !num_fpstats_used.is_null() {
            *num_fpstats_used += 1;
        }
        i += 1;
    }
    arf_boost = boost_score as core::ffi::c_int;
    boost_score = 0.0f64;
    init_gf_stats(&raw mut gf_stats);
    i = -(1 as core::ffi::c_int);
    while i >= -b_frames {
        let mut this_frame_0: *const FIRSTPASS_STATS = read_frame_stats(
            twopass,
            twopass_frame,
            i + offset,
        );
        if this_frame_0.is_null() {
            break;
        }
        accumulate_frame_motion_stats(
            this_frame_0,
            &raw mut gf_stats,
            (*frame_info).frame_width as core::ffi::c_double,
            (*frame_info).frame_height as core::ffi::c_double,
        );
        flash_detected = (detect_flash(twopass, twopass_frame, i + offset) != 0
            || detect_flash(twopass, twopass_frame, i + offset + 1 as core::ffi::c_int)
                != 0) as core::ffi::c_int;
        if flash_detected == 0 {
            gf_stats.decay_accumulator *= get_prediction_decay_rate(this_frame_0);
            gf_stats.decay_accumulator = if gf_stats.decay_accumulator < MIN_DECAY_FACTOR
            {
                MIN_DECAY_FACTOR
            } else {
                gf_stats.decay_accumulator
            };
        }
        boost_score
            += gf_stats.decay_accumulator
                * calc_frame_boost(
                    p_rc,
                    frame_info,
                    this_frame_0,
                    gf_stats.this_frame_mv_in_out,
                    GF_MAX_BOOST,
                    scale_max_boost,
                );
        if !num_fpstats_used.is_null() {
            *num_fpstats_used += 1;
        }
        i -= 1;
    }
    arf_boost += boost_score as core::ffi::c_int;
    if project_gfu_boost != 0 {
        *num_fpstats_required = f_frames + b_frames;
        arf_boost = get_projected_gfu_boost(
            p_rc,
            arf_boost,
            *num_fpstats_required,
            *num_fpstats_used,
        );
    }
    if arf_boost < (b_frames + f_frames) * GF_MIN_BOOST {
        arf_boost = (b_frames + f_frames) * GF_MIN_BOOST;
    }
    return arf_boost;
}
unsafe extern "C-unwind" fn calculate_section_intra_ratio(
    mut begin: *const FIRSTPASS_STATS,
    mut end: *const FIRSTPASS_STATS,
    mut section_length: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut s: *const FIRSTPASS_STATS = begin;
    let mut intra_error: core::ffi::c_double = 0.0f64;
    let mut coded_error: core::ffi::c_double = 0.0f64;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while s < end && i < section_length {
        intra_error += (*s).intra_error;
        coded_error += (*s).coded_error;
        s = s.offset(1);
        i += 1;
    }
    return (intra_error
        / (if coded_error < 0 as core::ffi::c_int as core::ffi::c_double {
            coded_error - 0.000001f64
        } else {
            coded_error + 0.000001f64
        })) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn calculate_total_gf_group_bits(
    mut cpi: *mut AV1_COMP,
    mut gf_group_err: core::ffi::c_double,
) -> int64_t {
    let rc: *const RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *const TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let max_bits: core::ffi::c_int = frame_max_bits(rc, &raw mut (*cpi).oxcf)
        as core::ffi::c_int;
    let mut total_group_bits: int64_t = 0;
    if (*twopass).kf_group_bits > 0 as int64_t
        && (*twopass).kf_group_error_left > 0 as core::ffi::c_int as core::ffi::c_double
    {
        total_group_bits = ((*twopass).kf_group_bits as core::ffi::c_double
            * (gf_group_err / (*twopass).kf_group_error_left)) as int64_t;
    } else {
        total_group_bits = 0 as int64_t;
    }
    total_group_bits = if total_group_bits < 0 as int64_t {
        0 as int64_t
    } else if total_group_bits > (*twopass).kf_group_bits {
        (*twopass).kf_group_bits
    } else {
        total_group_bits
    };
    if total_group_bits > max_bits as int64_t * (*p_rc).baseline_gf_interval as int64_t {
        total_group_bits = max_bits as int64_t * (*p_rc).baseline_gf_interval as int64_t;
    }
    return total_group_bits;
}
unsafe extern "C-unwind" fn calculate_boost_bits(
    mut frame_count: core::ffi::c_int,
    mut boost: core::ffi::c_int,
    mut total_group_bits: int64_t,
) -> core::ffi::c_int {
    let mut allocation_chunks: core::ffi::c_int = 0;
    if boost == 0 || total_group_bits <= 0 as int64_t {
        return 0 as core::ffi::c_int;
    }
    if frame_count <= 0 as core::ffi::c_int {
        return (if total_group_bits < 2147483647 as int64_t {
            total_group_bits
        } else {
            2147483647 as int64_t
        }) as core::ffi::c_int;
    }
    allocation_chunks = frame_count * 100 as core::ffi::c_int + boost;
    if boost > 1023 as core::ffi::c_int {
        let mut divisor: core::ffi::c_int = boost >> 10 as core::ffi::c_int;
        boost /= divisor;
        allocation_chunks /= divisor;
    }
    return if (boost as int64_t * total_group_bits / allocation_chunks as int64_t)
        as core::ffi::c_int > 0 as core::ffi::c_int
    {
        (boost as int64_t * total_group_bits / allocation_chunks as int64_t)
            as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn calculate_boost_factor(
    mut frame_count: core::ffi::c_int,
    mut bits: core::ffi::c_int,
    mut total_group_bits: int64_t,
) -> core::ffi::c_int {
    return (100.0f64 * frame_count as core::ffi::c_double * bits as core::ffi::c_double
        / (total_group_bits - bits as int64_t) as core::ffi::c_double)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn adjust_boost_bits_for_target_level(
    cpi: *const AV1_COMP,
    rc: *mut RATE_CONTROL,
    mut bits_assigned: core::ffi::c_int,
    mut group_bits: int64_t,
    mut frame_type: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let temporal_layer_id: core::ffi::c_int = (*cm).temporal_layer_id;
    let spatial_layer_id: core::ffi::c_int = (*cm).spatial_layer_id;
    let mut index: core::ffi::c_int = 0 as core::ffi::c_int;
    while index < (*seq_params).operating_points_cnt_minus_1 + 1 as core::ffi::c_int {
        if !(is_in_operating_point(
            (*seq_params).operating_point_idc[index as usize],
            temporal_layer_id,
            spatial_layer_id,
        ) == 0)
        {
            let target_level: AV1_LEVEL = (*(*cpi).ppi)
                .level_params
                .target_seq_level_idx[index as usize];
            if !(target_level as core::ffi::c_int >= SEQ_LEVELS as core::ffi::c_int) {
                let level_bitrate_limit: core::ffi::c_double = av1_get_max_bitrate_for_level(
                    target_level,
                    (*seq_params).tier[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int,
                    (*seq_params).profile,
                ) as core::ffi::c_double;
                let target_bits_per_frame: core::ffi::c_int = (level_bitrate_limit
                    / (*cpi).framerate) as core::ffi::c_int;
                if frame_type == 0 as core::ffi::c_int {
                    let level_enforced_max_kf_bits: core::ffi::c_int = target_bits_per_frame
                        * 8 as core::ffi::c_int;
                    if bits_assigned > level_enforced_max_kf_bits {
                        let frames: core::ffi::c_int = (*rc).frames_to_key
                            - 1 as core::ffi::c_int;
                        (*p_rc).kf_boost = calculate_boost_factor(
                            frames,
                            level_enforced_max_kf_bits,
                            group_bits,
                        );
                        bits_assigned = calculate_boost_bits(
                            frames,
                            (*p_rc).kf_boost,
                            group_bits,
                        );
                    }
                } else if frame_type == 1 as core::ffi::c_int {
                    let level_enforced_max_arf_bits: core::ffi::c_int = target_bits_per_frame
                        * 4 as core::ffi::c_int;
                    if bits_assigned > level_enforced_max_arf_bits {
                        (*p_rc).gfu_boost = calculate_boost_factor(
                            (*p_rc).baseline_gf_interval,
                            level_enforced_max_arf_bits,
                            group_bits,
                        );
                        bits_assigned = calculate_boost_bits(
                            (*p_rc).baseline_gf_interval,
                            (*p_rc).gfu_boost,
                            group_bits,
                        );
                    }
                }
            }
        }
        index += 1;
    }
    return bits_assigned;
}
unsafe extern "C-unwind" fn allocate_gf_group_bits(
    mut gf_group: *mut GF_GROUP,
    p_rc: *mut PRIMARY_RATE_CONTROL,
    rc: *mut RATE_CONTROL,
    mut gf_group_bits: int64_t,
    mut gf_arf_bits: core::ffi::c_int,
    mut key_frame: core::ffi::c_int,
    mut use_arf: core::ffi::c_int,
) {
    static mut layer_fraction: [core::ffi::c_double; 7] = [
        1.0f64,
        0.70f64,
        0.55f64,
        0.60f64,
        0.60f64,
        1.0f64,
        1.0f64,
    ];
    let mut total_group_bits: int64_t = gf_group_bits;
    let mut base_frame_bits: core::ffi::c_int = 0;
    let gf_group_size: core::ffi::c_int = (*gf_group).size;
    let mut layer_frames: [core::ffi::c_int; 7] = [0 as core::ffi::c_int; 7];
    let mut frame_index: core::ffi::c_int = (key_frame != 0) as core::ffi::c_int;
    if use_arf != 0 {
        total_group_bits -= gf_arf_bits as int64_t;
    }
    let mut num_frames: core::ffi::c_int = if 1 as core::ffi::c_int
        > (*p_rc).baseline_gf_interval
            - ((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        (*p_rc).baseline_gf_interval
            - ((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int
    };
    base_frame_bits = (total_group_bits / num_frames as int64_t) as core::ffi::c_int;
    let mut max_arf_layer: core::ffi::c_int = (*gf_group).max_layer_depth
        - 1 as core::ffi::c_int;
    let mut idx: core::ffi::c_int = frame_index;
    while idx < gf_group_size {
        if (*gf_group).update_type[idx as usize] as core::ffi::c_int
            == ARF_UPDATE as core::ffi::c_int
            || (*gf_group).update_type[idx as usize] as core::ffi::c_int
                == INTNL_ARF_UPDATE as core::ffi::c_int
        {
            layer_frames[(*gf_group).layer_depth[idx as usize] as usize] += 1;
        }
        idx += 1;
    }
    let mut i: core::ffi::c_int = 0;
    let mut layer_extra_bits: [core::ffi::c_int; 7] = [0 as core::ffi::c_int; 7];
    i = 1 as core::ffi::c_int;
    while i <= max_arf_layer {
        let mut fraction: core::ffi::c_double = if i == max_arf_layer {
            1.0f64
        } else {
            layer_fraction[i as usize]
        };
        layer_extra_bits[i as usize] = (gf_arf_bits as core::ffi::c_double * fraction
            / (if 1 as core::ffi::c_int > layer_frames[i as usize] {
                1 as core::ffi::c_int
            } else {
                layer_frames[i as usize]
            }) as core::ffi::c_double) as core::ffi::c_int;
        gf_arf_bits
            -= (gf_arf_bits as core::ffi::c_double * fraction) as core::ffi::c_int;
        i += 1;
    }
    let mut arf_extra_bits: core::ffi::c_int = 0;
    let mut idx_0: core::ffi::c_int = frame_index;
    while idx_0 < gf_group_size {
        match (*gf_group).update_type[idx_0 as usize] as core::ffi::c_int {
            3 | 6 => {
                arf_extra_bits = layer_extra_bits[(*gf_group).layer_depth[idx_0 as usize]
                    as usize];
                (*gf_group).bit_allocation[idx_0 as usize] = if base_frame_bits
                    > INT_MAX - arf_extra_bits
                {
                    INT_MAX
                } else {
                    base_frame_bits + arf_extra_bits
                };
            }
            5 | 4 => {
                (*gf_group).bit_allocation[idx_0 as usize] = 0 as core::ffi::c_int;
            }
            _ => {
                (*gf_group).bit_allocation[idx_0 as usize] = base_frame_bits;
            }
        }
        idx_0 += 1;
    }
    if gf_group_size < MAX_STATIC_GF_GROUP_LENGTH {
        (*gf_group).bit_allocation[gf_group_size as usize] = 0 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn is_almost_static(
    mut gf_zero_motion: core::ffi::c_double,
    mut kf_zero_motion: core::ffi::c_int,
    mut is_lap_enabled: core::ffi::c_int,
) -> core::ffi::c_int {
    if is_lap_enabled != 0 {
        return (gf_zero_motion >= 0.999f64) as core::ffi::c_int
    } else {
        return (gf_zero_motion >= 0.995f64 && kf_zero_motion >= STATIC_KF_GROUP_THRESH)
            as core::ffi::c_int
    };
}
pub const ARF_ABS_ZOOM_THRESH: core::ffi::c_double = 4.4f64;
#[inline]
unsafe extern "C-unwind" fn detect_gf_cut(
    mut cpi: *mut AV1_COMP,
    mut frame_index: core::ffi::c_int,
    mut cur_start: core::ffi::c_int,
    mut flash_detected: core::ffi::c_int,
    mut active_max_gf_interval: core::ffi::c_int,
    mut active_min_gf_interval: core::ffi::c_int,
    mut gf_stats: *mut GF_GROUP_STATS,
) -> core::ffi::c_int {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mv_ratio_accumulator_thresh: core::ffi::c_double = ((*cm).height + (*cm).width)
        as core::ffi::c_double / 4.0f64;
    if flash_detected == 0 {
        let mut index: core::ffi::c_int = ((*cpi).twopass_frame.stats_in)
            .offset_from((*(*twopass).stats_buf_ctx).stats_in_start) as core::ffi::c_long
            as core::ffi::c_int;
        if detect_transition_to_still(
            &raw mut (*twopass).firstpass_info,
            index,
            (*rc).min_gf_interval,
            frame_index - cur_start,
            5 as core::ffi::c_int,
            (*gf_stats).loop_decay_rate,
            (*gf_stats).last_loop_decay_rate,
        ) != 0
        {
            return 1 as core::ffi::c_int;
        }
    }
    if frame_index - cur_start >= active_min_gf_interval
        && (*rc).frames_to_key - frame_index >= (*rc).min_gf_interval
        && frame_index - cur_start & 0x1 as core::ffi::c_int != 0 && flash_detected == 0
        && ((*gf_stats).mv_ratio_accumulator > mv_ratio_accumulator_thresh
            || (*gf_stats).abs_mv_in_out_accumulator > ARF_ABS_ZOOM_THRESH)
    {
        return 1 as core::ffi::c_int;
    }
    if frame_index - cur_start >= active_max_gf_interval + 1 as core::ffi::c_int
        && ((*cpi).oxcf.mode as core::ffi::c_int != REALTIME as core::ffi::c_int
            || is_almost_static(
                (*gf_stats).zero_motion_accumulator,
                (*twopass).kf_zeromotion_pct,
                (*(*cpi).ppi).lap_enabled,
            ) == 0)
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_shorter_gf_interval_better(
    mut cpi: *mut AV1_COMP,
    mut frame_params: *const EncodeFrameParams,
) -> core::ffi::c_int {
    if av1_use_tpl_for_extrc(&raw mut (*cpi).ext_ratectrl) as core::ffi::c_int != 0
        || av1_encode_for_extrc(&raw mut (*cpi).ext_ratectrl) as core::ffi::c_int != 0
    {
        return 0 as core::ffi::c_int;
    }
    let rc: *const RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut gop_length_decision_method: core::ffi::c_int = (*cpi)
        .sf
        .tpl_sf
        .gop_length_decision_method;
    let mut shorten_gf_interval: core::ffi::c_int = 0;
    av1_tpl_preload_rc_estimate(
        cpi as *mut AV1_COMP,
        frame_params as *const EncodeFrameParams,
    );
    if gop_length_decision_method == 2 as core::ffi::c_int {
        shorten_gf_interval = (((*p_rc).gfu_boost as core::ffi::c_double)
            < ((*p_rc).num_stats_used_for_gfu_boost * GF_MIN_BOOST)
                as core::ffi::c_double * 1.4f64
            && av1_tpl_setup_stats(
                cpi as *mut AV1_COMP,
                3 as core::ffi::c_int,
                frame_params as *const EncodeFrameParams,
            ) == 0) as core::ffi::c_int;
    } else {
        let mut do_complete_tpl: core::ffi::c_int = 1 as core::ffi::c_int;
        let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
        let mut is_temporal_filter_enabled: core::ffi::c_int = ((*rc).frames_since_key
            > 0 as core::ffi::c_int && (*gf_group).arf_index > -(1 as core::ffi::c_int))
            as core::ffi::c_int;
        if gop_length_decision_method == 1 as core::ffi::c_int {
            let mut gop_length_eval: core::ffi::c_int = av1_tpl_setup_stats(
                cpi as *mut AV1_COMP,
                2 as core::ffi::c_int,
                frame_params as *const EncodeFrameParams,
            );
            if gop_length_eval != 2 as core::ffi::c_int {
                do_complete_tpl = 0 as core::ffi::c_int;
                shorten_gf_interval = (gop_length_eval == 0) as core::ffi::c_int;
            }
        }
        if do_complete_tpl != 0 {
            shorten_gf_interval = (av1_tpl_setup_stats(
                cpi as *mut AV1_COMP,
                1 as core::ffi::c_int,
                frame_params as *const EncodeFrameParams,
            ) == 0) as core::ffi::c_int;
            if is_temporal_filter_enabled != 0 && shorten_gf_interval == 0 {
                (*cpi).skip_tpl_setup_stats = 1 as core::ffi::c_int;
            }
        }
    }
    return shorten_gf_interval;
}
pub const SMOOTH_FILT_LEN: core::ffi::c_int = 7 as core::ffi::c_int;
pub const HALF_FILT_LEN: core::ffi::c_int = SMOOTH_FILT_LEN / 2 as core::ffi::c_int;
pub const WINDOW_SIZE: core::ffi::c_int = 7 as core::ffi::c_int;
pub const HALF_WIN: core::ffi::c_int = WINDOW_SIZE / 2 as core::ffi::c_int;
unsafe extern "C-unwind" fn smooth_filter_stats(
    mut stats: *const FIRSTPASS_STATS,
    mut start_idx: core::ffi::c_int,
    mut last_idx: core::ffi::c_int,
    mut filt_intra_err: *mut core::ffi::c_double,
    mut filt_coded_err: *mut core::ffi::c_double,
) {
    static mut smooth_filt: [core::ffi::c_double; 7] = [
        0.006f64,
        0.061f64,
        0.242f64,
        0.383f64,
        0.242f64,
        0.061f64,
        0.006f64,
    ];
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = start_idx;
    while i <= last_idx {
        let mut total_wt: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        j = -HALF_FILT_LEN;
        while j <= HALF_FILT_LEN {
            let mut idx: core::ffi::c_int = clamp(i + j, start_idx, last_idx);
            if !((*stats.offset(idx as isize)).is_flash != 0) {
                *filt_intra_err.offset(i as isize)
                    += smooth_filt[(j + HALF_FILT_LEN) as usize]
                        * (*stats.offset(idx as isize)).intra_error;
                total_wt += smooth_filt[(j + HALF_FILT_LEN) as usize];
            }
            j += 1;
        }
        if total_wt > 0.01f64 {
            *filt_intra_err.offset(i as isize) /= total_wt;
        } else {
            *filt_intra_err.offset(i as isize) = (*stats.offset(i as isize)).intra_error;
        }
        i += 1;
    }
    i = start_idx;
    while i <= last_idx {
        let mut total_wt_0: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        j = -HALF_FILT_LEN;
        while j <= HALF_FILT_LEN {
            let mut idx_0: core::ffi::c_int = clamp(i + j, start_idx, last_idx);
            if !((*stats.offset(idx_0 as isize)).is_flash != 0
                || idx_0 > 0 as core::ffi::c_int
                    && (*stats.offset((idx_0 - 1 as core::ffi::c_int) as isize)).is_flash
                        != 0)
            {
                *filt_coded_err.offset(i as isize)
                    += smooth_filt[(j + HALF_FILT_LEN) as usize]
                        * (*stats.offset(idx_0 as isize)).coded_error;
                total_wt_0 += smooth_filt[(j + HALF_FILT_LEN) as usize];
            }
            j += 1;
        }
        if total_wt_0 > 0.01f64 {
            *filt_coded_err.offset(i as isize) /= total_wt_0;
        } else {
            *filt_coded_err.offset(i as isize) = (*stats.offset(i as isize)).coded_error;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn get_gradient(
    mut values: *const core::ffi::c_double,
    mut start: core::ffi::c_int,
    mut last: core::ffi::c_int,
    mut grad: *mut core::ffi::c_double,
) {
    if start == last {
        *grad.offset(start as isize) = 0 as core::ffi::c_int as core::ffi::c_double;
        return;
    }
    let mut i: core::ffi::c_int = start;
    while i <= last {
        let mut prev: core::ffi::c_int = if i - 1 as core::ffi::c_int > start {
            i - 1 as core::ffi::c_int
        } else {
            start
        };
        let mut next: core::ffi::c_int = if (i + 1 as core::ffi::c_int) < last {
            i + 1 as core::ffi::c_int
        } else {
            last
        };
        *grad.offset(i as isize) = (*values.offset(next as isize)
            - *values.offset(prev as isize)) / (next - prev) as core::ffi::c_double;
        i += 1;
    }
}
unsafe extern "C-unwind" fn find_next_scenecut(
    stats_start: *const FIRSTPASS_STATS,
    mut first: core::ffi::c_int,
    mut last: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut this_ratio: core::ffi::c_double = 0.;
    let mut max_prev_ratio: core::ffi::c_double = 0.;
    let mut max_next_ratio: core::ffi::c_double = 0.;
    let mut max_prev_coded: core::ffi::c_double = 0.;
    let mut max_next_coded: core::ffi::c_double = 0.;
    if last - first == 0 as core::ffi::c_int {
        return -(1 as core::ffi::c_int);
    }
    let mut current_block_25: u64;
    let mut i: core::ffi::c_int = first;
    while i <= last {
        if !((*stats_start.offset(i as isize)).is_flash != 0
            || i > 0 as core::ffi::c_int
                && (*stats_start.offset((i - 1 as core::ffi::c_int) as isize)).is_flash
                    != 0)
        {
            let mut temp_intra: core::ffi::c_double = if (*stats_start
                .offset(i as isize))
                .intra_error > 0.01f64
            {
                (*stats_start.offset(i as isize)).intra_error
            } else {
                0.01f64
            };
            this_ratio = (*stats_start.offset(i as isize)).coded_error / temp_intra;
            max_prev_ratio = 0 as core::ffi::c_int as core::ffi::c_double;
            max_prev_coded = 0 as core::ffi::c_int as core::ffi::c_double;
            let mut j: core::ffi::c_int = if first
                > i - 7 as core::ffi::c_int / 2 as core::ffi::c_int
            {
                first
            } else {
                i - 7 as core::ffi::c_int / 2 as core::ffi::c_int
            };
            while j < i {
                if !((*stats_start.offset(j as isize)).is_flash != 0
                    || j > 0 as core::ffi::c_int
                        && (*stats_start.offset((j - 1 as core::ffi::c_int) as isize))
                            .is_flash != 0)
                {
                    temp_intra = if (*stats_start.offset(j as isize)).intra_error
                        > 0.01f64
                    {
                        (*stats_start.offset(j as isize)).intra_error
                    } else {
                        0.01f64
                    };
                    let mut temp_ratio: core::ffi::c_double = (*stats_start
                        .offset(j as isize))
                        .coded_error / temp_intra;
                    if temp_ratio > max_prev_ratio {
                        max_prev_ratio = temp_ratio;
                    }
                    if (*stats_start.offset(j as isize)).coded_error > max_prev_coded {
                        max_prev_coded = (*stats_start.offset(j as isize)).coded_error;
                    }
                }
                j += 1;
            }
            max_next_ratio = 0 as core::ffi::c_int as core::ffi::c_double;
            max_next_coded = 0 as core::ffi::c_int as core::ffi::c_double;
            let mut j_0: core::ffi::c_int = i + 1 as core::ffi::c_int;
            while j_0
                <= (if (i + 7 as core::ffi::c_int / 2 as core::ffi::c_int) < last {
                    i + 7 as core::ffi::c_int / 2 as core::ffi::c_int
                } else {
                    last
                })
            {
                if !((*stats_start.offset(i as isize)).is_flash != 0
                    || i > 0 as core::ffi::c_int
                        && (*stats_start.offset((i - 1 as core::ffi::c_int) as isize))
                            .is_flash != 0)
                {
                    temp_intra = if (*stats_start.offset(j_0 as isize)).intra_error
                        > 0.01f64
                    {
                        (*stats_start.offset(j_0 as isize)).intra_error
                    } else {
                        0.01f64
                    };
                    let mut temp_ratio_0: core::ffi::c_double = (*stats_start
                        .offset(j_0 as isize))
                        .coded_error / temp_intra;
                    if temp_ratio_0 > max_next_ratio {
                        max_next_ratio = temp_ratio_0;
                    }
                    if (*stats_start.offset(j_0 as isize)).coded_error > max_next_coded {
                        max_next_coded = (*stats_start.offset(j_0 as isize)).coded_error;
                    }
                }
                j_0 += 1;
            }
            if max_prev_ratio < 0.001f64 && max_next_ratio < 0.001f64 {
                if this_ratio < 0.02f64 {
                    current_block_25 = 0;
                } else {
                    current_block_25 = 1;
                }
            } else {
                let mut max_sr: core::ffi::c_double = (*stats_start.offset(i as isize))
                    .sr_coded_error;
                if i < last {
                    max_sr = if max_sr
                        > (*stats_start.offset((i + 1 as core::ffi::c_int) as isize))
                            .sr_coded_error
                    {
                        max_sr
                    } else {
                        (*stats_start.offset((i + 1 as core::ffi::c_int) as isize))
                            .sr_coded_error
                    };
                }
                let mut max_sr_fr_ratio: core::ffi::c_double = max_sr
                    / (if (*stats_start.offset(i as isize)).coded_error > 0.01f64 {
                        (*stats_start.offset(i as isize)).coded_error
                    } else {
                        0.01f64
                    });
                if max_sr_fr_ratio > 1.2f64 {
                    current_block_25 = 0;
                } else if this_ratio
                    < 2 as core::ffi::c_int as core::ffi::c_double
                        * (if max_prev_ratio > max_next_ratio {
                            max_prev_ratio
                        } else {
                            max_next_ratio
                        })
                    && (*stats_start.offset(i as isize)).coded_error
                        < 2 as core::ffi::c_int as core::ffi::c_double
                            * (if max_prev_coded > max_next_coded {
                                max_prev_coded
                            } else {
                                max_next_coded
                            })
                {
                    current_block_25 = 0;
                } else {
                    current_block_25 = 1;
                }
            }
            match current_block_25 {
                0 => {}
                _ => return i,
            }
        }
        i += 1;
    }
    return -(1 as core::ffi::c_int);
}
unsafe extern "C-unwind" fn remove_region(
    mut merge: core::ffi::c_int,
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
    mut next_region: *mut core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = *next_region;
    if *num_regions == 1 as core::ffi::c_int {
        *num_regions = 0 as core::ffi::c_int;
        return;
    }
    if k == 0 as core::ffi::c_int {
        merge = 1 as core::ffi::c_int;
    } else if k == *num_regions - 1 as core::ffi::c_int {
        merge = 0 as core::ffi::c_int;
    }
    let mut num_merge: core::ffi::c_int = if merge == 2 as core::ffi::c_int {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    match merge {
        0 => {
            (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last = (*regions
                .offset(k as isize))
                .last;
            *next_region = k;
        }
        1 => {
            (*regions.offset((k + 1 as core::ffi::c_int) as isize)).start = (*regions
                .offset(k as isize))
                .start;
            *next_region = k + 1 as core::ffi::c_int;
        }
        2 => {
            (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last = (*regions
                .offset((k + 1 as core::ffi::c_int) as isize))
                .last;
            *next_region = k;
        }
        _ => {}
    }
    *num_regions -= num_merge;
    k = *next_region - (merge == 1 as core::ffi::c_int) as core::ffi::c_int;
    while k < *num_regions {
        *regions.offset(k as isize) = *regions.offset((k + num_merge) as isize);
        k += 1;
    }
}
unsafe extern "C-unwind" fn insert_region(
    mut start: core::ffi::c_int,
    mut last: core::ffi::c_int,
    mut type_0: REGION_TYPES,
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
    mut cur_region_idx: *mut core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = *cur_region_idx;
    let mut this_region_type: REGION_TYPES = (*regions.offset(k as isize)).type_0;
    let mut this_region_last: core::ffi::c_int = (*regions.offset(k as isize)).last;
    let mut num_add: core::ffi::c_int = (start != (*regions.offset(k as isize)).start)
        as core::ffi::c_int
        + (last != (*regions.offset(k as isize)).last) as core::ffi::c_int;
    let mut r: core::ffi::c_int = *num_regions - 1 as core::ffi::c_int;
    while r > k {
        *regions.offset((r + num_add) as isize) = *regions.offset(r as isize);
        r -= 1;
    }
    *num_regions += num_add;
    if start > (*regions.offset(k as isize)).start {
        (*regions.offset(k as isize)).last = start - 1 as core::ffi::c_int;
        k += 1;
        (*regions.offset(k as isize)).start = start;
    }
    (*regions.offset(k as isize)).type_0 = type_0;
    if last < this_region_last {
        (*regions.offset(k as isize)).last = last;
        k += 1;
        (*regions.offset(k as isize)).start = last + 1 as core::ffi::c_int;
        (*regions.offset(k as isize)).last = this_region_last;
        (*regions.offset(k as isize)).type_0 = this_region_type;
    } else {
        (*regions.offset(k as isize)).last = this_region_last;
    }
    *cur_region_idx = k;
}
unsafe extern "C-unwind" fn analyze_region(
    mut stats: *const FIRSTPASS_STATS,
    mut k: core::ffi::c_int,
    mut regions: *mut REGIONS,
) {
    let mut i: core::ffi::c_int = 0;
    (*regions.offset(k as isize)).avg_cor_coeff = 0 as core::ffi::c_int
        as core::ffi::c_double;
    (*regions.offset(k as isize)).avg_sr_fr_ratio = 0 as core::ffi::c_int
        as core::ffi::c_double;
    (*regions.offset(k as isize)).avg_intra_err = 0 as core::ffi::c_int
        as core::ffi::c_double;
    (*regions.offset(k as isize)).avg_coded_err = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut check_first_sr: core::ffi::c_int = (k != 0 as core::ffi::c_int)
        as core::ffi::c_int;
    i = (*regions.offset(k as isize)).start;
    while i <= (*regions.offset(k as isize)).last {
        if i > (*regions.offset(k as isize)).start || check_first_sr != 0 {
            let mut num_frames: core::ffi::c_double = ((*regions.offset(k as isize)).last
                - (*regions.offset(k as isize)).start + check_first_sr)
                as core::ffi::c_double;
            let mut max_coded_error: core::ffi::c_double = if (*stats.offset(i as isize))
                .coded_error
                > (*stats.offset((i - 1 as core::ffi::c_int) as isize)).coded_error
            {
                (*stats.offset(i as isize)).coded_error
            } else {
                (*stats.offset((i - 1 as core::ffi::c_int) as isize)).coded_error
            };
            let mut this_ratio: core::ffi::c_double = (*stats.offset(i as isize))
                .sr_coded_error
                / (if max_coded_error > 0.001f64 { max_coded_error } else { 0.001f64 });
            (*regions.offset(k as isize)).avg_sr_fr_ratio += this_ratio / num_frames;
        }
        (*regions.offset(k as isize)).avg_intra_err
            += (*stats.offset(i as isize)).intra_error
                / ((*regions.offset(k as isize)).last
                    - (*regions.offset(k as isize)).start + 1 as core::ffi::c_int)
                    as core::ffi::c_double;
        (*regions.offset(k as isize)).avg_coded_err
            += (*stats.offset(i as isize)).coded_error
                / ((*regions.offset(k as isize)).last
                    - (*regions.offset(k as isize)).start + 1 as core::ffi::c_int)
                    as core::ffi::c_double;
        (*regions.offset(k as isize)).avg_cor_coeff
            += (if (*stats.offset(i as isize)).cor_coeff > 0.001f64 {
                (*stats.offset(i as isize)).cor_coeff
            } else {
                0.001f64
            })
                / ((*regions.offset(k as isize)).last
                    - (*regions.offset(k as isize)).start + 1 as core::ffi::c_int)
                    as core::ffi::c_double;
        (*regions.offset(k as isize)).avg_noise_var
            += (if (*stats.offset(i as isize)).noise_var > 0.001f64 {
                (*stats.offset(i as isize)).noise_var
            } else {
                0.001f64
            })
                / ((*regions.offset(k as isize)).last
                    - (*regions.offset(k as isize)).start + 1 as core::ffi::c_int)
                    as core::ffi::c_double;
        i += 1;
    }
}
unsafe extern "C-unwind" fn get_region_stats(
    mut stats: *const FIRSTPASS_STATS,
    mut regions: *mut REGIONS,
    mut num_regions: core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < num_regions {
        analyze_region(stats, k, regions);
        k += 1;
    }
}
unsafe extern "C-unwind" fn find_stable_regions(
    mut stats: *const FIRSTPASS_STATS,
    mut grad_coded: *const core::ffi::c_double,
    mut this_start: core::ffi::c_int,
    mut this_last: core::ffi::c_int,
    mut regions: *mut REGIONS,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    (*regions.offset(k as isize)).start = this_start;
    i = this_start;
    while i <= this_last {
        let mut mean_intra: core::ffi::c_double = 0.001f64;
        let mut var_intra: core::ffi::c_double = 0.001f64;
        let mut mean_coded: core::ffi::c_double = 0.001f64;
        let mut var_coded: core::ffi::c_double = 0.001f64;
        let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
        j = -HALF_WIN;
        while j <= HALF_WIN {
            let mut idx: core::ffi::c_int = clamp(i + j, this_start, this_last);
            if !((*stats.offset(idx as isize)).is_flash != 0
                || idx > 0 as core::ffi::c_int
                    && (*stats.offset((idx - 1 as core::ffi::c_int) as isize)).is_flash
                        != 0)
            {
                mean_intra += (*stats.offset(idx as isize)).intra_error;
                var_intra
                    += (*stats.offset(idx as isize)).intra_error
                        * (*stats.offset(idx as isize)).intra_error;
                mean_coded += (*stats.offset(idx as isize)).coded_error;
                var_coded
                    += (*stats.offset(idx as isize)).coded_error
                        * (*stats.offset(idx as isize)).coded_error;
                count += 1;
            }
            j += 1;
        }
        let mut cur_type: REGION_TYPES = STABLE_REGION;
        if count > 0 as core::ffi::c_int {
            mean_intra /= count as core::ffi::c_double;
            var_intra /= count as core::ffi::c_double;
            mean_coded /= count as core::ffi::c_double;
            var_coded /= count as core::ffi::c_double;
            let mut is_intra_stable: core::ffi::c_int = (var_intra
                / (mean_intra * mean_intra) < 1.03f64) as core::ffi::c_int;
            let mut is_coded_stable: core::ffi::c_int = (var_coded
                / (mean_coded * mean_coded) < 1.04f64
                && fabs(*grad_coded.offset(i as isize)) / mean_coded < 0.05f64
                || mean_coded / mean_intra < 0.05f64) as core::ffi::c_int;
            let mut is_coded_small: core::ffi::c_int = (mean_coded < 0.5f64 * mean_intra)
                as core::ffi::c_int;
            cur_type = (if is_intra_stable != 0 && is_coded_stable != 0
                && is_coded_small != 0
            {
                STABLE_REGION as core::ffi::c_int
            } else {
                HIGH_VAR_REGION as core::ffi::c_int
            }) as REGION_TYPES;
        } else {
            cur_type = HIGH_VAR_REGION;
        }
        if i == (*regions.offset(k as isize)).start {
            (*regions.offset(k as isize)).type_0 = cur_type;
        } else if cur_type as core::ffi::c_uint
            != (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
        {
            (*regions.offset(k as isize)).last = i - 1 as core::ffi::c_int;
            (*regions.offset((k + 1 as core::ffi::c_int) as isize)).start = i;
            (*regions.offset((k + 1 as core::ffi::c_int) as isize)).type_0 = cur_type;
            k += 1;
        }
        i += 1;
    }
    (*regions.offset(k as isize)).last = this_last;
    return k + 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn cleanup_regions(
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < *num_regions {
        if k > 0 as core::ffi::c_int
            && (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0
                as core::ffi::c_uint
                == (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            && (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
                != SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
            || (*regions.offset(k as isize)).last < (*regions.offset(k as isize)).start
        {
            remove_region(0 as core::ffi::c_int, regions, num_regions, &raw mut k);
        } else {
            k += 1;
        }
    }
}
unsafe extern "C-unwind" fn remove_short_regions(
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
    mut type_0: REGION_TYPES,
    mut length: core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < *num_regions && *num_regions > 1 as core::ffi::c_int {
        if ((*regions.offset(k as isize)).last - (*regions.offset(k as isize)).start
            + 1 as core::ffi::c_int) < length
            && (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
                == type_0 as core::ffi::c_uint
        {
            remove_region(2 as core::ffi::c_int, regions, num_regions, &raw mut k);
        } else {
            k += 1;
        }
    }
    cleanup_regions(regions, num_regions);
}
unsafe extern "C-unwind" fn adjust_unstable_region_bounds(
    mut stats: *const FIRSTPASS_STATS,
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    remove_short_regions(regions, num_regions, STABLE_REGION, HALF_WIN);
    remove_short_regions(regions, num_regions, HIGH_VAR_REGION, HALF_WIN);
    get_region_stats(stats, regions, *num_regions);
    k = 0 as core::ffi::c_int;
    while k < *num_regions {
        if !((*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            == STABLE_REGION as core::ffi::c_int as core::ffi::c_uint)
        {
            if k > 0 as core::ffi::c_int {
                let mut avg_intra_err: core::ffi::c_double = 0 as core::ffi::c_int
                    as core::ffi::c_double;
                let starti: core::ffi::c_int = if (*regions
                    .offset((k - 1 as core::ffi::c_int) as isize))
                    .last - 7 as core::ffi::c_int + 1 as core::ffi::c_int
                    > (*regions.offset((k - 1 as core::ffi::c_int) as isize)).start
                        + 1 as core::ffi::c_int
                {
                    (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                        - 7 as core::ffi::c_int + 1 as core::ffi::c_int
                } else {
                    (*regions.offset((k - 1 as core::ffi::c_int) as isize)).start
                        + 1 as core::ffi::c_int
                };
                let lasti: core::ffi::c_int = (*regions
                    .offset((k - 1 as core::ffi::c_int) as isize))
                    .last;
                let mut counti: core::ffi::c_int = 0 as core::ffi::c_int;
                i = starti;
                while i <= lasti {
                    avg_intra_err += (*stats.offset(i as isize)).intra_error;
                    counti += 1;
                    i += 1;
                }
                if counti > 0 as core::ffi::c_int {
                    avg_intra_err = if avg_intra_err / counti as core::ffi::c_double
                        > 0.001f64
                    {
                        avg_intra_err / counti as core::ffi::c_double
                    } else {
                        0.001f64
                    };
                    let mut count_coded: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut count_grad: core::ffi::c_int = 0 as core::ffi::c_int;
                    j = lasti + 1 as core::ffi::c_int;
                    while j <= (*regions.offset(k as isize)).last {
                        let intra_close: core::ffi::c_int = (fabs(
                            (*stats.offset(j as isize)).intra_error - avg_intra_err,
                        ) / avg_intra_err < 0.1f64) as core::ffi::c_int;
                        let coded_small: core::ffi::c_int = ((*stats.offset(j as isize))
                            .coded_error / avg_intra_err < 0.1f64) as core::ffi::c_int;
                        let coeff_close: core::ffi::c_int = ((*stats.offset(j as isize))
                            .cor_coeff > 0.995f64) as core::ffi::c_int;
                        if coeff_close == 0 || coded_small == 0 {
                            count_coded -= 1;
                        }
                        if !(intra_close != 0 && count_coded >= 0 as core::ffi::c_int
                            && count_grad >= 0 as core::ffi::c_int)
                        {
                            break;
                        }
                        (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last = j;
                        (*regions.offset(k as isize)).start = j + 1 as core::ffi::c_int;
                        j += 1;
                    }
                }
            }
            if k < *num_regions - 1 as core::ffi::c_int {
                let mut avg_intra_err_0: core::ffi::c_double = 0 as core::ffi::c_int
                    as core::ffi::c_double;
                let starti_0: core::ffi::c_int = (*regions
                    .offset((k + 1 as core::ffi::c_int) as isize))
                    .start;
                let lasti_0: core::ffi::c_int = if ((*regions
                    .offset((k + 1 as core::ffi::c_int) as isize))
                    .last - 1 as core::ffi::c_int)
                    < (*regions.offset((k + 1 as core::ffi::c_int) as isize)).start
                        + 7 as core::ffi::c_int - 1 as core::ffi::c_int
                {
                    (*regions.offset((k + 1 as core::ffi::c_int) as isize)).last
                        - 1 as core::ffi::c_int
                } else {
                    (*regions.offset((k + 1 as core::ffi::c_int) as isize)).start
                        + 7 as core::ffi::c_int - 1 as core::ffi::c_int
                };
                let mut counti_0: core::ffi::c_int = 0 as core::ffi::c_int;
                i = starti_0;
                while i <= lasti_0 {
                    avg_intra_err_0 += (*stats.offset(i as isize)).intra_error;
                    counti_0 += 1;
                    i += 1;
                }
                if counti_0 > 0 as core::ffi::c_int {
                    avg_intra_err_0 = if avg_intra_err_0
                        / counti_0 as core::ffi::c_double > 0.001f64
                    {
                        avg_intra_err_0 / counti_0 as core::ffi::c_double
                    } else {
                        0.001f64
                    };
                    let mut count_coded_0: core::ffi::c_int = 1 as core::ffi::c_int;
                    let mut count_grad_0: core::ffi::c_int = 1 as core::ffi::c_int;
                    j = starti_0 - 1 as core::ffi::c_int;
                    while j >= (*regions.offset(k as isize)).start {
                        let intra_close_0: core::ffi::c_int = (fabs(
                            (*stats.offset(j as isize)).intra_error - avg_intra_err_0,
                        ) / avg_intra_err_0 < 0.1f64) as core::ffi::c_int;
                        let coded_small_0: core::ffi::c_int = ((*stats
                            .offset((j + 1 as core::ffi::c_int) as isize))
                            .coded_error / avg_intra_err_0 < 0.1f64) as core::ffi::c_int;
                        let coeff_close_0: core::ffi::c_int = ((*stats
                            .offset(j as isize))
                            .cor_coeff > 0.995f64) as core::ffi::c_int;
                        if coeff_close_0 == 0 || coded_small_0 == 0 {
                            count_coded_0 -= 1;
                        }
                        if !(intra_close_0 != 0 && count_coded_0 >= 0 as core::ffi::c_int
                            && count_grad_0 >= 0 as core::ffi::c_int)
                        {
                            break;
                        }
                        (*regions.offset((k + 1 as core::ffi::c_int) as isize)).start = j;
                        (*regions.offset(k as isize)).last = j - 1 as core::ffi::c_int;
                        j -= 1;
                    }
                }
            }
        }
        k += 1;
    }
    cleanup_regions(regions, num_regions);
    remove_short_regions(regions, num_regions, HIGH_VAR_REGION, HALF_WIN);
    get_region_stats(stats, regions, *num_regions);
    k = 0 as core::ffi::c_int;
    while k < *num_regions && *num_regions > 1 as core::ffi::c_int {
        if (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            == STABLE_REGION as core::ffi::c_int as core::ffi::c_uint
            && ((*regions.offset(k as isize)).last - (*regions.offset(k as isize)).start
                + 1 as core::ffi::c_int) < 2 as core::ffi::c_int * WINDOW_SIZE
            && (k > 0 as core::ffi::c_int
                && ((*regions.offset(k as isize)).avg_coded_err
                    > (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                        .avg_coded_err * 1.01f64
                    || (*regions.offset(k as isize)).avg_cor_coeff
                        < (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                            .avg_cor_coeff * 0.999f64)
                && (k < *num_regions - 1 as core::ffi::c_int
                    && ((*regions.offset(k as isize)).avg_coded_err
                        > (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                            .avg_coded_err * 1.01f64
                        || (*regions.offset(k as isize)).avg_cor_coeff
                            < (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                                .avg_cor_coeff * 0.999f64)))
        {
            remove_region(2 as core::ffi::c_int, regions, num_regions, &raw mut k);
            analyze_region(stats, k - 1 as core::ffi::c_int, regions);
        } else if (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            == HIGH_VAR_REGION as core::ffi::c_int as core::ffi::c_uint
            && ((*regions.offset(k as isize)).last - (*regions.offset(k as isize)).start
                + 1 as core::ffi::c_int) < 2 as core::ffi::c_int * WINDOW_SIZE
            && (k > 0 as core::ffi::c_int
                && ((*regions.offset(k as isize)).avg_coded_err
                    < (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                        .avg_coded_err * 0.99f64
                    || (*regions.offset(k as isize)).avg_cor_coeff
                        > (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                            .avg_cor_coeff * 1.001f64)
                && (k < *num_regions - 1 as core::ffi::c_int
                    && ((*regions.offset(k as isize)).avg_coded_err
                        < (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                            .avg_coded_err * 0.99f64
                        || (*regions.offset(k as isize)).avg_cor_coeff
                            > (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                                .avg_cor_coeff * 1.001f64)))
        {
            remove_region(2 as core::ffi::c_int, regions, num_regions, &raw mut k);
            analyze_region(stats, k - 1 as core::ffi::c_int, regions);
        } else {
            k += 1;
        }
    }
    remove_short_regions(regions, num_regions, STABLE_REGION, WINDOW_SIZE);
    remove_short_regions(regions, num_regions, HIGH_VAR_REGION, HALF_WIN);
}
unsafe extern "C-unwind" fn find_blending_regions(
    mut stats: *const FIRSTPASS_STATS,
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut count_stable: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < *num_regions {
        if (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            == STABLE_REGION as core::ffi::c_int as core::ffi::c_uint
        {
            k += 1;
            count_stable += 1;
        } else {
            let mut dir: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut start: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut last: core::ffi::c_int = 0;
            i = (*regions.offset(k as isize)).start;
            while i <= (*regions.offset(k as isize)).last {
                if !(k == 0 as core::ffi::c_int
                    && i == (*regions.offset(k as isize)).start)
                {
                    if !((*stats.offset(i as isize)).is_flash != 0
                        || i > 0 as core::ffi::c_int
                            && (*stats.offset((i - 1 as core::ffi::c_int) as isize))
                                .is_flash != 0)
                    {
                        let mut grad: core::ffi::c_double = (*stats.offset(i as isize))
                            .intra_error
                            - (*stats.offset((i - 1 as core::ffi::c_int) as isize))
                                .intra_error;
                        let mut large_change: core::ffi::c_int = (fabs(grad)
                            / (if (*stats.offset(i as isize)).intra_error > 0.01f64 {
                                (*stats.offset(i as isize)).intra_error
                            } else {
                                0.01f64
                            }) > 0.05f64) as core::ffi::c_int;
                        let mut this_dir: core::ffi::c_int = 0 as core::ffi::c_int;
                        if large_change != 0 {
                            this_dir = if grad
                                > 0 as core::ffi::c_int as core::ffi::c_double
                            {
                                1 as core::ffi::c_int
                            } else {
                                -(1 as core::ffi::c_int)
                            };
                        }
                        if !(dir == this_dir) {
                            if dir != 0 as core::ffi::c_int {
                                last = i - 1 as core::ffi::c_int;
                                insert_region(
                                    start,
                                    last,
                                    BLENDING_REGION,
                                    regions,
                                    num_regions,
                                    &raw mut k,
                                );
                            }
                            dir = this_dir;
                            if k == 0 as core::ffi::c_int
                                && i
                                    == (*regions.offset(k as isize)).start
                                        + 1 as core::ffi::c_int
                            {
                                start = i - 1 as core::ffi::c_int;
                            } else {
                                start = i;
                            }
                        }
                    }
                }
                i += 1;
            }
            if dir != 0 as core::ffi::c_int {
                last = (*regions.offset(k as isize)).last;
                insert_region(
                    start,
                    last,
                    BLENDING_REGION,
                    regions,
                    num_regions,
                    &raw mut k,
                );
            }
            k += 1;
        }
    }
    get_region_stats(stats, regions, *num_regions);
    k = 0 as core::ffi::c_int;
    while k < *num_regions {
        if !((*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            != BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint)
        {
            if (*regions.offset(k as isize)).last == (*regions.offset(k as isize)).start
                || (*regions.offset(k as isize)).avg_cor_coeff < 0.6f64
                || count_stable == 0 as core::ffi::c_int
            {
                (*regions.offset(k as isize)).type_0 = HIGH_VAR_REGION;
            }
        }
        k += 1;
    }
    get_region_stats(stats, regions, *num_regions);
    k = 1 as core::ffi::c_int;
    while k < *num_regions {
        if k < *num_regions - 1 as core::ffi::c_int
            && (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
                == HIGH_VAR_REGION as core::ffi::c_int as core::ffi::c_uint
        {
            if (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0
                as core::ffi::c_uint
                == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
                && (*regions.offset((k + 1 as core::ffi::c_int) as isize)).type_0
                    as core::ffi::c_uint
                    == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
                && (*regions.offset(k as isize)).last
                    - (*regions.offset(k as isize)).start < 3 as core::ffi::c_int
            {
                let mut prev_dir: core::ffi::c_int = if (*stats
                    .offset(
                        (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                            as isize,
                    ))
                    .intra_error
                    - (*stats
                        .offset(
                            ((*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .intra_error > 0 as core::ffi::c_int as core::ffi::c_double
                {
                    1 as core::ffi::c_int
                } else {
                    -(1 as core::ffi::c_int)
                };
                let mut next_dir: core::ffi::c_int = if (*stats
                    .offset(
                        (*regions.offset((k + 1 as core::ffi::c_int) as isize)).last
                            as isize,
                    ))
                    .intra_error
                    - (*stats
                        .offset(
                            ((*regions.offset((k + 1 as core::ffi::c_int) as isize)).last
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .intra_error > 0 as core::ffi::c_int as core::ffi::c_double
                {
                    1 as core::ffi::c_int
                } else {
                    -(1 as core::ffi::c_int)
                };
                if prev_dir < 0 as core::ffi::c_int && next_dir > 0 as core::ffi::c_int {
                    let mut ratio_thres: core::ffi::c_double = (if (*regions
                        .offset((k - 1 as core::ffi::c_int) as isize))
                        .avg_sr_fr_ratio
                        < (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                            .avg_sr_fr_ratio
                    {
                        (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                            .avg_sr_fr_ratio
                    } else {
                        (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                            .avg_sr_fr_ratio
                    }) * 0.95f64;
                    if (*regions.offset(k as isize)).avg_sr_fr_ratio > ratio_thres {
                        (*regions.offset(k as isize)).type_0 = BLENDING_REGION;
                        remove_region(
                            2 as core::ffi::c_int,
                            regions,
                            num_regions,
                            &raw mut k,
                        );
                        analyze_region(stats, k - 1 as core::ffi::c_int, regions);
                        continue;
                    }
                }
            }
        }
        if (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0
            as core::ffi::c_uint
            == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
            && (*regions.offset(k as isize)).type_0 as core::ffi::c_uint
                == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
        {
            let mut prev_dir_0: core::ffi::c_int = if (*stats
                .offset(
                    (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last as isize,
                ))
                .intra_error
                - (*stats
                    .offset(
                        ((*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                            - 1 as core::ffi::c_int) as isize,
                    ))
                    .intra_error > 0 as core::ffi::c_int as core::ffi::c_double
            {
                1 as core::ffi::c_int
            } else {
                -(1 as core::ffi::c_int)
            };
            let mut next_dir_0: core::ffi::c_int = if (*stats
                .offset((*regions.offset(k as isize)).last as isize))
                .intra_error
                - (*stats
                    .offset(
                        ((*regions.offset(k as isize)).last - 1 as core::ffi::c_int)
                            as isize,
                    ))
                    .intra_error > 0 as core::ffi::c_int as core::ffi::c_double
            {
                1 as core::ffi::c_int
            } else {
                -(1 as core::ffi::c_int)
            };
            let mut total_length: core::ffi::c_int = (*regions.offset(k as isize)).last
                - (*regions.offset((k - 1 as core::ffi::c_int) as isize)).start
                + 1 as core::ffi::c_int;
            if total_length < 4 as core::ffi::c_int {
                (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0 = HIGH_VAR_REGION;
                k += 1;
                continue;
            } else {
                let mut to_merge: core::ffi::c_int = 0 as core::ffi::c_int;
                if prev_dir_0 < 0 as core::ffi::c_int
                    && next_dir_0 > 0 as core::ffi::c_int
                {
                    let mut prev_length: core::ffi::c_double = ((*regions
                        .offset((k - 1 as core::ffi::c_int) as isize))
                        .last
                        - (*regions.offset((k - 1 as core::ffi::c_int) as isize)).start
                        + 1 as core::ffi::c_int) as core::ffi::c_double;
                    let mut last_ratio: core::ffi::c_double = 0.;
                    let mut ratio_thres_0: core::ffi::c_double = 0.;
                    if prev_length < 2.01f64 {
                        let mut max_coded_error: core::ffi::c_double = if (*stats
                            .offset(
                                (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                    as isize,
                            ))
                            .coded_error
                            > (*stats
                                .offset(
                                    ((*regions.offset((k - 1 as core::ffi::c_int) as isize))
                                        .last - 1 as core::ffi::c_int) as isize,
                                ))
                                .coded_error
                        {
                            (*stats
                                .offset(
                                    (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                        as isize,
                                ))
                                .coded_error
                        } else {
                            (*stats
                                .offset(
                                    ((*regions.offset((k - 1 as core::ffi::c_int) as isize))
                                        .last - 1 as core::ffi::c_int) as isize,
                                ))
                                .coded_error
                        };
                        last_ratio = (*stats
                            .offset(
                                (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                    as isize,
                            ))
                            .sr_coded_error
                            / (if max_coded_error > 0.001f64 {
                                max_coded_error
                            } else {
                                0.001f64
                            });
                        ratio_thres_0 = (*regions.offset(k as isize)).avg_sr_fr_ratio
                            * 0.95f64;
                    } else {
                        let mut max_coded_error_0: core::ffi::c_double = if (*stats
                            .offset(
                                (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                    as isize,
                            ))
                            .coded_error
                            > (*stats
                                .offset(
                                    ((*regions.offset((k - 1 as core::ffi::c_int) as isize))
                                        .last - 1 as core::ffi::c_int) as isize,
                                ))
                                .coded_error
                        {
                            (*stats
                                .offset(
                                    (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                        as isize,
                                ))
                                .coded_error
                        } else {
                            (*stats
                                .offset(
                                    ((*regions.offset((k - 1 as core::ffi::c_int) as isize))
                                        .last - 1 as core::ffi::c_int) as isize,
                                ))
                                .coded_error
                        };
                        last_ratio = (*stats
                            .offset(
                                (*regions.offset((k - 1 as core::ffi::c_int) as isize)).last
                                    as isize,
                            ))
                            .sr_coded_error
                            / (if max_coded_error_0 > 0.001f64 {
                                max_coded_error_0
                            } else {
                                0.001f64
                            });
                        let mut prev_ratio: core::ffi::c_double = ((*regions
                            .offset((k - 1 as core::ffi::c_int) as isize))
                            .avg_sr_fr_ratio * prev_length - last_ratio)
                            / (prev_length - 1.0f64);
                        ratio_thres_0 = (if prev_ratio
                            < (*regions.offset(k as isize)).avg_sr_fr_ratio
                        {
                            prev_ratio
                        } else {
                            (*regions.offset(k as isize)).avg_sr_fr_ratio
                        }) * 0.95f64;
                    }
                    if last_ratio > ratio_thres_0 {
                        to_merge = 1 as core::ffi::c_int;
                    }
                }
                if to_merge != 0 {
                    remove_region(0 as core::ffi::c_int, regions, num_regions, &raw mut k);
                    analyze_region(stats, k - 1 as core::ffi::c_int, regions);
                    continue;
                } else {
                    let mut prev_k: core::ffi::c_int = k - 1 as core::ffi::c_int;
                    insert_region(
                        (*regions.offset(prev_k as isize)).last,
                        (*regions.offset(prev_k as isize)).last,
                        HIGH_VAR_REGION,
                        regions,
                        num_regions,
                        &raw mut prev_k,
                    );
                    analyze_region(stats, prev_k, regions);
                    k = prev_k + 1 as core::ffi::c_int;
                    analyze_region(stats, k, regions);
                }
            }
        }
        k += 1;
    }
    cleanup_regions(regions, num_regions);
}
unsafe extern "C-unwind" fn cleanup_blendings(
    mut regions: *mut REGIONS,
    mut num_regions: *mut core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < *num_regions && *num_regions > 1 as core::ffi::c_int {
        let mut is_short_blending: core::ffi::c_int = ((*regions.offset(k as isize))
            .type_0 as core::ffi::c_uint
            == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
            && ((*regions.offset(k as isize)).last - (*regions.offset(k as isize)).start
                + 1 as core::ffi::c_int) < 5 as core::ffi::c_int) as core::ffi::c_int;
        let mut is_short_hv: core::ffi::c_int = ((*regions.offset(k as isize)).type_0
            as core::ffi::c_uint
            == HIGH_VAR_REGION as core::ffi::c_int as core::ffi::c_uint
            && ((*regions.offset(k as isize)).last - (*regions.offset(k as isize)).start
                + 1 as core::ffi::c_int) < 5 as core::ffi::c_int) as core::ffi::c_int;
        let mut has_stable_neighbor: core::ffi::c_int = (k > 0 as core::ffi::c_int
            && (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0
                as core::ffi::c_uint
                == STABLE_REGION as core::ffi::c_int as core::ffi::c_uint
            || k < *num_regions - 1 as core::ffi::c_int
                && (*regions.offset((k + 1 as core::ffi::c_int) as isize)).type_0
                    as core::ffi::c_uint
                    == STABLE_REGION as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let mut has_blend_neighbor: core::ffi::c_int = (k > 0 as core::ffi::c_int
            && (*regions.offset((k - 1 as core::ffi::c_int) as isize)).type_0
                as core::ffi::c_uint
                == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
            || k < *num_regions - 1 as core::ffi::c_int
                && (*regions.offset((k + 1 as core::ffi::c_int) as isize)).type_0
                    as core::ffi::c_uint
                    == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let mut total_neighbors: core::ffi::c_int = (k > 0 as core::ffi::c_int)
            as core::ffi::c_int
            + (k < *num_regions - 1 as core::ffi::c_int) as core::ffi::c_int;
        if is_short_blending != 0
            || is_short_hv != 0
                && has_stable_neighbor + has_blend_neighbor >= total_neighbors
        {
            let mut merge: core::ffi::c_int = 0;
            let mut prev_diff: core::ffi::c_double = if k > 0 as core::ffi::c_int {
                fabs(
                    (*regions.offset(k as isize)).avg_cor_coeff
                        - (*regions.offset((k - 1 as core::ffi::c_int) as isize))
                            .avg_cor_coeff,
                )
            } else {
                1 as core::ffi::c_int as core::ffi::c_double
            };
            let mut next_diff: core::ffi::c_double = if k
                < *num_regions - 1 as core::ffi::c_int
            {
                fabs(
                    (*regions.offset(k as isize)).avg_cor_coeff
                        - (*regions.offset((k + 1 as core::ffi::c_int) as isize))
                            .avg_cor_coeff,
                )
            } else {
                1 as core::ffi::c_int as core::ffi::c_double
            };
            merge = (prev_diff > next_diff) as core::ffi::c_int;
            remove_region(merge, regions, num_regions, &raw mut k);
        } else {
            k += 1;
        }
    }
    cleanup_regions(regions, num_regions);
}
unsafe extern "C-unwind" fn free_firstpass_stats_buffers(
    mut temp_regions: *mut REGIONS,
    mut filt_intra_err: *mut core::ffi::c_double,
    mut filt_coded_err: *mut core::ffi::c_double,
    mut grad_coded: *mut core::ffi::c_double,
) {
    aom_free(temp_regions as *mut core::ffi::c_void);
    aom_free(filt_intra_err as *mut core::ffi::c_void);
    aom_free(filt_coded_err as *mut core::ffi::c_void);
    aom_free(grad_coded as *mut core::ffi::c_void);
}
unsafe extern "C-unwind" fn identify_regions(
    stats_start: *const FIRSTPASS_STATS,
    mut total_frames: core::ffi::c_int,
    mut offset: core::ffi::c_int,
    mut regions: *mut REGIONS,
    mut total_regions: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut k: core::ffi::c_int = 0;
    if total_frames <= 1 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut temp_regions: *mut REGIONS = aom_malloc(
        (total_frames as size_t)
            .wrapping_mul(::core::mem::size_of::<REGIONS>() as size_t),
    ) as *mut REGIONS;
    let mut filt_intra_err: *mut core::ffi::c_double = aom_calloc(
        total_frames as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    let mut filt_coded_err: *mut core::ffi::c_double = aom_calloc(
        total_frames as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    let mut grad_coded: *mut core::ffi::c_double = aom_calloc(
        total_frames as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    if !(!temp_regions.is_null() && !filt_intra_err.is_null()
        && !filt_coded_err.is_null() && !grad_coded.is_null())
    {
        free_firstpass_stats_buffers(
            temp_regions,
            filt_intra_err,
            filt_coded_err,
            grad_coded,
        );
        return -(1 as core::ffi::c_int);
    }
    memset(
        temp_regions as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (total_frames as size_t)
            .wrapping_mul(::core::mem::size_of::<REGIONS>() as size_t),
    );
    let mut cur_region: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut this_start: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut this_last: core::ffi::c_int = 0;
    let mut next_scenecut: core::ffi::c_int = -(1 as core::ffi::c_int);
    loop {
        next_scenecut = find_next_scenecut(
            stats_start,
            this_start,
            total_frames - 1 as core::ffi::c_int,
        );
        this_last = if next_scenecut >= 0 as core::ffi::c_int {
            next_scenecut - 1 as core::ffi::c_int
        } else {
            total_frames - 1 as core::ffi::c_int
        };
        smooth_filter_stats(
            stats_start,
            this_start,
            this_last,
            filt_intra_err,
            filt_coded_err,
        );
        get_gradient(filt_coded_err, this_start, this_last, grad_coded);
        let mut num_regions: core::ffi::c_int = find_stable_regions(
            stats_start,
            grad_coded,
            this_start,
            this_last,
            temp_regions,
        );
        adjust_unstable_region_bounds(stats_start, temp_regions, &raw mut num_regions);
        get_region_stats(stats_start, temp_regions, num_regions);
        find_blending_regions(stats_start, temp_regions, &raw mut num_regions);
        cleanup_blendings(temp_regions, &raw mut num_regions);
        k = 0 as core::ffi::c_int;
        while k < num_regions {
            if (*temp_regions.offset(k as isize)).type_0 as core::ffi::c_uint
                != STABLE_REGION as core::ffi::c_int as core::ffi::c_uint
            {
                k += 1;
            } else {
                let mut start: core::ffi::c_int = (*temp_regions.offset(k as isize))
                    .start;
                let mut last: core::ffi::c_int = (*temp_regions.offset(k as isize)).last;
                let mut i: core::ffi::c_int = start;
                while i <= last {
                    if (*stats_start.offset(i as isize)).is_flash != 0 {
                        insert_region(
                            i,
                            i,
                            HIGH_VAR_REGION,
                            temp_regions,
                            &raw mut num_regions,
                            &raw mut k,
                        );
                    }
                    i += 1;
                }
                k += 1;
            }
        }
        cleanup_regions(temp_regions, &raw mut num_regions);
        k = 0 as core::ffi::c_int;
        while k < num_regions {
            if (*temp_regions.offset(k as isize)).last
                < (*temp_regions.offset(k as isize)).start
                && k == num_regions - 1 as core::ffi::c_int
            {
                num_regions -= 1;
                break;
            } else {
                *regions.offset((k + cur_region) as isize) = *temp_regions
                    .offset(k as isize);
                k += 1;
            }
        }
        cur_region += num_regions;
        if next_scenecut > -(1 as core::ffi::c_int) {
            (*regions.offset(cur_region as isize)).type_0 = SCENECUT_REGION;
            (*regions.offset(cur_region as isize)).start = next_scenecut;
            (*regions.offset(cur_region as isize)).last = next_scenecut;
            cur_region += 1;
            this_start = next_scenecut + 1 as core::ffi::c_int;
        }
        if !(next_scenecut >= 0 as core::ffi::c_int) {
            break;
        }
    }
    *total_regions = cur_region;
    get_region_stats(stats_start, regions, *total_regions);
    k = 0 as core::ffi::c_int;
    while k < *total_regions {
        if !((*regions.offset(k as isize)).type_0 as core::ffi::c_uint
            != SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
            || (*regions.offset(k as isize)).avg_cor_coeff
                * (1 as core::ffi::c_int as core::ffi::c_double
                    - (*stats_start.offset((*regions.offset(k as isize)).start as isize))
                        .noise_var / (*regions.offset(k as isize)).avg_intra_err)
                < 0.8f64)
        {
            (*regions.offset(k as isize)).type_0 = HIGH_VAR_REGION;
        }
        k += 1;
    }
    cleanup_regions(regions, total_regions);
    get_region_stats(stats_start, regions, *total_regions);
    k = 0 as core::ffi::c_int;
    while k < *total_regions {
        (*regions.offset(k as isize)).start += offset;
        (*regions.offset(k as isize)).last += offset;
        k += 1;
    }
    free_firstpass_stats_buffers(
        temp_regions,
        filt_intra_err,
        filt_coded_err,
        grad_coded,
    );
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn find_regions_index(
    mut regions: *const REGIONS,
    mut num_regions: core::ffi::c_int,
    mut frame_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < num_regions {
        if (*regions.offset(k as isize)).start <= frame_idx
            && (*regions.offset(k as isize)).last >= frame_idx
        {
            return k;
        }
        k += 1;
    }
    return -(1 as core::ffi::c_int);
}
unsafe extern "C-unwind" fn calculate_gf_length(
    mut cpi: *mut AV1_COMP,
    mut max_gop_length: core::ffi::c_int,
    mut max_intervals: core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut next_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    let start_pos: *const FIRSTPASS_STATS = (*cpi).twopass_frame.stats_in;
    let stats: *const FIRSTPASS_STATS = start_pos
        .offset(
            -(((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int
                as isize),
        );
    let f_w: core::ffi::c_int = (*cpi).common.width;
    let f_h: core::ffi::c_int = (*cpi).common.height;
    let mut i: core::ffi::c_int = 0;
    let mut flash_detected: core::ffi::c_int = 0;
    memset(
        &raw mut next_frame as *mut FIRSTPASS_STATS as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    if has_no_stats_stage(cpi) != 0 {
        i = 0 as core::ffi::c_int;
        while i < MAX_NUM_GF_INTERVALS {
            (*p_rc).gf_intervals[i as usize] = if (*rc).max_gf_interval < max_gop_length
            {
                (*rc).max_gf_interval
            } else {
                max_gop_length
            };
            i += 1;
        }
        (*p_rc).cur_gf_index = 0 as core::ffi::c_int;
        (*rc).intervals_till_gf_calculate_due = MAX_NUM_GF_INTERVALS;
        return;
    }
    let active_min_gf_interval: core::ffi::c_int = (*rc).min_gf_interval;
    let active_max_gf_interval: core::ffi::c_int = if (*rc).max_gf_interval
        < max_gop_length
    {
        (*rc).max_gf_interval
    } else {
        max_gop_length
    };
    let min_shrink_int: core::ffi::c_int = if 6 as core::ffi::c_int
        > active_min_gf_interval
    {
        6 as core::ffi::c_int
    } else {
        active_min_gf_interval
    };
    i = ((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int;
    max_intervals = if (*(*cpi).ppi).lap_enabled != 0 {
        1 as core::ffi::c_int
    } else {
        max_intervals
    };
    let mut count_cuts: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut cur_start: core::ffi::c_int = -(1 as core::ffi::c_int)
        + ((*(*cpi).ppi).gf_state.arf_gf_boost_lst == 0) as core::ffi::c_int;
    let mut cur_last: core::ffi::c_int = 0;
    let mut cut_pos: [core::ffi::c_int; 16] = [
        -(1 as core::ffi::c_int),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut cut_here: core::ffi::c_int = 0;
    let mut gf_stats: GF_GROUP_STATS = GF_GROUP_STATS {
        gf_group_err: 0.,
        gf_group_raw_error: 0.,
        gf_group_skip_pct: 0.,
        gf_group_inactive_zone_rows: 0.,
        mv_ratio_accumulator: 0.,
        decay_accumulator: 0.,
        zero_motion_accumulator: 0.,
        loop_decay_rate: 0.,
        last_loop_decay_rate: 0.,
        this_frame_mv_in_out: 0.,
        mv_in_out_accumulator: 0.,
        abs_mv_in_out_accumulator: 0.,
        avg_sr_coded_error: 0.,
        avg_pcnt_second_ref: 0.,
        avg_new_mv_count: 0.,
        avg_wavelet_energy: 0.,
        avg_raw_err_stdev: 0.,
        non_zero_stdev_count: 0,
    };
    init_gf_stats(&raw mut gf_stats);
    while count_cuts < max_intervals + 1 as core::ffi::c_int {
        if i >= (*rc).frames_to_key {
            cut_here = 2 as core::ffi::c_int;
        } else if i - cur_start >= (*rc).static_scene_max_gf_interval {
            cut_here = 1 as core::ffi::c_int;
        } else if EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, &raw mut next_frame)
        {
            cut_here = 2 as core::ffi::c_int;
        } else {
            flash_detected = detect_flash(
                twopass,
                &raw mut (*cpi).twopass_frame,
                0 as core::ffi::c_int,
            );
            accumulate_next_frame_stats(
                &raw mut next_frame,
                flash_detected,
                (*rc).frames_since_key,
                i,
                &raw mut gf_stats,
                f_w,
                f_h,
            );
            cut_here = detect_gf_cut(
                cpi,
                i,
                cur_start,
                flash_detected,
                active_max_gf_interval,
                active_min_gf_interval,
                &raw mut gf_stats,
            );
        }
        if cut_here != 0 {
            cur_last = i - 1 as core::ffi::c_int;
            let mut ori_last: core::ffi::c_int = cur_last;
            let mut offset: core::ffi::c_int = (*rc).frames_since_key
                - (*p_rc).regions_offset;
            let mut regions: *mut REGIONS = ((*p_rc).regions).as_mut_ptr();
            let mut num_regions: core::ffi::c_int = (*p_rc).num_regions;
            let mut scenecut_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
            if cur_last - cur_start <= active_max_gf_interval && cur_last > cur_start {
                let mut k_start: core::ffi::c_int = find_regions_index(
                    regions,
                    num_regions,
                    cur_start + offset,
                );
                let mut k_last: core::ffi::c_int = find_regions_index(
                    regions,
                    num_regions,
                    cur_last + offset,
                );
                if cur_start + offset == 0 as core::ffi::c_int {
                    k_start = 0 as core::ffi::c_int;
                }
                let mut r: core::ffi::c_int = k_start + 1 as core::ffi::c_int;
                while r <= k_last {
                    if (*regions.offset(r as isize)).type_0 as core::ffi::c_uint
                        == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
                        && (*regions.offset(r as isize)).last - offset - cur_start
                            > active_min_gf_interval
                    {
                        scenecut_idx = r;
                        break;
                    } else {
                        r += 1;
                    }
                }
                if (*regions.offset((num_regions - 1 as core::ffi::c_int) as isize)).last
                    - (*regions.offset(scenecut_idx as isize)).last
                    < 4 as core::ffi::c_int
                {
                    scenecut_idx = -(1 as core::ffi::c_int);
                }
                if scenecut_idx != -(1 as core::ffi::c_int) {
                    let mut is_minor_sc: core::ffi::c_int = ((*regions
                        .offset(scenecut_idx as isize))
                        .avg_cor_coeff
                        * (1 as core::ffi::c_int as core::ffi::c_double
                            - (*stats
                                .offset(
                                    ((*regions.offset(scenecut_idx as isize)).start - offset)
                                        as isize,
                                ))
                                .noise_var
                                / (*regions.offset(scenecut_idx as isize)).avg_intra_err)
                        > 0.6f64) as core::ffi::c_int;
                    cur_last = (*regions.offset(scenecut_idx as isize)).last - offset
                        - (is_minor_sc == 0) as core::ffi::c_int;
                } else {
                    let mut is_last_analysed: core::ffi::c_int = (k_last
                        == num_regions - 1 as core::ffi::c_int
                        && cur_last + offset == (*regions.offset(k_last as isize)).last)
                        as core::ffi::c_int;
                    let mut not_enough_regions: core::ffi::c_int = (k_last - k_start
                        <= 1 as core::ffi::c_int
                            + ((*regions.offset(k_start as isize)).type_0
                                as core::ffi::c_uint
                                == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint)
                                as core::ffi::c_int) as core::ffi::c_int;
                    if !(is_last_analysed != 0 && not_enough_regions != 0) {
                        let arf_length_factor: core::ffi::c_double = 0.1f64;
                        let mut best_score: core::ffi::c_double = 0 as core::ffi::c_int
                            as core::ffi::c_double;
                        let mut best_j: core::ffi::c_int = -(1 as core::ffi::c_int);
                        let first_frame: core::ffi::c_int = (*regions
                            .offset(0 as core::ffi::c_int as isize))
                            .start - offset;
                        let last_frame: core::ffi::c_int = (*regions
                            .offset((num_regions - 1 as core::ffi::c_int) as isize))
                            .last - offset;
                        let mut base_score: core::ffi::c_double = 0.0f64;
                        let mut count_base: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut static_frames: bool = false_0 != 0;
                        let mut j: core::ffi::c_int = cur_start + 1 as core::ffi::c_int;
                        while j < cur_start + min_shrink_int {
                            if stats.offset(j as isize)
                                >= (*(*twopass).stats_buf_ctx).stats_in_end
                                    as *const FIRSTPASS_STATS
                            {
                                break;
                            }
                            base_score = (base_score + 1.0f64)
                                * (*stats.offset(j as isize)).cor_coeff;
                            count_base += 1;
                            j += 1;
                        }
                        if count_base != 0
                            && base_score / count_base as core::ffi::c_double > 0.992f64
                        {
                            static_frames = true_0 != 0;
                        }
                        let mut met_blending: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut last_blending: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut j_0: core::ffi::c_int = cur_start + min_shrink_int;
                        while j_0 <= cur_last {
                            if stats.offset(j_0 as isize)
                                >= (*(*twopass).stats_buf_ctx).stats_in_end
                                    as *const FIRSTPASS_STATS
                            {
                                break;
                            }
                            base_score = (base_score + 1.0f64)
                                * (*stats.offset(j_0 as isize)).cor_coeff;
                            let mut this_reg: core::ffi::c_int = find_regions_index(
                                regions,
                                num_regions,
                                j_0 + offset,
                            );
                            if !(this_reg < 0 as core::ffi::c_int) {
                                if (*regions.offset(this_reg as isize)).type_0
                                    as core::ffi::c_uint
                                    == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
                                {
                                    last_blending = 1 as core::ffi::c_int;
                                    if met_blending != 0 {
                                        break;
                                    }
                                    base_score = 0 as core::ffi::c_int as core::ffi::c_double;
                                } else {
                                    if last_blending != 0 {
                                        met_blending = 1 as core::ffi::c_int;
                                    }
                                    last_blending = 0 as core::ffi::c_int;
                                    let mut this_score: core::ffi::c_double = arf_length_factor
                                        * base_score;
                                    let mut temp_accu_coeff: core::ffi::c_double = 1.0f64;
                                    let mut count_f: core::ffi::c_int = 0 as core::ffi::c_int;
                                    let mut n: core::ffi::c_int = j_0 + 1 as core::ffi::c_int;
                                    while n <= j_0 + 3 as core::ffi::c_int && n <= last_frame {
                                        if stats.offset(n as isize)
                                            >= (*(*twopass).stats_buf_ctx).stats_in_end
                                                as *const FIRSTPASS_STATS
                                        {
                                            break;
                                        }
                                        temp_accu_coeff *= (*stats.offset(n as isize)).cor_coeff;
                                        this_score
                                            += temp_accu_coeff
                                                * sqrt(
                                                    (if 0.5f64
                                                        > 1 as core::ffi::c_int as core::ffi::c_double
                                                            - (*stats.offset(n as isize)).noise_var
                                                                / (if (*stats.offset(n as isize)).intra_error > 0.001f64 {
                                                                    (*stats.offset(n as isize)).intra_error
                                                                } else {
                                                                    0.001f64
                                                                })
                                                    {
                                                        0.5f64
                                                    } else {
                                                        1 as core::ffi::c_int as core::ffi::c_double
                                                            - (*stats.offset(n as isize)).noise_var
                                                                / (if (*stats.offset(n as isize)).intra_error > 0.001f64 {
                                                                    (*stats.offset(n as isize)).intra_error
                                                                } else {
                                                                    0.001f64
                                                                })
                                                    }),
                                                );
                                        count_f += 1;
                                        n += 1;
                                    }
                                    temp_accu_coeff = 1.0f64;
                                    let mut n_0: core::ffi::c_int = j_0;
                                    while n_0
                                        > j_0 - 3 as core::ffi::c_int * 2 as core::ffi::c_int
                                            + count_f && n_0 > first_frame
                                    {
                                        if stats.offset(n_0 as isize)
                                            < (*(*twopass).stats_buf_ctx).stats_in_start
                                                as *const FIRSTPASS_STATS
                                        {
                                            break;
                                        }
                                        temp_accu_coeff *= (*stats.offset(n_0 as isize)).cor_coeff;
                                        this_score
                                            += temp_accu_coeff
                                                * sqrt(
                                                    (if 0.5f64
                                                        > 1 as core::ffi::c_int as core::ffi::c_double
                                                            - (*stats.offset(n_0 as isize)).noise_var
                                                                / (if (*stats.offset(n_0 as isize)).intra_error > 0.001f64 {
                                                                    (*stats.offset(n_0 as isize)).intra_error
                                                                } else {
                                                                    0.001f64
                                                                })
                                                    {
                                                        0.5f64
                                                    } else {
                                                        1 as core::ffi::c_int as core::ffi::c_double
                                                            - (*stats.offset(n_0 as isize)).noise_var
                                                                / (if (*stats.offset(n_0 as isize)).intra_error > 0.001f64 {
                                                                    (*stats.offset(n_0 as isize)).intra_error
                                                                } else {
                                                                    0.001f64
                                                                })
                                                    }),
                                                );
                                        n_0 -= 1;
                                    }
                                    if this_score
                                        + (if static_frames as core::ffi::c_int != 0 {
                                            0.5f64
                                        } else {
                                            0 as core::ffi::c_int as core::ffi::c_double
                                        }) > best_score
                                    {
                                        best_score = this_score;
                                        best_j = j_0;
                                    }
                                }
                            }
                            j_0 += 1;
                        }
                        let mut best_reg: core::ffi::c_int = find_regions_index(
                            regions,
                            num_regions,
                            best_j + offset,
                        );
                        if best_reg < num_regions - 1 as core::ffi::c_int
                            && best_reg > 0 as core::ffi::c_int
                        {
                            if (*regions
                                .offset((best_reg - 1 as core::ffi::c_int) as isize))
                                .type_0 as core::ffi::c_uint
                                == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
                                && (*regions
                                    .offset((best_reg + 1 as core::ffi::c_int) as isize))
                                    .type_0 as core::ffi::c_uint
                                    == BLENDING_REGION as core::ffi::c_int as core::ffi::c_uint
                            {
                                if best_j + offset
                                    == (*regions.offset(best_reg as isize)).start
                                    && best_j + offset
                                        < (*regions.offset(best_reg as isize)).last
                                {
                                    best_j += 1 as core::ffi::c_int;
                                } else if best_j + offset
                                    == (*regions.offset(best_reg as isize)).last
                                    && best_j + offset
                                        > (*regions.offset(best_reg as isize)).start
                                {
                                    best_j -= 1 as core::ffi::c_int;
                                }
                            }
                        }
                        if cur_last - best_j < 2 as core::ffi::c_int {
                            best_j = cur_last;
                        }
                        if best_j > 0 as core::ffi::c_int && best_score > 0.1f64 {
                            cur_last = best_j;
                        }
                    }
                }
            }
            cut_pos[count_cuts as usize] = cur_last;
            count_cuts += 1;
            (*cpi).twopass_frame.stats_in = start_pos.offset(cur_last as isize);
            cur_start = cur_last;
            let mut cur_region_idx: core::ffi::c_int = find_regions_index(
                regions,
                num_regions,
                cur_start + 1 as core::ffi::c_int + offset,
            );
            if cur_region_idx >= 0 as core::ffi::c_int {
                if (*regions.offset(cur_region_idx as isize)).type_0 as core::ffi::c_uint
                    == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
                {
                    cur_start += 1;
                }
            }
            i = cur_last;
            if cut_here > 1 as core::ffi::c_int && cur_last == ori_last {
                break;
            }
            init_gf_stats(&raw mut gf_stats);
        }
        i += 1;
    }
    (*rc).intervals_till_gf_calculate_due = count_cuts - 1 as core::ffi::c_int;
    let mut n_1: core::ffi::c_int = 1 as core::ffi::c_int;
    while n_1 < count_cuts {
        (*p_rc).gf_intervals[(n_1 - 1 as core::ffi::c_int) as usize] = cut_pos[n_1
            as usize] - cut_pos[(n_1 - 1 as core::ffi::c_int) as usize];
        n_1 += 1;
    }
    (*p_rc).cur_gf_index = 0 as core::ffi::c_int;
    (*cpi).twopass_frame.stats_in = start_pos;
}
unsafe extern "C-unwind" fn correct_frames_to_key(mut cpi: *mut AV1_COMP) {
    let mut lookahead_size: core::ffi::c_int = av1_lookahead_depth(
        (*(*cpi).ppi).lookahead,
        (*cpi).compressor_stage,
    );
    if lookahead_size
        < av1_lookahead_pop_sz((*(*cpi).ppi).lookahead, (*cpi).compressor_stage)
    {
        (*cpi).rc.frames_to_key = if (*cpi).rc.frames_to_key < lookahead_size {
            (*cpi).rc.frames_to_key
        } else {
            lookahead_size
        };
    } else if (*(*cpi).ppi).frames_left > 0 as core::ffi::c_int {
        (*cpi).rc.frames_to_key = if (*cpi).rc.frames_to_key < (*(*cpi).ppi).frames_left
        {
            (*cpi).rc.frames_to_key
        } else {
            (*(*cpi).ppi).frames_left
        };
    }
}
unsafe extern "C-unwind" fn define_gf_group_pass0(
    mut cpi: *mut AV1_COMP,
    is_final_pass: core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let gf_cfg: *const GFConfig = &(*oxcf).gf_cfg;
    let mut target: core::ffi::c_int = 0;
    (*p_rc).baseline_gf_interval = (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize];
    (*rc).intervals_till_gf_calculate_due -= 1;
    (*p_rc).cur_gf_index += 1;
    correct_frames_to_key(cpi);
    if (*p_rc).baseline_gf_interval > (*rc).frames_to_key {
        (*p_rc).baseline_gf_interval = (*rc).frames_to_key;
    }
    (*p_rc).gfu_boost = DEFAULT_GF_BOOST;
    (*p_rc).constrained_gf_group = if (*p_rc).baseline_gf_interval >= (*rc).frames_to_key
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if (*oxcf).gf_cfg.gf_max_pyr_height == 0 as core::ffi::c_int
        || (*oxcf).gf_cfg.gf_min_pyr_height == 0 as core::ffi::c_int
    {
        (*gf_group).max_layer_depth_allowed = 1 as core::ffi::c_int;
    } else {
        (*gf_group).max_layer_depth_allowed = (*oxcf).gf_cfg.gf_max_pyr_height;
    }
    let thresh_sad: uint64_t = (8 as core::ffi::c_int * 64 as core::ffi::c_int
        * 64 as core::ffi::c_int) as uint64_t;
    if (*p_rc).baseline_gf_interval > (*gf_cfg).lag_in_frames
        || is_altref_enabled((*gf_cfg).lag_in_frames, (*gf_cfg).enable_auto_arf) == 0
        || (*p_rc).baseline_gf_interval < (*rc).min_gf_interval
        || (*rc).frame_source_sad_lag[0 as core::ffi::c_int as usize] > thresh_sad
    {
        (*gf_group).max_layer_depth_allowed = 0 as core::ffi::c_int;
    }
    av1_gop_setup_structure(cpi as *mut AV1_COMP, is_final_pass);
    let mut cur_index: core::ffi::c_int = 0 as core::ffi::c_int;
    while cur_index < (*gf_group).size {
        let cur_update_type: FRAME_UPDATE_TYPE = (*gf_group)
            .update_type[cur_index as usize];
        if (*oxcf).rc_cfg.mode as core::ffi::c_uint
            == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        {
            if cur_update_type as core::ffi::c_int == KF_UPDATE as core::ffi::c_int {
                target = av1_calc_iframe_target_size_one_pass_cbr(cpi);
            } else {
                target = av1_calc_pframe_target_size_one_pass_cbr(cpi, cur_update_type);
            }
        } else if cur_update_type as core::ffi::c_int == KF_UPDATE as core::ffi::c_int {
            target = av1_calc_iframe_target_size_one_pass_vbr(cpi);
        } else {
            target = av1_calc_pframe_target_size_one_pass_vbr(cpi, cur_update_type);
        }
        (*gf_group).bit_allocation[cur_index as usize] = target;
        cur_index += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_baseline_gf_interval(
    mut p_rc: *mut PRIMARY_RATE_CONTROL,
    mut arf_position: core::ffi::c_int,
) {
    (*p_rc).baseline_gf_interval = arf_position;
}
unsafe extern "C-unwind" fn init_gf_stats(mut gf_stats: *mut GF_GROUP_STATS) {
    (*gf_stats).gf_group_err = 0.0f64;
    (*gf_stats).gf_group_raw_error = 0.0f64;
    (*gf_stats).gf_group_skip_pct = 0.0f64;
    (*gf_stats).gf_group_inactive_zone_rows = 0.0f64;
    (*gf_stats).mv_ratio_accumulator = 0.0f64;
    (*gf_stats).decay_accumulator = 1.0f64;
    (*gf_stats).zero_motion_accumulator = 1.0f64;
    (*gf_stats).loop_decay_rate = 1.0f64;
    (*gf_stats).last_loop_decay_rate = 1.0f64;
    (*gf_stats).this_frame_mv_in_out = 0.0f64;
    (*gf_stats).mv_in_out_accumulator = 0.0f64;
    (*gf_stats).abs_mv_in_out_accumulator = 0.0f64;
    (*gf_stats).avg_sr_coded_error = 0.0f64;
    (*gf_stats).avg_pcnt_second_ref = 0.0f64;
    (*gf_stats).avg_new_mv_count = 0.0f64;
    (*gf_stats).avg_wavelet_energy = 0.0f64;
    (*gf_stats).avg_raw_err_stdev = 0.0f64;
    (*gf_stats).non_zero_stdev_count = 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn accumulate_gop_stats(
    mut cpi: *mut AV1_COMP,
    mut is_intra_only: core::ffi::c_int,
    mut f_w: core::ffi::c_int,
    mut f_h: core::ffi::c_int,
    mut next_frame: *mut FIRSTPASS_STATS,
    mut start_pos: *const FIRSTPASS_STATS,
    mut gf_stats: *mut GF_GROUP_STATS,
    mut idx: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut flash_detected: core::ffi::c_int = 0;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let mut frame_info: *mut FRAME_INFO = &raw mut (*cpi).frame_info;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    init_gf_stats(gf_stats);
    memset(
        next_frame as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    i = is_intra_only;
    while i < (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize] {
        if EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, next_frame) {
            break;
        }
        let mut mod_frame_err: core::ffi::c_double = calculate_modified_err(
            frame_info,
            twopass,
            oxcf,
            next_frame,
        );
        accumulate_this_frame_stats(next_frame, mod_frame_err, gf_stats);
        i += 1;
    }
    reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
    i = is_intra_only;
    input_stats(twopass, &raw mut (*cpi).twopass_frame, next_frame);
    while i < (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize] {
        if EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, next_frame) {
            break;
        }
        flash_detected = detect_flash(
            twopass,
            &raw mut (*cpi).twopass_frame,
            0 as core::ffi::c_int,
        );
        accumulate_next_frame_stats(
            next_frame,
            flash_detected,
            (*rc).frames_since_key,
            i,
            gf_stats,
            f_w,
            f_h,
        );
        i += 1;
    }
    i = (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize];
    average_gf_stats(i, gf_stats);
    *idx = i;
}
unsafe extern "C-unwind" fn update_gop_length(
    mut rc: *mut RATE_CONTROL,
    mut p_rc: *mut PRIMARY_RATE_CONTROL,
    mut idx: core::ffi::c_int,
    mut is_final_pass: core::ffi::c_int,
) {
    if is_final_pass != 0 {
        (*rc).intervals_till_gf_calculate_due -= 1;
        (*p_rc).cur_gf_index += 1;
    }
    (*p_rc).constrained_gf_group = if idx >= (*rc).frames_to_key {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    set_baseline_gf_interval(p_rc, idx);
    (*rc).frames_till_gf_update_due = (*p_rc).baseline_gf_interval;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_gop_bit_allocation(
    mut cpi: *const AV1_COMP,
    rc: *mut RATE_CONTROL,
    mut gf_group: *mut GF_GROUP,
    mut is_key_frame: core::ffi::c_int,
    mut use_arf: core::ffi::c_int,
    mut gf_group_bits: int64_t,
) {
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut gf_arf_bits: core::ffi::c_int = calculate_boost_bits(
        (*p_rc).baseline_gf_interval
            - ((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int,
        (*p_rc).gfu_boost,
        gf_group_bits,
    );
    gf_arf_bits = adjust_boost_bits_for_target_level(
        cpi,
        rc,
        gf_arf_bits,
        gf_group_bits,
        1 as core::ffi::c_int,
    );
    allocate_gf_group_bits(
        gf_group,
        p_rc,
        rc,
        gf_group_bits,
        gf_arf_bits,
        is_key_frame,
        use_arf,
    );
}
pub const REDUCE_GF_LENGTH_THRESH: core::ffi::c_int = 4 as core::ffi::c_int;
pub const REDUCE_GF_LENGTH_TO_KEY_THRESH: core::ffi::c_int = 9 as core::ffi::c_int;
pub const REDUCE_GF_LENGTH_BY: core::ffi::c_int = 1 as core::ffi::c_int;
unsafe extern "C-unwind" fn set_gop_bits_boost(
    mut cpi: *mut AV1_COMP,
    mut i: core::ffi::c_int,
    mut is_intra_only: core::ffi::c_int,
    mut is_final_pass: core::ffi::c_int,
    mut use_alt_ref: core::ffi::c_int,
    mut alt_offset: core::ffi::c_int,
    mut start_pos: *const FIRSTPASS_STATS,
    mut gf_stats: *mut GF_GROUP_STATS,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let mut frame_info: *mut FRAME_INFO = &raw mut (*cpi).frame_info;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    if (*cpi).oxcf.mode as core::ffi::c_int != REALTIME as core::ffi::c_int {
        let mut stats_in_backup: TWO_PASS_FRAME = (*cpi).twopass_frame;
        let mut gfu_boost_sum: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut gfu_count: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut accumulate_i: core::ffi::c_int = 0 as core::ffi::c_int;
        if (*rc).frames_since_key == 0 as core::ffi::c_int {
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < MAX_NUM_GF_INTERVALS {
                if (*p_rc).gf_intervals[k as usize] == 0 as core::ffi::c_int {
                    break;
                }
                let mut new_i: core::ffi::c_int = (*p_rc).gf_intervals[k as usize];
                let mut ext_len_new: core::ffi::c_int = new_i
                    - (if k == 0 as core::ffi::c_int {
                        is_intra_only
                    } else {
                        0 as core::ffi::c_int
                    });
                if use_alt_ref != 0 {
                    if accumulate_i >= (*rc).frames_to_key {
                        break;
                    }
                    let forward_frames: core::ffi::c_int = if (*rc).frames_to_key
                        - accumulate_i - new_i >= ext_len_new
                    {
                        ext_len_new
                    } else if 0 as core::ffi::c_int
                        > (*rc).frames_to_key - accumulate_i - new_i
                    {
                        0 as core::ffi::c_int
                    } else {
                        (*rc).frames_to_key - accumulate_i - new_i
                    };
                    if k != 0 {
                        (*cpi).twopass_frame.stats_in = ((*cpi).twopass_frame.stats_in)
                            .offset(new_i as isize);
                        if (*cpi).twopass_frame.stats_in
                            >= (*(*twopass).stats_buf_ctx).stats_in_end
                                as *const FIRSTPASS_STATS
                        {
                            (*cpi).twopass_frame.stats_in = (*(*twopass).stats_buf_ctx)
                                .stats_in_end;
                        }
                    }
                    reset_fpf_position(
                        &raw mut (*cpi).twopass_frame,
                        (*cpi).twopass_frame.stats_in,
                    );
                    let mut gfu_boost_tmp: core::ffi::c_int = av1_calc_arf_boost(
                        twopass,
                        &raw mut (*cpi).twopass_frame,
                        p_rc,
                        frame_info,
                        alt_offset,
                        forward_frames,
                        ext_len_new,
                        &raw mut (*p_rc).num_stats_used_for_gfu_boost,
                        &raw mut (*p_rc).num_stats_required_for_gfu_boost,
                        (*(*cpi).ppi).lap_enabled,
                        false_0 != 0,
                    );
                    gfu_boost_sum += gfu_boost_tmp;
                }
                gfu_count += 1;
                accumulate_i += new_i;
                k += 1;
            }
            (*p_rc).gfu_boost_average = gfu_boost_sum / gfu_count;
        }
        (*cpi).twopass_frame = stats_in_backup;
    }
    let mut ext_len: core::ffi::c_int = i - is_intra_only;
    let scale_max_boost: bool = (*cpi).oxcf.mode as core::ffi::c_int
        != REALTIME as core::ffi::c_int;
    if use_alt_ref != 0 {
        let forward_frames_0: core::ffi::c_int = if (*rc).frames_to_key - i >= ext_len {
            ext_len
        } else if 0 as core::ffi::c_int > (*rc).frames_to_key - i {
            0 as core::ffi::c_int
        } else {
            (*rc).frames_to_key - i
        };
        (*p_rc).gfu_boost = av1_calc_arf_boost(
            twopass,
            &raw mut (*cpi).twopass_frame,
            p_rc,
            frame_info,
            alt_offset,
            forward_frames_0,
            ext_len,
            &raw mut (*p_rc).num_stats_used_for_gfu_boost,
            &raw mut (*p_rc).num_stats_required_for_gfu_boost,
            (*(*cpi).ppi).lap_enabled,
            scale_max_boost,
        );
    } else {
        reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
        (*p_rc).gfu_boost = if (5400 as core::ffi::c_int)
            < av1_calc_arf_boost(
                twopass,
                &raw mut (*cpi).twopass_frame,
                p_rc,
                frame_info,
                alt_offset,
                ext_len,
                0 as core::ffi::c_int,
                &raw mut (*p_rc).num_stats_used_for_gfu_boost,
                &raw mut (*p_rc).num_stats_required_for_gfu_boost,
                (*(*cpi).ppi).lap_enabled,
                scale_max_boost,
            )
        {
            5400 as core::ffi::c_int
        } else {
            av1_calc_arf_boost(
                twopass,
                &raw mut (*cpi).twopass_frame,
                p_rc,
                frame_info,
                alt_offset,
                ext_len,
                0 as core::ffi::c_int,
                &raw mut (*p_rc).num_stats_used_for_gfu_boost,
                &raw mut (*p_rc).num_stats_required_for_gfu_boost,
                (*(*cpi).ppi).lap_enabled,
                scale_max_boost,
            )
        };
    }
    (*p_rc).arf_boost_factor = 1.0f32;
    if use_alt_ref != 0 && is_lossless_requested(rc_cfg) == 0 {
        if (*rc).frames_to_key - ext_len == REDUCE_GF_LENGTH_BY
            || (*rc).frames_to_key - ext_len == 0 as core::ffi::c_int
        {
            (*p_rc).arf_boost_factor = LAST_ALR_BOOST_FACTOR as float_t;
        }
    }
    reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
    if (*(*cpi).ppi).lap_enabled != 0 {
        (*gf_stats).gf_group_err = (*p_rc).baseline_gf_interval as core::ffi::c_double;
    }
    (*p_rc).gf_group_bits = calculate_total_gf_group_bits(cpi, (*gf_stats).gf_group_err);
    if (*rc_cfg).mode as core::ffi::c_uint
        != AOM_Q as core::ffi::c_int as core::ffi::c_uint
        && (*p_rc).baseline_gf_interval > 1 as core::ffi::c_int && is_final_pass != 0
    {
        let vbr_group_bits_per_frame: core::ffi::c_int = ((*p_rc).gf_group_bits
            / (*p_rc).baseline_gf_interval as int64_t) as core::ffi::c_int;
        let group_av_err: core::ffi::c_double = (*gf_stats).gf_group_raw_error
            / (*p_rc).baseline_gf_interval as core::ffi::c_double;
        let group_av_skip_pct: core::ffi::c_double = (*gf_stats).gf_group_skip_pct
            / (*p_rc).baseline_gf_interval as core::ffi::c_double;
        let group_av_inactive_zone: core::ffi::c_double = (*gf_stats)
            .gf_group_inactive_zone_rows * 2 as core::ffi::c_int as core::ffi::c_double
            / ((*p_rc).baseline_gf_interval as core::ffi::c_double
                * (*cm).mi_params.mb_rows as core::ffi::c_double);
        let mut tmp_q: core::ffi::c_int = 0;
        tmp_q = get_twopass_worst_quality(
            cpi,
            group_av_err,
            group_av_skip_pct + group_av_inactive_zone,
            vbr_group_bits_per_frame,
        );
        (*rc).active_worst_quality = if tmp_q
            > (*rc).active_worst_quality >> 1 as core::ffi::c_int
        {
            tmp_q
        } else {
            (*rc).active_worst_quality >> 1 as core::ffi::c_int
        };
    }
    if is_final_pass != 0 {
        (*twopass).kf_group_error_left -= (*gf_stats).gf_group_err;
    }
    reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
    if (*rc).frames_since_key != 0 as core::ffi::c_int {
        (*twopass).section_intra_rating = calculate_section_intra_ratio(
            start_pos,
            (*(*twopass).stats_buf_ctx).stats_in_end,
            (*p_rc).baseline_gf_interval,
        ) as core::ffi::c_uint;
    }
    av1_gop_bit_allocation(
        cpi,
        rc,
        gf_group,
        ((*rc).frames_since_key == 0 as core::ffi::c_int) as core::ffi::c_int,
        use_alt_ref,
        (*p_rc).gf_group_bits,
    );
    if is_final_pass != 0 {
        (*(*cpi).ppi).gf_state.arf_gf_boost_lst = use_alt_ref;
        (*twopass).rolling_arf_group_target_bits = 1 as core::ffi::c_int;
        (*twopass).rolling_arf_group_actual_bits = 1 as core::ffi::c_int;
    }
}
pub const LAST_ALR_BOOST_FACTOR: core::ffi::c_float = 0.2f32;
unsafe extern "C-unwind" fn define_gf_group(
    mut cpi: *mut AV1_COMP,
    mut frame_params: *mut EncodeFrameParams,
    mut is_final_pass: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut next_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    let start_pos: *const FIRSTPASS_STATS = (*cpi).twopass_frame.stats_in;
    let mut gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let gf_cfg: *const GFConfig = &(*oxcf).gf_cfg;
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    let f_w: core::ffi::c_int = (*cm).width;
    let f_h: core::ffi::c_int = (*cm).height;
    let mut i: core::ffi::c_int = 0;
    let is_intra_only: core::ffi::c_int = ((*rc).frames_since_key
        == 0 as core::ffi::c_int) as core::ffi::c_int;
    (*(*cpi).ppi).internal_altref_allowed = ((*gf_cfg).gf_max_pyr_height
        > 1 as core::ffi::c_int) as core::ffi::c_int;
    if is_intra_only == 0 {
        memset(
            &raw mut (*(*cpi).ppi).gf_group as *mut GF_GROUP as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<GF_GROUP>() as size_t,
        );
        (*cpi).gf_frame_index = 0 as core::ffi::c_uchar;
    }
    if has_no_stats_stage(cpi) != 0 {
        define_gf_group_pass0(cpi, is_final_pass);
        return;
    }
    if (*(*cpi).ppi).lap_enabled != 0 {
        correct_frames_to_key(cpi);
    }
    let mut gf_stats: GF_GROUP_STATS = GF_GROUP_STATS {
        gf_group_err: 0.,
        gf_group_raw_error: 0.,
        gf_group_skip_pct: 0.,
        gf_group_inactive_zone_rows: 0.,
        mv_ratio_accumulator: 0.,
        decay_accumulator: 0.,
        zero_motion_accumulator: 0.,
        loop_decay_rate: 0.,
        last_loop_decay_rate: 0.,
        this_frame_mv_in_out: 0.,
        mv_in_out_accumulator: 0.,
        abs_mv_in_out_accumulator: 0.,
        avg_sr_coded_error: 0.,
        avg_pcnt_second_ref: 0.,
        avg_new_mv_count: 0.,
        avg_wavelet_energy: 0.,
        avg_raw_err_stdev: 0.,
        non_zero_stdev_count: 0,
    };
    accumulate_gop_stats(
        cpi,
        is_intra_only,
        f_w,
        f_h,
        &raw mut next_frame,
        start_pos,
        &raw mut gf_stats,
        &raw mut i,
    );
    let can_disable_arf: core::ffi::c_int = ((*gf_cfg).gf_min_pyr_height == 0)
        as core::ffi::c_int;
    let active_min_gf_interval: core::ffi::c_int = (*rc).min_gf_interval;
    let can_disable_internal_arfs: core::ffi::c_int = ((*gf_cfg).gf_min_pyr_height
        <= 1 as core::ffi::c_int) as core::ffi::c_int;
    if can_disable_internal_arfs != 0
        && gf_stats.zero_motion_accumulator > MIN_ZERO_MOTION
        && gf_stats.avg_sr_coded_error < MAX_SR_CODED_ERROR as core::ffi::c_double
        && gf_stats.avg_raw_err_stdev < MAX_RAW_ERR_VAR as core::ffi::c_double
    {
        (*(*cpi).ppi).internal_altref_allowed = 0 as core::ffi::c_int;
    }
    let mut use_alt_ref: core::ffi::c_int = 0;
    if can_disable_arf != 0 {
        use_alt_ref = ((*p_rc).use_arf_in_this_kf_group != 0
            && i < (*gf_cfg).lag_in_frames && i >= MIN_GF_INTERVAL
            && ((*cpi).oxcf.mode as core::ffi::c_int != REALTIME as core::ffi::c_int
                || is_almost_static(
                    gf_stats.zero_motion_accumulator,
                    (*twopass).kf_zeromotion_pct,
                    (*(*cpi).ppi).lap_enabled,
                ) == 0)) as core::ffi::c_int;
    } else {
        use_alt_ref = ((*p_rc).use_arf_in_this_kf_group != 0
            && i < (*gf_cfg).lag_in_frames && i > 2 as core::ffi::c_int)
            as core::ffi::c_int;
    }
    if use_alt_ref != 0 {
        (*gf_group).max_layer_depth_allowed = (*gf_cfg).gf_max_pyr_height;
    } else {
        (*gf_group).max_layer_depth_allowed = 0 as core::ffi::c_int;
    }
    let mut alt_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let allow_gf_length_reduction: core::ffi::c_int = (((*rc_cfg).mode
        as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        && (*rc_cfg).cq_level <= 128 as core::ffi::c_int
        || (*(*cpi).ppi).internal_altref_allowed == 0)
        && is_lossless_requested(rc_cfg) == 0) as core::ffi::c_int;
    if allow_gf_length_reduction != 0 && use_alt_ref != 0 {
        let next_gf_len: core::ffi::c_int = (*rc).frames_to_key - i;
        let single_overlay_left: core::ffi::c_int = (next_gf_len == 0 as core::ffi::c_int
            && i > REDUCE_GF_LENGTH_THRESH) as core::ffi::c_int;
        let unbalanced_gf: core::ffi::c_int = (i > REDUCE_GF_LENGTH_TO_KEY_THRESH
            && (next_gf_len + 1 as core::ffi::c_int) < REDUCE_GF_LENGTH_TO_KEY_THRESH
            && next_gf_len + 1 as core::ffi::c_int >= (*rc).min_gf_interval)
            as core::ffi::c_int;
        if single_overlay_left != 0 || unbalanced_gf != 0 {
            let roll_back: core::ffi::c_int = REDUCE_GF_LENGTH_BY;
            if i - roll_back >= active_min_gf_interval + 1 as core::ffi::c_int {
                alt_offset = -roll_back;
                i -= roll_back;
                if is_final_pass != 0 {
                    (*rc).intervals_till_gf_calculate_due = 0 as core::ffi::c_int;
                }
                (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize] -= roll_back;
                reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
                accumulate_gop_stats(
                    cpi,
                    is_intra_only,
                    f_w,
                    f_h,
                    &raw mut next_frame,
                    start_pos,
                    &raw mut gf_stats,
                    &raw mut i,
                );
            }
        }
    }
    update_gop_length(rc, p_rc, i, is_final_pass);
    av1_gop_setup_structure(cpi as *mut AV1_COMP, is_final_pass);
    set_gop_bits_boost(
        cpi,
        i,
        is_intra_only,
        is_final_pass,
        use_alt_ref,
        alt_offset,
        start_pos,
        &raw mut gf_stats,
    );
    (*frame_params).frame_type = (if (*rc).frames_since_key == 0 as core::ffi::c_int {
        KEY_FRAME as core::ffi::c_int
    } else {
        INTER_FRAME as core::ffi::c_int
    }) as FRAME_TYPE;
    (*frame_params).show_frame = !((*gf_group)
        .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
        == ARF_UPDATE as core::ffi::c_int
        || (*gf_group).update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
            == INTNL_ARF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
}
pub const MIN_INTRA_LEVEL: core::ffi::c_double = 0.25f64;
pub const INTRA_VS_INTER_THRESH: core::ffi::c_double = 2.0f64;
pub const VERY_LOW_INTER_THRESH: core::ffi::c_double = 0.05f64;
pub const KF_II_ERR_THRESHOLD: core::ffi::c_double = 1.9f64;
pub const ERR_CHANGE_THRESHOLD: core::ffi::c_double = 0.4f64;
pub const II_IMPROVEMENT_THRESHOLD: core::ffi::c_double = 3.5f64;
pub const KF_II_MAX: core::ffi::c_double = 128.0f64;
pub const VERY_LOW_II: core::ffi::c_double = 1.5f64;
pub const ERROR_SPIKE: core::ffi::c_double = 5.0f64;
unsafe extern "C-unwind" fn slide_transition(
    mut this_frame: *const FIRSTPASS_STATS,
    mut last_frame: *const FIRSTPASS_STATS,
    mut next_frame: *const FIRSTPASS_STATS,
) -> core::ffi::c_int {
    return ((*this_frame).intra_error < (*this_frame).coded_error * VERY_LOW_II
        && (*this_frame).coded_error > (*last_frame).coded_error * ERROR_SPIKE
        && (*this_frame).coded_error > (*next_frame).coded_error * ERROR_SPIKE)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn get_second_ref_usage_thresh(
    mut frame_count_so_far: core::ffi::c_int,
) -> core::ffi::c_double {
    let adapt_upto: core::ffi::c_int = 32 as core::ffi::c_int;
    let min_second_ref_usage_thresh: core::ffi::c_double = 0.085f64;
    let second_ref_usage_thresh_max_delta: core::ffi::c_double = 0.035f64;
    if frame_count_so_far >= adapt_upto {
        return min_second_ref_usage_thresh + second_ref_usage_thresh_max_delta;
    }
    return min_second_ref_usage_thresh
        + frame_count_so_far as core::ffi::c_double
            / (adapt_upto - 1 as core::ffi::c_int) as core::ffi::c_double
            * second_ref_usage_thresh_max_delta;
}
unsafe extern "C-unwind" fn test_candidate_kf(
    mut firstpass_info: *const FIRSTPASS_INFO,
    mut this_stats_index: core::ffi::c_int,
    mut frame_count_so_far: core::ffi::c_int,
    mut rc_mode: aom_rc_mode,
    mut scenecut_mode: core::ffi::c_int,
    mut num_mbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut last_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
        firstpass_info,
        this_stats_index - 1 as core::ffi::c_int,
    );
    let mut this_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
        firstpass_info,
        this_stats_index,
    );
    let mut next_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
        firstpass_info,
        this_stats_index + 1 as core::ffi::c_int,
    );
    if last_stats.is_null() || this_stats.is_null() || next_stats.is_null() {
        return 0 as core::ffi::c_int;
    }
    let mut is_viable_kf: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut pcnt_intra: core::ffi::c_double = 1.0f64 - (*this_stats).pcnt_inter;
    let mut modified_pcnt_inter: core::ffi::c_double = (*this_stats).pcnt_inter
        - (*this_stats).pcnt_neutral;
    let second_ref_usage_thresh: core::ffi::c_double = get_second_ref_usage_thresh(
        frame_count_so_far,
    ) as core::ffi::c_double;
    let mut frames_to_test_after_candidate_key: core::ffi::c_int = SCENE_CUT_KEY_TEST_INTERVAL;
    let mut count_for_tolerable_prediction: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut stats_after_this_stats: core::ffi::c_int = av1_firstpass_info_future_count(
        firstpass_info,
        this_stats_index,
    ) - 1 as core::ffi::c_int;
    if scenecut_mode == ENABLE_SCENECUT_MODE_1 as core::ffi::c_int {
        if stats_after_this_stats < 3 as core::ffi::c_int {
            return 0 as core::ffi::c_int
        } else {
            frames_to_test_after_candidate_key = 3 as core::ffi::c_int;
            count_for_tolerable_prediction = 1 as core::ffi::c_int;
        }
    }
    frames_to_test_after_candidate_key = if frames_to_test_after_candidate_key
        < stats_after_this_stats
    {
        frames_to_test_after_candidate_key
    } else {
        stats_after_this_stats
    };
    if (!(rc_mode as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint)
        || frame_count_so_far >= 3 as core::ffi::c_int)
        && (*this_stats).pcnt_second_ref < second_ref_usage_thresh
        && (*next_stats).pcnt_second_ref < second_ref_usage_thresh
        && ((*this_stats).pcnt_inter < VERY_LOW_INTER_THRESH
            || slide_transition(this_stats, last_stats, next_stats) != 0
            || pcnt_intra > MIN_INTRA_LEVEL
                && pcnt_intra > INTRA_VS_INTER_THRESH * modified_pcnt_inter
                && (*this_stats).intra_error
                    / (if (*this_stats).coded_error
                        < 0 as core::ffi::c_int as core::ffi::c_double
                    {
                        (*this_stats).coded_error - 0.000001f64
                    } else {
                        (*this_stats).coded_error + 0.000001f64
                    }) < KF_II_ERR_THRESHOLD
                && (fabs((*last_stats).coded_error - (*this_stats).coded_error)
                    / (if (*this_stats).coded_error
                        < 0 as core::ffi::c_int as core::ffi::c_double
                    {
                        (*this_stats).coded_error - 0.000001f64
                    } else {
                        (*this_stats).coded_error + 0.000001f64
                    }) > ERR_CHANGE_THRESHOLD
                    || fabs((*last_stats).intra_error - (*this_stats).intra_error)
                        / (if (*this_stats).intra_error
                            < 0 as core::ffi::c_int as core::ffi::c_double
                        {
                            (*this_stats).intra_error - 0.000001f64
                        } else {
                            (*this_stats).intra_error + 0.000001f64
                        }) > ERR_CHANGE_THRESHOLD
                    || (*next_stats).intra_error
                        / (if (*next_stats).coded_error
                            < 0 as core::ffi::c_int as core::ffi::c_double
                        {
                            (*next_stats).coded_error - 0.000001f64
                        } else {
                            (*next_stats).coded_error + 0.000001f64
                        }) > II_IMPROVEMENT_THRESHOLD))
    {
        let mut i: core::ffi::c_int = 0;
        let mut boost_score: core::ffi::c_double = 0.0f64;
        let mut old_boost_score: core::ffi::c_double = 0.0f64;
        let mut decay_accumulator: core::ffi::c_double = 1.0f64;
        i = 1 as core::ffi::c_int;
        while i <= frames_to_test_after_candidate_key {
            let mut local_next_frame: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
                firstpass_info,
                this_stats_index + i,
            );
            if ((*local_next_frame).intra_error - (*this_stats).intra_error)
                / (if (*this_stats).intra_error
                    < 0 as core::ffi::c_int as core::ffi::c_double
                {
                    (*this_stats).intra_error - 0.000001f64
                } else {
                    (*this_stats).intra_error + 0.000001f64
                }) > 0.1f64
                && (*this_stats).coded_error
                    > (*local_next_frame).coded_error
                        * 6 as core::ffi::c_int as core::ffi::c_double
            {
                break;
            }
            let mut next_iiratio: core::ffi::c_double = BOOST_FACTOR
                * (*local_next_frame).intra_error
                / (if (*local_next_frame).coded_error
                    < 0 as core::ffi::c_int as core::ffi::c_double
                {
                    (*local_next_frame).coded_error - 0.000001f64
                } else {
                    (*local_next_frame).coded_error + 0.000001f64
                });
            if next_iiratio > KF_II_MAX {
                next_iiratio = KF_II_MAX;
            }
            if (*local_next_frame).pcnt_inter > 0.85f64 {
                decay_accumulator *= (*local_next_frame).pcnt_inter;
            } else {
                decay_accumulator *= (0.85f64 + (*local_next_frame).pcnt_inter) / 2.0f64;
            }
            boost_score += decay_accumulator * next_iiratio;
            if (*local_next_frame).pcnt_inter < 0.05f64 || next_iiratio < 1.5f64
                || (*local_next_frame).pcnt_inter - (*local_next_frame).pcnt_neutral
                    < 0.20f64 && next_iiratio < 3.0f64
                || boost_score - old_boost_score < 3.0f64
                || (*local_next_frame).intra_error
                    < 200.0f64 / num_mbs as core::ffi::c_double
            {
                break;
            }
            old_boost_score = boost_score;
            i += 1;
        }
        if boost_score > 30.0f64 && i > count_for_tolerable_prediction {
            is_viable_kf = 1 as core::ffi::c_int;
        } else {
            is_viable_kf = 0 as core::ffi::c_int;
        }
    }
    return is_viable_kf;
}
pub const FRAMES_TO_CHECK_DECAY: core::ffi::c_int = 8 as core::ffi::c_int;
pub const KF_MIN_FRAME_BOOST: core::ffi::c_double = 80.0f64;
pub const KF_MAX_FRAME_BOOST: core::ffi::c_double = 128.0f64;
unsafe extern "C-unwind" fn detect_app_forced_key(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    let mut num_frames_to_app_forced_key: core::ffi::c_int = is_forced_keyframe_pending(
        (*(*cpi).ppi).lookahead,
        (*(*(*cpi).ppi).lookahead).max_sz,
        (*cpi).compressor_stage,
    );
    return num_frames_to_app_forced_key;
}
unsafe extern "C-unwind" fn get_projected_kf_boost(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    if (*(*cpi).ppi).p_rc.num_stats_used_for_kf_boost >= (*cpi).rc.frames_to_key {
        return (*(*cpi).ppi).p_rc.kf_boost;
    }
    let mut tpl_factor: core::ffi::c_double = av1_get_kf_boost_projection_factor(
        (*cpi).rc.frames_to_key,
    );
    let mut tpl_factor_num_stats: core::ffi::c_double = av1_get_kf_boost_projection_factor(
        (*(*cpi).ppi).p_rc.num_stats_used_for_kf_boost,
    );
    let mut projected_kf_boost: core::ffi::c_int = rint(
        tpl_factor * (*(*cpi).ppi).p_rc.kf_boost as core::ffi::c_double
            / tpl_factor_num_stats,
    ) as core::ffi::c_int;
    return projected_kf_boost;
}
unsafe extern "C-unwind" fn define_kf_interval(
    mut cpi: *mut AV1_COMP,
    mut firstpass_info: *const FIRSTPASS_INFO,
    mut num_frames_to_detect_scenecut: core::ffi::c_int,
    mut search_start_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let twopass: *const TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let rc: *const RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let kf_cfg: *const KeyFrameCfg = &(*oxcf).kf_cfg;
    let mut recent_loop_decay: [core::ffi::c_double; 8] = [0.; 8];
    let mut decay_accumulator: core::ffi::c_double = 1.0f64;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0;
    let mut frames_to_key: core::ffi::c_int = search_start_idx;
    let mut frames_since_key: core::ffi::c_int = (*rc).frames_since_key
        + 1 as core::ffi::c_int;
    let mut scenecut_detected: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_frames_to_next_key: core::ffi::c_int = detect_app_forced_key(cpi);
    if num_frames_to_detect_scenecut == 0 as core::ffi::c_int {
        if num_frames_to_next_key != -(1 as core::ffi::c_int) {
            return num_frames_to_next_key
        } else {
            return (*rc).frames_to_key
        }
    }
    if num_frames_to_next_key != -(1 as core::ffi::c_int) {
        num_frames_to_detect_scenecut = if num_frames_to_detect_scenecut
            < num_frames_to_next_key
        {
            num_frames_to_detect_scenecut
        } else {
            num_frames_to_next_key
        };
    }
    j = 0 as core::ffi::c_int;
    while j < FRAMES_TO_CHECK_DECAY {
        recent_loop_decay[j as usize] = 1.0f64;
        j += 1;
    }
    i = 0 as core::ffi::c_int;
    let num_mbs: core::ffi::c_int = if (*oxcf).resize_cfg.resize_mode as core::ffi::c_int
        != RESIZE_NONE as core::ffi::c_int
    {
        (*cpi).initial_mbs
    } else {
        (*cpi).common.mi_params.MBs
    };
    let future_stats_count: core::ffi::c_int = av1_firstpass_info_future_count(
        firstpass_info,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    while frames_to_key < future_stats_count
        && frames_to_key < num_frames_to_detect_scenecut
    {
        if (*(*cpi).ppi).p_rc.enable_scenecut_detection > 0 as core::ffi::c_int
            && (*kf_cfg).auto_key as core::ffi::c_int != 0
            && (frames_to_key + 1 as core::ffi::c_int) < future_stats_count
        {
            let mut loop_decay_rate: core::ffi::c_double = 0.;
            if frames_since_key >= (*kf_cfg).key_freq_min {
                scenecut_detected = test_candidate_kf(
                    &(*twopass).firstpass_info,
                    frames_to_key,
                    frames_since_key,
                    (*oxcf).rc_cfg.mode,
                    (*(*cpi).ppi).p_rc.enable_scenecut_detection,
                    num_mbs,
                );
                if scenecut_detected != 0 {
                    let mut test_next_gop: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
                    while idx < 32 as core::ffi::c_int {
                        let mut next_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
                            firstpass_info,
                            frames_to_key + idx,
                        );
                        if !next_stats.is_null() {
                            if ((*cpi).common.current_frame.frame_number)
                                .wrapping_add(frames_to_key as core::ffi::c_uint)
                                .wrapping_add(idx as core::ffi::c_uint)
                                > 2 as core::ffi::c_uint
                                && (*next_stats).lt_coded_error * 2.5f64
                                    < (*next_stats).coded_error
                            {
                                test_next_gop = 1 as core::ffi::c_int;
                            }
                        }
                        idx += 1;
                    }
                    if test_next_gop == 0 {
                        break;
                    }
                }
            }
            let mut next_stats_0: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
                firstpass_info,
                frames_to_key + 1 as core::ffi::c_int,
            );
            loop_decay_rate = get_prediction_decay_rate(next_stats_0);
            recent_loop_decay[(i % FRAMES_TO_CHECK_DECAY) as usize] = loop_decay_rate;
            decay_accumulator = 1.0f64;
            j = 0 as core::ffi::c_int;
            while j < FRAMES_TO_CHECK_DECAY {
                decay_accumulator *= recent_loop_decay[j as usize];
                j += 1;
            }
            if frames_since_key >= (*kf_cfg).key_freq_min {
                scenecut_detected = detect_transition_to_still(
                    firstpass_info,
                    frames_to_key + 1 as core::ffi::c_int,
                    (*rc).min_gf_interval,
                    i,
                    (*kf_cfg).key_freq_max - i,
                    loop_decay_rate,
                    decay_accumulator,
                );
                if scenecut_detected != 0 {
                    (*p_rc).use_arf_in_this_kf_group = 0 as core::ffi::c_int;
                    break;
                }
            }
            frames_to_key += 1;
            frames_since_key += 1;
            if frames_to_key >= 2 as core::ffi::c_int * (*kf_cfg).key_freq_max {
                break;
            }
        } else {
            frames_to_key += 1;
            frames_since_key += 1;
        }
        i += 1;
    }
    if (*(*cpi).ppi).lap_enabled != 0 && scenecut_detected == 0 {
        frames_to_key = num_frames_to_next_key;
    }
    return frames_to_key;
}
unsafe extern "C-unwind" fn get_kf_group_avg_error(
    mut twopass: *mut TWO_PASS,
    mut twopass_frame: *mut TWO_PASS_FRAME,
    mut first_frame: *const FIRSTPASS_STATS,
    mut start_position: *const FIRSTPASS_STATS,
    mut frames_to_key: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut cur_frame: FIRSTPASS_STATS = *first_frame;
    let mut num_frames: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut kf_group_avg_error: core::ffi::c_double = 0.0f64;
    reset_fpf_position(twopass_frame, start_position);
    i = 0 as core::ffi::c_int;
    while i < frames_to_key {
        kf_group_avg_error += cur_frame.coded_error;
        if EOF == input_stats(twopass, twopass_frame, &raw mut cur_frame) {
            break;
        }
        i += 1;
    }
    num_frames = i + 1 as core::ffi::c_int;
    num_frames = if num_frames < frames_to_key { num_frames } else { frames_to_key };
    kf_group_avg_error = kf_group_avg_error / num_frames as core::ffi::c_double;
    return kf_group_avg_error;
}
unsafe extern "C-unwind" fn get_kf_group_bits(
    mut cpi: *mut AV1_COMP,
    mut kf_group_err: core::ffi::c_double,
    mut kf_group_avg_error: core::ffi::c_double,
) -> int64_t {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut kf_group_bits: int64_t = 0;
    if (*(*cpi).ppi).lap_enabled != 0 {
        kf_group_bits = (*rc).frames_to_key as int64_t
            * (*rc).avg_frame_bandwidth as int64_t;
        if (*cpi).oxcf.rc_cfg.vbr_corpus_complexity_lap != 0 {
            let mut vbr_corpus_complexity_lap: core::ffi::c_double = (*cpi)
                .oxcf
                .rc_cfg
                .vbr_corpus_complexity_lap as core::ffi::c_double / 10.0f64;
            kf_group_bits = (kf_group_bits as core::ffi::c_double
                * (kf_group_avg_error / vbr_corpus_complexity_lap)) as int64_t;
        }
    } else {
        kf_group_bits = ((*twopass).bits_left as core::ffi::c_double
            * (kf_group_err / (*twopass).modified_error_left)) as int64_t;
    }
    return kf_group_bits;
}
unsafe extern "C-unwind" fn calc_avg_stats(
    mut cpi: *mut AV1_COMP,
    mut avg_frame_stat: *mut FIRSTPASS_STATS,
) -> core::ffi::c_int {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut cur_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    memset(
        &raw mut cur_frame as *mut FIRSTPASS_STATS as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    let mut num_frames: core::ffi::c_int = 0 as core::ffi::c_int;
    num_frames = 0 as core::ffi::c_int;
    while num_frames < (*rc).frames_to_key - 1 as core::ffi::c_int {
        if EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, &raw mut cur_frame) {
            break;
        }
        av1_accumulate_stats(avg_frame_stat, &raw mut cur_frame);
        num_frames += 1;
    }
    if num_frames < 2 as core::ffi::c_int {
        return num_frames;
    }
    (*avg_frame_stat).weight = (*avg_frame_stat).weight
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).intra_error = (*avg_frame_stat).intra_error
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).frame_avg_wavelet_energy = (*avg_frame_stat)
        .frame_avg_wavelet_energy / num_frames as core::ffi::c_double;
    (*avg_frame_stat).coded_error = (*avg_frame_stat).coded_error
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).sr_coded_error = (*avg_frame_stat).sr_coded_error
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).pcnt_inter = (*avg_frame_stat).pcnt_inter
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).pcnt_motion = (*avg_frame_stat).pcnt_motion
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).pcnt_second_ref = (*avg_frame_stat).pcnt_second_ref
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).pcnt_neutral = (*avg_frame_stat).pcnt_neutral
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).intra_skip_pct = (*avg_frame_stat).intra_skip_pct
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).inactive_zone_rows = (*avg_frame_stat).inactive_zone_rows
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).inactive_zone_cols = (*avg_frame_stat).inactive_zone_cols
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).MVr = (*avg_frame_stat).MVr / num_frames as core::ffi::c_double;
    (*avg_frame_stat).mvr_abs = (*avg_frame_stat).mvr_abs
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).MVc = (*avg_frame_stat).MVc / num_frames as core::ffi::c_double;
    (*avg_frame_stat).mvc_abs = (*avg_frame_stat).mvc_abs
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).MVrv = (*avg_frame_stat).MVrv / num_frames as core::ffi::c_double;
    (*avg_frame_stat).MVcv = (*avg_frame_stat).MVcv / num_frames as core::ffi::c_double;
    (*avg_frame_stat).mv_in_out_count = (*avg_frame_stat).mv_in_out_count
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).new_mv_count = (*avg_frame_stat).new_mv_count
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).count = (*avg_frame_stat).count
        / num_frames as core::ffi::c_double;
    (*avg_frame_stat).duration = (*avg_frame_stat).duration
        / num_frames as core::ffi::c_double;
    return num_frames;
}
unsafe extern "C-unwind" fn get_kf_boost_score(
    mut cpi: *mut AV1_COMP,
    mut kf_raw_err: core::ffi::c_double,
    mut zero_motion_accumulator: *mut core::ffi::c_double,
    mut sr_accumulator: *mut core::ffi::c_double,
    mut use_avg_stat: core::ffi::c_int,
) -> core::ffi::c_double {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let frame_info: *mut FRAME_INFO = &raw mut (*cpi).frame_info;
    let mut frame_stat: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    memset(
        &raw mut frame_stat as *mut FIRSTPASS_STATS as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_stat_used: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut boost_score: core::ffi::c_double = 0.0f64;
    let kf_max_boost: core::ffi::c_double = if (*cpi).oxcf.rc_cfg.mode
        as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        fclamp(
            (*rc).frames_to_key as core::ffi::c_double * 2.0f64,
            KF_MIN_FRAME_BOOST,
            KF_MAX_FRAME_BOOST,
        ) as core::ffi::c_double
    } else {
        KF_MAX_FRAME_BOOST
    };
    if use_avg_stat != 0 {
        num_stat_used = calc_avg_stats(cpi, &raw mut frame_stat);
    }
    i = num_stat_used;
    while i < (*rc).frames_to_key - 1 as core::ffi::c_int {
        if use_avg_stat == 0
            && EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, &raw mut frame_stat)
        {
            break;
        }
        if i > 0 as core::ffi::c_int {
            *zero_motion_accumulator = if *zero_motion_accumulator
                < get_zero_motion_factor(&raw mut frame_stat)
            {
                *zero_motion_accumulator
            } else {
                get_zero_motion_factor(&raw mut frame_stat)
            };
        } else {
            *zero_motion_accumulator = frame_stat.pcnt_inter - frame_stat.pcnt_motion;
        }
        if *sr_accumulator < kf_raw_err * 1.50f64
            && i <= (*rc).max_gf_interval * 2 as core::ffi::c_int
        {
            let mut frame_boost: core::ffi::c_double = 0.;
            let mut zm_factor: core::ffi::c_double = 0.;
            zm_factor = 0.75f64 + *zero_motion_accumulator / 2.0f64;
            if i < 2 as core::ffi::c_int {
                *sr_accumulator = 0.0f64;
            }
            frame_boost = calc_kf_frame_boost(
                &raw mut (*(*cpi).ppi).p_rc,
                frame_info,
                &raw mut frame_stat,
                sr_accumulator,
                kf_max_boost,
            );
            boost_score += frame_boost * zm_factor;
        }
        i += 1;
    }
    return boost_score;
}
pub const MAX_KF_BITS_INTERVAL_SINGLE_PASS: core::ffi::c_int = 5 as core::ffi::c_int;
unsafe extern "C-unwind" fn find_next_key_frame(
    mut cpi: *mut AV1_COMP,
    mut this_frame: *mut FIRSTPASS_STATS,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let frame_info: *mut FRAME_INFO = &raw mut (*cpi).frame_info;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let kf_cfg: *const KeyFrameCfg = &(*oxcf).kf_cfg;
    let first_frame: FIRSTPASS_STATS = *this_frame;
    let mut next_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    let mut firstpass_info: *const FIRSTPASS_INFO = &raw mut (*twopass).firstpass_info;
    memset(
        &raw mut next_frame as *mut FIRSTPASS_STATS as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    (*rc).frames_since_key = 0 as core::ffi::c_int;
    (*p_rc).use_arf_in_this_kf_group = is_altref_enabled(
        (*oxcf).gf_cfg.lag_in_frames,
        (*oxcf).gf_cfg.enable_auto_arf,
    );
    memset(
        gf_group as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<GF_GROUP>() as size_t,
    );
    (*cpi).gf_frame_index = 0 as core::ffi::c_uchar;
    (*rc).frames_till_gf_update_due = 0 as core::ffi::c_int;
    if has_no_stats_stage(cpi) != 0 {
        let mut num_frames_to_app_forced_key: core::ffi::c_int = detect_app_forced_key(
            cpi,
        );
        (*p_rc).this_key_frame_forced = ((*current_frame).frame_number
            != 0 as core::ffi::c_uint && (*rc).frames_to_key == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        if num_frames_to_app_forced_key != -(1 as core::ffi::c_int) {
            (*rc).frames_to_key = num_frames_to_app_forced_key;
        } else {
            (*rc).frames_to_key = if 1 as core::ffi::c_int > (*kf_cfg).key_freq_max {
                1 as core::ffi::c_int
            } else {
                (*kf_cfg).key_freq_max
            };
        }
        correct_frames_to_key(cpi);
        (*p_rc).kf_boost = DEFAULT_KF_BOOST;
        (*gf_group).update_type[0 as core::ffi::c_int as usize] = KF_UPDATE
            as core::ffi::c_int as FRAME_UPDATE_TYPE;
        return;
    }
    let mut i: core::ffi::c_int = 0;
    let start_position: *const FIRSTPASS_STATS = (*cpi).twopass_frame.stats_in;
    let mut kf_bits: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut zero_motion_accumulator: core::ffi::c_double = 1.0f64;
    let mut boost_score: core::ffi::c_double = 0.0f64;
    let mut kf_raw_err: core::ffi::c_double = 0.0f64;
    let mut kf_mod_err: core::ffi::c_double = 0.0f64;
    let mut sr_accumulator: core::ffi::c_double = 0.0f64;
    let mut kf_group_avg_error: core::ffi::c_double = 0.0f64;
    let mut frames_to_key: core::ffi::c_int = 0;
    let mut frames_to_key_clipped: core::ffi::c_int = INT_MAX;
    let mut kf_group_bits_clipped: int64_t = INT64_MAX as int64_t;
    (*p_rc).this_key_frame_forced = (*p_rc).next_key_frame_forced;
    (*twopass).kf_group_bits = 0 as int64_t;
    (*twopass).kf_group_error_left = 0 as core::ffi::c_int as core::ffi::c_double;
    kf_raw_err = (*this_frame).intra_error;
    kf_mod_err = calculate_modified_err(frame_info, twopass, oxcf, this_frame);
    if (*cpi).ext_ratectrl.ready != 0
        && (*cpi).ext_ratectrl.funcs.rc_type as core::ffi::c_uint
            & AOM_RC_GOP as core::ffi::c_int as core::ffi::c_uint
            != 0 as core::ffi::c_uint
        && ((*cpi).ext_ratectrl.funcs.get_key_frame_decision).is_some()
    {
        let mut key_frame_decision: aom_rc_key_frame_decision_t = aom_rc_key_frame_decision {
            key_frame_show_index: 0,
            key_frame_group_size: 0,
        };
        let mut codec_status: aom_codec_err_t = av1_extrc_get_key_frame_decision(
            &raw mut (*cpi).ext_ratectrl,
            &raw mut key_frame_decision,
        );
        if codec_status as core::ffi::c_uint
            == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            (*rc).frames_to_key = key_frame_decision.key_frame_group_size;
        } else {
            aom_internal_error(
                (*cpi).common.error,
                codec_status,
                b"av1_extrc_get_key_frame_decision() failed\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        frames_to_key = define_kf_interval(
            cpi,
            firstpass_info,
            (*kf_cfg).key_freq_max,
            1 as core::ffi::c_int,
        );
        if frames_to_key != -(1 as core::ffi::c_int) {
            (*rc).frames_to_key = if (*kf_cfg).key_freq_max < frames_to_key {
                (*kf_cfg).key_freq_max
            } else {
                frames_to_key
            };
        } else {
            (*rc).frames_to_key = (*kf_cfg).key_freq_max;
        }
    }
    if (*(*cpi).ppi).lap_enabled != 0 {
        correct_frames_to_key(cpi);
    }
    if (*kf_cfg).auto_key as core::ffi::c_int != 0
        && (*rc).frames_to_key > (*kf_cfg).key_freq_max
    {
        let mut tmp_frame: FIRSTPASS_STATS = first_frame;
        (*rc).frames_to_key /= 2 as core::ffi::c_int;
        reset_fpf_position(&raw mut (*cpi).twopass_frame, start_position);
        i = 0 as core::ffi::c_int;
        while i < (*rc).frames_to_key {
            if EOF == input_stats(twopass, &raw mut (*cpi).twopass_frame, &raw mut tmp_frame) {
                break;
            }
            i += 1;
        }
        (*p_rc).next_key_frame_forced = 1 as core::ffi::c_int;
    } else if (*cpi).twopass_frame.stats_in
        == (*(*twopass).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS
        && is_stat_consumption_stage_twopass(cpi) != 0
        || (*rc).frames_to_key >= (*kf_cfg).key_freq_max
    {
        (*p_rc).next_key_frame_forced = 1 as core::ffi::c_int;
    } else {
        (*p_rc).next_key_frame_forced = 0 as core::ffi::c_int;
    }
    let mut kf_group_err: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    i = 0 as core::ffi::c_int;
    while i < (*rc).frames_to_key {
        let mut this_stats: *const FIRSTPASS_STATS = av1_firstpass_info_peek(
            &raw mut (*twopass).firstpass_info,
            i,
        );
        if !this_stats.is_null() {
            kf_group_err
                += calculate_modified_err_new(
                    frame_info,
                    &(*firstpass_info).total_stats,
                    this_stats,
                    (*oxcf).rc_cfg.vbrbias,
                    (*twopass).modified_error_min,
                    (*twopass).modified_error_max,
                );
            (*p_rc).num_stats_used_for_kf_boost += 1;
        }
        i += 1;
    }
    if (*twopass).bits_left > 0 as int64_t && (*twopass).modified_error_left > 0.0f64
        || (*(*cpi).ppi).lap_enabled != 0
            && (*oxcf).rc_cfg.mode as core::ffi::c_uint
                != AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        let max_bits: core::ffi::c_int = frame_max_bits(rc, oxcf) as core::ffi::c_int;
        let mut max_grp_bits: int64_t = 0;
        if (*oxcf).rc_cfg.vbr_corpus_complexity_lap != 0 {
            kf_group_avg_error = get_kf_group_avg_error(
                twopass,
                &raw mut (*cpi).twopass_frame,
                &first_frame,
                start_position,
                (*rc).frames_to_key,
            );
        }
        (*twopass).kf_group_bits = get_kf_group_bits(
            cpi,
            kf_group_err,
            kf_group_avg_error,
        );
        max_grp_bits = max_bits as int64_t * (*rc).frames_to_key as int64_t;
        if (*twopass).kf_group_bits > max_grp_bits {
            (*twopass).kf_group_bits = max_grp_bits;
        }
    } else {
        (*twopass).kf_group_bits = 0 as int64_t;
    }
    (*twopass).kf_group_bits = if 0 as int64_t > (*twopass).kf_group_bits {
        0 as int64_t
    } else {
        (*twopass).kf_group_bits
    };
    if (*(*cpi).ppi).lap_enabled != 0 {
        frames_to_key_clipped = (MAX_KF_BITS_INTERVAL_SINGLE_PASS as core::ffi::c_double
            * (*cpi).framerate) as core::ffi::c_int;
        if (*rc).frames_to_key > frames_to_key_clipped {
            kf_group_bits_clipped = ((*twopass).kf_group_bits as core::ffi::c_double
                * frames_to_key_clipped as core::ffi::c_double
                / (*rc).frames_to_key as core::ffi::c_double) as int64_t;
        }
    }
    reset_fpf_position(&raw mut (*cpi).twopass_frame, start_position);
    boost_score = get_kf_boost_score(
        cpi,
        kf_raw_err,
        &raw mut zero_motion_accumulator,
        &raw mut sr_accumulator,
        0 as core::ffi::c_int,
    );
    reset_fpf_position(&raw mut (*cpi).twopass_frame, start_position);
    (*twopass).kf_zeromotion_pct = (zero_motion_accumulator * 100.0f64)
        as core::ffi::c_int;
    (*twopass).section_intra_rating = calculate_section_intra_ratio(
        start_position,
        (*(*twopass).stats_buf_ctx).stats_in_end,
        (*rc).frames_to_key,
    ) as core::ffi::c_uint;
    (*p_rc).kf_boost = boost_score as core::ffi::c_int;
    if (*(*cpi).ppi).lap_enabled != 0 {
        if (*oxcf).rc_cfg.mode as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            (*p_rc).kf_boost = get_projected_kf_boost(cpi);
        } else {
            boost_score = get_kf_boost_score(
                cpi,
                kf_raw_err,
                &raw mut zero_motion_accumulator,
                &raw mut sr_accumulator,
                1 as core::ffi::c_int,
            );
            reset_fpf_position(&raw mut (*cpi).twopass_frame, start_position);
            (*p_rc).kf_boost += boost_score as core::ffi::c_int;
        }
    }
    if zero_motion_accumulator > STATIC_KF_GROUP_FLOAT_THRESH
        && (*rc).frames_to_key > 8 as core::ffi::c_int
    {
        (*p_rc).kf_boost = if (*p_rc).kf_boost > 5400 as core::ffi::c_int {
            (*p_rc).kf_boost
        } else {
            5400 as core::ffi::c_int
        };
    } else {
        (*p_rc).kf_boost = if (*p_rc).kf_boost
            > (*rc).frames_to_key * 3 as core::ffi::c_int
        {
            (*p_rc).kf_boost
        } else {
            (*rc).frames_to_key * 3 as core::ffi::c_int
        };
        (*p_rc).kf_boost = if (*p_rc).kf_boost > 600 as core::ffi::c_int {
            (*p_rc).kf_boost
        } else {
            600 as core::ffi::c_int
        };
    }
    kf_bits = calculate_boost_bits(
        (if (*rc).frames_to_key < frames_to_key_clipped {
            (*rc).frames_to_key
        } else {
            frames_to_key_clipped
        }) - 1 as core::ffi::c_int,
        (*p_rc).kf_boost,
        if (*twopass).kf_group_bits < kf_group_bits_clipped {
            (*twopass).kf_group_bits
        } else {
            kf_group_bits_clipped
        },
    );
    kf_bits = adjust_boost_bits_for_target_level(
        cpi,
        rc,
        kf_bits,
        (*twopass).kf_group_bits,
        0 as core::ffi::c_int,
    );
    (*twopass).kf_group_bits -= kf_bits as int64_t;
    (*gf_group).bit_allocation[0 as core::ffi::c_int as usize] = kf_bits;
    (*gf_group).update_type[0 as core::ffi::c_int as usize] = KF_UPDATE
        as core::ffi::c_int as FRAME_UPDATE_TYPE;
    if (*(*cpi).ppi).lap_enabled != 0 {
        (*twopass).kf_group_error_left = ((*rc).frames_to_key - 1 as core::ffi::c_int)
            as core::ffi::c_double;
    } else {
        (*twopass).kf_group_error_left = kf_group_err - kf_mod_err;
    }
    (*twopass).modified_error_left -= kf_group_err;
}
unsafe extern "C-unwind" fn get_section_target_bandwidth(
    mut cpi: *mut AV1_COMP,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut section_target_bandwidth: int64_t = 0;
    let frames_left: core::ffi::c_int = ((*(*(*twopass).stats_buf_ctx).total_stats).count
        - (*current_frame).frame_number as core::ffi::c_double) as core::ffi::c_int;
    if (*(*cpi).ppi).lap_enabled != 0 {
        section_target_bandwidth = (*rc).avg_frame_bandwidth as int64_t;
    } else {
        section_target_bandwidth = (*twopass).bits_left / frames_left as int64_t;
        section_target_bandwidth = if section_target_bandwidth < 2147483647 as int64_t {
            section_target_bandwidth
        } else {
            2147483647 as int64_t
        };
    }
    return section_target_bandwidth as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_twopass_params_based_on_fp_stats(
    mut cpi: *mut AV1_COMP,
    mut this_frame_ptr: *const FIRSTPASS_STATS,
) {
    if this_frame_ptr.is_null() {
        return;
    }
    let mut twopass_frame: *mut TWO_PASS_FRAME = &raw mut (*cpi).twopass_frame;
    (*twopass_frame).mb_av_energy = log1p((*this_frame_ptr).intra_error);
    let total_stats: *const FIRSTPASS_STATS = (*(*(*cpi).ppi).twopass.stats_buf_ctx)
        .total_stats;
    if is_fp_wavelet_energy_invalid(total_stats) == 0 as core::ffi::c_int {
        (*twopass_frame).frame_avg_haar_energy = log1p(
            (*this_frame_ptr).frame_avg_wavelet_energy,
        );
    }
    if (*this_frame_ptr).intra_skip_pct >= FC_ANIMATION_THRESH {
        (*twopass_frame).fr_content_type = FC_GRAPHICS_ANIMATION as core::ffi::c_int
            as FRAME_CONTENT_TYPE;
    } else {
        (*twopass_frame).fr_content_type = FC_NORMAL as core::ffi::c_int
            as FRAME_CONTENT_TYPE;
    };
}
unsafe extern "C-unwind" fn process_first_pass_stats(
    mut cpi: *mut AV1_COMP,
    mut this_frame: *mut FIRSTPASS_STATS,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let mut total_stats: *mut FIRSTPASS_STATS = (*(*twopass).stats_buf_ctx).total_stats;
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        != AOM_Q as core::ffi::c_int as core::ffi::c_uint
        && (*current_frame).frame_number == 0 as core::ffi::c_uint
        && (*cpi).gf_frame_index as core::ffi::c_int == 0 as core::ffi::c_int
        && !total_stats.is_null()
        && !((*(*twopass).stats_buf_ctx).total_left_stats).is_null()
    {
        if (*(*cpi).ppi).lap_enabled != 0 {
            *(*(*twopass).stats_buf_ctx).total_left_stats = *total_stats;
        }
        let section_target_bandwidth: core::ffi::c_int = get_section_target_bandwidth(
            cpi,
        ) as core::ffi::c_int;
        let section_length: core::ffi::c_double = (*(*(*twopass).stats_buf_ctx)
            .total_left_stats)
            .count;
        let section_error: core::ffi::c_double = (*(*(*twopass).stats_buf_ctx)
            .total_left_stats)
            .coded_error / section_length;
        let section_intra_skip: core::ffi::c_double = (*(*(*twopass).stats_buf_ctx)
            .total_left_stats)
            .intra_skip_pct / section_length;
        let section_inactive_zone: core::ffi::c_double = (*(*(*twopass).stats_buf_ctx)
            .total_left_stats)
            .inactive_zone_rows * 2 as core::ffi::c_int as core::ffi::c_double
            / ((*cm).mi_params.mb_rows as core::ffi::c_double * section_length);
        let tmp_q: core::ffi::c_int = get_twopass_worst_quality(
            cpi,
            section_error,
            section_intra_skip + section_inactive_zone,
            section_target_bandwidth,
        ) as core::ffi::c_int;
        (*rc).active_worst_quality = tmp_q;
        (*rc).ni_av_qi = tmp_q;
        (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize] = tmp_q;
        (*p_rc).avg_q = av1_convert_qindex_to_q(tmp_q, (*(*cm).seq_params).bit_depth);
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = tmp_q;
        (*p_rc).last_q[KEY_FRAME as core::ffi::c_int as usize] = (tmp_q
            + (*cpi).oxcf.rc_cfg.best_allowed_q) / 2 as core::ffi::c_int;
        (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize] = (*p_rc)
            .last_q[KEY_FRAME as core::ffi::c_int as usize];
    }
    if (*cpi).twopass_frame.stats_in
        < (*(*twopass).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS
    {
        *this_frame = *(*cpi).twopass_frame.stats_in;
        (*cpi).twopass_frame.stats_in = ((*cpi).twopass_frame.stats_in).offset(1);
    }
    set_twopass_params_based_on_fp_stats(cpi, this_frame);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_target_rate(mut cpi: *mut AV1_COMP) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let mut target_rate: core::ffi::c_int = (*gf_group)
        .bit_allocation[(*cpi).gf_frame_index as usize];
    if has_no_stats_stage(cpi) != 0 {
        av1_rc_set_frame_target(
            cpi as *mut AV1_COMP,
            target_rate,
            (*cpi).common.width,
            (*cpi).common.height,
        );
    }
    (*rc).base_frame_target = target_rate;
}
unsafe extern "C-unwind" fn mark_flashes(
    mut first_stats: *mut FIRSTPASS_STATS,
    mut last_stats: *mut FIRSTPASS_STATS,
) {
    let mut this_stats: *mut FIRSTPASS_STATS = first_stats;
    let mut next_stats: *mut FIRSTPASS_STATS = 0 as *mut FIRSTPASS_STATS;
    while this_stats < last_stats.offset(-(1 as core::ffi::c_int as isize)) {
        next_stats = this_stats.offset(1 as core::ffi::c_int as isize);
        if (*next_stats).pcnt_second_ref > (*next_stats).pcnt_inter
            && (*next_stats).pcnt_second_ref >= 0.5f64
        {
            (*this_stats).is_flash = 1 as int64_t;
        } else {
            (*this_stats).is_flash = 0 as int64_t;
        }
        this_stats = next_stats;
    }
    if last_stats.offset(-(1 as core::ffi::c_int as isize)) >= first_stats {
        (*last_stats.offset(-(1 as core::ffi::c_int as isize))).is_flash = 0 as int64_t;
    }
}
unsafe extern "C-unwind" fn smooth_filter_noise(
    mut first_stats: *mut FIRSTPASS_STATS,
    mut last_stats: *mut FIRSTPASS_STATS,
) -> core::ffi::c_int {
    let mut len: core::ffi::c_int = last_stats.offset_from(first_stats)
        as core::ffi::c_long as core::ffi::c_int;
    let mut smooth_noise: *mut core::ffi::c_double = aom_malloc(
        (len as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    if smooth_noise.is_null() {
        return -(1 as core::ffi::c_int);
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < len {
        let mut total_noise: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut total_wt: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut j: core::ffi::c_int = -HALF_FILT_LEN;
        while j <= HALF_FILT_LEN {
            let mut idx: core::ffi::c_int = clamp(
                i + j,
                0 as core::ffi::c_int,
                len - 1 as core::ffi::c_int,
            );
            if !((*first_stats.offset(idx as isize)).is_flash != 0) {
                total_noise += (*first_stats.offset(idx as isize)).noise_var;
                total_wt += 1.0f64;
            }
            j += 1;
        }
        if total_wt > 0.01f64 {
            total_noise /= total_wt;
        } else {
            total_noise = (*first_stats.offset(i as isize)).noise_var;
        }
        *smooth_noise.offset(i as isize) = total_noise;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < len {
        (*first_stats.offset(i_0 as isize)).noise_var = *smooth_noise
            .offset(i_0 as isize);
        i_0 += 1;
    }
    aom_free(smooth_noise as *mut core::ffi::c_void);
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn estimate_noise(
    mut first_stats: *mut FIRSTPASS_STATS,
    mut last_stats: *mut FIRSTPASS_STATS,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut this_stats: *mut FIRSTPASS_STATS = 0 as *mut FIRSTPASS_STATS;
    let mut next_stats: *mut FIRSTPASS_STATS = 0 as *mut FIRSTPASS_STATS;
    let mut C1: core::ffi::c_double = 0.;
    let mut C2: core::ffi::c_double = 0.;
    let mut C3: core::ffi::c_double = 0.;
    let mut noise: core::ffi::c_double = 0.;
    this_stats = first_stats.offset(2 as core::ffi::c_int as isize);
    while this_stats < last_stats {
        (*this_stats).noise_var = 0.0f64;
        if !((*this_stats).is_flash != 0
            || (*this_stats.offset(-(1 as core::ffi::c_int as isize))).is_flash != 0
            || (*this_stats.offset(-(2 as core::ffi::c_int as isize))).is_flash != 0)
        {
            C1 = (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                * ((*this_stats).intra_error - (*this_stats).coded_error);
            C2 = (*this_stats.offset(-(2 as core::ffi::c_int as isize))).intra_error
                * ((*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                    - (*this_stats.offset(-(1 as core::ffi::c_int as isize)))
                        .coded_error);
            C3 = (*this_stats.offset(-(2 as core::ffi::c_int as isize))).intra_error
                * ((*this_stats).intra_error - (*this_stats).sr_coded_error);
            if !(C1 <= 0 as core::ffi::c_int as core::ffi::c_double
                || C2 <= 0 as core::ffi::c_int as core::ffi::c_double
                || C3 <= 0 as core::ffi::c_int as core::ffi::c_double)
            {
                C1 = sqrt(C1);
                C2 = sqrt(C2);
                C3 = sqrt(C3);
                noise = (*this_stats.offset(-(1 as core::ffi::c_int as isize)))
                    .intra_error - C1 * C2 / C3;
                noise = if noise > 0.01f64 { noise } else { 0.01f64 };
                (*this_stats).noise_var = noise;
            }
        }
        this_stats = this_stats.offset(1);
    }
    this_stats = first_stats.offset(2 as core::ffi::c_int as isize);
    while this_stats < last_stats {
        if !((*this_stats).is_flash != 0
            || (*this_stats.offset(-(1 as core::ffi::c_int as isize))).is_flash != 0
            || (*this_stats.offset(-(2 as core::ffi::c_int as isize))).is_flash != 0)
        {
            if (*this_stats).noise_var < 1.0f64 {
                let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
                next_stats = this_stats.offset(1 as core::ffi::c_int as isize);
                while next_stats < last_stats {
                    if (*next_stats).is_flash != 0
                        || (*next_stats.offset(-(1 as core::ffi::c_int as isize)))
                            .is_flash != 0
                        || (*next_stats.offset(-(2 as core::ffi::c_int as isize)))
                            .is_flash != 0 || (*next_stats).noise_var < 1.0f64
                    {
                        next_stats = next_stats.offset(1);
                    } else {
                        found = 1 as core::ffi::c_int;
                        (*this_stats).noise_var = (*next_stats).noise_var;
                        break;
                    }
                }
                if !(found != 0) {
                    next_stats = this_stats.offset(-(1 as core::ffi::c_int as isize));
                    while next_stats
                        >= first_stats.offset(2 as core::ffi::c_int as isize)
                    {
                        if (*next_stats).is_flash != 0
                            || (*next_stats.offset(-(1 as core::ffi::c_int as isize)))
                                .is_flash != 0
                            || (*next_stats.offset(-(2 as core::ffi::c_int as isize)))
                                .is_flash != 0 || (*next_stats).noise_var < 1.0f64
                        {
                            next_stats = next_stats.offset(-1);
                        } else {
                            (*this_stats).noise_var = (*next_stats).noise_var;
                            break;
                        }
                    }
                }
            }
        }
        this_stats = this_stats.offset(1);
    }
    this_stats = first_stats.offset(2 as core::ffi::c_int as isize);
    while this_stats < last_stats {
        if (*this_stats).is_flash != 0
            || (*this_stats.offset(-(1 as core::ffi::c_int as isize))).is_flash != 0
            || (*this_stats.offset(-(2 as core::ffi::c_int as isize))).is_flash != 0
        {
            let mut found_0: core::ffi::c_int = 0 as core::ffi::c_int;
            next_stats = this_stats.offset(1 as core::ffi::c_int as isize);
            while next_stats < last_stats {
                if (*next_stats).is_flash != 0
                    || (*next_stats.offset(-(1 as core::ffi::c_int as isize))).is_flash
                        != 0
                    || (*next_stats.offset(-(2 as core::ffi::c_int as isize))).is_flash
                        != 0
                {
                    next_stats = next_stats.offset(1);
                } else {
                    found_0 = 1 as core::ffi::c_int;
                    (*this_stats).noise_var = (*next_stats).noise_var;
                    break;
                }
            }
            if !(found_0 != 0) {
                next_stats = this_stats.offset(-(1 as core::ffi::c_int as isize));
                while next_stats >= first_stats.offset(2 as core::ffi::c_int as isize) {
                    if (*next_stats).is_flash != 0
                        || (*next_stats.offset(-(1 as core::ffi::c_int as isize)))
                            .is_flash != 0
                        || (*next_stats.offset(-(2 as core::ffi::c_int as isize)))
                            .is_flash != 0
                    {
                        next_stats = next_stats.offset(-1);
                    } else {
                        (*this_stats).noise_var = (*next_stats).noise_var;
                        break;
                    }
                }
            }
        }
        this_stats = this_stats.offset(1);
    }
    this_stats = first_stats;
    while this_stats < first_stats.offset(2 as core::ffi::c_int as isize)
        && first_stats.offset(2 as core::ffi::c_int as isize) < last_stats
    {
        (*this_stats).noise_var = (*first_stats.offset(2 as core::ffi::c_int as isize))
            .noise_var;
        this_stats = this_stats.offset(1);
    }
    if smooth_filter_noise(first_stats, last_stats) == -(1 as core::ffi::c_int) {
        aom_internal_error(
            error_info,
            AOM_CODEC_MEM_ERROR,
            b"Error allocating buffers in smooth_filter_noise()\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
}
unsafe extern "C-unwind" fn estimate_coeff(
    mut first_stats: *mut FIRSTPASS_STATS,
    mut last_stats: *mut FIRSTPASS_STATS,
) {
    let mut this_stats: *mut FIRSTPASS_STATS = 0 as *mut FIRSTPASS_STATS;
    this_stats = first_stats.offset(1 as core::ffi::c_int as isize);
    while this_stats < last_stats {
        let C: core::ffi::c_double = sqrt(
            if (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                * ((*this_stats).intra_error - (*this_stats).coded_error) > 0.001f64
            {
                (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                    * ((*this_stats).intra_error - (*this_stats).coded_error)
            } else {
                0.001f64
            },
        ) as core::ffi::c_double;
        let cor_coeff: core::ffi::c_double = C
            / (if (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                - (*this_stats).noise_var > 0.001f64
            {
                (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                    - (*this_stats).noise_var
            } else {
                0.001f64
            });
        (*this_stats).cor_coeff = cor_coeff
            * sqrt(
                (if (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                    - (*this_stats).noise_var > 0.001f64
                {
                    (*this_stats.offset(-(1 as core::ffi::c_int as isize))).intra_error
                        - (*this_stats).noise_var
                } else {
                    0.001f64
                })
                    / (if (*this_stats).intra_error - (*this_stats).noise_var > 0.001f64
                    {
                        (*this_stats).intra_error - (*this_stats).noise_var
                    } else {
                        0.001f64
                    }),
            );
        (*this_stats).cor_coeff = fclamp((*this_stats).cor_coeff, 0.0f64, 1.0f64);
        this_stats = this_stats.offset(1);
    }
    (*first_stats).cor_coeff = 1.0f64;
}
unsafe extern "C-unwind" fn get_one_pass_rt_lag_params(
    mut cpi: *mut AV1_COMP,
    mut frame_flags: core::ffi::c_uint,
    frame_params: *mut EncodeFrameParams,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let frames_to_next_forced_key: core::ffi::c_int = detect_app_forced_key(cpi)
        as core::ffi::c_int;
    if frames_to_next_forced_key == 0 as core::ffi::c_int {
        (*rc).frames_to_key = 0 as core::ffi::c_int;
        frame_flags &= FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint;
    } else if frames_to_next_forced_key > 0 as core::ffi::c_int
        && frames_to_next_forced_key < (*rc).frames_to_key
    {
        (*rc).frames_to_key = frames_to_next_forced_key;
    }
    (*frame_params).frame_type = (*gf_group).frame_type[(*cpi).gf_frame_index as usize];
    if ((*cpi).gf_frame_index as core::ffi::c_int) < (*gf_group).size
        && frame_flags & FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint == 0
    {
        av1_setup_target_rate(cpi);
    }
    if (*rc).frames_to_key <= 0 as core::ffi::c_int {
        let kf_cfg: *const KeyFrameCfg = &(*oxcf).kf_cfg;
        (*frame_params).frame_type = KEY_FRAME as core::ffi::c_int as FRAME_TYPE;
        (*rc).frames_since_key = 0 as core::ffi::c_int;
        (*p_rc).use_arf_in_this_kf_group = is_altref_enabled(
            (*oxcf).gf_cfg.lag_in_frames,
            (*oxcf).gf_cfg.enable_auto_arf,
        );
        memset(
            gf_group as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<GF_GROUP>() as size_t,
        );
        (*cpi).gf_frame_index = 0 as core::ffi::c_uchar;
        (*rc).frames_till_gf_update_due = 0 as core::ffi::c_int;
        let mut num_frames_to_app_forced_key: core::ffi::c_int = detect_app_forced_key(
            cpi,
        );
        (*p_rc).this_key_frame_forced = ((*cpi).common.current_frame.frame_number
            != 0 as core::ffi::c_uint && (*rc).frames_to_key == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        if num_frames_to_app_forced_key != -(1 as core::ffi::c_int) {
            (*rc).frames_to_key = num_frames_to_app_forced_key;
        } else {
            (*rc).frames_to_key = if 1 as core::ffi::c_int > (*kf_cfg).key_freq_max {
                1 as core::ffi::c_int
            } else {
                (*kf_cfg).key_freq_max
            };
        }
        correct_frames_to_key(cpi);
        (*p_rc).kf_boost = DEFAULT_KF_BOOST;
        (*gf_group).update_type[0 as core::ffi::c_int as usize] = KF_UPDATE
            as core::ffi::c_int as FRAME_UPDATE_TYPE;
    }
    if (*cpi).gf_frame_index as core::ffi::c_int == (*gf_group).size {
        let mut max_gop_length: core::ffi::c_int = if (*oxcf).gf_cfg.lag_in_frames
            >= 32 as core::ffi::c_int
        {
            if (32 as core::ffi::c_int)
                < (*oxcf).gf_cfg.lag_in_frames
                    - (*oxcf).algo_cfg.arnr_max_frames / 2 as core::ffi::c_int
            {
                32 as core::ffi::c_int
            } else {
                (*oxcf).gf_cfg.lag_in_frames
                    - (*oxcf).algo_cfg.arnr_max_frames / 2 as core::ffi::c_int
            }
        } else {
            MAX_GF_LENGTH_LAP
        };
        max_gop_length = if max_gop_length < (*rc).frames_to_key {
            max_gop_length
        } else {
            (*rc).frames_to_key
        };
        let mut num_frames: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut scene_change_gop_frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
        (*rc).frame_source_sad_lag[0 as core::ffi::c_int as usize] = 0 as uint64_t;
        (*rc).avg_source_sad = 0 as uint64_t;
        let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
        while i < max_gop_length {
            let mut frame_input: EncodeFrameInput = EncodeFrameInput {
                source: 0 as *mut YV12_BUFFER_CONFIG,
                last_source: 0 as *mut YV12_BUFFER_CONFIG,
                ts_duration: 0,
            };
            memset(
                &raw mut frame_input as *mut EncodeFrameInput as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<EncodeFrameInput>() as size_t,
            );
            let mut e: *mut lookahead_entry = av1_lookahead_peek(
                (*(*cpi).ppi).lookahead,
                i,
                (*cpi).compressor_stage,
            );
            let mut e_prev: *mut lookahead_entry = av1_lookahead_peek(
                (*(*cpi).ppi).lookahead,
                i - 1 as core::ffi::c_int,
                (*cpi).compressor_stage,
            );
            if !e.is_null() && !e_prev.is_null() {
                frame_input.source = &raw mut (*e).img;
                frame_input.last_source = &raw mut (*e_prev).img;
                (*rc).high_source_sad_lag[i as usize] = -(1 as core::ffi::c_int);
                (*rc).frame_source_sad_lag[i as usize] = 0 as uint64_t;
                av1_rc_scene_detection_onepass_rt(
                    cpi as *mut AV1_COMP,
                    &raw mut frame_input,
                );
                (*rc).high_source_sad_lag[i as usize] = (*rc).high_source_sad;
                (*rc).frame_source_sad_lag[i as usize] = (*rc).frame_source_sad;
                if (*rc).high_source_sad_lag[i as usize] == 1 as core::ffi::c_int
                    && i > 1 as core::ffi::c_int
                {
                    scene_change_gop_frame_index = i;
                    break;
                } else {
                    num_frames += 1;
                    (*rc).frame_source_sad_lag[0 as core::ffi::c_int as usize] = ((*rc)
                        .frame_source_sad_lag[0 as core::ffi::c_int as usize])
                        .wrapping_add((*rc).frame_source_sad_lag[i as usize]);
                }
            }
            i += 1;
        }
        if scene_change_gop_frame_index > 0 as core::ffi::c_int {
            max_gop_length = if max_gop_length < scene_change_gop_frame_index {
                max_gop_length
            } else {
                scene_change_gop_frame_index
            };
        }
        (*rc).frame_source_sad_lag[0 as core::ffi::c_int as usize] = ((*rc)
            .frame_source_sad_lag[0 as core::ffi::c_int as usize])
            .wrapping_div(num_frames as uint64_t);
        calculate_gf_length(cpi, max_gop_length, MAX_NUM_GF_INTERVALS);
        define_gf_group(cpi, frame_params, 0 as core::ffi::c_int);
        (*rc).frames_till_gf_update_due = (*p_rc).baseline_gf_interval;
        (*frame_params).frame_type = (*gf_group)
            .frame_type[(*cpi).gf_frame_index as usize];
        av1_setup_target_rate(cpi);
        (*rc).high_source_sad = 0 as core::ffi::c_int;
        (*rc).frame_source_sad = UINT64_MAX as uint64_t;
        (*rc).avg_source_sad = 0 as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_second_pass_params(
    mut cpi: *mut AV1_COMP,
    frame_params: *mut EncodeFrameParams,
    mut frame_flags: core::ffi::c_uint,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    if is_one_pass_rt_lag_params(cpi) != 0 {
        get_one_pass_rt_lag_params(cpi, frame_flags, frame_params);
        return;
    }
    if (*cpi).ext_ratectrl.ready != 0
        && (*cpi).ext_ratectrl.funcs.rc_type as core::ffi::c_uint
            & AOM_RC_GOP as core::ffi::c_int as core::ffi::c_uint
            != 0 as core::ffi::c_uint
        && ((*cpi).ext_ratectrl.funcs.get_gop_decision).is_some()
    {
        (*frame_params).show_frame = !((*gf_group)
            .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
            == ARF_UPDATE as core::ffi::c_int
            || (*gf_group).update_type[(*cpi).gf_frame_index as usize]
                as core::ffi::c_int == INTNL_ARF_UPDATE as core::ffi::c_int)
            as core::ffi::c_int;
    }
    if (*cpi).use_ducky_encode != 0
        && (*cpi).ducky_encode_info.frame_info.gop_mode as core::ffi::c_int
            == DUCKY_ENCODE_GOP_MODE_RCL as core::ffi::c_int
    {
        (*frame_params).frame_type = (*gf_group)
            .frame_type[(*cpi).gf_frame_index as usize];
        (*frame_params).show_frame = !((*gf_group)
            .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
            == ARF_UPDATE as core::ffi::c_int
            || (*gf_group).update_type[(*cpi).gf_frame_index as usize]
                as core::ffi::c_int == INTNL_ARF_UPDATE as core::ffi::c_int)
            as core::ffi::c_int;
        if (*cpi).gf_frame_index as core::ffi::c_int == 0 as core::ffi::c_int {
            av1_tf_info_reset(&raw mut (*(*cpi).ppi).tf_info);
            av1_tf_info_filtering(
                &raw mut (*(*cpi).ppi).tf_info,
                cpi as *mut AV1_COMP,
                gf_group,
            );
        }
        return;
    }
    if (*cpi).common.current_frame.frame_number == 0 as core::ffi::c_uint
        && ((*cpi).ext_ratectrl.funcs.send_firstpass_stats).is_some()
    {
        let codec_status: aom_codec_err_t = av1_extrc_send_firstpass_stats(
            &raw mut (*cpi).ext_ratectrl,
            &raw mut (*(*cpi).ppi).twopass.firstpass_info,
        ) as aom_codec_err_t;
        if codec_status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            aom_internal_error(
                (*cpi).common.error,
                codec_status,
                b"av1_extrc_send_firstpass_stats() failed\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    let start_pos: *const FIRSTPASS_STATS = (*cpi).twopass_frame.stats_in;
    let mut update_total_stats: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_stat_consumption_stage(cpi) != 0 && ((*cpi).twopass_frame.stats_in).is_null() {
        return;
    }
    let frames_to_next_forced_key: core::ffi::c_int = detect_app_forced_key(cpi)
        as core::ffi::c_int;
    if frames_to_next_forced_key == 0 as core::ffi::c_int {
        (*rc).frames_to_key = 0 as core::ffi::c_int;
        frame_flags &= FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint;
    } else if frames_to_next_forced_key > 0 as core::ffi::c_int
        && frames_to_next_forced_key < (*rc).frames_to_key
    {
        (*rc).frames_to_key = frames_to_next_forced_key;
    }
    let update_type: core::ffi::c_int = (*gf_group)
        .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int;
    (*frame_params).frame_type = (*gf_group).frame_type[(*cpi).gf_frame_index as usize];
    if ((*cpi).gf_frame_index as core::ffi::c_int) < (*gf_group).size
        && frame_flags & FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint == 0
    {
        av1_setup_target_rate(cpi);
        if update_type == ARF_UPDATE as core::ffi::c_int
            || update_type == INTNL_ARF_UPDATE as core::ffi::c_int
        {
            let this_frame_ptr: *const FIRSTPASS_STATS = read_frame_stats(
                twopass,
                &raw mut (*cpi).twopass_frame,
                (*gf_group).arf_src_offset[(*cpi).gf_frame_index as usize]
                    as core::ffi::c_int,
            ) as *const FIRSTPASS_STATS;
            set_twopass_params_based_on_fp_stats(cpi, this_frame_ptr);
            return;
        }
    }
    if (*oxcf).rc_cfg.mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        (*rc).active_worst_quality = (*oxcf).rc_cfg.cq_level;
    }
    if (*cpi).gf_frame_index as core::ffi::c_int == (*gf_group).size {
        if (*(*cpi).ppi).lap_enabled != 0
            && (*(*cpi).ppi).p_rc.enable_scenecut_detection != 0
        {
            let num_frames_to_detect_scenecut: core::ffi::c_int = MAX_GF_LENGTH_LAP
                + 1 as core::ffi::c_int;
            let frames_to_key: core::ffi::c_int = define_kf_interval(
                cpi,
                &raw mut (*twopass).firstpass_info,
                num_frames_to_detect_scenecut,
                0 as core::ffi::c_int,
            ) as core::ffi::c_int;
            if frames_to_key != -(1 as core::ffi::c_int) {
                (*rc).frames_to_key = if (*rc).frames_to_key < frames_to_key {
                    (*rc).frames_to_key
                } else {
                    frames_to_key
                };
            }
        }
    }
    let mut this_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    memset(
        &raw mut this_frame as *mut FIRSTPASS_STATS as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    );
    if is_stat_consumption_stage(cpi) != 0 {
        if ((*cpi).gf_frame_index as core::ffi::c_int) < (*gf_group).size
            || (*rc).frames_to_key == 0 as core::ffi::c_int
        {
            process_first_pass_stats(cpi, &raw mut this_frame);
            update_total_stats = 1 as core::ffi::c_int;
        }
    } else {
        (*rc).active_worst_quality = (*oxcf).rc_cfg.cq_level;
    }
    let mut this_frame_copy: FIRSTPASS_STATS = FIRSTPASS_STATS {
        frame: 0.,
        weight: 0.,
        intra_error: 0.,
        frame_avg_wavelet_energy: 0.,
        coded_error: 0.,
        sr_coded_error: 0.,
        lt_coded_error: 0.,
        pcnt_inter: 0.,
        pcnt_motion: 0.,
        pcnt_second_ref: 0.,
        pcnt_neutral: 0.,
        intra_skip_pct: 0.,
        inactive_zone_rows: 0.,
        inactive_zone_cols: 0.,
        MVr: 0.,
        mvr_abs: 0.,
        MVc: 0.,
        mvc_abs: 0.,
        MVrv: 0.,
        MVcv: 0.,
        mv_in_out_count: 0.,
        new_mv_count: 0.,
        duration: 0.,
        count: 0.,
        raw_error_stdev: 0.,
        is_flash: 0,
        noise_var: 0.,
        cor_coeff: 0.,
        log_intra_error: 0.,
        log_coded_error: 0.,
    };
    this_frame_copy = this_frame;
    if (*rc).frames_to_key <= 0 as core::ffi::c_int {
        (*frame_params).frame_type = KEY_FRAME as core::ffi::c_int as FRAME_TYPE;
        find_next_key_frame(cpi, &raw mut this_frame);
        this_frame = this_frame_copy;
        (*(*cpi).ppi).tpl_data.prev_gop_arf_disp_order = -(1 as core::ffi::c_int)
            as int64_t;
    }
    if (*rc).frames_to_fwd_kf <= 0 as core::ffi::c_int {
        (*rc).frames_to_fwd_kf = (*oxcf).kf_cfg.fwd_kf_dist;
    }
    if (*cpi).gf_frame_index as core::ffi::c_int == (*gf_group).size {
        av1_tf_info_reset(&raw mut (*(*cpi).ppi).tf_info);
        let mut max_gop_length: core::ffi::c_int = if (*oxcf).gf_cfg.lag_in_frames
            >= 32 as core::ffi::c_int
        {
            if (32 as core::ffi::c_int)
                < (*oxcf).gf_cfg.lag_in_frames
                    - (*oxcf).algo_cfg.arnr_max_frames / 2 as core::ffi::c_int
            {
                32 as core::ffi::c_int
            } else {
                (*oxcf).gf_cfg.lag_in_frames
                    - (*oxcf).algo_cfg.arnr_max_frames / 2 as core::ffi::c_int
            }
        } else {
            MAX_GF_LENGTH_LAP
        };
        if (*oxcf).kf_cfg.fwd_kf_dist > 0 as core::ffi::c_int {
            max_gop_length = if ((*rc).frames_to_fwd_kf + 1 as core::ffi::c_int)
                < max_gop_length
            {
                (*rc).frames_to_fwd_kf + 1 as core::ffi::c_int
            } else {
                max_gop_length
            };
        }
        if (*oxcf).gf_cfg.lag_in_frames == 0 as core::ffi::c_int {
            max_gop_length = (*rc).max_gf_interval;
        }
        max_gop_length = if max_gop_length < (*rc).frames_to_key {
            max_gop_length
        } else {
            (*rc).frames_to_key
        };
        if (*rc).frames_since_key == 0 as core::ffi::c_int
            || (*rc).frames_since_key == 1 as core::ffi::c_int
            || (*p_rc).frames_till_regions_update - (*rc).frames_since_key
                < (*rc).frames_to_key
                && (*p_rc).frames_till_regions_update - (*rc).frames_since_key
                    < max_gop_length + 1 as core::ffi::c_int
        {
            let mut rest_frames: core::ffi::c_int = if (*rc).frames_to_key
                < 150 as core::ffi::c_int
            {
                (*rc).frames_to_key
            } else {
                150 as core::ffi::c_int
            };
            let mut available_frames: core::ffi::c_int = ((*(*twopass).stats_buf_ctx)
                .stats_in_end)
                .offset_from((*cpi).twopass_frame.stats_in) as core::ffi::c_long
                as core::ffi::c_int;
            if (*(*cpi).ppi).lap_enabled == 0 {
                available_frames
                    += ((*rc).frames_since_key == 0 as core::ffi::c_int)
                        as core::ffi::c_int;
            }
            rest_frames = if rest_frames < available_frames {
                rest_frames
            } else {
                available_frames
            };
            (*p_rc).frames_till_regions_update = rest_frames;
            let mut ret: core::ffi::c_int = 0;
            if (*(*cpi).ppi).lap_enabled != 0 {
                mark_flashes(
                    (*(*twopass).stats_buf_ctx).stats_in_start,
                    (*(*twopass).stats_buf_ctx).stats_in_end,
                );
                estimate_noise(
                    (*(*twopass).stats_buf_ctx).stats_in_start,
                    (*(*twopass).stats_buf_ctx).stats_in_end,
                    (*cpi).common.error,
                );
                estimate_coeff(
                    (*(*twopass).stats_buf_ctx).stats_in_start,
                    (*(*twopass).stats_buf_ctx).stats_in_end,
                );
                ret = identify_regions(
                    (*cpi).twopass_frame.stats_in,
                    rest_frames,
                    0 as core::ffi::c_int,
                    ((*p_rc).regions).as_mut_ptr(),
                    &raw mut (*p_rc).num_regions,
                );
            } else {
                ret = identify_regions(
                    ((*cpi).twopass_frame.stats_in)
                        .offset(
                            -(((*rc).frames_since_key == 0 as core::ffi::c_int)
                                as core::ffi::c_int as isize),
                        ),
                    rest_frames,
                    0 as core::ffi::c_int,
                    ((*p_rc).regions).as_mut_ptr(),
                    &raw mut (*p_rc).num_regions,
                );
            }
            if ret == -(1 as core::ffi::c_int) {
                aom_internal_error(
                    (*cpi).common.error,
                    AOM_CODEC_MEM_ERROR,
                    b"Error allocating buffers in identify_regions\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        let mut cur_region_idx: core::ffi::c_int = find_regions_index(
            ((*p_rc).regions).as_mut_ptr(),
            (*p_rc).num_regions,
            (*rc).frames_since_key - (*p_rc).regions_offset,
        );
        if cur_region_idx >= 0 as core::ffi::c_int
            && (*p_rc).regions[cur_region_idx as usize].type_0 as core::ffi::c_uint
                == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
            || (*rc).frames_since_key == 0 as core::ffi::c_int
        {
            (*(*cpi).ppi).gf_state.arf_gf_boost_lst = 0 as core::ffi::c_int;
        }
        let mut need_gf_len: core::ffi::c_int = 1 as core::ffi::c_int;
        if need_gf_len != 0 {
            if (*rc).intervals_till_gf_calculate_due == 0 as core::ffi::c_int
                || 1 as core::ffi::c_int != 0
            {
                calculate_gf_length(cpi, max_gop_length, MAX_NUM_GF_INTERVALS);
            }
            if max_gop_length > 16 as core::ffi::c_int
                && (*oxcf).algo_cfg.enable_tpl_model as core::ffi::c_int != 0
                && (*oxcf).gf_cfg.lag_in_frames >= 32 as core::ffi::c_int
                && (*cpi).sf.tpl_sf.gop_length_decision_method != 3 as core::ffi::c_int
            {
                let mut this_idx: core::ffi::c_int = (*rc).frames_since_key
                    + (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize]
                    - (*p_rc).regions_offset - 1 as core::ffi::c_int;
                let mut this_region: core::ffi::c_int = find_regions_index(
                    ((*p_rc).regions).as_mut_ptr(),
                    (*p_rc).num_regions,
                    this_idx,
                );
                let mut next_region: core::ffi::c_int = find_regions_index(
                    ((*p_rc).regions).as_mut_ptr(),
                    (*p_rc).num_regions,
                    this_idx + 1 as core::ffi::c_int,
                );
                let mut is_last_scenecut: core::ffi::c_int = ((*p_rc)
                    .gf_intervals[(*p_rc).cur_gf_index as usize] >= (*rc).frames_to_key
                    || this_region != -(1 as core::ffi::c_int)
                        && (*p_rc).regions[this_region as usize].type_0
                            as core::ffi::c_uint
                            == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint
                    || next_region != -(1 as core::ffi::c_int)
                        && (*p_rc).regions[next_region as usize].type_0
                            as core::ffi::c_uint
                            == SCENECUT_REGION as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int;
                let mut ori_gf_int: core::ffi::c_int = (*p_rc)
                    .gf_intervals[(*p_rc).cur_gf_index as usize];
                if (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize]
                    > 16 as core::ffi::c_int
                    && (*rc).min_gf_interval <= 16 as core::ffi::c_int
                {
                    define_gf_group(cpi, frame_params, 0 as core::ffi::c_int);
                    av1_tf_info_filtering(
                        &raw mut (*(*cpi).ppi).tf_info,
                        cpi as *mut AV1_COMP,
                        gf_group,
                    );
                    this_frame = this_frame_copy;
                    if is_shorter_gf_interval_better(cpi, frame_params) != 0 {
                        max_gop_length = 16 as core::ffi::c_int;
                        calculate_gf_length(cpi, max_gop_length, 1 as core::ffi::c_int);
                        if is_last_scenecut != 0
                            && ori_gf_int
                                - (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize]
                                < 4 as core::ffi::c_int
                        {
                            (*p_rc).gf_intervals[(*p_rc).cur_gf_index as usize] = ori_gf_int;
                        }
                    }
                }
            }
        }
        define_gf_group(cpi, frame_params, 0 as core::ffi::c_int);
        if (*gf_group).update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
            != ARF_UPDATE as core::ffi::c_int
            && (*rc).frames_since_key > 0 as core::ffi::c_int
        {
            process_first_pass_stats(cpi, &raw mut this_frame);
        }
        define_gf_group(cpi, frame_params, 1 as core::ffi::c_int);
        av1_tf_info_filtering(
            &raw mut (*(*cpi).ppi).tf_info,
            cpi as *mut AV1_COMP,
            gf_group,
        );
        (*rc).frames_till_gf_update_due = (*p_rc).baseline_gf_interval;
    }
    if (*gf_group).update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
        == ARF_UPDATE as core::ffi::c_int
        || (*gf_group).update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
            == INTNL_ARF_UPDATE as core::ffi::c_int
    {
        reset_fpf_position(&raw mut (*cpi).twopass_frame, start_pos);
        let this_frame_ptr_0: *const FIRSTPASS_STATS = read_frame_stats(
            twopass,
            &raw mut (*cpi).twopass_frame,
            (*gf_group).arf_src_offset[(*cpi).gf_frame_index as usize]
                as core::ffi::c_int,
        ) as *const FIRSTPASS_STATS;
        set_twopass_params_based_on_fp_stats(cpi, this_frame_ptr_0);
    } else {
        (*cpi).twopass_frame.this_frame = if update_total_stats != 0 {
            start_pos
        } else {
            0 as *const FIRSTPASS_STATS
        };
    }
    (*frame_params).frame_type = (*gf_group).frame_type[(*cpi).gf_frame_index as usize];
    av1_setup_target_rate(cpi);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_second_pass(mut cpi: *mut AV1_COMP) {
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let frame_info: *mut FRAME_INFO = &raw mut (*cpi).frame_info;
    let mut frame_rate: core::ffi::c_double = 0.;
    let mut stats: *mut FIRSTPASS_STATS = 0 as *mut FIRSTPASS_STATS;
    if ((*(*twopass).stats_buf_ctx).stats_in_end).is_null() {
        return;
    }
    mark_flashes(
        (*(*twopass).stats_buf_ctx).stats_in_start,
        (*(*twopass).stats_buf_ctx).stats_in_end,
    );
    estimate_noise(
        (*(*twopass).stats_buf_ctx).stats_in_start,
        (*(*twopass).stats_buf_ctx).stats_in_end,
        (*cpi).common.error,
    );
    estimate_coeff(
        (*(*twopass).stats_buf_ctx).stats_in_start,
        (*(*twopass).stats_buf_ctx).stats_in_end,
    );
    stats = (*(*twopass).stats_buf_ctx).total_stats;
    *stats = *(*(*twopass).stats_buf_ctx).stats_in_end;
    *(*(*twopass).stats_buf_ctx).total_left_stats = *stats;
    frame_rate = 10000000.0f64 * (*stats).count / (*stats).duration;
    av1_new_framerate(cpi, frame_rate);
    (*twopass).bits_left = ((*stats).duration
        * (*oxcf).rc_cfg.target_bandwidth as core::ffi::c_double / 10000000.0f64)
        as int64_t;
    (*twopass).sr_update_lag = 1 as core::ffi::c_int;
    let avg_error: core::ffi::c_double = (*stats).coded_error
        / (if (*stats).count < 0 as core::ffi::c_int as core::ffi::c_double {
            (*stats).count - 0.000001f64
        } else {
            (*stats).count + 0.000001f64
        });
    let mut s: *const FIRSTPASS_STATS = (*cpi).twopass_frame.stats_in;
    let mut modified_error_total: core::ffi::c_double = 0.0f64;
    (*twopass).modified_error_min = avg_error
        * (*oxcf).rc_cfg.vbrmin_section as core::ffi::c_double
        / 100 as core::ffi::c_int as core::ffi::c_double;
    (*twopass).modified_error_max = avg_error
        * (*oxcf).rc_cfg.vbrmax_section as core::ffi::c_double
        / 100 as core::ffi::c_int as core::ffi::c_double;
    while s < (*(*twopass).stats_buf_ctx).stats_in_end as *const FIRSTPASS_STATS {
        modified_error_total += calculate_modified_err(frame_info, twopass, oxcf, s);
        s = s.offset(1);
    }
    (*twopass).modified_error_left = modified_error_total;
    (*(*cpi).ppi).p_rc.vbr_bits_off_target = 0 as int64_t;
    (*(*cpi).ppi).p_rc.vbr_bits_off_target_fast = 0 as int64_t;
    (*(*cpi).ppi).p_rc.rate_error_estimate = 0 as core::ffi::c_int;
    (*twopass).kf_zeromotion_pct = 100 as core::ffi::c_int;
    (*twopass).last_kfgroup_zeromotion_pct = 100 as core::ffi::c_int;
    (*twopass).bpm_factor = 1.0f64;
    (*twopass).rolling_arf_group_target_bits = 1 as core::ffi::c_int;
    (*twopass).rolling_arf_group_actual_bits = 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_single_pass_lap(mut cpi: *mut AV1_COMP) {
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    if ((*(*twopass).stats_buf_ctx).stats_in_end).is_null() {
        return;
    }
    (*twopass).sr_update_lag = 1 as core::ffi::c_int;
    (*twopass).bits_left = 0 as int64_t;
    (*twopass).modified_error_min = 0.0f64;
    (*twopass).modified_error_max = 0.0f64;
    (*twopass).modified_error_left = 0.0f64;
    (*(*cpi).ppi).p_rc.vbr_bits_off_target = 0 as int64_t;
    (*(*cpi).ppi).p_rc.vbr_bits_off_target_fast = 0 as int64_t;
    (*(*cpi).ppi).p_rc.rate_error_estimate = 0 as core::ffi::c_int;
    (*twopass).kf_zeromotion_pct = 100 as core::ffi::c_int;
    (*twopass).last_kfgroup_zeromotion_pct = 100 as core::ffi::c_int;
    (*twopass).bpm_factor = 1.0f64;
    (*twopass).rolling_arf_group_target_bits = 1 as core::ffi::c_int;
    (*twopass).rolling_arf_group_actual_bits = 1 as core::ffi::c_int;
}
pub const MINQ_ADJ_LIMIT: core::ffi::c_int = 48 as core::ffi::c_int;
pub const MINQ_ADJ_LIMIT_CQ: core::ffi::c_int = 20 as core::ffi::c_int;
pub const HIGH_UNDERSHOOT_RATIO: core::ffi::c_int = 2 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_twopass_postencode_update(mut cpi: *mut AV1_COMP) {
    let twopass: *mut TWO_PASS = &raw mut (*(*cpi).ppi).twopass;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let rc_cfg: *const RateControlCfg = &raw mut (*cpi).oxcf.rc_cfg;
    if is_stat_consumption_stage(cpi) != 0
        && !((*cpi).use_ducky_encode != 0
            && (*cpi).ducky_encode_info.frame_info.gop_mode as core::ffi::c_int
                == DUCKY_ENCODE_GOP_MODE_RCL as core::ffi::c_int)
        && (((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size
            || (*rc).frames_to_key == 0 as core::ffi::c_int)
    {
        let update_type: core::ffi::c_int = (*(*cpi).ppi)
            .gf_group
            .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int;
        if update_type != ARF_UPDATE as core::ffi::c_int
            && update_type != INTNL_ARF_UPDATE as core::ffi::c_int
        {
            let mut this_frame: FIRSTPASS_STATS = FIRSTPASS_STATS {
                frame: 0.,
                weight: 0.,
                intra_error: 0.,
                frame_avg_wavelet_energy: 0.,
                coded_error: 0.,
                sr_coded_error: 0.,
                lt_coded_error: 0.,
                pcnt_inter: 0.,
                pcnt_motion: 0.,
                pcnt_second_ref: 0.,
                pcnt_neutral: 0.,
                intra_skip_pct: 0.,
                inactive_zone_rows: 0.,
                inactive_zone_cols: 0.,
                MVr: 0.,
                mvr_abs: 0.,
                MVc: 0.,
                mvc_abs: 0.,
                MVrv: 0.,
                MVcv: 0.,
                mv_in_out_count: 0.,
                new_mv_count: 0.,
                duration: 0.,
                count: 0.,
                raw_error_stdev: 0.,
                is_flash: 0,
                noise_var: 0.,
                cor_coeff: 0.,
                log_intra_error: 0.,
                log_coded_error: 0.,
            };
            (*cpi).twopass_frame.stats_in = ((*cpi).twopass_frame.stats_in).offset(-1);
            if (*(*cpi).ppi).lap_enabled != 0 {
                input_stats_lap(twopass, &raw mut (*cpi).twopass_frame, &raw mut this_frame);
            } else {
                input_stats(twopass, &raw mut (*cpi).twopass_frame, &raw mut this_frame);
            }
        } else if (*(*cpi).ppi).lap_enabled != 0 {
            (*cpi).twopass_frame.stats_in = (*(*twopass).stats_buf_ctx).stats_in_start;
        }
    }
    (*p_rc).vbr_bits_off_target
        += ((*rc).base_frame_target - (*rc).projected_frame_size) as int64_t;
    (*twopass).bits_left = if (*twopass).bits_left - (*rc).base_frame_target as int64_t
        > 0 as int64_t
    {
        (*twopass).bits_left - (*rc).base_frame_target as int64_t
    } else {
        0 as int64_t
    };
    if (*cpi).do_update_vbr_bits_off_target_fast != 0 {
        (*p_rc).vbr_bits_off_target_fast -= (*rc).frame_level_fast_extra_bits as int64_t;
        (*rc).frame_level_fast_extra_bits = 0 as core::ffi::c_int;
    }
    if (*twopass).rolling_arf_group_target_bits > INT_MAX - (*rc).base_frame_target {
        (*twopass).rolling_arf_group_target_bits = INT_MAX;
    } else {
        (*twopass).rolling_arf_group_target_bits += (*rc).base_frame_target;
    }
    (*twopass).rolling_arf_group_actual_bits += (*rc).projected_frame_size;
    if (*p_rc).total_actual_bits != 0 {
        (*p_rc).rate_error_estimate = ((*p_rc).vbr_bits_off_target * 100 as int64_t
            / (*p_rc).total_actual_bits) as core::ffi::c_int;
        (*p_rc).rate_error_estimate = clamp(
            (*p_rc).rate_error_estimate,
            -(100 as core::ffi::c_int),
            100 as core::ffi::c_int,
        );
    } else {
        (*p_rc).rate_error_estimate = 0 as core::ffi::c_int;
    }
    if (*rc).is_src_frame_alt_ref == 0 {
        let pyramid_level: core::ffi::c_int = (*(*cpi).ppi)
            .gf_group
            .layer_depth[(*cpi).gf_frame_index as usize];
        let mut i: core::ffi::c_int = 0;
        i = pyramid_level;
        while i <= MAX_ARF_LAYERS {
            (*p_rc).active_best_quality[i as usize] = (*cpi)
                .common
                .quant_params
                .base_qindex;
            i += 1;
        }
    }
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        != KEY_FRAME as core::ffi::c_int
    {
        (*twopass).kf_group_bits -= (*rc).base_frame_target as int64_t;
        (*twopass).last_kfgroup_zeromotion_pct = (*twopass).kf_zeromotion_pct;
    }
    (*twopass).kf_group_bits = if (*twopass).kf_group_bits > 0 as int64_t {
        (*twopass).kf_group_bits
    } else {
        0 as int64_t
    };
    if (*rc_cfg).mode as core::ffi::c_uint
        != AOM_Q as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).rc.is_src_frame_alt_ref == 0
        && (*p_rc).rolling_target_bits > 0 as core::ffi::c_int
    {
        let mut minq_adj_limit: core::ffi::c_int = 0;
        let mut maxq_adj_limit: core::ffi::c_int = 0;
        minq_adj_limit = if (*rc_cfg).mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        {
            MINQ_ADJ_LIMIT_CQ
        } else {
            MINQ_ADJ_LIMIT
        };
        maxq_adj_limit = (*rc).worst_quality - (*rc).active_worst_quality;
        if (*rc_cfg).under_shoot_pct < 100 as core::ffi::c_int
            && (*p_rc).rolling_actual_bits < (*p_rc).rolling_target_bits
        {
            let mut pct_error: core::ffi::c_int = ((*p_rc).rolling_target_bits
                - (*p_rc).rolling_actual_bits) * 100 as core::ffi::c_int
                / (*p_rc).rolling_target_bits;
            if pct_error >= (*rc_cfg).under_shoot_pct
                && (*p_rc).rate_error_estimate > 0 as core::ffi::c_int
            {
                (*twopass).extend_minq += 1 as core::ffi::c_int;
            }
            (*twopass).extend_maxq -= 1 as core::ffi::c_int;
        } else if (*rc_cfg).over_shoot_pct < 100 as core::ffi::c_int
            && (*p_rc).rolling_actual_bits > (*p_rc).rolling_target_bits
        {
            let mut pct_error_0: core::ffi::c_int = ((*p_rc).rolling_actual_bits
                - (*p_rc).rolling_target_bits) * 100 as core::ffi::c_int
                / (*p_rc).rolling_target_bits;
            pct_error_0 = clamp(
                pct_error_0,
                0 as core::ffi::c_int,
                100 as core::ffi::c_int,
            );
            if pct_error_0 >= (*rc_cfg).over_shoot_pct
                && (*p_rc).rate_error_estimate < 0 as core::ffi::c_int
            {
                (*twopass).extend_maxq += 1 as core::ffi::c_int;
            }
            (*twopass).extend_minq -= 1 as core::ffi::c_int;
        } else if (*rc).projected_frame_size
            > 2 as core::ffi::c_int * (*rc).base_frame_target
            && (*rc).projected_frame_size
                > 2 as core::ffi::c_int * (*rc).avg_frame_bandwidth
        {
            (*twopass).extend_maxq += 1;
        } else if (*p_rc).rolling_target_bits > (*p_rc).rolling_actual_bits {
            (*twopass).extend_maxq -= 1;
        }
        (*twopass).extend_minq = clamp(
            (*twopass).extend_minq,
            -minq_adj_limit,
            minq_adj_limit,
        );
        (*twopass).extend_maxq = clamp(
            (*twopass).extend_maxq,
            0 as core::ffi::c_int,
            maxq_adj_limit,
        );
        if frame_is_kf_gf_arf(cpi) == 0 && (*cpi).rc.is_src_frame_alt_ref == 0 {
            let mut fast_extra_thresh: core::ffi::c_int = (*rc).base_frame_target
                / HIGH_UNDERSHOOT_RATIO;
            if (*rc).projected_frame_size < fast_extra_thresh {
                (*p_rc).vbr_bits_off_target_fast
                    += (fast_extra_thresh - (*rc).projected_frame_size) as int64_t;
                (*p_rc).vbr_bits_off_target_fast = if (*p_rc).vbr_bits_off_target_fast
                    < 4 as int64_t * (*rc).avg_frame_bandwidth as int64_t
                {
                    (*p_rc).vbr_bits_off_target_fast
                } else {
                    4 as int64_t * (*rc).avg_frame_bandwidth as int64_t
                };
            }
        }
    }
    let frame_probs: *mut FrameProbInfo = &raw mut (*(*cpi).ppi).frame_probs;
    let mut i_0: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut loop_0: core::ffi::c_int = 0;
    loop_0 = 0 as core::ffi::c_int;
    while loop_0 <= (*cpi).num_frame_recode {
        if (*cpi).do_update_frame_probs_txtype[loop_0 as usize] != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            let update_type_0: FRAME_UPDATE_TYPE = get_frame_update_type(
                &raw mut (*(*cpi).ppi).gf_group,
                (*cpi).gf_frame_index as core::ffi::c_int,
            ) as FRAME_UPDATE_TYPE;
            i_0 = 0 as core::ffi::c_int;
            while i_0 < TX_SIZES_ALL as core::ffi::c_int {
                let mut left: core::ffi::c_int = 1024 as core::ffi::c_int;
                j = TX_TYPES as core::ffi::c_int - 1 as core::ffi::c_int;
                while j >= 0 as core::ffi::c_int {
                    let new_prob: core::ffi::c_int = (*cpi)
                        .frame_new_probs[loop_0 as usize]
                        .tx_type_probs[update_type_0 as usize][i_0 as usize][j as usize];
                    let mut prob: core::ffi::c_int = (*frame_probs)
                        .tx_type_probs[update_type_0 as usize][i_0 as usize][j as usize]
                        + new_prob >> 1 as core::ffi::c_int;
                    left -= prob;
                    if j == 0 as core::ffi::c_int {
                        prob += left;
                    }
                    (*frame_probs)
                        .tx_type_probs[update_type_0
                        as usize][i_0 as usize][j as usize] = prob;
                    j -= 1;
                }
                i_0 += 1;
            }
        }
        if (*cpi).do_update_frame_probs_obmc[loop_0 as usize] != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            let update_type_1: FRAME_UPDATE_TYPE = get_frame_update_type(
                &raw mut (*(*cpi).ppi).gf_group,
                (*cpi).gf_frame_index as core::ffi::c_int,
            ) as FRAME_UPDATE_TYPE;
            i_0 = 0 as core::ffi::c_int;
            while i_0 < BLOCK_SIZES_ALL as core::ffi::c_int {
                let new_prob_0: core::ffi::c_int = (*cpi)
                    .frame_new_probs[loop_0 as usize]
                    .obmc_probs[update_type_1 as usize][i_0 as usize];
                (*frame_probs).obmc_probs[update_type_1 as usize][i_0 as usize] = (*frame_probs)
                    .obmc_probs[update_type_1 as usize][i_0 as usize] + new_prob_0
                    >> 1 as core::ffi::c_int;
                i_0 += 1;
            }
        }
        if (*cpi).do_update_frame_probs_warp[loop_0 as usize] != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            let update_type_2: FRAME_UPDATE_TYPE = get_frame_update_type(
                &raw mut (*(*cpi).ppi).gf_group,
                (*cpi).gf_frame_index as core::ffi::c_int,
            ) as FRAME_UPDATE_TYPE;
            let new_prob_1: core::ffi::c_int = (*cpi)
                .frame_new_probs[loop_0 as usize]
                .warped_probs[update_type_2 as usize];
            (*frame_probs).warped_probs[update_type_2 as usize] = (*frame_probs)
                .warped_probs[update_type_2 as usize] + new_prob_1
                >> 1 as core::ffi::c_int;
        }
        if (*cpi).do_update_frame_probs_interpfilter[loop_0 as usize] != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            let update_type_3: FRAME_UPDATE_TYPE = get_frame_update_type(
                &raw mut (*(*cpi).ppi).gf_group,
                (*cpi).gf_frame_index as core::ffi::c_int,
            ) as FRAME_UPDATE_TYPE;
            i_0 = 0 as core::ffi::c_int;
            while i_0
                < (SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 4 as core::ffi::c_int
            {
                let mut left_0: core::ffi::c_int = 1536 as core::ffi::c_int;
                j = SWITCHABLE_FILTERS as core::ffi::c_int - 1 as core::ffi::c_int;
                while j >= 0 as core::ffi::c_int {
                    let new_prob_2: core::ffi::c_int = (*cpi)
                        .frame_new_probs[loop_0 as usize]
                        .switchable_interp_probs[update_type_3
                        as usize][i_0 as usize][j as usize];
                    let mut prob_0: core::ffi::c_int = (*frame_probs)
                        .switchable_interp_probs[update_type_3
                        as usize][i_0 as usize][j as usize] + new_prob_2
                        >> 1 as core::ffi::c_int;
                    left_0 -= prob_0;
                    if j == 0 as core::ffi::c_int {
                        prob_0 += left_0;
                    }
                    (*frame_probs)
                        .switchable_interp_probs[update_type_3
                        as usize][i_0 as usize][j as usize] = prob_0;
                    j -= 1;
                }
                i_0 += 1;
            }
        }
        loop_0 += 1;
    }
    if (*cpi).common.show_frame != 0
        && (*(*cpi).ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
            > 0 as core::ffi::c_int
    {
        (*cpi).framerate = (*cpi).new_framerate;
    }
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;

extern "C" {
    fn memmove(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn log1p(_: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(_: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(_: core::ffi::c_double, _: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(_: core::ffi::c_double) -> core::ffi::c_double;
    fn rint(_: core::ffi::c_double) -> core::ffi::c_double;
}
