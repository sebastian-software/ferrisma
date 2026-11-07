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

// Mechanically translated from upstream/aom/av1/encoder/encodeframe.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_get_qindex(
        seg: *const segmentation,
        segment_id: core::ffi::c_int,
        base_qindex: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_set_default_ref_deltas(ref_deltas: *mut int8_t);
    fn av1_set_default_mode_deltas(mode_deltas: *mut int8_t);
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
    fn av1_setup_block_planes(
        xd: *mut MACROBLOCKD,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        num_planes: core::ffi::c_int,
    );
    fn av1_reset_loop_filter_delta(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn cfl_init(cfl: *mut CFL_CTX, seq_params: *const SequenceHeader);
    fn av1_setup_frame_buf_refs(cm: *mut AV1_COMMON);
    fn av1_setup_frame_sign_bias(cm: *mut AV1_COMMON);
    fn av1_setup_skip_mode_allowed(cm: *mut AV1_COMMON);
    fn av1_calculate_ref_frame_side(cm: *mut AV1_COMMON);
    fn av1_setup_motion_field(cm: *mut AV1_COMMON);
    fn av1_get_mv_projection(
        output: *mut MV,
        ref_0: MV,
        num: core::ffi::c_int,
        den: core::ffi::c_int,
    );
    fn av1_crc32c_calculator_init(p_crc32c: *mut CRC32C);
    fn av1_hash_table_init(intra_bc_hash_info: *mut IntraBCHashInfo);
    fn av1_hash_table_destroy(p_hash_table: *mut hash_table);
    fn av1_hash_table_create(p_hash_table: *mut hash_table) -> bool;
    fn av1_generate_block_2x2_hash_value(
        picture: *const YV12_BUFFER_CONFIG,
        pic_block_hash: *mut uint32_t,
    );
    fn av1_generate_block_hash_value(
        intra_bc_hash_info: *mut IntraBCHashInfo,
        picture: *const YV12_BUFFER_CONFIG,
        block_size: core::ffi::c_int,
        src_pic_block_hash: *const uint32_t,
        dst_pic_block_hash: *mut uint32_t,
    );
    fn av1_add_to_hash_map_by_row_with_precal_data(
        p_hash_table: *mut hash_table,
        pic_hash: *const uint32_t,
        pic_width: core::ffi::c_int,
        pic_height: core::ffi::c_int,
        block_size: core::ffi::c_int,
    ) -> bool;
    fn av1_init_cyclic_refresh_counters(x: *mut MACROBLOCK);
    fn av1_accumulate_cyclic_refresh_counters(
        cyclic_refresh: *mut CYCLIC_REFRESH,
        x: *const MACROBLOCK,
    );
    fn av1_frame_init_quantizer(cpi: *mut AV1_COMP);
    fn av1_init_plane_quantizers(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        segment_id: core::ffi::c_int,
        do_update: core::ffi::c_int,
    );
    fn av1_initialize_rd_consts(cpi: *mut AV1_COMP);
    fn av1_set_sad_per_bit(
        cpi: *const AV1_COMP,
        sadperbit: *mut core::ffi::c_int,
        qindex: core::ffi::c_int,
    );
    fn av1_adjust_q_from_delta_q_res(
        delta_q_res: core::ffi::c_int,
        prev_qindex: core::ffi::c_int,
        curr_qindex: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_alloc_pc_tree_node(bsize: BLOCK_SIZE) -> *mut PC_TREE;
    fn av1_free_pc_tree_recursive(
        tree: *mut PC_TREE,
        num_planes: core::ffi::c_int,
        keep_best: core::ffi::c_int,
        keep_none: core::ffi::c_int,
        partition_search_type: PARTITION_SEARCH_TYPE,
    );
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
    fn av1_tpl_rdmult_setup_sb(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        sb_size: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_get_sbq_perceptual_ai(
        cpi: *const AV1_COMP,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_sbq_user_rating_based(
        cpi: *const AV1_COMP,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_sbq_variance_boost(
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
    ) -> core::ffi::c_int;
    fn av1_compute_q_from_energy_level_deltaq_mode(
        cpi: *const AV1_COMP,
        block_var_level: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_block_wavelet_energy_level(
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
        bs: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_log_block_var(
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
        bs: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_compute_global_motion_facade(cpi: *mut AV1_COMP);
    fn av1_get_cb_coeff_buffer(
        cpi: *const AV1_COMP,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> *mut CB_COEFF_BUFFER;
    fn av1_inter_mode_data_init(tile_data: *mut TileDataEnc);
    fn av1_inter_mode_data_fit(tile_data: *mut TileDataEnc, rdmult: core::ffi::c_int);
    fn av1_get_rdmult_delta(
        cpi: *mut AV1_COMP,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        orig_rdmult: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_tpl_stats_sb(
        cpi: *mut AV1_COMP,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        sb_enc: *mut SuperBlockEnc,
    );
    fn av1_get_q_for_deltaq_objective(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        delta_dist: *mut int64_t,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_q_for_hdr(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_set_fixed_partitioning(
        cpi: *mut AV1_COMP,
        tile: *const TileInfo,
        mib: *mut *mut MB_MODE_INFO,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    );
    fn av1_reset_simple_motion_tree_partition(
        sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
        bsize: BLOCK_SIZE,
    );
    fn av1_avg_cdf_symbols(
        ctx_left: *mut FRAME_CONTEXT,
        ctx_tr: *mut FRAME_CONTEXT,
        wt_left: core::ffi::c_int,
        wt_tr: core::ffi::c_int,
    );
    fn av1_source_content_sb(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        tile_data: *mut TileDataEnc,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_reset_mbmi(
        mi_params: *mut CommonModeInfoParams,
        sb_size: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_backup_sb_state(
        sb_fp_stats: *mut SB_FIRST_PASS_STATS,
        cpi: *const AV1_COMP,
        td: *mut ThreadData,
        tile_data: *const TileDataEnc,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_restore_sb_state(
        sb_fp_stats: *const SB_FIRST_PASS_STATS,
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_data: *mut TileDataEnc,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_set_cost_upd_freq(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_info: *const TileInfo,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_dealloc_src_diff_buf(mb: *mut macroblock, num_planes: core::ffi::c_int);
    fn av1_alloc_src_diff_buf(cm: *const AV1Common, mb: *mut macroblock);
    fn av1_row_mt_sync_read(
        row_mt_sync: *mut AV1EncRowMultiThreadSync,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
    );
    fn av1_row_mt_sync_write(
        row_mt_sync: *mut AV1EncRowMultiThreadSync,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
        cols: core::ffi::c_int,
    );
    fn av1_row_mt_sync_read_dummy(
        row_mt_sync: *mut AV1EncRowMultiThreadSync,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
    );
    fn av1_row_mt_sync_write_dummy(
        row_mt_sync: *mut AV1EncRowMultiThreadSync,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
        cols: core::ffi::c_int,
    );
    fn av1_encode_tiles_mt(cpi: *mut AV1_COMP);
    fn av1_encode_tiles_row_mt(cpi: *mut AV1_COMP);
    fn av1_row_mt_mem_dealloc(cpi: *mut AV1_COMP);
    fn av1_get_max_min_partition_features(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        features: *mut core::ffi::c_float,
    );
    fn av1_predict_max_partition(
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
        features: *const core::ffi::c_float,
    ) -> BLOCK_SIZE;
    fn av1_init_simple_motion_search_mvs_for_sb(
        cpi: *const AV1_COMP,
        tile_info: *const TileInfo,
        x: *mut MACROBLOCK,
        sms_root: *mut SIMPLE_MOTION_DATA_TREE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_set_offsets(
        cpi: *const AV1_COMP,
        tile: *const TileInfo,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    );
    fn av1_rd_use_partition(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_data: *mut TileDataEnc,
        mib: *mut *mut MB_MODE_INFO,
        tp: *mut *mut TokenExtra,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        rate: *mut core::ffi::c_int,
        dist: *mut int64_t,
        do_recon: core::ffi::c_int,
        pc_tree: *mut PC_TREE,
    );
    fn av1_nonrd_use_partition(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_data: *mut TileDataEnc,
        mib: *mut *mut MB_MODE_INFO,
        tp: *mut *mut TokenExtra,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        pc_tree: *mut PC_TREE,
    );
    fn av1_rd_pick_partition(
        cpi: *mut AV1_COMP,
        td: *mut ThreadData,
        tile_data: *mut TileDataEnc,
        tp: *mut *mut TokenExtra,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        rd_cost: *mut RD_STATS,
        best_rdc: RD_STATS,
        pc_tree: *mut PC_TREE,
        sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
        none_rd: *mut int64_t,
        multi_pass_mode: SB_MULTI_PASS_MODE,
        rect_part_win_info: *mut RD_RECT_PART_WIN_INFO,
    ) -> bool;
    fn av1_choose_var_based_partitioning(
        cpi: *mut AV1_COMP,
        tile: *const TileInfo,
        td: *mut ThreadData,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type float_t = core::ffi::c_float;
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
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
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
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub c2rust_unnamed_0: C2RustUnnamed_9,
    pub c2rust_unnamed_1: C2RustUnnamed_7,
    pub c2rust_unnamed_2: C2RustUnnamed_5,
    pub c2rust_unnamed_3: C2RustUnnamed_3,
    pub c2rust_unnamed_4: C2RustUnnamed_1,
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
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
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
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const TX_MODES: C2RustUnnamed_15 = 3;
pub const TX_MODE_SELECT: C2RustUnnamed_15 = 2;
pub const TX_MODE_LARGEST: C2RustUnnamed_15 = 1;
pub const ONLY_4X4: C2RustUnnamed_15 = 0;
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const AOM_REFFRAME_ALL: C2RustUnnamed_16 = 127;
pub const AOM_ALT_FLAG: C2RustUnnamed_16 = 64;
pub const AOM_ALT2_FLAG: C2RustUnnamed_16 = 32;
pub const AOM_BWD_FLAG: C2RustUnnamed_16 = 16;
pub const AOM_GOLD_FLAG: C2RustUnnamed_16 = 8;
pub const AOM_LAST3_FLAG: C2RustUnnamed_16 = 4;
pub const AOM_LAST2_FLAG: C2RustUnnamed_16 = 2;
pub const AOM_LAST_FLAG: C2RustUnnamed_16 = 1;
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_17 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_17 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_17 = 0;
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_18 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_18 = 7;
pub const BWD_REFS: C2RustUnnamed_18 = 3;
pub const FWD_REFS: C2RustUnnamed_18 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_18 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_18 = 8;
pub const REF_FRAMES: C2RustUnnamed_18 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_18 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_18 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_18 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_18 = 4;
pub const LAST3_FRAME: C2RustUnnamed_18 = 3;
pub const LAST2_FRAME: C2RustUnnamed_18 = 2;
pub const LAST_FRAME: C2RustUnnamed_18 = 1;
pub const INTRA_FRAME: C2RustUnnamed_18 = 0;
pub const NONE_FRAME: C2RustUnnamed_18 = -1;
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const UNIDIR_COMP_REFS: C2RustUnnamed_19 = 4;
pub const TOTAL_UNIDIR_COMP_REFS: C2RustUnnamed_19 = 9;
pub const ALTREF2_ALTREF_FRAMES: C2RustUnnamed_19 = 8;
pub const BWDREF_ALTREF2_FRAMES: C2RustUnnamed_19 = 7;
pub const LAST3_GOLDEN_FRAMES: C2RustUnnamed_19 = 6;
pub const LAST2_GOLDEN_FRAMES: C2RustUnnamed_19 = 5;
pub const LAST2_LAST3_FRAMES: C2RustUnnamed_19 = 4;
pub const BWDREF_ALTREF_FRAMES: C2RustUnnamed_19 = 3;
pub const LAST_GOLDEN_FRAMES: C2RustUnnamed_19 = 2;
pub const LAST_LAST3_FRAMES: C2RustUnnamed_19 = 1;
pub const LAST_LAST2_FRAMES: C2RustUnnamed_19 = 0;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
pub type aom_cdf_prob = uint16_t;
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_20 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_20 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_20 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_20 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_20 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_20 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_20 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_20 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_20 = 0;
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
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_21 = 4;
pub const AFFINE: C2RustUnnamed_21 = 3;
pub const ROTZOOM: C2RustUnnamed_21 = 2;
pub const TRANSLATION: C2RustUnnamed_21 = 1;
pub const IDENTITY: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const INTERP_SKIP_LUMA_SKIP_CHROMA: C2RustUnnamed_22 = 3;
pub const INTERP_EVAL_LUMA_SKIP_CHROMA: C2RustUnnamed_22 = 2;
pub const INTERP_SKIP_LUMA_EVAL_CHROMA: C2RustUnnamed_22 = 1;
pub const INTERP_EVAL_LUMA_EVAL_CHROMA: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_23 = 4;
pub const S_FRAME: C2RustUnnamed_23 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_23 = 2;
pub const INTER_FRAME: C2RustUnnamed_23 = 1;
pub const KEY_FRAME: C2RustUnnamed_23 = 0;
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
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_24 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_24 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_24 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const THR_INVALID: C2RustUnnamed_25 = 255;
pub const THR_INTER_MODE_END: C2RustUnnamed_25 = 156;
pub const THR_INTER_MODE_START: C2RustUnnamed_25 = 0;
pub const THR_MODE_END: C2RustUnnamed_25 = 169;
pub const THR_MODE_START: C2RustUnnamed_25 = 0;
pub const NUM_SINGLE_REF_MODES: C2RustUnnamed_25 = 28;
pub const SINGLE_REF_MODE_END: C2RustUnnamed_25 = 28;
pub const SINGLE_REF_MODE_START: C2RustUnnamed_25 = 0;
pub const MAX_MODES: C2RustUnnamed_25 = 169;
pub const THR_D45_PRED: C2RustUnnamed_25 = 168;
pub const THR_D113_PRED: C2RustUnnamed_25 = 167;
pub const THR_D67_PRED: C2RustUnnamed_25 = 166;
pub const THR_D157_PRED: C2RustUnnamed_25 = 165;
pub const THR_D203_PRED: C2RustUnnamed_25 = 164;
pub const THR_D135_PRED: C2RustUnnamed_25 = 163;
pub const THR_V_PRED: C2RustUnnamed_25 = 162;
pub const THR_H_PRED: C2RustUnnamed_25 = 161;
pub const THR_SMOOTH_H: C2RustUnnamed_25 = 160;
pub const THR_SMOOTH_V: C2RustUnnamed_25 = 159;
pub const THR_SMOOTH: C2RustUnnamed_25 = 158;
pub const THR_PAETH: C2RustUnnamed_25 = 157;
pub const THR_DC: C2RustUnnamed_25 = 156;
pub const THR_COMP_GLOBAL_GLOBALBA: C2RustUnnamed_25 = 155;
pub const THR_COMP_NEAR_NEWBA: C2RustUnnamed_25 = 154;
pub const THR_COMP_NEW_NEARBA: C2RustUnnamed_25 = 153;
pub const THR_COMP_NEAREST_NEWBA: C2RustUnnamed_25 = 152;
pub const THR_COMP_NEW_NEARESTBA: C2RustUnnamed_25 = 151;
pub const THR_COMP_NEW_NEWBA: C2RustUnnamed_25 = 150;
pub const THR_COMP_NEAR_NEARBA: C2RustUnnamed_25 = 149;
pub const THR_COMP_GLOBAL_GLOBALLG: C2RustUnnamed_25 = 148;
pub const THR_COMP_NEAR_NEWLG: C2RustUnnamed_25 = 147;
pub const THR_COMP_NEW_NEARLG: C2RustUnnamed_25 = 146;
pub const THR_COMP_NEAREST_NEWLG: C2RustUnnamed_25 = 145;
pub const THR_COMP_NEW_NEARESTLG: C2RustUnnamed_25 = 144;
pub const THR_COMP_NEW_NEWLG: C2RustUnnamed_25 = 143;
pub const THR_COMP_NEAR_NEARLG: C2RustUnnamed_25 = 142;
pub const THR_COMP_GLOBAL_GLOBALLL3: C2RustUnnamed_25 = 141;
pub const THR_COMP_NEAR_NEWLL3: C2RustUnnamed_25 = 140;
pub const THR_COMP_NEW_NEARLL3: C2RustUnnamed_25 = 139;
pub const THR_COMP_NEAREST_NEWLL3: C2RustUnnamed_25 = 138;
pub const THR_COMP_NEW_NEARESTLL3: C2RustUnnamed_25 = 137;
pub const THR_COMP_NEW_NEWLL3: C2RustUnnamed_25 = 136;
pub const THR_COMP_NEAR_NEARLL3: C2RustUnnamed_25 = 135;
pub const THR_COMP_GLOBAL_GLOBALLL2: C2RustUnnamed_25 = 134;
pub const THR_COMP_NEAR_NEWLL2: C2RustUnnamed_25 = 133;
pub const THR_COMP_NEW_NEARLL2: C2RustUnnamed_25 = 132;
pub const THR_COMP_NEAREST_NEWLL2: C2RustUnnamed_25 = 131;
pub const THR_COMP_NEW_NEARESTLL2: C2RustUnnamed_25 = 130;
pub const THR_COMP_NEW_NEWLL2: C2RustUnnamed_25 = 129;
pub const THR_COMP_NEAR_NEARLL2: C2RustUnnamed_25 = 128;
pub const THR_COMP_GLOBAL_GLOBALGA2: C2RustUnnamed_25 = 127;
pub const THR_COMP_NEAR_NEWGA2: C2RustUnnamed_25 = 126;
pub const THR_COMP_NEW_NEARGA2: C2RustUnnamed_25 = 125;
pub const THR_COMP_NEAREST_NEWGA2: C2RustUnnamed_25 = 124;
pub const THR_COMP_NEW_NEARESTGA2: C2RustUnnamed_25 = 123;
pub const THR_COMP_NEW_NEWGA2: C2RustUnnamed_25 = 122;
pub const THR_COMP_NEAR_NEARGA2: C2RustUnnamed_25 = 121;
pub const THR_COMP_GLOBAL_GLOBALL3A2: C2RustUnnamed_25 = 120;
pub const THR_COMP_NEAR_NEWL3A2: C2RustUnnamed_25 = 119;
pub const THR_COMP_NEW_NEARL3A2: C2RustUnnamed_25 = 118;
pub const THR_COMP_NEAREST_NEWL3A2: C2RustUnnamed_25 = 117;
pub const THR_COMP_NEW_NEARESTL3A2: C2RustUnnamed_25 = 116;
pub const THR_COMP_NEW_NEWL3A2: C2RustUnnamed_25 = 115;
pub const THR_COMP_NEAR_NEARL3A2: C2RustUnnamed_25 = 114;
pub const THR_COMP_GLOBAL_GLOBALL2A2: C2RustUnnamed_25 = 113;
pub const THR_COMP_NEAR_NEWL2A2: C2RustUnnamed_25 = 112;
pub const THR_COMP_NEW_NEARL2A2: C2RustUnnamed_25 = 111;
pub const THR_COMP_NEAREST_NEWL2A2: C2RustUnnamed_25 = 110;
pub const THR_COMP_NEW_NEARESTL2A2: C2RustUnnamed_25 = 109;
pub const THR_COMP_NEW_NEWL2A2: C2RustUnnamed_25 = 108;
pub const THR_COMP_NEAR_NEARL2A2: C2RustUnnamed_25 = 107;
pub const THR_COMP_GLOBAL_GLOBALLA2: C2RustUnnamed_25 = 106;
pub const THR_COMP_NEAR_NEWLA2: C2RustUnnamed_25 = 105;
pub const THR_COMP_NEW_NEARLA2: C2RustUnnamed_25 = 104;
pub const THR_COMP_NEAREST_NEWLA2: C2RustUnnamed_25 = 103;
pub const THR_COMP_NEW_NEARESTLA2: C2RustUnnamed_25 = 102;
pub const THR_COMP_NEW_NEWLA2: C2RustUnnamed_25 = 101;
pub const THR_COMP_NEAR_NEARLA2: C2RustUnnamed_25 = 100;
pub const THR_COMP_GLOBAL_GLOBALGB: C2RustUnnamed_25 = 99;
pub const THR_COMP_NEAR_NEWGB: C2RustUnnamed_25 = 98;
pub const THR_COMP_NEW_NEARGB: C2RustUnnamed_25 = 97;
pub const THR_COMP_NEAREST_NEWGB: C2RustUnnamed_25 = 96;
pub const THR_COMP_NEW_NEARESTGB: C2RustUnnamed_25 = 95;
pub const THR_COMP_NEW_NEWGB: C2RustUnnamed_25 = 94;
pub const THR_COMP_NEAR_NEARGB: C2RustUnnamed_25 = 93;
pub const THR_COMP_GLOBAL_GLOBALL3B: C2RustUnnamed_25 = 92;
pub const THR_COMP_NEAR_NEWL3B: C2RustUnnamed_25 = 91;
pub const THR_COMP_NEW_NEARL3B: C2RustUnnamed_25 = 90;
pub const THR_COMP_NEAREST_NEWL3B: C2RustUnnamed_25 = 89;
pub const THR_COMP_NEW_NEARESTL3B: C2RustUnnamed_25 = 88;
pub const THR_COMP_NEW_NEWL3B: C2RustUnnamed_25 = 87;
pub const THR_COMP_NEAR_NEARL3B: C2RustUnnamed_25 = 86;
pub const THR_COMP_GLOBAL_GLOBALL2B: C2RustUnnamed_25 = 85;
pub const THR_COMP_NEAR_NEWL2B: C2RustUnnamed_25 = 84;
pub const THR_COMP_NEW_NEARL2B: C2RustUnnamed_25 = 83;
pub const THR_COMP_NEAREST_NEWL2B: C2RustUnnamed_25 = 82;
pub const THR_COMP_NEW_NEARESTL2B: C2RustUnnamed_25 = 81;
pub const THR_COMP_NEW_NEWL2B: C2RustUnnamed_25 = 80;
pub const THR_COMP_NEAR_NEARL2B: C2RustUnnamed_25 = 79;
pub const THR_COMP_GLOBAL_GLOBALGA: C2RustUnnamed_25 = 78;
pub const THR_COMP_NEAR_NEWGA: C2RustUnnamed_25 = 77;
pub const THR_COMP_NEW_NEARGA: C2RustUnnamed_25 = 76;
pub const THR_COMP_NEAREST_NEWGA: C2RustUnnamed_25 = 75;
pub const THR_COMP_NEW_NEARESTGA: C2RustUnnamed_25 = 74;
pub const THR_COMP_NEW_NEWGA: C2RustUnnamed_25 = 73;
pub const THR_COMP_NEAR_NEARGA: C2RustUnnamed_25 = 72;
pub const THR_COMP_GLOBAL_GLOBALL3A: C2RustUnnamed_25 = 71;
pub const THR_COMP_NEAR_NEWL3A: C2RustUnnamed_25 = 70;
pub const THR_COMP_NEW_NEARL3A: C2RustUnnamed_25 = 69;
pub const THR_COMP_NEAREST_NEWL3A: C2RustUnnamed_25 = 68;
pub const THR_COMP_NEW_NEARESTL3A: C2RustUnnamed_25 = 67;
pub const THR_COMP_NEW_NEWL3A: C2RustUnnamed_25 = 66;
pub const THR_COMP_NEAR_NEARL3A: C2RustUnnamed_25 = 65;
pub const THR_COMP_GLOBAL_GLOBALL2A: C2RustUnnamed_25 = 64;
pub const THR_COMP_NEAR_NEWL2A: C2RustUnnamed_25 = 63;
pub const THR_COMP_NEW_NEARL2A: C2RustUnnamed_25 = 62;
pub const THR_COMP_NEAREST_NEWL2A: C2RustUnnamed_25 = 61;
pub const THR_COMP_NEW_NEARESTL2A: C2RustUnnamed_25 = 60;
pub const THR_COMP_NEW_NEWL2A: C2RustUnnamed_25 = 59;
pub const THR_COMP_NEAR_NEARL2A: C2RustUnnamed_25 = 58;
pub const THR_COMP_GLOBAL_GLOBALLA: C2RustUnnamed_25 = 57;
pub const THR_COMP_NEAR_NEWLA: C2RustUnnamed_25 = 56;
pub const THR_COMP_NEW_NEARLA: C2RustUnnamed_25 = 55;
pub const THR_COMP_NEAREST_NEWLA: C2RustUnnamed_25 = 54;
pub const THR_COMP_NEW_NEARESTLA: C2RustUnnamed_25 = 53;
pub const THR_COMP_NEW_NEWLA: C2RustUnnamed_25 = 52;
pub const THR_COMP_NEAR_NEARLA: C2RustUnnamed_25 = 51;
pub const THR_COMP_GLOBAL_GLOBALLB: C2RustUnnamed_25 = 50;
pub const THR_COMP_NEAR_NEWLB: C2RustUnnamed_25 = 49;
pub const THR_COMP_NEW_NEARLB: C2RustUnnamed_25 = 48;
pub const THR_COMP_NEAREST_NEWLB: C2RustUnnamed_25 = 47;
pub const THR_COMP_NEW_NEARESTLB: C2RustUnnamed_25 = 46;
pub const THR_COMP_NEW_NEWLB: C2RustUnnamed_25 = 45;
pub const THR_COMP_NEAR_NEARLB: C2RustUnnamed_25 = 44;
pub const THR_COMP_NEAREST_NEARESTBA: C2RustUnnamed_25 = 43;
pub const THR_COMP_NEAREST_NEARESTLG: C2RustUnnamed_25 = 42;
pub const THR_COMP_NEAREST_NEARESTLL3: C2RustUnnamed_25 = 41;
pub const THR_COMP_NEAREST_NEARESTLL2: C2RustUnnamed_25 = 40;
pub const THR_COMP_NEAREST_NEARESTGA2: C2RustUnnamed_25 = 39;
pub const THR_COMP_NEAREST_NEARESTL3A2: C2RustUnnamed_25 = 38;
pub const THR_COMP_NEAREST_NEARESTL2A2: C2RustUnnamed_25 = 37;
pub const THR_COMP_NEAREST_NEARESTLA2: C2RustUnnamed_25 = 36;
pub const THR_COMP_NEAREST_NEARESTGB: C2RustUnnamed_25 = 35;
pub const THR_COMP_NEAREST_NEARESTL3B: C2RustUnnamed_25 = 34;
pub const THR_COMP_NEAREST_NEARESTL2B: C2RustUnnamed_25 = 33;
pub const THR_COMP_NEAREST_NEARESTLB: C2RustUnnamed_25 = 32;
pub const THR_COMP_NEAREST_NEARESTGA: C2RustUnnamed_25 = 31;
pub const THR_COMP_NEAREST_NEARESTL3A: C2RustUnnamed_25 = 30;
pub const THR_COMP_NEAREST_NEARESTL2A: C2RustUnnamed_25 = 29;
pub const THR_COMP_NEAREST_NEARESTLA: C2RustUnnamed_25 = 28;
pub const THR_GLOBALG: C2RustUnnamed_25 = 27;
pub const THR_GLOBALA: C2RustUnnamed_25 = 26;
pub const THR_GLOBALA2: C2RustUnnamed_25 = 25;
pub const THR_GLOBALB: C2RustUnnamed_25 = 24;
pub const THR_GLOBALL3: C2RustUnnamed_25 = 23;
pub const THR_GLOBALL2: C2RustUnnamed_25 = 22;
pub const THR_GLOBALMV: C2RustUnnamed_25 = 21;
pub const THR_NEARG: C2RustUnnamed_25 = 20;
pub const THR_NEARA: C2RustUnnamed_25 = 19;
pub const THR_NEARA2: C2RustUnnamed_25 = 18;
pub const THR_NEARB: C2RustUnnamed_25 = 17;
pub const THR_NEARL3: C2RustUnnamed_25 = 16;
pub const THR_NEARL2: C2RustUnnamed_25 = 15;
pub const THR_NEARMV: C2RustUnnamed_25 = 14;
pub const THR_NEWG: C2RustUnnamed_25 = 13;
pub const THR_NEWA: C2RustUnnamed_25 = 12;
pub const THR_NEWA2: C2RustUnnamed_25 = 11;
pub const THR_NEWB: C2RustUnnamed_25 = 10;
pub const THR_NEWL3: C2RustUnnamed_25 = 9;
pub const THR_NEWL2: C2RustUnnamed_25 = 8;
pub const THR_NEWMV: C2RustUnnamed_25 = 7;
pub const THR_NEARESTG: C2RustUnnamed_25 = 6;
pub const THR_NEARESTA: C2RustUnnamed_25 = 5;
pub const THR_NEARESTA2: C2RustUnnamed_25 = 4;
pub const THR_NEARESTB: C2RustUnnamed_25 = 3;
pub const THR_NEARESTL3: C2RustUnnamed_25 = 2;
pub const THR_NEARESTL2: C2RustUnnamed_25 = 1;
pub const THR_NEARESTMV: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = core::ffi::c_uint;
pub const USE_LARGESTALL: C2RustUnnamed_26 = 2;
pub const USE_FAST_RD: C2RustUnnamed_26 = 1;
pub const USE_FULL_RD: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_27 = 2;
pub const REALTIME: C2RustUnnamed_27 = 1;
pub const GOOD: C2RustUnnamed_27 = 0;
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
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_28 = 2;
pub const LAP_STAGE: C2RustUnnamed_28 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const FRAME_UPDATE_TYPES: C2RustUnnamed_29 = 7;
pub const INTNL_ARF_UPDATE: C2RustUnnamed_29 = 6;
pub const INTNL_OVERLAY_UPDATE: C2RustUnnamed_29 = 5;
pub const OVERLAY_UPDATE: C2RustUnnamed_29 = 4;
pub const ARF_UPDATE: C2RustUnnamed_29 = 3;
pub const GF_UPDATE: C2RustUnnamed_29 = 2;
pub const LF_UPDATE: C2RustUnnamed_29 = 1;
pub const KF_UPDATE: C2RustUnnamed_29 = 0;
pub type C2RustUnnamed_30 = core::ffi::c_uint;
pub const MODE_EVAL_TYPES: C2RustUnnamed_30 = 3;
pub const WINNER_MODE_EVAL: C2RustUnnamed_30 = 2;
pub const MODE_EVAL: C2RustUnnamed_30 = 1;
pub const DEFAULT_EVAL: C2RustUnnamed_30 = 0;
pub type MODE_EVAL_TYPE = uint8_t;
pub type C2RustUnnamed_31 = core::ffi::c_uint;
pub const FULL_PEL: C2RustUnnamed_31 = 3;
pub const HALF_PEL: C2RustUnnamed_31 = 2;
pub const QUARTER_PEL: C2RustUnnamed_31 = 1;
pub const EIGHTH_PEL: C2RustUnnamed_31 = 0;
pub type C2RustUnnamed_32 = core::ffi::c_uint;
pub const NO_ESTIMATE_YRD_TRELLIS_OPT: C2RustUnnamed_32 = 3;
pub const FINAL_PASS_TRELLIS_OPT: C2RustUnnamed_32 = 2;
pub const FULL_TRELLIS_OPT: C2RustUnnamed_32 = 1;
pub const NO_TRELLIS_OPT: C2RustUnnamed_32 = 0;
pub type C2RustUnnamed_33 = core::ffi::c_uint;
pub const MULTI_WINNER_MODE_LEVELS: C2RustUnnamed_33 = 3;
pub const MULTI_WINNER_MODE_DEFAULT: C2RustUnnamed_33 = 2;
pub const MULTI_WINNER_MODE_FAST: C2RustUnnamed_33 = 1;
pub const MULTI_WINNER_MODE_OFF: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_34 = core::ffi::c_uint;
pub const VAR_BASED_PARTITION: C2RustUnnamed_34 = 2;
pub const FIXED_PARTITION: C2RustUnnamed_34 = 1;
pub const SEARCH_PARTITION: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = core::ffi::c_uint;
pub const ADAPT_PRED: C2RustUnnamed_35 = 3;
pub const RELAXED_PRED: C2RustUnnamed_35 = 2;
pub const DIRECT_PRED: C2RustUnnamed_35 = 1;
pub const NOT_IN_USE: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_36 = core::ffi::c_uint;
pub const AQ_MODE_COUNT: C2RustUnnamed_36 = 4;
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_36 = 3;
pub const COMPLEXITY_AQ: C2RustUnnamed_36 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_36 = 1;
pub const NO_AQ: C2RustUnnamed_36 = 0;
pub type C2RustUnnamed_37 = core::ffi::c_uint;
pub const DELTA_Q_MODE_COUNT: C2RustUnnamed_37 = 7;
pub const DELTA_Q_VARIANCE_BOOST: C2RustUnnamed_37 = 6;
pub const DELTA_Q_HDR: C2RustUnnamed_37 = 5;
pub const DELTA_Q_USER_RATING_BASED: C2RustUnnamed_37 = 4;
pub const DELTA_Q_PERCEPTUAL_AI: C2RustUnnamed_37 = 3;
pub const DELTA_Q_PERCEPTUAL: C2RustUnnamed_37 = 2;
pub const DELTA_Q_OBJECTIVE: C2RustUnnamed_37 = 1;
pub const NO_DELTA_Q: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_38 = core::ffi::c_uint;
pub const NUM_MT_MODULES: C2RustUnnamed_38 = 12;
pub const MOD_AI: C2RustUnnamed_38 = 11;
pub const MOD_FRAME_ENC: C2RustUnnamed_38 = 10;
pub const MOD_PACK_BS: C2RustUnnamed_38 = 9;
pub const MOD_LR: C2RustUnnamed_38 = 8;
pub const MOD_CDEF: C2RustUnnamed_38 = 7;
pub const MOD_CDEF_SEARCH: C2RustUnnamed_38 = 6;
pub const MOD_LPF: C2RustUnnamed_38 = 5;
pub const MOD_ENC: C2RustUnnamed_38 = 4;
pub const MOD_GME: C2RustUnnamed_38 = 3;
pub const MOD_TPL: C2RustUnnamed_38 = 2;
pub const MOD_TF: C2RustUnnamed_38 = 1;
pub const MOD_FP: C2RustUnnamed_38 = 0;
pub type InterModesInfo = inter_modes_info;
pub type C2RustUnnamed_39 = core::ffi::c_uint;
pub const DUCKY_ENCODE_FRAME_MODE_QINDEX_RDMULT: C2RustUnnamed_39 = 2;
pub const DUCKY_ENCODE_FRAME_MODE_QINDEX: C2RustUnnamed_39 = 1;
pub const DUCKY_ENCODE_FRAME_MODE_NONE: C2RustUnnamed_39 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_RECT_PART_WIN_INFO {
    pub rect_part_win: [core::ffi::c_int; 2],
}
pub type SB_MULTI_PASS_MODE = uint8_t;
pub const SB_WET_PASS: C2RustUnnamed_40 = 2;
pub const SB_DRY_PASS: C2RustUnnamed_40 = 1;
pub const SB_SINGLE_PASS: C2RustUnnamed_40 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefScoreData {
    pub score: core::ffi::c_int,
    pub index: core::ffi::c_int,
}
pub type C2RustUnnamed_40 = core::ffi::c_uint;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT64_MAX: core::ffi::c_ulonglong = 18446744073709551615
    as core::ffi::c_ulonglong;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_SB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2;
pub const MAX_SB_SQUARE: core::ffi::c_int = MAX_SB_SIZE * MAX_SB_SIZE;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MI_SIZE_64X64: core::ffi::c_int = 64 as core::ffi::c_int >> MI_SIZE_LOG2;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DEFAULT_DELTA_LF_MULTI: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_MODE_LF_DELTAS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const DEFAULT_DELTA_Q_RES_PERCEPTUAL: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DEFAULT_DELTA_Q_RES_OBJECTIVE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DEFAULT_DELTA_Q_RES_DUCKY_ENCODE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DEFAULT_DELTA_LF_RES: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const MINQ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAXQ: core::ffi::c_int = 255 as core::ffi::c_int;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
static mut kZeroMv: MV = {
    let mut init = mv {
        row: 0 as int16_t,
        col: 0 as int16_t,
    };
    init
};
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
        if !(1 as core::ffi::c_int
            & (res.as_mv.row as core::ffi::c_int | res.as_mv.col as core::ffi::c_int)
            == 0 || allow_hp != 0) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"gm_get_motion_vector\0"))
                    .as_ptr(),
                b"mv.h\0" as *const u8 as *const core::ffi::c_char,
                268 as core::ffi::c_int,
                b"IMPLIES(1 & (res.as_mv.row | res.as_mv.col), allow_hp)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if is_integer != 0 {
            integer_mv_precision(&raw mut res.as_mv);
        }
        return res;
    }
    x = block_center_x(mi_col, bsize);
    y = block_center_y(mi_row, bsize);
    if (*gm).wmtype as core::ffi::c_int == ROTZOOM as core::ffi::c_int {
        if !((*gm).wmmat[5 as core::ffi::c_int as usize]
            == (*gm).wmmat[2 as core::ffi::c_int as usize]) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"gm_get_motion_vector\0"))
                    .as_ptr(),
                b"mv.h\0" as *const u8 as *const core::ffi::c_char,
                279 as core::ffi::c_int,
                b"gm->wmmat[5] == gm->wmmat[2]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !((*gm).wmmat[4 as core::ffi::c_int as usize]
            == -(*gm).wmmat[3 as core::ffi::c_int as usize]) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"gm_get_motion_vector\0"))
                    .as_ptr(),
                b"mv.h\0" as *const u8 as *const core::ffi::c_char,
                280 as core::ffi::c_int,
                b"gm->wmmat[4] == -gm->wmmat[3]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
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
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
pub const REF_SCALE_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const REF_NO_SCALE: core::ffi::c_int = (1 as core::ffi::c_int) << REF_SCALE_SHIFT;
pub const REF_INVALID_SCALE: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn av1_scaled_x(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).x_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).x_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_scaled_y(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).y_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).y_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_unscaled_value(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    return val * ((1 as core::ffi::c_int) << SCALE_EXTRA_BITS);
}
#[inline]
unsafe extern "C-unwind" fn av1_is_valid_scale(
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    if sf.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_is_valid_scale\0"))
                .as_ptr(),
            b"scale.h\0" as *const u8 as *const core::ffi::c_char,
            65 as core::ffi::c_int,
            b"sf != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ((*sf).x_scale_fp != REF_INVALID_SCALE
        && (*sf).y_scale_fp != REF_INVALID_SCALE) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_scaled(mut sf: *const scale_factors) -> core::ffi::c_int {
    if sf.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"av1_is_scaled\0"))
                .as_ptr(),
            b"scale.h\0" as *const u8 as *const core::ffi::c_char,
            71 as core::ffi::c_int,
            b"sf != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return (av1_is_valid_scale(sf) != 0
        && ((*sf).x_scale_fp != REF_NO_SCALE || (*sf).y_scale_fp != REF_NO_SCALE))
        as core::ffi::c_int;
}
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
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
                (*xd).error_info as *mut aom_internal_error_info,
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
#[inline]
unsafe extern "C-unwind" fn av1_zero_left_context(xd: *mut MACROBLOCKD) {
    memset(
        &raw mut (*xd).left_entropy_context as *mut [[ENTROPY_CONTEXT; 32]; 3]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[ENTROPY_CONTEXT; 32]; 3]>() as size_t,
    );
    memset(
        &raw mut (*xd).left_partition_context as *mut [PARTITION_CONTEXT; 32]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[PARTITION_CONTEXT; 32]>() as size_t,
    );
    memset(
        ((*xd).left_txfm_context_buffer).as_mut_ptr() as *mut core::ffi::c_void,
        tx_size_high[TX_SIZES_LARGEST as core::ffi::c_int as usize],
        ::core::mem::size_of::<[TXFM_CONTEXT; 32]>() as size_t,
    );
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
unsafe extern "C-unwind" fn set_mi_offsets(
    mi_params: *const CommonModeInfoParams,
    xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mi_grid_idx: core::ffi::c_int = get_mi_grid_idx(mi_params, mi_row, mi_col)
        as core::ffi::c_int;
    let mi_alloc_idx: core::ffi::c_int = get_alloc_mi_idx(mi_params, mi_row, mi_col)
        as core::ffi::c_int;
    let fresh0 = &raw mut *((*mi_params).mi_grid_base).offset(mi_grid_idx as isize);
    *fresh0 = &raw mut *((*mi_params).mi_alloc).offset(mi_alloc_idx as isize)
        as *mut MB_MODE_INFO;
    (*xd).mi = ((*mi_params).mi_grid_base).offset(mi_grid_idx as isize);
    (*xd).tx_type_map = ((*mi_params).tx_type_map).offset(mi_grid_idx as isize)
        as *mut uint8_t;
    (*xd).tx_type_map_stride = (*mi_params).mi_stride;
}
#[inline]
unsafe extern "C-unwind" fn is_coded_lossless(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut coded_lossless: core::ffi::c_int = 1 as core::ffi::c_int;
    if (*cm).seg.enabled != 0 {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < MAX_SEGMENTS {
            if (*xd).lossless[i as usize] == 0 {
                coded_lossless = 0 as core::ffi::c_int;
                break;
            } else {
                i += 1;
            }
        }
    } else {
        coded_lossless = (*xd).lossless[0 as core::ffi::c_int as usize];
    }
    return coded_lossless;
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
    if !(bits >= 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_relative_dist\0"))
                .as_ptr(),
            b"mvref_common.h\0" as *const u8 as *const core::ffi::c_char,
            42 as core::ffi::c_int,
            b"bits >= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(a >= 0 as core::ffi::c_int && a < (1 as core::ffi::c_int) << bits)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_relative_dist\0"))
                .as_ptr(),
            b"mvref_common.h\0" as *const u8 as *const core::ffi::c_char,
            43 as core::ffi::c_int,
            b"a >= 0 && a < (1 << bits)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(b >= 0 as core::ffi::c_int && b < (1 as core::ffi::c_int) << bits)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_relative_dist\0"))
                .as_ptr(),
            b"mvref_common.h\0" as *const u8 as *const core::ffi::c_char,
            44 as core::ffi::c_int,
            b"b >= 0 && b < (1 << bits)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
static mut ref_frame_map: [[MV_REFERENCE_FRAME; 2]; 21] = [
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
    [
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ],
];
#[inline]
unsafe extern "C-unwind" fn av1_set_ref_frame(
    mut rf: *mut MV_REFERENCE_FRAME,
    mut ref_frame_type: MV_REFERENCE_FRAME,
) {
    if ref_frame_type as core::ffi::c_int >= REF_FRAMES as core::ffi::c_int {
        *rf.offset(0 as core::ffi::c_int as isize) = ref_frame_map[(ref_frame_type
            as core::ffi::c_int - REF_FRAMES as core::ffi::c_int)
            as usize][0 as core::ffi::c_int as usize];
        *rf.offset(1 as core::ffi::c_int as isize) = ref_frame_map[(ref_frame_type
            as core::ffi::c_int - REF_FRAMES as core::ffi::c_int)
            as usize][1 as core::ffi::c_int as usize];
    } else {
        if !(ref_frame_type as core::ffi::c_int > NONE_FRAME as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"av1_set_ref_frame\0"))
                    .as_ptr(),
                b"mvref_common.h\0" as *const u8 as *const core::ffi::c_char,
                158 as core::ffi::c_int,
                b"ref_frame_type > NONE_FRAME\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        *rf.offset(0 as core::ffi::c_int as isize) = ref_frame_type;
        *rf.offset(1 as core::ffi::c_int as isize) = NONE_FRAME as core::ffi::c_int
            as MV_REFERENCE_FRAME;
    };
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
unsafe extern "C-unwind" fn scaled_buffer_offset(
    mut x_offset: core::ffi::c_int,
    mut y_offset: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> int64_t {
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    if sf.is_null() {
        x = x_offset;
        y = y_offset;
    } else if av1_is_scaled(sf) != 0 {
        x = av1_scaled_x(x_offset, sf) >> SCALE_EXTRA_BITS;
        y = av1_scaled_y(y_offset, sf) >> SCALE_EXTRA_BITS;
    } else {
        x = av1_unscaled_value(x_offset, sf) >> SCALE_EXTRA_BITS;
        y = av1_unscaled_value(y_offset, sf) >> SCALE_EXTRA_BITS;
    }
    return y as int64_t * stride as int64_t + x as int64_t;
}
#[inline]
unsafe extern "C-unwind" fn setup_pred_plane(
    mut dst: *mut buf_2d,
    mut bsize: BLOCK_SIZE,
    mut src: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut scale: *const scale_factors,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) {
    if subsampling_y != 0 && mi_row & 0x1 as core::ffi::c_int != 0
        && mi_size_high[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
    {
        mi_row -= 1 as core::ffi::c_int;
    }
    if subsampling_x != 0 && mi_col & 0x1 as core::ffi::c_int != 0
        && mi_size_wide[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
    {
        mi_col -= 1 as core::ffi::c_int;
    }
    let x: core::ffi::c_int = MI_SIZE * mi_col >> subsampling_x;
    let y: core::ffi::c_int = MI_SIZE * mi_row >> subsampling_y;
    (*dst).buf = src.offset(scaled_buffer_offset(x, y, stride, scale) as isize);
    (*dst).buf0 = src;
    (*dst).width = width;
    (*dst).height = height;
    (*dst).stride = stride;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const MAX_TX_TYPE_PROB: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const AOM_DEFAULT_Q: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_token_alloc(
    mut mb_rows: core::ffi::c_int,
    mut mb_cols: core::ffi::c_int,
    mut sb_size_log2: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) -> core::ffi::c_uint {
    let shift: core::ffi::c_int = sb_size_log2 - 4 as core::ffi::c_int;
    let sb_size: core::ffi::c_int = (1 as core::ffi::c_int) << sb_size_log2;
    let sb_size_square: core::ffi::c_int = sb_size * sb_size;
    let sb_rows: core::ffi::c_int = mb_rows + ((1 as core::ffi::c_int) << shift)
        - 1 as core::ffi::c_int >> shift;
    let sb_cols: core::ffi::c_int = mb_cols + ((1 as core::ffi::c_int) << shift)
        - 1 as core::ffi::c_int >> shift;
    let sb_palette_toks: core::ffi::c_int = (if (2 as core::ffi::c_int) < num_planes {
        2 as core::ffi::c_int
    } else {
        num_planes
    }) * sb_size_square;
    return (sb_rows * sb_cols * sb_palette_toks) as core::ffi::c_uint;
}
#[inline]
unsafe extern "C-unwind" fn alloc_token_info(
    mut cm: *mut AV1_COMMON,
    mut token_info: *mut TokenInfo,
    mut tokens_required: core::ffi::c_uint,
) {
    let mut sb_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*(*cm).seq_params).mib_size_log2;
    (*token_info).tokens_allocated = tokens_required;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"alloc_token_info\0"))
                .as_ptr(),
            b"tokenize.h\0" as *const u8 as *const core::ffi::c_char,
            130 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*token_info)
        .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = aom_calloc(
        tokens_required as size_t,
        ::core::mem::size_of::<TokenExtra>() as size_t,
    ) as *mut TokenExtra;
    if ((*token_info)
        .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize])
        .is_null()
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate token_info->tile_tok[0][0] at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/tokenize.h\0" as *const u8
                as *const core::ffi::c_char,
            130 as core::ffi::c_int,
        );
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"alloc_token_info\0"))
                .as_ptr(),
            b"tokenize.h\0" as *const u8 as *const core::ffi::c_char,
            135 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*token_info)
        .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = aom_calloc(
        (sb_rows * 64 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t,
        ::core::mem::size_of::<TokenList>() as size_t,
    ) as *mut TokenList;
    if ((*token_info)
        .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize])
        .is_null()
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate token_info->tplist[0][0] at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/tokenize.h\0" as *const u8
                as *const core::ffi::c_char,
            135 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn is_token_info_allocated(mut token_info: *const TokenInfo) -> bool {
    return !((*token_info)
        .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize])
        .is_null()
        && !((*token_info)
            .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize])
            .is_null();
}
#[inline]
unsafe extern "C-unwind" fn free_token_info(mut token_info: *mut TokenInfo) {
    aom_free(
        (*token_info)
            .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            as *mut core::ffi::c_void,
    );
    (*token_info)
        .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 0
        as *mut TokenExtra;
    aom_free(
        (*token_info)
            .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            as *mut core::ffi::c_void,
    );
    (*token_info)
        .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 0
        as *mut TokenList;
    (*token_info).tokens_allocated = 0 as core::ffi::c_uint;
}
pub const RD_THRESH_FAC_FRAC_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const RD_THRESH_FAC_FRAC_VAL: core::ffi::c_int = (1 as core::ffi::c_int)
    << 5 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn reset_thresh_freq_fact(x: *mut MACROBLOCK) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < BLOCK_SIZES_ALL as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < MAX_MODES as core::ffi::c_int {
            (*x).thresh_freq_fact[i as usize][j as usize] = RD_THRESH_FAC_FRAC_VAL;
            j += 1;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn reset_mb_rd_record(mb_rd_record: *mut MB_RD_RECORD) {
    if mb_rd_record.is_null() {
        return;
    }
    (*mb_rd_record).index_start = 0 as core::ffi::c_int;
    (*mb_rd_record).num = (*mb_rd_record).index_start;
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
unsafe extern "C-unwind" fn av1_use_hash_me(cpi: *const AV1_COMP) -> core::ffi::c_int {
    return ((*cpi).common.features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cpi).common.features.allow_intrabc as core::ffi::c_int != 0
        && frame_is_intra_only(&(*cpi).common) != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn allocated_tokens(
    mut tile: *const TileInfo,
    mut sb_size_log2: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut tile_mb_rows: core::ffi::c_int = (*tile).mi_row_end - (*tile).mi_row_start
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 2 as core::ffi::c_int;
    let mut tile_mb_cols: core::ffi::c_int = (*tile).mi_col_end - (*tile).mi_col_start
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 2 as core::ffi::c_int;
    return get_token_alloc(tile_mb_rows, tile_mb_cols, sb_size_log2, num_planes);
}
#[inline]
unsafe extern "C-unwind" fn get_start_tok(
    mut cpi: *mut AV1_COMP,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut tok: *mut *mut TokenExtra,
    mut sb_size_log2: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let mut this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
        .offset((tile_row * tile_cols + tile_col) as isize) as *mut TileDataEnc;
    let tile_info: *const TileInfo = &raw mut (*this_tile).tile_info;
    let tile_mb_cols: core::ffi::c_int = (*tile_info).mi_col_end
        - (*tile_info).mi_col_start + 2 as core::ffi::c_int >> 2 as core::ffi::c_int;
    let tile_mb_row: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
        + 2 as core::ffi::c_int >> 2 as core::ffi::c_int;
    *tok = ((*cpi).token_info.tile_tok[tile_row as usize][tile_col as usize])
        .offset(
            get_token_alloc(tile_mb_row, tile_mb_cols, sb_size_log2, num_planes) as isize,
        );
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
unsafe extern "C-unwind" fn is_stat_consumption_stage_twopass(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        >= AOM_RC_SECOND_PASS as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn has_no_stats_stage(cpi: *const AV1_COMP) -> core::ffi::c_int {
    if !((*(*cpi).ppi).lap_enabled != 0
        || (*cpi).compressor_stage as core::ffi::c_int
            == ENCODE_STAGE as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"has_no_stats_stage\0"))
                .as_ptr(),
            b"encoder.h\0" as *const u8 as *const core::ffi::c_char,
            4176 as core::ffi::c_int,
            b"IMPLIES(!cpi->ppi->lap_enabled, cpi->compressor_stage == ENCODE_STAGE)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ((*cpi).oxcf.pass as core::ffi::c_uint
        == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && ((*(*cpi).ppi).lap_enabled == 0
            || (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int))
        as core::ffi::c_int;
}
static mut av1_ref_frame_flag_list: [uint8_t; 8] = [
    0 as core::ffi::c_int as uint8_t,
    AOM_LAST_FLAG as core::ffi::c_int as uint8_t,
    AOM_LAST2_FLAG as core::ffi::c_int as uint8_t,
    AOM_LAST3_FLAG as core::ffi::c_int as uint8_t,
    AOM_GOLD_FLAG as core::ffi::c_int as uint8_t,
    AOM_BWD_FLAG as core::ffi::c_int as uint8_t,
    AOM_ALT2_FLAG as core::ffi::c_int as uint8_t,
    AOM_ALT_FLAG as core::ffi::c_int as uint8_t,
];
static mut disable_order: [MV_REFERENCE_FRAME; 4] = [
    LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
];
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
unsafe extern "C-unwind" fn is_switchable_motion_mode_allowed(
    mut allow_warped_motion: bool,
    mut enable_obmc: bool,
) -> bool {
    return allow_warped_motion as core::ffi::c_int != 0
        || enable_obmc as core::ffi::c_int != 0;
}
pub const DELTA_Q_PERCEPTUAL_MODULATION: core::ffi::c_int = 1 as core::ffi::c_int;
static mut winner_mode_count_allowed: [core::ffi::c_int; 3] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn select_tx_mode(
    mut cm: *const AV1_COMMON,
    tx_size_search_method: TX_SIZE_SEARCH_METHOD,
) -> TX_MODE {
    if (*cm).features.coded_lossless {
        return ONLY_4X4 as core::ffi::c_int as TX_MODE;
    }
    if tx_size_search_method as core::ffi::c_int == USE_LARGESTALL as core::ffi::c_int {
        return TX_MODE_LARGEST as core::ffi::c_int as TX_MODE
    } else {
        if !(tx_size_search_method as core::ffi::c_int == USE_FULL_RD as core::ffi::c_int
            || tx_size_search_method as core::ffi::c_int
                == USE_FAST_RD as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"select_tx_mode\0"))
                    .as_ptr(),
                b"rdopt_utils.h\0" as *const u8 as *const core::ffi::c_char,
                458 as core::ffi::c_int,
                b"tx_size_search_method == USE_FULL_RD || tx_size_search_method == USE_FAST_RD\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return TX_MODE_SELECT as core::ffi::c_int as TX_MODE;
    };
}
#[inline]
unsafe extern "C-unwind" fn coded_to_superres_mi(
    mut mi_col: core::ffi::c_int,
    mut denom: core::ffi::c_int,
) -> core::ffi::c_int {
    return (mi_col * denom + SCALE_NUMERATOR / 2 as core::ffi::c_int) / SCALE_NUMERATOR;
}
#[inline]
unsafe extern "C-unwind" fn av1_encoder_get_relative_dist(
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(a >= 0 as core::ffi::c_int && b >= 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_encoder_get_relative_dist\0"))
                .as_ptr(),
            b"rdopt.h\0" as *const u8 as *const core::ffi::c_char,
            174 as core::ffi::c_int,
            b"a >= 0 && b >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return a - b;
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
unsafe extern "C-unwind" fn get_num_refs_to_disable(
    mut cpi: *const AV1_COMP,
    mut ref_frame_flags: *const core::ffi::c_int,
    mut ref_display_order_hint: *const core::ffi::c_uint,
    mut cur_frame_display_index: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut num_refs_to_disable: core::ffi::c_uint = 0 as core::ffi::c_uint;
    if (*cpi).sf.inter_sf.selective_ref_frame >= 3 as core::ffi::c_int {
        num_refs_to_disable = num_refs_to_disable.wrapping_add(1);
        if (*cpi).sf.inter_sf.selective_ref_frame >= 6 as core::ffi::c_int {
            num_refs_to_disable = num_refs_to_disable
                .wrapping_add(2 as core::ffi::c_uint);
        } else if (*cpi).sf.inter_sf.selective_ref_frame == 5 as core::ffi::c_int
            && *ref_frame_flags
                & av1_ref_frame_flag_list[LAST2_FRAME as core::ffi::c_int as usize]
                    as core::ffi::c_int != 0
        {
            let last2_frame_dist: core::ffi::c_int = av1_encoder_get_relative_dist(
                *ref_display_order_hint
                    .offset(
                        (LAST2_FRAME as core::ffi::c_int
                            - LAST_FRAME as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int,
                cur_frame_display_index as core::ffi::c_int,
            ) as core::ffi::c_int;
            if abs(last2_frame_dist) > 2 as core::ffi::c_int {
                num_refs_to_disable = num_refs_to_disable.wrapping_add(1);
            } else if is_stat_consumption_stage_twopass(cpi) != 0 {
                let this_frame_stats: *const FIRSTPASS_STATS = read_one_frame_stats(
                    &raw mut (*(*cpi).ppi).twopass,
                    cur_frame_display_index as core::ffi::c_int,
                ) as *const FIRSTPASS_STATS;
                let coded_error_per_mb: core::ffi::c_double = (*this_frame_stats)
                    .coded_error;
                if coded_error_per_mb < 100.0f64 {
                    num_refs_to_disable = num_refs_to_disable.wrapping_add(1);
                }
            }
        }
    }
    return num_refs_to_disable;
}
#[inline]
unsafe extern "C-unwind" fn get_max_allowed_ref_frames(
    mut cpi: *const AV1_COMP,
    mut ref_frame_flags: *const core::ffi::c_int,
    mut ref_display_order_hint: *const core::ffi::c_uint,
    mut cur_frame_display_index: core::ffi::c_uint,
) -> core::ffi::c_int {
    let max_reference_frames: core::ffi::c_uint = (*cpi)
        .oxcf
        .ref_frm_cfg
        .max_reference_frames;
    let num_refs_to_disable: core::ffi::c_uint = get_num_refs_to_disable(
        cpi,
        ref_frame_flags,
        ref_display_order_hint,
        cur_frame_display_index,
    ) as core::ffi::c_uint;
    let max_allowed_refs_for_given_speed: core::ffi::c_uint = (INTER_REFS_PER_FRAME
        as core::ffi::c_int as core::ffi::c_uint)
        .wrapping_sub(num_refs_to_disable);
    return (if max_allowed_refs_for_given_speed < max_reference_frames {
        max_allowed_refs_for_given_speed
    } else {
        max_reference_frames
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn enforce_max_ref_frames(
    mut cpi: *mut AV1_COMP,
    mut ref_frame_flags: *mut core::ffi::c_int,
    mut ref_display_order_hint: *const core::ffi::c_uint,
    mut cur_frame_display_index: core::ffi::c_uint,
) {
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    let mut total_valid_refs: core::ffi::c_int = 0 as core::ffi::c_int;
    ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        if *ref_frame_flags
            & av1_ref_frame_flag_list[ref_frame as usize] as core::ffi::c_int != 0
        {
            total_valid_refs += 1;
        }
        ref_frame += 1;
    }
    let max_allowed_refs: core::ffi::c_int = get_max_allowed_ref_frames(
        cpi,
        ref_frame_flags,
        ref_display_order_hint,
        cur_frame_display_index,
    ) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int && total_valid_refs > max_allowed_refs {
        let ref_frame_to_disable: MV_REFERENCE_FRAME = disable_order[i as usize];
        if !(*ref_frame_flags
            & av1_ref_frame_flag_list[ref_frame_to_disable as usize] as core::ffi::c_int
            == 0)
        {
            match ref_frame_to_disable as core::ffi::c_int {
                3 => {
                    *ref_frame_flags &= !(AOM_LAST3_FLAG as core::ffi::c_int);
                }
                2 => {
                    *ref_frame_flags &= !(AOM_LAST2_FLAG as core::ffi::c_int);
                }
                6 => {
                    *ref_frame_flags &= !(AOM_ALT2_FLAG as core::ffi::c_int);
                }
                5 => {
                    *ref_frame_flags &= !(AOM_GOLD_FLAG as core::ffi::c_int);
                }
                _ => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 23],
                                [core::ffi::c_char; 23],
                            >(*b"enforce_max_ref_frames\0"))
                                .as_ptr(),
                            b"encodeframe_utils.h\0" as *const u8
                                as *const core::ffi::c_char,
                            578 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
            }
            total_valid_refs -= 1;
        }
        i += 1;
    }
    if !(total_valid_refs <= max_allowed_refs) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"enforce_max_ref_frames\0"))
                .as_ptr(),
            b"encodeframe_utils.h\0" as *const u8 as *const core::ffi::c_char,
            582 as core::ffi::c_int,
            b"total_valid_refs <= max_allowed_refs\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
}
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
pub const BINS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const FIX_PREC_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_hist_bin_idx(
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
) -> core::ffi::c_int {
    let ratio: int32_t = dy as int32_t * ((1 as int32_t) << FIX_PREC_BITS)
        / dx as int32_t;
    static mut thresholds: [core::ffi::c_int; 32] = [
        -(1334015 as core::ffi::c_int),
        -(441798 as core::ffi::c_int),
        -(261605 as core::ffi::c_int),
        -(183158 as core::ffi::c_int),
        -(138560 as core::ffi::c_int),
        -(109331 as core::ffi::c_int),
        -(88359 as core::ffi::c_int),
        -(72303 as core::ffi::c_int),
        -(59392 as core::ffi::c_int),
        -(48579 as core::ffi::c_int),
        -(39272 as core::ffi::c_int),
        -(30982 as core::ffi::c_int),
        -(23445 as core::ffi::c_int),
        -(16400 as core::ffi::c_int),
        -(9715 as core::ffi::c_int),
        -(3194 as core::ffi::c_int),
        3227 as core::ffi::c_int,
        9748 as core::ffi::c_int,
        16433 as core::ffi::c_int,
        23478 as core::ffi::c_int,
        31015 as core::ffi::c_int,
        39305 as core::ffi::c_int,
        48611 as core::ffi::c_int,
        59425 as core::ffi::c_int,
        72336 as core::ffi::c_int,
        88392 as core::ffi::c_int,
        109364 as core::ffi::c_int,
        138593 as core::ffi::c_int,
        183191 as core::ffi::c_int,
        261638 as core::ffi::c_int,
        441831 as core::ffi::c_int,
        INT32_MAX,
    ];
    let mut lo_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut hi_idx: core::ffi::c_int = BINS - 1 as core::ffi::c_int;
    if ratio <= thresholds[7 as core::ffi::c_int as usize] {
        lo_idx = 0 as core::ffi::c_int;
        hi_idx = 7 as core::ffi::c_int;
    } else if ratio <= thresholds[15 as core::ffi::c_int as usize] {
        lo_idx = 8 as core::ffi::c_int;
        hi_idx = 15 as core::ffi::c_int;
    } else if ratio <= thresholds[23 as core::ffi::c_int as usize] {
        lo_idx = 16 as core::ffi::c_int;
        hi_idx = 23 as core::ffi::c_int;
    } else {
        lo_idx = 24 as core::ffi::c_int;
        hi_idx = 31 as core::ffi::c_int;
    }
    let mut idx: core::ffi::c_int = lo_idx;
    while idx <= hi_idx {
        if ratio <= thresholds[idx as usize] {
            return idx;
        }
        idx += 1;
    }
    if !(0 as core::ffi::c_int != 0
        && !(b"No valid histogram bin found!\0" as *const u8 as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"get_hist_bin_idx\0"))
                .as_ptr(),
            b"intra_mode_search_utils.h\0" as *const u8 as *const core::ffi::c_char,
            139 as core::ffi::c_int,
            b"0 && \"No valid histogram bin found!\"\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return BINS - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn lowbd_compute_gradient_info_sb(
    x: *mut MACROBLOCK,
    mut sb_size: BLOCK_SIZE,
    mut plane: PLANE_TYPE,
) {
    let grad_info_sb: *mut PixelLevelGradientInfo = ((*x).pixel_gradient_info)
        .offset((plane as core::ffi::c_int * MAX_SB_SQUARE) as isize);
    let mut src: *const uint8_t = (*x).plane[plane as usize].src.buf;
    let stride: core::ffi::c_int = (*x).plane[plane as usize].src.stride;
    let ss_x: core::ffi::c_int = (*x).e_mbd.plane[plane as usize].subsampling_x;
    let ss_y: core::ffi::c_int = (*x).e_mbd.plane[plane as usize].subsampling_y;
    let sb_height: core::ffi::c_int = block_size_high[sb_size as usize]
        as core::ffi::c_int >> ss_y;
    let sb_width: core::ffi::c_int = block_size_wide[sb_size as usize]
        as core::ffi::c_int >> ss_x;
    src = src.offset(stride as isize);
    let mut r: core::ffi::c_int = 1 as core::ffi::c_int;
    while r < sb_height - 1 as core::ffi::c_int {
        let mut c: core::ffi::c_int = 1 as core::ffi::c_int;
        while c < sb_width - 1 as core::ffi::c_int {
            let mut above: *const uint8_t = &*src.offset((c - stride) as isize)
                as *const uint8_t;
            let mut below: *const uint8_t = &*src.offset((c + stride) as isize)
                as *const uint8_t;
            let mut left: *const uint8_t = &*src
                .offset((c - 1 as core::ffi::c_int) as isize) as *const uint8_t;
            let mut right: *const uint8_t = &*src
                .offset((c + 1 as core::ffi::c_int) as isize) as *const uint8_t;
            let dx: core::ffi::c_int = *right.offset(-stride as isize)
                as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * *right.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *right.offset(stride as isize) as core::ffi::c_int
                - (*left.offset(-stride as isize) as core::ffi::c_int
                    + 2 as core::ffi::c_int
                        * *left.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                    + *left.offset(stride as isize) as core::ffi::c_int);
            let dy: core::ffi::c_int = *below.offset(-(1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * *below.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *below.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                - (*above.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
                    + 2 as core::ffi::c_int
                        * *above.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                    + *above.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int);
            (*grad_info_sb.offset((r * sb_width + c) as isize)).is_dx_zero = dx
                == 0 as core::ffi::c_int;
            (*grad_info_sb.offset((r * sb_width + c) as isize)).abs_dx_abs_dy_sum = (abs(
                dx,
            ) + abs(dy)) as uint16_t;
            (*grad_info_sb.offset((r * sb_width + c) as isize)).hist_bin_idx = (if dx
                != 0 as core::ffi::c_int
            {
                get_hist_bin_idx(dx, dy)
            } else {
                -(1 as core::ffi::c_int)
            }) as int8_t;
            c += 1;
        }
        src = src.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_compute_gradient_info_sb(
    x: *mut MACROBLOCK,
    mut sb_size: BLOCK_SIZE,
    mut plane: PLANE_TYPE,
) {
    let grad_info_sb: *mut PixelLevelGradientInfo = ((*x).pixel_gradient_info)
        .offset((plane as core::ffi::c_int * MAX_SB_SQUARE) as isize);
    let mut src: *const uint16_t = (((*x).plane[plane as usize].src.buf as uintptr_t)
        << 1 as core::ffi::c_int) as *mut uint16_t;
    let stride: core::ffi::c_int = (*x).plane[plane as usize].src.stride;
    let ss_x: core::ffi::c_int = (*x).e_mbd.plane[plane as usize].subsampling_x;
    let ss_y: core::ffi::c_int = (*x).e_mbd.plane[plane as usize].subsampling_y;
    let sb_height: core::ffi::c_int = block_size_high[sb_size as usize]
        as core::ffi::c_int >> ss_y;
    let sb_width: core::ffi::c_int = block_size_wide[sb_size as usize]
        as core::ffi::c_int >> ss_x;
    src = src.offset(stride as isize);
    let mut r: core::ffi::c_int = 1 as core::ffi::c_int;
    while r < sb_height - 1 as core::ffi::c_int {
        let mut c: core::ffi::c_int = 1 as core::ffi::c_int;
        while c < sb_width - 1 as core::ffi::c_int {
            let mut above: *const uint16_t = &*src.offset((c - stride) as isize)
                as *const uint16_t;
            let mut below: *const uint16_t = &*src.offset((c + stride) as isize)
                as *const uint16_t;
            let mut left: *const uint16_t = &*src
                .offset((c - 1 as core::ffi::c_int) as isize) as *const uint16_t;
            let mut right: *const uint16_t = &*src
                .offset((c + 1 as core::ffi::c_int) as isize) as *const uint16_t;
            let dx: core::ffi::c_int = *right.offset(-stride as isize)
                as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * *right.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *right.offset(stride as isize) as core::ffi::c_int
                - (*left.offset(-stride as isize) as core::ffi::c_int
                    + 2 as core::ffi::c_int
                        * *left.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                    + *left.offset(stride as isize) as core::ffi::c_int);
            let dy: core::ffi::c_int = *below.offset(-(1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * *below.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *below.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                - (*above.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
                    + 2 as core::ffi::c_int
                        * *above.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                    + *above.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int);
            (*grad_info_sb.offset((r * sb_width + c) as isize)).is_dx_zero = dx
                == 0 as core::ffi::c_int;
            (*grad_info_sb.offset((r * sb_width + c) as isize)).abs_dx_abs_dy_sum = (abs(
                dx,
            ) + abs(dy)) as uint16_t;
            (*grad_info_sb.offset((r * sb_width + c) as isize)).hist_bin_idx = (if dx
                != 0 as core::ffi::c_int
            {
                get_hist_bin_idx(dx, dy)
            } else {
                -(1 as core::ffi::c_int)
            }) as int8_t;
            c += 1;
        }
        src = src.offset(stride as isize);
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn compute_gradient_info_sb(
    x: *mut MACROBLOCK,
    mut sb_size: BLOCK_SIZE,
    mut plane: PLANE_TYPE,
) {
    if is_cur_buf_hbd(&raw mut (*x).e_mbd) != 0 {
        highbd_compute_gradient_info_sb(x, sb_size, plane);
        return;
    }
    lowbd_compute_gradient_info_sb(x, sb_size, plane);
}
#[inline]
unsafe extern "C-unwind" fn is_gradient_caching_for_hog_enabled(cpi: *const AV1_COMP) -> bool {
    let sf: *const SPEED_FEATURES = &(*cpi).sf;
    return frame_is_intra_only(&(*cpi).common) != 0
        && (*sf).rt_sf.use_nonrd_pick_mode == 0
        && (*sf).part_sf.partition_search_type as core::ffi::c_int
            == SEARCH_PARTITION as core::ffi::c_int
        && ((*sf).intra_sf.intra_pruning_with_hog != 0
            || (*sf).intra_sf.chroma_intra_pruning_with_hog != 0);
}
#[inline]
unsafe extern "C-unwind" fn produce_gradients_for_sb(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut sb_size: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    (*x).is_sb_gradient_cached[PLANE_TYPE_Y as core::ffi::c_int as usize] = false_0 != 0;
    (*x).is_sb_gradient_cached[PLANE_TYPE_UV as core::ffi::c_int as usize] = false_0
        != 0;
    if !is_gradient_caching_for_hog_enabled(cpi) {
        return;
    }
    let mut sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let num_planes: core::ffi::c_int = av1_num_planes(&raw mut (*cpi).common)
        as core::ffi::c_int;
    av1_setup_src_planes(x, (*cpi).source, mi_row, mi_col, num_planes, sb_size);
    if (*sf).intra_sf.intra_pruning_with_hog != 0 {
        compute_gradient_info_sb(
            x,
            sb_size,
            PLANE_TYPE_Y as core::ffi::c_int as PLANE_TYPE,
        );
        (*x).is_sb_gradient_cached[PLANE_TYPE_Y as core::ffi::c_int as usize] = true_0
            != 0;
    }
    if (*sf).intra_sf.chroma_intra_pruning_with_hog != 0
        && num_planes > 1 as core::ffi::c_int
    {
        compute_gradient_info_sb(
            x,
            sb_size,
            PLANE_TYPE_UV as core::ffi::c_int as PLANE_TYPE,
        );
        (*x).is_sb_gradient_cached[PLANE_TYPE_UV as core::ffi::c_int as usize] = true_0
            != 0;
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
unsafe extern "C-unwind" fn init_src_var_info_of_4x4_sub_blocks(
    cpi: *const AV1_COMP,
    mut src_var_info_of_4x4_sub_blocks: *mut Block4x4VarInfo,
    sb_size: BLOCK_SIZE,
) {
    if !is_src_var_for_4x4_sub_blocks_caching_enabled(cpi) {
        return;
    }
    let mi_count_in_sb: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int * mi_size_high[sb_size as usize] as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < mi_count_in_sb {
        (*src_var_info_of_4x4_sub_blocks.offset(i as isize)).var = -(1
            as core::ffi::c_int);
        (*src_var_info_of_4x4_sub_blocks.offset(i as isize)).log_var = -1.0f64;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn is_full_sb(
    mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut sb_size: BLOCK_SIZE,
) -> core::ffi::c_int {
    let sb_mi_wide: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int;
    let sb_mi_high: core::ffi::c_int = mi_size_high[sb_size as usize]
        as core::ffi::c_int;
    return (mi_row + sb_mi_high <= (*mi_params).mi_rows
        && mi_col + sb_mi_wide <= (*mi_params).mi_cols) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn use_auto_max_partition(
    cpi: *const AV1_COMP,
    mut sb_size: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"use_auto_max_partition\0"))
                .as_ptr(),
            b"partition_strategy.h\0" as *const u8 as *const core::ffi::c_char,
            189 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *const AV1_COMMON = &(*cpi).common;
    return (frame_is_intra_only(cm) == 0 && (*cpi).use_screen_content_tools == 0
        && (*cpi).sf.part_sf.auto_max_partition_based_on_simple_motion
            as core::ffi::c_int != NOT_IN_USE as core::ffi::c_int
        && sb_size as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        && is_full_sb(&(*cm).mi_params, mi_row, mi_col, sb_size) != 0
        && (*(*cpi).ppi).gf_group.update_type[(*cpi).gf_frame_index as usize]
            as core::ffi::c_int != OVERLAY_UPDATE as core::ffi::c_int
        && (*(*cpi).ppi).gf_group.update_type[(*cpi).gf_frame_index as usize]
            as core::ffi::c_int != INTNL_OVERLAY_UPDATE as core::ffi::c_int)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn dim_to_size(mut dim: core::ffi::c_int) -> BLOCK_SIZE {
    match dim {
        4 => return BLOCK_4X4,
        8 => return BLOCK_8X8,
        16 => return BLOCK_16X16,
        32 => return BLOCK_32X32,
        64 => return BLOCK_64X64,
        128 => return BLOCK_128X128,
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"dim_to_size\0"))
                        .as_ptr(),
                    b"partition_strategy.h\0" as *const u8 as *const core::ffi::c_char,
                    210 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            return BLOCK_4X4;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn set_max_min_partition_size(
    mut sb_enc: *mut SuperBlockEnc,
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut sf: *const SPEED_FEATURES,
    mut sb_size: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    (*sb_enc).max_partition_size = (if ((*sf).part_sf.default_max_partition_size
        as core::ffi::c_int)
        < dim_to_size((*cpi).oxcf.part_cfg.max_partition_size as core::ffi::c_int)
            as core::ffi::c_int
    {
        (*sf).part_sf.default_max_partition_size as core::ffi::c_int
    } else {
        dim_to_size((*cpi).oxcf.part_cfg.max_partition_size as core::ffi::c_int)
            as core::ffi::c_int
    }) as BLOCK_SIZE;
    (*sb_enc).min_partition_size = (if (*sf).part_sf.default_min_partition_size
        as core::ffi::c_int
        > dim_to_size((*cpi).oxcf.part_cfg.min_partition_size as core::ffi::c_int)
            as core::ffi::c_int
    {
        (*sf).part_sf.default_min_partition_size as core::ffi::c_int
    } else {
        dim_to_size((*cpi).oxcf.part_cfg.min_partition_size as core::ffi::c_int)
            as core::ffi::c_int
    }) as BLOCK_SIZE;
    (*sb_enc).max_partition_size = (if ((*sb_enc).max_partition_size as core::ffi::c_int)
        < (*(*cm).seq_params).sb_size as core::ffi::c_int
    {
        (*sb_enc).max_partition_size as core::ffi::c_int
    } else {
        (*(*cm).seq_params).sb_size as core::ffi::c_int
    }) as BLOCK_SIZE;
    (*sb_enc).min_partition_size = (if ((*sb_enc).min_partition_size as core::ffi::c_int)
        < (*(*cm).seq_params).sb_size as core::ffi::c_int
    {
        (*sb_enc).min_partition_size as core::ffi::c_int
    } else {
        (*(*cm).seq_params).sb_size as core::ffi::c_int
    }) as BLOCK_SIZE;
    if use_auto_max_partition(cpi, sb_size, mi_row, mi_col) != 0 {
        let mut features: [core::ffi::c_float; 13] = [
            0.0f32,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        av1_get_max_min_partition_features(
            cpi,
            x,
            mi_row,
            mi_col,
            features.as_mut_ptr(),
        );
        (*sb_enc).max_partition_size = (if (if (av1_predict_max_partition(
            cpi,
            x,
            features.as_mut_ptr(),
        ) as core::ffi::c_int) < (*sb_enc).max_partition_size as core::ffi::c_int
        {
            av1_predict_max_partition(cpi, x, features.as_mut_ptr()) as core::ffi::c_int
        } else {
            (*sb_enc).max_partition_size as core::ffi::c_int
        }) > (*sb_enc).min_partition_size as core::ffi::c_int
        {
            if (av1_predict_max_partition(cpi, x, features.as_mut_ptr())
                as core::ffi::c_int) < (*sb_enc).max_partition_size as core::ffi::c_int
            {
                av1_predict_max_partition(cpi, x, features.as_mut_ptr())
                    as core::ffi::c_int
            } else {
                (*sb_enc).max_partition_size as core::ffi::c_int
            }
        } else {
            (*sb_enc).min_partition_size as core::ffi::c_int
        }) as BLOCK_SIZE;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_cb_offsets(
    mut cb_offset: *mut uint16_t,
    cb_offset_y: uint16_t,
    cb_offset_uv: uint16_t,
) {
    *cb_offset.offset(PLANE_TYPE_Y as core::ffi::c_int as isize) = cb_offset_y;
    *cb_offset.offset(PLANE_TYPE_UV as core::ffi::c_int as isize) = cb_offset_uv;
}
static mut AV1_VAR_OFFS: [uint8_t; 128] = [
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
];
static mut AV1_HIGH_VAR_OFFS_8: [uint16_t; 128] = [
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
    128 as core::ffi::c_int as uint16_t,
];
static mut AV1_HIGH_VAR_OFFS_10: [uint16_t; 128] = [
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 4 as core::ffi::c_int) as uint16_t,
];
static mut AV1_HIGH_VAR_OFFS_12: [uint16_t; 128] = [
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
    (128 as core::ffi::c_int * 16 as core::ffi::c_int) as uint16_t,
];
unsafe extern "C-unwind" fn get_var_offs(
    mut use_hbd: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> *const uint8_t {
    if use_hbd != 0 {
        if !(bd == 8 as core::ffi::c_int || bd == 10 as core::ffi::c_int
            || bd == 12 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"get_var_offs\0"))
                    .as_ptr(),
                b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                159 as core::ffi::c_int,
                b"bd == 8 || bd == 10 || bd == 12\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let off_index: core::ffi::c_int = bd - 8 as core::ffi::c_int
            >> 1 as core::ffi::c_int;
        static mut high_var_offs: [*const uint16_t; 3] = unsafe {
            [
                core::ptr::addr_of!(AV1_HIGH_VAR_OFFS_8).cast::<uint16_t>(),
                core::ptr::addr_of!(AV1_HIGH_VAR_OFFS_10).cast::<uint16_t>(),
                core::ptr::addr_of!(AV1_HIGH_VAR_OFFS_12).cast::<uint16_t>(),
            ]
        };
        return (high_var_offs[off_index as usize] as uintptr_t >> 1 as core::ffi::c_int)
            as *mut uint8_t;
    }
    if !(bd == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"get_var_offs\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            171 as core::ffi::c_int,
            b"bd == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return core::ptr::addr_of!(AV1_VAR_OFFS).cast::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_rtc_counters(x: *mut MACROBLOCK) {
    av1_init_cyclic_refresh_counters(x);
    (*x).cnt_zeromv = 0 as core::ffi::c_int;
    (*x).sb_col_scroll = 0 as core::ffi::c_int;
    (*x).sb_row_scroll = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_accumulate_rtc_counters(
    mut cpi: *mut AV1_COMP,
    x: *const MACROBLOCK,
) {
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        == CYCLIC_REFRESH_AQ as core::ffi::c_int
    {
        av1_accumulate_cyclic_refresh_counters((*cpi).cyclic_refresh, x);
    }
    (*cpi).rc.cnt_zeromv += (*x).cnt_zeromv;
    (*cpi).rc.num_col_blscroll_last_tl0 += (*x).sb_col_scroll;
    (*cpi).rc.num_row_blscroll_last_tl0 += (*x).sb_row_scroll;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_perpixel_variance(
    mut cpi: *const AV1_COMP,
    mut xd: *const MACROBLOCKD,
    mut ref_0: *const buf_2d,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut use_hbd: core::ffi::c_int,
) -> core::ffi::c_uint {
    let subsampling_x: core::ffi::c_int = (*xd).plane[plane as usize].subsampling_x;
    let subsampling_y: core::ffi::c_int = (*xd).plane[plane as usize].subsampling_y;
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        subsampling_x,
        subsampling_y,
    ) as BLOCK_SIZE;
    let mut sse: core::ffi::c_uint = 0;
    let var: core::ffi::c_uint = ((*(*cpi).ppi).fn_ptr[plane_bsize as usize].vf)
        .expect(
            "non-null function pointer",
        )(
        (*ref_0).buf,
        (*ref_0).stride,
        get_var_offs(use_hbd, (*xd).bd),
        0 as core::ffi::c_int,
        &raw mut sse,
    ) as core::ffi::c_uint;
    return var
        .wrapping_add(
            ((1 as core::ffi::c_int)
                << num_pels_log2_lookup[plane_bsize as usize] as core::ffi::c_int
                >> 1 as core::ffi::c_int) as core::ffi::c_uint,
        ) >> num_pels_log2_lookup[plane_bsize as usize] as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_perpixel_variance_facade(
    mut cpi: *const AV1_COMP,
    mut xd: *const MACROBLOCKD,
    mut ref_0: *const buf_2d,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_uint {
    let use_hbd: core::ffi::c_int = is_cur_buf_hbd(xd) as core::ffi::c_int;
    return av1_get_perpixel_variance(cpi, xd, ref_0, bsize, plane, use_hbd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_src_planes(
    mut x: *mut MACROBLOCK,
    mut src: *const YV12_BUFFER_CONFIG,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    num_planes: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    (*x).e_mbd.cur_buf = src;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i
        < (if num_planes < 3 as core::ffi::c_int {
            num_planes
        } else {
            3 as core::ffi::c_int
        })
    {
        let is_uv: core::ffi::c_int = (i > 0 as core::ffi::c_int) as core::ffi::c_int;
        setup_pred_plane(
            &raw mut (*((*x).plane).as_mut_ptr().offset(i as isize)).src,
            bsize,
            (*src).c2rust_unnamed_4.buffers[i as usize],
            (*src).c2rust_unnamed_1.crop_widths[is_uv as usize],
            (*src).c2rust_unnamed_2.crop_heights[is_uv as usize],
            (*src).c2rust_unnamed_3.strides[is_uv as usize],
            mi_row,
            mi_col,
            0 as *const scale_factors,
            (*x).e_mbd.plane[i as usize].subsampling_x,
            (*x).e_mbd.plane[i as usize].subsampling_y,
        );
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_delta_q_nonrd(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    x: *mut MACROBLOCK,
    tile_info: *const TileInfo,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    if ((*delta_q_info).delta_q_present_flag == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"setup_delta_q_nonrd\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            252 as core::ffi::c_int,
            b"delta_q_info->delta_q_present_flag\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    av1_setup_src_planes(x, (*cpi).source, mi_row, mi_col, num_planes, sb_size);
    let delta_q_res: core::ffi::c_int = (*delta_q_info).delta_q_res;
    let mut current_qindex: core::ffi::c_int = (*cm).quant_params.base_qindex;
    if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_VARIANCE_BOOST as core::ffi::c_int
    {
        current_qindex = av1_get_sbq_variance_boost(cpi, x);
    }
    (*x).rdmult_cur_qindex = current_qindex;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    current_qindex = av1_adjust_q_from_delta_q_res(
        delta_q_res,
        (*xd).current_base_qindex,
        current_qindex,
    );
    (*x).delta_qindex = current_qindex - (*cm).quant_params.base_qindex;
    (*x).rdmult_delta_qindex = (*x).delta_qindex;
    av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).current_qindex = current_qindex;
    av1_init_plane_quantizers(
        cpi,
        x,
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
            as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    (*td).deltaq_used
        |= ((*x).delta_qindex != 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn setup_delta_q(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    x: *mut MACROBLOCK,
    tile_info: *const TileInfo,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    av1_setup_src_planes(x, (*cpi).source, mi_row, mi_col, num_planes, sb_size);
    let delta_q_res: core::ffi::c_int = (*delta_q_info).delta_q_res;
    let mut current_qindex: core::ffi::c_int = (*cm).quant_params.base_qindex;
    let sb_row: core::ffi::c_int = mi_row >> (*(*cm).seq_params).mib_size_log2;
    let sb_col: core::ffi::c_int = mi_col >> (*(*cm).seq_params).mib_size_log2;
    let sb_cols: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*(*cm).seq_params).mib_size_log2;
    let sb_index: core::ffi::c_int = sb_row * sb_cols + sb_col;
    if (*cpi).use_ducky_encode != 0
        && (*cpi).ducky_encode_info.frame_info.qp_mode as core::ffi::c_int
            == DUCKY_ENCODE_FRAME_MODE_QINDEX as core::ffi::c_int
    {
        current_qindex = *((*cpi).ducky_encode_info.frame_info.superblock_encode_qindex)
            .offset(sb_index as isize);
    } else if (*cpi).ext_ratectrl.ready != 0
        && (*cpi).ext_ratectrl.funcs.rc_type as core::ffi::c_uint
            & AOM_RC_QP as core::ffi::c_int as core::ffi::c_uint
            != 0 as core::ffi::c_uint
        && ((*cpi).ext_ratectrl.funcs.get_encodeframe_decision).is_some()
        && !((*cpi).ext_ratectrl.sb_params_list).is_null()
    {
        if (*cpi).ext_ratectrl.use_delta_q != 0 {
            current_qindex = av1_get_q_for_deltaq_objective(
                cpi,
                td,
                0 as *mut int64_t,
                sb_size,
                mi_row,
                mi_col,
            );
            let q_index: core::ffi::c_int = (*((*cpi).ext_ratectrl.sb_params_list)
                .offset(sb_index as isize))
                .q_index;
            if q_index != AOM_DEFAULT_Q {
                current_qindex = q_index;
            }
        }
    } else if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_PERCEPTUAL as core::ffi::c_int
    {
        if DELTA_Q_PERCEPTUAL_MODULATION == 1 as core::ffi::c_int {
            let block_wavelet_energy_level: core::ffi::c_int = av1_block_wavelet_energy_level(
                cpi,
                x,
                sb_size,
            ) as core::ffi::c_int;
            (*x).sb_energy_level = block_wavelet_energy_level;
            current_qindex = av1_compute_q_from_energy_level_deltaq_mode(
                cpi,
                block_wavelet_energy_level,
            );
        } else {
            let block_var_level: core::ffi::c_int = av1_log_block_var(cpi, x, sb_size)
                as core::ffi::c_int;
            (*x).sb_energy_level = block_var_level;
            current_qindex = av1_compute_q_from_energy_level_deltaq_mode(
                cpi,
                block_var_level,
            );
        }
    } else if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_OBJECTIVE as core::ffi::c_int
        && (*cpi).oxcf.algo_cfg.enable_tpl_model as core::ffi::c_int != 0
    {
        current_qindex = av1_get_q_for_deltaq_objective(
            cpi,
            td,
            0 as *mut int64_t,
            sb_size,
            mi_row,
            mi_col,
        );
    } else if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_PERCEPTUAL_AI as core::ffi::c_int
    {
        current_qindex = av1_get_sbq_perceptual_ai(cpi, sb_size, mi_row, mi_col);
    } else if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_USER_RATING_BASED as core::ffi::c_int
    {
        current_qindex = av1_get_sbq_user_rating_based(cpi, mi_row, mi_col);
    } else if (*cpi).oxcf.q_cfg.enable_hdr_deltaq {
        current_qindex = av1_get_q_for_hdr(cpi, x, sb_size, mi_row, mi_col);
    } else if (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
        == DELTA_Q_VARIANCE_BOOST as core::ffi::c_int
    {
        current_qindex = av1_get_sbq_variance_boost(cpi, x);
    }
    (*x).rdmult_cur_qindex = current_qindex;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let adjusted_qindex: core::ffi::c_int = av1_adjust_q_from_delta_q_res(
        delta_q_res,
        (*xd).current_base_qindex,
        current_qindex,
    ) as core::ffi::c_int;
    if (*cpi).use_ducky_encode != 0 {
        if !(adjusted_qindex == current_qindex) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"setup_delta_q\0"))
                    .as_ptr(),
                b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                366 as core::ffi::c_int,
                b"adjusted_qindex == current_qindex\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    current_qindex = adjusted_qindex;
    (*x).delta_qindex = if (*cm).delta_q_info.delta_q_present_flag != 0 {
        current_qindex - (*cm).quant_params.base_qindex
    } else {
        0 as core::ffi::c_int
    };
    (*x).rdmult_delta_qindex = current_qindex - (*cm).quant_params.base_qindex;
    av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).current_qindex = if (*cm)
        .delta_q_info
        .delta_q_present_flag != 0
    {
        current_qindex
    } else {
        (*cm).quant_params.base_qindex
    };
    av1_init_plane_quantizers(
        cpi,
        x,
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
            as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    (*td).deltaq_used
        |= ((*x).delta_qindex != 0 as core::ffi::c_int) as core::ffi::c_int;
    if (*cpi).oxcf.tool_cfg.enable_deltalf_mode as core::ffi::c_int != 0
        && (*cm).delta_q_info.delta_q_present_flag != 0
    {
        let delta_lf_res: core::ffi::c_int = (*delta_q_info).delta_lf_res;
        let lfmask: core::ffi::c_int = !(delta_lf_res - 1 as core::ffi::c_int);
        let delta_lf_from_base: core::ffi::c_int = (*x).delta_qindex
            / 4 as core::ffi::c_int + delta_lf_res / 2 as core::ffi::c_int & lfmask;
        let delta_lf: int8_t = clamp(
            delta_lf_from_base,
            -MAX_LOOP_FILTER,
            MAX_LOOP_FILTER,
        ) as int8_t;
        let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm)
            > 1 as core::ffi::c_int
        {
            FRAME_LF_COUNT
        } else {
            FRAME_LF_COUNT - 2 as core::ffi::c_int
        };
        let mib_size: core::ffi::c_int = (*(*cm).seq_params).mib_size;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j
            < (if mib_size < (*mi_params).mi_rows - mi_row {
                mib_size
            } else {
                (*mi_params).mi_rows - mi_row
            })
        {
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k
                < (if mib_size < (*mi_params).mi_cols - mi_col {
                    mib_size
                } else {
                    (*mi_params).mi_cols - mi_col
                })
            {
                let grid_idx: core::ffi::c_int = get_mi_grid_idx(
                    mi_params,
                    mi_row + j,
                    mi_col + k,
                ) as core::ffi::c_int;
                (*((*mi_params).mi_alloc).offset(grid_idx as isize))
                    .delta_lf_from_base = delta_lf;
                let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
                while lf_id < frame_lf_count {
                    (*((*mi_params).mi_alloc).offset(grid_idx as isize))
                        .delta_lf[lf_id as usize] = delta_lf;
                    lf_id += 1;
                }
                k += 1;
            }
            j += 1;
        }
    }
}
unsafe extern "C-unwind" fn init_ref_frame_space(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mut x: *mut MACROBLOCK = &raw mut (*td).mb;
    let frame_idx: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    let block_mis_log2: uint8_t = (*tpl_data).tpl_stats_block_mis_log2;
    memset(
        &raw mut (*x).tpl_keep_ref_frame as *mut [uint8_t; 8] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
    if av1_tpl_stats_ready(tpl_data, frame_idx) == 0 {
        return;
    }
    if is_frame_tpl_eligible(gf_group, (*cpi).gf_frame_index as uint8_t) == 0 {
        return;
    }
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int != NO_AQ as core::ffi::c_int {
        return;
    }
    let is_overlay: core::ffi::c_int = ((*(*cpi).ppi)
        .gf_group
        .update_type[frame_idx as usize] as core::ffi::c_int
        == OVERLAY_UPDATE as core::ffi::c_int) as core::ffi::c_int;
    if is_overlay != 0 {
        memset(
            ((*x).tpl_keep_ref_frame).as_mut_ptr() as *mut core::ffi::c_void,
            1 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
        );
        return;
    }
    let mut tpl_frame: *mut TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(frame_idx as isize) as *mut TplDepFrame;
    let mut tpl_stats: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let tpl_stride: core::ffi::c_int = (*tpl_frame).stride;
    let mut inter_cost: [int64_t; 7] = [
        0 as core::ffi::c_int as int64_t,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << block_mis_log2 as core::ffi::c_int;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let mi_row_end: core::ffi::c_int = if mi_size_high[sb_size as usize]
        as core::ffi::c_int + mi_row < (*mi_params).mi_rows
    {
        mi_size_high[sb_size as usize] as core::ffi::c_int + mi_row
    } else {
        (*mi_params).mi_rows
    };
    let mi_cols_sr: core::ffi::c_int = av1_pixels_to_mi((*cm).superres_upscaled_width)
        as core::ffi::c_int;
    let mi_col_sr: core::ffi::c_int = coded_to_superres_mi(
        mi_col,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mi_col_end_sr: core::ffi::c_int = if coded_to_superres_mi(
        mi_col + mi_size_wide[sb_size as usize] as core::ffi::c_int,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) < mi_cols_sr
    {
        coded_to_superres_mi(
            mi_col + mi_size_wide[sb_size as usize] as core::ffi::c_int,
            (*cm).superres_scale_denominator as core::ffi::c_int,
        ) as core::ffi::c_int
    } else {
        mi_cols_sr
    };
    let row_step: core::ffi::c_int = step;
    let col_step_sr: core::ffi::c_int = coded_to_superres_mi(
        step,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut row: core::ffi::c_int = mi_row;
    while row < mi_row_end {
        let mut col: core::ffi::c_int = mi_col_sr;
        while col < mi_col_end_sr {
            let mut this_stats: *const TplDepStats = &raw mut *tpl_stats
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
            let mut tpl_pred_error: [int64_t; 7] = [
                0 as core::ffi::c_int as int64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            let mut best_inter_cost: int64_t = (*this_stats)
                .pred_error[0 as core::ffi::c_int as usize];
            let mut best_rf_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut idx: core::ffi::c_int = 1 as core::ffi::c_int;
            while idx < INTER_REFS_PER_FRAME as core::ffi::c_int {
                if (*this_stats).pred_error[idx as usize] < best_inter_cost
                    && (*this_stats).pred_error[idx as usize] != 0 as int64_t
                {
                    best_inter_cost = (*this_stats).pred_error[idx as usize];
                    best_rf_idx = idx;
                }
                idx += 1;
            }
            tpl_pred_error[best_rf_idx as usize] = (*this_stats)
                .pred_error[best_rf_idx as usize]
                - (*this_stats)
                    .pred_error[(LAST_FRAME as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize];
            let mut rf_idx: core::ffi::c_int = 1 as core::ffi::c_int;
            while rf_idx < INTER_REFS_PER_FRAME as core::ffi::c_int {
                inter_cost[rf_idx as usize] += tpl_pred_error[rf_idx as usize];
                rf_idx += 1;
            }
            col += col_step_sr;
        }
        row += row_step;
    }
    let mut rank_index: [core::ffi::c_int; 6] = [0; 6];
    let mut idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx_0 < INTER_REFS_PER_FRAME as core::ffi::c_int - 1 as core::ffi::c_int {
        rank_index[idx_0 as usize] = idx_0 + 1 as core::ffi::c_int;
        let mut i: core::ffi::c_int = idx_0;
        while i > 0 as core::ffi::c_int {
            if inter_cost[rank_index[(i - 1 as core::ffi::c_int) as usize] as usize]
                > inter_cost[rank_index[i as usize] as usize]
            {
                let tmp: core::ffi::c_int = rank_index[(i - 1 as core::ffi::c_int)
                    as usize];
                rank_index[(i - 1 as core::ffi::c_int) as usize] = rank_index[i
                    as usize];
                rank_index[i as usize] = tmp;
            }
            i -= 1;
        }
        idx_0 += 1;
    }
    (*x).tpl_keep_ref_frame[INTRA_FRAME as core::ffi::c_int as usize] = 1 as uint8_t;
    (*x).tpl_keep_ref_frame[LAST_FRAME as core::ffi::c_int as usize] = 1 as uint8_t;
    let mut cutoff_ref: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut idx_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx_1 < INTER_REFS_PER_FRAME as core::ffi::c_int - 1 as core::ffi::c_int {
        (*x)
            .tpl_keep_ref_frame[(rank_index[idx_1 as usize]
            + LAST_FRAME as core::ffi::c_int) as usize] = 1 as uint8_t;
        if idx_1 > 2 as core::ffi::c_int {
            if cutoff_ref == 0 {
                if llabs(inter_cost[rank_index[idx_1 as usize] as usize])
                    < llabs(
                        inter_cost[rank_index[(idx_1 - 1 as core::ffi::c_int) as usize]
                            as usize],
                    ) / 8 as core::ffi::c_longlong
                    || inter_cost[rank_index[idx_1 as usize] as usize] == 0 as int64_t
                {
                    cutoff_ref = 1 as core::ffi::c_int;
                }
            }
            if cutoff_ref != 0 {
                (*x)
                    .tpl_keep_ref_frame[(rank_index[idx_1 as usize]
                    + LAST_FRAME as core::ffi::c_int) as usize] = 0 as uint8_t;
            }
        }
        idx_1 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn adjust_rdmult_tpl_model(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let sb_size: BLOCK_SIZE = (*(*cpi).common.seq_params).sb_size;
    let orig_rdmult: core::ffi::c_int = (*cpi).rd.RDMULT;
    if !(!((*(*cpi).ppi).gf_group.size > 0 as core::ffi::c_int)
        || ((*cpi).gf_frame_index as core::ffi::c_int) < (*(*cpi).ppi).gf_group.size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"adjust_rdmult_tpl_model\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            517 as core::ffi::c_int,
            b"IMPLIES(cpi->ppi->gf_group.size > 0, cpi->gf_frame_index < cpi->ppi->gf_group.size)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let gf_group_index: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    if (*cpi).oxcf.algo_cfg.enable_tpl_model as core::ffi::c_int != 0
        && (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int == NO_AQ as core::ffi::c_int
        && (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
            == NO_DELTA_Q as core::ffi::c_int && gf_group_index > 0 as core::ffi::c_int
        && (*(*cpi).ppi).gf_group.update_type[gf_group_index as usize]
            as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
    {
        let dr: core::ffi::c_int = av1_get_rdmult_delta(
            cpi,
            sb_size,
            mi_row,
            mi_col,
            orig_rdmult,
        ) as core::ffi::c_int;
        (*x).rdmult = dr;
    }
}
pub const AVG_CDF_WEIGHT_LEFT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const AVG_CDF_WEIGHT_TOP_RIGHT: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn encode_nonrd_sb(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    seg_skip: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
        .offset(get_mi_grid_idx(&raw mut (*cm).mi_params, mi_row, mi_col) as isize);
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let pc_root: *mut PC_TREE = (*td).pc_root;
    if (*cm).delta_q_info.delta_q_present_flag != 0 {
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        setup_delta_q_nonrd(cpi, td, x, tile_info, mi_row, mi_col, num_planes);
    }
    if (*sf).part_sf.partition_search_type as core::ffi::c_int
        == FIXED_PARTITION as core::ffi::c_int || seg_skip != 0
        || (*sf).rt_sf.use_fast_fixed_part != 0
            && (*x).sb_force_fixed_part == 1 as core::ffi::c_int
            && (frame_is_intra_only(cm) == 0
                && ((*(*cpi).ppi).use_svc == 0
                    || (*((*cpi).svc.layer_context)
                        .offset((*cpi).svc.temporal_layer_id as isize))
                        .is_key_frame == 0))
    {
        av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
        let mut bsize_select: BLOCK_SIZE = (*sf).part_sf.fixed_partition_size;
        if (*sf).rt_sf.use_fast_fixed_part != 0
            && ((*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint)
                < kLowSad as core::ffi::c_int as core::ffi::c_uint
        {
            bsize_select = (*(*cm).seq_params).sb_size;
        }
        if (*cpi).sf.rt_sf.skip_encoding_non_reference_slide_change != 0
            && (*cpi).rc.high_source_sad != 0
            && (*(*cpi).ppi).rtc_ref.non_reference_frame != 0
        {
            bsize_select = (*(*cm).seq_params).sb_size;
            (*x).force_zeromv_skip_for_sb = 1 as core::ffi::c_int;
        }
        let bsize: BLOCK_SIZE = (if seg_skip != 0 {
            sb_size as core::ffi::c_int
        } else {
            bsize_select as core::ffi::c_int
        }) as BLOCK_SIZE;
        if (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            > kZeroSad as core::ffi::c_int as core::ffi::c_uint
        {
            (*x).force_color_check_block_level = 1 as core::ffi::c_int;
        }
        av1_set_fixed_partitioning(cpi, tile_info, mi, mi_row, mi_col, bsize);
    } else if (*sf).part_sf.partition_search_type as core::ffi::c_int
        == VAR_BASED_PARTITION as core::ffi::c_int
    {
        av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
        av1_choose_var_based_partitioning(cpi, tile_info, td, x, mi_row, mi_col);
    }
    if !((*sf).part_sf.partition_search_type as core::ffi::c_int
        == FIXED_PARTITION as core::ffi::c_int || seg_skip != 0
        || (*sf).part_sf.partition_search_type as core::ffi::c_int
            == VAR_BASED_PARTITION as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"encode_nonrd_sb\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            646 as core::ffi::c_int,
            b"sf->part_sf.partition_search_type == FIXED_PARTITION || seg_skip || sf->part_sf.partition_search_type == VAR_BASED_PARTITION\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    set_cb_offsets(((*td).mb.cb_offset).as_mut_ptr(), 0 as uint16_t, 0 as uint16_t);
    if (*sf).rt_sf.skip_cdef_sb != 0 {
        let block64_in_sb: core::ffi::c_int = if sb_size as core::ffi::c_int
            == BLOCK_128X128 as core::ffi::c_int
        {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
        while r < block64_in_sb {
            let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
            while c < block64_in_sb {
                let idx_in_sb: core::ffi::c_int = r * MI_SIZE_64X64
                    * (*cm).mi_params.mi_stride + c * MI_SIZE_64X64;
                if !(*mi.offset(idx_in_sb as isize)).is_null() {
                    let fresh1 = &raw mut **mi.offset(idx_in_sb as isize);
                    (*fresh1).set_cdef_strength(1 as int8_t as int8_t);
                }
                c += 1;
            }
            r += 1;
        }
    }
    av1_nonrd_use_partition(
        cpi,
        td,
        tile_data,
        mi,
        tp,
        mi_row,
        mi_col,
        sb_size,
        pc_root,
    );
}
#[inline]
unsafe extern "C-unwind" fn init_encode_rd_sb(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *const TileDataEnc,
    mut sms_root: *mut SIMPLE_MOTION_DATA_TREE,
    mut rd_cost: *mut RD_STATS,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut gather_tpl_data: core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut tile_info: *const TileInfo = &(*tile_data).tile_info;
    let mut x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let use_simple_motion_search: core::ffi::c_int = (((*sf)
        .part_sf
        .simple_motion_search_split != 0
        || (*sf).part_sf.simple_motion_search_prune_rect != 0
        || (*sf).part_sf.simple_motion_search_early_term_none != 0
        || (*sf).part_sf.ml_early_term_after_part_split_level != 0)
        && frame_is_intra_only(cm) == 0) as core::ffi::c_int;
    if use_simple_motion_search != 0 {
        av1_init_simple_motion_search_mvs_for_sb(
            cpi,
            tile_info,
            x,
            sms_root,
            mi_row,
            mi_col,
        );
    }
    if !(has_no_stats_stage(cpi) != 0
        && (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int
        && (*cpi).oxcf.gf_cfg.lag_in_frames == 0 as core::ffi::c_int)
    {
        init_ref_frame_space(cpi, td, mi_row, mi_col);
        (*x).sb_energy_level = 0 as core::ffi::c_int;
        (*x).part_search_info.cnn_output_valid = 0 as core::ffi::c_int;
        if gather_tpl_data != 0 {
            if (*cpi).cb_delta_rdmult_enabled != 0 {
                let num_planes: core::ffi::c_int = av1_num_planes(cm)
                    as core::ffi::c_int;
                let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
                setup_delta_q(cpi, td, x, tile_info, mi_row, mi_col, num_planes);
                av1_tpl_rdmult_setup_sb(
                    cpi as *mut AV1_COMP,
                    x,
                    sb_size,
                    mi_row,
                    mi_col,
                );
            }
            if (*cpi).oxcf.algo_cfg.enable_tpl_model as core::ffi::c_int != 0
                && 0 as core::ffi::c_int != 0
            {
                adjust_rdmult_tpl_model(cpi, x, mi_row, mi_col);
            }
        }
    }
    (*x).reuse_inter_pred = false_0 != 0;
    (*x).txfm_search_params.mode_eval_type = DEFAULT_EVAL as core::ffi::c_int;
    reset_mb_rd_record((*x).txfm_search_info.mb_rd_record);
    memset(
        &raw mut (*x).picked_ref_frames_mask as *mut [core::ffi::c_int; 1024]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 1024]>() as size_t,
    );
    av1_invalid_rd_stats(rd_cost);
}
unsafe extern "C-unwind" fn sb_qp_sweep_init_quantizers(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *const TileDataEnc,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut rd_cost: *mut RD_STATS,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut delta_qp_ofs: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let mut tile_info: *const TileInfo = &(*tile_data).tile_info;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    if ((*delta_q_info).delta_q_present_flag == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"sb_qp_sweep_init_quantizers\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            741 as core::ffi::c_int,
            b"delta_q_info->delta_q_present_flag\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let delta_q_res: core::ffi::c_int = (*delta_q_info).delta_q_res;
    let mut sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let use_simple_motion_search: core::ffi::c_int = (((*sf)
        .part_sf
        .simple_motion_search_split != 0
        || (*sf).part_sf.simple_motion_search_prune_rect != 0
        || (*sf).part_sf.simple_motion_search_early_term_none != 0
        || (*sf).part_sf.ml_early_term_after_part_split_level != 0)
        && frame_is_intra_only(cm) == 0) as core::ffi::c_int;
    if use_simple_motion_search != 0 {
        av1_init_simple_motion_search_mvs_for_sb(
            cpi,
            tile_info,
            x,
            sms_tree,
            mi_row,
            mi_col,
        );
    }
    let mut current_qindex: core::ffi::c_int = (*x).rdmult_cur_qindex + delta_qp_ofs;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    current_qindex = av1_adjust_q_from_delta_q_res(
        delta_q_res,
        (*xd).current_base_qindex,
        current_qindex,
    );
    (*x).delta_qindex = current_qindex - (*cm).quant_params.base_qindex;
    av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).current_qindex = current_qindex;
    av1_init_plane_quantizers(
        cpi,
        x,
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
            as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    (*td).deltaq_used
        |= ((*x).delta_qindex != 0 as core::ffi::c_int) as core::ffi::c_int;
    if (*cpi).oxcf.tool_cfg.enable_deltalf_mode {
        let delta_lf_res: core::ffi::c_int = (*delta_q_info).delta_lf_res;
        let lfmask: core::ffi::c_int = !(delta_lf_res - 1 as core::ffi::c_int);
        let delta_lf_from_base: core::ffi::c_int = (*x).delta_qindex
            / 4 as core::ffi::c_int + delta_lf_res / 2 as core::ffi::c_int & lfmask;
        let delta_lf: int8_t = clamp(
            delta_lf_from_base,
            -MAX_LOOP_FILTER,
            MAX_LOOP_FILTER,
        ) as int8_t;
        let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm)
            > 1 as core::ffi::c_int
        {
            FRAME_LF_COUNT
        } else {
            FRAME_LF_COUNT - 2 as core::ffi::c_int
        };
        let mib_size: core::ffi::c_int = (*(*cm).seq_params).mib_size;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j
            < (if mib_size < (*mi_params).mi_rows - mi_row {
                mib_size
            } else {
                (*mi_params).mi_rows - mi_row
            })
        {
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k
                < (if mib_size < (*mi_params).mi_cols - mi_col {
                    mib_size
                } else {
                    (*mi_params).mi_cols - mi_col
                })
            {
                let grid_idx: core::ffi::c_int = get_mi_grid_idx(
                    mi_params,
                    mi_row + j,
                    mi_col + k,
                ) as core::ffi::c_int;
                (*((*mi_params).mi_alloc).offset(grid_idx as isize))
                    .delta_lf_from_base = delta_lf;
                let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
                while lf_id < frame_lf_count {
                    (*((*mi_params).mi_alloc).offset(grid_idx as isize))
                        .delta_lf[lf_id as usize] = delta_lf;
                    lf_id += 1;
                }
                k += 1;
            }
            j += 1;
        }
    }
    (*x).reuse_inter_pred = false_0 != 0;
    (*x).txfm_search_params.mode_eval_type = DEFAULT_EVAL as core::ffi::c_int;
    reset_mb_rd_record((*x).txfm_search_info.mb_rd_record);
    memset(
        &raw mut (*x).picked_ref_frames_mask as *mut [core::ffi::c_int; 1024]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 1024]>() as size_t,
    );
    av1_invalid_rd_stats(rd_cost);
}
unsafe extern "C-unwind" fn sb_qp_sweep(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut sb_org_stats: *mut SB_FIRST_PASS_STATS,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut rdc_winner: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut cur_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut rdc_winner);
    let mut best_qindex: core::ffi::c_int = (*td).mb.rdmult_delta_qindex;
    let start: core::ffi::c_int = if (*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        -(20 as core::ffi::c_int)
    } else {
        -(12 as core::ffi::c_int)
    };
    let end: core::ffi::c_int = if (*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        20 as core::ffi::c_int
    } else {
        12 as core::ffi::c_int
    };
    let step: core::ffi::c_int = (*cm).delta_q_info.delta_q_res;
    let mut sweep_qp_delta: core::ffi::c_int = start;
    while sweep_qp_delta <= end {
        sb_qp_sweep_init_quantizers(
            cpi,
            td,
            tile_data,
            sms_tree,
            &raw mut cur_rdc,
            mi_row,
            mi_col,
            sweep_qp_delta,
        );
        let alloc_mi_idx: core::ffi::c_int = get_alloc_mi_idx(
            &raw mut (*cm).mi_params,
            mi_row,
            mi_col,
        ) as core::ffi::c_int;
        let backup_current_qindex: core::ffi::c_int = (*((*cm).mi_params.mi_alloc)
            .offset(alloc_mi_idx as isize))
            .current_qindex;
        av1_reset_mbmi(&raw mut (*cm).mi_params, bsize, mi_row, mi_col);
        av1_restore_sb_state(sb_org_stats, cpi, td, tile_data, mi_row, mi_col);
        (*((*cm).mi_params.mi_alloc).offset(alloc_mi_idx as isize)).current_qindex = backup_current_qindex;
        (*td).pc_root = av1_alloc_pc_tree_node(bsize);
        if ((*td).pc_root).is_null() {
            aom_internal_error(
                (*x).e_mbd.error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        av1_rd_pick_partition(
            cpi,
            td,
            tile_data,
            tp,
            mi_row,
            mi_col,
            bsize,
            &raw mut cur_rdc,
            cur_rdc,
            (*td).pc_root,
            sms_tree,
            0 as *mut int64_t,
            SB_DRY_PASS as core::ffi::c_int as SB_MULTI_PASS_MODE,
            0 as *mut RD_RECT_PART_WIN_INFO,
        );
        if rdc_winner.rdcost > cur_rdc.rdcost
            || abs(sweep_qp_delta) < abs(best_qindex - (*x).rdmult_delta_qindex)
                && rdc_winner.rdcost == cur_rdc.rdcost
        {
            rdc_winner = cur_rdc;
            best_qindex = (*x).rdmult_delta_qindex + sweep_qp_delta;
        }
        sweep_qp_delta += step;
    }
    return best_qindex;
}
#[inline]
unsafe extern "C-unwind" fn encode_rd_sb(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    seg_skip: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
        .offset(get_mi_grid_idx(&raw mut (*cm).mi_params, mi_row, mi_col) as isize);
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut dummy_rate: core::ffi::c_int = 0;
    let mut dummy_dist: int64_t = 0;
    let mut dummy_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let sms_root: *mut SIMPLE_MOTION_DATA_TREE = (*td).sms_root;
    init_encode_rd_sb(
        cpi,
        td,
        tile_data,
        sms_root,
        &raw mut dummy_rdc,
        mi_row,
        mi_col,
        1 as core::ffi::c_int,
    );
    if (*sf).part_sf.partition_search_type as core::ffi::c_int
        == VAR_BASED_PARTITION as core::ffi::c_int
    {
        av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
        av1_choose_var_based_partitioning(cpi, tile_info, td, x, mi_row, mi_col);
        (*td).pc_root = av1_alloc_pc_tree_node(sb_size);
        if ((*td).pc_root).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        av1_rd_use_partition(
            cpi,
            td,
            tile_data,
            mi,
            tp,
            mi_row,
            mi_col,
            sb_size,
            &raw mut dummy_rate,
            &raw mut dummy_dist,
            1 as core::ffi::c_int,
            (*td).pc_root,
        );
        av1_free_pc_tree_recursive(
            (*td).pc_root,
            num_planes,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (*sf).part_sf.partition_search_type,
        );
        (*td).pc_root = 0 as *mut PC_TREE;
    } else if (*sf).part_sf.partition_search_type as core::ffi::c_int
        == FIXED_PARTITION as core::ffi::c_int || seg_skip != 0
    {
        av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, sb_size);
        let bsize: BLOCK_SIZE = (if seg_skip != 0 {
            sb_size as core::ffi::c_int
        } else {
            (*sf).part_sf.fixed_partition_size as core::ffi::c_int
        }) as BLOCK_SIZE;
        av1_set_fixed_partitioning(cpi, tile_info, mi, mi_row, mi_col, bsize);
        (*td).pc_root = av1_alloc_pc_tree_node(sb_size);
        if ((*td).pc_root).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        av1_rd_use_partition(
            cpi,
            td,
            tile_data,
            mi,
            tp,
            mi_row,
            mi_col,
            sb_size,
            &raw mut dummy_rate,
            &raw mut dummy_dist,
            1 as core::ffi::c_int,
            (*td).pc_root,
        );
        av1_free_pc_tree_recursive(
            (*td).pc_root,
            num_planes,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (*sf).part_sf.partition_search_type,
        );
        (*td).pc_root = 0 as *mut PC_TREE;
    } else {
        let mut sb_enc: *mut SuperBlockEnc = &raw mut (*x).sb_enc;
        av1_get_tpl_stats_sb(cpi, sb_size, mi_row, mi_col, sb_enc);
        av1_reset_simple_motion_tree_partition(sms_root, sb_size);
        set_max_min_partition_size(sb_enc, cpi, x, sf, sb_size, mi_row, mi_col);
        let num_passes: core::ffi::c_int = if (*cpi)
            .oxcf
            .unit_test_cfg
            .sb_multipass_unit_test != 0
        {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        if (*cpi).oxcf.sb_qp_sweep != 0
            && !(has_no_stats_stage(cpi) != 0
                && (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int
                && (*cpi).oxcf.gf_cfg.lag_in_frames == 0 as core::ffi::c_int)
            && (*cm).delta_q_info.delta_q_present_flag != 0
        {
            if ((*(*x).e_mbd.error_info).setjmp == 0) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"encode_rd_sb\0"))
                        .as_ptr(),
                    b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    948 as core::ffi::c_int,
                    b"(x->e_mbd.error_info)->setjmp\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            (*td).mb.sb_stats_cache = aom_malloc(
                ::core::mem::size_of::<SB_FIRST_PASS_STATS>() as size_t,
            ) as *mut SB_FIRST_PASS_STATS as *mut SB_FIRST_PASS_STATS;
            if ((*td).mb.sb_stats_cache).is_null() {
                aom_internal_error(
                    (*x).e_mbd.error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate td->mb.sb_stats_cache at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/encodeframe.c\0" as *const u8
                        as *const core::ffi::c_char,
                    948 as core::ffi::c_int,
                );
            }
            av1_backup_sb_state(
                (*td).mb.sb_stats_cache as *mut SB_FIRST_PASS_STATS,
                cpi,
                td,
                tile_data,
                mi_row,
                mi_col,
            );
            if !((*x).rdmult_delta_qindex == (*x).delta_qindex) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"encode_rd_sb\0"))
                        .as_ptr(),
                    b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    951 as core::ffi::c_int,
                    b"x->rdmult_delta_qindex == x->delta_qindex\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let best_qp_diff: core::ffi::c_int = sb_qp_sweep(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                sb_size,
                sms_root,
                (*td).mb.sb_stats_cache as *mut SB_FIRST_PASS_STATS,
            ) as core::ffi::c_int - (*x).rdmult_delta_qindex;
            sb_qp_sweep_init_quantizers(
                cpi,
                td,
                tile_data,
                sms_root,
                &raw mut dummy_rdc,
                mi_row,
                mi_col,
                best_qp_diff,
            );
            let alloc_mi_idx: core::ffi::c_int = get_alloc_mi_idx(
                &raw mut (*cm).mi_params,
                mi_row,
                mi_col,
            ) as core::ffi::c_int;
            let backup_current_qindex: core::ffi::c_int = (*((*cm).mi_params.mi_alloc)
                .offset(alloc_mi_idx as isize))
                .current_qindex;
            av1_reset_mbmi(&raw mut (*cm).mi_params, sb_size, mi_row, mi_col);
            av1_restore_sb_state(
                (*td).mb.sb_stats_cache,
                cpi,
                td,
                tile_data,
                mi_row,
                mi_col,
            );
            (*((*cm).mi_params.mi_alloc).offset(alloc_mi_idx as isize)).current_qindex = backup_current_qindex;
            aom_free((*td).mb.sb_stats_cache as *mut core::ffi::c_void);
            (*td).mb.sb_stats_cache = 0 as *mut SB_FIRST_PASS_STATS;
        }
        if num_passes == 1 as core::ffi::c_int {
            (*td).pc_root = av1_alloc_pc_tree_node(sb_size);
            if ((*td).pc_root).is_null() {
                aom_internal_error(
                    (*xd).error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate PC_TREE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            av1_rd_pick_partition(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                sb_size,
                &raw mut dummy_rdc,
                dummy_rdc,
                (*td).pc_root,
                sms_root,
                0 as *mut int64_t,
                SB_SINGLE_PASS as core::ffi::c_int as SB_MULTI_PASS_MODE,
                0 as *mut RD_RECT_PART_WIN_INFO,
            );
        } else {
            if ((*(*x).e_mbd.error_info).setjmp == 0) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"encode_rd_sb\0"))
                        .as_ptr(),
                    b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    1004 as core::ffi::c_int,
                    b"(x->e_mbd.error_info)->setjmp\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            (*td).mb.sb_fp_stats = aom_malloc(
                ::core::mem::size_of::<SB_FIRST_PASS_STATS>() as size_t,
            ) as *mut SB_FIRST_PASS_STATS as *mut SB_FIRST_PASS_STATS;
            if ((*td).mb.sb_fp_stats).is_null() {
                aom_internal_error(
                    (*x).e_mbd.error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate td->mb.sb_fp_stats at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/encodeframe.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1004 as core::ffi::c_int,
                );
            }
            av1_backup_sb_state(
                (*td).mb.sb_fp_stats as *mut SB_FIRST_PASS_STATS,
                cpi,
                td,
                tile_data,
                mi_row,
                mi_col,
            );
            (*td).pc_root = av1_alloc_pc_tree_node(sb_size);
            if ((*td).pc_root).is_null() {
                aom_internal_error(
                    (*xd).error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate PC_TREE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            av1_rd_pick_partition(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                sb_size,
                &raw mut dummy_rdc,
                dummy_rdc,
                (*td).pc_root,
                sms_root,
                0 as *mut int64_t,
                SB_DRY_PASS as core::ffi::c_int as SB_MULTI_PASS_MODE,
                0 as *mut RD_RECT_PART_WIN_INFO,
            );
            init_encode_rd_sb(
                cpi,
                td,
                tile_data,
                sms_root,
                &raw mut dummy_rdc,
                mi_row,
                mi_col,
                0 as core::ffi::c_int,
            );
            av1_reset_mbmi(&raw mut (*cm).mi_params, sb_size, mi_row, mi_col);
            av1_reset_simple_motion_tree_partition(sms_root, sb_size);
            av1_restore_sb_state(
                (*td).mb.sb_fp_stats,
                cpi,
                td,
                tile_data,
                mi_row,
                mi_col,
            );
            (*td).pc_root = av1_alloc_pc_tree_node(sb_size);
            if ((*td).pc_root).is_null() {
                aom_internal_error(
                    (*xd).error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate PC_TREE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            av1_rd_pick_partition(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                sb_size,
                &raw mut dummy_rdc,
                dummy_rdc,
                (*td).pc_root,
                sms_root,
                0 as *mut int64_t,
                SB_WET_PASS as core::ffi::c_int as SB_MULTI_PASS_MODE,
                0 as *mut RD_RECT_PART_WIN_INFO,
            );
            aom_free((*td).mb.sb_fp_stats as *mut core::ffi::c_void);
            (*td).mb.sb_fp_stats = 0 as *mut SB_FIRST_PASS_STATS;
        }
        (*sb_enc).tpl_data_count = 0 as core::ffi::c_int;
    }
    if (*cpi).sf.inter_sf.inter_mode_rd_model_estimation == 1 as core::ffi::c_int
        && (*cm).tiles.cols == 1 as core::ffi::c_int
        && (*cm).tiles.rows == 1 as core::ffi::c_int
    {
        av1_inter_mode_data_fit(tile_data, (*x).rdmult);
    }
}
#[inline]
unsafe extern "C-unwind" fn is_mode_coeff_dv_upd_freq_tile_or_off(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let inter_sf: *const INTER_MODE_SPEED_FEATURES = &(*cpi).sf.inter_sf;
    return ((*inter_sf).coeff_cost_upd_level as core::ffi::c_uint
        <= INTERNAL_COST_UPD_TILE as core::ffi::c_int as core::ffi::c_uint
        && (*inter_sf).mode_cost_upd_level as core::ffi::c_uint
            <= INTERNAL_COST_UPD_TILE as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).sf.intra_sf.dv_cost_upd_level as core::ffi::c_uint
            <= INTERNAL_COST_UPD_TILE as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn delay_wait_for_top_right_sb(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let mode: MODE = (*cpi).oxcf.mode;
    if mode as core::ffi::c_int == GOOD as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int {
        return is_mode_coeff_dv_upd_freq_tile_or_off(cpi)
    } else if mode as core::ffi::c_int == REALTIME as core::ffi::c_int {
        return (is_mode_coeff_dv_upd_freq_tile_or_off(cpi) != 0
            && (*cpi).sf.inter_sf.mv_cost_upd_level as core::ffi::c_uint
                <= INTERNAL_COST_UPD_TILE as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int
    } else {
        return 0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn get_sb_source_sad(
    mut cpi: *const AV1_COMP,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> uint64_t {
    if ((*cpi).src_sad_blk_64x64).is_null() {
        return UINT64_MAX as uint64_t;
    }
    let cm: *const AV1_COMMON = &(*cpi).common;
    let blk_64x64_in_mis: core::ffi::c_int = if (*(*cm).seq_params).sb_size
        as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
    {
        (*(*cm).seq_params).mib_size >> 1 as core::ffi::c_int
    } else {
        (*(*cm).seq_params).mib_size
    };
    let num_blk_64x64_cols: core::ffi::c_int = ((*cm).mi_params.mi_cols
        + blk_64x64_in_mis - 1 as core::ffi::c_int) / blk_64x64_in_mis;
    let num_blk_64x64_rows: core::ffi::c_int = ((*cm).mi_params.mi_rows
        + blk_64x64_in_mis - 1 as core::ffi::c_int) / blk_64x64_in_mis;
    let blk_64x64_col_index: core::ffi::c_int = mi_col / blk_64x64_in_mis;
    let blk_64x64_row_index: core::ffi::c_int = mi_row / blk_64x64_in_mis;
    let mut curr_sb_sad: uint64_t = UINT64_MAX as uint64_t;
    if blk_64x64_row_index >= num_blk_64x64_rows - 1 as core::ffi::c_int
        || blk_64x64_col_index >= num_blk_64x64_cols - 1 as core::ffi::c_int
    {
        return curr_sb_sad;
    }
    let src_sad_blk_64x64_data: *const uint64_t = &raw mut *((*cpi).src_sad_blk_64x64)
        .offset(
            (blk_64x64_col_index + blk_64x64_row_index * num_blk_64x64_cols) as isize,
        ) as *mut uint64_t;
    if (*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_128X128 as core::ffi::c_int
    {
        curr_sb_sad = (*src_sad_blk_64x64_data.offset(0 as core::ffi::c_int as isize))
            .wrapping_add(*src_sad_blk_64x64_data.offset(1 as core::ffi::c_int as isize))
            .wrapping_add(*src_sad_blk_64x64_data.offset(num_blk_64x64_cols as isize))
            .wrapping_add(
                *src_sad_blk_64x64_data
                    .offset((num_blk_64x64_cols + 1 as core::ffi::c_int) as isize),
            );
    } else if (*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_64X64 as core::ffi::c_int
    {
        curr_sb_sad = *src_sad_blk_64x64_data.offset(0 as core::ffi::c_int as isize);
    }
    return curr_sb_sad;
}
#[inline]
unsafe extern "C-unwind" fn is_calc_src_content_needed(
    mut cpi: *mut AV1_COMP,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> bool {
    if (*cpi).svc.spatial_layer_id
        < (*cpi).svc.number_spatial_layers - 1 as core::ffi::c_int
    {
        return true_0 != 0;
    }
    let curr_sb_sad: uint64_t = get_sb_source_sad(cpi, mi_row, mi_col) as uint64_t;
    if curr_sb_sad == UINT64_MAX as uint64_t {
        return true_0 != 0;
    }
    if curr_sb_sad == 0 as uint64_t {
        (*x).content_state_sb.source_sad_nonrd = kZeroSad;
        return false_0 != 0;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut do_calc_src_content: bool = true_0 != 0;
    if (*cpi).oxcf.speed < 9 as core::ffi::c_int {
        return do_calc_src_content;
    }
    if (if (*cm).width < (*cm).height { (*cm).width } else { (*cm).height })
        < 360 as core::ffi::c_int
    {
        let avg_64x64_blk_sad: uint64_t = if (*(*cm).seq_params).sb_size
            as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        {
            curr_sb_sad.wrapping_add(2 as uint64_t) >> 2 as core::ffi::c_int
        } else {
            curr_sb_sad
        };
        let mut thresh_low: uint64_t = 15000 as uint64_t;
        let mut thresh_high: uint64_t = 40000 as uint64_t;
        if (*cpi).sf.rt_sf.increase_source_sad_thresh != 0 {
            thresh_low = thresh_low << 1 as core::ffi::c_int;
            thresh_high = thresh_high << 1 as core::ffi::c_int;
        }
        if avg_64x64_blk_sad > thresh_low && avg_64x64_blk_sad < thresh_high {
            do_calc_src_content = false_0 != 0;
            (*x).content_state_sb.source_sad_nonrd = kMedSad;
        }
    }
    return do_calc_src_content;
}
#[inline]
unsafe extern "C-unwind" fn grade_source_content_sb(
    mut cpi: *mut AV1_COMP,
    x: *mut MACROBLOCK,
    mut tile_data: *mut TileDataEnc,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    if (*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*(*cpi).ppi).use_svc != 0
            && (*((*cpi).svc.layer_context)
                .offset((*cpi).svc.temporal_layer_id as isize))
                .is_key_frame != 0
    {
        if !((*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            == kMedSad as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"grade_source_content_sb\0"))
                    .as_ptr(),
                b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1184 as core::ffi::c_int,
                b"x->content_state_sb.source_sad_nonrd == kMedSad\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !((*x).content_state_sb.source_sad_rd as core::ffi::c_uint
            == kMedSad as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"grade_source_content_sb\0"))
                    .as_ptr(),
                b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1185 as core::ffi::c_int,
                b"x->content_state_sb.source_sad_rd == kMedSad\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return;
    }
    let mut calc_src_content: bool = false_0 != 0;
    if (*cpi).sf.rt_sf.source_metrics_sb_nonrd != 0 {
        if (*cpi).sf.rt_sf.check_scene_detection == 0
            || (*cpi).rc.frame_source_sad > 0 as uint64_t
        {
            calc_src_content = is_calc_src_content_needed(cpi, x, mi_row, mi_col);
        } else {
            (*x).content_state_sb.source_sad_nonrd = kZeroSad;
        }
    } else if (*cpi).sf.rt_sf.var_part_based_on_qidx >= 1 as core::ffi::c_int
        && (*cm).width * (*cm).height
            <= 352 as core::ffi::c_int * 288 as core::ffi::c_int
    {
        if (*cpi).rc.frame_source_sad > 0 as uint64_t {
            calc_src_content = true_0 != 0;
        } else {
            (*x).content_state_sb.source_sad_rd = kZeroSad;
        }
    }
    if calc_src_content {
        av1_source_content_sb(cpi, x, tile_data, mi_row, mi_col);
    }
}
#[inline]
unsafe extern "C-unwind" fn encode_sb_row(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mi_row: core::ffi::c_int,
    mut tp: *mut *mut TokenExtra,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*mt_info).enc_row_mt;
    let row_mt_sync: *mut AV1EncRowMultiThreadSync = &raw mut (*tile_data).row_mt_sync;
    let mut row_mt_enabled: bool = (*mt_info).row_mt_enabled;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let sb_cols_in_tile: core::ffi::c_int = av1_get_sb_cols_in_tile(cm, tile_info)
        as core::ffi::c_int;
    let sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let mib_size: core::ffi::c_int = (*(*cm).seq_params).mib_size;
    let mib_size_log2: core::ffi::c_int = (*(*cm).seq_params).mib_size_log2;
    let sb_row: core::ffi::c_int = mi_row - (*tile_info).mi_row_start >> mib_size_log2;
    let use_nonrd_mode: core::ffi::c_int = (*cpi).sf.rt_sf.use_nonrd_pick_mode;
    av1_zero_left_context(xd);
    if mi_row == (*tile_info).mi_row_start || row_mt_enabled as core::ffi::c_int != 0 {
        if (*cpi).cb_delta_rdmult_enabled != 0 {
            (*xd).current_base_qindex = (*cm).quant_params.base_qindex;
        }
        if (*cm).delta_q_info.delta_lf_present_flag != 0 {
            av1_reset_loop_filter_delta(xd, av1_num_planes(cm));
        }
    }
    reset_thresh_freq_fact(x);
    let mut mi_col: core::ffi::c_int = (*tile_info).mi_col_start;
    let mut sb_col_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
    while mi_col < (*tile_info).mi_col_end {
        ((*enc_row_mt).sync_read_ptr)
            .expect(
                "non-null function pointer",
            )(row_mt_sync, sb_row, sb_col_in_tile - delay_wait_for_top_right_sb(cpi));
        if row_mt_enabled {
            pthread_mutex_lock((*enc_row_mt).mutex_ as *mut pthread_mutex_t);
            let row_mt_exit: bool = (*enc_row_mt).row_mt_exit;
            pthread_mutex_unlock((*enc_row_mt).mutex_ as *mut pthread_mutex_t);
            if row_mt_exit {
                return;
            }
        }
        let update_cdf: core::ffi::c_int = ((*tile_data).allow_update_cdf
            as core::ffi::c_int != 0 && row_mt_enabled as core::ffi::c_int != 0)
            as core::ffi::c_int;
        if update_cdf != 0 && (*tile_info).mi_row_start != mi_row {
            if (*tile_info).mi_col_start == mi_col {
                *(*xd).tile_ctx = *(*x).row_ctx;
            } else {
                let mut wt_left: core::ffi::c_int = AVG_CDF_WEIGHT_LEFT;
                let mut wt_tr: core::ffi::c_int = AVG_CDF_WEIGHT_TOP_RIGHT;
                if (*tile_info).mi_col_end > mi_col + mib_size {
                    av1_avg_cdf_symbols(
                        (*xd).tile_ctx,
                        ((*x).row_ctx).offset(sb_col_in_tile as isize),
                        wt_left,
                        wt_tr,
                    );
                } else {
                    av1_avg_cdf_symbols(
                        (*xd).tile_ctx,
                        ((*x).row_ctx)
                            .offset(sb_col_in_tile as isize)
                            .offset(-(1 as core::ffi::c_int as isize)),
                        wt_left,
                        wt_tr,
                    );
                }
            }
        }
        av1_set_cost_upd_freq(cpi, td, tile_info, mi_row, mi_col);
        memset(
            &raw mut (*x).color_sensitivity_sb as *mut [uint8_t; 2]
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
        );
        memset(
            &raw mut (*x).color_sensitivity_sb_g as *mut [uint8_t; 2]
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
        );
        memset(
            &raw mut (*x).color_sensitivity_sb_alt as *mut [uint8_t; 2]
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
        );
        memset(
            &raw mut (*x).color_sensitivity as *mut [uint8_t; 2] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
        );
        (*x).content_state_sb.source_sad_nonrd = kMedSad;
        (*x).content_state_sb.source_sad_rd = kMedSad;
        (*x).content_state_sb.lighting_change = 0 as core::ffi::c_int;
        (*x).content_state_sb.low_sumdiff = 0 as core::ffi::c_int;
        (*x).force_zeromv_skip_for_sb = 0 as core::ffi::c_int;
        (*x).sb_me_block = 0 as core::ffi::c_int;
        (*x).sb_me_partition = 0 as core::ffi::c_int;
        (*x).sb_me_mv.as_int = 0 as uint32_t;
        (*x).sb_force_fixed_part = 1 as core::ffi::c_int;
        (*x).color_palette_thresh = 64 as core::ffi::c_int;
        (*x).force_color_check_block_level = 0 as core::ffi::c_int;
        (*x).nonrd_prune_ref_frame_search = (*cpi).sf.rt_sf.nonrd_prune_ref_frame_search;
        if (*cpi).oxcf.mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int {
            (*x).intra_sb_rdmult_modifier = 128 as core::ffi::c_int;
        }
        (*xd).cur_frame_force_integer_mv = (*cm).features.cur_frame_force_integer_mv
            as core::ffi::c_int;
        (*x).source_variance = UINT_MAX;
        (*td).mb.cb_coef_buff = av1_get_cb_coeff_buffer(cpi, mi_row, mi_col);
        let seg: *const segmentation = &raw mut (*cm).seg;
        let mut seg_skip: core::ffi::c_int = 0 as core::ffi::c_int;
        if (*seg).enabled != 0 {
            let map: *const uint8_t = if (*seg).update_map as core::ffi::c_int != 0 {
                (*cpi).enc_seg.map
            } else {
                (*cm).last_frame_seg_map
            };
            let segment_id: uint8_t = (if !map.is_null() {
                get_segment_id(&raw mut (*cm).mi_params, map, sb_size, mi_row, mi_col)
                    as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint8_t;
            seg_skip = segfeature_active(
                seg,
                segment_id,
                SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
            );
        }
        produce_gradients_for_sb(cpi, x, sb_size, mi_row, mi_col);
        init_src_var_info_of_4x4_sub_blocks(
            cpi,
            (*x).src_var_info_of_4x4_sub_blocks,
            sb_size,
        );
        if seg_skip == 0 {
            grade_source_content_sb(cpi, x, tile_data, mi_row, mi_col);
        }
        if use_nonrd_mode != 0 {
            encode_nonrd_sb(cpi, td, tile_data, tp, mi_row, mi_col, seg_skip);
        } else {
            encode_rd_sb(cpi, td, tile_data, tp, mi_row, mi_col, seg_skip);
        }
        if update_cdf != 0 && (*tile_info).mi_row_end > mi_row + mib_size {
            if sb_cols_in_tile == 1 as core::ffi::c_int {
                *((*x).row_ctx).offset(0 as core::ffi::c_int as isize) = *(*xd).tile_ctx;
            } else if sb_col_in_tile >= 1 as core::ffi::c_int {
                *((*x).row_ctx)
                    .offset((sb_col_in_tile - 1 as core::ffi::c_int) as isize) = *(*xd)
                    .tile_ctx;
            }
        }
        ((*enc_row_mt).sync_write_ptr)
            .expect(
                "non-null function pointer",
            )(row_mt_sync, sb_row, sb_col_in_tile, sb_cols_in_tile);
        mi_col += mib_size;
        sb_col_in_tile += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn init_encode_frame_mb_context(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let x: *mut MACROBLOCK = &raw mut (*cpi).td.mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    av1_setup_src_planes(
        x,
        (*cpi).source,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        num_planes,
        (*(*cm).seq_params).sb_size,
    );
    av1_setup_block_planes(
        xd,
        (*(*cm).seq_params).subsampling_x,
        (*(*cm).seq_params).subsampling_y,
        num_planes,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_tile_data(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    av1_row_mt_mem_dealloc(cpi);
    aom_free((*cpi).tile_data as *mut core::ffi::c_void);
    (*cpi).allocated_tiles = 0 as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_alloc_tile_data\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1388 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*cpi).tile_data = aom_memalign(
        32 as size_t,
        ((tile_cols * tile_rows) as size_t)
            .wrapping_mul(::core::mem::size_of::<TileDataEnc>() as size_t),
    ) as *mut TileDataEnc;
    if ((*cpi).tile_data).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate cpi->tile_data at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/encodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            1388 as core::ffi::c_int,
        );
    }
    (*cpi).allocated_tiles = tile_cols * tile_rows;
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let tile_index: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_index as isize) as *mut TileDataEnc;
            memset(
                &raw mut (*this_tile).row_mt_sync as *mut AV1EncRowMultiThreadSync
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<AV1EncRowMultiThreadSync>() as size_t,
            );
            (*this_tile).row_ctx = 0 as *mut FRAME_CONTEXT;
            tile_col += 1;
        }
        tile_row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_tile_data(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut tile_col: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = 0;
    let token_info: *mut TokenInfo = &raw mut (*cpi).token_info;
    let mut pre_tok: *mut TokenExtra = (*token_info)
        .tile_tok[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    let mut tplist: *mut TokenList = (*token_info)
        .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    let mut tile_tok: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut tplist_count: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_stat_generation_stage(cpi) == 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
    {
        let mut tokens_required: core::ffi::c_uint = get_token_alloc(
            (*cm).mi_params.mb_rows,
            (*cm).mi_params.mb_cols,
            MAX_SB_SIZE_LOG2,
            num_planes,
        );
        let realloc_tokens: bool = tokens_required > (*token_info).tokens_allocated;
        if realloc_tokens {
            free_token_info(token_info);
            alloc_token_info(cm, token_info, tokens_required);
            pre_tok = (*token_info)
                .tile_tok[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize];
            tplist = (*token_info)
                .tplist[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
        }
    }
    tile_row = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        tile_col = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let tile_data: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset((tile_row * tile_cols + tile_col) as isize) as *mut TileDataEnc;
            let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
            av1_tile_init(tile_info, cm, tile_row, tile_col);
            (*tile_data).firstpass_top_mv = kZeroMv;
            (*tile_data).abs_sum_level = 0 as uint64_t;
            if is_token_info_allocated(token_info) {
                (*token_info).tile_tok[tile_row as usize][tile_col as usize] = pre_tok
                    .offset(tile_tok as isize);
                pre_tok = (*token_info).tile_tok[tile_row as usize][tile_col as usize];
                tile_tok = allocated_tokens(
                    tile_info,
                    (*(*cm).seq_params).mib_size_log2 + MI_SIZE_LOG2,
                    num_planes,
                );
                (*token_info).tplist[tile_row as usize][tile_col as usize] = tplist
                    .offset(tplist_count as isize);
                tplist = (*token_info).tplist[tile_row as usize][tile_col as usize];
                tplist_count = av1_get_sb_rows_in_tile(cm, tile_info);
            }
            (*tile_data).allow_update_cdf = ((*cm).tiles.large_scale == 0)
                as core::ffi::c_int as uint8_t;
            (*tile_data).allow_update_cdf = ((*tile_data).allow_update_cdf
                as core::ffi::c_int != 0 && !(*cm).features.disable_cdf_update
                && delay_wait_for_top_right_sb(cpi) == 0) as core::ffi::c_int as uint8_t;
            (*tile_data).tctx = *(*cm).fc;
            tile_col += 1;
        }
        tile_row += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_token_start(
    mut cpi: *mut AV1_COMP,
    mut tile_info: *const TileInfo,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut tp: *mut *mut TokenExtra,
) {
    let mut token_info: *const TokenInfo = &raw mut (*cpi).token_info;
    if !is_token_info_allocated(token_info) {
        return;
    }
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tplist: *mut TokenList = (*cpi)
        .token_info
        .tplist[tile_row as usize][tile_col as usize];
    let sb_row_in_tile: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
        >> (*(*cm).seq_params).mib_size_log2;
    get_start_tok(
        cpi,
        tile_row,
        tile_col,
        mi_row,
        tp,
        (*(*cm).seq_params).mib_size_log2 + MI_SIZE_LOG2,
        num_planes,
    );
    if tplist.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"get_token_start\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1476 as core::ffi::c_int,
            b"tplist != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let fresh2 = &raw mut (*tplist.offset(sb_row_in_tile as isize)).start;
    *fresh2 = *tp;
}
#[inline]
unsafe extern "C-unwind" fn populate_token_count(
    mut cpi: *mut AV1_COMP,
    mut tile_info: *const TileInfo,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut tok: *mut TokenExtra,
) {
    let mut token_info: *const TokenInfo = &raw mut (*cpi).token_info;
    if !is_token_info_allocated(token_info) {
        return;
    }
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tplist: *mut TokenList = (*token_info)
        .tplist[tile_row as usize][tile_col as usize];
    let sb_row_in_tile: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
        >> (*(*cm).seq_params).mib_size_log2;
    let tile_mb_cols: core::ffi::c_int = (*tile_info).mi_col_end
        - (*tile_info).mi_col_start + 2 as core::ffi::c_int >> 2 as core::ffi::c_int;
    let num_mb_rows_in_sb: core::ffi::c_int = ((1 as core::ffi::c_int)
        << (*(*cm).seq_params).mib_size_log2 + MI_SIZE_LOG2) + 8 as core::ffi::c_int
        >> 4 as core::ffi::c_int;
    (*tplist.offset(sb_row_in_tile as isize)).count = tok
        .offset_from((*tplist.offset(sb_row_in_tile as isize)).start)
        as core::ffi::c_long as core::ffi::c_uint;
    if !(tok.offset_from((*tplist.offset(sb_row_in_tile as isize)).start)
        as core::ffi::c_long as core::ffi::c_uint
        <= get_token_alloc(
            num_mb_rows_in_sb,
            tile_mb_cols,
            (*(*cm).seq_params).mib_size_log2 + 2 as core::ffi::c_int,
            num_planes,
        )) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"populate_token_count\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1503 as core::ffi::c_int,
            b"(unsigned int)(tok - tplist[sb_row_in_tile].start) <= get_token_alloc(num_mb_rows_in_sb, tile_mb_cols, cm->seq_params->mib_size_log2 + MI_SIZE_LOG2, num_planes)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encode_sb_row(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let mut this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
        .offset((tile_row * tile_cols + tile_col) as isize) as *mut TileDataEnc;
    let tile_info: *const TileInfo = &raw mut (*this_tile).tile_info;
    let mut tok: *mut TokenExtra = 0 as *mut TokenExtra;
    get_token_start(cpi, tile_info, tile_row, tile_col, mi_row, &raw mut tok);
    encode_sb_row(cpi, td, this_tile, mi_row, &raw mut tok);
    populate_token_count(cpi, tile_info, tile_row, tile_col, mi_row, tok);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encode_tile(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
        .offset((tile_row * (*cm).tiles.cols + tile_col) as isize) as *mut TileDataEnc;
    let tile_info: *const TileInfo = &raw mut (*this_tile).tile_info;
    if (*cpi).sf.rt_sf.use_nonrd_pick_mode == 0 {
        av1_inter_mode_data_init(this_tile as *mut TileDataEnc);
    }
    av1_zero_above_context(
        cm,
        &raw mut (*td).mb.e_mbd,
        (*tile_info).mi_col_start,
        (*tile_info).mi_col_end,
        tile_row,
    );
    av1_init_above_context(
        &raw mut (*cm).above_contexts,
        av1_num_planes(cm),
        tile_row,
        &raw mut (*td).mb.e_mbd,
    );
    if (*cpi).oxcf.intra_mode_cfg.enable_cfl_intra {
        cfl_init(&raw mut (*td).mb.e_mbd.cfl, (*cm).seq_params);
    }
    if !((*td).mb.txfm_search_info.mb_rd_record).is_null() {
        av1_crc32c_calculator_init(
            &raw mut (*(*td).mb.txfm_search_info.mb_rd_record).crc_calculator,
        );
    }
    let mut mi_row: core::ffi::c_int = (*tile_info).mi_row_start;
    while mi_row < (*tile_info).mi_row_end {
        av1_encode_sb_row(cpi, td, tile_row, tile_col, mi_row);
        mi_row += (*(*cm).seq_params).mib_size;
    }
    (*this_tile).abs_sum_level = (*td).abs_sum_level;
}
#[inline]
unsafe extern "C-unwind" fn encode_tiles(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut tile_col: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = 0;
    let mb: *mut MACROBLOCK = &raw mut (*cpi).td.mb;
    if !(!((*cpi).tile_data).is_null()
        || (*cpi).allocated_tiles == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"encode_tiles\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1577 as core::ffi::c_int,
            b"IMPLIES(cpi->tile_data == NULL, cpi->allocated_tiles == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*cpi).allocated_tiles != tile_cols * tile_rows {
        av1_alloc_tile_data(cpi);
    }
    av1_init_tile_data(cpi);
    av1_alloc_mb_data(cpi, mb as *mut macroblock);
    tile_row = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        tile_col = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset((tile_row * (*cm).tiles.cols + tile_col) as isize)
                as *mut TileDataEnc;
            (*cpi).td.intrabc_used = 0 as core::ffi::c_int;
            (*cpi).td.deltaq_used = 0 as core::ffi::c_int;
            (*cpi).td.abs_sum_level = 0 as uint64_t;
            (*cpi).td.rd_counts.seg_tmp_pred_cost[0 as core::ffi::c_int as usize] = 0
                as uint64_t;
            (*cpi).td.rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize] = 0
                as uint64_t;
            (*cpi).td.mb.e_mbd.tile_ctx = &raw mut (*this_tile).tctx;
            (*cpi).td.mb.tile_pb_ctx = &raw mut (*this_tile).tctx;
            av1_init_rtc_counters(&raw mut (*cpi).td.mb);
            (*cpi).td.mb.palette_pixels = 0 as core::ffi::c_int;
            av1_encode_tile(cpi, &raw mut (*cpi).td, tile_row, tile_col);
            if frame_is_intra_only(&raw mut (*cpi).common) == 0 {
                av1_accumulate_rtc_counters(cpi, &raw mut (*cpi).td.mb);
            }
            (*cpi).palette_pixel_num += (*cpi).td.mb.palette_pixels;
            (*cpi).intrabc_used |= (*cpi).td.intrabc_used;
            (*cpi).deltaq_used |= (*cpi).td.deltaq_used;
            tile_col += 1;
        }
        tile_row += 1;
    }
    av1_dealloc_mb_data(mb as *mut macroblock, av1_num_planes(cm));
}
#[inline]
unsafe extern "C-unwind" fn set_rel_frame_dist(
    cm: *const AV1_COMMON,
    ref_frame_dist_info: *mut RefFrameDistanceInfo,
    ref_frame_flags: core::ffi::c_int,
) {
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    let mut min_past_dist: core::ffi::c_int = INT32_MAX;
    let mut min_future_dist: core::ffi::c_int = INT32_MAX;
    (*ref_frame_dist_info).nearest_past_ref = NONE_FRAME as core::ffi::c_int as int8_t;
    (*ref_frame_dist_info).nearest_future_ref = NONE_FRAME as core::ffi::c_int as int8_t;
    ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        (*ref_frame_dist_info)
            .ref_relative_dist[(ref_frame as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] = 0 as core::ffi::c_int;
        if ref_frame_flags
            & av1_ref_frame_flag_list[ref_frame as usize] as core::ffi::c_int != 0
        {
            let mut dist: core::ffi::c_int = av1_encoder_get_relative_dist(
                (*(*cm).cur_frame)
                    .ref_display_order_hint[(ref_frame as core::ffi::c_int
                    - LAST_FRAME as core::ffi::c_int) as usize] as core::ffi::c_int,
                (*cm).current_frame.display_order_hint as core::ffi::c_int,
            );
            (*ref_frame_dist_info)
                .ref_relative_dist[(ref_frame as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = dist;
            if abs(dist) < min_past_dist && dist < 0 as core::ffi::c_int {
                (*ref_frame_dist_info).nearest_past_ref = ref_frame as int8_t;
                min_past_dist = abs(dist);
            }
            if dist < min_future_dist && dist > 0 as core::ffi::c_int {
                (*ref_frame_dist_info).nearest_future_ref = ref_frame as int8_t;
                min_future_dist = dist;
            }
        }
        ref_frame += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn refs_are_one_sided(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    if (frame_is_intra_only(cm) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"refs_are_one_sided\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1638 as core::ffi::c_int,
            b"!frame_is_intra_only(cm)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut one_sided_refs: core::ffi::c_int = 1 as core::ffi::c_int;
    let cur_display_order_hint: core::ffi::c_int = (*cm).current_frame.display_order_hint
        as core::ffi::c_int;
    let mut ref_0: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while ref_0 <= ALTREF_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            ref_0 as MV_REFERENCE_FRAME,
        );
        if !buf.is_null() {
            if av1_encoder_get_relative_dist(
                (*buf).display_order_hint as core::ffi::c_int,
                cur_display_order_hint,
            ) > 0 as core::ffi::c_int
            {
                one_sided_refs = 0 as core::ffi::c_int;
                break;
            }
        }
        ref_0 += 1;
    }
    return one_sided_refs;
}
#[inline]
unsafe extern "C-unwind" fn get_skip_mode_ref_offsets(
    mut cm: *const AV1_COMMON,
    mut ref_order_hint: *mut core::ffi::c_int,
) {
    let skip_mode_info: *const SkipModeInfo = &(*cm).current_frame.skip_mode_info;
    let fresh3 = &raw mut *ref_order_hint.offset(1 as core::ffi::c_int as isize);
    *fresh3 = 0 as core::ffi::c_int;
    *ref_order_hint.offset(0 as core::ffi::c_int as isize) = *fresh3;
    if (*skip_mode_info).skip_mode_allowed == 0 {
        return;
    }
    let buf_0: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (LAST_FRAME as core::ffi::c_int + (*skip_mode_info).ref_frame_idx_0)
            as MV_REFERENCE_FRAME,
    );
    let buf_1: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (LAST_FRAME as core::ffi::c_int + (*skip_mode_info).ref_frame_idx_1)
            as MV_REFERENCE_FRAME,
    );
    if !(!buf_0.is_null() && !buf_1.is_null()) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"get_skip_mode_ref_offsets\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1664 as core::ffi::c_int,
            b"buf_0 != NULL && buf_1 != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *ref_order_hint.offset(0 as core::ffi::c_int as isize) = (*buf_0).order_hint
        as core::ffi::c_int;
    *ref_order_hint.offset(1 as core::ffi::c_int as isize) = (*buf_1).order_hint
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn check_skip_mode_enabled(cpi: *mut AV1_COMP) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    av1_setup_skip_mode_allowed(cm);
    if (*cm).current_frame.skip_mode_info.skip_mode_allowed == 0 {
        return 0 as core::ffi::c_int;
    }
    let cur_offset: core::ffi::c_int = (*cm).current_frame.order_hint
        as core::ffi::c_int;
    let mut ref_offset: [core::ffi::c_int; 2] = [0; 2];
    get_skip_mode_ref_offsets(cm, ref_offset.as_mut_ptr());
    let cur_to_ref0: core::ffi::c_int = get_relative_dist(
        &raw mut (*(*cm).seq_params).order_hint_info,
        cur_offset,
        ref_offset[0 as core::ffi::c_int as usize],
    ) as core::ffi::c_int;
    let cur_to_ref1: core::ffi::c_int = abs(
        get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            cur_offset,
            ref_offset[1 as core::ffi::c_int as usize],
        ),
    ) as core::ffi::c_int;
    if abs(cur_to_ref0 - cur_to_ref1) > 1 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    if (*cpi).all_one_sided_refs != 0
        && (*cpi).oxcf.gf_cfg.lag_in_frames > 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    let ref_frame: [core::ffi::c_int; 2] = [
        (*cm).current_frame.skip_mode_info.ref_frame_idx_0
            + LAST_FRAME as core::ffi::c_int,
        (*cm).current_frame.skip_mode_info.ref_frame_idx_1
            + LAST_FRAME as core::ffi::c_int,
    ];
    if (*cpi).ref_frame_flags
        & av1_ref_frame_flag_list[ref_frame[0 as core::ffi::c_int as usize] as usize]
            as core::ffi::c_int == 0
        || (*cpi).ref_frame_flags
            & av1_ref_frame_flag_list[ref_frame[1 as core::ffi::c_int as usize] as usize]
                as core::ffi::c_int == 0
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_default_interp_skip_flags(
    mut cm: *const AV1_COMMON,
    mut interp_search_flags: *mut InterpSearchFlags,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    (*interp_search_flags).default_interp_skip_flags = if num_planes
        == 1 as core::ffi::c_int
    {
        INTERP_SKIP_LUMA_EVAL_CHROMA as core::ffi::c_int
    } else {
        INTERP_SKIP_LUMA_SKIP_CHROMA as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn compare_score_data_asc(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let mut ra: *const RefScoreData = a as *const RefScoreData;
    let mut rb: *const RefScoreData = b as *const RefScoreData;
    let score_diff: core::ffi::c_int = (*ra).score - (*rb).score;
    if score_diff != 0 as core::ffi::c_int {
        return score_diff;
    }
    return (*ra).index - (*rb).index;
}
#[inline]
unsafe extern "C-unwind" fn setup_keep_ref_frame_mask(mut cpi: *mut AV1_COMP) {
    let prune_single_ref: core::ffi::c_int = (*cpi).sf.inter_sf.prune_single_ref;
    let prune_comp_ref_frames: core::ffi::c_int = (*cpi)
        .sf
        .inter_sf
        .prune_comp_ref_frames;
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    (*cpi).keep_single_ref_frame_mask = 0 as core::ffi::c_int;
    (*cpi).keep_comp_ref_frame_mask = 0 as core::ffi::c_int;
    if frame_is_intra_only(cm) != 0 {
        return;
    }
    let mut ref_score_data: [RefScoreData; 7] = [RefScoreData { score: 0, index: 0 }; 7];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
        ref_score_data[i as usize].score = INT_MAX;
        ref_score_data[i as usize].index = i;
        i += 1;
    }
    let mut ref_frame: MV_REFERENCE_FRAME = LAST_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        if (*cpi).ref_frame_flags
            & av1_ref_frame_flag_list[ref_frame as usize] as core::ffi::c_int != 0
        {
            let ref_frame_dist_info: *const RefFrameDistanceInfo = &raw mut (*cpi)
                .ref_frame_dist_info;
            let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
            ref_score_data[(ref_frame as core::ffi::c_int
                    - LAST_FRAME as core::ffi::c_int) as usize]
                .score = abs(
                (*ref_frame_dist_info)
                    .ref_relative_dist[(ref_frame as core::ffi::c_int
                    - LAST_FRAME as core::ffi::c_int) as usize],
            ) + (*buf).base_qindex;
        }
        ref_frame += 1;
    }
    qsort(
        ref_score_data.as_mut_ptr() as *mut core::ffi::c_void,
        INTER_REFS_PER_FRAME as core::ffi::c_int as size_t,
        ::core::mem::size_of::<RefScoreData>() as size_t,
        Some(
            compare_score_data_asc
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    static mut num_single_ref_to_keep_lookup: [core::ffi::c_int; 5] = [
        INTER_REFS_PER_FRAME as core::ffi::c_int,
        5 as core::ffi::c_int,
        3 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    if !(prune_single_ref >= 0 as core::ffi::c_int
        && prune_single_ref <= 4 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"setup_keep_ref_frame_mask\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1774 as core::ffi::c_int,
            b"prune_single_ref >= 0 && prune_single_ref <= 4\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let num_single_ref_to_keep: core::ffi::c_int = num_single_ref_to_keep_lookup[prune_single_ref
        as usize];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < num_single_ref_to_keep {
        let idx: core::ffi::c_int = ref_score_data[i_0 as usize].index;
        (*cpi).keep_single_ref_frame_mask |= (1 as core::ffi::c_int) << idx;
        i_0 += 1;
    }
    static mut num_comp_ref_to_keep_lookup: [core::ffi::c_int; 4] = [
        INTER_REFS_PER_FRAME as core::ffi::c_int,
        3 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    if !(prune_comp_ref_frames >= 0 as core::ffi::c_int
        && prune_comp_ref_frames <= 3 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"setup_keep_ref_frame_mask\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1794 as core::ffi::c_int,
            b"prune_comp_ref_frames >= 0 && prune_comp_ref_frames <= 3\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let num_comp_ref_to_keep: core::ffi::c_int = num_comp_ref_to_keep_lookup[prune_comp_ref_frames
        as usize];
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < num_comp_ref_to_keep {
        let idx_0: core::ffi::c_int = ref_score_data[i_1 as usize].index;
        (*cpi).keep_comp_ref_frame_mask |= (1 as core::ffi::c_int) << idx_0;
        i_1 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_prune_ref_frame_mask(mut cpi: *mut AV1_COMP) {
    if (!(*cpi).oxcf.ref_frm_cfg.enable_onesided_comp
        || (*cpi).sf.inter_sf.disable_onesided_comp != 0)
        && (*cpi).all_one_sided_refs != 0
    {
        (*cpi).prune_ref_frame_mask = ((1 as core::ffi::c_int)
            << REF_FRAMES as core::ffi::c_int
                + (FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                    + TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int))
            - ((1 as core::ffi::c_int) << REF_FRAMES as core::ffi::c_int);
    } else if (*cpi).sf.rt_sf.use_nonrd_pick_mode == 0
        && (*cpi).sf.inter_sf.selective_ref_frame >= 1 as core::ffi::c_int
    {
        let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
        let cur_frame_display_order_hint: core::ffi::c_int = (*cm)
            .current_frame
            .display_order_hint as core::ffi::c_int;
        let mut ref_display_order_hint: *mut core::ffi::c_uint = ((*(*cm).cur_frame)
            .ref_display_order_hint)
            .as_mut_ptr();
        let arf2_dist: core::ffi::c_int = av1_encoder_get_relative_dist(
            *ref_display_order_hint
                .offset(
                    (ALTREF2_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int,
            cur_frame_display_order_hint,
        ) as core::ffi::c_int;
        let bwd_dist: core::ffi::c_int = av1_encoder_get_relative_dist(
            *ref_display_order_hint
                .offset(
                    (BWDREF_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int,
            cur_frame_display_order_hint,
        ) as core::ffi::c_int;
        let mut ref_idx: core::ffi::c_int = REF_FRAMES as core::ffi::c_int;
        while ref_idx
            < REF_FRAMES as core::ffi::c_int
                + (FWD_REFS as core::ffi::c_int * BWD_REFS as core::ffi::c_int
                    + TOTAL_UNIDIR_COMP_REFS as core::ffi::c_int)
        {
            let mut rf: [MV_REFERENCE_FRAME; 2] = [0; 2];
            av1_set_ref_frame(rf.as_mut_ptr(), ref_idx as MV_REFERENCE_FRAME);
            if !((*cpi).ref_frame_flags
                & av1_ref_frame_flag_list[rf[0 as core::ffi::c_int as usize] as usize]
                    as core::ffi::c_int == 0
                || (*cpi).ref_frame_flags
                    & av1_ref_frame_flag_list[rf[1 as core::ffi::c_int as usize]
                        as usize] as core::ffi::c_int == 0)
            {
                if (*cpi).all_one_sided_refs == 0 {
                    let mut ref_dist: [core::ffi::c_int; 2] = [0; 2];
                    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                    while i < 2 as core::ffi::c_int {
                        ref_dist[i as usize] = av1_encoder_get_relative_dist(
                            *ref_display_order_hint
                                .offset(
                                    (rf[i as usize] as core::ffi::c_int
                                        - LAST_FRAME as core::ffi::c_int) as isize,
                                ) as core::ffi::c_int,
                            cur_frame_display_order_hint,
                        );
                        i += 1;
                    }
                    if (ref_dist[0 as core::ffi::c_int as usize] > 0 as core::ffi::c_int)
                        as core::ffi::c_int
                        == (ref_dist[1 as core::ffi::c_int as usize]
                            > 0 as core::ffi::c_int) as core::ffi::c_int
                    {
                        (*cpi).prune_ref_frame_mask
                            |= (1 as core::ffi::c_int) << ref_idx;
                    }
                }
                if (*cpi).sf.inter_sf.selective_ref_frame >= 4 as core::ffi::c_int
                    && (rf[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        == ALTREF2_FRAME as core::ffi::c_int
                        || rf[1 as core::ffi::c_int as usize] as core::ffi::c_int
                            == ALTREF2_FRAME as core::ffi::c_int)
                    && (*cpi).ref_frame_flags
                        & av1_ref_frame_flag_list[BWDREF_FRAME as core::ffi::c_int
                            as usize] as core::ffi::c_int != 0
                {
                    if arf2_dist > 0 as core::ffi::c_int
                        && bwd_dist > 0 as core::ffi::c_int && bwd_dist <= arf2_dist
                    {
                        (*cpi).prune_ref_frame_mask
                            |= (1 as core::ffi::c_int) << ref_idx;
                    }
                }
            }
            ref_idx += 1;
        }
    }
}
unsafe extern "C-unwind" fn allow_deltaq_mode(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut sb_size: BLOCK_SIZE = (*(*cm).seq_params).sb_size;
    let mut sbs_wide: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int;
    let mut sbs_high: core::ffi::c_int = mi_size_high[sb_size as usize]
        as core::ffi::c_int;
    let mut delta_rdcost: int64_t = 0 as int64_t;
    let mut mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while mi_row < (*cm).mi_params.mi_rows {
        let mut mi_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while mi_col < (*cm).mi_params.mi_cols {
            let mut this_delta_rdcost: int64_t = 0 as int64_t;
            av1_get_q_for_deltaq_objective(
                cpi,
                &raw mut (*cpi).td,
                &raw mut this_delta_rdcost,
                sb_size,
                mi_row,
                mi_col,
            );
            delta_rdcost += this_delta_rdcost;
            mi_col += sbs_wide;
        }
        mi_row += sbs_high;
    }
    return (delta_rdcost < 0 as int64_t) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn disable_deltaq_for_intl_arfs(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    if (*cpi).oxcf.mode as core::ffi::c_int == GOOD as core::ffi::c_int
        && is_stat_consumption_stage_twopass(cpi) != 0
        && (*cpi).oxcf.q_cfg.deltaq_mode as core::ffi::c_int
            == DELTA_Q_OBJECTIVE as core::ffi::c_int
        && (*cpi).oxcf.algo_cfg.enable_tpl_model as core::ffi::c_int != 0
        && (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int == NO_AQ as core::ffi::c_int
        && (*cpi).common.seg.enabled == 0 && (*cpi).roi.enabled == 0
        && (*cpi).oxcf.sb_qp_sweep == 0 && (*cpi).use_ducky_encode == 0
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn enable_delta_rdmult(mut cpi: *const AV1_COMP) -> core::ffi::c_int {
    if disable_deltaq_for_intl_arfs(cpi) == 0 {
        return (*cpi).common.delta_q_info.delta_q_present_flag;
    }
    let mut gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    return ((*gf_group).update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
        != LF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn enable_delta_q(mut cpi: *const AV1_COMP) -> core::ffi::c_int {
    let mut gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    if disable_deltaq_for_intl_arfs(cpi) == 0 {
        return ((*gf_group).update_type[(*cpi).gf_frame_index as usize]
            as core::ffi::c_int != LF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
    }
    return ((*cpi).common.current_frame.pyramid_level <= 1 as core::ffi::c_uint)
        as core::ffi::c_int;
}
pub const FORCE_ZMV_SKIP_128X128_BLK_DIFF: core::ffi::c_int = 10000 as core::ffi::c_int;
unsafe extern "C-unwind" fn populate_thresh_to_force_zeromv_skip(mut cpi: *mut AV1_COMP) {
    if (*cpi).sf.rt_sf.part_early_exit_zeromv == 0 as core::ffi::c_int {
        return;
    }
    let thresh_exit_128x128_part: core::ffi::c_uint = FORCE_ZMV_SKIP_128X128_BLK_DIFF
        as core::ffi::c_uint;
    let num_128x128_pix: core::ffi::c_int = block_size_wide[BLOCK_128X128
        as core::ffi::c_int as usize] as core::ffi::c_int
        * block_size_high[BLOCK_128X128 as core::ffi::c_int as usize]
            as core::ffi::c_int;
    let mut bsize: BLOCK_SIZE = BLOCK_4X4;
    while (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {
        let num_block_pix: core::ffi::c_int = block_size_wide[bsize as usize]
            as core::ffi::c_int * block_size_high[bsize as usize] as core::ffi::c_int;
        let mut thresh_exit_part_blk: core::ffi::c_uint = (thresh_exit_128x128_part
            as core::ffi::c_double
            * sqrt(
                num_block_pix as core::ffi::c_double
                    / num_128x128_pix as core::ffi::c_double,
            ) + 0.5f64) as core::ffi::c_uint;
        thresh_exit_part_blk = if thresh_exit_part_blk
            < (4 as core::ffi::c_int * num_block_pix) as core::ffi::c_uint
        {
            thresh_exit_part_blk
        } else {
            (4 as core::ffi::c_int * num_block_pix) as core::ffi::c_uint
        };
        (*cpi).zeromv_skip_thresh_exit_part[bsize as usize] = thresh_exit_part_blk;
        bsize += 1;
    }
}
unsafe extern "C-unwind" fn free_block_hash_buffers(mut block_hash_values: *mut *mut uint32_t) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < 2 as core::ffi::c_int {
        aom_free(*block_hash_values.offset(j as isize) as *mut core::ffi::c_void);
        j += 1;
    }
}
unsafe extern "C-unwind" fn aom_get_variance_boost_delta_q_res(
    mut qindex: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut delta_q_res: core::ffi::c_int = 1 as core::ffi::c_int;
    if qindex >= 160 as core::ffi::c_int {
        delta_q_res = 8 as core::ffi::c_int;
    } else if qindex >= 120 as core::ffi::c_int {
        delta_q_res = 4 as core::ffi::c_int;
    } else if qindex >= 80 as core::ffi::c_int {
        delta_q_res = 2 as core::ffi::c_int;
    } else {
        delta_q_res = 1 as core::ffi::c_int;
    }
    return delta_q_res;
}
unsafe extern "C-unwind" fn get_thresh_based_on_q(
    mut qindex: core::ffi::c_int,
    mut speed: core::ffi::c_int,
) -> core::ffi::c_float {
    let min_threshold_arr: [core::ffi::c_float; 3] = [0.084f32, 0.087f32, 0.126f32];
    let max_threshold_arr: [core::ffi::c_float; 3] = [0.140f32, 0.150f32, 0.182f32];
    let idx: core::ffi::c_int = if speed >= 3 as core::ffi::c_int {
        2 as core::ffi::c_int
    } else {
        speed - 1 as core::ffi::c_int
    };
    let min_thresh: core::ffi::c_float = min_threshold_arr[idx as usize];
    let max_thresh: core::ffi::c_float = max_threshold_arr[idx as usize];
    let thresh: core::ffi::c_float = min_thresh
        + (max_thresh - min_thresh)
            * (MAXQ as core::ffi::c_float - qindex as core::ffi::c_float)
            / (MAXQ - MINQ) as core::ffi::c_float;
    return thresh;
}
unsafe extern "C-unwind" fn get_mv_err(mut cur_mv: MV, mut ref_mv: MV) -> core::ffi::c_int {
    let diff: MV = {
        let mut init = mv {
            row: (cur_mv.row as core::ffi::c_int - ref_mv.row as core::ffi::c_int)
                as int16_t,
            col: (cur_mv.col as core::ffi::c_int - ref_mv.col as core::ffi::c_int)
                as int16_t,
        };
        init
    };
    let abs_diff: MV = {
        let mut init = mv {
            row: abs(diff.row as core::ffi::c_int) as int16_t,
            col: abs(diff.col as core::ffi::c_int) as int16_t,
        };
        init
    };
    let mv_err: core::ffi::c_int = abs_diff.row as core::ffi::c_int
        + abs_diff.col as core::ffi::c_int;
    return mv_err;
}
unsafe extern "C-unwind" fn check_mv_err_and_update(
    mut cur_mv: MV,
    mut ref_mv: MV,
    mut best_mv_err: *mut core::ffi::c_int,
) {
    let mv_err: core::ffi::c_int = get_mv_err(cur_mv, ref_mv) as core::ffi::c_int;
    *best_mv_err = if mv_err < *best_mv_err { mv_err } else { *best_mv_err };
}
unsafe extern "C-unwind" fn is_inside_frame_border(
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut row_offset: core::ffi::c_int,
    mut col_offset: core::ffi::c_int,
    mut num_mi_rows: core::ffi::c_int,
    mut num_mi_cols: core::ffi::c_int,
) -> core::ffi::c_int {
    if mi_row + row_offset < 0 as core::ffi::c_int || mi_row + row_offset >= num_mi_rows
        || mi_col + col_offset < 0 as core::ffi::c_int
        || mi_col + col_offset >= num_mi_cols
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn get_spatial_mvpred_err(
    mut cm: *mut AV1_COMMON,
    tpl_data: *mut TplParams,
    mut tpl_idx: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut ref_idx: core::ffi::c_int,
    mut cur_mv: int_mv,
    mut allow_hp: core::ffi::c_int,
    mut is_integer: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut tpl_frame: *const TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(tpl_idx as isize) as *mut TplDepFrame;
    let mut tpl_ptr: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let block_mis_log2: uint8_t = (*tpl_data).tpl_stats_block_mis_log2;
    let mut mv_err: core::ffi::c_int = INT32_MAX;
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << block_mis_log2 as core::ffi::c_int;
    let mv_pred_pos_in_mis: [[core::ffi::c_int; 2]; 8] = [
        [-step, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, -step],
        [-step, step],
        [-step, -step],
        [-(2 as core::ffi::c_int) * step, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, -(2 as core::ffi::c_int) * step],
        [-(3 as core::ffi::c_int) * step, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, -(3 as core::ffi::c_int) * step],
    ];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let mut row_offset: core::ffi::c_int = mv_pred_pos_in_mis[i
            as usize][0 as core::ffi::c_int as usize];
        let mut col_offset: core::ffi::c_int = mv_pred_pos_in_mis[i
            as usize][1 as core::ffi::c_int as usize];
        if !(is_inside_frame_border(
            mi_row,
            mi_col,
            row_offset,
            col_offset,
            (*tpl_frame).mi_rows,
            (*tpl_frame).mi_cols,
        ) == 0)
        {
            let mut tpl_stats: *const TplDepStats = &raw mut *tpl_ptr
                .offset(
                    (av1_tpl_ptr_pos
                        as unsafe extern "C-unwind" fn(
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            uint8_t,
                        ) -> core::ffi::c_int)(
                        mi_row + row_offset,
                        mi_col + col_offset,
                        (*tpl_frame).stride,
                        block_mis_log2,
                    ) as isize,
                ) as *mut TplDepStats;
            let mut this_refmv: int_mv = (*tpl_stats).mv[ref_idx as usize];
            lower_mv_precision(&raw mut this_refmv.as_mv, allow_hp, is_integer);
            check_mv_err_and_update(cur_mv.as_mv, this_refmv.as_mv, &raw mut mv_err);
        }
        i += 1;
    }
    let mut gm_mv: int_mv = int_mv {
        as_int: 0 as core::ffi::c_int as uint32_t,
    };
    if (*cm).global_motion[(ref_idx + LAST_FRAME as core::ffi::c_int) as usize].wmtype
        as core::ffi::c_int > TRANSLATION as core::ffi::c_int
    {
        let bsize: BLOCK_SIZE = convert_length_to_bsize(
            (*tpl_data).tpl_bsize_1d as core::ffi::c_int,
        ) as BLOCK_SIZE;
        gm_mv = gm_get_motion_vector(
            &raw mut *((*cm).global_motion)
                .as_mut_ptr()
                .offset((ref_idx + LAST_FRAME as core::ffi::c_int) as isize),
            allow_hp,
            bsize,
            mi_col,
            mi_row,
            is_integer,
        );
    }
    check_mv_err_and_update(cur_mv.as_mv, gm_mv.as_mv, &raw mut mv_err);
    return mv_err;
}
unsafe extern "C-unwind" fn get_temporal_mvpred_err(
    mut cm: *mut AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut num_mi_rows: core::ffi::c_int,
    mut num_mi_cols: core::ffi::c_int,
    mut ref_idx: core::ffi::c_int,
    mut cur_mv: int_mv,
    mut allow_hp: core::ffi::c_int,
    mut is_integer: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut ref_buf: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (ref_idx + LAST_FRAME as core::ffi::c_int) as MV_REFERENCE_FRAME,
    );
    if ref_buf.is_null() {
        return INT32_MAX;
    }
    let mut cur_to_ref_dist: core::ffi::c_int = get_relative_dist(
        &raw mut (*(*cm).seq_params).order_hint_info,
        (*(*cm).cur_frame).order_hint as core::ffi::c_int,
        (*ref_buf).order_hint as core::ffi::c_int,
    );
    let mut mv_err: core::ffi::c_int = INT32_MAX;
    let mv_pred_pos_in_mis: [[core::ffi::c_int; 2]; 7] = [
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, 2 as core::ffi::c_int],
        [2 as core::ffi::c_int, 0 as core::ffi::c_int],
        [2 as core::ffi::c_int, 2 as core::ffi::c_int],
        [4 as core::ffi::c_int, -(2 as core::ffi::c_int)],
        [4 as core::ffi::c_int, 4 as core::ffi::c_int],
        [2 as core::ffi::c_int, 4 as core::ffi::c_int],
    ];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 7 as core::ffi::c_int {
        let mut row_offset: core::ffi::c_int = mv_pred_pos_in_mis[i
            as usize][0 as core::ffi::c_int as usize];
        let mut col_offset: core::ffi::c_int = mv_pred_pos_in_mis[i
            as usize][1 as core::ffi::c_int as usize];
        if !(is_inside_frame_border(
            mi_row,
            mi_col,
            row_offset,
            col_offset,
            num_mi_rows,
            num_mi_cols,
        ) == 0)
        {
            let mut ref_mvs: *const TPL_MV_REF = ((*cm).tpl_mvs)
                .offset(
                    ((mi_row + row_offset >> 1 as core::ffi::c_int)
                        * ((*cm).mi_params.mi_stride >> 1 as core::ffi::c_int)) as isize,
                )
                .offset((mi_col + col_offset >> 1 as core::ffi::c_int) as isize);
            if !((*ref_mvs).mfmv0.as_int == INVALID_MV as uint32_t) {
                let mut this_refmv: int_mv = int_mv { as_int: 0 };
                av1_get_mv_projection(
                    &raw mut this_refmv.as_mv,
                    (*ref_mvs).mfmv0.as_mv,
                    cur_to_ref_dist,
                    (*ref_mvs).ref_frame_offset as core::ffi::c_int,
                );
                lower_mv_precision(&raw mut this_refmv.as_mv, allow_hp, is_integer);
                check_mv_err_and_update(cur_mv.as_mv, this_refmv.as_mv, &raw mut mv_err);
            }
        }
        i += 1;
    }
    return mv_err;
}
unsafe extern "C-unwind" fn check_to_disable_ref_frame_mvs(mut cpi: *mut AV1_COMP) {
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    if !(*cm).features.allow_ref_frame_mvs
        || (*cpi).sf.hl_sf.ref_frame_mvs_lvl != 1 as core::ffi::c_int
    {
        return;
    }
    let tpl_idx: core::ffi::c_int = (*cpi).gf_frame_index as core::ffi::c_int;
    let tpl_data: *mut TplParams = &raw mut (*(*cpi).ppi).tpl_data;
    if av1_tpl_stats_ready(tpl_data, tpl_idx) == 0 {
        return;
    }
    let tpl_subpel_precision: SUBPEL_FORCE_STOP = (*cpi).sf.tpl_sf.subpel_force_stop;
    let allow_high_precision_mv: core::ffi::c_int = (tpl_subpel_precision
        as core::ffi::c_int == EIGHTH_PEL as core::ffi::c_int
        && (*cm).features.allow_high_precision_mv as core::ffi::c_int != 0)
        as core::ffi::c_int;
    let force_integer_mv: core::ffi::c_int = (tpl_subpel_precision as core::ffi::c_int
        == FULL_PEL as core::ffi::c_int
        || (*cm).features.cur_frame_force_integer_mv as core::ffi::c_int != 0)
        as core::ffi::c_int;
    let mut tpl_frame: *const TplDepFrame = &raw mut *((*tpl_data).tpl_frame)
        .offset(tpl_idx as isize) as *mut TplDepFrame;
    let mut tpl_ptr: *mut TplDepStats = (*tpl_frame).tpl_stats_ptr;
    let block_mis_log2: uint8_t = (*tpl_data).tpl_stats_block_mis_log2;
    let step: core::ffi::c_int = (1 as core::ffi::c_int)
        << block_mis_log2 as core::ffi::c_int;
    let mut accum_spatial_mvpred_err: uint64_t = 0 as uint64_t;
    let mut accum_best_err: uint64_t = 0 as uint64_t;
    let mut mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while mi_row < (*tpl_frame).mi_rows {
        let mut mi_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while mi_col < (*tpl_frame).mi_cols {
            let mut tpl_stats_ptr: *mut TplDepStats = &raw mut *tpl_ptr
                .offset(
                    (av1_tpl_ptr_pos
                        as unsafe extern "C-unwind" fn(
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            uint8_t,
                        ) -> core::ffi::c_int)(
                        mi_row,
                        mi_col,
                        (*tpl_frame).stride,
                        block_mis_log2,
                    ) as isize,
                ) as *mut TplDepStats;
            let cur_best_ref_idx: core::ffi::c_int = (*tpl_stats_ptr)
                .ref_frame_index[0 as core::ffi::c_int as usize] as core::ffi::c_int;
            if !(cur_best_ref_idx == NONE_FRAME as core::ffi::c_int) {
                let mut cur_mv: int_mv = (*tpl_stats_ptr).mv[cur_best_ref_idx as usize];
                lower_mv_precision(
                    &raw mut cur_mv.as_mv,
                    allow_high_precision_mv,
                    force_integer_mv,
                );
                let cur_spatial_mvpred_err: core::ffi::c_int = get_spatial_mvpred_err(
                    cm,
                    tpl_data,
                    tpl_idx,
                    mi_row,
                    mi_col,
                    cur_best_ref_idx,
                    cur_mv,
                    allow_high_precision_mv,
                    force_integer_mv,
                ) as core::ffi::c_int;
                let cur_temporal_mvpred_err: core::ffi::c_int = get_temporal_mvpred_err(
                    cm,
                    mi_row,
                    mi_col,
                    (*tpl_frame).mi_rows,
                    (*tpl_frame).mi_cols,
                    cur_best_ref_idx,
                    cur_mv,
                    allow_high_precision_mv,
                    force_integer_mv,
                ) as core::ffi::c_int;
                let cur_best_err: core::ffi::c_int = if cur_spatial_mvpred_err
                    < cur_temporal_mvpred_err
                {
                    cur_spatial_mvpred_err
                } else {
                    cur_temporal_mvpred_err
                };
                accum_spatial_mvpred_err = accum_spatial_mvpred_err
                    .wrapping_add(cur_spatial_mvpred_err as uint64_t);
                accum_best_err = accum_best_err.wrapping_add(cur_best_err as uint64_t);
            }
            mi_col += step;
        }
        mi_row += step;
    }
    let threshold: core::ffi::c_float = get_thresh_based_on_q(
        (*cm).quant_params.base_qindex,
        (*cpi).oxcf.speed,
    ) as core::ffi::c_float;
    let mv_err_reduction: core::ffi::c_float = accum_spatial_mvpred_err
        .wrapping_sub(accum_best_err) as core::ffi::c_float;
    if mv_err_reduction <= threshold * accum_spatial_mvpred_err as core::ffi::c_float {
        (*cm).features.allow_ref_frame_mvs = 0 as core::ffi::c_int != 0;
    }
}
#[inline]
unsafe extern "C-unwind" fn encode_frame_internal(mut cpi: *mut AV1_COMP) {
    let td: *mut ThreadData = &raw mut (*cpi).td;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *mut CommonModeInfoParams = &raw mut (*cm).mi_params;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let rdc: *mut RD_COUNTS = &raw mut (*cpi).td.rd_counts;
    let frame_probs: *mut FrameProbInfo = &raw mut (*(*cpi).ppi).frame_probs;
    let intrabc_hash_info: *mut IntraBCHashInfo = &raw mut (*x).intrabc_hash_info;
    let mt_info: *mut MultiThreadInfo = &raw mut (*cpi).mt_info;
    let enc_row_mt: *mut AV1EncRowMultiThreadInfo = &raw mut (*mt_info).enc_row_mt;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let deltaq_mode: DELTAQ_MODE = (*oxcf).q_cfg.deltaq_mode;
    let mut i: core::ffi::c_int = 0;
    if (*cpi).sf.rt_sf.use_nonrd_pick_mode == 0 {
        ((*mi_params).setup_mi)
            .expect("non-null function pointer")(mi_params as *mut CommonModeInfoParams);
    }
    set_mi_offsets(mi_params, xd, 0 as core::ffi::c_int, 0 as core::ffi::c_int);
    memset(
        (*td).counts as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FRAME_COUNTS>() as size_t,
    );
    memset(
        &raw mut (*rdc).tx_type_used as *mut [[core::ffi::c_int; 16]; 19]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 16]; 19]>() as size_t,
    );
    memset(
        &raw mut (*rdc).obmc_used as *mut [[core::ffi::c_int; 2]; 22]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 2]; 22]>() as size_t,
    );
    memset(
        &raw mut (*rdc).warped_used as *mut [core::ffi::c_int; 2] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t,
    );
    memset(
        &raw mut (*rdc).seg_tmp_pred_cost as *mut [uint64_t; 2] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint64_t; 2]>() as size_t,
    );
    (*cpi).intrabc_used = 0 as core::ffi::c_int;
    if av1_superres_scaled(cm) != 0 {
        (*features).allow_intrabc = 0 as core::ffi::c_int != 0;
    }
    (*features).allow_intrabc = ((*features).allow_intrabc as core::ffi::c_int
        & (*oxcf).kf_cfg.enable_intrabc as core::ffi::c_int) != 0;
    if (*features).allow_warped_motion as core::ffi::c_int != 0
        && (*cpi).sf.inter_sf.prune_warped_prob_thresh > 0 as core::ffi::c_int
    {
        let update_type: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        let mut warped_probability: core::ffi::c_int = (*frame_probs)
            .warped_probs[update_type as usize];
        if warped_probability < (*cpi).sf.inter_sf.prune_warped_prob_thresh {
            (*features).allow_warped_motion = 0 as core::ffi::c_int != 0;
        }
    }
    let mut hash_table_created: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_stat_generation_stage(cpi) == 0 && av1_use_hash_me(cpi) != 0
        && ((*cpi).sf.rt_sf.use_nonrd_pick_mode == 0
            || (*cpi).sf.rt_sf.rt_use_intrabc != 0)
    {
        let pic_width: core::ffi::c_int = (*(*cpi).source)
            .c2rust_unnamed_1
            .c2rust_unnamed
            .y_crop_width;
        let pic_height: core::ffi::c_int = (*(*cpi).source)
            .c2rust_unnamed_2
            .c2rust_unnamed
            .y_crop_height;
        let mut block_hash_values: [*mut uint32_t; 2] = [
            0 as *mut uint32_t,
            0 as *mut uint32_t,
        ];
        let mut error: bool = false_0 != 0;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 2 as core::ffi::c_int {
            block_hash_values[j as usize] = aom_malloc(
                (::core::mem::size_of::<uint32_t>() as size_t)
                    .wrapping_mul(pic_width as size_t)
                    .wrapping_mul(pic_height as size_t),
            ) as *mut uint32_t;
            if (block_hash_values[j as usize]).is_null() {
                error = true_0 != 0;
                break;
            } else {
                j += 1;
            }
        }
        av1_hash_table_init(intrabc_hash_info);
        if error as core::ffi::c_int != 0
            || !av1_hash_table_create(&raw mut (*intrabc_hash_info).intrabc_hash_table)
        {
            free_block_hash_buffers(block_hash_values.as_mut_ptr());
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Error allocating intrabc_hash_table and buffers\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        hash_table_created = 1 as core::ffi::c_int;
        av1_generate_block_2x2_hash_value(
            (*cpi).source,
            block_hash_values[0 as core::ffi::c_int as usize],
        );
        let min_alloc_size: core::ffi::c_int = block_size_wide[(*mi_params)
            .mi_alloc_bsize as usize] as core::ffi::c_int;
        let mut max_sb_size: core::ffi::c_int = (1 as core::ffi::c_int)
            << (*(*cm).seq_params).mib_size_log2 + MI_SIZE_LOG2;
        if (*cpi).sf.mv_sf.hash_max_8x8_intrabc_blocks != 0 {
            max_sb_size = if (8 as core::ffi::c_int) < max_sb_size {
                8 as core::ffi::c_int
            } else {
                max_sb_size
            };
        }
        let mut src_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut size: core::ffi::c_int = 4 as core::ffi::c_int;
        while size <= max_sb_size {
            let dst_idx: core::ffi::c_int = (src_idx == 0) as core::ffi::c_int;
            av1_generate_block_hash_value(
                intrabc_hash_info,
                (*cpi).source,
                size,
                block_hash_values[src_idx as usize],
                block_hash_values[dst_idx as usize],
            );
            if size >= min_alloc_size
                && !av1_add_to_hash_map_by_row_with_precal_data(
                    &raw mut (*intrabc_hash_info).intrabc_hash_table,
                    block_hash_values[dst_idx as usize],
                    pic_width,
                    pic_height,
                    size,
                )
            {
                error = true_0 != 0;
                break;
            } else {
                size *= 2 as core::ffi::c_int;
                src_idx = (src_idx == 0) as core::ffi::c_int;
            }
        }
        free_block_hash_buffers(block_hash_values.as_mut_ptr());
        if error {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Error adding data to intrabc_hash_table\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    let mut quant_params: *const CommonQuantParams = &raw mut (*cm).quant_params;
    i = 0 as core::ffi::c_int;
    while i < MAX_SEGMENTS {
        let qindex: core::ffi::c_int = if (*cm).seg.enabled as core::ffi::c_int != 0 {
            av1_get_qindex(&raw mut (*cm).seg, i, (*quant_params).base_qindex)
                as core::ffi::c_int
        } else {
            (*quant_params).base_qindex
        };
        (*xd).lossless[i as usize] = (qindex == 0 as core::ffi::c_int
            && (*quant_params).y_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).u_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).u_ac_delta_q == 0 as core::ffi::c_int
            && (*quant_params).v_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).v_ac_delta_q == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        if (*xd).lossless[i as usize] != 0 {
            (*cpi).enc_seg.has_lossless_segment = 1 as core::ffi::c_int != 0;
        }
        (*xd).qindex[i as usize] = qindex;
        if (*xd).lossless[i as usize] != 0 {
            (*cpi).optimize_seg_arr[i as usize] = NO_TRELLIS_OPT as core::ffi::c_int
                as TRELLIS_OPT_TYPE;
        } else {
            (*cpi).optimize_seg_arr[i as usize] = (*cpi).sf.rd_sf.optimize_coefficients;
        }
        i += 1;
    }
    (*features).coded_lossless = is_coded_lossless(cm, xd) != 0;
    (*features).all_lossless = (*features).coded_lossless as core::ffi::c_int != 0
        && av1_superres_scaled(cm) == 0;
    (*cm).delta_q_info.delta_q_res = 0 as core::ffi::c_int;
    if (*cpi).use_ducky_encode != 0 {
        (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_DUCKY_ENCODE;
    } else if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        != CYCLIC_REFRESH_AQ as core::ffi::c_int && (*cpi).roi.enabled == 0
    {
        if deltaq_mode as core::ffi::c_int == DELTA_Q_OBJECTIVE as core::ffi::c_int {
            (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_OBJECTIVE;
        } else if deltaq_mode as core::ffi::c_int
            == DELTA_Q_PERCEPTUAL as core::ffi::c_int
        {
            (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_PERCEPTUAL;
        } else if deltaq_mode as core::ffi::c_int
            == DELTA_Q_PERCEPTUAL_AI as core::ffi::c_int
        {
            (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_PERCEPTUAL;
        } else if deltaq_mode as core::ffi::c_int
            == DELTA_Q_USER_RATING_BASED as core::ffi::c_int
        {
            (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_PERCEPTUAL;
        } else if deltaq_mode as core::ffi::c_int == DELTA_Q_HDR as core::ffi::c_int {
            (*cm).delta_q_info.delta_q_res = DEFAULT_DELTA_Q_RES_PERCEPTUAL;
        } else if deltaq_mode as core::ffi::c_int
            == DELTA_Q_VARIANCE_BOOST as core::ffi::c_int
        {
            (*cm).delta_q_info.delta_q_res = aom_get_variance_boost_delta_q_res(
                (*quant_params).base_qindex,
            );
        }
        (*cm).delta_q_info.delta_lf_res = DEFAULT_DELTA_LF_RES;
        (*cm).delta_q_info.delta_q_present_flag = (deltaq_mode as core::ffi::c_int
            != NO_DELTA_Q as core::ffi::c_int) as core::ffi::c_int;
        if (*cm).delta_q_info.delta_q_present_flag != 0 {
            if deltaq_mode as core::ffi::c_int == DELTA_Q_OBJECTIVE as core::ffi::c_int
                && enable_delta_q(cpi) == 0
            {
                (*cm).delta_q_info.delta_q_present_flag = 0 as core::ffi::c_int;
            }
            if deltaq_mode as core::ffi::c_int == DELTA_Q_OBJECTIVE as core::ffi::c_int
                && (*cm).delta_q_info.delta_q_present_flag != 0
            {
                (*cm).delta_q_info.delta_q_present_flag &= allow_deltaq_mode(cpi);
            }
        }
        (*cpi).deltaq_used = 0 as core::ffi::c_int;
        (*cm).delta_q_info.delta_lf_present_flag = ((*cm)
            .delta_q_info
            .delta_q_present_flag != 0
            && (*oxcf).tool_cfg.enable_deltalf_mode as core::ffi::c_int != 0)
            as core::ffi::c_int;
        (*cm).delta_q_info.delta_lf_multi = DEFAULT_DELTA_LF_MULTI;
        (*cm).delta_q_info.delta_q_present_flag
            &= ((*quant_params).base_qindex > 0 as core::ffi::c_int) as core::ffi::c_int;
        (*cm).delta_q_info.delta_lf_present_flag
            &= ((*quant_params).base_qindex > 0 as core::ffi::c_int) as core::ffi::c_int;
    } else if (*(*cpi).cyclic_refresh).apply_cyclic_refresh != 0
        || (*cpi).svc.number_temporal_layers == 1 as core::ffi::c_int
    {
        (*(*cpi).cyclic_refresh).actual_num_seg1_blocks = 0 as core::ffi::c_int;
        (*(*cpi).cyclic_refresh).actual_num_seg2_blocks = 0 as core::ffi::c_int;
    }
    (*cpi).rc.cnt_zeromv = 0 as core::ffi::c_int;
    (*cpi).cb_delta_rdmult_enabled = enable_delta_rdmult(cpi);
    av1_frame_init_quantizer(cpi as *mut AV1_COMP);
    init_encode_frame_mb_context(cpi);
    set_default_interp_skip_flags(cm, &raw mut (*cpi).interp_search_flags);
    if !((*cm).prev_frame).is_null()
        && (*(*cm).prev_frame).seg.enabled as core::ffi::c_int != 0
        && (*cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
    {
        (*cm).last_frame_seg_map = (*(*cm).prev_frame).seg_map;
    } else {
        (*cm).last_frame_seg_map = 0 as *mut uint8_t;
    }
    if (*features).allow_intrabc as core::ffi::c_int != 0
        || (*features).coded_lossless as core::ffi::c_int != 0
    {
        av1_set_default_ref_deltas(((*cm).lf.ref_deltas).as_mut_ptr());
        av1_set_default_mode_deltas(((*cm).lf.mode_deltas).as_mut_ptr());
    } else if !((*cm).prev_frame).is_null() {
        memcpy(
            ((*cm).lf.ref_deltas).as_mut_ptr() as *mut core::ffi::c_void,
            ((*(*cm).prev_frame).ref_deltas).as_mut_ptr() as *const core::ffi::c_void,
            REF_FRAMES as core::ffi::c_int as size_t,
        );
        memcpy(
            ((*cm).lf.mode_deltas).as_mut_ptr() as *mut core::ffi::c_void,
            ((*(*cm).prev_frame).mode_deltas).as_mut_ptr() as *const core::ffi::c_void,
            MAX_MODE_LF_DELTAS as size_t,
        );
    }
    (*cm).lf.mode_ref_delta_enabled = (*oxcf).algo_cfg.mode_ref_delta_enabled as uint8_t;
    memcpy(
        ((*(*cm).cur_frame).ref_deltas).as_mut_ptr() as *mut core::ffi::c_void,
        ((*cm).lf.ref_deltas).as_mut_ptr() as *const core::ffi::c_void,
        REF_FRAMES as core::ffi::c_int as size_t,
    );
    memcpy(
        ((*(*cm).cur_frame).mode_deltas).as_mut_ptr() as *mut core::ffi::c_void,
        ((*cm).lf.mode_deltas).as_mut_ptr() as *const core::ffi::c_void,
        MAX_MODE_LF_DELTAS as size_t,
    );
    (*cpi).all_one_sided_refs = if frame_is_intra_only(cm) != 0 {
        0 as core::ffi::c_int
    } else {
        refs_are_one_sided(cm)
    };
    (*cpi).prune_ref_frame_mask = 0 as core::ffi::c_int;
    setup_prune_ref_frame_mask(cpi);
    setup_keep_ref_frame_mask(cpi);
    (*x).txfm_search_info.txb_split_count = 0 as core::ffi::c_uint;
    av1_compute_global_motion_facade(cpi as *mut AV1_COMP);
    av1_calculate_ref_frame_side(cm);
    (*features).allow_ref_frame_mvs = ((*features).allow_ref_frame_mvs
        as core::ffi::c_int
        & !((*cpi).sf.hl_sf.ref_frame_mvs_lvl == 2 as core::ffi::c_int)
            as core::ffi::c_int) != 0;
    if (*features).allow_ref_frame_mvs {
        av1_setup_motion_field(cm);
    }
    check_to_disable_ref_frame_mvs(cpi);
    (*cm).current_frame.skip_mode_info.skip_mode_flag = check_skip_mode_enabled(cpi);
    av1_initialize_rd_consts(cpi as *mut AV1_COMP);
    av1_set_sad_per_bit(cpi, &raw mut (*x).sadperbit, (*quant_params).base_qindex);
    populate_thresh_to_force_zeromv_skip(cpi);
    (*enc_row_mt).sync_read_ptr = Some(
        av1_row_mt_sync_read_dummy
            as unsafe extern "C-unwind" fn(
                *mut AV1EncRowMultiThreadSync,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut AV1EncRowMultiThreadSync,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    (*enc_row_mt).sync_write_ptr = Some(
        av1_row_mt_sync_write_dummy
            as unsafe extern "C-unwind" fn(
                *mut AV1EncRowMultiThreadSync,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut AV1EncRowMultiThreadSync,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
        >;
    (*mt_info).row_mt_enabled = 0 as core::ffi::c_int != 0;
    (*mt_info).pack_bs_mt_enabled = (if (*mt_info)
        .num_mod_workers[MOD_PACK_BS as core::ffi::c_int as usize]
        < (*cm).tiles.cols * (*cm).tiles.rows
    {
        (*mt_info).num_mod_workers[MOD_PACK_BS as core::ffi::c_int as usize]
    } else {
        (*cm).tiles.cols * (*cm).tiles.rows
    }) > 1 as core::ffi::c_int;
    if (*oxcf).row_mt as core::ffi::c_int != 0
        && (*mt_info).num_workers > 1 as core::ffi::c_int
    {
        (*mt_info).row_mt_enabled = 1 as core::ffi::c_int != 0;
        (*enc_row_mt).sync_read_ptr = Some(
            av1_row_mt_sync_read
                as unsafe extern "C-unwind" fn(
                    *mut AV1EncRowMultiThreadSync,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut AV1EncRowMultiThreadSync,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
        (*enc_row_mt).sync_write_ptr = Some(
            av1_row_mt_sync_write
                as unsafe extern "C-unwind" fn(
                    *mut AV1EncRowMultiThreadSync,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C-unwind" fn(
                    *mut AV1EncRowMultiThreadSync,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
        av1_encode_tiles_row_mt(cpi as *mut AV1_COMP);
    } else if (if (*mt_info).num_workers < (*cm).tiles.cols * (*cm).tiles.rows {
        (*mt_info).num_workers
    } else {
        (*cm).tiles.cols * (*cm).tiles.rows
    }) > 1 as core::ffi::c_int
    {
        av1_encode_tiles_mt(cpi as *mut AV1_COMP);
    } else {
        let use_nonrd_mode: core::ffi::c_int = (*cpi).sf.rt_sf.use_nonrd_pick_mode;
        if use_nonrd_mode != 0 {
            (*td).pc_root = av1_alloc_pc_tree_node((*(*cm).seq_params).sb_size);
            if ((*td).pc_root).is_null() {
                aom_internal_error(
                    (*xd).error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate PC_TREE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else {
            (*td).pc_root = 0 as *mut PC_TREE;
        }
        encode_tiles(cpi);
        av1_free_pc_tree_recursive(
            (*td).pc_root,
            av1_num_planes(cm),
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (*cpi).sf.part_sf.partition_search_type,
        );
        (*td).pc_root = 0 as *mut PC_TREE;
    }
    if (*features).allow_intrabc as core::ffi::c_int != 0 && (*cpi).intrabc_used == 0 {
        (*features).allow_intrabc = 0 as core::ffi::c_int != 0;
    }
    if (*features).allow_intrabc {
        (*cm).delta_q_info.delta_lf_present_flag = 0 as core::ffi::c_int;
    }
    if (*cm).delta_q_info.delta_q_present_flag != 0
        && (*cpi).deltaq_used == 0 as core::ffi::c_int
    {
        (*cm).delta_q_info.delta_q_present_flag = 0 as core::ffi::c_int;
    }
    let eval_type: MODE_EVAL_TYPE = (if (*cpi)
        .sf
        .winner_mode_sf
        .enable_winner_mode_for_tx_size_srch != 0
    {
        WINNER_MODE_EVAL as core::ffi::c_int
    } else {
        DEFAULT_EVAL as core::ffi::c_int
    }) as MODE_EVAL_TYPE;
    let tx_search_type: TX_SIZE_SEARCH_METHOD = (*cpi)
        .winner_mode_params
        .tx_size_search_methods[eval_type as usize];
    if !((*oxcf).txfm_cfg.enable_tx64 as core::ffi::c_int != 0
        || tx_search_type as core::ffi::c_int != USE_LARGESTALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"encode_frame_internal\0"))
                .as_ptr(),
            b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2496 as core::ffi::c_int,
            b"oxcf->txfm_cfg.enable_tx64 || tx_search_type != USE_LARGESTALL\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*features).tx_mode = select_tx_mode(cm, tx_search_type);
    if (*(*cpi).ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
        > 0 as core::ffi::c_int
    {
        (*cpi).do_update_frame_probs_txtype[(*cpi).num_frame_recode as usize] = (*cpi)
            .sf
            .tx_sf
            .tx_type_search
            .prune_tx_type_using_stats;
        (*cpi).do_update_frame_probs_obmc[(*cpi).num_frame_recode as usize] = ((*cpi)
            .sf
            .inter_sf
            .prune_obmc_prob_thresh > 0 as core::ffi::c_int
            && (*cpi).sf.inter_sf.prune_obmc_prob_thresh < INT_MAX) as core::ffi::c_int;
        (*cpi).do_update_frame_probs_warp[(*cpi).num_frame_recode as usize] = ((*features)
            .allow_warped_motion as core::ffi::c_int != 0
            && (*cpi).sf.inter_sf.prune_warped_prob_thresh > 0 as core::ffi::c_int)
            as core::ffi::c_int;
        (*cpi).do_update_frame_probs_interpfilter[(*cpi).num_frame_recode as usize] = ((*cm)
            .current_frame
            .frame_type as core::ffi::c_int != KEY_FRAME as core::ffi::c_int
            && (*cpi).sf.interp_sf.adaptive_interp_filter_search == 2 as core::ffi::c_int
            && (*features).interp_filter as core::ffi::c_int
                == SWITCHABLE as core::ffi::c_int) as core::ffi::c_int;
    }
    if (*cpi).sf.tx_sf.tx_type_search.prune_tx_type_using_stats != 0
        || (*cpi).sf.tx_sf.tx_type_search.fast_inter_tx_type_prob_thresh != INT_MAX
            && (*cpi).sf.tx_sf.tx_type_search.fast_inter_tx_type_prob_thresh
                != 0 as core::ffi::c_int
    {
        let update_type_0: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        i = 0 as core::ffi::c_int;
        while i < TX_SIZES_ALL as core::ffi::c_int {
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut j_0: core::ffi::c_int = 0;
            let mut left: core::ffi::c_int = MAX_TX_TYPE_PROB;
            j_0 = 0 as core::ffi::c_int;
            while j_0 < TX_TYPES as core::ffi::c_int {
                sum += (*cpi).td.rd_counts.tx_type_used[i as usize][j_0 as usize];
                j_0 += 1;
            }
            j_0 = TX_TYPES as core::ffi::c_int - 1 as core::ffi::c_int;
            while j_0 >= 0 as core::ffi::c_int {
                let mut update_txtype_frameprobs: core::ffi::c_int = 1
                    as core::ffi::c_int;
                let new_prob: core::ffi::c_int = if sum != 0 {
                    (MAX_TX_TYPE_PROB as int64_t
                        * (*cpi).td.rd_counts.tx_type_used[i as usize][j_0 as usize]
                            as int64_t / sum as int64_t) as core::ffi::c_int
                } else if j_0 != 0 {
                    0 as core::ffi::c_int
                } else {
                    MAX_TX_TYPE_PROB
                };
                if (*(*cpi).ppi)
                    .gf_group
                    .frame_parallel_level[(*cpi).gf_frame_index as usize]
                    > 0 as core::ffi::c_int
                {
                    update_txtype_frameprobs = 0 as core::ffi::c_int;
                    (*cpi)
                        .frame_new_probs[(*cpi).num_frame_recode as usize]
                        .tx_type_probs[update_type_0
                        as usize][i as usize][j_0 as usize] = new_prob;
                }
                if update_txtype_frameprobs != 0 {
                    let mut prob: core::ffi::c_int = (*frame_probs)
                        .tx_type_probs[update_type_0 as usize][i as usize][j_0 as usize]
                        + new_prob >> 1 as core::ffi::c_int;
                    left -= prob;
                    if j_0 == 0 as core::ffi::c_int {
                        prob += left;
                    }
                    (*frame_probs)
                        .tx_type_probs[update_type_0
                        as usize][i as usize][j_0 as usize] = prob;
                }
                j_0 -= 1;
            }
            i += 1;
        }
    }
    if (*cm).seg.enabled != 0 {
        (*cm).seg.temporal_update = 1 as uint8_t;
        if (*rdc).seg_tmp_pred_cost[0 as core::ffi::c_int as usize]
            < (*rdc).seg_tmp_pred_cost[1 as core::ffi::c_int as usize]
        {
            (*cm).seg.temporal_update = 0 as uint8_t;
        }
    }
    if (*cpi).sf.inter_sf.prune_obmc_prob_thresh > 0 as core::ffi::c_int
        && (*cpi).sf.inter_sf.prune_obmc_prob_thresh < INT_MAX
    {
        let update_type_1: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        i = 0 as core::ffi::c_int;
        while i < BLOCK_SIZES_ALL as core::ffi::c_int {
            let mut sum_0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut update_obmc_frameprobs: core::ffi::c_int = 1 as core::ffi::c_int;
            let mut j_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_1 < 2 as core::ffi::c_int {
                sum_0 += (*cpi).td.rd_counts.obmc_used[i as usize][j_1 as usize];
                j_1 += 1;
            }
            let new_prob_0: core::ffi::c_int = if sum_0 != 0 {
                128 as core::ffi::c_int
                    * (*cpi)
                        .td
                        .rd_counts
                        .obmc_used[i as usize][1 as core::ffi::c_int as usize] / sum_0
            } else {
                0 as core::ffi::c_int
            };
            if (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
            {
                update_obmc_frameprobs = 0 as core::ffi::c_int;
                (*cpi)
                    .frame_new_probs[(*cpi).num_frame_recode as usize]
                    .obmc_probs[update_type_1 as usize][i as usize] = new_prob_0;
            }
            if update_obmc_frameprobs != 0 {
                (*frame_probs).obmc_probs[update_type_1 as usize][i as usize] = (*frame_probs)
                    .obmc_probs[update_type_1 as usize][i as usize] + new_prob_0
                    >> 1 as core::ffi::c_int;
            }
            i += 1;
        }
    }
    if (*features).allow_warped_motion as core::ffi::c_int != 0
        && (*cpi).sf.inter_sf.prune_warped_prob_thresh > 0 as core::ffi::c_int
    {
        let update_type_2: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        let mut update_warp_frameprobs: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut sum_1: core::ffi::c_int = 0 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        while i < 2 as core::ffi::c_int {
            sum_1 += (*cpi).td.rd_counts.warped_used[i as usize];
            i += 1;
        }
        let new_prob_1: core::ffi::c_int = if sum_1 != 0 {
            128 as core::ffi::c_int
                * (*cpi).td.rd_counts.warped_used[1 as core::ffi::c_int as usize] / sum_1
        } else {
            0 as core::ffi::c_int
        };
        if (*(*cpi).ppi).gf_group.frame_parallel_level[(*cpi).gf_frame_index as usize]
            > 0 as core::ffi::c_int
        {
            update_warp_frameprobs = 0 as core::ffi::c_int;
            (*cpi)
                .frame_new_probs[(*cpi).num_frame_recode as usize]
                .warped_probs[update_type_2 as usize] = new_prob_1;
        }
        if update_warp_frameprobs != 0 {
            (*frame_probs).warped_probs[update_type_2 as usize] = (*frame_probs)
                .warped_probs[update_type_2 as usize] + new_prob_1
                >> 1 as core::ffi::c_int;
        }
    }
    if (*cm).current_frame.frame_type as core::ffi::c_int
        != KEY_FRAME as core::ffi::c_int
        && (*cpi).sf.interp_sf.adaptive_interp_filter_search == 2 as core::ffi::c_int
        && (*features).interp_filter as core::ffi::c_int
            == SWITCHABLE as core::ffi::c_int
    {
        let update_type_3: FRAME_UPDATE_TYPE = get_frame_update_type(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as FRAME_UPDATE_TYPE;
        i = 0 as core::ffi::c_int;
        while i
            < (SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int)
                * 4 as core::ffi::c_int
        {
            let mut sum_2: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut j_2: core::ffi::c_int = 0;
            let mut left_0: core::ffi::c_int = 1536 as core::ffi::c_int;
            j_2 = 0 as core::ffi::c_int;
            while j_2 < SWITCHABLE_FILTERS as core::ffi::c_int {
                sum_2 = (sum_2 as core::ffi::c_uint)
                    .wrapping_add(
                        (*(*cpi).td.counts).switchable_interp[i as usize][j_2 as usize],
                    ) as core::ffi::c_int as core::ffi::c_int;
                j_2 += 1;
            }
            j_2 = SWITCHABLE_FILTERS as core::ffi::c_int - 1 as core::ffi::c_int;
            while j_2 >= 0 as core::ffi::c_int {
                let mut update_interpfilter_frameprobs: core::ffi::c_int = 1
                    as core::ffi::c_int;
                let new_prob_2: core::ffi::c_int = (if sum_2 != 0 {
                    (1536 as core::ffi::c_uint)
                        .wrapping_mul(
                            (*(*cpi).td.counts)
                                .switchable_interp[i as usize][j_2 as usize],
                        )
                        .wrapping_div(sum_2 as core::ffi::c_uint)
                } else {
                    (if j_2 != 0 {
                        0 as core::ffi::c_int
                    } else {
                        1536 as core::ffi::c_int
                    }) as core::ffi::c_uint
                }) as core::ffi::c_int;
                if (*(*cpi).ppi)
                    .gf_group
                    .frame_parallel_level[(*cpi).gf_frame_index as usize]
                    > 0 as core::ffi::c_int
                {
                    update_interpfilter_frameprobs = 0 as core::ffi::c_int;
                    (*cpi)
                        .frame_new_probs[(*cpi).num_frame_recode as usize]
                        .switchable_interp_probs[update_type_3
                        as usize][i as usize][j_2 as usize] = new_prob_2;
                }
                if update_interpfilter_frameprobs != 0 {
                    let mut prob_0: core::ffi::c_int = (*frame_probs)
                        .switchable_interp_probs[update_type_3
                        as usize][i as usize][j_2 as usize] + new_prob_2
                        >> 1 as core::ffi::c_int;
                    left_0 -= prob_0;
                    if j_2 == 0 as core::ffi::c_int {
                        prob_0 += left_0;
                    }
                    (*frame_probs)
                        .switchable_interp_probs[update_type_3
                        as usize][i as usize][j_2 as usize] = prob_0;
                }
                j_2 -= 1;
            }
            i += 1;
        }
    }
    if hash_table_created != 0 {
        av1_hash_table_destroy(&raw mut (*intrabc_hash_info).intrabc_hash_table);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encode_frame(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    let rdc: *mut RD_COUNTS = &raw mut (*cpi).td.rd_counts;
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    (*features).reduced_tx_set_used = (*oxcf).txfm_cfg.reduced_tx_type_set;
    if (*cm).seg.enabled as core::ffi::c_int != 0
        && (*cm).seg.update_map as core::ffi::c_int != 0
    {
        let mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
        let mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
        let last_active_segid: core::ffi::c_int = (*cm).seg.last_active_segid;
        let mut map: *mut uint8_t = (*cpi).enc_seg.map;
        let mut mi_row: core::ffi::c_int = 0 as core::ffi::c_int;
        while mi_row < mi_rows {
            let mut mi_col: core::ffi::c_int = 0 as core::ffi::c_int;
            while mi_col < mi_cols {
                *map.offset(mi_col as isize) = (if (*map.offset(mi_col as isize)
                    as core::ffi::c_int) < last_active_segid
                {
                    *map.offset(mi_col as isize) as core::ffi::c_int
                } else {
                    last_active_segid
                }) as uint8_t;
                mi_col += 1;
            }
            map = map.offset(mi_cols as isize);
            mi_row += 1;
        }
    }
    av1_setup_frame_buf_refs(cm);
    enforce_max_ref_frames(
        cpi,
        &raw mut (*cpi).ref_frame_flags,
        ((*(*cm).cur_frame).ref_display_order_hint).as_mut_ptr(),
        (*cm).current_frame.display_order_hint,
    );
    set_rel_frame_dist(
        &raw mut (*cpi).common,
        &raw mut (*cpi).ref_frame_dist_info,
        (*cpi).ref_frame_flags,
    );
    av1_setup_frame_sign_bias(cm);
    if (*cpi).alloc_pyramid {
        if ((*(*cpi).source).y_pyramid).is_null() as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"av1_encode_frame\0"))
                    .as_ptr(),
                b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                2776 as core::ffi::c_int,
                b"cpi->source->y_pyramid\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut ref_frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
        while ref_frame <= ALTREF_FRAME as core::ffi::c_int {
            let buf: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                ref_frame as MV_REFERENCE_FRAME,
            );
            if !buf.is_null() {
                if ((*buf).buf.y_pyramid).is_null() as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 17],
                            [core::ffi::c_char; 17],
                        >(*b"av1_encode_frame\0"))
                            .as_ptr(),
                        b"encodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                        2780 as core::ffi::c_int,
                        b"buf->buf.y_pyramid\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            ref_frame += 1;
        }
    }
    (*rdc).newmv_or_intra_blocks = 0 as core::ffi::c_int;
    (*cpi).palette_pixel_num = 0 as core::ffi::c_int;
    if (*cpi).sf.hl_sf.frame_parameter_update != 0
        || (*cpi).sf.rt_sf.use_comp_ref_nonrd != 0
    {
        if frame_is_intra_only(cm) != 0 {
            (*current_frame).reference_mode = SINGLE_REFERENCE as core::ffi::c_int
                as REFERENCE_MODE;
        } else {
            (*current_frame).reference_mode = REFERENCE_MODE_SELECT as core::ffi::c_int
                as REFERENCE_MODE;
        }
        (*features).interp_filter = SWITCHABLE;
        if (*cm).tiles.large_scale != 0 {
            (*features).interp_filter = EIGHTTAP_REGULAR;
        }
        (*features).switchable_motion_mode = is_switchable_motion_mode_allowed(
            (*features).allow_warped_motion,
            (*oxcf).motion_mode_cfg.enable_obmc,
        );
        (*rdc).compound_ref_used_flag = 0 as core::ffi::c_int;
        (*rdc).skip_mode_used_flag = 0 as core::ffi::c_int;
        encode_frame_internal(cpi);
        if (*current_frame).reference_mode as core::ffi::c_int
            == REFERENCE_MODE_SELECT as core::ffi::c_int
        {
            if (*rdc).compound_ref_used_flag == 0 as core::ffi::c_int {
                (*current_frame).reference_mode = SINGLE_REFERENCE as core::ffi::c_int
                    as REFERENCE_MODE;
            }
        }
        let skip_mode_info: *mut SkipModeInfo = &raw mut (*current_frame).skip_mode_info;
        if frame_is_intra_only(cm) != 0
            || (*current_frame).reference_mode as core::ffi::c_int
                == SINGLE_REFERENCE as core::ffi::c_int
        {
            (*skip_mode_info).skip_mode_allowed = 0 as core::ffi::c_int;
            (*skip_mode_info).skip_mode_flag = 0 as core::ffi::c_int;
        }
        if (*skip_mode_info).skip_mode_flag != 0
            && (*rdc).skip_mode_used_flag == 0 as core::ffi::c_int
        {
            (*skip_mode_info).skip_mode_flag = 0 as core::ffi::c_int;
        }
        if (*cm).tiles.large_scale == 0 {
            if (*features).tx_mode as core::ffi::c_int
                == TX_MODE_SELECT as core::ffi::c_int
                && (*cpi).td.mb.txfm_search_info.txb_split_count
                    == 0 as core::ffi::c_uint
            {
                (*features).tx_mode = TX_MODE_LARGEST as core::ffi::c_int as TX_MODE;
            }
        }
    } else {
        if (*current_frame).reference_mode as core::ffi::c_int
            == REFERENCE_MODE_SELECT as core::ffi::c_int
        {
            (*current_frame).reference_mode = SINGLE_REFERENCE as core::ffi::c_int
                as REFERENCE_MODE;
        }
        encode_frame_internal(cpi);
    };
}

extern "C" {
    fn sqrt(_: core::ffi::c_double) -> core::ffi::c_double;
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
    fn llabs(_: core::ffi::c_longlong) -> core::ffi::c_longlong;
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
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
