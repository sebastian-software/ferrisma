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

// Mechanically translated from upstream/aom/av1/av1_cx_iface.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    pub type __sFILEX;
    pub type aom_denoise_and_model_t;
    fn aom_uleb_size_in_bytes(value: uint64_t) -> size_t;
    fn aom_img_plane_width(
        img: *const aom_image_t,
        plane: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_img_plane_height(
        img: *const aom_image_t,
        plane: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn arg_match_helper(
        arg_: *mut arg,
        def: *const arg_def,
        argv: *mut *mut core::ffi::c_char,
        err_msg: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn arg_parse_uint_helper(
        arg: *const arg,
        err_msg: *mut core::ffi::c_char,
    ) -> core::ffi::c_uint;
    fn arg_parse_int_helper(
        arg: *const arg,
        err_msg: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn arg_parse_enum_helper(
        arg: *const arg,
        err_msg: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn arg_parse_list_helper(
        arg: *const arg,
        list: *mut core::ffi::c_int,
        n: core::ffi::c_int,
        err_msg: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn aom_codec_pkt_list_add(
        _: *mut aom_codec_pkt_list,
        _: *const aom_codec_cx_pkt,
    ) -> core::ffi::c_int;
    fn aom_codec_pkt_list_get(
        list: *mut aom_codec_pkt_list,
        iter: *mut aom_codec_iter_t,
    ) -> *const aom_codec_cx_pkt_t;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_internal_error_copy(
        info: *mut aom_internal_error_info,
        src: *const aom_internal_error_info,
    );
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_free_ref_frame_buffers(pool: *mut BufferPool);
    fn av1_lookahead_init(
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        subsampling_x: core::ffi::c_int,
        subsampling_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        depth: core::ffi::c_int,
        border_in_pixels: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        num_lap_buffers: core::ffi::c_int,
        is_all_intra: bool,
        alloc_pyramid: bool,
    ) -> *mut lookahead_ctx;
    fn av1_twopass_zero_stats(section: *mut FIRSTPASS_STATS);
    fn av1_extrc_create(
        funcs: aom_rc_funcs_t,
        ratectrl_config: aom_rc_config_t,
        ext_ratectrl: *mut AOM_EXT_RATECTRL,
    ) -> aom_codec_err_t;
    fn av1_extrc_delete(ext_ratectrl: *mut AOM_EXT_RATECTRL) -> aom_codec_err_t;
    fn av1_quantizer_to_qindex(quantizer: core::ffi::c_int) -> core::ffi::c_int;
    fn av1_qindex_to_quantizer(qindex: core::ffi::c_int) -> core::ffi::c_int;
    fn av1_free_shared_coeff_buffer(shared_bufs: *mut PC_TREE_SHARED_BUFFERS);
    fn av1_free_pc_tree_recursive(
        tree: *mut PC_TREE,
        num_planes: core::ffi::c_int,
        keep_best: core::ffi::c_int,
        keep_none: core::ffi::c_int,
        partition_search_type: PARTITION_SEARCH_TYPE,
    );
    fn av1_free_pmc(ctx: *mut PICK_MODE_CONTEXT, num_planes: core::ffi::c_int);
    fn av1_free_sms_tree(td: *mut ThreadData);
    fn av1_ext_part_create(
        funcs: aom_ext_part_funcs_t,
        config: aom_ext_part_config_t,
        ext_part_controller: *mut ExtPartController,
    ) -> aom_codec_err_t;
    fn av1_get_seq_level_idx(
        seq_params: *const SequenceHeader,
        level_params: *const AV1LevelParams,
        seq_level_idx: *mut core::ffi::c_int,
    ) -> aom_codec_err_t;
    fn av1_get_target_seq_level_idx(
        seq_params: *const SequenceHeader,
        level_params: *const AV1LevelParams,
        target_seq_level_idx: *mut core::ffi::c_int,
    ) -> aom_codec_err_t;
    fn av1_init_layer_context(cpi: *mut AV1_COMP);
    fn av1_alloc_layer_context(cpi: *mut AV1_COMP, num_layers: core::ffi::c_int) -> bool;
    fn av1_update_layer_context_change_config(
        cpi: *mut AV1_COMP,
        target_bandwidth: int64_t,
    );
    fn av1_free_svc_cyclic_refresh(cpi: *mut AV1_COMP);
    fn av1_set_svc_fixed_mode(cpi: *mut AV1_COMP);
    fn av1_svc_check_reset_layer_rc_flag(cpi: *mut AV1_COMP);
    fn av1_write_obu_header(
        level_params: *mut AV1LevelParams,
        frame_header_count: *mut core::ffi::c_int,
        obu_type: OBU_TYPE,
        has_nonzero_operating_point_idc: bool,
        is_layer_specific_obu: bool,
        obu_extension: core::ffi::c_int,
        dst: *mut uint8_t,
    ) -> uint32_t;
    fn av1_write_uleb_obu_size(
        obu_payload_size: size_t,
        dest: *mut uint8_t,
        dest_size: size_t,
    ) -> core::ffi::c_int;
    fn av1_initialize_enc(usage: core::ffi::c_uint, end_usage: aom_rc_mode);
    fn av1_create_compressor(
        ppi: *mut AV1_PRIMARY,
        oxcf: *const AV1EncoderConfig,
        pool: *mut BufferPool,
        stage: COMPRESSOR_STAGE,
        lap_lag_in_frames: core::ffi::c_int,
    ) -> *mut AV1_COMP;
    fn av1_create_primary_compressor(
        pkt_list_head: *mut aom_codec_pkt_list,
        num_lap_buffers: core::ffi::c_int,
        oxcf: *const AV1EncoderConfig,
    ) -> *mut AV1_PRIMARY;
    fn av1_remove_compressor(cpi: *mut AV1_COMP);
    fn av1_remove_primary_compressor(ppi: *mut AV1_PRIMARY);
    fn av1_change_config_seq(
        ppi: *mut AV1_PRIMARY,
        oxcf: *const AV1EncoderConfig,
        sb_size_changed: *mut bool,
    );
    fn av1_change_config(
        cpi: *mut AV1_COMP,
        oxcf: *const AV1EncoderConfig,
        sb_size_changed: bool,
    );
    fn av1_check_initial_width(
        cpi: *mut AV1_COMP,
        use_highbitdepth: core::ffi::c_int,
        subsampling_x: core::ffi::c_int,
        subsampling_y: core::ffi::c_int,
    ) -> aom_codec_err_t;
    fn av1_post_encode_updates(cpi: *mut AV1_COMP, cpi_data: *const AV1_COMP_DATA);
    fn av1_init_sc_decisions(ppi: *mut AV1_PRIMARY);
    fn av1_get_parallel_frame_enc_data(
        ppi: *mut AV1_PRIMARY,
        first_cpi_data: *mut AV1_COMP_DATA,
    ) -> *mut AV1_COMP;
    fn av1_receive_raw_frame(
        cpi: *mut AV1_COMP,
        frame_flags: aom_enc_frame_flags_t,
        sd: *const YV12_BUFFER_CONFIG,
        time_stamp: int64_t,
        end_time_stamp: int64_t,
    ) -> core::ffi::c_int;
    fn av1_get_compressed_data(
        cpi: *mut AV1_COMP,
        cpi_data: *mut AV1_COMP_DATA,
    ) -> core::ffi::c_int;
    fn av1_get_preview_raw_frame(
        cpi: *mut AV1_COMP,
        dest: *mut YV12_BUFFER_CONFIG,
    ) -> core::ffi::c_int;
    fn av1_get_last_show_frame(
        cpi: *mut AV1_COMP,
        frame: *mut YV12_BUFFER_CONFIG,
    ) -> core::ffi::c_int;
    fn av1_copy_new_frame_enc(
        cm: *mut AV1_COMMON,
        new_frame: *mut YV12_BUFFER_CONFIG,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> aom_codec_err_t;
    fn av1_use_as_reference(
        ext_ref_frame_flags: *mut core::ffi::c_int,
        ref_frame_flags: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_copy_reference_enc(
        cpi: *mut AV1_COMP,
        idx: core::ffi::c_int,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> core::ffi::c_int;
    fn av1_set_reference_enc(
        cpi: *mut AV1_COMP,
        idx: core::ffi::c_int,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> core::ffi::c_int;
    fn av1_set_roi_map(
        cpi: *mut AV1_COMP,
        map: *mut core::ffi::c_uchar,
        rows: core::ffi::c_uint,
        cols: core::ffi::c_uint,
        delta_q: *mut core::ffi::c_int,
        delta_lf: *mut core::ffi::c_int,
        skip: *mut core::ffi::c_int,
        ref_frame: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_set_active_map(
        cpi: *mut AV1_COMP,
        map: *mut core::ffi::c_uchar,
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_active_map(
        cpi: *mut AV1_COMP,
        map: *mut core::ffi::c_uchar,
        rows: core::ffi::c_int,
        cols: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_set_internal_size(
        oxcf: *mut AV1EncoderConfig,
        resize_pending_params: *mut ResizePendingParams,
        horiz_mode: AOM_SCALING_MODE,
        vert_mode: AOM_SCALING_MODE,
    ) -> core::ffi::c_int;
    fn av1_get_quantizer(cpi: *mut AV1_COMP) -> core::ffi::c_int;
    fn av1_convert_sect5obus_to_annexb(
        buffer: *mut uint8_t,
        buffer_size: size_t,
        input_size: *mut size_t,
    ) -> core::ffi::c_int;
    fn av1_dealloc_mb_wiener_var_pred_buf(td: *mut ThreadData);
    fn av1_set_svc_seq_params(ppi: *mut AV1_PRIMARY);
    fn av1_apply_encoding_flags(cpi: *mut AV1_COMP, flags: aom_enc_frame_flags_t);
    fn av1_new_framerate(cpi: *mut AV1_COMP, framerate: core::ffi::c_double);
    fn av1_get_global_headers(ppi: *mut AV1_PRIMARY) -> *mut aom_fixed_buf_t;
    fn av1_dealloc_src_diff_buf(mb: *mut macroblock, num_planes: core::ffi::c_int);
    fn av1_select_sb_size(
        oxcf: *const AV1EncoderConfig,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        number_spatial_layers: core::ffi::c_int,
    ) -> BLOCK_SIZE;
    fn av1_save_all_coding_context(cpi: *mut AV1_COMP);
    fn av1_fp_compute_num_enc_workers(cpi: *mut AV1_COMP) -> core::ffi::c_int;
    fn av1_compute_num_workers_for_mt(cpi: *mut AV1_COMP);
    fn av1_get_max_num_workers(cpi: *const AV1_COMP) -> core::ffi::c_int;
    fn av1_create_workers(ppi: *mut AV1_PRIMARY, num_workers: core::ffi::c_int);
    fn av1_terminate_workers(ppi: *mut AV1_PRIMARY);
    fn av1_init_frame_mt(ppi: *mut AV1_PRIMARY, cpi: *mut AV1_COMP);
    fn av1_init_mt_sync(cpi: *mut AV1_COMP, is_first_pass: core::ffi::c_int);
    fn av1_get_num_mod_workers_for_alloc(
        p_mt_info: *const PrimaryMultiThreadInfo,
        mod_name: MULTI_THREADED_MODULES,
    ) -> core::ffi::c_int;
    fn av1_init_tile_thread_data(ppi: *mut AV1_PRIMARY, is_first_pass: core::ffi::c_int);
    fn av1_compute_num_fp_contexts(
        ppi: *mut AV1_PRIMARY,
        oxcf: *mut AV1EncoderConfig,
    ) -> core::ffi::c_int;
    fn av1_check_fpmt_config(
        ppi: *mut AV1_PRIMARY,
        oxcf: *const AV1EncoderConfig,
    ) -> core::ffi::c_int;
    fn av1_compress_parallel_frames(
        ppi: *mut AV1_PRIMARY,
        first_cpi_data: *mut AV1_COMP_DATA,
    );
    static g_av1_codec_arg_defs: av1_codec_arg_definitions_t;
}
pub type __builtin_va_list = *mut core::ffi::c_char;
pub type float_t = core::ffi::c_float;
pub type jmp_buf = [core::ffi::c_int; 48];
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
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
pub struct aom_codec_alg_priv {
    pub base: aom_codec_priv_t,
    pub cfg: aom_codec_enc_cfg_t,
    pub extra_cfg: av1_extracfg,
    pub timestamp_ratio: aom_rational64_t,
    pub pts_offset: aom_codec_pts_t,
    pub pts_offset_initialized: core::ffi::c_uchar,
    pub oxcf: AV1EncoderConfig,
    pub ppi: *mut AV1_PRIMARY,
    pub cx_data: *mut core::ffi::c_uchar,
    pub cx_data_sz: size_t,
    pub pending_cx_data_sz: size_t,
    pub preview_img: aom_image_t,
    pub next_frame_flags: aom_enc_frame_flags_t,
    pub pkt_list: C2RustUnnamed_11,
    pub fixed_kf_cntr: core::ffi::c_uint,
    pub buffer_pool: *mut BufferPool,
    pub buffer_pool_lap: *mut BufferPool,
    pub frame_stats_buffer: *mut FIRSTPASS_STATS,
    pub num_lap_buffers: core::ffi::c_int,
    pub stats_buf_context: STATS_BUFFER_CTX,
    pub monochrome_on_init: bool,
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
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
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
pub type MV_REFERENCE_FRAME = int8_t;
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
pub type aom_release_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type aom_get_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        size_t,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub head: aom_codec_pkt_list,
    pub alloc: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub head: aom_codec_pkt_list,
    pub pkts: [aom_codec_cx_pkt; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_cx_pkt {
    pub kind: aom_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub frame: C2RustUnnamed_14,
    pub twopass_stats: aom_fixed_buf_t,
    pub firstpass_mb_stats: aom_fixed_buf_t,
    pub psnr: aom_psnr_pkt,
    pub raw: aom_fixed_buf_t,
}
pub type aom_fixed_buf_t = aom_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
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
pub struct C2RustUnnamed_14 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_pkt_list {
    pub cnt: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
    pub pkts: [aom_codec_cx_pkt; 1],
}
pub type aom_enc_frame_flags_t = core::ffi::c_long;
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
pub type pthread_cond_t = libc::pthread_cond_t;
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
pub struct FrameProbInfo {
    pub obmc_probs: [[core::ffi::c_int; 22]; 7],
    pub warped_probs: [core::ffi::c_int; 7],
    pub tx_type_probs: [[[core::ffi::c_int; 16]; 19]; 7],
    pub switchable_interp_probs: [[[core::ffi::c_int; 3]; 16]; 7],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
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
pub type TXFM_CONTEXT = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type ENTROPY_CONTEXT = core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
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
pub type TX_SIZE = uint8_t;
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
pub type FILTER_INTRA_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERINTER_COMPOUND_DATA {
    pub seg_mask: *mut uint8_t,
    pub wedge_index: int8_t,
    pub wedge_sign: int8_t,
    pub mask_type: DIFFWTD_MASK_TYPE,
    pub type_0: COMPOUND_TYPE,
}
pub type COMPOUND_TYPE = uint8_t;
pub type DIFFWTD_MASK_TYPE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
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
pub type UV_PREDICTION_MODE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type PARTITION_TYPE = uint8_t;
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
pub type qm_val_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_2d {
    pub buf: *mut uint8_t,
    pub buf0: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type PLANE_TYPE = uint8_t;
pub type AV1_COMMON = AV1Common;
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
pub struct CommonContexts {
    pub partition: *mut *mut PARTITION_CONTEXT,
    pub entropy: [*mut *mut ENTROPY_CONTEXT; 3],
    pub txfm: *mut *mut TXFM_CONTEXT,
    pub num_planes: core::ffi::c_int,
    pub num_tile_rows: core::ffi::c_int,
    pub num_mi_cols: core::ffi::c_int,
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
pub type AV1_LEVEL = uint8_t;
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
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type BITSTREAM_PROFILE = int8_t;
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
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
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
pub type TX_TYPE = uint8_t;
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
pub type TX_MODE = uint8_t;
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
pub struct EncWorkerData {
    pub cpi: *mut AV1_COMP,
    pub td: *mut ThreadData,
    pub original_td: *mut ThreadData,
    pub error_info: aom_internal_error_info,
    pub lf_sync: *mut AV1LfSync,
    pub lf_data: *mut LFWorkerData,
    pub start: core::ffi::c_int,
    pub thread_id: core::ffi::c_int,
}
pub type LFWorkerData = LoopFilterWorkerData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_DEBLOCKING_PARAMETERS {
    pub filter_length: uint8_t,
    pub lfthr: *const loop_filter_thresh,
}
pub type AV1LfSync = AV1LfSyncData;
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
pub struct RD_STATS {
    pub rate: core::ffi::c_int,
    pub zero_rate: core::ffi::c_int,
    pub dist: int64_t,
    pub rdcost: int64_t,
    pub sse: int64_t,
    pub skip_txfm: uint8_t,
}
pub type tran_low_t = int32_t;
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
pub struct Block4x4VarInfo {
    pub log_var: core::ffi::c_double,
    pub var: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixelLevelGradientInfo {
    pub abs_dx_abs_dy_sum: uint16_t,
    pub hist_bin_idx: int8_t,
    pub is_dx_zero: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalMotionData {
    pub motion_models: [MotionModel; 1],
    pub segment_map: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
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
pub type MACROBLOCK = macroblock;
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
pub struct FullMvLimits {
    pub col_min: core::ffi::c_int,
    pub col_max: core::ffi::c_int,
    pub row_min: core::ffi::c_int,
    pub row_max: core::ffi::c_int,
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
pub struct CB_COEFF_BUFFER {
    pub tcoeff: [*mut tran_low_t; 3],
    pub eobs: [*mut uint16_t; 3],
    pub entropy_ctx: [*mut uint8_t; 3],
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
pub struct AomTplGopStats {
    pub size: core::ffi::c_int,
    pub frame_stats_list: *mut AomTplFrameStats,
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
pub union aom_int_mv {
    pub as_int: uint32_t,
    pub as_mv: AOM_MV,
    pub as_fullmv: AOM_FULLPEL_MV,
}
pub type AOM_FULLPEL_MV = aom_fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type AOM_MV = aom_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_mv {
    pub row: int16_t,
    pub col: int16_t,
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
pub type aom_sb_params = aom_sb_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_parameters {
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
}
pub type aom_rc_firstpass_stats_t = aom_rc_firstpass_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_firstpass_stats {
    pub frame_stats: *mut aom_rc_frame_stats_t,
    pub num_frames: core::ffi::c_int,
}
pub type aom_rc_frame_stats_t = aom_rc_frame_stats;
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
pub type aom_rc_config_t = aom_rc_config;
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
pub type aom_ext_rc_mode_t = aom_ext_rc_mode;
pub type aom_ext_rc_mode = core::ffi::c_uint;
pub const AOM_RC_CQ: aom_ext_rc_mode = 2;
pub const AOM_RC_VBR: aom_ext_rc_mode = 1;
pub const AOM_RC_QMODE: aom_ext_rc_mode = 0;
pub type aom_rc_funcs_t = aom_rc_funcs;
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
pub type aom_rc_delete_model_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t) -> aom_rc_status_t,
>;
pub type aom_rc_model_t = *mut core::ffi::c_void;
pub type aom_rc_status_t = aom_rc_status;
pub type aom_rc_status = core::ffi::c_uint;
pub const AOM_RC_ERROR: aom_rc_status = 1;
pub const AOM_RC_OK: aom_rc_status = 0;
pub type aom_rc_get_frame_rdmult_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_encodeframe_info_t,
        *mut core::ffi::c_int,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_encodeframe_info_t = aom_rc_encodeframe_info;
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
pub type aom_rc_get_gop_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t, *mut aom_rc_gop_decision_t) -> aom_rc_status_t,
>;
pub type aom_rc_gop_decision_t = aom_rc_gop_decision;
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
pub type aom_rc_gop_frame_t = aom_rc_gop_frame;
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
pub type aom_rc_single_ref_frame_t = aom_rc_single_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_single_ref_frame {
    pub index: core::ffi::c_int,
    pub name: aom_rc_ref_name_t,
}
pub type aom_rc_ref_name_t = aom_rc_ref_name;
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
pub type aom_rc_ref_frame_t = aom_rc_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_ref_frame {
    pub index: [core::ffi::c_int; 8],
    pub name: [aom_rc_ref_name_t; 8],
}
pub type aom_rc_frame_update_type_t = aom_rc_frame_update_type;
pub type aom_rc_frame_update_type = core::ffi::c_int;
pub const AOM_RC_INTNL_ARF_UPDATE: aom_rc_frame_update_type = 6;
pub const AOM_RC_INTNL_OVERLAY_UPDATE: aom_rc_frame_update_type = 5;
pub const AOM_RC_OVERLAY_UPDATE: aom_rc_frame_update_type = 4;
pub const AOM_RC_ARF_UPDATE: aom_rc_frame_update_type = 3;
pub const AOM_RC_GF_UPDATE: aom_rc_frame_update_type = 2;
pub const AOM_RC_LF_UPDATE: aom_rc_frame_update_type = 1;
pub const AOM_RC_KF_UPDATE: aom_rc_frame_update_type = 0;
pub const AOM_RC_INVALID_UPDATE_TYPE: aom_rc_frame_update_type = -1;
pub type aom_rc_get_key_frame_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *mut aom_rc_key_frame_decision_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_key_frame_decision_t = aom_rc_key_frame_decision;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_key_frame_decision {
    pub key_frame_show_index: core::ffi::c_int,
    pub key_frame_group_size: core::ffi::c_int,
}
pub type aom_rc_update_encodeframe_result_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_encodeframe_result_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_encodeframe_result_t = aom_rc_encodeframe_result;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_encodeframe_result {
    pub bit_count: int64_t,
    pub actual_encoding_qindex: core::ffi::c_int,
}
pub type aom_rc_get_encodeframe_decision_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        core::ffi::c_int,
        *mut aom_rc_encodeframe_decision_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_encodeframe_decision_t = aom_rc_encodeframe_decision;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rc_encodeframe_decision {
    pub q_index: core::ffi::c_int,
    pub rdmult: core::ffi::c_int,
    pub use_delta_q: *mut core::ffi::c_int,
    pub sb_params_list: *mut aom_sb_params,
}
pub type aom_rc_send_tpl_gop_stats_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_rc_model_t, *const AomTplGopStats) -> aom_rc_status_t,
>;
pub type aom_rc_send_firstpass_stats_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_rc_model_t,
        *const aom_rc_firstpass_stats_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_create_model_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *const aom_rc_config_t,
        *mut aom_rc_model_t,
    ) -> aom_rc_status_t,
>;
pub type aom_rc_type_t = aom_rc_type;
pub type aom_rc_type = core::ffi::c_uint;
pub const AOM_RC_GOP_QP_RDMULT: aom_rc_type = 7;
pub const AOM_RC_GOP_QP: aom_rc_type = 3;
pub const AOM_RC_RDMULT: aom_rc_type = 4;
pub const AOM_RC_GOP: aom_rc_type = 2;
pub const AOM_RC_QP: aom_rc_type = 1;
pub const AOM_RC_NONE: aom_rc_type = 0;
pub type aom_roi_map_t = aom_roi_map;
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
pub type FILE = libc::FILE;
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
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
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
pub type aom_ext_part_funcs_t = aom_ext_part_funcs;
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
pub type aom_ext_part_decision_mode_t = aom_ext_part_decision_mode;
pub type aom_ext_part_decision_mode = core::ffi::c_uint;
pub const AOM_EXT_PART_RECURSIVE: aom_ext_part_decision_mode = 1;
pub const AOM_EXT_PART_WHOLE_TREE: aom_ext_part_decision_mode = 0;
pub type aom_ext_part_delete_model_fn_t = Option<
    unsafe extern "C-unwind" fn(aom_ext_part_model_t) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_model_t = *mut core::ffi::c_void;
pub type aom_ext_part_status_t = aom_ext_part_status;
pub type aom_ext_part_status = core::ffi::c_uint;
pub const AOM_EXT_PART_TEST: aom_ext_part_status = 2;
pub const AOM_EXT_PART_ERROR: aom_ext_part_status = 1;
pub const AOM_EXT_PART_OK: aom_ext_part_status = 0;
pub type aom_ext_part_send_partition_stats_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *const aom_partition_stats_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_partition_stats_t = aom_partition_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_stats {
    pub rate: core::ffi::c_int,
    pub dist: int64_t,
    pub rdcost: int64_t,
}
pub type aom_ext_part_get_decision_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *mut aom_partition_decision_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_partition_decision_t = aom_partition_decision;
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
pub type aom_ext_part_send_features_fn_t = Option<
    unsafe extern "C-unwind" fn(
        aom_ext_part_model_t,
        *const aom_partition_features_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_partition_features_t = aom_partition_features;
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
pub type aom_sb_features_t = aom_sb_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_features {
    pub motion_features: aom_sb_simple_motion_features_t,
    pub tpl_features: aom_sb_tpl_features_t,
}
pub type aom_sb_tpl_features_t = aom_sb_tpl_features;
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
pub type aom_sb_simple_motion_features_t = aom_sb_simple_motion_features;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_sb_simple_motion_features {
    pub unit_length: core::ffi::c_int,
    pub num_units: core::ffi::c_int,
    pub block_sse: [core::ffi::c_int; 64],
    pub block_var: [core::ffi::c_int; 64],
}
pub type aom_partition_features_ab_t = aom_partition_features_ab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_ab {
    pub f: [core::ffi::c_float; 18],
}
pub type aom_partition_features_rect_t = aom_partition_features_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_rect {
    pub f: [core::ffi::c_float; 10],
}
pub type aom_partition_features_split_t = aom_partition_features_split;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_split {
    pub f_terminate: [core::ffi::c_float; 31],
    pub f_prune_rect: [core::ffi::c_float; 9],
}
pub type aom_partition_features_none_t = aom_partition_features_none;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_none {
    pub f: [core::ffi::c_float; 4],
    pub f_terminate: [core::ffi::c_float; 28],
}
pub type aom_partition_features_before_none_t = aom_partition_features_before_none;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_partition_features_before_none {
    pub f: [core::ffi::c_float; 17],
    pub f_part2: [core::ffi::c_float; 25],
}
pub type AOM_EXT_PART_FEATURE_ID = core::ffi::c_uint;
pub const AOM_EXT_PART_FEATURE_AFTER_AB: AOM_EXT_PART_FEATURE_ID = 7;
pub const AOM_EXT_PART_FEATURE_AFTER_RECT: AOM_EXT_PART_FEATURE_ID = 6;
pub const AOM_EXT_PART_FEATURE_AFTER_SPLIT_PART2: AOM_EXT_PART_FEATURE_ID = 5;
pub const AOM_EXT_PART_FEATURE_AFTER_SPLIT: AOM_EXT_PART_FEATURE_ID = 4;
pub const AOM_EXT_PART_FEATURE_AFTER_NONE_PART2: AOM_EXT_PART_FEATURE_ID = 3;
pub const AOM_EXT_PART_FEATURE_AFTER_NONE: AOM_EXT_PART_FEATURE_ID = 2;
pub const AOM_EXT_PART_FEATURE_BEFORE_NONE_PART2: AOM_EXT_PART_FEATURE_ID = 1;
pub const AOM_EXT_PART_FEATURE_BEFORE_NONE: AOM_EXT_PART_FEATURE_ID = 0;
pub type aom_ext_part_create_model_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *const aom_ext_part_config_t,
        *mut aom_ext_part_model_t,
    ) -> aom_ext_part_status_t,
>;
pub type aom_ext_part_config_t = aom_ext_part_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_ext_part_config {
    pub superblock_size: core::ffi::c_int,
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
pub type aom_superres_mode = core::ffi::c_uint;
pub const AOM_SUPERRES_AUTO: aom_superres_mode = 4;
pub const AOM_SUPERRES_QTHRESH: aom_superres_mode = 3;
pub const AOM_SUPERRES_RANDOM: aom_superres_mode = 2;
pub const AOM_SUPERRES_FIXED: aom_superres_mode = 1;
pub const AOM_SUPERRES_NONE: aom_superres_mode = 0;
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
pub type AOM_SVC_FRAME_DROP_MODE = core::ffi::c_uint;
pub const AOM_FULL_SUPERFRAME_DROP: AOM_SVC_FRAME_DROP_MODE = 1;
pub const AOM_LAYER_DROP: AOM_SVC_FRAME_DROP_MODE = 0;
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
pub struct aom_film_grain_table_t {
    pub head: *mut aom_film_grain_table_entry_t,
    pub tail: *mut aom_film_grain_table_entry_t,
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
pub struct AV1LrStruct {
    pub on_rest_unit: rest_unit_visitor_t,
    pub ctxt: [FilterFrameCtxt; 3],
    pub frame: *mut YV12_BUFFER_CONFIG,
    pub dst: *mut YV12_BUFFER_CONFIG,
}
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
pub struct RestorationTileLimits {
    pub h_start: core::ffi::c_int,
    pub h_end: core::ffi::c_int,
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
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
pub type AV1CdefSync = AV1CdefSyncData;
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
pub type AV1LrSync = AV1LrSyncData;
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
pub struct VarBasedPartitionInfo {
    pub thresholds: [int64_t; 5],
    pub threshold_minmax: int64_t,
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
pub struct TokenList {
    pub start: *mut TokenExtra,
    pub count: core::ffi::c_uint,
}
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
pub type SUBPEL_SEARCH_TYPE = uint8_t;
pub type aom_variance_fn_ptr_t = aom_variance_vtable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpelMvLimits {
    pub col_min: core::ffi::c_int,
    pub col_max: core::ffi::c_int,
    pub row_min: core::ffi::c_int,
    pub row_max: core::ffi::c_int,
}
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
pub type WARP_SEARCH_METHOD = uint8_t;
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
pub type SEARCH_METHODS = uint8_t;
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
pub type MODE = uint8_t;
pub type aom_enc_pass = core::ffi::c_uint;
pub const AOM_RC_LAST_PASS: aom_enc_pass = 2;
pub const AOM_RC_THIRD_PASS: aom_enc_pass = 3;
pub const AOM_RC_SECOND_PASS: aom_enc_pass = 2;
pub const AOM_RC_FIRST_PASS: aom_enc_pass = 1;
pub const AOM_RC_ONE_PASS: aom_enc_pass = 0;
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
pub type aom_dist_metric = core::ffi::c_uint;
pub const AOM_DIST_METRIC_QM_PSNR: aom_dist_metric = 1;
pub const AOM_DIST_METRIC_PSNR: aom_dist_metric = 0;
pub type aom_tune_content = core::ffi::c_uint;
pub const AOM_CONTENT_INVALID: aom_tune_content = 3;
pub const AOM_CONTENT_FILM: aom_tune_content = 2;
pub const AOM_CONTENT_SCREEN: aom_tune_content = 1;
pub const AOM_CONTENT_DEFAULT: aom_tune_content = 0;
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
pub type aom_superblock_size_t = aom_superblock_size;
pub type aom_superblock_size = core::ffi::c_uint;
pub const AOM_SUPERBLOCK_SIZE_DYNAMIC: aom_superblock_size = 2;
pub const AOM_SUPERBLOCK_SIZE_128X128: aom_superblock_size = 1;
pub const AOM_SUPERBLOCK_SIZE_64X64: aom_superblock_size = 0;
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
pub type aom_rc_mode = core::ffi::c_uint;
pub const AOM_Q: aom_rc_mode = 3;
pub const AOM_CQ: aom_rc_mode = 2;
pub const AOM_CBR: aom_rc_mode = 1;
pub const AOM_VBR: aom_rc_mode = 0;
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
pub type aom_screen_detection_mode = core::ffi::c_uint;
pub const AOM_SCREEN_DETECTION_ANTIALIASING_AWARE: aom_screen_detection_mode = 2;
pub const AOM_SCREEN_DETECTION_STANDARD: aom_screen_detection_mode = 1;
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
pub struct av1_extracfg {
    pub cpu_used: core::ffi::c_int,
    pub enable_auto_alt_ref: core::ffi::c_uint,
    pub enable_auto_bwd_ref: core::ffi::c_uint,
    pub noise_sensitivity: core::ffi::c_uint,
    pub sharpness: core::ffi::c_uint,
    pub enable_adaptive_sharpness: core::ffi::c_uint,
    pub static_thresh: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub fp_mt: core::ffi::c_uint,
    pub tile_columns: core::ffi::c_uint,
    pub tile_rows: core::ffi::c_uint,
    pub auto_tiles: core::ffi::c_uint,
    pub enable_tpl_model: core::ffi::c_uint,
    pub enable_keyframe_filtering: core::ffi::c_uint,
    pub arnr_max_frames: core::ffi::c_uint,
    pub arnr_strength: core::ffi::c_uint,
    pub min_gf_interval: core::ffi::c_uint,
    pub max_gf_interval: core::ffi::c_uint,
    pub gf_min_pyr_height: core::ffi::c_uint,
    pub gf_max_pyr_height: core::ffi::c_uint,
    pub tuning: aom_tune_metric,
    pub vmaf_model_path: *const core::ffi::c_char,
    pub partition_info_path: *const core::ffi::c_char,
    pub enable_rate_guide_deltaq: core::ffi::c_uint,
    pub rate_distribution_info: *const core::ffi::c_char,
    pub dist_metric: aom_dist_metric,
    pub cq_level: core::ffi::c_uint,
    pub rc_max_intra_bitrate_pct: core::ffi::c_uint,
    pub rc_max_inter_bitrate_pct: core::ffi::c_uint,
    pub gf_cbr_boost_pct: core::ffi::c_uint,
    pub lossless: core::ffi::c_uint,
    pub enable_cdef: core::ffi::c_uint,
    pub enable_restoration: core::ffi::c_uint,
    pub force_video_mode: core::ffi::c_uint,
    pub enable_obmc: core::ffi::c_uint,
    pub disable_trellis_quant: core::ffi::c_uint,
    pub enable_qm: core::ffi::c_uint,
    pub qm_y: core::ffi::c_uint,
    pub qm_u: core::ffi::c_uint,
    pub qm_v: core::ffi::c_uint,
    pub qm_min: core::ffi::c_uint,
    pub qm_max: core::ffi::c_uint,
    pub num_tg: core::ffi::c_uint,
    pub mtu_size: core::ffi::c_uint,
    pub timing_info_type: aom_timing_info_type_t,
    pub frame_parallel_decoding_mode: core::ffi::c_uint,
    pub enable_dual_filter: core::ffi::c_int,
    pub enable_chroma_deltaq: core::ffi::c_uint,
    pub aq_mode: AQ_MODE,
    pub deltaq_mode: DELTAQ_MODE,
    pub deltaq_strength: core::ffi::c_uint,
    pub deltalf_mode: core::ffi::c_int,
    pub frame_periodic_boost: core::ffi::c_uint,
    pub content: aom_tune_content,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub color_range: core::ffi::c_int,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub superblock_size: aom_superblock_size_t,
    pub single_tile_decoding: core::ffi::c_uint,
    pub error_resilient_mode: core::ffi::c_int,
    pub s_frame_mode: core::ffi::c_int,
    pub film_grain_test_vector: core::ffi::c_int,
    pub film_grain_table_filename: *const core::ffi::c_char,
    pub motion_vector_unit_test: core::ffi::c_uint,
    pub cdf_update_mode: core::ffi::c_uint,
    pub enable_rect_partitions: core::ffi::c_int,
    pub enable_ab_partitions: core::ffi::c_int,
    pub enable_1to4_partitions: core::ffi::c_int,
    pub min_partition_size: core::ffi::c_int,
    pub max_partition_size: core::ffi::c_int,
    pub enable_intra_edge_filter: core::ffi::c_int,
    pub enable_order_hint: core::ffi::c_int,
    pub enable_tx64: core::ffi::c_int,
    pub enable_flip_idtx: core::ffi::c_int,
    pub enable_rect_tx: core::ffi::c_int,
    pub enable_dist_wtd_comp: core::ffi::c_int,
    pub max_reference_frames: core::ffi::c_int,
    pub enable_reduced_reference_set: core::ffi::c_int,
    pub enable_ref_frame_mvs: core::ffi::c_int,
    pub allow_ref_frame_mvs: core::ffi::c_int,
    pub enable_masked_comp: core::ffi::c_int,
    pub enable_onesided_comp: core::ffi::c_int,
    pub enable_interintra_comp: core::ffi::c_int,
    pub enable_smooth_interintra: core::ffi::c_int,
    pub enable_diff_wtd_comp: core::ffi::c_int,
    pub enable_interinter_wedge: core::ffi::c_int,
    pub enable_interintra_wedge: core::ffi::c_int,
    pub enable_global_motion: core::ffi::c_int,
    pub enable_warped_motion: core::ffi::c_int,
    pub allow_warped_motion: core::ffi::c_int,
    pub enable_filter_intra: core::ffi::c_int,
    pub enable_smooth_intra: core::ffi::c_int,
    pub enable_paeth_intra: core::ffi::c_int,
    pub enable_cfl_intra: core::ffi::c_int,
    pub enable_directional_intra: core::ffi::c_int,
    pub enable_diagonal_intra: core::ffi::c_int,
    pub enable_superres: core::ffi::c_int,
    pub enable_overlay: core::ffi::c_int,
    pub enable_palette: core::ffi::c_int,
    pub enable_intrabc: core::ffi::c_int,
    pub enable_angle_delta: core::ffi::c_int,
    pub noise_level: core::ffi::c_float,
    pub noise_block_size: core::ffi::c_int,
    pub enable_dnl_denoising: core::ffi::c_int,
    pub enable_low_complexity_decode: core::ffi::c_uint,
    pub chroma_subsampling_x: core::ffi::c_uint,
    pub chroma_subsampling_y: core::ffi::c_uint,
    pub reduced_tx_type_set: core::ffi::c_int,
    pub use_intra_dct_only: core::ffi::c_int,
    pub use_inter_dct_only: core::ffi::c_int,
    pub use_intra_default_tx_only: core::ffi::c_int,
    pub enable_tx_size_search: core::ffi::c_int,
    pub quant_b_adapt: core::ffi::c_int,
    pub vbr_corpus_complexity_lap: core::ffi::c_uint,
    pub target_seq_level_idx: [AV1_LEVEL; 32],
    pub tier_mask: core::ffi::c_uint,
    pub min_cr: core::ffi::c_uint,
    pub coeff_cost_upd_freq: COST_UPDATE_TYPE,
    pub mode_cost_upd_freq: COST_UPDATE_TYPE,
    pub mv_cost_upd_freq: COST_UPDATE_TYPE,
    pub dv_cost_upd_freq: COST_UPDATE_TYPE,
    pub ext_tile_debug: core::ffi::c_uint,
    pub sb_multipass_unit_test: core::ffi::c_uint,
    pub passes: core::ffi::c_int,
    pub fwd_kf_dist: core::ffi::c_int,
    pub loopfilter_control: LOOPFILTER_CONTROL,
    pub skip_postproc_filtering: core::ffi::c_uint,
    pub mode_ref_delta_enabled: core::ffi::c_int,
    pub two_pass_output: *const core::ffi::c_char,
    pub second_pass_log: *const core::ffi::c_char,
    pub auto_intra_tools_off: core::ffi::c_int,
    pub strict_level_conformance: core::ffi::c_int,
    pub kf_max_pyr_height: core::ffi::c_int,
    pub sb_qp_sweep: core::ffi::c_int,
    pub screen_detection_mode: aom_screen_detection_mode,
    pub validate_hbd_input: core::ffi::c_uint,
    pub force_max_q: core::ffi::c_uint,
}
pub type aom_timing_info_type_t = core::ffi::c_uint;
pub const AOM_TIMING_DEC_MODEL: aom_timing_info_type_t = 2;
pub const AOM_TIMING_EQUAL: aom_timing_info_type_t = 1;
pub const AOM_TIMING_UNSPECIFIED: aom_timing_info_type_t = 0;
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
pub type aom_codec_er_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
pub type aom_codec_priv_t = aom_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_priv {
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub enc: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub cx_data_dst_buf: aom_fixed_buf_t,
    pub cx_data_pad_before: core::ffi::c_uint,
    pub cx_data_pad_after: core::ffi::c_uint,
    pub cx_data_pkt: aom_codec_cx_pkt_t,
}
pub type aom_codec_cx_pkt_t = aom_codec_cx_pkt;
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
pub type aom_codec_enc_config_set_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_codec_enc_cfg_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_get_cx_data_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_iter_t,
    ) -> *const aom_codec_cx_pkt_t,
>;
pub type aom_codec_iter_t = *const core::ffi::c_void;
pub type aom_codec_encode_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_image_t,
        aom_codec_pts_t,
        core::ffi::c_ulong,
        aom_enc_frame_flags_t,
    ) -> aom_codec_err_t,
>;
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
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t, va_list) -> aom_codec_err_t,
>;
pub type va_list<'a> = core::ffi::VaList<'a>;
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
    pub config: C2RustUnnamed_16,
    pub priv_0: *mut aom_codec_priv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
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
pub type OBU_TYPE = core::ffi::c_uchar;
pub const OBU_PADDING: OBU_TYPE = 15;
pub const OBU_TILE_LIST: OBU_TYPE = 8;
pub const OBU_REDUNDANT_FRAME_HEADER: OBU_TYPE = 7;
pub const OBU_FRAME: OBU_TYPE = 6;
pub const OBU_METADATA: OBU_TYPE = 5;
pub const OBU_TILE_GROUP: OBU_TYPE = 4;
pub const OBU_FRAME_HEADER: OBU_TYPE = 3;
pub const OBU_TEMPORAL_DELIMITER: OBU_TYPE = 2;
pub const OBU_SEQUENCE_HEADER: OBU_TYPE = 1;
pub type aom_com_control_id = core::ffi::c_uint;
pub const AOM_DECODER_CTRL_ID_START: aom_com_control_id = 256;
pub const AV1_COPY_NEW_FRAME_IMAGE: aom_com_control_id = 234;
pub const AV1_GET_NEW_FRAME_IMAGE: aom_com_control_id = 233;
pub const AV1_COPY_REFERENCE: aom_com_control_id = 232;
pub const AV1_SET_REFERENCE: aom_com_control_id = 231;
pub const AV1_GET_REFERENCE: aom_com_control_id = 230;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct av1_ref_frame {
    pub idx: core::ffi::c_int,
    pub use_external_ref: core::ffi::c_int,
    pub img: aom_image_t,
}
pub type av1_ref_frame_t = av1_ref_frame;
pub type aom_rational_t = aom_rational;
pub const SEQ_LEVEL_KEEP_STATS: C2RustUnnamed_18 = 32;
pub const SEQ_LEVEL_8_3: C2RustUnnamed_18 = 27;
pub const SEQ_LEVEL_8_2: C2RustUnnamed_18 = 26;
pub const SEQ_LEVEL_8_1: C2RustUnnamed_18 = 25;
pub const SEQ_LEVEL_8_0: C2RustUnnamed_18 = 24;
pub const SEQ_LEVEL_7_3: C2RustUnnamed_18 = 23;
pub const SEQ_LEVEL_7_2: C2RustUnnamed_18 = 22;
pub const SEQ_LEVEL_7_1: C2RustUnnamed_18 = 21;
pub const SEQ_LEVEL_7_0: C2RustUnnamed_18 = 20;
pub const SEQ_LEVEL_4_3: C2RustUnnamed_18 = 11;
pub const SEQ_LEVEL_4_2: C2RustUnnamed_18 = 10;
pub const SEQ_LEVEL_3_3: C2RustUnnamed_18 = 7;
pub const SEQ_LEVEL_3_2: C2RustUnnamed_18 = 6;
pub const SEQ_LEVEL_2_3: C2RustUnnamed_18 = 3;
pub const SEQ_LEVEL_2_2: C2RustUnnamed_18 = 2;
pub const SEQ_LEVELS: C2RustUnnamed_18 = 28;
pub const SEQ_LEVEL_MAX: C2RustUnnamed_18 = 31;
pub const PROFILE_1: C2RustUnnamed_17 = 1;
pub const RESIZE_MODES: C2RustUnnamed_27 = 4;
pub const DELTA_Q_MODE_COUNT: C2RustUnnamed_26 = 7;
pub const AQ_MODE_COUNT: C2RustUnnamed_25 = 4;
pub const MAX_PROFILES: C2RustUnnamed_17 = 3;
pub const GOOD: C2RustUnnamed_21 = 0;
pub const REALTIME: C2RustUnnamed_21 = 1;
pub const RESIZE_NONE: C2RustUnnamed_27 = 0;
pub const RESIZE_FIXED: C2RustUnnamed_27 = 1;
pub const NO_DELTA_Q: C2RustUnnamed_26 = 0;
pub const DELTA_Q_HDR: C2RustUnnamed_26 = 5;
pub const ALLINTRA: C2RustUnnamed_21 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg {
    pub argv: *mut *mut core::ffi::c_char,
    pub name: *const core::ffi::c_char,
    pub val: *const core::ffi::c_char,
    pub argv_step: core::ffi::c_uint,
    pub def: *const arg_def,
}
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
pub type av1_codec_arg_definitions_t = av1_codec_arg_definitions;
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
pub const DELTA_Q_OBJECTIVE: C2RustUnnamed_26 = 1;
pub const NO_AQ: C2RustUnnamed_25 = 0;
pub const DELTA_Q_VARIANCE_BOOST: C2RustUnnamed_26 = 6;
pub const FRAMEFLAGS_ERROR_RESILIENT: C2RustUnnamed_24 = 64;
pub const FRAMEFLAGS_SWITCH: C2RustUnnamed_24 = 32;
pub const FRAMEFLAGS_INTRAONLY: C2RustUnnamed_24 = 16;
pub const FRAMEFLAGS_KEY: C2RustUnnamed_24 = 1;
pub const KEY_FRAME: C2RustUnnamed_20 = 0;
pub const GOLDEN_FRAME: C2RustUnnamed_19 = 4;
pub const LAST_FRAME: C2RustUnnamed_19 = 1;
pub const EXTREF_FRAME: C2RustUnnamed_19 = 8;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_19 = 7;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_20 = 2;
pub const ENCODE_STAGE: C2RustUnnamed_22 = 0;
pub const SEARCH_PARTITION: C2RustUnnamed_23 = 0;
pub const MOD_TPL: MULTI_THREADED_MODULES = 2;
pub const MOD_TF: MULTI_THREADED_MODULES = 1;
pub type MULTI_THREADED_MODULES = core::ffi::c_uint;
pub const NUM_MT_MODULES: MULTI_THREADED_MODULES = 12;
pub const MOD_AI: MULTI_THREADED_MODULES = 11;
pub const MOD_FRAME_ENC: MULTI_THREADED_MODULES = 10;
pub const MOD_PACK_BS: MULTI_THREADED_MODULES = 9;
pub const MOD_LR: MULTI_THREADED_MODULES = 8;
pub const MOD_CDEF: MULTI_THREADED_MODULES = 7;
pub const MOD_CDEF_SEARCH: MULTI_THREADED_MODULES = 6;
pub const MOD_LPF: MULTI_THREADED_MODULES = 5;
pub const MOD_ENC: MULTI_THREADED_MODULES = 4;
pub const MOD_GME: MULTI_THREADED_MODULES = 3;
pub const MOD_FP: MULTI_THREADED_MODULES = 0;
pub const LAP_STAGE: C2RustUnnamed_22 = 1;
pub const PROFILE_2: C2RustUnnamed_17 = 2;
pub const PROFILE_0: C2RustUnnamed_17 = 0;
pub type aom_gop_info_t = aom_gop_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_gop_info {
    pub gop_size: core::ffi::c_int,
    pub coding_index: [core::ffi::c_int; 250],
    pub display_index: [core::ffi::c_int; 250],
    pub layer_depth: [core::ffi::c_int; 250],
}
pub const AV1E_GET_GOP_INFO: aome_enc_control_id = 174;
pub const AV1E_GET_HIGH_MOTION_CONTENT_SCREEN_RTC: aome_enc_control_id = 167;
pub const AV1E_GET_LUMA_CDEF_STRENGTH: aome_enc_control_id = 162;
pub const AV1E_GET_NUM_OPERATING_POINTS: aome_enc_control_id = 156;
pub const AV1E_GET_TARGET_SEQ_LEVEL_IDX: aome_enc_control_id = 155;
pub const AV1E_GET_BASELINE_GF_INTERVAL: aome_enc_control_id = 139;
pub const AV1E_GET_SEQ_LEVEL_IDX: aome_enc_control_id = 55;
pub type aom_codec_control_type_AV1E_SET_CHROMA_SUBSAMPLING_Y = core::ffi::c_uint;
pub const AV1E_SET_CHROMA_SUBSAMPLING_Y: aome_enc_control_id = 117;
pub type aom_codec_control_type_AV1E_SET_CHROMA_SUBSAMPLING_X = core::ffi::c_uint;
pub const AV1E_SET_CHROMA_SUBSAMPLING_X: aome_enc_control_id = 116;
pub type aom_active_map_t = aom_active_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_active_map {
    pub active_map: *mut core::ffi::c_uchar,
    pub rows: core::ffi::c_uint,
    pub cols: core::ffi::c_uint,
}
pub const AV1E_GET_ACTIVEMAP: aome_enc_control_id = 51;
pub const REF_FRAMES: C2RustUnnamed_19 = 8;
pub const AOME_GET_LOOPFILTER_LEVEL: aome_enc_control_id = 150;
pub const AOME_GET_LAST_QUANTIZER_64: aome_enc_control_id = 20;
pub const AOME_GET_LAST_QUANTIZER: aome_enc_control_id = 19;
pub type aom_codec_control_type_AOME_SET_VALIDATE_HBD_INPUT = core::ffi::c_int;
pub const AOME_SET_VALIDATE_HBD_INPUT: aome_enc_control_id = 175;
pub type aom_codec_control_type_AV1E_SET_EXTERNAL_RATE_CONTROL = *mut aom_rc_funcs_t;
pub const AV1E_SET_EXTERNAL_RATE_CONTROL: aome_enc_control_id = 173;
pub type aom_codec_control_type_AV1E_SET_ENABLE_ADAPTIVE_SHARPNESS = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_ADAPTIVE_SHARPNESS: aome_enc_control_id = 172;
pub type aom_codec_control_type_AV1E_SET_SCREEN_CONTENT_DETECTION_MODE = core::ffi::c_int;
pub const AV1E_SET_SCREEN_CONTENT_DETECTION_MODE: aome_enc_control_id = 171;
pub type aom_codec_control_type_AV1E_SET_ENABLE_LOW_COMPLEXITY_DECODE = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_LOW_COMPLEXITY_DECODE: aome_enc_control_id = 170;
pub type aom_codec_control_type_AV1E_SET_MAX_CONSEC_FRAME_DROP_MS_CBR = core::ffi::c_int;
pub const AV1E_SET_MAX_CONSEC_FRAME_DROP_MS_CBR: aome_enc_control_id = 169;
pub type aom_codec_control_type_AV1E_SET_POSTENCODE_DROP_RTC = core::ffi::c_int;
pub const AV1E_SET_POSTENCODE_DROP_RTC: aome_enc_control_id = 168;
pub type aom_codec_control_type_AV1E_SET_AUTO_TILES = core::ffi::c_uint;
pub const AV1E_SET_AUTO_TILES: aome_enc_control_id = 166;
pub type aom_codec_control_type_AV1E_SET_SVC_FRAME_DROP_MODE = core::ffi::c_uint;
pub const AV1E_SET_SVC_FRAME_DROP_MODE: aome_enc_control_id = 165;
pub type aom_codec_control_type_AV1E_SET_MAX_CONSEC_FRAME_DROP_CBR = core::ffi::c_int;
pub const AV1E_SET_MAX_CONSEC_FRAME_DROP_CBR: aome_enc_control_id = 164;
pub type aom_codec_control_type_AV1E_SET_BITRATE_ONE_PASS_CBR = core::ffi::c_uint;
pub const AV1E_SET_BITRATE_ONE_PASS_CBR: aome_enc_control_id = 163;
pub type aom_codec_control_type_AV1E_SET_QUANTIZER_ONE_PASS = core::ffi::c_int;
pub const AV1E_SET_QUANTIZER_ONE_PASS: aome_enc_control_id = 159;
pub type aom_codec_control_type_AV1E_SET_RTC_EXTERNAL_RC = core::ffi::c_int;
pub const AV1E_SET_RTC_EXTERNAL_RC: aome_enc_control_id = 152;
pub type aom_codec_control_type_AV1E_SET_AUTO_INTRA_TOOLS_OFF = core::ffi::c_uint;
pub const AV1E_SET_AUTO_INTRA_TOOLS_OFF: aome_enc_control_id = 151;
pub type aom_codec_control_type_AV1E_SET_MODE_REF_DELTA_ENABLED = core::ffi::c_int;
pub const AV1E_SET_MODE_REF_DELTA_ENABLED: aome_enc_control_id = 176;
pub type aom_codec_control_type_AV1E_SET_SKIP_POSTPROC_FILTERING = core::ffi::c_uint;
pub const AV1E_SET_SKIP_POSTPROC_FILTERING: aome_enc_control_id = 157;
pub type aom_codec_control_type_AV1E_SET_LOOPFILTER_CONTROL = core::ffi::c_int;
pub const AV1E_SET_LOOPFILTER_CONTROL: aome_enc_control_id = 149;
pub type aom_codec_control_type_AV1E_SET_ENABLE_TX_SIZE_SEARCH = core::ffi::c_int;
pub const AV1E_SET_ENABLE_TX_SIZE_SEARCH: aome_enc_control_id = 146;
pub type aom_codec_control_type_AV1E_SET_EXTERNAL_PARTITION = *mut aom_ext_part_funcs_t;
pub const AV1E_SET_EXTERNAL_PARTITION: aome_enc_control_id = 144;
pub type aom_codec_control_type_AV1E_SET_DV_COST_UPD_FREQ = core::ffi::c_uint;
pub const AV1E_SET_DV_COST_UPD_FREQ: aome_enc_control_id = 142;
pub type aom_codec_control_type_AV1E_ENABLE_SB_QP_SWEEP = core::ffi::c_uint;
pub const AV1E_ENABLE_SB_QP_SWEEP: aome_enc_control_id = 158;
pub type aom_codec_control_type_AV1E_ENABLE_SB_MULTIPASS_UNIT_TEST = core::ffi::c_uint;
pub const AV1E_ENABLE_SB_MULTIPASS_UNIT_TEST: aome_enc_control_id = 136;
pub type aom_codec_control_type_AV1E_SET_VBR_CORPUS_COMPLEXITY_LAP = core::ffi::c_uint;
pub const AV1E_SET_VBR_CORPUS_COMPLEXITY_LAP: aome_enc_control_id = 138;
pub type aom_svc_ref_frame_comp_pred_t = aom_svc_ref_frame_comp_pred;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_svc_ref_frame_comp_pred {
    pub use_comp_pred: [core::ffi::c_int; 3],
}
pub const AV1E_SET_SVC_REF_FRAME_COMP_PRED: aome_enc_control_id = 147;
pub type aom_svc_ref_frame_config_t = aom_svc_ref_frame_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_svc_ref_frame_config {
    pub reference: [core::ffi::c_int; 7],
    pub ref_idx: [core::ffi::c_int; 7],
    pub refresh: [core::ffi::c_int; 8],
}
pub const AV1E_SET_SVC_REF_FRAME_CONFIG: aome_enc_control_id = 133;
pub type aom_svc_params_t = aom_svc_params;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_svc_params {
    pub number_spatial_layers: core::ffi::c_int,
    pub number_temporal_layers: core::ffi::c_int,
    pub max_quantizers: [core::ffi::c_int; 32],
    pub min_quantizers: [core::ffi::c_int; 32],
    pub scaling_factor_num: [core::ffi::c_int; 4],
    pub scaling_factor_den: [core::ffi::c_int; 4],
    pub layer_target_bitrate: [core::ffi::c_int; 32],
    pub framerate_factor: [core::ffi::c_int; 8],
}
pub const AV1E_SET_SVC_PARAMS: aome_enc_control_id = 132;
pub type aom_svc_layer_id_t = aom_svc_layer_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_svc_layer_id {
    pub spatial_layer_id: core::ffi::c_int,
    pub temporal_layer_id: core::ffi::c_int,
}
pub const AV1E_SET_SVC_LAYER_ID: aome_enc_control_id = 131;
pub type aom_codec_control_type_AV1E_SET_MIN_CR = core::ffi::c_uint;
pub const AV1E_SET_MIN_CR: aome_enc_control_id = 130;
pub type aom_codec_control_type_AV1E_SET_TIER_MASK = core::ffi::c_uint;
pub const AV1E_SET_TIER_MASK: aome_enc_control_id = 129;
pub type aom_codec_control_type_AV1E_SET_TARGET_SEQ_LEVEL_IDX = core::ffi::c_int;
pub const AV1E_SET_TARGET_SEQ_LEVEL_IDX: aome_enc_control_id = 54;
pub type aom_codec_control_type_AV1E_ENABLE_EXT_TILE_DEBUG = core::ffi::c_uint;
pub const AV1E_ENABLE_EXT_TILE_DEBUG: aome_enc_control_id = 135;
pub const AV1E_SET_FP_MT_UNIT_TEST: aome_enc_control_id = 154;
pub type aom_codec_control_type_AV1E_ENABLE_MOTION_VECTOR_UNIT_TEST = core::ffi::c_uint;
pub const AV1E_ENABLE_MOTION_VECTOR_UNIT_TEST: aome_enc_control_id = 110;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DNL_DENOISING = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DNL_DENOISING: aome_enc_control_id = 140;
pub type aom_codec_control_type_AV1E_SET_DENOISE_BLOCK_SIZE = core::ffi::c_uint;
pub const AV1E_SET_DENOISE_BLOCK_SIZE: aome_enc_control_id = 115;
pub type aom_codec_control_type_AV1E_SET_DENOISE_NOISE_LEVEL = core::ffi::c_int;
pub const AV1E_SET_DENOISE_NOISE_LEVEL: aome_enc_control_id = 114;
pub type aom_codec_control_type_AV1E_SET_FILM_GRAIN_TABLE = *const core::ffi::c_char;
pub const AV1E_SET_FILM_GRAIN_TABLE: aome_enc_control_id = 113;
pub type aom_codec_control_type_AV1E_SET_FILM_GRAIN_TEST_VECTOR = core::ffi::c_int;
pub const AV1E_SET_FILM_GRAIN_TEST_VECTOR: aome_enc_control_id = 112;
pub type aom_codec_control_type_AV1E_SET_RATE_DISTRIBUTION_INFO = *const core::ffi::c_char;
pub const AV1E_SET_RATE_DISTRIBUTION_INFO: aome_enc_control_id = 161;
pub type aom_codec_control_type_AV1E_ENABLE_RATE_GUIDE_DELTAQ = core::ffi::c_uint;
pub const AV1E_ENABLE_RATE_GUIDE_DELTAQ: aome_enc_control_id = 160;
pub type aom_codec_control_type_AV1E_SET_PARTITION_INFO_PATH = *const core::ffi::c_char;
pub const AV1E_SET_PARTITION_INFO_PATH: aome_enc_control_id = 143;
pub type aom_codec_control_type_AV1E_SET_VMAF_MODEL_PATH = *const core::ffi::c_char;
pub const AV1E_SET_VMAF_MODEL_PATH: aome_enc_control_id = 134;
pub type aom_codec_control_type_AV1E_SET_SINGLE_TILE_DECODING = core::ffi::c_uint;
pub const AV1E_SET_SINGLE_TILE_DECODING: aome_enc_control_id = 109;
pub type aom_codec_control_type_AV1E_SET_SUPERBLOCK_SIZE = core::ffi::c_uint;
pub const AV1E_SET_SUPERBLOCK_SIZE: aome_enc_control_id = 56;
pub const AV1E_SET_RENDER_SIZE: aome_enc_control_id = 53;
pub type aom_codec_control_type_AV1E_SET_GF_MAX_PYRAMID_HEIGHT = core::ffi::c_uint;
pub const AV1E_SET_GF_MAX_PYRAMID_HEIGHT: aome_enc_control_id = 123;
pub type aom_codec_control_type_AV1E_SET_GF_MIN_PYRAMID_HEIGHT = core::ffi::c_uint;
pub const AV1E_SET_GF_MIN_PYRAMID_HEIGHT: aome_enc_control_id = 137;
pub type aom_codec_control_type_AV1E_SET_MAX_GF_INTERVAL = core::ffi::c_uint;
pub const AV1E_SET_MAX_GF_INTERVAL: aome_enc_control_id = 50;
pub type aom_codec_control_type_AV1E_SET_MIN_GF_INTERVAL = core::ffi::c_uint;
pub const AV1E_SET_MIN_GF_INTERVAL: aome_enc_control_id = 49;
pub type aom_codec_control_type_AV1E_SET_NOISE_SENSITIVITY = core::ffi::c_uint;
pub const AV1E_SET_NOISE_SENSITIVITY: aome_enc_control_id = 42;
pub type aom_codec_control_type_AV1E_SET_COLOR_RANGE = core::ffi::c_int;
pub const AV1E_SET_COLOR_RANGE: aome_enc_control_id = 52;
pub type aom_codec_control_type_AV1E_SET_CHROMA_SAMPLE_POSITION = core::ffi::c_int;
pub const AV1E_SET_CHROMA_SAMPLE_POSITION: aome_enc_control_id = 48;
pub type aom_codec_control_type_AV1E_SET_MATRIX_COEFFICIENTS = core::ffi::c_int;
pub const AV1E_SET_MATRIX_COEFFICIENTS: aome_enc_control_id = 47;
pub type aom_codec_control_type_AV1E_SET_TRANSFER_CHARACTERISTICS = core::ffi::c_int;
pub const AV1E_SET_TRANSFER_CHARACTERISTICS: aome_enc_control_id = 46;
pub type aom_codec_control_type_AV1E_SET_COLOR_PRIMARIES = core::ffi::c_int;
pub const AV1E_SET_COLOR_PRIMARIES: aome_enc_control_id = 45;
pub type aom_codec_control_type_AV1E_SET_CDF_UPDATE_MODE = core::ffi::c_uint;
pub const AV1E_SET_CDF_UPDATE_MODE: aome_enc_control_id = 44;
pub type aom_codec_control_type_AV1E_SET_TUNE_CONTENT = core::ffi::c_int;
pub const AV1E_SET_TUNE_CONTENT: aome_enc_control_id = 43;
pub type aom_codec_control_type_AV1E_SET_FRAME_PERIODIC_BOOST = core::ffi::c_uint;
pub const AV1E_SET_FRAME_PERIODIC_BOOST: aome_enc_control_id = 41;
pub type aom_codec_control_type_AV1E_SET_DELTALF_MODE = core::ffi::c_uint;
pub const AV1E_SET_DELTALF_MODE: aome_enc_control_id = 108;
pub type aom_codec_control_type_AV1E_SET_DELTAQ_STRENGTH = core::ffi::c_uint;
pub const AV1E_SET_DELTAQ_STRENGTH: aome_enc_control_id = 148;
pub type aom_codec_control_type_AV1E_SET_DELTAQ_MODE = core::ffi::c_uint;
pub const AV1E_SET_DELTAQ_MODE: aome_enc_control_id = 107;
pub type aom_codec_control_type_AV1E_SET_MV_COST_UPD_FREQ = core::ffi::c_uint;
pub const AV1E_SET_MV_COST_UPD_FREQ: aome_enc_control_id = 128;
pub type aom_codec_control_type_AV1E_SET_MODE_COST_UPD_FREQ = core::ffi::c_uint;
pub const AV1E_SET_MODE_COST_UPD_FREQ: aome_enc_control_id = 127;
pub type aom_codec_control_type_AV1E_SET_COEFF_COST_UPD_FREQ = core::ffi::c_uint;
pub const AV1E_SET_COEFF_COST_UPD_FREQ: aome_enc_control_id = 126;
pub type aom_codec_control_type_AV1E_SET_QUANT_B_ADAPT = core::ffi::c_int;
pub const AV1E_SET_QUANT_B_ADAPT: aome_enc_control_id = 122;
pub type aom_codec_control_type_AV1E_SET_INTRA_DEFAULT_TX_ONLY = core::ffi::c_int;
pub const AV1E_SET_INTRA_DEFAULT_TX_ONLY: aome_enc_control_id = 121;
pub type aom_codec_control_type_AV1E_SET_INTER_DCT_ONLY = core::ffi::c_int;
pub const AV1E_SET_INTER_DCT_ONLY: aome_enc_control_id = 120;
pub type aom_codec_control_type_AV1E_SET_INTRA_DCT_ONLY = core::ffi::c_int;
pub const AV1E_SET_INTRA_DCT_ONLY: aome_enc_control_id = 119;
pub type aom_codec_control_type_AV1E_SET_REDUCED_TX_TYPE_SET = core::ffi::c_int;
pub const AV1E_SET_REDUCED_TX_TYPE_SET: aome_enc_control_id = 118;
pub type aom_codec_control_type_AV1E_SET_AQ_MODE = core::ffi::c_uint;
pub const AV1E_SET_AQ_MODE: aome_enc_control_id = 40;
pub type aom_codec_control_type_AV1E_SET_ENABLE_ANGLE_DELTA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_ANGLE_DELTA: aome_enc_control_id = 106;
pub type aom_codec_control_type_AV1E_SET_ENABLE_INTRABC = core::ffi::c_int;
pub const AV1E_SET_ENABLE_INTRABC: aome_enc_control_id = 105;
pub type aom_codec_control_type_AV1E_SET_ENABLE_PALETTE = core::ffi::c_int;
pub const AV1E_SET_ENABLE_PALETTE: aome_enc_control_id = 104;
pub type aom_codec_control_type_AV1E_SET_ENABLE_OVERLAY = core::ffi::c_int;
pub const AV1E_SET_ENABLE_OVERLAY: aome_enc_control_id = 103;
pub type aom_codec_control_type_AV1E_SET_ENABLE_SUPERRES = core::ffi::c_int;
pub const AV1E_SET_ENABLE_SUPERRES: aome_enc_control_id = 102;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DIAGONAL_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DIAGONAL_INTRA: aome_enc_control_id = 141;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DIRECTIONAL_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DIRECTIONAL_INTRA: aome_enc_control_id = 145;
pub type aom_codec_control_type_AV1E_SET_ENABLE_CFL_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_CFL_INTRA: aome_enc_control_id = 101;
pub type aom_codec_control_type_AV1E_SET_ENABLE_PAETH_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_PAETH_INTRA: aome_enc_control_id = 100;
pub type aom_codec_control_type_AV1E_SET_ENABLE_SMOOTH_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_SMOOTH_INTRA: aome_enc_control_id = 99;
pub type aom_codec_control_type_AV1E_SET_ENABLE_FILTER_INTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_FILTER_INTRA: aome_enc_control_id = 98;
pub type aom_codec_control_type_AV1E_SET_ALLOW_WARPED_MOTION = core::ffi::c_int;
pub const AV1E_SET_ALLOW_WARPED_MOTION: aome_enc_control_id = 97;
pub type aom_codec_control_type_AV1E_SET_ENABLE_WARPED_MOTION = core::ffi::c_int;
pub const AV1E_SET_ENABLE_WARPED_MOTION: aome_enc_control_id = 96;
pub type aom_codec_control_type_AV1E_SET_ENABLE_GLOBAL_MOTION = core::ffi::c_int;
pub const AV1E_SET_ENABLE_GLOBAL_MOTION: aome_enc_control_id = 95;
pub type aom_codec_control_type_AV1E_SET_ENABLE_INTERINTRA_WEDGE = core::ffi::c_int;
pub const AV1E_SET_ENABLE_INTERINTRA_WEDGE: aome_enc_control_id = 94;
pub type aom_codec_control_type_AV1E_SET_ENABLE_INTERINTER_WEDGE = core::ffi::c_int;
pub const AV1E_SET_ENABLE_INTERINTER_WEDGE: aome_enc_control_id = 93;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DIFF_WTD_COMP = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DIFF_WTD_COMP: aome_enc_control_id = 92;
pub type aom_codec_control_type_AV1E_SET_ENABLE_SMOOTH_INTERINTRA = core::ffi::c_int;
pub const AV1E_SET_ENABLE_SMOOTH_INTERINTRA: aome_enc_control_id = 91;
pub type aom_codec_control_type_AV1E_SET_ENABLE_INTERINTRA_COMP = core::ffi::c_int;
pub const AV1E_SET_ENABLE_INTERINTRA_COMP: aome_enc_control_id = 90;
pub type aom_codec_control_type_AV1E_SET_ENABLE_ONESIDED_COMP = core::ffi::c_int;
pub const AV1E_SET_ENABLE_ONESIDED_COMP: aome_enc_control_id = 89;
pub type aom_codec_control_type_AV1E_SET_ENABLE_MASKED_COMP = core::ffi::c_int;
pub const AV1E_SET_ENABLE_MASKED_COMP: aome_enc_control_id = 88;
pub type aom_codec_control_type_AV1E_SET_ALLOW_REF_FRAME_MVS = core::ffi::c_int;
pub const AV1E_SET_ALLOW_REF_FRAME_MVS: aome_enc_control_id = 85;
pub type aom_codec_control_type_AV1E_SET_ENABLE_REF_FRAME_MVS = core::ffi::c_int;
pub const AV1E_SET_ENABLE_REF_FRAME_MVS: aome_enc_control_id = 84;
pub type aom_codec_control_type_AV1E_SET_REDUCED_REFERENCE_SET = core::ffi::c_int;
pub const AV1E_SET_REDUCED_REFERENCE_SET: aome_enc_control_id = 125;
pub type aom_codec_control_type_AV1E_SET_MAX_REFERENCE_FRAMES = core::ffi::c_int;
pub const AV1E_SET_MAX_REFERENCE_FRAMES: aome_enc_control_id = 124;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DIST_WTD_COMP = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DIST_WTD_COMP: aome_enc_control_id = 83;
pub type aom_codec_control_type_AV1E_SET_ENABLE_RECT_TX = core::ffi::c_int;
pub const AV1E_SET_ENABLE_RECT_TX: aome_enc_control_id = 82;
pub type aom_codec_control_type_AV1E_SET_ENABLE_FLIP_IDTX = core::ffi::c_int;
pub const AV1E_SET_ENABLE_FLIP_IDTX: aome_enc_control_id = 81;
pub type aom_codec_control_type_AV1E_SET_ENABLE_TX64 = core::ffi::c_int;
pub const AV1E_SET_ENABLE_TX64: aome_enc_control_id = 80;
pub type aom_codec_control_type_AV1E_SET_ENABLE_ORDER_HINT = core::ffi::c_int;
pub const AV1E_SET_ENABLE_ORDER_HINT: aome_enc_control_id = 79;
pub type aom_codec_control_type_AV1E_SET_ENABLE_INTRA_EDGE_FILTER = core::ffi::c_int;
pub const AV1E_SET_ENABLE_INTRA_EDGE_FILTER: aome_enc_control_id = 78;
pub type aom_codec_control_type_AV1E_SET_ENABLE_CHROMA_DELTAQ = core::ffi::c_int;
pub const AV1E_SET_ENABLE_CHROMA_DELTAQ: aome_enc_control_id = 87;
pub type aom_codec_control_type_AV1E_SET_ENABLE_DUAL_FILTER = core::ffi::c_int;
pub const AV1E_SET_ENABLE_DUAL_FILTER: aome_enc_control_id = 86;
pub type aom_codec_control_type_AV1E_SET_MAX_PARTITION_SIZE = core::ffi::c_int;
pub const AV1E_SET_MAX_PARTITION_SIZE: aome_enc_control_id = 77;
pub type aom_codec_control_type_AV1E_SET_MIN_PARTITION_SIZE = core::ffi::c_int;
pub const AV1E_SET_MIN_PARTITION_SIZE: aome_enc_control_id = 76;
pub type aom_codec_control_type_AV1E_SET_ENABLE_1TO4_PARTITIONS = core::ffi::c_int;
pub const AV1E_SET_ENABLE_1TO4_PARTITIONS: aome_enc_control_id = 75;
pub type aom_codec_control_type_AV1E_SET_ENABLE_AB_PARTITIONS = core::ffi::c_int;
pub const AV1E_SET_ENABLE_AB_PARTITIONS: aome_enc_control_id = 74;
pub type aom_codec_control_type_AV1E_SET_ENABLE_RECT_PARTITIONS = core::ffi::c_int;
pub const AV1E_SET_ENABLE_RECT_PARTITIONS: aome_enc_control_id = 73;
pub type aom_codec_control_type_AV1E_SET_S_FRAME_MODE = core::ffi::c_int;
pub const AV1E_SET_S_FRAME_MODE: aome_enc_control_id = 39;
pub type aom_codec_control_type_AV1E_SET_ERROR_RESILIENT_MODE = core::ffi::c_int;
pub const AV1E_SET_ERROR_RESILIENT_MODE: aome_enc_control_id = 38;
pub type aom_codec_control_type_AV1E_SET_FRAME_PARALLEL_DECODING = core::ffi::c_uint;
pub const AV1E_SET_FRAME_PARALLEL_DECODING: aome_enc_control_id = 37;
pub type aom_codec_control_type_AV1E_SET_TIMING_INFO_TYPE = core::ffi::c_int;
pub const AV1E_SET_TIMING_INFO_TYPE: aome_enc_control_id = 111;
pub type aom_codec_control_type_AV1E_SET_MTU = core::ffi::c_uint;
pub const AV1E_SET_MTU: aome_enc_control_id = 71;
pub type aom_codec_control_type_AV1E_SET_NUM_TG = core::ffi::c_uint;
pub const AV1E_SET_NUM_TG: aome_enc_control_id = 70;
pub type aom_codec_control_type_AV1E_SET_QM_MAX = core::ffi::c_uint;
pub const AV1E_SET_QM_MAX: aome_enc_control_id = 65;
pub type aom_codec_control_type_AV1E_SET_QM_MIN = core::ffi::c_uint;
pub const AV1E_SET_QM_MIN: aome_enc_control_id = 64;
pub type aom_codec_control_type_AV1E_SET_QM_V = core::ffi::c_uint;
pub const AV1E_SET_QM_V: aome_enc_control_id = 68;
pub type aom_codec_control_type_AV1E_SET_QM_U = core::ffi::c_uint;
pub const AV1E_SET_QM_U: aome_enc_control_id = 67;
pub type aom_codec_control_type_AV1E_SET_QM_Y = core::ffi::c_uint;
pub const AV1E_SET_QM_Y: aome_enc_control_id = 66;
pub type aom_codec_control_type_AV1E_SET_ENABLE_QM = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_QM: aome_enc_control_id = 63;
pub type aom_codec_control_type_AV1E_SET_DISABLE_TRELLIS_QUANT = core::ffi::c_uint;
pub const AV1E_SET_DISABLE_TRELLIS_QUANT: aome_enc_control_id = 62;
pub type aom_codec_control_type_AV1E_SET_ENABLE_OBMC = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_OBMC: aome_enc_control_id = 61;
pub type aom_codec_control_type_AV1E_SET_FORCE_VIDEO_MODE = core::ffi::c_uint;
pub const AV1E_SET_FORCE_VIDEO_MODE: aome_enc_control_id = 60;
pub type aom_codec_control_type_AV1E_SET_ENABLE_RESTORATION = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_RESTORATION: aome_enc_control_id = 59;
pub type aom_codec_control_type_AV1E_SET_ENABLE_CDEF = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_CDEF: aome_enc_control_id = 58;
pub type aom_codec_control_type_AV1E_SET_LOSSLESS = core::ffi::c_uint;
pub const AV1E_SET_LOSSLESS: aome_enc_control_id = 31;
pub type aom_codec_control_type_AV1E_SET_GF_CBR_BOOST_PCT = core::ffi::c_uint;
pub const AV1E_SET_GF_CBR_BOOST_PCT: aome_enc_control_id = 29;
pub type aom_codec_control_type_AOME_SET_MAX_INTER_BITRATE_PCT = core::ffi::c_uint;
pub const AV1E_SET_MAX_INTER_BITRATE_PCT: aome_enc_control_id = 28;
pub const AOME_SET_NUMBER_SPATIAL_LAYERS: aome_enc_control_id = 27;
pub type aom_codec_control_type_AOME_SET_MAX_INTRA_BITRATE_PCT = core::ffi::c_uint;
pub const AOME_SET_MAX_INTRA_BITRATE_PCT: aome_enc_control_id = 26;
pub type aom_codec_control_type_AOME_SET_CQ_LEVEL = core::ffi::c_uint;
pub const AOME_SET_CQ_LEVEL: aome_enc_control_id = 25;
pub type aom_codec_control_type_AOME_SET_TUNING = core::ffi::c_int;
pub const AOME_SET_TUNING: aome_enc_control_id = 24;
pub type aom_codec_control_type_AOME_SET_ARNR_STRENGTH = core::ffi::c_uint;
pub const AOME_SET_ARNR_STRENGTH: aome_enc_control_id = 22;
pub type aom_codec_control_type_AOME_SET_ARNR_MAXFRAMES = core::ffi::c_uint;
pub const AOME_SET_ARNR_MAXFRAMES: aome_enc_control_id = 21;
pub type aom_codec_control_type_AV1E_SET_ENABLE_KEYFRAME_FILTERING = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_KEYFRAME_FILTERING: aome_enc_control_id = 36;
pub type aom_codec_control_type_AV1E_SET_ENABLE_TPL_MODEL = core::ffi::c_uint;
pub const AV1E_SET_ENABLE_TPL_MODEL: aome_enc_control_id = 35;
pub type aom_codec_control_type_AV1E_SET_TILE_ROWS = core::ffi::c_uint;
pub const AV1E_SET_TILE_ROWS: aome_enc_control_id = 34;
pub type aom_codec_control_type_AV1E_SET_TILE_COLUMNS = core::ffi::c_uint;
pub const AV1E_SET_TILE_COLUMNS: aome_enc_control_id = 33;
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
pub type aom_codec_control_type_AV1E_SET_FP_MT = core::ffi::c_uint;
pub const AV1E_SET_FP_MT: aome_enc_control_id = 153;
pub type aom_codec_control_type_AV1E_SET_ROW_MT = core::ffi::c_uint;
pub const AV1E_SET_ROW_MT: aome_enc_control_id = 32;
pub type aom_codec_control_type_AOME_SET_STATIC_THRESHOLD = core::ffi::c_uint;
pub const AOME_SET_STATIC_THRESHOLD: aome_enc_control_id = 17;
pub type aom_codec_control_type_AOME_SET_SHARPNESS = core::ffi::c_uint;
pub const AOME_SET_SHARPNESS: aome_enc_control_id = 16;
pub type aom_codec_control_type_AOME_SET_ENABLEAUTOBWDREF = core::ffi::c_uint;
pub const AOME_SET_ENABLEAUTOBWDREF: aome_enc_control_id = 57;
pub type aom_codec_control_type_AOME_SET_ENABLEAUTOALTREF = core::ffi::c_uint;
pub const AOME_SET_ENABLEAUTOALTREF: aome_enc_control_id = 14;
pub type aom_codec_control_type_AOME_SET_CPUUSED = core::ffi::c_int;
pub const AOME_SET_CPUUSED: aome_enc_control_id = 13;
pub const AOME_SET_SPATIAL_LAYER_ID: aome_enc_control_id = 12;
pub type AOM_SCALING_MODE = aom_scaling_mode_1d;
pub type aom_scaling_mode_1d = core::ffi::c_uint;
pub const AOME_ONETHREE: aom_scaling_mode_1d = 8;
pub const AOME_TWOTHREE: aom_scaling_mode_1d = 7;
pub const AOME_ONETWO: aom_scaling_mode_1d = 6;
pub const AOME_ONEEIGHT: aom_scaling_mode_1d = 5;
pub const AOME_ONEFOUR: aom_scaling_mode_1d = 4;
pub const AOME_THREEFOUR: aom_scaling_mode_1d = 3;
pub const AOME_THREEFIVE: aom_scaling_mode_1d = 2;
pub const AOME_FOURFIVE: aom_scaling_mode_1d = 1;
pub const AOME_NORMAL: aom_scaling_mode_1d = 0;
pub type aom_scaling_mode_t = aom_scaling_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_scaling_mode {
    pub h_scaling_mode: AOM_SCALING_MODE,
    pub v_scaling_mode: AOM_SCALING_MODE,
}
pub const AOME_SET_SCALEMODE: aome_enc_control_id = 11;
pub const AOME_SET_ACTIVEMAP: aome_enc_control_id = 9;
pub const AOME_SET_ROI_MAP: aome_enc_control_id = 8;
pub const AOME_USE_REFERENCE: aome_enc_control_id = 7;
pub type aome_enc_control_id = core::ffi::c_uint;
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const SEQ_LEVEL_6_3: C2RustUnnamed_18 = 19;
pub const SEQ_LEVEL_6_2: C2RustUnnamed_18 = 18;
pub const SEQ_LEVEL_6_1: C2RustUnnamed_18 = 17;
pub const SEQ_LEVEL_6_0: C2RustUnnamed_18 = 16;
pub const SEQ_LEVEL_5_3: C2RustUnnamed_18 = 15;
pub const SEQ_LEVEL_5_2: C2RustUnnamed_18 = 14;
pub const SEQ_LEVEL_5_1: C2RustUnnamed_18 = 13;
pub const SEQ_LEVEL_5_0: C2RustUnnamed_18 = 12;
pub const SEQ_LEVEL_4_1: C2RustUnnamed_18 = 9;
pub const SEQ_LEVEL_4_0: C2RustUnnamed_18 = 8;
pub const SEQ_LEVEL_3_1: C2RustUnnamed_18 = 5;
pub const SEQ_LEVEL_3_0: C2RustUnnamed_18 = 4;
pub const SEQ_LEVEL_2_1: C2RustUnnamed_18 = 1;
pub const SEQ_LEVEL_2_0: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_19 = 7;
pub const BWD_REFS: C2RustUnnamed_19 = 3;
pub const FWD_REFS: C2RustUnnamed_19 = 4;
pub const ALTREF_FRAME: C2RustUnnamed_19 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_19 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_19 = 5;
pub const LAST3_FRAME: C2RustUnnamed_19 = 3;
pub const LAST2_FRAME: C2RustUnnamed_19 = 2;
pub const INTRA_FRAME: C2RustUnnamed_19 = 0;
pub const NONE_FRAME: C2RustUnnamed_19 = -1;
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_20 = 4;
pub const S_FRAME: C2RustUnnamed_20 = 3;
pub const INTER_FRAME: C2RustUnnamed_20 = 1;
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_22 = 2;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const VAR_BASED_PARTITION: C2RustUnnamed_23 = 2;
pub const FIXED_PARTITION: C2RustUnnamed_23 = 1;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const FRAMEFLAGS_ALTREF: C2RustUnnamed_24 = 8;
pub const FRAMEFLAGS_BWDREF: C2RustUnnamed_24 = 4;
pub const FRAMEFLAGS_GOLDEN: C2RustUnnamed_24 = 2;
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_25 = 3;
pub const COMPLEXITY_AQ: C2RustUnnamed_25 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_25 = 1;
pub type C2RustUnnamed_26 = core::ffi::c_uint;
pub const DELTA_Q_USER_RATING_BASED: C2RustUnnamed_26 = 4;
pub const DELTA_Q_PERCEPTUAL_AI: C2RustUnnamed_26 = 3;
pub const DELTA_Q_PERCEPTUAL: C2RustUnnamed_26 = 2;
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const RESIZE_DYNAMIC: C2RustUnnamed_27 = 3;
pub const RESIZE_RANDOM: C2RustUnnamed_27 = 2;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const CONFIG_AV1_HIGHBITDEPTH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const UINTPTR_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const SIZE_MAX: core::ffi::c_ulong = UINTPTR_MAX;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const AOM_CODEC_CAP_ENCODER: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const AOM_FRAME_IS_KEY: core::ffi::c_uint = 0x1 as core::ffi::c_uint;
pub const AOM_FRAME_IS_DROPPABLE: core::ffi::c_uint = 0x2 as core::ffi::c_uint;
pub const AOM_FRAME_IS_INTRAONLY: core::ffi::c_uint = 0x10 as core::ffi::c_uint;
pub const AOM_FRAME_IS_SWITCH: core::ffi::c_uint = 0x20 as core::ffi::c_uint;
pub const AOM_FRAME_IS_ERROR_RESILIENT: core::ffi::c_uint = 0x40 as core::ffi::c_uint;
pub const AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT: core::ffi::c_uint = 0x80
    as core::ffi::c_uint;
pub const AOM_CODEC_CAP_PSNR: core::ffi::c_int = 0x10000 as core::ffi::c_int;
pub const AOM_CODEC_CAP_HIGHBITDEPTH: core::ffi::c_int = 0x40000 as core::ffi::c_int;
pub const AOM_CODEC_USE_PSNR: core::ffi::c_int = 0x10000 as core::ffi::c_int;
pub const AOM_CODEC_USE_HIGHBITDEPTH: core::ffi::c_int = 0x40000 as core::ffi::c_int;
pub const AOM_EFLAG_FORCE_KF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 0 as core::ffi::c_int;
pub const AOM_EFLAG_CALCULATE_PSNR: core::ffi::c_int = (1 as core::ffi::c_int)
    << 1 as core::ffi::c_int;
pub const AOM_EFLAG_FREEZE_INTERNAL_STATE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 2 as core::ffi::c_int;
pub const MAX_TILE_WIDTHS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_TILE_HEIGHTS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const AOM_USAGE_GOOD_QUALITY: core::ffi::c_uint = 0 as core::ffi::c_uint;
pub const AOM_USAGE_REALTIME: core::ffi::c_uint = 1 as core::ffi::c_uint;
pub const AOM_USAGE_ALL_INTRA: core::ffi::c_uint = 2 as core::ffi::c_uint;
pub const ARG_ERR_MSG_MAX_LEN: core::ffi::c_int = 200 as core::ffi::c_int;
pub const AOM_CODEC_INTERNAL_ABI_VERSION: core::ffi::c_int = 7 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const AOM_ENC_ALLINTRA_BORDER: core::ffi::c_int = 64 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_BUFFERS_ALLINTRA: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const DEFAULT_QM_Y: core::ffi::c_int = 10 as core::ffi::c_int;
pub const DEFAULT_QM_U: core::ffi::c_int = 11 as core::ffi::c_int;
pub const DEFAULT_QM_V: core::ffi::c_int = 12 as core::ffi::c_int;
pub const DEFAULT_QM_FIRST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const DEFAULT_QM_LAST: core::ffi::c_int = 9 as core::ffi::c_int;
pub const DEFAULT_QM_FIRST_ALLINTRA: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DEFAULT_QM_LAST_ALLINTRA: core::ffi::c_int = 10 as core::ffi::c_int;
pub const QM_FIRST_IQ_SSIMULACRA2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const QM_LAST_IQ_SSIMULACRA2: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn valid_ref_frame_size(
    mut ref_width: core::ffi::c_int,
    mut ref_height: core::ffi::c_int,
    mut this_width: core::ffi::c_int,
    mut this_height: core::ffi::c_int,
) -> core::ffi::c_int {
    return (2 as core::ffi::c_int * this_width >= ref_width
        && 2 as core::ffi::c_int * this_height >= ref_height
        && this_width <= 16 as core::ffi::c_int * ref_width
        && this_height <= 16 as core::ffi::c_int * ref_height) as core::ffi::c_int;
}
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const CDEF_MAX_STRENGTHS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_NUM_TEMPORAL_LAYERS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_NUM_SPATIAL_LAYERS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_NUM_OPERATING_POINTS: core::ffi::c_int = MAX_NUM_TEMPORAL_LAYERS
    * MAX_NUM_SPATIAL_LAYERS;
#[inline]
unsafe extern "C-unwind" fn get_ref_frame(
    mut cm: *mut AV1_COMMON,
    mut index: core::ffi::c_int,
) -> *mut YV12_BUFFER_CONFIG {
    if index < 0 as core::ffi::c_int || index >= REF_FRAMES as core::ffi::c_int {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    if ((*cm).ref_frame_map[index as usize]).is_null() {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    return &raw mut (**((*cm).ref_frame_map).as_mut_ptr().offset(index as isize)).buf;
}
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
unsafe extern "C-unwind" fn is_valid_seq_level_idx(
    mut seq_level_idx: AV1_LEVEL,
) -> core::ffi::c_int {
    return (seq_level_idx as core::ffi::c_int == SEQ_LEVEL_MAX as core::ffi::c_int
        || (seq_level_idx as core::ffi::c_int) < SEQ_LEVELS as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_2_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_2_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_3_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_3_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_4_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_4_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_0 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_1 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_0 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_1 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_3 as core::ffi::c_int)
        as core::ffi::c_int;
}
pub const MAX_LAG_BUFFERS: core::ffi::c_int = 48 as core::ffi::c_int;
pub const LAP_LAG_IN_FRAMES: core::ffi::c_int = 17 as core::ffi::c_int;
pub const MAX_STATIC_GF_GROUP_LENGTH: core::ffi::c_int = 250 as core::ffi::c_int;
pub const RANSAC_NUM_MOTIONS: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn tf_dealloc_data(
    mut tf_data: *mut TemporalFilterData,
    mut is_high_bitdepth: core::ffi::c_int,
) {
    if is_high_bitdepth != 0 {
        (*tf_data).pred = (((*tf_data).pred as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t as *mut uint8_t;
    }
    aom_free((*tf_data).tmp_mbmi as *mut core::ffi::c_void);
    (*tf_data).tmp_mbmi = 0 as *mut MB_MODE_INFO;
    aom_free((*tf_data).accum as *mut core::ffi::c_void);
    (*tf_data).accum = 0 as *mut uint32_t;
    aom_free((*tf_data).count as *mut core::ffi::c_void);
    (*tf_data).count = 0 as *mut uint16_t;
    aom_free((*tf_data).pred as *mut core::ffi::c_void);
    (*tf_data).pred = 0 as *mut uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn tpl_dealloc_temp_buffers(mut tpl_tmp_buffers: *mut TplBuffers) {
    aom_free((*tpl_tmp_buffers).predictor8 as *mut core::ffi::c_void);
    (*tpl_tmp_buffers).predictor8 = 0 as *mut uint8_t;
    aom_free((*tpl_tmp_buffers).src_diff as *mut core::ffi::c_void);
    (*tpl_tmp_buffers).src_diff = 0 as *mut int16_t;
    aom_free((*tpl_tmp_buffers).coeff as *mut core::ffi::c_void);
    (*tpl_tmp_buffers).coeff = 0 as *mut tran_low_t;
    aom_free((*tpl_tmp_buffers).qcoeff as *mut core::ffi::c_void);
    (*tpl_tmp_buffers).qcoeff = 0 as *mut tran_low_t;
    aom_free((*tpl_tmp_buffers).dqcoeff as *mut core::ffi::c_void);
    (*tpl_tmp_buffers).dqcoeff = 0 as *mut tran_low_t;
}
pub const MAX_PARALLEL_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const TICKS_PER_SEC: core::ffi::c_longlong = 10000000 as core::ffi::c_longlong;
#[inline]
unsafe extern "C-unwind" fn timebase_units_to_ticks(
    mut timestamp_ratio: *const aom_rational64_t,
    mut n: int64_t,
) -> int64_t {
    return n * (*timestamp_ratio).num / (*timestamp_ratio).den as int64_t;
}
#[inline]
unsafe extern "C-unwind" fn ticks_to_timebase_units(
    mut timestamp_ratio: *const aom_rational64_t,
    mut n: int64_t,
) -> int64_t {
    let mut round: int64_t = (*timestamp_ratio).num / 2 as int64_t;
    if round > 0 as int64_t {
        round -= 1;
    }
    return (n * (*timestamp_ratio).den as int64_t + round) / (*timestamp_ratio).num;
}
#[inline]
unsafe extern "C-unwind" fn is_stat_generation_stage(cpi: *const AV1_COMP) -> core::ffi::c_int {
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
unsafe extern "C-unwind" fn is_one_pass_rt_params(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    return (has_no_stats_stage(cpi) != 0
        && (*cpi).oxcf.gf_cfg.lag_in_frames == 0 as core::ffi::c_int
        && ((*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int
            || (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int))
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_stats_buf_size(
    mut num_lap_buffer: core::ffi::c_int,
    mut num_lag_buffer: core::ffi::c_int,
) -> core::ffi::c_int {
    if num_lap_buffer > 0 as core::ffi::c_int {
        return if num_lap_buffer + 1 as core::ffi::c_int
            > 16 as core::ffi::c_int + 1 as core::ffi::c_int
        {
            num_lap_buffer + 1 as core::ffi::c_int
        } else {
            16 as core::ffi::c_int + 1 as core::ffi::c_int
        };
    }
    return num_lag_buffer;
}
#[inline]
unsafe extern "C-unwind" fn yuvconfig2image(
    mut img: *mut aom_image_t,
    mut yv12: *const YV12_BUFFER_CONFIG,
    mut user_priv: *mut core::ffi::c_void,
) {
    let mut bps: core::ffi::c_int = 0;
    if (*yv12).subsampling_y == 0 {
        if (*yv12).subsampling_x == 0 {
            (*img).fmt = AOM_IMG_FMT_I444;
            bps = 24 as core::ffi::c_int;
        } else {
            (*img).fmt = AOM_IMG_FMT_I422;
            bps = 16 as core::ffi::c_int;
        }
    } else {
        (*img).fmt = AOM_IMG_FMT_I420;
        bps = 12 as core::ffi::c_int;
    }
    (*img).cp = (*yv12).color_primaries;
    (*img).tc = (*yv12).transfer_characteristics;
    (*img).mc = (*yv12).matrix_coefficients;
    (*img).monochrome = (*yv12).monochrome as core::ffi::c_int;
    (*img).csp = (*yv12).chroma_sample_position;
    (*img).range = (*yv12).color_range;
    (*img).bit_depth = 8 as core::ffi::c_uint;
    (*img).w = (*yv12).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint;
    (*img).h = (*yv12).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint;
    (*img).d_w = (*yv12).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        as core::ffi::c_uint;
    (*img).d_h = (*yv12).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        as core::ffi::c_uint;
    (*img).r_w = (*yv12).render_width as core::ffi::c_uint;
    (*img).r_h = (*yv12).render_height as core::ffi::c_uint;
    (*img).x_chroma_shift = (*yv12).subsampling_x as core::ffi::c_uint;
    (*img).y_chroma_shift = (*yv12).subsampling_y as core::ffi::c_uint;
    (*img).planes[AOM_PLANE_Y as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .y_buffer as *mut core::ffi::c_uchar;
    (*img).planes[AOM_PLANE_U as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .u_buffer as *mut core::ffi::c_uchar;
    (*img).planes[AOM_PLANE_V as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .v_buffer as *mut core::ffi::c_uchar;
    (*img).stride[AOM_PLANE_Y as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    (*img).stride[AOM_PLANE_U as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .uv_stride;
    (*img).stride[AOM_PLANE_V as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .uv_stride;
    if (*yv12).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        bps *= 2 as core::ffi::c_int;
        (*img).fmt = ((*img).fmt as core::ffi::c_uint
            | AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint) as aom_img_fmt_t;
        (*img).bit_depth = (*yv12).bit_depth;
        (*img).planes[AOM_PLANE_Y as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).planes[AOM_PLANE_U as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).planes[AOM_PLANE_V as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).stride[AOM_PLANE_Y as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride;
        (*img).stride[AOM_PLANE_U as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride;
        (*img).stride[AOM_PLANE_V as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride;
    }
    (*img).bps = bps;
    (*img).user_priv = user_priv;
    (*img).img_data = (*yv12).buffer_alloc as *mut core::ffi::c_uchar;
    (*img).img_data_owner = 0 as core::ffi::c_int;
    (*img).self_allocd = 0 as core::ffi::c_int;
    (*img).sz = (*yv12).frame_size;
    (*img).metadata = 0 as *mut aom_metadata_array_t;
}
#[inline]
unsafe extern "C-unwind" fn image2yuvconfig(
    mut img: *const aom_image_t,
    mut yv12: *mut YV12_BUFFER_CONFIG,
) -> aom_codec_err_t {
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.y_buffer = (*img)
        .planes[AOM_PLANE_Y as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.u_buffer = (*img)
        .planes[AOM_PLANE_U as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.v_buffer = (*img)
        .planes[AOM_PLANE_V as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_1.c2rust_unnamed.y_crop_width = (*img).d_w
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_2.c2rust_unnamed.y_crop_height = (*img).d_h
        as core::ffi::c_int;
    (*yv12).render_width = (*img).r_w as core::ffi::c_int;
    (*yv12).render_height = (*img).r_h as core::ffi::c_int;
    (*yv12).c2rust_unnamed.c2rust_unnamed.y_width = (*img).w as core::ffi::c_int;
    (*yv12).c2rust_unnamed_0.c2rust_unnamed.y_height = (*img).h as core::ffi::c_int;
    (*yv12).c2rust_unnamed.c2rust_unnamed.uv_width = (((*yv12)
        .c2rust_unnamed
        .c2rust_unnamed
        .y_width as core::ffi::c_uint)
        .wrapping_add((*img).x_chroma_shift) >> (*img).x_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_0.c2rust_unnamed.uv_height = (((*yv12)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .y_height as core::ffi::c_uint)
        .wrapping_add((*img).y_chroma_shift) >> (*img).y_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width = (((*yv12)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .y_crop_width as core::ffi::c_uint)
        .wrapping_add((*img).x_chroma_shift) >> (*img).x_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height = (((*yv12)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height as core::ffi::c_uint)
        .wrapping_add((*img).y_chroma_shift) >> (*img).y_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride = (*img)
        .stride[AOM_PLANE_Y as usize];
    (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride = (*img)
        .stride[AOM_PLANE_U as usize];
    (*yv12).color_primaries = (*img).cp;
    (*yv12).transfer_characteristics = (*img).tc;
    (*yv12).matrix_coefficients = (*img).mc;
    (*yv12).monochrome = (*img).monochrome as uint8_t;
    (*yv12).chroma_sample_position = (*img).csp;
    (*yv12).color_range = (*img).range;
    if (*img).fmt as core::ffi::c_uint & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint
        != 0
    {
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.y_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.u_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.v_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride >>= 1 as core::ffi::c_int;
        (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride >>= 1 as core::ffi::c_int;
        (*yv12).flags = YV12_FLAG_HIGHBITDEPTH;
    } else {
        (*yv12).flags = 0 as core::ffi::c_int;
    }
    let mut border: core::ffi::c_int = ((*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride
        - (((*img).w).wrapping_add(31 as core::ffi::c_uint) & !(31 as core::ffi::c_uint))
            as core::ffi::c_int) / 2 as core::ffi::c_int;
    (*yv12).border = if border < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        border
    };
    (*yv12).subsampling_x = (*img).x_chroma_shift as core::ffi::c_int;
    (*yv12).subsampling_y = (*img).y_chroma_shift as core::ffi::c_int;
    (*yv12).metadata = (*img).metadata;
    return AOM_CODEC_OK;
}
#[inline]
unsafe extern "C-unwind" fn av1_dealloc_mb_data(
    mut mb: *mut macroblock,
    mut num_planes: core::ffi::c_int,
) {
    aom_free((*mb).txfm_search_info.mb_rd_record as *mut core::ffi::c_void);
    (*mb).txfm_search_info.mb_rd_record = 0 as *mut MB_RD_RECORD;
    aom_free((*mb).inter_modes_info as *mut core::ffi::c_void);
    (*mb).inter_modes_info = 0 as *mut inter_modes_info;
    av1_dealloc_src_diff_buf(mb, num_planes);
    aom_free((*mb).e_mbd.seg_mask as *mut core::ffi::c_void);
    (*mb).e_mbd.seg_mask = 0 as *mut uint8_t;
    aom_free((*mb).winner_mode_stats as *mut core::ffi::c_void);
    (*mb).winner_mode_stats = 0 as *mut WinnerModeStats;
    aom_free((*mb).dqcoeff_buf as *mut core::ffi::c_void);
    (*mb).dqcoeff_buf = 0 as *mut tran_low_t;
}
#[inline]
unsafe extern "C-unwind" fn restore_cdef_coding_context(
    dst: *mut CdefInfo,
    src: *const CdefInfo,
) {
    (*dst).cdef_bits = (*src).cdef_bits;
    (*dst).cdef_damping = (*src).cdef_damping;
    memcpy(
        ((*dst).cdef_strengths).as_mut_ptr() as *mut core::ffi::c_void,
        ((*src).cdef_strengths).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_int; 16]>() as size_t,
    );
    memcpy(
        ((*dst).cdef_uv_strengths).as_mut_ptr() as *mut core::ffi::c_void,
        ((*src).cdef_uv_strengths).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_int; 16]>() as size_t,
    );
    (*dst).nb_cdef_strengths = (*src).nb_cdef_strengths;
}
#[inline]
unsafe extern "C-unwind" fn restore_extra_coding_context(mut cpi: *mut AV1_COMP) {
    let cc: *mut CODING_CONTEXT = &raw mut (*cpi).coding_context;
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    (*cm).lf = (*cc).lf;
    restore_cdef_coding_context(&raw mut (*cm).cdef_info, &raw mut (*cc).cdef_info);
    (*cpi).rc = (*cc).rc;
    (*(*cpi).ppi).mv_stats = (*cc).mv_stats;
    (*cpi).common.current_frame.frame_number = (*cc).frame_number;
    (*cpi).frame_index_set = (*cc).frame_index_set;
}
#[inline]
unsafe extern "C-unwind" fn release_scaled_references(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let refresh_golden: bool = if (*cpi).refresh_frame.golden_frame as core::ffi::c_int
        != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    } != 0;
    let mut release_golden: bool = true_0 != 0;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
        let buf: *mut RefCntBuffer = (*cpi).scaled_ref_buf[i as usize];
        let golden_ref: core::ffi::c_int = (i
            == GOLDEN_FRAME as core::ffi::c_int - 1 as core::ffi::c_int)
            as core::ffi::c_int;
        if golden_ref != 0 && is_one_pass_rt_params(cpi) != 0
            && (*(*cpi).ppi).use_svc == 0 && !buf.is_null()
        {
            let ref_0: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            );
            let same_resoln: bool = (*buf)
                .buf
                .c2rust_unnamed_1
                .c2rust_unnamed
                .y_crop_width
                == (*ref_0).buf.c2rust_unnamed_1.c2rust_unnamed.y_crop_width
                && (*buf).buf.c2rust_unnamed_2.c2rust_unnamed.y_crop_height
                    == (*ref_0).buf.c2rust_unnamed_2.c2rust_unnamed.y_crop_height;
            release_golden = refresh_golden as core::ffi::c_int != 0
                || same_resoln as core::ffi::c_int != 0;
        }
        if !buf.is_null()
            && (golden_ref == 0
                || golden_ref != 0 && release_golden as core::ffi::c_int != 0)
        {
            (*buf).ref_count -= 1;
            (*cpi).scaled_ref_buf[i as usize] = 0 as *mut RefCntBuffer;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn restore_all_coding_context(mut cpi: *mut AV1_COMP) {
    restore_extra_coding_context(cpi);
    if frame_is_intra_only(&raw mut (*cpi).common) == 0 {
        release_scaled_references(cpi);
    }
}
#[inline]
unsafe extern "C-unwind" fn av1_get_enc_border_size(
    mut resize: bool,
    mut all_intra: bool,
    mut sb_size: BLOCK_SIZE,
) -> core::ffi::c_int {
    if resize {
        return AOM_BORDER_IN_PIXELS;
    }
    if all_intra {
        return AOM_ENC_ALLINTRA_BORDER;
    }
    return block_size_wide[sb_size as usize] as core::ffi::c_int
        + 32 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_resize_needed(mut oxcf: *const AV1EncoderConfig) -> bool {
    let mut resize_cfg: *const ResizeCfg = &(*oxcf).resize_cfg;
    let mut superres_cfg: *const SuperResCfg = &(*oxcf).superres_cfg;
    return (*resize_cfg).resize_mode as core::ffi::c_int != 0
        || (*superres_cfg).superres_mode as core::ffi::c_uint != 0;
}
#[inline]
unsafe extern "C-unwind" fn gm_dealloc_data(mut gm_data: *mut GlobalMotionData) {
    aom_free((*gm_data).segment_map as *mut core::ffi::c_void);
    (*gm_data).segment_map = 0 as *mut uint8_t;
    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
    while m < RANSAC_NUM_MOTIONS {
        aom_free((*gm_data).motion_models[m as usize].inliers as *mut core::ffi::c_void);
        (*gm_data).motion_models[m as usize].inliers = 0 as *mut core::ffi::c_int;
        m += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn release_obmc_buffers(mut obmc_buffer: *mut OBMCBuffer) {
    aom_free((*obmc_buffer).mask as *mut core::ffi::c_void);
    aom_free((*obmc_buffer).above_pred as *mut core::ffi::c_void);
    aom_free((*obmc_buffer).left_pred as *mut core::ffi::c_void);
    aom_free((*obmc_buffer).wsrc as *mut core::ffi::c_void);
    (*obmc_buffer).mask = 0 as *mut int32_t;
    (*obmc_buffer).above_pred = 0 as *mut uint8_t;
    (*obmc_buffer).left_pred = 0 as *mut uint8_t;
    (*obmc_buffer).wsrc = 0 as *mut int32_t;
}
#[inline]
unsafe extern "C-unwind" fn release_compound_type_rd_buffers(bufs: *mut CompoundTypeRdBuffers) {
    aom_free((*bufs).pred0 as *mut core::ffi::c_void);
    aom_free((*bufs).pred1 as *mut core::ffi::c_void);
    aom_free((*bufs).residual1 as *mut core::ffi::c_void);
    aom_free((*bufs).diff10 as *mut core::ffi::c_void);
    aom_free((*bufs).tmp_best_mask_buf as *mut core::ffi::c_void);
    memset(
        bufs as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<CompoundTypeRdBuffers>() as size_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn free_thread_data(mut ppi: *mut AV1_PRIMARY) {
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    let num_tf_workers: core::ffi::c_int = if (*p_mt_info)
        .num_mod_workers[MOD_TF as core::ffi::c_int as usize] < (*p_mt_info).num_workers
    {
        (*p_mt_info).num_mod_workers[MOD_TF as core::ffi::c_int as usize]
    } else {
        (*p_mt_info).num_workers
    };
    let num_tpl_workers: core::ffi::c_int = if (*p_mt_info)
        .num_mod_workers[MOD_TPL as core::ffi::c_int as usize] < (*p_mt_info).num_workers
    {
        (*p_mt_info).num_mod_workers[MOD_TPL as core::ffi::c_int as usize]
    } else {
        (*p_mt_info).num_workers
    };
    let is_highbitdepth: core::ffi::c_int = (*ppi).seq_params.use_highbitdepth
        as core::ffi::c_int;
    let num_planes: core::ffi::c_int = if (*ppi).seq_params.monochrome
        as core::ffi::c_int != 0
    {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
    let mut t: core::ffi::c_int = 1 as core::ffi::c_int;
    while t < (*p_mt_info).num_workers {
        let thread_data: *mut EncWorkerData = &raw mut *((*p_mt_info).tile_thr_data)
            .offset(t as isize) as *mut EncWorkerData;
        (*thread_data).td = (*thread_data).original_td;
        let td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if !td.is_null() {
            aom_free((*td).tctx as *mut core::ffi::c_void);
            aom_free((*td).palette_buffer as *mut core::ffi::c_void);
            aom_free((*td).tmp_conv_dst as *mut core::ffi::c_void);
            aom_free((*td).upsample_pred as *mut core::ffi::c_void);
            release_compound_type_rd_buffers(&raw mut (*td).comp_rd_buffer);
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < 2 as core::ffi::c_int {
                aom_free((*td).tmp_pred_bufs[j as usize] as *mut core::ffi::c_void);
                j += 1;
            }
            aom_free((*td).pixel_gradient_info as *mut core::ffi::c_void);
            aom_free((*td).src_var_info_of_4x4_sub_blocks as *mut core::ffi::c_void);
            release_obmc_buffers(&raw mut (*td).obmc_buffer);
            aom_free((*td).vt64x64 as *mut core::ffi::c_void);
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < 2 as core::ffi::c_int {
                aom_free((*td).hash_value_buffer[x as usize] as *mut core::ffi::c_void);
                (*td).hash_value_buffer[x as usize] = 0 as *mut uint32_t;
                x += 1;
            }
            aom_free((*td).mv_costs_alloc as *mut core::ffi::c_void);
            (*td).mv_costs_alloc = 0 as *mut MvCosts;
            aom_free((*td).dv_costs_alloc as *mut core::ffi::c_void);
            (*td).dv_costs_alloc = 0 as *mut IntraBCMVCosts;
            aom_free((*td).counts as *mut core::ffi::c_void);
            av1_free_pmc((*td).firstpass_ctx, num_planes);
            (*td).firstpass_ctx = 0 as *mut PICK_MODE_CONTEXT;
            av1_free_shared_coeff_buffer(&raw mut (*td).shared_coeff_buf);
            av1_free_sms_tree(td as *mut ThreadData);
            if t < num_tf_workers {
                tf_dealloc_data(&raw mut (*td).tf_data, is_highbitdepth);
            }
            if t < num_tpl_workers {
                tpl_dealloc_temp_buffers(&raw mut (*td).tpl_tmp_buffers);
            }
            gm_dealloc_data(&raw mut (*td).gm_data);
            av1_dealloc_mb_data(&raw mut (*td).mb, num_planes);
            aom_free((*td).mb.sb_stats_cache as *mut core::ffi::c_void);
            (*td).mb.sb_stats_cache = 0 as *mut SB_FIRST_PASS_STATS;
            aom_free((*td).mb.sb_fp_stats as *mut core::ffi::c_void);
            (*td).mb.sb_fp_stats = 0 as *mut SB_FIRST_PASS_STATS;
            av1_free_pc_tree_recursive(
                (*td).pc_root,
                num_planes,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                SEARCH_PARTITION as core::ffi::c_int as PARTITION_SEARCH_TYPE,
            );
            (*td).pc_root = 0 as *mut PC_TREE;
            av1_dealloc_mb_wiener_var_pred_buf(td);
            aom_free(td as *mut core::ffi::c_void);
            (*thread_data).td = 0 as *mut ThreadData;
            (*thread_data).original_td = 0 as *mut ThreadData;
        }
        t += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn check_reset_rc_flag(mut cpi: *mut AV1_COMP) {
    let mut rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    if (*cpi).common.current_frame.frame_number
        > (*cpi).svc.number_spatial_layers as core::ffi::c_uint
    {
        if (*(*cpi).ppi).use_svc != 0 {
            av1_svc_check_reset_layer_rc_flag(cpi as *mut AV1_COMP);
        } else if (*rc).avg_frame_bandwidth / 3 as core::ffi::c_int
            > (*rc).prev_avg_frame_bandwidth >> 1 as core::ffi::c_int
            || (*rc).avg_frame_bandwidth
                < (*rc).prev_avg_frame_bandwidth >> 1 as core::ffi::c_int
        {
            (*rc).rc_1_frame = 0 as core::ffi::c_int;
            (*rc).rc_2_frame = 0 as core::ffi::c_int;
            (*p_rc).bits_off_target = (*p_rc).optimal_buffer_level;
            (*p_rc).buffer_level = (*p_rc).optimal_buffer_level;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn set_primary_rc_buffer_sizes(
    mut oxcf: *const AV1EncoderConfig,
    mut ppi: *mut AV1_PRIMARY,
) {
    let mut p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*ppi).p_rc;
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    let bandwidth: int64_t = (*rc_cfg).target_bandwidth;
    let starting: int64_t = (*rc_cfg).starting_buffer_level_ms;
    let optimal: int64_t = (*rc_cfg).optimal_buffer_level_ms;
    let maximum: int64_t = (*rc_cfg).maximum_buffer_size_ms;
    (*p_rc).starting_buffer_level = starting * bandwidth / 1000 as int64_t;
    (*p_rc).optimal_buffer_level = if optimal == 0 as int64_t {
        bandwidth / 8 as int64_t
    } else {
        optimal * bandwidth / 1000 as int64_t
    };
    (*p_rc).maximum_buffer_size = if maximum == 0 as int64_t {
        bandwidth / 8 as int64_t
    } else {
        maximum * bandwidth / 1000 as int64_t
    };
    (*p_rc).bits_off_target = if (*p_rc).bits_off_target < (*p_rc).maximum_buffer_size {
        (*p_rc).bits_off_target
    } else {
        (*p_rc).maximum_buffer_size
    };
    (*p_rc).buffer_level = if (*p_rc).buffer_level < (*p_rc).maximum_buffer_size {
        (*p_rc).buffer_level
    } else {
        (*p_rc).maximum_buffer_size
    };
}
static mut default_extra_cfg: av1_extracfg = {
    let mut init = av1_extracfg {
        cpu_used: 0 as core::ffi::c_int,
        enable_auto_alt_ref: 1 as core::ffi::c_uint,
        enable_auto_bwd_ref: 0 as core::ffi::c_uint,
        noise_sensitivity: 0 as core::ffi::c_uint,
        sharpness: 0 as core::ffi::c_uint,
        enable_adaptive_sharpness: 0 as core::ffi::c_uint,
        static_thresh: 0 as core::ffi::c_uint,
        row_mt: 1 as core::ffi::c_uint,
        fp_mt: 0 as core::ffi::c_uint,
        tile_columns: 0 as core::ffi::c_uint,
        tile_rows: 0 as core::ffi::c_uint,
        auto_tiles: 0 as core::ffi::c_uint,
        enable_tpl_model: 1 as core::ffi::c_uint,
        enable_keyframe_filtering: 1 as core::ffi::c_uint,
        arnr_max_frames: 7 as core::ffi::c_uint,
        arnr_strength: 5 as core::ffi::c_uint,
        min_gf_interval: 0 as core::ffi::c_uint,
        max_gf_interval: 0 as core::ffi::c_uint,
        gf_min_pyr_height: 0 as core::ffi::c_uint,
        gf_max_pyr_height: 5 as core::ffi::c_uint,
        tuning: AOM_TUNE_PSNR,
        vmaf_model_path: b"/usr/local/share/model/vmaf_v0.6.1.json\0" as *const u8
            as *const core::ffi::c_char,
        partition_info_path: b".\0" as *const u8 as *const core::ffi::c_char,
        enable_rate_guide_deltaq: 0 as core::ffi::c_uint,
        rate_distribution_info: b"./rate_map.txt\0" as *const u8
            as *const core::ffi::c_char,
        dist_metric: AOM_DIST_METRIC_PSNR,
        cq_level: 10 as core::ffi::c_uint,
        rc_max_intra_bitrate_pct: 0 as core::ffi::c_uint,
        rc_max_inter_bitrate_pct: 0 as core::ffi::c_uint,
        gf_cbr_boost_pct: 0 as core::ffi::c_uint,
        lossless: 0 as core::ffi::c_uint,
        enable_cdef: 1 as core::ffi::c_uint,
        enable_restoration: 1 as core::ffi::c_uint,
        force_video_mode: 0 as core::ffi::c_uint,
        enable_obmc: 1 as core::ffi::c_uint,
        disable_trellis_quant: 3 as core::ffi::c_uint,
        enable_qm: 0 as core::ffi::c_uint,
        qm_y: DEFAULT_QM_Y as core::ffi::c_uint,
        qm_u: DEFAULT_QM_U as core::ffi::c_uint,
        qm_v: DEFAULT_QM_V as core::ffi::c_uint,
        qm_min: DEFAULT_QM_FIRST as core::ffi::c_uint,
        qm_max: DEFAULT_QM_LAST as core::ffi::c_uint,
        num_tg: 1 as core::ffi::c_uint,
        mtu_size: 0 as core::ffi::c_uint,
        timing_info_type: AOM_TIMING_UNSPECIFIED,
        frame_parallel_decoding_mode: 0 as core::ffi::c_uint,
        enable_dual_filter: 1 as core::ffi::c_int,
        enable_chroma_deltaq: 0 as core::ffi::c_uint,
        aq_mode: NO_AQ as core::ffi::c_int as AQ_MODE,
        deltaq_mode: DELTA_Q_OBJECTIVE as core::ffi::c_int as DELTAQ_MODE,
        deltaq_strength: 100 as core::ffi::c_uint,
        deltalf_mode: 0 as core::ffi::c_int,
        frame_periodic_boost: 0 as core::ffi::c_uint,
        content: AOM_CONTENT_DEFAULT,
        color_primaries: AOM_CICP_CP_UNSPECIFIED,
        transfer_characteristics: AOM_CICP_TC_UNSPECIFIED,
        matrix_coefficients: AOM_CICP_MC_UNSPECIFIED,
        chroma_sample_position: AOM_CSP_UNKNOWN,
        color_range: 0 as core::ffi::c_int,
        render_width: 0 as core::ffi::c_int,
        render_height: 0 as core::ffi::c_int,
        superblock_size: AOM_SUPERBLOCK_SIZE_DYNAMIC,
        single_tile_decoding: 1 as core::ffi::c_uint,
        error_resilient_mode: 0 as core::ffi::c_int,
        s_frame_mode: 0 as core::ffi::c_int,
        film_grain_test_vector: 0 as core::ffi::c_int,
        film_grain_table_filename: 0 as *const core::ffi::c_char,
        motion_vector_unit_test: 0 as core::ffi::c_uint,
        cdf_update_mode: 1 as core::ffi::c_uint,
        enable_rect_partitions: 1 as core::ffi::c_int,
        enable_ab_partitions: 1 as core::ffi::c_int,
        enable_1to4_partitions: 1 as core::ffi::c_int,
        min_partition_size: 4 as core::ffi::c_int,
        max_partition_size: 128 as core::ffi::c_int,
        enable_intra_edge_filter: 1 as core::ffi::c_int,
        enable_order_hint: 1 as core::ffi::c_int,
        enable_tx64: 1 as core::ffi::c_int,
        enable_flip_idtx: 1 as core::ffi::c_int,
        enable_rect_tx: 1 as core::ffi::c_int,
        enable_dist_wtd_comp: 1 as core::ffi::c_int,
        max_reference_frames: 7 as core::ffi::c_int,
        enable_reduced_reference_set: 0 as core::ffi::c_int,
        enable_ref_frame_mvs: 1 as core::ffi::c_int,
        allow_ref_frame_mvs: 1 as core::ffi::c_int,
        enable_masked_comp: 1 as core::ffi::c_int,
        enable_onesided_comp: 1 as core::ffi::c_int,
        enable_interintra_comp: 1 as core::ffi::c_int,
        enable_smooth_interintra: 1 as core::ffi::c_int,
        enable_diff_wtd_comp: 1 as core::ffi::c_int,
        enable_interinter_wedge: 1 as core::ffi::c_int,
        enable_interintra_wedge: 1 as core::ffi::c_int,
        enable_global_motion: 1 as core::ffi::c_int,
        enable_warped_motion: 1 as core::ffi::c_int,
        allow_warped_motion: 1 as core::ffi::c_int,
        enable_filter_intra: 1 as core::ffi::c_int,
        enable_smooth_intra: 1 as core::ffi::c_int,
        enable_paeth_intra: 1 as core::ffi::c_int,
        enable_cfl_intra: 1 as core::ffi::c_int,
        enable_directional_intra: 1 as core::ffi::c_int,
        enable_diagonal_intra: 1 as core::ffi::c_int,
        enable_superres: 1 as core::ffi::c_int,
        enable_overlay: 1 as core::ffi::c_int,
        enable_palette: 1 as core::ffi::c_int,
        enable_intrabc: 1 as core::ffi::c_int,
        enable_angle_delta: 1 as core::ffi::c_int,
        noise_level: 0 as core::ffi::c_int as core::ffi::c_float,
        noise_block_size: 32 as core::ffi::c_int,
        enable_dnl_denoising: 1 as core::ffi::c_int,
        enable_low_complexity_decode: 0 as core::ffi::c_uint,
        chroma_subsampling_x: 0 as core::ffi::c_uint,
        chroma_subsampling_y: 0 as core::ffi::c_uint,
        reduced_tx_type_set: 0 as core::ffi::c_int,
        use_intra_dct_only: 0 as core::ffi::c_int,
        use_inter_dct_only: 0 as core::ffi::c_int,
        use_intra_default_tx_only: 0 as core::ffi::c_int,
        enable_tx_size_search: 1 as core::ffi::c_int,
        quant_b_adapt: 0 as core::ffi::c_int,
        vbr_corpus_complexity_lap: 0 as core::ffi::c_uint,
        target_seq_level_idx: [
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
            SEQ_LEVEL_MAX as core::ffi::c_int as AV1_LEVEL,
        ],
        tier_mask: 0 as core::ffi::c_uint,
        min_cr: 0 as core::ffi::c_uint,
        coeff_cost_upd_freq: COST_UPD_SB,
        mode_cost_upd_freq: COST_UPD_SB,
        mv_cost_upd_freq: COST_UPD_SB,
        dv_cost_upd_freq: COST_UPD_SB,
        ext_tile_debug: 0 as core::ffi::c_uint,
        sb_multipass_unit_test: 0 as core::ffi::c_uint,
        passes: -(1 as core::ffi::c_int),
        fwd_kf_dist: -(1 as core::ffi::c_int),
        loopfilter_control: LOOPFILTER_ALL,
        skip_postproc_filtering: 0 as core::ffi::c_uint,
        mode_ref_delta_enabled: 1 as core::ffi::c_int,
        two_pass_output: 0 as *const core::ffi::c_char,
        second_pass_log: 0 as *const core::ffi::c_char,
        auto_intra_tools_off: 0 as core::ffi::c_int,
        strict_level_conformance: 0 as core::ffi::c_int,
        kf_max_pyr_height: -(1 as core::ffi::c_int),
        sb_qp_sweep: 0 as core::ffi::c_int,
        screen_detection_mode: AOM_SCREEN_DETECTION_STANDARD,
        validate_hbd_input: 1 as core::ffi::c_uint,
        force_max_q: 0 as core::ffi::c_uint,
    };
    init
};
#[inline]
unsafe extern "C-unwind" fn gcd(mut a: int64_t, mut b: core::ffi::c_int) -> core::ffi::c_int {
    let mut remainder: core::ffi::c_int = 0;
    while b > 0 as core::ffi::c_int {
        remainder = (a % b as int64_t) as core::ffi::c_int;
        a = b as int64_t;
        b = remainder;
    }
    return a as core::ffi::c_int;
}
unsafe extern "C-unwind" fn reduce_ratio(mut ratio: *mut aom_rational64_t) {
    let denom: core::ffi::c_int = gcd((*ratio).num, (*ratio).den) as core::ffi::c_int;
    (*ratio).num /= denom as int64_t;
    (*ratio).den /= denom;
}
unsafe extern "C-unwind" fn update_error_state(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut error: *const aom_internal_error_info,
) -> aom_codec_err_t {
    let res: aom_codec_err_t = (*error).error_code;
    if res as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).base.err_detail = if (*error).has_detail != 0 {
            ((*error).detail).as_ptr()
        } else {
            0 as *const core::ffi::c_char
        };
    }
    return res;
}
unsafe extern "C-unwind" fn allocate_and_set_string(
    mut src: *const core::ffi::c_char,
    mut default_src: *const core::ffi::c_char,
    mut dst: *mut *const core::ffi::c_char,
    mut err_detail: *mut core::ffi::c_char,
) -> aom_codec_err_t {
    if src.is_null() {
        snprintf(
            err_detail,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Null pointer given to a string parameter.\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AOM_CODEC_INVALID_PARAM;
    }
    if !(*dst).is_null() && strcmp(src, *dst) == 0 as core::ffi::c_int {
        return AOM_CODEC_OK;
    }
    if *dst != default_src {
        aom_free(*dst as *mut core::ffi::c_void);
    }
    if !default_src.is_null() && strcmp(src, default_src) == 0 as core::ffi::c_int {
        *dst = default_src;
    } else {
        let mut len: size_t = (strlen(src)).wrapping_add(1 as size_t);
        let mut tmp: *mut core::ffi::c_char = aom_malloc(
            len.wrapping_mul(::core::mem::size_of::<core::ffi::c_char>() as size_t),
        ) as *mut core::ffi::c_char;
        if tmp.is_null() {
            snprintf(
                err_detail,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Failed to allocate memory for copying parameters.\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AOM_CODEC_MEM_ERROR;
        }
        memcpy(tmp as *mut core::ffi::c_void, src as *const core::ffi::c_void, len);
        *dst = tmp;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn validate_config(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut cfg: *const aom_codec_enc_cfg_t,
    mut extra_cfg: *const av1_extracfg,
) -> aom_codec_err_t {
    if !((*cfg).g_w >= 1 as core::ffi::c_uint
        && (*cfg).g_w <= 65536 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_w out of range [1..65536]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_h >= 1 as core::ffi::c_uint
        && (*cfg).g_h <= 65536 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_h out of range [1..65536]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_forced_max_frame_width <= 65536 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"g_forced_max_frame_width out of range [..65536]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_forced_max_frame_height <= 65536 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"g_forced_max_frame_height out of range [..65536]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_forced_max_frame_width != 0 {
        if !((*cfg).g_w <= (*cfg).g_forced_max_frame_width) {
            (*ctx).base.err_detail = b"g_w out of range [..cfg->g_forced_max_frame_width]\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if (*cfg).g_forced_max_frame_height != 0 {
        if !((*cfg).g_h <= (*cfg).g_forced_max_frame_height) {
            (*ctx).base.err_detail = b"g_h out of range [..cfg->g_forced_max_frame_height]\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    let max_frame_width: core::ffi::c_uint = if (*cfg).g_forced_max_frame_width != 0 {
        (*cfg).g_forced_max_frame_width
    } else {
        (*cfg).g_w
    };
    let max_frame_height: core::ffi::c_uint = if (*cfg).g_forced_max_frame_height != 0 {
        (*cfg).g_forced_max_frame_height
    } else {
        (*cfg).g_h
    };
    let max_frame_area: int64_t = max_frame_width as int64_t
        * max_frame_height as int64_t;
    if max_frame_area > ((1 as core::ffi::c_int) << 30 as core::ffi::c_int) as int64_t {
        (*ctx).base.err_detail = b"max_frame_area out of range [..2^30]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_timebase.den >= 1 as core::ffi::c_int
        && (*cfg).g_timebase.den <= 1000000000 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"g_timebase.den out of range [1..1000000000]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_timebase.num >= 1 as core::ffi::c_int
        && (*cfg).g_timebase.num <= 1000000000 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"g_timebase.num out of range [1..1000000000]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_profile
        <= (MAX_PROFILES as core::ffi::c_int - 1 as core::ffi::c_int)
            as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_profile out of range [..MAX_PROFILES - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_target_bitrate <= 2000000 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_target_bitrate out of range [..2000000]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_max_quantizer <= 63 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_max_quantizer out of range [..63]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_min_quantizer <= (*cfg).rc_max_quantizer) {
        (*ctx).base.err_detail = b"rc_min_quantizer out of range [..cfg->rc_max_quantizer]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).lossless != 0) as core::ffi::c_int as core::ffi::c_uint
        != (*extra_cfg).lossless
    {
        (*ctx).base.err_detail = b"lossless expected boolean\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).aq_mode as core::ffi::c_int
        <= AQ_MODE_COUNT as core::ffi::c_int - 1 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"aq_mode out of range [..AQ_MODE_COUNT - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).deltaq_mode as core::ffi::c_int
        <= DELTA_Q_MODE_COUNT as core::ffi::c_int - 1 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"deltaq_mode out of range [..DELTA_Q_MODE_COUNT - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).deltalf_mode <= 1 as core::ffi::c_int) {
        (*ctx).base.err_detail = b"deltalf_mode out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).frame_periodic_boost <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"frame_periodic_boost out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_usage <= 2 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"g_usage out of range [..AOM_USAGE_ALL_INTRA]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_threads <= 64 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"g_threads out of range [..MAX_NUM_THREADS]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_end_usage as core::ffi::c_uint
        >= AOM_VBR as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).rc_end_usage as core::ffi::c_uint
            <= AOM_Q as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_end_usage out of range [AOM_VBR..AOM_Q]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_undershoot_pct <= 100 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_undershoot_pct out of range [..100]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_overshoot_pct <= 100 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_overshoot_pct out of range [..100]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_2pass_vbr_bias_pct <= 100 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_2pass_vbr_bias_pct out of range [..100]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).kf_mode as core::ffi::c_uint
        >= AOM_KF_DISABLED as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).kf_mode as core::ffi::c_uint
            <= AOM_KF_AUTO as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"kf_mode out of range [AOM_KF_DISABLED..AOM_KF_AUTO]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_dropframe_thresh <= 100 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"rc_dropframe_thresh out of range [..100]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_pass as core::ffi::c_uint
        >= AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).g_pass as core::ffi::c_uint
            <= AOM_RC_THIRD_PASS as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_pass out of range [AOM_RC_ONE_PASS..AOM_RC_THIRD_PASS]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_lag_in_frames <= 48 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"g_lag_in_frames out of range [..MAX_LAG_BUFFERS]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_usage == AOM_USAGE_ALL_INTRA {
        if !((*cfg).g_lag_in_frames <= 0 as core::ffi::c_uint) {
            (*ctx).base.err_detail = b"g_lag_in_frames out of range [..0]\0" as *const u8
                as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        if !((*cfg).kf_max_dist <= 0 as core::ffi::c_uint) {
            (*ctx).base.err_detail = b"kf_max_dist out of range [..0]\0" as *const u8
                as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if !((*extra_cfg).min_gf_interval
        <= (48 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"min_gf_interval out of range [..MAX_LAG_BUFFERS - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).max_gf_interval
        <= (48 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"max_gf_interval out of range [..MAX_LAG_BUFFERS - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).max_gf_interval > 0 as core::ffi::c_uint {
        if !((*extra_cfg).max_gf_interval
            >= (if 2 as core::ffi::c_uint > (*extra_cfg).min_gf_interval {
                2 as core::ffi::c_uint
            } else {
                (*extra_cfg).min_gf_interval
            })
            && (*extra_cfg).max_gf_interval
                <= (48 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_uint)
        {
            (*ctx).base.err_detail = b"max_gf_interval out of range [AOMMAX(2, extra_cfg->min_gf_interval)..(MAX_LAG_BUFFERS - 1)]\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if !((*extra_cfg).gf_min_pyr_height <= 5 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"gf_min_pyr_height out of range [..5]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).gf_max_pyr_height <= 5 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"gf_max_pyr_height out of range [..5]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).gf_min_pyr_height > (*extra_cfg).gf_max_pyr_height {
        (*ctx).base.err_detail = b"gf_min_pyr_height must be less than or equal to gf_max_pyramid_height\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_resize_mode
        <= (RESIZE_MODES as core::ffi::c_int - 1 as core::ffi::c_int)
            as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_resize_mode out of range [..RESIZE_MODES - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_resize_denominator >= 8 as core::ffi::c_uint
        && (*cfg).rc_resize_denominator
            <= ((8 as core::ffi::c_int) << 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_resize_denominator out of range [SCALE_NUMERATOR..SCALE_NUMERATOR << 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_resize_kf_denominator >= 8 as core::ffi::c_uint
        && (*cfg).rc_resize_kf_denominator
            <= ((8 as core::ffi::c_int) << 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_resize_kf_denominator out of range [SCALE_NUMERATOR..SCALE_NUMERATOR << 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_superres_mode as core::ffi::c_uint
        <= AOM_SUPERRES_AUTO as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_superres_mode out of range [..AOM_SUPERRES_AUTO]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_superres_denominator >= 8 as core::ffi::c_uint
        && (*cfg).rc_superres_denominator
            <= ((8 as core::ffi::c_int) << 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_superres_denominator out of range [SCALE_NUMERATOR..SCALE_NUMERATOR << 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_superres_kf_denominator >= 8 as core::ffi::c_uint
        && (*cfg).rc_superres_kf_denominator
            <= ((8 as core::ffi::c_int) << 1 as core::ffi::c_int) as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_superres_kf_denominator out of range [SCALE_NUMERATOR..SCALE_NUMERATOR << 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_superres_qthresh >= 1 as core::ffi::c_uint
        && (*cfg).rc_superres_qthresh <= 63 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_superres_qthresh out of range [1..63]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).rc_superres_kf_qthresh >= 1 as core::ffi::c_uint
        && (*cfg).rc_superres_kf_qthresh <= 63 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"rc_superres_kf_qthresh out of range [1..63]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).cdf_update_mode <= 2 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"cdf_update_mode out of range [..2]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).motion_vector_unit_test <= 2 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"motion_vector_unit_test out of range [..2]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).sb_multipass_unit_test <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"sb_multipass_unit_test out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).ext_tile_debug <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"ext_tile_debug out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_auto_alt_ref <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_auto_alt_ref out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_auto_bwd_ref <= 2 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_auto_bwd_ref out of range [..2]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).cpu_used >= 0 as core::ffi::c_int
        && (*extra_cfg).cpu_used
            <= (if (*cfg).g_usage == 1 as core::ffi::c_uint {
                12 as core::ffi::c_int
            } else {
                9 as core::ffi::c_int
            }))
    {
        (*ctx).base.err_detail = b"cpu_used out of range [0..(cfg->g_usage == AOM_USAGE_REALTIME) ? 12 : 9]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).noise_sensitivity <= 6 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"noise_sensitivity out of range [..6]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).superblock_size as core::ffi::c_uint
        >= AOM_SUPERBLOCK_SIZE_64X64 as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).superblock_size as core::ffi::c_uint
            <= AOM_SUPERBLOCK_SIZE_DYNAMIC as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"superblock_size out of range [AOM_SUPERBLOCK_SIZE_64X64..AOM_SUPERBLOCK_SIZE_DYNAMIC]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).large_scale_tile <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"large_scale_tile out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).single_tile_decoding <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"single_tile_decoding out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_rate_guide_deltaq <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_rate_guide_deltaq out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).row_mt <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"row_mt out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).fp_mt <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"fp_mt out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).tile_columns <= 6 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"tile_columns out of range [..6]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).tile_rows <= 6 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"tile_rows out of range [..6]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).auto_tiles <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"auto_tiles out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).monochrome <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"monochrome out of range [..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).large_scale_tile != 0 && (*extra_cfg).aq_mode as core::ffi::c_int != 0 {
        (*ctx).base.err_detail = b"Adaptive quantization are not supported in large scale tile coding.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).sharpness <= 7 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"sharpness out of range [..7]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_adaptive_sharpness <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_adaptive_sharpness out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).arnr_max_frames <= 15 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"arnr_max_frames out of range [..15]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).arnr_strength <= 6 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"arnr_strength out of range [..6]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).cq_level <= 63 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"cq_level out of range [..63]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_bit_depth as core::ffi::c_uint
        >= AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).g_bit_depth as core::ffi::c_uint
            <= AOM_BITS_12 as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_bit_depth out of range [AOM_BITS_8..AOM_BITS_12]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*cfg).g_input_bit_depth >= 8 as core::ffi::c_uint
        && (*cfg).g_input_bit_depth <= 12 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"g_input_bit_depth out of range [8..12]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_input_bit_depth > (*cfg).g_bit_depth as core::ffi::c_uint {
        (*ctx).base.err_detail = b"Input bit-depth must not exceed codec bit-depth\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).content as core::ffi::c_uint
        >= AOM_CONTENT_DEFAULT as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).content as core::ffi::c_uint
            <= (AOM_CONTENT_INVALID as core::ffi::c_int - 1 as core::ffi::c_int)
                as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"content out of range [AOM_CONTENT_DEFAULT..AOM_CONTENT_INVALID - 1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_pass as core::ffi::c_uint
        >= AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        let packet_sz: size_t = ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t;
        let n_packets: core::ffi::c_int = ((*cfg).rc_twopass_stats_in.sz)
            .wrapping_div(packet_sz) as core::ffi::c_int;
        let mut stats: *const FIRSTPASS_STATS = 0 as *const FIRSTPASS_STATS;
        if ((*cfg).rc_twopass_stats_in.buf).is_null() {
            (*ctx).base.err_detail = b"rc_twopass_stats_in.buf not set.\0" as *const u8
                as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        if ((*cfg).rc_twopass_stats_in.sz).wrapping_rem(packet_sz) != 0 {
            (*ctx).base.err_detail = b"rc_twopass_stats_in.sz indicates truncated packet.\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        if (*cfg).rc_twopass_stats_in.sz < (2 as size_t).wrapping_mul(packet_sz) {
            (*ctx).base.err_detail = b"rc_twopass_stats_in requires at least two packets.\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        stats = ((*cfg).rc_twopass_stats_in.buf as *const FIRSTPASS_STATS)
            .offset(n_packets as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        if ((*stats).count + 0.5f64) as core::ffi::c_int
            != n_packets - 1 as core::ffi::c_int
        {
            (*ctx).base.err_detail = b"rc_twopass_stats_in missing EOS stats packet\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if (*extra_cfg).passes != -(1 as core::ffi::c_int)
        && (*cfg).g_pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).passes != 1 as core::ffi::c_int
    {
        (*ctx).base.err_detail = b"One pass encoding but passes != 1.\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).passes != -(1 as core::ffi::c_int)
        && (*cfg).g_pass as core::ffi::c_int > (*extra_cfg).passes
    {
        (*ctx).base.err_detail = b"Current pass is larger than total number of passes.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_profile == PROFILE_1 as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).monochrome != 0
    {
        (*ctx).base.err_detail = b"Monochrome is not supported in profile 1\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_profile <= PROFILE_1 as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).g_bit_depth as core::ffi::c_uint
            > AOM_BITS_10 as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).base.err_detail = b"Codec bit-depth 12 not supported in profile < 2\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).rc_end_usage as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        if !((*cfg).use_fixed_qp_offsets <= 2 as core::ffi::c_uint) {
            (*ctx).base.err_detail = b"use_fixed_qp_offsets out of range [..2]\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    } else if (*cfg).use_fixed_qp_offsets > 0 as core::ffi::c_uint {
        (*ctx).base.err_detail = b"--use_fixed_qp_offsets can only be used with --end-usage=q\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).color_primaries as core::ffi::c_uint
        >= AOM_CICP_CP_BT_709 as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).color_primaries as core::ffi::c_uint
            <= AOM_CICP_CP_EBU_3213 as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"color_primaries out of range [AOM_CICP_CP_BT_709..AOM_CICP_CP_EBU_3213]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).transfer_characteristics as core::ffi::c_uint
        >= AOM_CICP_TC_BT_709 as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).transfer_characteristics as core::ffi::c_uint
            <= AOM_CICP_TC_HLG as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"transfer_characteristics out of range [AOM_CICP_TC_BT_709..AOM_CICP_TC_HLG]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).matrix_coefficients as core::ffi::c_uint
        >= AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).matrix_coefficients as core::ffi::c_uint
            <= AOM_CICP_MC_YCGCO_RO as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"matrix_coefficients out of range [AOM_CICP_MC_IDENTITY..AOM_CICP_MC_YCGCO_RO]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).color_range >= 0 as core::ffi::c_int
        && (*extra_cfg).color_range <= 1 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"color_range out of range [0..1]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_pass as core::ffi::c_uint
        == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).rc_end_usage as core::ffi::c_uint
            == AOM_VBR as core::ffi::c_int as core::ffi::c_uint
    {
        if !((*extra_cfg).vbr_corpus_complexity_lap <= 10000 as core::ffi::c_uint) {
            (*ctx).base.err_detail = b"vbr_corpus_complexity_lap out of range [..MAX_VBR_CORPUS_COMPLEXITY]\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    } else if (*extra_cfg).vbr_corpus_complexity_lap != 0 as core::ffi::c_uint {
        (*ctx).base.err_detail = b"VBR corpus complexity is supported only in the case of single pass VBR mode.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).tuning as core::ffi::c_uint
        == AOM_TUNE_BUTTERAUGLI as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).base.err_detail = b"This error may be related to the wrong configuration options: try to set -DCONFIG_TUNE_BUTTERAUGLI=1 at the time CMake is run.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).tuning as core::ffi::c_uint
        >= AOM_TUNE_VMAF_WITH_PREPROCESSING as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).tuning as core::ffi::c_uint
            <= AOM_TUNE_VMAF_NEG_MAX_GAIN as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).base.err_detail = b"This error may be related to the wrong configuration options: try to set -DCONFIG_TUNE_VMAF=1 at the time CMake is run.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).tuning as core::ffi::c_uint
        >= AOM_TUNE_PSNR as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).tuning as core::ffi::c_uint
            <= AOM_TUNE_SSIMULACRA2 as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"tuning out of range [AOM_TUNE_PSNR..AOM_TUNE_SSIMULACRA2]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).dist_metric as core::ffi::c_uint
        >= AOM_DIST_METRIC_PSNR as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).dist_metric as core::ffi::c_uint
            <= AOM_DIST_METRIC_QM_PSNR as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"dist_metric out of range [AOM_DIST_METRIC_PSNR..AOM_DIST_METRIC_QM_PSNR]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).timing_info_type as core::ffi::c_uint
        >= AOM_TIMING_UNSPECIFIED as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).timing_info_type as core::ffi::c_uint
            <= AOM_TIMING_DEC_MODEL as core::ffi::c_int as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"timing_info_type out of range [AOM_TIMING_UNSPECIFIED..AOM_TIMING_DEC_MODEL]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).film_grain_test_vector >= 0 as core::ffi::c_int
        && (*extra_cfg).film_grain_test_vector <= 16 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"film_grain_test_vector out of range [0..16]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).lossless != 0 {
        if (*extra_cfg).aq_mode as core::ffi::c_int != 0 as core::ffi::c_int {
            (*ctx).base.err_detail = b"Only --aq_mode=0 can be used with --lossless=1.\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        if (*extra_cfg).enable_chroma_deltaq != 0 {
            (*ctx).base.err_detail = b"Only --enable_chroma_deltaq=0 can be used with --lossless=1.\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if !((*extra_cfg).max_reference_frames >= 3 as core::ffi::c_int
        && (*extra_cfg).max_reference_frames <= 7 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"max_reference_frames out of range [3..7]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_reduced_reference_set >= 0 as core::ffi::c_int
        && (*extra_cfg).enable_reduced_reference_set <= 1 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"enable_reduced_reference_set out of range [0..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_low_complexity_decode <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_low_complexity_decode out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).chroma_subsampling_x <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"chroma_subsampling_x out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).chroma_subsampling_y <= 1 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"chroma_subsampling_y out of range [..1]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).matrix_coefficients as core::ffi::c_uint
        == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
        && ((*extra_cfg).chroma_subsampling_x != 0 as core::ffi::c_uint
            || (*extra_cfg).chroma_subsampling_y != 0 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"Subsampling must be 0 with AOM_CICP_MC_IDENTITY.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).disable_trellis_quant <= 3 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"disable_trellis_quant out of range [..3]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).coeff_cost_upd_freq as core::ffi::c_uint >= 0 as core::ffi::c_uint
        && (*extra_cfg).coeff_cost_upd_freq as core::ffi::c_uint
            <= 3 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"coeff_cost_upd_freq out of range [0..3]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).mode_cost_upd_freq as core::ffi::c_uint >= 0 as core::ffi::c_uint
        && (*extra_cfg).mode_cost_upd_freq as core::ffi::c_uint
            <= 3 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"mode_cost_upd_freq out of range [0..3]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).mv_cost_upd_freq as core::ffi::c_uint >= 0 as core::ffi::c_uint
        && (*extra_cfg).mv_cost_upd_freq as core::ffi::c_uint <= 3 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"mv_cost_upd_freq out of range [0..3]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).dv_cost_upd_freq as core::ffi::c_uint >= 0 as core::ffi::c_uint
        && (*extra_cfg).dv_cost_upd_freq as core::ffi::c_uint <= 3 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"dv_cost_upd_freq out of range [0..3]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).min_partition_size >= 4 as core::ffi::c_int
        && (*extra_cfg).min_partition_size <= 128 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"min_partition_size out of range [4..128]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).max_partition_size >= 4 as core::ffi::c_int
        && (*extra_cfg).max_partition_size <= 128 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"max_partition_size out of range [4..128]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).min_partition_size <= (*extra_cfg).max_partition_size) {
        (*ctx).base.err_detail = b"min_partition_size out of range [..extra_cfg->max_partition_size]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_NUM_OPERATING_POINTS {
        let level_idx: core::ffi::c_int = (*extra_cfg).target_seq_level_idx[i as usize]
            as core::ffi::c_int;
        if is_valid_seq_level_idx(level_idx as AV1_LEVEL) == 0
            && level_idx != SEQ_LEVEL_KEEP_STATS as core::ffi::c_int
        {
            (*ctx).base.err_detail = b"Target sequence level index is invalid\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        i += 1;
    }
    if !((*extra_cfg).deltaq_strength <= 1000 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"deltaq_strength out of range [..1000]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).loopfilter_control as core::ffi::c_uint <= 3 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"loopfilter_control out of range [..3]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).skip_postproc_filtering != 0) as core::ffi::c_int
        as core::ffi::c_uint != (*extra_cfg).skip_postproc_filtering
    {
        (*ctx).base.err_detail = b"skip_postproc_filtering expected boolean\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).mode_ref_delta_enabled != 0) as core::ffi::c_int
        != (*extra_cfg).mode_ref_delta_enabled
    {
        (*ctx).base.err_detail = b"mode_ref_delta_enabled expected boolean\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).enable_cdef <= 3 as core::ffi::c_uint) {
        (*ctx).base.err_detail = b"enable_cdef out of range [..3]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).auto_intra_tools_off != 0) as core::ffi::c_int
        != (*extra_cfg).auto_intra_tools_off
    {
        (*ctx).base.err_detail = b"auto_intra_tools_off expected boolean\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).strict_level_conformance != 0) as core::ffi::c_int
        != (*extra_cfg).strict_level_conformance
    {
        (*ctx).base.err_detail = b"strict_level_conformance expected boolean\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*extra_cfg).sb_qp_sweep != 0) as core::ffi::c_int != (*extra_cfg).sb_qp_sweep {
        (*ctx).base.err_detail = b"sb_qp_sweep expected boolean\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).kf_max_pyr_height >= -(1 as core::ffi::c_int)
        && (*extra_cfg).kf_max_pyr_height <= 5 as core::ffi::c_int)
    {
        (*ctx).base.err_detail = b"kf_max_pyr_height out of range [-1..5]\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*extra_cfg).kf_max_pyr_height != -(1 as core::ffi::c_int)
        && (*extra_cfg).kf_max_pyr_height
            < (*extra_cfg).gf_min_pyr_height as core::ffi::c_int
    {
        (*ctx).base.err_detail = b"The value of kf-max-pyr-height should not be smaller than gf-min-pyr-height\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*extra_cfg).screen_detection_mode as core::ffi::c_uint
        >= 1 as core::ffi::c_uint
        && (*extra_cfg).screen_detection_mode as core::ffi::c_uint
            <= 2 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"screen_detection_mode out of range [1..2]\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn validate_img(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut img: *const aom_image_t,
) -> aom_codec_err_t {
    match (*img).fmt as core::ffi::c_uint {
        769 | 263 | 258 | 2817 | 2306 => {}
        262 | 2310 => {
            if (*ctx).cfg.g_profile == PROFILE_0 as core::ffi::c_int as core::ffi::c_uint
                && (*ctx).cfg.monochrome == 0
            {
                (*ctx).base.err_detail = b"Invalid image format. I444 images not supported in profile.\0"
                    as *const u8 as *const core::ffi::c_char;
                return AOM_CODEC_INVALID_PARAM;
            }
        }
        261 | 2309 => {
            if (*ctx).cfg.g_profile != PROFILE_2 as core::ffi::c_int as core::ffi::c_uint
            {
                (*ctx).base.err_detail = b"Invalid image format. I422 images not supported in profile.\0"
                    as *const u8 as *const core::ffi::c_char;
                return AOM_CODEC_INVALID_PARAM;
            }
        }
        _ => {
            (*ctx).base.err_detail = b"Invalid image format. Only YV12, NV12, I420, I422, I444 images are supported.\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
    }
    if (*img).d_w != (*ctx).cfg.g_w || (*img).d_h != (*ctx).cfg.g_h {
        (*ctx).base.err_detail = b"Image size must match encoder init configuration size\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*ctx).cfg.monochrome == 0
        && (*img).fmt as core::ffi::c_uint
            != AOM_IMG_FMT_NV12 as core::ffi::c_int as core::ffi::c_uint
        && (*img).stride[AOM_PLANE_U as usize] != (*img).stride[AOM_PLANE_V as usize]
    {
        (*ctx).base.err_detail = b"Image U/V strides must match\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    let matrix_coefficients: aom_matrix_coefficients_t = (*ctx)
        .extra_cfg
        .matrix_coefficients;
    if matrix_coefficients as core::ffi::c_uint
        == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
        && ((*img).x_chroma_shift != 0 as core::ffi::c_uint
            || (*img).y_chroma_shift != 0 as core::ffi::c_uint)
    {
        (*ctx).base.err_detail = b"Subsampling must be 0 with AOM_CICP_MC_IDENTITY.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*ctx).extra_cfg.validate_hbd_input != 0
        && (*img).fmt as core::ffi::c_uint
            & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
    {
        let bit_depth: core::ffi::c_uint = (*ctx).cfg.g_bit_depth as core::ffi::c_uint;
        let max_val: core::ffi::c_int = (1 as core::ffi::c_int) << bit_depth;
        let num_planes: core::ffi::c_int = if (*ctx).cfg.monochrome != 0 {
            1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < num_planes {
            let mut src: *const core::ffi::c_ushort = (*img).planes[plane as usize]
                as *const core::ffi::c_ushort;
            if src.is_null() {
                return AOM_CODEC_INVALID_PARAM;
            }
            let stride: core::ffi::c_uint = ((*img).stride[plane as usize]
                / 2 as core::ffi::c_int) as core::ffi::c_uint;
            let ph: core::ffi::c_uint = aom_img_plane_height(img, plane)
                as core::ffi::c_uint;
            let pw: core::ffi::c_uint = aom_img_plane_width(img, plane)
                as core::ffi::c_uint;
            let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while i < ph {
                let mut j: core::ffi::c_uint = 0 as core::ffi::c_uint;
                while j < pw {
                    if core::ptr::read_unaligned(src.offset(j as isize)) as core::ffi::c_int >= max_val {
                        (*ctx).base.err_detail = b"Input pixel value out of range for encoder bit depth\0"
                            as *const u8 as *const core::ffi::c_char;
                        return AOM_CODEC_INVALID_PARAM;
                    }
                    j = j.wrapping_add(1);
                }
                src = src.offset(stride as isize);
                i = i.wrapping_add(1);
            }
            plane += 1;
        }
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn get_image_bps(mut img: *const aom_image_t) -> core::ffi::c_int {
    match (*img).fmt as core::ffi::c_uint {
        769 | 263 | 258 => return 12 as core::ffi::c_int,
        261 => return 16 as core::ffi::c_int,
        262 => return 24 as core::ffi::c_int,
        2817 | 2306 => return 24 as core::ffi::c_int,
        2309 => return 32 as core::ffi::c_int,
        2310 => return 48 as core::ffi::c_int,
        _ => {}
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn disable_superres(superres_cfg: *mut SuperResCfg) {
    (*superres_cfg).superres_mode = AOM_SUPERRES_NONE;
    (*superres_cfg).superres_scale_denominator = SCALE_NUMERATOR as uint8_t;
    (*superres_cfg).superres_kf_scale_denominator = SCALE_NUMERATOR as uint8_t;
    (*superres_cfg).superres_qthresh = 255 as core::ffi::c_int;
    (*superres_cfg).superres_kf_qthresh = 255 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_auto_tiles(
    tile_cfg: *mut TileConfig,
    mut width: core::ffi::c_uint,
    mut height: core::ffi::c_uint,
    mut threads: core::ffi::c_uint,
) {
    let mut tile_cols_log2: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_rows_log2: core::ffi::c_int = 0 as core::ffi::c_int;
    if threads < 2 as core::ffi::c_uint {
        return;
    }
    let kMinTileArea: uint32_t = (128 as core::ffi::c_int * 128 as core::ffi::c_int)
        as uint32_t;
    let kMaxTiles: uint32_t = 32 as uint32_t;
    let mut frame_area: uint32_t = (width as uint32_t).wrapping_mul(height as uint32_t);
    let mut tiles: uint32_t = frame_area
        .wrapping_add(kMinTileArea)
        .wrapping_sub(1 as uint32_t)
        .wrapping_div(kMinTileArea);
    if tiles > kMaxTiles {
        tiles = kMaxTiles;
    }
    if tiles > threads as uint32_t {
        tiles = threads as uint32_t;
    }
    let mut tiles_log2: core::ffi::c_int = log2(tiles as core::ffi::c_double)
        as core::ffi::c_int;
    if width >= height {
        tile_cols_log2 = (tiles_log2 + 1 as core::ffi::c_int) / 2 as core::ffi::c_int;
        tile_rows_log2 = tiles_log2 - tile_cols_log2;
    } else {
        tile_rows_log2 = (tiles_log2 + 1 as core::ffi::c_int) / 2 as core::ffi::c_int;
        tile_cols_log2 = tiles_log2 - tile_rows_log2;
    }
    (*tile_cfg).tile_columns = tile_cols_log2;
    (*tile_cfg).tile_rows = tile_rows_log2;
}
unsafe extern "C-unwind" fn update_default_encoder_config(
    mut cfg: *const cfg_options_t,
    mut extra_cfg: *mut av1_extracfg,
) {
    (*extra_cfg).enable_cdef = (if (*cfg).disable_cdef == 0 as core::ffi::c_uint {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    }) as core::ffi::c_uint;
    (*extra_cfg).enable_restoration = ((*cfg).disable_lr == 0 as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_uint;
    (*extra_cfg).superblock_size = (if (*cfg).super_block_size == 64 as core::ffi::c_uint
    {
        AOM_SUPERBLOCK_SIZE_64X64 as core::ffi::c_int
    } else if (*cfg).super_block_size == 128 as core::ffi::c_uint {
        AOM_SUPERBLOCK_SIZE_128X128 as core::ffi::c_int
    } else {
        AOM_SUPERBLOCK_SIZE_DYNAMIC as core::ffi::c_int
    }) as aom_superblock_size_t;
    (*extra_cfg).enable_warped_motion = ((*cfg).disable_warp_motion
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_dist_wtd_comp = ((*cfg).disable_dist_wtd_comp
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_diff_wtd_comp = ((*cfg).disable_diff_wtd_comp
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_dual_filter = ((*cfg).disable_dual_filter
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_angle_delta = ((*cfg).disable_intra_angle_delta
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_rect_partitions = ((*cfg).disable_rect_partition_type
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_ab_partitions = ((*cfg).disable_ab_partition_type
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_1to4_partitions = ((*cfg).disable_1to4_partition_type
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).max_partition_size = (*cfg).max_partition_size as core::ffi::c_int;
    (*extra_cfg).min_partition_size = (*cfg).min_partition_size as core::ffi::c_int;
    (*extra_cfg).enable_intra_edge_filter = ((*cfg).disable_intra_edge_filter
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_tx64 = ((*cfg).disable_tx_64x64 == 0 as core::ffi::c_uint)
        as core::ffi::c_int;
    (*extra_cfg).enable_flip_idtx = ((*cfg).disable_flip_idtx == 0 as core::ffi::c_uint)
        as core::ffi::c_int;
    (*extra_cfg).enable_masked_comp = ((*cfg).disable_masked_comp
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_interintra_comp = ((*cfg).disable_inter_intra_comp
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_smooth_interintra = ((*cfg).disable_smooth_inter_intra
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_interinter_wedge = ((*cfg).disable_inter_inter_wedge
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_interintra_wedge = ((*cfg).disable_inter_intra_wedge
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_global_motion = ((*cfg).disable_global_motion
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_filter_intra = ((*cfg).disable_filter_intra
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_smooth_intra = ((*cfg).disable_smooth_intra
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_paeth_intra = ((*cfg).disable_paeth_intra
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_cfl_intra = ((*cfg).disable_cfl == 0 as core::ffi::c_uint)
        as core::ffi::c_int;
    (*extra_cfg).enable_obmc = ((*cfg).disable_obmc == 0 as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_uint;
    (*extra_cfg).enable_palette = ((*cfg).disable_palette == 0 as core::ffi::c_uint)
        as core::ffi::c_int;
    (*extra_cfg).enable_intrabc = ((*cfg).disable_intrabc == 0 as core::ffi::c_uint)
        as core::ffi::c_int;
    (*extra_cfg).disable_trellis_quant = (*cfg).disable_trellis_quant;
    (*extra_cfg).allow_ref_frame_mvs = ((*cfg).disable_ref_frame_mv
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_ref_frame_mvs = ((*cfg).disable_ref_frame_mv
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_onesided_comp = ((*cfg).disable_one_sided_comp
        == 0 as core::ffi::c_uint) as core::ffi::c_int;
    (*extra_cfg).enable_reduced_reference_set = (*cfg).reduced_reference_set
        as core::ffi::c_int;
    (*extra_cfg).reduced_tx_type_set = (*cfg).reduced_tx_type_set as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_encoder_config(
    mut oxcf: *mut AV1EncoderConfig,
    mut cfg: *const aom_codec_enc_cfg_t,
    mut extra_cfg: *mut av1_extracfg,
) {
    if (*cfg).encoder_cfg.init_by_cfg_file != 0 {
        update_default_encoder_config(&(*cfg).encoder_cfg, extra_cfg);
    }
    let tune_cfg: *mut TuneCfg = &raw mut (*oxcf).tune_cfg;
    let frm_dim_cfg: *mut FrameDimensionCfg = &raw mut (*oxcf).frm_dim_cfg;
    let tile_cfg: *mut TileConfig = &raw mut (*oxcf).tile_cfg;
    let resize_cfg: *mut ResizeCfg = &raw mut (*oxcf).resize_cfg;
    let gf_cfg: *mut GFConfig = &raw mut (*oxcf).gf_cfg;
    let part_cfg: *mut PartitionCfg = &raw mut (*oxcf).part_cfg;
    let intra_mode_cfg: *mut IntraModeCfg = &raw mut (*oxcf).intra_mode_cfg;
    let txfm_cfg: *mut TxfmSizeTypeCfg = &raw mut (*oxcf).txfm_cfg;
    let comp_type_cfg: *mut CompoundTypeCfg = &raw mut (*oxcf).comp_type_cfg;
    let superres_cfg: *mut SuperResCfg = &raw mut (*oxcf).superres_cfg;
    let kf_cfg: *mut KeyFrameCfg = &raw mut (*oxcf).kf_cfg;
    let dec_model_cfg: *mut DecoderModelCfg = &raw mut (*oxcf).dec_model_cfg;
    let rc_cfg: *mut RateControlCfg = &raw mut (*oxcf).rc_cfg;
    let q_cfg: *mut QuantizationCfg = &raw mut (*oxcf).q_cfg;
    let color_cfg: *mut ColorCfg = &raw mut (*oxcf).color_cfg;
    let input_cfg: *mut InputCfg = &raw mut (*oxcf).input_cfg;
    let algo_cfg: *mut AlgoCfg = &raw mut (*oxcf).algo_cfg;
    let tool_cfg: *mut ToolCfg = &raw mut (*oxcf).tool_cfg;
    (*oxcf).profile = (*cfg).g_profile as BITSTREAM_PROFILE;
    (*oxcf).max_threads = (*cfg).g_threads as core::ffi::c_int;
    match (*cfg).g_usage {
        AOM_USAGE_REALTIME => {
            (*oxcf).mode = REALTIME as core::ffi::c_int as MODE;
        }
        AOM_USAGE_ALL_INTRA => {
            (*oxcf).mode = ALLINTRA as core::ffi::c_int as MODE;
        }
        _ => {
            (*oxcf).mode = GOOD as core::ffi::c_int as MODE;
        }
    }
    (*frm_dim_cfg).width = (*cfg).g_w as core::ffi::c_int;
    (*frm_dim_cfg).height = (*cfg).g_h as core::ffi::c_int;
    (*frm_dim_cfg).forced_max_frame_width = (*cfg).g_forced_max_frame_width
        as core::ffi::c_int;
    (*frm_dim_cfg).forced_max_frame_height = (*cfg).g_forced_max_frame_height
        as core::ffi::c_int;
    (*frm_dim_cfg).render_width = (*extra_cfg).render_width;
    (*frm_dim_cfg).render_height = (*extra_cfg).render_height;
    (*input_cfg).input_bit_depth = (*cfg).g_input_bit_depth;
    (*input_cfg).init_framerate = (*cfg).g_timebase.den as core::ffi::c_double
        / (*cfg).g_timebase.num as core::ffi::c_double;
    if (*cfg).g_pass as core::ffi::c_uint
        >= AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        let packet_sz: size_t = ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t;
        let n_packets: core::ffi::c_int = ((*cfg).rc_twopass_stats_in.sz)
            .wrapping_div(packet_sz) as core::ffi::c_int;
        (*input_cfg).limit = (n_packets - 1 as core::ffi::c_int) as core::ffi::c_uint;
    } else {
        (*input_cfg).limit = (*cfg).g_limit;
    }
    (*input_cfg).chroma_subsampling_x = (*extra_cfg).chroma_subsampling_x;
    (*input_cfg).chroma_subsampling_y = (*extra_cfg).chroma_subsampling_y;
    if (*input_cfg).init_framerate > 180 as core::ffi::c_int as core::ffi::c_double {
        (*input_cfg).init_framerate = 30 as core::ffi::c_int as core::ffi::c_double;
        (*dec_model_cfg).timing_info_present = 0 as core::ffi::c_int != 0;
    }
    if (*extra_cfg).timing_info_type as core::ffi::c_uint
        == AOM_TIMING_EQUAL as core::ffi::c_int as core::ffi::c_uint
        || (*extra_cfg).timing_info_type as core::ffi::c_uint
            == AOM_TIMING_DEC_MODEL as core::ffi::c_int as core::ffi::c_uint
    {
        (*dec_model_cfg).timing_info_present = 1 as core::ffi::c_int != 0;
        (*dec_model_cfg).timing_info.num_units_in_display_tick = (*cfg).g_timebase.num
            as uint32_t;
        (*dec_model_cfg).timing_info.time_scale = (*cfg).g_timebase.den as uint32_t;
        (*dec_model_cfg).timing_info.num_ticks_per_picture = 1 as uint32_t;
    } else {
        (*dec_model_cfg).timing_info_present = 0 as core::ffi::c_int != 0;
    }
    if (*extra_cfg).timing_info_type as core::ffi::c_uint
        == AOM_TIMING_EQUAL as core::ffi::c_int as core::ffi::c_uint
    {
        (*dec_model_cfg).timing_info.equal_picture_interval = 1 as core::ffi::c_int;
        (*dec_model_cfg).decoder_model_info_present_flag = 0 as core::ffi::c_int != 0;
        (*dec_model_cfg).display_model_info_present_flag = 1 as core::ffi::c_int != 0;
    } else if (*extra_cfg).timing_info_type as core::ffi::c_uint
        == AOM_TIMING_DEC_MODEL as core::ffi::c_int as core::ffi::c_uint
    {
        (*dec_model_cfg).num_units_in_decoding_tick = (*cfg).g_timebase.num as uint32_t;
        (*dec_model_cfg).timing_info.equal_picture_interval = 0 as core::ffi::c_int;
        (*dec_model_cfg).decoder_model_info_present_flag = 1 as core::ffi::c_int != 0;
        (*dec_model_cfg).display_model_info_present_flag = 1 as core::ffi::c_int != 0;
    }
    (*oxcf).pass = (*cfg).g_pass;
    if (*extra_cfg).passes == -(1 as core::ffi::c_int) {
        if (*cfg).g_pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        {
            (*oxcf).passes = 1 as core::ffi::c_int;
        } else {
            (*oxcf).passes = 2 as core::ffi::c_int;
        }
    } else {
        (*oxcf).passes = (*extra_cfg).passes;
    }
    (*rc_cfg).max_intra_bitrate_pct = (*extra_cfg).rc_max_intra_bitrate_pct;
    (*rc_cfg).max_inter_bitrate_pct = (*extra_cfg).rc_max_inter_bitrate_pct;
    (*rc_cfg).gf_cbr_boost_pct = (*extra_cfg).gf_cbr_boost_pct;
    (*rc_cfg).mode = (*cfg).rc_end_usage;
    (*rc_cfg).min_cr = (*extra_cfg).min_cr;
    (*rc_cfg).best_allowed_q = if (*extra_cfg).lossless != 0 {
        0 as core::ffi::c_int
    } else {
        av1_quantizer_to_qindex((*cfg).rc_min_quantizer as core::ffi::c_int)
    };
    (*rc_cfg).worst_allowed_q = if (*extra_cfg).lossless != 0 {
        0 as core::ffi::c_int
    } else {
        av1_quantizer_to_qindex((*cfg).rc_max_quantizer as core::ffi::c_int)
    };
    (*rc_cfg).cq_level = av1_quantizer_to_qindex(
        (*extra_cfg).cq_level as core::ffi::c_int,
    );
    (*rc_cfg).under_shoot_pct = (*cfg).rc_undershoot_pct as core::ffi::c_int;
    (*rc_cfg).over_shoot_pct = (*cfg).rc_overshoot_pct as core::ffi::c_int;
    (*rc_cfg).maximum_buffer_size_ms = (*cfg).rc_buf_sz as int64_t;
    (*rc_cfg).starting_buffer_level_ms = (*cfg).rc_buf_initial_sz as int64_t;
    (*rc_cfg).optimal_buffer_level_ms = (*cfg).rc_buf_optimal_sz as int64_t;
    (*rc_cfg).target_bandwidth = (1000 as core::ffi::c_uint)
        .wrapping_mul((*cfg).rc_target_bitrate) as int64_t;
    (*rc_cfg).drop_frames_water_mark = (*cfg).rc_dropframe_thresh as core::ffi::c_int;
    (*rc_cfg).vbr_corpus_complexity_lap = (*extra_cfg).vbr_corpus_complexity_lap;
    (*rc_cfg).vbrbias = (*cfg).rc_2pass_vbr_bias_pct as core::ffi::c_int;
    (*rc_cfg).vbrmin_section = (*cfg).rc_2pass_vbr_minsection_pct as core::ffi::c_int;
    (*rc_cfg).vbrmax_section = (*cfg).rc_2pass_vbr_maxsection_pct as core::ffi::c_int;
    (*rc_cfg).force_max_q = (*extra_cfg).force_max_q as core::ffi::c_int;
    (*tool_cfg).bit_depth = (*cfg).g_bit_depth;
    (*tool_cfg).cdef_control = (*extra_cfg).enable_cdef as CDEF_CONTROL;
    (*tool_cfg).enable_restoration = if (*cfg).g_usage == AOM_USAGE_REALTIME {
        0 as core::ffi::c_uint
    } else {
        (*extra_cfg).enable_restoration
    } != 0;
    (*tool_cfg).force_video_mode = (*extra_cfg).force_video_mode != 0;
    (*tool_cfg).enable_palette = (*extra_cfg).enable_palette != 0;
    (*tool_cfg).enable_ref_frame_mvs = (*extra_cfg).allow_ref_frame_mvs != 0
        && (*cfg).large_scale_tile == 0;
    (*tool_cfg).superblock_size = (*extra_cfg).superblock_size;
    (*tool_cfg).enable_monochrome = (*cfg).monochrome != 0;
    (*tool_cfg).full_still_picture_hdr = (*cfg).full_still_picture_hdr
        != 0 as core::ffi::c_uint;
    (*tool_cfg).enable_dual_filter = (*extra_cfg).enable_dual_filter != 0;
    (*tool_cfg).enable_order_hint = (*extra_cfg).enable_order_hint != 0;
    (*tool_cfg).enable_interintra_comp = (*extra_cfg).enable_interintra_comp != 0;
    (*tool_cfg).ref_frame_mvs_present = (*extra_cfg).enable_ref_frame_mvs
        & (*extra_cfg).enable_order_hint != 0;
    (*tool_cfg).enable_global_motion = (*extra_cfg).enable_global_motion != 0
        && (*cfg).g_usage != AOM_USAGE_REALTIME && (*cfg).large_scale_tile == 0;
    (*tool_cfg).error_resilient_mode = (*cfg).g_error_resilient
        | (*extra_cfg).error_resilient_mode as aom_codec_er_flags_t != 0;
    (*tool_cfg).frame_parallel_decoding_mode = (*extra_cfg).frame_parallel_decoding_mode
        != 0;
    (*q_cfg).using_qm = (*extra_cfg).enable_qm != 0;
    (*q_cfg).qm_minlevel = (*extra_cfg).qm_min as core::ffi::c_int;
    (*q_cfg).qm_maxlevel = (*extra_cfg).qm_max as core::ffi::c_int;
    (*q_cfg).quant_b_adapt = (*extra_cfg).quant_b_adapt;
    (*q_cfg).enable_chroma_deltaq = (*extra_cfg).enable_chroma_deltaq != 0;
    (*q_cfg).aq_mode = (*extra_cfg).aq_mode;
    (*q_cfg).deltaq_mode = (*extra_cfg).deltaq_mode;
    (*q_cfg).deltaq_strength = (*extra_cfg).deltaq_strength;
    (*q_cfg).use_fixed_qp_offsets = (if (*rc_cfg).mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        (*cfg).use_fixed_qp_offsets
    } else {
        0 as core::ffi::c_uint
    }) as core::ffi::c_int;
    (*q_cfg).enable_hdr_deltaq = (*q_cfg).deltaq_mode as core::ffi::c_int
        == DELTA_Q_HDR as core::ffi::c_int
        && (*cfg).g_bit_depth as core::ffi::c_uint
            == AOM_BITS_10 as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).color_primaries as core::ffi::c_uint
            == AOM_CICP_CP_BT_2020 as core::ffi::c_int as core::ffi::c_uint;
    (*tool_cfg).enable_deltalf_mode = (*q_cfg).deltaq_mode as core::ffi::c_int
        != NO_DELTA_Q as core::ffi::c_int && (*extra_cfg).deltalf_mode != 0;
    (*oxcf).cost_upd_freq.coeff = (*extra_cfg).coeff_cost_upd_freq;
    (*oxcf).cost_upd_freq.mode = (*extra_cfg).mode_cost_upd_freq;
    (*oxcf).cost_upd_freq.mv = (if (*cfg).kf_max_dist != 0 as core::ffi::c_uint {
        (*extra_cfg).mv_cost_upd_freq as core::ffi::c_uint
    } else {
        COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
    }) as COST_UPDATE_TYPE;
    (*oxcf).cost_upd_freq.dv = (*extra_cfg).dv_cost_upd_freq;
    (*resize_cfg).resize_mode = (*cfg).rc_resize_mode as RESIZE_MODE;
    (*resize_cfg).resize_scale_denominator = (*cfg).rc_resize_denominator as uint8_t;
    (*resize_cfg).resize_kf_scale_denominator = (*cfg).rc_resize_kf_denominator
        as uint8_t;
    if (*resize_cfg).resize_mode as core::ffi::c_int == RESIZE_FIXED as core::ffi::c_int
        && (*resize_cfg).resize_scale_denominator as core::ffi::c_int == SCALE_NUMERATOR
        && (*resize_cfg).resize_kf_scale_denominator as core::ffi::c_int
            == SCALE_NUMERATOR
    {
        (*resize_cfg).resize_mode = RESIZE_NONE as core::ffi::c_int as RESIZE_MODE;
    }
    (*algo_cfg).enable_overlay = (*extra_cfg).enable_overlay != 0;
    (*algo_cfg).disable_trellis_quant = (*extra_cfg).disable_trellis_quant
        as core::ffi::c_int;
    (*algo_cfg).sharpness = (*extra_cfg).sharpness as core::ffi::c_int;
    (*algo_cfg).enable_adaptive_sharpness = (*extra_cfg).enable_adaptive_sharpness != 0;
    (*algo_cfg).arnr_max_frames = (*extra_cfg).arnr_max_frames as core::ffi::c_int;
    (*algo_cfg).arnr_strength = (*extra_cfg).arnr_strength as core::ffi::c_int;
    (*algo_cfg).cdf_update_mode = (*extra_cfg).cdf_update_mode as uint8_t;
    (*algo_cfg).enable_tpl_model = if (*resize_cfg).resize_mode as core::ffi::c_int != 0
    {
        0 as core::ffi::c_uint
    } else {
        (*extra_cfg).enable_tpl_model
    } != 0;
    (*algo_cfg).loopfilter_control = (*extra_cfg).loopfilter_control;
    (*algo_cfg).skip_postproc_filtering = (*extra_cfg).skip_postproc_filtering != 0;
    (*algo_cfg).mode_ref_delta_enabled = (*extra_cfg).mode_ref_delta_enabled;
    (*algo_cfg).screen_detection_mode = (*extra_cfg).screen_detection_mode;
    (*oxcf).twopass_stats_in = (*cfg).rc_twopass_stats_in;
    if !((*extra_cfg).two_pass_output).is_null() {
        (*oxcf).two_pass_output = (*extra_cfg).two_pass_output;
    }
    (*oxcf).second_pass_log = (*extra_cfg).second_pass_log;
    (*kf_cfg).fwd_kf_enabled = (*cfg).fwd_kf_enabled != 0;
    (*kf_cfg).auto_key = (*cfg).kf_mode as core::ffi::c_uint
        == AOM_KF_AUTO as core::ffi::c_int as core::ffi::c_uint
        && (*cfg).kf_min_dist != (*cfg).kf_max_dist;
    (*kf_cfg).key_freq_min = (*cfg).kf_min_dist as core::ffi::c_int;
    (*kf_cfg).key_freq_max = (*cfg).kf_max_dist as core::ffi::c_int;
    (*kf_cfg).sframe_dist = (*cfg).sframe_dist as core::ffi::c_int;
    (*kf_cfg).sframe_mode = (*cfg).sframe_mode as core::ffi::c_int;
    (*kf_cfg).enable_sframe = (*extra_cfg).s_frame_mode != 0;
    (*kf_cfg).enable_keyframe_filtering = (*extra_cfg).enable_keyframe_filtering
        as core::ffi::c_int;
    (*kf_cfg).fwd_kf_dist = (*extra_cfg).fwd_kf_dist;
    if (*cfg).kf_max_dist == 0 as core::ffi::c_uint {
        (*kf_cfg).enable_keyframe_filtering = 0 as core::ffi::c_int;
    }
    (*kf_cfg).enable_intrabc = (*extra_cfg).enable_intrabc != 0;
    (*oxcf).speed = (*extra_cfg).cpu_used;
    if (*oxcf).mode as core::ffi::c_int == REALTIME as core::ffi::c_int {
        if (if (*cfg).g_w < (*cfg).g_h { (*cfg).g_w } else { (*cfg).g_h })
            >= 1080 as core::ffi::c_uint && (*oxcf).speed < 7 as core::ffi::c_int
        {
            (*oxcf).speed = 7 as core::ffi::c_int;
        }
    }
    let is_low_complexity_decode_mode_supported: core::ffi::c_int = ((*cfg).g_usage
        == AOM_USAGE_GOOD_QUALITY
        && ((*oxcf).speed >= 1 as core::ffi::c_int
            && (*oxcf).speed <= 3 as core::ffi::c_int)
        && ((if (*cfg).g_w < (*cfg).g_h { (*cfg).g_w } else { (*cfg).g_h })
            >= 608 as core::ffi::c_uint
            && (if (*cfg).g_w < (*cfg).g_h { (*cfg).g_w } else { (*cfg).g_h })
                <= 1080 as core::ffi::c_uint)
        && (*extra_cfg).sharpness != 3 as core::ffi::c_uint) as core::ffi::c_int;
    (*oxcf).enable_low_complexity_decode = ((*extra_cfg).enable_low_complexity_decode
        != 0 && is_low_complexity_decode_mode_supported != 0) as core::ffi::c_int
        as core::ffi::c_uint;
    (*color_cfg).color_primaries = (*extra_cfg).color_primaries;
    (*color_cfg).transfer_characteristics = (*extra_cfg).transfer_characteristics;
    (*color_cfg).matrix_coefficients = (*extra_cfg).matrix_coefficients;
    (*color_cfg).color_range = (*extra_cfg).color_range as aom_color_range_t;
    (*color_cfg).chroma_sample_position = (*extra_cfg).chroma_sample_position;
    (*gf_cfg).lag_in_frames = clamp(
        (*cfg).g_lag_in_frames as core::ffi::c_int,
        0 as core::ffi::c_int,
        MAX_LAG_BUFFERS,
    );
    if (*oxcf).mode as core::ffi::c_int == GOOD as core::ffi::c_int
        && (*gf_cfg).lag_in_frames >= 32 as core::ffi::c_int
        && (*gf_cfg).lag_in_frames < 39 as core::ffi::c_int
    {
        (*gf_cfg).lag_in_frames = if (39 as core::ffi::c_int) < 48 as core::ffi::c_int {
            39 as core::ffi::c_int
        } else {
            48 as core::ffi::c_int
        };
    }
    (*gf_cfg).enable_auto_arf = (*extra_cfg).enable_auto_alt_ref != 0;
    (*gf_cfg).enable_auto_brf = (*extra_cfg).enable_auto_bwd_ref != 0;
    (*gf_cfg).min_gf_interval = (*extra_cfg).min_gf_interval as core::ffi::c_int;
    (*gf_cfg).max_gf_interval = (*extra_cfg).max_gf_interval as core::ffi::c_int;
    (*gf_cfg).gf_min_pyr_height = (*extra_cfg).gf_min_pyr_height as core::ffi::c_int;
    (*gf_cfg).gf_max_pyr_height = (*extra_cfg).gf_max_pyr_height as core::ffi::c_int;
    (*tune_cfg).tuning = (*extra_cfg).tuning;
    (*tune_cfg).vmaf_model_path = (*extra_cfg).vmaf_model_path;
    (*tune_cfg).content = (*extra_cfg).content;
    if (*cfg).large_scale_tile != 0 {
        (*tune_cfg).film_grain_test_vector = 0 as core::ffi::c_int;
        (*tune_cfg).film_grain_table_filename = 0 as *const core::ffi::c_char;
    } else {
        (*tune_cfg).film_grain_test_vector = (*extra_cfg).film_grain_test_vector;
        (*tune_cfg).film_grain_table_filename = (*extra_cfg).film_grain_table_filename;
    }
    (*tune_cfg).dist_metric = (*extra_cfg).dist_metric;
    (*oxcf).noise_level = (*extra_cfg).noise_level;
    (*oxcf).noise_block_size = (*extra_cfg).noise_block_size;
    (*oxcf).enable_dnl_denoising = (*extra_cfg).enable_dnl_denoising;
    (*tile_cfg).num_tile_groups = (*extra_cfg).num_tg;
    if (*cfg).large_scale_tile != 0 {
        (*tile_cfg).num_tile_groups = 1 as core::ffi::c_uint;
    }
    (*tile_cfg).mtu = (*extra_cfg).mtu_size;
    (*tile_cfg).enable_large_scale_tile = (*cfg).large_scale_tile != 0;
    (*tile_cfg).enable_single_tile_decoding = if (*tile_cfg).enable_large_scale_tile
        as core::ffi::c_int != 0
    {
        (*extra_cfg).single_tile_decoding
    } else {
        0 as core::ffi::c_uint
    } != 0;
    if (*extra_cfg).auto_tiles != 0 {
        set_auto_tiles(tile_cfg, (*cfg).g_w, (*cfg).g_h, (*cfg).g_threads);
        (*extra_cfg).tile_columns = (*tile_cfg).tile_columns as core::ffi::c_uint;
        (*extra_cfg).tile_rows = (*tile_cfg).tile_rows as core::ffi::c_uint;
    } else {
        (*tile_cfg).tile_columns = (*extra_cfg).tile_columns as core::ffi::c_int;
        (*tile_cfg).tile_rows = (*extra_cfg).tile_rows as core::ffi::c_int;
    }
    (*tile_cfg).tile_width_count = if (*cfg).tile_width_count < 64 as core::ffi::c_int {
        (*cfg).tile_width_count
    } else {
        64 as core::ffi::c_int
    };
    (*tile_cfg).tile_height_count = if (*cfg).tile_height_count < 64 as core::ffi::c_int
    {
        (*cfg).tile_height_count
    } else {
        64 as core::ffi::c_int
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*tile_cfg).tile_width_count {
        (*tile_cfg).tile_widths[i as usize] = (*cfg).tile_widths[i as usize];
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*tile_cfg).tile_height_count {
        (*tile_cfg).tile_heights[i_0 as usize] = (*cfg).tile_heights[i_0 as usize];
        i_0 += 1;
    }
    (*tile_cfg).enable_ext_tile_debug = (*extra_cfg).ext_tile_debug != 0;
    if (*tile_cfg).enable_large_scale_tile {
        if (*extra_cfg).superblock_size as core::ffi::c_uint
            != AOM_SUPERBLOCK_SIZE_64X64 as core::ffi::c_int as core::ffi::c_uint
            && (*extra_cfg).superblock_size as core::ffi::c_uint
                != AOM_SUPERBLOCK_SIZE_128X128 as core::ffi::c_int as core::ffi::c_uint
        {
            (*tool_cfg).superblock_size = AOM_SUPERBLOCK_SIZE_64X64;
        }
    }
    (*oxcf).ref_frm_cfg.max_reference_frames = (*extra_cfg).max_reference_frames
        as core::ffi::c_uint;
    (*oxcf).ref_frm_cfg.enable_reduced_reference_set = (*extra_cfg)
        .enable_reduced_reference_set != 0;
    (*oxcf).ref_frm_cfg.enable_onesided_comp = (*extra_cfg).enable_onesided_comp != 0;
    (*oxcf).row_mt = (*extra_cfg).row_mt != 0;
    (*oxcf).fp_mt = (*extra_cfg).fp_mt != 0;
    (*oxcf).motion_mode_cfg.enable_obmc = (*extra_cfg).enable_obmc != 0;
    (*oxcf).motion_mode_cfg.enable_warped_motion = (*extra_cfg).enable_warped_motion
        != 0;
    if (*cfg).g_usage == AOM_USAGE_REALTIME && (*oxcf).speed >= 7 as core::ffi::c_int
        && (*oxcf).tune_cfg.content as core::ffi::c_uint
            == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
    {
        (*oxcf).motion_mode_cfg.enable_warped_motion = 0 as core::ffi::c_int != 0;
        (*oxcf).motion_mode_cfg.allow_warped_motion = 0 as core::ffi::c_int != 0;
    } else {
        (*oxcf).motion_mode_cfg.allow_warped_motion = (*extra_cfg).allow_warped_motion
            & (*extra_cfg).enable_warped_motion != 0;
    }
    (*part_cfg).enable_rect_partitions = (*extra_cfg).enable_rect_partitions != 0;
    (*part_cfg).enable_ab_partitions = (*extra_cfg).enable_ab_partitions != 0;
    (*part_cfg).enable_1to4_partitions = (*extra_cfg).enable_1to4_partitions != 0;
    (*part_cfg).min_partition_size = (*extra_cfg).min_partition_size as BLOCK_SIZE;
    (*part_cfg).max_partition_size = (*extra_cfg).max_partition_size as BLOCK_SIZE;
    (*intra_mode_cfg).enable_angle_delta = (*extra_cfg).enable_angle_delta != 0;
    (*intra_mode_cfg).enable_intra_edge_filter = (*extra_cfg).enable_intra_edge_filter
        != 0;
    (*intra_mode_cfg).enable_filter_intra = (*extra_cfg).enable_filter_intra != 0;
    (*intra_mode_cfg).enable_smooth_intra = (*extra_cfg).enable_smooth_intra != 0;
    (*intra_mode_cfg).enable_paeth_intra = (*extra_cfg).enable_paeth_intra != 0;
    (*intra_mode_cfg).enable_cfl_intra = (*extra_cfg).enable_cfl_intra != 0;
    (*intra_mode_cfg).enable_directional_intra = (*extra_cfg).enable_directional_intra
        != 0;
    (*intra_mode_cfg).enable_diagonal_intra = (*extra_cfg).enable_diagonal_intra != 0;
    (*intra_mode_cfg).auto_intra_tools_off = (*extra_cfg).auto_intra_tools_off != 0;
    (*txfm_cfg).enable_tx64 = (*extra_cfg).enable_tx64 != 0;
    (*txfm_cfg).enable_flip_idtx = (*extra_cfg).enable_flip_idtx != 0;
    (*txfm_cfg).enable_rect_tx = (*extra_cfg).enable_rect_tx != 0;
    (*txfm_cfg).reduced_tx_type_set = (*extra_cfg).reduced_tx_type_set != 0;
    (*txfm_cfg).use_intra_dct_only = (*extra_cfg).use_intra_dct_only != 0;
    (*txfm_cfg).use_inter_dct_only = (*extra_cfg).use_inter_dct_only != 0;
    (*txfm_cfg).use_intra_default_tx_only = (*extra_cfg).use_intra_default_tx_only != 0;
    (*txfm_cfg).enable_tx_size_search = (*extra_cfg).enable_tx_size_search != 0;
    (*comp_type_cfg).enable_dist_wtd_comp = (*extra_cfg).enable_dist_wtd_comp
        & (*extra_cfg).enable_order_hint != 0;
    (*comp_type_cfg).enable_masked_comp = (*extra_cfg).enable_masked_comp != 0;
    (*comp_type_cfg).enable_diff_wtd_comp = (*extra_cfg).enable_masked_comp
        & (*extra_cfg).enable_diff_wtd_comp != 0;
    (*comp_type_cfg).enable_interinter_wedge = (*extra_cfg).enable_masked_comp
        & (*extra_cfg).enable_interinter_wedge != 0;
    (*comp_type_cfg).enable_smooth_interintra = (*extra_cfg).enable_interintra_comp != 0
        && (*extra_cfg).enable_smooth_interintra != 0;
    (*comp_type_cfg).enable_interintra_wedge = (*extra_cfg).enable_interintra_comp
        & (*extra_cfg).enable_interintra_wedge != 0;
    if (*extra_cfg).lossless != 0 || (*cfg).large_scale_tile != 0 {
        disable_superres(superres_cfg);
    } else {
        (*superres_cfg).superres_mode = (*cfg).rc_superres_mode;
        (*superres_cfg).superres_scale_denominator = (*cfg).rc_superres_denominator
            as uint8_t;
        (*superres_cfg).superres_kf_scale_denominator = (*cfg).rc_superres_kf_denominator
            as uint8_t;
        (*superres_cfg).superres_qthresh = av1_quantizer_to_qindex(
            (*cfg).rc_superres_qthresh as core::ffi::c_int,
        );
        (*superres_cfg).superres_kf_qthresh = av1_quantizer_to_qindex(
            (*cfg).rc_superres_kf_qthresh as core::ffi::c_int,
        );
        if (*superres_cfg).superres_mode as core::ffi::c_uint
            == AOM_SUPERRES_FIXED as core::ffi::c_int as core::ffi::c_uint
            && (*superres_cfg).superres_scale_denominator as core::ffi::c_int
                == SCALE_NUMERATOR
            && (*superres_cfg).superres_kf_scale_denominator as core::ffi::c_int
                == SCALE_NUMERATOR
        {
            disable_superres(superres_cfg);
        }
        if (*superres_cfg).superres_mode as core::ffi::c_uint
            == AOM_SUPERRES_QTHRESH as core::ffi::c_int as core::ffi::c_uint
            && (*superres_cfg).superres_qthresh == 255 as core::ffi::c_int
            && (*superres_cfg).superres_kf_qthresh == 255 as core::ffi::c_int
        {
            disable_superres(superres_cfg);
        }
    }
    (*superres_cfg).enable_superres = (*superres_cfg).superres_mode as core::ffi::c_uint
        != AOM_SUPERRES_NONE as core::ffi::c_int as core::ffi::c_uint
        && (*extra_cfg).enable_superres != 0;
    if !(*superres_cfg).enable_superres {
        disable_superres(superres_cfg);
    }
    if (*input_cfg).limit == 1 as core::ffi::c_uint {
        (*dec_model_cfg).display_model_info_present_flag = 0 as core::ffi::c_int != 0;
        (*dec_model_cfg).timing_info_present = 0 as core::ffi::c_int != 0;
    }
    (*oxcf).save_as_annexb = (*cfg).save_as_annexb != 0;
    (*oxcf).unit_test_cfg.motion_vector_unit_test = (*extra_cfg).motion_vector_unit_test;
    (*oxcf).unit_test_cfg.sb_multipass_unit_test = (*extra_cfg).sb_multipass_unit_test;
    (*oxcf).border_in_pixels = av1_get_enc_border_size(
        av1_is_resize_needed(oxcf),
        (*oxcf).kf_cfg.key_freq_max == 0 as core::ffi::c_int,
        BLOCK_128X128,
    );
    memcpy(
        ((*oxcf).target_seq_level_idx).as_mut_ptr() as *mut core::ffi::c_void,
        ((*extra_cfg).target_seq_level_idx).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[AV1_LEVEL; 32]>() as size_t,
    );
    (*oxcf).tier_mask = (*extra_cfg).tier_mask;
    (*oxcf).partition_info_path = (*extra_cfg).partition_info_path;
    (*oxcf).enable_rate_guide_deltaq = (*extra_cfg).enable_rate_guide_deltaq;
    (*oxcf).rate_distribution_info = (*extra_cfg).rate_distribution_info;
    (*oxcf).strict_level_conformance = (*extra_cfg).strict_level_conformance;
    (*oxcf).kf_max_pyr_height = (*extra_cfg).kf_max_pyr_height;
    (*oxcf).sb_qp_sweep = (*extra_cfg).sb_qp_sweep;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_encoder_config(
    mut cfg: *const aom_codec_enc_cfg_t,
) -> AV1EncoderConfig {
    let mut oxcf: AV1EncoderConfig = AV1EncoderConfig {
        input_cfg: InputCfg {
            init_framerate: 0.,
            input_bit_depth: 0,
            limit: 0,
            chroma_subsampling_x: 0,
            chroma_subsampling_y: 0,
        },
        frm_dim_cfg: FrameDimensionCfg {
            width: 0,
            height: 0,
            forced_max_frame_width: 0,
            forced_max_frame_height: 0,
            render_width: 0,
            render_height: 0,
        },
        algo_cfg: AlgoCfg {
            sharpness: 0,
            enable_adaptive_sharpness: false,
            disable_trellis_quant: 0,
            arnr_max_frames: 0,
            arnr_strength: 0,
            cdf_update_mode: 0,
            enable_tpl_model: false,
            enable_overlay: false,
            loopfilter_control: LOOPFILTER_NONE,
            skip_postproc_filtering: false,
            mode_ref_delta_enabled: 0,
            screen_detection_mode: 0 as aom_screen_detection_mode,
        },
        kf_cfg: KeyFrameCfg {
            key_freq_min: 0,
            key_freq_max: 0,
            enable_keyframe_filtering: 0,
            sframe_dist: 0,
            sframe_mode: 0,
            auto_key: false,
            fwd_kf_dist: 0,
            fwd_kf_enabled: false,
            enable_sframe: false,
            enable_intrabc: false,
        },
        rc_cfg: RateControlCfg {
            starting_buffer_level_ms: 0,
            optimal_buffer_level_ms: 0,
            maximum_buffer_size_ms: 0,
            target_bandwidth: 0,
            vbr_corpus_complexity_lap: 0,
            max_intra_bitrate_pct: 0,
            max_inter_bitrate_pct: 0,
            gf_cbr_boost_pct: 0,
            min_cr: 0,
            drop_frames_water_mark: 0,
            under_shoot_pct: 0,
            over_shoot_pct: 0,
            worst_allowed_q: 0,
            best_allowed_q: 0,
            cq_level: 0,
            mode: AOM_VBR,
            vbrbias: 0,
            vbrmin_section: 0,
            vbrmax_section: 0,
            max_consec_drop_ms: 0,
            force_max_q: 0,
        },
        q_cfg: QuantizationCfg {
            use_fixed_qp_offsets: 0,
            qm_minlevel: 0,
            qm_maxlevel: 0,
            quant_b_adapt: 0,
            aq_mode: 0,
            deltaq_mode: 0,
            deltaq_strength: 0,
            enable_chroma_deltaq: false,
            enable_hdr_deltaq: false,
            using_qm: false,
        },
        resize_cfg: ResizeCfg {
            resize_mode: 0,
            resize_scale_denominator: 0,
            resize_kf_scale_denominator: 0,
        },
        superres_cfg: SuperResCfg {
            superres_qthresh: 0,
            superres_kf_qthresh: 0,
            superres_scale_denominator: 0,
            superres_kf_scale_denominator: 0,
            superres_mode: AOM_SUPERRES_NONE,
            enable_superres: false,
        },
        twopass_stats_in: aom_fixed_buf {
            buf: 0 as *const core::ffi::c_void as *mut core::ffi::c_void,
            sz: 0,
        },
        tool_cfg: ToolCfg {
            bit_depth: 0 as aom_bit_depth_t,
            superblock_size: AOM_SUPERBLOCK_SIZE_64X64,
            enable_deltalf_mode: false,
            cdef_control: CDEF_NONE,
            enable_restoration: false,
            force_video_mode: false,
            error_resilient_mode: false,
            frame_parallel_decoding_mode: false,
            enable_monochrome: false,
            full_still_picture_hdr: false,
            enable_dual_filter: false,
            enable_order_hint: false,
            ref_frame_mvs_present: false,
            enable_ref_frame_mvs: false,
            enable_interintra_comp: false,
            enable_global_motion: false,
            enable_palette: false,
        },
        gf_cfg: GFConfig {
            lag_in_frames: 0,
            min_gf_interval: 0,
            max_gf_interval: 0,
            gf_min_pyr_height: 0,
            gf_max_pyr_height: 0,
            enable_auto_arf: false,
            enable_auto_brf: false,
        },
        tile_cfg: TileConfig {
            num_tile_groups: 0,
            mtu: 0,
            tile_columns: 0,
            tile_rows: 0,
            tile_width_count: 0,
            tile_height_count: 0,
            tile_widths: [0; 64],
            tile_heights: [0; 64],
            enable_large_scale_tile: false,
            enable_single_tile_decoding: false,
            enable_ext_tile_debug: false,
        },
        tune_cfg: TuneCfg {
            vmaf_model_path: 0 as *const core::ffi::c_char,
            film_grain_table_filename: 0 as *const core::ffi::c_char,
            tuning: AOM_TUNE_PSNR,
            content: AOM_CONTENT_DEFAULT,
            film_grain_test_vector: 0,
            dist_metric: AOM_DIST_METRIC_PSNR,
        },
        color_cfg: ColorCfg {
            color_primaries: AOM_CICP_CP_RESERVED_0,
            transfer_characteristics: AOM_CICP_TC_RESERVED_0,
            matrix_coefficients: AOM_CICP_MC_IDENTITY,
            chroma_sample_position: AOM_CSP_UNKNOWN,
            color_range: AOM_CR_STUDIO_RANGE,
        },
        dec_model_cfg: DecoderModelCfg {
            timing_info: aom_timing {
                num_units_in_display_tick: 0,
                time_scale: 0,
                equal_picture_interval: 0,
                num_ticks_per_picture: 0,
            },
            num_units_in_decoding_tick: 0,
            decoder_model_info_present_flag: false,
            display_model_info_present_flag: false,
            timing_info_present: false,
        },
        ref_frm_cfg: RefFrameCfg {
            max_reference_frames: 0,
            enable_reduced_reference_set: false,
            enable_onesided_comp: false,
        },
        unit_test_cfg: UnitTestCfg {
            motion_vector_unit_test: 0,
            sb_multipass_unit_test: 0,
        },
        motion_mode_cfg: MotionModeCfg {
            enable_warped_motion: false,
            allow_warped_motion: false,
            enable_obmc: false,
        },
        intra_mode_cfg: IntraModeCfg {
            enable_intra_edge_filter: false,
            enable_filter_intra: false,
            enable_smooth_intra: false,
            enable_paeth_intra: false,
            enable_cfl_intra: false,
            enable_directional_intra: false,
            enable_diagonal_intra: false,
            enable_angle_delta: false,
            auto_intra_tools_off: false,
        },
        txfm_cfg: TxfmSizeTypeCfg {
            enable_tx64: false,
            enable_flip_idtx: false,
            enable_rect_tx: false,
            reduced_tx_type_set: false,
            use_intra_dct_only: false,
            use_inter_dct_only: false,
            use_intra_default_tx_only: false,
            enable_tx_size_search: false,
        },
        comp_type_cfg: CompoundTypeCfg {
            enable_dist_wtd_comp: false,
            enable_masked_comp: false,
            enable_smooth_interintra: false,
            enable_diff_wtd_comp: false,
            enable_interinter_wedge: false,
            enable_interintra_wedge: false,
        },
        part_cfg: PartitionCfg {
            enable_rect_partitions: false,
            enable_ab_partitions: false,
            enable_1to4_partitions: false,
            min_partition_size: BLOCK_4X4,
            max_partition_size: BLOCK_4X4,
        },
        cost_upd_freq: CostUpdateFreq {
            coeff: COST_UPD_SB,
            mode: COST_UPD_SB,
            mv: COST_UPD_SB,
            dv: COST_UPD_SB,
        },
        noise_level: 0.,
        noise_block_size: 0,
        enable_dnl_denoising: 0,
        tier_mask: 0,
        border_in_pixels: 0,
        max_threads: 0,
        speed: 0,
        enable_low_complexity_decode: 0,
        target_seq_level_idx: [0; 32],
        profile: 0,
        pass: AOM_RC_ONE_PASS,
        passes: 0,
        two_pass_output: 0 as *const core::ffi::c_char,
        second_pass_log: 0 as *const core::ffi::c_char,
        mode: 0,
        row_mt: false,
        fp_mt: false,
        use_highbitdepth: false,
        save_as_annexb: false,
        partition_info_path: 0 as *const core::ffi::c_char,
        enable_rate_guide_deltaq: 0,
        rate_distribution_info: 0 as *const core::ffi::c_char,
        strict_level_conformance: 0,
        kf_max_pyr_height: 0,
        sb_qp_sweep: 0,
    };
    let mut extra_cfg: av1_extracfg = default_extra_cfg;
    set_encoder_config(&raw mut oxcf, cfg, &raw mut extra_cfg);
    return oxcf;
}
unsafe extern "C-unwind" fn encoder_set_config(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut cfg: *const aom_codec_enc_cfg_t,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    let mut force_key: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*cfg).g_w != (*ctx).cfg.g_w || (*cfg).g_h != (*ctx).cfg.g_h {
        if (*cfg).g_lag_in_frames > 1 as core::ffi::c_uint
            || (*cfg).g_pass as core::ffi::c_uint
                != AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        {
            (*ctx).base.err_detail = b"Cannot change width or height after initialization\0"
                as *const u8 as *const core::ffi::c_char;
            return AOM_CODEC_INVALID_PARAM;
        }
        let allow_ref_scaled_upscale: bool = (*cfg).g_forced_max_frame_width != 0
            && (*cfg).g_forced_max_frame_height != 0
            && (*cfg).g_pass as core::ffi::c_uint
                == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            && (*cfg).g_lag_in_frames == 0 as core::ffi::c_uint;
        if (*(*(*ctx).ppi).cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
            && (*(*(*ctx).ppi).cpi).last_coded_width != 0
            && (*(*(*ctx).ppi).cpi).last_coded_height != 0
            && (valid_ref_frame_size(
                (*(*(*ctx).ppi).cpi).last_coded_width,
                (*(*(*ctx).ppi).cpi).last_coded_height,
                (*cfg).g_w as core::ffi::c_int,
                (*cfg).g_h as core::ffi::c_int,
            ) == 0
                || !allow_ref_scaled_upscale
                    && ((*cfg).g_w as core::ffi::c_int
                        > (*(*(*ctx).ppi).cpi).last_coded_width
                        || (*cfg).g_h as core::ffi::c_int
                            > (*(*(*ctx).ppi).cpi).last_coded_height))
        {
            ::core::ptr::write_volatile(
                &raw mut force_key as *mut core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
    }
    if (*ctx).monochrome_on_init as core::ffi::c_int != 0
        && (*cfg).monochrome == 0 as core::ffi::c_uint
    {
        (*ctx).base.err_detail = b"Cannot change to monochrome = 0 after init with monochrome\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_lag_in_frames > (*ctx).cfg.g_lag_in_frames {
        (*ctx).base.err_detail = b"Cannot increase lag_in_frames\0" as *const u8
            as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    if (*cfg).g_lag_in_frames != (*ctx).cfg.g_lag_in_frames
        && (*ctx).num_lap_buffers > 0 as core::ffi::c_int
    {
        (*ctx).base.err_detail = b"Cannot change lag_in_frames if LAP is enabled\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    res = validate_config(ctx, cfg, &raw mut (*ctx).extra_cfg);
    if res as core::ffi::c_uint == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).cfg = *cfg;
        set_encoder_config(&raw mut (*ctx).oxcf, &raw mut (*ctx).cfg, &raw mut (*ctx).extra_cfg);
        ::core::ptr::write_volatile(
            &raw mut force_key as *mut core::ffi::c_int,
            ::core::ptr::read_volatile::<
                core::ffi::c_int,
            >(&force_key as *const core::ffi::c_int)
                | ((*(*ctx).ppi).seq_params.profile as core::ffi::c_int
                    != (*ctx).oxcf.profile as core::ffi::c_int) as core::ffi::c_int,
        );
        let mut is_sb_size_changed: bool = false_0 != 0;
        av1_change_config_seq((*ctx).ppi, &raw mut (*ctx).oxcf, &raw mut is_sb_size_changed);
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*(*ctx).ppi).num_fp_contexts {
            let cpi: *mut AV1_COMP = (*(*ctx).ppi).parallel_cpi[i as usize];
            let enable_setjmp_error: *mut aom_internal_error_info = (*cpi).common.error;
            loop {
match crate::jump::scope((((*enable_setjmp_error).jmp).as_mut_ptr()).cast(), || {

            (*enable_setjmp_error).setjmp = 1 as core::ffi::c_int;
            av1_change_config(cpi, &raw mut (*ctx).oxcf, is_sb_size_changed);
            let enable_setjmp_error_0: *mut aom_internal_error_info = (*cpi)
                .common
                .error;
            (*enable_setjmp_error_0).setjmp = 0 as core::ffi::c_int;
            i += 1;
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
                (*enable_setjmp_error).setjmp = 0 as core::ffi::c_int;
                (*ctx).base.err_detail = if (*enable_setjmp_error).has_detail != 0 {
                    ((*enable_setjmp_error).detail).as_mut_ptr()
                } else {
                    0 as *mut core::ffi::c_char
                };
                return (*enable_setjmp_error).error_code;}
}
}}
        if !((*(*ctx).ppi).cpi_lap).is_null() {
            let cpi_0: *mut AV1_COMP = (*(*ctx).ppi).cpi_lap as *mut AV1_COMP;
            let enable_setjmp_error_1: *mut aom_internal_error_info = (*cpi_0)
                .common
                .error;
            loop {
match crate::jump::scope((((*enable_setjmp_error_1).jmp).as_mut_ptr()).cast(), || {

            (*enable_setjmp_error_1).setjmp = 1 as core::ffi::c_int;
            av1_change_config(cpi_0, &raw mut (*ctx).oxcf, is_sb_size_changed);
            let enable_setjmp_error_2: *mut aom_internal_error_info = (*cpi_0)
                .common
                .error;
            (*enable_setjmp_error_2).setjmp = 0 as core::ffi::c_int;
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
                (*enable_setjmp_error_1).setjmp = 0 as core::ffi::c_int;
                (*ctx).base.err_detail = if (*enable_setjmp_error_1).has_detail != 0 {
                    ((*enable_setjmp_error_1).detail).as_mut_ptr()
                } else {
                    0 as *mut core::ffi::c_char
                };
                return (*enable_setjmp_error_1).error_code;}
}
}}
    }
    if force_key != 0 {
        (*ctx).next_frame_flags |= AOM_EFLAG_FORCE_KF as aom_enc_frame_flags_t;
    }
    return res;
}
unsafe extern "C-unwind" fn encoder_get_global_headers(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> *mut aom_fixed_buf_t {
    return av1_get_global_headers((*ctx).ppi);
}
unsafe extern "C-unwind" fn ctrl_get_quantizer(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = av1_get_quantizer((*(*ctx).ppi).cpi);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_quantizer64(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = av1_qindex_to_quantizer(av1_get_quantizer((*(*ctx).ppi).cpi));
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_loopfilter_level(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = (*(*(*ctx).ppi).cpi).common.lf.filter_level[0 as core::ffi::c_int as usize];
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_baseline_gf_interval(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = (*(*ctx).ppi).p_rc.baseline_gf_interval;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn update_encoder_cfg(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> aom_codec_err_t {
    if (*(*(*ctx).ppi).cpi).svc.number_spatial_layers > 1 as core::ffi::c_int {
        (*ctx).extra_cfg.noise_sensitivity = 0 as core::ffi::c_uint;
    }
    set_encoder_config(&raw mut (*ctx).oxcf, &raw mut (*ctx).cfg, &raw mut (*ctx).extra_cfg);
    av1_check_fpmt_config((*ctx).ppi, &raw mut (*ctx).oxcf);
    let mut is_sb_size_changed: bool = false_0 != 0;
    av1_change_config_seq((*ctx).ppi, &raw mut (*ctx).oxcf, &raw mut is_sb_size_changed);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*(*ctx).ppi).num_fp_contexts {
        let cpi: *mut AV1_COMP = (*(*ctx).ppi).parallel_cpi[i as usize];
        let enable_setjmp_error: *mut aom_internal_error_info = (*cpi).common.error;
        loop {
match crate::jump::scope((((*enable_setjmp_error).jmp).as_mut_ptr()).cast(), || {

        (*enable_setjmp_error).setjmp = 1 as core::ffi::c_int;
        av1_change_config(cpi, &raw mut (*ctx).oxcf, is_sb_size_changed);
        let enable_setjmp_error_0: *mut aom_internal_error_info = (*cpi).common.error;
        (*enable_setjmp_error_0).setjmp = 0 as core::ffi::c_int;
        i += 1;
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
            (*enable_setjmp_error).setjmp = 0 as core::ffi::c_int;
            (*ctx).base.err_detail = if (*enable_setjmp_error).has_detail != 0 {
                ((*enable_setjmp_error).detail).as_mut_ptr()
            } else {
                0 as *mut core::ffi::c_char
            };
            return (*enable_setjmp_error).error_code;}
}
}}
    if !((*(*ctx).ppi).cpi_lap).is_null() {
        let cpi_lap: *mut AV1_COMP = (*(*ctx).ppi).cpi_lap as *mut AV1_COMP;
        let enable_setjmp_error_1: *mut aom_internal_error_info = (*cpi_lap)
            .common
            .error;
        loop {
match crate::jump::scope((((*enable_setjmp_error_1).jmp).as_mut_ptr()).cast(), || {

        (*enable_setjmp_error_1).setjmp = 1 as core::ffi::c_int;
        av1_change_config(cpi_lap, &raw mut (*ctx).oxcf, is_sb_size_changed);
        let enable_setjmp_error_2: *mut aom_internal_error_info = (*cpi_lap)
            .common
            .error;
        (*enable_setjmp_error_2).setjmp = 0 as core::ffi::c_int;
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
            (*enable_setjmp_error_1).setjmp = 0 as core::ffi::c_int;
            (*ctx).base.err_detail = if (*enable_setjmp_error_1).has_detail != 0 {
                ((*enable_setjmp_error_1).detail).as_mut_ptr()
            } else {
                0 as *mut core::ffi::c_char
            };
            return (*enable_setjmp_error_1).error_code;}
}
}}
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn update_extra_cfg(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut extra_cfg: *const av1_extracfg,
) -> aom_codec_err_t {
    let res: aom_codec_err_t = validate_config(ctx, &raw mut (*ctx).cfg, extra_cfg)
        as aom_codec_err_t;
    if res as core::ffi::c_uint == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).extra_cfg = *extra_cfg;
        return update_encoder_cfg(ctx);
    }
    return res;
}
unsafe extern "C-unwind" fn ctrl_get_gop_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let gop_info: *mut aom_gop_info_t = args.next_arg::<*mut aom_gop_info_t>();
    if gop_info.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    let gf_group: *const GF_GROUP = &raw mut (*(*ctx).ppi).gf_group;
    (*gop_info).gop_size = (*gf_group).size;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*gf_group).size {
        (*gop_info).coding_index[i as usize] = i;
        (*gop_info).display_index[i as usize] = (*gf_group).display_idx[i as usize];
        (*gop_info).layer_depth[i as usize] = (*gf_group).layer_depth[i as usize];
        i += 1;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_cpuused(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.cpu_used = args.next_arg::<aom_codec_control_type_AOME_SET_CPUUSED>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_auto_alt_ref(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_auto_alt_ref = args
        .next_arg::<aom_codec_control_type_AOME_SET_ENABLEAUTOALTREF>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_auto_bwd_ref(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_auto_bwd_ref = args
        .next_arg::<aom_codec_control_type_AOME_SET_ENABLEAUTOBWDREF>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_noise_sensitivity(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.noise_sensitivity = args
        .next_arg::<aom_codec_control_type_AV1E_SET_NOISE_SENSITIVITY>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_sharpness(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.sharpness = args.next_arg::<aom_codec_control_type_AOME_SET_SHARPNESS>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_adaptive_sharpness(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_adaptive_sharpness = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_ADAPTIVE_SHARPNESS>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_static_thresh(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.static_thresh = args
        .next_arg::<aom_codec_control_type_AOME_SET_STATIC_THRESHOLD>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_row_mt(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut row_mt: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ROW_MT>();
    if row_mt == (*ctx).extra_cfg.row_mt {
        return AOM_CODEC_OK;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.row_mt = row_mt;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_tile_columns(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    if (*ctx).extra_cfg.auto_tiles != 0 {
        (*ctx).base.err_detail = b"AUTO_TILES is set so AV1E_SET_TILE_COLUMNS should not be called.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut tile_columns: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_TILE_COLUMNS>();
    if tile_columns == extra_cfg.tile_columns {
        return AOM_CODEC_OK;
    }
    extra_cfg.tile_columns = tile_columns;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_tile_rows(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    if (*ctx).extra_cfg.auto_tiles != 0 {
        (*ctx).base.err_detail = b"AUTO_TILES is set so AV1E_SET_TILE_ROWS should not be called.\0"
            as *const u8 as *const core::ffi::c_char;
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut tile_rows: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_TILE_ROWS>();
    if tile_rows == extra_cfg.tile_rows {
        return AOM_CODEC_OK;
    }
    extra_cfg.tile_rows = tile_rows;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_tpl_model(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let tpl_model_arg: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_TPL_MODEL>();
    extra_cfg.enable_tpl_model = tpl_model_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_keyframe_filtering(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_keyframe_filtering = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_KEYFRAME_FILTERING>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_validate_hbd_input(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.validate_hbd_input = args
        .next_arg::<aom_codec_control_type_AOME_SET_VALIDATE_HBD_INPUT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_arnr_max_frames(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.arnr_max_frames = args
        .next_arg::<aom_codec_control_type_AOME_SET_ARNR_MAXFRAMES>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_arnr_strength(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.arnr_strength = args.next_arg::<aom_codec_control_type_AOME_SET_ARNR_STRENGTH>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn handle_tuning(mut extra_cfg: *mut av1_extracfg) -> aom_codec_err_t {
    if (*extra_cfg).tuning as core::ffi::c_uint
        == AOM_TUNE_IQ as core::ffi::c_int as core::ffi::c_uint
        || (*extra_cfg).tuning as core::ffi::c_uint
            == AOM_TUNE_SSIMULACRA2 as core::ffi::c_int as core::ffi::c_uint
    {
        (*extra_cfg).enable_qm = 1 as core::ffi::c_uint;
        (*extra_cfg).qm_min = QM_FIRST_IQ_SSIMULACRA2 as core::ffi::c_uint;
        (*extra_cfg).qm_max = QM_LAST_IQ_SSIMULACRA2 as core::ffi::c_uint;
        (*extra_cfg).sharpness = 7 as core::ffi::c_uint;
        (*extra_cfg).dist_metric = AOM_DIST_METRIC_QM_PSNR;
        (*extra_cfg).enable_cdef = CDEF_ADAPTIVE as core::ffi::c_int
            as core::ffi::c_uint;
        (*extra_cfg).enable_chroma_deltaq = 1 as core::ffi::c_uint;
        (*extra_cfg).deltaq_mode = DELTA_Q_VARIANCE_BOOST as core::ffi::c_int
            as DELTAQ_MODE;
        (*extra_cfg).screen_detection_mode = AOM_SCREEN_DETECTION_ANTIALIASING_AWARE;
    }
    if (*extra_cfg).tuning as core::ffi::c_uint
        == AOM_TUNE_IQ as core::ffi::c_int as core::ffi::c_uint
    {
        (*extra_cfg).enable_adaptive_sharpness = 1 as core::ffi::c_uint;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_tuning(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.tuning = args.next_arg::<aom_codec_control_type_AOME_SET_TUNING>()
        as aom_tune_metric;
    let mut err: aom_codec_err_t = handle_tuning(&raw mut extra_cfg);
    if err as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return err;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_cq_level(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.cq_level = args.next_arg::<aom_codec_control_type_AOME_SET_CQ_LEVEL>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_rc_max_intra_bitrate_pct(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.rc_max_intra_bitrate_pct = args
        .next_arg::<aom_codec_control_type_AOME_SET_MAX_INTRA_BITRATE_PCT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_rc_max_inter_bitrate_pct(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.rc_max_inter_bitrate_pct = args
        .next_arg::<aom_codec_control_type_AOME_SET_MAX_INTER_BITRATE_PCT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_rc_gf_cbr_boost_pct(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.gf_cbr_boost_pct = args
        .next_arg::<aom_codec_control_type_AV1E_SET_GF_CBR_BOOST_PCT>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_lossless(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.lossless = args.next_arg::<aom_codec_control_type_AV1E_SET_LOSSLESS>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_cdef(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_cdef = args.next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_CDEF>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_restoration(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let restoration_arg: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_RESTORATION>();
    extra_cfg.enable_restoration = restoration_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_force_video_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.force_video_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_FORCE_VIDEO_MODE>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_obmc(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let obmc_arg: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_OBMC>();
    extra_cfg.enable_obmc = obmc_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_disable_trellis_quant(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.disable_trellis_quant = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DISABLE_TRELLIS_QUANT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_qm(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_qm = args.next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_QM>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_qm_y(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.qm_y = args.next_arg::<aom_codec_control_type_AV1E_SET_QM_Y>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_qm_u(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.qm_u = args.next_arg::<aom_codec_control_type_AV1E_SET_QM_U>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_qm_v(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.qm_v = args.next_arg::<aom_codec_control_type_AV1E_SET_QM_V>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_qm_min(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.qm_min = args.next_arg::<aom_codec_control_type_AV1E_SET_QM_MIN>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_qm_max(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.qm_max = args.next_arg::<aom_codec_control_type_AV1E_SET_QM_MAX>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_num_tg(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.num_tg = args.next_arg::<aom_codec_control_type_AV1E_SET_NUM_TG>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_mtu(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.mtu_size = args.next_arg::<aom_codec_control_type_AV1E_SET_MTU>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_timing_info_type(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.timing_info_type = args
        .next_arg::<aom_codec_control_type_AV1E_SET_TIMING_INFO_TYPE>()
        as aom_timing_info_type_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_low_complexity_decode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_low_complexity_decode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_LOW_COMPLEXITY_DECODE>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_dual_filter(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_dual_filter = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DUAL_FILTER>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_chroma_deltaq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_chroma_deltaq = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_CHROMA_DELTAQ>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_rect_partitions(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_rect_partitions = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_RECT_PARTITIONS>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_ab_partitions(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_ab_partitions = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_AB_PARTITIONS>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_1to4_partitions(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_1to4_partitions = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_1TO4_PARTITIONS>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_min_partition_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.min_partition_size = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MIN_PARTITION_SIZE>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_max_partition_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.max_partition_size = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MAX_PARTITION_SIZE>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_intra_edge_filter(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_intra_edge_filter = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_INTRA_EDGE_FILTER>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_order_hint(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_order_hint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_ORDER_HINT>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_tx64(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_tx64 = args.next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_TX64>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_flip_idtx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_flip_idtx = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_FLIP_IDTX>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_rect_tx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_rect_tx = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_RECT_TX>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_dist_wtd_comp(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_dist_wtd_comp = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DIST_WTD_COMP>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_max_reference_frames(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.max_reference_frames = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MAX_REFERENCE_FRAMES>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_reduced_reference_set(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_reduced_reference_set = args
        .next_arg::<aom_codec_control_type_AV1E_SET_REDUCED_REFERENCE_SET>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_ref_frame_mvs(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_ref_frame_mvs = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_REF_FRAME_MVS>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_allow_ref_frame_mvs(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.allow_ref_frame_mvs = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ALLOW_REF_FRAME_MVS>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_masked_comp(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_masked_comp = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_MASKED_COMP>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_onesided_comp(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_onesided_comp = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_ONESIDED_COMP>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_interintra_comp(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_interintra_comp = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_INTERINTRA_COMP>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_smooth_interintra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_smooth_interintra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_SMOOTH_INTERINTRA>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_diff_wtd_comp(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_diff_wtd_comp = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DIFF_WTD_COMP>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_interinter_wedge(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_interinter_wedge = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_INTERINTER_WEDGE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_interintra_wedge(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_interintra_wedge = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_INTERINTRA_WEDGE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_global_motion(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let global_motion_arg: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_GLOBAL_MOTION>();
    extra_cfg.enable_global_motion = global_motion_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_warped_motion(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let warped_motion_arg: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_WARPED_MOTION>();
    extra_cfg.enable_warped_motion = warped_motion_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_allow_warped_motion(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.allow_warped_motion = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ALLOW_WARPED_MOTION>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_filter_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_filter_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_FILTER_INTRA>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_smooth_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_smooth_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_SMOOTH_INTRA>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_directional_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_directional_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DIRECTIONAL_INTRA>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_diagonal_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_diagonal_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DIAGONAL_INTRA>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_paeth_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_paeth_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_PAETH_INTRA>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_cfl_intra(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_cfl_intra = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_CFL_INTRA>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_superres(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_superres = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_SUPERRES>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_overlay(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_overlay = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_OVERLAY>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_palette(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_palette = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_PALETTE>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_intrabc(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_intrabc = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_INTRABC>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_angle_delta(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_angle_delta = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_ANGLE_DELTA>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_error_resilient_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.error_resilient_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ERROR_RESILIENT_MODE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_s_frame_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.s_frame_mode = args.next_arg::<aom_codec_control_type_AV1E_SET_S_FRAME_MODE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_frame_parallel_decoding_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.frame_parallel_decoding_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_FRAME_PARALLEL_DECODING>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_single_tile_decoding(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.single_tile_decoding = args
        .next_arg::<aom_codec_control_type_AV1E_SET_SINGLE_TILE_DECODING>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_aq_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.aq_mode = args.next_arg::<aom_codec_control_type_AV1E_SET_AQ_MODE>() as AQ_MODE;
    if (*(*(*ctx).ppi).cpi).rc.use_external_qp_one_pass != 0 {
        extra_cfg.aq_mode = 0 as AQ_MODE;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_reduced_tx_type_set(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.reduced_tx_type_set = args
        .next_arg::<aom_codec_control_type_AV1E_SET_REDUCED_TX_TYPE_SET>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_intra_dct_only(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.use_intra_dct_only = args
        .next_arg::<aom_codec_control_type_AV1E_SET_INTRA_DCT_ONLY>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_inter_dct_only(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.use_inter_dct_only = args
        .next_arg::<aom_codec_control_type_AV1E_SET_INTER_DCT_ONLY>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_intra_default_tx_only(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.use_intra_default_tx_only = args
        .next_arg::<aom_codec_control_type_AV1E_SET_INTRA_DEFAULT_TX_ONLY>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_tx_size_search(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_tx_size_search = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_TX_SIZE_SEARCH>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_quant_b_adapt(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.quant_b_adapt = args.next_arg::<aom_codec_control_type_AV1E_SET_QUANT_B_ADAPT>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_vbr_corpus_complexity_lap(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.vbr_corpus_complexity_lap = args
        .next_arg::<aom_codec_control_type_AV1E_SET_VBR_CORPUS_COMPLEXITY_LAP>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_coeff_cost_upd_freq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.coeff_cost_upd_freq = args
        .next_arg::<aom_codec_control_type_AV1E_SET_COEFF_COST_UPD_FREQ>()
        as COST_UPDATE_TYPE;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_mode_cost_upd_freq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.mode_cost_upd_freq = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MODE_COST_UPD_FREQ>() as COST_UPDATE_TYPE;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_mv_cost_upd_freq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.mv_cost_upd_freq = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MV_COST_UPD_FREQ>() as COST_UPDATE_TYPE;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_dv_cost_upd_freq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.dv_cost_upd_freq = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DV_COST_UPD_FREQ>() as COST_UPDATE_TYPE;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_vmaf_model_path(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut str: *const core::ffi::c_char = args
        .next_arg::<aom_codec_control_type_AV1E_SET_VMAF_MODEL_PATH>();
    let ret: aom_codec_err_t = allocate_and_set_string(
        str,
        default_extra_cfg.vmaf_model_path,
        &raw mut extra_cfg.vmaf_model_path,
        ((*(*ctx).ppi).error.detail).as_mut_ptr(),
    ) as aom_codec_err_t;
    if ret as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return ret;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_partition_info_path(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut str: *const core::ffi::c_char = args
        .next_arg::<aom_codec_control_type_AV1E_SET_PARTITION_INFO_PATH>();
    let ret: aom_codec_err_t = allocate_and_set_string(
        str,
        default_extra_cfg.partition_info_path,
        &raw mut extra_cfg.partition_info_path,
        ((*(*ctx).ppi).error.detail).as_mut_ptr(),
    ) as aom_codec_err_t;
    if ret as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return ret;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_enable_rate_guide_deltaq(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_rate_guide_deltaq = args
        .next_arg::<aom_codec_control_type_AV1E_ENABLE_RATE_GUIDE_DELTAQ>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_rate_distribution_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut str: *const core::ffi::c_char = args
        .next_arg::<aom_codec_control_type_AV1E_SET_RATE_DISTRIBUTION_INFO>();
    let ret: aom_codec_err_t = allocate_and_set_string(
        str,
        default_extra_cfg.rate_distribution_info,
        &raw mut extra_cfg.rate_distribution_info,
        ((*(*ctx).ppi).error.detail).as_mut_ptr(),
    ) as aom_codec_err_t;
    if ret as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return ret;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_film_grain_test_vector(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.film_grain_test_vector = args
        .next_arg::<aom_codec_control_type_AV1E_SET_FILM_GRAIN_TEST_VECTOR>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_film_grain_table(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut str: *const core::ffi::c_char = args
        .next_arg::<aom_codec_control_type_AV1E_SET_FILM_GRAIN_TABLE>();
    if str.is_null() {
        extra_cfg.film_grain_table_filename = str;
    } else {
        let ret: aom_codec_err_t = allocate_and_set_string(
            str,
            default_extra_cfg.film_grain_table_filename,
            &raw mut extra_cfg.film_grain_table_filename,
            ((*(*ctx).ppi).error.detail).as_mut_ptr(),
        ) as aom_codec_err_t;
        if ret as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return ret;
        }
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_denoise_noise_level(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.noise_level = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DENOISE_NOISE_LEVEL>()
        as core::ffi::c_float / 10.0f32;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_denoise_block_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.noise_block_size = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DENOISE_BLOCK_SIZE>() as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_enable_dnl_denoising(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.enable_dnl_denoising = args
        .next_arg::<aom_codec_control_type_AV1E_SET_ENABLE_DNL_DENOISING>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_deltaq_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let deltaq_arg: DELTAQ_MODE = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DELTAQ_MODE>() as DELTAQ_MODE;
    extra_cfg.deltaq_mode = deltaq_arg;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_deltaq_strength(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.deltaq_strength = args
        .next_arg::<aom_codec_control_type_AV1E_SET_DELTAQ_STRENGTH>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_deltalf_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.deltalf_mode = args.next_arg::<aom_codec_control_type_AV1E_SET_DELTALF_MODE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_min_gf_interval(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.min_gf_interval = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MIN_GF_INTERVAL>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_max_gf_interval(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.max_gf_interval = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MAX_GF_INTERVAL>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_gf_min_pyr_height(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.gf_min_pyr_height = args
        .next_arg::<aom_codec_control_type_AV1E_SET_GF_MIN_PYRAMID_HEIGHT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_gf_max_pyr_height(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.gf_max_pyr_height = args
        .next_arg::<aom_codec_control_type_AV1E_SET_GF_MAX_PYRAMID_HEIGHT>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_frame_periodic_boost(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.frame_periodic_boost = args
        .next_arg::<aom_codec_control_type_AV1E_SET_FRAME_PERIODIC_BOOST>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_enable_motion_vector_unit_test(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.motion_vector_unit_test = args
        .next_arg::<aom_codec_control_type_AV1E_ENABLE_MOTION_VECTOR_UNIT_TEST>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_enable_fpmt_unit_test(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    return AOM_CODEC_INCAPABLE;
}
unsafe extern "C-unwind" fn ctrl_enable_ext_tile_debug(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.ext_tile_debug = args
        .next_arg::<aom_codec_control_type_AV1E_ENABLE_EXT_TILE_DEBUG>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_target_seq_level_idx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let val: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_TARGET_SEQ_LEVEL_IDX>();
    let level: core::ffi::c_int = val % 100 as core::ffi::c_int;
    let operating_point_idx: core::ffi::c_int = val / 100 as core::ffi::c_int;
    if operating_point_idx < 0 as core::ffi::c_int
        || operating_point_idx >= MAX_NUM_OPERATING_POINTS
    {
        let err_string: *mut core::ffi::c_char = ((*(*ctx).ppi).error.detail)
            .as_mut_ptr();
        snprintf(
            err_string,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Invalid operating point index: %d\0" as *const u8
                as *const core::ffi::c_char,
            operating_point_idx,
        );
        (*ctx).base.err_detail = err_string;
        return AOM_CODEC_INVALID_PARAM;
    }
    extra_cfg.target_seq_level_idx[operating_point_idx as usize] = level as AV1_LEVEL;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_tier_mask(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.tier_mask = args.next_arg::<aom_codec_control_type_AV1E_SET_TIER_MASK>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_min_cr(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.min_cr = args.next_arg::<aom_codec_control_type_AV1E_SET_MIN_CR>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_enable_sb_multipass_unit_test(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.sb_multipass_unit_test = args
        .next_arg::<aom_codec_control_type_AV1E_ENABLE_SB_MULTIPASS_UNIT_TEST>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_enable_sb_qp_sweep(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.sb_qp_sweep = args.next_arg::<aom_codec_control_type_AV1E_ENABLE_SB_QP_SWEEP>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_external_partition(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let cpi: *mut AV1_COMP = (*(*ctx).ppi).cpi as *mut AV1_COMP;
    let mut funcs: aom_ext_part_funcs_t = *args
        .next_arg::<aom_codec_control_type_AV1E_SET_EXTERNAL_PARTITION>();
    let mut config: aom_ext_part_config_t = aom_ext_part_config {
        superblock_size: 0,
    };
    config.superblock_size = (*(*cpi).common.seq_params).sb_size as core::ffi::c_int;
    let status: aom_codec_err_t = av1_ext_part_create(
        funcs,
        config,
        &raw mut (*cpi).ext_part_controller,
    ) as aom_codec_err_t;
    return status;
}
unsafe extern "C-unwind" fn ctrl_set_loopfilter_control(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.loopfilter_control = args
        .next_arg::<aom_codec_control_type_AV1E_SET_LOOPFILTER_CONTROL>()
        as LOOPFILTER_CONTROL;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_skip_postproc_filtering(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    if (*ctx).cfg.g_usage != AOM_USAGE_ALL_INTRA {
        return AOM_CODEC_INCAPABLE;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.skip_postproc_filtering = args
        .next_arg::<aom_codec_control_type_AV1E_SET_SKIP_POSTPROC_FILTERING>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_mode_ref_delta_enabled(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.mode_ref_delta_enabled = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MODE_REF_DELTA_ENABLED>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_rtc_external_rc(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    (*(*(*ctx).ppi).cpi).rc.rtc_external_ratectrl = args
        .next_arg::<aom_codec_control_type_AV1E_SET_RTC_EXTERNAL_RC>() as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_quantizer_one_pass(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let qp: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_QUANTIZER_ONE_PASS>();
    if qp < 0 as core::ffi::c_int || qp > 63 as core::ffi::c_int {
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut cfg: *mut aom_codec_enc_cfg_t = &raw mut (*ctx).cfg;
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    (*cfg).rc_max_quantizer = qp as core::ffi::c_uint;
    (*cfg).rc_min_quantizer = (*cfg).rc_max_quantizer;
    extra_cfg.aq_mode = 0 as AQ_MODE;
    (*(*(*ctx).ppi).cpi).rc.use_external_qp_one_pass = 1 as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_bitrate_one_pass_cbr(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    let mut oxcf: *mut AV1EncoderConfig = &raw mut (*cpi).oxcf;
    if is_one_pass_rt_params(cpi) == 0
        || (*oxcf).rc_cfg.mode as core::ffi::c_uint
            != AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        || (*(*cpi).ppi).use_svc != 0 || (*ppi).num_fp_contexts != 1 as core::ffi::c_int
        || !((*ppi).cpi_lap).is_null()
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    let new_bitrate: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_BITRATE_ONE_PASS_CBR>()
        as core::ffi::c_int;
    (*ctx).cfg.rc_target_bitrate = new_bitrate as core::ffi::c_uint;
    (*oxcf).rc_cfg.target_bandwidth = (new_bitrate * 1000 as core::ffi::c_int)
        as int64_t;
    set_primary_rc_buffer_sizes(oxcf, ppi);
    av1_new_framerate(cpi, (*cpi).framerate);
    check_reset_rc_flag(cpi);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_max_consec_frame_drop_cbr(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    let max_consec_drop: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MAX_CONSEC_FRAME_DROP_CBR>();
    if max_consec_drop < 0 as core::ffi::c_int {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*cpi).rc.max_consec_drop = max_consec_drop;
    (*cpi).rc.drop_count_consec = 0 as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_max_consec_frame_drop_ms_cbr(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    let max_consec_drop_ms: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MAX_CONSEC_FRAME_DROP_MS_CBR>();
    if max_consec_drop_ms < 0 as core::ffi::c_int {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ctx).oxcf.rc_cfg.max_consec_drop_ms = max_consec_drop_ms;
    (*cpi).rc.drop_count_consec = 0 as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_svc_frame_drop_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    (*cpi).svc.framedrop_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_SVC_FRAME_DROP_MODE>()
        as AOM_SVC_FRAME_DROP_MODE;
    if (*cpi).svc.framedrop_mode as core::ffi::c_uint
        != AOM_LAYER_DROP as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).svc.framedrop_mode as core::ffi::c_uint
            != AOM_FULL_SUPERFRAME_DROP as core::ffi::c_int as core::ffi::c_uint
    {
        return AOM_CODEC_INVALID_PARAM
    } else {
        return AOM_CODEC_OK
    };
}
unsafe extern "C-unwind" fn ctrl_set_auto_tiles(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut auto_tiles: core::ffi::c_uint = args
        .next_arg::<aom_codec_control_type_AV1E_SET_AUTO_TILES>();
    if auto_tiles == (*ctx).extra_cfg.auto_tiles {
        return AOM_CODEC_OK;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.auto_tiles = auto_tiles;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_postencode_drop_rtc(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    let mut enable_postencode_drop: core::ffi::c_int = args
        .next_arg::<aom_codec_control_type_AV1E_SET_POSTENCODE_DROP_RTC>();
    if enable_postencode_drop > 1 as core::ffi::c_int
        || enable_postencode_drop < 0 as core::ffi::c_int
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*cpi).rc.postencode_drop = enable_postencode_drop;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_screen_content_detection_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let screen_detection_mode: aom_screen_detection_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_SCREEN_CONTENT_DETECTION_MODE>()
        as aom_screen_detection_mode;
    extra_cfg.screen_detection_mode = screen_detection_mode;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn create_stats_buffer(
    mut frame_stats_buffer: *mut *mut FIRSTPASS_STATS,
    mut stats_buf_context: *mut STATS_BUFFER_CTX,
    mut num_lap_buffers: core::ffi::c_int,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    let mut size: core::ffi::c_int = get_stats_buf_size(
        num_lap_buffers,
        MAX_LAG_BUFFERS,
    );
    *frame_stats_buffer = aom_calloc(
        size as size_t,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    ) as *mut FIRSTPASS_STATS;
    if (*frame_stats_buffer).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    (*stats_buf_context).stats_in_start = *frame_stats_buffer;
    (*stats_buf_context).stats_in_end = (*stats_buf_context).stats_in_start;
    (*stats_buf_context).stats_in_buf_end = ((*stats_buf_context).stats_in_start)
        .offset(size as isize);
    (*stats_buf_context).total_left_stats = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    ) as *mut FIRSTPASS_STATS;
    if ((*stats_buf_context).total_left_stats).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    av1_twopass_zero_stats((*stats_buf_context).total_left_stats);
    (*stats_buf_context).total_stats = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<FIRSTPASS_STATS>() as size_t,
    ) as *mut FIRSTPASS_STATS;
    if ((*stats_buf_context).total_stats).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    av1_twopass_zero_stats((*stats_buf_context).total_stats);
    return res;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_create_context_and_bufferpool(
    mut ppi: *mut AV1_PRIMARY,
    mut p_cpi: *mut *mut AV1_COMP,
    mut p_buffer_pool: *mut *mut BufferPool,
    mut oxcf: *const AV1EncoderConfig,
    mut stage: COMPRESSOR_STAGE,
    mut lap_lag_in_frames: core::ffi::c_int,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    let mut buffer_pool: *mut BufferPool = *p_buffer_pool;
    if buffer_pool.is_null() {
        buffer_pool = aom_calloc(
            1 as size_t,
            ::core::mem::size_of::<BufferPool>() as size_t,
        ) as *mut BufferPool;
        if buffer_pool.is_null() {
            return AOM_CODEC_MEM_ERROR;
        }
        (*buffer_pool).num_frame_bufs = (if (*oxcf).mode as core::ffi::c_int
            == ALLINTRA as core::ffi::c_int
        {
            FRAME_BUFFERS_ALLINTRA
        } else {
            REF_FRAMES as core::ffi::c_int + 1 as core::ffi::c_int
                + INTER_REFS_PER_FRAME as core::ffi::c_int
        }) as uint8_t;
        (*buffer_pool).frame_bufs = aom_calloc(
            (*buffer_pool).num_frame_bufs as size_t,
            ::core::mem::size_of::<RefCntBuffer>() as size_t,
        ) as *mut RefCntBuffer;
        if ((*buffer_pool).frame_bufs).is_null() {
            (*buffer_pool).num_frame_bufs = 0 as uint8_t;
            aom_free(buffer_pool as *mut core::ffi::c_void);
            return AOM_CODEC_MEM_ERROR;
        }
        if pthread_mutex_init(
            &raw mut (*buffer_pool).pool_mutex,
            0 as *const pthread_mutexattr_t,
        ) != 0
        {
            aom_free((*buffer_pool).frame_bufs as *mut core::ffi::c_void);
            (*buffer_pool).frame_bufs = 0 as *mut RefCntBuffer;
            (*buffer_pool).num_frame_bufs = 0 as uint8_t;
            aom_free(buffer_pool as *mut core::ffi::c_void);
            return AOM_CODEC_MEM_ERROR;
        }
        *p_buffer_pool = buffer_pool;
    }
    *p_cpi = av1_create_compressor(
        ppi as *mut AV1_PRIMARY,
        oxcf,
        buffer_pool as *mut BufferPool,
        stage,
        lap_lag_in_frames,
    ) as *mut AV1_COMP;
    if (*p_cpi).is_null() {
        res = AOM_CODEC_MEM_ERROR;
    }
    return res;
}
unsafe extern "C-unwind" fn ctrl_set_fp_mt(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.fp_mt = args.next_arg::<aom_codec_control_type_AV1E_SET_FP_MT>()
        as core::ffi::c_uint;
    let result: aom_codec_err_t = update_extra_cfg(ctx, &raw mut extra_cfg)
        as aom_codec_err_t;
    let mut num_fp_contexts: core::ffi::c_int = 1 as core::ffi::c_int;
    if (*(*ctx).ppi).num_fp_contexts == 1 as core::ffi::c_int {
        num_fp_contexts = av1_compute_num_fp_contexts(
            (*ctx).ppi,
            &raw mut (**((*(*ctx).ppi).parallel_cpi)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .oxcf,
        );
        if num_fp_contexts > 1 as core::ffi::c_int {
            let mut i: core::ffi::c_int = 0;
            i = 1 as core::ffi::c_int;
            while i < num_fp_contexts {
                let mut res: core::ffi::c_int = av1_create_context_and_bufferpool(
                    (*ctx).ppi,
                    &raw mut *((*(*ctx).ppi).parallel_cpi).as_mut_ptr().offset(i as isize),
                    &raw mut (*ctx).buffer_pool,
                    &raw mut (*ctx).oxcf,
                    ENCODE_STAGE as core::ffi::c_int as COMPRESSOR_STAGE,
                    -(1 as core::ffi::c_int),
                ) as core::ffi::c_int;
                if res != AOM_CODEC_OK as core::ffi::c_int {
                    return res as aom_codec_err_t;
                }
                (*(*(*ctx).ppi).parallel_cpi[i as usize]).twopass_frame.stats_in = (*(*(*ctx)
                    .ppi)
                    .twopass
                    .stats_buf_ctx)
                    .stats_in_start;
                i += 1;
            }
        }
    }
    (*(*ctx).ppi).num_fp_contexts = num_fp_contexts;
    return result;
}
unsafe extern "C-unwind" fn ctrl_set_auto_intra_tools_off(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.auto_intra_tools_off = args
        .next_arg::<aom_codec_control_type_AV1E_SET_AUTO_INTRA_TOOLS_OFF>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn encoder_init(mut ctx: *mut aom_codec_ctx_t) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    if ((*ctx).priv_0).is_null() {
        let priv_0: *mut aom_codec_alg_priv_t = aom_calloc(
            1 as size_t,
            ::core::mem::size_of::<aom_codec_alg_priv_t>() as size_t,
        ) as *mut aom_codec_alg_priv_t;
        if priv_0.is_null() {
            return AOM_CODEC_MEM_ERROR;
        }
        (*ctx).priv_0 = priv_0 as *mut aom_codec_priv_t;
        (*(*ctx).priv_0).init_flags = (*ctx).init_flags;
        (*priv_0).cfg = *(*ctx).config.enc as aom_codec_enc_cfg_t;
        (*ctx).config.enc = &raw mut (*priv_0).cfg;
        (*priv_0).extra_cfg = default_extra_cfg;
        if (*priv_0).cfg.g_usage == AOM_USAGE_ALL_INTRA {
            (*priv_0).extra_cfg.enable_cdef = 0 as core::ffi::c_uint;
            (*priv_0).extra_cfg.screen_detection_mode = AOM_SCREEN_DETECTION_ANTIALIASING_AWARE;
            (*priv_0).extra_cfg.qm_min = DEFAULT_QM_FIRST_ALLINTRA as core::ffi::c_uint;
            (*priv_0).extra_cfg.qm_max = DEFAULT_QM_LAST_ALLINTRA as core::ffi::c_uint;
        }
        av1_initialize_enc((*priv_0).cfg.g_usage, (*priv_0).cfg.rc_end_usage);
        res = validate_config(priv_0, &raw mut (*priv_0).cfg, &raw mut (*priv_0).extra_cfg);
        if res as core::ffi::c_uint
            == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            let mut num_lap_buffers: *mut core::ffi::c_int = &raw mut (*priv_0)
                .num_lap_buffers;
            let mut lap_lag_in_frames: core::ffi::c_int = 0 as core::ffi::c_int;
            *num_lap_buffers = 0 as core::ffi::c_int;
            (*priv_0).timestamp_ratio.den = (*priv_0).cfg.g_timebase.den;
            (*priv_0).timestamp_ratio.num = (*priv_0).cfg.g_timebase.num as int64_t
                * TICKS_PER_SEC as int64_t;
            reduce_ratio(&raw mut (*priv_0).timestamp_ratio);
            set_encoder_config(
                &raw mut (*priv_0).oxcf,
                &raw mut (*priv_0).cfg,
                &raw mut (*priv_0).extra_cfg,
            );
            if (*priv_0).oxcf.pass as core::ffi::c_uint
                == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            {
                let lag_in_frames: core::ffi::c_int = (*priv_0).cfg.g_lag_in_frames
                    as core::ffi::c_int;
                *num_lap_buffers = if lag_in_frames
                    < (if (48 as core::ffi::c_int)
                        < (*priv_0).oxcf.kf_cfg.key_freq_max + 16 as core::ffi::c_int
                    {
                        48 as core::ffi::c_int
                    } else {
                        (*priv_0).oxcf.kf_cfg.key_freq_max + 16 as core::ffi::c_int
                    })
                {
                    lag_in_frames
                } else if (48 as core::ffi::c_int)
                    < (*priv_0).oxcf.kf_cfg.key_freq_max + 16 as core::ffi::c_int
                {
                    48 as core::ffi::c_int
                } else {
                    (*priv_0).oxcf.kf_cfg.key_freq_max + 16 as core::ffi::c_int
                };
                if (*priv_0).cfg.g_lag_in_frames as core::ffi::c_int - *num_lap_buffers
                    >= LAP_LAG_IN_FRAMES
                {
                    lap_lag_in_frames = LAP_LAG_IN_FRAMES;
                }
            }
            (*priv_0).oxcf.use_highbitdepth = if (*ctx).init_flags
                & AOM_CODEC_USE_HIGHBITDEPTH as aom_codec_flags_t != 0
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            } != 0;
            (*priv_0).monochrome_on_init = (*priv_0).cfg.monochrome != 0;
            (*priv_0).ppi = av1_create_primary_compressor(
                &raw mut (*priv_0).pkt_list.head,
                *num_lap_buffers,
                &raw mut (*priv_0).oxcf,
            ) as *mut AV1_PRIMARY;
            if ((*priv_0).ppi).is_null() {
                return AOM_CODEC_MEM_ERROR;
            }
            res = create_stats_buffer(
                &raw mut (*priv_0).frame_stats_buffer,
                &raw mut (*priv_0).stats_buf_context,
                *num_lap_buffers,
            );
            if res as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return res;
            }
            let mut size: core::ffi::c_int = get_stats_buf_size(
                *num_lap_buffers,
                MAX_LAG_BUFFERS,
            );
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < size {
                (*(*priv_0).ppi).twopass.frame_stats_arr[i as usize] = &raw mut *((*priv_0)
                    .frame_stats_buffer)
                    .offset(i as isize) as *mut FIRSTPASS_STATS;
                i += 1;
            }
            (*(*priv_0).ppi).twopass.stats_buf_ctx = &raw mut (*priv_0).stats_buf_context;
            res = av1_create_context_and_bufferpool(
                (*priv_0).ppi,
                &raw mut *((*(*priv_0).ppi).parallel_cpi)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                &raw mut (*priv_0).buffer_pool,
                &raw mut (*priv_0).oxcf,
                ENCODE_STAGE as core::ffi::c_int as COMPRESSOR_STAGE,
                -(1 as core::ffi::c_int),
            );
            if res as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                (*priv_0).base.err_detail = b"av1_create_context_and_bufferpool() failed\0"
                    as *const u8 as *const core::ffi::c_char;
                return res;
            }
            (*(*(*priv_0).ppi).parallel_cpi[0 as core::ffi::c_int as usize])
                .twopass_frame
                .stats_in = (*(*(*priv_0).ppi).twopass.stats_buf_ctx).stats_in_start;
            (*(*priv_0).ppi).cpi = (*(*priv_0).ppi)
                .parallel_cpi[0 as core::ffi::c_int as usize];
            if res as core::ffi::c_uint
                == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                && *num_lap_buffers != 0
            {
                res = av1_create_context_and_bufferpool(
                    (*priv_0).ppi,
                    &raw mut (*(*priv_0).ppi).cpi_lap,
                    &raw mut (*priv_0).buffer_pool_lap,
                    &raw mut (*priv_0).oxcf,
                    LAP_STAGE as core::ffi::c_int as COMPRESSOR_STAGE,
                    clamp(lap_lag_in_frames, 0 as core::ffi::c_int, MAX_LAG_BUFFERS),
                );
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_destroy_context_and_bufferpool(
    mut cpi: *mut AV1_COMP,
    mut p_buffer_pool: *mut *mut BufferPool,
) {
    av1_remove_compressor(cpi);
    if !(*p_buffer_pool).is_null() {
        av1_free_ref_frame_buffers(*p_buffer_pool);
        pthread_mutex_destroy(&raw mut (**p_buffer_pool).pool_mutex);
        aom_free(*p_buffer_pool as *mut core::ffi::c_void);
        *p_buffer_pool = 0 as *mut BufferPool;
    }
}
unsafe extern "C-unwind" fn destroy_stats_buffer(
    mut stats_buf_context: *mut STATS_BUFFER_CTX,
    mut frame_stats_buffer: *mut FIRSTPASS_STATS,
) {
    aom_free((*stats_buf_context).total_left_stats as *mut core::ffi::c_void);
    aom_free((*stats_buf_context).total_stats as *mut core::ffi::c_void);
    aom_free(frame_stats_buffer as *mut core::ffi::c_void);
}
unsafe extern "C-unwind" fn check_and_free_string(
    mut default_str: *const core::ffi::c_char,
    mut ptr: *mut *const core::ffi::c_char,
) {
    if *ptr == default_str {
        return;
    }
    aom_free(*ptr as *mut core::ffi::c_void);
    *ptr = 0 as *const core::ffi::c_char;
}
unsafe extern "C-unwind" fn destroy_extra_config(mut extra_cfg: *mut av1_extracfg) {
    check_and_free_string(
        default_extra_cfg.two_pass_output,
        &raw mut (*extra_cfg).two_pass_output,
    );
    check_and_free_string(
        default_extra_cfg.two_pass_output,
        &raw mut (*extra_cfg).second_pass_log,
    );
    check_and_free_string(
        default_extra_cfg.partition_info_path,
        &raw mut (*extra_cfg).partition_info_path,
    );
    check_and_free_string(
        default_extra_cfg.rate_distribution_info,
        &raw mut (*extra_cfg).rate_distribution_info,
    );
    check_and_free_string(
        default_extra_cfg.film_grain_table_filename,
        &raw mut (*extra_cfg).film_grain_table_filename,
    );
}
unsafe extern "C-unwind" fn encoder_destroy(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> aom_codec_err_t {
    free((*ctx).cx_data as *mut core::ffi::c_void);
    destroy_extra_config(&raw mut (*ctx).extra_cfg);
    if !((*ctx).ppi).is_null() {
        let mut ppi: *mut AV1_PRIMARY = (*ctx).ppi;
        if !((*ppi).cpi).is_null() {
            av1_extrc_delete(&raw mut (*(*ppi).cpi).ext_ratectrl);
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < MAX_PARALLEL_FRAMES - 1 as core::ffi::c_int {
            if !((*ppi).parallel_frames_data[i as usize].cx_data).is_null() {
                free(
                    (*ppi).parallel_frames_data[i as usize].cx_data
                        as *mut core::ffi::c_void,
                );
            }
            i += 1;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < MAX_PARALLEL_FRAMES {
            av1_destroy_context_and_bufferpool(
                (*ppi).parallel_cpi[i_0 as usize],
                &raw mut (*ctx).buffer_pool,
            );
            i_0 += 1;
        }
        (*ppi).cpi = 0 as *mut AV1_COMP;
        if !((*ppi).cpi_lap).is_null() {
            av1_destroy_context_and_bufferpool(
                (*ppi).cpi_lap as *mut AV1_COMP,
                &raw mut (*ctx).buffer_pool_lap,
            );
        }
        av1_remove_primary_compressor(ppi);
    }
    destroy_stats_buffer(&raw mut (*ctx).stats_buf_context, (*ctx).frame_stats_buffer);
    aom_free(ctx as *mut core::ffi::c_void);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn get_frame_pkt_flags(
    mut cpi: *const AV1_COMP,
    mut lib_flags: core::ffi::c_uint,
) -> aom_codec_frame_flags_t {
    let mut flags: aom_codec_frame_flags_t = (lib_flags as aom_codec_frame_flags_t)
        << 16 as core::ffi::c_int;
    if lib_flags & FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint != 0 {
        flags = (flags as core::ffi::c_uint | AOM_FRAME_IS_KEY)
            as aom_codec_frame_flags_t;
    }
    if lib_flags & FRAMEFLAGS_INTRAONLY as core::ffi::c_int as core::ffi::c_uint != 0 {
        flags = (flags as core::ffi::c_uint | AOM_FRAME_IS_INTRAONLY)
            as aom_codec_frame_flags_t;
    }
    if lib_flags & FRAMEFLAGS_SWITCH as core::ffi::c_int as core::ffi::c_uint != 0 {
        flags = (flags as core::ffi::c_uint | AOM_FRAME_IS_SWITCH)
            as aom_codec_frame_flags_t;
    }
    if lib_flags & FRAMEFLAGS_ERROR_RESILIENT as core::ffi::c_int as core::ffi::c_uint
        != 0
    {
        flags = (flags as core::ffi::c_uint | AOM_FRAME_IS_ERROR_RESILIENT)
            as aom_codec_frame_flags_t;
    }
    if (*cpi).droppable != 0 {
        flags = (flags as core::ffi::c_uint | AOM_FRAME_IS_DROPPABLE)
            as aom_codec_frame_flags_t;
    }
    return flags;
}
#[inline]
unsafe extern "C-unwind" fn get_src_border_in_pixels(
    mut cpi: *mut AV1_COMP,
    mut sb_size: BLOCK_SIZE,
) -> core::ffi::c_int {
    if (*cpi).oxcf.mode as core::ffi::c_int != REALTIME as core::ffi::c_int
        || av1_is_resize_needed(&raw mut (*cpi).oxcf) as core::ffi::c_int != 0
    {
        return (*cpi).oxcf.border_in_pixels;
    }
    let sb_size_in_pixels_log2: core::ffi::c_int = mi_size_wide_log2[sb_size as usize]
        as core::ffi::c_int + MI_SIZE_LOG2;
    let sb_aligned_width: core::ffi::c_int = (*cpi).oxcf.frm_dim_cfg.width
        + (((1 as core::ffi::c_int) << sb_size_in_pixels_log2) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << sb_size_in_pixels_log2) - 1 as core::ffi::c_int);
    let sb_aligned_height: core::ffi::c_int = (*cpi).oxcf.frm_dim_cfg.height
        + (((1 as core::ffi::c_int) << sb_size_in_pixels_log2) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << sb_size_in_pixels_log2) - 1 as core::ffi::c_int);
    let border_pixels_width: core::ffi::c_int = sb_aligned_width
        - (*cpi).oxcf.frm_dim_cfg.width
        + (((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int);
    let border_pixels_height: core::ffi::c_int = sb_aligned_height
        - (*cpi).oxcf.frm_dim_cfg.height
        + (((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int);
    let border_in_pixels: core::ffi::c_int = if (if border_pixels_width
        > border_pixels_height
    {
        border_pixels_width
    } else {
        border_pixels_height
    }) > 32 as core::ffi::c_int
    {
        if border_pixels_width > border_pixels_height {
            border_pixels_width
        } else {
            border_pixels_height
        }
    } else {
        32 as core::ffi::c_int
    };
    return border_in_pixels;
}
unsafe extern "C-unwind" fn encoder_encode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut img: *const aom_image_t,
    mut pts: aom_codec_pts_t,
    mut duration: core::ffi::c_ulong,
    mut enc_flags: aom_enc_frame_flags_t,
) -> aom_codec_err_t {
    let kMinCompressedSize: size_t = 8192 as size_t;
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let mut ptsvol: aom_codec_pts_t = pts;
    let mut cpi_data: AV1_COMP_DATA = {
        let mut init = AV1_COMP_DATA {
            cx_data: 0 as *mut core::ffi::c_uchar,
            cx_data_sz: 0,
            frame_size: 0,
            lib_flags: 0,
            ts_frame_start: 0,
            ts_frame_end: 0,
            flush: 0,
            timestamp_ratio: 0 as *const aom_rational64_t,
            pop_lookahead: 0,
        };
        init
    };
    cpi_data.timestamp_ratio = &raw mut (*ctx).timestamp_ratio;
    cpi_data.flush = img.is_null() as core::ffi::c_int;
    let mut cpi_lap: *mut AV1_COMP = (*ppi).cpi_lap as *mut AV1_COMP;
    if ((*ppi).cpi).is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*(*ppi).cpi).last_coded_width = (*(*ppi).cpi).oxcf.frm_dim_cfg.width;
    (*(*ppi).cpi).last_coded_height = (*(*ppi).cpi).oxcf.frm_dim_cfg.height;
    if (*ppi).lap_enabled != 0 && cpi_lap.is_null()
        && (*(*ppi).cpi).oxcf.pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    if !img.is_null() {
        ::core::ptr::write_volatile(
            &raw mut res as *mut aom_codec_err_t,
            validate_img(ctx, img),
        );
        if res as core::ffi::c_uint
            == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            let uncompressed_frame_sz64: uint64_t = ((((*ctx).cfg.g_w)
                .wrapping_add(
                    ((1 as core::ffi::c_uint) << 5 as core::ffi::c_int)
                        .wrapping_sub(1 as core::ffi::c_uint),
                )
                & !((1 as core::ffi::c_uint) << 5 as core::ffi::c_int)
                    .wrapping_sub(1 as core::ffi::c_uint)) as uint64_t)
                .wrapping_mul(
                    (((*ctx).cfg.g_h)
                        .wrapping_add(
                            ((1 as core::ffi::c_uint) << 5 as core::ffi::c_int)
                                .wrapping_sub(1 as core::ffi::c_uint),
                        )
                        & !((1 as core::ffi::c_uint) << 5 as core::ffi::c_int)
                            .wrapping_sub(1 as core::ffi::c_uint)) as uint64_t,
                )
                .wrapping_mul(get_image_bps(img) as uint64_t)
                .wrapping_div(8 as uint64_t);
            let uncompressed_frame_sz: size_t = uncompressed_frame_sz64 as size_t;
            let mut multiplier: aom_rational_t = aom_rational { num: 0, den: 0 };
            multiplier.num = 8 as core::ffi::c_int;
            multiplier.den = 1 as core::ffi::c_int;
            if (*(*ppi).cpi).oxcf.kf_cfg.key_freq_max == 0 as core::ffi::c_int
                && !(*(*ppi).cpi).oxcf.kf_cfg.fwd_kf_enabled
            {
                multiplier.num = 5 as core::ffi::c_int;
                multiplier.den = 2 as core::ffi::c_int;
            }
            if uncompressed_frame_sz
                > (SIZE_MAX as size_t).wrapping_div(multiplier.num as size_t)
            {
                return AOM_CODEC_MEM_ERROR;
            }
            let mut data_sz: size_t = uncompressed_frame_sz
                .wrapping_mul(multiplier.num as size_t)
                .wrapping_div(multiplier.den as size_t);
            if data_sz < kMinCompressedSize {
                data_sz = kMinCompressedSize;
            }
            if ((*ctx).cx_data).is_null() || (*ctx).cx_data_sz < data_sz {
                (*ctx).cx_data_sz = data_sz;
                free((*ctx).cx_data as *mut core::ffi::c_void);
                (*ctx).cx_data = malloc((*ctx).cx_data_sz) as *mut core::ffi::c_uchar;
                if ((*ctx).cx_data).is_null() {
                    (*ctx).cx_data_sz = 0 as size_t;
                    return AOM_CODEC_MEM_ERROR;
                }
            }
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < (*ppi).num_fp_contexts - 1 as core::ffi::c_int {
                if ((*ppi).parallel_frames_data[i as usize].cx_data).is_null()
                    || (*ppi).parallel_frames_data[i as usize].cx_data_sz < data_sz
                {
                    (*ppi).parallel_frames_data[i as usize].cx_data_sz = data_sz;
                    free(
                        (*ppi).parallel_frames_data[i as usize].cx_data
                            as *mut core::ffi::c_void,
                    );
                    (*ppi).parallel_frames_data[i as usize].frame_size = 0 as size_t;
                    (*ppi).parallel_frames_data[i as usize].cx_data = malloc(
                        (*ppi).parallel_frames_data[i as usize].cx_data_sz,
                    ) as *mut core::ffi::c_uchar;
                    if ((*ppi).parallel_frames_data[i as usize].cx_data).is_null() {
                        (*ppi).parallel_frames_data[i as usize].cx_data_sz = 0 as size_t;
                        return AOM_CODEC_MEM_ERROR;
                    }
                }
                i += 1;
            }
        }
    }
    (*ctx).pkt_list.alloc.head.cnt = 0 as core::ffi::c_uint;
    (*ctx).pkt_list.alloc.head.max = (::core::mem::size_of::<[aom_codec_cx_pkt; 256]>()
        as usize)
        .wrapping_div(::core::mem::size_of::<aom_codec_cx_pkt>() as usize)
        as core::ffi::c_uint;
    let mut flags: aom_enc_frame_flags_t = enc_flags;
    loop {
match crate::jump::value_scope((((*ppi).error.jmp).as_mut_ptr()).cast(), || {
    (*ppi).error.setjmp = 1 as core::ffi::c_int;
    if (*ppi).use_svc != 0
        && (*(*ppi).cpi).svc.use_flexible_mode == 0 as core::ffi::c_int
        && flags == 0 as aom_enc_frame_flags_t
    {
        av1_set_svc_fixed_mode((*ppi).cpi);
    }
    av1_apply_encoding_flags((*ppi).cpi as *mut AV1_COMP, flags);
    if !cpi_lap.is_null() {
        av1_apply_encoding_flags(cpi_lap, flags);
    }
    if is_stat_generation_stage((*ppi).cpi) != 0
        || is_one_pass_rt_params((*ppi).cpi) != 0
    {
        if (*ctx).cfg.kf_mode as core::ffi::c_uint
            == AOM_KF_AUTO as core::ffi::c_int as core::ffi::c_uint
            && (*ctx).cfg.kf_min_dist == (*ctx).cfg.kf_max_dist
        {
            if (*(*ppi).cpi).common.spatial_layer_id == 0 as core::ffi::c_int
                && {
                    (*ctx).fixed_kf_cntr = ((*ctx).fixed_kf_cntr).wrapping_add(1);
                    (*ctx).fixed_kf_cntr > (*ctx).cfg.kf_min_dist
                }
            {
                ::core::ptr::write_volatile(
                    &raw mut flags as *mut aom_enc_frame_flags_t,
                    ::core::ptr::read_volatile::<
                        aom_enc_frame_flags_t,
                    >(&flags as *const aom_enc_frame_flags_t)
                        | AOM_EFLAG_FORCE_KF as aom_enc_frame_flags_t,
                );
                (*ctx).fixed_kf_cntr = 1 as core::ffi::c_uint;
            }
        }
    }
    if res as core::ffi::c_uint == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        let mut cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
        if flags & AOM_EFLAG_CALCULATE_PSNR as aom_enc_frame_flags_t != 0
            && (*ctx).cfg.g_lag_in_frames != 0 as core::ffi::c_uint
        {
            aom_internal_error(
                &raw mut (*ppi).error as *mut aom_internal_error_info,
                AOM_CODEC_INCAPABLE,
                b"Cannot calculate per-frame PSNR when g_lag_in_frames is nonzero\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if flags & AOM_EFLAG_FREEZE_INTERNAL_STATE as aom_enc_frame_flags_t != 0
            && (*ctx).cfg.g_lag_in_frames != 0 as core::ffi::c_uint
        {
            aom_internal_error(
                &raw mut (*ppi).error as *mut aom_internal_error_info,
                AOM_CODEC_INCAPABLE,
                b"Cannot freeze internal state when g_lag_in_frames is nonzero\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        (*ppi).b_calculate_psnr = ((*ctx).base.init_flags
            & AOM_CODEC_USE_PSNR as aom_codec_flags_t != 0
            || flags & AOM_EFLAG_CALCULATE_PSNR as aom_enc_frame_flags_t != 0)
            as core::ffi::c_int;
        (*ppi).b_freeze_internal_state = (flags
            & AOM_EFLAG_FREEZE_INTERNAL_STATE as aom_enc_frame_flags_t
            != 0 as aom_enc_frame_flags_t) as core::ffi::c_int;
        if !img.is_null() {
            if (*ctx).pts_offset_initialized == 0 {
                (*ctx).pts_offset = ptsvol;
                (*ctx).pts_offset_initialized = 1 as core::ffi::c_uchar;
            }
            if ptsvol < (*ctx).pts_offset {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_INVALID_PARAM,
                    b"pts is smaller than initial pts\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            ::core::ptr::write_volatile(
                &raw mut ptsvol as *mut aom_codec_pts_t,
                ::core::ptr::read_volatile::<
                    aom_codec_pts_t,
                >(&ptsvol as *const aom_codec_pts_t) - (*ctx).pts_offset,
            );
            if ptsvol > INT64_MAX as int64_t / (*cpi_data.timestamp_ratio).num {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_INVALID_PARAM,
                    b"conversion of relative pts to ticks would overflow\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            let mut src_time_stamp: int64_t = timebase_units_to_ticks(
                cpi_data.timestamp_ratio,
                ptsvol as int64_t,
            );
            if duration as core::ffi::c_ulonglong > INT64_MAX as core::ffi::c_ulonglong {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_INVALID_PARAM,
                    b"duration is too big\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if ptsvol > INT64_MAX as int64_t - duration as int64_t {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_INVALID_PARAM,
                    b"relative pts + duration is too big\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            let mut pts_end: aom_codec_pts_t = ptsvol + duration as aom_codec_pts_t;
            if pts_end > INT64_MAX as int64_t / (*cpi_data.timestamp_ratio).num {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_INVALID_PARAM,
                    b"conversion of relative pts + duration to ticks would overflow\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            let mut src_end_time_stamp: int64_t = timebase_units_to_ticks(
                cpi_data.timestamp_ratio,
                pts_end as int64_t,
            );
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
                c2rust_unnamed: C2RustUnnamed_9 {
                    c2rust_unnamed: C2RustUnnamed_10 {
                        y_width: 0,
                        uv_width: 0,
                    },
                },
                c2rust_unnamed_0: C2RustUnnamed_7 {
                    c2rust_unnamed: C2RustUnnamed_8 {
                        y_height: 0,
                        uv_height: 0,
                    },
                },
                c2rust_unnamed_1: C2RustUnnamed_5 {
                    c2rust_unnamed: C2RustUnnamed_6 {
                        y_crop_width: 0,
                        uv_crop_width: 0,
                    },
                },
                c2rust_unnamed_2: C2RustUnnamed_3 {
                    c2rust_unnamed: C2RustUnnamed_4 {
                        y_crop_height: 0,
                        uv_crop_height: 0,
                    },
                },
                c2rust_unnamed_3: C2RustUnnamed_1 {
                    c2rust_unnamed: C2RustUnnamed_2 {
                        y_stride: 0,
                        uv_stride: 0,
                    },
                },
                c2rust_unnamed_4: C2RustUnnamed {
                    c2rust_unnamed: C2RustUnnamed_0 {
                        y_buffer: 0 as *mut uint8_t,
                        u_buffer: 0 as *mut uint8_t,
                        v_buffer: 0 as *mut uint8_t,
                    },
                },
                use_external_reference_buffers: 0,
                store_buf_adr: [0 as *mut uint8_t; 3],
                y_pyramid: 0 as *mut image_pyramid,
                corners: 0 as *mut corner_list,
                buffer_alloc: 0 as *mut uint8_t,
                buffer_alloc_sz: 0,
                border: 0,
                frame_size: 0,
                subsampling_x: 0,
                subsampling_y: 0,
                bit_depth: 0,
                color_primaries: AOM_CICP_CP_RESERVED_0,
                transfer_characteristics: AOM_CICP_TC_RESERVED_0,
                matrix_coefficients: AOM_CICP_MC_IDENTITY,
                monochrome: 0,
                chroma_sample_position: AOM_CSP_UNKNOWN,
                color_range: AOM_CR_STUDIO_RANGE,
                render_width: 0,
                render_height: 0,
                corrupted: 0,
                flags: 0,
                metadata: 0 as *mut aom_metadata_array_t,
            };
            ::core::ptr::write_volatile(
                &raw mut res as *mut aom_codec_err_t,
                image2yuvconfig(img, &raw mut sd),
            );
            if (*ctx).cfg.monochrome != 0 {
                sd.c2rust_unnamed_4.c2rust_unnamed.v_buffer = 0 as *mut uint8_t;
                sd.c2rust_unnamed_4.c2rust_unnamed.u_buffer = sd
                    .c2rust_unnamed_4
                    .c2rust_unnamed
                    .v_buffer;
                sd.c2rust_unnamed_3.c2rust_unnamed.uv_stride = 0 as core::ffi::c_int;
                sd.monochrome = 1 as uint8_t;
            }
            let mut use_highbitdepth: core::ffi::c_int = (sd.flags
                & YV12_FLAG_HIGHBITDEPTH != 0 as core::ffi::c_int) as core::ffi::c_int;
            let mut subsampling_x: core::ffi::c_int = sd.subsampling_x;
            let mut subsampling_y: core::ffi::c_int = sd.subsampling_y;
            if ((*ppi).lookahead).is_null() {
                let mut lag_in_frames: core::ffi::c_int = if !cpi_lap.is_null() {
                    (*cpi_lap).oxcf.gf_cfg.lag_in_frames
                } else {
                    (*cpi).oxcf.gf_cfg.lag_in_frames
                };
                let mut oxcf: *mut AV1EncoderConfig = &raw mut (*cpi).oxcf;
                let sb_size: BLOCK_SIZE = av1_select_sb_size(
                    oxcf,
                    (*oxcf).frm_dim_cfg.width,
                    (*oxcf).frm_dim_cfg.height,
                    (*ppi).number_spatial_layers as core::ffi::c_int,
                ) as BLOCK_SIZE;
                (*oxcf).border_in_pixels = av1_get_enc_border_size(
                    av1_is_resize_needed(oxcf),
                    (*oxcf).kf_cfg.key_freq_max == 0 as core::ffi::c_int,
                    sb_size,
                );
                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_0 < (*ppi).num_fp_contexts {
                    (*(*ppi).parallel_cpi[i_0 as usize]).oxcf.border_in_pixels = (*oxcf)
                        .border_in_pixels;
                    i_0 += 1;
                }
                let src_border_in_pixels: core::ffi::c_int = get_src_border_in_pixels(
                    cpi,
                    sb_size,
                ) as core::ffi::c_int;
                (*ppi).lookahead = av1_lookahead_init(
                    (*cpi).oxcf.frm_dim_cfg.width,
                    (*cpi).oxcf.frm_dim_cfg.height,
                    subsampling_x,
                    subsampling_y,
                    use_highbitdepth,
                    lag_in_frames,
                    src_border_in_pixels,
                    (*cpi).common.features.byte_alignment,
                    (*ctx).num_lap_buffers,
                    (*cpi).oxcf.kf_cfg.key_freq_max == 0 as core::ffi::c_int,
                    (*cpi).alloc_pyramid,
                );
            }
            if ((*ppi).lookahead).is_null() {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate lag buffers\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_1 < (*ppi).num_fp_contexts {
                let mut err: aom_codec_err_t = av1_check_initial_width(
                    (*ppi).parallel_cpi[i_1 as usize],
                    use_highbitdepth,
                    subsampling_x,
                    subsampling_y,
                );
                if err as core::ffi::c_uint
                    != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        err,
                        b"av1_check_initial_width() failed\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                i_1 += 1;
            }
            if !cpi_lap.is_null() {
                let mut err_0: aom_codec_err_t = av1_check_initial_width(
                    cpi_lap,
                    use_highbitdepth,
                    subsampling_x,
                    subsampling_y,
                );
                if err_0 as core::ffi::c_uint
                    != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        err_0,
                        b"av1_check_initial_width() failed\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            if av1_receive_raw_frame(
                cpi,
                flags | (*ctx).next_frame_flags,
                &raw mut sd,
                src_time_stamp,
                src_end_time_stamp,
            ) != 0
            {
                ::core::ptr::write_volatile(
                    &raw mut res as *mut aom_codec_err_t,
                    update_error_state(ctx, (*cpi).common.error),
                );
            }
            (*ctx).next_frame_flags = 0 as aom_enc_frame_flags_t;
        }
        cpi_data.cx_data = (*ctx).cx_data;
        cpi_data.cx_data_sz = (*ctx).cx_data_sz;
        if (*ctx).pending_cx_data_sz != 0 {
            cpi_data.cx_data = (cpi_data.cx_data)
                .offset((*ctx).pending_cx_data_sz as isize);
            cpi_data.cx_data_sz = (cpi_data.cx_data_sz)
                .wrapping_sub((*ctx).pending_cx_data_sz);
            if cpi_data.cx_data_sz < ((*ctx).cx_data_sz).wrapping_div(2 as size_t) {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_ERROR,
                    b"Compressed data buffer too small\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        let mut is_frame_visible: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut has_no_show_keyframe: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut num_workers: core::ffi::c_int = 0 as core::ffi::c_int;
        if (*cpi).oxcf.pass as core::ffi::c_uint
            == AOM_RC_FIRST_PASS as core::ffi::c_int as core::ffi::c_uint
        {
            (*ppi).p_mt_info.num_mod_workers[MOD_FP as core::ffi::c_int as usize] = av1_fp_compute_num_enc_workers(
                cpi,
            );
            num_workers = (*ppi)
                .p_mt_info
                .num_mod_workers[MOD_FP as core::ffi::c_int as usize];
        } else {
            av1_compute_num_workers_for_mt(cpi);
            num_workers = av1_get_max_num_workers(cpi);
        }
        if num_workers > 1 as core::ffi::c_int
            && (*ppi).p_mt_info.num_workers < num_workers
        {
            if is_stat_consumption_stage(cpi) != 0 {
                (*ppi).num_fp_contexts = av1_compute_num_fp_contexts(
                    ppi,
                    &raw mut (*cpi).oxcf,
                );
            }
            if (*ppi).p_mt_info.num_workers > 0 as core::ffi::c_int {
                av1_terminate_workers(ppi);
                free_thread_data(ppi);
                aom_free((*ppi).p_mt_info.tile_thr_data as *mut core::ffi::c_void);
                (*ppi).p_mt_info.tile_thr_data = 0 as *mut EncWorkerData;
                aom_free((*ppi).p_mt_info.workers as *mut core::ffi::c_void);
                (*ppi).p_mt_info.workers = 0 as *mut AVxWorker;
                (*ppi).p_mt_info.num_workers = 0 as core::ffi::c_int;
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < (*ppi).num_fp_contexts {
                    aom_free(
                        (*(*ppi).parallel_cpi[j as usize]).td.tctx
                            as *mut core::ffi::c_void,
                    );
                    (*(*ppi).parallel_cpi[j as usize]).td.tctx = 0 as *mut FRAME_CONTEXT;
                    j += 1;
                }
            }
            av1_create_workers(ppi, num_workers);
            av1_init_tile_thread_data(
                ppi,
                ((*cpi).oxcf.pass as core::ffi::c_uint
                    == AOM_RC_FIRST_PASS as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int,
            );
        }
        let num_enc_workers: core::ffi::c_int = av1_get_num_mod_workers_for_alloc(
            &raw mut (*ppi).p_mt_info,
            MOD_ENC,
        ) as core::ffi::c_int;
        if (*ppi).p_mt_info.prev_num_enc_workers < num_enc_workers
            && num_enc_workers <= (*ppi).p_mt_info.num_workers
        {
            free_thread_data(ppi);
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < (*ppi).num_fp_contexts {
                aom_free(
                    (*(*ppi).parallel_cpi[j_0 as usize]).td.tctx
                        as *mut core::ffi::c_void,
                );
                (*(*ppi).parallel_cpi[j_0 as usize]).td.tctx = 0 as *mut FRAME_CONTEXT;
                j_0 += 1;
            }
            av1_init_tile_thread_data(
                ppi,
                ((*cpi).oxcf.pass as core::ffi::c_uint
                    == AOM_RC_FIRST_PASS as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int,
            );
        }
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < (*ppi).num_fp_contexts {
            av1_init_frame_mt(ppi, (*ppi).parallel_cpi[i_2 as usize]);
            i_2 += 1;
        }
        if !cpi_lap.is_null() {
            av1_init_frame_mt(ppi, cpi_lap);
        }
        if (*ppi).p_mt_info.num_workers > 1 as core::ffi::c_int {
            let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_3 < (*ppi).num_fp_contexts {
                av1_init_mt_sync(
                    (*ppi).parallel_cpi[i_3 as usize],
                    ((*(*ppi).parallel_cpi[i_3 as usize]).oxcf.pass as core::ffi::c_uint
                        == AOM_RC_FIRST_PASS as core::ffi::c_int as core::ffi::c_uint)
                        as core::ffi::c_int,
                );
                i_3 += 1;
            }
            if !cpi_lap.is_null() {
                av1_init_mt_sync(cpi_lap, 1 as core::ffi::c_int);
            }
        }
        if !cpi_lap.is_null() && is_one_pass_rt_lag_params(cpi) == 0 {
            if (*(*cpi_lap).ppi).b_freeze_internal_state != 0 {
                av1_save_all_coding_context(cpi_lap);
            }
            let mut cpi_lap_data: AV1_COMP_DATA = {
                let mut init = AV1_COMP_DATA {
                    cx_data: 0 as *mut core::ffi::c_uchar,
                    cx_data_sz: 0,
                    frame_size: 0,
                    lib_flags: 0,
                    ts_frame_start: 0,
                    ts_frame_end: 0,
                    flush: 0,
                    timestamp_ratio: 0 as *const aom_rational64_t,
                    pop_lookahead: 0,
                };
                init
            };
            cpi_lap_data.flush = img.is_null() as core::ffi::c_int;
            cpi_lap_data.timestamp_ratio = &raw mut (*ctx).timestamp_ratio;
            let status: core::ffi::c_int = av1_get_compressed_data(
                cpi_lap,
                &raw mut cpi_lap_data,
            ) as core::ffi::c_int;
            if status > AOM_CODEC_OK as core::ffi::c_int {
                aom_internal_error_copy(&raw mut (*ppi).error, (*cpi_lap).common.error);
            }
            av1_post_encode_updates(cpi_lap, &raw mut cpi_lap_data);
            if (*(*cpi_lap).ppi).b_freeze_internal_state != 0 {
                restore_all_coding_context(cpi_lap);
            }
        }
        if is_stat_consumption_stage(cpi) != 0
            && (*ppi).gf_group.size > 0 as core::ffi::c_int
            && (*cpi).gf_frame_index as core::ffi::c_int == (*ppi).gf_group.size
        {
            (*ppi).num_fp_contexts = av1_compute_num_fp_contexts(ppi, &raw mut (*cpi).oxcf);
        }
        while cpi_data.cx_data_sz >= ((*ctx).cx_data_sz).wrapping_div(2 as size_t)
            && is_frame_visible == 0
        {
            let mut simulate_parallel_frame: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut status_0: core::ffi::c_int = -(1 as core::ffi::c_int);
            (*cpi).do_frame_data_update = true_0 != 0;
            (*cpi).ref_idx_to_skip = INVALID_IDX;
            (*cpi).ref_refresh_index = INVALID_IDX;
            (*cpi).refresh_idx_available = false_0 != 0;
            if (*(*cpi).ppi).b_freeze_internal_state != 0 {
                av1_save_all_coding_context(cpi);
            }
            if simulate_parallel_frame == 0 {
                if (*cpi).gf_frame_index as core::ffi::c_int
                    >= MAX_STATIC_GF_GROUP_LENGTH
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        b"cpi->gf_frame_index is out of range\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if (*ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
                    == 0 as core::ffi::c_int
                {
                    status_0 = av1_get_compressed_data(cpi, &raw mut cpi_data);
                } else if (*ppi)
                    .gf_group
                    .frame_parallel_level[(*cpi).gf_frame_index as usize]
                    == 1 as core::ffi::c_int
                {
                    av1_compress_parallel_frames(ppi, &raw mut cpi_data);
                    status_0 = AOM_CODEC_OK as core::ffi::c_int;
                } else {
                    cpi = av1_get_parallel_frame_enc_data(ppi, &raw mut cpi_data);
                    status_0 = AOM_CODEC_OK as core::ffi::c_int;
                }
            }
            if status_0 == -(1 as core::ffi::c_int) {
                break;
            }
            if status_0 != AOM_CODEC_OK as core::ffi::c_int {
                aom_internal_error_copy(&raw mut (*ppi).error, (*cpi).common.error);
            }
            if (*ppi).num_fp_contexts > 0 as core::ffi::c_int
                && frame_is_intra_only(&raw mut (*cpi).common) != 0
            {
                av1_init_sc_decisions(ppi);
            }
            (*ppi).seq_params_locked = 1 as core::ffi::c_int;
            av1_post_encode_updates(cpi, &raw mut cpi_data);
            if (*(*cpi).ppi).b_freeze_internal_state != 0 {
                restore_all_coding_context(cpi);
            }
            if cpi_data.frame_size == 0 {
                continue;
            }
            if cpi_data.frame_size > cpi_data.cx_data_sz {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_ERROR,
                    b"cpi_data.cx_data buffer overflow\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            let write_temporal_delimiter: core::ffi::c_int = ((*cpi)
                .common
                .spatial_layer_id == 0 && (*ctx).pending_cx_data_sz == 0)
                as core::ffi::c_int;
            if write_temporal_delimiter != 0 {
                let mut obu_header_size: uint32_t = 1 as uint32_t;
                let obu_payload_size: uint32_t = 0 as uint32_t;
                let length_field_size: size_t = aom_uleb_size_in_bytes(
                    obu_payload_size as uint64_t,
                ) as size_t;
                let move_offset: size_t = (obu_header_size as size_t)
                    .wrapping_add(length_field_size);
                if move_offset > ((*ctx).cx_data_sz).wrapping_sub(cpi_data.frame_size) {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        b"ctx->cx_data buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                memmove(
                    ((*ctx).cx_data).offset(move_offset as isize)
                        as *mut core::ffi::c_void,
                    (*ctx).cx_data as *const core::ffi::c_void,
                    cpi_data.frame_size,
                );
                obu_header_size = av1_write_obu_header(
                    &raw mut (*ppi).level_params,
                    &raw mut (*cpi).frame_header_count,
                    OBU_TEMPORAL_DELIMITER,
                    (*ppi).seq_params.has_nonzero_operating_point_idc,
                    false_0 != 0,
                    0 as core::ffi::c_int,
                    (*ctx).cx_data as *mut uint8_t,
                );
                if obu_header_size != 1 as uint32_t {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        0 as *const core::ffi::c_char,
                    );
                }
                if av1_write_uleb_obu_size(
                    obu_payload_size as size_t,
                    ((*ctx).cx_data).offset(obu_header_size as isize),
                    length_field_size,
                ) != AOM_CODEC_OK as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        0 as *const core::ffi::c_char,
                    );
                }
                cpi_data.frame_size = (cpi_data.frame_size).wrapping_add(move_offset);
            }
            if (*ctx).oxcf.save_as_annexb {
                if av1_convert_sect5obus_to_annexb(
                    cpi_data.cx_data as *mut uint8_t,
                    cpi_data.cx_data_sz,
                    &raw mut cpi_data.frame_size,
                ) != AOM_CODEC_OK as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        0 as *const core::ffi::c_char,
                    );
                }
                let length_field_size_0: size_t = aom_uleb_size_in_bytes(
                    cpi_data.frame_size as uint64_t,
                ) as size_t;
                if length_field_size_0
                    > (cpi_data.cx_data_sz).wrapping_sub(cpi_data.frame_size)
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        b"cpi_data.cx_data buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                memmove(
                    (cpi_data.cx_data).offset(length_field_size_0 as isize)
                        as *mut core::ffi::c_void,
                    cpi_data.cx_data as *const core::ffi::c_void,
                    cpi_data.frame_size,
                );
                if av1_write_uleb_obu_size(
                    cpi_data.frame_size,
                    cpi_data.cx_data as *mut uint8_t,
                    length_field_size_0,
                ) != AOM_CODEC_OK as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        0 as *const core::ffi::c_char,
                    );
                }
                cpi_data.frame_size = (cpi_data.frame_size)
                    .wrapping_add(length_field_size_0);
            }
            (*ctx).pending_cx_data_sz = ((*ctx).pending_cx_data_sz)
                .wrapping_add(cpi_data.frame_size);
            cpi_data.cx_data = (cpi_data.cx_data).offset(cpi_data.frame_size as isize);
            cpi_data.cx_data_sz = (cpi_data.cx_data_sz)
                .wrapping_sub(cpi_data.frame_size);
            is_frame_visible = (*cpi).common.show_frame;
            has_no_show_keyframe
                |= (is_frame_visible == 0
                    && (*cpi).common.current_frame.frame_type as core::ffi::c_int
                        == KEY_FRAME as core::ffi::c_int) as core::ffi::c_int;
        }
        if is_frame_visible != 0 {
            let mut pkt: aom_codec_cx_pkt_t = aom_codec_cx_pkt {
                kind: AOM_CODEC_CX_FRAME_PKT,
                data: C2RustUnnamed_13 {
                    frame: C2RustUnnamed_14 {
                        buf: 0 as *mut core::ffi::c_void,
                        sz: 0,
                        pts: 0,
                        duration: 0,
                        flags: 0,
                        partition_id: 0,
                        vis_frame_size: 0,
                    },
                },
            };
            (*ppi).frames_left = if 0 as core::ffi::c_int
                > (*ppi).frames_left - 1 as core::ffi::c_int
            {
                0 as core::ffi::c_int
            } else {
                (*ppi).frames_left - 1 as core::ffi::c_int
            };
            if (*ctx).oxcf.save_as_annexb {
                let mut tu_size: size_t = (*ctx).pending_cx_data_sz;
                let length_field_size_1: size_t = aom_uleb_size_in_bytes(
                    tu_size as uint64_t,
                ) as size_t;
                if tu_size > (*ctx).cx_data_sz {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        b"ctx->cx_data buffer overflow\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if length_field_size_1 > ((*ctx).cx_data_sz).wrapping_sub(tu_size) {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        b"ctx->cx_data buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                memmove(
                    ((*ctx).cx_data).offset(length_field_size_1 as isize)
                        as *mut core::ffi::c_void,
                    (*ctx).cx_data as *const core::ffi::c_void,
                    tu_size,
                );
                if av1_write_uleb_obu_size(
                    tu_size,
                    (*ctx).cx_data as *mut uint8_t,
                    length_field_size_1,
                ) != AOM_CODEC_OK as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_ERROR,
                        0 as *const core::ffi::c_char,
                    );
                }
                (*ctx).pending_cx_data_sz = ((*ctx).pending_cx_data_sz)
                    .wrapping_add(length_field_size_1);
            }
            pkt.kind = AOM_CODEC_CX_FRAME_PKT;
            pkt.data.frame.buf = (*ctx).cx_data as *mut core::ffi::c_void;
            pkt.data.frame.sz = (*ctx).pending_cx_data_sz;
            pkt.data.frame.partition_id = -(1 as core::ffi::c_int);
            pkt.data.frame.vis_frame_size = cpi_data.frame_size;
            pkt.data.frame.pts = (ticks_to_timebase_units(
                cpi_data.timestamp_ratio,
                cpi_data.ts_frame_start,
            ) + (*ctx).pts_offset as int64_t) as aom_codec_pts_t;
            pkt.data.frame.flags = get_frame_pkt_flags(cpi, cpi_data.lib_flags);
            if has_no_show_keyframe != 0 {
                pkt.data.frame.flags = (pkt.data.frame.flags as core::ffi::c_uint
                    | AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT)
                    as aom_codec_frame_flags_t;
            }
            let duration64: int64_t = ticks_to_timebase_units(
                cpi_data.timestamp_ratio,
                cpi_data.ts_frame_end - cpi_data.ts_frame_start,
            ) as int64_t;
            if duration64 > UINT32_MAX as int64_t {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_ERROR,
                    0 as *const core::ffi::c_char,
                );
            }
            pkt.data.frame.duration = duration64 as uint32_t as core::ffi::c_ulong;
            aom_codec_pkt_list_add(&raw mut (*ctx).pkt_list.head, &raw mut pkt);
            (*ctx).pending_cx_data_sz = 0 as size_t;
        }
    }
    (*ppi).error.setjmp = 0 as core::ffi::c_int;
    return res;}) {
Some(value) => return value,
None => {
        (*ppi).error.setjmp = 0 as core::ffi::c_int;
        ::core::ptr::write_volatile(
            &raw mut res as *mut aom_codec_err_t,
            update_error_state(ctx, &raw mut (*ppi).error),
        );
        return res;}
}
}}
unsafe extern "C-unwind" fn encoder_get_cxdata(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut iter: *mut aom_codec_iter_t,
) -> *const aom_codec_cx_pkt_t {
    return aom_codec_pkt_list_get(&raw mut (*ctx).pkt_list.head, iter);
}
unsafe extern "C-unwind" fn ctrl_set_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let frame: *mut av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !frame.is_null() {
        let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
            c2rust_unnamed: C2RustUnnamed_9 {
                c2rust_unnamed: C2RustUnnamed_10 {
                    y_width: 0,
                    uv_width: 0,
                },
            },
            c2rust_unnamed_0: C2RustUnnamed_7 {
                c2rust_unnamed: C2RustUnnamed_8 {
                    y_height: 0,
                    uv_height: 0,
                },
            },
            c2rust_unnamed_1: C2RustUnnamed_5 {
                c2rust_unnamed: C2RustUnnamed_6 {
                    y_crop_width: 0,
                    uv_crop_width: 0,
                },
            },
            c2rust_unnamed_2: C2RustUnnamed_3 {
                c2rust_unnamed: C2RustUnnamed_4 {
                    y_crop_height: 0,
                    uv_crop_height: 0,
                },
            },
            c2rust_unnamed_3: C2RustUnnamed_1 {
                c2rust_unnamed: C2RustUnnamed_2 {
                    y_stride: 0,
                    uv_stride: 0,
                },
            },
            c2rust_unnamed_4: C2RustUnnamed {
                c2rust_unnamed: C2RustUnnamed_0 {
                    y_buffer: 0 as *mut uint8_t,
                    u_buffer: 0 as *mut uint8_t,
                    v_buffer: 0 as *mut uint8_t,
                },
            },
            use_external_reference_buffers: 0,
            store_buf_adr: [0 as *mut uint8_t; 3],
            y_pyramid: 0 as *mut image_pyramid,
            corners: 0 as *mut corner_list,
            buffer_alloc: 0 as *mut uint8_t,
            buffer_alloc_sz: 0,
            border: 0,
            frame_size: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            bit_depth: 0,
            color_primaries: AOM_CICP_CP_RESERVED_0,
            transfer_characteristics: AOM_CICP_TC_RESERVED_0,
            matrix_coefficients: AOM_CICP_MC_IDENTITY,
            monochrome: 0,
            chroma_sample_position: AOM_CSP_UNKNOWN,
            color_range: AOM_CR_STUDIO_RANGE,
            render_width: 0,
            render_height: 0,
            corrupted: 0,
            flags: 0,
            metadata: 0 as *mut aom_metadata_array_t,
        };
        image2yuvconfig(&raw mut (*frame).img, &raw mut sd);
        av1_set_reference_enc((*(*ctx).ppi).cpi as *mut AV1_COMP, (*frame).idx, &raw mut sd);
        return AOM_CODEC_OK;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_copy_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    if (*(*(*ctx).ppi).cpi).oxcf.algo_cfg.skip_postproc_filtering {
        return AOM_CODEC_INCAPABLE;
    }
    let frame: *mut av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !frame.is_null() {
        let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
            c2rust_unnamed: C2RustUnnamed_9 {
                c2rust_unnamed: C2RustUnnamed_10 {
                    y_width: 0,
                    uv_width: 0,
                },
            },
            c2rust_unnamed_0: C2RustUnnamed_7 {
                c2rust_unnamed: C2RustUnnamed_8 {
                    y_height: 0,
                    uv_height: 0,
                },
            },
            c2rust_unnamed_1: C2RustUnnamed_5 {
                c2rust_unnamed: C2RustUnnamed_6 {
                    y_crop_width: 0,
                    uv_crop_width: 0,
                },
            },
            c2rust_unnamed_2: C2RustUnnamed_3 {
                c2rust_unnamed: C2RustUnnamed_4 {
                    y_crop_height: 0,
                    uv_crop_height: 0,
                },
            },
            c2rust_unnamed_3: C2RustUnnamed_1 {
                c2rust_unnamed: C2RustUnnamed_2 {
                    y_stride: 0,
                    uv_stride: 0,
                },
            },
            c2rust_unnamed_4: C2RustUnnamed {
                c2rust_unnamed: C2RustUnnamed_0 {
                    y_buffer: 0 as *mut uint8_t,
                    u_buffer: 0 as *mut uint8_t,
                    v_buffer: 0 as *mut uint8_t,
                },
            },
            use_external_reference_buffers: 0,
            store_buf_adr: [0 as *mut uint8_t; 3],
            y_pyramid: 0 as *mut image_pyramid,
            corners: 0 as *mut corner_list,
            buffer_alloc: 0 as *mut uint8_t,
            buffer_alloc_sz: 0,
            border: 0,
            frame_size: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            bit_depth: 0,
            color_primaries: AOM_CICP_CP_RESERVED_0,
            transfer_characteristics: AOM_CICP_TC_RESERVED_0,
            matrix_coefficients: AOM_CICP_MC_IDENTITY,
            monochrome: 0,
            chroma_sample_position: AOM_CSP_UNKNOWN,
            color_range: AOM_CR_STUDIO_RANGE,
            render_width: 0,
            render_height: 0,
            corrupted: 0,
            flags: 0,
            metadata: 0 as *mut aom_metadata_array_t,
        };
        image2yuvconfig(&raw mut (*frame).img, &raw mut sd);
        av1_copy_reference_enc(
            (*(*ctx).ppi).cpi as *mut AV1_COMP,
            (*frame).idx,
            &raw mut sd,
        );
        return AOM_CODEC_OK;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    if (*(*(*ctx).ppi).cpi).oxcf.algo_cfg.skip_postproc_filtering {
        return AOM_CODEC_INCAPABLE;
    }
    let frame: *mut av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !frame.is_null() {
        let mut fb: *mut YV12_BUFFER_CONFIG = get_ref_frame(
            &raw mut (*(*(*ctx).ppi).cpi).common,
            (*frame).idx,
        );
        if fb.is_null() {
            return AOM_CODEC_ERROR;
        }
        yuvconfig2image(&raw mut (*frame).img, fb, NULL);
        return AOM_CODEC_OK;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_new_frame_image(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let new_img: *mut aom_image_t = args.next_arg::<*mut aom_image_t>();
    if !new_img.is_null() {
        let mut new_frame: YV12_BUFFER_CONFIG = yv12_buffer_config {
            c2rust_unnamed: C2RustUnnamed_9 {
                c2rust_unnamed: C2RustUnnamed_10 {
                    y_width: 0,
                    uv_width: 0,
                },
            },
            c2rust_unnamed_0: C2RustUnnamed_7 {
                c2rust_unnamed: C2RustUnnamed_8 {
                    y_height: 0,
                    uv_height: 0,
                },
            },
            c2rust_unnamed_1: C2RustUnnamed_5 {
                c2rust_unnamed: C2RustUnnamed_6 {
                    y_crop_width: 0,
                    uv_crop_width: 0,
                },
            },
            c2rust_unnamed_2: C2RustUnnamed_3 {
                c2rust_unnamed: C2RustUnnamed_4 {
                    y_crop_height: 0,
                    uv_crop_height: 0,
                },
            },
            c2rust_unnamed_3: C2RustUnnamed_1 {
                c2rust_unnamed: C2RustUnnamed_2 {
                    y_stride: 0,
                    uv_stride: 0,
                },
            },
            c2rust_unnamed_4: C2RustUnnamed {
                c2rust_unnamed: C2RustUnnamed_0 {
                    y_buffer: 0 as *mut uint8_t,
                    u_buffer: 0 as *mut uint8_t,
                    v_buffer: 0 as *mut uint8_t,
                },
            },
            use_external_reference_buffers: 0,
            store_buf_adr: [0 as *mut uint8_t; 3],
            y_pyramid: 0 as *mut image_pyramid,
            corners: 0 as *mut corner_list,
            buffer_alloc: 0 as *mut uint8_t,
            buffer_alloc_sz: 0,
            border: 0,
            frame_size: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            bit_depth: 0,
            color_primaries: AOM_CICP_CP_RESERVED_0,
            transfer_characteristics: AOM_CICP_TC_RESERVED_0,
            matrix_coefficients: AOM_CICP_MC_IDENTITY,
            monochrome: 0,
            chroma_sample_position: AOM_CSP_UNKNOWN,
            color_range: AOM_CR_STUDIO_RANGE,
            render_width: 0,
            render_height: 0,
            corrupted: 0,
            flags: 0,
            metadata: 0 as *mut aom_metadata_array_t,
        };
        if av1_get_last_show_frame((*(*ctx).ppi).cpi as *mut AV1_COMP, &raw mut new_frame)
            == 0 as core::ffi::c_int
        {
            yuvconfig2image(new_img, &raw mut new_frame, NULL);
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_copy_new_frame_image(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let new_img: *mut aom_image_t = args.next_arg::<*mut aom_image_t>();
    if !new_img.is_null() {
        let mut new_frame: YV12_BUFFER_CONFIG = yv12_buffer_config {
            c2rust_unnamed: C2RustUnnamed_9 {
                c2rust_unnamed: C2RustUnnamed_10 {
                    y_width: 0,
                    uv_width: 0,
                },
            },
            c2rust_unnamed_0: C2RustUnnamed_7 {
                c2rust_unnamed: C2RustUnnamed_8 {
                    y_height: 0,
                    uv_height: 0,
                },
            },
            c2rust_unnamed_1: C2RustUnnamed_5 {
                c2rust_unnamed: C2RustUnnamed_6 {
                    y_crop_width: 0,
                    uv_crop_width: 0,
                },
            },
            c2rust_unnamed_2: C2RustUnnamed_3 {
                c2rust_unnamed: C2RustUnnamed_4 {
                    y_crop_height: 0,
                    uv_crop_height: 0,
                },
            },
            c2rust_unnamed_3: C2RustUnnamed_1 {
                c2rust_unnamed: C2RustUnnamed_2 {
                    y_stride: 0,
                    uv_stride: 0,
                },
            },
            c2rust_unnamed_4: C2RustUnnamed {
                c2rust_unnamed: C2RustUnnamed_0 {
                    y_buffer: 0 as *mut uint8_t,
                    u_buffer: 0 as *mut uint8_t,
                    v_buffer: 0 as *mut uint8_t,
                },
            },
            use_external_reference_buffers: 0,
            store_buf_adr: [0 as *mut uint8_t; 3],
            y_pyramid: 0 as *mut image_pyramid,
            corners: 0 as *mut corner_list,
            buffer_alloc: 0 as *mut uint8_t,
            buffer_alloc_sz: 0,
            border: 0,
            frame_size: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            bit_depth: 0,
            color_primaries: AOM_CICP_CP_RESERVED_0,
            transfer_characteristics: AOM_CICP_TC_RESERVED_0,
            matrix_coefficients: AOM_CICP_MC_IDENTITY,
            monochrome: 0,
            chroma_sample_position: AOM_CSP_UNKNOWN,
            color_range: AOM_CR_STUDIO_RANGE,
            render_width: 0,
            render_height: 0,
            corrupted: 0,
            flags: 0,
            metadata: 0 as *mut aom_metadata_array_t,
        };
        if av1_get_last_show_frame((*(*ctx).ppi).cpi as *mut AV1_COMP, &raw mut new_frame)
            == 0 as core::ffi::c_int
        {
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
                c2rust_unnamed: C2RustUnnamed_9 {
                    c2rust_unnamed: C2RustUnnamed_10 {
                        y_width: 0,
                        uv_width: 0,
                    },
                },
                c2rust_unnamed_0: C2RustUnnamed_7 {
                    c2rust_unnamed: C2RustUnnamed_8 {
                        y_height: 0,
                        uv_height: 0,
                    },
                },
                c2rust_unnamed_1: C2RustUnnamed_5 {
                    c2rust_unnamed: C2RustUnnamed_6 {
                        y_crop_width: 0,
                        uv_crop_width: 0,
                    },
                },
                c2rust_unnamed_2: C2RustUnnamed_3 {
                    c2rust_unnamed: C2RustUnnamed_4 {
                        y_crop_height: 0,
                        uv_crop_height: 0,
                    },
                },
                c2rust_unnamed_3: C2RustUnnamed_1 {
                    c2rust_unnamed: C2RustUnnamed_2 {
                        y_stride: 0,
                        uv_stride: 0,
                    },
                },
                c2rust_unnamed_4: C2RustUnnamed {
                    c2rust_unnamed: C2RustUnnamed_0 {
                        y_buffer: 0 as *mut uint8_t,
                        u_buffer: 0 as *mut uint8_t,
                        v_buffer: 0 as *mut uint8_t,
                    },
                },
                use_external_reference_buffers: 0,
                store_buf_adr: [0 as *mut uint8_t; 3],
                y_pyramid: 0 as *mut image_pyramid,
                corners: 0 as *mut corner_list,
                buffer_alloc: 0 as *mut uint8_t,
                buffer_alloc_sz: 0,
                border: 0,
                frame_size: 0,
                subsampling_x: 0,
                subsampling_y: 0,
                bit_depth: 0,
                color_primaries: AOM_CICP_CP_RESERVED_0,
                transfer_characteristics: AOM_CICP_TC_RESERVED_0,
                matrix_coefficients: AOM_CICP_MC_IDENTITY,
                monochrome: 0,
                chroma_sample_position: AOM_CSP_UNKNOWN,
                color_range: AOM_CR_STUDIO_RANGE,
                render_width: 0,
                render_height: 0,
                corrupted: 0,
                flags: 0,
                metadata: 0 as *mut aom_metadata_array_t,
            };
            image2yuvconfig(new_img, &raw mut sd);
            return av1_copy_new_frame_enc(
                &raw mut (*(*(*ctx).ppi).cpi).common,
                &raw mut new_frame,
                &raw mut sd,
            );
        } else {
            return AOM_CODEC_ERROR
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn encoder_get_preview(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> *mut aom_image_t {
    let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
        c2rust_unnamed: C2RustUnnamed_9 {
            c2rust_unnamed: C2RustUnnamed_10 {
                y_width: 0,
                uv_width: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_7 {
            c2rust_unnamed: C2RustUnnamed_8 {
                y_height: 0,
                uv_height: 0,
            },
        },
        c2rust_unnamed_1: C2RustUnnamed_5 {
            c2rust_unnamed: C2RustUnnamed_6 {
                y_crop_width: 0,
                uv_crop_width: 0,
            },
        },
        c2rust_unnamed_2: C2RustUnnamed_3 {
            c2rust_unnamed: C2RustUnnamed_4 {
                y_crop_height: 0,
                uv_crop_height: 0,
            },
        },
        c2rust_unnamed_3: C2RustUnnamed_1 {
            c2rust_unnamed: C2RustUnnamed_2 {
                y_stride: 0,
                uv_stride: 0,
            },
        },
        c2rust_unnamed_4: C2RustUnnamed {
            c2rust_unnamed: C2RustUnnamed_0 {
                y_buffer: 0 as *mut uint8_t,
                u_buffer: 0 as *mut uint8_t,
                v_buffer: 0 as *mut uint8_t,
            },
        },
        use_external_reference_buffers: 0,
        store_buf_adr: [0 as *mut uint8_t; 3],
        y_pyramid: 0 as *mut image_pyramid,
        corners: 0 as *mut corner_list,
        buffer_alloc: 0 as *mut uint8_t,
        buffer_alloc_sz: 0,
        border: 0,
        frame_size: 0,
        subsampling_x: 0,
        subsampling_y: 0,
        bit_depth: 0,
        color_primaries: AOM_CICP_CP_RESERVED_0,
        transfer_characteristics: AOM_CICP_TC_RESERVED_0,
        matrix_coefficients: AOM_CICP_MC_IDENTITY,
        monochrome: 0,
        chroma_sample_position: AOM_CSP_UNKNOWN,
        color_range: AOM_CR_STUDIO_RANGE,
        render_width: 0,
        render_height: 0,
        corrupted: 0,
        flags: 0,
        metadata: 0 as *mut aom_metadata_array_t,
    };
    if av1_get_preview_raw_frame((*(*ctx).ppi).cpi as *mut AV1_COMP, &raw mut sd)
        == 0 as core::ffi::c_int
    {
        yuvconfig2image(&raw mut (*ctx).preview_img, &raw mut sd, NULL);
        return &raw mut (*ctx).preview_img;
    } else {
        return 0 as *mut aom_image_t
    };
}
unsafe extern "C-unwind" fn ctrl_use_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let reference_flag: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    av1_use_as_reference(
        &raw mut (*(*(*ctx).ppi).cpi).ext_flags.ref_frame_flags,
        reference_flag,
    );
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_roi_map(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut data: *mut aom_roi_map_t = args.next_arg::<*mut aom_roi_map_t>();
    if !data.is_null() {
        let mut roi: *mut aom_roi_map_t = data;
        return av1_set_roi_map(
            (*(*ctx).ppi).cpi as *mut AV1_COMP,
            (*roi).roi_map,
            (*roi).rows,
            (*roi).cols,
            ((*roi).delta_q).as_mut_ptr(),
            ((*roi).delta_lf).as_mut_ptr(),
            ((*roi).skip).as_mut_ptr(),
            ((*roi).ref_frame).as_mut_ptr(),
        ) as aom_codec_err_t;
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_set_active_map(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let map: *mut aom_active_map_t = args.next_arg::<*mut aom_active_map_t>();
    if !map.is_null() {
        if av1_set_active_map(
            (*(*ctx).ppi).cpi as *mut AV1_COMP,
            (*map).active_map,
            (*map).rows as core::ffi::c_int,
            (*map).cols as core::ffi::c_int,
        ) == 0
        {
            return AOM_CODEC_OK
        } else {
            return AOM_CODEC_INVALID_PARAM
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_active_map(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let map: *mut aom_active_map_t = args.next_arg::<*mut aom_active_map_t>();
    if !map.is_null() {
        if av1_get_active_map(
            (*(*ctx).ppi).cpi as *mut AV1_COMP,
            (*map).active_map,
            (*map).rows as core::ffi::c_int,
            (*map).cols as core::ffi::c_int,
        ) == 0
        {
            return AOM_CODEC_OK
        } else {
            return AOM_CODEC_INVALID_PARAM
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_set_scale_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mode: *mut aom_scaling_mode_t = args.next_arg::<*mut aom_scaling_mode_t>();
    if !mode.is_null() {
        let oxcf: *mut AV1EncoderConfig = if (*(*ctx).ppi).seq_params_locked != 0 {
            &raw mut (*(*(*ctx).ppi).cpi).oxcf
        } else {
            &raw mut (*ctx).oxcf
        };
        let res: core::ffi::c_int = av1_set_internal_size(
            oxcf,
            &raw mut (*(*(*ctx).ppi).cpi).resize_pending_params,
            (*mode).h_scaling_mode,
            (*mode).v_scaling_mode,
        ) as core::ffi::c_int;
        if res == 0 as core::ffi::c_int {
            if (*(*ctx).ppi).seq_params_locked != 0 {
                av1_check_fpmt_config((*ctx).ppi, &raw mut (*(*(*ctx).ppi).cpi).oxcf);
                return AOM_CODEC_OK;
            }
            return update_encoder_cfg(ctx);
        }
        return AOM_CODEC_INVALID_PARAM;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_set_spatial_layer_id(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let spatial_layer_id: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    if spatial_layer_id < 0 as core::ffi::c_int
        || spatial_layer_id >= (*(*ctx).ppi).number_spatial_layers as core::ffi::c_int
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*(*(*ctx).ppi).cpi).common.spatial_layer_id = spatial_layer_id;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_number_spatial_layers(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let number_spatial_layers: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    if number_spatial_layers <= 0 as core::ffi::c_int
        || number_spatial_layers > MAX_NUM_SPATIAL_LAYERS || (*(*ctx).ppi).use_svc != 0
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*(*ctx).ppi).number_spatial_layers = number_spatial_layers as core::ffi::c_uint;
    (*(*(*ctx).ppi).cpi).common.spatial_layer_id = clamp(
        (*(*(*ctx).ppi).cpi).common.spatial_layer_id,
        0 as core::ffi::c_int,
        number_spatial_layers - 1 as core::ffi::c_int,
    );
    if (*(*ctx).ppi).seq_params_locked == 0 {
        return update_encoder_cfg(ctx);
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_layer_id(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let data: *mut aom_svc_layer_id_t = args.next_arg::<*mut aom_svc_layer_id_t>();
    if (*data).spatial_layer_id < 0 as core::ffi::c_int
        || (*data).temporal_layer_id < 0 as core::ffi::c_int
        || (*data).spatial_layer_id
            >= (*(*ctx).ppi).number_spatial_layers as core::ffi::c_int
        || (*data).temporal_layer_id
            >= (*(*ctx).ppi).number_temporal_layers as core::ffi::c_int
        || (*data).spatial_layer_id >= (*(*(*ctx).ppi).cpi).svc.number_spatial_layers
        || (*data).temporal_layer_id >= (*(*(*ctx).ppi).cpi).svc.number_temporal_layers
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*(*(*ctx).ppi).cpi).common.spatial_layer_id = (*data).spatial_layer_id;
    (*(*(*ctx).ppi).cpi).common.temporal_layer_id = (*data).temporal_layer_id;
    (*(*(*ctx).ppi).cpi).svc.spatial_layer_id = (*data).spatial_layer_id;
    (*(*(*ctx).ppi).cpi).svc.temporal_layer_id = (*data).temporal_layer_id;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn disable_svc(mut ppi: *mut AV1_PRIMARY, mut cpi: *mut AV1_COMP) {
    (*cpi).svc.number_spatial_layers = 1 as core::ffi::c_int;
    (*cpi).svc.number_temporal_layers = 1 as core::ffi::c_int;
    (*ppi).number_spatial_layers = 1 as core::ffi::c_uint;
    (*ppi).number_temporal_layers = 1 as core::ffi::c_uint;
    (*ppi).use_svc = 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn ctrl_set_svc_params(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let ppi: *mut AV1_PRIMARY = (*ctx).ppi;
    let cpi: *mut AV1_COMP = (*ppi).cpi as *mut AV1_COMP;
    let params: *mut aom_svc_params_t = args.next_arg::<*mut aom_svc_params_t>();
    let mut target_bandwidth: int64_t = 0 as int64_t;
    if (*params).number_spatial_layers <= 0 as core::ffi::c_int
        || (*params).number_spatial_layers > MAX_NUM_SPATIAL_LAYERS
        || (*params).number_temporal_layers <= 0 as core::ffi::c_int
        || (*params).number_temporal_layers > MAX_NUM_TEMPORAL_LAYERS
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ppi).number_spatial_layers = (*params).number_spatial_layers as core::ffi::c_uint;
    (*ppi).number_temporal_layers = (*params).number_temporal_layers
        as core::ffi::c_uint;
    (*cpi).svc.number_spatial_layers = (*params).number_spatial_layers;
    (*cpi).svc.number_temporal_layers = (*params).number_temporal_layers;
    if (*cpi).svc.prev_number_temporal_layers != 0
        && (*cpi).svc.prev_number_spatial_layers != 0
        && ((*cpi).svc.number_temporal_layers != (*cpi).svc.prev_number_temporal_layers
            || (*cpi).svc.number_spatial_layers != (*cpi).svc.prev_number_spatial_layers)
    {
        let seq_params: *mut SequenceHeader = &raw mut (*ppi).seq_params;
        (*seq_params).operating_points_cnt_minus_1 = ((*ppi).number_spatial_layers)
            .wrapping_mul((*ppi).number_temporal_layers)
            .wrapping_sub(1 as core::ffi::c_uint) as core::ffi::c_int;
        (*ctx).next_frame_flags |= AOM_EFLAG_FORCE_KF as aom_enc_frame_flags_t;
        av1_set_svc_seq_params(ppi);
        av1_free_svc_cyclic_refresh(cpi as *mut AV1_COMP);
    }
    (*cpi).svc.spatial_layer_id = clamp(
        (*cpi).svc.spatial_layer_id,
        0 as core::ffi::c_int,
        (*cpi).svc.number_spatial_layers - 1 as core::ffi::c_int,
    );
    (*cpi).common.spatial_layer_id = (*cpi).svc.spatial_layer_id;
    (*cpi).svc.temporal_layer_id = clamp(
        (*cpi).svc.temporal_layer_id,
        0 as core::ffi::c_int,
        (*cpi).svc.number_temporal_layers - 1 as core::ffi::c_int,
    );
    (*cpi).common.temporal_layer_id = (*cpi).svc.temporal_layer_id;
    if (*ppi).number_spatial_layers > 1 as core::ffi::c_uint
        || (*ppi).number_temporal_layers > 1 as core::ffi::c_uint
    {
        let mut sl: core::ffi::c_uint = 0;
        let mut tl: core::ffi::c_uint = 0;
        if (*cpi).oxcf.gf_cfg.lag_in_frames > 0 as core::ffi::c_int {
            return AOM_CODEC_INVALID_PARAM;
        }
        let num_layers: core::ffi::c_int = ((*ppi).number_spatial_layers)
            .wrapping_mul((*ppi).number_temporal_layers) as core::ffi::c_int;
        let mut layer: core::ffi::c_int = 0 as core::ffi::c_int;
        while layer < num_layers {
            if (*params).max_quantizers[layer as usize] > 63 as core::ffi::c_int
                || (*params).min_quantizers[layer as usize] < 0 as core::ffi::c_int
                || (*params).min_quantizers[layer as usize]
                    > (*params).max_quantizers[layer as usize]
            {
                disable_svc(ppi, cpi);
                return AOM_CODEC_INVALID_PARAM;
            }
            layer += 1;
        }
        sl = 0 as core::ffi::c_uint;
        while sl < (*ppi).number_spatial_layers {
            if (*params).scaling_factor_den[sl as usize]
                < (*params).scaling_factor_num[sl as usize]
            {
                disable_svc(ppi, cpi);
                return AOM_CODEC_INVALID_PARAM;
            }
            sl = sl.wrapping_add(1);
        }
        if !av1_alloc_layer_context(cpi as *mut AV1_COMP, num_layers) {
            return AOM_CODEC_MEM_ERROR;
        }
        (*ppi).use_svc = 1 as core::ffi::c_int;
        sl = 0 as core::ffi::c_uint;
        while sl < (*ppi).number_spatial_layers {
            tl = 0 as core::ffi::c_uint;
            while tl < (*ppi).number_temporal_layers {
                let layer_0: core::ffi::c_int = sl
                    .wrapping_mul((*ppi).number_temporal_layers)
                    .wrapping_add(tl) as core::ffi::c_int;
                let mut lc: *mut LAYER_CONTEXT = &raw mut *((*cpi).svc.layer_context)
                    .offset(layer_0 as isize) as *mut LAYER_CONTEXT;
                (*lc).max_q = (*params).max_quantizers[layer_0 as usize];
                (*lc).min_q = (*params).min_quantizers[layer_0 as usize];
                (*lc).scaling_factor_num = if 1 as core::ffi::c_int
                    > (*params).scaling_factor_num[sl as usize]
                {
                    1 as core::ffi::c_int
                } else {
                    (*params).scaling_factor_num[sl as usize]
                };
                (*lc).scaling_factor_den = if 1 as core::ffi::c_int
                    > (*params).scaling_factor_den[sl as usize]
                {
                    1 as core::ffi::c_int
                } else {
                    (*params).scaling_factor_den[sl as usize]
                };
                let layer_target_bitrate: core::ffi::c_int = (*params)
                    .layer_target_bitrate[layer_0 as usize];
                if layer_target_bitrate > INT_MAX / 1000 as core::ffi::c_int {
                    (*lc).layer_target_bitrate = INT_MAX as int64_t;
                } else {
                    (*lc).layer_target_bitrate = (1000 as core::ffi::c_int
                        * layer_target_bitrate) as int64_t;
                }
                (*lc).framerate_factor = if 1 as core::ffi::c_int
                    > (*params).framerate_factor[tl as usize]
                {
                    1 as core::ffi::c_int
                } else {
                    (*params).framerate_factor[tl as usize]
                };
                if tl
                    == ((*ppi).number_temporal_layers)
                        .wrapping_sub(1 as core::ffi::c_uint)
                {
                    ::core::ptr::write_volatile(
                        &raw mut target_bandwidth as *mut int64_t,
                        ::core::ptr::read_volatile::<
                            int64_t,
                        >(&target_bandwidth as *const int64_t)
                            + (*lc).layer_target_bitrate,
                    );
                }
                tl = tl.wrapping_add(1);
            }
            sl = sl.wrapping_add(1);
        }
        if (*ppi).seq_params_locked != 0 {
            let oxcf: *mut AV1EncoderConfig = &raw mut (*cpi).oxcf;
            (*oxcf).rc_cfg.target_bandwidth = target_bandwidth;
            (*ctx).oxcf.rc_cfg.target_bandwidth = (*oxcf).rc_cfg.target_bandwidth;
            set_primary_rc_buffer_sizes(oxcf, ppi);
            let enable_setjmp_error: *mut aom_internal_error_info = (*cpi).common.error;
            loop {
match crate::jump::scope((((*enable_setjmp_error).jmp).as_mut_ptr()).cast(), || {

            (*enable_setjmp_error).setjmp = 1 as core::ffi::c_int;
            av1_update_layer_context_change_config(
                cpi as *mut AV1_COMP,
                target_bandwidth,
            );
            let enable_setjmp_error_0: *mut aom_internal_error_info = (*cpi)
                .common
                .error;
            (*enable_setjmp_error_0).setjmp = 0 as core::ffi::c_int;
            check_reset_rc_flag(cpi);
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
                (*enable_setjmp_error).setjmp = 0 as core::ffi::c_int;
                (*ctx).base.err_detail = if (*enable_setjmp_error).has_detail != 0 {
                    ((*enable_setjmp_error).detail).as_mut_ptr()
                } else {
                    0 as *mut core::ffi::c_char
                };
                return (*enable_setjmp_error).error_code;}
}
}} else {
            (*cpi).oxcf.rc_cfg.target_bandwidth = target_bandwidth;
            (*ctx).oxcf.rc_cfg.target_bandwidth = (*cpi).oxcf.rc_cfg.target_bandwidth;
            let seq_params_0: *mut SequenceHeader = &raw mut (*ppi).seq_params;
            (*seq_params_0).operating_points_cnt_minus_1 = ((*ppi).number_spatial_layers)
                .wrapping_mul((*ppi).number_temporal_layers)
                .wrapping_sub(1 as core::ffi::c_uint) as core::ffi::c_int;
            let enable_setjmp_error_1: *mut aom_internal_error_info = (*cpi)
                .common
                .error;
            loop {
match crate::jump::value_scope((((*enable_setjmp_error_1).jmp).as_mut_ptr()).cast(), || {
            (*enable_setjmp_error_1).setjmp = 1 as core::ffi::c_int;
            av1_init_layer_context(cpi as *mut AV1_COMP);
            let enable_setjmp_error_2: *mut aom_internal_error_info = (*cpi)
                .common
                .error;
            (*enable_setjmp_error_2).setjmp = 0 as core::ffi::c_int;
            return update_encoder_cfg(ctx);}) {
Some(value) => return value,
None => {
                (*enable_setjmp_error_1).setjmp = 0 as core::ffi::c_int;
                (*ctx).base.err_detail = if (*enable_setjmp_error_1).has_detail != 0 {
                    ((*enable_setjmp_error_1).detail).as_mut_ptr()
                } else {
                    0 as *mut core::ffi::c_char
                };
                return (*enable_setjmp_error_1).error_code;}
}
}}
    } else if (*ppi).seq_params_locked == 0 {
        return update_encoder_cfg(ctx)
    }
    av1_check_fpmt_config((*ctx).ppi, &raw mut (*(*(*ctx).ppi).cpi).oxcf);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_svc_ref_frame_config(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let cpi: *mut AV1_COMP = (*(*ctx).ppi).cpi as *mut AV1_COMP;
    let data: *mut aom_svc_ref_frame_config_t = args
        .next_arg::<*mut aom_svc_ref_frame_config_t>();
    (*(*cpi).ppi).rtc_ref.set_ref_frame_config = 1 as core::ffi::c_int;
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int as core::ffi::c_uint {
        if (*data).reference[i as usize] != 0 as core::ffi::c_int
            && (*data).reference[i as usize] != 1 as core::ffi::c_int
        {
            return AOM_CODEC_INVALID_PARAM;
        }
        if (*data).ref_idx[i as usize] > 7 as core::ffi::c_int
            || (*data).ref_idx[i as usize] < 0 as core::ffi::c_int
        {
            return AOM_CODEC_INVALID_PARAM;
        }
        (*(*cpi).ppi).rtc_ref.reference[i as usize] = (*data).reference[i as usize];
        (*(*cpi).ppi).rtc_ref.ref_idx[i as usize] = (*data).ref_idx[i as usize];
        i = i.wrapping_add(1);
    }
    let mut i_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while i_0 < REF_FRAMES as core::ffi::c_int as core::ffi::c_uint {
        if (*data).refresh[i_0 as usize] != 0 as core::ffi::c_int
            && (*data).refresh[i_0 as usize] != 1 as core::ffi::c_int
        {
            return AOM_CODEC_INVALID_PARAM;
        }
        (*(*cpi).ppi).rtc_ref.refresh[i_0 as usize] = (*data).refresh[i_0 as usize];
        i_0 = i_0.wrapping_add(1);
    }
    (*cpi).svc.use_flexible_mode = 1 as core::ffi::c_int;
    (*cpi).svc.ksvc_fixed_mode = 0 as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_svc_ref_frame_comp_pred(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let cpi: *mut AV1_COMP = (*(*ctx).ppi).cpi as *mut AV1_COMP;
    let data: *mut aom_svc_ref_frame_comp_pred_t = args
        .next_arg::<*mut aom_svc_ref_frame_comp_pred_t>();
    (*(*cpi).ppi).rtc_ref.ref_frame_comp[0 as core::ffi::c_int as usize] = (*data)
        .use_comp_pred[0 as core::ffi::c_int as usize];
    (*(*cpi).ppi).rtc_ref.ref_frame_comp[1 as core::ffi::c_int as usize] = (*data)
        .use_comp_pred[1 as core::ffi::c_int as usize];
    (*(*cpi).ppi).rtc_ref.ref_frame_comp[2 as core::ffi::c_int as usize] = (*data)
        .use_comp_pred[2 as core::ffi::c_int as usize];
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_tune_content(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.content = args.next_arg::<aom_codec_control_type_AV1E_SET_TUNE_CONTENT>()
        as aom_tune_content;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_cdf_update_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.cdf_update_mode = args
        .next_arg::<aom_codec_control_type_AV1E_SET_CDF_UPDATE_MODE>() as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_color_primaries(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.color_primaries = args
        .next_arg::<aom_codec_control_type_AV1E_SET_COLOR_PRIMARIES>()
        as aom_color_primaries_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_transfer_characteristics(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.transfer_characteristics = args
        .next_arg::<aom_codec_control_type_AV1E_SET_TRANSFER_CHARACTERISTICS>()
        as aom_transfer_characteristics_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_matrix_coefficients(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.matrix_coefficients = args
        .next_arg::<aom_codec_control_type_AV1E_SET_MATRIX_COEFFICIENTS>()
        as aom_matrix_coefficients_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_chroma_sample_position(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.chroma_sample_position = args
        .next_arg::<aom_codec_control_type_AV1E_SET_CHROMA_SAMPLE_POSITION>()
        as aom_chroma_sample_position_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_color_range(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.color_range = args.next_arg::<aom_codec_control_type_AV1E_SET_COLOR_RANGE>()
        as core::ffi::c_int;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_render_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let render_size: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    extra_cfg.render_width = *render_size.offset(0 as core::ffi::c_int as isize);
    extra_cfg.render_height = *render_size.offset(1 as core::ffi::c_int as isize);
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_superblock_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.superblock_size = args
        .next_arg::<aom_codec_control_type_AV1E_SET_SUPERBLOCK_SIZE>()
        as aom_superblock_size_t;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_chroma_subsampling_x(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.chroma_subsampling_x = args
        .next_arg::<aom_codec_control_type_AV1E_SET_CHROMA_SUBSAMPLING_X>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_chroma_subsampling_y(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    extra_cfg.chroma_subsampling_y = args
        .next_arg::<aom_codec_control_type_AV1E_SET_CHROMA_SUBSAMPLING_Y>()
        as core::ffi::c_uint;
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_set_external_rate_control(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut funcs: aom_rc_funcs_t = *args
        .next_arg::<aom_codec_control_type_AV1E_SET_EXTERNAL_RATE_CONTROL>();
    let mut cpi: *mut AV1_COMP = (*(*ctx).ppi).cpi as *mut AV1_COMP;
    let mut ext_ratectrl: *mut AOM_EXT_RATECTRL = &raw mut (*cpi).ext_ratectrl;
    let mut oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    if (*oxcf).pass as core::ffi::c_uint
        == AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        let mut frame_info: *const FRAME_INFO = &raw mut (*cpi).frame_info;
        let mut ratectrl_config: aom_rc_config_t = aom_rc_config {
            frame_width: 0,
            frame_height: 0,
            show_frame_count: 0,
            max_gf_interval: 0,
            min_gf_interval: 0,
            target_bitrate_kbps: 0,
            frame_rate_num: 0,
            frame_rate_den: 0,
            rc_mode: AOM_RC_QMODE,
            overshoot_percent: 0,
            undershoot_percent: 0,
            min_base_q_index: 0,
            max_base_q_index: 0,
            base_qp: 0,
            superblock_size: 0,
        };
        let mut codec_status: aom_codec_err_t = AOM_CODEC_OK;
        memset(
            &raw mut ratectrl_config as *mut aom_rc_config_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_rc_config_t>() as size_t,
        );
        ratectrl_config.frame_width = (*frame_info).frame_width;
        ratectrl_config.frame_height = (*frame_info).frame_height;
        ratectrl_config.show_frame_count = (*(*cpi).ppi)
            .twopass
            .firstpass_info
            .stats_count;
        ratectrl_config.max_gf_interval = (*ctx).extra_cfg.max_gf_interval
            as core::ffi::c_int;
        ratectrl_config.min_gf_interval = (*ctx).extra_cfg.min_gf_interval
            as core::ffi::c_int;
        ratectrl_config.target_bitrate_kbps = ((*oxcf).rc_cfg.target_bandwidth
            / 1000 as int64_t) as core::ffi::c_int;
        ratectrl_config.frame_rate_num = (*ctx).cfg.g_timebase.den;
        ratectrl_config.frame_rate_den = (*ctx).cfg.g_timebase.num;
        ratectrl_config.overshoot_percent = (*oxcf).rc_cfg.over_shoot_pct;
        ratectrl_config.undershoot_percent = (*oxcf).rc_cfg.under_shoot_pct;
        ratectrl_config.min_base_q_index = (*oxcf).rc_cfg.best_allowed_q;
        ratectrl_config.max_base_q_index = (*oxcf).rc_cfg.worst_allowed_q;
        ratectrl_config.base_qp = (*ctx).extra_cfg.cq_level as core::ffi::c_int;
        let sb_size: BLOCK_SIZE = av1_select_sb_size(
            oxcf,
            (*frame_info).frame_width,
            (*frame_info).frame_height,
            (*(*cpi).ppi).number_spatial_layers as core::ffi::c_int,
        ) as BLOCK_SIZE;
        ratectrl_config.superblock_size = if sb_size as core::ffi::c_int
            == BLOCK_128X128 as core::ffi::c_int
        {
            128 as core::ffi::c_int
        } else {
            64 as core::ffi::c_int
        };
        if (*ctx).cfg.rc_end_usage as core::ffi::c_uint
            == AOM_VBR as core::ffi::c_int as core::ffi::c_uint
        {
            ratectrl_config.rc_mode = AOM_RC_VBR;
        } else if (*ctx).cfg.rc_end_usage as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            ratectrl_config.rc_mode = AOM_RC_QMODE;
        } else if (*ctx).cfg.rc_end_usage as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        {
            ratectrl_config.rc_mode = AOM_RC_CQ;
        }
        codec_status = av1_extrc_create(funcs, ratectrl_config, ext_ratectrl);
        if codec_status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return codec_status;
        }
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn encoder_set_option(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut name: *const core::ffi::c_char,
    mut value: *const core::ffi::c_char,
) -> aom_codec_err_t {
    if ctx.is_null() || name.is_null() || value.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut extra_cfg: av1_extracfg = (*ctx).extra_cfg;
    let mut argv: [*mut core::ffi::c_char; 2] = [
        0 as *mut core::ffi::c_char,
        b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    ];
    let mut len: size_t = (strlen(name))
        .wrapping_add(strlen(value))
        .wrapping_add(4 as size_t);
    let err_string: *mut core::ffi::c_char = ((*(*ctx).ppi).error.detail).as_mut_ptr();
    argv[0 as core::ffi::c_int as usize] = aom_malloc(
        len.wrapping_mul(::core::mem::size_of::<core::ffi::c_char>() as size_t),
    ) as *mut core::ffi::c_char;
    if (argv[0 as core::ffi::c_int as usize]).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    snprintf(
        argv[0 as core::ffi::c_int as usize],
        len,
        b"--%s=%s\0" as *const u8 as *const core::ffi::c_char,
        name,
        value,
    );
    let mut arg: arg = arg {
        argv: 0 as *mut *mut core::ffi::c_char,
        name: 0 as *const core::ffi::c_char,
        val: 0 as *const core::ffi::c_char,
        argv_step: 0,
        def: 0 as *const arg_def,
    };
    let mut err: aom_codec_err_t = AOM_CODEC_OK;
    let mut match_0: core::ffi::c_int = 1 as core::ffi::c_int;
    if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_keyframe_filtering,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_keyframe_filtering = arg_parse_uint_helper(
            &raw mut arg,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.min_gf_interval,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.min_gf_interval = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.max_gf_interval,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.max_gf_interval = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.gf_min_pyr_height,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.gf_min_pyr_height = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.gf_max_pyr_height,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.gf_max_pyr_height = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.cpu_used_av1,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.cpu_used = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.auto_altref,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_auto_alt_ref = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.noise_sens,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.noise_sensitivity = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.sharpness,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.sharpness = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_adaptive_sharpness,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_adaptive_sharpness = arg_parse_int_helper(&raw mut arg, err_string)
            as core::ffi::c_uint;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.static_thresh,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.static_thresh = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.rowmtarg,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.row_mt = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.fpmtarg,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.fp_mt = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tile_cols,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.tile_columns = arg_parse_uint_helper(&raw mut arg, err_string);
        if extra_cfg.auto_tiles != 0 {
            snprintf(
                err_string,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Cannot set tile-cols because auto-tiles is already set.\0" as *const u8
                    as *const core::ffi::c_char,
            );
            err = AOM_CODEC_INVALID_PARAM;
        }
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tile_rows,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.tile_rows = arg_parse_uint_helper(&raw mut arg, err_string);
        if extra_cfg.auto_tiles != 0 {
            snprintf(
                err_string,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Cannot set tile-rows because auto-tiles is already set.\0" as *const u8
                    as *const core::ffi::c_char,
            );
            err = AOM_CODEC_INVALID_PARAM;
        }
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.auto_tiles,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.auto_tiles = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_tpl_model,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_tpl_model = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.arnr_maxframes,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.arnr_max_frames = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.arnr_strength,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.arnr_strength = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tune_metric,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.tuning = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_tune_metric;
        err = handle_tuning(&raw mut extra_cfg);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.partition_info_path,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        err = allocate_and_set_string(
            value,
            default_extra_cfg.partition_info_path,
            &raw mut extra_cfg.partition_info_path,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_rate_guide_deltaq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_rate_guide_deltaq = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.rate_distribution_info,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        err = allocate_and_set_string(
            value,
            default_extra_cfg.rate_distribution_info,
            &raw mut extra_cfg.rate_distribution_info,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.dist_metric,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.dist_metric = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_dist_metric;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.cq_level,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.cq_level = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.max_intra_rate_pct,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.rc_max_intra_bitrate_pct = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.max_inter_rate_pct,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.rc_max_inter_bitrate_pct = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.gf_cbr_boost_pct,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.gf_cbr_boost_pct = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.lossless,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.lossless = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_cdef,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_cdef = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_restoration,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_restoration = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.force_video_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.force_video_mode = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_obmc,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_obmc = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.disable_trellis_quant,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.disable_trellis_quant = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_qm,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_qm = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.qm_max,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.qm_max = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.qm_min,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.qm_min = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.num_tg,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.num_tg = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.mtu_size,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.mtu_size = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.timing_info,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.timing_info_type = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_timing_info_type_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.frame_parallel_decoding,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.frame_parallel_decoding_mode = arg_parse_uint_helper(
            &raw mut arg,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_dual_filter,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_dual_filter = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_chroma_deltaq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_chroma_deltaq = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.aq_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.aq_mode = arg_parse_uint_helper(&raw mut arg, err_string) as AQ_MODE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.deltaq_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.deltaq_mode = arg_parse_uint_helper(&raw mut arg, err_string)
            as DELTAQ_MODE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.deltaq_strength,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.deltaq_strength = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.deltalf_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.deltalf_mode = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.frame_periodic_boost,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.frame_periodic_boost = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tune_content,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.content = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_tune_content;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_color_primaries,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.color_primaries = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_color_primaries_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_transfer_characteristics,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.transfer_characteristics = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_transfer_characteristics_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_matrix_coefficients,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.matrix_coefficients = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_matrix_coefficients_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_chroma_sample_position,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.chroma_sample_position = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_chroma_sample_position_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.superblock_size,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.superblock_size = arg_parse_enum_helper(&raw mut arg, err_string)
            as aom_superblock_size_t;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.error_resilient_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.error_resilient_mode = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.sframe_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.s_frame_mode = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.film_grain_test,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.film_grain_test_vector = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.film_grain_table,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        if value.is_null() {
            extra_cfg.film_grain_table_filename = value;
        } else {
            err = allocate_and_set_string(
                value,
                default_extra_cfg.film_grain_table_filename,
                &raw mut extra_cfg.film_grain_table_filename,
                err_string,
            );
        }
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.cdf_update_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.cdf_update_mode = arg_parse_int_helper(&raw mut arg, err_string)
            as core::ffi::c_uint;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_rect_partitions,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_rect_partitions = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_ab_partitions,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_ab_partitions = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_1to4_partitions,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_1to4_partitions = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.min_partition_size,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.min_partition_size = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.max_partition_size,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.max_partition_size = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_intra_edge_filter,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_intra_edge_filter = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_order_hint,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_order_hint = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_tx64,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_tx64 = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_flip_idtx,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_flip_idtx = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_rect_tx,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_rect_tx = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_dist_wtd_comp,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_dist_wtd_comp = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.max_reference_frames,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.max_reference_frames = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.reduced_reference_set,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_reduced_reference_set = arg_parse_int_helper(
            &raw mut arg,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_ref_frame_mvs,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_ref_frame_mvs = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_masked_comp,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_masked_comp = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_onesided_comp,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_onesided_comp = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_interintra_comp,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_interintra_comp = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_smooth_interintra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_smooth_interintra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_diff_wtd_comp,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_diff_wtd_comp = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_interinter_wedge,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_interinter_wedge = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_interintra_wedge,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_interintra_wedge = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_global_motion,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_global_motion = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_warped_motion,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_warped_motion = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_filter_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_filter_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_smooth_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_smooth_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_paeth_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_paeth_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_cfl_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_cfl_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_directional_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_directional_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_diagonal_intra,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_diagonal_intra = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_overlay,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_overlay = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_palette,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_palette = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_intrabc,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_intrabc = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_angle_delta,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_angle_delta = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_low_complexity_decode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_low_complexity_decode = arg_parse_int_helper(
            &raw mut arg,
            err_string,
        ) as core::ffi::c_uint;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.reduced_tx_type_set,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.reduced_tx_type_set = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.use_intra_dct_only,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.use_intra_dct_only = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.use_inter_dct_only,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.use_inter_dct_only = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.use_intra_default_tx_only,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.use_intra_default_tx_only = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.quant_b_adapt,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.quant_b_adapt = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.vbr_corpus_complexity_lap,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.vbr_corpus_complexity_lap = arg_parse_uint_helper(
            &raw mut arg,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.set_tier_mask,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.tier_mask = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.set_min_cr,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.min_cr = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.coeff_cost_upd_freq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.coeff_cost_upd_freq = arg_parse_uint_helper(&raw mut arg, err_string)
            as COST_UPDATE_TYPE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.mode_cost_upd_freq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.mode_cost_upd_freq = arg_parse_uint_helper(&raw mut arg, err_string)
            as COST_UPDATE_TYPE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.mv_cost_upd_freq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.mv_cost_upd_freq = arg_parse_uint_helper(&raw mut arg, err_string)
            as COST_UPDATE_TYPE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.dv_cost_upd_freq,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.dv_cost_upd_freq = arg_parse_uint_helper(&raw mut arg, err_string)
            as COST_UPDATE_TYPE;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.denoise_noise_level,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.noise_level = arg_parse_int_helper(&raw mut arg, err_string)
            as core::ffi::c_float / 10.0f32;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.denoise_block_size,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.noise_block_size = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.enable_dnl_denoising,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.enable_dnl_denoising = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.target_seq_level_idx,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        let val: core::ffi::c_int = arg_parse_int_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
        let level: core::ffi::c_int = val % 100 as core::ffi::c_int;
        let operating_point_idx: core::ffi::c_int = val / 100 as core::ffi::c_int;
        if operating_point_idx < 0 as core::ffi::c_int
            || operating_point_idx >= MAX_NUM_OPERATING_POINTS
        {
            snprintf(
                err_string,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Invalid operating point index: %d\0" as *const u8
                    as *const core::ffi::c_char,
                operating_point_idx,
            );
            err = AOM_CODEC_INVALID_PARAM;
        } else {
            extra_cfg.target_seq_level_idx[operating_point_idx as usize] = level
                as AV1_LEVEL;
        }
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_chroma_subsampling_x,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.chroma_subsampling_x = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.input_chroma_subsampling_y,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.chroma_subsampling_y = arg_parse_uint_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.passes,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.passes = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.fwd_kf_dist,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.fwd_kf_dist = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.two_pass_output,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        err = allocate_and_set_string(
            value,
            default_extra_cfg.two_pass_output,
            &raw mut extra_cfg.two_pass_output,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.second_pass_log,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        err = allocate_and_set_string(
            value,
            default_extra_cfg.second_pass_log,
            &raw mut extra_cfg.second_pass_log,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.loopfilter_control,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.loopfilter_control = arg_parse_int_helper(&raw mut arg, err_string)
            as LOOPFILTER_CONTROL;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.auto_intra_tools_off,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.auto_intra_tools_off = arg_parse_uint_helper(&raw mut arg, err_string)
            as core::ffi::c_int;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.strict_level_conformance,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.strict_level_conformance = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.sb_qp_sweep,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.sb_qp_sweep = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.kf_max_pyr_height,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.kf_max_pyr_height = arg_parse_int_helper(&raw mut arg, err_string);
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tile_width,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        (*ctx).cfg.tile_width_count = arg_parse_list_helper(
            &raw mut arg,
            ((*ctx).cfg.tile_widths).as_mut_ptr(),
            MAX_TILE_WIDTHS,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.tile_height,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        (*ctx).cfg.tile_height_count = arg_parse_list_helper(
            &raw mut arg,
            ((*ctx).cfg.tile_heights).as_mut_ptr(),
            MAX_TILE_HEIGHTS,
            err_string,
        );
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.screen_detection_mode,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.screen_detection_mode = arg_parse_int_helper(&raw mut arg, err_string)
            as aom_screen_detection_mode;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.validate_hbd_input,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.validate_hbd_input = arg_parse_int_helper(&raw mut arg, err_string)
            as core::ffi::c_uint;
    } else if arg_match_helper(
        &raw mut arg,
        &g_av1_codec_arg_defs.force_max_q,
        argv.as_mut_ptr(),
        err_string,
    ) != 0
    {
        extra_cfg.force_max_q = arg_parse_uint_helper(&raw mut arg, err_string);
    } else {
        match_0 = 0 as core::ffi::c_int;
        snprintf(
            err_string,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Cannot find aom option %s\0" as *const u8 as *const core::ffi::c_char,
            name,
        );
    }
    aom_free(argv[0 as core::ffi::c_int as usize] as *mut core::ffi::c_void);
    if err as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*ctx).base.err_detail = err_string;
        return err;
    }
    if strlen(err_string) != 0 as size_t {
        (*ctx).base.err_detail = err_string;
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ctx).base.err_detail = 0 as *const core::ffi::c_char;
    if match_0 == 0 {
        return AOM_CODEC_INVALID_PARAM;
    }
    return update_extra_cfg(ctx, &raw mut extra_cfg);
}
unsafe extern "C-unwind" fn ctrl_get_seq_level_idx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    return av1_get_seq_level_idx(
        &raw mut (*(*ctx).ppi).seq_params,
        &raw mut (*(*ctx).ppi).level_params,
        arg,
    );
}
unsafe extern "C-unwind" fn ctrl_get_target_seq_level_idx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    return av1_get_target_seq_level_idx(
        &raw mut (*(*ctx).ppi).seq_params,
        &raw mut (*(*ctx).ppi).level_params,
        arg,
    );
}
unsafe extern "C-unwind" fn ctrl_get_num_operating_points(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = (*(*ctx).ppi).seq_params.operating_points_cnt_minus_1 + 1 as core::ffi::c_int;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_luma_cdef_strength(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    let mut cm: *const AV1_COMMON = &raw mut (*(*(*ctx).ppi).cpi).common;
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    memcpy(
        arg as *mut core::ffi::c_void,
        ((*cm).cdef_info.cdef_strengths).as_ptr() as *const core::ffi::c_void,
        (CDEF_MAX_STRENGTHS as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    );
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_high_motion_content_screen_rtc(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: va_list,
) -> aom_codec_err_t {
    let mut arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    let cpi: *mut AV1_COMP = (*(*ctx).ppi).cpi as *mut AV1_COMP;
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    *arg = (*cpi).rc.high_motion_content_screen_rtc;
    return AOM_CODEC_OK;
}
static mut encoder_ctrl_maps: [aom_codec_ctrl_fn_map_t; 169] = unsafe {
    [
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_COPY_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_copy_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_USE_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_use_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ROI_MAP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_roi_map
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ACTIVEMAP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_active_map
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_SCALEMODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_scale_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_SPATIAL_LAYER_ID as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_spatial_layer_id
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_CPUUSED as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_cpuused
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ENABLEAUTOALTREF as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_auto_alt_ref
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ENABLEAUTOBWDREF as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_auto_bwd_ref
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_SHARPNESS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_sharpness
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_STATIC_THRESHOLD as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_static_thresh
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ROW_MT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_row_mt
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FP_MT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_fp_mt
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TILE_COLUMNS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tile_columns
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TILE_ROWS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tile_rows
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_TPL_MODEL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_tpl_model
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_KEYFRAME_FILTERING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_keyframe_filtering
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ARNR_MAXFRAMES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_arnr_max_frames
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_ARNR_STRENGTH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_arnr_strength
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_TUNING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tuning
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_CQ_LEVEL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_cq_level
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_MAX_INTRA_BITRATE_PCT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_rc_max_intra_bitrate_pct
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_NUMBER_SPATIAL_LAYERS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_number_spatial_layers
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_INTER_BITRATE_PCT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_rc_max_inter_bitrate_pct
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_GF_CBR_BOOST_PCT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_rc_gf_cbr_boost_pct
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_LOSSLESS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_lossless
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_CDEF as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_cdef
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_RESTORATION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_restoration
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FORCE_VIDEO_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_force_video_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_OBMC as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_obmc
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DISABLE_TRELLIS_QUANT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_disable_trellis_quant
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_QM as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_qm
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QM_Y as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_qm_y
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QM_U as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_qm_u
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QM_V as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_qm_v
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QM_MIN as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_qm_min
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QM_MAX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_qm_max
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_NUM_TG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_num_tg
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MTU as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_mtu
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TIMING_INFO_TYPE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_timing_info_type
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FRAME_PARALLEL_DECODING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_frame_parallel_decoding_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ERROR_RESILIENT_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_error_resilient_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_S_FRAME_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_s_frame_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_RECT_PARTITIONS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_rect_partitions
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_AB_PARTITIONS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_ab_partitions
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_1TO4_PARTITIONS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_1to4_partitions
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MIN_PARTITION_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_min_partition_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_PARTITION_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_max_partition_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DUAL_FILTER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_dual_filter
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_CHROMA_DELTAQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_chroma_deltaq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_INTRA_EDGE_FILTER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_intra_edge_filter
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_ORDER_HINT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_order_hint
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_TX64 as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_tx64
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_FLIP_IDTX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_flip_idtx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_RECT_TX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_rect_tx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DIST_WTD_COMP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_dist_wtd_comp
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_REFERENCE_FRAMES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_max_reference_frames
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_REDUCED_REFERENCE_SET as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_reduced_reference_set
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_REF_FRAME_MVS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_ref_frame_mvs
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ALLOW_REF_FRAME_MVS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_allow_ref_frame_mvs
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_MASKED_COMP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_masked_comp
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_ONESIDED_COMP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_onesided_comp
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_INTERINTRA_COMP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_interintra_comp
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_SMOOTH_INTERINTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_smooth_interintra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DIFF_WTD_COMP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_diff_wtd_comp
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_INTERINTER_WEDGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_interinter_wedge
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_INTERINTRA_WEDGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_interintra_wedge
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_GLOBAL_MOTION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_global_motion
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_WARPED_MOTION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_warped_motion
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ALLOW_WARPED_MOTION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_allow_warped_motion
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_FILTER_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_filter_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_SMOOTH_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_smooth_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_PAETH_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_paeth_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_CFL_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_cfl_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DIRECTIONAL_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_directional_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DIAGONAL_INTRA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_diagonal_intra
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_SUPERRES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_superres
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_OVERLAY as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_overlay
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_PALETTE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_palette
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_INTRABC as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_intrabc
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_ANGLE_DELTA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_angle_delta
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_AQ_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_aq_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_REDUCED_TX_TYPE_SET as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_reduced_tx_type_set
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_INTRA_DCT_ONLY as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_intra_dct_only
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_INTER_DCT_ONLY as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_inter_dct_only
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_INTRA_DEFAULT_TX_ONLY as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_intra_default_tx_only
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QUANT_B_ADAPT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_quant_b_adapt
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_COEFF_COST_UPD_FREQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_coeff_cost_upd_freq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MODE_COST_UPD_FREQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_mode_cost_upd_freq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MV_COST_UPD_FREQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_mv_cost_upd_freq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DELTAQ_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_deltaq_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DELTAQ_STRENGTH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_deltaq_strength
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DELTALF_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_deltalf_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FRAME_PERIODIC_BOOST as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_frame_periodic_boost
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TUNE_CONTENT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tune_content
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_CDF_UPDATE_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_cdf_update_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_COLOR_PRIMARIES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_color_primaries
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TRANSFER_CHARACTERISTICS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_transfer_characteristics
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MATRIX_COEFFICIENTS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_matrix_coefficients
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_CHROMA_SAMPLE_POSITION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_chroma_sample_position
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_COLOR_RANGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_color_range
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_NOISE_SENSITIVITY as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_noise_sensitivity
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MIN_GF_INTERVAL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_min_gf_interval
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_GF_INTERVAL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_max_gf_interval
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_GF_MIN_PYRAMID_HEIGHT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_gf_min_pyr_height
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_GF_MAX_PYRAMID_HEIGHT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_gf_max_pyr_height
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_RENDER_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_render_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SUPERBLOCK_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_superblock_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SINGLE_TILE_DECODING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_single_tile_decoding
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_VMAF_MODEL_PATH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_vmaf_model_path
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_PARTITION_INFO_PATH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_partition_info_path
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_ENABLE_RATE_GUIDE_DELTAQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_rate_guide_deltaq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_RATE_DISTRIBUTION_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_rate_distribution_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FILM_GRAIN_TEST_VECTOR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_film_grain_test_vector
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FILM_GRAIN_TABLE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_film_grain_table
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DENOISE_NOISE_LEVEL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_denoise_noise_level
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DENOISE_BLOCK_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_denoise_block_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_DNL_DENOISING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_dnl_denoising
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_ENABLE_MOTION_VECTOR_UNIT_TEST as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_motion_vector_unit_test
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_FP_MT_UNIT_TEST as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_fpmt_unit_test
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_ENABLE_EXT_TILE_DEBUG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_ext_tile_debug
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TARGET_SEQ_LEVEL_IDX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_target_seq_level_idx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_TIER_MASK as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tier_mask
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MIN_CR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_min_cr
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SVC_LAYER_ID as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_layer_id
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SVC_PARAMS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_svc_params
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SVC_REF_FRAME_CONFIG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_svc_ref_frame_config
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SVC_REF_FRAME_COMP_PRED as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_svc_ref_frame_comp_pred
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_VBR_CORPUS_COMPLEXITY_LAP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_vbr_corpus_complexity_lap
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_ENABLE_SB_MULTIPASS_UNIT_TEST as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_sb_multipass_unit_test
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_ENABLE_SB_QP_SWEEP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_enable_sb_qp_sweep
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_DV_COST_UPD_FREQ as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_dv_cost_upd_freq
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_EXTERNAL_PARTITION as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_external_partition
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_TX_SIZE_SEARCH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_tx_size_search
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_LOOPFILTER_CONTROL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_loopfilter_control
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SKIP_POSTPROC_FILTERING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_skip_postproc_filtering
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MODE_REF_DELTA_ENABLED as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_mode_ref_delta_enabled
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_AUTO_INTRA_TOOLS_OFF as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_auto_intra_tools_off
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_RTC_EXTERNAL_RC as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_rtc_external_rc
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_QUANTIZER_ONE_PASS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_quantizer_one_pass
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_BITRATE_ONE_PASS_CBR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_bitrate_one_pass_cbr
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_CONSEC_FRAME_DROP_CBR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_max_consec_frame_drop_cbr
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SVC_FRAME_DROP_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_svc_frame_drop_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_AUTO_TILES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_auto_tiles
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_POSTENCODE_DROP_RTC as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_postencode_drop_rtc
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_MAX_CONSEC_FRAME_DROP_MS_CBR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_max_consec_frame_drop_ms_cbr
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_LOW_COMPLEXITY_DECODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_low_complexity_decode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_SCREEN_CONTENT_DETECTION_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_screen_content_detection_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_ENABLE_ADAPTIVE_SHARPNESS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_enable_adaptive_sharpness
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_EXTERNAL_RATE_CONTROL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_external_rate_control
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_SET_VALIDATE_HBD_INPUT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_validate_hbd_input
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_GET_LAST_QUANTIZER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_quantizer
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_GET_LAST_QUANTIZER_64 as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_quantizer64
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOME_GET_LOOPFILTER_LEVEL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_loopfilter_level
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_GET_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_ACTIVEMAP as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_active_map
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_GET_NEW_FRAME_IMAGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_new_frame_image
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_COPY_NEW_FRAME_IMAGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_copy_new_frame_image
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_CHROMA_SUBSAMPLING_X as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_chroma_subsampling_x
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_SET_CHROMA_SUBSAMPLING_Y as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_chroma_subsampling_y
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_SEQ_LEVEL_IDX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_seq_level_idx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_BASELINE_GF_INTERVAL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_baseline_gf_interval
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_TARGET_SEQ_LEVEL_IDX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_target_seq_level_idx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_NUM_OPERATING_POINTS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_num_operating_points
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_LUMA_CDEF_STRENGTH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_luma_cdef_strength
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_HIGH_MOTION_CONTENT_SCREEN_RTC as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_high_motion_content_screen_rtc
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1E_GET_GOP_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_gop_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            va_list,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: 0 as core::ffi::c_int,
                fn_0: None,
            };
            init
        },
    ]
};
static mut encoder_usage_cfg: [aom_codec_enc_cfg_t; 3] = [
    {
        let mut init = aom_codec_enc_cfg {
            g_usage: AOM_USAGE_GOOD_QUALITY,
            g_threads: 0 as core::ffi::c_uint,
            g_profile: 0 as core::ffi::c_uint,
            g_w: 320 as core::ffi::c_uint,
            g_h: 240 as core::ffi::c_uint,
            g_limit: 0 as core::ffi::c_uint,
            g_forced_max_frame_width: 0 as core::ffi::c_uint,
            g_forced_max_frame_height: 0 as core::ffi::c_uint,
            g_bit_depth: AOM_BITS_8,
            g_input_bit_depth: 8 as core::ffi::c_uint,
            g_timebase: {
                let mut init = aom_rational {
                    num: 1 as core::ffi::c_int,
                    den: 30 as core::ffi::c_int,
                };
                init
            },
            g_error_resilient: 0 as aom_codec_er_flags_t,
            g_pass: AOM_RC_ONE_PASS,
            g_lag_in_frames: 35 as core::ffi::c_uint,
            rc_dropframe_thresh: 0 as core::ffi::c_uint,
            rc_resize_mode: RESIZE_NONE as core::ffi::c_int as core::ffi::c_uint,
            rc_resize_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_resize_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_mode: AOM_SUPERRES_NONE,
            rc_superres_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_qthresh: 63 as core::ffi::c_uint,
            rc_superres_kf_qthresh: 32 as core::ffi::c_uint,
            rc_end_usage: AOM_VBR,
            rc_twopass_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_firstpass_mb_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_target_bitrate: 256 as core::ffi::c_uint,
            rc_min_quantizer: 0 as core::ffi::c_uint,
            rc_max_quantizer: 63 as core::ffi::c_uint,
            rc_undershoot_pct: 25 as core::ffi::c_uint,
            rc_overshoot_pct: 25 as core::ffi::c_uint,
            rc_buf_sz: 6000 as core::ffi::c_uint,
            rc_buf_initial_sz: 4000 as core::ffi::c_uint,
            rc_buf_optimal_sz: 5000 as core::ffi::c_uint,
            rc_2pass_vbr_bias_pct: 50 as core::ffi::c_uint,
            rc_2pass_vbr_minsection_pct: 0 as core::ffi::c_uint,
            rc_2pass_vbr_maxsection_pct: 2000 as core::ffi::c_uint,
            fwd_kf_enabled: 0 as core::ffi::c_int,
            kf_mode: AOM_KF_AUTO,
            kf_min_dist: 0 as core::ffi::c_uint,
            kf_max_dist: 9999 as core::ffi::c_uint,
            sframe_dist: 0 as core::ffi::c_uint,
            sframe_mode: 1 as core::ffi::c_uint,
            large_scale_tile: 0 as core::ffi::c_uint,
            monochrome: 0 as core::ffi::c_uint,
            full_still_picture_hdr: 0 as core::ffi::c_uint,
            save_as_annexb: 0 as core::ffi::c_uint,
            tile_width_count: 0 as core::ffi::c_int,
            tile_height_count: 0 as core::ffi::c_int,
            tile_widths: [0 as core::ffi::c_int; 64],
            tile_heights: [0 as core::ffi::c_int; 64],
            use_fixed_qp_offsets: 0 as core::ffi::c_uint,
            fixed_qp_offsets: [
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
            ],
            encoder_cfg: {
                let mut init = cfg_options {
                    init_by_cfg_file: 0 as core::ffi::c_uint,
                    super_block_size: 128 as core::ffi::c_uint,
                    max_partition_size: 128 as core::ffi::c_uint,
                    min_partition_size: 4 as core::ffi::c_uint,
                    disable_ab_partition_type: 0 as core::ffi::c_uint,
                    disable_rect_partition_type: 0 as core::ffi::c_uint,
                    disable_1to4_partition_type: 0 as core::ffi::c_uint,
                    disable_flip_idtx: 0 as core::ffi::c_uint,
                    disable_cdef: 0 as core::ffi::c_uint,
                    disable_lr: 0 as core::ffi::c_uint,
                    disable_obmc: 0 as core::ffi::c_uint,
                    disable_warp_motion: 0 as core::ffi::c_uint,
                    disable_global_motion: 0 as core::ffi::c_uint,
                    disable_dist_wtd_comp: 0 as core::ffi::c_uint,
                    disable_diff_wtd_comp: 0 as core::ffi::c_uint,
                    disable_inter_intra_comp: 0 as core::ffi::c_uint,
                    disable_masked_comp: 0 as core::ffi::c_uint,
                    disable_one_sided_comp: 0 as core::ffi::c_uint,
                    disable_palette: 0 as core::ffi::c_uint,
                    disable_intrabc: 0 as core::ffi::c_uint,
                    disable_cfl: 0 as core::ffi::c_uint,
                    disable_smooth_intra: 0 as core::ffi::c_uint,
                    disable_filter_intra: 0 as core::ffi::c_uint,
                    disable_dual_filter: 0 as core::ffi::c_uint,
                    disable_intra_angle_delta: 0 as core::ffi::c_uint,
                    disable_intra_edge_filter: 0 as core::ffi::c_uint,
                    disable_tx_64x64: 0 as core::ffi::c_uint,
                    disable_smooth_inter_intra: 0 as core::ffi::c_uint,
                    disable_inter_inter_wedge: 0 as core::ffi::c_uint,
                    disable_inter_intra_wedge: 0 as core::ffi::c_uint,
                    disable_paeth_intra: 0 as core::ffi::c_uint,
                    disable_trellis_quant: 0 as core::ffi::c_uint,
                    disable_ref_frame_mv: 0 as core::ffi::c_uint,
                    reduced_reference_set: 0 as core::ffi::c_uint,
                    reduced_tx_type_set: 0 as core::ffi::c_uint,
                };
                init
            },
        };
        init
    },
    {
        let mut init = aom_codec_enc_cfg {
            g_usage: AOM_USAGE_REALTIME,
            g_threads: 0 as core::ffi::c_uint,
            g_profile: 0 as core::ffi::c_uint,
            g_w: 320 as core::ffi::c_uint,
            g_h: 240 as core::ffi::c_uint,
            g_limit: 0 as core::ffi::c_uint,
            g_forced_max_frame_width: 0 as core::ffi::c_uint,
            g_forced_max_frame_height: 0 as core::ffi::c_uint,
            g_bit_depth: AOM_BITS_8,
            g_input_bit_depth: 8 as core::ffi::c_uint,
            g_timebase: {
                let mut init = aom_rational {
                    num: 1 as core::ffi::c_int,
                    den: 30 as core::ffi::c_int,
                };
                init
            },
            g_error_resilient: 0 as aom_codec_er_flags_t,
            g_pass: AOM_RC_ONE_PASS,
            g_lag_in_frames: 0 as core::ffi::c_uint,
            rc_dropframe_thresh: 0 as core::ffi::c_uint,
            rc_resize_mode: RESIZE_NONE as core::ffi::c_int as core::ffi::c_uint,
            rc_resize_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_resize_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_mode: AOM_SUPERRES_NONE,
            rc_superres_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_qthresh: 63 as core::ffi::c_uint,
            rc_superres_kf_qthresh: 32 as core::ffi::c_uint,
            rc_end_usage: AOM_CBR,
            rc_twopass_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_firstpass_mb_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_target_bitrate: 256 as core::ffi::c_uint,
            rc_min_quantizer: 0 as core::ffi::c_uint,
            rc_max_quantizer: 63 as core::ffi::c_uint,
            rc_undershoot_pct: 50 as core::ffi::c_uint,
            rc_overshoot_pct: 50 as core::ffi::c_uint,
            rc_buf_sz: 1000 as core::ffi::c_uint,
            rc_buf_initial_sz: 600 as core::ffi::c_uint,
            rc_buf_optimal_sz: 600 as core::ffi::c_uint,
            rc_2pass_vbr_bias_pct: 50 as core::ffi::c_uint,
            rc_2pass_vbr_minsection_pct: 0 as core::ffi::c_uint,
            rc_2pass_vbr_maxsection_pct: 2000 as core::ffi::c_uint,
            fwd_kf_enabled: 0 as core::ffi::c_int,
            kf_mode: AOM_KF_AUTO,
            kf_min_dist: 0 as core::ffi::c_uint,
            kf_max_dist: 9999 as core::ffi::c_uint,
            sframe_dist: 0 as core::ffi::c_uint,
            sframe_mode: 1 as core::ffi::c_uint,
            large_scale_tile: 0 as core::ffi::c_uint,
            monochrome: 0 as core::ffi::c_uint,
            full_still_picture_hdr: 0 as core::ffi::c_uint,
            save_as_annexb: 0 as core::ffi::c_uint,
            tile_width_count: 0 as core::ffi::c_int,
            tile_height_count: 0 as core::ffi::c_int,
            tile_widths: [0 as core::ffi::c_int; 64],
            tile_heights: [0 as core::ffi::c_int; 64],
            use_fixed_qp_offsets: 0 as core::ffi::c_uint,
            fixed_qp_offsets: [
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
            ],
            encoder_cfg: {
                let mut init = cfg_options {
                    init_by_cfg_file: 0 as core::ffi::c_uint,
                    super_block_size: 128 as core::ffi::c_uint,
                    max_partition_size: 128 as core::ffi::c_uint,
                    min_partition_size: 4 as core::ffi::c_uint,
                    disable_ab_partition_type: 0 as core::ffi::c_uint,
                    disable_rect_partition_type: 0 as core::ffi::c_uint,
                    disable_1to4_partition_type: 0 as core::ffi::c_uint,
                    disable_flip_idtx: 0 as core::ffi::c_uint,
                    disable_cdef: 0 as core::ffi::c_uint,
                    disable_lr: 0 as core::ffi::c_uint,
                    disable_obmc: 0 as core::ffi::c_uint,
                    disable_warp_motion: 0 as core::ffi::c_uint,
                    disable_global_motion: 0 as core::ffi::c_uint,
                    disable_dist_wtd_comp: 0 as core::ffi::c_uint,
                    disable_diff_wtd_comp: 0 as core::ffi::c_uint,
                    disable_inter_intra_comp: 0 as core::ffi::c_uint,
                    disable_masked_comp: 0 as core::ffi::c_uint,
                    disable_one_sided_comp: 0 as core::ffi::c_uint,
                    disable_palette: 0 as core::ffi::c_uint,
                    disable_intrabc: 0 as core::ffi::c_uint,
                    disable_cfl: 0 as core::ffi::c_uint,
                    disable_smooth_intra: 0 as core::ffi::c_uint,
                    disable_filter_intra: 0 as core::ffi::c_uint,
                    disable_dual_filter: 0 as core::ffi::c_uint,
                    disable_intra_angle_delta: 0 as core::ffi::c_uint,
                    disable_intra_edge_filter: 0 as core::ffi::c_uint,
                    disable_tx_64x64: 0 as core::ffi::c_uint,
                    disable_smooth_inter_intra: 0 as core::ffi::c_uint,
                    disable_inter_inter_wedge: 0 as core::ffi::c_uint,
                    disable_inter_intra_wedge: 0 as core::ffi::c_uint,
                    disable_paeth_intra: 0 as core::ffi::c_uint,
                    disable_trellis_quant: 0 as core::ffi::c_uint,
                    disable_ref_frame_mv: 0 as core::ffi::c_uint,
                    reduced_reference_set: 0 as core::ffi::c_uint,
                    reduced_tx_type_set: 0 as core::ffi::c_uint,
                };
                init
            },
        };
        init
    },
    {
        let mut init = aom_codec_enc_cfg {
            g_usage: AOM_USAGE_ALL_INTRA,
            g_threads: 0 as core::ffi::c_uint,
            g_profile: 0 as core::ffi::c_uint,
            g_w: 320 as core::ffi::c_uint,
            g_h: 240 as core::ffi::c_uint,
            g_limit: 0 as core::ffi::c_uint,
            g_forced_max_frame_width: 0 as core::ffi::c_uint,
            g_forced_max_frame_height: 0 as core::ffi::c_uint,
            g_bit_depth: AOM_BITS_8,
            g_input_bit_depth: 8 as core::ffi::c_uint,
            g_timebase: {
                let mut init = aom_rational {
                    num: 1 as core::ffi::c_int,
                    den: 30 as core::ffi::c_int,
                };
                init
            },
            g_error_resilient: 0 as aom_codec_er_flags_t,
            g_pass: AOM_RC_ONE_PASS,
            g_lag_in_frames: 0 as core::ffi::c_uint,
            rc_dropframe_thresh: 0 as core::ffi::c_uint,
            rc_resize_mode: RESIZE_NONE as core::ffi::c_int as core::ffi::c_uint,
            rc_resize_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_resize_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_mode: AOM_SUPERRES_NONE,
            rc_superres_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_kf_denominator: SCALE_NUMERATOR as core::ffi::c_uint,
            rc_superres_qthresh: 63 as core::ffi::c_uint,
            rc_superres_kf_qthresh: 32 as core::ffi::c_uint,
            rc_end_usage: AOM_Q,
            rc_twopass_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_firstpass_mb_stats_in: {
                let mut init = aom_fixed_buf {
                    buf: NULL,
                    sz: 0 as size_t,
                };
                init
            },
            rc_target_bitrate: 256 as core::ffi::c_uint,
            rc_min_quantizer: 0 as core::ffi::c_uint,
            rc_max_quantizer: 63 as core::ffi::c_uint,
            rc_undershoot_pct: 25 as core::ffi::c_uint,
            rc_overshoot_pct: 25 as core::ffi::c_uint,
            rc_buf_sz: 6000 as core::ffi::c_uint,
            rc_buf_initial_sz: 4000 as core::ffi::c_uint,
            rc_buf_optimal_sz: 5000 as core::ffi::c_uint,
            rc_2pass_vbr_bias_pct: 50 as core::ffi::c_uint,
            rc_2pass_vbr_minsection_pct: 0 as core::ffi::c_uint,
            rc_2pass_vbr_maxsection_pct: 2000 as core::ffi::c_uint,
            fwd_kf_enabled: 0 as core::ffi::c_int,
            kf_mode: AOM_KF_DISABLED,
            kf_min_dist: 0 as core::ffi::c_uint,
            kf_max_dist: 0 as core::ffi::c_uint,
            sframe_dist: 0 as core::ffi::c_uint,
            sframe_mode: 1 as core::ffi::c_uint,
            large_scale_tile: 0 as core::ffi::c_uint,
            monochrome: 0 as core::ffi::c_uint,
            full_still_picture_hdr: 0 as core::ffi::c_uint,
            save_as_annexb: 0 as core::ffi::c_uint,
            tile_width_count: 0 as core::ffi::c_int,
            tile_height_count: 0 as core::ffi::c_int,
            tile_widths: [0 as core::ffi::c_int; 64],
            tile_heights: [0 as core::ffi::c_int; 64],
            use_fixed_qp_offsets: 0 as core::ffi::c_uint,
            fixed_qp_offsets: [
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
            ],
            encoder_cfg: {
                let mut init = cfg_options {
                    init_by_cfg_file: 0 as core::ffi::c_uint,
                    super_block_size: 128 as core::ffi::c_uint,
                    max_partition_size: 128 as core::ffi::c_uint,
                    min_partition_size: 4 as core::ffi::c_uint,
                    disable_ab_partition_type: 0 as core::ffi::c_uint,
                    disable_rect_partition_type: 0 as core::ffi::c_uint,
                    disable_1to4_partition_type: 0 as core::ffi::c_uint,
                    disable_flip_idtx: 0 as core::ffi::c_uint,
                    disable_cdef: 0 as core::ffi::c_uint,
                    disable_lr: 0 as core::ffi::c_uint,
                    disable_obmc: 0 as core::ffi::c_uint,
                    disable_warp_motion: 0 as core::ffi::c_uint,
                    disable_global_motion: 0 as core::ffi::c_uint,
                    disable_dist_wtd_comp: 0 as core::ffi::c_uint,
                    disable_diff_wtd_comp: 0 as core::ffi::c_uint,
                    disable_inter_intra_comp: 0 as core::ffi::c_uint,
                    disable_masked_comp: 0 as core::ffi::c_uint,
                    disable_one_sided_comp: 0 as core::ffi::c_uint,
                    disable_palette: 0 as core::ffi::c_uint,
                    disable_intrabc: 0 as core::ffi::c_uint,
                    disable_cfl: 0 as core::ffi::c_uint,
                    disable_smooth_intra: 0 as core::ffi::c_uint,
                    disable_filter_intra: 0 as core::ffi::c_uint,
                    disable_dual_filter: 0 as core::ffi::c_uint,
                    disable_intra_angle_delta: 0 as core::ffi::c_uint,
                    disable_intra_edge_filter: 0 as core::ffi::c_uint,
                    disable_tx_64x64: 0 as core::ffi::c_uint,
                    disable_smooth_inter_intra: 0 as core::ffi::c_uint,
                    disable_inter_inter_wedge: 0 as core::ffi::c_uint,
                    disable_inter_intra_wedge: 0 as core::ffi::c_uint,
                    disable_paeth_intra: 0 as core::ffi::c_uint,
                    disable_trellis_quant: 0 as core::ffi::c_uint,
                    disable_ref_frame_mv: 0 as core::ffi::c_uint,
                    reduced_reference_set: 0 as core::ffi::c_uint,
                    reduced_tx_type_set: 0 as core::ffi::c_uint,
                };
                init
            },
        };
        init
    },
];
#[no_mangle]
pub static mut aom_codec_av1_cx_algo: aom_codec_iface_t = aom_codec_iface {
    name: 0 as *const core::ffi::c_char,
    abi_version: 0,
    caps: 0,
    init: None,
    destroy: None,
    ctrl_maps: 0 as *const aom_codec_ctrl_fn_map_t,
    dec: aom_codec_dec_iface {
        peek_si: None,
        get_si: None,
        decode: None,
        get_frame: None,
        set_fb_fn: None,
    },
    enc: aom_codec_enc_iface {
        cfg_count: 0,
        cfgs: 0 as *const aom_codec_enc_cfg_t,
        encode: None,
        get_cx_data: None,
        cfg_set: None,
        get_glob_hdrs: None,
        get_preview: None,
    },
    set_option: None,
};
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_av1_cx() -> *const aom_codec_iface_t {
    return &raw const aom_codec_av1_cx_algo;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
unsafe extern "C-unwind" fn run_static_initializers() {
    aom_codec_av1_cx_algo = {
        let mut init = aom_codec_iface {
            name: b"AOMedia Project AV1 Encoder v3.15.2\0" as *const u8
                as *const core::ffi::c_char,
            abi_version: AOM_CODEC_INTERNAL_ABI_VERSION,
            caps: ((if CONFIG_AV1_HIGHBITDEPTH != 0 {
                AOM_CODEC_CAP_HIGHBITDEPTH
            } else {
                0 as core::ffi::c_int
            }) | AOM_CODEC_CAP_ENCODER | AOM_CODEC_CAP_PSNR) as aom_codec_caps_t,
            init: Some(
                encoder_init
                    as unsafe extern "C-unwind" fn(*mut aom_codec_ctx_t) -> aom_codec_err_t,
            ),
            destroy: Some(
                encoder_destroy
                    as unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> aom_codec_err_t,
            ),
            ctrl_maps: core::ptr::addr_of!(encoder_ctrl_maps).cast::<aom_codec_ctrl_fn_map_t>(),
            dec: {
                let mut init = aom_codec_dec_iface {
                    peek_si: None,
                    get_si: None,
                    decode: None,
                    get_frame: None,
                    set_fb_fn: None,
                };
                init
            },
            enc: {
                let mut init = aom_codec_enc_iface {
                    cfg_count: (::core::mem::size_of::<[aom_codec_enc_cfg_t; 3]>()
                        as usize)
                        .wrapping_div(
                            ::core::mem::size_of::<aom_codec_enc_cfg_t>() as usize,
                        ) as core::ffi::c_int,
                    cfgs: core::ptr::addr_of!(encoder_usage_cfg).cast::<aom_codec_enc_cfg_t>(),
                    encode: Some(
                        encoder_encode
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *const aom_image_t,
                                aom_codec_pts_t,
                                core::ffi::c_ulong,
                                aom_enc_frame_flags_t,
                            ) -> aom_codec_err_t,
                    ),
                    get_cx_data: Some(
                        encoder_get_cxdata
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *mut aom_codec_iter_t,
                            ) -> *const aom_codec_cx_pkt_t,
                    ),
                    cfg_set: Some(
                        encoder_set_config
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *const aom_codec_enc_cfg_t,
                            ) -> aom_codec_err_t,
                    ),
                    get_glob_hdrs: Some(
                        encoder_get_global_headers
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                            ) -> *mut aom_fixed_buf_t,
                    ),
                    get_preview: Some(
                        encoder_get_preview
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                            ) -> *mut aom_image_t,
                    ),
                };
                init
            },
            set_option: Some(
                encoder_set_option
                    as unsafe extern "C-unwind" fn(
                        *mut aom_codec_alg_priv_t,
                        *const core::ffi::c_char,
                        *const core::ffi::c_char,
                    ) -> aom_codec_err_t,
            ),
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C-unwind" fn(); 1] = [run_static_initializers];

extern "C" {
    fn log2(_: core::ffi::c_double) -> core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
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
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn snprintf(
        __str: *mut core::ffi::c_char,
        __size: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
}
