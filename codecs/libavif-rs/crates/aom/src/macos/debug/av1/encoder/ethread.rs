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

// Mechanically translated from upstream/aom/av1/encoder/ethread.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type aom_write_bit_buffer;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
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
    fn av1_tile_init(
        tile: *mut TileInfo,
        cm: *const AV1Common,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
    );
    fn av1_get_sb_rows_in_tile(
        cm: *const AV1Common,
        tile: *const TileInfo,
    ) -> core::ffi::c_int;
    fn av1_get_sb_cols_in_tile(
        cm: *const AV1Common,
        tile: *const TileInfo,
    ) -> core::ffi::c_int;
    fn av1_reset_loop_filter_delta(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    fn av1_loop_filter_frame_init(
        cm: *mut AV1Common,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
    );
    fn av1_lr_count_units(
        unit_size: core::ffi::c_int,
        plane_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_alloc_cdef_buffers(
        cm: *mut AV1Common,
        cdef_worker: *mut *mut AV1CdefWorker,
        cdef_sync: *mut AV1CdefSyncData,
        num_workers: core::ffi::c_int,
        init_worker: core::ffi::c_int,
    );
    fn cfl_init(cfl: *mut CFL_CTX, seq_params: *const SequenceHeader);
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
    fn av1_loop_filter_dealloc(lf_sync: *mut AV1LfSync);
    fn av1_loop_filter_alloc(
        lf_sync: *mut AV1LfSync,
        cm: *mut AV1_COMMON,
        rows: core::ffi::c_int,
        width: core::ffi::c_int,
        num_workers: core::ffi::c_int,
    );
    fn av1_set_vert_loop_filter_done(
        cm: *mut AV1_COMMON,
        lf_sync: *mut AV1LfSync,
        num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
    );
    fn av1_loop_restoration_dealloc(lr_sync: *mut AV1LrSync);
    fn av1_loop_restoration_alloc(
        lr_sync: *mut AV1LrSync,
        cm: *mut AV1_COMMON,
        num_workers: core::ffi::c_int,
        num_rows_lr: core::ffi::c_int,
        num_planes: core::ffi::c_int,
        width: core::ffi::c_int,
    );
    fn av1_get_intrabc_extra_top_right_sb_delay(
        cm: *const AV1_COMMON,
    ) -> core::ffi::c_int;
    fn av1_thread_loop_filter_rows(
        frame_buffer: *const YV12_BUFFER_CONFIG,
        cm: *mut AV1_COMMON,
        planes: *mut macroblockd_plane,
        xd: *mut MACROBLOCKD,
        mi_row: core::ffi::c_int,
        plane: core::ffi::c_int,
        dir: core::ffi::c_int,
        lpf_opt_level: core::ffi::c_int,
        lf_sync: *mut AV1LfSync,
        error_info: *mut aom_internal_error_info,
        params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
        tx_buf: *mut TX_SIZE,
        mib_size_log2: core::ffi::c_int,
    );
    fn av1_crc32c_calculator_init(p_crc32c: *mut CRC32C);
    fn av1_get_unit_rows_in_tile(
        tile: *const TileInfo,
        fp_block_size: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_get_unit_cols_in_tile(
        tile: *const TileInfo,
        fp_block_size: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_first_pass_row(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_data: *mut TileDataEnc,
        mb_row: core::ffi::c_int,
        fp_block_size: BLOCK_SIZE,
    );
    fn av1_init_obmc_buffer(obmc_buffer: *mut OBMCBuffer);
    fn av1_setup_shared_coeff_buffer(
        seq_params: *const SequenceHeader,
        shared_bufs: *mut PC_TREE_SHARED_BUFFERS,
        error: *mut aom_internal_error_info,
    );
    fn av1_alloc_pc_tree_node(bsize: BLOCK_SIZE) -> *mut PC_TREE;
    fn av1_free_pc_tree_recursive(
        tree: *mut PC_TREE,
        num_planes: core::ffi::c_int,
        keep_best: core::ffi::c_int,
        keep_none: core::ffi::c_int,
        partition_search_type: PARTITION_SEARCH_TYPE,
    );
    fn av1_alloc_pmc(
        cpi: *const AV1_COMP,
        bsize: BLOCK_SIZE,
        shared_bufs: *mut PC_TREE_SHARED_BUFFERS,
    ) -> *mut PICK_MODE_CONTEXT;
    fn av1_setup_sms_tree(cpi: *mut AV1_COMP, td: *mut ThreadData) -> core::ffi::c_int;
    fn av1_cdef_mse_calc_block(
        cdef_search_ctx: *mut CdefSearchCtx,
        error_info: *mut aom_internal_error_info,
        fbr: core::ffi::c_int,
        fbc: core::ffi::c_int,
        sb_count: core::ffi::c_int,
        adaptive_cdef_mode: core::ffi::c_int,
    );
    fn av1_tf_do_filtering_row(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        mb_row: core::ffi::c_int,
    );
    fn av1_mc_flow_dispenser_row(
        cpi: *mut AV1_COMP,
        tpl_txfm_stats: *mut TplTxfmStats,
        tpl_tmp_buffers: *mut TplBuffers,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
    );
    fn av1_init_tpl_txfm_stats(tpl_txfm_stats: *mut TplTxfmStats);
    fn av1_pack_tile_info(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        pack_bs_params: *mut PackBSParams,
    );
    fn av1_write_last_tile_info(
        cpi: *mut AV1_COMP,
        fh_info: *const FrameHeaderInfo,
        saved_wb: *mut aom_write_bit_buffer,
        curr_tg_data_size: *mut size_t,
        curr_tg_start: *mut uint8_t,
        total_size: *mut uint32_t,
        tile_data_start: *mut *mut uint8_t,
        largest_tile_id: *mut core::ffi::c_int,
        is_first_tg: *mut core::ffi::c_int,
        obu_header_size: uint32_t,
        obu_extn_header: uint8_t,
    );
    fn av1_reset_pack_bs_thread_data(td: *mut ThreadData);
    fn av1_accumulate_pack_bs_thread_data(cpi: *mut AV1_COMP, td: *const ThreadData);
    fn av1_write_obu_tg_tile_headers(
        cpi: *mut AV1_COMP,
        xd: *mut MACROBLOCKD,
        pack_bs_params: *mut PackBSParams,
        tile_idx: core::ffi::c_int,
    );
    fn av1_release_scaled_references_fpmt(cpi: *mut AV1_COMP);
    fn av1_decrement_ref_counts_fpmt(
        buffer_pool: *mut BufferPool,
        ref_buffers_used_map: core::ffi::c_int,
    );
    fn av1_init_parallel_frame_context(
        first_cpi_data: *const AV1_COMP_DATA,
        ppi: *mut AV1_PRIMARY,
        ref_buffers_used_map: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_compressed_data(
        cpi: *mut AV1_COMP,
        cpi_data: *mut AV1_COMP_DATA,
    ) -> core::ffi::c_int;
    fn av1_alloc_mb_wiener_var_pred_buf(cm: *mut AV1_COMMON, td: *mut ThreadData);
    fn av1_dealloc_mb_wiener_var_pred_buf(td: *mut ThreadData);
    fn av1_calc_mb_wiener_var_row(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        xd: *mut MACROBLOCKD,
        mi_row: core::ffi::c_int,
        src_diff: *mut int16_t,
        coeff: *mut tran_low_t,
        qcoeff: *mut tran_low_t,
        dqcoeff: *mut tran_low_t,
        sum_rec_distortion: *mut core::ffi::c_double,
        sum_est_rate: *mut core::ffi::c_double,
        pred_buffer: *mut uint8_t,
    );
    fn av1_init_rtc_counters(x: *mut macroblock);
    fn av1_accumulate_rtc_counters(cpi: *mut AV1_COMP, x: *const macroblock);
    fn av1_alloc_tile_data(cpi: *mut AV1_COMP);
    fn av1_init_tile_data(cpi: *mut AV1_COMP);
    fn av1_encode_tile(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_row: core::ffi::c_int,
        tile_col: core::ffi::c_int,
    );
    fn av1_encode_sb_row(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_row: core::ffi::c_int,
        tile_col: core::ffi::c_int,
        mi_row: core::ffi::c_int,
    );
    fn av1_inter_mode_data_init(tile_data: *mut TileDataEnc);
    fn av1_dealloc_src_diff_buf(mb: *mut macroblock, num_planes: core::ffi::c_int);
    fn av1_alloc_src_diff_buf(cm: *const AV1Common, mb: *mut macroblock);
    fn av1_compute_gm_for_valid_ref_frames(
        cpi: *mut AV1_COMP,
        error_info: *mut aom_internal_error_info,
        ref_buf: *mut *mut YV12_BUFFER_CONFIG,
        frame: core::ffi::c_int,
        motion_models: *mut MotionModel,
        segment_map: *mut uint8_t,
        segment_map_w: core::ffi::c_int,
        segment_map_h: core::ffi::c_int,
    );
    fn av1_pick_filter_level(
        sd: *const yv12_buffer_config,
        cpi: *mut AV1_COMP,
        method: LPF_PICK_METHOD,
    );
}
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
pub struct _opaque_pthread_condattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
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
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type pthread_cond_t = libc::pthread_cond_t;
pub type pthread_condattr_t = libc::pthread_condattr_t;
pub type pthread_mutex_t = libc::pthread_mutex_t;
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type float_t = core::ffi::c_float;
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
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
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed_1 = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed_1 = 4;
pub const TX_SIZES: C2RustUnnamed_1 = 5;
pub const TX_SIZES_ALL: C2RustUnnamed_1 = 19;
pub const TX_64X16: C2RustUnnamed_1 = 18;
pub const TX_16X64: C2RustUnnamed_1 = 17;
pub const TX_32X8: C2RustUnnamed_1 = 16;
pub const TX_8X32: C2RustUnnamed_1 = 15;
pub const TX_16X4: C2RustUnnamed_1 = 14;
pub const TX_4X16: C2RustUnnamed_1 = 13;
pub const TX_64X32: C2RustUnnamed_1 = 12;
pub const TX_32X64: C2RustUnnamed_1 = 11;
pub const TX_32X16: C2RustUnnamed_1 = 10;
pub const TX_16X32: C2RustUnnamed_1 = 9;
pub const TX_16X8: C2RustUnnamed_1 = 8;
pub const TX_8X16: C2RustUnnamed_1 = 7;
pub const TX_8X4: C2RustUnnamed_1 = 6;
pub const TX_4X8: C2RustUnnamed_1 = 5;
pub const TX_64X64: C2RustUnnamed_1 = 4;
pub const TX_32X32: C2RustUnnamed_1 = 3;
pub const TX_16X16: C2RustUnnamed_1 = 2;
pub const TX_8X8: C2RustUnnamed_1 = 1;
pub const TX_4X4: C2RustUnnamed_1 = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_2 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_2 = 15;
pub const TX_TYPES: C2RustUnnamed_2 = 16;
pub const H_FLIPADST: C2RustUnnamed_2 = 15;
pub const V_FLIPADST: C2RustUnnamed_2 = 14;
pub const H_ADST: C2RustUnnamed_2 = 13;
pub const V_ADST: C2RustUnnamed_2 = 12;
pub const H_DCT: C2RustUnnamed_2 = 11;
pub const V_DCT: C2RustUnnamed_2 = 10;
pub const IDTX: C2RustUnnamed_2 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_2 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_2 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_2 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_2 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_2 = 4;
pub const ADST_ADST: C2RustUnnamed_2 = 3;
pub const DCT_ADST: C2RustUnnamed_2 = 2;
pub const ADST_DCT: C2RustUnnamed_2 = 1;
pub const DCT_DCT: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_3 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_3 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_3 = 0;
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_4 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_4 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_4 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_4 = 0;
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
pub type InterpKernel = [int16_t; 8];
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
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
}
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_18 = 4;
pub const AFFINE: C2RustUnnamed_18 = 3;
pub const ROTZOOM: C2RustUnnamed_18 = 2;
pub const TRANSLATION: C2RustUnnamed_18 = 1;
pub const IDENTITY: C2RustUnnamed_18 = 0;
pub type TransformationType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type FULLPEL_MV = fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type MV = mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
    pub as_fullmv: FULLPEL_MV,
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
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct candidate_mv {
    pub this_mv: int_mv,
    pub comp_mv: int_mv,
}
pub type CANDIDATE_MV = candidate_mv;
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
pub type CONV_BUF_TYPE = uint16_t;
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
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_19 = 4;
pub const S_FRAME: C2RustUnnamed_19 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_19 = 2;
pub const INTER_FRAME: C2RustUnnamed_19 = 1;
pub const KEY_FRAME: C2RustUnnamed_19 = 0;
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
pub struct AVxWorkerInterface {
    pub init: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub reset: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub sync: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub launch: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub execute: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub end: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
}
pub type AV1LfSync = AV1LfSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_20 = 2;
pub const REALTIME: C2RustUnnamed_20 = 1;
pub const GOOD: C2RustUnnamed_20 = 0;
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
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_21 = 2;
pub const LAP_STAGE: C2RustUnnamed_21 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const LPF_PICK_MINIMAL_LPF: C2RustUnnamed_22 = 4;
pub const LPF_PICK_FROM_Q: C2RustUnnamed_22 = 3;
pub const LPF_PICK_FROM_SUBIMAGE: C2RustUnnamed_22 = 2;
pub const LPF_PICK_FROM_FULL_IMAGE_NON_DUAL: C2RustUnnamed_22 = 1;
pub const LPF_PICK_FROM_FULL_IMAGE: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const MULTI_WINNER_MODE_LEVELS: C2RustUnnamed_23 = 3;
pub const MULTI_WINNER_MODE_DEFAULT: C2RustUnnamed_23 = 2;
pub const MULTI_WINNER_MODE_FAST: C2RustUnnamed_23 = 1;
pub const MULTI_WINNER_MODE_OFF: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const VAR_BASED_PARTITION: C2RustUnnamed_24 = 2;
pub const FIXED_PARTITION: C2RustUnnamed_24 = 1;
pub const SEARCH_PARTITION: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileBufferEnc {
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameHeaderInfo {
    pub frame_header: *mut uint8_t,
    pub obu_header_byte_offset: size_t,
    pub total_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PackBSParams {
    pub saved_wb: *mut aom_write_bit_buffer,
    pub buf: TileBufferEnc,
    pub total_size: *mut uint32_t,
    pub dst: *mut uint8_t,
    pub tile_data_curr: *mut uint8_t,
    pub tile_buf_size: size_t,
    pub obu_extn_header: uint8_t,
    pub obu_header_size: uint32_t,
    pub curr_tg_hdr_size: core::ffi::c_int,
    pub tile_size_mi: core::ffi::c_int,
    pub tile_row: core::ffi::c_int,
    pub tile_col: core::ffi::c_int,
    pub is_last_tile_in_tg: core::ffi::c_int,
    pub new_tg: core::ffi::c_int,
}
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
pub const MOD_TPL: MULTI_THREADED_MODULES = 2;
pub const MOD_TF: MULTI_THREADED_MODULES = 1;
pub const MOD_FP: MULTI_THREADED_MODULES = 0;
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const SKIP_APPLY_LOOPFILTER: C2RustUnnamed_25 = 8;
pub const SKIP_APPLY_CDEF: C2RustUnnamed_25 = 4;
pub const SKIP_APPLY_SUPERRES: C2RustUnnamed_25 = 2;
pub const SKIP_APPLY_RESTORATION: C2RustUnnamed_25 = 1;
pub type InterModesInfo = inter_modes_info;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
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
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MI_SIZE_64X64: core::ffi::c_int = 64 as core::ffi::c_int >> MI_SIZE_LOG2;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_TILES: core::ffi::c_int = 512 as core::ffi::c_int;
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
static mut mi_size_high_log2: [uint8_t; 22] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
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
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
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
unsafe extern "C-unwind" fn av1_init_above_context(
    mut above_contexts: *mut CommonContexts,
    mut num_planes: core::ffi::c_int,
    mut tile_row: core::ffi::c_int,
    mut xd: *mut MACROBLOCKD,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        (*xd).above_entropy_context[i as usize] = *((*above_contexts)
            .entropy[i as usize])
            .offset(tile_row as isize);
        i += 1;
    }
    (*xd).above_partition_context = *((*above_contexts).partition)
        .offset(tile_row as isize);
    (*xd).above_txfm_context = *((*above_contexts).txfm).offset(tile_row as isize);
}
#[inline]
unsafe extern "C-unwind" fn av1_zero_above_context(
    cm: *mut AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_col_start: core::ffi::c_int,
    mut mi_col_end: core::ffi::c_int,
    tile_row: core::ffi::c_int,
) {
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let width: core::ffi::c_int = mi_col_end - mi_col_start;
    let aligned_width: core::ffi::c_int = width
        + (((1 as core::ffi::c_int) << (*seq_params).mib_size_log2)
            - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << (*seq_params).mib_size_log2)
            - 1 as core::ffi::c_int);
    let offset_y: core::ffi::c_int = mi_col_start;
    let width_y: core::ffi::c_int = aligned_width;
    let offset_uv: core::ffi::c_int = offset_y >> (*seq_params).subsampling_x;
    let width_uv: core::ffi::c_int = width_y >> (*seq_params).subsampling_x;
    let above_contexts: *mut CommonContexts = &raw mut (*cm).above_contexts;
    memset(
        (*((*above_contexts).entropy[0 as core::ffi::c_int as usize])
            .offset(tile_row as isize))
            .offset(offset_y as isize) as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (width_y as size_t)
            .wrapping_mul(::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t),
    );
    if num_planes > 1 as core::ffi::c_int {
        if !(*((*above_contexts).entropy[1 as core::ffi::c_int as usize])
            .offset(tile_row as isize))
            .is_null()
            && !(*((*above_contexts).entropy[2 as core::ffi::c_int as usize])
                .offset(tile_row as isize))
                .is_null()
        {
            memset(
                (*((*above_contexts).entropy[1 as core::ffi::c_int as usize])
                    .offset(tile_row as isize))
                    .offset(offset_uv as isize) as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (width_uv as size_t)
                    .wrapping_mul(::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t),
            );
            memset(
                (*((*above_contexts).entropy[2 as core::ffi::c_int as usize])
                    .offset(tile_row as isize))
                    .offset(offset_uv as isize) as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (width_uv as size_t)
                    .wrapping_mul(::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t),
            );
        } else {
            aom_internal_error(
                (*xd).error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid value of planes\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    memset(
        (*((*above_contexts).partition).offset(tile_row as isize))
            .offset(mi_col_start as isize) as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (aligned_width as size_t)
            .wrapping_mul(::core::mem::size_of::<PARTITION_CONTEXT>() as size_t),
    );
    memset(
        (*((*above_contexts).txfm).offset(tile_row as isize))
            .offset(mi_col_start as isize) as *mut core::ffi::c_void,
        tx_size_wide[TX_SIZES_LARGEST as core::ffi::c_int as usize],
        (aligned_width as size_t)
            .wrapping_mul(::core::mem::size_of::<TXFM_CONTEXT>() as size_t),
    );
}
#[inline(always)]
unsafe extern "C-unwind" fn skip_loop_filter_plane(
    mut planes_to_lf: *const core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
) -> bool {
    if lpf_opt_level == 2 as core::ffi::c_int {
        if plane == AOM_PLANE_Y {
            return *planes_to_lf.offset(plane as isize) == 0;
        }
        if plane == AOM_PLANE_U {
            return *planes_to_lf.offset(1 as core::ffi::c_int as isize) == 0
                && *planes_to_lf.offset(2 as core::ffi::c_int as isize) == 0;
        }
        if !(plane == 2 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"skip_loop_filter_plane\0"))
                    .as_ptr(),
                b"thread_common.h\0" as *const u8 as *const core::ffi::c_char,
                224 as core::ffi::c_int,
                b"plane == AOM_PLANE_V\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if plane == AOM_PLANE_V {
            return true_0 != 0;
        }
    }
    return *planes_to_lf.offset(plane as isize) == 0;
}
#[inline]
unsafe extern "C-unwind" fn enqueue_lf_jobs(
    mut lf_sync: *mut AV1LfSync,
    mut start: core::ffi::c_int,
    mut stop: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
    mut num_mis_in_lpf_unit_height: core::ffi::c_int,
) {
    let mut mi_row: core::ffi::c_int = 0;
    let mut plane: core::ffi::c_int = 0;
    let mut dir: core::ffi::c_int = 0;
    let mut lf_job_queue: *mut AV1LfMTInfo = (*lf_sync).job_queue;
    (*lf_sync).jobs_enqueued = 0 as core::ffi::c_int;
    (*lf_sync).jobs_dequeued = 0 as core::ffi::c_int;
    dir = 0 as core::ffi::c_int;
    while dir < 2 as core::ffi::c_int {
        mi_row = start;
        while mi_row < stop {
            plane = 0 as core::ffi::c_int;
            while plane < MAX_MB_PLANE {
                if !skip_loop_filter_plane(planes_to_lf, plane, lpf_opt_level) {
                    if !(*planes_to_lf.offset(plane as isize) == 0) {
                        (*lf_job_queue).mi_row = mi_row;
                        (*lf_job_queue).plane = plane;
                        (*lf_job_queue).dir = dir;
                        (*lf_job_queue).lpf_opt_level = lpf_opt_level;
                        lf_job_queue = lf_job_queue.offset(1);
                        (*lf_sync).jobs_enqueued += 1;
                    }
                }
                plane += 1;
            }
            mi_row += num_mis_in_lpf_unit_height;
        }
        dir += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn loop_filter_frame_mt_init(
    mut cm: *mut AV1_COMMON,
    mut start_mi_row: core::ffi::c_int,
    mut end_mi_row: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut num_workers: core::ffi::c_int,
    mut lf_sync: *mut AV1LfSync,
    mut lpf_opt_level: core::ffi::c_int,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let sb_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2)
        - 1 as core::ffi::c_int >> num_mis_in_lpf_unit_height_log2;
    if (*lf_sync).sync_range == 0 || sb_rows != (*lf_sync).rows
        || num_workers > (*lf_sync).num_workers
    {
        av1_loop_filter_dealloc(lf_sync);
        av1_loop_filter_alloc(lf_sync, cm, sb_rows, (*cm).width, num_workers);
    }
    (*lf_sync).lf_mt_exit = false_0 != 0;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_MB_PLANE {
        memset(
            (*lf_sync).cur_sb_col[i as usize] as *mut core::ffi::c_void,
            -(1 as core::ffi::c_int),
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(sb_rows as size_t),
        );
        i += 1;
    }
    enqueue_lf_jobs(
        lf_sync,
        start_mi_row,
        end_mi_row,
        planes_to_lf,
        lpf_opt_level,
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2,
    );
}
#[inline]
unsafe extern "C-unwind" fn get_lf_job_info(mut lf_sync: *mut AV1LfSync) -> *mut AV1LfMTInfo {
    let mut cur_job_info: *mut AV1LfMTInfo = 0 as *mut AV1LfMTInfo;
    pthread_mutex_lock((*lf_sync).job_mutex as *mut pthread_mutex_t);
    if !(*lf_sync).lf_mt_exit && (*lf_sync).jobs_dequeued < (*lf_sync).jobs_enqueued {
        cur_job_info = ((*lf_sync).job_queue).offset((*lf_sync).jobs_dequeued as isize);
        (*lf_sync).jobs_dequeued += 1;
    }
    pthread_mutex_unlock((*lf_sync).job_mutex as *mut pthread_mutex_t);
    return cur_job_info;
}
#[inline]
unsafe extern "C-unwind" fn loop_filter_data_reset(
    mut lf_data: *mut LFWorkerData,
    mut frame_buffer: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1Common,
    mut xd: *mut MACROBLOCKD,
) {
    let mut pd: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
    (*lf_data).frame_buffer = frame_buffer;
    (*lf_data).cm = cm;
    (*lf_data).xd = xd;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_MB_PLANE {
        (*lf_data).planes[i as usize].dst = (*pd.offset(i as isize)).dst;
        (*lf_data).planes[i as usize].subsampling_x = (*pd.offset(i as isize))
            .subsampling_x;
        (*lf_data).planes[i as usize].subsampling_y = (*pd.offset(i as isize))
            .subsampling_y;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_planes_to_loop_filter(
    mut lf: *const loopfilter,
    mut planes_to_lf: *mut core::ffi::c_int,
    mut plane_start: core::ffi::c_int,
    mut plane_end: core::ffi::c_int,
) {
    *planes_to_lf.offset(0 as core::ffi::c_int as isize) = (((*lf)
        .filter_level[0 as core::ffi::c_int as usize] != 0
        || (*lf).filter_level[1 as core::ffi::c_int as usize] != 0)
        && plane_start <= 0 as core::ffi::c_int && (0 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
    *planes_to_lf.offset(1 as core::ffi::c_int as isize) = ((*lf).filter_level_u != 0
        && plane_start <= 1 as core::ffi::c_int && (1 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
    *planes_to_lf.offset(2 as core::ffi::c_int as isize) = ((*lf).filter_level_v != 0
        && plane_start <= 2 as core::ffi::c_int && (2 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
}
pub const MAX_NUM_THREADS: core::ffi::c_int = 64 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_set_mv_row_limits(
    mi_params: *const CommonModeInfoParams,
    mut mv_limits: *mut FullMvLimits,
    mut mi_row: core::ffi::c_int,
    mut mi_height: core::ffi::c_int,
    mut border: core::ffi::c_int,
) {
    let min1: core::ffi::c_int = -(mi_row * MI_SIZE + border
        - 2 as core::ffi::c_int * AOM_INTERP_EXTEND);
    let min2: core::ffi::c_int = -((mi_row + mi_height) * MI_SIZE
        + 2 as core::ffi::c_int * AOM_INTERP_EXTEND);
    (*mv_limits).row_min = if min1 > min2 { min1 } else { min2 };
    let max1: core::ffi::c_int = ((*mi_params).mi_rows - mi_row - mi_height) * MI_SIZE
        + border - 2 as core::ffi::c_int * AOM_INTERP_EXTEND;
    let max2: core::ffi::c_int = ((*mi_params).mi_rows - mi_row) * MI_SIZE
        + 2 as core::ffi::c_int * AOM_INTERP_EXTEND;
    (*mv_limits).row_max = if max1 < max2 { max1 } else { max2 };
}
pub const RANSAC_NUM_MOTIONS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_DIRECTIONS: core::ffi::c_int = 2 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn sb_all_skip(
    mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let maxr: core::ffi::c_int = if (*mi_params).mi_rows - mi_row
        < 64 as core::ffi::c_int >> 2 as core::ffi::c_int
    {
        (*mi_params).mi_rows - mi_row
    } else {
        64 as core::ffi::c_int >> 2 as core::ffi::c_int
    };
    let maxc: core::ffi::c_int = if (*mi_params).mi_cols - mi_col
        < 64 as core::ffi::c_int >> 2 as core::ffi::c_int
    {
        (*mi_params).mi_cols - mi_col
    } else {
        64 as core::ffi::c_int >> 2 as core::ffi::c_int
    };
    let stride: core::ffi::c_int = (*mi_params).mi_stride;
    let mut mbmi: *mut *mut MB_MODE_INFO = ((*mi_params).mi_grid_base)
        .offset((mi_row * stride) as isize)
        .offset(mi_col as isize);
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < maxr {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < maxc {
            if (**mbmi.offset(c as isize)).skip_txfm == 0 {
                return 0 as core::ffi::c_int;
            }
            c += 1;
        }
        r += 1;
        mbmi = mbmi.offset(stride as isize);
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn cdef_sb_skip(
    mi_params: *const CommonModeInfoParams,
    mut fbr: core::ffi::c_int,
    mut fbc: core::ffi::c_int,
) -> core::ffi::c_int {
    let mbmi: *const MB_MODE_INFO = *((*mi_params).mi_grid_base)
        .offset(
            (MI_SIZE_64X64 * fbr * (*mi_params).mi_stride + MI_SIZE_64X64 * fbc) as isize,
        );
    if sb_all_skip(mi_params, fbr * MI_SIZE_64X64, fbc * MI_SIZE_64X64) != 0 {
        return 1 as core::ffi::c_int;
    }
    if fbc & 1 as core::ffi::c_int != 0
        && ((*mbmi).bsize as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
            || (*mbmi).bsize as core::ffi::c_int == BLOCK_128X64 as core::ffi::c_int)
        || fbr & 1 as core::ffi::c_int != 0
            && ((*mbmi).bsize as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
                || (*mbmi).bsize as core::ffi::c_int == BLOCK_64X128 as core::ffi::c_int)
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn tf_alloc_and_reset_data(
    mut tf_data: *mut TemporalFilterData,
    mut num_pels: core::ffi::c_int,
    mut is_high_bitdepth: core::ffi::c_int,
) -> bool {
    (*tf_data).tmp_mbmi = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<MB_MODE_INFO>() as size_t,
    ) as *mut MB_MODE_INFO;
    (*tf_data).accum = aom_memalign(
        16 as size_t,
        (num_pels as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    ) as *mut uint32_t;
    (*tf_data).count = aom_memalign(
        16 as size_t,
        (num_pels as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    ) as *mut uint16_t;
    if is_high_bitdepth != 0 {
        (*tf_data).pred = (aom_memalign(
            32 as size_t,
            ((num_pels * 2 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
    } else {
        (*tf_data).pred = aom_memalign(
            32 as size_t,
            (num_pels as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
    }
    if !(!((*tf_data).tmp_mbmi).is_null() && !((*tf_data).accum).is_null()
        && !((*tf_data).count).is_null() && !((*tf_data).pred).is_null())
    {
        return false_0 != 0;
    }
    memset(
        &raw mut (*tf_data).diff as *mut FRAME_DIFF as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FRAME_DIFF>() as size_t,
    );
    return true_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn tf_setup_macroblockd(
    mut mbd: *mut MACROBLOCKD,
    mut tf_data: *mut TemporalFilterData,
    mut scale: *const scale_factors,
) {
    (*mbd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = scale;
    (*mbd).block_ref_scale_factors[1 as core::ffi::c_int as usize] = scale;
    (*mbd).mi = &raw mut (*tf_data).tmp_mbmi;
    (**((*mbd).mi).offset(0 as core::ffi::c_int as isize)).motion_mode = SIMPLE_TRANSLATION
        as core::ffi::c_int as MOTION_MODE;
}
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
unsafe extern "C-unwind" fn tf_save_state(
    mut mbd: *mut MACROBLOCKD,
    mut input_mbmi: *mut *mut *mut MB_MODE_INFO,
    mut input_buffer: *mut *mut uint8_t,
    mut num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        let fresh0 = &raw mut *input_buffer.offset(i as isize);
        *fresh0 = (*mbd).plane[i as usize].pre[0 as core::ffi::c_int as usize].buf;
        i += 1;
    }
    *input_mbmi = (*mbd).mi;
}
#[inline]
unsafe extern "C-unwind" fn tf_restore_state(
    mut mbd: *mut MACROBLOCKD,
    mut input_mbmi: *mut *mut MB_MODE_INFO,
    mut input_buffer: *mut *mut uint8_t,
    mut num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        (*mbd).plane[i as usize].pre[0 as core::ffi::c_int as usize].buf = *input_buffer
            .offset(i as isize);
        i += 1;
    }
    (*mbd).mi = input_mbmi;
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
#[inline]
unsafe extern "C-unwind" fn tpl_alloc_temp_buffers(
    mut tpl_tmp_buffers: *mut TplBuffers,
    mut tpl_bsize_1d: uint8_t,
) -> bool {
    let tpl_block_pels: core::ffi::c_int = tpl_bsize_1d as core::ffi::c_int
        * tpl_bsize_1d as core::ffi::c_int;
    (*tpl_tmp_buffers).predictor8 = aom_memalign(
        32 as size_t,
        ((tpl_block_pels * 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    (*tpl_tmp_buffers).src_diff = aom_memalign(
        32 as size_t,
        (tpl_block_pels as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    (*tpl_tmp_buffers).coeff = aom_memalign(
        32 as size_t,
        (tpl_block_pels as size_t)
            .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    ) as *mut tran_low_t;
    (*tpl_tmp_buffers).qcoeff = aom_memalign(
        32 as size_t,
        (tpl_block_pels as size_t)
            .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    ) as *mut tran_low_t;
    (*tpl_tmp_buffers).dqcoeff = aom_memalign(
        32 as size_t,
        (tpl_block_pels as size_t)
            .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    ) as *mut tran_low_t;
    if !(!((*tpl_tmp_buffers).predictor8).is_null()
        && !((*tpl_tmp_buffers).src_diff).is_null()
        && !((*tpl_tmp_buffers).coeff).is_null()
        && !((*tpl_tmp_buffers).qcoeff).is_null()
        && !((*tpl_tmp_buffers).dqcoeff).is_null())
    {
        tpl_dealloc_temp_buffers(tpl_tmp_buffers);
        return false_0 != 0;
    }
    return true_0 != 0;
}
pub const MAX_PARALLEL_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_num_blocks(
    frame_length: core::ffi::c_int,
    mb_length: core::ffi::c_int,
) -> core::ffi::c_int {
    return (frame_length + mb_length - 1 as core::ffi::c_int) / mb_length;
}
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
unsafe extern "C-unwind" fn av1_need_dv_costs(cpi: *const AV1_COMP) -> bool {
    return ((*cpi).sf.rt_sf.use_nonrd_pick_mode == 0
        || (*cpi).sf.rt_sf.rt_use_intrabc != 0) && av1_allow_intrabc(&(*cpi).common) != 0
        && is_stat_generation_stage(cpi) == 0;
}
#[inline]
unsafe extern "C-unwind" fn is_loopfilter_used(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (!(*cm).features.coded_lossless && (*cm).tiles.large_scale == 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_cdef_used(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*(*cm).seq_params).enable_cdef as core::ffi::c_int != 0
        && !(*cm).features.coded_lossless && (*cm).tiles.large_scale == 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_restoration_used(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*(*cm).seq_params).enable_restoration as core::ffi::c_int != 0
        && !(*cm).features.all_lossless && (*cm).tiles.large_scale == 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn derive_skip_apply_postproc_filters(
    mut cpi: *const AV1_COMP,
    mut use_loopfilter: core::ffi::c_int,
    mut use_cdef: core::ffi::c_int,
    mut use_superres: core::ffi::c_int,
    mut use_restoration: core::ffi::c_int,
) -> core::ffi::c_uint {
    if (*(*cpi).ppi).rtc_ref.non_reference_frame != 0 {
        return (SKIP_APPLY_LOOPFILTER as core::ffi::c_int
            | SKIP_APPLY_CDEF as core::ffi::c_int) as core::ffi::c_uint;
    }
    if !(*cpi).oxcf.algo_cfg.skip_postproc_filtering
        || (*(*cpi).ppi).b_calculate_psnr != 0
    {
        return 0 as core::ffi::c_uint;
    }
    if !((*cpi).oxcf.mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"derive_skip_apply_postproc_filters\0"))
                .as_ptr(),
            b"encoder.h\0" as *const u8 as *const core::ffi::c_char,
            4490 as core::ffi::c_int,
            b"cpi->oxcf.mode == ALLINTRA\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if use_restoration != 0 {
        return SKIP_APPLY_RESTORATION as core::ffi::c_int as core::ffi::c_uint;
    }
    if use_superres != 0 {
        return SKIP_APPLY_SUPERRES as core::ffi::c_int as core::ffi::c_uint;
    }
    if use_cdef != 0 {
        return (if (*cpi).sf.lpf_sf.cdef_pick_method as core::ffi::c_uint
            == CDEF_PICK_FROM_Q as core::ffi::c_int as core::ffi::c_uint
            && use_loopfilter != 0
        {
            SKIP_APPLY_LOOPFILTER as core::ffi::c_int
                | SKIP_APPLY_CDEF as core::ffi::c_int
        } else {
            SKIP_APPLY_CDEF as core::ffi::c_int
        }) as core::ffi::c_uint;
    }
    if use_loopfilter != 0 {
        return SKIP_APPLY_LOOPFILTER as core::ffi::c_int as core::ffi::c_uint;
    }
    return 0 as core::ffi::c_uint;
}
#[inline]
unsafe extern "C-unwind" fn set_postproc_filter_default_params(mut cm: *mut AV1_COMMON) {
    let lf: *mut loopfilter = &raw mut (*cm).lf;
    let cdef_info: *mut CdefInfo = &raw mut (*cm).cdef_info;
    let rst_info: *mut RestorationInfo = ((*cm).rst_info).as_mut_ptr();
    (*lf).filter_level[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    (*lf).filter_level[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    (*lf).backup_filter_level[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    (*lf).backup_filter_level[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    (*cdef_info).cdef_bits = 0 as core::ffi::c_int;
    (*cdef_info).cdef_strengths[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    (*cdef_info).nb_cdef_strengths = 1 as core::ffi::c_int;
    (*cdef_info).cdef_uv_strengths[0 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int;
    (*rst_info.offset(0 as core::ffi::c_int as isize)).frame_restoration_type = RESTORE_NONE;
    (*rst_info.offset(1 as core::ffi::c_int as isize)).frame_restoration_type = RESTORE_NONE;
    (*rst_info.offset(2 as core::ffi::c_int as isize)).frame_restoration_type = RESTORE_NONE;
}
#[inline]
unsafe extern "C-unwind" fn is_inter_tx_size_search_level_one(
    mut tx_sf: *const TX_SPEED_FEATURES,
) -> core::ffi::c_int {
    return ((*tx_sf).inter_tx_size_search_init_depth_rect >= 1 as core::ffi::c_int
        && (*tx_sf).inter_tx_size_search_init_depth_sqr >= 1 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_lpf_opt_level(
    mut sf: *const SPEED_FEATURES,
) -> core::ffi::c_int {
    let mut lpf_opt_level: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_inter_tx_size_search_level_one(&(*sf).tx_sf) != 0 {
        lpf_opt_level = if (*sf).lpf_sf.lpf_pick as core::ffi::c_int
            == LPF_PICK_FROM_Q as core::ffi::c_int
        {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
    }
    return lpf_opt_level;
}
static mut winner_mode_count_allowed: [core::ffi::c_int; 3] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
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
unsafe extern "C-unwind" fn allocate_winner_mode_stats(
    mut cpi: *const AV1_COMP,
    mut mb: *mut macroblock,
) {
    let mut sf: *const SPEED_FEATURES = &(*cpi).sf;
    if is_stat_generation_stage(cpi) != 0
        || (*sf).rt_sf.use_nonrd_pick_mode != 0 && (*sf).rt_sf.hybrid_intra_pickmode == 0
        || (*sf).winner_mode_sf.multi_winner_mode_type as core::ffi::c_int
            == MULTI_WINNER_MODE_OFF as core::ffi::c_int
    {
        return;
    }
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let winner_mode_count: core::ffi::c_int = winner_mode_count_allowed[(*sf)
        .winner_mode_sf
        .multi_winner_mode_type as usize];
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"allocate_winner_mode_stats\0"))
                .as_ptr(),
            b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
            463 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*mb).winner_mode_stats = aom_malloc(
        (winner_mode_count as size_t)
            .wrapping_mul(::core::mem::size_of::<WinnerModeStats>() as size_t),
    ) as *mut WinnerModeStats;
    if ((*mb).winner_mode_stats).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate mb->winner_mode_stats at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encodeframe_utils.h\0" as *const u8
                as *const core::ffi::c_char,
            463 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn av1_alloc_mb_data(
    mut cpi: *const AV1_COMP,
    mut mb: *mut macroblock,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut sf: *const SPEED_FEATURES = &(*cpi).sf;
    if (*sf).rt_sf.use_nonrd_pick_mode == 0 {
        if (*sf).rd_sf.use_mb_rd_hash != 0 {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"av1_alloc_mb_data\0"))
                        .as_ptr(),
                    b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
                    477 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*mb).txfm_search_info.mb_rd_record = aom_malloc(
                ::core::mem::size_of::<MB_RD_RECORD>() as size_t,
            ) as *mut MB_RD_RECORD;
            if ((*mb).txfm_search_info.mb_rd_record).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate mb->txfm_search_info.mb_rd_record at %s:%d\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/encodeframe_utils.h\0" as *const u8
                        as *const core::ffi::c_char,
                    477 as core::ffi::c_int,
                );
            }
        }
        if frame_is_intra_only(cm) == 0 {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"av1_alloc_mb_data\0"))
                        .as_ptr(),
                    b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
                    481 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*mb).inter_modes_info = aom_malloc(
                ::core::mem::size_of::<inter_modes_info>() as size_t,
            ) as *mut InterModesInfo as *mut inter_modes_info;
            if ((*mb).inter_modes_info).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate mb->inter_modes_info at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/encodeframe_utils.h\0" as *const u8
                        as *const core::ffi::c_char,
                    481 as core::ffi::c_int,
                );
            }
        }
    }
    av1_alloc_src_diff_buf(cm as *const AV1Common, mb);
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_alloc_mb_data\0"))
                .as_ptr(),
            b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
            488 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*mb).e_mbd.seg_mask = aom_memalign(
        16 as size_t,
        ((2 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*mb).e_mbd.seg_mask).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate mb->e_mbd.seg_mask at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encodeframe_utils.h\0" as *const u8
                as *const core::ffi::c_char,
            488 as core::ffi::c_int,
        );
    }
    allocate_winner_mode_stats(cpi, mb);
    let max_sb_square_y: core::ffi::c_int = (1 as core::ffi::c_int)
        << num_pels_log2_lookup[(*(*cm).seq_params).sb_size as usize]
            as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_alloc_mb_data\0"))
                .as_ptr(),
            b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
            496 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*mb).dqcoeff_buf = aom_memalign(
        32 as size_t,
        (max_sb_square_y as size_t)
            .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    ) as *mut tran_low_t;
    if ((*mb).dqcoeff_buf).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate mb->dqcoeff_buf at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encodeframe_utils.h\0" as *const u8
                as *const core::ffi::c_char,
            496 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn gm_alloc_data(
    mut cpi: *mut AV1_COMP,
    mut gm_data: *mut GlobalMotionData,
) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut gm_info: *mut GlobalMotionInfo = &raw mut (*cpi).gm_info;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"gm_alloc_data\0"))
                .as_ptr(),
            b"global_motion_facade.h\0" as *const u8 as *const core::ffi::c_char,
            28 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*gm_data).segment_map = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul((*gm_info).segment_map_w as size_t)
            .wrapping_mul((*gm_info).segment_map_h as size_t),
    ) as *mut uint8_t;
    if ((*gm_data).segment_map).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate gm_data->segment_map at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/global_motion_facade.h\0" as *const u8
                as *const core::ffi::c_char,
            28 as core::ffi::c_int,
        );
    }
    memset(
        ((*gm_data).motion_models).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (1 as size_t).wrapping_mul(::core::mem::size_of::<MotionModel>() as size_t),
    );
    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
    while m < RANSAC_NUM_MOTIONS {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"gm_alloc_data\0"))
                    .as_ptr(),
                b"global_motion_facade.h\0" as *const u8 as *const core::ffi::c_char,
                34 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*gm_data).motion_models[m as usize].inliers = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(2 as size_t)
                .wrapping_mul(4096 as size_t),
        ) as *mut core::ffi::c_int;
        if ((*gm_data).motion_models[m as usize].inliers).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate gm_data->motion_models[m].inliers at %s:%d\0"
                    as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/global_motion_facade.h\0" as *const u8
                    as *const core::ffi::c_char,
                34 as core::ffi::c_int,
            );
        }
        m += 1;
    }
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
unsafe extern "C-unwind" fn is_src_var_for_4x4_sub_blocks_caching_enabled(
    cpi: *const AV1_COMP,
) -> bool {
    let sf: *const SPEED_FEATURES = &(*cpi).sf;
    if (*cpi).oxcf.mode as core::ffi::c_int != ALLINTRA as core::ffi::c_int {
        return false_0 != 0;
    }
    if (*sf).part_sf.partition_search_type as core::ffi::c_int
        == SEARCH_PARTITION as core::ffi::c_int
    {
        return true_0 != 0;
    }
    if 1.0f64 - 0.25f64 * (*cpi).oxcf.speed as core::ffi::c_double
        <= 0 as core::ffi::c_int as core::ffi::c_double
        || (*sf).rt_sf.use_nonrd_pick_mode != 0 && (*sf).rt_sf.hybrid_intra_pickmode == 0
    {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn alloc_obmc_buffers(
    mut obmc_buffer: *mut OBMCBuffer,
    mut error: *mut aom_internal_error_info,
) {
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"alloc_obmc_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            138 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*obmc_buffer).wsrc = aom_memalign(
        16 as size_t,
        ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    ) as *mut int32_t;
    if ((*obmc_buffer).wsrc).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate obmc_buffer->wsrc at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            138 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"alloc_obmc_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            141 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*obmc_buffer).mask = aom_memalign(
        16 as size_t,
        ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    ) as *mut int32_t;
    if ((*obmc_buffer).mask).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate obmc_buffer->mask at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            141 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"alloc_obmc_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            145 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*obmc_buffer).above_pred = aom_memalign(
        16 as size_t,
        ((3 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*obmc_buffer).above_pred).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate obmc_buffer->above_pred at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            145 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"alloc_obmc_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            149 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*obmc_buffer).left_pred = aom_memalign(
        16 as size_t,
        ((3 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*obmc_buffer).left_pred).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate obmc_buffer->left_pred at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            149 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn alloc_compound_type_rd_buffers(
    mut error: *mut aom_internal_error_info,
    bufs: *mut CompoundTypeRdBuffers,
) {
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"alloc_compound_type_rd_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            168 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*bufs).pred0 = aom_memalign(
        16 as size_t,
        ((2 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*bufs).pred0).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bufs->pred0 at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            168 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"alloc_compound_type_rd_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            171 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*bufs).pred1 = aom_memalign(
        16 as size_t,
        ((2 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*bufs).pred1).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bufs->pred1 at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            171 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"alloc_compound_type_rd_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            174 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*bufs).residual1 = aom_memalign(
        32 as size_t,
        ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    if ((*bufs).residual1).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bufs->residual1 at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            174 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"alloc_compound_type_rd_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            177 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*bufs).diff10 = aom_memalign(
        32 as size_t,
        ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    if ((*bufs).diff10).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bufs->diff10 at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            177 as core::ffi::c_int,
        );
    }
    if ((*error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"alloc_compound_type_rd_buffers\0"))
                .as_ptr(),
            b"encoder_alloc.h\0" as *const u8 as *const core::ffi::c_char,
            180 as core::ffi::c_int,
            b"(error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*bufs).tmp_best_mask_buf = aom_malloc(
        ((2 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*bufs).tmp_best_mask_buf).is_null() {
        aom_internal_error(
            error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bufs->tmp_best_mask_buf at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encoder_alloc.h\0" as *const u8
                as *const core::ffi::c_char,
            180 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn accumulate_rd_opt(
    mut td: *mut ThreadData,
    mut td_t: *mut ThreadData,
) {
    (*td).rd_counts.compound_ref_used_flag |= (*td_t).rd_counts.compound_ref_used_flag;
    (*td).rd_counts.skip_mode_used_flag |= (*td_t).rd_counts.skip_mode_used_flag;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < TX_SIZES_ALL as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < TX_TYPES as core::ffi::c_int {
            (*td).rd_counts.tx_type_used[i as usize][j as usize]
                += (*td_t).rd_counts.tx_type_used[i as usize][j as usize];
            j += 1;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < BLOCK_SIZES_ALL as core::ffi::c_int {
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < 2 as core::ffi::c_int {
            (*td).rd_counts.obmc_used[i_0 as usize][j_0 as usize]
                += (*td_t).rd_counts.obmc_used[i_0 as usize][j_0 as usize];
            j_0 += 1;
        }
        i_0 += 1;
    }
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < 2 as core::ffi::c_int {
        (*td).rd_counts.warped_used[i_1 as usize]
            += (*td_t).rd_counts.warped_used[i_1 as usize];
        i_1 += 1;
    }
    (*td).rd_counts.seg_tmp_pred_cost[0 as core::ffi::c_int as usize] = ((*td)
        .rd_counts
        .seg_tmp_pred_cost[0 as core::ffi::c_int as usize])
        .wrapping_add(
            (*td_t).rd_counts.seg_tmp_pred_cost[0 as core::ffi::c_int as usize],
        );
    (*td).rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize] = ((*td)
        .rd_counts
        .seg_tmp_pred_cost[1 as core::ffi::c_int as usize])
        .wrapping_add(
            (*td_t).rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize],
        );
    (*td).rd_counts.newmv_or_intra_blocks += (*td_t).rd_counts.newmv_or_intra_blocks;
}
#[inline]
unsafe extern "C-unwind" fn update_delta_lf_for_row_mt(mut cpi: *mut AV1_COMP) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*cpi).td.mb.e_mbd;
    let mib_size: core::ffi::c_int = (*(*cm).seq_params).mib_size;
    let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm) > 1 as core::ffi::c_int
    {
        FRAME_LF_COUNT
    } else {
        FRAME_LF_COUNT - 2 as core::ffi::c_int
    };
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < (*cm).tiles.rows {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < (*cm).tiles.cols {
            let mut tile_data: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset((row * (*cm).tiles.cols + col) as isize) as *mut TileDataEnc;
            let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
            let mut mi_row: core::ffi::c_int = (*tile_info).mi_row_start;
            while mi_row < (*tile_info).mi_row_end {
                if mi_row == (*tile_info).mi_row_start {
                    av1_reset_loop_filter_delta(xd, av1_num_planes(cm));
                }
                let mut mi_col: core::ffi::c_int = (*tile_info).mi_col_start;
                while mi_col < (*tile_info).mi_col_end {
                    let idx_str: core::ffi::c_int = (*cm).mi_params.mi_stride * mi_row
                        + mi_col;
                    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
                        .offset(idx_str as isize);
                    let mut mbmi: *mut MB_MODE_INFO = *mi
                        .offset(0 as core::ffi::c_int as isize);
                    if (*mbmi).skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int
                        && (*mbmi).bsize as core::ffi::c_int
                            == (*(*cm).seq_params).sb_size as core::ffi::c_int
                    {
                        let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
                        while lf_id < frame_lf_count {
                            (*mbmi).delta_lf[lf_id as usize] = (*xd)
                                .delta_lf[lf_id as usize];
                            lf_id += 1;
                        }
                        (*mbmi).delta_lf_from_base = (*xd).delta_lf_from_base;
                    } else if (*cm).delta_q_info.delta_lf_multi != 0 {
                        let mut lf_id_0: core::ffi::c_int = 0 as core::ffi::c_int;
                        while lf_id_0 < frame_lf_count {
                            (*xd).delta_lf[lf_id_0 as usize] = (*mbmi)
                                .delta_lf[lf_id_0 as usize];
                            lf_id_0 += 1;
                        }
                    } else {
                        (*xd).delta_lf_from_base = (*mbmi).delta_lf_from_base;
                    }
                    mi_col += mib_size;
                }
                mi_row += mib_size;
            }
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_sync_read_dummy(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_sync_write_dummy(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut cols: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_sync_read(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*row_mt_sync).sync_range;
    if r != 0 {
        let mutex: *mut pthread_mutex_t = &raw mut *((*row_mt_sync).mutex_)
            .offset((r - 1 as core::ffi::c_int) as isize) as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex as *mut pthread_mutex_t);
        while c
            > *((*row_mt_sync).num_finished_cols)
                .offset((r - 1 as core::ffi::c_int) as isize) - nsync
                - (*row_mt_sync).intrabc_extra_top_right_sb_delay
        {
            pthread_cond_wait(
                &raw mut *((*row_mt_sync).cond_)
                    .offset((r - 1 as core::ffi::c_int) as isize),
                mutex as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock(mutex as *mut pthread_mutex_t);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_sync_write(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut cols: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*row_mt_sync).sync_range;
    let mut cur: core::ffi::c_int = 0;
    let mut sig: core::ffi::c_int = 1 as core::ffi::c_int;
    if c < cols - 1 as core::ffi::c_int {
        cur = c;
        if c % nsync != 0 {
            sig = 0 as core::ffi::c_int;
        }
    } else {
        cur = cols + nsync + (*row_mt_sync).intrabc_extra_top_right_sb_delay;
    }
    if sig != 0 {
        pthread_mutex_lock(&raw mut *((*row_mt_sync).mutex_).offset(r as isize));
        *((*row_mt_sync).num_finished_cols).offset(r as isize) = if *((*row_mt_sync)
            .num_finished_cols)
            .offset(r as isize) > cur
        {
            *((*row_mt_sync).num_finished_cols).offset(r as isize)
        } else {
            cur
        };
        pthread_cond_signal(&raw mut *((*row_mt_sync).cond_).offset(r as isize));
        pthread_mutex_unlock(&raw mut *((*row_mt_sync).mutex_).offset(r as isize));
    }
}
unsafe extern "C-unwind" fn row_mt_sync_mem_alloc(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut cm: *mut AV1_COMMON,
    mut rows: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"row_mt_sync_mem_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            184 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*row_mt_sync).mutex_ = aom_malloc(
        (::core::mem::size_of::<pthread_mutex_t>() as size_t)
            .wrapping_mul(rows as size_t),
    ) as *mut pthread_mutex_t;
    if ((*row_mt_sync).mutex_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate row_mt_sync->mutex_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            184 as core::ffi::c_int,
        );
    }
    if !((*row_mt_sync).mutex_).is_null() {
        i = 0 as core::ffi::c_int;
        while i < rows {
            pthread_mutex_init(
                &raw mut *((*row_mt_sync).mutex_).offset(i as isize),
                0 as *const pthread_mutexattr_t,
            );
            i += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"row_mt_sync_mem_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            192 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*row_mt_sync).cond_ = aom_malloc(
        (::core::mem::size_of::<pthread_cond_t>() as size_t).wrapping_mul(rows as size_t),
    ) as *mut pthread_cond_t;
    if ((*row_mt_sync).cond_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate row_mt_sync->cond_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            192 as core::ffi::c_int,
        );
    }
    if !((*row_mt_sync).cond_).is_null() {
        i = 0 as core::ffi::c_int;
        while i < rows {
            pthread_cond_init(
                &raw mut *((*row_mt_sync).cond_).offset(i as isize),
                0 as *const pthread_condattr_t,
            );
            i += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"row_mt_sync_mem_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            201 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*row_mt_sync).num_finished_cols = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(rows as size_t),
    ) as *mut core::ffi::c_int;
    if ((*row_mt_sync).num_finished_cols).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate row_mt_sync->num_finished_cols at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            201 as core::ffi::c_int,
        );
    }
    (*row_mt_sync).rows = rows;
    (*row_mt_sync).sync_range = 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_sync_mem_dealloc(
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
) {
    if !row_mt_sync.is_null() {
        let mut i: core::ffi::c_int = 0;
        if !((*row_mt_sync).mutex_).is_null() {
            i = 0 as core::ffi::c_int;
            while i < (*row_mt_sync).rows {
                pthread_mutex_destroy(&raw mut *((*row_mt_sync).mutex_).offset(i as isize));
                i += 1;
            }
            aom_free((*row_mt_sync).mutex_ as *mut core::ffi::c_void);
        }
        if !((*row_mt_sync).cond_).is_null() {
            i = 0 as core::ffi::c_int;
            while i < (*row_mt_sync).rows {
                pthread_cond_destroy(&raw mut *((*row_mt_sync).cond_).offset(i as isize));
                i += 1;
            }
            aom_free((*row_mt_sync).cond_ as *mut core::ffi::c_void);
        }
        aom_free((*row_mt_sync).num_finished_cols as *mut core::ffi::c_void);
        memset(
            row_mt_sync as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1EncRowMultiThreadSync>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn get_sb_rows_in_frame(mut cm: *mut AV1_COMMON) -> core::ffi::c_int {
    return (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*(*cm).seq_params).mib_size_log2;
}
unsafe extern "C-unwind" fn row_mt_mem_alloc(
    mut cpi: *mut AV1_COMP,
    mut max_rows: core::ffi::c_int,
    mut max_cols: core::ffi::c_int,
    mut alloc_row_ctx: core::ffi::c_int,
) {
    let mut cm: *mut AV1Common = &raw mut (*cpi).common;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut tile_col: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = 0;
    av1_row_mt_mem_dealloc(cpi);
    if !((*cpi).allocated_tiles == tile_cols * tile_rows) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"row_mt_mem_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            252 as core::ffi::c_int,
            b"cpi->allocated_tiles == tile_cols * tile_rows\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    tile_row = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        tile_col = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let mut tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_index as isize) as *mut TileDataEnc;
            row_mt_sync_mem_alloc(
                &raw mut (*this_tile).row_mt_sync,
                cm as *mut AV1_COMMON,
                max_rows,
            );
            if alloc_row_ctx != 0 {
                if !(max_cols > 0 as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 17],
                            [core::ffi::c_char; 17],
                        >(*b"row_mt_mem_alloc\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        261 as core::ffi::c_int,
                        b"max_cols > 0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let num_row_ctx: core::ffi::c_int = if 1 as core::ffi::c_int
                    > max_cols - 1 as core::ffi::c_int
                {
                    1 as core::ffi::c_int
                } else {
                    max_cols - 1 as core::ffi::c_int
                };
                if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 17],
                            [core::ffi::c_char; 17],
                        >(*b"row_mt_mem_alloc\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        265 as core::ffi::c_int,
                        b"((cm)->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*this_tile).row_ctx = aom_memalign(
                    16 as size_t,
                    (num_row_ctx as size_t)
                        .wrapping_mul(::core::mem::size_of::<FRAME_CONTEXT>() as size_t),
                ) as *mut FRAME_CONTEXT;
                if ((*this_tile).row_ctx).is_null() {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate this_tile->row_ctx at %s:%d\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        265 as core::ffi::c_int,
                    );
                }
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
    (*enc_row_mt).allocated_tile_cols = tile_cols;
    (*enc_row_mt).allocated_tile_rows = tile_rows;
    let sb_rows: core::ffi::c_int = get_sb_rows_in_frame(cm as *mut AV1_COMMON)
        as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"row_mt_mem_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            274 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*enc_row_mt).num_tile_cols_done = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(sb_rows as size_t),
    ) as *mut core::ffi::c_int;
    if ((*enc_row_mt).num_tile_cols_done).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate enc_row_mt->num_tile_cols_done at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            274 as core::ffi::c_int,
        );
    }
    (*enc_row_mt).allocated_rows = max_rows;
    (*enc_row_mt).allocated_cols = max_cols - 1 as core::ffi::c_int;
    (*enc_row_mt).allocated_sb_rows = sb_rows;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_row_mt_mem_dealloc(mut cpi: *mut AV1_COMP) {
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let tile_cols: core::ffi::c_int = (*enc_row_mt).allocated_tile_cols;
    let tile_rows: core::ffi::c_int = (*enc_row_mt).allocated_tile_rows;
    let mut tile_col: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = 0;
    tile_row = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        tile_col = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let mut tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_index as isize) as *mut TileDataEnc;
            av1_row_mt_sync_mem_dealloc(&raw mut (*this_tile).row_mt_sync);
            if (*cpi).oxcf.algo_cfg.cdf_update_mode != 0 {
                aom_free((*this_tile).row_ctx as *mut core::ffi::c_void);
                (*this_tile).row_ctx = 0 as *mut FRAME_CONTEXT;
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
    (*enc_row_mt).allocated_tile_cols = 0 as core::ffi::c_int;
    (*enc_row_mt).allocated_tile_rows = 0 as core::ffi::c_int;
    aom_free((*enc_row_mt).num_tile_cols_done as *mut core::ffi::c_void);
    (*enc_row_mt).num_tile_cols_done = 0 as *mut core::ffi::c_int;
    (*enc_row_mt).allocated_rows = 0 as core::ffi::c_int;
    (*enc_row_mt).allocated_cols = 0 as core::ffi::c_int;
    (*enc_row_mt).allocated_sb_rows = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn assign_tile_to_thread(
    mut thread_id_to_tile_id: *mut core::ffi::c_int,
    mut num_tiles: core::ffi::c_int,
    mut num_workers: core::ffi::c_int,
) {
    let mut tile_id: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < num_workers {
        let fresh3 = tile_id;
        tile_id = tile_id + 1;
        *thread_id_to_tile_id.offset(i as isize) = fresh3;
        if tile_id == num_tiles {
            tile_id = 0 as core::ffi::c_int;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_next_job(
    tile_data: *mut TileDataEnc,
    mut current_mi_row: *mut core::ffi::c_int,
    mut mib_size: core::ffi::c_int,
) -> core::ffi::c_int {
    let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*tile_data).row_mt_sync;
    let mi_row_end: core::ffi::c_int = (*tile_data).tile_info.mi_row_end;
    if (*row_mt_sync).next_mi_row < mi_row_end {
        *current_mi_row = (*row_mt_sync).next_mi_row;
        (*row_mt_sync).num_threads_working += 1;
        (*row_mt_sync).next_mi_row += mib_size;
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn switch_tile_and_get_next_job(
    cm: *mut AV1_COMMON,
    tile_data: *mut TileDataEnc,
    mut cur_tile_id: *mut core::ffi::c_int,
    mut current_mi_row: *mut core::ffi::c_int,
    mut end_of_frame: *mut core::ffi::c_int,
    mut is_firstpass: core::ffi::c_int,
    fp_block_size: BLOCK_SIZE,
) {
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut tile_id: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut max_mis_to_encode: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut min_num_threads_working: core::ffi::c_int = INT_MAX;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let mut tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *tile_data.offset(tile_index as isize)
                as *mut TileDataEnc;
            let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile)
                .row_mt_sync;
            let mut num_b_rows_in_tile: core::ffi::c_int = if is_firstpass != 0 {
                av1_get_unit_rows_in_tile(&raw mut (*this_tile).tile_info, fp_block_size)
            } else {
                av1_get_sb_rows_in_tile(cm, &raw mut (*this_tile).tile_info)
            };
            let mut num_b_cols_in_tile: core::ffi::c_int = if is_firstpass != 0 {
                av1_get_unit_cols_in_tile(&raw mut (*this_tile).tile_info, fp_block_size)
            } else {
                av1_get_sb_cols_in_tile(cm, &raw mut (*this_tile).tile_info)
            };
            let mut theoretical_limit_on_threads: core::ffi::c_int = if (num_b_cols_in_tile
                + 1 as core::ffi::c_int >> 1 as core::ffi::c_int) < num_b_rows_in_tile
            {
                num_b_cols_in_tile + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
            } else {
                num_b_rows_in_tile
            };
            let mut num_threads_working: core::ffi::c_int = (*row_mt_sync)
                .num_threads_working;
            if num_threads_working < theoretical_limit_on_threads {
                let mut num_mis_to_encode: core::ffi::c_int = (*this_tile)
                    .tile_info
                    .mi_row_end - (*row_mt_sync).next_mi_row;
                if num_mis_to_encode > 0 as core::ffi::c_int {
                    if num_threads_working < min_num_threads_working {
                        min_num_threads_working = num_threads_working;
                        max_mis_to_encode = 0 as core::ffi::c_int;
                    }
                    if num_threads_working == min_num_threads_working
                        && num_mis_to_encode > max_mis_to_encode
                    {
                        tile_id = tile_index;
                        max_mis_to_encode = num_mis_to_encode;
                    }
                }
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
    if tile_id == -(1 as core::ffi::c_int) {
        *end_of_frame = 1 as core::ffi::c_int;
    } else {
        *cur_tile_id = tile_id;
        let unit_height: core::ffi::c_int = mi_size_high[fp_block_size as usize]
            as core::ffi::c_int;
        get_next_job(
            &raw mut *tile_data.offset(tile_id as isize),
            current_mi_row,
            if is_firstpass != 0 { unit_height } else { (*(*cm).seq_params).mib_size },
        );
    };
}
unsafe extern "C-unwind" fn set_firstpass_encode_done(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let fp_block_size: BLOCK_SIZE = (*cpi).fp_block_size;
    let unit_height: core::ffi::c_int = mi_size_high[fp_block_size as usize]
        as core::ffi::c_int;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let tile_data: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset((tile_row * tile_cols + tile_col) as isize) as *mut TileDataEnc;
            let mut tile: *mut TileInfo = &raw mut (*tile_data).tile_info;
            let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*tile_data)
                .row_mt_sync;
            let unit_cols_in_tile: core::ffi::c_int = av1_get_unit_cols_in_tile(
                tile,
                fp_block_size,
            ) as core::ffi::c_int;
            let mut mi_row: core::ffi::c_int = (*tile).mi_row_start;
            let mut unit_row_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
            while mi_row < (*tile).mi_row_end {
                ((*enc_row_mt).sync_write_ptr)
                    .expect(
                        "non-null function pointer",
                    )(
                    row_mt_sync,
                    unit_row_in_tile,
                    unit_cols_in_tile - 1 as core::ffi::c_int,
                    unit_cols_in_tile,
                );
                mi_row += unit_height;
                unit_row_in_tile += 1;
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
}
unsafe extern "C-unwind" fn fp_enc_row_mt_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut thread_id: core::ffi::c_int = (*thread_data).thread_id;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let mut enc_row_mt_mutex_: *mut pthread_mutex_t = (*enc_row_mt).mutex_;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut cur_tile_id: core::ffi::c_int = (*enc_row_mt)
        .thread_id_to_tile_id[thread_id as usize];
    if !(cur_tile_id != -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"fp_enc_row_mt_worker_hook\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            471 as core::ffi::c_int,
            b"cur_tile_id != -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let fp_block_size: BLOCK_SIZE = (*cpi).fp_block_size;
    let unit_height: core::ffi::c_int = mi_size_high[fp_block_size as usize]
        as core::ffi::c_int;
    let mut end_of_frame: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let mut current_mi_row: core::ffi::c_int = -(1 as core::ffi::c_int);
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        let mut firstpass_mt_exit: bool = (*enc_row_mt).firstpass_mt_exit;
        if !firstpass_mt_exit
            && get_next_job(
                &raw mut *((*cpi).tile_data).offset(cur_tile_id as isize),
                &raw mut current_mi_row,
                unit_height,
            ) == 0
        {
            switch_tile_and_get_next_job(
                cm,
                (*cpi).tile_data,
                &raw mut cur_tile_id,
                &raw mut current_mi_row,
                &raw mut end_of_frame,
                1 as core::ffi::c_int,
                fp_block_size,
            );
        }
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        if firstpass_mt_exit as core::ffi::c_int != 0 || end_of_frame != 0 {
            break;
        }
        let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
            .offset(cur_tile_id as isize) as *mut TileDataEnc;
        let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile).row_mt_sync;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if !(current_mi_row != -(1 as core::ffi::c_int)
            && current_mi_row < (*this_tile).tile_info.mi_row_end) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 26],
                    [core::ffi::c_char; 26],
                >(*b"fp_enc_row_mt_worker_hook\0"))
                    .as_ptr(),
                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                502 as core::ffi::c_int,
                b"current_mi_row != -1 && current_mi_row < this_tile->tile_info.mi_row_end\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let unit_height_log2: core::ffi::c_int = mi_size_high_log2[fp_block_size
            as usize] as core::ffi::c_int;
        av1_first_pass_row(
            cpi as *mut AV1_COMP,
            td as *mut ThreadData,
            this_tile as *mut TileDataEnc,
            current_mi_row >> unit_height_log2,
            fp_block_size,
        );
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        (*row_mt_sync).num_threads_working -= 1;
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        (*enc_row_mt).firstpass_mt_exit = true_0 != 0;
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        set_firstpass_encode_done(cpi);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn launch_loop_filter_rows(
    mut cm: *mut AV1_COMMON,
    mut thread_data: *mut EncWorkerData,
    mut enc_row_mt: *mut AV1EncRowMultiThreadInfo,
    mut mib_size_log2: core::ffi::c_int,
) {
    let lf_sync: *mut AV1LfSync = (*thread_data).lf_sync;
    let sb_rows: core::ffi::c_int = get_sb_rows_in_frame(cm) as core::ffi::c_int;
    let mut cur_job_info: *mut AV1LfMTInfo = 0 as *mut AV1LfMTInfo;
    let mut row_mt_exit: bool = false_0 != 0;
    let mut enc_row_mt_mutex_: *mut pthread_mutex_t = (*enc_row_mt).mutex_;
    loop {
        cur_job_info = get_lf_job_info(lf_sync);
        if cur_job_info.is_null() {
            break;
        }
        let lf_data: *mut LFWorkerData = (*thread_data).lf_data;
        let lpf_opt_level: core::ffi::c_int = (*cur_job_info).lpf_opt_level;
        let cur_sb_row: core::ffi::c_int = (*cur_job_info).mi_row >> mib_size_log2;
        let next_sb_row: core::ffi::c_int = if (sb_rows - 1 as core::ffi::c_int)
            < cur_sb_row + 1 as core::ffi::c_int
        {
            sb_rows - 1 as core::ffi::c_int
        } else {
            cur_sb_row + 1 as core::ffi::c_int
        };
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        while !(*enc_row_mt).row_mt_exit
            && (*((*enc_row_mt).num_tile_cols_done).offset(cur_sb_row as isize)
                < (*cm).tiles.cols
                || *((*enc_row_mt).num_tile_cols_done).offset(next_sb_row as isize)
                    < (*cm).tiles.cols)
        {
            pthread_cond_wait(
                (*enc_row_mt).cond_ as *mut pthread_cond_t,
                enc_row_mt_mutex_ as *mut pthread_mutex_t,
            );
        }
        row_mt_exit = (*enc_row_mt).row_mt_exit;
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        if row_mt_exit {
            return;
        }
        av1_thread_loop_filter_rows(
            (*lf_data).frame_buffer,
            (*lf_data).cm as *mut AV1_COMMON,
            ((*lf_data).planes).as_mut_ptr(),
            (*lf_data).xd,
            (*cur_job_info).mi_row,
            (*cur_job_info).plane,
            (*cur_job_info).dir,
            lpf_opt_level,
            lf_sync,
            &raw mut (*thread_data).error_info,
            ((*lf_data).params_buf).as_mut_ptr(),
            ((*lf_data).tx_buf).as_mut_ptr(),
            mib_size_log2,
        );
    };
}
unsafe extern "C-unwind" fn set_encoding_done(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let mib_size: core::ffi::c_int = (*(*cm).seq_params).mib_size;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset((tile_row * tile_cols + tile_col) as isize) as *mut TileDataEnc;
            let tile_info: *const TileInfo = &raw mut (*this_tile).tile_info;
            let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile)
                .row_mt_sync;
            let sb_cols_in_tile: core::ffi::c_int = av1_get_sb_cols_in_tile(
                cm,
                tile_info,
            ) as core::ffi::c_int;
            let mut mi_row: core::ffi::c_int = (*tile_info).mi_row_start;
            let mut sb_row_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
            while mi_row < (*tile_info).mi_row_end {
                ((*enc_row_mt).sync_write_ptr)
                    .expect(
                        "non-null function pointer",
                    )(
                    row_mt_sync,
                    sb_row_in_tile,
                    sb_cols_in_tile - 1 as core::ffi::c_int,
                    sb_cols_in_tile,
                );
                mi_row += mib_size;
                sb_row_in_tile += 1;
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
}
unsafe extern "C-unwind" fn lpf_mt_with_enc_enabled(
    mut pipeline_lpf_mt_with_enc: core::ffi::c_int,
    mut filter_level: *const core::ffi::c_int,
) -> bool {
    return pipeline_lpf_mt_with_enc != 0
        && (*filter_level.offset(0 as core::ffi::c_int as isize) != 0
            || *filter_level.offset(1 as core::ffi::c_int as isize) != 0);
}
unsafe extern "C-unwind" fn enc_row_mt_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut thread_id: core::ffi::c_int = (*thread_data).thread_id;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let mut enc_row_mt_mutex_: *mut pthread_mutex_t = (*enc_row_mt).mutex_;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    let lf_sync: *mut AV1LfSync = (*thread_data).lf_sync;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    (*xd).error_info = error_info;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let do_pipelined_lpf_mt_with_enc: bool = lpf_mt_with_enc_enabled(
        (*cpi).mt_info.pipeline_lpf_mt_with_enc,
        ((*cm).lf.filter_level).as_mut_ptr() as *const core::ffi::c_int,
    ) as bool;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mib_size_log2: core::ffi::c_int = (*(*cm).seq_params).mib_size_log2;
    let mut cur_tile_id: core::ffi::c_int = (*enc_row_mt)
        .thread_id_to_tile_id[thread_id as usize];
    if (*cpi).sf.rt_sf.use_nonrd_pick_mode != 0 {
        (*(*thread_data).td).pc_root = av1_alloc_pc_tree_node(
            (*(*cm).seq_params).sb_size,
        );
        if ((*(*thread_data).td).pc_root).is_null() {
            aom_internal_error(
                (*xd).error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    } else {
        (*(*thread_data).td).pc_root = 0 as *mut PC_TREE;
    }
    if !(cur_tile_id != -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"enc_row_mt_worker_hook\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            653 as core::ffi::c_int,
            b"cur_tile_id != -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let fp_block_size: BLOCK_SIZE = (*cpi).fp_block_size;
    let mut end_of_frame: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row_mt_exit: bool = false_0 != 0;
    (*(*thread_data).td).mb.e_mbd.tile_ctx = (*cm).fc;
    loop {
        let mut current_mi_row: core::ffi::c_int = -(1 as core::ffi::c_int);
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        row_mt_exit = (*enc_row_mt).row_mt_exit;
        if !row_mt_exit
            && get_next_job(
                &raw mut *((*cpi).tile_data).offset(cur_tile_id as isize),
                &raw mut current_mi_row,
                (*(*cm).seq_params).mib_size,
            ) == 0
        {
            switch_tile_and_get_next_job(
                cm,
                (*cpi).tile_data,
                &raw mut cur_tile_id,
                &raw mut current_mi_row,
                &raw mut end_of_frame,
                0 as core::ffi::c_int,
                fp_block_size,
            );
        }
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        if row_mt_exit {
            (*error_info).setjmp = 0 as core::ffi::c_int;
            return 1 as core::ffi::c_int;
        }
        if end_of_frame != 0 {
            break;
        }
        let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
            .offset(cur_tile_id as isize) as *mut TileDataEnc;
        let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile).row_mt_sync;
        let tile_info: *const TileInfo = &raw mut (*this_tile).tile_info;
        let tile_row: core::ffi::c_int = (*tile_info).tile_row;
        let tile_col: core::ffi::c_int = (*tile_info).tile_col;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        let sb_row: core::ffi::c_int = current_mi_row >> mib_size_log2;
        if !(current_mi_row != -(1 as core::ffi::c_int)
            && current_mi_row <= (*tile_info).mi_row_end) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"enc_row_mt_worker_hook\0"))
                    .as_ptr(),
                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                703 as core::ffi::c_int,
                b"current_mi_row != -1 && current_mi_row <= tile_info->mi_row_end\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*td).mb.e_mbd.tile_ctx = (*td).tctx;
        (*td).mb.tile_pb_ctx = &raw mut (*this_tile).tctx;
        (*td).abs_sum_level = 0 as uint64_t;
        if (*this_tile).allow_update_cdf != 0 {
            (*td).mb.row_ctx = (*this_tile).row_ctx;
            if current_mi_row == (*tile_info).mi_row_start {
                *(*td).mb.e_mbd.tile_ctx = (*this_tile).tctx;
            }
        } else {
            *(*td).mb.e_mbd.tile_ctx = (*this_tile).tctx;
        }
        av1_init_above_context(
            &raw mut (*cm).above_contexts,
            av1_num_planes(cm),
            tile_row,
            &raw mut (*td).mb.e_mbd,
        );
        cfl_init(&raw mut (*td).mb.e_mbd.cfl, (*cm).seq_params);
        if !((*td).mb.txfm_search_info.mb_rd_record).is_null() {
            av1_crc32c_calculator_init(
                &raw mut (*(*td).mb.txfm_search_info.mb_rd_record).crc_calculator,
            );
        }
        av1_encode_sb_row(
            cpi as *mut AV1_COMP,
            td as *mut ThreadData,
            tile_row,
            tile_col,
            current_mi_row,
        );
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        (*this_tile).abs_sum_level = ((*this_tile).abs_sum_level)
            .wrapping_add((*td).abs_sum_level);
        (*row_mt_sync).num_threads_working -= 1;
        let fresh2 = &raw mut *((*enc_row_mt).num_tile_cols_done).offset(sb_row as isize);
        *fresh2 += 1;
        pthread_cond_broadcast((*enc_row_mt).cond_ as *mut pthread_cond_t);
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
    }
    if do_pipelined_lpf_mt_with_enc {
        launch_loop_filter_rows(cm, thread_data, enc_row_mt, mib_size_log2);
    }
    av1_free_pc_tree_recursive(
        (*(*thread_data).td).pc_root,
        av1_num_planes(cm),
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        (*cpi).sf.part_sf.partition_search_type,
    );
    (*(*thread_data).td).pc_root = 0 as *mut PC_TREE;
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        (*enc_row_mt).row_mt_exit = true_0 != 0;
        pthread_cond_broadcast((*enc_row_mt).cond_ as *mut pthread_cond_t);
        pthread_mutex_unlock(enc_row_mt_mutex_ as *mut pthread_mutex_t);
        set_encoding_done(cpi);
        if do_pipelined_lpf_mt_with_enc {
            pthread_mutex_lock((*lf_sync).job_mutex as *mut pthread_mutex_t);
            (*lf_sync).lf_mt_exit = true_0 != 0;
            pthread_mutex_unlock((*lf_sync).job_mutex as *mut pthread_mutex_t);
            av1_set_vert_loop_filter_done(
                &raw mut (*cpi).common,
                lf_sync,
                (*(*cpi).common.seq_params).mib_size_log2,
            );
        }
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn enc_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut t: core::ffi::c_int = 0;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    if (*cpi).sf.rt_sf.use_nonrd_pick_mode != 0 {
        (*(*thread_data).td).pc_root = av1_alloc_pc_tree_node(
            (*(*cm).seq_params).sb_size,
        );
        if ((*(*thread_data).td).pc_root).is_null() {
            aom_internal_error(
                (*xd).error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    } else {
        (*(*thread_data).td).pc_root = 0 as *mut PC_TREE;
    }
    t = (*thread_data).start;
    while t < tile_rows * tile_cols {
        let mut tile_row: core::ffi::c_int = t / tile_cols;
        let mut tile_col: core::ffi::c_int = t % tile_cols;
        let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
            .offset((tile_row * (*cm).tiles.cols + tile_col) as isize)
            as *mut TileDataEnc;
        (*(*thread_data).td).mb.e_mbd.tile_ctx = &raw mut (*this_tile).tctx;
        (*(*thread_data).td).mb.tile_pb_ctx = &raw mut (*this_tile).tctx;
        av1_encode_tile(cpi as *mut AV1_COMP, (*thread_data).td, tile_row, tile_col);
        t += (*cpi).mt_info.num_workers;
    }
    av1_free_pc_tree_recursive(
        (*(*thread_data).td).pc_root,
        av1_num_planes(cm),
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        (*cpi).sf.part_sf.partition_search_type,
    );
    (*(*thread_data).td).pc_root = 0 as *mut PC_TREE;
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        return 0 as core::ffi::c_int;}
}
}}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_frame_mt(
    mut ppi: *mut AV1_PRIMARY,
    mut cpi: *mut AV1_COMP,
) {
    (*cpi).mt_info.workers = (*ppi).p_mt_info.workers;
    (*cpi).mt_info.num_workers = (*ppi).p_mt_info.num_workers;
    (*cpi).mt_info.tile_thr_data = (*ppi).p_mt_info.tile_thr_data;
    let mut i: core::ffi::c_int = 0;
    i = MOD_FP as core::ffi::c_int;
    while i < NUM_MT_MODULES as core::ffi::c_int {
        (*cpi).mt_info.num_mod_workers[i as usize] = if (*cpi).mt_info.num_workers
            < (*ppi).p_mt_info.num_mod_workers[i as usize]
        {
            (*cpi).mt_info.num_workers
        } else {
            (*ppi).p_mt_info.num_mod_workers[i as usize]
        };
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_cdef_worker(mut cpi: *mut AV1_COMP) {
    if (*(*cpi).ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
        > 0 as core::ffi::c_int
    {
        return;
    }
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*(*cpi).ppi).p_mt_info;
    let mut num_cdef_workers: core::ffi::c_int = av1_get_num_mod_workers_for_alloc(
        p_mt_info,
        MOD_CDEF,
    );
    av1_alloc_cdef_buffers(
        &raw mut (*cpi).common,
        &raw mut (*p_mt_info).cdef_worker,
        &raw mut (*cpi).mt_info.cdef_sync,
        num_cdef_workers,
        1 as core::ffi::c_int,
    );
    (*cpi).mt_info.cdef_worker = (*p_mt_info).cdef_worker;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_lr_mt_buffers(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut lr_sync: *mut AV1LrSync = &raw mut (*cpi).mt_info.lr_row_sync;
    if (*lr_sync).sync_range != 0 {
        if (*(*cpi).ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
            > 0 as core::ffi::c_int
        {
            return;
        }
        let mut num_lr_workers: core::ffi::c_int = (*lr_sync).num_workers;
        let fresh5 = &raw mut (*((*lr_sync).lrworkerdata)
            .offset((num_lr_workers - 1 as core::ffi::c_int) as isize))
            .rst_tmpbuf;
        *fresh5 = (*cm).rst_tmpbuf;
        let fresh6 = &raw mut (*((*lr_sync).lrworkerdata)
            .offset((num_lr_workers - 1 as core::ffi::c_int) as isize))
            .rlbs;
        *fresh6 = (*cm).rlbs as *mut core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_mt_sync(
    mut cpi: *mut AV1_COMP,
    mut is_first_pass: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    loop {
match crate::jump::value_scope((((*(*cm).error).jmp).as_mut_ptr()).cast(), || {
    (*(*cm).error).setjmp = 1 as core::ffi::c_int;
    if is_first_pass != 0
        || (*cpi).oxcf.row_mt as core::ffi::c_int == 1 as core::ffi::c_int
    {
        let mut enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*mt_info).enc_row_mt;
        if ((*enc_row_mt).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    859 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*enc_row_mt).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*enc_row_mt).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate enc_row_mt->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    859 as core::ffi::c_int,
                );
            }
            if !((*enc_row_mt).mutex_).is_null() {
                pthread_mutex_init(
                    (*enc_row_mt).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
        if ((*enc_row_mt).cond_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    864 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*enc_row_mt).cond_ = aom_malloc(
                ::core::mem::size_of::<pthread_cond_t>() as size_t,
            ) as *mut pthread_cond_t;
            if ((*enc_row_mt).cond_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate enc_row_mt->cond_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    864 as core::ffi::c_int,
                );
            }
            if !((*enc_row_mt).cond_).is_null() {
                pthread_cond_init(
                    (*enc_row_mt).cond_ as *mut pthread_cond_t,
                    0 as *const pthread_condattr_t,
                );
            }
        }
    }
    if is_first_pass == 0 {
        let mut gm_sync: *mut AV1GlobalMotionSync = &raw mut (*mt_info).gm_sync;
        if ((*gm_sync).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    874 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*gm_sync).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*gm_sync).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate gm_sync->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    874 as core::ffi::c_int,
                );
            }
            if !((*gm_sync).mutex_).is_null() {
                pthread_mutex_init(
                    (*gm_sync).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
        let mut tf_sync: *mut AV1TemporalFilterSync = &raw mut (*mt_info).tf_sync;
        if ((*tf_sync).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    882 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*tf_sync).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*tf_sync).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate tf_sync->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    882 as core::ffi::c_int,
                );
            }
            if !((*tf_sync).mutex_).is_null() {
                pthread_mutex_init(
                    (*tf_sync).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
        let mut cdef_sync: *mut AV1CdefSync = &raw mut (*mt_info).cdef_sync;
        if ((*cdef_sync).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    890 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*cdef_sync).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*cdef_sync).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate cdef_sync->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    890 as core::ffi::c_int,
                );
            }
            if !((*cdef_sync).mutex_).is_null() {
                pthread_mutex_init(
                    (*cdef_sync).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
        let mut lf_sync: *mut AV1LfSync = &raw mut (*mt_info).lf_row_sync;
        let sb_rows: core::ffi::c_int = ((*cm).height >> 2 as core::ffi::c_int)
            + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int >> 7 as core::ffi::c_int - 2 as core::ffi::c_int;
        let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*(*cpi).ppi).p_mt_info;
        let mut num_lf_workers: core::ffi::c_int = av1_get_num_mod_workers_for_alloc(
            p_mt_info,
            MOD_LPF,
        );
        if (*lf_sync).sync_range == 0 || sb_rows != (*lf_sync).rows
            || num_lf_workers > (*lf_sync).num_workers
        {
            av1_loop_filter_dealloc(lf_sync);
            av1_loop_filter_alloc(lf_sync, cm, sb_rows, (*cm).width, num_lf_workers);
        }
        let mut tpl_row_mt: *mut AV1TplRowMultiThreadInfo = &raw mut (*mt_info).tpl_row_mt;
        if ((*tpl_row_mt).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    912 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*tpl_row_mt).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*tpl_row_mt).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate tpl_row_mt->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    912 as core::ffi::c_int,
                );
            }
            if !((*tpl_row_mt).mutex_).is_null() {
                pthread_mutex_init(
                    (*tpl_row_mt).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
        if is_restoration_used(cm) != 0 {
            let mut lr_sync: *mut AV1LrSync = &raw mut (*mt_info).lr_row_sync;
            let mut rst_unit_size: core::ffi::c_int = (*cpi).sf.lpf_sf.min_lr_unit_size;
            let mut num_rows_lr: core::ffi::c_int = av1_lr_count_units(
                rst_unit_size,
                (*cm).height,
            );
            let mut num_lr_workers: core::ffi::c_int = av1_get_num_mod_workers_for_alloc(
                p_mt_info,
                MOD_LR,
            );
            let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
            if (*lr_sync).sync_range == 0 || num_rows_lr > (*lr_sync).rows
                || num_lr_workers > (*lr_sync).num_workers
                || num_planes > (*lr_sync).num_planes
            {
                av1_loop_restoration_dealloc(lr_sync);
                av1_loop_restoration_alloc(
                    lr_sync,
                    cm,
                    num_lr_workers,
                    num_rows_lr,
                    num_planes,
                    (*cm).width,
                );
            }
        }
        let mut pack_bs_sync: *mut AV1EncPackBSSync = &raw mut (*mt_info).pack_bs_sync;
        if ((*pack_bs_sync).mutex_).is_null() {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"av1_init_mt_sync\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    938 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*pack_bs_sync).mutex_ = aom_malloc(
                ::core::mem::size_of::<pthread_mutex_t>() as size_t,
            ) as *mut pthread_mutex_t;
            if ((*pack_bs_sync).mutex_).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate pack_bs_sync->mutex_ at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    938 as core::ffi::c_int,
                );
            }
            if !((*pack_bs_sync).mutex_).is_null() {
                pthread_mutex_init(
                    (*pack_bs_sync).mutex_ as *mut pthread_mutex_t,
                    0 as *const pthread_mutexattr_t,
                );
            }
        }
    }
    (*(*cm).error).setjmp = 0 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*(*cm).error).setjmp = 0 as core::ffi::c_int;
        aom_internal_error_copy(&raw mut (*(*cpi).ppi).error, (*cm).error);}
}
}}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_num_mod_workers_for_alloc(
    mut p_mt_info: *const PrimaryMultiThreadInfo,
    mut mod_name: MULTI_THREADED_MODULES,
) -> core::ffi::c_int {
    let mut num_mod_workers: core::ffi::c_int = (*p_mt_info)
        .num_mod_workers[mod_name as usize];
    if (*p_mt_info).num_mod_workers[MOD_FRAME_ENC as core::ffi::c_int as usize]
        > 1 as core::ffi::c_int
    {
        num_mod_workers = (*p_mt_info).num_workers;
    }
    return num_mod_workers;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_tile_thread_data(
    mut ppi: *mut AV1_PRIMARY,
    mut is_first_pass: core::ffi::c_int,
) {
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    if ((*p_mt_info).workers).is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_init_tile_thread_data\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            964 as core::ffi::c_int,
            b"p_mt_info->workers != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if ((*p_mt_info).tile_thr_data).is_null() as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_init_tile_thread_data\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            965 as core::ffi::c_int,
            b"p_mt_info->tile_thr_data != NULL\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let is_highbitdepth: core::ffi::c_int = (*ppi).seq_params.use_highbitdepth
        as core::ffi::c_int;
    let mut num_workers: core::ffi::c_int = (*p_mt_info).num_workers;
    let mut num_enc_workers: core::ffi::c_int = av1_get_num_mod_workers_for_alloc(
        p_mt_info,
        MOD_ENC,
    );
    if !(num_enc_workers <= num_workers) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_init_tile_thread_data\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            970 as core::ffi::c_int,
            b"num_enc_workers <= num_workers\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let thread_data: *mut EncWorkerData = &raw mut *((*p_mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        if i > 0 as core::ffi::c_int {
            let mut td: *mut ThreadData = 0 as *mut ThreadData;
            if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 26],
                        [core::ffi::c_char; 26],
                    >(*b"av1_init_tile_thread_data\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    977 as core::ffi::c_int,
                    b"(&ppi->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            td = aom_memalign(
                32 as size_t,
                ::core::mem::size_of::<ThreadData>() as size_t,
            ) as *mut ThreadData;
            if td.is_null() {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate td at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    977 as core::ffi::c_int,
                );
            }
            memset(
                td as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ThreadData>() as size_t,
            );
            (*thread_data).td = td as *mut ThreadData;
            (*thread_data).original_td = (*thread_data).td;
            av1_setup_shared_coeff_buffer(
                &raw mut (*ppi).seq_params,
                &raw mut (*td).shared_coeff_buf,
                &raw mut (*ppi).error,
            );
            if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 26],
                        [core::ffi::c_char; 26],
                    >(*b"av1_init_tile_thread_data\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    986 as core::ffi::c_int,
                    b"(&ppi->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*td).tmp_conv_dst = aom_memalign(
                32 as size_t,
                ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
                    .wrapping_mul(::core::mem::size_of::<CONV_BUF_TYPE>() as size_t),
            ) as *mut CONV_BUF_TYPE;
            if ((*td).tmp_conv_dst).is_null() {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate td->tmp_conv_dst at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    986 as core::ffi::c_int,
                );
            }
            if i < (*p_mt_info).num_mod_workers[MOD_FP as core::ffi::c_int as usize] {
                (*td).firstpass_ctx = av1_alloc_pmc(
                    (*ppi).cpi,
                    BLOCK_16X16,
                    &raw mut (*td).shared_coeff_buf,
                );
                if ((*td).firstpass_ctx).is_null() {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 26],
                        [core::ffi::c_char; 26],
                    >(*b"av1_init_tile_thread_data\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    1000 as core::ffi::c_int,
                    b"(&ppi->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*td).upsample_pred = aom_memalign(
                16 as size_t,
                (((1 as core::ffi::c_int + is_highbitdepth)
                    * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                        + 16 as core::ffi::c_int + 16 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t;
            if ((*td).upsample_pred).is_null() {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate td->upsample_pred at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1000 as core::ffi::c_int,
                );
            }
            if is_first_pass == 0 && i < num_enc_workers {
                if av1_setup_sms_tree((*ppi).cpi, td as *mut ThreadData) != 0 {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate SMS tree\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < 2 as core::ffi::c_int {
                    if ((*ppi).error.setjmp == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 26],
                                [core::ffi::c_char; 26],
                            >(*b"av1_init_tile_thread_data\0"))
                                .as_ptr(),
                            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                            1013 as core::ffi::c_int,
                            b"(&ppi->error)->setjmp\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    (*td).hash_value_buffer[x as usize] = aom_malloc(
                        (4096 as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    ) as *mut uint32_t;
                    if ((*td).hash_value_buffer[x as usize]).is_null() {
                        aom_internal_error(
                            &raw mut (*ppi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate td->hash_value_buffer[x] at %s:%d\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1013 as core::ffi::c_int,
                        );
                    }
                    x += 1;
                }
                if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 26],
                            [core::ffi::c_char; 26],
                        >(*b"av1_init_tile_thread_data\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        1018 as core::ffi::c_int,
                        b"(&ppi->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*td).counts = aom_calloc(
                    1 as size_t,
                    ::core::mem::size_of::<FRAME_COUNTS>() as size_t,
                ) as *mut FRAME_COUNTS;
                if ((*td).counts).is_null() {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate td->counts at %s:%d\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1018 as core::ffi::c_int,
                    );
                }
                if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 26],
                            [core::ffi::c_char; 26],
                        >(*b"av1_init_tile_thread_data\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        1022 as core::ffi::c_int,
                        b"(&ppi->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*td).palette_buffer = aom_memalign(
                    16 as size_t,
                    ::core::mem::size_of::<PALETTE_BUFFER>() as size_t,
                ) as *mut PALETTE_BUFFER;
                if ((*td).palette_buffer).is_null() {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate td->palette_buffer at %s:%d\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1022 as core::ffi::c_int,
                    );
                }
                if (*(*ppi).cpi).oxcf.kf_cfg.key_freq_max != 0 as core::ffi::c_int {
                    alloc_obmc_buffers(&raw mut (*td).obmc_buffer, &raw mut (*ppi).error);
                    alloc_compound_type_rd_buffers(
                        &raw mut (*ppi).error,
                        &raw mut (*td).comp_rd_buffer,
                    );
                    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                    while j < 2 as core::ffi::c_int {
                        if ((*ppi).error.setjmp == 0) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 26],
                                    [core::ffi::c_char; 26],
                                >(*b"av1_init_tile_thread_data\0"))
                                    .as_ptr(),
                                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                                1038 as core::ffi::c_int,
                                b"(&ppi->error)->setjmp\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        (*td).tmp_pred_bufs[j as usize] = aom_memalign(
                            32 as size_t,
                            ((2 as core::ffi::c_int * 3 as core::ffi::c_int
                                * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)))
                                as size_t)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                        ) as *mut uint8_t;
                        if ((*td).tmp_pred_bufs[j as usize]).is_null() {
                            aom_internal_error(
                                &raw mut (*ppi).error as *mut aom_internal_error_info,
                                AOM_CODEC_MEM_ERROR,
                                b"Failed to allocate td->tmp_pred_bufs[j] at %s:%d\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1038 as core::ffi::c_int,
                            );
                        }
                        j += 1;
                    }
                }
                let plane_types: core::ffi::c_int = PLANE_TYPES as core::ffi::c_int
                    as core::ffi::c_int
                    >> (*ppi).seq_params.monochrome as core::ffi::c_int;
                if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 26],
                            [core::ffi::c_char; 26],
                        >(*b"av1_init_tile_thread_data\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        1045 as core::ffi::c_int,
                        b"(&ppi->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*td).pixel_gradient_info = aom_malloc(
                    (::core::mem::size_of::<PixelLevelGradientInfo>() as size_t)
                        .wrapping_mul(plane_types as size_t)
                        .wrapping_mul(
                            (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))
                                as size_t,
                        ),
                ) as *mut PixelLevelGradientInfo;
                if ((*td).pixel_gradient_info).is_null() {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate td->pixel_gradient_info at %s:%d\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1045 as core::ffi::c_int,
                    );
                }
                if is_src_var_for_4x4_sub_blocks_caching_enabled((*ppi).cpi) {
                    let sb_size: BLOCK_SIZE = (*(*(*ppi).cpi).common.seq_params).sb_size;
                    let mi_count_in_sb: core::ffi::c_int = mi_size_wide[sb_size as usize]
                        as core::ffi::c_int
                        * mi_size_high[sb_size as usize] as core::ffi::c_int;
                    if ((*ppi).error.setjmp == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 26],
                                [core::ffi::c_char; 26],
                            >(*b"av1_init_tile_thread_data\0"))
                                .as_ptr(),
                            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                            1055 as core::ffi::c_int,
                            b"(&ppi->error)->setjmp\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    (*td).src_var_info_of_4x4_sub_blocks = aom_malloc(
                        (::core::mem::size_of::<Block4x4VarInfo>() as size_t)
                            .wrapping_mul(mi_count_in_sb as size_t),
                    ) as *mut Block4x4VarInfo;
                    if ((*td).src_var_info_of_4x4_sub_blocks).is_null() {
                        aom_internal_error(
                            &raw mut (*ppi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate td->src_var_info_of_4x4_sub_blocks at %s:%d\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1055 as core::ffi::c_int,
                        );
                    }
                }
                if (*(*ppi).cpi).sf.part_sf.partition_search_type as core::ffi::c_int
                    == VAR_BASED_PARTITION as core::ffi::c_int
                {
                    let num_64x64_blocks: core::ffi::c_int = if (*ppi).seq_params.sb_size
                        as core::ffi::c_int == BLOCK_64X64 as core::ffi::c_int
                    {
                        1 as core::ffi::c_int
                    } else {
                        4 as core::ffi::c_int
                    };
                    if ((*ppi).error.setjmp == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 26],
                                [core::ffi::c_char; 26],
                            >(*b"av1_init_tile_thread_data\0"))
                                .as_ptr(),
                            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                            1063 as core::ffi::c_int,
                            b"(&ppi->error)->setjmp\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    (*td).vt64x64 = aom_malloc(
                        (::core::mem::size_of::<VP64x64>() as size_t)
                            .wrapping_mul(num_64x64_blocks as size_t),
                    ) as *mut VP64x64;
                    if ((*td).vt64x64).is_null() {
                        aom_internal_error(
                            &raw mut (*ppi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate td->vt64x64 at %s:%d\0" as *const u8
                                as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1063 as core::ffi::c_int,
                        );
                    }
                }
            }
        }
        if is_first_pass == 0
            && (*(*ppi).cpi).oxcf.row_mt as core::ffi::c_int == 1 as core::ffi::c_int
            && i < num_enc_workers
        {
            if i == 0 as core::ffi::c_int {
                let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while j_0 < (*ppi).num_fp_contexts {
                    if ((*ppi).error.setjmp == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 26],
                                [core::ffi::c_char; 26],
                            >(*b"av1_init_tile_thread_data\0"))
                                .as_ptr(),
                            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                            1073 as core::ffi::c_int,
                            b"(&ppi->error)->setjmp\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    (*(*ppi).parallel_cpi[j_0 as usize]).td.tctx = aom_memalign(
                        16 as size_t,
                        ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
                    ) as *mut FRAME_CONTEXT;
                    if ((*(*ppi).parallel_cpi[j_0 as usize]).td.tctx).is_null() {
                        aom_internal_error(
                            &raw mut (*ppi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate ppi->parallel_cpi[j]->td.tctx at %s:%d\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1073 as core::ffi::c_int,
                        );
                    }
                    j_0 += 1;
                }
            } else {
                if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 26],
                            [core::ffi::c_char; 26],
                        >(*b"av1_init_tile_thread_data\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        1078 as core::ffi::c_int,
                        b"(&ppi->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*(*thread_data).td).tctx = aom_memalign(
                    16 as size_t,
                    ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
                ) as *mut FRAME_CONTEXT;
                if ((*(*thread_data).td).tctx).is_null() {
                    aom_internal_error(
                        &raw mut (*ppi).error as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate thread_data->td->tctx at %s:%d\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1078 as core::ffi::c_int,
                    );
                }
            }
        }
        i -= 1;
    }
    (*p_mt_info).prev_num_enc_workers = num_enc_workers;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_create_workers(
    mut ppi: *mut AV1_PRIMARY,
    mut num_workers: core::ffi::c_int,
) {
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    if !((*p_mt_info).num_workers == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_create_workers\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1091 as core::ffi::c_int,
            b"p_mt_info->num_workers == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_create_workers\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1094 as core::ffi::c_int,
            b"(&ppi->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*p_mt_info).workers = aom_malloc(
        (num_workers as size_t)
            .wrapping_mul(::core::mem::size_of::<AVxWorker>() as size_t),
    ) as *mut AVxWorker;
    if ((*p_mt_info).workers).is_null() {
        aom_internal_error(
            &raw mut (*ppi).error as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate p_mt_info->workers at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            1094 as core::ffi::c_int,
        );
    }
    if ((*ppi).error.setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_create_workers\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1098 as core::ffi::c_int,
            b"(&ppi->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*p_mt_info).tile_thr_data = aom_calloc(
        num_workers as size_t,
        ::core::mem::size_of::<EncWorkerData>() as size_t,
    ) as *mut EncWorkerData;
    if ((*p_mt_info).tile_thr_data).is_null() {
        aom_internal_error(
            &raw mut (*ppi).error as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate p_mt_info->tile_thr_data at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            1098 as core::ffi::c_int,
        );
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_workers {
        let worker: *mut AVxWorker = &raw mut *((*p_mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = &raw mut *((*p_mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        ((*winterface).init).expect("non-null function pointer")(worker);
        (*worker).thread_name = b"aom enc worker\0" as *const u8
            as *const core::ffi::c_char;
        (*thread_data).thread_id = i;
        (*thread_data).start = i;
        if i > 0 as core::ffi::c_int {
            if ((*winterface).reset).expect("non-null function pointer")(worker) == 0 {
                aom_internal_error(
                    &raw mut (*ppi).error as *mut aom_internal_error_info,
                    AOM_CODEC_ERROR,
                    b"Tile encoder thread creation failed\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        ((*winterface).sync).expect("non-null function pointer")(worker);
        (*p_mt_info).num_workers += 1;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_terminate_workers(mut ppi: *mut AV1_PRIMARY) {
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    let mut t: core::ffi::c_int = 0 as core::ffi::c_int;
    while t < (*p_mt_info).num_workers {
        let worker: *mut AVxWorker = &raw mut *((*p_mt_info).workers).offset(t as isize)
            as *mut AVxWorker;
        ((*aom_get_worker_interface()).end).expect("non-null function pointer")(worker);
        t += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn is_fpmt_config(
    mut ppi: *const AV1_PRIMARY,
    mut oxcf: *const AV1EncoderConfig,
) -> core::ffi::c_int {
    if (*oxcf).rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        || (*oxcf).rc_cfg.mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
    {
        return 0 as core::ffi::c_int;
    }
    if (*ppi).use_svc != 0 {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).tile_cfg.enable_large_scale_tile {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).dec_model_cfg.timing_info_present {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).mode as core::ffi::c_int != GOOD as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).tool_cfg.error_resilient_mode {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).resize_cfg.resize_mode != 0 {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).pass as core::ffi::c_uint
        != AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        return 0 as core::ffi::c_int;
    }
    if (*oxcf).max_threads < 2 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if !(*oxcf).fp_mt {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_check_fpmt_config(
    ppi: *mut AV1_PRIMARY,
    oxcf: *const AV1EncoderConfig,
) -> core::ffi::c_int {
    if is_fpmt_config(ppi, oxcf) != 0 {
        return 1 as core::ffi::c_int;
    }
    if (*ppi).num_fp_contexts > 1 as core::ffi::c_int {
        let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
        while i < (*ppi).num_fp_contexts {
            if !((*(*ppi).parallel_cpi[i as usize]).common.cur_frame).is_null() {
                (*(*(*ppi).parallel_cpi[i as usize]).common.cur_frame).ref_count -= 1;
                (*(*ppi).parallel_cpi[i as usize]).common.cur_frame = 0
                    as *mut RefCntBuffer;
            }
            i += 1;
        }
        let mut cur_gf_index: core::ffi::c_int = (*(*ppi).cpi).gf_frame_index
            as core::ffi::c_int;
        let mut reset_size: core::ffi::c_int = if 0 as core::ffi::c_int
            > (*ppi).gf_group.size - cur_gf_index
        {
            0 as core::ffi::c_int
        } else {
            (*ppi).gf_group.size - cur_gf_index
        };
        memset(
            &raw mut *((*ppi).gf_group.frame_parallel_level)
                .as_mut_ptr()
                .offset(cur_gf_index as isize) as *mut core::ffi::c_int
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (reset_size as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        memset(
            &raw mut *((*ppi).gf_group.is_frame_non_ref)
                .as_mut_ptr()
                .offset(cur_gf_index as isize) as *mut bool as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (reset_size as size_t).wrapping_mul(::core::mem::size_of::<bool>() as size_t),
        );
        memset(
            &raw mut *((*ppi).gf_group.src_offset).as_mut_ptr().offset(cur_gf_index as isize)
                as *mut core::ffi::c_int as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (reset_size as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        memset(
            &raw mut *(*((*ppi).gf_group.skip_frame_refresh)
                .as_mut_ptr()
                .offset(cur_gf_index as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_int
                as *mut core::ffi::c_void,
            INVALID_IDX,
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(reset_size as size_t)
                .wrapping_mul(REF_FRAMES as core::ffi::c_int as size_t),
        );
        memset(
            &raw mut *((*ppi).gf_group.skip_frame_as_ref)
                .as_mut_ptr()
                .offset(cur_gf_index as isize) as *mut core::ffi::c_int
                as *mut core::ffi::c_void,
            INVALID_IDX,
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(reset_size as size_t),
        );
        (*ppi).num_fp_contexts = 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn compute_max_num_enc_workers(
    mi_params: *mut CommonModeInfoParams,
    mut mib_size_log2: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut num_sb_rows: core::ffi::c_int = (*mi_params).mi_rows
        + ((1 as core::ffi::c_int) << mib_size_log2) - 1 as core::ffi::c_int
        >> mib_size_log2;
    let mut num_sb_cols: core::ffi::c_int = (*mi_params).mi_cols
        + ((1 as core::ffi::c_int) << mib_size_log2) - 1 as core::ffi::c_int
        >> mib_size_log2;
    return if (num_sb_cols + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        < num_sb_rows
    {
        num_sb_cols + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
    } else {
        num_sb_rows
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_num_fp_contexts(
    mut ppi: *mut AV1_PRIMARY,
    mut oxcf: *mut AV1EncoderConfig,
) -> core::ffi::c_int {
    (*ppi).p_mt_info.num_mod_workers[MOD_FRAME_ENC as core::ffi::c_int as usize] = 0
        as core::ffi::c_int;
    if av1_check_fpmt_config(ppi, oxcf) == 0 {
        return 1 as core::ffi::c_int;
    }
    let mut max_num_enc_workers: core::ffi::c_int = compute_max_num_enc_workers(
        &raw mut (*(*ppi).cpi).common.mi_params,
        (*(*(*ppi).cpi).common.seq_params).mib_size_log2,
    );
    let mut rounding_factor: [core::ffi::c_int; 2] = [
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
    ];
    let mut scaling_factor: [core::ffi::c_int; 2] = [
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    ];
    let mut is_480p_or_lesser: core::ffi::c_int = ((if (*oxcf).frm_dim_cfg.width
        < (*oxcf).frm_dim_cfg.height
    {
        (*oxcf).frm_dim_cfg.width
    } else {
        (*oxcf).frm_dim_cfg.height
    }) <= 480 as core::ffi::c_int) as core::ffi::c_int;
    let mut is_sb_64: core::ffi::c_int = 0 as core::ffi::c_int;
    if !((*ppi).cpi).is_null() {
        is_sb_64 = ((*(*(*ppi).cpi).common.seq_params).sb_size as core::ffi::c_int
            == BLOCK_64X64 as core::ffi::c_int) as core::ffi::c_int;
    }
    let mut index: core::ffi::c_int = if is_480p_or_lesser == 0 && is_sb_64 != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut workers_per_frame: core::ffi::c_int = if 1 as core::ffi::c_int
        > (max_num_enc_workers + rounding_factor[index as usize])
            / scaling_factor[index as usize]
    {
        1 as core::ffi::c_int
    } else {
        (max_num_enc_workers + rounding_factor[index as usize])
            / scaling_factor[index as usize]
    };
    let mut max_threads: core::ffi::c_int = (*oxcf).max_threads;
    let mut num_fp_contexts: core::ffi::c_int = max_threads / workers_per_frame;
    if (*oxcf).tile_cfg.tile_columns > 0 as core::ffi::c_int
        || (*oxcf).tile_cfg.tile_rows > 0 as core::ffi::c_int
    {
        if num_fp_contexts < MAX_PARALLEL_FRAMES {
            num_fp_contexts = 1 as core::ffi::c_int;
        }
    }
    num_fp_contexts = clamp(num_fp_contexts, 1 as core::ffi::c_int, MAX_PARALLEL_FRAMES);
    num_fp_contexts = if (*ppi).num_fp_contexts == 1 as core::ffi::c_int {
        num_fp_contexts
    } else if num_fp_contexts < (*ppi).num_fp_contexts {
        num_fp_contexts
    } else {
        (*ppi).num_fp_contexts
    };
    if num_fp_contexts > 1 as core::ffi::c_int {
        (*ppi).p_mt_info.num_mod_workers[MOD_FRAME_ENC as core::ffi::c_int as usize] = if max_num_enc_workers
            * num_fp_contexts < (*oxcf).max_threads
        {
            max_num_enc_workers * num_fp_contexts
        } else {
            (*oxcf).max_threads
        };
    }
    return num_fp_contexts;
}
#[inline]
unsafe extern "C-unwind" fn compute_num_workers_per_frame(
    num_workers: core::ffi::c_int,
    parallel_frame_count: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut workers_per_frame: core::ffi::c_int = num_workers / parallel_frame_count;
    return workers_per_frame;
}
#[inline]
unsafe extern "C-unwind" fn prepare_fpmt_workers(
    mut ppi: *mut AV1_PRIMARY,
    mut first_cpi_data: *mut AV1_COMP_DATA,
    mut hook: AVxWorkerHook,
    mut parallel_frame_count: core::ffi::c_int,
) {
    if !(parallel_frame_count <= (*ppi).num_fp_contexts
        && parallel_frame_count > 1 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"prepare_fpmt_workers\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1288 as core::ffi::c_int,
            b"parallel_frame_count <= ppi->num_fp_contexts && parallel_frame_count > 1\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    let mut num_workers: core::ffi::c_int = (*p_mt_info).num_workers;
    let mut frame_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_workers {
        (*p_mt_info).p_workers[frame_idx as usize] = &raw mut *((*p_mt_info).workers)
            .offset(i as isize) as *mut AVxWorker;
        let mut frame_worker: *mut AVxWorker = (*p_mt_info)
            .p_workers[frame_idx as usize];
        let mut cur_cpi: *mut AV1_COMP = (*ppi).parallel_cpi[frame_idx as usize];
        let mut mt_info: *mut MultiThreadInfo = &raw mut (*cur_cpi).mt_info;
        let error: *mut aom_internal_error_info = (*cur_cpi).common.error;
        loop {
match crate::jump::scope((((*error).jmp).as_mut_ptr()).cast(), || {

        (*error).setjmp = 1 as core::ffi::c_int;
        let cm: *mut AV1_COMMON = &raw mut (*cur_cpi).common;
        (*mt_info).workers = &raw mut *((*p_mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        (*mt_info).tile_thr_data = &raw mut *((*p_mt_info).tile_thr_data).offset(i as isize)
            as *mut EncWorkerData;
        (*mt_info).num_workers = compute_num_workers_per_frame(
            num_workers - i,
            parallel_frame_count - frame_idx,
        );
        let mut j: core::ffi::c_int = MOD_FP as core::ffi::c_int;
        while j < NUM_MT_MODULES as core::ffi::c_int {
            (*mt_info).num_mod_workers[j as usize] = if (*mt_info).num_workers
                < (*p_mt_info).num_mod_workers[j as usize]
            {
                (*mt_info).num_workers
            } else {
                (*p_mt_info).num_mod_workers[j as usize]
            };
            j += 1;
        }
        if !((*p_mt_info).cdef_worker).is_null() {
            (*mt_info).cdef_worker = &raw mut *((*p_mt_info).cdef_worker).offset(i as isize)
                as *mut AV1CdefWorkerData;
            (*mt_info).restore_state_buf.cdef_srcbuf = (*(*mt_info).cdef_worker).srcbuf;
            let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
            let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane < num_planes {
                (*mt_info).restore_state_buf.cdef_colbuf[plane as usize] = (*(*mt_info)
                    .cdef_worker)
                    .colbuf[plane as usize];
                plane += 1;
            }
        }
        if is_restoration_used(cm) != 0 {
            let mut idx: core::ffi::c_int = i + (*mt_info).num_workers
                - 1 as core::ffi::c_int;
            if !(idx < (*mt_info).lr_row_sync.num_workers) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"prepare_fpmt_workers\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    1341 as core::ffi::c_int,
                    b"idx < mt_info->lr_row_sync.num_workers\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            (*mt_info).restore_state_buf.rst_tmpbuf = (*((*mt_info)
                .lr_row_sync
                .lrworkerdata)
                .offset(idx as isize))
                .rst_tmpbuf;
            (*mt_info).restore_state_buf.rlbs = (*((*mt_info).lr_row_sync.lrworkerdata)
                .offset(idx as isize))
                .rlbs as *mut RestorationLineBuffers;
            let fresh10 = &raw mut (*((*mt_info).lr_row_sync.lrworkerdata)
                .offset(idx as isize))
                .rst_tmpbuf;
            *fresh10 = (*cm).rst_tmpbuf;
            let fresh11 = &raw mut (*((*mt_info).lr_row_sync.lrworkerdata)
                .offset(idx as isize))
                .rlbs;
            *fresh11 = (*cm).rlbs as *mut core::ffi::c_void;
        }
        ::core::ptr::write_volatile(
            &raw mut i as *mut core::ffi::c_int,
            ::core::ptr::read_volatile::<core::ffi::c_int>(&i as *const core::ffi::c_int)
                + (*mt_info).num_workers,
        );
        av1_alloc_cdef_buffers(
            cm as *mut AV1Common,
            &raw mut (*p_mt_info).cdef_worker,
            &raw mut (*mt_info).cdef_sync,
            (*p_mt_info).num_workers,
            0 as core::ffi::c_int,
        );
        (*frame_worker).hook = hook;
        (*frame_worker).data1 = cur_cpi as *mut core::ffi::c_void;
        (*frame_worker).data2 = (if frame_idx == 0 as core::ffi::c_int {
            first_cpi_data as *mut AV1_COMP_DATA
        } else {
            &raw mut *((*ppi).parallel_frames_data)
                .as_mut_ptr()
                .offset((frame_idx - 1 as core::ffi::c_int) as isize)
                as *mut AV1_COMP_DATA
        }) as *mut core::ffi::c_void;
        ::core::ptr::write_volatile(
            &raw mut frame_idx as *mut core::ffi::c_int,
            ::core::ptr::read_volatile::<
                core::ffi::c_int,
            >(&frame_idx as *const core::ffi::c_int) + 1,
        );
        (*error).setjmp = 0 as core::ffi::c_int;
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {
            (*error).setjmp = 0 as core::ffi::c_int;
            restore_workers_after_fpmt(ppi, parallel_frame_count, i);
            aom_internal_error_copy(&raw mut (*ppi).error, error);}
}
}}
    (*p_mt_info).p_num_workers = parallel_frame_count;
}
#[inline]
unsafe extern "C-unwind" fn launch_fpmt_workers(mut ppi: *mut AV1_PRIMARY) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut num_workers: core::ffi::c_int = (*ppi).p_mt_info.p_num_workers;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = (*ppi).p_mt_info.p_workers[i as usize];
        if i == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn restore_workers_after_fpmt(
    mut ppi: *mut AV1_PRIMARY,
    mut parallel_frame_count: core::ffi::c_int,
    mut num_fpmt_workers_prepared: core::ffi::c_int,
) {
    if !(parallel_frame_count <= (*ppi).num_fp_contexts
        && parallel_frame_count > 1 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"restore_workers_after_fpmt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1391 as core::ffi::c_int,
            b"parallel_frame_count <= ppi->num_fp_contexts && parallel_frame_count > 1\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let p_mt_info: *mut PrimaryMultiThreadInfo = &raw mut (*ppi).p_mt_info;
    let mut frame_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_fpmt_workers_prepared {
        let mut cur_cpi: *mut AV1_COMP = (*ppi).parallel_cpi[frame_idx as usize];
        let mut mt_info: *mut MultiThreadInfo = &raw mut (*cur_cpi).mt_info;
        let cm: *const AV1_COMMON = &raw mut (*cur_cpi).common;
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        if !((*p_mt_info).cdef_worker).is_null() {
            (*(*mt_info).cdef_worker).srcbuf = (*mt_info).restore_state_buf.cdef_srcbuf;
            let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane < num_planes {
                (*(*mt_info).cdef_worker).colbuf[plane as usize] = (*mt_info)
                    .restore_state_buf
                    .cdef_colbuf[plane as usize];
                plane += 1;
            }
        }
        if is_restoration_used(cm) != 0 {
            let mut idx: core::ffi::c_int = i + (*mt_info).num_workers
                - 1 as core::ffi::c_int;
            if !(idx < (*mt_info).lr_row_sync.num_workers) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 27],
                        [core::ffi::c_char; 27],
                    >(*b"restore_workers_after_fpmt\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    1415 as core::ffi::c_int,
                    b"idx < mt_info->lr_row_sync.num_workers\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let fresh8 = &raw mut (*((*mt_info).lr_row_sync.lrworkerdata)
                .offset(idx as isize))
                .rst_tmpbuf;
            *fresh8 = (*mt_info).restore_state_buf.rst_tmpbuf;
            let fresh9 = &raw mut (*((*mt_info).lr_row_sync.lrworkerdata)
                .offset(idx as isize))
                .rlbs;
            *fresh9 = (*mt_info).restore_state_buf.rlbs as *mut core::ffi::c_void;
        }
        frame_idx += 1;
        i += (*mt_info).num_workers;
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_fpmt_workers(
    mut ppi: *mut AV1_PRIMARY,
    mut frames_in_parallel_set: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut num_workers: core::ffi::c_int = (*ppi).p_mt_info.p_num_workers;
    let mut had_error: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut error: *const aom_internal_error_info = 0 as *const aom_internal_error_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = (*ppi).p_mt_info.p_workers[i as usize];
        if ((*winterface).sync).expect("non-null function pointer")(worker) == 0 {
            had_error = 1 as core::ffi::c_int;
            error = (*(*ppi).parallel_cpi[i as usize]).common.error;
        }
        i -= 1;
    }
    restore_workers_after_fpmt(
        ppi,
        frames_in_parallel_set,
        (*ppi).p_mt_info.num_workers,
    );
    if had_error != 0 {
        aom_internal_error_copy(&raw mut (*ppi).error, error);
    }
}
unsafe extern "C-unwind" fn get_compressed_data_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut cpi: *mut AV1_COMP = arg1 as *mut AV1_COMP;
    let mut cpi_data: *mut AV1_COMP_DATA = arg2 as *mut AV1_COMP_DATA;
    let mut status: core::ffi::c_int = av1_get_compressed_data(cpi, cpi_data);
    return (status == 0) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compress_parallel_frames(
    ppi: *mut AV1_PRIMARY,
    first_cpi_data: *mut AV1_COMP_DATA,
) {
    let mut ref_buffers_used_map: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut frames_in_parallel_set: core::ffi::c_int = av1_init_parallel_frame_context(
        first_cpi_data,
        ppi,
        &raw mut ref_buffers_used_map,
    );
    prepare_fpmt_workers(
        ppi,
        first_cpi_data,
        Some(
            get_compressed_data_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        frames_in_parallel_set,
    );
    launch_fpmt_workers(ppi);
    sync_fpmt_workers(ppi, frames_in_parallel_set);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < frames_in_parallel_set {
        av1_release_scaled_references_fpmt((*ppi).parallel_cpi[i as usize]);
        i += 1;
    }
    av1_decrement_ref_counts_fpmt(
        (*(*ppi).cpi).common.buffer_pool,
        ref_buffers_used_map,
    );
}
#[inline]
unsafe extern "C-unwind" fn launch_workers(
    mt_info: *mut MultiThreadInfo,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        (*worker).had_error = 0 as core::ffi::c_int;
        if i == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_enc_workers(
    mt_info: *mut MultiThreadInfo,
    cm: *mut AV1_COMMON,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let worker_main: *const AVxWorker = &raw mut *((*mt_info).workers)
        .offset(0 as core::ffi::c_int as isize) as *mut AVxWorker;
    let mut had_error: core::ffi::c_int = (*worker_main).had_error;
    let mut error_info: aom_internal_error_info = aom_internal_error_info {
        error_code: AOM_CODEC_OK,
        has_detail: 0,
        detail: [0; 200],
        setjmp: 0,
        jmp: [0; 48],
    };
    if had_error != 0 {
        error_info = (*((*worker_main).data1 as *mut EncWorkerData)).error_info;
    }
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i > 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        if ((*winterface).sync).expect("non-null function pointer")(worker) == 0 {
            had_error = 1 as core::ffi::c_int;
            error_info = (*((*worker).data1 as *mut EncWorkerData)).error_info;
        }
        i -= 1;
    }
    if had_error != 0 {
        aom_internal_error_copy((*cm).error, &raw mut error_info);
    }
    let xd: *mut MACROBLOCKD = &raw mut (*(*((*worker_main).data1 as *mut EncWorkerData)).td)
        .mb
        .e_mbd;
    (*xd).error_info = (*cm).error;
}
#[inline]
unsafe extern "C-unwind" fn accumulate_counters_enc_workers(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*cpi).mt_info.workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = (*worker).data1 as *mut EncWorkerData;
        (*cpi).intrabc_used |= (*(*thread_data).td).intrabc_used;
        (*cpi).deltaq_used |= (*(*thread_data).td).deltaq_used;
        if frame_is_intra_only(&raw mut (*cpi).common) == 0 {
            av1_accumulate_rtc_counters(
                cpi as *mut AV1_COMP,
                &raw mut (*(*thread_data).td).mb,
            );
        }
        (*cpi).palette_pixel_num += (*(*thread_data).td).mb.palette_pixels;
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            if (*cpi).sf.inter_sf.mv_cost_upd_level as core::ffi::c_uint
                != INTERNAL_COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
            {
                aom_free((*(*thread_data).td).mv_costs_alloc as *mut core::ffi::c_void);
                (*(*thread_data).td).mv_costs_alloc = 0 as *mut MvCosts;
            }
            if (*cpi).sf.intra_sf.dv_cost_upd_level as core::ffi::c_uint
                != INTERNAL_COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
            {
                aom_free((*(*thread_data).td).dv_costs_alloc as *mut core::ffi::c_void);
                (*(*thread_data).td).dv_costs_alloc = 0 as *mut IntraBCMVCosts;
            }
        }
        av1_dealloc_mb_data(
            &raw mut (*(*thread_data).td).mb,
            av1_num_planes(&raw mut (*cpi).common),
        );
        if i > 0 as core::ffi::c_int {
            av1_accumulate_frame_counts(&raw mut (*cpi).counts, (*(*thread_data).td).counts);
            accumulate_rd_opt(&raw mut (*cpi).td, (*thread_data).td as *mut ThreadData);
            (*cpi).td.mb.txfm_search_info.txb_split_count = ((*cpi)
                .td
                .mb
                .txfm_search_info
                .txb_split_count)
                .wrapping_add((*(*thread_data).td).mb.txfm_search_info.txb_split_count);
        }
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn prepare_enc_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        let td: *mut ThreadData = if i == 0 as core::ffi::c_int {
            &raw mut (*cpi).td
        } else {
            (*thread_data).original_td as *mut ThreadData
        };
        if !td.is_null() {
            av1_dealloc_mb_data(&raw mut (*td).mb, av1_num_planes(cm));
        }
        i -= 1;
    }
    let mut i_0: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i_0 >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i_0 as isize)
            as *mut AVxWorker;
        let thread_data_0: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i_0 as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data_0 as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data_0).thread_id = i_0;
        (*thread_data_0).start = i_0;
        (*thread_data_0).cpi = cpi as *mut AV1_COMP;
        if i_0 == 0 as core::ffi::c_int {
            (*thread_data_0).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data_0).td = (*thread_data_0).original_td;
        }
        (*(*thread_data_0).td).intrabc_used = 0 as core::ffi::c_int;
        (*(*thread_data_0).td).deltaq_used = 0 as core::ffi::c_int;
        (*(*thread_data_0).td).abs_sum_level = 0 as uint64_t;
        (*(*thread_data_0).td)
            .rd_counts
            .seg_tmp_pred_cost[0 as core::ffi::c_int as usize] = 0 as uint64_t;
        (*(*thread_data_0).td)
            .rd_counts
            .seg_tmp_pred_cost[1 as core::ffi::c_int as usize] = 0 as uint64_t;
        if (*thread_data_0).td != &raw mut (*cpi).td as *mut ThreadData {
            (*(*thread_data_0).td).mb = (*cpi).td.mb;
            (*(*thread_data_0).td).rd_counts = (*cpi).td.rd_counts;
            (*(*thread_data_0).td).mb.obmc_buffer = (*(*thread_data_0).td).obmc_buffer;
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < 2 as core::ffi::c_int {
                (*(*thread_data_0).td)
                    .mb
                    .intrabc_hash_info
                    .hash_value_buffer[x as usize] = (*(*thread_data_0).td)
                    .hash_value_buffer[x as usize];
                x += 1;
            }
            if (*cpi).sf.inter_sf.mv_cost_upd_level as core::ffi::c_uint
                != INTERNAL_COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
            {
                if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"prepare_enc_workers\0"))
                            .as_ptr(),
                        b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                        1623 as core::ffi::c_int,
                        b"((cm)->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*(*thread_data_0).td).mv_costs_alloc = aom_malloc(
                    ::core::mem::size_of::<MvCosts>() as size_t,
                ) as *mut MvCosts;
                if ((*(*thread_data_0).td).mv_costs_alloc).is_null() {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate thread_data->td->mv_costs_alloc at %s:%d\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1623 as core::ffi::c_int,
                    );
                }
                (*(*thread_data_0).td).mb.mv_costs = (*(*thread_data_0).td)
                    .mv_costs_alloc;
                *(*(*thread_data_0).td).mb.mv_costs = *(*cpi).td.mb.mv_costs;
            }
            if (*cpi).sf.intra_sf.dv_cost_upd_level as core::ffi::c_uint
                != INTERNAL_COST_UPD_OFF as core::ffi::c_int as core::ffi::c_uint
            {
                (*(*thread_data_0).td).mb.dv_costs = 0 as *mut IntraBCMVCosts;
                if av1_need_dv_costs(cpi) {
                    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"prepare_enc_workers\0"))
                                .as_ptr(),
                            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                            1635 as core::ffi::c_int,
                            b"((cm)->error)->setjmp\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    (*(*thread_data_0).td).dv_costs_alloc = aom_malloc(
                        ::core::mem::size_of::<IntraBCMVCosts>() as size_t,
                    ) as *mut IntraBCMVCosts;
                    if ((*(*thread_data_0).td).dv_costs_alloc).is_null() {
                        aom_internal_error(
                            (*cm).error,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate thread_data->td->dv_costs_alloc at %s:%d\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1635 as core::ffi::c_int,
                        );
                    }
                    (*(*thread_data_0).td).mb.dv_costs = (*(*thread_data_0).td)
                        .dv_costs_alloc;
                    *(*(*thread_data_0).td).mb.dv_costs = *(*cpi).td.mb.dv_costs;
                }
            }
        }
        av1_alloc_mb_data(cpi, &raw mut (*(*thread_data_0).td).mb);
        av1_init_rtc_counters(&raw mut (*(*thread_data_0).td).mb);
        (*(*thread_data_0).td).mb.palette_pixels = 0 as core::ffi::c_int;
        if (*(*thread_data_0).td).counts != &raw mut (*cpi).counts as *mut FRAME_COUNTS {
            *(*(*thread_data_0).td).counts = (*cpi).counts;
        }
        if i_0 > 0 as core::ffi::c_int {
            (*(*thread_data_0).td).mb.palette_buffer = (*(*thread_data_0).td)
                .palette_buffer;
            (*(*thread_data_0).td).mb.comp_rd_buffer = (*(*thread_data_0).td)
                .comp_rd_buffer;
            (*(*thread_data_0).td).mb.tmp_conv_dst = (*(*thread_data_0).td).tmp_conv_dst;
            (*(*thread_data_0).td).mb.upsample_pred = (*(*thread_data_0).td)
                .upsample_pred;
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < 2 as core::ffi::c_int {
                (*(*thread_data_0).td).mb.tmp_pred_bufs[j as usize] = (*(*thread_data_0)
                    .td)
                    .tmp_pred_bufs[j as usize];
                j += 1;
            }
            (*(*thread_data_0).td).mb.pixel_gradient_info = (*(*thread_data_0).td)
                .pixel_gradient_info;
            (*(*thread_data_0).td).mb.src_var_info_of_4x4_sub_blocks = (*(*thread_data_0)
                .td)
                .src_var_info_of_4x4_sub_blocks;
            (*(*thread_data_0).td).mb.e_mbd.tmp_conv_dst = (*(*thread_data_0).td)
                .mb
                .tmp_conv_dst;
            (*(*thread_data_0).td).mb.e_mbd.tmp_upsample_pred = (*(*thread_data_0).td)
                .mb
                .upsample_pred;
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < 2 as core::ffi::c_int {
                (*(*thread_data_0).td).mb.e_mbd.tmp_obmc_bufs[j_0 as usize] = (*(*thread_data_0)
                    .td)
                    .mb
                    .tmp_pred_bufs[j_0 as usize];
                j_0 += 1;
            }
        }
        i_0 -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn fp_prepare_enc_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data).thread_id = i;
        (*thread_data).start = i;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
            (*(*thread_data).td).mb = (*cpi).td.mb;
        }
        av1_alloc_src_diff_buf(cm, &raw mut (*(*thread_data).td).mb);
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn compute_num_enc_row_mt_workers(
    mut cm: *const AV1_COMMON,
    mut max_threads: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut tile_info: TileInfo = TileInfo {
        mi_row_start: 0,
        mi_row_end: 0,
        mi_col_start: 0,
        mi_col_end: 0,
        tile_row: 0,
        tile_col: 0,
    };
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut total_num_threads_row_mt: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < tile_cols {
            av1_tile_init(&raw mut tile_info, cm as *const AV1Common, row, col);
            let num_sb_rows_in_tile: core::ffi::c_int = av1_get_sb_rows_in_tile(
                cm as *const AV1Common,
                &raw mut tile_info,
            ) as core::ffi::c_int;
            let num_sb_cols_in_tile: core::ffi::c_int = av1_get_sb_cols_in_tile(
                cm as *const AV1Common,
                &raw mut tile_info,
            ) as core::ffi::c_int;
            total_num_threads_row_mt
                += if (num_sb_cols_in_tile + 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) < num_sb_rows_in_tile
                {
                    num_sb_cols_in_tile + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
                } else {
                    num_sb_rows_in_tile
                };
            col += 1;
        }
        row += 1;
    }
    return if max_threads < total_num_threads_row_mt {
        max_threads
    } else {
        total_num_threads_row_mt
    };
}
#[inline]
unsafe extern "C-unwind" fn compute_num_enc_tile_mt_workers(
    mut cm: *const AV1_COMMON,
    mut max_threads: core::ffi::c_int,
) -> core::ffi::c_int {
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    return if max_threads < tile_cols * tile_rows {
        max_threads
    } else {
        tile_cols * tile_rows
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_max_num_workers(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let mut max_num_workers: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = MOD_FP as core::ffi::c_int;
    while i < NUM_MT_MODULES as core::ffi::c_int {
        max_num_workers = if (*(*cpi).ppi).p_mt_info.num_mod_workers[i as usize]
            > max_num_workers
        {
            (*(*cpi).ppi).p_mt_info.num_mod_workers[i as usize]
        } else {
            max_num_workers
        };
        i += 1;
    }
    if !(max_num_workers >= 1 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_get_max_num_workers\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1741 as core::ffi::c_int,
            b"max_num_workers >= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return if max_num_workers < (*cpi).oxcf.max_threads {
        max_num_workers
    } else {
        (*cpi).oxcf.max_threads
    };
}
unsafe extern "C-unwind" fn compute_num_enc_workers(
    mut cpi: *const AV1_COMP,
    mut max_workers: core::ffi::c_int,
) -> core::ffi::c_int {
    if max_workers <= 1 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    if (*cpi).oxcf.row_mt {
        return compute_num_enc_row_mt_workers(&(*cpi).common, max_workers)
    } else {
        return compute_num_enc_tile_mt_workers(&(*cpi).common, max_workers)
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encode_tiles_mt(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut num_workers: core::ffi::c_int = (*mt_info)
        .num_mod_workers[MOD_ENC as core::ffi::c_int as usize];
    if !(!((*cpi).tile_data).is_null()
        || (*cpi).allocated_tiles == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_encode_tiles_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1761 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, cpi->allocated_tiles == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*cpi).allocated_tiles != tile_cols * tile_rows {
        av1_alloc_tile_data(cpi as *mut AV1_COMP);
    }
    av1_init_tile_data(cpi as *mut AV1_COMP);
    num_workers = if num_workers < (*mt_info).num_workers {
        num_workers
    } else {
        (*mt_info).num_workers
    };
    prepare_enc_workers(
        cpi,
        Some(
            enc_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(&raw mut (*cpi).mt_info, num_workers);
    sync_enc_workers(&raw mut (*cpi).mt_info, cm, num_workers);
    accumulate_counters_enc_workers(cpi, num_workers);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_accumulate_frame_counts(
    mut acc_counts: *mut FRAME_COUNTS,
    mut counts: *const FRAME_COUNTS,
) {
    let acc: *mut core::ffi::c_uint = acc_counts as *mut core::ffi::c_uint;
    let cnt: *const core::ffi::c_uint = counts as *const core::ffi::c_uint;
    let n_counts: core::ffi::c_uint = (::core::mem::size_of::<FRAME_COUNTS>() as usize)
        .wrapping_div(::core::mem::size_of::<core::ffi::c_uint>() as usize)
        as core::ffi::c_uint;
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while i < n_counts {
        let fresh1 = &raw mut *acc.offset(i as isize);
        *fresh1 = (*fresh1).wrapping_add(*cnt.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C-unwind" fn compute_max_sb_rows_cols(
    mut cm: *const AV1_COMMON,
    mut max_sb_rows_in_tile: *mut core::ffi::c_int,
    mut max_sb_cols_in_tile: *mut core::ffi::c_int,
) {
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mib_size_log2: core::ffi::c_int = (*(*cm).seq_params).mib_size_log2;
    let num_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    let row_start_sb: *const core::ffi::c_int = ((*cm).tiles.row_start_sb).as_ptr();
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mi_row_start: core::ffi::c_int = *row_start_sb.offset(row as isize)
            << mib_size_log2;
        let mi_row_end: core::ffi::c_int = if *row_start_sb
            .offset((row + 1 as core::ffi::c_int) as isize) << mib_size_log2
            < num_mi_rows
        {
            *row_start_sb.offset((row + 1 as core::ffi::c_int) as isize) << mib_size_log2
        } else {
            num_mi_rows
        };
        let num_sb_rows_in_tile: core::ffi::c_int = mi_row_end - mi_row_start
            + ((1 as core::ffi::c_int) << mib_size_log2) - 1 as core::ffi::c_int
            >> mib_size_log2;
        *max_sb_rows_in_tile = if *max_sb_rows_in_tile > num_sb_rows_in_tile {
            *max_sb_rows_in_tile
        } else {
            num_sb_rows_in_tile
        };
        row += 1;
    }
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let num_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let col_start_sb: *const core::ffi::c_int = ((*cm).tiles.col_start_sb).as_ptr();
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < tile_cols {
        let mi_col_start: core::ffi::c_int = *col_start_sb.offset(col as isize)
            << mib_size_log2;
        let mi_col_end: core::ffi::c_int = if *col_start_sb
            .offset((col + 1 as core::ffi::c_int) as isize) << mib_size_log2
            < num_mi_cols
        {
            *col_start_sb.offset((col + 1 as core::ffi::c_int) as isize) << mib_size_log2
        } else {
            num_mi_cols
        };
        let num_sb_cols_in_tile: core::ffi::c_int = mi_col_end - mi_col_start
            + ((1 as core::ffi::c_int) << mib_size_log2) - 1 as core::ffi::c_int
            >> mib_size_log2;
        *max_sb_cols_in_tile = if *max_sb_cols_in_tile > num_sb_cols_in_tile {
            *max_sb_cols_in_tile
        } else {
            num_sb_cols_in_tile
        };
        col += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fp_compute_num_enc_workers(
    mut cpi: *mut AV1_COMP,
) -> core::ffi::c_int {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut total_num_threads_row_mt: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_info: TileInfo = TileInfo {
        mi_row_start: 0,
        mi_row_end: 0,
        mi_col_start: 0,
        mi_col_end: 0,
        tile_row: 0,
        tile_col: 0,
    };
    if (*cpi).oxcf.max_threads <= 1 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < tile_cols {
            av1_tile_init(&raw mut tile_info, cm, row, col);
            let num_mb_rows_in_tile: core::ffi::c_int = av1_get_unit_rows_in_tile(
                &raw mut tile_info,
                (*cpi).fp_block_size,
            ) as core::ffi::c_int;
            let num_mb_cols_in_tile: core::ffi::c_int = av1_get_unit_cols_in_tile(
                &raw mut tile_info,
                (*cpi).fp_block_size,
            ) as core::ffi::c_int;
            total_num_threads_row_mt
                += if (num_mb_cols_in_tile + 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) < num_mb_rows_in_tile
                {
                    num_mb_cols_in_tile + 1 as core::ffi::c_int >> 1 as core::ffi::c_int
                } else {
                    num_mb_rows_in_tile
                };
            col += 1;
        }
        row += 1;
    }
    return if (*cpi).oxcf.max_threads < total_num_threads_row_mt {
        (*cpi).oxcf.max_threads
    } else {
        total_num_threads_row_mt
    };
}
#[inline]
unsafe extern "C-unwind" fn fp_compute_max_mb_rows(
    mut cm: *const AV1_COMMON,
    mut fp_block_size: BLOCK_SIZE,
) -> core::ffi::c_int {
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let unit_height_log2: core::ffi::c_int = mi_size_high_log2[fp_block_size as usize]
        as core::ffi::c_int;
    let mib_size_log2: core::ffi::c_int = (*(*cm).seq_params).mib_size_log2;
    let num_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    let row_start_sb: *const core::ffi::c_int = ((*cm).tiles.row_start_sb).as_ptr();
    let mut max_mb_rows: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mi_row_start: core::ffi::c_int = *row_start_sb.offset(row as isize)
            << mib_size_log2;
        let mi_row_end: core::ffi::c_int = if *row_start_sb
            .offset((row + 1 as core::ffi::c_int) as isize) << mib_size_log2
            < num_mi_rows
        {
            *row_start_sb.offset((row + 1 as core::ffi::c_int) as isize) << mib_size_log2
        } else {
            num_mi_rows
        };
        let num_mb_rows_in_tile: core::ffi::c_int = mi_row_end - mi_row_start
            + ((1 as core::ffi::c_int) << unit_height_log2) - 1 as core::ffi::c_int
            >> unit_height_log2;
        max_mb_rows = if max_mb_rows > num_mb_rows_in_tile {
            max_mb_rows
        } else {
            num_mb_rows_in_tile
        };
        row += 1;
    }
    return max_mb_rows;
}
unsafe extern "C-unwind" fn lpf_pipeline_mt_init(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let use_loopfilter: core::ffi::c_int = is_loopfilter_used(cm) as core::ffi::c_int;
    let use_superres: core::ffi::c_int = av1_superres_scaled(cm) as core::ffi::c_int;
    let use_cdef: core::ffi::c_int = is_cdef_used(cm) as core::ffi::c_int;
    let use_restoration: core::ffi::c_int = is_restoration_used(cm) as core::ffi::c_int;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut xd: *mut MACROBLOCKD = &raw mut (*cpi).td.mb.e_mbd;
    let skip_apply_postproc_filters: core::ffi::c_uint = derive_skip_apply_postproc_filters(
        cpi,
        use_loopfilter,
        use_cdef,
        use_superres,
        use_restoration,
    ) as core::ffi::c_uint;
    (*mt_info).pipeline_lpf_mt_with_enc = ((*cpi).oxcf.mode as core::ffi::c_int
        == REALTIME as core::ffi::c_int && (*cpi).oxcf.speed >= 5 as core::ffi::c_int
        && (*cpi).sf.lpf_sf.lpf_pick as core::ffi::c_int
            == LPF_PICK_FROM_Q as core::ffi::c_int
        && (*cpi).oxcf.algo_cfg.loopfilter_control as core::ffi::c_uint
            != LOOPFILTER_SELECTIVELY as core::ffi::c_int as core::ffi::c_uint
        && (*(*cpi).ppi).rtc_ref.non_reference_frame == 0
        && !(*cm).features.allow_intrabc
        && skip_apply_postproc_filters
            & SKIP_APPLY_LOOPFILTER as core::ffi::c_int as core::ffi::c_uint
            == 0 as core::ffi::c_uint) as core::ffi::c_int;
    if (*mt_info).pipeline_lpf_mt_with_enc == 0 {
        return;
    }
    set_postproc_filter_default_params(cm);
    if use_loopfilter == 0 {
        return;
    }
    let method: LPF_PICK_METHOD = (*cpi).sf.lpf_sf.lpf_pick;
    if !(method as core::ffi::c_int == LPF_PICK_FROM_Q as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"lpf_pipeline_mt_init\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1895 as core::ffi::c_int,
            b"method == LPF_PICK_FROM_Q\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*cpi).oxcf.algo_cfg.loopfilter_control as core::ffi::c_uint
        != LOOPFILTER_SELECTIVELY as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"lpf_pipeline_mt_init\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1896 as core::ffi::c_int,
            b"cpi->oxcf.algo_cfg.loopfilter_control != LOOPFILTER_SELECTIVELY\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_pick_filter_level((*cpi).source, cpi as *mut AV1_COMP, method);
    let mut lf: *mut loopfilter = &raw mut (*cm).lf;
    let plane_start: core::ffi::c_int = 0 as core::ffi::c_int;
    let plane_end: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut planes_to_lf: [core::ffi::c_int; 3] = [0; 3];
    if lpf_mt_with_enc_enabled(
        (*cpi).mt_info.pipeline_lpf_mt_with_enc,
        ((*lf).filter_level).as_mut_ptr() as *const core::ffi::c_int,
    ) {
        set_planes_to_loop_filter(lf, planes_to_lf.as_mut_ptr(), plane_start, plane_end);
        let mut lpf_opt_level: core::ffi::c_int = get_lpf_opt_level(&raw mut (*cpi).sf);
        if !(lpf_opt_level == 2 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"lpf_pipeline_mt_init\0"))
                    .as_ptr(),
                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                1908 as core::ffi::c_int,
                b"lpf_opt_level == 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let start_mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
        let end_mi_row: core::ffi::c_int = start_mi_row + (*cm).mi_params.mi_rows;
        av1_loop_filter_frame_init(cm as *mut AV1Common, plane_start, plane_end);
        if !((*mt_info).num_mod_workers[MOD_ENC as core::ffi::c_int as usize]
            == (*mt_info).num_mod_workers[MOD_LPF as core::ffi::c_int as usize])
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"lpf_pipeline_mt_init\0"))
                    .as_ptr(),
                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                1916 as core::ffi::c_int,
                b"mt_info->num_mod_workers[MOD_ENC] == mt_info->num_mod_workers[MOD_LPF]\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        loop_filter_frame_mt_init(
            cm,
            start_mi_row,
            end_mi_row,
            planes_to_lf.as_mut_ptr() as *const core::ffi::c_int,
            (*mt_info).num_mod_workers[MOD_LPF as core::ffi::c_int as usize],
            &raw mut (*mt_info).lf_row_sync,
            lpf_opt_level,
            (*(*cm).seq_params).mib_size_log2,
        );
        let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
        while i >= 0 as core::ffi::c_int {
            let thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
                .offset(i as isize) as *mut EncWorkerData;
            (*thread_data).lf_sync = &raw mut (*mt_info).lf_row_sync;
            (*thread_data).lf_data = &raw mut *((*(*thread_data).lf_sync).lfdata)
                .offset(i as isize) as *mut LFWorkerData;
            loop_filter_data_reset(
                (*thread_data).lf_data,
                &raw mut (*(*cm).cur_frame).buf,
                cm as *mut AV1Common,
                xd,
            );
            i -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encode_tiles_row_mt(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*mt_info).enc_row_mt;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let sb_rows_in_frame: core::ffi::c_int = get_sb_rows_in_frame(cm)
        as core::ffi::c_int;
    let mut thread_id_to_tile_id: *mut core::ffi::c_int = ((*enc_row_mt)
        .thread_id_to_tile_id)
        .as_mut_ptr();
    let mut max_sb_rows_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_sb_cols_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_workers: core::ffi::c_int = (*mt_info)
        .num_mod_workers[MOD_ENC as core::ffi::c_int as usize];
    compute_max_sb_rows_cols(cm, &raw mut max_sb_rows_in_tile, &raw mut max_sb_cols_in_tile);
    let alloc_row_mt_mem: bool = (*enc_row_mt).allocated_tile_cols != tile_cols
        || (*enc_row_mt).allocated_tile_rows != tile_rows
        || (*enc_row_mt).allocated_rows != max_sb_rows_in_tile
        || (*enc_row_mt).allocated_cols != max_sb_cols_in_tile - 1 as core::ffi::c_int
        || (*enc_row_mt).allocated_sb_rows != sb_rows_in_frame;
    let alloc_tile_data: bool = (*cpi).allocated_tiles != tile_cols * tile_rows;
    if !(!((*cpi).tile_data).is_null() || alloc_tile_data as core::ffi::c_int != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1952 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, alloc_tile_data)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(!((*cpi).tile_data).is_null()
        || (*cpi).allocated_tiles == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1953 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, cpi->allocated_tiles == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if alloc_tile_data {
        av1_alloc_tile_data(cpi as *mut AV1_COMP);
    }
    if !(!alloc_tile_data || alloc_row_mt_mem as core::ffi::c_int != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            1958 as core::ffi::c_int,
            b"IMPLIES(alloc_tile_data, alloc_row_mt_mem)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if alloc_row_mt_mem {
        row_mt_mem_alloc(
            cpi,
            max_sb_rows_in_tile,
            max_sb_cols_in_tile,
            (*cpi).oxcf.algo_cfg.cdf_update_mode as core::ffi::c_int,
        );
    }
    num_workers = if num_workers < (*mt_info).num_workers {
        num_workers
    } else {
        (*mt_info).num_workers
    };
    lpf_pipeline_mt_init(cpi, num_workers);
    av1_init_tile_data(cpi as *mut AV1_COMP);
    memset(
        thread_id_to_tile_id as *mut core::ffi::c_void,
        -(1 as core::ffi::c_int),
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(MAX_NUM_THREADS as size_t),
    );
    memset(
        (*enc_row_mt).num_tile_cols_done as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(sb_rows_in_frame as size_t),
    );
    (*enc_row_mt).row_mt_exit = false_0 != 0;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let mut tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_index as isize) as *mut TileDataEnc;
            let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile)
                .row_mt_sync;
            memset(
                (*row_mt_sync).num_finished_cols as *mut core::ffi::c_void,
                -(1 as core::ffi::c_int),
                (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                    .wrapping_mul(max_sb_rows_in_tile as size_t),
            );
            (*row_mt_sync).next_mi_row = (*this_tile).tile_info.mi_row_start;
            (*row_mt_sync).num_threads_working = 0 as core::ffi::c_int;
            (*row_mt_sync).intrabc_extra_top_right_sb_delay = av1_get_intrabc_extra_top_right_sb_delay(
                cm,
            );
            av1_inter_mode_data_init(this_tile as *mut TileDataEnc);
            av1_zero_above_context(
                cm,
                &raw mut (*cpi).td.mb.e_mbd,
                (*this_tile).tile_info.mi_col_start,
                (*this_tile).tile_info.mi_col_end,
                tile_row,
            );
            tile_col += 1;
        }
        tile_row += 1;
    }
    assign_tile_to_thread(thread_id_to_tile_id, tile_cols * tile_rows, num_workers);
    prepare_enc_workers(
        cpi,
        Some(
            enc_row_mt_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(&raw mut (*cpi).mt_info, num_workers);
    sync_enc_workers(&raw mut (*cpi).mt_info, cm, num_workers);
    if (*cm).delta_q_info.delta_lf_present_flag != 0 {
        update_delta_lf_for_row_mt(cpi);
    }
    accumulate_counters_enc_workers(cpi, num_workers);
}
unsafe extern "C-unwind" fn dealloc_thread_data_src_diff_buf(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let thread_data: *mut EncWorkerData = &raw mut *((*cpi).mt_info.tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            av1_dealloc_src_diff_buf(
                &raw mut (*(*thread_data).td).mb,
                av1_num_planes(&raw mut (*cpi).common),
            );
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fp_encode_tiles_row_mt(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*mt_info).enc_row_mt;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut thread_id_to_tile_id: *mut core::ffi::c_int = ((*enc_row_mt)
        .thread_id_to_tile_id)
        .as_mut_ptr();
    let mut num_workers: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_mb_rows: core::ffi::c_int = 0 as core::ffi::c_int;
    max_mb_rows = fp_compute_max_mb_rows(cm, (*cpi).fp_block_size);
    let alloc_row_mt_mem: bool = (*enc_row_mt).allocated_tile_cols != tile_cols
        || (*enc_row_mt).allocated_tile_rows != tile_rows
        || (*enc_row_mt).allocated_rows != max_mb_rows;
    let alloc_tile_data: bool = (*cpi).allocated_tiles != tile_cols * tile_rows;
    if !(!((*cpi).tile_data).is_null() || alloc_tile_data as core::ffi::c_int != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_fp_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2031 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, alloc_tile_data)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(!((*cpi).tile_data).is_null()
        || (*cpi).allocated_tiles == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_fp_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2032 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, cpi->allocated_tiles == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if alloc_tile_data {
        av1_alloc_tile_data(cpi as *mut AV1_COMP);
    }
    if !(!alloc_tile_data || alloc_row_mt_mem as core::ffi::c_int != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_fp_encode_tiles_row_mt\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2037 as core::ffi::c_int,
            b"IMPLIES(alloc_tile_data, alloc_row_mt_mem)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if alloc_row_mt_mem {
        row_mt_mem_alloc(
            cpi,
            max_mb_rows,
            -(1 as core::ffi::c_int),
            0 as core::ffi::c_int,
        );
    }
    av1_init_tile_data(cpi as *mut AV1_COMP);
    if (*mt_info).num_mod_workers[MOD_FP as core::ffi::c_int as usize]
        == 0 as core::ffi::c_int
    {
        num_workers = av1_fp_compute_num_enc_workers(cpi);
    } else {
        num_workers = (*mt_info).num_mod_workers[MOD_FP as core::ffi::c_int as usize];
    }
    memset(
        thread_id_to_tile_id as *mut core::ffi::c_void,
        -(1 as core::ffi::c_int),
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(MAX_NUM_THREADS as size_t),
    );
    (*enc_row_mt).firstpass_mt_exit = false_0 != 0;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let mut tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_index as isize) as *mut TileDataEnc;
            let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*this_tile)
                .row_mt_sync;
            memset(
                (*row_mt_sync).num_finished_cols as *mut core::ffi::c_void,
                -(1 as core::ffi::c_int),
                (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                    .wrapping_mul(max_mb_rows as size_t),
            );
            (*row_mt_sync).next_mi_row = (*this_tile).tile_info.mi_row_start;
            (*row_mt_sync).num_threads_working = 0 as core::ffi::c_int;
            (*row_mt_sync).intrabc_extra_top_right_sb_delay = 0 as core::ffi::c_int;
            tile_col += 1;
        }
        tile_row += 1;
    }
    num_workers = if num_workers < (*mt_info).num_workers {
        num_workers
    } else {
        (*mt_info).num_workers
    };
    assign_tile_to_thread(thread_id_to_tile_id, tile_cols * tile_rows, num_workers);
    fp_prepare_enc_workers(
        cpi,
        Some(
            fp_enc_row_mt_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(&raw mut (*cpi).mt_info, num_workers);
    sync_enc_workers(&raw mut (*cpi).mt_info, cm, num_workers);
    dealloc_thread_data_src_diff_buf(cpi, num_workers);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tpl_row_mt_sync_read_dummy(
    mut tpl_mt_sync: *mut AV1TplRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tpl_row_mt_sync_write_dummy(
    mut tpl_mt_sync: *mut AV1TplRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut cols: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tpl_row_mt_sync_read(
    mut tpl_row_mt_sync: *mut AV1TplRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
) {
    let mut nsync: core::ffi::c_int = (*tpl_row_mt_sync).sync_range;
    if r != 0 {
        let mutex: *mut pthread_mutex_t = &raw mut *((*tpl_row_mt_sync).mutex_)
            .offset((r - 1 as core::ffi::c_int) as isize) as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex as *mut pthread_mutex_t);
        while c
            > *((*tpl_row_mt_sync).num_finished_cols)
                .offset((r - 1 as core::ffi::c_int) as isize) - nsync
        {
            pthread_cond_wait(
                &raw mut *((*tpl_row_mt_sync).cond_)
                    .offset((r - 1 as core::ffi::c_int) as isize),
                mutex as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock(mutex as *mut pthread_mutex_t);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tpl_row_mt_sync_write(
    mut tpl_row_mt_sync: *mut AV1TplRowMultiThreadSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut cols: core::ffi::c_int,
) {
    let mut nsync: core::ffi::c_int = (*tpl_row_mt_sync).sync_range;
    let mut cur: core::ffi::c_int = 0;
    let mut sig: core::ffi::c_int = 1 as core::ffi::c_int;
    if c < cols - 1 as core::ffi::c_int {
        cur = c;
        if c % nsync != 0 {
            sig = 0 as core::ffi::c_int;
        }
    } else {
        cur = cols + nsync;
    }
    if sig != 0 {
        pthread_mutex_lock(&raw mut *((*tpl_row_mt_sync).mutex_).offset(r as isize));
        *((*tpl_row_mt_sync).num_finished_cols).offset(r as isize) = if *((*tpl_row_mt_sync)
            .num_finished_cols)
            .offset(r as isize) > cur
        {
            *((*tpl_row_mt_sync).num_finished_cols).offset(r as isize)
        } else {
            cur
        };
        pthread_cond_signal(&raw mut *((*tpl_row_mt_sync).cond_).offset(r as isize));
        pthread_mutex_unlock(&raw mut *((*tpl_row_mt_sync).mutex_).offset(r as isize));
    }
}
#[inline]
unsafe extern "C-unwind" fn set_mode_estimation_done(mut cpi: *mut AV1_COMP) {
    let mi_params: *const CommonModeInfoParams = &raw mut (*cpi).common.mi_params;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let bsize: BLOCK_SIZE = convert_length_to_bsize(
        (*(*cpi).ppi).tpl_data.tpl_bsize_1d as core::ffi::c_int,
    ) as BLOCK_SIZE;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let tpl_row_mt: *mut AV1TplRowMultiThreadInfo = &raw mut (*cpi).mt_info.tpl_row_mt;
    let tplb_cols_in_tile: core::ffi::c_int = (*mi_params).mi_cols
        + ((1 as core::ffi::c_int)
            << mi_size_wide_log2[bsize as usize] as core::ffi::c_int
            >> 1 as core::ffi::c_int)
        >> mi_size_wide_log2[bsize as usize] as core::ffi::c_int;
    let mut mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tplb_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while mi_row < (*mi_params).mi_rows {
        (Some(((*tpl_row_mt).sync_write_ptr).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            &raw mut (*tpl_data).tpl_mt_sync,
            tplb_row,
            tplb_cols_in_tile - 1 as core::ffi::c_int,
            tplb_cols_in_tile,
        );
        mi_row += mi_height;
        tplb_row += 1;
    }
}
unsafe extern "C-unwind" fn tpl_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let mut cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut x: *mut MACROBLOCK = &raw mut (*(*thread_data).td).mb;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut tpl_txfm_stats: *mut TplTxfmStats = &raw mut (*(*thread_data).td).tpl_txfm_stats;
    let mut tpl_tmp_buffers: *mut TplBuffers = &raw mut (*(*thread_data).td).tpl_tmp_buffers;
    let mut mi_params: *mut CommonModeInfoParams = &raw mut (*cm).mi_params;
    let mut num_active_workers: core::ffi::c_int = (*(*cpi).ppi)
        .tpl_data
        .tpl_mt_sync
        .num_threads_working;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    (*xd).error_info = error_info;
    let tpl_row_mt: *mut AV1TplRowMultiThreadInfo = &raw mut (*cpi).mt_info.tpl_row_mt;
    let mut tpl_error_mutex_: *mut pthread_mutex_t = (*tpl_row_mt).mutex_;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mut bsize: BLOCK_SIZE = convert_length_to_bsize(
        (*(*cpi).ppi).tpl_data.tpl_bsize_1d as core::ffi::c_int,
    );
    let mut tx_size: TX_SIZE = max_txsize_lookup[bsize as usize];
    let mut mi_height: core::ffi::c_int = mi_size_high[bsize as usize]
        as core::ffi::c_int;
    av1_init_tpl_txfm_stats(tpl_txfm_stats);
    let mut mi_row: core::ffi::c_int = (*thread_data).start * mi_height;
    while mi_row < (*mi_params).mi_rows {
        av1_set_mv_row_limits(
            mi_params,
            &raw mut (*x).mv_limits,
            mi_row,
            mi_height,
            (*cpi).oxcf.border_in_pixels,
        );
        (*xd).mb_to_top_edge = -(mi_row
            * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int)
            * 8 as core::ffi::c_int);
        (*xd).mb_to_bottom_edge = ((*mi_params).mi_rows - mi_height - mi_row)
            * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int;
        av1_mc_flow_dispenser_row(
            cpi as *mut AV1_COMP,
            tpl_txfm_stats,
            tpl_tmp_buffers,
            x,
            mi_row,
            bsize,
            tx_size,
        );
        mi_row += num_active_workers * mi_height;
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(tpl_error_mutex_ as *mut pthread_mutex_t);
        (*tpl_row_mt).tpl_mt_exit = true_0 != 0;
        pthread_mutex_unlock(tpl_error_mutex_ as *mut pthread_mutex_t);
        set_mode_estimation_done(cpi);
        return 0 as core::ffi::c_int;}
}
}}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tpl_dealloc(mut tpl_sync: *mut AV1TplRowMultiThreadSync) {
    if tpl_sync.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_tpl_dealloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2234 as core::ffi::c_int,
            b"tpl_sync != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*tpl_sync).mutex_).is_null() {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*tpl_sync).rows {
            pthread_mutex_destroy(&raw mut *((*tpl_sync).mutex_).offset(i as isize));
            i += 1;
        }
        aom_free((*tpl_sync).mutex_ as *mut core::ffi::c_void);
    }
    if !((*tpl_sync).cond_).is_null() {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < (*tpl_sync).rows {
            pthread_cond_destroy(&raw mut *((*tpl_sync).cond_).offset(i_0 as isize));
            i_0 += 1;
        }
        aom_free((*tpl_sync).cond_ as *mut core::ffi::c_void);
    }
    aom_free((*tpl_sync).num_finished_cols as *mut core::ffi::c_void);
    memset(
        tpl_sync as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AV1TplRowMultiThreadSync>() as size_t,
    );
}
unsafe extern "C-unwind" fn av1_tpl_alloc(
    mut tpl_sync: *mut AV1TplRowMultiThreadSync,
    mut cm: *mut AV1_COMMON,
    mut mb_rows: core::ffi::c_int,
) {
    (*tpl_sync).rows = mb_rows;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"av1_tpl_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2262 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tpl_sync).mutex_ = aom_malloc(
        (::core::mem::size_of::<pthread_mutex_t>() as size_t)
            .wrapping_mul(mb_rows as size_t),
    ) as *mut pthread_mutex_t;
    if ((*tpl_sync).mutex_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate tpl_sync->mutex_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            2262 as core::ffi::c_int,
        );
    }
    if !((*tpl_sync).mutex_).is_null() {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < mb_rows {
            pthread_mutex_init(
                &raw mut *((*tpl_sync).mutex_).offset(i as isize),
                0 as *const pthread_mutexattr_t,
            );
            i += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"av1_tpl_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2269 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tpl_sync).cond_ = aom_malloc(
        (::core::mem::size_of::<pthread_cond_t>() as size_t)
            .wrapping_mul(mb_rows as size_t),
    ) as *mut pthread_cond_t;
    if ((*tpl_sync).cond_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate tpl_sync->cond_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            2269 as core::ffi::c_int,
        );
    }
    if !((*tpl_sync).cond_).is_null() {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < mb_rows {
            pthread_cond_init(
                &raw mut *((*tpl_sync).cond_).offset(i_0 as isize),
                0 as *const pthread_condattr_t,
            );
            i_0 += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"av1_tpl_alloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2277 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tpl_sync).num_finished_cols = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(mb_rows as size_t),
    ) as *mut core::ffi::c_int;
    if ((*tpl_sync).num_finished_cols).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate tpl_sync->num_finished_cols at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/ethread.c\0" as *const u8
                as *const core::ffi::c_char,
            2277 as core::ffi::c_int,
        );
    }
    (*tpl_sync).sync_range = 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn prepare_tpl_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data).thread_id = i;
        (*thread_data).start = i;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
        }
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            (*(*thread_data).td).mb = (*cpi).td.mb;
            av1_init_obmc_buffer(&raw mut (*(*thread_data).td).mb.obmc_buffer);
            if !tpl_alloc_temp_buffers(
                &raw mut (*(*thread_data).td).tpl_tmp_buffers,
                (*(*cpi).ppi).tpl_data.tpl_bsize_1d,
            ) {
                aom_internal_error(
                    (*cpi).common.error,
                    AOM_CODEC_MEM_ERROR,
                    b"Error allocating tpl data\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            (*(*thread_data).td).mb.tmp_conv_dst = (*(*thread_data).td).tmp_conv_dst;
            (*(*thread_data).td).mb.e_mbd.tmp_conv_dst = (*(*thread_data).td)
                .mb
                .tmp_conv_dst;
            (*(*thread_data).td).mb.upsample_pred = (*(*thread_data).td).upsample_pred;
            (*(*thread_data).td).mb.e_mbd.tmp_upsample_pred = (*(*thread_data).td)
                .mb
                .upsample_pred;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_mc_flow_dispenser_mt(mut cpi: *mut AV1_COMP) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut mi_params: *mut CommonModeInfoParams = &raw mut (*cm).mi_params;
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let mut tpl_sync: *mut AV1TplRowMultiThreadSync = &raw mut (*tpl_data).tpl_mt_sync;
    let mut mb_rows: core::ffi::c_int = (*mi_params).mb_rows;
    let mut num_workers: core::ffi::c_int = if (*mt_info)
        .num_mod_workers[MOD_TPL as core::ffi::c_int as usize] < (*mt_info).num_workers
    {
        (*mt_info).num_mod_workers[MOD_TPL as core::ffi::c_int as usize]
    } else {
        (*mt_info).num_workers
    };
    if mb_rows != (*tpl_sync).rows {
        av1_tpl_dealloc(tpl_sync);
        av1_tpl_alloc(tpl_sync, cm, mb_rows);
    }
    (*tpl_sync).num_threads_working = num_workers;
    (*mt_info).tpl_row_mt.tpl_mt_exit = false_0 != 0;
    memset(
        (*tpl_sync).num_finished_cols as *mut core::ffi::c_void,
        -(1 as core::ffi::c_int),
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(mb_rows as size_t),
    );
    prepare_tpl_workers(
        cpi,
        Some(
            tpl_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(&raw mut (*cpi).mt_info, num_workers);
    sync_enc_workers(&raw mut (*cpi).mt_info, cm, num_workers);
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if td != &raw mut (*cpi).td as *mut ThreadData {
            tpl_dealloc_temp_buffers(&raw mut (*td).tpl_tmp_buffers);
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tf_mt_dealloc(mut tf_sync: *mut AV1TemporalFilterSync) {
    if tf_sync.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"av1_tf_mt_dealloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2382 as core::ffi::c_int,
            b"tf_sync != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*tf_sync).mutex_).is_null() {
        pthread_mutex_destroy((*tf_sync).mutex_ as *mut pthread_mutex_t);
        aom_free((*tf_sync).mutex_ as *mut core::ffi::c_void);
    }
    (*tf_sync).next_tf_row = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn tf_get_next_job(
    mut tf_mt_sync: *mut AV1TemporalFilterSync,
    mut current_mb_row: *mut core::ffi::c_int,
    mut mb_rows: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut do_next_row: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tf_mutex_: *mut pthread_mutex_t = (*tf_mt_sync).mutex_;
    pthread_mutex_lock(tf_mutex_ as *mut pthread_mutex_t);
    if !(*tf_mt_sync).tf_mt_exit && (*tf_mt_sync).next_tf_row < mb_rows {
        *current_mb_row = (*tf_mt_sync).next_tf_row;
        (*tf_mt_sync).next_tf_row += 1;
        do_next_row = 1 as core::ffi::c_int;
    }
    pthread_mutex_unlock(tf_mutex_ as *mut pthread_mutex_t);
    return do_next_row;
}
unsafe extern "C-unwind" fn tf_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let mut cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
    let mut tf_ctx: *mut TemporalFilterCtx = &raw mut (*cpi).tf_ctx;
    let mut tf_sync: *mut AV1TemporalFilterSync = &raw mut (*cpi).mt_info.tf_sync;
    let mut scale: *const scale_factors = &raw mut (*cpi).tf_ctx.sf;
    let mut tf_mutex_: *mut pthread_mutex_t = (*tf_sync).mutex_;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let num_planes: core::ffi::c_int = av1_num_planes(&raw mut (*cpi).common)
        as core::ffi::c_int;
    if !(num_planes >= 1 as core::ffi::c_int && num_planes <= 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"tf_worker_hook\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2444 as core::ffi::c_int,
            b"num_planes >= 1 && num_planes <= MAX_MB_PLANE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut mbd: *mut MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    let mut input_buffer: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut input_mb_mode_info: *mut *mut MB_MODE_INFO = 0 as *mut *mut MB_MODE_INFO;
    tf_save_state(mbd, &raw mut input_mb_mode_info, input_buffer.as_mut_ptr(), num_planes);
    tf_setup_macroblockd(mbd, &raw mut (*td).tf_data, scale);
    let mut current_mb_row: core::ffi::c_int = -(1 as core::ffi::c_int);
    while tf_get_next_job(tf_sync, &raw mut current_mb_row, (*tf_ctx).mb_rows) != 0 {
        av1_tf_do_filtering_row(
            cpi as *mut AV1_COMP,
            td as *mut ThreadData,
            current_mb_row,
        );
    }
    tf_restore_state(mbd, input_mb_mode_info, input_buffer.as_mut_ptr(), num_planes);
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(tf_mutex_ as *mut pthread_mutex_t);
        (*tf_sync).tf_mt_exit = true_0 != 0;
        pthread_mutex_unlock(tf_mutex_ as *mut pthread_mutex_t);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn prepare_tf_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
    mut is_highbitdepth: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    (*mt_info).tf_sync.next_tf_row = 0 as core::ffi::c_int;
    (*mt_info).tf_sync.tf_mt_exit = false_0 != 0;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data).thread_id = i;
        (*thread_data).start = i;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
        }
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            (*(*thread_data).td).mb = (*cpi).td.mb;
            av1_init_obmc_buffer(&raw mut (*(*thread_data).td).mb.obmc_buffer);
            if !tf_alloc_and_reset_data(
                &raw mut (*(*thread_data).td).tf_data,
                (*cpi).tf_ctx.num_pels,
                is_highbitdepth,
            ) {
                aom_internal_error(
                    (*cpi).common.error,
                    AOM_CODEC_MEM_ERROR,
                    b"Error allocating temporal filter data\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            (*(*thread_data).td).mb.upsample_pred = (*(*thread_data).td).upsample_pred;
            (*(*thread_data).td).mb.e_mbd.tmp_upsample_pred = (*(*thread_data).td)
                .mb
                .upsample_pred;
        }
        i -= 1;
    }
}
unsafe extern "C-unwind" fn tf_dealloc_thread_data(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
    mut is_highbitdepth: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if td != &raw mut (*cpi).td as *mut ThreadData {
            tf_dealloc_data(&raw mut (*td).tf_data, is_highbitdepth);
        }
        i -= 1;
    }
}
unsafe extern "C-unwind" fn tf_accumulate_frame_diff(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    let mut total_diff: *mut FRAME_DIFF = &raw mut (*cpi).td.tf_data.diff;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*cpi).mt_info.workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = (*worker).data1 as *mut EncWorkerData;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        let mut diff: *mut FRAME_DIFF = &raw mut (*td).tf_data.diff;
        if td != &raw mut (*cpi).td as *mut ThreadData {
            (*total_diff).sse += (*diff).sse;
            (*total_diff).sum += (*diff).sum;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_tf_do_filtering_mt(mut cpi: *mut AV1_COMP) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let is_highbitdepth: core::ffi::c_int = (*cpi).tf_ctx.is_highbitdepth;
    let mut num_workers: core::ffi::c_int = if (*mt_info)
        .num_mod_workers[MOD_TF as core::ffi::c_int as usize] < (*mt_info).num_workers
    {
        (*mt_info).num_mod_workers[MOD_TF as core::ffi::c_int as usize]
    } else {
        (*mt_info).num_workers
    };
    prepare_tf_workers(
        cpi,
        Some(
            tf_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
        is_highbitdepth,
    );
    launch_workers(mt_info, num_workers);
    sync_enc_workers(mt_info, cm, num_workers);
    tf_accumulate_frame_diff(cpi, num_workers);
    tf_dealloc_thread_data(cpi, num_workers, is_highbitdepth);
}
#[inline]
unsafe extern "C-unwind" fn get_next_gm_job(
    mut cpi: *mut AV1_COMP,
    mut frame_idx: *mut core::ffi::c_int,
    mut cur_dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut gm_info: *mut GlobalMotionInfo = &raw mut (*cpi).gm_info;
    let mut job_info: *mut GlobalMotionJobInfo = &raw mut (*cpi).mt_info.gm_sync.job_info;
    let mut total_refs: core::ffi::c_int = (*gm_info).num_ref_frames[cur_dir as usize];
    let mut cur_frame_to_process: int8_t = (*job_info)
        .next_frame_to_process[cur_dir as usize];
    if (cur_frame_to_process as core::ffi::c_int) < total_refs
        && (*job_info).early_exit[cur_dir as usize] == 0
    {
        *frame_idx = (*gm_info)
            .reference_frames[cur_dir as usize][cur_frame_to_process as usize]
            .frame as core::ffi::c_int;
        (*job_info).next_frame_to_process[cur_dir as usize] = ((*job_info)
            .next_frame_to_process[cur_dir as usize] as core::ffi::c_int
            + 1 as core::ffi::c_int) as int8_t;
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn switch_direction(
    mut cpi: *mut AV1_COMP,
    mut frame_idx: *mut core::ffi::c_int,
    mut cur_dir: *mut core::ffi::c_int,
) {
    if (*cpi).sf.gm_sf.prune_ref_frame_for_gm_search != 0 {
        return;
    }
    *cur_dir = (*cur_dir == 0) as core::ffi::c_int;
    get_next_gm_job(cpi, frame_idx, *cur_dir);
}
unsafe extern "C-unwind" fn gm_mt_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let mut cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut gm_info: *mut GlobalMotionInfo = &raw mut (*cpi).gm_info;
    let mut gm_sync: *mut AV1GlobalMotionSync = &raw mut (*cpi).mt_info.gm_sync;
    let mut job_info: *mut GlobalMotionJobInfo = &raw mut (*gm_sync).job_info;
    let mut thread_id: core::ffi::c_int = (*thread_data).thread_id;
    let mut gm_thread_data: *mut GlobalMotionData = &raw mut (*(*thread_data).td).gm_data;
    let mut gm_mt_mutex_: *mut pthread_mutex_t = (*gm_sync).mutex_;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mut cur_dir: core::ffi::c_int = (*job_info).thread_id_to_dir[thread_id as usize]
        as core::ffi::c_int;
    let mut gm_mt_exit: bool = false_0 != 0;
    loop {
        let mut ref_buf_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
        pthread_mutex_lock(gm_mt_mutex_ as *mut pthread_mutex_t);
        gm_mt_exit = (*gm_sync).gm_mt_exit;
        if !gm_mt_exit && get_next_gm_job(cpi, &raw mut ref_buf_idx, cur_dir) == 0 {
            switch_direction(cpi, &raw mut ref_buf_idx, &raw mut cur_dir);
        }
        pthread_mutex_unlock(gm_mt_mutex_ as *mut pthread_mutex_t);
        if gm_mt_exit as core::ffi::c_int != 0 || ref_buf_idx == -(1 as core::ffi::c_int)
        {
            break;
        }
        av1_compute_gm_for_valid_ref_frames(
            cpi,
            error_info,
            ((*gm_info).ref_buf).as_mut_ptr(),
            ref_buf_idx,
            ((*gm_thread_data).motion_models).as_mut_ptr(),
            (*gm_thread_data).segment_map,
            (*gm_info).segment_map_w,
            (*gm_info).segment_map_h,
        );
        pthread_mutex_lock(gm_mt_mutex_ as *mut pthread_mutex_t);
        if (*cpi).sf.gm_sf.prune_ref_frame_for_gm_search != 0
            && (*cpi).common.global_motion[ref_buf_idx as usize].wmtype
                as core::ffi::c_int <= TRANSLATION as core::ffi::c_int
        {
            (*job_info).early_exit[cur_dir as usize] = 1 as int8_t;
        }
        pthread_mutex_unlock(gm_mt_mutex_ as *mut pthread_mutex_t);
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(gm_mt_mutex_ as *mut pthread_mutex_t);
        (*gm_sync).gm_mt_exit = true_0 != 0;
        pthread_mutex_unlock(gm_mt_mutex_ as *mut pthread_mutex_t);
        return 0 as core::ffi::c_int;}
}
}}
#[inline]
unsafe extern "C-unwind" fn prepare_gm_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    (*mt_info).gm_sync.gm_mt_exit = false_0 != 0;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data).thread_id = i;
        (*thread_data).start = i;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
        }
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            gm_alloc_data(cpi, &raw mut (*(*thread_data).td).gm_data);
        }
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn assign_thread_to_dir(
    mut thread_id_to_dir: *mut int8_t,
    mut num_workers: core::ffi::c_int,
) {
    let mut frame_dir_idx: int8_t = 0 as int8_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_workers {
        let fresh4 = frame_dir_idx;
        frame_dir_idx = frame_dir_idx + 1;
        *thread_id_to_dir.offset(i as isize) = fresh4;
        if frame_dir_idx as core::ffi::c_int == MAX_DIRECTIONS {
            frame_dir_idx = 0 as int8_t;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn compute_gm_workers(mut cpi: *const AV1_COMP) -> core::ffi::c_int {
    let mut total_refs: core::ffi::c_int = (*cpi)
        .gm_info
        .num_ref_frames[0 as core::ffi::c_int as usize]
        + (*cpi).gm_info.num_ref_frames[1 as core::ffi::c_int as usize];
    let mut num_gm_workers: core::ffi::c_int = if (*cpi)
        .sf
        .gm_sf
        .prune_ref_frame_for_gm_search != 0
    {
        if (2 as core::ffi::c_int) < total_refs {
            2 as core::ffi::c_int
        } else {
            total_refs
        }
    } else {
        total_refs
    };
    num_gm_workers = if num_gm_workers < (*cpi).mt_info.num_workers {
        num_gm_workers
    } else {
        (*cpi).mt_info.num_workers
    };
    return num_gm_workers;
}
#[inline]
unsafe extern "C-unwind" fn gm_dealloc_thread_data(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < num_workers {
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(j as isize) as *mut EncWorkerData;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if td != &raw mut (*cpi).td as *mut ThreadData {
            gm_dealloc_data(&raw mut (*td).gm_data);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_global_motion_estimation_mt(mut cpi: *mut AV1_COMP) {
    let mut job_info: *mut GlobalMotionJobInfo = &raw mut (*cpi).mt_info.gm_sync.job_info;
    memset(
        job_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<GlobalMotionJobInfo>() as size_t,
    );
    let mut num_workers: core::ffi::c_int = compute_gm_workers(cpi);
    assign_thread_to_dir(((*job_info).thread_id_to_dir).as_mut_ptr(), num_workers);
    prepare_gm_workers(
        cpi,
        Some(
            gm_mt_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(&raw mut (*cpi).mt_info, num_workers);
    sync_enc_workers(&raw mut (*cpi).mt_info, &raw mut (*cpi).common, num_workers);
    gm_dealloc_thread_data(cpi, num_workers);
}
#[inline]
unsafe extern "C-unwind" fn get_next_job_allintra(
    row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mi_row_end: core::ffi::c_int,
    mut current_mi_row: *mut core::ffi::c_int,
    mut mib_size: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*row_mt_sync).next_mi_row < mi_row_end {
        *current_mi_row = (*row_mt_sync).next_mi_row;
        (*row_mt_sync).num_threads_working += 1;
        (*row_mt_sync).next_mi_row += mib_size;
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn prepare_wiener_var_workers(
    cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    num_workers: core::ffi::c_int,
) {
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        (*thread_data).thread_id = i;
        (*thread_data).start = 0 as core::ffi::c_int;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
        }
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            (*(*thread_data).td).mb = (*cpi).td.mb;
            av1_alloc_mb_wiener_var_pred_buf(
                &raw mut (*cpi).common,
                (*thread_data).td as *mut ThreadData,
            );
        }
        i -= 1;
    }
}
unsafe extern "C-unwind" fn set_mb_wiener_var_calc_done(cpi: *mut AV1_COMP) {
    let mi_params: *const CommonModeInfoParams = &raw mut (*cpi).common.mi_params;
    let bsize: BLOCK_SIZE = (*cpi).weber_bsize;
    let mb_step: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    if !((BLOCK_64X64 as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"set_mb_wiener_var_calc_done\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2782 as core::ffi::c_int,
            b"MB_WIENER_MT_UNIT_SIZE < BLOCK_SIZES_ALL\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mt_unit_step: core::ffi::c_int = mi_size_wide[BLOCK_64X64 as core::ffi::c_int
        as usize] as core::ffi::c_int;
    let mt_unit_cols: core::ffi::c_int = ((*mi_params).mi_cols
        + (mt_unit_step >> 1 as core::ffi::c_int)) / mt_unit_step;
    let intra_mt: *const AV1EncAllIntraMultiThreadInfo = &raw mut (*cpi).mt_info.intra_mt;
    let intra_row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*(*cpi).ppi)
        .intra_row_mt_sync;
    let mut mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mt_thread_id: core::ffi::c_int = 0 as core::ffi::c_int;
    while mi_row < (*mi_params).mi_rows {
        ((*intra_mt).intra_sync_write_ptr)
            .expect(
                "non-null function pointer",
            )(
            intra_row_mt_sync,
            mt_thread_id,
            mt_unit_cols - 1 as core::ffi::c_int,
            mt_unit_cols,
        );
        mi_row += mb_step;
        mt_thread_id += 1;
    }
}
unsafe extern "C-unwind" fn cal_mb_wiener_var_hook(
    mut arg1: *mut core::ffi::c_void,
    mut unused: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let mut x: *mut MACROBLOCK = &raw mut (*(*thread_data).td).mb;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let bsize: BLOCK_SIZE = (*cpi).weber_bsize;
    let mb_step: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let intra_row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*(*cpi).ppi)
        .intra_row_mt_sync;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*cpi).mt_info.enc_row_mt;
    let mut enc_row_mt_mutex: *mut pthread_mutex_t = (*enc_row_mt).mutex_;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mut src_diff: crate::platform::Aligned32<[int16_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut coeff: crate::platform::Aligned32<[tran_low_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut qcoeff: crate::platform::Aligned32<[tran_low_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut dqcoeff: crate::platform::Aligned32<[tran_low_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut sum_rec_distortion: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut sum_est_rate: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    loop {
        let mut current_mi_row: core::ffi::c_int = -(1 as core::ffi::c_int);
        pthread_mutex_lock(enc_row_mt_mutex as *mut pthread_mutex_t);
        let mut has_jobs: core::ffi::c_int = if (*enc_row_mt).mb_wiener_mt_exit
            as core::ffi::c_int != 0
        {
            0 as core::ffi::c_int
        } else {
            get_next_job_allintra(
                intra_row_mt_sync,
                (*cpi).common.mi_params.mi_rows,
                &raw mut current_mi_row,
                mb_step,
            )
        };
        pthread_mutex_unlock(enc_row_mt_mutex as *mut pthread_mutex_t);
        if has_jobs == 0 {
            break;
        }
        av1_calc_mb_wiener_var_row(
            cpi,
            x,
            xd,
            current_mi_row,
            src_diff.0.as_mut_ptr(),
            coeff.0.as_mut_ptr(),
            qcoeff.0.as_mut_ptr(),
            dqcoeff.0.as_mut_ptr(),
            &raw mut sum_rec_distortion,
            &raw mut sum_est_rate,
            (*(*thread_data).td).wiener_tmp_pred_buf,
        );
        pthread_mutex_lock(enc_row_mt_mutex as *mut pthread_mutex_t);
        (*intra_row_mt_sync).num_threads_working -= 1;
        pthread_mutex_unlock(enc_row_mt_mutex as *mut pthread_mutex_t);
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(enc_row_mt_mutex as *mut pthread_mutex_t);
        (*enc_row_mt).mb_wiener_mt_exit = true_0 != 0;
        pthread_mutex_unlock(enc_row_mt_mutex as *mut pthread_mutex_t);
        set_mb_wiener_var_calc_done(cpi);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn dealloc_mb_wiener_var_mt_data(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
) {
    av1_row_mt_sync_mem_dealloc(&raw mut (*(*cpi).ppi).intra_row_mt_sync);
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < num_workers {
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(j as isize) as *mut EncWorkerData;
        let mut td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
        if td != &raw mut (*cpi).td as *mut ThreadData {
            av1_dealloc_mb_wiener_var_pred_buf(td);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_mb_wiener_var_mt(
    mut cpi: *mut AV1_COMP,
    mut num_workers: core::ffi::c_int,
    mut sum_rec_distortion: *mut core::ffi::c_double,
    mut sum_est_rate: *mut core::ffi::c_double,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let intra_row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*(*cpi).ppi)
        .intra_row_mt_sync;
    let mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    row_mt_sync_mem_alloc(intra_row_mt_sync, cm, mi_rows);
    (*intra_row_mt_sync).intrabc_extra_top_right_sb_delay = 0 as core::ffi::c_int;
    (*intra_row_mt_sync).num_threads_working = num_workers;
    (*intra_row_mt_sync).next_mi_row = 0 as core::ffi::c_int;
    memset(
        (*intra_row_mt_sync).num_finished_cols as *mut core::ffi::c_void,
        -(1 as core::ffi::c_int),
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(mi_rows as size_t),
    );
    (*mt_info).enc_row_mt.mb_wiener_mt_exit = false_0 != 0;
    prepare_wiener_var_workers(
        cpi,
        Some(
            cal_mb_wiener_var_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(mt_info, num_workers);
    sync_enc_workers(mt_info, cm, num_workers);
    dealloc_mb_wiener_var_mt_data(cpi, num_workers);
}
unsafe extern "C-unwind" fn compare_tile_order(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let tile_a: *const PackBSTileOrder = a as *const PackBSTileOrder;
    let tile_b: *const PackBSTileOrder = b as *const PackBSTileOrder;
    if (*tile_a).abs_sum_level > (*tile_b).abs_sum_level {
        return -(1 as core::ffi::c_int)
    } else if (*tile_a).abs_sum_level == (*tile_b).abs_sum_level {
        return if (*tile_a).tile_idx as core::ffi::c_int
            > (*tile_b).tile_idx as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            -(1 as core::ffi::c_int)
        }
    } else {
        return 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn get_next_pack_bs_tile_idx(
    pack_bs_sync: *mut AV1EncPackBSSync,
    num_tiles: core::ffi::c_int,
) -> core::ffi::c_int {
    if !((*pack_bs_sync).next_job_idx <= num_tiles) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"get_next_pack_bs_tile_idx\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2930 as core::ffi::c_int,
            b"pack_bs_sync->next_job_idx <= num_tiles\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*pack_bs_sync).next_job_idx == num_tiles {
        return -(1 as core::ffi::c_int);
    }
    let fresh7 = (*pack_bs_sync).next_job_idx;
    (*pack_bs_sync).next_job_idx = (*pack_bs_sync).next_job_idx + 1;
    return (*pack_bs_sync).pack_bs_tile_order[fresh7 as usize].tile_idx
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_bs_chunk_size(
    mut tg_or_tile_size: core::ffi::c_int,
    frame_or_tg_size: core::ffi::c_int,
    mut remain_buf_size: *mut size_t,
    mut max_buf_size: size_t,
    mut is_last_chunk: core::ffi::c_int,
) -> size_t {
    let mut this_chunk_size: size_t = 0;
    if !(*remain_buf_size > 0 as size_t) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_bs_chunk_size\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2944 as core::ffi::c_int,
            b"*remain_buf_size > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if is_last_chunk != 0 {
        this_chunk_size = *remain_buf_size;
        *remain_buf_size = 0 as size_t;
    } else {
        let size_scale: uint64_t = (max_buf_size as uint64_t)
            .wrapping_mul(tg_or_tile_size as uint64_t);
        this_chunk_size = size_scale.wrapping_div(frame_or_tg_size as uint64_t)
            as size_t;
        *remain_buf_size = (*remain_buf_size).wrapping_sub(this_chunk_size);
        if !(*remain_buf_size > 0 as size_t) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"get_bs_chunk_size\0"))
                    .as_ptr(),
                b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                2952 as core::ffi::c_int,
                b"*remain_buf_size > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if !(this_chunk_size > 0 as size_t) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_bs_chunk_size\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            2954 as core::ffi::c_int,
            b"this_chunk_size > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return this_chunk_size;
}
unsafe extern "C-unwind" fn init_tile_pack_bs_params(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    pack_bs_params_arr: *mut PackBSParams,
    mut obu_extn_header: uint8_t,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*cpi).td.mb.e_mbd;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let num_tiles: core::ffi::c_int = (*tiles).cols * (*tiles).rows;
    let num_tg_hdrs: core::ffi::c_int = (*cpi).num_tg;
    let tg_size_in_tiles: core::ffi::c_int = (num_tiles + num_tg_hdrs
        - 1 as core::ffi::c_int) / num_tg_hdrs;
    let mut tile_dst: *mut uint8_t = dst;
    let mut tile_data_curr: *mut uint8_t = dst;
    let mut tg_size_mi: [core::ffi::c_int; 512] = [0 as core::ffi::c_int; 512];
    let mut tile_idx: core::ffi::c_int = 0;
    let mut tg_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_count_in_tg: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut new_tg: core::ffi::c_int = 1 as core::ffi::c_int;
    tile_idx = 0 as core::ffi::c_int;
    while tile_idx < num_tiles {
        let tile_info: *const TileInfo = &raw mut (*((*cpi).tile_data)
            .offset(tile_idx as isize))
            .tile_info;
        let pack_bs_params: *mut PackBSParams = &raw mut *pack_bs_params_arr
            .offset(tile_idx as isize) as *mut PackBSParams;
        let tile_size_mi: core::ffi::c_int = ((*tile_info).mi_col_end
            - (*tile_info).mi_col_start)
            * ((*tile_info).mi_row_end - (*tile_info).mi_row_start);
        let mut is_last_tile_in_tg: core::ffi::c_int = 0 as core::ffi::c_int;
        tile_count_in_tg += 1;
        if tile_count_in_tg == tg_size_in_tiles
            || tile_idx == num_tiles - 1 as core::ffi::c_int
        {
            is_last_tile_in_tg = 1 as core::ffi::c_int;
        }
        (*pack_bs_params).curr_tg_hdr_size = 0 as core::ffi::c_int;
        (*pack_bs_params).obu_extn_header = obu_extn_header;
        (*pack_bs_params).saved_wb = saved_wb;
        (*pack_bs_params).obu_header_size = 0 as uint32_t;
        (*pack_bs_params).is_last_tile_in_tg = is_last_tile_in_tg;
        (*pack_bs_params).new_tg = new_tg;
        (*pack_bs_params).tile_col = (*tile_info).tile_col;
        (*pack_bs_params).tile_row = (*tile_info).tile_row;
        (*pack_bs_params).tile_size_mi = tile_size_mi;
        tg_size_mi[tg_idx as usize] += tile_size_mi;
        if new_tg != 0 {
            new_tg = 0 as core::ffi::c_int;
        }
        if is_last_tile_in_tg != 0 {
            tile_count_in_tg = 0 as core::ffi::c_int;
            new_tg = 1 as core::ffi::c_int;
            tg_idx += 1;
        }
        tile_idx += 1;
    }
    if !(dst_size > 0 as size_t) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"init_tile_pack_bs_params\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            3013 as core::ffi::c_int,
            b"dst_size > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut tg_buf_size: [size_t; 512] = [
        0 as core::ffi::c_int as size_t,
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
        0,
    ];
    let mut max_buf_size: size_t = dst_size;
    let mut remain_buf_size: size_t = max_buf_size;
    let frame_size_mi: core::ffi::c_int = (*cm).mi_params.mi_rows
        * (*cm).mi_params.mi_cols;
    tile_idx = 0 as core::ffi::c_int;
    tg_idx = 0 as core::ffi::c_int;
    while tg_idx < (*cpi).num_tg {
        let pack_bs_params_0: *mut PackBSParams = &raw mut *pack_bs_params_arr
            .offset(tile_idx as isize) as *mut PackBSParams;
        let mut is_last_tg: core::ffi::c_int = (tg_idx
            == (*cpi).num_tg - 1 as core::ffi::c_int) as core::ffi::c_int;
        tg_buf_size[tg_idx as usize] = get_bs_chunk_size(
            tg_size_mi[tg_idx as usize],
            frame_size_mi,
            &raw mut remain_buf_size,
            max_buf_size,
            is_last_tg,
        );
        (*pack_bs_params_0).dst = tile_dst;
        (*pack_bs_params_0).tile_data_curr = tile_dst;
        av1_write_obu_tg_tile_headers(
            cpi as *mut AV1_COMP,
            xd,
            pack_bs_params_0,
            tile_idx,
        );
        tile_dst = tile_dst.offset(tg_buf_size[tg_idx as usize] as isize);
        tg_buf_size[tg_idx as usize] = (tg_buf_size[tg_idx as usize])
            .wrapping_sub((*pack_bs_params_0).curr_tg_hdr_size as size_t);
        tile_idx += tg_size_in_tiles;
        tg_idx += 1;
    }
    tg_idx = 0 as core::ffi::c_int;
    tile_idx = 0 as core::ffi::c_int;
    while tile_idx < num_tiles {
        let pack_bs_params_1: *mut PackBSParams = &raw mut *pack_bs_params_arr
            .offset(tile_idx as isize) as *mut PackBSParams;
        if (*pack_bs_params_1).new_tg != 0 {
            max_buf_size = tg_buf_size[tg_idx as usize];
            remain_buf_size = max_buf_size;
        }
        let tile_buf_size: size_t = get_bs_chunk_size(
            (*pack_bs_params_1).tile_size_mi,
            tg_size_mi[tg_idx as usize],
            &raw mut remain_buf_size,
            max_buf_size,
            (*pack_bs_params_1).is_last_tile_in_tg,
        ) as size_t;
        (*pack_bs_params_1).tile_buf_size = tile_buf_size;
        if (*pack_bs_params_1).new_tg != 0 {
            tile_dst = (*pack_bs_params_1).dst;
            tile_data_curr = (*pack_bs_params_1).tile_data_curr;
            (*pack_bs_params_1).tile_buf_size = ((*pack_bs_params_1).tile_buf_size)
                .wrapping_add((*pack_bs_params_1).curr_tg_hdr_size as size_t);
        } else {
            (*pack_bs_params_1).dst = tile_dst;
            (*pack_bs_params_1).tile_data_curr = tile_data_curr;
        }
        if (*pack_bs_params_1).is_last_tile_in_tg != 0 {
            tg_idx += 1;
        }
        tile_dst = tile_dst.offset((*pack_bs_params_1).tile_buf_size as isize);
        tile_idx += 1;
    }
}
unsafe extern "C-unwind" fn pack_bs_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let pack_bs_params: *mut PackBSParams = arg2 as *mut PackBSParams;
    let cpi: *mut AV1_COMP = (*thread_data).cpi as *mut AV1_COMP;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let pack_bs_sync: *mut AV1EncPackBSSync = &raw mut (*cpi).mt_info.pack_bs_sync;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let num_tiles: core::ffi::c_int = (*tiles).cols * (*tiles).rows;
    let pack_bs_mutex: *mut pthread_mutex_t = (*pack_bs_sync).mutex_;
    let xd: *mut MACROBLOCKD = &raw mut (*(*thread_data).td).mb.e_mbd;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    (*xd).error_info = error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    loop {
        pthread_mutex_lock(pack_bs_mutex as *mut pthread_mutex_t);
        let tile_idx: core::ffi::c_int = if (*pack_bs_sync).pack_bs_mt_exit
            as core::ffi::c_int != 0
        {
            -(1 as core::ffi::c_int)
        } else {
            get_next_pack_bs_tile_idx(pack_bs_sync, num_tiles) as core::ffi::c_int
        };
        pthread_mutex_unlock(pack_bs_mutex as *mut pthread_mutex_t);
        if tile_idx == -(1 as core::ffi::c_int) {
            break;
        }
        let mut this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
            .offset(tile_idx as isize) as *mut TileDataEnc;
        (*(*thread_data).td).mb.e_mbd.tile_ctx = &raw mut (*this_tile).tctx;
        av1_pack_tile_info(
            cpi as *mut AV1_COMP,
            (*thread_data).td,
            &raw mut *pack_bs_params.offset(tile_idx as isize),
        );
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(pack_bs_mutex as *mut pthread_mutex_t);
        (*pack_bs_sync).pack_bs_mt_exit = true_0 != 0;
        pthread_mutex_unlock(pack_bs_mutex as *mut pthread_mutex_t);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn prepare_pack_bs_workers(
    cpi: *mut AV1_COMP,
    pack_bs_params: *mut PackBSParams,
    mut hook: AVxWorkerHook,
    num_workers: core::ffi::c_int,
) {
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        if i == 0 as core::ffi::c_int {
            (*thread_data).td = &raw mut (*cpi).td as *mut ThreadData;
        } else {
            (*thread_data).td = (*thread_data).original_td;
        }
        if (*thread_data).td != &raw mut (*cpi).td as *mut ThreadData {
            (*(*thread_data).td).mb = (*cpi).td.mb;
        }
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        (*thread_data).start = i;
        (*thread_data).thread_id = i;
        av1_reset_pack_bs_thread_data((*thread_data).td);
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = pack_bs_params as *mut core::ffi::c_void;
        i -= 1;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let pack_bs_sync: *mut AV1EncPackBSSync = &raw mut (*mt_info).pack_bs_sync;
    let num_tiles: uint16_t = ((*cm).tiles.rows * (*cm).tiles.cols) as uint16_t;
    (*pack_bs_sync).next_job_idx = 0 as core::ffi::c_int;
    (*pack_bs_sync).pack_bs_mt_exit = false_0 != 0;
    let pack_bs_tile_order: *mut PackBSTileOrder = ((*pack_bs_sync).pack_bs_tile_order)
        .as_mut_ptr();
    memset(
        pack_bs_tile_order as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (num_tiles as size_t)
            .wrapping_mul(::core::mem::size_of::<PackBSTileOrder>() as size_t),
    );
    let mut tile_idx: uint16_t = 0 as uint16_t;
    while (tile_idx as core::ffi::c_int) < num_tiles as core::ffi::c_int {
        (*pack_bs_tile_order.offset(tile_idx as isize)).abs_sum_level = (*((*cpi)
            .tile_data)
            .offset(tile_idx as isize))
            .abs_sum_level;
        (*pack_bs_tile_order.offset(tile_idx as isize)).tile_idx = tile_idx;
        tile_idx = tile_idx.wrapping_add(1);
    }
    qsort(
        pack_bs_tile_order as *mut core::ffi::c_void,
        num_tiles as size_t,
        ::core::mem::size_of::<PackBSTileOrder>() as size_t,
        Some(
            compare_tile_order
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
}
unsafe extern "C-unwind" fn accumulate_pack_bs_data(
    cpi: *mut AV1_COMP,
    pack_bs_params_arr: *const PackBSParams,
    dst: *mut uint8_t,
    mut total_size: *mut uint32_t,
    mut fh_info: *const FrameHeaderInfo,
    largest_tile_id: *mut core::ffi::c_int,
    mut max_tile_size: *mut core::ffi::c_uint,
    obu_header_size: *mut uint32_t,
    mut tile_data_start: *mut *mut uint8_t,
    num_workers: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &(*cm).tiles;
    let tile_count: core::ffi::c_int = (*tiles).cols * (*tiles).rows;
    let mut curr_tg_data_size: size_t = 0 as size_t;
    let mut is_first_tg: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut curr_tg_start: *mut uint8_t = dst;
    let mut src_offset: size_t = 0 as size_t;
    let mut dst_offset: size_t = 0 as size_t;
    let mut tile_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_idx < tile_count {
        let pack_bs_params: *const PackBSParams = &*pack_bs_params_arr
            .offset(tile_idx as isize) as *const PackBSParams;
        let mut tile_size: uint32_t = 0 as uint32_t;
        if (*pack_bs_params).new_tg != 0 {
            curr_tg_start = dst.offset(*total_size as isize);
            curr_tg_data_size = (*pack_bs_params).curr_tg_hdr_size as size_t;
            *tile_data_start = (*tile_data_start)
                .offset((*pack_bs_params).curr_tg_hdr_size as isize);
            *obu_header_size = (*pack_bs_params).obu_header_size;
        }
        curr_tg_data_size = curr_tg_data_size
            .wrapping_add(
                ((*pack_bs_params).buf.size)
                    .wrapping_add(
                        (if (*pack_bs_params).is_last_tile_in_tg != 0 {
                            0 as core::ffi::c_int
                        } else {
                            4 as core::ffi::c_int
                        }) as size_t,
                    ),
            );
        if (*pack_bs_params).buf.size > *max_tile_size as size_t {
            *largest_tile_id = tile_idx;
            *max_tile_size = (*pack_bs_params).buf.size as core::ffi::c_uint;
        }
        tile_size = tile_size
            .wrapping_add(
                ((*pack_bs_params).buf.size as uint32_t)
                    .wrapping_add(*(*pack_bs_params).total_size),
            );
        if tile_idx != 0 as core::ffi::c_int {
            memmove(
                dst.offset(dst_offset as isize) as *mut core::ffi::c_void,
                dst.offset(src_offset as isize) as *const core::ffi::c_void,
                tile_size as size_t,
            );
        }
        if (*pack_bs_params).is_last_tile_in_tg != 0 {
            av1_write_last_tile_info(
                cpi as *mut AV1_COMP,
                fh_info,
                (*pack_bs_params).saved_wb,
                &raw mut curr_tg_data_size,
                curr_tg_start,
                &raw mut tile_size,
                tile_data_start,
                largest_tile_id,
                &raw mut is_first_tg,
                *obu_header_size,
                (*pack_bs_params).obu_extn_header,
            );
        }
        src_offset = src_offset.wrapping_add((*pack_bs_params).tile_buf_size);
        dst_offset = dst_offset.wrapping_add(tile_size as size_t);
        *total_size = (*total_size).wrapping_add(tile_size);
        tile_idx += 1;
    }
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut idx: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while idx >= 0 as core::ffi::c_int {
        let mut td: *const ThreadData = (*((*mt_info).tile_thr_data)
            .offset(idx as isize))
            .td;
        av1_accumulate_pack_bs_thread_data(
            cpi as *mut AV1_COMP,
            td as *const ThreadData,
        );
        idx -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_tile_obu_mt(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut total_size: *mut uint32_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut obu_extn_header: uint8_t,
    mut fh_info: *const FrameHeaderInfo,
    largest_tile_id: *mut core::ffi::c_int,
    mut max_tile_size: *mut core::ffi::c_uint,
    obu_header_size: *mut uint32_t,
    mut tile_data_start: *mut *mut uint8_t,
    num_workers: core::ffi::c_int,
) {
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut pack_bs_params: [PackBSParams; 512] = [PackBSParams {
        saved_wb: 0 as *mut aom_write_bit_buffer,
        buf: TileBufferEnc {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        total_size: 0 as *mut uint32_t,
        dst: 0 as *mut uint8_t,
        tile_data_curr: 0 as *mut uint8_t,
        tile_buf_size: 0,
        obu_extn_header: 0,
        obu_header_size: 0,
        curr_tg_hdr_size: 0,
        tile_size_mi: 0,
        tile_row: 0,
        tile_col: 0,
        is_last_tile_in_tg: 0,
        new_tg: 0,
    }; 512];
    let mut tile_size: [uint32_t; 512] = [
        0 as core::ffi::c_int as uint32_t,
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
        0,
    ];
    let mut tile_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_idx < MAX_TILES {
        pack_bs_params[tile_idx as usize].total_size = &raw mut *tile_size
            .as_mut_ptr()
            .offset(tile_idx as isize) as *mut uint32_t;
        tile_idx += 1;
    }
    init_tile_pack_bs_params(
        cpi,
        dst,
        dst_size,
        saved_wb,
        pack_bs_params.as_mut_ptr(),
        obu_extn_header,
    );
    prepare_pack_bs_workers(
        cpi,
        pack_bs_params.as_mut_ptr(),
        Some(
            pack_bs_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(mt_info, num_workers);
    sync_enc_workers(mt_info, &raw mut (*cpi).common, num_workers);
    accumulate_pack_bs_data(
        cpi,
        pack_bs_params.as_mut_ptr(),
        dst,
        total_size,
        fh_info,
        largest_tile_id,
        max_tile_size,
        obu_header_size,
        tile_data_start,
        num_workers,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_mt_dealloc(mut cdef_sync: *mut AV1CdefSync) {
    if cdef_sync.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_cdef_mt_dealloc\0"))
                .as_ptr(),
            b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
            3270 as core::ffi::c_int,
            b"cdef_sync != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*cdef_sync).mutex_).is_null() {
        pthread_mutex_destroy((*cdef_sync).mutex_ as *mut pthread_mutex_t);
        aom_free((*cdef_sync).mutex_ as *mut core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn update_next_job_info(
    mut cdef_sync: *mut AV1CdefSync,
    mut nvfb: core::ffi::c_int,
    mut nhfb: core::ffi::c_int,
) {
    (*cdef_sync).fbc += 1;
    if (*cdef_sync).fbc == nhfb {
        (*cdef_sync).fbr += 1;
        if (*cdef_sync).fbr == nvfb {
            (*cdef_sync).end_of_frame = 1 as core::ffi::c_int;
        } else {
            (*cdef_sync).fbc = 0 as core::ffi::c_int;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn cdef_reset_job_info(mut cdef_sync: *mut AV1CdefSync) {
    if !((*cdef_sync).mutex_).is_null() {
        pthread_mutex_init(
            (*cdef_sync).mutex_ as *mut pthread_mutex_t,
            0 as *const pthread_mutexattr_t,
        );
    }
    (*cdef_sync).end_of_frame = 0 as core::ffi::c_int;
    (*cdef_sync).fbr = 0 as core::ffi::c_int;
    (*cdef_sync).fbc = 0 as core::ffi::c_int;
    (*cdef_sync).cdef_mt_exit = false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn cdef_get_next_job(
    mut cdef_sync: *mut AV1CdefSync,
    mut cdef_search_ctx: *mut CdefSearchCtx,
    mut cur_fbr: *mut core::ffi::c_int,
    mut cur_fbc: *mut core::ffi::c_int,
    mut sb_count: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    pthread_mutex_lock((*cdef_sync).mutex_ as *mut pthread_mutex_t);
    let mut do_next_block: core::ffi::c_int = 0 as core::ffi::c_int;
    let nvfb: core::ffi::c_int = (*cdef_search_ctx).nvfb;
    let nhfb: core::ffi::c_int = (*cdef_search_ctx).nhfb;
    while !(*cdef_sync).cdef_mt_exit && (*cdef_sync).end_of_frame == 0
        && cdef_sb_skip((*cdef_search_ctx).mi_params, (*cdef_sync).fbr, (*cdef_sync).fbc)
            != 0
    {
        update_next_job_info(cdef_sync, nvfb, nhfb);
    }
    if !(*cdef_sync).cdef_mt_exit && (*cdef_sync).end_of_frame == 0 as core::ffi::c_int {
        do_next_block = 1 as core::ffi::c_int;
        ::core::ptr::write_volatile(cur_fbr, (*cdef_sync).fbr);
        ::core::ptr::write_volatile(cur_fbc, (*cdef_sync).fbc);
        ::core::ptr::write_volatile(sb_count, (*cdef_search_ctx).sb_count);
        (*cdef_search_ctx).sb_count += 1;
        update_next_job_info(cdef_sync, nvfb, nhfb);
    }
    pthread_mutex_unlock((*cdef_sync).mutex_ as *mut pthread_mutex_t);
    return do_next_block;
}
unsafe extern "C-unwind" fn cdef_filter_block_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut thread_data: *mut EncWorkerData = arg1 as *mut EncWorkerData;
    let cdef_sync: *mut AV1CdefSync = arg2 as *mut AV1CdefSync;
    let mut cdef_mutex_: *mut pthread_mutex_t = (*cdef_sync).mutex_;
    let error_info: *mut aom_internal_error_info = &raw mut (*thread_data).error_info;
    let mut cdef_search_ctx: *mut CdefSearchCtx = (*(*thread_data).cpi).cdef_search_ctx;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mut cur_fbr: core::ffi::c_int = 0;
    let mut cur_fbc: core::ffi::c_int = 0;
    let mut sb_count: core::ffi::c_int = 0;
    while cdef_get_next_job(
        cdef_sync,
        cdef_search_ctx,
        &raw mut cur_fbr,
        &raw mut cur_fbc,
        &raw mut sb_count,
    ) != 0
    {
        av1_cdef_mse_calc_block(
            cdef_search_ctx,
            error_info,
            cur_fbr,
            cur_fbc,
            sb_count,
            (*(*thread_data).cpi).sf.lpf_sf.adaptive_cdef_mode,
        );
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(cdef_mutex_ as *mut pthread_mutex_t);
        (*cdef_sync).cdef_mt_exit = true_0 != 0;
        pthread_mutex_unlock(cdef_mutex_ as *mut pthread_mutex_t);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn prepare_cdef_workers(
    mut cpi: *mut AV1_COMP,
    mut hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let mut worker: *mut AVxWorker = &raw mut *((*mt_info).workers).offset(i as isize)
            as *mut AVxWorker;
        let mut thread_data: *mut EncWorkerData = &raw mut *((*mt_info).tile_thr_data)
            .offset(i as isize) as *mut EncWorkerData;
        (*thread_data).cpi = cpi as *mut AV1_COMP;
        (*worker).hook = hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = &raw mut (*mt_info).cdef_sync as *mut AV1CdefSync
            as *mut core::ffi::c_void;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_mse_calc_frame_mt(mut cpi: *mut AV1_COMP) {
    let mut mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let mut cdef_sync: *mut AV1CdefSync = &raw mut (*mt_info).cdef_sync;
    let num_workers: core::ffi::c_int = (*mt_info)
        .num_mod_workers[MOD_CDEF_SEARCH as core::ffi::c_int as usize];
    cdef_reset_job_info(cdef_sync);
    prepare_cdef_workers(
        cpi,
        Some(
            cdef_filter_block_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_workers(mt_info, num_workers);
    sync_enc_workers(mt_info, &raw mut (*cpi).common, num_workers);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_tf_workers(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    if ((*cpi).oxcf.pass as core::ffi::c_uint)
        < AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
    {
        return compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
    }
    if (*cpi).oxcf.max_threads <= 1 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    let frame_height: core::ffi::c_int = (*cpi).common.height;
    let block_size: BLOCK_SIZE = BLOCK_64X64;
    let mb_height: core::ffi::c_int = block_size_high[block_size as usize]
        as core::ffi::c_int;
    let mb_rows: core::ffi::c_int = get_num_blocks(frame_height, mb_height)
        as core::ffi::c_int;
    return if (*cpi).oxcf.max_threads < mb_rows {
        (*cpi).oxcf.max_threads
    } else {
        mb_rows
    };
}
#[inline]
unsafe extern "C-unwind" fn compute_num_tpl_workers(
    mut cpi: *mut AV1_COMP,
) -> core::ffi::c_int {
    return compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_lf_workers(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    return compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_cdef_workers(
    mut cpi: *mut AV1_COMP,
) -> core::ffi::c_int {
    return compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_lr_workers(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    return compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_pack_bs_workers(
    mut cpi: *mut AV1_COMP,
) -> core::ffi::c_int {
    if (*cpi).oxcf.max_threads <= 1 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    return compute_num_enc_tile_mt_workers(&raw mut (*cpi).common, (*cpi).oxcf.max_threads);
}
#[inline]
unsafe extern "C-unwind" fn compute_num_ai_workers(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    if (*cpi).oxcf.max_threads <= 1 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    if !(*cpi).oxcf.row_mt {
        return 1 as core::ffi::c_int;
    }
    (*cpi).weber_bsize = BLOCK_8X8;
    let bsize: BLOCK_SIZE = (*cpi).weber_bsize;
    let mb_step: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let num_mb_rows: core::ffi::c_int = (*cpi).common.mi_params.mi_rows / mb_step;
    return if num_mb_rows < (*cpi).oxcf.max_threads {
        num_mb_rows
    } else {
        (*cpi).oxcf.max_threads
    };
}
unsafe extern "C-unwind" fn compute_num_mod_workers(
    mut cpi: *mut AV1_COMP,
    mut mod_name: MULTI_THREADED_MODULES,
) -> core::ffi::c_int {
    let mut num_mod_workers: core::ffi::c_int = 0 as core::ffi::c_int;
    match mod_name as core::ffi::c_uint {
        0 => {
            if (*cpi).oxcf.pass as core::ffi::c_uint
                >= AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint
            {
                num_mod_workers = 0 as core::ffi::c_int;
            } else {
                num_mod_workers = compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
            }
        }
        1 => {
            num_mod_workers = compute_num_tf_workers(cpi);
        }
        2 => {
            num_mod_workers = compute_num_tpl_workers(cpi);
        }
        3 => {
            num_mod_workers = 1 as core::ffi::c_int;
        }
        4 => {
            num_mod_workers = compute_num_enc_workers(cpi, (*cpi).oxcf.max_threads);
        }
        5 => {
            num_mod_workers = compute_num_lf_workers(cpi);
        }
        6 => {
            num_mod_workers = compute_num_cdef_workers(cpi);
        }
        7 => {
            num_mod_workers = compute_num_cdef_workers(cpi);
        }
        8 => {
            num_mod_workers = compute_num_lr_workers(cpi);
        }
        9 => {
            num_mod_workers = compute_num_pack_bs_workers(cpi);
        }
        10 => {
            num_mod_workers = (*(*cpi).ppi)
                .p_mt_info
                .num_mod_workers[MOD_FRAME_ENC as core::ffi::c_int as usize];
        }
        11 => {
            if (*cpi).oxcf.pass as core::ffi::c_uint
                == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            {
                num_mod_workers = compute_num_ai_workers(cpi);
            } else {
                num_mod_workers = 0 as core::ffi::c_int;
            }
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"compute_num_mod_workers\0"))
                        .as_ptr(),
                    b"ethread.c\0" as *const u8 as *const core::ffi::c_char,
                    3494 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    return num_mod_workers;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_num_workers_for_mt(mut cpi: *mut AV1_COMP) {
    let mut i: core::ffi::c_int = MOD_FP as core::ffi::c_int;
    while i < NUM_MT_MODULES as core::ffi::c_int {
        (*(*cpi).ppi).p_mt_info.num_mod_workers[i as usize] = compute_num_mod_workers(
            cpi,
            i as MULTI_THREADED_MODULES,
        );
        i += 1;
    }
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn pthread_cond_broadcast(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_init(
        _: *mut pthread_cond_t,
        _: *const pthread_condattr_t,
    ) -> core::ffi::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const core::ffi::c_void,
                *const core::ffi::c_void,
            ) -> core::ffi::c_int,
        >,
    );
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
}
