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

/*!\file
 * \brief Provides the high level interface to wrap encoder algorithms.
 *
 */

// Mechanically translated from upstream/aom/aom/src/aom_encoder.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type aom_metadata_array;
    pub type aom_codec_alg_priv;
    fn aom_codec_destroy(ctx: *mut aom_codec_ctx_t) -> aom_codec_err_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type aom_img_fmt = core::ffi::c_uint;
pub const AOM_IMG_FMT_I44416: aom_img_fmt = 2310;
pub const AOM_IMG_FMT_I42216: aom_img_fmt = 2309;
pub const AOM_IMG_FMT_YV1216: aom_img_fmt = 2817;
pub const AOM_IMG_FMT_I42016: aom_img_fmt = 2306;
pub const AOM_IMG_FMT_NV12: aom_img_fmt = 263;
pub const AOM_IMG_FMT_I444: aom_img_fmt = 262;
pub const AOM_IMG_FMT_I422: aom_img_fmt = 261;
pub const AOM_IMG_FMT_AOMI420: aom_img_fmt = 260;
pub const AOM_IMG_FMT_AOMYV12: aom_img_fmt = 771;
pub const AOM_IMG_FMT_I420: aom_img_fmt = 258;
pub const AOM_IMG_FMT_YV12: aom_img_fmt = 769;
pub const AOM_IMG_FMT_NONE: aom_img_fmt = 0;
pub type aom_img_fmt_t = aom_img_fmt;
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
pub type aom_metadata_array_t = aom_metadata_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_image {
    pub fmt: aom_img_fmt_t,
    pub cp: aom_color_primaries_t,
    pub tc: aom_transfer_characteristics_t,
    pub mc: aom_matrix_coefficients_t,
    pub monochrome: core::ffi::c_int,
    pub csp: aom_chroma_sample_position_t,
    pub range: aom_color_range_t,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub bit_depth: core::ffi::c_uint,
    pub d_w: core::ffi::c_uint,
    pub d_h: core::ffi::c_uint,
    pub r_w: core::ffi::c_uint,
    pub r_h: core::ffi::c_uint,
    pub x_chroma_shift: core::ffi::c_uint,
    pub y_chroma_shift: core::ffi::c_uint,
    pub planes: [*mut core::ffi::c_uchar; 3],
    pub stride: [core::ffi::c_int; 3],
    pub sz: size_t,
    pub bps: core::ffi::c_int,
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub user_priv: *mut core::ffi::c_void,
    pub img_data: *mut core::ffi::c_uchar,
    pub img_data_owner: core::ffi::c_int,
    pub self_allocd: core::ffi::c_int,
    pub metadata: *mut aom_metadata_array_t,
    pub fb_priv: *mut core::ffi::c_void,
}
pub type aom_image_t = aom_image;
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
pub type aom_codec_caps_t = core::ffi::c_long;
pub type aom_codec_flags_t = core::ffi::c_long;
pub type aom_codec_pts_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_iface {
    pub name: *const core::ffi::c_char,
    pub abi_version: core::ffi::c_int,
    pub caps: aom_codec_caps_t,
    pub init: aom_codec_init_fn_t,
    pub destroy: aom_codec_destroy_fn_t,
    pub ctrl_maps: *const aom_codec_ctrl_fn_map_t,
    pub dec: aom_codec_dec_iface,
    pub enc: aom_codec_enc_iface,
    pub set_option: aom_codec_set_option_fn_t,
}
pub type aom_codec_set_option_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const core::ffi::c_char,
        *const core::ffi::c_char,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_alg_priv_t = aom_codec_alg_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_enc_iface {
    pub cfg_count: core::ffi::c_int,
    pub cfgs: *const aom_codec_enc_cfg_t,
    pub encode: aom_codec_encode_fn_t,
    pub get_cx_data: aom_codec_get_cx_data_fn_t,
    pub cfg_set: aom_codec_enc_config_set_fn_t,
    pub get_glob_hdrs: aom_codec_get_global_headers_fn_t,
    pub get_preview: aom_codec_get_preview_frame_fn_t,
}
pub type aom_codec_get_preview_frame_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> *mut aom_image_t,
>;
pub type aom_codec_get_global_headers_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> *mut aom_fixed_buf_t,
>;
pub type aom_fixed_buf_t = aom_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
}
pub type aom_codec_enc_config_set_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_codec_enc_cfg_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_enc_cfg_t = aom_codec_enc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_enc_cfg {
    pub g_usage: core::ffi::c_uint,
    pub g_threads: core::ffi::c_uint,
    pub g_profile: core::ffi::c_uint,
    pub g_w: core::ffi::c_uint,
    pub g_h: core::ffi::c_uint,
    pub g_limit: core::ffi::c_uint,
    pub g_forced_max_frame_width: core::ffi::c_uint,
    pub g_forced_max_frame_height: core::ffi::c_uint,
    pub g_bit_depth: aom_bit_depth_t,
    pub g_input_bit_depth: core::ffi::c_uint,
    pub g_timebase: aom_rational,
    pub g_error_resilient: aom_codec_er_flags_t,
    pub g_pass: aom_enc_pass,
    pub g_lag_in_frames: core::ffi::c_uint,
    pub rc_dropframe_thresh: core::ffi::c_uint,
    pub rc_resize_mode: core::ffi::c_uint,
    pub rc_resize_denominator: core::ffi::c_uint,
    pub rc_resize_kf_denominator: core::ffi::c_uint,
    pub rc_superres_mode: aom_superres_mode,
    pub rc_superres_denominator: core::ffi::c_uint,
    pub rc_superres_kf_denominator: core::ffi::c_uint,
    pub rc_superres_qthresh: core::ffi::c_uint,
    pub rc_superres_kf_qthresh: core::ffi::c_uint,
    pub rc_end_usage: aom_rc_mode,
    pub rc_twopass_stats_in: aom_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: aom_fixed_buf_t,
    pub rc_target_bitrate: core::ffi::c_uint,
    pub rc_min_quantizer: core::ffi::c_uint,
    pub rc_max_quantizer: core::ffi::c_uint,
    pub rc_undershoot_pct: core::ffi::c_uint,
    pub rc_overshoot_pct: core::ffi::c_uint,
    pub rc_buf_sz: core::ffi::c_uint,
    pub rc_buf_initial_sz: core::ffi::c_uint,
    pub rc_buf_optimal_sz: core::ffi::c_uint,
    pub rc_2pass_vbr_bias_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_minsection_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_maxsection_pct: core::ffi::c_uint,
    pub fwd_kf_enabled: core::ffi::c_int,
    pub kf_mode: aom_kf_mode,
    pub kf_min_dist: core::ffi::c_uint,
    pub kf_max_dist: core::ffi::c_uint,
    pub sframe_dist: core::ffi::c_uint,
    pub sframe_mode: core::ffi::c_uint,
    pub large_scale_tile: core::ffi::c_uint,
    pub monochrome: core::ffi::c_uint,
    pub full_still_picture_hdr: core::ffi::c_uint,
    pub save_as_annexb: core::ffi::c_uint,
    pub tile_width_count: core::ffi::c_int,
    pub tile_height_count: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub use_fixed_qp_offsets: core::ffi::c_uint,
    pub fixed_qp_offsets: [core::ffi::c_int; 5],
    pub encoder_cfg: cfg_options_t,
}
pub type cfg_options_t = cfg_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_options {
    pub init_by_cfg_file: core::ffi::c_uint,
    pub super_block_size: core::ffi::c_uint,
    pub max_partition_size: core::ffi::c_uint,
    pub min_partition_size: core::ffi::c_uint,
    pub disable_ab_partition_type: core::ffi::c_uint,
    pub disable_rect_partition_type: core::ffi::c_uint,
    pub disable_1to4_partition_type: core::ffi::c_uint,
    pub disable_flip_idtx: core::ffi::c_uint,
    pub disable_cdef: core::ffi::c_uint,
    pub disable_lr: core::ffi::c_uint,
    pub disable_obmc: core::ffi::c_uint,
    pub disable_warp_motion: core::ffi::c_uint,
    pub disable_global_motion: core::ffi::c_uint,
    pub disable_dist_wtd_comp: core::ffi::c_uint,
    pub disable_diff_wtd_comp: core::ffi::c_uint,
    pub disable_inter_intra_comp: core::ffi::c_uint,
    pub disable_masked_comp: core::ffi::c_uint,
    pub disable_one_sided_comp: core::ffi::c_uint,
    pub disable_palette: core::ffi::c_uint,
    pub disable_intrabc: core::ffi::c_uint,
    pub disable_cfl: core::ffi::c_uint,
    pub disable_smooth_intra: core::ffi::c_uint,
    pub disable_filter_intra: core::ffi::c_uint,
    pub disable_dual_filter: core::ffi::c_uint,
    pub disable_intra_angle_delta: core::ffi::c_uint,
    pub disable_intra_edge_filter: core::ffi::c_uint,
    pub disable_tx_64x64: core::ffi::c_uint,
    pub disable_smooth_inter_intra: core::ffi::c_uint,
    pub disable_inter_inter_wedge: core::ffi::c_uint,
    pub disable_inter_intra_wedge: core::ffi::c_uint,
    pub disable_paeth_intra: core::ffi::c_uint,
    pub disable_trellis_quant: core::ffi::c_uint,
    pub disable_ref_frame_mv: core::ffi::c_uint,
    pub reduced_reference_set: core::ffi::c_uint,
    pub reduced_tx_type_set: core::ffi::c_uint,
}
pub type aom_kf_mode = core::ffi::c_uint;
pub const AOM_KF_DISABLED: aom_kf_mode = 0;
pub const AOM_KF_AUTO: aom_kf_mode = 1;
pub const AOM_KF_FIXED: aom_kf_mode = 0;
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
pub type aom_codec_er_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_codec_get_cx_data_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_iter_t,
    ) -> *const aom_codec_cx_pkt_t,
