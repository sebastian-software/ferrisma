/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/arg_defs.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
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
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_rc_mode = core::ffi::c_uint;
pub const AOM_Q: aom_rc_mode = 3;
pub const AOM_CQ: aom_rc_mode = 2;
pub const AOM_CBR: aom_rc_mode = 1;
pub const AOM_VBR: aom_rc_mode = 0;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_superblock_size = core::ffi::c_uint;
pub const AOM_SUPERBLOCK_SIZE_DYNAMIC: aom_superblock_size = 2;
pub const AOM_SUPERBLOCK_SIZE_128X128: aom_superblock_size = 1;
pub const AOM_SUPERBLOCK_SIZE_64X64: aom_superblock_size = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_def {
    pub short_name: *const core::ffi::c_char,
    pub long_name: *const core::ffi::c_char,
    pub has_val: core::ffi::c_int,
    pub desc: *const core::ffi::c_char,
    pub enums: *const arg_enum_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_enum_list {
    pub name: *const core::ffi::c_char,
    pub val: core::ffi::c_int,
}
pub type arg_def_t = arg_def;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const STEREO_FORMAT_RIGHT_LEFT: C2RustUnnamed = 11;
pub const STEREO_FORMAT_TOP_BOTTOM: C2RustUnnamed = 3;
pub const STEREO_FORMAT_BOTTOM_TOP: C2RustUnnamed = 2;
pub const STEREO_FORMAT_LEFT_RIGHT: C2RustUnnamed = 1;
pub const STEREO_FORMAT_MONO: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AOM_CONTENT_INVALID: C2RustUnnamed_0 = 3;
pub const AOM_CONTENT_FILM: C2RustUnnamed_0 = 2;
pub const AOM_CONTENT_SCREEN: C2RustUnnamed_0 = 1;
pub const AOM_CONTENT_DEFAULT: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const AOM_TIMING_DEC_MODEL: C2RustUnnamed_1 = 2;
pub const AOM_TIMING_EQUAL: C2RustUnnamed_1 = 1;
pub const AOM_TIMING_UNSPECIFIED: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const AOM_TUNE_SSIMULACRA2: C2RustUnnamed_2 = 11;
pub const AOM_TUNE_IQ: C2RustUnnamed_2 = 10;
pub const AOM_TUNE_VMAF_SALIENCY_MAP: C2RustUnnamed_2 = 9;
pub const AOM_TUNE_BUTTERAUGLI: C2RustUnnamed_2 = 8;
pub const AOM_TUNE_VMAF_NEG_MAX_GAIN: C2RustUnnamed_2 = 7;
pub const AOM_TUNE_VMAF_MAX_GAIN: C2RustUnnamed_2 = 6;
pub const AOM_TUNE_VMAF_WITHOUT_PREPROCESSING: C2RustUnnamed_2 = 5;
pub const AOM_TUNE_VMAF_WITH_PREPROCESSING: C2RustUnnamed_2 = 4;
pub const AOM_TUNE_SSIM: C2RustUnnamed_2 = 1;
pub const AOM_TUNE_PSNR: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const AOM_DIST_METRIC_QM_PSNR: C2RustUnnamed_3 = 1;
pub const AOM_DIST_METRIC_PSNR: C2RustUnnamed_3 = 0;
pub type TestDecodeFatality = core::ffi::c_uint;
pub const TEST_DECODE_WARN: TestDecodeFatality = 2;
pub const TEST_DECODE_FATAL: TestDecodeFatality = 1;
pub const TEST_DECODE_OFF: TestDecodeFatality = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct av1_codec_arg_definitions {
    pub help: arg_def_t,
    pub debugmode: arg_def_t,
    pub outputfile: arg_def_t,
    pub use_nv12: arg_def_t,
    pub use_yv12: arg_def_t,
    pub use_i420: arg_def_t,
    pub use_i422: arg_def_t,
    pub use_i444: arg_def_t,
    pub codecarg: arg_def_t,
    pub passes: arg_def_t,
    pub pass_arg: arg_def_t,
    pub fpf_name: arg_def_t,
    pub limit: arg_def_t,
    pub skip: arg_def_t,
    pub good_dl: arg_def_t,
    pub rt_dl: arg_def_t,
    pub ai_dl: arg_def_t,
    pub quietarg: arg_def_t,
    pub verbosearg: arg_def_t,
    pub psnrarg: arg_def_t,
    pub use_cfg: arg_def_t,
    pub recontest: arg_def_t,
    pub framerate: arg_def_t,
    pub use_webm: arg_def_t,
    pub use_ivf: arg_def_t,
    pub use_obu: arg_def_t,
    pub q_hist_n: arg_def_t,
    pub rate_hist_n: arg_def_t,
    pub disable_warnings: arg_def_t,
    pub disable_warning_prompt: arg_def_t,
    pub bitdeptharg: arg_def_t,
    pub inbitdeptharg: arg_def_t,
    pub input_chroma_subsampling_x: arg_def_t,
    pub input_chroma_subsampling_y: arg_def_t,
    pub usage: arg_def_t,
    pub threads: arg_def_t,
    pub profile: arg_def_t,
    pub width: arg_def_t,
    pub height: arg_def_t,
    pub forced_max_frame_width: arg_def_t,
    pub forced_max_frame_height: arg_def_t,
    pub stereo_mode: arg_def_t,
    pub timebase: arg_def_t,
    pub global_error_resilient: arg_def_t,
    pub lag_in_frames: arg_def_t,
    pub large_scale_tile: arg_def_t,
    pub monochrome: arg_def_t,
    pub full_still_picture_hdr: arg_def_t,
    pub use_16bit_internal: arg_def_t,
    pub dropframe_thresh: arg_def_t,
    pub resize_mode: arg_def_t,
    pub resize_denominator: arg_def_t,
    pub resize_kf_denominator: arg_def_t,
    pub superres_mode: arg_def_t,
    pub superres_denominator: arg_def_t,
    pub superres_kf_denominator: arg_def_t,
    pub superres_qthresh: arg_def_t,
    pub superres_kf_qthresh: arg_def_t,
    pub end_usage: arg_def_t,
    pub target_bitrate: arg_def_t,
    pub min_quantizer: arg_def_t,
    pub max_quantizer: arg_def_t,
    pub undershoot_pct: arg_def_t,
    pub overshoot_pct: arg_def_t,
    pub buf_sz: arg_def_t,
    pub buf_initial_sz: arg_def_t,
    pub buf_optimal_sz: arg_def_t,
    pub bias_pct: arg_def_t,
    pub minsection_pct: arg_def_t,
    pub maxsection_pct: arg_def_t,
    pub fwd_kf_enabled: arg_def_t,
    pub kf_min_dist: arg_def_t,
    pub kf_max_dist: arg_def_t,
    pub kf_disabled: arg_def_t,
    pub sframe_dist: arg_def_t,
    pub sframe_mode: arg_def_t,
    pub save_as_annexb: arg_def_t,
    pub noise_sens: arg_def_t,
    pub sharpness: arg_def_t,
    pub enable_adaptive_sharpness: arg_def_t,
    pub static_thresh: arg_def_t,
    pub auto_altref: arg_def_t,
    pub arnr_maxframes: arg_def_t,
    pub arnr_strength: arg_def_t,
    pub tune_metric: arg_def_t,
    pub dist_metric: arg_def_t,
    pub cq_level: arg_def_t,
    pub max_intra_rate_pct: arg_def_t,
    pub cpu_used_av1: arg_def_t,
    pub rowmtarg: arg_def_t,
    pub fpmtarg: arg_def_t,
    pub tile_cols: arg_def_t,
    pub tile_rows: arg_def_t,
    pub auto_tiles: arg_def_t,
    pub enable_tpl_model: arg_def_t,
    pub enable_keyframe_filtering: arg_def_t,
    pub tile_width: arg_def_t,
    pub tile_height: arg_def_t,
    pub lossless: arg_def_t,
    pub enable_cdef: arg_def_t,
    pub enable_restoration: arg_def_t,
    pub enable_rect_partitions: arg_def_t,
    pub enable_ab_partitions: arg_def_t,
    pub enable_1to4_partitions: arg_def_t,
    pub min_partition_size: arg_def_t,
    pub max_partition_size: arg_def_t,
    pub enable_dual_filter: arg_def_t,
    pub enable_chroma_deltaq: arg_def_t,
    pub enable_intra_edge_filter: arg_def_t,
    pub enable_order_hint: arg_def_t,
    pub enable_tx64: arg_def_t,
    pub enable_flip_idtx: arg_def_t,
    pub enable_rect_tx: arg_def_t,
    pub enable_dist_wtd_comp: arg_def_t,
    pub enable_masked_comp: arg_def_t,
    pub enable_onesided_comp: arg_def_t,
    pub enable_interintra_comp: arg_def_t,
    pub enable_smooth_interintra: arg_def_t,
    pub enable_diff_wtd_comp: arg_def_t,
    pub enable_interinter_wedge: arg_def_t,
    pub enable_interintra_wedge: arg_def_t,
    pub enable_global_motion: arg_def_t,
    pub enable_warped_motion: arg_def_t,
    pub enable_filter_intra: arg_def_t,
    pub enable_smooth_intra: arg_def_t,
    pub enable_paeth_intra: arg_def_t,
    pub enable_cfl_intra: arg_def_t,
    pub enable_directional_intra: arg_def_t,
    pub enable_diagonal_intra: arg_def_t,
    pub force_video_mode: arg_def_t,
    pub enable_obmc: arg_def_t,
    pub enable_overlay: arg_def_t,
    pub enable_palette: arg_def_t,
    pub enable_intrabc: arg_def_t,
    pub enable_angle_delta: arg_def_t,
    pub disable_trellis_quant: arg_def_t,
    pub enable_qm: arg_def_t,
    pub qm_min: arg_def_t,
    pub qm_max: arg_def_t,
    pub reduced_tx_type_set: arg_def_t,
    pub use_intra_dct_only: arg_def_t,
    pub use_inter_dct_only: arg_def_t,
    pub use_intra_default_tx_only: arg_def_t,
    pub quant_b_adapt: arg_def_t,
    pub coeff_cost_upd_freq: arg_def_t,
    pub mode_cost_upd_freq: arg_def_t,
    pub mv_cost_upd_freq: arg_def_t,
    pub dv_cost_upd_freq: arg_def_t,
    pub num_tg: arg_def_t,
    pub mtu_size: arg_def_t,
    pub timing_info: arg_def_t,
    pub partition_info_path: arg_def_t,
    pub enable_rate_guide_deltaq: arg_def_t,
    pub rate_distribution_info: arg_def_t,
    pub film_grain_test: arg_def_t,
    pub film_grain_table: arg_def_t,
    pub denoise_noise_level: arg_def_t,
    pub denoise_block_size: arg_def_t,
    pub enable_dnl_denoising: arg_def_t,
    pub enable_ref_frame_mvs: arg_def_t,
    pub frame_parallel_decoding: arg_def_t,
    pub error_resilient_mode: arg_def_t,
    pub aq_mode: arg_def_t,
    pub deltaq_mode: arg_def_t,
    pub deltaq_strength: arg_def_t,
    pub deltalf_mode: arg_def_t,
    pub frame_periodic_boost: arg_def_t,
    pub gf_cbr_boost_pct: arg_def_t,
    pub max_inter_rate_pct: arg_def_t,
    pub min_gf_interval: arg_def_t,
    pub max_gf_interval: arg_def_t,
    pub gf_min_pyr_height: arg_def_t,
    pub gf_max_pyr_height: arg_def_t,
    pub max_reference_frames: arg_def_t,
    pub reduced_reference_set: arg_def_t,
    pub target_seq_level_idx: arg_def_t,
    pub set_min_cr: arg_def_t,
    pub input_color_primaries: arg_def_t,
    pub input_transfer_characteristics: arg_def_t,
    pub input_matrix_coefficients: arg_def_t,
    pub input_chroma_sample_position: arg_def_t,
    pub tune_content: arg_def_t,
    pub cdf_update_mode: arg_def_t,
    pub superblock_size: arg_def_t,
    pub set_tier_mask: arg_def_t,
    pub use_fixed_qp_offsets: arg_def_t,
    pub fixed_qp_offsets: arg_def_t,
    pub vbr_corpus_complexity_lap: arg_def_t,
    pub fwd_kf_dist: arg_def_t,
    pub enable_tx_size_search: arg_def_t,
    pub loopfilter_control: arg_def_t,
    pub two_pass_input: arg_def_t,
    pub two_pass_output: arg_def_t,
    pub two_pass_width: arg_def_t,
    pub two_pass_height: arg_def_t,
    pub second_pass_log: arg_def_t,
    pub auto_intra_tools_off: arg_def_t,
    pub strict_level_conformance: arg_def_t,
    pub kf_max_pyr_height: arg_def_t,
    pub sb_qp_sweep: arg_def_t,
    pub enable_low_complexity_decode: arg_def_t,
    pub screen_detection_mode: arg_def_t,
    pub validate_hbd_input: arg_def_t,
    pub force_max_q: arg_def_t,
}
pub type av1_codec_arg_definitions_t = av1_codec_arg_definitions;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut test_decode_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"off\0" as *const u8 as *const core::ffi::c_char,
            val: TEST_DECODE_OFF as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"fatal\0" as *const u8 as *const core::ffi::c_char,
            val: TEST_DECODE_FATAL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"warn\0" as *const u8 as *const core::ffi::c_char,
            val: TEST_DECODE_WARN as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut bitdepth_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"8\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_BITS_8 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"10\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_BITS_10 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"12\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_BITS_12 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut stereo_mode_enum: [arg_enum_list; 6] = [
    {
        let mut init = arg_enum_list {
            name: b"mono\0" as *const u8 as *const core::ffi::c_char,
            val: STEREO_FORMAT_MONO as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"left-right\0" as *const u8 as *const core::ffi::c_char,
            val: STEREO_FORMAT_LEFT_RIGHT as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bottom-top\0" as *const u8 as *const core::ffi::c_char,
            val: STEREO_FORMAT_BOTTOM_TOP as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"top-bottom\0" as *const u8 as *const core::ffi::c_char,
            val: STEREO_FORMAT_TOP_BOTTOM as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"right-left\0" as *const u8 as *const core::ffi::c_char,
            val: STEREO_FORMAT_RIGHT_LEFT as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut end_usage_enum: [arg_enum_list; 5] = [
    {
        let mut init = arg_enum_list {
            name: b"vbr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_VBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"cbr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"cq\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CQ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"q\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_Q as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut tuning_enum: [arg_enum_list; 11] = [
    {
        let mut init = arg_enum_list {
            name: b"psnr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_PSNR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ssim\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_SSIM as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vmaf_with_preprocessing\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_VMAF_WITH_PREPROCESSING as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vmaf_without_preprocessing\0" as *const u8
                as *const core::ffi::c_char,
            val: AOM_TUNE_VMAF_WITHOUT_PREPROCESSING as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vmaf\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_VMAF_MAX_GAIN as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vmaf_neg\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_VMAF_NEG_MAX_GAIN as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"butteraugli\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_BUTTERAUGLI as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vmaf_saliency_map\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_VMAF_SALIENCY_MAP as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"iq\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_IQ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ssimulacra2\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_SSIMULACRA2 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut dist_metric_enum: [arg_enum_list; 3] = [
    {
        let mut init = arg_enum_list {
            name: b"psnr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_DIST_METRIC_PSNR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"qm-psnr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_DIST_METRIC_QM_PSNR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut timing_info_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"unspecified\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TIMING_UNSPECIFIED as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"constant\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TIMING_EQUAL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"model\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TIMING_DEC_MODEL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut superblock_size_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"dynamic\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_SUPERBLOCK_SIZE_DYNAMIC as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"64\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_SUPERBLOCK_SIZE_64X64 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"128\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_SUPERBLOCK_SIZE_128X128 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut matrix_coefficients_enum: [arg_enum_list; 18] = [
    {
        let mut init = arg_enum_list {
            name: b"identity\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_IDENTITY as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt709\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_BT_709 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"unspecified\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_UNSPECIFIED as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"fcc73\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_FCC as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt470bg\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_BT_470_B_G as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt601\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_BT_601 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte240\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_SMPTE_240 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ycgco\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_SMPTE_YCGCO as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt2020ncl\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_BT_2020_NCL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt2020cl\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_BT_2020_CL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte2085\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_SMPTE_2085 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"chromncl\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_CHROMAT_NCL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"chromcl\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_CHROMAT_CL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ictcp\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_ICTCP as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ipt-c2\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_IPT_C2 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ycgco-re\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_YCGCO_RE as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ycgco-ro\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_MC_YCGCO_RO as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut chroma_sample_position_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"unknown\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CSP_UNKNOWN as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"vertical\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CSP_VERTICAL as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"colocated\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CSP_COLOCATED as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut tune_content_enum: [arg_enum_list; 4] = [
    {
        let mut init = arg_enum_list {
            name: b"default\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CONTENT_DEFAULT as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"screen\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CONTENT_SCREEN as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"film\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CONTENT_FILM as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut transfer_characteristics_enum: [arg_enum_list; 18] = [
    {
        let mut init = arg_enum_list {
            name: b"unspecified\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_UNSPECIFIED as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt709\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_709 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt470m\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_470_M as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt470bg\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_470_B_G as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt601\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_601 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte240\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_SMPTE_240 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"lin\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_LINEAR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"log100\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_LOG_100 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"log100sq10\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_LOG_100_SQRT10 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"iec61966\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_IEC_61966 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt1361\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_1361 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"srgb\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_SRGB as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt2020-10bit\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_2020_10_BIT as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt2020-12bit\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_BT_2020_12_BIT as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte2084\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_SMPTE_2084 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"hlg\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_HLG as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte428\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_TC_SMPTE_428 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
static mut color_primaries_enum: [arg_enum_list; 13] = [
    {
        let mut init = arg_enum_list {
            name: b"bt709\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_BT_709 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"unspecified\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_UNSPECIFIED as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt601\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_BT_601 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt470m\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_BT_470_M as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt470bg\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_BT_470_B_G as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte240\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_SMPTE_240 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"film\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_GENERIC_FILM as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"bt2020\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_BT_2020 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"xyz\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_XYZ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte431\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_SMPTE_431 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"smpte432\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_SMPTE_432 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: b"ebu3213\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CICP_CP_EBU_3213 as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = arg_enum_list {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut g_av1_codec_arg_defs: av1_codec_arg_definitions_t = unsafe {
    {
        let mut init = av1_codec_arg_definitions {
            help: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"help\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Show usage options and exit\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            debugmode: {
                let mut init = arg_def {
                    short_name: b"D\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"debug\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Debug mode (makes output deterministic)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            outputfile: {
                let mut init = arg_def {
                    short_name: b"o\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"output\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Output filename\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_nv12: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"nv12\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Input file is NV12\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_yv12: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"yv12\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Input file is YV12\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_i420: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"i420\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Input file is I420 (default)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_i422: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"i422\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Input file is I422\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_i444: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"i444\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Input file is I444\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            codecarg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"codec\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Codec to use\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            passes: {
                let mut init = arg_def {
                    short_name: b"p\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"passes\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Number of passes (1/2/3)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            pass_arg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"pass\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Pass to execute (1/2/3)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            fpf_name: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"fpf\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"First pass statistics file name\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            limit: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"limit\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Stop encoding after n input frames\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            skip: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"skip\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Skip the first n input frames\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            good_dl: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"good\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Use Good Quality Deadline\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            rt_dl: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"rt\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Use Realtime Quality Deadline\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            ai_dl: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"allintra\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Use all intra mode\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            quietarg: {
                let mut init = arg_def {
                    short_name: b"q\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"quiet\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Do not print encode progress\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            verbosearg: {
                let mut init = arg_def {
                    short_name: b"v\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"verbose\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Show encoder parameters\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            psnrarg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"psnr\0" as *const u8 as *const core::ffi::c_char,
                    has_val: -(1 as core::ffi::c_int),
                    desc: b"Show PSNR in status line (0: Disable PSNR status line display, 1: PSNR calculated using input bit-depth (default), 2: PSNR calculated using stream bit-depth); takes default option when arguments are not specified\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_cfg: {
                let mut init = arg_def {
                    short_name: b"c\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"cfg\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Config file to use\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            recontest: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"test-decode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Test encode/decode mismatch\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(test_decode_enum).cast::<arg_enum_list>(),
                };
                init
            },
            framerate: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"fps\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Stream frame rate (rate/scale)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_webm: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"webm\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Output WebM (default when WebM IO is enabled)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_ivf: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"ivf\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Output IVF\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_obu: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"obu\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Output OBU\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            q_hist_n: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"q-hist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Show quantizer histogram (n-buckets)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            rate_hist_n: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"rate-hist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Show rate histogram (n-buckets)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            disable_warnings: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"disable-warnings\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Disable warnings about potentially incorrect encode settings\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            disable_warning_prompt: {
                let mut init = arg_def {
                    short_name: b"y\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"disable-warning-prompt\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Display warnings, but do not prompt user to continue\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            bitdeptharg: {
                let mut init = arg_def {
                    short_name: b"b\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"bit-depth\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Bit depth for codec\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(bitdepth_enum).cast::<arg_enum_list>(),
                };
                init
            },
            inbitdeptharg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"input-bit-depth\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Actual bit depth of input source\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            input_chroma_subsampling_x: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"input-chroma-subsampling-x\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Chroma subsampling x value\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            input_chroma_subsampling_y: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"input-chroma-subsampling-y\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Chroma subsampling y value\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            usage: {
                let mut init = arg_def {
                    short_name: b"u\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"usage\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Usage profile number to use (0: good, 1: rt, 2: allintra)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            threads: {
                let mut init = arg_def {
                    short_name: b"t\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"threads\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max number of threads to use\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            profile: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"profile\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Bitstream profile number to use\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            width: {
                let mut init = arg_def {
                    short_name: b"w\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"width\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame width\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            height: {
                let mut init = arg_def {
                    short_name: b"h\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"height\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame height\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            forced_max_frame_width: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"forced_max_frame_width\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum frame width value to force\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            forced_max_frame_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"forced_max_frame_height\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum frame height value to force\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            stereo_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"stereo-mode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Stereo 3D video format\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(stereo_mode_enum).cast::<arg_enum_list>(),
                };
                init
            },
            timebase: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"timebase\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Output timestamp precision (fractional seconds)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            global_error_resilient: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"global-error-resilient\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable global error resiliency features\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            lag_in_frames: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"lag-in-frames\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max number of frames to lag\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            large_scale_tile: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"large-scale-tile\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Large scale tile coding (0: off (default), 1: on (ivf output only))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            monochrome: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"monochrome\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Monochrome video (no chroma planes)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            full_still_picture_hdr: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"full-still-picture-hdr\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Use full header for still picture\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_16bit_internal: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"use-16bit-internal\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Force use of 16-bit pipeline\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            dropframe_thresh: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"drop-frame\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Temporal resampling threshold (buf %)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            resize_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"resize-mode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame resize mode (0: off (default), 1: fixed, 2: random, 3: dynamic)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            resize_denominator: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"resize-denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame resize denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            resize_kf_denominator: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"resize-kf-denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame resize keyframe denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superres_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"superres-mode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame super-resolution mode (0: disabled (default), 1: fixed, 2: random, 3: qthresh, 4: auto)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superres_denominator: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"superres-denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame super-resolution denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superres_kf_denominator: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"superres-kf-denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame super-resolution keyframe denominator\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superres_qthresh: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"superres-qthresh\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame super-resolution qindex threshold\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superres_kf_qthresh: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"superres-kf-qthresh\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Frame super-resolution keyframe qindex threshold\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            end_usage: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"end-usage\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Rate control mode\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(end_usage_enum).cast::<arg_enum_list>(),
                };
                init
            },
            target_bitrate: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"target-bitrate\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Bitrate (kbps)\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            min_quantizer: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"min-q\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Minimum (best) quantizer\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_quantizer: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-q\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum (worst) quantizer\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            undershoot_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"undershoot-pct\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Datarate undershoot (min) target (%)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            overshoot_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"overshoot-pct\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Datarate overshoot (max) target (%)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            buf_sz: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"buf-sz\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Client buffer size (ms)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            buf_initial_sz: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"buf-initial-sz\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Client initial buffer size (ms)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            buf_optimal_sz: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"buf-optimal-sz\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Client optimal buffer size (ms)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            bias_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"bias-pct\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"CBR/VBR bias (0=CBR, 100=VBR)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            minsection_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"minsection-pct\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"GOP min bitrate (% of target)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            maxsection_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"maxsection-pct\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"GOP max bitrate (% of target)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            fwd_kf_enabled: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-fwd-kf\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable forward reference keyframes\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            kf_min_dist: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"kf-min-dist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Minimum keyframe interval (frames)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            kf_max_dist: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"kf-max-dist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum keyframe interval (frames)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            kf_disabled: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"disable-kf\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 0 as core::ffi::c_int,
                    desc: b"Disable keyframe placement\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            sframe_dist: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"sframe-dist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"S-Frame interval (frames)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            sframe_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"sframe-mode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"S-Frame insertion mode (1..2)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            save_as_annexb: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"annexb\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Save as Annex-B\0" as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            noise_sens: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"noise-sensitivity\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Noise sensitivity (frames to blur)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            sharpness: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"sharpness\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Bias towards block sharpness in rate-distortion optimization of transform coefficients and (in all intra mode only) reduce block edge filtering for better sharpness (0..7), default is 0\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_adaptive_sharpness: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-adaptive-sharpness\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable adaptive sharpness (0: disabled (default), 1: enabled)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            static_thresh: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"static-thresh\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Motion detection threshold\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            auto_altref: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"auto-alt-ref\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable automatic alt reference frames\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            arnr_maxframes: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"arnr-maxframes\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"AltRef max frames (0..15)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            arnr_strength: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"arnr-strength\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"AltRef filter strength (0..6)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            tune_metric: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tune\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Distortion metric tuned with\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(tuning_enum).cast::<arg_enum_list>(),
                };
                init
            },
            dist_metric: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"dist-metric\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Distortion metric to use for in-block optimization\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(dist_metric_enum).cast::<arg_enum_list>(),
                };
                init
            },
            cq_level: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"cq-level\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Constant/Constrained Quality level\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_intra_rate_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-intra-rate\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max I-frame bitrate (pct)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            cpu_used_av1: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"cpu-used\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Speed setting (0..6 in good mode, 5..12 in realtime mode, 0..9 in all intra mode)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            rowmtarg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"row-mt\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable row based multi-threading (0: off, 1: on (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            fpmtarg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"fp-mt\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable frame parallel multi-threading (0: off (default), 1: on)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            tile_cols: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tile-columns\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Number of tile columns to use, log2\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            tile_rows: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tile-rows\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Number of tile rows to use, log2\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            auto_tiles: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"auto-tiles\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable auto tiles (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_tpl_model: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-tpl-model\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"RDO based on frame temporal dependency (0: off, 1: backward source based); required for --deltaq-mode=1\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_keyframe_filtering: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-keyframe-filtering\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Apply temporal filtering on key frame (0: no filter, 1: filter without overlay (default), 2: filter with overlay - experimental, may break random access in players)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            tile_width: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tile-width\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Tile widths (comma separated)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            tile_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tile-height\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Tile heights (command separated)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            lossless: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"lossless\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Lossless mode (0: false (default), 1: true)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_cdef: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-cdef\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable the constrained directional enhancement filter (0: false, 1: true (default), 2: disable for non-reference frames, 3: enable adaptively based on frame qindex)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_restoration: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-restoration\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable the loop restoration filter (0: false (default in realtime mode), 1: true (default in non-realtime mode))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_rect_partitions: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-rect-partitions\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable rectangular partitions (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_ab_partitions: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-ab-partitions\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable ab partitions (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_1to4_partitions: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-1to4-partitions\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable 1:4 and 4:1 partitions (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            min_partition_size: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"min-partition-size\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set min partition size (4:4x4, 8:8x8, 16:16x16, 32:32x32, 64:64x64, 128:128x128); with 4k+ resolutions or higher speed settings, min partition size will have a minimum of 8\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_partition_size: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-partition-size\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set max partition size (4:4x4, 8:8x8, 16:16x16, 32:32x32, 64:64x64, 128:128x128)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_dual_filter: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-dual-filter\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable dual filter (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_chroma_deltaq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-chroma-deltaq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable chroma delta quant (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_intra_edge_filter: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-intra-edge-filter\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable intra edge filtering (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_order_hint: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-order-hint\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable order hint (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_tx64: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-tx64\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable 64-pt transform (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_flip_idtx: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-flip-idtx\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable extended transform type (0: false, 1: true (default)) including FLIPADST_DCT, DCT_FLIPADST, FLIPADST_FLIPADST, ADST_FLIPADST, FLIPADST_ADST, IDTX, V_DCT, H_DCT, V_ADST, H_ADST, V_FLIPADST, H_FLIPADST\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_rect_tx: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-rect-tx\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable rectangular transform (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_dist_wtd_comp: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-dist-wtd-comp\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable distance-weighted compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_masked_comp: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-masked-comp\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable masked (wedge/diff-wtd) compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_onesided_comp: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-onesided-comp\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable one sided compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_interintra_comp: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-interintra-comp\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable interintra compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_smooth_interintra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-smooth-interintra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable smooth interintra mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_diff_wtd_comp: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-diff-wtd-comp\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable difference-weighted compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_interinter_wedge: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-interinter-wedge\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable interinter wedge compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_interintra_wedge: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-interintra-wedge\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable interintra wedge compound (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_global_motion: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-global-motion\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable global motion (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_warped_motion: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-warped-motion\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable local warped motion (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_filter_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-filter-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable filter intra prediction mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_smooth_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-smooth-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable smooth intra prediction modes (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_paeth_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-paeth-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable Paeth intra prediction mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_cfl_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-cfl-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable chroma from luma intra prediction mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_directional_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-directional-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable directional intra prediction modes (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_diagonal_intra: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-diagonal-intra\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable diagonal (D45 to D203) intra prediction modes, which are a subset of directional modes; has no effect if enable-directional-intra is 0 (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            force_video_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"force-video-mode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Force video mode even for a single frame (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_obmc: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-obmc\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable OBMC (0: false, 1: true (default))\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_overlay: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-overlay\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable coding overlay frames (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_palette: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-palette\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable palette prediction mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_intrabc: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-intrabc\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable intra block copy prediction mode (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_angle_delta: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-angle-delta\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable intra angle delta (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            disable_trellis_quant: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"disable-trellis-quant\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Disable trellis optimization of quantized coefficients (0: false 1: true  2: true for rd search 3: true for estimate yrd search (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_qm: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-qm\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable quantisation matrices (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            qm_min: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"qm-min\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Min quant matrix flatness (0..15), default is 5 (4 for all intra mode)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            qm_max: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"qm-max\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max quant matrix flatness (0..15), default is 9 (10 for all intra mode)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            reduced_tx_type_set: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"reduced-tx-type-set\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use reduced set of transform types\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_intra_dct_only: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"use-intra-dct-only\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use DCT only for INTRA modes\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_inter_dct_only: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"use-inter-dct-only\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use DCT only for INTER modes\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_intra_default_tx_only: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"use-intra-default-tx-only\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use Default-transform only for INTRA modes\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            quant_b_adapt: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"quant-b-adapt\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use adaptive quantize_b\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            coeff_cost_upd_freq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"coeff-cost-upd-freq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Update freq for coeff costs. 0: SB, 1: SB Row per Tile, 2: Tile, 3: Off\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            mode_cost_upd_freq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"mode-cost-upd-freq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Update freq for mode costs. 0: SB, 1: SB Row per Tile, 2: Tile, 3: Off\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            mv_cost_upd_freq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"mv-cost-upd-freq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Update freq for mv costs. 0: SB, 1: SB Row per Tile, 2: Tile, 3: Off\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            dv_cost_upd_freq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"dv-cost-upd-freq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Update freq for dv costs. 0: SB, 1: SB Row per Tile, 2: Tile, 3: Off\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            num_tg: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"num-tile-groups\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum number of tile groups, default is 1\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            mtu_size: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"mtu-size\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"MTU size for a tile group, default is 0 (no MTU targeting), overrides maximum number of tile groups\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            timing_info: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"timing-info\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Signal timing info in the bitstream (model only works for no hidden frames, no super-res yet):\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(timing_info_enum).cast::<arg_enum_list>(),
                };
                init
            },
            partition_info_path: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"partition-info-path\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Partition information read and write path\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_rate_guide_deltaq: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-rate-guide-deltaq\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable rate guide deltaq (1), by default off (0). It requires --deltaq-mode=3. If turned on, it requires an input file specified by --rate-distribution-info.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            rate_distribution_info: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"rate-distribution-info\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Rate distribution information input.It requires --enable-rate-guide-deltaq=1.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            film_grain_test: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"film-grain-test\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Film grain test vectors (0: none (default), 1: test-1  2: test-2, ... 16: test-16)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            film_grain_table: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"film-grain-table\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Path to file containing film grain parameters\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            denoise_noise_level: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"denoise-noise-level\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Amount of noise (from 0 = don't denoise, to 50)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            denoise_block_size: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"denoise-block-size\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Denoise block size (default = 32)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_dnl_denoising: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-dnl-denoising\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Apply denoising to the frame being encoded when denoise-noise-level is enabled (0: false, 1: true (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_ref_frame_mvs: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-ref-frame-mvs\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable temporal mv prediction (default is 1)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            frame_parallel_decoding: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"frame-parallel\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable frame parallel decodability features (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            error_resilient_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"error-resilient\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable error resilient features (0: false (default), 1: true)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            aq_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"aq-mode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Adaptive quantization mode (0: off (default), 1: variance 2: complexity, 3: cyclic refresh)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            deltaq_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"deltaq-mode\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Delta qindex mode (0: off, 1: deltaq objective (default), 2: deltaq placeholder, 3: key frame visual quality, 4: user rating based visual quality optimization, 5: HDR video, 6: Variance Boost); --deltaq-mode=1 requires --enable-tpl-model=1\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            deltaq_strength: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"deltaq-strength\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Deltaq strength for --deltaq-mode=4 and --deltaq-mode=6 (%)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            deltalf_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"delta-lf-mode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable delta-lf-mode (0: off (default), 1: on)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            frame_periodic_boost: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"frame-boost\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable frame periodic boost (0: off (default), 1: on)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            gf_cbr_boost_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"gf-cbr-boost\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Boost for Golden Frame in CBR mode (pct)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_inter_rate_pct: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-inter-rate\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max P-frame bitrate (pct)\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            min_gf_interval: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"min-gf-interval\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Min gf/arf frame interval (default 0, indicating in-built behavior)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_gf_interval: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-gf-interval\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Max gf/arf frame interval (default 0, indicating in-built behavior)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            gf_min_pyr_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"gf-min-pyr-height\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Min height for GF group pyramid structure (0 (default) to 5)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            gf_max_pyr_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"gf-max-pyr-height\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum height for GF group pyramid structure (0 to 5 (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            max_reference_frames: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"max-reference-frames\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum number of reference frames allowed per frame (3 to 7 (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            reduced_reference_set: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"reduced-reference-set\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Use reduced set of single and compound references (0: off (default), 1: on)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            target_seq_level_idx: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"target-seq-level-idx\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Target sequence level index. Possible values are in the form of \"ABxy\". AB: Operating point (OP) index, xy: Target level index for the OP. E.g. \"0\" means target level index 0 (2.0) for the 0th OP, \"1019\" means target level index 19 (6.3) for the 10th OP.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            set_min_cr: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"min-cr\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set minimum compression ratio. Take integer values. Default is 0. If non-zero, encoder will try to keep the compression ratio of each frame to be higher than the given value divided by 100.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            input_color_primaries: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"color-primaries\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Color primaries (CICP) of input content:\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(color_primaries_enum).cast::<arg_enum_list>(),
                };
                init
            },
            input_transfer_characteristics: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"transfer-characteristics\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Transfer characteristics (CICP) of input content:\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(transfer_characteristics_enum).cast::<arg_enum_list>(),
                };
                init
            },
            input_matrix_coefficients: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"matrix-coefficients\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Matrix coefficients (CICP) of input content:\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(matrix_coefficients_enum).cast::<arg_enum_list>(),
                };
                init
            },
            input_chroma_sample_position: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"chroma-sample-position\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"The chroma sample position when chroma 4:2:0 is signaled:\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(chroma_sample_position_enum).cast::<arg_enum_list>(),
                };
                init
            },
            tune_content: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"tune-content\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Tune content type\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(tune_content_enum).cast::<arg_enum_list>(),
                };
                init
            },
            cdf_update_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"cdf-update-mode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"CDF update mode for entropy coding (0: no CDF update, 1: update CDF on all frames (default), 2: selectively update CDF on some frames)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            superblock_size: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"sb-size\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Superblock size to use\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: core::ptr::addr_of!(superblock_size_enum).cast::<arg_enum_list>(),
                };
                init
            },
            set_tier_mask: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"set-tier-mask\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set bit mask to specify which tier each of the 32 possible operating points conforms to. Bit value 0 (default): Main Tier, 1: High Tier.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            use_fixed_qp_offsets: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"use-fixed-qp-offsets\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Controls how the encoder applies fixed QP offsets for frames at different levels of the pyramid (0: adaptively-chosen offsets from --cq-level if --fixed-qp-offsets is not provided (default), 1: fixed QP offsets, 2: no QP offsets)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            fixed_qp_offsets: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"fixed-qp-offsets\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set fixed QP offsets for frames at different levels of the pyramid. Comma-separated list of 5 offsets for keyframe, ALTREF, and 3 levels of internal alt-refs. If this option is not specified (default), offsets are adaptively chosen by the encoder.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            vbr_corpus_complexity_lap: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"vbr-corpus-complexity-lap\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Set average corpus complexity per mb for single pass VBR using lap. (0..10000), default is 0\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            fwd_kf_dist: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"fwd-kf-dist\0" as *const u8 as *const core::ffi::c_char,
                    has_val: -(1 as core::ffi::c_int),
                    desc: b"Set distance between forward keyframes. A value of -1 (default) means no repetitive forward keyframes.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_tx_size_search: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-tx-size-search\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable transform size search to find the best size for each block. If false, transforms always have the largest possible size (0: false, 1: true (default)). Ignored in non rd pick mode in real-time coding.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            loopfilter_control: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"loopfilter-control\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Control loop filtering (0: Loopfilter disabled for all frames, 1: Enable loopfilter for all frames (default), 2: Disable loopfilter for non-reference frames, 3: Disable loopfilter for frames with low motion)\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            two_pass_input: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"two-pass-input\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"The input file for the second pass for three-pass encoding\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            two_pass_output: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"two-pass-output\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"The output file for the first two passes for three-pass encoding\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            two_pass_width: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"two-pass-width\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"The width of two-pass-input\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            two_pass_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"two-pass-height\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"The height of two-pass-input\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            second_pass_log: {
                let mut init = arg_def {
                    short_name: b"spf\0" as *const u8 as *const core::ffi::c_char,
                    long_name: b"second-pass-log\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Log file from second pass\0" as *const u8
                        as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            auto_intra_tools_off: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"auto-intra-tools-off\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Automatically turn off several intra coding tools for all intra mode; only in effect if --deltaq-mode=3\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            strict_level_conformance: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"strict-level-conformance\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"When set to 1, exit the encoder when it fails to encode to a given target level\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            kf_max_pyr_height: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"kf-max-pyr-height\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Maximum height of pyramid structure used for the GOP starting with a key frame (-1 to 5). When set to -1 (default), it does not have any effect. The actual maximum pyramid height will be the minimum of this value and the value of gf_max_pyr_height.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            sb_qp_sweep: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"sb-qp-sweep\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"When set to 1, enable the superblock level qp sweep for a given lambda to minimize the rdcost.\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            enable_low_complexity_decode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"enable-low-complexity-decode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Enable low complexity decode (0: false (default), 1: true). This supports good-quality encoding (speed 1 to 3) for vertical videos (608p to 1080p) and horizontal videos (720p to 1080p).\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            screen_detection_mode: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"screen-detection-mode\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Screen content detection mode (1: standard (default in good quality and realtime modes), 2: anti-aliased text and graphics aware (default in all intra mode))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            validate_hbd_input: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"validate-hbd-input\0" as *const u8
                        as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Check that input samples are within the valid range for the chosen bit depth with high bit depth encoding (0: disabled, 1: enabled (default))\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
            force_max_q: {
                let mut init = arg_def {
                    short_name: 0 as *const core::ffi::c_char,
                    long_name: b"force-max-q\0" as *const u8 as *const core::ffi::c_char,
                    has_val: 1 as core::ffi::c_int,
                    desc: b"Force the use of max Q in VBR mode (0: false (default), 1: true).\0"
                        as *const u8 as *const core::ffi::c_char,
                    enums: 0 as *const arg_enum_list,
                };
                init
            },
        };
        init
    }
};
