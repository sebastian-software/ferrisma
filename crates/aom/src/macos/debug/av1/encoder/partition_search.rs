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

// Mechanically translated from upstream/aom/av1/encoder/partition_search.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_get_qindex(
        seg: *const segmentation,
        segment_id: core::ffi::c_int,
        base_qindex: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_reset_entropy_context(
        xd: *mut MACROBLOCKD,
        bsize: BLOCK_SIZE,
        num_planes: core::ffi::c_int,
    );
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_caq_select_segment(
        cpi: *const AV1_COMP,
        _: *mut macroblock,
        bs: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        projected_rate: core::ffi::c_int,
    );
    fn av1_find_mv_refs(
        cm: *const AV1_COMMON,
        xd: *const MACROBLOCKD,
        mi: *mut MB_MODE_INFO,
        ref_frame: MV_REFERENCE_FRAME,
        ref_mv_count: *mut uint8_t,
        ref_mv_stack: *mut [CANDIDATE_MV; 8],
        ref_mv_weight: *mut [uint16_t; 8],
        mv_ref_list: *mut [int_mv; 2],
        global_mvs: *mut int_mv,
        mode_context: *mut int16_t,
    );
    fn av1_findSamples(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        pts: *mut core::ffi::c_int,
        pts_inref: *mut core::ffi::c_int,
    ) -> uint8_t;
    fn av1_tokenize_sb_vartx(
        cpi: *const AV1_COMP,
        td: *mut ThreadData,
        dry_run: RUN_TYPE,
        bsize: BLOCK_SIZE,
        rate: *mut core::ffi::c_int,
        allow_update_cdf: uint8_t,
    );
    fn av1_cost_color_map(
        x: *const MACROBLOCK,
        plane: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
        type_0: COLOR_MAP_TYPE,
    ) -> core::ffi::c_int;
    fn av1_tokenize_color_map(
        x: *const MACROBLOCK,
        plane: core::ffi::c_int,
        t: *mut *mut TokenExtra,
        bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
        type_0: COLOR_MAP_TYPE,
        allow_update_cdf: core::ffi::c_int,
        counts: *mut FRAME_COUNTS,
    );
    fn av1_cyclic_reset_segment_skip(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        dry_run: RUN_TYPE,
    );
    fn av1_cyclic_refresh_get_rdmult(cr: *const CYCLIC_REFRESH) -> core::ffi::c_int;
    fn av1_init_plane_quantizers(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        segment_id: core::ffi::c_int,
        do_update: core::ffi::c_int,
    );
    static av1_prob_cost: [uint16_t; 128];
    fn av1_cost_tokens_from_cdf(
        costs: *mut core::ffi::c_int,
        cdf: *const aom_cdf_prob,
        inv_map: *const core::ffi::c_int,
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
    fn av1_get_scaled_ref_frame(
        cpi: *const AV1_COMP,
        ref_frame: core::ffi::c_int,
    ) -> *mut YV12_BUFFER_CONFIG;
    fn av1_mv_pred(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        ref_y_buffer: *mut uint8_t,
        ref_y_stride: core::ffi::c_int,
        ref_frame: core::ffi::c_int,
        block_size: BLOCK_SIZE,
    );
    fn av1_setup_pred_block(
        xd: *const MACROBLOCKD,
        dst: *mut buf_2d,
        src: *const YV12_BUFFER_CONFIG,
        scale: *const scale_factors,
        scale_uv: *const scale_factors,
        num_planes: core::ffi::c_int,
    );
    fn av1_encode_sb(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        dry_run: RUN_TYPE,
    );
    fn av1_encode_intra_block_plane(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        plane: core::ffi::c_int,
        dry_run: RUN_TYPE,
        enable_optimize_b: TRELLIS_OPT_TYPE,
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
    fn av1_reset_pmc(ctx: *mut PICK_MODE_CONTEXT);
    fn av1_copy_tree_context(
        dst_ctx: *mut PICK_MODE_CONTEXT,
        src_ctx: *mut PICK_MODE_CONTEXT,
    );
    fn av1_log_block_var(
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
        bs: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_setup_src_planes(
        x: *mut macroblock,
        src: *const yv12_buffer_config,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        num_planes: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    );
    static av1_wedge_params_lookup: [wedge_params_type; 22];
    fn av1_setup_dst_planes(
        planes: *mut macroblockd_plane,
        bsize: BLOCK_SIZE,
        src: *const YV12_BUFFER_CONFIG,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
    );
    fn av1_setup_pre_planes(
        xd: *mut MACROBLOCKD,
        idx: core::ffi::c_int,
        src: *const YV12_BUFFER_CONFIG,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        sf: *const scale_factors,
        num_planes: core::ffi::c_int,
    );
    fn av1_count_overlappable_neighbors(cm: *const AV1_COMMON, xd: *mut MACROBLOCKD);
    fn av1_update_intra_mb_txb_context(
        cpi: *const AV1_COMP,
        td: *mut ThreadData,
        dry_run: RUN_TYPE,
        bsize: BLOCK_SIZE,
        allow_update_cdf: uint8_t,
    );
    fn cfl_store_block(xd: *mut MACROBLOCKD, bsize: BLOCK_SIZE, tx_size: TX_SIZE);
    fn av1_get_pred_context_switchable_interp(
        xd: *const MACROBLOCKD,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_intra_inter_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_reference_mode_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_comp_reference_type_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_uni_comp_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_bwdref_p(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_comp_bwdref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p1(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p2(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p3(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p4(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p5(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_pred_context_single_ref_p6(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_perpixel_variance_facade(
        cpi: *const AV1_COMP,
        xd: *const MACROBLOCKD,
        ref_0: *const buf_2d,
        bsize: BLOCK_SIZE,
        plane: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn av1_rd_pick_intra_mode_sb(
        cpi: *const AV1_COMP,
        x: *mut macroblock,
        rd_cost: *mut RD_STATS,
        bsize: BLOCK_SIZE,
        ctx: *mut PICK_MODE_CONTEXT,
        best_rd: int64_t,
    );
    fn av1_rd_pick_inter_mode(
        cpi: *mut AV1_COMP,
        tile_data: *mut TileDataEnc,
        x: *mut macroblock,
        rd_cost: *mut RD_STATS,
        bsize: BLOCK_SIZE,
        ctx: *mut PICK_MODE_CONTEXT,
        best_rd_so_far: int64_t,
    );
    fn av1_nonrd_pick_intra_mode(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        rd_cost: *mut RD_STATS,
        bsize: BLOCK_SIZE,
        ctx: *mut PICK_MODE_CONTEXT,
    );
    fn av1_nonrd_pick_inter_mode_sb(
        cpi: *mut AV1_COMP,
        tile_data: *mut TileDataEnc,
        x: *mut macroblock,
        rd_cost: *mut RD_STATS,
        bsize: BLOCK_SIZE,
        ctx: *mut PICK_MODE_CONTEXT,
    );
    fn av1_rd_pick_inter_mode_sb_seg_skip(
        cpi: *const AV1_COMP,
        tile_data: *mut TileDataEnc,
        x: *mut macroblock,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        rd_cost: *mut RD_STATS,
        bsize: BLOCK_SIZE,
        ctx: *mut PICK_MODE_CONTEXT,
        best_rd_so_far: int64_t,
    );
    fn av1_active_h_edge(
        cpi: *const AV1_COMP,
        mi_row: core::ffi::c_int,
        mi_step: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_active_v_edge(
        cpi: *const AV1_COMP,
        mi_col: core::ffi::c_int,
        mi_step: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_cb_rdmult(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_set_ssim_rdmult(
        cpi: *const AV1_COMP,
        errorperbit: *mut core::ffi::c_int,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        rdmult: *mut core::ffi::c_int,
    );
    fn av1_update_state(
        cpi: *const AV1_COMP,
        td: *mut ThreadData,
        ctx: *const PICK_MODE_CONTEXT,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        dry_run: RUN_TYPE,
    );
    fn av1_update_inter_mode_stats(
        fc: *mut FRAME_CONTEXT,
        counts: *mut FRAME_COUNTS,
        mode: PREDICTION_MODE,
        mode_context: int16_t,
    );
    fn av1_sum_intra_stats(
        cm: *const AV1_COMMON,
        counts: *mut FRAME_COUNTS,
        xd: *mut MACROBLOCKD,
        mbmi: *const MB_MODE_INFO,
        above_mi: *const MB_MODE_INFO,
        left_mi: *const MB_MODE_INFO,
        intraonly: core::ffi::c_int,
    );
    fn av1_restore_context(
        x: *mut MACROBLOCK,
        ctx: *const RD_SEARCH_MACROBLOCK_CONTEXT,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        num_planes: core::ffi::c_int,
    );
    fn av1_save_context(
        x: *const MACROBLOCK,
        ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        num_planes: core::ffi::c_int,
    );
    fn av1_is_leaf_split_partition(
        cm: *mut AV1_COMMON,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_update_picked_ref_frames_mask(
        x: *mut MACROBLOCK,
        ref_type: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        mib_size: core::ffi::c_int,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    );
    fn av1_update_mv_stats(
        mv: *const MV,
        ref_0: *const MV,
        mvctx: *mut nmv_context,
        precision: MvSubpelPrecision,
    );
    fn av1_get_ref_mv(x: *const MACROBLOCK, ref_idx: core::ffi::c_int) -> int_mv;
    fn av1_find_best_ref_mvs_from_stack(
        allow_hp: core::ffi::c_int,
        mbmi_ext: *const MB_MODE_INFO_EXT,
        ref_frame: MV_REFERENCE_FRAME,
        nearest_mv: *mut int_mv,
        near_mv: *mut int_mv,
        is_integer: core::ffi::c_int,
    );
    fn av1_calc_normalized_variance(
        vf: aom_variance_fn_t,
        buf: *const uint8_t,
        stride: core::ffi::c_int,
        is_hbd: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_simple_motion_search_sse_var(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        ref_0: core::ffi::c_int,
        start_mv: FULLPEL_MV,
        num_planes: core::ffi::c_int,
        use_subpixel: core::ffi::c_int,
        sse: *mut core::ffi::c_uint,
        var: *mut core::ffi::c_uint,
    ) -> int_mv;
    fn av1_simple_motion_search_early_term_none(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
        none_rdc: *const RD_STATS,
        part_state: *mut PartitionSearchState,
    );
    fn av1_ml_early_term_after_split(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
        best_rd: int64_t,
        part_none_rd: int64_t,
        part_split_rd: int64_t,
        split_block_rd: *mut int64_t,
        part_state: *mut PartitionSearchState,
    );
    fn av1_ml_prune_rect_partition(
        cpi: *mut AV1_COMP,
        x: *const MACROBLOCK,
        best_rd: int64_t,
        none_rd: int64_t,
        split_rd: *const int64_t,
        part_state: *mut PartitionSearchState,
    );
    fn av1_ml_prune_4_partition(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        part_ctx: core::ffi::c_int,
        best_rd: int64_t,
        part_state: *mut PartitionSearchState,
        part4_allowed: *mut core::ffi::c_int,
        pb_source_variance: core::ffi::c_uint,
    );
    fn av1_ml_predict_breakout(
        cpi: *mut AV1_COMP,
        x: *const MACROBLOCK,
        rd_stats: *const RD_STATS,
        pb_source_variance: core::ffi::c_uint,
        bit_depth: core::ffi::c_int,
        part_state: *mut PartitionSearchState,
    );
    fn av1_prune_partitions_before_search(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
        part_state: *mut PartitionSearchState,
    );
    fn av1_prune_partitions_by_max_min_bsize(
        sb_enc: *mut SuperBlockEnc,
        part_state: *mut PartitionSearchState,
    );
    fn av1_prune_ab_partitions(
        cpi: *mut AV1_COMP,
        x: *const MACROBLOCK,
        pc_tree: *const PC_TREE,
        pb_source_variance: core::ffi::c_int,
        best_rdcost: int64_t,
        rect_part_win_info: *const RD_RECT_PART_WIN_INFO,
        ext_partition_allowed: bool,
        part_state: *mut PartitionSearchState,
        ab_partitions_allowed: *mut core::ffi::c_int,
    );
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
    fn av1_build_obmc_inter_predictors_sb(cm: *const AV1_COMMON, xd: *mut MACROBLOCKD);
}
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
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
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
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub frame: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
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
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const TX_MODES: C2RustUnnamed_3 = 3;
pub const TX_MODE_SELECT: C2RustUnnamed_3 = 2;
pub const TX_MODE_LARGEST: C2RustUnnamed_3 = 1;
pub const ONLY_4X4: C2RustUnnamed_3 = 0;
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const AOM_REFFRAME_ALL: C2RustUnnamed_4 = 127;
pub const AOM_ALT_FLAG: C2RustUnnamed_4 = 64;
pub const AOM_ALT2_FLAG: C2RustUnnamed_4 = 32;
pub const AOM_BWD_FLAG: C2RustUnnamed_4 = 16;
pub const AOM_GOLD_FLAG: C2RustUnnamed_4 = 8;
pub const AOM_LAST3_FLAG: C2RustUnnamed_4 = 4;
pub const AOM_LAST2_FLAG: C2RustUnnamed_4 = 2;
pub const AOM_LAST_FLAG: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const COMP_REFERENCE_TYPES: C2RustUnnamed_5 = 2;
pub const BIDIR_COMP_REFERENCE: C2RustUnnamed_5 = 1;
pub const UNIDIR_COMP_REFERENCE: C2RustUnnamed_5 = 0;
pub type COMP_REFERENCE_TYPE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_6 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_6 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_6 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const CFL_ALLOWED_TYPES: C2RustUnnamed_7 = 2;
pub const CFL_ALLOWED: C2RustUnnamed_7 = 1;
pub const CFL_DISALLOWED: C2RustUnnamed_7 = 0;
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_8 = core::ffi::c_uint;
pub const COLOR_MAP_TYPES: C2RustUnnamed_8 = 1;
pub const PALETTE_MAP: C2RustUnnamed_8 = 0;
pub type COLOR_MAP_TYPE = uint8_t;
pub type C2RustUnnamed_9 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_9 = 25;
pub const INTRA_MODES: C2RustUnnamed_9 = 13;
pub const INTER_MODE_END: C2RustUnnamed_9 = 25;
pub const INTER_MODE_START: C2RustUnnamed_9 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_9 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_9 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_9 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_9 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_9 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_9 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_9 = 13;
pub const DIR_MODE_END: C2RustUnnamed_9 = 9;
pub const DIR_MODE_START: C2RustUnnamed_9 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_9 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_9 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_9 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_9 = 25;
pub const NEW_NEWMV: C2RustUnnamed_9 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_9 = 23;
pub const NEW_NEARMV: C2RustUnnamed_9 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_9 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_9 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_9 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_9 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_9 = 17;
pub const NEWMV: C2RustUnnamed_9 = 16;
pub const GLOBALMV: C2RustUnnamed_9 = 15;
pub const NEARMV: C2RustUnnamed_9 = 14;
pub const NEARESTMV: C2RustUnnamed_9 = 13;
pub const PAETH_PRED: C2RustUnnamed_9 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_9 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_9 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_9 = 9;
pub const D67_PRED: C2RustUnnamed_9 = 8;
pub const D203_PRED: C2RustUnnamed_9 = 7;
pub const D157_PRED: C2RustUnnamed_9 = 6;
pub const D113_PRED: C2RustUnnamed_9 = 5;
pub const D135_PRED: C2RustUnnamed_9 = 4;
pub const D45_PRED: C2RustUnnamed_9 = 3;
pub const H_PRED: C2RustUnnamed_9 = 2;
pub const V_PRED: C2RustUnnamed_9 = 1;
pub const DC_PRED: C2RustUnnamed_9 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_10 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_10 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_10 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_10 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_10 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_10 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_10 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_10 = 9;
pub const UV_D67_PRED: C2RustUnnamed_10 = 8;
pub const UV_D203_PRED: C2RustUnnamed_10 = 7;
pub const UV_D157_PRED: C2RustUnnamed_10 = 6;
pub const UV_D113_PRED: C2RustUnnamed_10 = 5;
pub const UV_D135_PRED: C2RustUnnamed_10 = 4;
pub const UV_D45_PRED: C2RustUnnamed_10 = 3;
pub const UV_H_PRED: C2RustUnnamed_10 = 2;
pub const UV_V_PRED: C2RustUnnamed_10 = 1;
pub const UV_DC_PRED: C2RustUnnamed_10 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_11 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_11 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_11 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_11 = 0;
pub type MOTION_MODE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const INTERINTRA_MODES: C2RustUnnamed_12 = 4;
pub const II_SMOOTH_PRED: C2RustUnnamed_12 = 3;
pub const II_H_PRED: C2RustUnnamed_12 = 2;
pub const II_V_PRED: C2RustUnnamed_12 = 1;
pub const II_DC_PRED: C2RustUnnamed_12 = 0;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_13 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_13 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_13 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_13 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_13 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_13 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_14 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_14 = 7;
pub const BWD_REFS: C2RustUnnamed_14 = 3;
pub const FWD_REFS: C2RustUnnamed_14 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_14 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_14 = 8;
pub const REF_FRAMES: C2RustUnnamed_14 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_14 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_14 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_14 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_14 = 4;
pub const LAST3_FRAME: C2RustUnnamed_14 = 3;
pub const LAST2_FRAME: C2RustUnnamed_14 = 2;
pub const LAST_FRAME: C2RustUnnamed_14 = 1;
pub const INTRA_FRAME: C2RustUnnamed_14 = 0;
pub const NONE_FRAME: C2RustUnnamed_14 = -1;
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const UNIDIR_COMP_REFS: C2RustUnnamed_15 = 4;
pub const TOTAL_UNIDIR_COMP_REFS: C2RustUnnamed_15 = 9;
pub const ALTREF2_ALTREF_FRAMES: C2RustUnnamed_15 = 8;
pub const BWDREF_ALTREF2_FRAMES: C2RustUnnamed_15 = 7;
pub const LAST3_GOLDEN_FRAMES: C2RustUnnamed_15 = 6;
pub const LAST2_GOLDEN_FRAMES: C2RustUnnamed_15 = 5;
pub const LAST2_LAST3_FRAMES: C2RustUnnamed_15 = 4;
pub const BWDREF_ALTREF_FRAMES: C2RustUnnamed_15 = 3;
pub const LAST_GOLDEN_FRAMES: C2RustUnnamed_15 = 2;
pub const LAST_LAST3_FRAMES: C2RustUnnamed_15 = 1;
pub const LAST_LAST2_FRAMES: C2RustUnnamed_15 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub c2rust_unnamed_0: C2RustUnnamed_24,
    pub c2rust_unnamed_1: C2RustUnnamed_22,
    pub c2rust_unnamed_2: C2RustUnnamed_20,
    pub c2rust_unnamed_3: C2RustUnnamed_18,
    pub c2rust_unnamed_4: C2RustUnnamed_16,
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
pub union C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_17,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub c2rust_unnamed: C2RustUnnamed_19,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub c2rust_unnamed: C2RustUnnamed_21,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub c2rust_unnamed: C2RustUnnamed_23,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub c2rust_unnamed: C2RustUnnamed_25,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub c2rust_unnamed: C2RustUnnamed_27,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub above: PARTITION_CONTEXT,
    pub left: PARTITION_CONTEXT,
}
pub type aom_cdf_prob = uint16_t;
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_29 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_29 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_29 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_29 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_29 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_29 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_29 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_29 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_29 = 0;
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
pub type C2RustUnnamed_30 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_30 = 4;
pub const AFFINE: C2RustUnnamed_30 = 3;
pub const ROTZOOM: C2RustUnnamed_30 = 2;
pub const TRANSLATION: C2RustUnnamed_30 = 1;
pub const IDENTITY: C2RustUnnamed_30 = 0;
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
pub type C2RustUnnamed_31 = core::ffi::c_int;
pub const MV_SUBPEL_HIGH_PRECISION: C2RustUnnamed_31 = 1;
pub const MV_SUBPEL_LOW_PRECISION: C2RustUnnamed_31 = 0;
pub const MV_SUBPEL_NONE: C2RustUnnamed_31 = -1;
pub type MvSubpelPrecision = int8_t;
pub type SUBPEL_SEARCH_TYPE = uint8_t;
pub type C2RustUnnamed_32 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_32 = 4;
pub const S_FRAME: C2RustUnnamed_32 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_32 = 2;
pub const INTER_FRAME: C2RustUnnamed_32 = 1;
pub const KEY_FRAME: C2RustUnnamed_32 = 0;
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
pub type C2RustUnnamed_33 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_33 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_33 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_33 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_33 = 0;
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
pub type AV1CdefWorkerData = AV1CdefWorker;
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
pub type AV1LfSync = AV1LfSyncData;
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
pub type MACROBLOCK = macroblock;
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
pub type C2RustUnnamed_34 = core::ffi::c_uint;
pub const USE_LARGESTALL: C2RustUnnamed_34 = 2;
pub const USE_FAST_RD: C2RustUnnamed_34 = 1;
pub const USE_FULL_RD: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_35 = 2;
pub const REALTIME: C2RustUnnamed_35 = 1;
pub const GOOD: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_36 = core::ffi::c_uint;
pub const DRY_RUN_COSTCOEFFS: C2RustUnnamed_36 = 2;
pub const DRY_RUN_NORMAL: C2RustUnnamed_36 = 1;
pub const OUTPUT_ENABLED: C2RustUnnamed_36 = 0;
pub type RUN_TYPE = uint8_t;
pub type C2RustUnnamed_37 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_37 = 2;
pub const LAP_STAGE: C2RustUnnamed_37 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_38 = core::ffi::c_uint;
pub const MODE_EVAL_TYPES: C2RustUnnamed_38 = 3;
pub const WINNER_MODE_EVAL: C2RustUnnamed_38 = 2;
pub const MODE_EVAL: C2RustUnnamed_38 = 1;
pub const DEFAULT_EVAL: C2RustUnnamed_38 = 0;
pub type MODE_EVAL_TYPE = uint8_t;
pub type C2RustUnnamed_39 = core::ffi::c_uint;
pub const TX_TYPE_PRUNE_5: C2RustUnnamed_39 = 5;
pub const TX_TYPE_PRUNE_4: C2RustUnnamed_39 = 4;
pub const TX_TYPE_PRUNE_3: C2RustUnnamed_39 = 3;
pub const TX_TYPE_PRUNE_2: C2RustUnnamed_39 = 2;
pub const TX_TYPE_PRUNE_1: C2RustUnnamed_39 = 1;
pub const TX_TYPE_PRUNE_0: C2RustUnnamed_39 = 0;
pub type C2RustUnnamed_40 = core::ffi::c_uint;
pub const VAR_BASED_PARTITION: C2RustUnnamed_40 = 2;
pub const FIXED_PARTITION: C2RustUnnamed_40 = 1;
pub const SEARCH_PARTITION: C2RustUnnamed_40 = 0;
pub type C2RustUnnamed_41 = core::ffi::c_uint;
pub const AQ_MODE_COUNT: C2RustUnnamed_41 = 4;
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_41 = 3;
pub const COMPLEXITY_AQ: C2RustUnnamed_41 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_41 = 1;
pub const NO_AQ: C2RustUnnamed_41 = 0;
pub type WedgeDirectionType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wedge_code_type {
    pub direction: WedgeDirectionType,
    pub x_offset: core::ffi::c_int,
    pub y_offset: core::ffi::c_int,
}
pub type wedge_masks_type = [uint32_t; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wedge_params_type {
    pub wedge_types: core::ffi::c_int,
    pub codebook: *const wedge_code_type,
    pub signflip: *const uint8_t,
    pub masks: *const wedge_masks_type,
}
pub type C2RustUnnamed_42 = core::ffi::c_uint;
pub const NUM_PART4_TYPES: C2RustUnnamed_42 = 2;
pub const VERT4: C2RustUnnamed_42 = 1;
pub const HORZ4: C2RustUnnamed_42 = 0;
pub type PART4_TYPES = uint8_t;
pub type C2RustUnnamed_43 = core::ffi::c_uint;
pub const NUM_AB_PARTS: C2RustUnnamed_43 = 4;
pub const VERT_B: C2RustUnnamed_43 = 3;
pub const VERT_A: C2RustUnnamed_43 = 2;
pub const HORZ_B: C2RustUnnamed_43 = 1;
pub const HORZ_A: C2RustUnnamed_43 = 0;
pub type AB_PART_TYPE = uint8_t;
pub type C2RustUnnamed_44 = core::ffi::c_uint;
pub const NUM_RECT_PARTS: C2RustUnnamed_44 = 2;
pub const VERT: C2RustUnnamed_44 = 1;
pub const HORZ: C2RustUnnamed_44 = 0;
pub type RECT_PART_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RD_RECT_PART_WIN_INFO {
    pub rect_part_win: [core::ffi::c_int; 2],
}
pub type C2RustUnnamed_45 = core::ffi::c_uint;
pub const SB_WET_PASS: C2RustUnnamed_45 = 2;
pub const SB_DRY_PASS: C2RustUnnamed_45 = 1;
pub const SB_SINGLE_PASS: C2RustUnnamed_45 = 0;
pub type SB_MULTI_PASS_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionBlkParams {
    pub mi_step: core::ffi::c_int,
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub mi_row_edge: core::ffi::c_int,
    pub mi_col_edge: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub min_partition_size_1d: core::ffi::c_int,
    pub bsize_at_least_8x8: core::ffi::c_int,
    pub has_rows: core::ffi::c_int,
    pub has_cols: core::ffi::c_int,
    pub bsize: BLOCK_SIZE,
    pub subsize: BLOCK_SIZE,
    pub split_bsize2: BLOCK_SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionSearchState {
    pub intra_part_info: *mut PartitionSearchInfo,
    pub part_blk_params: PartitionBlkParams,
    pub split_part_rect_win: [RD_RECT_PART_WIN_INFO; 4],
    pub this_rdc: RD_STATS,
    pub sum_rdc: RD_STATS,
    pub tmp_partition_cost: [core::ffi::c_int; 4],
    pub partition_cost: *mut core::ffi::c_int,
    pub none_rd: int64_t,
    pub split_rd: [int64_t; 4],
    pub rect_part_rd: [[int64_t; 2]; 2],
    pub is_split_ctx_is_ready: [core::ffi::c_int; 2],
    pub is_rect_ctx_is_ready: [core::ffi::c_int; 2],
    pub terminate_partition_search: core::ffi::c_int,
    pub partition_none_allowed: core::ffi::c_int,
    pub partition_rect_allowed: [core::ffi::c_int; 2],
    pub do_rectangular_split: core::ffi::c_int,
    pub do_square_split: core::ffi::c_int,
    pub prune_rect_part: [core::ffi::c_int; 2],
    pub ss_x: core::ffi::c_int,
    pub ss_y: core::ffi::c_int,
    pub pl_ctx_idx: core::ffi::c_int,
    pub found_best_partition: bool,
}
pub type active_edge_info = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMP,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
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
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    if !(n != 0 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"get_msb\0"))
                .as_ptr(),
            b"bitops.h\0" as *const u8 as *const core::ffi::c_char,
            43 as core::ffi::c_int,
            b"n != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MAX_MIB_MASK: core::ffi::c_int = MAX_MIB_SIZE - 1 as core::ffi::c_int;
pub const MAX_VARTX_DEPTH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE_64X64: core::ffi::c_int = 64 as core::ffi::c_int >> MI_SIZE_LOG2;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SQR_BLOCK_SIZES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PARTITION_PLOFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NEWMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << GLOBALMV_OFFSET)
    - 1 as core::ffi::c_int;
pub const REFMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 8 as core::ffi::c_int - REFMV_OFFSET) - 1 as core::ffi::c_int;
pub const USABLE_REF_MV_STACK_SIZE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const REF_CAT_LEVEL: core::ffi::c_int = 640 as core::ffi::c_int;
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
static mut max_txsize_rect_lookup: [TX_SIZE; 22] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X32 as core::ffi::c_int as TX_SIZE,
    TX_32X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X64 as core::ffi::c_int as TX_SIZE,
    TX_64X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_4X16 as core::ffi::c_int as TX_SIZE,
    TX_16X4 as core::ffi::c_int as TX_SIZE,
    TX_8X32 as core::ffi::c_int as TX_SIZE,
    TX_32X8 as core::ffi::c_int as TX_SIZE,
    TX_16X64 as core::ffi::c_int as TX_SIZE,
    TX_64X16 as core::ffi::c_int as TX_SIZE,
];
static mut sub_tx_size_map: [TX_SIZE; 19] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X32 as core::ffi::c_int as TX_SIZE,
    TX_32X16 as core::ffi::c_int as TX_SIZE,
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
static mut tx_mode_to_biggest_tx_size: [TX_SIZE; 3] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
];
static mut partition_context_lookup: [C2RustUnnamed_28; 22] = [
    {
        let mut init = C2RustUnnamed_28 {
            above: 31 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 31 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 30 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 30 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 30 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 28 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 28 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 28 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 24 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 24 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 24 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 16 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 16 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 16 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 0 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 0 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 31 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 28 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 30 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 24 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 28 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_28 {
            above: 16 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
];
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
#[inline]
unsafe extern "C-unwind" fn get_prob(
    mut num: core::ffi::c_uint,
    mut den: core::ffi::c_uint,
) -> uint8_t {
    if !(den != 0 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<[u8; 9], [core::ffi::c_char; 9]>(*b"get_prob\0"))
                .as_ptr(),
            b"prob.h\0" as *const u8 as *const core::ffi::c_char,
            101 as core::ffi::c_int,
            b"den != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let p: core::ffi::c_int = (num as uint64_t)
        .wrapping_mul(256 as uint64_t)
        .wrapping_add((den >> 1 as core::ffi::c_int) as uint64_t)
        .wrapping_div(den as uint64_t) as core::ffi::c_int;
    let clipped_prob: core::ffi::c_int = p
        | 255 as core::ffi::c_int - p >> 23 as core::ffi::c_int
        | (p == 0 as core::ffi::c_int) as core::ffi::c_int;
    return clipped_prob as uint8_t;
}
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
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
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
pub const MAX_PALETTE_BLOCK_WIDTH: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_PALETTE_BLOCK_HEIGHT: core::ffi::c_int = 64 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_comp_ref_allowed(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return ((if (block_size_wide[bsize as usize] as core::ffi::c_int)
        < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_inter_mode(mut mode: PREDICTION_MODE) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_inter_compound_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= COMP_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < COMP_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn have_nearmv_in_inter_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int == NEARMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAR_NEARMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
        || mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int)
        as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_masked_compound_type(
    mut type_0: COMPOUND_TYPE,
) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == COMPOUND_WEDGE as core::ffi::c_int
        || type_0 as core::ffi::c_int == COMPOUND_DIFFWTD as core::ffi::c_int)
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
unsafe extern "C-unwind" fn has_uni_comp_refs(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (has_second_ref(mbmi) != 0
        && ((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            >= BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int
            ^ ((*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                >= BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int == 0)
        as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_global_mv_block(
    mbmi: *const MB_MODE_INFO,
    mut type_0: TransformationType,
) -> core::ffi::c_int {
    let mode: PREDICTION_MODE = (*mbmi).mode;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let block_size_allowed: core::ffi::c_int = ((if (block_size_wide[bsize as usize]
        as core::ffi::c_int) < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
    return ((mode as core::ffi::c_int == GLOBALMV as core::ffi::c_int
        || mode as core::ffi::c_int == GLOBAL_GLOBALMV as core::ffi::c_int)
        && type_0 as core::ffi::c_int > TRANSLATION as core::ffi::c_int
        && block_size_allowed != 0) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_rect_tx(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    return (tx_size as core::ffi::c_int >= TX_SIZES as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn block_signals_txsize(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn tx_size_from_tx_mode(
    mut bsize: BLOCK_SIZE,
    mut tx_mode: TX_MODE,
) -> TX_SIZE {
    let largest_tx_size: TX_SIZE = tx_mode_to_biggest_tx_size[tx_mode as usize];
    let max_rect_tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    if bsize as core::ffi::c_int == BLOCK_4X4 as core::ffi::c_int {
        return (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
            < largest_tx_size as core::ffi::c_int
        {
            max_txsize_lookup[bsize as usize] as core::ffi::c_int
        } else {
            largest_tx_size as core::ffi::c_int
        }) as TX_SIZE;
    }
    if txsize_sqr_map[max_rect_tx_size as usize] as core::ffi::c_int
        <= largest_tx_size as core::ffi::c_int
    {
        return max_rect_tx_size
    } else {
        return largest_tx_size
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
unsafe extern "C-unwind" fn av1_get_txb_size_index(
    mut bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
) -> core::ffi::c_int {
    static mut tw_w_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
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
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
    ];
    static mut tw_h_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
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
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
    ];
    static mut stride_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
    ];
    let index: core::ffi::c_int = ((blk_row
        >> tw_h_log2_table[bsize as usize] as core::ffi::c_int)
        << stride_log2_table[bsize as usize] as core::ffi::c_int)
        + (blk_col >> tw_w_log2_table[bsize as usize] as core::ffi::c_int);
    if !(index < 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_get_txb_size_index\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1225 as core::ffi::c_int,
            b"index < INTER_TX_SIZE_BUF_LEN\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return index;
}
#[inline]
unsafe extern "C-unwind" fn bsize_to_max_depth(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    static mut bsize_to_max_depth_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
    ];
    return bsize_to_max_depth_table[bsize as usize] as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn bsize_to_tx_size_cat(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"bsize_to_tx_size_cat\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1350 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    static mut bsize_to_tx_size_depth_table: [uint8_t; 22] = [
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
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
    ];
    let depth: core::ffi::c_int = bsize_to_tx_size_depth_table[bsize as usize]
        as core::ffi::c_int;
    if !(depth
        <= TX_SIZES as core::ffi::c_int
            - (TX_4X4 as core::ffi::c_int + 1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"bsize_to_tx_size_cat\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1355 as core::ffi::c_int,
            b"depth <= MAX_TX_CATS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return depth - 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_interintra_allowed_bsize(bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_mode(
    mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= SINGLE_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < SINGLE_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_ref(
    mut rf: *const MV_REFERENCE_FRAME,
) -> core::ffi::c_int {
    return (*rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            <= INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (is_interintra_allowed_bsize((*mbmi).bsize) != 0
        && is_interintra_allowed_mode((*mbmi).mode) != 0
        && is_interintra_allowed_ref(((*mbmi).ref_frame).as_ptr()) != 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_vartx_max_txsize(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*xd)
        .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
        as usize] != 0
    {
        return TX_4X4 as core::ffi::c_int;
    }
    let max_txsize: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    if plane == 0 as core::ffi::c_int {
        return max_txsize as core::ffi::c_int;
    }
    return av1_get_adjusted_tx_size(max_txsize) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_motion_variation_allowed_bsize(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"is_motion_variation_allowed_bsize\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1461 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ((if (block_size_wide[bsize as usize] as core::ffi::c_int)
        < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    }) >= 8 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_motion_variation_allowed_compound(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (has_second_ref(mbmi) == 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn check_num_overlappable_neighbors(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).overlappable_neighbors as core::ffi::c_int != 0 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn motion_mode_allowed(
    mut gm_params: *const WarpedMotionParams,
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
    mut allow_warped_motion: core::ffi::c_int,
) -> MOTION_MODE {
    if check_num_overlappable_neighbors(mbmi) == 0 {
        return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    }
    if (*xd).cur_frame_force_integer_mv == 0 as core::ffi::c_int {
        let gm_type: TransformationType = (*gm_params
            .offset((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize))
            .wmtype;
        if is_global_mv_block(mbmi, gm_type) != 0 {
            return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
        }
    }
    if is_motion_variation_allowed_bsize((*mbmi).bsize) != 0
        && is_inter_mode((*mbmi).mode) != 0
        && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            != INTRA_FRAME as core::ffi::c_int
        && is_motion_variation_allowed_compound(mbmi) != 0
    {
        if (has_second_ref(mbmi) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"motion_mode_allowed\0"))
                    .as_ptr(),
                b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
                1488 as core::ffi::c_int,
                b"!has_second_ref(mbmi)\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (*mbmi).num_proj_ref as core::ffi::c_int >= 1 as core::ffi::c_int
            && allow_warped_motion != 0 && (*xd).cur_frame_force_integer_mv == 0
            && av1_is_scaled(
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
            ) == 0
        {
            return WARPED_CAUSAL as core::ffi::c_int as MOTION_MODE;
        }
        return OBMC_CAUSAL as core::ffi::c_int as MOTION_MODE;
    }
    return SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
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
unsafe extern "C-unwind" fn get_ref_scale_factors(
    cm: *mut AV1_COMMON,
    ref_frame: MV_REFERENCE_FRAME,
) -> *mut scale_factors {
    let map_idx: core::ffi::c_int = get_ref_frame_map_idx(cm, ref_frame)
        as core::ffi::c_int;
    return if map_idx != INVALID_IDX {
        &raw mut *((*cm).ref_scale_factors).as_mut_ptr().offset(map_idx as isize)
            as *mut scale_factors
    } else {
        0 as *mut scale_factors
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
unsafe extern "C-unwind" fn set_entropy_context(
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut row_offset: core::ffi::c_int = mi_row;
    let mut col_offset: core::ffi::c_int = mi_col;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(i as isize) as *mut macroblockd_plane;
        let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
            .bsize;
        if (*pd).subsampling_y != 0 && mi_row & 0x1 as core::ffi::c_int != 0
            && mi_size_high[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            row_offset = mi_row - 1 as core::ffi::c_int;
        }
        if (*pd).subsampling_x != 0 && mi_col & 0x1 as core::ffi::c_int != 0
            && mi_size_wide[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            col_offset = mi_col - 1 as core::ffi::c_int;
        }
        let mut above_idx: core::ffi::c_int = col_offset;
        let mut left_idx: core::ffi::c_int = row_offset & MAX_MIB_MASK;
        (*pd).above_entropy_context = &raw mut *(*((*xd).above_entropy_context)
            .as_mut_ptr()
            .offset(i as isize))
            .offset((above_idx >> (*pd).subsampling_x) as isize) as *mut ENTROPY_CONTEXT;
        (*pd).left_entropy_context = &raw mut *(*((*xd).left_entropy_context)
            .as_mut_ptr()
            .offset(i as isize))
            .as_mut_ptr()
            .offset((left_idx >> (*pd).subsampling_y) as isize) as *mut ENTROPY_CONTEXT;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_plane_n4(
    xd: *mut MACROBLOCKD,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        (*xd).plane[i as usize].width = (bw * MI_SIZE
            >> (*xd).plane[i as usize].subsampling_x) as uint8_t;
        (*xd).plane[i as usize].height = (bh * MI_SIZE
            >> (*xd).plane[i as usize].subsampling_y) as uint8_t;
        (*xd).plane[i as usize].width = (if (*xd).plane[i as usize].width
            as core::ffi::c_int > 4 as core::ffi::c_int
        {
            (*xd).plane[i as usize].width as core::ffi::c_int
        } else {
            4 as core::ffi::c_int
        }) as uint8_t;
        (*xd).plane[i as usize].height = (if (*xd).plane[i as usize].height
            as core::ffi::c_int > 4 as core::ffi::c_int
        {
            (*xd).plane[i as usize].height as core::ffi::c_int
        } else {
            4 as core::ffi::c_int
        }) as uint8_t;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_mi_row_col(
    mut xd: *mut MACROBLOCKD,
    tile: *const TileInfo,
    mut mi_row: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut mi_rows: core::ffi::c_int,
    mut mi_cols: core::ffi::c_int,
) {
    (*xd).mb_to_top_edge = -(mi_row * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int)
        * 8 as core::ffi::c_int);
    (*xd).mb_to_bottom_edge = (mi_rows - bh - mi_row)
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int;
    (*xd).mb_to_left_edge = -(mi_col * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int)
        * 8 as core::ffi::c_int);
    (*xd).mb_to_right_edge = (mi_cols - bw - mi_col)
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int;
    (*xd).mi_row = mi_row;
    (*xd).mi_col = mi_col;
    (*xd).up_available = mi_row > (*tile).mi_row_start;
    let ss_x: core::ffi::c_int = (*xd)
        .plane[1 as core::ffi::c_int as usize]
        .subsampling_x;
    let ss_y: core::ffi::c_int = (*xd)
        .plane[1 as core::ffi::c_int as usize]
        .subsampling_y;
    (*xd).left_available = mi_col > (*tile).mi_col_start;
    (*xd).chroma_up_available = (*xd).up_available;
    (*xd).chroma_left_available = (*xd).left_available;
    if ss_x != 0
        && bw < mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
    {
        (*xd).chroma_left_available = mi_col - 1 as core::ffi::c_int
            > (*tile).mi_col_start;
    }
    if ss_y != 0
        && bh < mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
    {
        (*xd).chroma_up_available = mi_row - 1 as core::ffi::c_int
            > (*tile).mi_row_start;
    }
    if (*xd).up_available {
        (*xd).above_mbmi = *((*xd).mi).offset(-(*xd).mi_stride as isize);
    } else {
        (*xd).above_mbmi = 0 as *mut MB_MODE_INFO;
    }
    if (*xd).left_available {
        (*xd).left_mbmi = *((*xd).mi).offset(-(1 as core::ffi::c_int) as isize);
    } else {
        (*xd).left_mbmi = 0 as *mut MB_MODE_INFO;
    }
    let chroma_ref: core::ffi::c_int = ((mi_row & 0x1 as core::ffi::c_int != 0
        || bh & 0x1 as core::ffi::c_int == 0 || ss_y == 0)
        && (mi_col & 0x1 as core::ffi::c_int != 0 || bw & 0x1 as core::ffi::c_int == 0
            || ss_x == 0)) as core::ffi::c_int;
    (*xd).is_chroma_ref = chroma_ref != 0;
    if chroma_ref != 0 {
        let mut base_mi: *mut *mut MB_MODE_INFO = &raw mut *((*xd).mi)
            .offset((-(mi_row & ss_y) * (*xd).mi_stride - (mi_col & ss_x)) as isize)
            as *mut *mut MB_MODE_INFO;
        let mut chroma_above_mi: *mut MB_MODE_INFO = if (*xd).chroma_up_available
            as core::ffi::c_int != 0
        {
            *base_mi.offset((-(*xd).mi_stride + ss_x) as isize)
        } else {
            0 as *mut MB_MODE_INFO
        };
        (*xd).chroma_above_mbmi = chroma_above_mi;
        let mut chroma_left_mi: *mut MB_MODE_INFO = if (*xd).chroma_left_available
            as core::ffi::c_int != 0
        {
            *base_mi.offset((ss_y * (*xd).mi_stride - 1 as core::ffi::c_int) as isize)
        } else {
            0 as *mut MB_MODE_INFO
        };
        (*xd).chroma_left_mbmi = chroma_left_mi;
    }
    (*xd).height = bh as uint8_t;
    (*xd).width = bw as uint8_t;
    (*xd).is_last_vertical_rect = 0 as core::ffi::c_int != 0;
    if ((*xd).width as core::ffi::c_int) < (*xd).height as core::ffi::c_int {
        if mi_col + (*xd).width as core::ffi::c_int
            & (*xd).height as core::ffi::c_int - 1 as core::ffi::c_int == 0
        {
            (*xd).is_last_vertical_rect = 1 as core::ffi::c_int != 0;
        }
    }
    (*xd).is_first_horizontal_rect = 0 as core::ffi::c_int != 0;
    if (*xd).width as core::ffi::c_int > (*xd).height as core::ffi::c_int {
        if mi_row & (*xd).width as core::ffi::c_int - 1 as core::ffi::c_int == 0 {
            (*xd).is_first_horizontal_rect = 1 as core::ffi::c_int != 0;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn update_partition_context(
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut subsize: BLOCK_SIZE,
    mut bsize: BLOCK_SIZE,
) {
    let above_ctx: *mut PARTITION_CONTEXT = ((*xd).above_partition_context)
        .offset(mi_col as isize);
    let left_ctx: *mut PARTITION_CONTEXT = ((*xd).left_partition_context)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    memset(
        above_ctx as *mut core::ffi::c_void,
        partition_context_lookup[subsize as usize].above as core::ffi::c_int,
        bw as size_t,
    );
    memset(
        left_ctx as *mut core::ffi::c_void,
        partition_context_lookup[subsize as usize].left as core::ffi::c_int,
        bh as size_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn cdf_element_prob(
    mut cdf: *const aom_cdf_prob,
    mut element: size_t,
) -> aom_cdf_prob {
    if cdf.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"cdf_element_prob\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1468 as core::ffi::c_int,
            b"cdf != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ((if element > 0 as size_t {
        *cdf.offset(element.wrapping_sub(1 as size_t) as isize) as core::ffi::c_int
    } else {
        CDF_PROB_TOP
    }) - *cdf.offset(element as isize) as core::ffi::c_int) as aom_cdf_prob;
}
#[inline]
unsafe extern "C-unwind" fn partition_gather_horz_alike(
    mut out: *mut aom_cdf_prob,
    in_0: *const aom_cdf_prob,
    mut bsize: BLOCK_SIZE,
) {
    *out.offset(0 as core::ffi::c_int as isize) = CDF_PROB_TOP as aom_cdf_prob;
    let fresh3 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh3 = (*fresh3 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh4 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh4 = (*fresh4 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_SPLIT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh5 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh5 = (*fresh5 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh6 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh6 = (*fresh6 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_B as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh7 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh7 = (*fresh7 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    if bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int {
        let fresh8 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
        *fresh8 = (*fresh8 as core::ffi::c_int
            - cdf_element_prob(in_0, PARTITION_HORZ_4 as core::ffi::c_int as size_t)
                as core::ffi::c_int) as aom_cdf_prob;
    }
    *out.offset(0 as core::ffi::c_int as isize) = (CDF_PROB_TOP
        - *out.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        as aom_cdf_prob;
    *out.offset(1 as core::ffi::c_int as isize) = (CDF_PROB_TOP
        - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob;
}
#[inline]
unsafe extern "C-unwind" fn partition_gather_vert_alike(
    mut out: *mut aom_cdf_prob,
    in_0: *const aom_cdf_prob,
    mut bsize: BLOCK_SIZE,
) {
    *out.offset(0 as core::ffi::c_int as isize) = CDF_PROB_TOP as aom_cdf_prob;
    let fresh9 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh9 = (*fresh9 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh10 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh10 = (*fresh10 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_SPLIT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh11 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh11 = (*fresh11 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh12 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh12 = (*fresh12 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh13 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh13 = (*fresh13 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_B as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    if bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int {
        let fresh14 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
        *fresh14 = (*fresh14 as core::ffi::c_int
            - cdf_element_prob(in_0, PARTITION_VERT_4 as core::ffi::c_int as size_t)
                as core::ffi::c_int) as aom_cdf_prob;
    }
    *out.offset(0 as core::ffi::c_int as isize) = (CDF_PROB_TOP
        - *out.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        as aom_cdf_prob;
    *out.offset(1 as core::ffi::c_int as isize) = (CDF_PROB_TOP
        - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob;
}
#[inline]
unsafe extern "C-unwind" fn update_ext_partition_context(
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut subsize: BLOCK_SIZE,
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
) {
    if bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int {
        let hbs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
            / 2 as core::ffi::c_int;
        let mut bsize2: BLOCK_SIZE = get_partition_subsize(
            bsize,
            PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
        );
        let mut current_block_11: u64;
        match partition as core::ffi::c_int {
            3 => {
                if bsize as core::ffi::c_int != BLOCK_8X8 as core::ffi::c_int {
                    current_block_11 = 0;
                } else {
                    current_block_11 = 1;
                }
            }
            0 | 1 | 2 | 8 | 9 => {
                current_block_11 = 1;
            }
            4 => {
                update_partition_context(xd, mi_row, mi_col, bsize2, subsize);
                update_partition_context(xd, mi_row + hbs, mi_col, subsize, subsize);
                current_block_11 = 0;
            }
            5 => {
                update_partition_context(xd, mi_row, mi_col, subsize, subsize);
                update_partition_context(xd, mi_row + hbs, mi_col, bsize2, subsize);
                current_block_11 = 0;
            }
            6 => {
                update_partition_context(xd, mi_row, mi_col, bsize2, subsize);
                update_partition_context(xd, mi_row, mi_col + hbs, subsize, subsize);
                current_block_11 = 0;
            }
            7 => {
                update_partition_context(xd, mi_row, mi_col, subsize, subsize);
                update_partition_context(xd, mi_row, mi_col + hbs, bsize2, subsize);
                current_block_11 = 0;
            }
            _ => {
                if !(0 as core::ffi::c_int != 0
                    && !(b"Invalid partition type\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 29],
                            [core::ffi::c_char; 29],
                        >(*b"update_ext_partition_context\0"))
                            .as_ptr(),
                        b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                        1536 as core::ffi::c_int,
                        b"0 && \"Invalid partition type\"\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                current_block_11 = 0;
            }
        }
        match current_block_11 {
            1 => {
                update_partition_context(xd, mi_row, mi_col, subsize, bsize);
            }
            _ => {}
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn partition_plane_context(
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut above_ctx: *const PARTITION_CONTEXT = ((*xd).above_partition_context)
        .offset(mi_col as isize);
    let mut left_ctx: *const PARTITION_CONTEXT = ((*xd).left_partition_context)
        .as_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    let bsl: core::ffi::c_int = mi_size_wide_log2[bsize as usize] as core::ffi::c_int
        - mi_size_wide_log2[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int;
    let mut above: core::ffi::c_int = *above_ctx as core::ffi::c_int >> bsl
        & 1 as core::ffi::c_int;
    let mut left: core::ffi::c_int = *left_ctx as core::ffi::c_int >> bsl
        & 1 as core::ffi::c_int;
    if !(mi_size_wide_log2[bsize as usize] as core::ffi::c_int
        == mi_size_high_log2[bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"partition_plane_context\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1550 as core::ffi::c_int,
            b"mi_size_wide_log2[bsize] == mi_size_high_log2[bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(bsl >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"partition_plane_context\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1551 as core::ffi::c_int,
            b"bsl >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return left * 2 as core::ffi::c_int + above + bsl * PARTITION_PLOFFSET;
}
#[inline]
unsafe extern "C-unwind" fn partition_cdf_length(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    if bsize as core::ffi::c_int <= BLOCK_8X8 as core::ffi::c_int {
        return PARTITION_TYPES as core::ffi::c_int
    } else if bsize as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int {
        return EXT_PARTITION_TYPES as core::ffi::c_int - 2 as core::ffi::c_int
    } else {
        return EXT_PARTITION_TYPES as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn max_block_wide(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"max_block_wide\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1569 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut max_blocks_wide: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    if (*xd).mb_to_right_edge < 0 as core::ffi::c_int {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(plane as isize) as *const macroblockd_plane;
        max_blocks_wide
            += (*xd).mb_to_right_edge >> 3 as core::ffi::c_int + (*pd).subsampling_x;
    }
    return max_blocks_wide >> MI_SIZE_LOG2;
}
#[inline]
unsafe extern "C-unwind" fn max_block_high(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut max_blocks_high: core::ffi::c_int = block_size_high[bsize as usize]
        as core::ffi::c_int;
    if (*xd).mb_to_bottom_edge < 0 as core::ffi::c_int {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(plane as isize) as *const macroblockd_plane;
        max_blocks_high
            += (*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int + (*pd).subsampling_y;
    }
    return max_blocks_high >> MI_SIZE_LOG2;
}
#[inline]
unsafe extern "C-unwind" fn set_txfm_ctx(
    mut txfm_ctx: *mut TXFM_CONTEXT,
    mut txs: uint8_t,
    mut len: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < len {
        *txfm_ctx.offset(i as isize) = txs as TXFM_CONTEXT;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_txfm_ctxs(
    mut tx_size: TX_SIZE,
    mut n4_w: core::ffi::c_int,
    mut n4_h: core::ffi::c_int,
    mut skip: core::ffi::c_int,
    mut xd: *const MACROBLOCKD,
) {
    let mut bw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let mut bh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    if skip != 0 {
        bw = (n4_w * MI_SIZE) as uint8_t;
        bh = (n4_h * MI_SIZE) as uint8_t;
    }
    set_txfm_ctx((*xd).above_txfm_context, bw, n4_w);
    set_txfm_ctx((*xd).left_txfm_context, bh, n4_h);
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
    let fresh15 = &raw mut *((*mi_params).mi_grid_base).offset(mi_grid_idx as isize);
    *fresh15 = &raw mut *((*mi_params).mi_alloc).offset(mi_alloc_idx as isize)
        as *mut MB_MODE_INFO;
    (*xd).mi = ((*mi_params).mi_grid_base).offset(mi_grid_idx as isize);
    (*xd).tx_type_map = ((*mi_params).tx_type_map).offset(mi_grid_idx as isize)
        as *mut uint8_t;
    (*xd).tx_type_map_stride = (*mi_params).mi_stride;
}
#[inline]
unsafe extern "C-unwind" fn txfm_partition_update(
    mut above_ctx: *mut TXFM_CONTEXT,
    mut left_ctx: *mut TXFM_CONTEXT,
    mut tx_size: TX_SIZE,
    mut txb_size: TX_SIZE,
) {
    let mut bsize: BLOCK_SIZE = txsize_to_bsize[txb_size as usize];
    let mut bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let mut bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mut txw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let mut txh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < bh {
        *left_ctx.offset(i as isize) = txh as TXFM_CONTEXT;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < bw {
        *above_ctx.offset(i as isize) = txw as TXFM_CONTEXT;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_sqr_tx_size(mut tx_dim: core::ffi::c_int) -> TX_SIZE {
    match tx_dim {
        128 | 64 => return TX_64X64 as core::ffi::c_int as TX_SIZE,
        32 => return TX_32X32 as core::ffi::c_int as TX_SIZE,
        16 => return TX_16X16 as core::ffi::c_int as TX_SIZE,
        8 => return TX_8X8 as core::ffi::c_int as TX_SIZE,
        _ => return TX_4X4 as core::ffi::c_int as TX_SIZE,
    };
}
#[inline]
unsafe extern "C-unwind" fn txfm_partition_context(
    above_ctx: *const TXFM_CONTEXT,
    left_ctx: *const TXFM_CONTEXT,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    let txw: uint8_t = tx_size_wide[tx_size as usize] as uint8_t;
    let txh: uint8_t = tx_size_high[tx_size as usize] as uint8_t;
    let above: core::ffi::c_int = ((*above_ctx as core::ffi::c_int)
        < txw as core::ffi::c_int) as core::ffi::c_int;
    let left: core::ffi::c_int = ((*left_ctx as core::ffi::c_int)
        < txh as core::ffi::c_int) as core::ffi::c_int;
    let mut category: core::ffi::c_int = (TX_SIZES as core::ffi::c_int
        - TX_8X8 as core::ffi::c_int) * 6 as core::ffi::c_int - 3 as core::ffi::c_int;
    if tx_size as core::ffi::c_int <= TX_4X4 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut max_tx_size: TX_SIZE = get_sqr_tx_size(
        if block_size_wide[bsize as usize] as core::ffi::c_int
            > block_size_high[bsize as usize] as core::ffi::c_int
        {
            block_size_wide[bsize as usize] as core::ffi::c_int
        } else {
            block_size_high[bsize as usize] as core::ffi::c_int
        },
    );
    if max_tx_size as core::ffi::c_int >= TX_8X8 as core::ffi::c_int {
        category = (txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            != max_tx_size as core::ffi::c_int
            && max_tx_size as core::ffi::c_int > TX_8X8 as core::ffi::c_int)
            as core::ffi::c_int
            + (TX_SIZES as core::ffi::c_int - 1 as core::ffi::c_int
                - max_tx_size as core::ffi::c_int) * 2 as core::ffi::c_int;
    }
    if !(category
        != (TX_SIZES as core::ffi::c_int - TX_8X8 as core::ffi::c_int)
            * 6 as core::ffi::c_int - 3 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"txfm_partition_context\0"))
                .as_ptr(),
            b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
            1769 as core::ffi::c_int,
            b"category != TXFM_PARTITION_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return category * 3 as core::ffi::c_int + above + left;
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
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
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
static mut compound_mode_ctx_map: [[uint16_t; 5]; 3] = [
    [
        0 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
        1 as core::ffi::c_int as uint16_t,
    ],
    [
        1 as core::ffi::c_int as uint16_t,
        2 as core::ffi::c_int as uint16_t,
        3 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
    ],
    [
        4 as core::ffi::c_int as uint16_t,
        4 as core::ffi::c_int as uint16_t,
        5 as core::ffi::c_int as uint16_t,
        6 as core::ffi::c_int as uint16_t,
        7 as core::ffi::c_int as uint16_t,
    ],
];
#[inline]
unsafe extern "C-unwind" fn av1_mode_context_analyzer(
    mode_context: *const int16_t,
    rf: *const MV_REFERENCE_FRAME,
) -> int16_t {
    let ref_frame: int8_t = av1_ref_frame_type(rf) as int8_t;
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        <= INTRA_FRAME as core::ffi::c_int
    {
        return *mode_context.offset(ref_frame as isize);
    }
    let newmv_ctx: int16_t = (*mode_context.offset(ref_frame as isize)
        as core::ffi::c_int & NEWMV_CTX_MASK) as int16_t;
    let refmv_ctx: int16_t = (*mode_context.offset(ref_frame as isize)
        as core::ffi::c_int >> REFMV_OFFSET & REFMV_CTX_MASK) as int16_t;
    let comp_ctx: int16_t = compound_mode_ctx_map[(refmv_ctx as core::ffi::c_int
        >> 1 as core::ffi::c_int)
        as usize][(if (newmv_ctx as core::ffi::c_int)
        < 5 as core::ffi::c_int - 1 as core::ffi::c_int
    {
        newmv_ctx as core::ffi::c_int
    } else {
        5 as core::ffi::c_int - 1 as core::ffi::c_int
    }) as usize] as int16_t;
    return comp_ctx;
}
#[inline]
unsafe extern "C-unwind" fn av1_drl_ctx(
    mut ref_mv_weight: *const uint16_t,
    mut ref_idx: core::ffi::c_int,
) -> uint8_t {
    if *ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int >= REF_CAT_LEVEL
        && *ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int >= REF_CAT_LEVEL
    {
        return 0 as uint8_t;
    }
    if *ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int >= REF_CAT_LEVEL
        && (*ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < REF_CAT_LEVEL
    {
        return 1 as uint8_t;
    }
    if (*ref_mv_weight.offset(ref_idx as isize) as core::ffi::c_int) < REF_CAT_LEVEL
        && (*ref_mv_weight.offset((ref_idx + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int) < REF_CAT_LEVEL
    {
        return 2 as uint8_t;
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_collect_neighbors_ref_counts(xd: *mut MACROBLOCKD) {
    memset(
        &raw mut (*xd).neighbors_ref_counts as *mut [uint8_t; 8] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
    let ref_counts: *mut uint8_t = ((*xd).neighbors_ref_counts).as_mut_ptr();
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_in_image: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let left_in_image: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    if above_in_image != 0 && is_inter_block(above_mbmi) != 0 {
        let fresh16 = &raw mut *ref_counts
            .offset((*above_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh16 = (*fresh16).wrapping_add(1);
        if has_second_ref(above_mbmi) != 0 {
            let fresh17 = &raw mut *ref_counts
                .offset(
                    (*above_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize,
                );
            *fresh17 = (*fresh17).wrapping_add(1);
        }
    }
    if left_in_image != 0 && is_inter_block(left_mbmi) != 0 {
        let fresh18 = &raw mut *ref_counts
            .offset((*left_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh18 = (*fresh18).wrapping_add(1);
        if has_second_ref(left_mbmi) != 0 {
            let fresh19 = &raw mut *ref_counts
                .offset((*left_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize);
            *fresh19 = (*fresh19).wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn is_rect_tx_allowed_bsize(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    static mut LUT: [core::ffi::c_char; 22] = [
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        0 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
        1 as core::ffi::c_int as core::ffi::c_char,
    ];
    return LUT[bsize as usize] as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_rect_tx_allowed(
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (is_rect_tx_allowed_bsize((*mbmi).bsize) != 0
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn tx_size_to_depth(
    mut tx_size: TX_SIZE,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut ctx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    let mut depth: core::ffi::c_int = 0 as core::ffi::c_int;
    while tx_size as core::ffi::c_int != ctx_size as core::ffi::c_int {
        depth += 1;
        ctx_size = sub_tx_size_map[ctx_size as usize];
        if !(depth <= 2 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"tx_size_to_depth\0"))
                    .as_ptr(),
                b"block.h\0" as *const u8 as *const core::ffi::c_char,
                1527 as core::ffi::c_int,
                b"depth <= MAX_TX_DEPTH\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    return depth;
}
pub const CR_SEGMENT_ID_BOOST1: core::ffi::c_int = 1 as core::ffi::c_int;
pub const CR_SEGMENT_ID_BOOST2: core::ffi::c_int = 2 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn cyclic_refresh_segment_id_boosted(
    mut segment_id: core::ffi::c_int,
) -> core::ffi::c_int {
    return (segment_id == CR_SEGMENT_ID_BOOST1 || segment_id == CR_SEGMENT_ID_BOOST2)
        as core::ffi::c_int;
}
pub const AV1_PROB_COST_SHIFT: core::ffi::c_int = 9 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_cost_symbol(mut p15: aom_cdf_prob) -> core::ffi::c_int {
    p15 = clamp(
        p15 as core::ffi::c_int,
        1 as core::ffi::c_int,
        CDF_PROB_TOP - 1 as core::ffi::c_int,
    ) as aom_cdf_prob;
    if !((0 as core::ffi::c_int) < p15 as core::ffi::c_int
        && (p15 as core::ffi::c_int) < (1 as core::ffi::c_int) << 15 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_cost_symbol\0"))
                .as_ptr(),
            b"cost.h\0" as *const u8 as *const core::ffi::c_char,
            37 as core::ffi::c_int,
            b"0 < p15 && p15 < CDF_PROB_TOP\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let shift: core::ffi::c_int = CDF_PROB_BITS - 1 as core::ffi::c_int
        - get_msb(p15 as core::ffi::c_uint) as core::ffi::c_int;
    let prob: core::ffi::c_int = get_prob(
        ((p15 as core::ffi::c_int) << shift) as core::ffi::c_uint,
        CDF_PROB_TOP as core::ffi::c_uint,
    ) as core::ffi::c_int;
    if !(prob >= 128 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_cost_symbol\0"))
                .as_ptr(),
            b"cost.h\0" as *const u8 as *const core::ffi::c_char,
            40 as core::ffi::c_int,
            b"prob >= 128\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return av1_prob_cost[(prob - 128 as core::ffi::c_int) as usize] as core::ffi::c_int
        + shift * ((1 as core::ffi::c_int) << AV1_PROB_COST_SHIFT);
}
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_calculate_rd_cost(
    mut mult: core::ffi::c_int,
    mut rate: core::ffi::c_int,
    mut dist: int64_t,
) -> int64_t {
    if !(mult >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_calculate_rd_cost\0"))
                .as_ptr(),
            b"rd.h\0" as *const u8 as *const core::ffi::c_char,
            220 as core::ffi::c_int,
            b"mult >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if rate >= 0 as core::ffi::c_int {
        return (rate as int64_t * mult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    }
    return dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
        - (-rate as int64_t * mult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn av1_rd_cost_update(
    mut mult: core::ffi::c_int,
    mut rd_cost: *mut RD_STATS,
) {
    if (*rd_cost).rate < INT_MAX && (*rd_cost).dist < INT64_MAX as int64_t
        && (*rd_cost).rdcost < INT64_MAX as int64_t
    {
        (*rd_cost).rdcost = av1_calculate_rd_cost(
            mult,
            (*rd_cost).rate,
            (*rd_cost).dist,
        );
    } else {
        av1_invalid_rd_stats(rd_cost);
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_rd_stats_subtraction(
    mut mult: core::ffi::c_int,
    left: *const RD_STATS,
    right: *const RD_STATS,
    mut result: *mut RD_STATS,
) {
    if (*left).rate == INT_MAX || (*right).rate == INT_MAX
        || (*left).dist == INT64_MAX as int64_t || (*right).dist == INT64_MAX as int64_t
        || (*left).rdcost == INT64_MAX as int64_t
        || (*right).rdcost == INT64_MAX as int64_t
    {
        av1_invalid_rd_stats(result);
    } else {
        (*result).rate = (*left).rate - (*right).rate;
        (*result).dist = (*left).dist - (*right).dist;
        (*result).rdcost = av1_calculate_rd_cost(mult, (*result).rate, (*result).dist);
    };
}
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
unsafe extern "C-unwind" fn get_rd_opt_coeff_thresh(
    coeff_opt_threshold: *const [uint32_t; 2],
    mut txfm_params: *mut TxfmSearchParams,
    mut enable_winner_mode_for_coeff_opt: core::ffi::c_int,
    mut is_winner_mode: core::ffi::c_int,
) {
    if enable_winner_mode_for_coeff_opt == 0 {
        (*txfm_params).coeff_opt_thresholds[0 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                DEFAULT_EVAL as core::ffi::c_int as isize,
            ))[0 as core::ffi::c_int as usize] as core::ffi::c_uint;
        (*txfm_params).coeff_opt_thresholds[1 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                DEFAULT_EVAL as core::ffi::c_int as isize,
            ))[1 as core::ffi::c_int as usize] as core::ffi::c_uint;
        return;
    }
    if is_winner_mode != 0 {
        (*txfm_params).coeff_opt_thresholds[0 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                WINNER_MODE_EVAL as core::ffi::c_int as isize,
            ))[0 as core::ffi::c_int as usize] as core::ffi::c_uint;
        (*txfm_params).coeff_opt_thresholds[1 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                WINNER_MODE_EVAL as core::ffi::c_int as isize,
            ))[1 as core::ffi::c_int as usize] as core::ffi::c_uint;
    } else {
        (*txfm_params).coeff_opt_thresholds[0 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                MODE_EVAL as core::ffi::c_int as isize,
            ))[0 as core::ffi::c_int as usize] as core::ffi::c_uint;
        (*txfm_params).coeff_opt_thresholds[1 as core::ffi::c_int as usize] = (*coeff_opt_threshold
            .offset(
                MODE_EVAL as core::ffi::c_int as isize,
            ))[1 as core::ffi::c_int as usize] as core::ffi::c_uint;
    };
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
#[inline]
unsafe extern "C-unwind" fn av1_set_mv_col_limits(
    mi_params: *const CommonModeInfoParams,
    mut mv_limits: *mut FullMvLimits,
    mut mi_col: core::ffi::c_int,
    mut mi_width: core::ffi::c_int,
    mut border: core::ffi::c_int,
) {
    let min1: core::ffi::c_int = -(mi_col * MI_SIZE + border
        - 2 as core::ffi::c_int * AOM_INTERP_EXTEND);
    let min2: core::ffi::c_int = -((mi_col + mi_width) * MI_SIZE
        + 2 as core::ffi::c_int * AOM_INTERP_EXTEND);
    (*mv_limits).col_min = if min1 > min2 { min1 } else { min2 };
    let max1: core::ffi::c_int = ((*mi_params).mi_cols - mi_col - mi_width) * MI_SIZE
        + border - 2 as core::ffi::c_int * AOM_INTERP_EXTEND;
    let max2: core::ffi::c_int = ((*mi_params).mi_cols - mi_col) * MI_SIZE
        + 2 as core::ffi::c_int * AOM_INTERP_EXTEND;
    (*mv_limits).col_max = if max1 < max2 { max1 } else { max2 };
}
#[inline]
unsafe extern "C-unwind" fn av1_set_mv_limits(
    mi_params: *const CommonModeInfoParams,
    mut mv_limits: *mut FullMvLimits,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut mi_height: core::ffi::c_int,
    mut mi_width: core::ffi::c_int,
    mut border: core::ffi::c_int,
) {
    av1_set_mv_row_limits(mi_params, mv_limits, mi_row, mi_height, border);
    av1_set_mv_col_limits(mi_params, mv_limits, mi_col, mi_width, border);
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame_yv12_buf(
    cm: *const AV1_COMMON,
    mut ref_frame: MV_REFERENCE_FRAME,
) -> *const YV12_BUFFER_CONFIG {
    let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
    return if !buf.is_null() { &(*buf).buf } else { 0 as *const YV12_BUFFER_CONFIG };
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
unsafe extern "C-unwind" fn set_ref_ptrs(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut ref0: MV_REFERENCE_FRAME,
    mut ref1: MV_REFERENCE_FRAME,
) {
    (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = get_ref_scale_factors_const(
        cm,
        (if ref0 as core::ffi::c_int >= LAST_FRAME as core::ffi::c_int {
            ref0 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        }) as MV_REFERENCE_FRAME,
    );
    (*xd).block_ref_scale_factors[1 as core::ffi::c_int as usize] = get_ref_scale_factors_const(
        cm,
        (if ref1 as core::ffi::c_int >= LAST_FRAME as core::ffi::c_int {
            ref1 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        }) as MV_REFERENCE_FRAME,
    );
}
#[inline]
unsafe extern "C-unwind" fn get_mi_ext_idx(
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    mi_alloc_bsize: BLOCK_SIZE,
    mbmi_ext_stride: core::ffi::c_int,
) -> core::ffi::c_int {
    let mi_ext_size_1d: core::ffi::c_int = mi_size_wide[mi_alloc_bsize as usize]
        as core::ffi::c_int;
    let mi_ext_row: core::ffi::c_int = mi_row / mi_ext_size_1d;
    let mi_ext_col: core::ffi::c_int = mi_col / mi_ext_size_1d;
    return mi_ext_row * mbmi_ext_stride + mi_ext_col;
}
#[inline]
unsafe extern "C-unwind" fn set_pixels_to_frame_edge(
    mut x: *mut MACROBLOCK,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_cols: core::ffi::c_int,
    mut mi_rows: core::ffi::c_int,
    mut frame_width: core::ffi::c_int,
    mut frame_height: core::ffi::c_int,
    mut do_border_pad: bool,
) {
    let mut boundary_frame_width: core::ffi::c_int = if do_border_pad as core::ffi::c_int
        != 0
    {
        frame_width
    } else {
        mi_cols << MI_SIZE_LOG2
    };
    let mut boundary_frame_height: core::ffi::c_int = if do_border_pad
        as core::ffi::c_int != 0
    {
        frame_height
    } else {
        mi_rows << MI_SIZE_LOG2
    };
    (*x).pix_to_bottom_edge = boundary_frame_height - (mi_row + bh << MI_SIZE_LOG2);
    (*x).pix_to_right_edge = boundary_frame_width - (mi_col + bw << MI_SIZE_LOG2);
}
#[inline]
unsafe extern "C-unwind" fn set_mode_info_offsets(
    mi_params: *const CommonModeInfoParams,
    mbmi_ext_info: *const MBMIExtFrameBufferInfo,
    x: *mut MACROBLOCK,
    xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    set_mi_offsets(mi_params, xd, mi_row, mi_col);
    let ext_idx: core::ffi::c_int = get_mi_ext_idx(
        mi_row,
        mi_col,
        (*mi_params).mi_alloc_bsize,
        (*mbmi_ext_info).stride,
    ) as core::ffi::c_int;
    (*x).mbmi_ext_frame = ((*mbmi_ext_info).frame_base).offset(ext_idx as isize);
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
#[inline]
unsafe extern "C-unwind" fn warped_motion_update_num_proj_ref(
    cpi: *const AV1_COMP,
    mi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*cpi).oxcf.motion_mode_cfg.allow_warped_motion as core::ffi::c_int != 0
        && is_inter_block(mi) != 0 && (*mi).skip_mode() == 0
        && is_motion_variation_allowed_bsize((*mi).bsize) != 0
        && has_second_ref(mi) == 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interinter_compound_used(
    mut type_0: COMPOUND_TYPE,
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    let comp_allowed: core::ffi::c_int = is_comp_ref_allowed(sb_type)
        as core::ffi::c_int;
    match type_0 as core::ffi::c_int {
        0 | 1 | 3 => return comp_allowed,
        2 => {
            return (comp_allowed != 0
                && av1_wedge_params_lookup[sb_type as usize].wedge_types
                    > 0 as core::ffi::c_int) as core::ffi::c_int;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 28],
                        [core::ffi::c_char; 28],
                    >(*b"is_interinter_compound_used\0"))
                        .as_ptr(),
                    b"reconinter.h\0" as *const u8 as *const core::ffi::c_char,
                    313 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            return 0 as core::ffi::c_int;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn is_any_masked_compound_used(
    mut sb_type: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut comp_type: COMPOUND_TYPE = 0;
    let mut i: core::ffi::c_int = 0;
    if is_comp_ref_allowed(sb_type) == 0 {
        return 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < COMPOUND_TYPES as core::ffi::c_int {
        comp_type = i as COMPOUND_TYPE;
        if is_masked_compound_type(comp_type) != 0
            && is_interinter_compound_used(comp_type, sb_type) != 0
        {
            return 1 as core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_wedge_used(mut sb_type: BLOCK_SIZE) -> core::ffi::c_int {
    return (av1_wedge_params_lookup[sb_type as usize].wedge_types
        > 0 as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn store_cfl_required(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
) -> CFL_ALLOWED_TYPE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    if (*(*cm).seq_params).monochrome != 0 {
        return CFL_DISALLOWED as core::ffi::c_int as CFL_ALLOWED_TYPE;
    }
    if !(*xd).is_chroma_ref {
        return CFL_ALLOWED as core::ffi::c_int as CFL_ALLOWED_TYPE;
    }
    return (is_inter_block(mbmi) == 0
        && (*mbmi).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int)
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
unsafe extern "C-unwind" fn get_comp_index_context(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bck_buf: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
    );
    let fwd_buf: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
    );
    let mut bck_frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut fwd_frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut cur_frame_index: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    if !bck_buf.is_null() {
        bck_frame_index = (*bck_buf).order_hint as core::ffi::c_int;
    }
    if !fwd_buf.is_null() {
        fwd_frame_index = (*fwd_buf).order_hint as core::ffi::c_int;
    }
    let mut fwd: core::ffi::c_int = abs(
        get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            fwd_frame_index,
            cur_frame_index,
        ),
    );
    let mut bck: core::ffi::c_int = abs(
        get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            cur_frame_index,
            bck_frame_index,
        ),
    );
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut above_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut left_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let offset: core::ffi::c_int = (fwd == bck) as core::ffi::c_int;
    if !above_mi.is_null() {
        if has_second_ref(above_mi) != 0 {
            above_ctx = (*above_mi).compound_idx() as core::ffi::c_int;
        } else if (*above_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            above_ctx = 1 as core::ffi::c_int;
        }
    }
    if !left_mi.is_null() {
        if has_second_ref(left_mi) != 0 {
            left_ctx = (*left_mi).compound_idx() as core::ffi::c_int;
        } else if (*left_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            left_ctx = 1 as core::ffi::c_int;
        }
    }
    return above_ctx + left_ctx + 3 as core::ffi::c_int * offset;
}
#[inline]
unsafe extern "C-unwind" fn get_comp_group_idx_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut above_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut left_ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mi.is_null() {
        if has_second_ref(above_mi) != 0 {
            above_ctx = (*above_mi).comp_group_idx() as core::ffi::c_int;
        } else if (*above_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            above_ctx = 3 as core::ffi::c_int;
        }
    }
    if !left_mi.is_null() {
        if has_second_ref(left_mi) != 0 {
            left_ctx = (*left_mi).comp_group_idx() as core::ffi::c_int;
        } else if (*left_mi).ref_frame[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        {
            left_ctx = 3 as core::ffi::c_int;
        }
    }
    return if (5 as core::ffi::c_int) < above_ctx + left_ctx {
        5 as core::ffi::c_int
    } else {
        above_ctx + left_ctx
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_skip_mode_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_skip_mode: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).skip_mode() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_skip_mode: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).skip_mode() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return above_skip_mode + left_skip_mode;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_skip_txfm_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_skip_txfm: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).skip_txfm as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_skip_txfm: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).skip_txfm as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return above_skip_txfm + left_skip_txfm;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_reference_mode_cdf(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .comp_inter_cdf[av1_get_reference_mode_context(xd) as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_comp_reference_type_cdf(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_comp_reference_type_context(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx).comp_ref_type_cdf[pred_context as usize]).as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_uni_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_uni_comp_ref_p2(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .uni_comp_ref_cdf[pred_context as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_ref_p2(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_ref_cdf[pred_context as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_bwdref_p(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_bwdref_p(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_bwdref_cdf[pred_context as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_comp_bwdref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    let pred_context: core::ffi::c_int = av1_get_pred_context_comp_bwdref_p1(xd)
        as core::ffi::c_int;
    return ((*(*xd).tile_ctx)
        .comp_bwdref_cdf[pred_context as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p1(xd)
        as usize][0 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p2(xd)
        as usize][1 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p3(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p3(xd)
        as usize][2 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p4(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p4(xd)
        as usize][3 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p5(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p5(xd)
        as usize][4 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_cdf_single_ref_p6(
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*(*xd).tile_ctx)
        .single_ref_cdf[av1_get_pred_context_single_ref_p6(xd)
        as usize][5 as core::ffi::c_int as usize])
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn get_tx_size_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let max_tx_size: TX_SIZE = max_txsize_rect_lookup[(*mbmi).bsize as usize];
    let max_tx_wide: core::ffi::c_int = tx_size_wide[max_tx_size as usize];
    let max_tx_high: core::ffi::c_int = tx_size_high[max_tx_size as usize];
    let has_above: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let has_left: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    let mut above: core::ffi::c_int = (*((*xd).above_txfm_context)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int >= max_tx_wide)
        as core::ffi::c_int;
    let mut left: core::ffi::c_int = (*((*xd).left_txfm_context)
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int >= max_tx_high)
        as core::ffi::c_int;
    if has_above != 0 {
        if is_inter_block(above_mbmi) != 0 {
            above = (block_size_wide[(*above_mbmi).bsize as usize] as core::ffi::c_int
                >= max_tx_wide) as core::ffi::c_int;
        }
    }
    if has_left != 0 {
        if is_inter_block(left_mbmi) != 0 {
            left = (block_size_high[(*left_mbmi).bsize as usize] as core::ffi::c_int
                >= max_tx_high) as core::ffi::c_int;
        }
    }
    if has_above != 0 && has_left != 0 {
        return above + left
    } else if has_above != 0 {
        return above
    } else if has_left != 0 {
        return left
    } else {
        return 0 as core::ffi::c_int
    };
}
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
unsafe extern "C-unwind" fn set_tx_size_search_method(
    mut cm: *const AV1_COMMON,
    mut winner_mode_params: *const WinnerModeParams,
    mut txfm_params: *mut TxfmSearchParams,
    mut enable_winner_mode_for_tx_size_srch: core::ffi::c_int,
    mut is_winner_mode: core::ffi::c_int,
) {
    (*txfm_params).tx_size_search_method = (*winner_mode_params)
        .tx_size_search_methods[DEFAULT_EVAL as core::ffi::c_int as usize];
    if enable_winner_mode_for_tx_size_srch != 0 {
        if is_winner_mode != 0 {
            (*txfm_params).tx_size_search_method = (*winner_mode_params)
                .tx_size_search_methods[WINNER_MODE_EVAL as core::ffi::c_int as usize];
        } else {
            (*txfm_params).tx_size_search_method = (*winner_mode_params)
                .tx_size_search_methods[MODE_EVAL as core::ffi::c_int as usize];
        }
    }
    (*txfm_params).tx_mode_search_type = select_tx_mode(
        cm,
        (*txfm_params).tx_size_search_method,
    );
}
#[inline]
unsafe extern "C-unwind" fn set_tx_type_prune(
    mut sf: *const SPEED_FEATURES,
    mut txfm_params: *mut TxfmSearchParams,
    mut winner_mode_tx_type_pruning: core::ffi::c_int,
    mut is_winner_mode: core::ffi::c_int,
) {
    (*txfm_params).prune_2d_txfm_mode = (*sf).tx_sf.tx_type_search.prune_2d_txfm_mode
        as core::ffi::c_int;
    if winner_mode_tx_type_pruning == 0 {
        return;
    }
    let prune_mode: [[core::ffi::c_int; 2]; 4] = [
        [TX_TYPE_PRUNE_3 as core::ffi::c_int, TX_TYPE_PRUNE_0 as core::ffi::c_int],
        [TX_TYPE_PRUNE_4 as core::ffi::c_int, TX_TYPE_PRUNE_0 as core::ffi::c_int],
        [TX_TYPE_PRUNE_5 as core::ffi::c_int, TX_TYPE_PRUNE_2 as core::ffi::c_int],
        [TX_TYPE_PRUNE_5 as core::ffi::c_int, TX_TYPE_PRUNE_3 as core::ffi::c_int],
    ];
    (*txfm_params).prune_2d_txfm_mode = prune_mode[(winner_mode_tx_type_pruning
        - 1 as core::ffi::c_int) as usize][is_winner_mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn set_tx_domain_dist_params(
    mut winner_mode_params: *const WinnerModeParams,
    mut txfm_params: *mut TxfmSearchParams,
    mut enable_winner_mode_for_tx_domain_dist: core::ffi::c_int,
    mut is_winner_mode: core::ffi::c_int,
) {
    if (*txfm_params).use_qm_dist_metric != 0 {
        (*txfm_params).use_transform_domain_distortion = 1 as core::ffi::c_uint;
        (*txfm_params).tx_domain_dist_threshold = 0 as core::ffi::c_uint;
        return;
    }
    if enable_winner_mode_for_tx_domain_dist == 0 {
        (*txfm_params).use_transform_domain_distortion = (*winner_mode_params)
            .use_transform_domain_distortion[DEFAULT_EVAL as core::ffi::c_int as usize];
        (*txfm_params).tx_domain_dist_threshold = (*winner_mode_params)
            .tx_domain_dist_threshold[DEFAULT_EVAL as core::ffi::c_int as usize];
        return;
    }
    if is_winner_mode != 0 {
        (*txfm_params).use_transform_domain_distortion = (*winner_mode_params)
            .use_transform_domain_distortion[WINNER_MODE_EVAL as core::ffi::c_int
            as usize];
        (*txfm_params).tx_domain_dist_threshold = (*winner_mode_params)
            .tx_domain_dist_threshold[WINNER_MODE_EVAL as core::ffi::c_int as usize];
    } else {
        (*txfm_params).use_transform_domain_distortion = (*winner_mode_params)
            .use_transform_domain_distortion[MODE_EVAL as core::ffi::c_int as usize];
        (*txfm_params).tx_domain_dist_threshold = (*winner_mode_params)
            .tx_domain_dist_threshold[MODE_EVAL as core::ffi::c_int as usize];
    };
}
#[inline]
unsafe extern "C-unwind" fn set_mode_eval_params(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mode_eval_type: MODE_EVAL_TYPE,
) {
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut sf: *const SPEED_FEATURES = &(*cpi).sf;
    let mut winner_mode_params: *const WinnerModeParams = &(*cpi).winner_mode_params;
    let mut txfm_params: *mut TxfmSearchParams = &raw mut (*x).txfm_search_params;
    (*txfm_params).use_qm_dist_metric = ((*cpi).oxcf.tune_cfg.dist_metric
        as core::ffi::c_uint
        == AOM_DIST_METRIC_QM_PSNR as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    match mode_eval_type as core::ffi::c_int {
        0 => {
            (*txfm_params).default_inter_tx_type_prob_thresh = INT_MAX;
            (*txfm_params).use_default_intra_tx_type = 0 as core::ffi::c_int;
            (*txfm_params).use_derived_intra_tx_type_set = 0 as core::ffi::c_int;
            (*txfm_params).skip_txfm_level = (*winner_mode_params)
                .skip_txfm_level[DEFAULT_EVAL as core::ffi::c_int as usize];
            (*txfm_params).predict_dc_level = (*winner_mode_params)
                .predict_dc_level[DEFAULT_EVAL as core::ffi::c_int as usize];
            set_tx_domain_dist_params(
                winner_mode_params,
                txfm_params,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            get_rd_opt_coeff_thresh(
                ((*winner_mode_params).coeff_opt_thresholds).as_ptr()
                    as *const [uint32_t; 2],
                txfm_params,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            set_tx_size_search_method(
                cm,
                winner_mode_params,
                txfm_params,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            set_tx_type_prune(
                sf,
                txfm_params,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
        }
        1 => {
            (*txfm_params).use_default_intra_tx_type = ((*cpi)
                .sf
                .tx_sf
                .tx_type_search
                .fast_intra_tx_type_search == 2 as core::ffi::c_int
                || (*cpi).oxcf.txfm_cfg.use_intra_default_tx_only as core::ffi::c_int
                    != 0) as core::ffi::c_int;
            (*txfm_params).use_derived_intra_tx_type_set = ((*cpi)
                .sf
                .tx_sf
                .tx_type_search
                .fast_intra_tx_type_search == 1 as core::ffi::c_int) as core::ffi::c_int;
            (*txfm_params).default_inter_tx_type_prob_thresh = (*cpi)
                .sf
                .tx_sf
                .tx_type_search
                .fast_inter_tx_type_prob_thresh;
            (*txfm_params).skip_txfm_level = (*winner_mode_params)
                .skip_txfm_level[MODE_EVAL as core::ffi::c_int as usize];
            (*txfm_params).predict_dc_level = (*winner_mode_params)
                .predict_dc_level[MODE_EVAL as core::ffi::c_int as usize];
            set_tx_domain_dist_params(
                winner_mode_params,
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_use_tx_domain_dist,
                0 as core::ffi::c_int,
            );
            get_rd_opt_coeff_thresh(
                ((*winner_mode_params).coeff_opt_thresholds).as_ptr()
                    as *const [uint32_t; 2],
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_coeff_opt,
                0 as core::ffi::c_int,
            );
            set_tx_size_search_method(
                cm,
                winner_mode_params,
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_tx_size_srch,
                0 as core::ffi::c_int,
            );
            set_tx_type_prune(
                sf,
                txfm_params,
                (*sf).tx_sf.tx_type_search.winner_mode_tx_type_pruning,
                0 as core::ffi::c_int,
            );
        }
        2 => {
            (*txfm_params).default_inter_tx_type_prob_thresh = INT_MAX;
            (*txfm_params).use_default_intra_tx_type = 0 as core::ffi::c_int;
            (*txfm_params).use_derived_intra_tx_type_set = 0 as core::ffi::c_int;
            (*txfm_params).skip_txfm_level = (*winner_mode_params)
                .skip_txfm_level[WINNER_MODE_EVAL as core::ffi::c_int as usize];
            (*txfm_params).predict_dc_level = (*winner_mode_params)
                .predict_dc_level[WINNER_MODE_EVAL as core::ffi::c_int as usize];
            set_tx_domain_dist_params(
                winner_mode_params,
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_use_tx_domain_dist,
                1 as core::ffi::c_int,
            );
            get_rd_opt_coeff_thresh(
                ((*winner_mode_params).coeff_opt_thresholds).as_ptr()
                    as *const [uint32_t; 2],
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_coeff_opt,
                1 as core::ffi::c_int,
            );
            set_tx_size_search_method(
                cm,
                winner_mode_params,
                txfm_params,
                (*sf).winner_mode_sf.enable_winner_mode_for_tx_size_srch,
                1 as core::ffi::c_int,
            );
            set_tx_type_prune(
                sf,
                txfm_params,
                (*sf).tx_sf.tx_type_search.winner_mode_tx_type_pruning,
                1 as core::ffi::c_int,
            );
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"set_mode_eval_params\0"))
                        .as_ptr(),
                    b"rdopt_utils.h\0" as *const u8 as *const core::ffi::c_char,
                    700 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    if (*txfm_params).mode_eval_type != mode_eval_type as core::ffi::c_int {
        reset_mb_rd_record((*x).txfm_search_info.mb_rd_record);
    }
    (*txfm_params).mode_eval_type = mode_eval_type as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_copy_usable_ref_mv_stack_and_weight(
    mut xd: *const MACROBLOCKD,
    mbmi_ext: *mut MB_MODE_INFO_EXT,
    mut ref_frame: MV_REFERENCE_FRAME,
) {
    memcpy(
        ((*mbmi_ext).weight[ref_frame as usize]).as_mut_ptr() as *mut core::ffi::c_void,
        ((*xd).weight[ref_frame as usize]).as_ptr() as *const core::ffi::c_void,
        (USABLE_REF_MV_STACK_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    memcpy(
        ((*mbmi_ext).ref_mv_stack[ref_frame as usize]).as_mut_ptr()
            as *mut core::ffi::c_void,
        ((*xd).ref_mv_stack[ref_frame as usize]).as_ptr() as *const core::ffi::c_void,
        (USABLE_REF_MV_STACK_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<CANDIDATE_MV>() as size_t),
    );
}
#[inline]
unsafe extern "C-unwind" fn av1_copy_mbmi_ext_to_mbmi_ext_frame(
    mut mbmi_ext_best: *mut MB_MODE_INFO_EXT_FRAME,
    mbmi_ext: *const MB_MODE_INFO_EXT,
    mut ref_frame_type: uint8_t,
) {
    memcpy(
        ((*mbmi_ext_best).ref_mv_stack).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mbmi_ext).ref_mv_stack[ref_frame_type as usize]).as_ptr()
            as *const core::ffi::c_void,
        ::core::mem::size_of::<[CANDIDATE_MV; 4]>() as size_t,
    );
    memcpy(
        ((*mbmi_ext_best).weight).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mbmi_ext).weight[ref_frame_type as usize]).as_ptr()
            as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as size_t,
    );
    (*mbmi_ext_best).mode_context = (*mbmi_ext).mode_context[ref_frame_type as usize];
    (*mbmi_ext_best).ref_mv_count = (*mbmi_ext).ref_mv_count[ref_frame_type as usize];
    memcpy(
        ((*mbmi_ext_best).global_mvs).as_mut_ptr() as *mut core::ffi::c_void,
        ((*mbmi_ext).global_mvs).as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[int_mv; 8]>() as size_t,
    );
}
pub const SUB_PARTITIONS_RECT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SUB_PARTITIONS_SPLIT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUB_PARTITIONS_AB: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUB_PARTITIONS_PART4: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_blk_has_rows_and_cols(
    mut blk_params: *const PartitionBlkParams,
) -> bool {
    return (*blk_params).has_rows != 0 && (*blk_params).has_cols != 0;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_whole_blk_in_frame(
    mut blk_params: *const PartitionBlkParams,
    mut mi_params: *const CommonModeInfoParams,
) -> bool {
    let mi_row: core::ffi::c_int = (*blk_params).mi_row;
    let mi_col: core::ffi::c_int = (*blk_params).mi_col;
    let bsize: BLOCK_SIZE = (*blk_params).bsize;
    return mi_row + mi_size_high[bsize as usize] as core::ffi::c_int
        <= (*mi_params).mi_rows
        && mi_col + mi_size_wide[bsize as usize] as core::ffi::c_int
            <= (*mi_params).mi_cols;
}
#[inline]
unsafe extern "C-unwind" fn update_filter_type_cdf(
    mut xd: *const MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
    mut dual_filter: core::ffi::c_int,
) {
    let mut dir: core::ffi::c_int = 0 as core::ffi::c_int;
    while dir < 2 as core::ffi::c_int {
        if dir != 0 && dual_filter == 0 {
            break;
        }
        let ctx: core::ffi::c_int = av1_get_pred_context_switchable_interp(xd, dir)
            as core::ffi::c_int;
        let mut filter: InterpFilter = av1_extract_interp_filter(
            (*mbmi).interp_filters,
            dir,
        );
        update_cdf(
            ((*(*xd).tile_ctx).switchable_interp_cdf[ctx as usize]).as_mut_ptr(),
            filter as int8_t,
            SWITCHABLE_FILTERS as core::ffi::c_int,
        );
        dir += 1;
    }
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
unsafe extern "C-unwind" fn get_model_rd_flag(
    mut cpi: *const AV1_COMP,
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let large_block: core::ffi::c_int = (bsize as core::ffi::c_int
        >= BLOCK_32X32 as core::ffi::c_int) as core::ffi::c_int;
    return ((*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint && large_block != 0
        && cyclic_refresh_segment_id_boosted(
            (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
                as core::ffi::c_int,
        ) == 0 && (*cm).quant_params.base_qindex != 0 && !(*cpi).oxcf.use_highbitdepth)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn find_predictors(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut frame_mv: *mut [int_mv; 8],
    mut yv12_mb: *mut [buf_2d; 3],
    mut bsize: BLOCK_SIZE,
    mut force_skip_low_temp_var: core::ffi::c_int,
    mut skip_pred_mv: core::ffi::c_int,
    mut use_scaled_ref_frame: *mut bool,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mbmi_ext: *mut MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let mut ref_0: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(cm, ref_frame);
    let ref_is_scaled: bool = (*ref_0).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        != (*cm).height
        || (*ref_0).c2rust_unnamed_1.c2rust_unnamed.y_crop_width != (*cm).width;
    let mut scaled_ref: *const YV12_BUFFER_CONFIG = av1_get_scaled_ref_frame(
        cpi,
        ref_frame as core::ffi::c_int,
    );
    let mut yv12: *const YV12_BUFFER_CONFIG = if ref_is_scaled as core::ffi::c_int != 0
        && !scaled_ref.is_null()
    {
        scaled_ref
    } else {
        ref_0
    };
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    (*x).pred_mv_sad[ref_frame as usize] = INT_MAX;
    (*x).pred_mv0_sad[ref_frame as usize] = INT_MAX;
    (*x).pred_mv1_sad[ref_frame as usize] = INT_MAX;
    (*frame_mv.offset(NEWMV as core::ffi::c_int as isize))[ref_frame as usize].as_int = INVALID_MV
        as uint32_t;
    if yv12.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"find_predictors\0"))
                .as_ptr(),
            b"nonrd_opt.h\0" as *const u8 as *const core::ffi::c_char,
            486 as core::ffi::c_int,
            b"yv12 != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !yv12.is_null() {
        let sf: *mut scale_factors = if !scaled_ref.is_null() {
            0 as *mut scale_factors
        } else {
            get_ref_scale_factors(cm, ref_frame) as *mut scale_factors
        };
        av1_setup_pred_block(
            xd,
            (*yv12_mb.offset(ref_frame as isize)).as_mut_ptr(),
            yv12,
            sf,
            sf,
            num_planes,
        );
        av1_find_mv_refs(
            cm,
            xd,
            mbmi,
            ref_frame,
            ((*mbmi_ext).ref_mv_count).as_mut_ptr(),
            ((*xd).ref_mv_stack).as_mut_ptr(),
            ((*xd).weight).as_mut_ptr(),
            0 as *mut [int_mv; 2],
            ((*mbmi_ext).global_mvs).as_mut_ptr(),
            ((*mbmi_ext).mode_context).as_mut_ptr(),
        );
        av1_copy_usable_ref_mv_stack_and_weight(xd, mbmi_ext, ref_frame);
        av1_find_best_ref_mvs_from_stack(
            (*cm).features.allow_high_precision_mv as core::ffi::c_int,
            mbmi_ext,
            ref_frame,
            &raw mut *(*frame_mv.offset(NEARESTMV as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(ref_frame as isize),
            &raw mut *(*frame_mv.offset(NEARMV as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(ref_frame as isize),
            0 as core::ffi::c_int,
        );
        (*frame_mv.offset(GLOBALMV as core::ffi::c_int as isize))[ref_frame as usize] = (*mbmi_ext)
            .global_mvs[ref_frame as usize];
        if is_one_pass_rt_lag_params(cpi) == 0 && !ref_is_scaled
            && bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
            && skip_pred_mv == 0
            && !(force_skip_low_temp_var != 0
                && ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int)
        {
            av1_mv_pred(
                cpi,
                x,
                (*yv12_mb.offset(ref_frame as isize))[0 as core::ffi::c_int as usize]
                    .buf,
                (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride,
                ref_frame as core::ffi::c_int,
                bsize,
            );
        }
    }
    if (*cm).features.switchable_motion_mode {
        av1_count_overlappable_neighbors(cm, xd);
    }
    (*mbmi).num_proj_ref = 1 as uint8_t;
    *use_scaled_ref_frame = ref_is_scaled as core::ffi::c_int != 0
        && !scaled_ref.is_null();
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
#[inline]
unsafe extern "C-unwind" fn update_cb_offsets(
    mut x: *mut MACROBLOCK,
    bsize: BLOCK_SIZE,
    subsampling_x: core::ffi::c_int,
    subsampling_y: core::ffi::c_int,
) {
    (*x).cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize] = ((*x)
        .cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize] as core::ffi::c_int
        + block_size_wide[bsize as usize] as core::ffi::c_int
            * block_size_high[bsize as usize] as core::ffi::c_int) as uint16_t;
    if (*x).e_mbd.is_chroma_ref {
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            subsampling_x,
            subsampling_y,
        ) as BLOCK_SIZE;
        if !(plane_bsize as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"update_cb_offsets\0"))
                    .as_ptr(),
                b"partition_search.h\0" as *const u8 as *const core::ffi::c_char,
                77 as core::ffi::c_int,
                b"plane_bsize != BLOCK_INVALID\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        (*x).cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize] = ((*x)
            .cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize] as core::ffi::c_int
            + block_size_wide[plane_bsize as usize] as core::ffi::c_int
                * block_size_high[plane_bsize as usize] as core::ffi::c_int) as uint16_t;
    }
}
unsafe extern "C-unwind" fn update_txfm_count(
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut counts: *mut FRAME_COUNTS,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut allow_update_cdf: uint8_t,
) {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let max_blocks_high: core::ffi::c_int = max_block_high(
        xd,
        bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(
        xd,
        bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut ctx: core::ffi::c_int = txfm_partition_context(
        ((*xd).above_txfm_context).offset(blk_col as isize),
        ((*xd).left_txfm_context).offset(blk_row as isize),
        (*mbmi).bsize,
        tx_size,
    );
    let txb_size_index: core::ffi::c_int = av1_get_txb_size_index(
        bsize,
        blk_row,
        blk_col,
    ) as core::ffi::c_int;
    let plane_tx_size: TX_SIZE = (*mbmi).inter_tx_size[txb_size_index as usize];
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    if !(tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"update_txfm_count\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            229 as core::ffi::c_int,
            b"tx_size > TX_4X4\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if depth == MAX_VARTX_DEPTH {
        (*mbmi).tx_size = tx_size;
        txfm_partition_update(
            ((*xd).above_txfm_context).offset(blk_col as isize),
            ((*xd).left_txfm_context).offset(blk_row as isize),
            tx_size,
            tx_size,
        );
        return;
    }
    if tx_size as core::ffi::c_int == plane_tx_size as core::ffi::c_int {
        if allow_update_cdf != 0 {
            update_cdf(
                ((*(*xd).tile_ctx).txfm_partition_cdf[ctx as usize]).as_mut_ptr(),
                0 as int8_t,
                2 as core::ffi::c_int,
            );
        }
        (*mbmi).tx_size = tx_size;
        txfm_partition_update(
            ((*xd).above_txfm_context).offset(blk_col as isize),
            ((*xd).left_txfm_context).offset(blk_row as isize),
            tx_size,
            tx_size,
        );
    } else {
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        let bsw: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let bsh: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        if allow_update_cdf != 0 {
            update_cdf(
                ((*(*xd).tile_ctx).txfm_partition_cdf[ctx as usize]).as_mut_ptr(),
                1 as int8_t,
                2 as core::ffi::c_int,
            );
        }
        (*x).txfm_search_info.txb_split_count = ((*x).txfm_search_info.txb_split_count)
            .wrapping_add(1);
        if sub_txs as core::ffi::c_int == TX_4X4 as core::ffi::c_int {
            (*mbmi).inter_tx_size[txb_size_index as usize] = TX_4X4 as core::ffi::c_int
                as TX_SIZE;
            (*mbmi).tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            txfm_partition_update(
                ((*xd).above_txfm_context).offset(blk_col as isize),
                ((*xd).left_txfm_context).offset(blk_row as isize),
                TX_4X4 as core::ffi::c_int as TX_SIZE,
                tx_size,
            );
            return;
        }
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < tx_size_high_unit[tx_size as usize] {
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < tx_size_wide_unit[tx_size as usize] {
                let mut offsetr: core::ffi::c_int = row;
                let mut offsetc: core::ffi::c_int = col;
                update_txfm_count(
                    x,
                    xd,
                    counts,
                    sub_txs,
                    depth + 1 as core::ffi::c_int,
                    blk_row + offsetr,
                    blk_col + offsetc,
                    allow_update_cdf,
                );
                col += bsw;
            }
            row += bsh;
        }
    };
}
unsafe extern "C-unwind" fn tx_partition_count_update(
    cm: *const AV1_COMMON,
    mut x: *mut MACROBLOCK,
    mut plane_bsize: BLOCK_SIZE,
    mut td_counts: *mut FRAME_COUNTS,
    mut allow_update_cdf: uint8_t,
) {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi_width: core::ffi::c_int = mi_size_wide[plane_bsize as usize]
        as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[plane_bsize as usize]
        as core::ffi::c_int;
    let max_tx_size: TX_SIZE = get_vartx_max_txsize(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as TX_SIZE;
    let bh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
    let bw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*xd).tile.tile_row as isize))
        .offset((*xd).mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset(((*xd).mi_row & MAX_MIB_MASK) as isize);
    let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy < mi_height {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < mi_width {
            update_txfm_count(
                x,
                xd,
                td_counts,
                max_tx_size,
                0 as core::ffi::c_int,
                idy,
                idx,
                allow_update_cdf,
            );
            idx += bw;
        }
        idy += bh;
    }
}
unsafe extern "C-unwind" fn set_txfm_context(
    mut xd: *mut MACROBLOCKD,
    mut tx_size: TX_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
) {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let max_blocks_high: core::ffi::c_int = max_block_high(
        xd,
        bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(
        xd,
        bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let txb_size_index: core::ffi::c_int = av1_get_txb_size_index(
        bsize,
        blk_row,
        blk_col,
    ) as core::ffi::c_int;
    let plane_tx_size: TX_SIZE = (*mbmi).inter_tx_size[txb_size_index as usize];
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    if tx_size as core::ffi::c_int == plane_tx_size as core::ffi::c_int {
        (*mbmi).tx_size = tx_size;
        txfm_partition_update(
            ((*xd).above_txfm_context).offset(blk_col as isize),
            ((*xd).left_txfm_context).offset(blk_row as isize),
            tx_size,
            tx_size,
        );
    } else {
        if tx_size as core::ffi::c_int == TX_8X8 as core::ffi::c_int {
            (*mbmi).inter_tx_size[txb_size_index as usize] = TX_4X4 as core::ffi::c_int
                as TX_SIZE;
            (*mbmi).tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            txfm_partition_update(
                ((*xd).above_txfm_context).offset(blk_col as isize),
                ((*xd).left_txfm_context).offset(blk_row as isize),
                TX_4X4 as core::ffi::c_int as TX_SIZE,
                tx_size,
            );
            return;
        }
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        let bsw: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let bsh: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        let row_end: core::ffi::c_int = if tx_size_high_unit[tx_size as usize]
            < max_blocks_high - blk_row
        {
            tx_size_high_unit[tx_size as usize]
        } else {
            max_blocks_high - blk_row
        };
        let col_end: core::ffi::c_int = if tx_size_wide_unit[tx_size as usize]
            < max_blocks_wide - blk_col
        {
            tx_size_wide_unit[tx_size as usize]
        } else {
            max_blocks_wide - blk_col
        };
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < row_end {
            let offsetr: core::ffi::c_int = blk_row + row;
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < col_end {
                let offsetc: core::ffi::c_int = blk_col + col;
                set_txfm_context(xd, sub_txs, offsetr, offsetc);
                col += bsw;
            }
            row += bsh;
        }
    };
}
unsafe extern "C-unwind" fn tx_partition_set_contexts(
    cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane_bsize: BLOCK_SIZE,
) {
    let mi_width: core::ffi::c_int = mi_size_wide[plane_bsize as usize]
        as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[plane_bsize as usize]
        as core::ffi::c_int;
    let max_tx_size: TX_SIZE = get_vartx_max_txsize(
        xd,
        plane_bsize,
        0 as core::ffi::c_int,
    ) as TX_SIZE;
    let bh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
    let bw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*xd).tile.tile_row as isize))
        .offset((*xd).mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset(((*xd).mi_row & MAX_MIB_MASK) as isize);
    let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy < mi_height {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < mi_width {
            set_txfm_context(xd, max_tx_size, idy, idx);
            idx += bw;
        }
        idy += bh;
    }
}
unsafe extern "C-unwind" fn update_zeromv_cnt(
    cpi: *const AV1_COMP,
    mi: *const MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    if (*mi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        != LAST_FRAME as core::ffi::c_int || is_inter_block(mi) == 0
        || (*mi).segment_id() as core::ffi::c_int > CR_SEGMENT_ID_BOOST2
    {
        return;
    }
    let cm: *const AV1_COMMON = &(*cpi).common;
    let mv: MV = (*mi).mv[0 as core::ffi::c_int as usize].as_mv;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    let xmis: core::ffi::c_int = if ((*cm).mi_params.mi_cols - mi_col
        >> 1 as core::ffi::c_int) < bw
    {
        (*cm).mi_params.mi_cols - mi_col >> 1 as core::ffi::c_int
    } else {
        bw
    };
    let ymis: core::ffi::c_int = if ((*cm).mi_params.mi_rows - mi_row
        >> 1 as core::ffi::c_int) < bh
    {
        (*cm).mi_params.mi_rows - mi_row >> 1 as core::ffi::c_int
    } else {
        bh
    };
    let block_index: core::ffi::c_int = (mi_row >> 1 as core::ffi::c_int)
        * ((*cm).mi_params.mi_cols >> 1 as core::ffi::c_int)
        + (mi_col >> 1 as core::ffi::c_int);
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < ymis {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < xmis {
            let map_offset: core::ffi::c_int = block_index
                + y * ((*cm).mi_params.mi_cols >> 1 as core::ffi::c_int) + x;
            if abs(mv.row as core::ffi::c_int) < 10 as core::ffi::c_int
                && abs(mv.col as core::ffi::c_int) < 10 as core::ffi::c_int
            {
                if (*((*cpi).consec_zero_mv).offset(map_offset as isize)
                    as core::ffi::c_int) < 255 as core::ffi::c_int
                {
                    let fresh20 = &raw mut *((*cpi).consec_zero_mv)
                        .offset(map_offset as isize);
                    *fresh20 = (*fresh20).wrapping_add(1);
                }
            } else {
                *((*cpi).consec_zero_mv).offset(map_offset as isize) = 0 as uint8_t;
            }
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C-unwind" fn encode_superblock(
    cpi: *const AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    mut td: *mut ThreadData,
    mut t: *mut *mut TokenExtra,
    mut dry_run: RUN_TYPE,
    mut bsize: BLOCK_SIZE,
    mut rate: *mut core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mi_4x4: *mut *mut MB_MODE_INFO = (*xd).mi;
    let mut mbmi: *mut MB_MODE_INFO = *mi_4x4.offset(0 as core::ffi::c_int as isize);
    let seg_skip: core::ffi::c_int = segfeature_active(
        &(*cm).seg,
        (*mbmi).segment_id(),
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) as core::ffi::c_int;
    let mis: core::ffi::c_int = (*cm).mi_params.mi_stride;
    let mi_width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let mut txfm_params: *mut TxfmSearchParams = &raw mut (*x).txfm_search_params;
    set_tx_size_search_method(
        cm,
        &(*cpi).winner_mode_params,
        txfm_params,
        (*cpi).sf.winner_mode_sf.enable_winner_mode_for_tx_size_srch,
        1 as core::ffi::c_int,
    );
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    if is_inter == 0 {
        (*xd).cfl.store_y = store_cfl_required(cm, xd) as core::ffi::c_int;
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < num_planes {
            av1_encode_intra_block_plane(
                cpi as *const AV1_COMP,
                x,
                bsize,
                plane,
                dry_run,
                (*cpi).optimize_seg_arr[(*mbmi).segment_id() as usize],
            );
            plane += 1;
        }
        if (*cpi).common.seg.segid_preskip == 0
            && (*cpi).common.seg.update_map as core::ffi::c_int != 0
            && (*cpi).enc_seg.has_lossless_segment as core::ffi::c_int != 0
        {
            (*mbmi).skip_txfm = 0 as uint8_t;
        }
        (*xd).cfl.store_y = 0 as core::ffi::c_int;
        if av1_allow_palette(
            (*cm).features.allow_screen_content_tools as core::ffi::c_int,
            bsize,
        ) != 0
        {
            if (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize]
                as core::ffi::c_int > 0 as core::ffi::c_int
                && dry_run as core::ffi::c_int == OUTPUT_ENABLED as core::ffi::c_int
            {
                (*x).palette_pixels
                    += block_size_wide[bsize as usize] as core::ffi::c_int
                        * block_size_high[bsize as usize] as core::ffi::c_int;
            }
            let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane_0
                < (if (2 as core::ffi::c_int) < num_planes {
                    2 as core::ffi::c_int
                } else {
                    num_planes
                })
            {
                if (*mbmi).palette_mode_info.palette_size[plane_0 as usize]
                    as core::ffi::c_int > 0 as core::ffi::c_int
                {
                    if dry_run == 0 {
                        av1_tokenize_color_map(
                            x,
                            plane_0,
                            t,
                            bsize,
                            (*mbmi).tx_size,
                            PALETTE_MAP as core::ffi::c_int as COLOR_MAP_TYPE,
                            (*tile_data).allow_update_cdf as core::ffi::c_int,
                            (*td).counts as *mut FRAME_COUNTS,
                        );
                    } else if dry_run as core::ffi::c_int
                        == DRY_RUN_COSTCOEFFS as core::ffi::c_int
                    {
                        *rate
                            += av1_cost_color_map(
                                x,
                                plane_0,
                                bsize,
                                (*mbmi).tx_size,
                                PALETTE_MAP as core::ffi::c_int as COLOR_MAP_TYPE,
                            );
                    }
                }
                plane_0 += 1;
            }
        }
        av1_update_intra_mb_txb_context(
            cpi,
            td,
            dry_run,
            bsize,
            (*tile_data).allow_update_cdf,
        );
    } else {
        let mut ref_0: core::ffi::c_int = 0;
        let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
        set_ref_ptrs(
            cm,
            xd,
            (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
            (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
        );
        ref_0 = 0 as core::ffi::c_int;
        while ref_0 < 1 as core::ffi::c_int + is_compound {
            let mut cfg: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
                cm,
                (*mbmi).ref_frame[ref_0 as usize],
            );
            if !(is_intrabc_block(mbmi) != 0 || !cfg.is_null()) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"encode_superblock\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    463 as core::ffi::c_int,
                    b"IMPLIES(!is_intrabc_block(mbmi), cfg)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            av1_setup_pre_planes(
                xd,
                ref_0,
                cfg,
                mi_row,
                mi_col,
                (*xd).block_ref_scale_factors[ref_0 as usize],
                num_planes,
            );
            ref_0 += 1;
        }
        let start_plane: core::ffi::c_int = if (*x).reuse_inter_pred as core::ffi::c_int
            != 0 && (*cpi).sf.rt_sf.nonrd_check_partition_split == 0
            && (*(*cm).seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        av1_enc_build_inter_predictor(
            cm,
            xd,
            mi_row,
            mi_col,
            0 as *const BUFFER_SET,
            bsize,
            start_plane,
            av1_num_planes(cm) - 1 as core::ffi::c_int,
        );
        if (*mbmi).motion_mode as core::ffi::c_int == OBMC_CAUSAL as core::ffi::c_int {
            if !(*cpi).oxcf.motion_mode_cfg.enable_obmc as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"encode_superblock\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    478 as core::ffi::c_int,
                    b"cpi->oxcf.motion_mode_cfg.enable_obmc\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            av1_build_obmc_inter_predictors_sb(cm, xd);
        }
        av1_encode_sb(cpi as *const AV1_COMP, x, bsize, dry_run);
        av1_tokenize_sb_vartx(
            cpi as *const AV1_COMP,
            td as *mut ThreadData,
            dry_run,
            bsize,
            rate,
            (*tile_data).allow_update_cdf,
        );
    }
    if dry_run == 0 {
        if av1_allow_intrabc(cm) != 0 && is_intrabc_block(mbmi) != 0 {
            (*td).intrabc_used = 1 as core::ffi::c_int;
        }
        if (*txfm_params).tx_mode_search_type as core::ffi::c_int
            == TX_MODE_SELECT as core::ffi::c_int
            && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
            && (*mbmi).bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int
            && !(is_inter != 0
                && ((*mbmi).skip_txfm as core::ffi::c_int != 0 || seg_skip != 0))
        {
            if is_inter != 0 {
                tx_partition_count_update(
                    cm,
                    x,
                    bsize,
                    (*td).counts,
                    (*tile_data).allow_update_cdf,
                );
            } else {
                if (*mbmi).tx_size as core::ffi::c_int
                    != max_txsize_rect_lookup[bsize as usize] as core::ffi::c_int
                {
                    (*x).txfm_search_info.txb_split_count = ((*x)
                        .txfm_search_info
                        .txb_split_count)
                        .wrapping_add(1);
                }
                if block_signals_txsize(bsize) != 0 {
                    let tx_size_ctx: core::ffi::c_int = get_tx_size_context(xd)
                        as core::ffi::c_int;
                    let tx_size_cat: int32_t = bsize_to_tx_size_cat(bsize) as int32_t;
                    let depth: core::ffi::c_int = tx_size_to_depth(
                        (*mbmi).tx_size,
                        bsize,
                    ) as core::ffi::c_int;
                    let max_depths: core::ffi::c_int = bsize_to_max_depth(bsize)
                        as core::ffi::c_int;
                    if (*tile_data).allow_update_cdf != 0 {
                        update_cdf(
                            ((*(*xd).tile_ctx)
                                .tx_size_cdf[tx_size_cat as usize][tx_size_ctx as usize])
                                .as_mut_ptr(),
                            depth as int8_t,
                            max_depths + 1 as core::ffi::c_int,
                        );
                    }
                }
            }
            if !(is_rect_tx((*mbmi).tx_size) == 0 || is_rect_tx_allowed(xd, mbmi) != 0)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"encode_superblock\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    532 as core::ffi::c_int,
                    b"IMPLIES(is_rect_tx(mbmi->tx_size), is_rect_tx_allowed(xd, mbmi))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else {
            let mut i: core::ffi::c_int = 0;
            let mut j: core::ffi::c_int = 0;
            let mut intra_tx_size: TX_SIZE = 0;
            if is_inter != 0 {
                if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
                    intra_tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
                } else {
                    intra_tx_size = tx_size_from_tx_mode(
                        bsize,
                        (*txfm_params).tx_mode_search_type,
                    );
                }
            } else {
                intra_tx_size = (*mbmi).tx_size;
            }
            let cols: core::ffi::c_int = if (*cm).mi_params.mi_cols - mi_col < mi_width {
                (*cm).mi_params.mi_cols - mi_col
            } else {
                mi_width
            };
            let rows: core::ffi::c_int = if (*cm).mi_params.mi_rows - mi_row < mi_height
            {
                (*cm).mi_params.mi_rows - mi_row
            } else {
                mi_height
            };
            j = 0 as core::ffi::c_int;
            while j < rows {
                i = 0 as core::ffi::c_int;
                while i < cols {
                    (**mi_4x4.offset((mis * j + i) as isize)).tx_size = intra_tx_size;
                    i += 1;
                }
                j += 1;
            }
            if intra_tx_size as core::ffi::c_int
                != max_txsize_rect_lookup[bsize as usize] as core::ffi::c_int
            {
                (*x).txfm_search_info.txb_split_count = ((*x)
                    .txfm_search_info
                    .txb_split_count)
                    .wrapping_add(1);
            }
        }
    }
    if (*txfm_params).tx_mode_search_type as core::ffi::c_int
        == TX_MODE_SELECT as core::ffi::c_int && block_signals_txsize((*mbmi).bsize) != 0
        && is_inter != 0
        && !((*mbmi).skip_txfm as core::ffi::c_int != 0 || seg_skip != 0)
        && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
    {
        if dry_run != 0 {
            tx_partition_set_contexts(cm, xd, bsize);
        }
    } else {
        let mut tx_size: TX_SIZE = (*mbmi).tx_size;
        if is_inter != 0 {
            if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
                tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            } else {
                tx_size = tx_size_from_tx_mode(
                    bsize,
                    (*txfm_params).tx_mode_search_type,
                );
            }
        } else {
            tx_size = (if bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int {
                tx_size as core::ffi::c_int
            } else {
                TX_4X4 as core::ffi::c_int
            }) as TX_SIZE;
        }
        (*mbmi).tx_size = tx_size;
        set_txfm_ctxs(
            tx_size,
            (*xd).width as core::ffi::c_int,
            (*xd).height as core::ffi::c_int,
            (((*mbmi).skip_txfm as core::ffi::c_int != 0 || seg_skip != 0)
                && is_inter_block(mbmi) != 0) as core::ffi::c_int,
            xd,
        );
    }
    if is_inter_block(mbmi) != 0 && !(*xd).is_chroma_ref
        && is_cfl_allowed(xd) as core::ffi::c_int != 0
    {
        cfl_store_block(xd, (*mbmi).bsize, (*mbmi).tx_size);
    }
    if dry_run == 0 {
        if (*cpi).oxcf.pass as core::ffi::c_uint
            == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
            && (*cpi).svc.temporal_layer_id == 0 as core::ffi::c_int
            && (*cpi).sf.rt_sf.use_temporal_noise_estimate != 0
            && ((*(*cpi).ppi).use_svc == 0
                || (*(*cpi).ppi).use_svc != 0
                    && (*((*cpi).svc.layer_context)
                        .offset((*cpi).svc.temporal_layer_id as isize))
                        .is_key_frame == 0
                    && (*cpi).svc.spatial_layer_id
                        == (*cpi).svc.number_spatial_layers - 1 as core::ffi::c_int)
        {
            update_zeromv_cnt(cpi, mbmi, mi_row, mi_col, bsize);
        }
    }
}
unsafe extern "C-unwind" fn setup_block_rdmult(
    cpi: *const AV1_COMP,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut aq_mode: AQ_MODE,
    mut mbmi: *mut MB_MODE_INFO,
) {
    (*x).rdmult = (*cpi).rd.RDMULT;
    if aq_mode as core::ffi::c_int != NO_AQ as core::ffi::c_int {
        if mbmi.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"setup_block_rdmult\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                602 as core::ffi::c_int,
                b"mbmi != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if aq_mode as core::ffi::c_int == VARIANCE_AQ as core::ffi::c_int {
            if (*cpi).vaq_refresh != 0 {
                let energy: core::ffi::c_int = if bsize as core::ffi::c_int
                    <= BLOCK_16X16 as core::ffi::c_int
                {
                    (*x).mb_energy
                } else {
                    av1_log_block_var(cpi, x, bsize) as core::ffi::c_int
                };
                (*mbmi).set_segment_id(energy as uint8_t as uint8_t);
            }
            (*x).rdmult = set_rdmult(cpi, x, (*mbmi).segment_id() as core::ffi::c_int);
        } else if aq_mode as core::ffi::c_int == COMPLEXITY_AQ as core::ffi::c_int {
            (*x).rdmult = set_rdmult(cpi, x, (*mbmi).segment_id() as core::ffi::c_int);
        } else if aq_mode as core::ffi::c_int == CYCLIC_REFRESH_AQ as core::ffi::c_int {
            if cyclic_refresh_segment_id_boosted(
                (*mbmi).segment_id() as core::ffi::c_int,
            ) != 0
            {
                (*x).rdmult = av1_cyclic_refresh_get_rdmult((*cpi).cyclic_refresh);
            }
        }
    }
    if (*cpi).cb_delta_rdmult_enabled != 0 && (*cpi).sf.rt_sf.use_nonrd_pick_mode == 0 {
        (*x).rdmult = av1_get_cb_rdmult(cpi, x, bsize, mi_row, mi_col);
    }
    if (*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
        == AOM_TUNE_SSIM as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
            == AOM_TUNE_IQ as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).oxcf.tune_cfg.tuning as core::ffi::c_uint
            == AOM_TUNE_SSIMULACRA2 as core::ffi::c_int as core::ffi::c_uint
    {
        av1_set_ssim_rdmult(
            cpi,
            &raw mut (*x).errorperbit,
            bsize,
            mi_row,
            mi_col,
            &raw mut (*x).rdmult,
        );
    }
    if (*cpi).oxcf.mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int {
        (*x).rdmult = ((*x).rdmult as int64_t * (*x).intra_sb_rdmult_modifier as int64_t
            >> 7 as core::ffi::c_int) as core::ffi::c_int;
    }
    (*x).rdmult = if (*x).rdmult > 0 as core::ffi::c_int {
        (*x).rdmult
    } else {
        1 as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_offsets_without_segment_id(
    cpi: *const AV1_COMP,
    tile: *const TileInfo,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_set_offsets_without_segment_id\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            667 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mi_width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mi_height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    set_mode_info_offsets(
        &(*cpi).common.mi_params,
        &(*cpi).mbmi_ext_info,
        x,
        xd,
        mi_row,
        mi_col,
    );
    set_entropy_context(xd, mi_row, mi_col, num_planes);
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        bsize,
        &raw mut (*(*cm).cur_frame).buf,
        mi_row,
        mi_col,
        0 as core::ffi::c_int,
        num_planes,
    );
    av1_set_mv_limits(
        &(*cm).mi_params,
        &raw mut (*x).mv_limits,
        mi_row,
        mi_col,
        mi_height,
        mi_width,
        (*cpi).oxcf.border_in_pixels,
    );
    set_plane_n4(xd, mi_width, mi_height, num_planes);
    if !(mi_col & mi_width - 1 as core::ffi::c_int == 0
        && mi_row & mi_height - 1 as core::ffi::c_int == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_set_offsets_without_segment_id\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            691 as core::ffi::c_int,
            b"!(mi_col & (mi_width - 1)) && !(mi_row & (mi_height - 1))\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    set_mi_row_col(
        xd,
        tile,
        mi_row,
        mi_height,
        mi_col,
        mi_width,
        (*cm).mi_params.mi_rows,
        (*cm).mi_params.mi_cols,
    );
    set_pixels_to_frame_edge(
        x,
        mi_width,
        mi_height,
        mi_col,
        mi_row,
        (*cm).mi_params.mi_cols,
        (*cm).mi_params.mi_rows,
        (*cm).width,
        (*cm).height,
        (*cpi).do_border_pad,
    );
    av1_setup_src_planes(
        x as *mut macroblock,
        (*cpi).source,
        mi_row,
        mi_col,
        num_planes,
        bsize,
    );
    (*xd).tile = *tile;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_offsets(
    cpi: *const AV1_COMP,
    tile: *const TileInfo,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let seg: *const segmentation = &(*cm).seg;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = 0 as *mut MB_MODE_INFO;
    av1_set_offsets_without_segment_id(cpi, tile, x, mi_row, mi_col, bsize);
    mbmi = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).set_segment_id(0 as uint8_t as uint8_t);
    if (*seg).enabled != 0 {
        if (*seg).enabled as core::ffi::c_int != 0 && (*cpi).vaq_refresh == 0 {
            let map: *const uint8_t = if (*seg).update_map as core::ffi::c_int != 0 {
                (*cpi).enc_seg.map
            } else {
                (*cm).last_frame_seg_map
            };
            (*mbmi)
                .set_segment_id(
                    (if !map.is_null() {
                        get_segment_id(&(*cm).mi_params, map, bsize, mi_row, mi_col)
                            as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint8_t as uint8_t,
                );
        }
        av1_init_plane_quantizers(
            cpi as *const AV1_COMP,
            x,
            (*mbmi).segment_id() as core::ffi::c_int,
            0 as core::ffi::c_int,
        );
    }
    (*x).last_set_offsets_loc.mi_row = mi_row;
    (*x).last_set_offsets_loc.mi_col = mi_col;
    (*x).last_set_offsets_loc.bsize = bsize;
}
#[inline]
unsafe extern "C-unwind" fn hybrid_intra_mode_search(
    mut cpi: *mut AV1_COMP,
    x: *mut MACROBLOCK,
    mut rd_cost: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ctx: *mut PICK_MODE_CONTEXT,
) {
    let mut use_rdopt: core::ffi::c_int = 0 as core::ffi::c_int;
    let hybrid_intra_pickmode: core::ffi::c_int = (*cpi).sf.rt_sf.hybrid_intra_pickmode;
    if hybrid_intra_pickmode != 0
        && (bsize as core::ffi::c_int) < BLOCK_16X16 as core::ffi::c_int
    {
        let mut var_thresh: [core::ffi::c_uint; 3] = [
            0 as core::ffi::c_int as core::ffi::c_uint,
            101 as core::ffi::c_int as core::ffi::c_uint,
            201 as core::ffi::c_int as core::ffi::c_uint,
        ];
        if !(hybrid_intra_pickmode <= 3 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"hybrid_intra_mode_search\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                766 as core::ffi::c_int,
                b"hybrid_intra_pickmode <= 3\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (*x).source_variance
            >= var_thresh[(hybrid_intra_pickmode - 1 as core::ffi::c_int) as usize]
        {
            use_rdopt = 1 as core::ffi::c_int;
        }
    }
    if use_rdopt != 0 {
        av1_rd_pick_intra_mode_sb(
            cpi,
            x as *mut macroblock,
            rd_cost as *mut RD_STATS,
            bsize,
            ctx,
            INT64_MAX as int64_t,
        );
    } else {
        av1_nonrd_pick_intra_mode(cpi, x, rd_cost, bsize, ctx);
    };
}
#[inline]
unsafe extern "C-unwind" fn wait_for_top_right_sb(
    mut enc_row_mt: *mut AV1EncRowMultiThreadInfo,
    mut row_mt_sync: *mut AV1EncRowMultiThreadSync,
    mut tile_info: *mut TileInfo,
    mut sb_size: BLOCK_SIZE,
    mut sb_mi_size_log2: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let sb_size_in_mi: core::ffi::c_int = mi_size_wide[sb_size as usize]
        as core::ffi::c_int;
    let bw_in_mi: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let blk_row_in_sb: core::ffi::c_int = mi_row & sb_size_in_mi - 1 as core::ffi::c_int;
    let blk_col_in_sb: core::ffi::c_int = mi_col & sb_size_in_mi - 1 as core::ffi::c_int;
    let top_right_block_in_sb: core::ffi::c_int = (blk_row_in_sb == 0 as core::ffi::c_int
        && blk_col_in_sb + bw_in_mi >= sb_size_in_mi) as core::ffi::c_int;
    if top_right_block_in_sb == 0 {
        return;
    }
    let sb_row_in_tile: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
        >> sb_mi_size_log2;
    let sb_col_in_tile: core::ffi::c_int = mi_col - (*tile_info).mi_col_start
        >> sb_mi_size_log2;
    ((*enc_row_mt).sync_read_ptr)
        .expect(
            "non-null function pointer",
        )(row_mt_sync, sb_row_in_tile, sb_col_in_tile);
}
unsafe extern "C-unwind" fn pick_sb_modes(
    cpi: *mut AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut rd_cost: *mut RD_STATS,
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
    mut ctx: *mut PICK_MODE_CONTEXT,
    mut best_rd: RD_STATS,
) {
    if (*cpi).sf.part_sf.use_best_rd_for_pruning != 0 && best_rd.rdcost < 0 as int64_t {
        (*ctx).rd_stats.rdcost = INT64_MAX as int64_t;
        (*ctx).rd_stats.skip_txfm = 0 as uint8_t;
        av1_invalid_rd_stats(rd_cost);
        return;
    }
    av1_set_offsets(cpi, &raw mut (*tile_data).tile_info, x, mi_row, mi_col, bsize);
    if (*cpi).sf.part_sf.reuse_prev_rd_results_for_part_ab != 0
        && (*ctx).rd_mode_is_ready != 0
    {
        if !((*ctx).mic.bsize as core::ffi::c_int == bsize as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"pick_sb_modes\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                859 as core::ffi::c_int,
                b"ctx->mic.bsize == bsize\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*ctx).mic.partition as core::ffi::c_int == partition as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"pick_sb_modes\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                860 as core::ffi::c_int,
                b"ctx->mic.partition == partition\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        (*rd_cost).rate = (*ctx).rd_stats.rate;
        (*rd_cost).dist = (*ctx).rd_stats.dist;
        (*rd_cost).rdcost = (*ctx).rd_stats.rdcost;
        return;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = 0 as *mut MB_MODE_INFO;
    let p: *mut macroblock_plane = ((*x).plane).as_mut_ptr();
    let pd: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
    let aq_mode: AQ_MODE = (*cpi).oxcf.q_cfg.aq_mode;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    let mut i: core::ffi::c_int = 0;
    wait_for_top_right_sb(
        &raw mut (*cpi).mt_info.enc_row_mt,
        &raw mut (*tile_data).row_mt_sync,
        &raw mut (*tile_data).tile_info,
        (*(*cm).seq_params).sb_size,
        (*(*cm).seq_params).mib_size_log2,
        bsize,
        mi_row,
        mi_col,
    );
    mbmi = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).bsize = bsize;
    (*mbmi).partition = partition;
    (*xd).tx_type_map = ((*txfm_info).tx_type_map_).as_mut_ptr();
    (*xd).tx_type_map_stride = mi_size_wide[bsize as usize] as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        let fresh21 = &raw mut (*p.offset(i as isize)).coeff;
        *fresh21 = (*ctx).coeff[i as usize];
        let fresh22 = &raw mut (*p.offset(i as isize)).qcoeff;
        *fresh22 = (*ctx).qcoeff[i as usize];
        let fresh23 = &raw mut (*p.offset(i as isize)).dqcoeff;
        *fresh23 = (*ctx).dqcoeff[i as usize];
        let fresh24 = &raw mut (*p.offset(i as isize)).eobs;
        *fresh24 = (*ctx).eobs[i as usize];
        let fresh25 = &raw mut (*p.offset(i as isize)).txb_entropy_ctx;
        *fresh25 = (*ctx).txb_entropy_ctx[i as usize];
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let fresh26 = &raw mut (*pd.offset(i as isize)).color_index_map;
        *fresh26 = (*ctx).color_index_map[i as usize];
        i += 1;
    }
    (*ctx).skippable = 0 as core::ffi::c_int;
    (*mbmi).skip_txfm = 0 as uint8_t;
    (*mbmi).set_skip_mode(0 as uint8_t as uint8_t);
    (*x).source_variance = av1_get_perpixel_variance_facade(
        cpi,
        xd,
        &raw mut (*((*x).plane).as_mut_ptr().offset(0 as core::ffi::c_int as isize)).src,
        bsize,
        AOM_PLANE_Y,
    );
    set_mode_eval_params(cpi, x, DEFAULT_EVAL as core::ffi::c_int as MODE_EVAL_TYPE);
    let orig_rdmult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(cpi, x, mi_row, mi_col, bsize, aq_mode, mbmi);
    av1_set_error_per_bit(&raw mut (*x).errorperbit, (*x).rdmult);
    av1_rd_cost_update((*x).rdmult, &raw mut best_rd);
    if (*cpi).sf.part_sf.use_best_rd_for_pruning == 0 {
        av1_invalid_rd_stats(&raw mut best_rd);
    }
    if frame_is_intra_only(cm) != 0 {
        av1_rd_pick_intra_mode_sb(
            cpi,
            x as *mut macroblock,
            rd_cost as *mut RD_STATS,
            bsize,
            ctx,
            best_rd.rdcost,
        );
    } else if segfeature_active(
        &raw mut (*cm).seg,
        (*mbmi).segment_id(),
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        av1_rd_pick_inter_mode_sb_seg_skip(
            cpi,
            tile_data as *mut TileDataEnc,
            x as *mut macroblock,
            mi_row,
            mi_col,
            rd_cost as *mut RD_STATS,
            bsize,
            ctx,
            best_rd.rdcost,
        );
    } else {
        av1_rd_pick_inter_mode(
            cpi as *mut AV1_COMP,
            tile_data as *mut TileDataEnc,
            x as *mut macroblock,
            rd_cost as *mut RD_STATS,
            bsize,
            ctx,
            best_rd.rdcost,
        );
    }
    if (*rd_cost).rate != INT_MAX
        && aq_mode as core::ffi::c_int == COMPLEXITY_AQ as core::ffi::c_int
        && bsize as core::ffi::c_int >= BLOCK_16X16 as core::ffi::c_int
    {
        av1_caq_select_segment(
            cpi,
            x as *mut macroblock,
            bsize,
            mi_row,
            mi_col,
            (*rd_cost).rate,
        );
    }
    (*x).rdmult = orig_rdmult;
    if (*rd_cost).rate == INT_MAX {
        (*rd_cost).rdcost = INT64_MAX as int64_t;
    }
    (*ctx).rd_stats.rate = (*rd_cost).rate;
    (*ctx).rd_stats.dist = (*rd_cost).dist;
    (*ctx).rd_stats.rdcost = (*rd_cost).rdcost;
}
unsafe extern "C-unwind" fn update_stats(cm: *const AV1_COMMON, mut td: *mut ThreadData) {
    let mut x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mbmi_ext: *const MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let current_frame: *const CurrentFrame = &(*cm).current_frame;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let mut fc: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let seg_ref_active: core::ffi::c_int = segfeature_active(
        &(*cm).seg,
        (*mbmi).segment_id(),
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) as core::ffi::c_int;
    if (*current_frame).skip_mode_info.skip_mode_flag != 0 && seg_ref_active == 0
        && is_comp_ref_allowed(bsize) != 0
    {
        let skip_mode_ctx: core::ffi::c_int = av1_get_skip_mode_context(xd)
            as core::ffi::c_int;
        update_cdf(
            ((*fc).skip_mode_cdfs[skip_mode_ctx as usize]).as_mut_ptr(),
            (*mbmi).skip_mode() as int8_t,
            2 as core::ffi::c_int,
        );
    }
    if (*mbmi).skip_mode() == 0 && seg_ref_active == 0 {
        let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd)
            as core::ffi::c_int;
        update_cdf(
            ((*fc).skip_txfm_cdfs[skip_ctx as usize]).as_mut_ptr(),
            (*mbmi).skip_txfm as int8_t,
            2 as core::ffi::c_int,
        );
    }
    if is_inter_block(mbmi) == 0 {
        av1_sum_intra_stats(
            cm,
            (*td).counts,
            xd,
            mbmi,
            (*xd).above_mbmi,
            (*xd).left_mbmi,
            frame_is_intra_only(cm),
        );
    }
    if av1_allow_intrabc(cm) != 0 {
        let is_intrabc: core::ffi::c_int = is_intrabc_block(mbmi) as core::ffi::c_int;
        update_cdf(
            ((*fc).intrabc_cdf).as_mut_ptr(),
            is_intrabc as int8_t,
            2 as core::ffi::c_int,
        );
        if is_intrabc != 0 {
            let ref_frame_type: int8_t = av1_ref_frame_type(((*mbmi).ref_frame).as_ptr())
                as int8_t;
            let dv_ref: int_mv = (*mbmi_ext)
                .ref_mv_stack[ref_frame_type as usize][0 as core::ffi::c_int as usize]
                .this_mv;
            av1_update_mv_stats(
                &(*((*mbmi).mv).as_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
                &dv_ref.as_mv,
                &raw mut (*fc).ndvc,
                MV_SUBPEL_NONE as core::ffi::c_int as MvSubpelPrecision,
            );
        }
    }
    if frame_is_intra_only(cm) != 0 || (*mbmi).skip_mode() as core::ffi::c_int != 0 {
        return;
    }
    let counts: *mut FRAME_COUNTS = (*td).counts;
    let inter_block: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    if seg_ref_active == 0 {
        update_cdf(
            ((*fc).intra_inter_cdf[av1_get_intra_inter_context(xd) as usize])
                .as_mut_ptr(),
            inter_block as int8_t,
            2 as core::ffi::c_int,
        );
        if inter_block != 0 {
            let ref0: MV_REFERENCE_FRAME = (*mbmi)
                .ref_frame[0 as core::ffi::c_int as usize];
            let ref1: MV_REFERENCE_FRAME = (*mbmi)
                .ref_frame[1 as core::ffi::c_int as usize];
            if (*current_frame).reference_mode as core::ffi::c_int
                == REFERENCE_MODE_SELECT as core::ffi::c_int
            {
                if is_comp_ref_allowed(bsize) != 0 {
                    update_cdf(
                        av1_get_reference_mode_cdf(xd),
                        has_second_ref(mbmi) as int8_t,
                        2 as core::ffi::c_int,
                    );
                }
            }
            if has_second_ref(mbmi) != 0 {
                let comp_ref_type: COMP_REFERENCE_TYPE = (if has_uni_comp_refs(mbmi) != 0
                {
                    UNIDIR_COMP_REFERENCE as core::ffi::c_int
                } else {
                    BIDIR_COMP_REFERENCE as core::ffi::c_int
                }) as COMP_REFERENCE_TYPE;
                update_cdf(
                    av1_get_comp_reference_type_cdf(xd),
                    comp_ref_type as int8_t,
                    COMP_REFERENCE_TYPES as core::ffi::c_int,
                );
                if comp_ref_type as core::ffi::c_int
                    == UNIDIR_COMP_REFERENCE as core::ffi::c_int
                {
                    let bit: core::ffi::c_int = (ref0 as core::ffi::c_int
                        == BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
                    update_cdf(
                        av1_get_pred_cdf_uni_comp_ref_p(xd),
                        bit as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if bit == 0 {
                        let bit1: core::ffi::c_int = (ref1 as core::ffi::c_int
                            == LAST3_FRAME as core::ffi::c_int
                            || ref1 as core::ffi::c_int
                                == GOLDEN_FRAME as core::ffi::c_int) as core::ffi::c_int;
                        update_cdf(
                            av1_get_pred_cdf_uni_comp_ref_p1(xd),
                            bit1 as int8_t,
                            2 as core::ffi::c_int,
                        );
                        if bit1 != 0 {
                            update_cdf(
                                av1_get_pred_cdf_uni_comp_ref_p2(xd),
                                (ref1 as core::ffi::c_int
                                    == GOLDEN_FRAME as core::ffi::c_int) as core::ffi::c_int
                                    as int8_t,
                                2 as core::ffi::c_int,
                            );
                        }
                    }
                } else {
                    let bit_0: core::ffi::c_int = (ref0 as core::ffi::c_int
                        == GOLDEN_FRAME as core::ffi::c_int
                        || ref0 as core::ffi::c_int == LAST3_FRAME as core::ffi::c_int)
                        as core::ffi::c_int;
                    update_cdf(
                        av1_get_pred_cdf_comp_ref_p(xd),
                        bit_0 as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if bit_0 == 0 {
                        update_cdf(
                            av1_get_pred_cdf_comp_ref_p1(xd),
                            (ref0 as core::ffi::c_int == LAST2_FRAME as core::ffi::c_int)
                                as core::ffi::c_int as int8_t,
                            2 as core::ffi::c_int,
                        );
                    } else {
                        update_cdf(
                            av1_get_pred_cdf_comp_ref_p2(xd),
                            (ref0 as core::ffi::c_int
                                == GOLDEN_FRAME as core::ffi::c_int) as core::ffi::c_int
                                as int8_t,
                            2 as core::ffi::c_int,
                        );
                    }
                    update_cdf(
                        av1_get_pred_cdf_comp_bwdref_p(xd),
                        (ref1 as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int)
                            as core::ffi::c_int as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if ref1 as core::ffi::c_int != ALTREF_FRAME as core::ffi::c_int {
                        update_cdf(
                            av1_get_pred_cdf_comp_bwdref_p1(xd),
                            (ref1 as core::ffi::c_int
                                == ALTREF2_FRAME as core::ffi::c_int) as core::ffi::c_int
                                as int8_t,
                            2 as core::ffi::c_int,
                        );
                    }
                }
            } else {
                let bit_1: core::ffi::c_int = (ref0 as core::ffi::c_int
                    >= BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
                update_cdf(
                    av1_get_pred_cdf_single_ref_p1(xd),
                    bit_1 as int8_t,
                    2 as core::ffi::c_int,
                );
                if bit_1 != 0 {
                    if !(ref0 as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int)
                        as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 13],
                                [core::ffi::c_char; 13],
                            >(*b"update_stats\0"))
                                .as_ptr(),
                            b"partition_search.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1180 as core::ffi::c_int,
                            b"ref0 <= ALTREF_FRAME\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    update_cdf(
                        av1_get_pred_cdf_single_ref_p2(xd),
                        (ref0 as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int)
                            as core::ffi::c_int as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if ref0 as core::ffi::c_int != ALTREF_FRAME as core::ffi::c_int {
                        update_cdf(
                            av1_get_pred_cdf_single_ref_p6(xd),
                            (ref0 as core::ffi::c_int
                                == ALTREF2_FRAME as core::ffi::c_int) as core::ffi::c_int
                                as int8_t,
                            2 as core::ffi::c_int,
                        );
                    }
                } else {
                    let bit1_0: core::ffi::c_int = !(ref0 as core::ffi::c_int
                        == LAST2_FRAME as core::ffi::c_int
                        || ref0 as core::ffi::c_int == LAST_FRAME as core::ffi::c_int)
                        as core::ffi::c_int;
                    update_cdf(
                        av1_get_pred_cdf_single_ref_p3(xd),
                        bit1_0 as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if bit1_0 == 0 {
                        update_cdf(
                            av1_get_pred_cdf_single_ref_p4(xd),
                            (ref0 as core::ffi::c_int != LAST_FRAME as core::ffi::c_int)
                                as core::ffi::c_int as int8_t,
                            2 as core::ffi::c_int,
                        );
                    } else {
                        update_cdf(
                            av1_get_pred_cdf_single_ref_p5(xd),
                            (ref0 as core::ffi::c_int != LAST3_FRAME as core::ffi::c_int)
                                as core::ffi::c_int as int8_t,
                            2 as core::ffi::c_int,
                        );
                    }
                }
            }
            if (*(*cm).seq_params).enable_interintra_compound as core::ffi::c_int != 0
                && is_interintra_allowed(mbmi) != 0
            {
                let bsize_group: core::ffi::c_int = size_group_lookup[bsize as usize]
                    as core::ffi::c_int;
                if (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    == INTRA_FRAME as core::ffi::c_int
                {
                    update_cdf(
                        ((*fc).interintra_cdf[bsize_group as usize]).as_mut_ptr(),
                        1 as int8_t,
                        2 as core::ffi::c_int,
                    );
                    update_cdf(
                        ((*fc).interintra_mode_cdf[bsize_group as usize]).as_mut_ptr(),
                        (*mbmi).interintra_mode as int8_t,
                        INTERINTRA_MODES as core::ffi::c_int,
                    );
                    if av1_is_wedge_used(bsize) != 0 {
                        update_cdf(
                            ((*fc).wedge_interintra_cdf[bsize as usize]).as_mut_ptr(),
                            (*mbmi).use_wedge_interintra() as int8_t,
                            2 as core::ffi::c_int,
                        );
                        if (*mbmi).use_wedge_interintra() != 0 {
                            update_cdf(
                                ((*fc).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                                (*mbmi).interintra_wedge_index,
                                16 as core::ffi::c_int,
                            );
                        }
                    }
                } else {
                    update_cdf(
                        ((*fc).interintra_cdf[bsize_group as usize]).as_mut_ptr(),
                        0 as int8_t,
                        2 as core::ffi::c_int,
                    );
                }
            }
            let motion_allowed: MOTION_MODE = (if (*cm).features.switchable_motion_mode
                as core::ffi::c_int != 0
            {
                motion_mode_allowed(
                    (*xd).global_motion,
                    xd,
                    mbmi,
                    (*cm).features.allow_warped_motion as core::ffi::c_int,
                ) as core::ffi::c_int
            } else {
                SIMPLE_TRANSLATION as core::ffi::c_int
            }) as MOTION_MODE;
            if (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                != INTRA_FRAME as core::ffi::c_int
            {
                if motion_allowed as core::ffi::c_int
                    == WARPED_CAUSAL as core::ffi::c_int
                {
                    update_cdf(
                        ((*fc).motion_mode_cdf[bsize as usize]).as_mut_ptr(),
                        (*mbmi).motion_mode as int8_t,
                        MOTION_MODES as core::ffi::c_int,
                    );
                } else if motion_allowed as core::ffi::c_int
                    == OBMC_CAUSAL as core::ffi::c_int
                {
                    update_cdf(
                        ((*fc).obmc_cdf[bsize as usize]).as_mut_ptr(),
                        ((*mbmi).motion_mode as core::ffi::c_int
                            == OBMC_CAUSAL as core::ffi::c_int) as core::ffi::c_int
                            as int8_t,
                        2 as core::ffi::c_int,
                    );
                }
            }
            if has_second_ref(mbmi) != 0 {
                if !((*current_frame).reference_mode as core::ffi::c_int
                    != SINGLE_REFERENCE as core::ffi::c_int
                    && is_inter_compound_mode((*mbmi).mode) != 0
                    && (*mbmi).motion_mode as core::ffi::c_int
                        == SIMPLE_TRANSLATION as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 13],
                            [core::ffi::c_char; 13],
                        >(*b"update_stats\0"))
                            .as_ptr(),
                        b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                        1277 as core::ffi::c_int,
                        b"current_frame->reference_mode != SINGLE_REFERENCE && is_inter_compound_mode(mbmi->mode) && mbmi->motion_mode == SIMPLE_TRANSLATION\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let masked_compound_used: core::ffi::c_int = (is_any_masked_compound_used(
                    bsize,
                ) != 0
                    && (*(*cm).seq_params).enable_masked_compound as core::ffi::c_int
                        != 0) as core::ffi::c_int;
                if masked_compound_used != 0 {
                    let comp_group_idx_ctx: core::ffi::c_int = get_comp_group_idx_context(
                        xd,
                    ) as core::ffi::c_int;
                    update_cdf(
                        ((*fc).comp_group_idx_cdf[comp_group_idx_ctx as usize])
                            .as_mut_ptr(),
                        (*mbmi).comp_group_idx() as int8_t,
                        2 as core::ffi::c_int,
                    );
                }
                if (*mbmi).comp_group_idx() as core::ffi::c_int == 0 as core::ffi::c_int
                {
                    let comp_index_ctx: core::ffi::c_int = get_comp_index_context(cm, xd)
                        as core::ffi::c_int;
                    update_cdf(
                        ((*fc).compound_index_cdf[comp_index_ctx as usize]).as_mut_ptr(),
                        (*mbmi).compound_idx() as int8_t,
                        2 as core::ffi::c_int,
                    );
                } else {
                    if (masked_compound_used == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 13],
                                [core::ffi::c_char; 13],
                            >(*b"update_stats\0"))
                                .as_ptr(),
                            b"partition_search.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1298 as core::ffi::c_int,
                            b"masked_compound_used\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    if is_interinter_compound_used(
                        COMPOUND_WEDGE as core::ffi::c_int as COMPOUND_TYPE,
                        bsize,
                    ) != 0
                    {
                        update_cdf(
                            ((*fc).compound_type_cdf[bsize as usize]).as_mut_ptr(),
                            ((*mbmi).interinter_comp.type_0 as core::ffi::c_int
                                - COMPOUND_WEDGE as core::ffi::c_int) as int8_t,
                            MASKED_COMPOUND_TYPES as core::ffi::c_int,
                        );
                    }
                }
            }
            if (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                == COMPOUND_WEDGE as core::ffi::c_int
            {
                if is_interinter_compound_used(
                    COMPOUND_WEDGE as core::ffi::c_int as COMPOUND_TYPE,
                    bsize,
                ) != 0
                {
                    update_cdf(
                        ((*fc).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                        (*mbmi).interinter_comp.wedge_index,
                        16 as core::ffi::c_int,
                    );
                }
            }
        }
    }
    if inter_block != 0
        && (*cm).features.interp_filter as core::ffi::c_int
            == SWITCHABLE as core::ffi::c_int && av1_is_interp_needed(xd) != 0
    {
        update_filter_type_cdf(
            xd,
            mbmi,
            (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int,
        );
    }
    if inter_block != 0
        && segfeature_active(
            &(*cm).seg,
            (*mbmi).segment_id(),
            SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
        ) == 0
    {
        let mode: PREDICTION_MODE = (*mbmi).mode;
        let mode_ctx: int16_t = av1_mode_context_analyzer(
            ((*mbmi_ext).mode_context).as_ptr(),
            ((*mbmi).ref_frame).as_ptr(),
        ) as int16_t;
        if has_second_ref(mbmi) != 0 {
            update_cdf(
                ((*fc).inter_compound_mode_cdf[mode_ctx as usize]).as_mut_ptr(),
                (mode as core::ffi::c_int - NEAREST_NEARESTMV as core::ffi::c_int)
                    as uint8_t as int8_t,
                1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
                    - NEAREST_NEARESTMV as core::ffi::c_int,
            );
        } else {
            av1_update_inter_mode_stats(fc, counts, mode, mode_ctx);
        }
        let new_mv: core::ffi::c_int = ((*mbmi).mode as core::ffi::c_int
            == NEWMV as core::ffi::c_int
            || (*mbmi).mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int)
            as core::ffi::c_int;
        if new_mv != 0 {
            let ref_frame_type_0: uint8_t = av1_ref_frame_type(
                ((*mbmi).ref_frame).as_ptr(),
            ) as uint8_t;
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < 2 as core::ffi::c_int {
                if (*mbmi_ext).ref_mv_count[ref_frame_type_0 as usize]
                    as core::ffi::c_int > idx + 1 as core::ffi::c_int
                {
                    let drl_ctx: uint8_t = av1_drl_ctx(
                        ((*mbmi_ext).weight[ref_frame_type_0 as usize]).as_ptr(),
                        idx,
                    ) as uint8_t;
                    update_cdf(
                        ((*fc).drl_cdf[drl_ctx as usize]).as_mut_ptr(),
                        ((*mbmi).ref_mv_idx() as core::ffi::c_int != idx)
                            as core::ffi::c_int as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if (*mbmi).ref_mv_idx() as core::ffi::c_int == idx {
                        break;
                    }
                }
                idx += 1;
            }
        }
        if have_nearmv_in_inter_mode((*mbmi).mode) != 0 {
            let ref_frame_type_1: uint8_t = av1_ref_frame_type(
                ((*mbmi).ref_frame).as_ptr(),
            ) as uint8_t;
            let mut idx_0: core::ffi::c_int = 1 as core::ffi::c_int;
            while idx_0 < 3 as core::ffi::c_int {
                if (*mbmi_ext).ref_mv_count[ref_frame_type_1 as usize]
                    as core::ffi::c_int > idx_0 + 1 as core::ffi::c_int
                {
                    let drl_ctx_0: uint8_t = av1_drl_ctx(
                        ((*mbmi_ext).weight[ref_frame_type_1 as usize]).as_ptr(),
                        idx_0,
                    ) as uint8_t;
                    update_cdf(
                        ((*fc).drl_cdf[drl_ctx_0 as usize]).as_mut_ptr(),
                        ((*mbmi).ref_mv_idx() as core::ffi::c_int
                            != idx_0 - 1 as core::ffi::c_int) as core::ffi::c_int
                            as int8_t,
                        2 as core::ffi::c_int,
                    );
                    if (*mbmi).ref_mv_idx() as core::ffi::c_int
                        == idx_0 - 1 as core::ffi::c_int
                    {
                        break;
                    }
                }
                idx_0 += 1;
            }
        }
        if have_newmv_in_inter_mode((*mbmi).mode) != 0 {
            let allow_hp: core::ffi::c_int = if (*cm).features.cur_frame_force_integer_mv
                as core::ffi::c_int != 0
            {
                MV_SUBPEL_NONE as core::ffi::c_int as core::ffi::c_int
            } else {
                (*cm).features.allow_high_precision_mv as core::ffi::c_int
            };
            if new_mv != 0 {
                let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while ref_0 < 1 as core::ffi::c_int + has_second_ref(mbmi) {
                    let ref_mv: int_mv = av1_get_ref_mv(x, ref_0) as int_mv;
                    av1_update_mv_stats(
                        &(*((*mbmi).mv).as_ptr().offset(ref_0 as isize)).as_mv,
                        &ref_mv.as_mv,
                        &raw mut (*fc).nmvc,
                        allow_hp as MvSubpelPrecision,
                    );
                    ref_0 += 1;
                }
            } else if (*mbmi).mode as core::ffi::c_int
                == NEAREST_NEWMV as core::ffi::c_int
                || (*mbmi).mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
            {
                let ref_1: core::ffi::c_int = 1 as core::ffi::c_int;
                let ref_mv_0: int_mv = av1_get_ref_mv(x, ref_1) as int_mv;
                av1_update_mv_stats(
                    &(*((*mbmi).mv).as_ptr().offset(ref_1 as isize)).as_mv,
                    &ref_mv_0.as_mv,
                    &raw mut (*fc).nmvc,
                    allow_hp as MvSubpelPrecision,
                );
            } else if (*mbmi).mode as core::ffi::c_int
                == NEW_NEARESTMV as core::ffi::c_int
                || (*mbmi).mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int
            {
                let ref_2: core::ffi::c_int = 0 as core::ffi::c_int;
                let ref_mv_1: int_mv = av1_get_ref_mv(x, ref_2) as int_mv;
                av1_update_mv_stats(
                    &(*((*mbmi).mv).as_ptr().offset(ref_2 as isize)).as_mv,
                    &ref_mv_1.as_mv,
                    &raw mut (*fc).nmvc,
                    allow_hp as MvSubpelPrecision,
                );
            }
        }
    }
}
unsafe extern "C-unwind" fn encode_b(
    cpi: *const AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    mut td: *mut ThreadData,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut dry_run: RUN_TYPE,
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
    ctx: *mut PICK_MODE_CONTEXT,
    mut rate: *mut core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let tile: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let subsampling_x: core::ffi::c_int = (*(*cm).seq_params).subsampling_x;
    let subsampling_y: core::ffi::c_int = (*(*cm).seq_params).subsampling_y;
    av1_set_offsets_without_segment_id(cpi, tile, x, mi_row, mi_col, bsize);
    let origin_mult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(
        cpi,
        x,
        mi_row,
        mi_col,
        bsize,
        NO_AQ as core::ffi::c_int as AQ_MODE,
        0 as *mut MB_MODE_INFO,
    );
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).partition = partition;
    av1_update_state(cpi, td, ctx, mi_row, mi_col, bsize, dry_run);
    if dry_run == 0 {
        set_cb_offsets(
            ((*(*x).mbmi_ext_frame).cb_offset).as_mut_ptr(),
            (*x).cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize],
            (*x).cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize],
        );
        if !(((*x).cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize]
            as core::ffi::c_int)
            < (1 as core::ffi::c_int)
                << num_pels_log2_lookup[(*(*cpi).common.seq_params).sb_size as usize]
                    as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 9],
                    [core::ffi::c_char; 9],
                >(*b"encode_b\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                1445 as core::ffi::c_int,
                b"x->cb_offset[PLANE_TYPE_Y] < (1 << num_pels_log2_lookup[cpi->common.seq_params->sb_size])\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(((*x).cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize]
            as core::ffi::c_int)
            < (1 as core::ffi::c_int)
                << num_pels_log2_lookup[(*(*cpi).common.seq_params).sb_size as usize]
                    as core::ffi::c_int >> subsampling_x + subsampling_y)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 9],
                    [core::ffi::c_char; 9],
                >(*b"encode_b\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                1448 as core::ffi::c_int,
                b"x->cb_offset[PLANE_TYPE_UV] < ((1 << num_pels_log2_lookup[cpi->common.seq_params->sb_size]) >> (subsampling_x + subsampling_y))\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    encode_superblock(cpi, tile_data, td, tp, dry_run, bsize, rate);
    if dry_run == 0 {
        update_cb_offsets(x, bsize, subsampling_x, subsampling_y);
        if bsize as core::ffi::c_int
            == (*(*cpi).common.seq_params).sb_size as core::ffi::c_int
            && (*mbmi).skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int
            && (*cm).delta_q_info.delta_lf_present_flag != 0
        {
            let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm)
                > 1 as core::ffi::c_int
            {
                FRAME_LF_COUNT
            } else {
                FRAME_LF_COUNT - 2 as core::ffi::c_int
            };
            let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
            while lf_id < frame_lf_count {
                (*mbmi).delta_lf[lf_id as usize] = (*xd).delta_lf[lf_id as usize];
                lf_id += 1;
            }
            (*mbmi).delta_lf_from_base = (*xd).delta_lf_from_base;
        }
        if has_second_ref(mbmi) != 0 {
            if (*mbmi).compound_idx() as core::ffi::c_int == 0 as core::ffi::c_int
                || (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                    == COMPOUND_AVERAGE as core::ffi::c_int
            {
                (*mbmi).set_comp_group_idx(0 as uint8_t as uint8_t);
            } else {
                (*mbmi).set_comp_group_idx(1 as uint8_t as uint8_t);
            }
        }
        let super_block_upper_left: core::ffi::c_int = (mi_row
            & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int
            == 0 as core::ffi::c_int
            && mi_col & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int
                == 0 as core::ffi::c_int) as core::ffi::c_int;
        let delta_q_info: *const DeltaQInfo = &(*cm).delta_q_info;
        if (*delta_q_info).delta_q_present_flag != 0
            && (bsize as core::ffi::c_int
                != (*(*cm).seq_params).sb_size as core::ffi::c_int
                || (*mbmi).skip_txfm == 0) && super_block_upper_left != 0
        {
            (*xd).current_base_qindex = (*mbmi).current_qindex;
            if (*delta_q_info).delta_lf_present_flag != 0 {
                if (*delta_q_info).delta_lf_multi != 0 {
                    let frame_lf_count_0: core::ffi::c_int = if av1_num_planes(cm)
                        > 1 as core::ffi::c_int
                    {
                        FRAME_LF_COUNT
                    } else {
                        FRAME_LF_COUNT - 2 as core::ffi::c_int
                    };
                    let mut lf_id_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while lf_id_0 < frame_lf_count_0 {
                        (*xd).delta_lf[lf_id_0 as usize] = (*mbmi)
                            .delta_lf[lf_id_0 as usize];
                        lf_id_0 += 1;
                    }
                } else {
                    (*xd).delta_lf_from_base = (*mbmi).delta_lf_from_base;
                }
            }
        }
        let mut rdc: *mut RD_COUNTS = &raw mut (*td).rd_counts;
        if (*mbmi).skip_mode() != 0 {
            if (frame_is_intra_only(cm) != 0) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 9],
                        [core::ffi::c_char; 9],
                    >(*b"encode_b\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1495 as core::ffi::c_int,
                    b"!frame_is_intra_only(cm)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            (*rdc).skip_mode_used_flag = 1 as core::ffi::c_int;
            if (*cm).current_frame.reference_mode as core::ffi::c_int
                == REFERENCE_MODE_SELECT as core::ffi::c_int
            {
                if (has_second_ref(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"encode_b\0"))
                            .as_ptr(),
                        b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                        1498 as core::ffi::c_int,
                        b"has_second_ref(mbmi)\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*rdc).compound_ref_used_flag = 1 as core::ffi::c_int;
            }
            set_ref_ptrs(
                cm,
                xd,
                (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
                (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
            );
        } else {
            let seg_ref_active: core::ffi::c_int = segfeature_active(
                &(*cm).seg,
                (*mbmi).segment_id(),
                SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
            ) as core::ffi::c_int;
            if seg_ref_active == 0 {
                if is_inter_block(mbmi) != 0 {
                    av1_collect_neighbors_ref_counts(xd);
                    if (*cm).current_frame.reference_mode as core::ffi::c_int
                        == REFERENCE_MODE_SELECT as core::ffi::c_int
                    {
                        if has_second_ref(mbmi) != 0 {
                            (*rdc).compound_ref_used_flag = 1 as core::ffi::c_int;
                        }
                    }
                    set_ref_ptrs(
                        cm,
                        xd,
                        (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
                        (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
                    );
                }
            }
        }
        if (*tile_data).allow_update_cdf != 0 {
            update_stats(&(*cpi).common, td);
        }
        if (*cpi).sf.inter_sf.prune_obmc_prob_thresh > 0 as core::ffi::c_int
            && (*cpi).sf.inter_sf.prune_obmc_prob_thresh < INT_MAX
            || (*cm).features.allow_warped_motion as core::ffi::c_int != 0
                && (*cpi).sf.inter_sf.prune_warped_prob_thresh > 0 as core::ffi::c_int
        {
            let inter_block: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
            let seg_ref_active_0: core::ffi::c_int = segfeature_active(
                &(*cm).seg,
                (*mbmi).segment_id(),
                SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
            ) as core::ffi::c_int;
            if seg_ref_active_0 == 0 && inter_block != 0 {
                let motion_allowed: MOTION_MODE = (if (*cm)
                    .features
                    .switchable_motion_mode as core::ffi::c_int != 0
                {
                    motion_mode_allowed(
                        (*xd).global_motion,
                        xd,
                        mbmi,
                        (*cm).features.allow_warped_motion as core::ffi::c_int,
                    ) as core::ffi::c_int
                } else {
                    SIMPLE_TRANSLATION as core::ffi::c_int
                }) as MOTION_MODE;
                if (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    != INTRA_FRAME as core::ffi::c_int
                {
                    if motion_allowed as core::ffi::c_int
                        >= OBMC_CAUSAL as core::ffi::c_int
                    {
                        (*td)
                            .rd_counts
                            .obmc_used[bsize
                            as usize][((*mbmi).motion_mode as core::ffi::c_int
                            == OBMC_CAUSAL as core::ffi::c_int) as core::ffi::c_int
                            as usize] += 1;
                    }
                    if motion_allowed as core::ffi::c_int
                        == WARPED_CAUSAL as core::ffi::c_int
                    {
                        (*td)
                            .rd_counts
                            .warped_used[((*mbmi).motion_mode as core::ffi::c_int
                            == WARPED_CAUSAL as core::ffi::c_int) as core::ffi::c_int
                            as usize] += 1;
                    }
                }
            }
        }
    }
    av1_copy_mbmi_ext_to_mbmi_ext_frame(
        (*x).mbmi_ext_frame,
        &raw mut (*x).mbmi_ext,
        av1_ref_frame_type(
            ((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).ref_frame)
                .as_mut_ptr(),
        ) as uint8_t,
    );
    (*x).rdmult = origin_mult;
}
unsafe extern "C-unwind" fn encode_sb(
    cpi: *const AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut dry_run: RUN_TYPE,
    mut bsize: BLOCK_SIZE,
    mut pc_tree: *mut PC_TREE,
    mut rate: *mut core::ffi::c_int,
) {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"encode_sb\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            1588 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *const AV1_COMMON = &(*cpi).common;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"encode_sb\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            1593 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let hbs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
        / 2 as core::ffi::c_int;
    let is_partition_root: core::ffi::c_int = (bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int) as core::ffi::c_int;
    let ctx: core::ffi::c_int = if is_partition_root != 0 {
        partition_plane_context(xd, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    let partition: PARTITION_TYPE = (*pc_tree).partitioning;
    let subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition) as BLOCK_SIZE;
    let mut quarter_step: core::ffi::c_int = mi_size_wide[bsize as usize]
        as core::ffi::c_int / 4 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut bsize2: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    );
    if mi_row >= (*mi_params).mi_rows || mi_col >= (*mi_params).mi_cols {
        return;
    }
    if subsize as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int {
        return;
    }
    if dry_run == 0 && ctx >= 0 as core::ffi::c_int {
        let has_rows: core::ffi::c_int = (mi_row + hbs < (*mi_params).mi_rows)
            as core::ffi::c_int;
        let has_cols: core::ffi::c_int = (mi_col + hbs < (*mi_params).mi_cols)
            as core::ffi::c_int;
        if has_rows != 0 && has_cols != 0 {
            if (*tile_data).allow_update_cdf != 0 {
                let mut fc: *mut FRAME_CONTEXT = (*xd).tile_ctx;
                update_cdf(
                    ((*fc).partition_cdf[ctx as usize]).as_mut_ptr(),
                    partition as int8_t,
                    partition_cdf_length(bsize),
                );
            }
        }
    }
    match partition as core::ffi::c_int {
        0 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).none,
                rate,
            );
        }
        2 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).vertical[0 as core::ffi::c_int as usize],
                rate,
            );
            if mi_col + hbs < (*mi_params).mi_cols {
                encode_b(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row,
                    mi_col + hbs,
                    dry_run,
                    subsize,
                    partition,
                    (*pc_tree).vertical[1 as core::ffi::c_int as usize],
                    rate,
                );
            }
        }
        1 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).horizontal[0 as core::ffi::c_int as usize],
                rate,
            );
            if mi_row + hbs < (*mi_params).mi_rows {
                encode_b(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row + hbs,
                    mi_col,
                    dry_run,
                    subsize,
                    partition,
                    (*pc_tree).horizontal[1 as core::ffi::c_int as usize],
                    rate,
                );
            }
        }
        3 => {
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                (*pc_tree).split[0 as core::ffi::c_int as usize],
                rate,
            );
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col + hbs,
                dry_run,
                subsize,
                (*pc_tree).split[1 as core::ffi::c_int as usize],
                rate,
            );
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row + hbs,
                mi_col,
                dry_run,
                subsize,
                (*pc_tree).split[2 as core::ffi::c_int as usize],
                rate,
            );
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row + hbs,
                mi_col + hbs,
                dry_run,
                subsize,
                (*pc_tree).split[3 as core::ffi::c_int as usize],
                rate,
            );
        }
        4 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).horizontala[0 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col + hbs,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).horizontala[1 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row + hbs,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).horizontala[2 as core::ffi::c_int as usize],
                rate,
            );
        }
        5 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).horizontalb[0 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row + hbs,
                mi_col,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).horizontalb[1 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row + hbs,
                mi_col + hbs,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).horizontalb[2 as core::ffi::c_int as usize],
                rate,
            );
        }
        6 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).verticala[0 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row + hbs,
                mi_col,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).verticala[1 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col + hbs,
                dry_run,
                subsize,
                partition,
                (*pc_tree).verticala[2 as core::ffi::c_int as usize],
                rate,
            );
        }
        7 => {
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                dry_run,
                subsize,
                partition,
                (*pc_tree).verticalb[0 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col + hbs,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).verticalb[1 as core::ffi::c_int as usize],
                rate,
            );
            encode_b(
                cpi,
                tile_data,
                td,
                tp,
                mi_row + hbs,
                mi_col + hbs,
                dry_run,
                bsize2,
                partition,
                (*pc_tree).verticalb[2 as core::ffi::c_int as usize],
                rate,
            );
        }
        8 => {
            i = 0 as core::ffi::c_int;
            while i < SUB_PARTITIONS_PART4 {
                let mut this_mi_row: core::ffi::c_int = mi_row + i * quarter_step;
                if i > 0 as core::ffi::c_int && this_mi_row >= (*mi_params).mi_rows {
                    break;
                }
                encode_b(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    this_mi_row,
                    mi_col,
                    dry_run,
                    subsize,
                    partition,
                    (*pc_tree).horizontal4[i as usize],
                    rate,
                );
                i += 1;
            }
        }
        9 => {
            i = 0 as core::ffi::c_int;
            while i < SUB_PARTITIONS_PART4 {
                let mut this_mi_col: core::ffi::c_int = mi_col + i * quarter_step;
                if i > 0 as core::ffi::c_int && this_mi_col >= (*mi_params).mi_cols {
                    break;
                }
                encode_b(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row,
                    this_mi_col,
                    dry_run,
                    subsize,
                    partition,
                    (*pc_tree).vertical4[i as usize],
                    rate,
                );
                i += 1;
            }
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid partition type.\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 10],
                        [core::ffi::c_char; 10],
                    >(*b"encode_sb\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1711 as core::ffi::c_int,
                    b"0 && \"Invalid partition type.\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    update_ext_partition_context(xd, mi_row, mi_col, subsize, bsize, partition);
}
#[inline]
unsafe extern "C-unwind" fn is_adjust_var_based_part_enabled(
    cm: *mut AV1_COMMON,
    part_sf: *const PARTITION_SPEED_FEATURES,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if (*part_sf).partition_search_type as core::ffi::c_int
        != VAR_BASED_PARTITION as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*part_sf).adjust_var_based_rd_partitioning == 0 as core::ffi::c_int
        || (*part_sf).adjust_var_based_rd_partitioning > 2 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int {
        return 1 as core::ffi::c_int;
    }
    if (*part_sf).adjust_var_based_rd_partitioning == 2 as core::ffi::c_int {
        let is_larger_qindex: core::ffi::c_int = ((*cm).quant_params.base_qindex
            > 190 as core::ffi::c_int) as core::ffi::c_int;
        let is_360p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
            (*cm).width
        } else {
            (*cm).height
        }) >= 360 as core::ffi::c_int) as core::ffi::c_int;
        return (is_360p_or_larger != 0 && is_larger_qindex != 0
            && bsize as core::ffi::c_int == BLOCK_64X64 as core::ffi::c_int)
            as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rd_use_partition(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mib: *mut *mut MB_MODE_INFO,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut rate: *mut core::ffi::c_int,
    mut dist: *mut int64_t,
    mut do_recon: core::ffi::c_int,
    mut pc_tree: *mut PC_TREE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let bs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bs / 2 as core::ffi::c_int;
    let pl: core::ffi::c_int = if bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int
    {
        partition_plane_context(xd, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let partition: PARTITION_TYPE = (if bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int
    {
        get_partition(cm, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        PARTITION_NONE as core::ffi::c_int
    }) as PARTITION_TYPE;
    let subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition) as BLOCK_SIZE;
    let mut x_ctx: RD_SEARCH_MACROBLOCK_CONTEXT = RD_SEARCH_MACROBLOCK_CONTEXT {
        a: [0; 96],
        l: [0; 96],
        sa: [0; 32],
        sl: [0; 32],
        p_ta: 0 as *mut TXFM_CONTEXT,
        p_tl: 0 as *mut TXFM_CONTEXT,
        ta: [0; 32],
        tl: [0; 32],
    };
    let mut last_part_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut none_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut chosen_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut invalid_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut bs_type: BLOCK_SIZE = (**mib.offset(0 as core::ffi::c_int as isize)).bsize;
    let mut use_partition_none: core::ffi::c_int = 0 as core::ffi::c_int;
    (*x).try_merge_partition = 0 as core::ffi::c_int;
    if ((*pc_tree).none).is_null() {
        (*pc_tree).none = av1_alloc_pmc(cpi, bsize, &raw mut (*td).shared_coeff_buf);
        if ((*pc_tree).none).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    let mut ctx_none: *mut PICK_MODE_CONTEXT = (*pc_tree).none;
    if mi_row >= (*mi_params).mi_rows || mi_col >= (*mi_params).mi_cols {
        return;
    }
    if !(mi_size_wide[bsize as usize] as core::ffi::c_int
        == mi_size_high[bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_rd_use_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            1803 as core::ffi::c_int,
            b"mi_size_wide[bsize] == mi_size_high[bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(bsize as core::ffi::c_int
        >= (*cpi).sf.part_sf.default_min_partition_size as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_rd_use_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            1805 as core::ffi::c_int,
            b"bsize >= cpi->sf.part_sf.default_min_partition_size\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    av1_invalid_rd_stats(&raw mut last_part_rdc);
    av1_invalid_rd_stats(&raw mut none_rdc);
    av1_invalid_rd_stats(&raw mut chosen_rdc);
    av1_invalid_rd_stats(&raw mut invalid_rdc);
    (*pc_tree).partitioning = partition;
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    av1_save_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
    if bsize as core::ffi::c_int == BLOCK_16X16 as core::ffi::c_int
        && (*cpi).vaq_refresh != 0
    {
        av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, bsize);
        (*x).mb_energy = av1_log_block_var(cpi, x, bsize);
    }
    let orig_rdmult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(
        cpi,
        x,
        mi_row,
        mi_col,
        bsize,
        NO_AQ as core::ffi::c_int as AQ_MODE,
        0 as *mut MB_MODE_INFO,
    );
    if partition as core::ffi::c_int != PARTITION_NONE as core::ffi::c_int
        && is_adjust_var_based_part_enabled(cm, &raw mut (*cpi).sf.part_sf, bsize) != 0
        && (mi_row + hbs < (*mi_params).mi_rows && mi_col + hbs < (*mi_params).mi_cols)
    {
        if !(bsize as core::ffi::c_int
            > (*cpi).sf.part_sf.default_min_partition_size as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"av1_rd_use_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                1833 as core::ffi::c_int,
                b"bsize > cpi->sf.part_sf.default_min_partition_size\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = bsize;
        (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
        (*x).try_merge_partition = 1 as core::ffi::c_int;
        pick_sb_modes(
            cpi,
            tile_data,
            x,
            mi_row,
            mi_col,
            &raw mut none_rdc,
            PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
            bsize,
            ctx_none,
            invalid_rdc,
        );
        if none_rdc.rate < INT_MAX {
            none_rdc.rate
                += (*mode_costs)
                    .partition_cost[pl
                    as usize][PARTITION_NONE as core::ffi::c_int as usize];
            none_rdc.rdcost = (none_rdc.rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + none_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        }
        if none_rdc.rate < INT_MAX
            && none_rdc.skip_txfm as core::ffi::c_int == 1 as core::ffi::c_int
        {
            use_partition_none = 1 as core::ffi::c_int;
        }
        av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = bs_type;
        (*pc_tree).partitioning = partition;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SUB_PARTITIONS_SPLIT {
        (*pc_tree).split[i as usize] = av1_alloc_pc_tree_node(subsize) as *mut PC_TREE;
        if ((*pc_tree).split[i as usize]).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        (*(*pc_tree).split[i as usize]).index = i;
        i += 1;
    }
    let mut current_block_88: u64;
    match partition as core::ffi::c_int {
        0 => {
            pick_sb_modes(
                cpi,
                tile_data,
                x,
                mi_row,
                mi_col,
                &raw mut last_part_rdc,
                PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
                bsize,
                ctx_none,
                invalid_rdc,
            );
            current_block_88 = 2;
        }
        1 => {
            if use_partition_none != 0 {
                av1_invalid_rd_stats(&raw mut last_part_rdc);
            } else {
                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_0 < SUB_PARTITIONS_RECT {
                    (*pc_tree).horizontal[i_0 as usize] = av1_alloc_pmc(
                        cpi,
                        subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if ((*pc_tree).horizontal[i_0 as usize]).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    i_0 += 1;
                }
                pick_sb_modes(
                    cpi,
                    tile_data,
                    x,
                    mi_row,
                    mi_col,
                    &raw mut last_part_rdc,
                    PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
                    subsize,
                    (*pc_tree).horizontal[0 as core::ffi::c_int as usize],
                    invalid_rdc,
                );
                if last_part_rdc.rate != INT_MAX
                    && bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
                    && mi_row + hbs < (*mi_params).mi_rows
                {
                    let mut tmp_rdc: RD_STATS = RD_STATS {
                        rate: 0,
                        zero_rate: 0,
                        dist: 0,
                        rdcost: 0,
                        sse: 0,
                        skip_txfm: 0,
                    };
                    let ctx_h: *const PICK_MODE_CONTEXT = (*pc_tree)
                        .horizontal[0 as core::ffi::c_int as usize];
                    av1_init_rd_stats(&raw mut tmp_rdc);
                    av1_update_state(
                        cpi,
                        td,
                        ctx_h,
                        mi_row,
                        mi_col,
                        subsize,
                        1 as RUN_TYPE,
                    );
                    encode_superblock(
                        cpi,
                        tile_data,
                        td,
                        tp,
                        DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                        subsize,
                        0 as *mut core::ffi::c_int,
                    );
                    pick_sb_modes(
                        cpi,
                        tile_data,
                        x,
                        mi_row + hbs,
                        mi_col,
                        &raw mut tmp_rdc,
                        PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
                        subsize,
                        (*pc_tree).horizontal[1 as core::ffi::c_int as usize],
                        invalid_rdc,
                    );
                    if tmp_rdc.rate == INT_MAX || tmp_rdc.dist == INT64_MAX as int64_t {
                        av1_invalid_rd_stats(&raw mut last_part_rdc);
                    } else {
                        last_part_rdc.rate += tmp_rdc.rate;
                        last_part_rdc.dist += tmp_rdc.dist;
                        last_part_rdc.rdcost += tmp_rdc.rdcost;
                    }
                }
            }
            current_block_88 = 2;
        }
        2 => {
            if use_partition_none != 0 {
                av1_invalid_rd_stats(&raw mut last_part_rdc);
            } else {
                let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_1 < SUB_PARTITIONS_RECT {
                    (*pc_tree).vertical[i_1 as usize] = av1_alloc_pmc(
                        cpi,
                        subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if ((*pc_tree).vertical[i_1 as usize]).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    i_1 += 1;
                }
                pick_sb_modes(
                    cpi,
                    tile_data,
                    x,
                    mi_row,
                    mi_col,
                    &raw mut last_part_rdc,
                    PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
                    subsize,
                    (*pc_tree).vertical[0 as core::ffi::c_int as usize],
                    invalid_rdc,
                );
                if last_part_rdc.rate != INT_MAX
                    && bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
                    && mi_col + hbs < (*mi_params).mi_cols
                {
                    let mut tmp_rdc_0: RD_STATS = RD_STATS {
                        rate: 0,
                        zero_rate: 0,
                        dist: 0,
                        rdcost: 0,
                        sse: 0,
                        skip_txfm: 0,
                    };
                    let ctx_v: *const PICK_MODE_CONTEXT = (*pc_tree)
                        .vertical[0 as core::ffi::c_int as usize];
                    av1_init_rd_stats(&raw mut tmp_rdc_0);
                    av1_update_state(
                        cpi,
                        td,
                        ctx_v,
                        mi_row,
                        mi_col,
                        subsize,
                        1 as RUN_TYPE,
                    );
                    encode_superblock(
                        cpi,
                        tile_data,
                        td,
                        tp,
                        DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                        subsize,
                        0 as *mut core::ffi::c_int,
                    );
                    pick_sb_modes(
                        cpi,
                        tile_data,
                        x,
                        mi_row,
                        mi_col + hbs,
                        &raw mut tmp_rdc_0,
                        PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
                        subsize,
                        (*pc_tree)
                            .vertical[(bsize as core::ffi::c_int
                            > BLOCK_8X8 as core::ffi::c_int) as core::ffi::c_int
                            as usize],
                        invalid_rdc,
                    );
                    if tmp_rdc_0.rate == INT_MAX
                        || tmp_rdc_0.dist == INT64_MAX as int64_t
                    {
                        av1_invalid_rd_stats(&raw mut last_part_rdc);
                    } else {
                        last_part_rdc.rate += tmp_rdc_0.rate;
                        last_part_rdc.dist += tmp_rdc_0.dist;
                        last_part_rdc.rdcost += tmp_rdc_0.rdcost;
                    }
                }
            }
            current_block_88 = 2;
        }
        3 => {
            if use_partition_none != 0 {
                av1_invalid_rd_stats(&raw mut last_part_rdc);
            } else {
                last_part_rdc.rate = 0 as core::ffi::c_int;
                last_part_rdc.dist = 0 as int64_t;
                last_part_rdc.rdcost = 0 as int64_t;
                let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_2 < SUB_PARTITIONS_SPLIT {
                    let mut x_idx: core::ffi::c_int = (i_2 & 1 as core::ffi::c_int)
                        * hbs;
                    let mut y_idx: core::ffi::c_int = (i_2 >> 1 as core::ffi::c_int)
                        * hbs;
                    let mut jj: core::ffi::c_int = i_2 >> 1 as core::ffi::c_int;
                    let mut ii: core::ffi::c_int = i_2 & 0x1 as core::ffi::c_int;
                    let mut tmp_rdc_1: RD_STATS = RD_STATS {
                        rate: 0,
                        zero_rate: 0,
                        dist: 0,
                        rdcost: 0,
                        sse: 0,
                        skip_txfm: 0,
                    };
                    if !(mi_row + y_idx >= (*mi_params).mi_rows
                        || mi_col + x_idx >= (*mi_params).mi_cols)
                    {
                        av1_init_rd_stats(&raw mut tmp_rdc_1);
                        av1_rd_use_partition(
                            cpi,
                            td,
                            tile_data,
                            mib
                                .offset((jj * hbs * (*mi_params).mi_stride) as isize)
                                .offset((ii * hbs) as isize),
                            tp,
                            mi_row + y_idx,
                            mi_col + x_idx,
                            subsize,
                            &raw mut tmp_rdc_1.rate,
                            &raw mut tmp_rdc_1.dist,
                            (i_2 != SUB_PARTITIONS_SPLIT - 1 as core::ffi::c_int)
                                as core::ffi::c_int,
                            (*pc_tree).split[i_2 as usize],
                        );
                        if tmp_rdc_1.rate == INT_MAX
                            || tmp_rdc_1.dist == INT64_MAX as int64_t
                        {
                            av1_invalid_rd_stats(&raw mut last_part_rdc);
                            break;
                        } else {
                            last_part_rdc.rate += tmp_rdc_1.rate;
                            last_part_rdc.dist += tmp_rdc_1.dist;
                        }
                    }
                    i_2 += 1;
                }
            }
            current_block_88 = 2;
        }
        6 | 7 | 4 | 5 | 8 | 9 => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Cannot handle extended partition types\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"av1_rd_use_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1977 as core::ffi::c_int,
                    b"0 && \"Cannot handle extended partition types\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            current_block_88 = 3;
        }
        _ => {
            current_block_88 = 3;
        }
    }
    match current_block_88 {
        3 => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"av1_rd_use_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    1978 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        _ => {}
    }
    if last_part_rdc.rate < INT_MAX {
        last_part_rdc.rate
            += (*mode_costs).partition_cost[pl as usize][partition as usize];
        last_part_rdc.rdcost = (last_part_rdc.rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + last_part_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    }
    if (*cpi).sf.part_sf.partition_search_type as core::ffi::c_int
        == VAR_BASED_PARTITION as core::ffi::c_int
        && (*cpi).sf.part_sf.adjust_var_based_rd_partitioning > 2 as core::ffi::c_int
        && partition as core::ffi::c_int != PARTITION_SPLIT as core::ffi::c_int
        && bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int
        && (mi_row + bs < (*mi_params).mi_rows || mi_row + hbs == (*mi_params).mi_rows)
        && (mi_col + bs < (*mi_params).mi_cols || mi_col + hbs == (*mi_params).mi_cols)
    {
        let mut split_subsize: BLOCK_SIZE = get_partition_subsize(
            bsize,
            PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
        );
        chosen_rdc.rate = 0 as core::ffi::c_int;
        chosen_rdc.dist = 0 as int64_t;
        av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
        (*pc_tree).partitioning = PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 < SUB_PARTITIONS_SPLIT {
            let mut x_idx_0: core::ffi::c_int = (i_3 & 1 as core::ffi::c_int) * hbs;
            let mut y_idx_0: core::ffi::c_int = (i_3 >> 1 as core::ffi::c_int) * hbs;
            let mut tmp_rdc_2: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            if !(mi_row + y_idx_0 >= (*mi_params).mi_rows
                || mi_col + x_idx_0 >= (*mi_params).mi_cols)
            {
                av1_save_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
                (*(*pc_tree).split[i_3 as usize]).partitioning = PARTITION_NONE
                    as core::ffi::c_int as PARTITION_TYPE;
                if ((*(*pc_tree).split[i_3 as usize]).none).is_null() {
                    (*(*pc_tree).split[i_3 as usize]).none = av1_alloc_pmc(
                        cpi,
                        split_subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                }
                if ((*(*pc_tree).split[i_3 as usize]).none).is_null() {
                    aom_internal_error(
                        (*xd).error_info as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                pick_sb_modes(
                    cpi,
                    tile_data,
                    x,
                    mi_row + y_idx_0,
                    mi_col + x_idx_0,
                    &raw mut tmp_rdc_2,
                    PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
                    split_subsize,
                    (*(*pc_tree).split[i_3 as usize]).none,
                    invalid_rdc,
                );
                av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
                if tmp_rdc_2.rate == INT_MAX || tmp_rdc_2.dist == INT64_MAX as int64_t {
                    av1_invalid_rd_stats(&raw mut chosen_rdc);
                    break;
                } else {
                    chosen_rdc.rate += tmp_rdc_2.rate;
                    chosen_rdc.dist += tmp_rdc_2.dist;
                    if i_3 != SUB_PARTITIONS_SPLIT - 1 as core::ffi::c_int {
                        encode_sb(
                            cpi,
                            td,
                            tile_data,
                            tp,
                            mi_row + y_idx_0,
                            mi_col + x_idx_0,
                            OUTPUT_ENABLED as core::ffi::c_int as RUN_TYPE,
                            split_subsize,
                            (*pc_tree).split[i_3 as usize],
                            0 as *mut core::ffi::c_int,
                        );
                    }
                    chosen_rdc.rate
                        += (*mode_costs)
                            .partition_cost[pl
                            as usize][PARTITION_NONE as core::ffi::c_int as usize];
                }
            }
            i_3 += 1;
        }
        if chosen_rdc.rate < INT_MAX {
            chosen_rdc.rate
                += (*mode_costs)
                    .partition_cost[pl
                    as usize][PARTITION_SPLIT as core::ffi::c_int as usize];
            chosen_rdc.rdcost = (chosen_rdc.rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + chosen_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        }
    }
    if last_part_rdc.rdcost < chosen_rdc.rdcost {
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = bs_type;
        if bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int {
            (*pc_tree).partitioning = partition;
        }
        chosen_rdc = last_part_rdc;
    }
    if none_rdc.rdcost < INT64_MAX as int64_t
        && none_rdc.rdcost - (none_rdc.rdcost >> 9 as core::ffi::c_int)
            < chosen_rdc.rdcost
    {
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = bsize;
        if bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int {
            (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int
                as PARTITION_TYPE;
        }
        chosen_rdc = none_rdc;
    }
    av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
    if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
        if !(chosen_rdc.rate < 2147483647 as core::ffi::c_int
            && chosen_rdc.dist < 9223372036854775807 as int64_t) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"av1_rd_use_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                2064 as core::ffi::c_int,
                b"chosen_rdc.rate < INT_MAX && chosen_rdc.dist < INT64_MAX\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if do_recon != 0 {
        if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
            set_cb_offsets(((*x).cb_offset).as_mut_ptr(), 0 as uint16_t, 0 as uint16_t);
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                OUTPUT_ENABLED as core::ffi::c_int as RUN_TYPE,
                bsize,
                pc_tree,
                0 as *mut core::ffi::c_int,
            );
        } else {
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                bsize,
                pc_tree,
                0 as *mut core::ffi::c_int,
            );
        }
    }
    *rate = chosen_rdc.rate;
    *dist = chosen_rdc.dist;
    (*x).rdmult = orig_rdmult;
}
unsafe extern "C-unwind" fn encode_b_nonrd(
    cpi: *const AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    mut td: *mut ThreadData,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut dry_run: RUN_TYPE,
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
    ctx: *mut PICK_MODE_CONTEXT,
    mut rate: *mut core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let tile: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    av1_set_offsets_without_segment_id(cpi, tile, x, mi_row, mi_col, bsize);
    let origin_mult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(
        cpi,
        x,
        mi_row,
        mi_col,
        bsize,
        NO_AQ as core::ffi::c_int as AQ_MODE,
        0 as *mut MB_MODE_INFO,
    );
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).partition = partition;
    av1_update_state(cpi, td, ctx, mi_row, mi_col, bsize, dry_run);
    let subsampling_x: core::ffi::c_int = (*(*cpi).common.seq_params).subsampling_x;
    let subsampling_y: core::ffi::c_int = (*(*cpi).common.seq_params).subsampling_y;
    if dry_run == 0 {
        set_cb_offsets(
            ((*(*x).mbmi_ext_frame).cb_offset).as_mut_ptr(),
            (*x).cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize],
            (*x).cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize],
        );
        if !(((*x).cb_offset[PLANE_TYPE_Y as core::ffi::c_int as usize]
            as core::ffi::c_int)
            < (1 as core::ffi::c_int)
                << num_pels_log2_lookup[(*(*cpi).common.seq_params).sb_size as usize]
                    as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"encode_b_nonrd\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                2116 as core::ffi::c_int,
                b"x->cb_offset[PLANE_TYPE_Y] < (1 << num_pels_log2_lookup[cpi->common.seq_params->sb_size])\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(((*x).cb_offset[PLANE_TYPE_UV as core::ffi::c_int as usize]
            as core::ffi::c_int)
            < (1 as core::ffi::c_int)
                << num_pels_log2_lookup[(*(*cpi).common.seq_params).sb_size as usize]
                    as core::ffi::c_int >> subsampling_x + subsampling_y)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"encode_b_nonrd\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                2119 as core::ffi::c_int,
                b"x->cb_offset[PLANE_TYPE_UV] < ((1 << num_pels_log2_lookup[cpi->common.seq_params->sb_size]) >> (subsampling_x + subsampling_y))\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) == 0 {
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).skip_txfm = 0 as uint8_t;
    }
    encode_superblock(cpi, tile_data, td, tp, dry_run, bsize, rate);
    if dry_run == 0 {
        update_cb_offsets(x, bsize, subsampling_x, subsampling_y);
        if has_second_ref(mbmi) != 0 {
            if (*mbmi).compound_idx() as core::ffi::c_int == 0 as core::ffi::c_int
                || (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                    == COMPOUND_AVERAGE as core::ffi::c_int
            {
                (*mbmi).set_comp_group_idx(0 as uint8_t as uint8_t);
            } else {
                (*mbmi).set_comp_group_idx(1 as uint8_t as uint8_t);
            }
            (*mbmi).set_compound_idx(1 as uint8_t as uint8_t);
        }
        let rdc: *mut RD_COUNTS = &raw mut (*td).rd_counts;
        if (*mbmi).skip_mode() != 0 {
            if (frame_is_intra_only(cm) != 0) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 15],
                        [core::ffi::c_char; 15],
                    >(*b"encode_b_nonrd\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    2136 as core::ffi::c_int,
                    b"!frame_is_intra_only(cm)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            (*rdc).skip_mode_used_flag = 1 as core::ffi::c_int;
            if (*cm).current_frame.reference_mode as core::ffi::c_int
                == REFERENCE_MODE_SELECT as core::ffi::c_int && has_second_ref(mbmi) != 0
            {
                (*rdc).compound_ref_used_flag = 1 as core::ffi::c_int;
            }
            set_ref_ptrs(
                cm,
                xd,
                (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
                (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
            );
        } else {
            let seg_ref_active: core::ffi::c_int = segfeature_active(
                &(*cm).seg,
                (*mbmi).segment_id(),
                SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
            ) as core::ffi::c_int;
            if seg_ref_active == 0 {
                if is_inter_block(mbmi) != 0 {
                    av1_collect_neighbors_ref_counts(xd);
                    if (*cm).current_frame.reference_mode as core::ffi::c_int
                        == REFERENCE_MODE_SELECT as core::ffi::c_int
                        && has_second_ref(mbmi) != 0
                    {
                        (*rdc).compound_ref_used_flag = 1 as core::ffi::c_int;
                    }
                    set_ref_ptrs(
                        cm,
                        xd,
                        (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
                        (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
                    );
                }
            }
        }
        if (*cpi).oxcf.algo_cfg.loopfilter_control as core::ffi::c_uint
            == LOOPFILTER_SELECTIVELY as core::ffi::c_int as core::ffi::c_uint
            && ((*mbmi).mode as core::ffi::c_int == NEWMV as core::ffi::c_int
                || ((*mbmi).mode as core::ffi::c_int)
                    < INTRA_MODE_END as core::ffi::c_int)
        {
            let mut blocks: int32_t = mi_size_high[bsize as usize] as int32_t
                * mi_size_wide[bsize as usize] as int32_t;
            (*rdc).newmv_or_intra_blocks += blocks as core::ffi::c_int;
        }
        if (*tile_data).allow_update_cdf != 0 {
            update_stats(&(*cpi).common, td);
        }
    }
    if ((*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        == CYCLIC_REFRESH_AQ as core::ffi::c_int || (*cpi).active_map.enabled != 0
        || (*cpi).roi.enabled as core::ffi::c_int != 0)
        && (*mbmi).skip_txfm as core::ffi::c_int != 0
        && (*cpi).rc.rtc_external_ratectrl == 0
        && (*cm).seg.enabled as core::ffi::c_int != 0
    {
        av1_cyclic_reset_segment_skip(
            cpi as *const AV1_COMP,
            x,
            mi_row,
            mi_col,
            bsize,
            dry_run,
        );
    }
    av1_copy_mbmi_ext_to_mbmi_ext_frame(
        (*x).mbmi_ext_frame,
        &raw mut (*x).mbmi_ext,
        av1_ref_frame_type(
            ((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).ref_frame)
                .as_mut_ptr(),
        ) as uint8_t,
    );
    (*x).rdmult = origin_mult;
}
unsafe extern "C-unwind" fn get_force_zeromv_skip_flag_for_blk(
    mut cpi: *const AV1_COMP,
    mut x: *const MACROBLOCK,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if (*x).force_zeromv_skip_for_sb < 2 as core::ffi::c_int {
        return (*x).force_zeromv_skip_for_sb;
    }
    let cm: *const AV1_COMMON = &(*cpi).common;
    if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let thresh_exit_part_y: core::ffi::c_uint = (*cpi)
        .zeromv_skip_thresh_exit_part[bsize as usize];
    let thresh_exit_part_uv: core::ffi::c_uint = (3 as core::ffi::c_uint)
        .wrapping_mul(thresh_exit_part_y) >> 2 as core::ffi::c_int;
    let thresh_exit_part: [core::ffi::c_uint; 3] = [
        thresh_exit_part_y,
        thresh_exit_part_uv,
        thresh_exit_part_uv,
    ];
    let yv12: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *const YV12_BUFFER_CONFIG;
    let sf: *const scale_factors = get_ref_scale_factors_const(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *const scale_factors;
    let mut yv12_mb: [buf_2d; 3] = [buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3];
    av1_setup_pred_block(xd, yv12_mb.as_mut_ptr(), yv12, sf, sf, num_planes);
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let p: *const macroblock_plane = &*((*x).plane).as_ptr().offset(plane as isize)
            as *const macroblock_plane;
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(plane as isize) as *const macroblockd_plane;
        let bs: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        let plane_sad: core::ffi::c_uint = ((*(*cpi).ppi).fn_ptr[bs as usize].sdf)
            .expect(
                "non-null function pointer",
            )(
            (*p).src.buf,
            (*p).src.stride,
            yv12_mb[plane as usize].buf,
            yv12_mb[plane as usize].stride,
        ) as core::ffi::c_uint;
        if !(plane < 3 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 35],
                    [core::ffi::c_char; 35],
                >(*b"get_force_zeromv_skip_flag_for_blk\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                2219 as core::ffi::c_int,
                b"plane < MAX_MB_PLANE\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if plane_sad >= thresh_exit_part[plane as usize] {
            return 0 as core::ffi::c_int;
        }
        plane += 1;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn pick_sb_modes_nonrd(
    cpi: *mut AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut rd_cost: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ctx: *mut PICK_MODE_CONTEXT,
) {
    if bsize as core::ffi::c_int
        != (*(*cpi).common.seq_params).sb_size as core::ffi::c_int
        || (*cpi).sf.rt_sf.nonrd_check_partition_split == 1 as core::ffi::c_int
    {
        av1_set_offsets(cpi, &raw mut (*tile_data).tile_info, x, mi_row, mi_col, bsize);
    }
    if !((*x).last_set_offsets_loc.mi_row == mi_row
        && (*x).last_set_offsets_loc.mi_col == mi_col
        && (*x).last_set_offsets_loc.bsize as core::ffi::c_int
            == bsize as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"pick_sb_modes_nonrd\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            2269 as core::ffi::c_int,
            b"x->last_set_offsets_loc.mi_row == mi_row && x->last_set_offsets_loc.mi_col == mi_col && x->last_set_offsets_loc.bsize == bsize\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let p: *mut macroblock_plane = ((*x).plane).as_mut_ptr();
    let pd: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
    let aq_mode: AQ_MODE = (*cpi).oxcf.q_cfg.aq_mode;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    let mut i: core::ffi::c_int = 0;
    let seg_skip: core::ffi::c_int = segfeature_active(
        &raw mut (*cm).seg,
        (*mbmi).segment_id(),
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) as core::ffi::c_int;
    wait_for_top_right_sb(
        &raw mut (*cpi).mt_info.enc_row_mt,
        &raw mut (*tile_data).row_mt_sync,
        &raw mut (*tile_data).tile_info,
        (*(*cm).seq_params).sb_size,
        (*(*cm).seq_params).mib_size_log2,
        bsize,
        mi_row,
        mi_col,
    );
    (*xd).tx_type_map = ((*txfm_info).tx_type_map_).as_mut_ptr();
    (*xd).tx_type_map_stride = mi_size_wide[bsize as usize] as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        let fresh28 = &raw mut (*p.offset(i as isize)).coeff;
        *fresh28 = (*ctx).coeff[i as usize];
        let fresh29 = &raw mut (*p.offset(i as isize)).qcoeff;
        *fresh29 = (*ctx).qcoeff[i as usize];
        let fresh30 = &raw mut (*p.offset(i as isize)).dqcoeff;
        *fresh30 = (*ctx).dqcoeff[i as usize];
        let fresh31 = &raw mut (*p.offset(i as isize)).eobs;
        *fresh31 = (*ctx).eobs[i as usize];
        let fresh32 = &raw mut (*p.offset(i as isize)).txb_entropy_ctx;
        *fresh32 = (*ctx).txb_entropy_ctx[i as usize];
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let fresh33 = &raw mut (*pd.offset(i as isize)).color_index_map;
        *fresh33 = (*ctx).color_index_map[i as usize];
        i += 1;
    }
    if seg_skip == 0 {
        (*x).force_zeromv_skip_for_blk = get_force_zeromv_skip_flag_for_blk(
            cpi,
            x,
            bsize,
        );
        if (*x).force_zeromv_skip_for_blk == 0
            && ((*x).source_variance == UINT_MAX
                || (bsize as core::ffi::c_int)
                    < (*(*cm).seq_params).sb_size as core::ffi::c_int)
        {
            (*x).source_variance = av1_get_perpixel_variance_facade(
                cpi,
                xd,
                &raw mut (*((*x).plane).as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                    .src,
                bsize,
                AOM_PLANE_Y,
            );
        }
    }
    let orig_rdmult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(cpi, x, mi_row, mi_col, bsize, aq_mode, mbmi);
    if (*cpi).roi.enabled as core::ffi::c_int != 0 && (*cpi).roi.delta_qp_enabled != 0
        && (*mbmi).segment_id() as core::ffi::c_int != 0
    {
        (*x).rdmult = (*cpi).roi.rdmult_delta_qp;
    }
    av1_set_error_per_bit(&raw mut (*x).errorperbit, (*x).rdmult);
    if frame_is_intra_only(cm) != 0 {
        hybrid_intra_mode_search(cpi, x, rd_cost, bsize, ctx);
    } else {
        if seg_skip != 0 {
            (*x).force_zeromv_skip_for_blk = 1 as core::ffi::c_int;
        }
        av1_nonrd_pick_inter_mode_sb(
            cpi as *mut AV1_COMP,
            tile_data as *mut TileDataEnc,
            x as *mut macroblock,
            rd_cost as *mut RD_STATS,
            bsize,
            ctx,
        );
    }
    if (*cpi).sf.rt_sf.skip_cdef_sb != 0 {
        let allow_cdef_skipping: core::ffi::c_int = (seg_skip != 0
            || (*cpi).rc.frames_since_key > 10 as core::ffi::c_int
                && (*cpi).rc.high_source_sad == 0
                && !((*x)
                    .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int != 0
                    || (*x)
                        .color_sensitivity[(2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as core::ffi::c_int != 0))
            as core::ffi::c_int;
        let mi_row_sb: core::ffi::c_int = mi_row - mi_row % MI_SIZE_64X64;
        let mi_col_sb: core::ffi::c_int = mi_col - mi_col % MI_SIZE_64X64;
        let mut mi_sb: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
            .offset(
                get_mi_grid_idx(&raw mut (*cm).mi_params, mi_row_sb, mi_col_sb) as isize,
            );
        let is_720p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
            (*cm).width
        } else {
            (*cm).height
        }) >= 720 as core::ffi::c_int) as core::ffi::c_int;
        let mut thresh_spatial_var: core::ffi::c_uint = if (*cpi).oxcf.speed
            >= 11 as core::ffi::c_int && is_720p_or_larger == 0
            && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
                != AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        {
            400 as core::ffi::c_uint
        } else {
            UINT_MAX
        };
        if (*cpi).sf.rt_sf.skip_cdef_sb >= 2 as core::ffi::c_int {
            let fresh34 = &raw mut **mi_sb.offset(0 as core::ffi::c_int as isize);
            (*fresh34)
                .set_cdef_strength(
                    ((**mi_sb.offset(0 as core::ffi::c_int as isize)).cdef_strength()
                        as core::ffi::c_int != 0
                        && (allow_cdef_skipping != 0
                            || (*x).source_variance == 0 as core::ffi::c_uint))
                        as core::ffi::c_int as int8_t as int8_t,
                );
        } else {
            let fresh35 = &raw mut **mi_sb.offset(0 as core::ffi::c_int as isize);
            (*fresh35)
                .set_cdef_strength(
                    ((**mi_sb.offset(0 as core::ffi::c_int as isize)).cdef_strength()
                        as core::ffi::c_int != 0 && allow_cdef_skipping != 0
                        && !((*x).source_variance < thresh_spatial_var
                            && (((*mbmi).mode as core::ffi::c_int)
                                < INTRA_MODES as core::ffi::c_int
                                || (*mbmi).mode as core::ffi::c_int
                                    == NEWMV as core::ffi::c_int))) as core::ffi::c_int
                        as int8_t as int8_t,
                );
        }
        ((*ctx).mic)
            .set_cdef_strength(
                (**mi_sb.offset(0 as core::ffi::c_int as isize)).cdef_strength()
                    as int8_t,
            );
    }
    (*x).rdmult = orig_rdmult;
    (*ctx).rd_stats.rate = (*rd_cost).rate;
    (*ctx).rd_stats.dist = (*rd_cost).dist;
    (*ctx).rd_stats.rdcost = (*rd_cost).rdcost;
}
unsafe extern "C-unwind" fn try_split_partition(
    cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    tile_data: *mut TileDataEnc,
    tile_info: *mut TileInfo,
    mut tp: *mut *mut TokenExtra,
    x: *mut MACROBLOCK,
    xd: *mut MACROBLOCKD,
    mi_params: *const CommonModeInfoParams,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    bsize: BLOCK_SIZE,
    pl: core::ffi::c_int,
    mut pc_tree: *mut PC_TREE,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let hbs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
        / 2 as core::ffi::c_int;
    if mi_row + mi_size_high[bsize as usize] as core::ffi::c_int >= (*mi_params).mi_rows
        || mi_col + mi_size_wide[bsize as usize] as core::ffi::c_int
            >= (*mi_params).mi_cols
    {
        return 0 as core::ffi::c_int;
    }
    if bsize as core::ffi::c_int <= BLOCK_8X8 as core::ffi::c_int
        || frame_is_intra_only(cm) != 0
    {
        return 0 as core::ffi::c_int;
    }
    if (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
        <= kLowSad as core::ffi::c_int as core::ffi::c_uint
    {
        return 0 as core::ffi::c_int;
    }
    let yv12: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *const YV12_BUFFER_CONFIG;
    let sf: *const scale_factors = get_ref_scale_factors_const(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *const scale_factors;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    av1_setup_src_planes(
        x as *mut macroblock,
        (*cpi).source,
        mi_row,
        mi_col,
        num_planes,
        bsize,
    );
    av1_setup_pre_planes(
        xd,
        0 as core::ffi::c_int,
        yv12,
        mi_row,
        mi_col,
        sf,
        num_planes,
    );
    let mut block_sad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let p: *const macroblock_plane = &raw mut *((*x).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblock_plane;
        let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblockd_plane;
        let bs: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        let plane_sad: core::ffi::c_uint = ((*(*cpi).ppi).fn_ptr[bs as usize].sdf)
            .expect(
                "non-null function pointer",
            )(
            (*p).src.buf,
            (*p).src.stride,
            (*pd).pre[0 as core::ffi::c_int as usize].buf,
            (*pd).pre[0 as core::ffi::c_int as usize].stride,
        ) as core::ffi::c_uint;
        block_sad = (block_sad as core::ffi::c_uint).wrapping_add(plane_sad)
            as core::ffi::c_int as core::ffi::c_int;
        plane += 1;
    }
    let blk_pix: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int
        * block_size_high[bsize as usize] as core::ffi::c_int;
    let block_avg_sad: core::ffi::c_int = block_sad / blk_pix;
    let threshold: core::ffi::c_int = 25 as core::ffi::c_int;
    if block_avg_sad < threshold {
        return 0 as core::ffi::c_int;
    }
    let mut x_ctx: RD_SEARCH_MACROBLOCK_CONTEXT = RD_SEARCH_MACROBLOCK_CONTEXT {
        a: [0; 96],
        l: [0; 96],
        sa: [0; 32],
        sl: [0; 32],
        p_ta: 0 as *mut TXFM_CONTEXT,
        p_tl: 0 as *mut TXFM_CONTEXT,
        ta: [0; 32],
        tl: [0; 32],
    };
    let mut split_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut none_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut split_rdc);
    av1_invalid_rd_stats(&raw mut none_rdc);
    av1_save_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, 3 as core::ffi::c_int);
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
    av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, bsize);
    if ((*pc_tree).none).is_null() {
        (*pc_tree).none = av1_alloc_pmc(cpi, bsize, &raw mut (*td).shared_coeff_buf);
        if ((*pc_tree).none).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        av1_reset_pmc((*pc_tree).none);
    }
    pick_sb_modes_nonrd(
        cpi,
        tile_data,
        x,
        mi_row,
        mi_col,
        &raw mut none_rdc,
        bsize,
        (*pc_tree).none,
    );
    none_rdc.rate
        += (*mode_costs)
            .partition_cost[pl as usize][PARTITION_NONE as core::ffi::c_int as usize];
    none_rdc.rdcost = (none_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + none_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, 3 as core::ffi::c_int);
    (*pc_tree).partitioning = PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
    let subsize: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    av1_init_rd_stats(&raw mut split_rdc);
    split_rdc.rate
        += (*mode_costs)
            .partition_cost[pl as usize][PARTITION_SPLIT as core::ffi::c_int as usize];
    if subsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int {
        split_rdc.rate
            += (*mode_costs)
                .partition_cost[pl as usize][PARTITION_NONE as core::ffi::c_int as usize]
                * 4 as core::ffi::c_int;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SUB_PARTITIONS_SPLIT {
        if ((*pc_tree).split[i as usize]).is_null() {
            (*pc_tree).split[i as usize] = av1_alloc_pc_tree_node(subsize)
                as *mut PC_TREE;
            if ((*pc_tree).split[i as usize]).is_null() {
                aom_internal_error(
                    (*xd).error_info as *mut aom_internal_error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate PC_TREE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        (*(*pc_tree).split[i as usize]).index = i;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < SUB_PARTITIONS_SPLIT {
        let mut block_rdc: RD_STATS = RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        };
        av1_invalid_rd_stats(&raw mut block_rdc);
        let mut x_idx: core::ffi::c_int = (i_0 & 1 as core::ffi::c_int) * hbs;
        let mut y_idx: core::ffi::c_int = (i_0 >> 1 as core::ffi::c_int) * hbs;
        if !(mi_row + y_idx >= (*mi_params).mi_rows
            || mi_col + x_idx >= (*mi_params).mi_cols)
        {
            (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
                .offset((*tile_info).tile_row as isize))
                .offset(mi_col as isize)
                .offset(x_idx as isize);
            (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
                .as_mut_ptr()
                .offset((mi_row + y_idx & MAX_MIB_MASK) as isize);
            if ((*(*pc_tree).split[i_0 as usize]).none).is_null() {
                (*(*pc_tree).split[i_0 as usize]).none = av1_alloc_pmc(
                    cpi,
                    subsize,
                    &raw mut (*td).shared_coeff_buf,
                );
                if ((*(*pc_tree).split[i_0 as usize]).none).is_null() {
                    aom_internal_error(
                        (*xd).error_info as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            } else {
                av1_reset_pmc((*(*pc_tree).split[i_0 as usize]).none);
            }
            (*(*pc_tree).split[i_0 as usize]).partitioning = PARTITION_NONE
                as core::ffi::c_int as PARTITION_TYPE;
            pick_sb_modes_nonrd(
                cpi,
                tile_data,
                x,
                mi_row + y_idx,
                mi_col + x_idx,
                &raw mut block_rdc,
                subsize,
                (*(*pc_tree).split[i_0 as usize]).none,
            );
            split_rdc.rate += block_rdc.rate;
            split_rdc.dist += block_rdc.dist;
            av1_rd_cost_update((*x).rdmult, &raw mut split_rdc);
            if none_rdc.rdcost < split_rdc.rdcost {
                break;
            }
            if i_0 != SUB_PARTITIONS_SPLIT - 1 as core::ffi::c_int {
                encode_b_nonrd(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row + y_idx,
                    mi_col + x_idx,
                    1 as RUN_TYPE,
                    subsize,
                    PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
                    (*(*pc_tree).split[i_0 as usize]).none,
                    0 as *mut core::ffi::c_int,
                );
            }
        }
        i_0 += 1;
    }
    av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, 3 as core::ffi::c_int);
    split_rdc.rdcost = (split_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + split_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    let split: core::ffi::c_int = (split_rdc.rdcost < none_rdc.rdcost)
        as core::ffi::c_int;
    return split;
}
unsafe extern "C-unwind" fn calc_do_split_flag(
    mut cpi: *const AV1_COMP,
    mut x: *const MACROBLOCK,
    mut pc_tree: *const PC_TREE,
    mut none_rdc: *const RD_STATS,
    mut mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut hbs: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
) -> bool {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let is_larger_qindex: core::ffi::c_int = ((*cm).quant_params.base_qindex
        > 100 as core::ffi::c_int) as core::ffi::c_int;
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let mut do_split: bool = if (*cpi).sf.rt_sf.nonrd_check_partition_merge_mode
        == 3 as core::ffi::c_int
    {
        (bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int
            || is_larger_qindex != 0
                && bsize as core::ffi::c_int <= BLOCK_64X64 as core::ffi::c_int)
            as core::ffi::c_int
    } else {
        true_0
    } != 0;
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).sf.rt_sf.nonrd_check_partition_merge_mode < 2 as core::ffi::c_int
        || cyclic_refresh_segment_id_boosted(
            (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
                as core::ffi::c_int,
        ) != 0 || (*none_rdc).skip_txfm == 0
    {
        return do_split;
    }
    let use_model_yrd_large: core::ffi::c_int = get_model_rd_flag(cpi, xd, bsize)
        as core::ffi::c_int;
    if use_model_yrd_large == 0 || is_larger_qindex == 0 {
        return false_0 != 0;
    }
    if (*(*pc_tree).none).mic.mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        && bsize as core::ffi::c_int == BLOCK_32X32 as core::ffi::c_int
        && do_split as core::ffi::c_int != 0
    {
        let subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition) as BLOCK_SIZE;
        if !((subsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"calc_do_split_flag\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                2557 as core::ffi::c_int,
                b"subsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut min_per_pixel_error: core::ffi::c_double = DBL_MAX;
        let mut max_per_pixel_error: core::ffi::c_double = 0.0f64;
        let mut i: core::ffi::c_int = 0;
        i = 0 as core::ffi::c_int;
        while i < SUB_PARTITIONS_SPLIT {
            let x_idx: core::ffi::c_int = (i & 1 as core::ffi::c_int) * hbs;
            let y_idx: core::ffi::c_int = (i >> 1 as core::ffi::c_int) * hbs;
            if mi_row + y_idx >= (*mi_params).mi_rows
                || mi_col + x_idx >= (*mi_params).mi_cols
            {
                break;
            }
            let mut src_split_buf_2d: buf_2d = buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0,
                height: 0,
                stride: 0,
            };
            let mut pred_split_buf_2d: buf_2d = buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0,
                height: 0,
                stride: 0,
            };
            let mut src_none_buf_2d: *const buf_2d = &(*((*x).plane)
                .as_ptr()
                .offset(AOM_PLANE_Y as isize))
                .src;
            setup_pred_plane(
                &raw mut src_split_buf_2d,
                subsize,
                (*src_none_buf_2d).buf,
                (*src_none_buf_2d).width,
                (*src_none_buf_2d).height,
                (*src_none_buf_2d).stride,
                y_idx,
                x_idx,
                0 as *const scale_factors,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            let mut pred_none_buf_2d: *const buf_2d = &(*((*xd).plane)
                .as_ptr()
                .offset(AOM_PLANE_Y as isize))
                .dst;
            setup_pred_plane(
                &raw mut pred_split_buf_2d,
                subsize,
                (*pred_none_buf_2d).buf,
                (*pred_none_buf_2d).width,
                (*pred_none_buf_2d).height,
                (*pred_none_buf_2d).stride,
                y_idx,
                x_idx,
                0 as *const scale_factors,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            let mut curr_uint_mse: core::ffi::c_uint = 0;
            let curr_uint_var: core::ffi::c_uint = ((*(*cpi).ppi)
                .fn_ptr[subsize as usize]
                .vf)
                .expect(
                    "non-null function pointer",
                )(
                src_split_buf_2d.buf,
                src_split_buf_2d.stride,
                pred_split_buf_2d.buf,
                pred_split_buf_2d.stride,
                &raw mut curr_uint_mse,
            ) as core::ffi::c_uint;
            let curr_per_pixel_error: core::ffi::c_double = sqrt(
                curr_uint_var as core::ffi::c_double
                    / block_size_wide[subsize as usize] as core::ffi::c_int
                        as core::ffi::c_double
                    / block_size_high[subsize as usize] as core::ffi::c_int
                        as core::ffi::c_double,
            ) as core::ffi::c_double;
            if curr_per_pixel_error < min_per_pixel_error {
                min_per_pixel_error = curr_per_pixel_error;
            }
            if curr_per_pixel_error > max_per_pixel_error {
                max_per_pixel_error = curr_per_pixel_error;
            }
            i += 1;
        }
        if i == SUB_PARTITIONS_SPLIT {
            if max_per_pixel_error - min_per_pixel_error <= 1.5f64 {
                do_split = false_0 != 0;
            }
        }
    }
    return do_split;
}
unsafe extern "C-unwind" fn try_merge(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mib: *mut *mut MB_MODE_INFO,
    mut tp: *mut *mut TokenExtra,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    bsize: BLOCK_SIZE,
    pc_tree: *mut PC_TREE,
    partition: PARTITION_TYPE,
    subsize: BLOCK_SIZE,
    pl: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if !(bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && bsize as core::ffi::c_int <= BLOCK_128X128 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 10],
                [core::ffi::c_char; 10],
            >(*b"try_merge\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            2619 as core::ffi::c_int,
            b"bsize >= BLOCK_8X8 && bsize <= BLOCK_128X128\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let bs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bs / 2 as core::ffi::c_int;
    let mut do_split: bool = false_0 != 0;
    let mut x_ctx: RD_SEARCH_MACROBLOCK_CONTEXT = RD_SEARCH_MACROBLOCK_CONTEXT {
        a: [0; 96],
        l: [0; 96],
        sa: [0; 32],
        sl: [0; 32],
        p_ta: 0 as *mut TXFM_CONTEXT,
        p_tl: 0 as *mut TXFM_CONTEXT,
        ta: [0; 32],
        tl: [0; 32],
    };
    let mut split_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut none_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut split_rdc);
    av1_invalid_rd_stats(&raw mut none_rdc);
    av1_save_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
    if ((*pc_tree).none).is_null() {
        (*pc_tree).none = av1_alloc_pmc(cpi, bsize, &raw mut (*td).shared_coeff_buf);
        if ((*pc_tree).none).is_null() {
            aom_internal_error(
                (*xd).error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        av1_reset_pmc((*pc_tree).none);
    }
    pick_sb_modes_nonrd(
        cpi,
        tile_data,
        x,
        mi_row,
        mi_col,
        &raw mut none_rdc,
        bsize,
        (*pc_tree).none,
    );
    none_rdc.rate
        += (*mode_costs)
            .partition_cost[pl as usize][PARTITION_NONE as core::ffi::c_int as usize];
    none_rdc.rdcost = (none_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + none_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
    if (*cpi).sf.rt_sf.nonrd_check_partition_merge_mode < 2 as core::ffi::c_int
        || none_rdc.skip_txfm as core::ffi::c_int != 1 as core::ffi::c_int
        || (*(*pc_tree).none).mic.mode as core::ffi::c_int == NEWMV as core::ffi::c_int
    {
        do_split = calc_do_split_flag(
            cpi,
            x,
            pc_tree,
            &raw mut none_rdc,
            mi_params,
            mi_row,
            mi_col,
            hbs,
            bsize,
            partition,
        );
        if do_split {
            av1_init_rd_stats(&raw mut split_rdc);
            split_rdc.rate
                += (*mode_costs)
                    .partition_cost[pl
                    as usize][PARTITION_SPLIT as core::ffi::c_int as usize];
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < SUB_PARTITIONS_SPLIT {
                let mut block_rdc: RD_STATS = RD_STATS {
                    rate: 0,
                    zero_rate: 0,
                    dist: 0,
                    rdcost: 0,
                    sse: 0,
                    skip_txfm: 0,
                };
                av1_invalid_rd_stats(&raw mut block_rdc);
                let mut x_idx: core::ffi::c_int = (i & 1 as core::ffi::c_int) * hbs;
                let mut y_idx: core::ffi::c_int = (i >> 1 as core::ffi::c_int) * hbs;
                if !(mi_row + y_idx >= (*mi_params).mi_rows
                    || mi_col + x_idx >= (*mi_params).mi_cols)
                {
                    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
                        .offset((*tile_info).tile_row as isize))
                        .offset(mi_col as isize)
                        .offset(x_idx as isize);
                    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
                        .as_mut_ptr()
                        .offset((mi_row + y_idx & MAX_MIB_MASK) as isize);
                    if ((*(*pc_tree).split[i as usize]).none).is_null() {
                        (*(*pc_tree).split[i as usize]).none = av1_alloc_pmc(
                            cpi,
                            subsize,
                            &raw mut (*td).shared_coeff_buf,
                        );
                        if ((*(*pc_tree).split[i as usize]).none).is_null() {
                            aom_internal_error(
                                (*xd).error_info as *mut aom_internal_error_info,
                                AOM_CODEC_MEM_ERROR,
                                b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                    } else {
                        av1_reset_pmc((*(*pc_tree).split[i as usize]).none);
                    }
                    (*(*pc_tree).split[i as usize]).partitioning = PARTITION_NONE
                        as core::ffi::c_int as PARTITION_TYPE;
                    pick_sb_modes_nonrd(
                        cpi,
                        tile_data,
                        x,
                        mi_row + y_idx,
                        mi_col + x_idx,
                        &raw mut block_rdc,
                        subsize,
                        (*(*pc_tree).split[i as usize]).none,
                    );
                    split_rdc.rate += block_rdc.rate;
                    split_rdc.dist += block_rdc.dist;
                    av1_rd_cost_update((*x).rdmult, &raw mut split_rdc);
                    if none_rdc.rdcost < split_rdc.rdcost {
                        break;
                    }
                    if i != SUB_PARTITIONS_SPLIT - 1 as core::ffi::c_int {
                        encode_b_nonrd(
                            cpi,
                            tile_data,
                            td,
                            tp,
                            mi_row + y_idx,
                            mi_col + x_idx,
                            1 as RUN_TYPE,
                            subsize,
                            PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
                            (*(*pc_tree).split[i as usize]).none,
                            0 as *mut core::ffi::c_int,
                        );
                    }
                }
                i += 1;
            }
            av1_restore_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
            split_rdc.rdcost = (split_rdc.rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + split_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        }
    }
    if none_rdc.rdcost < split_rdc.rdcost {
        if do_split {
            (*x).reuse_inter_pred = false_0 != 0;
        }
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = bsize;
        (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE;
        encode_b_nonrd(
            cpi,
            tile_data,
            td,
            tp,
            mi_row,
            mi_col,
            0 as RUN_TYPE,
            bsize,
            partition,
            (*pc_tree).none,
            0 as *mut core::ffi::c_int,
        );
    } else {
        (**mib.offset(0 as core::ffi::c_int as isize)).bsize = subsize;
        (*pc_tree).partitioning = PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
        (*x).reuse_inter_pred = false_0 != 0;
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < SUB_PARTITIONS_SPLIT {
            let mut x_idx_0: core::ffi::c_int = (i_0 & 1 as core::ffi::c_int) * hbs;
            let mut y_idx_0: core::ffi::c_int = (i_0 >> 1 as core::ffi::c_int) * hbs;
            if !(mi_row + y_idx_0 >= (*mi_params).mi_rows
                || mi_col + x_idx_0 >= (*mi_params).mi_cols)
            {
                if ((*(*pc_tree).split[i_0 as usize]).none).is_null() {
                    (*(*pc_tree).split[i_0 as usize]).none = av1_alloc_pmc(
                        cpi,
                        subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if ((*(*pc_tree).split[i_0 as usize]).none).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                }
                encode_b_nonrd(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row + y_idx_0,
                    mi_col + x_idx_0,
                    0 as RUN_TYPE,
                    subsize,
                    PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
                    (*(*pc_tree).split[i_0 as usize]).none,
                    0 as *mut core::ffi::c_int,
                );
            }
            i_0 += 1;
        }
    };
}
unsafe extern "C-unwind" fn direct_partition_merging(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mib: *mut *mut MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let bs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bs / 2 as core::ffi::c_int;
    let partition: PARTITION_TYPE = (if bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int
    {
        get_partition(cm, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        PARTITION_NONE as core::ffi::c_int
    }) as PARTITION_TYPE;
    let mut subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition);
    let mut b0: *mut *mut MB_MODE_INFO = mib;
    let mut b1: *mut *mut MB_MODE_INFO = mib.offset(hbs as isize);
    let mut b2: *mut *mut MB_MODE_INFO = mib
        .offset((hbs * (*mi_params).mi_stride) as isize);
    let mut b3: *mut *mut MB_MODE_INFO = mib
        .offset((hbs * (*mi_params).mi_stride) as isize)
        .offset(hbs as isize);
    let further_split: core::ffi::c_int = (((**b0.offset(0 as core::ffi::c_int as isize))
        .bsize as core::ffi::c_int) < subsize as core::ffi::c_int
        || ((**b1.offset(0 as core::ffi::c_int as isize)).bsize as core::ffi::c_int)
            < subsize as core::ffi::c_int
        || ((**b2.offset(0 as core::ffi::c_int as isize)).bsize as core::ffi::c_int)
            < subsize as core::ffi::c_int
        || ((**b3.offset(0 as core::ffi::c_int as isize)).bsize as core::ffi::c_int)
            < subsize as core::ffi::c_int) as core::ffi::c_int;
    if further_split != 0 {
        return;
    }
    let no_skip: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .skip_txfm == 0 || (**b1.offset(0 as core::ffi::c_int as isize)).skip_txfm == 0
        || (**b2.offset(0 as core::ffi::c_int as isize)).skip_txfm == 0
        || (**b3.offset(0 as core::ffi::c_int as isize)).skip_txfm == 0)
        as core::ffi::c_int;
    if no_skip != 0 {
        return;
    }
    let compound: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        != (**b1.offset(0 as core::ffi::c_int as isize))
            .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            != (**b2.offset(0 as core::ffi::c_int as isize))
                .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            != (**b3.offset(0 as core::ffi::c_int as isize))
                .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > NONE_FRAME as core::ffi::c_int) as core::ffi::c_int;
    if compound != 0 {
        return;
    }
    let different_ref: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        != (**b1.offset(0 as core::ffi::c_int as isize))
            .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            != (**b2.offset(0 as core::ffi::c_int as isize))
                .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            != (**b3.offset(0 as core::ffi::c_int as isize))
                .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            <= INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
    if different_ref != 0 {
        return;
    }
    let different_mode: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .mode as core::ffi::c_int
        != (**b1.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int
            != (**b2.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int
            != (**b3.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int)
        as core::ffi::c_int;
    if different_mode != 0 {
        return;
    }
    let unsupported_mode: core::ffi::c_int = ((**b0
        .offset(0 as core::ffi::c_int as isize))
        .mode as core::ffi::c_int != NEARESTMV as core::ffi::c_int
        && (**b0.offset(0 as core::ffi::c_int as isize)).mode as core::ffi::c_int
            != GLOBALMV as core::ffi::c_int) as core::ffi::c_int;
    if unsupported_mode != 0 {
        return;
    }
    let different_mv: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .mv[0 as core::ffi::c_int as usize]
        .as_int
        != (**b1.offset(0 as core::ffi::c_int as isize))
            .mv[0 as core::ffi::c_int as usize]
            .as_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .mv[0 as core::ffi::c_int as usize]
            .as_int
            != (**b2.offset(0 as core::ffi::c_int as isize))
                .mv[0 as core::ffi::c_int as usize]
                .as_int
        || (**b0.offset(0 as core::ffi::c_int as isize))
            .mv[0 as core::ffi::c_int as usize]
            .as_int
            != (**b3.offset(0 as core::ffi::c_int as isize))
                .mv[0 as core::ffi::c_int as usize]
                .as_int) as core::ffi::c_int;
    if different_mv != 0 {
        return;
    }
    let unsupported_motion_mode: core::ffi::c_int = ((**b0
        .offset(0 as core::ffi::c_int as isize))
        .motion_mode as core::ffi::c_int
        != (**b1.offset(0 as core::ffi::c_int as isize)).motion_mode as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).motion_mode as core::ffi::c_int
            != (**b2.offset(0 as core::ffi::c_int as isize)).motion_mode
                as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).motion_mode as core::ffi::c_int
            != (**b3.offset(0 as core::ffi::c_int as isize)).motion_mode
                as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).motion_mode as core::ffi::c_int
            != SIMPLE_TRANSLATION as core::ffi::c_int) as core::ffi::c_int;
    if unsupported_motion_mode != 0 {
        return;
    }
    let diffent_filter: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .interp_filters
        .as_int != (**b1.offset(0 as core::ffi::c_int as isize)).interp_filters.as_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).interp_filters.as_int
            != (**b2.offset(0 as core::ffi::c_int as isize)).interp_filters.as_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).interp_filters.as_int
            != (**b3.offset(0 as core::ffi::c_int as isize)).interp_filters.as_int)
        as core::ffi::c_int;
    if diffent_filter != 0 {
        return;
    }
    let different_seg: core::ffi::c_int = ((**b0.offset(0 as core::ffi::c_int as isize))
        .segment_id() as core::ffi::c_int
        != (**b1.offset(0 as core::ffi::c_int as isize)).segment_id() as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).segment_id() as core::ffi::c_int
            != (**b2.offset(0 as core::ffi::c_int as isize)).segment_id()
                as core::ffi::c_int
        || (**b0.offset(0 as core::ffi::c_int as isize)).segment_id() as core::ffi::c_int
            != (**b3.offset(0 as core::ffi::c_int as isize)).segment_id()
                as core::ffi::c_int) as core::ffi::c_int;
    if different_seg != 0 {
        return;
    }
    let mut this_mi: *mut *mut MB_MODE_INFO = mib;
    let mut orig_bsize: BLOCK_SIZE = (**this_mi.offset(0 as core::ffi::c_int as isize))
        .bsize;
    let orig_partition: PARTITION_TYPE = (**this_mi
        .offset(0 as core::ffi::c_int as isize))
        .partition;
    (**this_mi.offset(0 as core::ffi::c_int as isize)).bsize = bsize;
    (**this_mi.offset(0 as core::ffi::c_int as isize)).partition = PARTITION_NONE
        as core::ffi::c_int as PARTITION_TYPE;
    (**this_mi.offset(0 as core::ffi::c_int as isize)).skip_txfm = 1 as uint8_t;
    av1_set_offsets_without_segment_id(
        cpi,
        &raw mut (*tile_data).tile_info,
        x,
        mi_row,
        mi_col,
        bsize,
    );
    let ref_frame: MV_REFERENCE_FRAME = (**this_mi
        .offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize];
    let mut frame_mv: [[int_mv; 8]; 25] = [[int_mv { as_int: 0 }; 8]; 25];
    let mut yv12_mb: [[buf_2d; 3]; 8] = [[buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3]; 8];
    let mut force_skip_low_temp_var: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut skip_pred_mv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut use_scaled_ref: bool = false;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MB_MODE_COUNT as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < REF_FRAMES as core::ffi::c_int {
            frame_mv[i as usize][j as usize].as_int = INVALID_MV as uint32_t;
            j += 1;
        }
        i += 1;
    }
    memcpy(
        ((*x).color_sensitivity).as_mut_ptr() as *mut core::ffi::c_void,
        ((*x).color_sensitivity_sb).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
    );
    skip_pred_mv = ((*x).nonrd_prune_ref_frame_search > 2 as core::ffi::c_int
        && (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 2 as core::ffi::c_int
        && (*x)
            .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 2 as core::ffi::c_int) as core::ffi::c_int;
    find_predictors(
        cpi,
        x,
        ref_frame,
        frame_mv.as_mut_ptr(),
        yv12_mb.as_mut_ptr(),
        bsize,
        force_skip_low_temp_var,
        skip_pred_mv,
        &raw mut use_scaled_ref,
    );
    let mut continue_merging: core::ffi::c_int = 1 as core::ffi::c_int;
    if frame_mv[NEARESTMV as core::ffi::c_int as usize][ref_frame as usize].as_mv.row
        as core::ffi::c_int
        != (**b0.offset(0 as core::ffi::c_int as isize))
            .mv[0 as core::ffi::c_int as usize]
            .as_mv
            .row as core::ffi::c_int
        || frame_mv[NEARESTMV as core::ffi::c_int as usize][ref_frame as usize].as_mv.col
            as core::ffi::c_int
            != (**b0.offset(0 as core::ffi::c_int as isize))
                .mv[0 as core::ffi::c_int as usize]
                .as_mv
                .col as core::ffi::c_int
    {
        continue_merging = 0 as core::ffi::c_int;
    }
    if continue_merging == 0 {
        (**this_mi.offset(0 as core::ffi::c_int as isize)).bsize = orig_bsize;
        (**this_mi.offset(0 as core::ffi::c_int as isize)).partition = orig_partition;
        av1_set_offsets_without_segment_id(
            cpi,
            &raw mut (*tile_data).tile_info,
            x,
            mi_row,
            mi_col,
            (**this_mi.offset(0 as core::ffi::c_int as isize)).bsize,
        );
        find_predictors(
            cpi,
            x,
            ref_frame,
            frame_mv.as_mut_ptr(),
            yv12_mb.as_mut_ptr(),
            (**this_mi.offset(0 as core::ffi::c_int as isize)).bsize,
            force_skip_low_temp_var,
            skip_pred_mv,
            &raw mut use_scaled_ref,
        );
    } else {
        let mut sf: *mut scale_factors = get_ref_scale_factors(cm, ref_frame);
        let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
        let is_y_subpel_mv: core::ffi::c_int = (abs(
            (**this_mi.offset(0 as core::ffi::c_int as isize))
                .mv[0 as core::ffi::c_int as usize]
                .as_mv
                .row as core::ffi::c_int,
        ) % 8 as core::ffi::c_int != 0
            || abs(
                (**this_mi.offset(0 as core::ffi::c_int as isize))
                    .mv[0 as core::ffi::c_int as usize]
                    .as_mv
                    .col as core::ffi::c_int,
            ) % 8 as core::ffi::c_int != 0) as core::ffi::c_int;
        let is_uv_subpel_mv: core::ffi::c_int = (abs(
            (**this_mi.offset(0 as core::ffi::c_int as isize))
                .mv[0 as core::ffi::c_int as usize]
                .as_mv
                .row as core::ffi::c_int,
        ) % 16 as core::ffi::c_int != 0
            || abs(
                (**this_mi.offset(0 as core::ffi::c_int as isize))
                    .mv[0 as core::ffi::c_int as usize]
                    .as_mv
                    .col as core::ffi::c_int,
            ) % 16 as core::ffi::c_int != 0) as core::ffi::c_int;
        if (*(*cpi).ppi).use_svc != 0 || is_scaled != 0 || is_y_subpel_mv != 0
            || is_uv_subpel_mv != 0
        {
            let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
            set_ref_ptrs(
                cm,
                xd,
                ref_frame,
                (**this_mi.offset(0 as core::ffi::c_int as isize))
                    .ref_frame[1 as core::ffi::c_int as usize],
            );
            let mut cfg: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
                cm,
                ref_frame,
            );
            av1_setup_pre_planes(
                xd,
                0 as core::ffi::c_int,
                cfg,
                mi_row,
                mi_col,
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
                num_planes,
            );
            if (*(*cpi).ppi).use_svc == 0 && is_scaled == 0 && is_y_subpel_mv == 0 {
                if !(is_uv_subpel_mv == 1 as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 25],
                            [core::ffi::c_char; 25],
                        >(*b"direct_partition_merging\0"))
                            .as_ptr(),
                        b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                        2884 as core::ffi::c_int,
                        b"is_uv_subpel_mv == 1\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    1 as core::ffi::c_int,
                    num_planes - 1 as core::ffi::c_int,
                );
            } else {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    0 as core::ffi::c_int,
                    num_planes - 1 as core::ffi::c_int,
                );
            }
        }
        let mbmi_ext: *mut MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
        let mut mbmi_ext_frame: *mut MB_MODE_INFO_EXT_FRAME = (*x).mbmi_ext_frame;
        av1_copy_mbmi_ext_to_mbmi_ext_frame(
            mbmi_ext_frame,
            mbmi_ext,
            av1_ref_frame_type(
                ((**this_mi.offset(0 as core::ffi::c_int as isize)).ref_frame)
                    .as_mut_ptr(),
            ) as uint8_t,
        );
        let this_subsize: BLOCK_SIZE = get_partition_subsize(
            bsize,
            (**this_mi.offset(0 as core::ffi::c_int as isize)).partition,
        ) as BLOCK_SIZE;
        update_ext_partition_context(
            xd,
            mi_row,
            mi_col,
            this_subsize,
            bsize,
            (**this_mi.offset(0 as core::ffi::c_int as isize)).partition,
        );
        let num_planes_0: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        av1_reset_entropy_context(xd, bsize, num_planes_0);
        let mut tx_size: TX_SIZE = tx_size_from_tx_mode(
            bsize,
            (*x).txfm_search_params.tx_mode_search_type,
        );
        if (*xd)
            .lossless[(**this_mi.offset(0 as core::ffi::c_int as isize)).segment_id()
            as usize] != 0
        {
            tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
        }
        (**this_mi.offset(0 as core::ffi::c_int as isize)).tx_size = tx_size;
        memset(
            ((**this_mi.offset(0 as core::ffi::c_int as isize)).inter_tx_size)
                .as_mut_ptr() as *mut core::ffi::c_void,
            (**this_mi.offset(0 as core::ffi::c_int as isize)).tx_size
                as core::ffi::c_int,
            ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
        );
        (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
            .offset((*tile_info).tile_row as isize))
            .offset(mi_col as isize);
        (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
            .as_mut_ptr()
            .offset((mi_row & MAX_MIB_MASK) as isize);
        set_txfm_ctxs(
            (**this_mi.offset(0 as core::ffi::c_int as isize)).tx_size,
            (*xd).width as core::ffi::c_int,
            (*xd).height as core::ffi::c_int,
            ((**this_mi.offset(0 as core::ffi::c_int as isize)).skip_txfm
                as core::ffi::c_int != 0
                && is_inter_block(*this_mi.offset(0 as core::ffi::c_int as isize)) != 0)
                as core::ffi::c_int,
            xd,
        );
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < bs {
            let mut x_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_idx < bs {
                let fresh27 = &raw mut *this_mi
                    .offset((x_idx + y * (*mi_params).mi_stride) as isize);
                *fresh27 = *this_mi.offset(0 as core::ffi::c_int as isize);
                x_idx += 1;
            }
            y += 1;
        }
        (**this_mi.offset(0 as core::ffi::c_int as isize)).num_proj_ref = 0 as uint8_t;
        if warped_motion_update_num_proj_ref(
            cpi,
            *this_mi.offset(0 as core::ffi::c_int as isize),
        ) != 0
        {
            let mut pts: [core::ffi::c_int; 16] = [0; 16];
            let mut pts_inref: [core::ffi::c_int; 16] = [0; 16];
            (**this_mi.offset(0 as core::ffi::c_int as isize)).num_proj_ref = av1_findSamples(
                cm,
                xd,
                pts.as_mut_ptr(),
                pts_inref.as_mut_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nonrd_use_partition(
    mut cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut mib: *mut *mut MB_MODE_INFO,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut pc_tree: *mut PC_TREE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    if !(bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && bsize as core::ffi::c_int <= BLOCK_128X128 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_nonrd_use_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            2980 as core::ffi::c_int,
            b"bsize >= BLOCK_8X8 && bsize <= BLOCK_128X128\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let bs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bs / 2 as core::ffi::c_int;
    let mut partition: PARTITION_TYPE = (if bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int
    {
        get_partition(cm, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        PARTITION_NONE as core::ffi::c_int
    }) as PARTITION_TYPE;
    let mut subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition);
    if !(subsize as core::ffi::c_int <= BLOCK_LARGEST as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_nonrd_use_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            2987 as core::ffi::c_int,
            b"subsize <= BLOCK_LARGEST\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let pl: core::ffi::c_int = if bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int
    {
        partition_plane_context(xd, mi_row, mi_col, bsize) as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut dummy_cost: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut dummy_cost);
    if mi_row >= (*mi_params).mi_rows || mi_col >= (*mi_params).mi_cols {
        return;
    }
    if !(mi_size_wide[bsize as usize] as core::ffi::c_int
        == mi_size_high[bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_nonrd_use_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            2997 as core::ffi::c_int,
            b"mi_size_wide[bsize] == mi_size_high[bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    set_mode_eval_params(cpi, x, DEFAULT_EVAL as core::ffi::c_int as MODE_EVAL_TYPE);
    (*x).reuse_inter_pred = (*cpi).sf.rt_sf.reuse_inter_pred_nonrd != 0;
    let mut change_none_to_split: core::ffi::c_int = 0 as core::ffi::c_int;
    if partition as core::ffi::c_int == PARTITION_NONE as core::ffi::c_int
        && (*cpi).sf.rt_sf.nonrd_check_partition_split == 1 as core::ffi::c_int
    {
        change_none_to_split = try_split_partition(
            cpi,
            td,
            tile_data,
            tile_info,
            tp,
            x,
            xd,
            mi_params,
            mi_row,
            mi_col,
            bsize,
            pl,
            pc_tree,
        );
        if change_none_to_split != 0 {
            partition = PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
            subsize = get_partition_subsize(bsize, partition);
            if !(subsize as core::ffi::c_int <= BLOCK_LARGEST as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_nonrd_use_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    3018 as core::ffi::c_int,
                    b"subsize <= BLOCK_LARGEST\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    (*pc_tree).partitioning = partition;
    let mut current_block_78: u64;
    match partition as core::ffi::c_int {
        0 => {
            if ((*pc_tree).none).is_null() {
                (*pc_tree).none = av1_alloc_pmc(cpi, bsize, &raw mut (*td).shared_coeff_buf);
                if ((*pc_tree).none).is_null() {
                    aom_internal_error(
                        (*xd).error_info as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            } else {
                av1_reset_pmc((*pc_tree).none);
            }
            pick_sb_modes_nonrd(
                cpi,
                tile_data,
                x,
                mi_row,
                mi_col,
                &raw mut dummy_cost,
                bsize,
                (*pc_tree).none,
            );
            encode_b_nonrd(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                0 as RUN_TYPE,
                bsize,
                partition,
                (*pc_tree).none,
                0 as *mut core::ffi::c_int,
            );
            current_block_78 = 4;
        }
        2 => {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < SUB_PARTITIONS_RECT {
                if ((*pc_tree).vertical[i as usize]).is_null() {
                    (*pc_tree).vertical[i as usize] = av1_alloc_pmc(
                        cpi,
                        subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if ((*pc_tree).vertical[i as usize]).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                } else {
                    av1_reset_pmc((*pc_tree).vertical[i as usize]);
                }
                i += 1;
            }
            pick_sb_modes_nonrd(
                cpi,
                tile_data,
                x,
                mi_row,
                mi_col,
                &raw mut dummy_cost,
                subsize,
                (*pc_tree).vertical[0 as core::ffi::c_int as usize],
            );
            encode_b_nonrd(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                0 as RUN_TYPE,
                subsize,
                PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
                (*pc_tree).vertical[0 as core::ffi::c_int as usize],
                0 as *mut core::ffi::c_int,
            );
            if mi_col + hbs < (*mi_params).mi_cols
                && bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int
            {
                pick_sb_modes_nonrd(
                    cpi,
                    tile_data,
                    x,
                    mi_row,
                    mi_col + hbs,
                    &raw mut dummy_cost,
                    subsize,
                    (*pc_tree).vertical[1 as core::ffi::c_int as usize],
                );
                encode_b_nonrd(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row,
                    mi_col + hbs,
                    0 as RUN_TYPE,
                    subsize,
                    PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
                    (*pc_tree).vertical[1 as core::ffi::c_int as usize],
                    0 as *mut core::ffi::c_int,
                );
            }
            current_block_78 = 4;
        }
        1 => {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < SUB_PARTITIONS_RECT {
                if ((*pc_tree).horizontal[i_0 as usize]).is_null() {
                    (*pc_tree).horizontal[i_0 as usize] = av1_alloc_pmc(
                        cpi,
                        subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if ((*pc_tree).horizontal[i_0 as usize]).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                } else {
                    av1_reset_pmc((*pc_tree).horizontal[i_0 as usize]);
                }
                i_0 += 1;
            }
            pick_sb_modes_nonrd(
                cpi,
                tile_data,
                x,
                mi_row,
                mi_col,
                &raw mut dummy_cost,
                subsize,
                (*pc_tree).horizontal[0 as core::ffi::c_int as usize],
            );
            encode_b_nonrd(
                cpi,
                tile_data,
                td,
                tp,
                mi_row,
                mi_col,
                0 as RUN_TYPE,
                subsize,
                PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
                (*pc_tree).horizontal[0 as core::ffi::c_int as usize],
                0 as *mut core::ffi::c_int,
            );
            if mi_row + hbs < (*mi_params).mi_rows
                && bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int
            {
                pick_sb_modes_nonrd(
                    cpi,
                    tile_data,
                    x,
                    mi_row + hbs,
                    mi_col,
                    &raw mut dummy_cost,
                    subsize,
                    (*pc_tree).horizontal[1 as core::ffi::c_int as usize],
                );
                encode_b_nonrd(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    mi_row + hbs,
                    mi_col,
                    0 as RUN_TYPE,
                    subsize,
                    PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
                    (*pc_tree).horizontal[1 as core::ffi::c_int as usize],
                    0 as *mut core::ffi::c_int,
                );
            }
            current_block_78 = 4;
        }
        3 => {
            let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_1 < SUB_PARTITIONS_SPLIT {
                if ((*pc_tree).split[i_1 as usize]).is_null() {
                    (*pc_tree).split[i_1 as usize] = av1_alloc_pc_tree_node(subsize)
                        as *mut PC_TREE;
                    if ((*pc_tree).split[i_1 as usize]).is_null() {
                        aom_internal_error(
                            (*xd).error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PC_TREE\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                }
                (*(*pc_tree).split[i_1 as usize]).index = i_1;
                i_1 += 1;
            }
            if (*cpi).sf.rt_sf.nonrd_check_partition_merge_mode != 0
                && av1_is_leaf_split_partition(cm, mi_row, mi_col, bsize) != 0
                && frame_is_intra_only(cm) == 0
                && bsize as core::ffi::c_int <= BLOCK_64X64 as core::ffi::c_int
            {
                try_merge(
                    cpi,
                    td,
                    tile_data,
                    mib,
                    tp,
                    mi_row,
                    mi_col,
                    bsize,
                    pc_tree,
                    partition,
                    subsize,
                    pl,
                );
            } else {
                let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_2 < SUB_PARTITIONS_SPLIT {
                    let mut x_idx: core::ffi::c_int = (i_2 & 1 as core::ffi::c_int)
                        * hbs;
                    let mut y_idx: core::ffi::c_int = (i_2 >> 1 as core::ffi::c_int)
                        * hbs;
                    let mut jj: core::ffi::c_int = i_2 >> 1 as core::ffi::c_int;
                    let mut ii: core::ffi::c_int = i_2 & 0x1 as core::ffi::c_int;
                    if !(mi_row + y_idx >= (*mi_params).mi_rows
                        || mi_col + x_idx >= (*mi_params).mi_cols)
                    {
                        av1_nonrd_use_partition(
                            cpi,
                            td,
                            tile_data,
                            mib
                                .offset((jj * hbs * (*mi_params).mi_stride) as isize)
                                .offset((ii * hbs) as isize),
                            tp,
                            mi_row + y_idx,
                            mi_col + x_idx,
                            subsize,
                            (*pc_tree).split[i_2 as usize],
                        );
                    }
                    i_2 += 1;
                }
                if change_none_to_split == 0 {
                    if frame_is_intra_only(cm) == 0 && (*tile_data).allow_update_cdf == 0
                        && (*cpi).sf.rt_sf.partition_direct_merging != 0
                        && (*mode_costs)
                            .partition_cost[pl
                            as usize][PARTITION_NONE as core::ffi::c_int as usize]
                            < (*mode_costs)
                                .partition_cost[pl
                                as usize][PARTITION_SPLIT as core::ffi::c_int as usize]
                        && mi_row + bs <= (*mi_params).mi_rows
                        && mi_col + bs <= (*mi_params).mi_cols
                    {
                        direct_partition_merging(
                            cpi,
                            td,
                            tile_data,
                            mib,
                            mi_row,
                            mi_col,
                            bsize,
                        );
                    }
                }
            }
            current_block_78 = 4;
        }
        6 | 7 | 4 | 5 | 8 | 9 => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Cannot handle extended partition types\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_nonrd_use_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    3135 as core::ffi::c_int,
                    b"0 && \"Cannot handle extended partition types\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            current_block_78 = 5;
        }
        _ => {
            current_block_78 = 5;
        }
    }
    match current_block_78 {
        5 => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_nonrd_use_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    3136 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        _ => {}
    };
}
unsafe extern "C-unwind" fn rd_try_subblock(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut is_last: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut subsize: BLOCK_SIZE,
    mut best_rdcost: RD_STATS,
    mut sum_rdc: *mut RD_STATS,
    mut partition: PARTITION_TYPE,
    mut this_ctx: *mut PICK_MODE_CONTEXT,
) -> core::ffi::c_int {
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let orig_mult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(
        cpi,
        x,
        mi_row,
        mi_col,
        subsize,
        NO_AQ as core::ffi::c_int as AQ_MODE,
        0 as *mut MB_MODE_INFO,
    );
    av1_rd_cost_update((*x).rdmult, &raw mut best_rdcost);
    let mut rdcost_remaining: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_rd_stats_subtraction(
        (*x).rdmult,
        &raw mut best_rdcost,
        sum_rdc,
        &raw mut rdcost_remaining,
    );
    let mut this_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    pick_sb_modes(
        cpi,
        tile_data,
        x,
        mi_row,
        mi_col,
        &raw mut this_rdc,
        partition,
        subsize,
        this_ctx,
        rdcost_remaining,
    );
    if this_rdc.rate == INT_MAX {
        (*sum_rdc).rdcost = INT64_MAX as int64_t;
    } else {
        (*sum_rdc).rate += this_rdc.rate;
        (*sum_rdc).dist += this_rdc.dist;
        av1_rd_cost_update((*x).rdmult, sum_rdc);
    }
    if (*sum_rdc).rdcost >= best_rdcost.rdcost {
        (*x).rdmult = orig_mult;
        return 0 as core::ffi::c_int;
    }
    if is_last == 0 {
        av1_update_state(cpi, td, this_ctx, mi_row, mi_col, subsize, 1 as RUN_TYPE);
        encode_superblock(
            cpi,
            tile_data,
            td,
            tp,
            DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
            subsize,
            0 as *mut core::ffi::c_int,
        );
    }
    (*x).rdmult = orig_mult;
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn rd_test_partition3(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut pc_tree: *mut PC_TREE,
    mut best_rdc: *mut RD_STATS,
    mut this_rdcost: *mut int64_t,
    mut ctxs: *mut *mut PICK_MODE_CONTEXT,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut partition: PARTITION_TYPE,
    mut ab_subsize: *const BLOCK_SIZE,
    mut ab_mi_pos: *const [core::ffi::c_int; 2],
    mut mode_cache: *mut *const MB_MODE_INFO,
) -> bool {
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *const MACROBLOCKD = &raw mut (*x).e_mbd;
    let pl: core::ffi::c_int = partition_plane_context(xd, mi_row, mi_col, bsize)
        as core::ffi::c_int;
    let mut sum_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_init_rd_stats(&raw mut sum_rdc);
    sum_rdc.rate = (*x).mode_costs.partition_cost[pl as usize][partition as usize];
    sum_rdc.rdcost = (sum_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SUB_PARTITIONS_AB {
        if !mode_cache.is_null() && !(*mode_cache.offset(i as isize)).is_null() {
            (*x).use_mb_mode_cache = 1 as core::ffi::c_int;
            (*x).mb_mode_cache = *mode_cache.offset(i as isize);
        }
        let mode_search_success: core::ffi::c_int = rd_try_subblock(
            cpi,
            td,
            tile_data,
            tp,
            (i == SUB_PARTITIONS_AB - 1 as core::ffi::c_int) as core::ffi::c_int,
            (*ab_mi_pos.offset(i as isize))[0 as core::ffi::c_int as usize],
            (*ab_mi_pos.offset(i as isize))[1 as core::ffi::c_int as usize],
            *ab_subsize.offset(i as isize),
            *best_rdc,
            &raw mut sum_rdc,
            partition,
            *ctxs.offset(i as isize),
        ) as core::ffi::c_int;
        (*x).use_mb_mode_cache = 0 as core::ffi::c_int;
        (*x).mb_mode_cache = 0 as *const MB_MODE_INFO;
        if mode_search_success == 0 {
            return false_0 != 0;
        }
        i += 1;
    }
    av1_rd_cost_update((*x).rdmult, &raw mut sum_rdc);
    *this_rdcost = sum_rdc.rdcost;
    if sum_rdc.rdcost >= (*best_rdc).rdcost {
        return false_0 != 0;
    }
    sum_rdc.rdcost = (sum_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + sum_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    *this_rdcost = sum_rdc.rdcost;
    if sum_rdc.rdcost >= (*best_rdc).rdcost {
        return false_0 != 0;
    }
    *best_rdc = sum_rdc;
    (*pc_tree).partitioning = partition;
    return true_0 != 0;
}
unsafe extern "C-unwind" fn init_partition_search_state_params(
    mut x: *mut MACROBLOCK,
    cpi: *mut AV1_COMP,
    mut part_search_state: *mut PartitionSearchState,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: *mut PartitionBlkParams = &raw mut (*part_search_state)
        .part_blk_params;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cpi).common.mi_params;
    (*blk_params).mi_step = mi_size_wide[bsize as usize] as core::ffi::c_int
        / 2 as core::ffi::c_int;
    (*blk_params).mi_row = mi_row;
    (*blk_params).mi_col = mi_col;
    (*blk_params).mi_row_edge = mi_row + (*blk_params).mi_step;
    (*blk_params).mi_col_edge = mi_col + (*blk_params).mi_step;
    (*blk_params).width = block_size_wide[bsize as usize] as core::ffi::c_int;
    (*blk_params).min_partition_size_1d = block_size_wide[(*x).sb_enc.min_partition_size
        as usize] as core::ffi::c_int;
    (*blk_params).subsize = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    );
    (*blk_params).split_bsize2 = (*blk_params).subsize;
    (*blk_params).bsize_at_least_8x8 = (bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int) as core::ffi::c_int;
    (*blk_params).bsize = bsize;
    (*blk_params).has_rows = ((*blk_params).mi_row_edge < (*mi_params).mi_rows)
        as core::ffi::c_int;
    (*blk_params).has_cols = ((*blk_params).mi_col_edge < (*mi_params).mi_cols)
        as core::ffi::c_int;
    (*part_search_state).intra_part_info = &raw mut (*x).part_search_info;
    if frame_is_intra_only(cm) != 0
        && bsize as core::ffi::c_int == BLOCK_64X64 as core::ffi::c_int
    {
        (*(*part_search_state).intra_part_info).quad_tree_idx = 0 as core::ffi::c_int;
        (*(*part_search_state).intra_part_info).cnn_output_valid = 0 as core::ffi::c_int;
    }
    (*part_search_state).pl_ctx_idx = if (*blk_params).bsize_at_least_8x8 != 0 {
        partition_plane_context(xd, mi_row, mi_col, bsize)
    } else {
        0 as core::ffi::c_int
    };
    let mut mode_costs: *mut ModeCosts = &raw mut (*x).mode_costs;
    (*part_search_state).partition_cost = ((*mode_costs)
        .partition_cost[(*part_search_state).pl_ctx_idx as usize])
        .as_mut_ptr();
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SUB_PARTITIONS_SPLIT {
        (*part_search_state)
            .split_part_rect_win[i as usize]
            .rect_part_win[HORZ as core::ffi::c_int as usize] = true_0;
        (*part_search_state)
            .split_part_rect_win[i as usize]
            .rect_part_win[VERT as core::ffi::c_int as usize] = true_0;
        i += 1;
    }
    av1_init_rd_stats(&raw mut (*part_search_state).this_rdc);
    (*part_search_state).none_rd = 0 as int64_t;
    memset(
        &raw mut (*part_search_state).split_rd as *mut [int64_t; 4]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[int64_t; 4]>() as size_t,
    );
    memset(
        &raw mut (*part_search_state).rect_part_rd as *mut [[int64_t; 2]; 2]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[int64_t; 2]; 2]>() as size_t,
    );
    memset(
        &raw mut (*part_search_state).is_split_ctx_is_ready as *mut [core::ffi::c_int; 2]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t,
    );
    memset(
        &raw mut (*part_search_state).is_rect_ctx_is_ready as *mut [core::ffi::c_int; 2]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t,
    );
    (*part_search_state).ss_x = (*x)
        .e_mbd
        .plane[1 as core::ffi::c_int as usize]
        .subsampling_x;
    (*part_search_state).ss_y = (*x)
        .e_mbd
        .plane[1 as core::ffi::c_int as usize]
        .subsampling_y;
    (*part_search_state).terminate_partition_search = 0 as core::ffi::c_int;
    (*part_search_state).do_square_split = (*blk_params).bsize_at_least_8x8;
    (*part_search_state).do_rectangular_split = ((*cpi)
        .oxcf
        .part_cfg
        .enable_rect_partitions as core::ffi::c_int != 0
        && (*blk_params).bsize_at_least_8x8 != 0) as core::ffi::c_int;
    memset(
        &raw mut (*part_search_state).prune_rect_part as *mut [core::ffi::c_int; 2]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t,
    );
    (*part_search_state).partition_none_allowed = av1_blk_has_rows_and_cols(blk_params)
        as core::ffi::c_int;
    (*part_search_state).partition_rect_allowed[HORZ as core::ffi::c_int as usize] = ((*part_search_state)
        .do_rectangular_split != 0 && (*blk_params).has_cols != 0
        && get_plane_block_size(
            get_partition_subsize(
                bsize,
                PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
            ),
            (*part_search_state).ss_x,
            (*part_search_state).ss_y,
        ) as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int) as core::ffi::c_int;
    (*part_search_state).partition_rect_allowed[VERT as core::ffi::c_int as usize] = ((*part_search_state)
        .do_rectangular_split != 0 && (*blk_params).has_rows != 0
        && get_plane_block_size(
            get_partition_subsize(
                bsize,
                PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
            ),
            (*part_search_state).ss_x,
            (*part_search_state).ss_y,
        ) as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int) as core::ffi::c_int;
    (*part_search_state).found_best_partition = false_0 != 0;
}
unsafe extern "C-unwind" fn set_partition_cost_for_edge_blk(
    mut cm: *const AV1_COMMON,
    mut part_search_state: *mut PartitionSearchState,
) {
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    if !(blk_params.bsize_at_least_8x8 != 0
        && (*part_search_state).pl_ctx_idx >= 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"set_partition_cost_for_edge_blk\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            3423 as core::ffi::c_int,
            b"blk_params.bsize_at_least_8x8 && part_search_state->pl_ctx_idx >= 0\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut partition_cdf: *const aom_cdf_prob = ((*(*cm).fc)
        .partition_cdf[(*part_search_state).pl_ctx_idx as usize])
        .as_mut_ptr();
    let max_cost: core::ffi::c_int = av1_cost_symbol(0 as aom_cdf_prob)
        as core::ffi::c_int;
    let mut i: PARTITION_TYPE = 0 as PARTITION_TYPE;
    while (i as core::ffi::c_int) < PARTITION_TYPES as core::ffi::c_int {
        (*part_search_state).tmp_partition_cost[i as usize] = max_cost;
        i = i.wrapping_add(1);
    }
    if blk_params.has_cols != 0 {
        let mut bot_cdf: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_vert_alike(
            bot_cdf.as_mut_ptr(),
            partition_cdf,
            blk_params.bsize,
        );
        static mut bot_inv_map: [core::ffi::c_int; 2] = [
            PARTITION_HORZ as core::ffi::c_int,
            PARTITION_SPLIT as core::ffi::c_int,
        ];
        av1_cost_tokens_from_cdf(
            ((*part_search_state).tmp_partition_cost).as_mut_ptr(),
            bot_cdf.as_mut_ptr(),
            core::ptr::addr_of!(bot_inv_map).cast::<core::ffi::c_int>(),
        );
    } else if blk_params.has_rows != 0 {
        let mut rhs_cdf: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_horz_alike(
            rhs_cdf.as_mut_ptr(),
            partition_cdf,
            blk_params.bsize,
        );
        static mut rhs_inv_map: [core::ffi::c_int; 2] = [
            PARTITION_VERT as core::ffi::c_int,
            PARTITION_SPLIT as core::ffi::c_int,
        ];
        av1_cost_tokens_from_cdf(
            ((*part_search_state).tmp_partition_cost).as_mut_ptr(),
            rhs_cdf.as_mut_ptr(),
            core::ptr::addr_of!(rhs_inv_map).cast::<core::ffi::c_int>(),
        );
    } else {
        (*part_search_state)
            .tmp_partition_cost[PARTITION_SPLIT as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
    }
    (*part_search_state).partition_cost = ((*part_search_state).tmp_partition_cost)
        .as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn reset_part_limitations(
    cpi: *mut AV1_COMP,
    mut part_search_state: *mut PartitionSearchState,
) {
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let is_rect_part_allowed_0: core::ffi::c_int = (blk_params.bsize_at_least_8x8 != 0
        && (*cpi).oxcf.part_cfg.enable_rect_partitions as core::ffi::c_int != 0
        && blk_params.width > blk_params.min_partition_size_1d) as core::ffi::c_int;
    (*part_search_state).do_square_split = (blk_params.bsize_at_least_8x8 != 0
        && blk_params.width > blk_params.min_partition_size_1d) as core::ffi::c_int;
    (*part_search_state).partition_none_allowed = (av1_blk_has_rows_and_cols(
        &raw mut blk_params,
    ) as core::ffi::c_int != 0 && blk_params.width >= blk_params.min_partition_size_1d)
        as core::ffi::c_int;
    (*part_search_state).partition_rect_allowed[HORZ as core::ffi::c_int as usize] = (blk_params
        .has_cols != 0 && is_rect_part_allowed_0 != 0
        && get_plane_block_size(
            get_partition_subsize(
                blk_params.bsize,
                PARTITION_HORZ as core::ffi::c_int as PARTITION_TYPE,
            ),
            (*part_search_state).ss_x,
            (*part_search_state).ss_y,
        ) as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int) as core::ffi::c_int;
    (*part_search_state).partition_rect_allowed[VERT as core::ffi::c_int as usize] = (blk_params
        .has_rows != 0 && is_rect_part_allowed_0 != 0
        && get_plane_block_size(
            get_partition_subsize(
                blk_params.bsize,
                PARTITION_VERT as core::ffi::c_int as PARTITION_TYPE,
            ),
            (*part_search_state).ss_x,
            (*part_search_state).ss_y,
        ) as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int) as core::ffi::c_int;
    (*part_search_state).terminate_partition_search = 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn rd_pick_rect_partition(
    cpi: *mut AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    mut x: *mut MACROBLOCK,
    mut cur_partition_ctx: *mut PICK_MODE_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    idx: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut partition_type: PARTITION_TYPE,
) {
    let mut best_remain_rdcost: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_rd_stats_subtraction(
        (*x).rdmult,
        best_rdc,
        &raw mut (*part_search_state).sum_rdc,
        &raw mut best_remain_rdcost,
    );
    pick_sb_modes(
        cpi,
        tile_data,
        x,
        mi_row,
        mi_col,
        &raw mut (*part_search_state).this_rdc,
        partition_type,
        bsize,
        cur_partition_ctx,
        best_remain_rdcost,
    );
    av1_rd_cost_update((*x).rdmult, &raw mut (*part_search_state).this_rdc);
    if (*part_search_state).this_rdc.rate == INT_MAX {
        (*part_search_state).sum_rdc.rdcost = INT64_MAX as int64_t;
    } else {
        (*part_search_state).sum_rdc.rate += (*part_search_state).this_rdc.rate;
        (*part_search_state).sum_rdc.dist += (*part_search_state).this_rdc.dist;
        av1_rd_cost_update((*x).rdmult, &raw mut (*part_search_state).sum_rdc);
    }
    let rect_part: RECT_PART_TYPE = (if partition_type as core::ffi::c_int
        == PARTITION_HORZ as core::ffi::c_int
    {
        HORZ as core::ffi::c_int
    } else {
        VERT as core::ffi::c_int
    }) as RECT_PART_TYPE;
    (*part_search_state).rect_part_rd[rect_part as usize][idx as usize] = (*part_search_state)
        .this_rdc
        .rdcost;
}
#[inline]
unsafe extern "C-unwind" fn is_rect_part_allowed(
    mut cpi: *const AV1_COMP,
    mut part_search_state: *const PartitionSearchState,
    mut active_edge: *const active_edge_info,
    mut rect_part: RECT_PART_TYPE,
    mi_pos: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut blk_params: *const PartitionBlkParams = &(*part_search_state)
        .part_blk_params;
    let is_part_allowed: core::ffi::c_int = ((*part_search_state)
        .terminate_partition_search == 0
        && (*part_search_state).partition_rect_allowed[rect_part as usize] != 0
        && (*part_search_state).prune_rect_part[rect_part as usize] == 0
        && ((*part_search_state).do_rectangular_split != 0
            || (*active_edge.offset(rect_part as isize))
                .expect("non-null function pointer")(cpi, mi_pos, (*blk_params).mi_step)
                != 0)) as core::ffi::c_int;
    return is_part_allowed;
}
unsafe extern "C-unwind" fn rectangular_partition_search(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut x: *mut MACROBLOCK,
    mut pc_tree: *mut PC_TREE,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut rect_part_win_info: *mut RD_RECT_PART_WIN_INFO,
    start_type: RECT_PART_TYPE,
    end_type: RECT_PART_TYPE,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mut sum_rdc: *mut RD_STATS = &raw mut (*part_search_state).sum_rdc;
    let rect_partition_type: [core::ffi::c_int; 2] = [
        PARTITION_HORZ as core::ffi::c_int,
        PARTITION_VERT as core::ffi::c_int,
    ];
    let mi_pos_rect: [[[core::ffi::c_int; 2]; 2]; 2] = [
        [
            [blk_params.mi_row, blk_params.mi_col],
            [blk_params.mi_row_edge, blk_params.mi_col],
        ],
        [
            [blk_params.mi_row, blk_params.mi_col],
            [blk_params.mi_row, blk_params.mi_col_edge],
        ],
    ];
    let mut active_edge_type: [active_edge_info; 2] = [
        Some(
            av1_active_h_edge
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMP,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            av1_active_v_edge
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMP,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
    ];
    let is_not_edge_block: [core::ffi::c_int; 2] = [
        blk_params.has_rows,
        blk_params.has_cols,
    ];
    let mut cur_ctx: [[*mut *mut PICK_MODE_CONTEXT; 2]; 2] = [
        [
            &raw mut *((*pc_tree).horizontal)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT,
            &raw mut *((*pc_tree).horizontal)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT,
        ],
        [
            &raw mut *((*pc_tree).vertical)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT,
            &raw mut *((*pc_tree).vertical)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT,
        ],
    ];
    let mut i: RECT_PART_TYPE = start_type;
    while i as core::ffi::c_int <= end_type as core::ffi::c_int {
        if !((*cpi).oxcf.part_cfg.enable_rect_partitions
            || (*part_search_state).partition_rect_allowed[i as usize] == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"rectangular_partition_search\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                3571 as core::ffi::c_int,
                b"IMPLIES(!cpi->oxcf.part_cfg.enable_rect_partitions, !part_search_state->partition_rect_allowed[i])\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(is_rect_part_allowed(
            cpi,
            part_search_state,
            active_edge_type.as_mut_ptr(),
            i,
            mi_pos_rect[i as usize][0 as core::ffi::c_int as usize][i as usize],
        ) == 0)
        {
            let mut sub_part_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut partition_type: PARTITION_TYPE = rect_partition_type[i as usize]
                as PARTITION_TYPE;
            blk_params.subsize = get_partition_subsize(blk_params.bsize, partition_type);
            if !(blk_params.subsize as core::ffi::c_int
                <= BLOCK_LARGEST as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"rectangular_partition_search\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    3583 as core::ffi::c_int,
                    b"blk_params.subsize <= BLOCK_LARGEST\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            av1_init_rd_stats(sum_rdc);
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < SUB_PARTITIONS_RECT {
                if (*(cur_ctx[i as usize][j as usize])
                    .offset(0 as core::ffi::c_int as isize))
                    .is_null()
                {
                    let fresh43 = &raw mut *(cur_ctx[i as usize][j as usize])
                        .offset(0 as core::ffi::c_int as isize);
                    *fresh43 = av1_alloc_pmc(
                        cpi,
                        blk_params.subsize,
                        &raw mut (*td).shared_coeff_buf,
                    );
                    if (*(cur_ctx[i as usize][j as usize])
                        .offset(0 as core::ffi::c_int as isize))
                        .is_null()
                    {
                        aom_internal_error(
                            (*x).e_mbd.error_info as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                }
                j += 1;
            }
            (*sum_rdc).rate = *((*part_search_state).partition_cost)
                .offset(partition_type as isize);
            (*sum_rdc).rdcost = ((*sum_rdc).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS))
                    as int64_t;
            rd_pick_rect_partition(
                cpi,
                tile_data,
                x,
                *(cur_ctx[i as usize][sub_part_idx as usize])
                    .offset(0 as core::ffi::c_int as isize),
                part_search_state,
                best_rdc,
                0 as core::ffi::c_int,
                mi_pos_rect[i
                    as usize][sub_part_idx as usize][0 as core::ffi::c_int as usize],
                mi_pos_rect[i
                    as usize][sub_part_idx as usize][1 as core::ffi::c_int as usize],
                blk_params.subsize,
                partition_type,
            );
            if (*sum_rdc).rdcost < (*best_rdc).rdcost
                && is_not_edge_block[i as usize] != 0
            {
                let mbmi: *const MB_MODE_INFO = &raw mut (**(*(*cur_ctx
                    .as_mut_ptr()
                    .offset(i as isize))
                    .as_mut_ptr()
                    .offset(sub_part_idx as isize))
                    .offset(0 as core::ffi::c_int as isize))
                    .mic;
                let pmi: *const PALETTE_MODE_INFO = &(*mbmi).palette_mode_info;
                if (*pmi).palette_size[PLANE_TYPE_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int == 0 as core::ffi::c_int
                    && (*pmi).palette_size[PLANE_TYPE_UV as core::ffi::c_int as usize]
                        as core::ffi::c_int == 0 as core::ffi::c_int
                {
                    if (*mbmi).uv_mode as core::ffi::c_int
                        != UV_CFL_PRED as core::ffi::c_int
                    {
                        (*part_search_state).is_rect_ctx_is_ready[i as usize] = 1
                            as core::ffi::c_int;
                    }
                }
                av1_update_state(
                    cpi,
                    td,
                    *(cur_ctx[i as usize][sub_part_idx as usize])
                        .offset(0 as core::ffi::c_int as isize),
                    blk_params.mi_row,
                    blk_params.mi_col,
                    blk_params.subsize,
                    DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                );
                encode_superblock(
                    cpi,
                    tile_data,
                    td,
                    tp,
                    DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                    blk_params.subsize,
                    0 as *mut core::ffi::c_int,
                );
                sub_part_idx = 1 as core::ffi::c_int;
                rd_pick_rect_partition(
                    cpi,
                    tile_data,
                    x,
                    *(cur_ctx[i as usize][sub_part_idx as usize])
                        .offset(0 as core::ffi::c_int as isize),
                    part_search_state,
                    best_rdc,
                    1 as core::ffi::c_int,
                    mi_pos_rect[i
                        as usize][sub_part_idx as usize][0 as core::ffi::c_int as usize],
                    mi_pos_rect[i
                        as usize][sub_part_idx as usize][1 as core::ffi::c_int as usize],
                    blk_params.subsize,
                    partition_type,
                );
            }
            if (*sum_rdc).rdcost < (*best_rdc).rdcost {
                (*sum_rdc).rdcost = ((*sum_rdc).rate as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*sum_rdc).dist
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                if (*sum_rdc).rdcost < (*best_rdc).rdcost {
                    *best_rdc = *sum_rdc;
                    (*part_search_state).found_best_partition = true_0 != 0;
                    (*pc_tree).partitioning = partition_type;
                }
            } else if !rect_part_win_info.is_null() {
                (*rect_part_win_info).rect_part_win[i as usize] = false_0;
            }
            av1_restore_context(
                x,
                x_ctx,
                blk_params.mi_row,
                blk_params.mi_col,
                blk_params.bsize,
                av1_num_planes(cm),
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn rd_pick_ab_part(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut x: *mut MACROBLOCK,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut pc_tree: *mut PC_TREE,
    mut dst_ctxs: *mut *mut PICK_MODE_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut ab_subsize: *const BLOCK_SIZE,
    mut ab_mi_pos: *const [core::ffi::c_int; 2],
    part_type: PARTITION_TYPE,
    mut mode_cache: *mut *const MB_MODE_INFO,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mi_row: core::ffi::c_int = blk_params.mi_row;
    let mi_col: core::ffi::c_int = blk_params.mi_col;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    let mut this_rdcost: int64_t = 0 as int64_t;
    let find_best_ab_part: bool = rd_test_partition3(
        cpi,
        td,
        tile_data,
        tp,
        pc_tree,
        best_rdc,
        &raw mut this_rdcost,
        dst_ctxs,
        mi_row,
        mi_col,
        bsize,
        part_type,
        ab_subsize,
        ab_mi_pos,
        mode_cache,
    ) as bool;
    (*part_search_state).found_best_partition = ((*part_search_state)
        .found_best_partition as core::ffi::c_int
        | find_best_ab_part as core::ffi::c_int) != 0;
    av1_restore_context(x, x_ctx, mi_row, mi_col, bsize, av1_num_planes(cm));
}
#[inline]
unsafe extern "C-unwind" fn set_mode_search_ctx(
    mut pc_tree: *mut PC_TREE,
    mut is_ctx_ready: *const [core::ffi::c_int; 2],
    mut mode_srch_ctx: *mut [*mut *mut PICK_MODE_CONTEXT; 2],
) {
    let fresh38 = &raw mut (*mode_srch_ctx
        .offset(HORZ_B as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize];
    *fresh38 = &raw mut *((*pc_tree).horizontal)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT;
    let fresh39 = &raw mut (*mode_srch_ctx
        .offset(VERT_B as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize];
    *fresh39 = &raw mut *((*pc_tree).vertical)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut *mut PICK_MODE_CONTEXT;
    if (*is_ctx_ready
        .offset(HORZ_A as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        != 0
    {
        let fresh40 = &raw mut (*mode_srch_ctx
            .offset(
                HORZ_A as core::ffi::c_int as isize,
            ))[0 as core::ffi::c_int as usize];
        *fresh40 = &raw mut (**((*pc_tree).split)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .none;
    }
    if (*is_ctx_ready
        .offset(VERT_A as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        != 0
    {
        let fresh41 = &raw mut (*mode_srch_ctx
            .offset(
                VERT_A as core::ffi::c_int as isize,
            ))[0 as core::ffi::c_int as usize];
        *fresh41 = &raw mut (**((*pc_tree).split)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .none;
    }
    if (*is_ctx_ready
        .offset(HORZ_A as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        != 0
    {
        let fresh42 = &raw mut (*mode_srch_ctx
            .offset(
                HORZ_A as core::ffi::c_int as isize,
            ))[1 as core::ffi::c_int as usize];
        *fresh42 = &raw mut (**((*pc_tree).split)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize))
            .none;
    }
}
#[inline]
unsafe extern "C-unwind" fn copy_partition_mode_from_mode_context(
    mut dst_mode: *mut *const MB_MODE_INFO,
    mut ctx: *const PICK_MODE_CONTEXT,
) {
    if !ctx.is_null() && (*ctx).rd_stats.rate < INT_MAX {
        *dst_mode = &(*ctx).mic;
    } else {
        *dst_mode = 0 as *const MB_MODE_INFO;
    };
}
#[inline]
unsafe extern "C-unwind" fn copy_partition_mode_from_pc_tree(
    mut dst_mode: *mut *const MB_MODE_INFO,
    mut pc_tree: *const PC_TREE,
) {
    if !pc_tree.is_null() {
        copy_partition_mode_from_mode_context(dst_mode, (*pc_tree).none);
    } else {
        *dst_mode = 0 as *const MB_MODE_INFO;
    };
}
#[inline]
unsafe extern "C-unwind" fn set_mode_cache_for_partition_ab(
    mut mode_cache: *mut *const MB_MODE_INFO,
    mut pc_tree: *const PC_TREE,
    mut ab_part_type: AB_PART_TYPE,
) {
    match ab_part_type as core::ffi::c_int {
        0 => {
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(0 as core::ffi::c_int as isize),
                (*pc_tree).split[0 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(1 as core::ffi::c_int as isize),
                (*pc_tree).split[1 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_mode_context(
                &raw mut *mode_cache.offset(2 as core::ffi::c_int as isize),
                (*pc_tree).horizontal[1 as core::ffi::c_int as usize],
            );
        }
        1 => {
            copy_partition_mode_from_mode_context(
                &raw mut *mode_cache.offset(0 as core::ffi::c_int as isize),
                (*pc_tree).horizontal[0 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(1 as core::ffi::c_int as isize),
                (*pc_tree).split[2 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(2 as core::ffi::c_int as isize),
                (*pc_tree).split[3 as core::ffi::c_int as usize],
            );
        }
        2 => {
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(0 as core::ffi::c_int as isize),
                (*pc_tree).split[0 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(1 as core::ffi::c_int as isize),
                (*pc_tree).split[2 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_mode_context(
                &raw mut *mode_cache.offset(2 as core::ffi::c_int as isize),
                (*pc_tree).vertical[1 as core::ffi::c_int as usize],
            );
        }
        3 => {
            copy_partition_mode_from_mode_context(
                &raw mut *mode_cache.offset(0 as core::ffi::c_int as isize),
                (*pc_tree).vertical[0 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(1 as core::ffi::c_int as isize),
                (*pc_tree).split[1 as core::ffi::c_int as usize],
            );
            copy_partition_mode_from_pc_tree(
                &raw mut *mode_cache.offset(2 as core::ffi::c_int as isize),
                (*pc_tree).split[3 as core::ffi::c_int as usize],
            );
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid ab partition type!\n\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 32],
                        [core::ffi::c_char; 32],
                    >(*b"set_mode_cache_for_partition_ab\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    3766 as core::ffi::c_int,
                    b"0 && \"Invalid ab partition type!\\n\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    };
}
unsafe extern "C-unwind" fn ab_partitions_search(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut x: *mut MACROBLOCK,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut pc_tree: *mut PC_TREE,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut rect_part_win_info: *mut RD_RECT_PART_WIN_INFO,
    mut pb_source_variance: core::ffi::c_int,
    mut ext_partition_allowed: core::ffi::c_int,
    start_type: AB_PART_TYPE,
    end_type: AB_PART_TYPE,
) {
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mi_row: core::ffi::c_int = blk_params.mi_row;
    let mi_col: core::ffi::c_int = blk_params.mi_col;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if (*part_search_state).terminate_partition_search != 0 {
        return;
    }
    let mut ab_partitions_allowed: [core::ffi::c_int; 4] = [0; 4];
    av1_prune_ab_partitions(
        cpi,
        x,
        pc_tree,
        pb_source_variance,
        (*best_rdc).rdcost,
        rect_part_win_info,
        ext_partition_allowed != 0,
        part_search_state,
        ab_partitions_allowed.as_mut_ptr(),
    );
    let is_ctx_ready: [[core::ffi::c_int; 2]; 4] = [
        [
            (*part_search_state).is_split_ctx_is_ready[0 as core::ffi::c_int as usize],
            (*part_search_state).is_split_ctx_is_ready[1 as core::ffi::c_int as usize],
        ],
        [
            (*part_search_state).is_rect_ctx_is_ready[HORZ as core::ffi::c_int as usize],
            0 as core::ffi::c_int,
        ],
        [
            (*part_search_state).is_split_ctx_is_ready[0 as core::ffi::c_int as usize],
            0 as core::ffi::c_int,
        ],
        [
            (*part_search_state).is_rect_ctx_is_ready[VERT as core::ffi::c_int as usize],
            0 as core::ffi::c_int,
        ],
    ];
    let mut cur_part_ctxs: [*mut *mut PICK_MODE_CONTEXT; 4] = [
        ((*pc_tree).horizontala).as_mut_ptr(),
        ((*pc_tree).horizontalb).as_mut_ptr(),
        ((*pc_tree).verticala).as_mut_ptr(),
        ((*pc_tree).verticalb).as_mut_ptr(),
    ];
    let mut mode_srch_ctx: [[*mut *mut PICK_MODE_CONTEXT; 2]; 4] = [[0
        as *mut *mut PICK_MODE_CONTEXT; 2]; 4];
    set_mode_search_ctx(pc_tree, is_ctx_ready.as_ptr(), mode_srch_ctx.as_mut_ptr());
    let ab_subsize: [[BLOCK_SIZE; 3]; 4] = [
        [
            blk_params.split_bsize2,
            blk_params.split_bsize2,
            get_partition_subsize(
                bsize,
                PARTITION_HORZ_A as core::ffi::c_int as PARTITION_TYPE,
            ),
        ],
        [
            get_partition_subsize(
                bsize,
                PARTITION_HORZ_B as core::ffi::c_int as PARTITION_TYPE,
            ),
            blk_params.split_bsize2,
            blk_params.split_bsize2,
        ],
        [
            blk_params.split_bsize2,
            blk_params.split_bsize2,
            get_partition_subsize(
                bsize,
                PARTITION_VERT_A as core::ffi::c_int as PARTITION_TYPE,
            ),
        ],
        [
            get_partition_subsize(
                bsize,
                PARTITION_VERT_B as core::ffi::c_int as PARTITION_TYPE,
            ),
            blk_params.split_bsize2,
            blk_params.split_bsize2,
        ],
    ];
    let ab_mi_pos: [[[core::ffi::c_int; 2]; 3]; 4] = [
        [
            [mi_row, mi_col],
            [mi_row, blk_params.mi_col_edge],
            [blk_params.mi_row_edge, mi_col],
        ],
        [
            [mi_row, mi_col],
            [blk_params.mi_row_edge, mi_col],
            [blk_params.mi_row_edge, blk_params.mi_col_edge],
        ],
        [
            [mi_row, mi_col],
            [blk_params.mi_row_edge, mi_col],
            [mi_row, blk_params.mi_col_edge],
        ],
        [
            [mi_row, mi_col],
            [mi_row, blk_params.mi_col_edge],
            [blk_params.mi_row_edge, blk_params.mi_col_edge],
        ],
    ];
    let mut ab_part_type: AB_PART_TYPE = start_type;
    while ab_part_type as core::ffi::c_int <= end_type as core::ffi::c_int {
        let part_type: PARTITION_TYPE = (ab_part_type as core::ffi::c_int
            + PARTITION_HORZ_A as core::ffi::c_int) as PARTITION_TYPE;
        if !(ab_partitions_allowed[ab_part_type as usize] == 0) {
            blk_params.subsize = get_partition_subsize(bsize, part_type);
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < SUB_PARTITIONS_AB {
                let fresh37 = &raw mut *(cur_part_ctxs[ab_part_type as usize])
                    .offset(i as isize);
                *fresh37 = av1_alloc_pmc(
                    cpi,
                    ab_subsize[ab_part_type as usize][i as usize],
                    &raw mut (*td).shared_coeff_buf,
                );
                if (*(cur_part_ctxs[ab_part_type as usize]).offset(i as isize)).is_null()
                {
                    aom_internal_error(
                        (*x).e_mbd.error_info as *mut aom_internal_error_info,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                (**(cur_part_ctxs[ab_part_type as usize]).offset(i as isize))
                    .rd_mode_is_ready = 0 as core::ffi::c_int;
                i += 1;
            }
            if (*cpi).sf.part_sf.reuse_prev_rd_results_for_part_ab != 0 {
                if is_ctx_ready[ab_part_type as usize][0 as core::ffi::c_int as usize]
                    != 0
                {
                    av1_copy_tree_context(
                        *(cur_part_ctxs[ab_part_type as usize])
                            .offset(0 as core::ffi::c_int as isize),
                        *(mode_srch_ctx[ab_part_type
                            as usize][0 as core::ffi::c_int as usize])
                            .offset(0 as core::ffi::c_int as isize),
                    );
                    (**(cur_part_ctxs[ab_part_type as usize])
                        .offset(0 as core::ffi::c_int as isize))
                        .mic
                        .partition = part_type;
                    (**(cur_part_ctxs[ab_part_type as usize])
                        .offset(0 as core::ffi::c_int as isize))
                        .rd_mode_is_ready = 1 as core::ffi::c_int;
                    if is_ctx_ready[ab_part_type
                        as usize][1 as core::ffi::c_int as usize] != 0
                    {
                        av1_copy_tree_context(
                            *(cur_part_ctxs[ab_part_type as usize])
                                .offset(1 as core::ffi::c_int as isize),
                            *(mode_srch_ctx[ab_part_type
                                as usize][1 as core::ffi::c_int as usize])
                                .offset(0 as core::ffi::c_int as isize),
                        );
                        (**(cur_part_ctxs[ab_part_type as usize])
                            .offset(1 as core::ffi::c_int as isize))
                            .mic
                            .partition = part_type;
                        (**(cur_part_ctxs[ab_part_type as usize])
                            .offset(1 as core::ffi::c_int as isize))
                            .rd_mode_is_ready = 1 as core::ffi::c_int;
                    }
                }
            }
            let mut mode_cache: [*const MB_MODE_INFO; 3] = [
                0 as *const MB_MODE_INFO,
                0 as *const MB_MODE_INFO,
                0 as *const MB_MODE_INFO,
            ];
            if (*cpi).sf.part_sf.reuse_best_prediction_for_part_ab != 0 {
                set_mode_cache_for_partition_ab(
                    mode_cache.as_mut_ptr(),
                    pc_tree,
                    ab_part_type,
                );
            }
            rd_pick_ab_part(
                cpi,
                td,
                tile_data,
                tp,
                x,
                x_ctx,
                pc_tree,
                cur_part_ctxs[ab_part_type as usize],
                part_search_state,
                best_rdc,
                (ab_subsize[ab_part_type as usize]).as_ptr(),
                (ab_mi_pos[ab_part_type as usize]).as_ptr(),
                part_type,
                mode_cache.as_mut_ptr(),
            );
        }
        ab_part_type = ab_part_type.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn set_mi_pos_partition4(
    mut inc_step: *const core::ffi::c_int,
    mut mi_pos: *mut [core::ffi::c_int; 2],
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
) {
    let mut i: PART4_TYPES = 0 as PART4_TYPES;
    while (i as core::ffi::c_int) < SUB_PARTITIONS_PART4 {
        (*mi_pos.offset(i as isize))[0 as core::ffi::c_int as usize] = mi_row
            + i as core::ffi::c_int
                * *inc_step.offset(HORZ4 as core::ffi::c_int as isize);
        (*mi_pos.offset(i as isize))[1 as core::ffi::c_int as usize] = mi_col
            + i as core::ffi::c_int
                * *inc_step.offset(VERT4 as core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn set_4_part_ctx_and_rdcost(
    mut x: *mut MACROBLOCK,
    cpi: *const AV1_COMP,
    mut td: *mut ThreadData,
    mut cur_part_ctx: *mut *mut PICK_MODE_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut partition_type: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) {
    av1_init_rd_stats(&raw mut (*part_search_state).sum_rdc);
    let subsize: core::ffi::c_int = get_partition_subsize(bsize, partition_type)
        as core::ffi::c_int;
    (*part_search_state).sum_rdc.rate = *((*part_search_state).partition_cost)
        .offset(partition_type as isize);
    (*part_search_state).sum_rdc.rdcost = ((*part_search_state).sum_rdc.rate as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let mut i: PART4_TYPES = 0 as PART4_TYPES;
    while (i as core::ffi::c_int) < SUB_PARTITIONS_PART4 {
        let fresh36 = &raw mut *cur_part_ctx.offset(i as isize);
        *fresh36 = av1_alloc_pmc(
            cpi as *const AV1_COMP,
            subsize as BLOCK_SIZE,
            &raw mut (*td).shared_coeff_buf,
        );
        if (*cur_part_ctx.offset(i as isize)).is_null() {
            aom_internal_error(
                (*x).e_mbd.error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn rd_pick_4partition(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut x: *mut MACROBLOCK,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut pc_tree: *mut PC_TREE,
    mut cur_part_ctx: *mut *mut PICK_MODE_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut inc_step: *const core::ffi::c_int,
    mut partition_type: PARTITION_TYPE,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mut mi_pos_check: [core::ffi::c_int; 2] = [
        (*cm).mi_params.mi_rows,
        (*cm).mi_params.mi_cols,
    ];
    let part4_idx: PART4_TYPES = (partition_type as core::ffi::c_int
        != PARTITION_HORZ_4 as core::ffi::c_int) as core::ffi::c_int as PART4_TYPES;
    let mut mi_pos: [[core::ffi::c_int; 2]; 4] = [[0; 2]; 4];
    blk_params.subsize = get_partition_subsize(blk_params.bsize, partition_type);
    set_4_part_ctx_and_rdcost(
        x,
        cpi,
        td,
        cur_part_ctx,
        part_search_state,
        partition_type,
        blk_params.bsize,
    );
    set_mi_pos_partition4(
        inc_step,
        mi_pos.as_mut_ptr(),
        blk_params.mi_row,
        blk_params.mi_col,
    );
    let mut i: PART4_TYPES = 0 as PART4_TYPES;
    while (i as core::ffi::c_int) < SUB_PARTITIONS_PART4 {
        if i as core::ffi::c_int > 0 as core::ffi::c_int
            && mi_pos[i as usize][part4_idx as usize] >= mi_pos_check[part4_idx as usize]
        {
            break;
        }
        (**cur_part_ctx.offset(i as isize)).rd_mode_is_ready = 0 as core::ffi::c_int;
        if rd_try_subblock(
            cpi,
            td,
            tile_data,
            tp,
            (i as core::ffi::c_int == SUB_PARTITIONS_PART4 - 1 as core::ffi::c_int)
                as core::ffi::c_int,
            mi_pos[i as usize][0 as core::ffi::c_int as usize],
            mi_pos[i as usize][1 as core::ffi::c_int as usize],
            blk_params.subsize,
            *best_rdc,
            &raw mut (*part_search_state).sum_rdc,
            partition_type,
            *cur_part_ctx.offset(i as isize),
        ) == 0
        {
            av1_invalid_rd_stats(&raw mut (*part_search_state).sum_rdc);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    av1_rd_cost_update((*x).rdmult, &raw mut (*part_search_state).sum_rdc);
    if (*part_search_state).sum_rdc.rdcost < (*best_rdc).rdcost {
        *best_rdc = (*part_search_state).sum_rdc;
        (*part_search_state).found_best_partition = true_0 != 0;
        (*pc_tree).partitioning = partition_type;
    }
    av1_restore_context(
        x,
        x_ctx,
        blk_params.mi_row,
        blk_params.mi_col,
        blk_params.bsize,
        av1_num_planes(cm),
    );
}
#[inline]
unsafe extern "C-unwind" fn prune_ext_part_none_skippable(
    mut part_none: *mut PICK_MODE_CONTEXT,
    mut must_find_valid_partition: core::ffi::c_int,
    mut skip_non_sq_part_based_on_none: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if skip_non_sq_part_based_on_none >= 1 as core::ffi::c_int && !part_none.is_null() {
        if (*part_none).skippable != 0 && must_find_valid_partition == 0
            && bsize as core::ffi::c_int >= BLOCK_16X16 as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn allow_ab_partition_search(
    mut part_search_state: *mut PartitionSearchState,
    mut part_sf: *mut PARTITION_SPEED_FEATURES,
    mut curr_best_part: PARTITION_TYPE,
    mut must_find_valid_partition: core::ffi::c_int,
    mut prune_ext_part_state: core::ffi::c_int,
    mut best_rdcost: int64_t,
) -> core::ffi::c_int {
    let blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if best_rdcost == INT64_MAX as int64_t {
        return 1 as core::ffi::c_int;
    }
    let mut ab_bsize_thresh: BLOCK_SIZE = (*part_sf).ext_partition_eval_thresh;
    if (*part_sf).ext_part_eval_based_on_cur_best != 0 && must_find_valid_partition == 0
        && !(curr_best_part as core::ffi::c_int == PARTITION_HORZ as core::ffi::c_int
            || curr_best_part as core::ffi::c_int == PARTITION_VERT as core::ffi::c_int)
    {
        ab_bsize_thresh = BLOCK_128X128;
    }
    if !(ab_bsize_thresh as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"allow_ab_partition_search\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4022 as core::ffi::c_int,
            b"ab_bsize_thresh >= BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut ab_partition_allowed: core::ffi::c_int = ((*part_search_state)
        .do_rectangular_split != 0
        && bsize as core::ffi::c_int > ab_bsize_thresh as core::ffi::c_int
        && av1_blk_has_rows_and_cols(&blk_params) as core::ffi::c_int != 0
        && prune_ext_part_state == 0) as core::ffi::c_int;
    return ab_partition_allowed;
}
unsafe extern "C-unwind" fn prune_4_partition_using_split_info(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut part_search_state: *mut PartitionSearchState,
    mut part4_search_allowed: *mut core::ffi::c_int,
) {
    let mut cur_part: [PART4_TYPES; 2] = [
        HORZ4 as core::ffi::c_int as PART4_TYPES,
        VERT4 as core::ffi::c_int as PART4_TYPES,
    ];
    let mut num_child_rect_win: [core::ffi::c_int; 2] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    let num_win_thresh: core::ffi::c_int = if (3 as core::ffi::c_int
        * (255 as core::ffi::c_int - (*x).qindex) / 255 as core::ffi::c_int
        + 1 as core::ffi::c_int) < 3 as core::ffi::c_int
    {
        3 as core::ffi::c_int * (255 as core::ffi::c_int - (*x).qindex)
            / 255 as core::ffi::c_int + 1 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    let mut i: RECT_PART_TYPE = HORZ as core::ffi::c_int as RECT_PART_TYPE;
    while (i as core::ffi::c_int) < NUM_RECT_PARTS as core::ffi::c_int {
        if (*cpi).sf.part_sf.prune_ext_part_using_split_info != 0
            && *part4_search_allowed.offset(cur_part[i as usize] as isize) != 0
        {
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < SUB_PARTITIONS_SPLIT {
                num_child_rect_win[i as usize]
                    += if (*part_search_state)
                        .split_part_rect_win[idx as usize]
                        .rect_part_win[i as usize] != 0
                    {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    };
                idx += 1;
            }
            if num_child_rect_win[i as usize] < num_win_thresh {
                *part4_search_allowed.offset(cur_part[i as usize] as isize) = 0
                    as core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn prune_part4_using_sms(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut part_search_state: *const PartitionSearchState,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut part4_search_allowed: *mut core::ffi::c_int,
) {
    if *part4_search_allowed.offset(HORZ4 as core::ffi::c_int as isize) == 0
        || *part4_search_allowed.offset(VERT4 as core::ffi::c_int as isize) == 0
    {
        return;
    }
    let mut sms_h_part4_sse: [core::ffi::c_uint; 4] = [0; 4];
    let mut sms_v_part4_sse: [core::ffi::c_uint; 4] = [0; 4];
    let subsize_h4: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_HORZ_4 as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    let subsize_v4: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_VERT_4 as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    if !(subsize_h4 as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"prune_part4_using_sms\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4072 as core::ffi::c_int,
            b"subsize_h4 != BLOCK_INVALID\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subsize_v4 as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"prune_part4_using_sms\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4073 as core::ffi::c_int,
            b"subsize_v4 != BLOCK_INVALID\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let h_mi: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let w_mi: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let ref_0: core::ffi::c_int = if (*cpi).rc.is_src_frame_alt_ref != 0 {
        ALTREF_FRAME as core::ffi::c_int as core::ffi::c_int
    } else {
        LAST_FRAME as core::ffi::c_int as core::ffi::c_int
    };
    if (*cpi).ref_frame_flags
        & av1_ref_frame_flag_list[ref_0 as usize] as core::ffi::c_int == 0
    {
        return;
    }
    let mut part4_h_sse_sum: int64_t = 0 as int64_t;
    let mut part4_v_sse_sum: int64_t = 0 as int64_t;
    let mut r_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while r_idx < SUB_PARTITIONS_PART4 {
        let mut part_var: core::ffi::c_uint = 0;
        let sub_mi_row: core::ffi::c_int = mi_row + r_idx * h_mi / 4 as core::ffi::c_int;
        let sub_mi_col: core::ffi::c_int = mi_col;
        av1_simple_motion_search_sse_var(
            cpi as *mut AV1_COMP,
            x,
            sub_mi_row,
            sub_mi_col,
            subsize_h4,
            ref_0,
            (*sms_tree).start_mvs[ref_0 as usize],
            1 as core::ffi::c_int,
            1 as core::ffi::c_int,
            &raw mut *sms_h_part4_sse.as_mut_ptr().offset(r_idx as isize),
            &raw mut part_var,
        );
        part4_h_sse_sum += sms_h_part4_sse[r_idx as usize] as int64_t;
        r_idx += 1;
    }
    let mut r_idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while r_idx_0 < SUB_PARTITIONS_PART4 {
        let mut part_var_0: core::ffi::c_uint = 0;
        let sub_mi_row_0: core::ffi::c_int = mi_row;
        let sub_mi_col_0: core::ffi::c_int = mi_col
            + r_idx_0 * w_mi / 4 as core::ffi::c_int;
        av1_simple_motion_search_sse_var(
            cpi as *mut AV1_COMP,
            x,
            sub_mi_row_0,
            sub_mi_col_0,
            subsize_v4,
            ref_0,
            (*sms_tree).start_mvs[ref_0 as usize],
            1 as core::ffi::c_int,
            1 as core::ffi::c_int,
            &raw mut *sms_v_part4_sse.as_mut_ptr().offset(r_idx_0 as isize),
            &raw mut part_var_0,
        );
        part4_v_sse_sum += sms_v_part4_sse[r_idx_0 as usize] as int64_t;
        r_idx_0 += 1;
    }
    let part4_h_rd: int64_t = (*((*part_search_state).partition_cost)
        .offset(PARTITION_HORZ_4 as core::ffi::c_int as isize) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + part4_h_sse_sum * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    let part4_v_rd: int64_t = (*((*part_search_state).partition_cost)
        .offset(PARTITION_VERT_4 as core::ffi::c_int as isize) as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + part4_v_sse_sum * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    if part4_h_rd > part4_v_rd {
        *part4_search_allowed.offset(HORZ4 as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
    }
    if part4_v_rd > part4_h_rd {
        *part4_search_allowed.offset(VERT4 as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn prune_4_way_partition_search(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut pc_tree: *mut PC_TREE,
    mut part_search_state: *mut PartitionSearchState,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut best_rdc: *mut RD_STATS,
    mut pb_source_variance: core::ffi::c_int,
    mut prune_ext_part_state: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut part4_search_allowed: *mut core::ffi::c_int,
) {
    let blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    let part_cfg: *const PartitionCfg = &raw mut (*cpi).oxcf.part_cfg;
    if (*best_rdc).rdcost == INT64_MAX as int64_t
        && (*part_cfg).enable_1to4_partitions as core::ffi::c_int != 0
        && bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int
    {
        return;
    }
    let mut part4_bsize_thresh: BLOCK_SIZE = (*cpi).sf.part_sf.ext_partition_eval_thresh;
    if (*cpi).sf.part_sf.ext_part_eval_based_on_cur_best != 0
        && (*x).must_find_valid_partition == 0
        && (*pc_tree).partitioning as core::ffi::c_int
            == PARTITION_NONE as core::ffi::c_int
    {
        part4_bsize_thresh = BLOCK_128X128;
    }
    if !(part4_bsize_thresh as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"prune_4_way_partition_search\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4155 as core::ffi::c_int,
            b"part4_bsize_thresh >= BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut partition4_allowed: bool = (*part_search_state).do_rectangular_split != 0
        && bsize as core::ffi::c_int > part4_bsize_thresh as core::ffi::c_int
        && av1_blk_has_rows_and_cols(&blk_params) as core::ffi::c_int != 0
        && prune_ext_part_state == 0;
    if blk_params.width
        < blk_params.min_partition_size_1d << (*cpi).sf.part_sf.prune_part4_search
    {
        *part4_search_allowed.offset(HORZ4 as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
        *part4_search_allowed.offset(VERT4 as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
        return;
    }
    let mut cur_part: [PARTITION_TYPE; 2] = [
        PARTITION_HORZ_4 as core::ffi::c_int as PARTITION_TYPE,
        PARTITION_VERT_4 as core::ffi::c_int as PARTITION_TYPE,
    ];
    partition4_allowed = (partition4_allowed as core::ffi::c_int
        & ((*part_cfg).enable_1to4_partitions as core::ffi::c_int != 0
            && bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int)
            as core::ffi::c_int) != 0;
    let mut i: PART4_TYPES = HORZ4 as core::ffi::c_int as PART4_TYPES;
    while (i as core::ffi::c_int) < NUM_PART4_TYPES as core::ffi::c_int {
        *part4_search_allowed.offset(i as isize) = (partition4_allowed
            as core::ffi::c_int != 0
            && (*part_search_state).partition_rect_allowed[i as usize] != 0
            && get_plane_block_size(
                get_partition_subsize(bsize, cur_part[i as usize]),
                (*part_search_state).ss_x,
                (*part_search_state).ss_y,
            ) as core::ffi::c_int != BLOCK_INVALID as core::ffi::c_int)
            as core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    if (*cpi).sf.part_sf.prune_ext_partition_types_search_level == 2 as core::ffi::c_int
    {
        *part4_search_allowed.offset(HORZ4 as core::ffi::c_int as isize)
            &= ((*pc_tree).partitioning as core::ffi::c_int
                == PARTITION_HORZ as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_HORZ_A as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_HORZ_B as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_SPLIT as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_NONE as core::ffi::c_int) as core::ffi::c_int;
        *part4_search_allowed.offset(VERT4 as core::ffi::c_int as isize)
            &= ((*pc_tree).partitioning as core::ffi::c_int
                == PARTITION_VERT as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_VERT_A as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_VERT_B as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_SPLIT as core::ffi::c_int
                || (*pc_tree).partitioning as core::ffi::c_int
                    == PARTITION_NONE as core::ffi::c_int) as core::ffi::c_int;
    }
    if (*cpi).sf.part_sf.ml_prune_partition != 0
        && partition4_allowed as core::ffi::c_int != 0
        && (*part_search_state).partition_rect_allowed[HORZ as core::ffi::c_int as usize]
            != 0
        && (*part_search_state).partition_rect_allowed[VERT as core::ffi::c_int as usize]
            != 0
    {
        av1_ml_prune_4_partition(
            cpi,
            x,
            (*pc_tree).partitioning as core::ffi::c_int,
            (*best_rdc).rdcost,
            part_search_state,
            part4_search_allowed as *mut core::ffi::c_int,
            pb_source_variance as core::ffi::c_uint,
        );
    }
    prune_4_partition_using_split_info(cpi, x, part_search_state, part4_search_allowed);
    if (*cpi).sf.part_sf.prune_h_or_v_4part_using_sms_info as core::ffi::c_int != 0
        && partition4_allowed as core::ffi::c_int != 0
        && (*best_rdc).rdcost != INT64_MAX as int64_t
        && av1_is_whole_blk_in_frame(
            &raw mut (*part_search_state).part_blk_params,
            &raw mut (*cpi).common.mi_params,
        ) as core::ffi::c_int != 0 && frame_is_intra_only(&raw mut (*cpi).common) == 0
    {
        prune_part4_using_sms(
            cpi,
            x,
            part_search_state,
            sms_tree,
            mi_row,
            mi_col,
            bsize,
            part4_search_allowed as *mut core::ffi::c_int,
        );
    }
}
unsafe extern "C-unwind" fn set_none_partition_params(
    cpi: *const AV1_COMP,
    mut td: *mut ThreadData,
    mut x: *mut MACROBLOCK,
    mut pc_tree: *mut PC_TREE,
    mut part_search_state: *mut PartitionSearchState,
    mut best_remain_rdcost: *mut RD_STATS,
    mut best_rdc: *mut RD_STATS,
    mut pt_cost: *mut core::ffi::c_int,
) {
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mut partition_rdcost: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    if ((*pc_tree).none).is_null() {
        (*pc_tree).none = av1_alloc_pmc(
            cpi as *const AV1_COMP,
            blk_params.bsize,
            &raw mut (*td).shared_coeff_buf,
        );
    }
    if ((*pc_tree).none).is_null() {
        aom_internal_error(
            (*x).e_mbd.error_info as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate PICK_MODE_CONTEXT\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if (*part_search_state).partition_none_allowed != 0 {
        if blk_params.bsize_at_least_8x8 != 0 {
            *pt_cost = if *((*part_search_state).partition_cost)
                .offset(PARTITION_NONE as core::ffi::c_int as isize) < INT_MAX
            {
                *((*part_search_state).partition_cost)
                    .offset(PARTITION_NONE as core::ffi::c_int as isize)
            } else {
                0 as core::ffi::c_int
            };
        }
        av1_init_rd_stats(&raw mut partition_rdcost);
        partition_rdcost.rate = *pt_cost;
        av1_rd_cost_update((*x).rdmult, &raw mut partition_rdcost);
        av1_rd_stats_subtraction(
            (*x).rdmult,
            best_rdc,
            &raw mut partition_rdcost,
            best_remain_rdcost,
        );
    }
}
unsafe extern "C-unwind" fn prune_partitions_after_none(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut ctx_none: *mut PICK_MODE_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut pb_source_variance: *mut core::ffi::c_uint,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mut this_rdc: *mut RD_STATS = &raw mut (*part_search_state).this_rdc;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"prune_partitions_after_none\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4269 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if frame_is_intra_only(cm) == 0
        && ((*part_search_state).do_square_split != 0
            || (*part_search_state).do_rectangular_split != 0)
        && (*x)
            .e_mbd
            .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
            as usize] == 0 && (*ctx_none).skippable != 0
    {
        let use_ml_based_breakout: core::ffi::c_int = (bsize as core::ffi::c_int
            <= (*cpi).sf.part_sf.use_square_partition_only_threshold as core::ffi::c_int
            && bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int
            && (*cpi).sf.part_sf.ml_predict_breakout_level >= 1 as core::ffi::c_int)
            as core::ffi::c_int;
        if use_ml_based_breakout != 0 {
            av1_ml_predict_breakout(
                cpi,
                x,
                this_rdc,
                *pb_source_variance,
                (*xd).bd,
                part_search_state,
            );
        }
        let dist_breakout_thr: int64_t = (*cpi)
            .sf
            .part_sf
            .partition_search_breakout_dist_thr
            >> 2 as core::ffi::c_int * (MAX_SB_SIZE_LOG2 - 2 as core::ffi::c_int)
                - (mi_size_wide_log2[bsize as usize] as core::ffi::c_int
                    + mi_size_high_log2[bsize as usize] as core::ffi::c_int);
        let rate_breakout_thr: core::ffi::c_int = (*cpi)
            .sf
            .part_sf
            .partition_search_breakout_rate_thr
            * num_pels_log2_lookup[bsize as usize] as core::ffi::c_int;
        if (*best_rdc).dist < dist_breakout_thr && (*best_rdc).rate < rate_breakout_thr {
            (*part_search_state).do_square_split = 0 as core::ffi::c_int;
            (*part_search_state).do_rectangular_split = 0 as core::ffi::c_int;
        }
    }
    if (*cpi).sf.part_sf.simple_motion_search_early_term_none != 0
        && (*cm).show_frame != 0 && frame_is_intra_only(cm) == 0
        && bsize as core::ffi::c_int >= BLOCK_16X16 as core::ffi::c_int
        && av1_blk_has_rows_and_cols(&blk_params) as core::ffi::c_int != 0
        && (*this_rdc).rdcost < INT64_MAX as int64_t
        && (*this_rdc).rdcost >= 0 as int64_t && (*this_rdc).rate < INT_MAX
        && (*this_rdc).rate >= 0 as core::ffi::c_int
        && ((*part_search_state).do_square_split != 0
            || (*part_search_state).do_rectangular_split != 0)
    {
        av1_simple_motion_search_early_term_none(
            cpi,
            x,
            sms_tree,
            this_rdc,
            part_search_state,
        );
    }
}
unsafe extern "C-unwind" fn prune_partitions_after_split(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut part_none_rd: int64_t,
    mut part_split_rd: int64_t,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mi_row: core::ffi::c_int = blk_params.mi_row;
    let mi_col: core::ffi::c_int = blk_params.mi_col;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"prune_partitions_after_split\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4329 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*cpi).sf.part_sf.ml_early_term_after_part_split_level != 0
        && frame_is_intra_only(cm) == 0
        && (*part_search_state).terminate_partition_search == 0
        && (*part_search_state).do_rectangular_split != 0
        && ((*part_search_state)
            .partition_rect_allowed[HORZ as core::ffi::c_int as usize] != 0
            || (*part_search_state)
                .partition_rect_allowed[VERT as core::ffi::c_int as usize] != 0)
    {
        av1_ml_early_term_after_split(
            cpi,
            x,
            sms_tree,
            (*best_rdc).rdcost,
            part_none_rd,
            part_split_rd,
            ((*part_search_state).split_rd).as_mut_ptr(),
            part_search_state,
        );
    }
    if (*cpi).sf.part_sf.ml_early_term_after_part_split_level == 0
        && (*cpi).sf.part_sf.ml_prune_partition != 0 && frame_is_intra_only(cm) == 0
        && ((*part_search_state)
            .partition_rect_allowed[HORZ as core::ffi::c_int as usize] != 0
            || (*part_search_state)
                .partition_rect_allowed[VERT as core::ffi::c_int as usize] != 0)
        && !((*part_search_state).prune_rect_part[HORZ as core::ffi::c_int as usize] != 0
            || (*part_search_state).prune_rect_part[VERT as core::ffi::c_int as usize]
                != 0) && (*part_search_state).terminate_partition_search == 0
    {
        av1_setup_src_planes(
            x as *mut macroblock,
            (*cpi).source,
            mi_row,
            mi_col,
            av1_num_planes(cm),
            bsize,
        );
        av1_ml_prune_rect_partition(
            cpi,
            x,
            (*best_rdc).rdcost,
            (*part_search_state).none_rd,
            ((*part_search_state).split_rd).as_mut_ptr(),
            part_search_state,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn is_neighbor_blk_larger_than_cur_blk(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
) -> bool {
    let cur_blk_area: core::ffi::c_int = block_size_high[bsize as usize]
        as core::ffi::c_int * block_size_wide[bsize as usize] as core::ffi::c_int;
    if (*xd).left_available {
        let left_bsize: BLOCK_SIZE = (*(*xd).left_mbmi).bsize;
        if block_size_high[left_bsize as usize] as core::ffi::c_int
            * block_size_wide[left_bsize as usize] as core::ffi::c_int > cur_blk_area
        {
            return true_0 != 0;
        }
    }
    if (*xd).up_available {
        let above_bsize: BLOCK_SIZE = (*(*xd).above_mbmi).bsize;
        if block_size_high[above_bsize as usize] as core::ffi::c_int
            * block_size_wide[above_bsize as usize] as core::ffi::c_int > cur_blk_area
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn prune_rect_part_using_none_pred_mode(
    mut xd: *const MACROBLOCKD,
    mut part_state: *mut PartitionSearchState,
    mut mode: PREDICTION_MODE,
    mut bsize: BLOCK_SIZE,
) {
    if mode as core::ffi::c_int == DC_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_PRED as core::ffi::c_int
    {
        if is_neighbor_blk_larger_than_cur_blk(xd, bsize) {
            (*part_state).prune_rect_part[HORZ as core::ffi::c_int as usize] = 1
                as core::ffi::c_int;
            (*part_state).prune_rect_part[VERT as core::ffi::c_int as usize] = 1
                as core::ffi::c_int;
        }
    } else if mode as core::ffi::c_int == D67_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == V_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == D113_PRED as core::ffi::c_int
    {
        (*part_state).prune_rect_part[HORZ as core::ffi::c_int as usize] = 1
            as core::ffi::c_int;
    } else if mode as core::ffi::c_int == D157_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == H_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == D203_PRED as core::ffi::c_int
    {
        (*part_state).prune_rect_part[VERT as core::ffi::c_int as usize] = 1
            as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn none_partition_search(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut x: *mut MACROBLOCK,
    mut pc_tree: *mut PC_TREE,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut pb_source_variance: *mut core::ffi::c_uint,
    mut none_rd: *mut int64_t,
    mut part_none_rd: *mut int64_t,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mut this_rdc: *mut RD_STATS = &raw mut (*part_search_state).this_rdc;
    let mi_row: core::ffi::c_int = blk_params.mi_row;
    let mi_col: core::ffi::c_int = blk_params.mi_col;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"none_partition_search\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4422 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*part_search_state).terminate_partition_search != 0
        || (*part_search_state).partition_none_allowed == 0
    {
        return;
    }
    let mut pt_cost: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_remain_rdcost: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut best_remain_rdcost);
    set_none_partition_params(
        cpi,
        td,
        x,
        pc_tree,
        part_search_state,
        &raw mut best_remain_rdcost,
        best_rdc,
        &raw mut pt_cost,
    );
    pick_sb_modes(
        cpi,
        tile_data,
        x,
        mi_row,
        mi_col,
        this_rdc,
        PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
        bsize,
        (*pc_tree).none,
        best_remain_rdcost,
    );
    av1_rd_cost_update((*x).rdmult, this_rdc);
    *pb_source_variance = (*x).source_variance;
    if !none_rd.is_null() {
        *none_rd = (*this_rdc).rdcost;
    }
    (*part_search_state).none_rd = (*this_rdc).rdcost;
    if (*this_rdc).rate != INT_MAX {
        if (*cpi).sf.inter_sf.prune_ref_frame_for_rect_partitions != 0 {
            let ref_type: core::ffi::c_int = av1_ref_frame_type(
                ((*(*pc_tree).none).mic.ref_frame).as_mut_ptr(),
            ) as core::ffi::c_int;
            av1_update_picked_ref_frames_mask(
                x,
                ref_type,
                bsize,
                (*(*cm).seq_params).mib_size,
                mi_row,
                mi_col,
            );
        }
        if blk_params.bsize_at_least_8x8 != 0 {
            (*this_rdc).rate += pt_cost;
            (*this_rdc).rdcost = ((*this_rdc).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*this_rdc).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        }
        *part_none_rd = (*this_rdc).rdcost;
        if (*this_rdc).rdcost < (*best_rdc).rdcost {
            *best_rdc = *this_rdc;
            (*part_search_state).found_best_partition = true_0 != 0;
            if blk_params.bsize_at_least_8x8 != 0 {
                (*pc_tree).partitioning = PARTITION_NONE as core::ffi::c_int
                    as PARTITION_TYPE;
            }
            prune_partitions_after_none(
                cpi,
                x,
                sms_tree,
                (*pc_tree).none,
                part_search_state,
                best_rdc,
                pb_source_variance,
            );
        }
        if (*cpi).sf.part_sf.prune_rect_part_using_none_pred_mode {
            prune_rect_part_using_none_pred_mode(
                &raw mut (*x).e_mbd,
                part_search_state,
                (*(*pc_tree).none).mic.mode,
                bsize,
            );
        }
    }
    av1_restore_context(x, x_ctx, mi_row, mi_col, bsize, av1_num_planes(cm));
}
#[inline]
unsafe extern "C-unwind" fn get_split_partition_penalty(
    mut bsize: BLOCK_SIZE,
    mut split_partition_penalty_level: core::ffi::c_int,
) -> core::ffi::c_double {
    if split_partition_penalty_level == 0 {
        return 1.00f64;
    }
    static mut penalty_factors: [[core::ffi::c_double; 5]; 2] = [
        [1.080f64, 1.040f64, 1.020f64, 1.010f64, 1.000f64],
        [1.100f64, 1.075f64, 1.050f64, 1.025f64, 1.000f64],
    ];
    let sqr_bsize_idx: core::ffi::c_int = get_sqr_bsize_idx(bsize) as core::ffi::c_int;
    if !(sqr_bsize_idx > 0 as core::ffi::c_int && sqr_bsize_idx < 6 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"get_split_partition_penalty\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4516 as core::ffi::c_int,
            b"sqr_bsize_idx > 0 && sqr_bsize_idx < SQR_BLOCK_SIZES\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let this_penalty_factor: core::ffi::c_double = penalty_factors[(split_partition_penalty_level
        - 1 as core::ffi::c_int)
        as usize][(sqr_bsize_idx - 1 as core::ffi::c_int) as usize];
    return this_penalty_factor;
}
unsafe extern "C-unwind" fn split_partition_search(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut x: *mut MACROBLOCK,
    mut pc_tree: *mut PC_TREE,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut x_ctx: *mut RD_SEARCH_MACROBLOCK_CONTEXT,
    mut part_search_state: *mut PartitionSearchState,
    mut best_rdc: *mut RD_STATS,
    mut multi_pass_mode: SB_MULTI_PASS_MODE,
    mut part_split_rd: *mut int64_t,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut blk_params: PartitionBlkParams = (*part_search_state).part_blk_params;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mi_row: core::ffi::c_int = blk_params.mi_row;
    let mi_col: core::ffi::c_int = blk_params.mi_col;
    let bsize: BLOCK_SIZE = blk_params.bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"split_partition_search\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            4535 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut sum_rdc: RD_STATS = (*part_search_state).sum_rdc;
    let subsize: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    if (*part_search_state).terminate_partition_search != 0
        || (*part_search_state).do_square_split == 0
    {
        return;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SUB_PARTITIONS_SPLIT {
        if ((*pc_tree).split[i as usize]).is_null() {
            (*pc_tree).split[i as usize] = av1_alloc_pc_tree_node(subsize)
                as *mut PC_TREE;
        }
        if ((*pc_tree).split[i as usize]).is_null() {
            aom_internal_error(
                (*x).e_mbd.error_info as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate PC_TREE\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        (*(*pc_tree).split[i as usize]).index = i;
        i += 1;
    }
    av1_init_rd_stats(&raw mut sum_rdc);
    sum_rdc.rate = *((*part_search_state).partition_cost)
        .offset(PARTITION_SPLIT as core::ffi::c_int as isize);
    sum_rdc.rdcost = (sum_rdc.rate as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t;
    let mut idx: core::ffi::c_int = 0;
    idx = 0 as core::ffi::c_int;
    while idx < SUB_PARTITIONS_SPLIT && sum_rdc.rdcost < (*best_rdc).rdcost {
        let x_idx: core::ffi::c_int = (idx & 1 as core::ffi::c_int) * blk_params.mi_step;
        let y_idx: core::ffi::c_int = (idx >> 1 as core::ffi::c_int)
            * blk_params.mi_step;
        if !(mi_row + y_idx >= (*mi_params).mi_rows
            || mi_col + x_idx >= (*mi_params).mi_cols)
        {
            (*(*pc_tree).split[idx as usize]).index = idx;
            let mut p_split_rd: *mut int64_t = &raw mut *((*part_search_state).split_rd)
                .as_mut_ptr()
                .offset(idx as isize) as *mut int64_t;
            let mut best_remain_rdcost: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            av1_rd_stats_subtraction(
                (*x).rdmult,
                best_rdc,
                &raw mut sum_rdc,
                &raw mut best_remain_rdcost,
            );
            let mut curr_quad_tree_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            if frame_is_intra_only(cm) != 0
                && bsize as core::ffi::c_int <= BLOCK_64X64 as core::ffi::c_int
            {
                curr_quad_tree_idx = (*(*part_search_state).intra_part_info)
                    .quad_tree_idx;
                (*(*part_search_state).intra_part_info).quad_tree_idx = 4
                    as core::ffi::c_int * curr_quad_tree_idx + idx
                    + 1 as core::ffi::c_int;
            }
            let sms_tree_split: *mut SIMPLE_MOTION_DATA_TREE = if sms_tree.is_null() {
                0 as *mut SIMPLE_MOTION_DATA_TREE
            } else {
                (*sms_tree).split[idx as usize]
            };
            if !av1_rd_pick_partition(
                cpi,
                td,
                tile_data,
                tp,
                mi_row + y_idx,
                mi_col + x_idx,
                subsize,
                &raw mut (*part_search_state).this_rdc,
                best_remain_rdcost,
                (*pc_tree).split[idx as usize],
                sms_tree_split,
                p_split_rd,
                multi_pass_mode,
                &raw mut *((*part_search_state).split_part_rect_win)
                    .as_mut_ptr()
                    .offset(idx as isize),
            ) {
                av1_invalid_rd_stats(&raw mut sum_rdc);
                break;
            } else {
                if frame_is_intra_only(cm) != 0
                    && bsize as core::ffi::c_int <= BLOCK_64X64 as core::ffi::c_int
                {
                    (*(*part_search_state).intra_part_info).quad_tree_idx = curr_quad_tree_idx;
                }
                sum_rdc.rate += (*part_search_state).this_rdc.rate;
                sum_rdc.dist += (*part_search_state).this_rdc.dist;
                av1_rd_cost_update((*x).rdmult, &raw mut sum_rdc);
                if idx <= 1 as core::ffi::c_int
                    && (bsize as core::ffi::c_int <= BLOCK_8X8 as core::ffi::c_int
                        || (*(*pc_tree).split[idx as usize]).partitioning
                            as core::ffi::c_int == PARTITION_NONE as core::ffi::c_int)
                {
                    let mbmi: *const MB_MODE_INFO = &raw mut (*(**((*pc_tree).split)
                        .as_mut_ptr()
                        .offset(idx as isize))
                        .none)
                        .mic;
                    let pmi: *const PALETTE_MODE_INFO = &(*mbmi).palette_mode_info;
                    if (*pmi).palette_size[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int == 0 as core::ffi::c_int
                        && (*pmi).palette_size[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int == 0 as core::ffi::c_int
                    {
                        if (*mbmi).uv_mode as core::ffi::c_int
                            != UV_CFL_PRED as core::ffi::c_int
                        {
                            (*part_search_state).is_split_ctx_is_ready[idx as usize] = 1
                                as core::ffi::c_int;
                        }
                    }
                }
            }
        }
        idx += 1;
    }
    let reached_last_index: core::ffi::c_int = (idx == SUB_PARTITIONS_SPLIT)
        as core::ffi::c_int;
    *part_split_rd = sum_rdc.rdcost;
    if reached_last_index != 0 && sum_rdc.rdcost < (*best_rdc).rdcost {
        sum_rdc.rdcost = (sum_rdc.rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + sum_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        let penalty_factor: core::ffi::c_double = get_split_partition_penalty(
            bsize,
            (*cpi).sf.part_sf.split_partition_penalty_level,
        ) as core::ffi::c_double;
        let this_rdcost: int64_t = (sum_rdc.rdcost as core::ffi::c_double
            * penalty_factor) as int64_t;
        if this_rdcost < (*best_rdc).rdcost {
            *best_rdc = sum_rdc;
            (*part_search_state).found_best_partition = true_0 != 0;
            (*pc_tree).partitioning = PARTITION_SPLIT as core::ffi::c_int
                as PARTITION_TYPE;
        }
    } else if (*cpi).sf.part_sf.less_rectangular_check_level > 0 as core::ffi::c_int {
        if (*cpi).sf.part_sf.less_rectangular_check_level == 2 as core::ffi::c_int
            || idx <= 2 as core::ffi::c_int
        {
            let partition_none_valid: core::ffi::c_int = ((*part_search_state).none_rd
                > 0 as int64_t) as core::ffi::c_int;
            let partition_none_better: core::ffi::c_int = ((*part_search_state).none_rd
                < sum_rdc.rdcost) as core::ffi::c_int;
            (*part_search_state).do_rectangular_split
                &= !(partition_none_valid != 0 && partition_none_better != 0)
                    as core::ffi::c_int;
        }
    }
    if bsize as core::ffi::c_int <= (*x).sb_enc.max_partition_size as core::ffi::c_int
        || bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int
    {
        av1_restore_context(x, x_ctx, mi_row, mi_col, bsize, av1_num_planes(cm));
    }
}
#[inline]
unsafe extern "C-unwind" fn should_do_dry_run_encode_for_current_block(
    mut sb_size: BLOCK_SIZE,
    mut max_partition_size: BLOCK_SIZE,
    mut curr_block_index: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> bool {
    if bsize as core::ffi::c_int > max_partition_size as core::ffi::c_int {
        return false_0 != 0;
    }
    if curr_block_index != 3 as core::ffi::c_int {
        return true_0 != 0;
    }
    let sub_sb_size: BLOCK_SIZE = get_partition_subsize(
        sb_size,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    ) as BLOCK_SIZE;
    return bsize as core::ffi::c_int == max_partition_size as core::ffi::c_int
        && sub_sb_size as core::ffi::c_int != max_partition_size as core::ffi::c_int;
}
unsafe extern "C-unwind" fn log_sub_block_var(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bs: BLOCK_SIZE,
    mut var_min: *mut core::ffi::c_double,
    mut var_max: *mut core::ffi::c_double,
) {
    let xd: *const MACROBLOCKD = &raw mut (*x).e_mbd;
    let is_hbd: core::ffi::c_int = is_cur_buf_hbd(xd) as core::ffi::c_int;
    let right_overflow: core::ffi::c_int = if (*xd).mb_to_right_edge
        < 0 as core::ffi::c_int
    {
        -(*xd).mb_to_right_edge >> 3 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let bottom_overflow: core::ffi::c_int = if (*xd).mb_to_bottom_edge
        < 0 as core::ffi::c_int
    {
        -(*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let bw: core::ffi::c_int = MI_SIZE * mi_size_wide[bs as usize] as core::ffi::c_int
        - right_overflow;
    let bh: core::ffi::c_int = MI_SIZE * mi_size_high[bs as usize] as core::ffi::c_int
        - bottom_overflow;
    let mut min_var_4x4: core::ffi::c_double = INT_MAX as core::ffi::c_double;
    let mut max_var_4x4: core::ffi::c_double = 0.0f64;
    let mut vf: aom_variance_fn_t = (*(*cpi).ppi)
        .fn_ptr[BLOCK_4X4 as core::ffi::c_int as usize]
        .vf;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < bh {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < bw {
            let mut var: core::ffi::c_int = 0;
            var = av1_calc_normalized_variance(
                vf,
                ((*x).plane[0 as core::ffi::c_int as usize].src.buf)
                    .offset(
                        (i * (*x).plane[0 as core::ffi::c_int as usize].src.stride)
                            as isize,
                    )
                    .offset(j as isize),
                (*x).plane[0 as core::ffi::c_int as usize].src.stride,
                is_hbd,
            );
            min_var_4x4 = if min_var_4x4 < var as core::ffi::c_double {
                min_var_4x4
            } else {
                var as core::ffi::c_double
            };
            max_var_4x4 = if max_var_4x4 > var as core::ffi::c_double {
                max_var_4x4
            } else {
                var as core::ffi::c_double
            };
            j += MI_SIZE;
        }
        i += MI_SIZE;
    }
    *var_min = log1p(min_var_4x4 / 16.0f64);
    *var_max = log1p(max_var_4x4 / 16.0f64);
}
#[inline]
unsafe extern "C-unwind" fn set_sms_tree_partitioning(
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut partition: PARTITION_TYPE,
) {
    if sms_tree.is_null() {
        return;
    }
    (*sms_tree).partitioning = partition;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rd_pick_partition(
    cpi: *mut AV1_COMP,
    mut td: *mut ThreadData,
    mut tile_data: *mut TileDataEnc,
    mut tp: *mut *mut TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut rd_cost: *mut RD_STATS,
    mut best_rdc: RD_STATS,
    mut pc_tree: *mut PC_TREE,
    mut sms_tree: *mut SIMPLE_MOTION_DATA_TREE,
    mut none_rd: *mut int64_t,
    mut multi_pass_mode: SB_MULTI_PASS_MODE,
    mut rect_part_win_info: *mut RD_RECT_PART_WIN_INFO,
) -> bool {
    let mut pb_source_variance: core::ffi::c_uint = 0;
    let mut part_none_rd: int64_t = 0;
    let mut part_split_rd: int64_t = 0;
    let mut prune_ext_part_state: core::ffi::c_int = 0;
    let mut ab_partition_allowed: core::ffi::c_int = 0;
    let mut part4_search_allowed: [core::ffi::c_int; 2] = [0; 2];
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tile_info: *mut TileInfo = &raw mut (*tile_data).tile_info;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut x_ctx: RD_SEARCH_MACROBLOCK_CONTEXT = RD_SEARCH_MACROBLOCK_CONTEXT {
        a: [0; 96],
        l: [0; 96],
        sa: [0; 32],
        sl: [0; 32],
        p_ta: 0 as *mut TXFM_CONTEXT,
        p_tl: 0 as *mut TXFM_CONTEXT,
        ta: [0; 32],
        tl: [0; 32],
    };
    let tp_orig: *const TokenExtra = *tp;
    let mut part_search_state: PartitionSearchState = PartitionSearchState {
        intra_part_info: 0 as *mut PartitionSearchInfo,
        part_blk_params: PartitionBlkParams {
            mi_step: 0,
            mi_row: 0,
            mi_col: 0,
            mi_row_edge: 0,
            mi_col_edge: 0,
            width: 0,
            min_partition_size_1d: 0,
            bsize_at_least_8x8: 0,
            has_rows: 0,
            has_cols: 0,
            bsize: BLOCK_4X4,
            subsize: BLOCK_4X4,
            split_bsize2: BLOCK_4X4,
        },
        split_part_rect_win: [RD_RECT_PART_WIN_INFO {
            rect_part_win: [0; 2],
        }; 4],
        this_rdc: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        sum_rdc: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        tmp_partition_cost: [0; 4],
        partition_cost: 0 as *mut core::ffi::c_int,
        none_rd: 0,
        split_rd: [0; 4],
        rect_part_rd: [[0; 2]; 2],
        is_split_ctx_is_ready: [0; 2],
        is_rect_ctx_is_ready: [0; 2],
        terminate_partition_search: 0,
        partition_none_allowed: 0,
        partition_rect_allowed: [0; 2],
        do_rectangular_split: 0,
        do_square_split: 0,
        prune_rect_part: [0; 2],
        ss_x: 0,
        ss_y: 0,
        pl_ctx_idx: 0,
        found_best_partition: false,
    };
    init_partition_search_state_params(
        x,
        cpi,
        &raw mut part_search_state,
        mi_row,
        mi_col,
        bsize,
    );
    let mut blk_params: PartitionBlkParams = part_search_state.part_blk_params;
    set_sms_tree_partitioning(
        sms_tree,
        PARTITION_NONE as core::ffi::c_int as PARTITION_TYPE,
    );
    if best_rdc.rdcost < 0 as int64_t {
        av1_invalid_rd_stats(rd_cost);
        return part_search_state.found_best_partition;
    }
    if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
        (*x).must_find_valid_partition = 0 as core::ffi::c_int;
    }
    if !none_rd.is_null() {
        *none_rd = 0 as int64_t;
    }
    if !av1_blk_has_rows_and_cols(&raw mut blk_params) {
        set_partition_cost_for_edge_blk(cm, &raw mut part_search_state);
    }
    if bsize as core::ffi::c_int
        > (*cpi).sf.part_sf.use_square_partition_only_threshold as core::ffi::c_int
    {
        part_search_state.partition_rect_allowed[HORZ as core::ffi::c_int as usize]
            &= (blk_params.has_rows == 0) as core::ffi::c_int;
        part_search_state.partition_rect_allowed[VERT as core::ffi::c_int as usize]
            &= (blk_params.has_cols == 0) as core::ffi::c_int;
    }
    if !(mi_size_wide[bsize as usize] as core::ffi::c_int
        == mi_size_high[bsize as usize] as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_rd_pick_partition\0"))
                .as_ptr(),
            b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
            5716 as core::ffi::c_int,
            b"mi_size_wide[bsize] == mi_size_high[bsize]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    av1_set_offsets(cpi, tile_info, x, mi_row, mi_col, bsize);
    if (*cpi).oxcf.mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int {
        if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
            let mut var_min: core::ffi::c_double = 0.;
            let mut var_max: core::ffi::c_double = 0.;
            log_sub_block_var(cpi, x, bsize, &raw mut var_min, &raw mut var_max);
            (*x).intra_sb_rdmult_modifier = 128 as core::ffi::c_int;
            if var_min < 2.0f64 && var_max > 4.0f64 {
                if var_max - var_min > 8.0f64 {
                    (*x).intra_sb_rdmult_modifier -= 48 as core::ffi::c_int;
                } else {
                    (*x).intra_sb_rdmult_modifier
                        -= ((var_max - var_min)
                            * 6 as core::ffi::c_int as core::ffi::c_double)
                            as core::ffi::c_int;
                }
            }
        }
    }
    let orig_rdmult: core::ffi::c_int = (*x).rdmult;
    setup_block_rdmult(
        cpi,
        x,
        mi_row,
        mi_col,
        bsize,
        NO_AQ as core::ffi::c_int as AQ_MODE,
        0 as *mut MB_MODE_INFO,
    );
    av1_rd_cost_update((*x).rdmult, &raw mut best_rdc);
    if bsize as core::ffi::c_int == BLOCK_16X16 as core::ffi::c_int
        && (*cpi).vaq_refresh != 0
    {
        (*x).mb_energy = av1_log_block_var(cpi, x, bsize);
    }
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile_info).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    av1_save_context(x, &raw mut x_ctx, mi_row, mi_col, bsize, num_planes);
    av1_prune_partitions_before_search(cpi, x, sms_tree, &raw mut part_search_state);
    av1_prune_partitions_by_max_min_bsize(&raw mut (*x).sb_enc, &raw mut part_search_state);
    loop {
        if (*x).must_find_valid_partition != 0 {
            reset_part_limitations(cpi, &raw mut part_search_state);
            av1_prune_partitions_by_max_min_bsize(
                &raw mut (*x).sb_enc,
                &raw mut part_search_state,
            );
            if frame_is_intra_only(cm) != 0
                && bsize as core::ffi::c_int == BLOCK_64X64 as core::ffi::c_int
            {
                (*part_search_state.intra_part_info).quad_tree_idx = 0
                    as core::ffi::c_int;
                (*part_search_state.intra_part_info).cnn_output_valid = 0
                    as core::ffi::c_int;
            }
        }
        pb_source_variance = UINT_MAX;
        if (*cpi).oxcf.mode as core::ffi::c_int == ALLINTRA as core::ffi::c_int {
            let bsize_at_least_16x16: bool = bsize as core::ffi::c_int
                >= BLOCK_16X16 as core::ffi::c_int;
            let prune_rect_part_using_4x4_var_deviation: bool = (*cpi)
                .sf
                .part_sf
                .prune_rect_part_using_4x4_var_deviation as core::ffi::c_int != 0
                && (*x).must_find_valid_partition == 0;
            if bsize_at_least_16x16 as core::ffi::c_int != 0
                || prune_rect_part_using_4x4_var_deviation as core::ffi::c_int != 0
            {
                let mut var_min_0: core::ffi::c_double = 0.;
                let mut var_max_0: core::ffi::c_double = 0.;
                log_sub_block_var(cpi, x, bsize, &raw mut var_min_0, &raw mut var_max_0);
                if bsize_at_least_16x16 as core::ffi::c_int != 0 && var_min_0 < 0.272f64
                    && var_max_0 - var_min_0 > 3.0f64
                {
                    part_search_state.partition_none_allowed = 0 as core::ffi::c_int;
                    part_search_state.terminate_partition_search = 0 as core::ffi::c_int;
                    part_search_state.do_square_split = 1 as core::ffi::c_int;
                } else if prune_rect_part_using_4x4_var_deviation as core::ffi::c_int
                    != 0 && var_max_0 - var_min_0 < 3.0f64
                {
                    part_search_state.do_rectangular_split = 0 as core::ffi::c_int;
                }
            }
        }
        part_none_rd = INT64_MAX as int64_t;
        none_partition_search(
            cpi,
            td,
            tile_data,
            x,
            pc_tree,
            sms_tree,
            &raw mut x_ctx,
            &raw mut part_search_state,
            &raw mut best_rdc,
            &raw mut pb_source_variance,
            none_rd,
            &raw mut part_none_rd,
        );
        part_split_rd = INT64_MAX as int64_t;
        split_partition_search(
            cpi,
            td,
            tile_data,
            tp,
            x,
            pc_tree,
            sms_tree,
            &raw mut x_ctx,
            &raw mut part_search_state,
            &raw mut best_rdc,
            multi_pass_mode,
            &raw mut part_split_rd,
        );
        if (*cpi).sf.part_sf.early_term_after_none_split != 0
            && part_none_rd == INT64_MAX as int64_t
            && part_split_rd == INT64_MAX as int64_t
            && (*x).must_find_valid_partition == 0
            && bsize as core::ffi::c_int
                != (*(*cm).seq_params).sb_size as core::ffi::c_int
        {
            part_search_state.terminate_partition_search = 1 as core::ffi::c_int;
        }
        if (*cpi).sf.part_sf.skip_non_sq_part_based_on_none >= 2 as core::ffi::c_int
            && !((*pc_tree).none).is_null()
        {
            if (*x).qindex <= 200 as core::ffi::c_int
                && is_inter_mode((*(*pc_tree).none).mic.mode) != 0
                && have_newmv_in_inter_mode((*(*pc_tree).none).mic.mode) == 0
                && (*(*pc_tree).none).skippable != 0
                && (*x).must_find_valid_partition == 0
                && bsize as core::ffi::c_int >= BLOCK_16X16 as core::ffi::c_int
            {
                part_search_state.do_rectangular_split = 0 as core::ffi::c_int;
            }
        }
        prune_partitions_after_split(
            cpi,
            x,
            sms_tree,
            &raw mut part_search_state,
            &raw mut best_rdc,
            part_none_rd,
            part_split_rd,
        );
        rectangular_partition_search(
            cpi,
            td,
            tile_data,
            tp,
            x,
            pc_tree,
            &raw mut x_ctx,
            &raw mut part_search_state,
            &raw mut best_rdc,
            rect_part_win_info,
            HORZ as core::ffi::c_int as RECT_PART_TYPE,
            VERT as core::ffi::c_int as RECT_PART_TYPE,
        );
        if pb_source_variance == UINT_MAX {
            av1_setup_src_planes(
                x as *mut macroblock,
                (*cpi).source,
                mi_row,
                mi_col,
                num_planes,
                bsize,
            );
            pb_source_variance = av1_get_perpixel_variance_facade(
                cpi,
                xd,
                &raw mut (*((*x).plane).as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                    .src,
                bsize,
                AOM_PLANE_Y,
            );
        }
        if !((*cpi).oxcf.part_cfg.enable_rect_partitions
            || part_search_state.do_rectangular_split == 0) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                5900 as core::ffi::c_int,
                b"IMPLIES(!cpi->oxcf.part_cfg.enable_rect_partitions, !part_search_state.do_rectangular_split)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        prune_ext_part_state = prune_ext_part_none_skippable(
            (*pc_tree).none,
            (*x).must_find_valid_partition,
            (*cpi).sf.part_sf.skip_non_sq_part_based_on_none,
            bsize,
        ) as core::ffi::c_int;
        ab_partition_allowed = allow_ab_partition_search(
            &raw mut part_search_state,
            &raw mut (*cpi).sf.part_sf,
            (*pc_tree).partitioning,
            (*x).must_find_valid_partition,
            prune_ext_part_state,
            best_rdc.rdcost,
        ) as core::ffi::c_int;
        ab_partitions_search(
            cpi,
            td,
            tile_data,
            tp,
            x,
            &raw mut x_ctx,
            pc_tree,
            &raw mut part_search_state,
            &raw mut best_rdc,
            rect_part_win_info,
            pb_source_variance as core::ffi::c_int,
            ab_partition_allowed,
            HORZ_A as core::ffi::c_int as AB_PART_TYPE,
            VERT_B as core::ffi::c_int as AB_PART_TYPE,
        );
        part4_search_allowed = [1 as core::ffi::c_int, 1 as core::ffi::c_int];
        prune_4_way_partition_search(
            cpi,
            x,
            pc_tree,
            &raw mut part_search_state,
            sms_tree,
            &raw mut best_rdc,
            pb_source_variance as core::ffi::c_int,
            prune_ext_part_state,
            mi_row,
            mi_col,
            part4_search_allowed.as_mut_ptr(),
        );
        if !((*cpi).oxcf.part_cfg.enable_rect_partitions
            || part4_search_allowed[HORZ4 as core::ffi::c_int as usize] == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                5935 as core::ffi::c_int,
                b"IMPLIES(!cpi->oxcf.part_cfg.enable_rect_partitions, !part4_search_allowed[HORZ4])\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if part_search_state.terminate_partition_search == 0
            && part4_search_allowed[HORZ4 as core::ffi::c_int as usize] != 0
        {
            let inc_step: [core::ffi::c_int; 2] = [
                mi_size_high[blk_params.bsize as usize] as core::ffi::c_int
                    / 4 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ];
            rd_pick_4partition(
                cpi,
                td,
                tile_data,
                tp,
                x,
                &raw mut x_ctx,
                pc_tree,
                ((*pc_tree).horizontal4).as_mut_ptr(),
                &raw mut part_search_state,
                &raw mut best_rdc,
                inc_step.as_ptr(),
                PARTITION_HORZ_4 as core::ffi::c_int as PARTITION_TYPE,
            );
        }
        if !((*cpi).oxcf.part_cfg.enable_rect_partitions
            || part4_search_allowed[VERT4 as core::ffi::c_int as usize] == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                5948 as core::ffi::c_int,
                b"IMPLIES(!cpi->oxcf.part_cfg.enable_rect_partitions, !part4_search_allowed[VERT4])\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if part_search_state.terminate_partition_search == 0
            && part4_search_allowed[VERT4 as core::ffi::c_int as usize] != 0
            && blk_params.has_cols != 0
        {
            let inc_step_0: [core::ffi::c_int; 2] = [
                0 as core::ffi::c_int,
                mi_size_wide[blk_params.bsize as usize] as core::ffi::c_int
                    / 4 as core::ffi::c_int,
            ];
            rd_pick_4partition(
                cpi,
                td,
                tile_data,
                tp,
                x,
                &raw mut x_ctx,
                pc_tree,
                ((*pc_tree).vertical4).as_mut_ptr(),
                &raw mut part_search_state,
                &raw mut best_rdc,
                inc_step_0.as_ptr(),
                PARTITION_VERT_4 as core::ffi::c_int as PARTITION_TYPE,
            );
        }
        if !(bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int
            && !part_search_state.found_best_partition)
        {
            break;
        }
        (*x).must_find_valid_partition = 1 as core::ffi::c_int;
    }
    *rd_cost = best_rdc;
    set_sms_tree_partitioning(sms_tree, (*pc_tree).partitioning);
    let mut pc_tree_dealloc: core::ffi::c_int = 0 as core::ffi::c_int;
    if part_search_state.found_best_partition {
        if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
            let emit_output: core::ffi::c_int = (multi_pass_mode as core::ffi::c_int
                != SB_DRY_PASS as core::ffi::c_int) as core::ffi::c_int;
            let run_type: RUN_TYPE = (if emit_output != 0 {
                OUTPUT_ENABLED as core::ffi::c_int
            } else {
                DRY_RUN_NORMAL as core::ffi::c_int
            }) as RUN_TYPE;
            set_cb_offsets(((*x).cb_offset).as_mut_ptr(), 0 as uint16_t, 0 as uint16_t);
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                run_type,
                bsize,
                pc_tree,
                0 as *mut core::ffi::c_int,
            );
            if !(pc_tree == (*td).pc_root) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_rd_pick_partition\0"))
                        .as_ptr(),
                    b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                    6023 as core::ffi::c_int,
                    b"pc_tree == td->pc_root\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            av1_free_pc_tree_recursive(
                pc_tree,
                num_planes,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                (*cpi).sf.part_sf.partition_search_type,
            );
            pc_tree = 0 as *mut PC_TREE;
            (*td).pc_root = 0 as *mut PC_TREE;
            pc_tree_dealloc = 1 as core::ffi::c_int;
        } else if should_do_dry_run_encode_for_current_block(
            (*(*cm).seq_params).sb_size,
            (*x).sb_enc.max_partition_size,
            (*pc_tree).index,
            bsize,
        ) {
            encode_sb(
                cpi,
                td,
                tile_data,
                tp,
                mi_row,
                mi_col,
                DRY_RUN_NORMAL as core::ffi::c_int as RUN_TYPE,
                bsize,
                pc_tree,
                0 as *mut core::ffi::c_int,
            );
        }
    }
    if pc_tree_dealloc == 0 as core::ffi::c_int {
        av1_free_pc_tree_recursive(
            pc_tree,
            num_planes,
            1 as core::ffi::c_int,
            1 as core::ffi::c_int,
            (*cpi).sf.part_sf.partition_search_type,
        );
    }
    if bsize as core::ffi::c_int == (*(*cm).seq_params).sb_size as core::ffi::c_int {
        if !(best_rdc.rate < 2147483647 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                6049 as core::ffi::c_int,
                b"best_rdc.rate < INT_MAX\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(best_rdc.dist < 9223372036854775807 as int64_t) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                6050 as core::ffi::c_int,
                b"best_rdc.dist < INT64_MAX\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    } else {
        if !(tp_orig == *tp as *const TokenExtra) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"av1_rd_pick_partition\0"))
                    .as_ptr(),
                b"partition_search.c\0" as *const u8 as *const core::ffi::c_char,
                6052 as core::ffi::c_int,
                b"tp_orig == *tp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    (*x).rdmult = orig_rdmult;
    return part_search_state.found_best_partition;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const __DBL_MAX__: core::ffi::c_double = 1.7976931348623157e+308f64;
pub const DBL_MAX: core::ffi::c_double = __DBL_MAX__;

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
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
    fn log1p(_: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(_: core::ffi::c_double) -> core::ffi::c_double;
}