>;
pub type aom_codec_iter_t = *const core::ffi::c_void;
pub type aom_codec_cx_pkt_t = aom_codec_cx_pkt;
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
pub type aom_codec_encode_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_image_t,
        aom_codec_pts_t,
        core::ffi::c_ulong,
        aom_enc_frame_flags_t,
    ) -> aom_codec_err_t,
>;
pub type aom_enc_frame_flags_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_dec_iface {
    pub peek_si: aom_codec_peek_si_fn_t,
    pub get_si: aom_codec_get_si_fn_t,
    pub decode: aom_codec_decode_fn_t,
    pub get_frame: aom_codec_get_frame_fn_t,
    pub set_fb_fn: aom_codec_set_fb_fn_t,
}
pub type aom_codec_set_fb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        aom_get_frame_buffer_cb_fn_t,
        aom_release_frame_buffer_cb_fn_t,
        *mut core::ffi::c_void,
    ) -> aom_codec_err_t,
>;
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
pub type aom_codec_get_frame_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_iter_t,
    ) -> *mut aom_image_t,
>;
pub type aom_codec_decode_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const uint8_t,
        size_t,
        *mut core::ffi::c_void,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_get_si_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_stream_info_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_stream_info_t = aom_codec_stream_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_stream_info {
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub is_kf: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
    pub number_temporal_layers: core::ffi::c_uint,
    pub is_annexb: core::ffi::c_uint,
}
pub type aom_codec_peek_si_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        size_t,
        *mut aom_codec_stream_info_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_ctrl_fn_map_t = aom_codec_ctrl_fn_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_ctrl_fn_map {
    pub ctrl_id: core::ffi::c_int,
    pub fn_0: aom_codec_control_fn_t,
}
pub type aom_codec_control_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut __va_list_tag,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_destroy_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> aom_codec_err_t,
>;
pub type aom_codec_init_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_ctx_t) -> aom_codec_err_t,
>;
pub type aom_codec_ctx_t = aom_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_ctx {
    pub name: *const core::ffi::c_char,
    pub iface: *const aom_codec_iface_t,
    pub err: aom_codec_err_t,
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub config: C2RustUnnamed_2,
    pub priv_0: *mut aom_codec_priv_t,
}
pub type aom_codec_priv_t = aom_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_priv {
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub enc: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub cx_data_dst_buf: aom_fixed_buf_t,
    pub cx_data_pad_before: core::ffi::c_uint,
    pub cx_data_pad_after: core::ffi::c_uint,
    pub cx_data_pkt: aom_codec_cx_pkt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dec: *const aom_codec_dec_cfg,
    pub enc: *const aom_codec_enc_cfg,
    pub raw: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_dec_cfg {
    pub threads: core::ffi::c_uint,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub allow_lowbitdepth: core::ffi::c_uint,
}
pub type aom_codec_iface_t = aom_codec_iface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_pkt_list {
    pub cnt: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
    pub pkts: [aom_codec_cx_pkt; 1],
}
pub const FE_INVALID: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const FE_DIVBYZERO: core::ffi::c_int = 0x4 as core::ffi::c_int;
pub const FE_OVERFLOW: core::ffi::c_int = 0x8 as core::ffi::c_int;
pub const FE_UNDERFLOW: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const FE_INEXACT: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const FE_ALL_EXCEPT: core::ffi::c_int = FE_INEXACT | FE_DIVBYZERO | FE_UNDERFLOW
    | FE_OVERFLOW | FE_INVALID;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_CODEC_INTERNAL_ABI_VERSION: core::ffi::c_int = 7 as core::ffi::c_int;
pub const AOM_IMAGE_ABI_VERSION: core::ffi::c_int = 9 as core::ffi::c_int;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AOM_CODEC_ABI_VERSION: core::ffi::c_int = 7 as core::ffi::c_int
    + AOM_IMAGE_ABI_VERSION;
pub const AOM_CODEC_CAP_ENCODER: core::ffi::c_int = 0x2 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_alg_priv(
    mut ctx: *mut aom_codec_ctx_t,
) -> *mut aom_codec_alg_priv_t {
    return (*ctx).priv_0 as *mut aom_codec_alg_priv_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_enc_init_ver(
    mut ctx: *mut aom_codec_ctx_t,
    mut iface: *const aom_codec_iface_t,
    mut cfg: *const aom_codec_enc_cfg_t,
    mut flags: aom_codec_flags_t,
    mut ver: core::ffi::c_int,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    let aom_encoder_abi_version_25: core::ffi::c_int = 25 as core::ffi::c_int;
    if ver != AOM_ENCODER_ABI_VERSION && ver != aom_encoder_abi_version_25 {
        res = AOM_CODEC_ABI_MISMATCH;
    } else if ctx.is_null() || iface.is_null() || cfg.is_null() {
        res = AOM_CODEC_INVALID_PARAM;
    } else if (*iface).abi_version != AOM_CODEC_INTERNAL_ABI_VERSION {
        res = AOM_CODEC_ABI_MISMATCH;
    } else if (*iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
        res = AOM_CODEC_INCAPABLE;
    } else if flags & AOM_CODEC_USE_PSNR as aom_codec_flags_t != 0
        && (*iface).caps & AOM_CODEC_CAP_PSNR as aom_codec_caps_t == 0
    {
        res = AOM_CODEC_INCAPABLE;
    } else if flags & AOM_CODEC_USE_HIGHBITDEPTH as aom_codec_flags_t != 0
        && (*iface).caps & AOM_CODEC_CAP_HIGHBITDEPTH as aom_codec_caps_t == 0
    {
        res = AOM_CODEC_INCAPABLE;
    } else if (*cfg).g_bit_depth as core::ffi::c_uint > 8 as core::ffi::c_uint
        && flags & AOM_CODEC_USE_HIGHBITDEPTH as aom_codec_flags_t
            == 0 as aom_codec_flags_t
    {
        res = AOM_CODEC_INVALID_PARAM;
        (*ctx).err_detail = b"High bit-depth used without the AOM_CODEC_USE_HIGHBITDEPTH flag.\0"
            as *const u8 as *const core::ffi::c_char;
    } else {
        (*ctx).iface = iface;
        (*ctx).name = (*iface).name;
        (*ctx).priv_0 = 0 as *mut aom_codec_priv_t;
        (*ctx).init_flags = flags;
        (*ctx).config.enc = cfg as *const aom_codec_enc_cfg;
        res = ((*(*ctx).iface).init).expect("non-null function pointer")(ctx);
        if res as u64 != 0 {
            (*ctx).err_detail = if !((*ctx).priv_0).is_null() {
                (*(*ctx).priv_0).err_detail
            } else {
                0 as *const core::ffi::c_char
            };
            aom_codec_destroy(ctx);
        }
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as core::ffi::c_uint
    } else {
        res as core::ffi::c_uint
    }) as aom_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_enc_config_default(
    mut iface: *const aom_codec_iface_t,
    mut cfg: *mut aom_codec_enc_cfg_t,
    mut usage: core::ffi::c_uint,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    if iface.is_null() || cfg.is_null() {
        res = AOM_CODEC_INVALID_PARAM;
    } else if (*iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
        res = AOM_CODEC_INCAPABLE;
    } else {
        res = AOM_CODEC_INVALID_PARAM;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*iface).enc.cfg_count {
            if (*((*iface).enc.cfgs).offset(i as isize)).g_usage == usage {
                *cfg = *((*iface).enc.cfgs).offset(i as isize);
                res = AOM_CODEC_OK;
                memset(
                    &raw mut (*cfg).encoder_cfg as *mut cfg_options_t
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<cfg_options_t>() as size_t,
                );
                (*cfg).encoder_cfg.super_block_size = 0 as core::ffi::c_uint;
                (*cfg).encoder_cfg.max_partition_size = 128 as core::ffi::c_uint;
                (*cfg).encoder_cfg.min_partition_size = 4 as core::ffi::c_uint;
                (*cfg).encoder_cfg.disable_trellis_quant = 3 as core::ffi::c_uint;
                break;
            } else {
                i += 1;
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_encode(
    mut ctx: *mut aom_codec_ctx_t,
    mut img: *const aom_image_t,
    mut pts: aom_codec_pts_t,
    mut duration: core::ffi::c_ulong,
    mut flags: aom_enc_frame_flags_t,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    if ctx.is_null() || !img.is_null() && duration == 0 {
        res = AOM_CODEC_INVALID_PARAM;
    } else if ((*ctx).iface).is_null() || ((*ctx).priv_0).is_null() {
        res = AOM_CODEC_ERROR;
    } else if (*(*ctx).iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
        res = AOM_CODEC_INCAPABLE;
    } else if !img.is_null()
        && ((*img).fmt as core::ffi::c_uint
            & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0 as core::ffi::c_uint)
            as core::ffi::c_int
            != ((*ctx).init_flags & AOM_CODEC_USE_HIGHBITDEPTH as aom_codec_flags_t
                != 0 as aom_codec_flags_t) as core::ffi::c_int
    {
        res = AOM_CODEC_INVALID_PARAM;
        (*(*ctx).priv_0).err_detail = b"img is high bit-depth and codec was not initialized with AOM_CODEC_USE_HIGHBITDEPTH, or vice versa.\0"
            as *const u8 as *const core::ffi::c_char;
    } else if duration > UINT32_MAX as core::ffi::c_ulong {
        res = AOM_CODEC_INVALID_PARAM;
    } else {
        let float_excepts: core::ffi::c_int = feenableexcept(
            FE_DIVBYZERO | FE_UNDERFLOW | FE_OVERFLOW,
        ) as core::ffi::c_int;
        res = ((*(*ctx).iface).enc.encode)
            .expect(
                "non-null function pointer",
            )(get_alg_priv(ctx), img, pts, duration, flags);
        if float_excepts != -(1 as core::ffi::c_int) {
            fedisableexcept(FE_ALL_EXCEPT);
            feenableexcept(float_excepts);
        }
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as core::ffi::c_uint
    } else {
        res as core::ffi::c_uint
    }) as aom_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_get_cx_data(
    mut ctx: *mut aom_codec_ctx_t,
    mut iter: *mut aom_codec_iter_t,
) -> *const aom_codec_cx_pkt_t {
    let mut pkt: *const aom_codec_cx_pkt_t = 0 as *const aom_codec_cx_pkt_t;
    if !ctx.is_null() {
        if iter.is_null() {
            (*ctx).err = AOM_CODEC_INVALID_PARAM;
        } else if ((*ctx).iface).is_null() || ((*ctx).priv_0).is_null() {
            (*ctx).err = AOM_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
            (*ctx).err = AOM_CODEC_INCAPABLE;
        } else {
            pkt = ((*(*ctx).iface).enc.get_cx_data)
                .expect("non-null function pointer")(get_alg_priv(ctx), iter);
        }
    }
    if !pkt.is_null()
        && (*pkt).kind as core::ffi::c_uint
            == AOM_CODEC_CX_FRAME_PKT as core::ffi::c_int as core::ffi::c_uint
    {
        let priv_0: *mut aom_codec_priv_t = (*ctx).priv_0;
        let dst_buf: *mut core::ffi::c_char = (*priv_0).enc.cx_data_dst_buf.buf
            as *mut core::ffi::c_char;
        if !dst_buf.is_null() && (*pkt).data.raw.buf != dst_buf as *mut core::ffi::c_void
            && ((*pkt).data.raw.sz)
                .wrapping_add((*priv_0).enc.cx_data_pad_before as size_t)
                .wrapping_add((*priv_0).enc.cx_data_pad_after as size_t)
                <= (*priv_0).enc.cx_data_dst_buf.sz
        {
            let mut modified_pkt: *mut aom_codec_cx_pkt_t = &raw mut (*priv_0)
                .enc
                .cx_data_pkt;
            memcpy(
                dst_buf.offset((*priv_0).enc.cx_data_pad_before as isize)
                    as *mut core::ffi::c_void,
                (*pkt).data.raw.buf,
                (*pkt).data.raw.sz,
            );
            *modified_pkt = *pkt;
            (*modified_pkt).data.raw.buf = dst_buf as *mut core::ffi::c_void;
            (*modified_pkt).data.raw.sz = ((*modified_pkt).data.raw.sz)
                .wrapping_add(
                    ((*priv_0).enc.cx_data_pad_before)
                        .wrapping_add((*priv_0).enc.cx_data_pad_after) as size_t,
                );
            pkt = modified_pkt;
        }
        if dst_buf == (*pkt).data.raw.buf as *mut core::ffi::c_char {
            (*priv_0).enc.cx_data_dst_buf.buf = dst_buf
                .offset((*pkt).data.raw.sz as isize) as *mut core::ffi::c_void;
            (*priv_0).enc.cx_data_dst_buf.sz = ((*priv_0).enc.cx_data_dst_buf.sz)
                .wrapping_sub((*pkt).data.raw.sz);
        }
    }
    return pkt;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_set_cx_data_buf(
    mut ctx: *mut aom_codec_ctx_t,
    mut buf: *const aom_fixed_buf_t,
    mut pad_before: core::ffi::c_uint,
    mut pad_after: core::ffi::c_uint,
) -> aom_codec_err_t {
    if ctx.is_null() || ((*ctx).priv_0).is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if !buf.is_null() {
        (*(*ctx).priv_0).enc.cx_data_dst_buf = *buf;
        (*(*ctx).priv_0).enc.cx_data_pad_before = pad_before;
        (*(*ctx).priv_0).enc.cx_data_pad_after = pad_after;
    } else {
        (*(*ctx).priv_0).enc.cx_data_dst_buf.buf = NULL;
        (*(*ctx).priv_0).enc.cx_data_dst_buf.sz = 0 as size_t;
        (*(*ctx).priv_0).enc.cx_data_pad_before = 0 as core::ffi::c_uint;
        (*(*ctx).priv_0).enc.cx_data_pad_after = 0 as core::ffi::c_uint;
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_get_preview_frame(
    mut ctx: *mut aom_codec_ctx_t,
) -> *const aom_image_t {
    let mut img: *mut aom_image_t = 0 as *mut aom_image_t;
    if !ctx.is_null() {
        if ((*ctx).iface).is_null() || ((*ctx).priv_0).is_null() {
            (*ctx).err = AOM_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
            (*ctx).err = AOM_CODEC_INCAPABLE;
        } else if ((*(*ctx).iface).enc.get_preview).is_none() {
            (*ctx).err = AOM_CODEC_INCAPABLE;
        } else {
            img = ((*(*ctx).iface).enc.get_preview)
                .expect("non-null function pointer")(get_alg_priv(ctx));
        }
    }
    return img;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_get_global_headers(
    mut ctx: *mut aom_codec_ctx_t,
) -> *mut aom_fixed_buf_t {
    let mut buf: *mut aom_fixed_buf_t = 0 as *mut aom_fixed_buf_t;
    if !ctx.is_null() {
        if ((*ctx).iface).is_null() || ((*ctx).priv_0).is_null() {
            (*ctx).err = AOM_CODEC_ERROR;
        } else if (*(*ctx).iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
            (*ctx).err = AOM_CODEC_INCAPABLE;
        } else if ((*(*ctx).iface).enc.get_glob_hdrs).is_none() {
            (*ctx).err = AOM_CODEC_INCAPABLE;
        } else {
            buf = ((*(*ctx).iface).enc.get_glob_hdrs)
                .expect("non-null function pointer")(get_alg_priv(ctx));
        }
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_enc_config_set(
    mut ctx: *mut aom_codec_ctx_t,
    mut cfg: *const aom_codec_enc_cfg_t,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    if ctx.is_null() || ((*ctx).iface).is_null() || ((*ctx).priv_0).is_null()
        || cfg.is_null()
    {
        res = AOM_CODEC_INVALID_PARAM;
    } else if (*(*ctx).iface).caps & AOM_CODEC_CAP_ENCODER as aom_codec_caps_t == 0 {
        res = AOM_CODEC_INCAPABLE;
    } else {
        res = ((*(*ctx).iface).enc.cfg_set)
            .expect("non-null function pointer")(get_alg_priv(ctx), cfg);
    }
    return (if !ctx.is_null() {
        (*ctx).err = res;
        (*ctx).err as core::ffi::c_uint
    } else {
        res as core::ffi::c_uint
    }) as aom_codec_err_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_pkt_list_add(
    mut list: *mut aom_codec_pkt_list,
    mut pkt: *const aom_codec_cx_pkt,
) -> core::ffi::c_int {
    if (*list).cnt < (*list).max {
        let fresh0 = (*list).cnt;
        (*list).cnt = ((*list).cnt).wrapping_add(1);
        *((*list).pkts).as_mut_ptr().offset(fresh0 as isize) = *pkt;
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_pkt_list_get(
    mut list: *mut aom_codec_pkt_list,
    mut iter: *mut aom_codec_iter_t,
) -> *const aom_codec_cx_pkt_t {
    let mut pkt: *const aom_codec_cx_pkt_t = 0 as *const aom_codec_cx_pkt_t;
    if (*iter).is_null() {
        *iter = ((*list).pkts).as_mut_ptr() as aom_codec_iter_t;
    }
    pkt = *iter as *const aom_codec_cx_pkt_t;
    if (pkt.offset_from(((*list).pkts).as_mut_ptr()) as core::ffi::c_long as size_t)
        < (*list).cnt as size_t
    {
        *iter = pkt.offset(1 as core::ffi::c_int as isize) as aom_codec_iter_t;
    } else {
        pkt = 0 as *const aom_codec_cx_pkt_t;
    }
    return pkt;
}
pub const AOM_ENCODER_ABI_VERSION: core::ffi::c_int = 10 as core::ffi::c_int
    + AOM_CODEC_ABI_VERSION + 3 as core::ffi::c_int;
pub const AOM_CODEC_CAP_PSNR: core::ffi::c_int = 0x10000 as core::ffi::c_int;
pub const AOM_CODEC_CAP_HIGHBITDEPTH: core::ffi::c_int = 0x40000 as core::ffi::c_int;
pub const AOM_CODEC_USE_PSNR: core::ffi::c_int = 0x10000 as core::ffi::c_int;
pub const AOM_CODEC_USE_HIGHBITDEPTH: core::ffi::c_int = 0x40000 as core::ffi::c_int;

extern "C" {
    fn feenableexcept(__excepts: core::ffi::c_int) -> core::ffi::c_int;
    fn fedisableexcept(__excepts: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
