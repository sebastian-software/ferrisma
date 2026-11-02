/*
 * Copyright (c) 2025, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/encoder/av1_ext_ratectrl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_init(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
) -> aom_codec_err_t {
    if ext_ratectrl.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    memset(
        ext_ratectrl as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AOM_EXT_RATECTRL>() as size_t,
    );
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_create(
    mut funcs: aom_rc_funcs_t,
    mut ratectrl_config: aom_rc_config_t,
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
) -> aom_codec_err_t {
    let mut rc_status: aom_rc_status_t = AOM_RC_OK;
    let mut rc_firstpass_stats: *mut aom_rc_firstpass_stats_t = 0
        as *mut aom_rc_firstpass_stats_t;
    if ext_ratectrl.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    av1_extrc_delete(ext_ratectrl);
    (*ext_ratectrl).funcs = funcs;
    (*ext_ratectrl).ratectrl_config = ratectrl_config;
    rc_status = ((*ext_ratectrl).funcs.create_model)
        .expect(
            "non-null function pointer",
        )(
        (*ext_ratectrl).funcs.priv_0,
        &raw mut (*ext_ratectrl).ratectrl_config,
        &raw mut (*ext_ratectrl).model,
    );
    if rc_status as core::ffi::c_uint
        == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
    {
        return AOM_CODEC_ERROR;
    }
    rc_firstpass_stats = &raw mut (*ext_ratectrl).rc_firstpass_stats;
    (*rc_firstpass_stats).num_frames = ratectrl_config.show_frame_count;
    (*rc_firstpass_stats).frame_stats = aom_malloc(
        (::core::mem::size_of::<aom_rc_frame_stats_t>() as size_t)
            .wrapping_mul((*rc_firstpass_stats).num_frames as size_t),
    ) as *mut aom_rc_frame_stats_t;
    if ((*rc_firstpass_stats).frame_stats).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    (*ext_ratectrl).ready = 1 as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn gen_rc_firstpass_stats(
    mut stats: *const FIRSTPASS_STATS,
    mut rc_frame_stats: *mut aom_rc_frame_stats_t,
) {
    (*rc_frame_stats).frame = (*stats).frame;
    (*rc_frame_stats).weight = (*stats).weight;
    (*rc_frame_stats).intra_error = (*stats).intra_error;
    (*rc_frame_stats).coded_error = (*stats).coded_error;
    (*rc_frame_stats).sr_coded_error = (*stats).sr_coded_error;
    (*rc_frame_stats).frame_avg_wavelet_energy = (*stats).frame_avg_wavelet_energy;
    (*rc_frame_stats).pcnt_inter = (*stats).pcnt_inter;
    (*rc_frame_stats).pcnt_motion = (*stats).pcnt_motion;
    (*rc_frame_stats).pcnt_second_ref = (*stats).pcnt_second_ref;
    (*rc_frame_stats).pcnt_neutral = (*stats).pcnt_neutral;
    (*rc_frame_stats).intra_skip_pct = (*stats).intra_skip_pct;
    (*rc_frame_stats).inactive_zone_rows = (*stats).inactive_zone_rows;
    (*rc_frame_stats).inactive_zone_cols = (*stats).inactive_zone_cols;
    (*rc_frame_stats).MVr = (*stats).MVr;
    (*rc_frame_stats).mvr_abs = (*stats).mvr_abs;
    (*rc_frame_stats).MVc = (*stats).MVc;
    (*rc_frame_stats).mvc_abs = (*stats).mvc_abs;
    (*rc_frame_stats).MVrv = (*stats).MVrv;
    (*rc_frame_stats).MVcv = (*stats).MVcv;
    (*rc_frame_stats).mv_in_out_count = (*stats).mv_in_out_count;
    (*rc_frame_stats).duration = (*stats).duration;
    (*rc_frame_stats).count = (*stats).count;
    (*rc_frame_stats).new_mv_count = (*stats).new_mv_count;
    (*rc_frame_stats).raw_error_stdev = (*stats).raw_error_stdev;
    (*rc_frame_stats).is_flash = (*stats).is_flash;
    (*rc_frame_stats).noise_var = (*stats).noise_var;
    (*rc_frame_stats).cor_coeff = (*stats).cor_coeff;
    (*rc_frame_stats).log_intra_error = (*stats).log_intra_error;
    (*rc_frame_stats).log_coded_error = (*stats).log_coded_error;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_send_firstpass_stats(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut first_pass_info: *const FIRSTPASS_INFO,
) -> aom_codec_err_t {
    if !first_pass_info.is_null() {} else {
        __assert_fail(
            b"first_pass_info != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            88 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 91],
                [core::ffi::c_char; 91],
            >(
                *b"aom_codec_err_t av1_extrc_send_firstpass_stats(AOM_EXT_RATECTRL *, const FIRSTPASS_INFO *)\0",
            ))
                .as_ptr(),
        );
    };
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            89 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 91],
                [core::ffi::c_char; 91],
            >(
                *b"aom_codec_err_t av1_extrc_send_firstpass_stats(AOM_EXT_RATECTRL *, const FIRSTPASS_INFO *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).ready != 0 {
        let mut rc_status: aom_rc_status_t = AOM_RC_OK;
        let mut rc_firstpass_stats: *mut aom_rc_firstpass_stats_t = &raw mut (*ext_ratectrl)
            .rc_firstpass_stats;
        if (*rc_firstpass_stats).num_frames == (*first_pass_info).stats_buf_size
        {} else {
            __assert_fail(
                b"rc_firstpass_stats->num_frames == first_pass_info->stats_buf_size\0"
                    as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                    as *const core::ffi::c_char,
                94 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"aom_codec_err_t av1_extrc_send_firstpass_stats(AOM_EXT_RATECTRL *, const FIRSTPASS_INFO *)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*rc_firstpass_stats).num_frames {
            gen_rc_firstpass_stats(
                &raw mut *((*first_pass_info).stats_buf).offset(i as isize),
                &raw mut *((*rc_firstpass_stats).frame_stats).offset(i as isize),
            );
            i += 1;
        }
        rc_status = ((*ext_ratectrl).funcs.send_firstpass_stats)
            .expect(
                "non-null function pointer",
            )((*ext_ratectrl).model, rc_firstpass_stats);
        if rc_status as core::ffi::c_uint
            == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
        {
            return AOM_CODEC_ERROR;
        }
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_send_tpl_stats(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut extrc_tpl_gop_stats: *const AomTplGopStats,
) -> aom_codec_err_t {
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            110 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 85],
                [core::ffi::c_char; 85],
            >(
                *b"aom_codec_err_t av1_extrc_send_tpl_stats(AOM_EXT_RATECTRL *, const AomTplGopStats *)\0",
            ))
                .as_ptr(),
        );
    };
    if !extrc_tpl_gop_stats.is_null() {} else {
        __assert_fail(
            b"extrc_tpl_gop_stats != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            111 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 85],
                [core::ffi::c_char; 85],
            >(
                *b"aom_codec_err_t av1_extrc_send_tpl_stats(AOM_EXT_RATECTRL *, const AomTplGopStats *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).ready != 0 && ((*ext_ratectrl).funcs.send_tpl_gop_stats).is_some()
    {
        let mut rc_status: aom_rc_status_t = ((*ext_ratectrl).funcs.send_tpl_gop_stats)
            .expect(
                "non-null function pointer",
            )((*ext_ratectrl).model, extrc_tpl_gop_stats);
        if rc_status as core::ffi::c_uint
            == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
        {
            return AOM_CODEC_ERROR;
        }
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_get_gop_decision(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut gop_decision: *mut aom_rc_gop_decision_t,
) -> aom_codec_err_t {
    let mut rc_status: aom_rc_status_t = AOM_RC_OK;
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            125 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 88],
                [core::ffi::c_char; 88],
            >(
                *b"aom_codec_err_t av1_extrc_get_gop_decision(AOM_EXT_RATECTRL *, aom_rc_gop_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    if !gop_decision.is_null() {} else {
        __assert_fail(
            b"gop_decision != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            126 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 88],
                [core::ffi::c_char; 88],
            >(
                *b"aom_codec_err_t av1_extrc_get_gop_decision(AOM_EXT_RATECTRL *, aom_rc_gop_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).funcs.rc_type as core::ffi::c_uint
        & AOM_RC_GOP as core::ffi::c_int as core::ffi::c_uint == 0 as core::ffi::c_uint
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    rc_status = ((*ext_ratectrl).funcs.get_gop_decision)
        .expect("non-null function pointer")((*ext_ratectrl).model, gop_decision);
    if rc_status as core::ffi::c_uint
        == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
    {
        return AOM_CODEC_ERROR;
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_get_encodeframe_decision(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut gop_index: core::ffi::c_int,
    mut encode_frame_decision: *mut aom_rc_encodeframe_decision_t,
) -> aom_codec_err_t {
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            141 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 109],
                [core::ffi::c_char; 109],
            >(
                *b"aom_codec_err_t av1_extrc_get_encodeframe_decision(AOM_EXT_RATECTRL *, int, aom_rc_encodeframe_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).ready != 0
        && (*ext_ratectrl).funcs.rc_type as core::ffi::c_uint
            & AOM_RC_QP as core::ffi::c_int as core::ffi::c_uint
            != 0 as core::ffi::c_uint
    {} else {
        __assert_fail(
            b"ext_ratectrl->ready && (ext_ratectrl->funcs.rc_type & AOM_RC_QP) != 0\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            142 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 109],
                [core::ffi::c_char; 109],
            >(
                *b"aom_codec_err_t av1_extrc_get_encodeframe_decision(AOM_EXT_RATECTRL *, int, aom_rc_encodeframe_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    if !encode_frame_decision.is_null() {} else {
        __assert_fail(
            b"encode_frame_decision != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            143 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 109],
                [core::ffi::c_char; 109],
            >(
                *b"aom_codec_err_t av1_extrc_get_encodeframe_decision(AOM_EXT_RATECTRL *, int, aom_rc_encodeframe_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut rc_status: aom_rc_status_t = ((*ext_ratectrl).funcs.get_encodeframe_decision)
        .expect(
            "non-null function pointer",
        )((*ext_ratectrl).model, gop_index, encode_frame_decision);
    if rc_status as core::ffi::c_uint
        == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
    {
        return AOM_CODEC_ERROR;
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_update_encodeframe_result(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut bit_count: int64_t,
    mut actual_encoding_qindex: core::ffi::c_int,
) -> aom_codec_err_t {
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            155 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 86],
                [core::ffi::c_char; 86],
            >(
                *b"aom_codec_err_t av1_extrc_update_encodeframe_result(AOM_EXT_RATECTRL *, int64_t, int)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).ready != 0 {
        let mut rc_status: aom_rc_status_t = AOM_RC_OK;
        let mut encode_frame_result: aom_rc_encodeframe_result_t = aom_rc_encodeframe_result {
            bit_count: 0,
            actual_encoding_qindex: 0,
        };
        encode_frame_result.bit_count = bit_count;
        encode_frame_result.actual_encoding_qindex = actual_encoding_qindex;
        rc_status = ((*ext_ratectrl).funcs.update_encodeframe_result)
            .expect(
                "non-null function pointer",
            )((*ext_ratectrl).model, &raw mut encode_frame_result);
        if rc_status as core::ffi::c_uint
            == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
        {
            return AOM_CODEC_ERROR;
        }
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_get_key_frame_decision(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
    mut key_frame_decision: *mut aom_rc_key_frame_decision_t,
) -> aom_codec_err_t {
    if !ext_ratectrl.is_null() {} else {
        __assert_fail(
            b"ext_ratectrl != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/av1_ext_ratectrl.c\0" as *const u8
                as *const core::ffi::c_char,
            173 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 100],
                [core::ffi::c_char; 100],
            >(
                *b"aom_codec_err_t av1_extrc_get_key_frame_decision(AOM_EXT_RATECTRL *, aom_rc_key_frame_decision_t *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ext_ratectrl).ready == 0
        || (*ext_ratectrl).funcs.rc_type as core::ffi::c_uint
            & AOM_RC_GOP as core::ffi::c_int as core::ffi::c_uint
            == 0 as core::ffi::c_uint
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut rc_status: aom_rc_status_t = ((*ext_ratectrl).funcs.get_key_frame_decision)
        .expect("non-null function pointer")((*ext_ratectrl).model, key_frame_decision);
    return (if rc_status as core::ffi::c_uint
        == AOM_RC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        AOM_CODEC_OK as core::ffi::c_int
    } else {
        AOM_CODEC_ERROR as core::ffi::c_int
    }) as aom_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extrc_delete(
    mut ext_ratectrl: *mut AOM_EXT_RATECTRL,
) -> aom_codec_err_t {
    if ext_ratectrl.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*ext_ratectrl).ready != 0 {
        let mut rc_status: aom_rc_status_t = ((*ext_ratectrl).funcs.delete_model)
            .expect("non-null function pointer")((*ext_ratectrl).model);
        if rc_status as core::ffi::c_uint
            == AOM_RC_ERROR as core::ffi::c_int as core::ffi::c_uint
        {
            return AOM_CODEC_ERROR;
        }
        aom_free(
            (*ext_ratectrl).rc_firstpass_stats.frame_stats as *mut core::ffi::c_void,
        );
        aom_free((*ext_ratectrl).sb_params_list as *mut core::ffi::c_void);
    }
    return av1_extrc_init(ext_ratectrl);
}

extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
