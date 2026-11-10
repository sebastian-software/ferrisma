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

// Mechanically translated from upstream/aom/av1/decoder/decodeframe.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_merge_corrupted_flag(
        corrupted: *mut core::ffi::c_int,
        value: core::ffi::c_int,
    );
    fn od_ec_decode_bool_q15(
        dec: *mut od_ec_dec,
        f: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn od_ec_decode_cdf_q15(
        dec: *mut od_ec_dec,
        cdf: *const uint16_t,
        nsyms: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_reader_init(
        r: *mut aom_reader,
        buffer: *const uint8_t,
        size: size_t,
    ) -> core::ffi::c_int;
    fn aom_reader_find_begin(r: *mut aom_reader) -> *const uint8_t;
    fn aom_reader_find_end(r: *mut aom_reader) -> *const uint8_t;
    fn aom_reader_has_overflowed(r: *const aom_reader) -> core::ffi::c_int;
    fn aom_reader_tell(r: *const aom_reader) -> uint32_t;
    fn aom_rb_bytes_read(rb: *const aom_read_bit_buffer) -> size_t;
    fn aom_rb_read_bit(rb: *mut aom_read_bit_buffer) -> core::ffi::c_int;
    fn aom_rb_read_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_rb_read_uvlc(rb: *mut aom_read_bit_buffer) -> uint32_t;
    fn aom_rb_read_unsigned_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> uint32_t;
    fn aom_rb_read_inv_signed_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_rb_read_signed_primitive_refsubexpfin(
        rb: *mut aom_read_bit_buffer,
        n: uint16_t,
        k: uint16_t,
        ref_0: int16_t,
    ) -> int16_t;
    fn aom_read_primitive_refsubexpfin_(
        r: *mut aom_reader,
        n: uint16_t,
        k: uint16_t,
        ref_0: uint16_t,
    ) -> uint16_t;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_realloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        fb: *mut aom_codec_frame_buffer_t,
        cb: aom_get_frame_buffer_cb_fn_t,
        cb_priv: *mut core::ffi::c_void,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
    fn av1_alloc_above_context_buffers(
        above_contexts: *mut CommonContexts,
        num_tile_rows: core::ffi::c_int,
        num_mi_cols: core::ffi::c_int,
        num_planes: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_free_above_context_buffers(above_contexts: *mut CommonContexts);
    fn av1_alloc_context_buffers(
        cm: *mut AV1Common,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        min_partition_size: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_init_mi_buffers(mi_params: *mut CommonModeInfoParams);
    fn av1_alloc_cdef_buffers(
        cm: *mut AV1Common,
        cdef_worker: *mut *mut AV1CdefWorker,
        cdef_sync: *mut AV1CdefSyncData,
        num_workers: core::ffi::c_int,
        init_worker: core::ffi::c_int,
    );
    fn av1_alloc_restoration_buffers(cm: *mut AV1Common, is_sgr_enabled: bool);
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_clearall_segfeatures(seg: *mut segmentation);
    fn av1_enable_segfeature(
        seg: *mut segmentation,
        segment_id: core::ffi::c_int,
        feature_id: SEG_LVL_FEATURES,
    );
    fn av1_calculate_segdata(seg: *mut segmentation);
    fn av1_seg_feature_data_max(feature_id: SEG_LVL_FEATURES) -> core::ffi::c_int;
    fn av1_is_segfeature_signed(feature_id: SEG_LVL_FEATURES) -> core::ffi::c_int;
    fn av1_set_segdata(
        seg: *mut segmentation,
        segment_id: core::ffi::c_int,
        feature_id: SEG_LVL_FEATURES,
        seg_data: core::ffi::c_int,
    );
    fn av1_reset_cdf_symbol_counters(fc: *mut frame_contexts);
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
    fn av1_use_qmatrix(
        quant_params: *const CommonQuantParams,
        xd: *const macroblockd,
        segment_id: core::ffi::c_int,
    ) -> bool;
    fn av1_set_default_ref_deltas(ref_deltas: *mut int8_t);
    fn av1_set_default_mode_deltas(mode_deltas: *mut int8_t);
    fn av1_setup_past_independence(cm: *mut AV1Common);
    fn av1_scale_mv(
        mv: *const MV,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        sf: *const scale_factors,
    ) -> MV32;
    fn av1_setup_scale_factors_for_frame(
        sf: *mut scale_factors,
        other_w: core::ffi::c_int,
        other_h: core::ffi::c_int,
        this_w: core::ffi::c_int,
        this_h: core::ffi::c_int,
    );
    fn av1_tile_init(
        tile: *mut TileInfo,
        cm: *const AV1Common,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
    );
    fn av1_tile_set_row(
        tile: *mut TileInfo,
        cm: *const AV1Common,
        row: core::ffi::c_int,
    );
    fn av1_tile_set_col(
        tile: *mut TileInfo,
        cm: *const AV1Common,
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
    fn av1_get_uniform_tile_size(
        cm: *const AV1Common,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> bool;
    fn av1_get_tile_limits(cm: *mut AV1Common);
    fn av1_calculate_tile_cols(
        seq_params: *const SequenceHeader,
        cm_mi_rows: core::ffi::c_int,
        cm_mi_cols: core::ffi::c_int,
        tiles: *mut CommonTileParams,
    );
    fn av1_calculate_tile_rows(
        seq_params: *const SequenceHeader,
        cm_mi_rows: core::ffi::c_int,
        tiles: *mut CommonTileParams,
    );
    fn av1_is_min_tile_width_satisfied(cm: *const AV1Common) -> core::ffi::c_int;
    fn av1_setup_block_planes(
        xd: *mut MACROBLOCKD,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        num_planes: core::ffi::c_int,
    );
    fn av1_reset_entropy_context(
        xd: *mut MACROBLOCKD,
        bsize: BLOCK_SIZE,
        num_planes: core::ffi::c_int,
    );
    fn av1_reset_loop_filter_delta(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    fn av1_reset_loop_restoration(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    static av1_sgr_params: [sgr_params_type; 16];
    fn av1_alloc_restoration_struct(
        cm: *mut AV1Common,
        rsi: *mut RestorationInfo,
        is_uv: core::ffi::c_int,
    );
    fn av1_loop_restoration_filter_frame(
        frame: *mut YV12_BUFFER_CONFIG,
        cm: *mut AV1Common,
        optimized_lr: core::ffi::c_int,
        lr_ctxt: *mut core::ffi::c_void,
    );
    fn av1_loop_restoration_corners_in_sb(
        cm: *const AV1Common,
        plane: core::ffi::c_int,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
        rcol0: *mut core::ffi::c_int,
        rcol1: *mut core::ffi::c_int,
        rrow0: *mut core::ffi::c_int,
        rrow1: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_loop_restoration_save_boundary_lines(
        frame: *const YV12_BUFFER_CONFIG,
        cm: *mut AV1Common,
        after_cdef: core::ffi::c_int,
    );
    fn av1_zero_unused_internal_frame_buffers(list: *mut InternalFrameBufferList);
    fn cfl_init(cfl: *mut CFL_CTX, seq_params: *const SequenceHeader);
    fn av1_cdef_frame(
        frame: *mut YV12_BUFFER_CONFIG,
        cm: *mut AV1_COMMON,
        xd: *mut MACROBLOCKD,
        cdef_init_fb_row_fn: cdef_init_fb_row_t,
    );
    fn av1_cdef_init_fb_row(
        cm: *const AV1_COMMON,
        xd: *const MACROBLOCKD,
        fb_info: *mut CdefBlockInfo,
        linebuf: *mut *mut uint16_t,
        src: *mut uint16_t,
        cdef_sync: *mut AV1CdefSyncData,
        fbr: core::ffi::c_int,
    );
    fn cfl_store_block(xd: *mut MACROBLOCKD, bsize: BLOCK_SIZE, tx_size: TX_SIZE);
    fn cfl_store_tx(
        xd: *mut MACROBLOCKD,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
        tx_size: TX_SIZE,
        bsize: BLOCK_SIZE,
    );
    fn av1_inverse_transform_block(
        xd: *const MACROBLOCKD,
        dqcoeff: *const tran_low_t,
        plane: core::ffi::c_int,
        tx_type: TX_TYPE,
        tx_size: TX_SIZE,
        dst: *mut uint8_t,
        stride: core::ffi::c_int,
        eob: core::ffi::c_int,
        reduced_tx_set: core::ffi::c_int,
    );
    fn av1_setup_frame_buf_refs(cm: *mut AV1_COMMON);
    fn av1_setup_frame_sign_bias(cm: *mut AV1_COMMON);
    fn av1_setup_skip_mode_allowed(cm: *mut AV1_COMMON);
    fn av1_calculate_ref_frame_side(cm: *mut AV1_COMMON);
    fn av1_setup_motion_field(cm: *mut AV1_COMMON);
    fn av1_set_frame_refs(
        cm: *mut AV1_COMMON,
        remapped_ref_idx: *mut core::ffi::c_int,
        lst_map_idx: core::ffi::c_int,
        gld_map_idx: core::ffi::c_int,
    );
    fn av1_get_shear_params(wm: *mut WarpedMotionParams) -> core::ffi::c_int;
    fn av1_init_warp_params(
        inter_pred_params: *mut InterPredParams,
        warp_types: *const WarpTypesAllowed,
        ref_0: core::ffi::c_int,
        xd: *const MACROBLOCKD,
        mi: *const MB_MODE_INFO,
    );
    fn av1_skip_u4x4_pred_in_obmc(
        bsize: BLOCK_SIZE,
        pd: *const macroblockd_plane,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_make_inter_predictor(
        src: *const uint8_t,
        src_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        inter_pred_params: *mut InterPredParams,
        subpel_params: *const SubpelParams,
    );
    fn av1_make_masked_inter_predictor(
        pre: *const uint8_t,
        pre_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        inter_pred_params: *mut InterPredParams,
        subpel_params: *const SubpelParams,
    );
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
    fn av1_setup_obmc_dst_bufs(
        xd: *mut MACROBLOCKD,
        dst_buf1: *mut *mut uint8_t,
        dst_buf2: *mut *mut uint8_t,
    );
    fn av1_setup_build_prediction_by_above_pred(
        xd: *mut MACROBLOCKD,
        rel_mi_col: core::ffi::c_int,
        above_mi_width: uint8_t,
        above_mbmi: *mut MB_MODE_INFO,
        ctxt: *mut build_prediction_ctxt,
        num_planes: core::ffi::c_int,
    );
    fn av1_setup_build_prediction_by_left_pred(
        xd: *mut MACROBLOCKD,
        rel_mi_row: core::ffi::c_int,
        left_mi_height: uint8_t,
        left_mbmi: *mut MB_MODE_INFO,
        ctxt: *mut build_prediction_ctxt,
        num_planes: core::ffi::c_int,
    );
    fn av1_build_obmc_inter_prediction(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        above: *mut *mut uint8_t,
        above_stride: *mut core::ffi::c_int,
        left: *mut *mut uint8_t,
        left_stride: *mut core::ffi::c_int,
    );
    fn av1_dist_wtd_comp_weight_assign(
        cm: *const AV1_COMMON,
        mbmi: *const MB_MODE_INFO,
        fwd_offset: *mut core::ffi::c_int,
        bck_offset: *mut core::ffi::c_int,
        use_dist_wtd_comp_avg: *mut core::ffi::c_int,
        is_compound: core::ffi::c_int,
    );
    fn av1_build_interintra_predictor(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        pred: *mut uint8_t,
        stride: core::ffi::c_int,
        ctx: *const BUFFER_SET,
        plane: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    );
    fn av1_predict_intra_block_facade(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        plane: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        blk_row: core::ffi::c_int,
        tx_size: TX_SIZE,
    );
    fn av1_calculate_scaled_superres_size(
        width: *mut core::ffi::c_int,
        height: *mut core::ffi::c_int,
        superres_denom: core::ffi::c_int,
    );
    fn av1_superres_upscale(
        cm: *mut AV1_COMMON,
        pool: *mut BufferPool,
        alloc_pyramid: bool,
    );
    fn av1_cdef_frame_mt(
        cm: *mut AV1_COMMON,
        xd: *mut MACROBLOCKD,
        cdef_worker: *mut AV1CdefWorkerData,
        workers: *mut AVxWorker,
        cdef_sync: *mut AV1CdefSync,
        num_workers: core::ffi::c_int,
        cdef_init_fb_row_fn: cdef_init_fb_row_t,
        do_extend_border: core::ffi::c_int,
    );
    fn av1_cdef_init_fb_row_mt(
        cm: *const AV1_COMMON,
        xd: *const MACROBLOCKD,
        fb_info: *mut CdefBlockInfo,
        linebuf: *mut *mut uint16_t,
        src: *mut uint16_t,
        cdef_sync: *mut AV1CdefSyncData,
        fbr: core::ffi::c_int,
    );
    fn av1_alloc_cdef_sync(
        cm: *mut AV1_COMMON,
        cdef_sync: *mut AV1CdefSync,
        num_workers: core::ffi::c_int,
    );
    fn av1_loop_filter_frame_mt(
        frame: *mut YV12_BUFFER_CONFIG,
        cm: *mut AV1Common,
        xd: *mut macroblockd,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
        partial_frame: core::ffi::c_int,
        workers: *mut AVxWorker,
        num_workers: core::ffi::c_int,
        lf_sync: *mut AV1LfSync,
        lpf_opt_level: core::ffi::c_int,
    );
    fn av1_loop_restoration_filter_frame_mt(
        frame: *mut YV12_BUFFER_CONFIG,
        cm: *mut AV1Common,
        optimized_lr: core::ffi::c_int,
        workers: *mut AVxWorker,
        num_workers: core::ffi::c_int,
        lr_sync: *mut AV1LrSync,
        lr_ctxt: *mut core::ffi::c_void,
        do_extend_border: core::ffi::c_int,
    );
    fn av1_get_intrabc_extra_top_right_sb_delay(
        cm: *const AV1_COMMON,
    ) -> core::ffi::c_int;
    fn av1_dealloc_dec_jobs(tile_mt_info: *mut AV1DecTileMTData);
    fn av1_dec_free_cb_buf(pbi: *mut AV1Decoder);
    fn av1_visit_palette(
        pbi: *mut AV1Decoder,
        xd: *mut MACROBLOCKD,
        r: *mut aom_reader,
        visit: palette_visitor_fn_t,
    );
    fn av1_read_mode_info(
        pbi: *mut AV1Decoder,
        dcb: *mut DecoderCodingBlock,
        r: *mut aom_reader,
        x_mis: core::ffi::c_int,
        y_mis: core::ffi::c_int,
    );
    fn av1_read_coeffs_txb(
        cm: *const AV1Common,
        dcb: *mut DecoderCodingBlock,
        r: *mut aom_reader,
        plane: core::ffi::c_int,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
        tx_size: TX_SIZE,
    );
    fn av1_decode_palette_tokens(
        xd: *mut MACROBLOCKD,
        plane: core::ffi::c_int,
        r: *mut aom_reader,
    );
}
pub type size_t = usize;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
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
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
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
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint16_t = u16;
pub type uint64_t = u64;
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
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type uintptr_t = usize;
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
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_s_frame_info {
    pub is_s_frame: core::ffi::c_int,
    pub is_s_frame_at_altref: core::ffi::c_int,
}
pub type aom_cdf_prob = uint16_t;
pub type od_ec_window = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_dec {
    pub buf: *const core::ffi::c_uchar,
    pub tell_offs: int32_t,
    pub end: *const core::ffi::c_uchar,
    pub bptr: *const core::ffi::c_uchar,
    pub dif: od_ec_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
pub type aom_rb_error_handler = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: uint32_t,
    pub error_handler_data: *mut core::ffi::c_void,
    pub error_handler: aom_rb_error_handler,
}
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed_11 = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed_11 = 4;
pub const TX_SIZES: C2RustUnnamed_11 = 5;
pub const TX_SIZES_ALL: C2RustUnnamed_11 = 19;
pub const TX_64X16: C2RustUnnamed_11 = 18;
pub const TX_16X64: C2RustUnnamed_11 = 17;
pub const TX_32X8: C2RustUnnamed_11 = 16;
pub const TX_8X32: C2RustUnnamed_11 = 15;
pub const TX_16X4: C2RustUnnamed_11 = 14;
pub const TX_4X16: C2RustUnnamed_11 = 13;
pub const TX_64X32: C2RustUnnamed_11 = 12;
pub const TX_32X64: C2RustUnnamed_11 = 11;
pub const TX_32X16: C2RustUnnamed_11 = 10;
pub const TX_16X32: C2RustUnnamed_11 = 9;
pub const TX_16X8: C2RustUnnamed_11 = 8;
pub const TX_8X16: C2RustUnnamed_11 = 7;
pub const TX_8X4: C2RustUnnamed_11 = 6;
pub const TX_4X8: C2RustUnnamed_11 = 5;
pub const TX_64X64: C2RustUnnamed_11 = 4;
pub const TX_32X32: C2RustUnnamed_11 = 3;
pub const TX_16X16: C2RustUnnamed_11 = 2;
pub const TX_8X8: C2RustUnnamed_11 = 1;
pub const TX_4X4: C2RustUnnamed_11 = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_12 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_12 = 15;
pub const TX_TYPES: C2RustUnnamed_12 = 16;
pub const H_FLIPADST: C2RustUnnamed_12 = 15;
pub const V_FLIPADST: C2RustUnnamed_12 = 14;
pub const H_ADST: C2RustUnnamed_12 = 13;
pub const V_ADST: C2RustUnnamed_12 = 12;
pub const H_DCT: C2RustUnnamed_12 = 11;
pub const V_DCT: C2RustUnnamed_12 = 10;
pub const IDTX: C2RustUnnamed_12 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_12 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_12 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_12 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_12 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_12 = 4;
pub const ADST_ADST: C2RustUnnamed_12 = 3;
pub const DCT_ADST: C2RustUnnamed_12 = 2;
pub const ADST_DCT: C2RustUnnamed_12 = 1;
pub const DCT_DCT: C2RustUnnamed_12 = 0;
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const EXT_TX_SET_TYPES: C2RustUnnamed_13 = 6;
pub const EXT_TX_SET_ALL16: C2RustUnnamed_13 = 5;
pub const EXT_TX_SET_DTT9_IDTX_1DDCT: C2RustUnnamed_13 = 4;
pub const EXT_TX_SET_DTT4_IDTX_1DDCT: C2RustUnnamed_13 = 3;
pub const EXT_TX_SET_DTT4_IDTX: C2RustUnnamed_13 = 2;
pub const EXT_TX_SET_DCT_IDTX: C2RustUnnamed_13 = 1;
pub const EXT_TX_SET_DCTONLY: C2RustUnnamed_13 = 0;
pub type TxSetType = uint8_t;
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type pthread_cond_t = libc::pthread_cond_t;
pub type pthread_condattr_t = libc::pthread_condattr_t;
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
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
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const MAX_PROFILES: C2RustUnnamed_14 = 3;
pub const PROFILE_2: C2RustUnnamed_14 = 2;
pub const PROFILE_1: C2RustUnnamed_14 = 1;
pub const PROFILE_0: C2RustUnnamed_14 = 0;
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
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const PARTITION_INVALID: C2RustUnnamed_15 = 255;
pub const PARTITION_TYPES: C2RustUnnamed_15 = 4;
pub const EXT_PARTITION_TYPES: C2RustUnnamed_15 = 10;
pub const PARTITION_VERT_4: C2RustUnnamed_15 = 9;
pub const PARTITION_HORZ_4: C2RustUnnamed_15 = 8;
pub const PARTITION_VERT_B: C2RustUnnamed_15 = 7;
pub const PARTITION_VERT_A: C2RustUnnamed_15 = 6;
pub const PARTITION_HORZ_B: C2RustUnnamed_15 = 5;
pub const PARTITION_HORZ_A: C2RustUnnamed_15 = 4;
pub const PARTITION_SPLIT: C2RustUnnamed_15 = 3;
pub const PARTITION_VERT: C2RustUnnamed_15 = 2;
pub const PARTITION_HORZ: C2RustUnnamed_15 = 1;
pub const PARTITION_NONE: C2RustUnnamed_15 = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const TX_MODES: C2RustUnnamed_16 = 3;
pub const TX_MODE_SELECT: C2RustUnnamed_16 = 2;
pub const TX_MODE_LARGEST: C2RustUnnamed_16 = 1;
pub const ONLY_4X4: C2RustUnnamed_16 = 0;
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_17 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_17 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_17 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const CFL_ALLOWED_TYPES: C2RustUnnamed_18 = 2;
pub const CFL_ALLOWED: C2RustUnnamed_18 = 1;
pub const CFL_DISALLOWED: C2RustUnnamed_18 = 0;
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_19 = 25;
pub const INTRA_MODES: C2RustUnnamed_19 = 13;
pub const INTER_MODE_END: C2RustUnnamed_19 = 25;
pub const INTER_MODE_START: C2RustUnnamed_19 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_19 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_19 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_19 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_19 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_19 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_19 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_19 = 13;
pub const DIR_MODE_END: C2RustUnnamed_19 = 9;
pub const DIR_MODE_START: C2RustUnnamed_19 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_19 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_19 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_19 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_19 = 25;
pub const NEW_NEWMV: C2RustUnnamed_19 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_19 = 23;
pub const NEW_NEARMV: C2RustUnnamed_19 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_19 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_19 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_19 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_19 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_19 = 17;
pub const NEWMV: C2RustUnnamed_19 = 16;
pub const GLOBALMV: C2RustUnnamed_19 = 15;
pub const NEARMV: C2RustUnnamed_19 = 14;
pub const NEARESTMV: C2RustUnnamed_19 = 13;
pub const PAETH_PRED: C2RustUnnamed_19 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_19 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_19 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_19 = 9;
pub const D67_PRED: C2RustUnnamed_19 = 8;
pub const D203_PRED: C2RustUnnamed_19 = 7;
pub const D157_PRED: C2RustUnnamed_19 = 6;
pub const D113_PRED: C2RustUnnamed_19 = 5;
pub const D135_PRED: C2RustUnnamed_19 = 4;
pub const D45_PRED: C2RustUnnamed_19 = 3;
pub const H_PRED: C2RustUnnamed_19 = 2;
pub const V_PRED: C2RustUnnamed_19 = 1;
pub const DC_PRED: C2RustUnnamed_19 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_20 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_20 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_20 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_20 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_20 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_20 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_20 = 9;
pub const UV_D67_PRED: C2RustUnnamed_20 = 8;
pub const UV_D203_PRED: C2RustUnnamed_20 = 7;
pub const UV_D157_PRED: C2RustUnnamed_20 = 6;
pub const UV_D113_PRED: C2RustUnnamed_20 = 5;
pub const UV_D135_PRED: C2RustUnnamed_20 = 4;
pub const UV_D45_PRED: C2RustUnnamed_20 = 3;
pub const UV_H_PRED: C2RustUnnamed_20 = 2;
pub const UV_V_PRED: C2RustUnnamed_20 = 1;
pub const UV_DC_PRED: C2RustUnnamed_20 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_21 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_21 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_21 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_21 = 0;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_22 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_22 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_22 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_22 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_22 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_22 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_23 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_23 = 7;
pub const BWD_REFS: C2RustUnnamed_23 = 3;
pub const FWD_REFS: C2RustUnnamed_23 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_23 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_23 = 8;
pub const REF_FRAMES: C2RustUnnamed_23 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_23 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_23 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_23 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_23 = 4;
pub const LAST3_FRAME: C2RustUnnamed_23 = 3;
pub const LAST2_FRAME: C2RustUnnamed_23 = 2;
pub const LAST_FRAME: C2RustUnnamed_23 = 1;
pub const INTRA_FRAME: C2RustUnnamed_23 = 0;
pub const NONE_FRAME: C2RustUnnamed_23 = -1;
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
pub type AV1_COMMON = AV1Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub above: PARTITION_CONTEXT,
    pub left: PARTITION_CONTEXT,
}
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_25 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_25 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_25 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_25 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_25 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_25 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_25 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_25 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_25 = 0;
pub type SEG_LVL_FEATURES = uint8_t;
pub type C2RustUnnamed_26 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_26 = 4;
pub const AFFINE: C2RustUnnamed_26 = 3;
pub const ROTZOOM: C2RustUnnamed_26 = 2;
pub const TRANSLATION: C2RustUnnamed_26 = 1;
pub const IDENTITY: C2RustUnnamed_26 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv32 {
    pub row: int32_t,
    pub col: int32_t,
}
pub type MV32 = mv32;
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
pub struct WarpTypesAllowed {
    pub global_warp_allowed: core::ffi::c_int,
    pub local_warp_allowed: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilterParams {
    pub filter_ptr: *const int16_t,
    pub taps: uint16_t,
    pub interp_filter: InterpFilter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConvolveParams {
    pub do_average: core::ffi::c_int,
    pub dst: *mut CONV_BUF_TYPE,
    pub dst_stride: core::ffi::c_int,
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub is_compound: core::ffi::c_int,
    pub use_dist_wtd_comp_avg: core::ffi::c_int,
    pub fwd_offset: core::ffi::c_int,
    pub bck_offset: core::ffi::c_int,
}
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_27 = 4;
pub const S_FRAME: C2RustUnnamed_27 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_27 = 2;
pub const INTER_FRAME: C2RustUnnamed_27 = 1;
pub const KEY_FRAME: C2RustUnnamed_27 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUFFER_SET {
    pub plane: [*mut uint8_t; 3],
    pub stride: [core::ffi::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CB_BUFFER {
    pub __aom_align_dqcoeff: [crate::platform::Align32; 0],

    pub dqcoeff: [[tran_low_t; 16384]; 3],
    pub eob_data: [[eob_info; 1024]; 3],
    pub __aom_align_color_index_map: [crate::platform::Align16; 0],
    pub color_index_map: [[uint8_t; 16384]; 2],
}
pub type MACROBLOCKD_PLANE = macroblockd_plane;
pub type MapCdf = *mut [[aom_cdf_prob; 9]; 5];
pub type ColorCost = *const [[core::ffi::c_int; 8]; 5];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1ColorMapParam {
    pub rows: core::ffi::c_int,
    pub cols: core::ffi::c_int,
    pub n_colors: core::ffi::c_int,
    pub plane_width: core::ffi::c_int,
    pub plane_height: core::ffi::c_int,
    pub color_map: *mut uint8_t,
    pub map_cdf: MapCdf,
    pub color_cost: ColorCost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sgr_params_type {
    pub r: [core::ffi::c_int; 2],
    pub s: [core::ffi::c_int; 2],
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
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_28 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_28 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_28 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const REFRESH_FRAME_CONTEXT_BACKWARD: C2RustUnnamed_29 = 1;
pub const REFRESH_FRAME_CONTEXT_DISABLED: C2RustUnnamed_29 = 0;
pub type overlappable_nb_visitor_t = Option<
    unsafe extern "C-unwind" fn(
        *mut MACROBLOCKD,
        core::ffi::c_int,
        core::ffi::c_int,
        uint8_t,
        core::ffi::c_int,
        *mut MB_MODE_INFO,
        *mut core::ffi::c_void,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpelParams {
    pub xs: core::ffi::c_int,
    pub ys: core::ffi::c_int,
    pub subpel_x: core::ffi::c_int,
    pub subpel_y: core::ffi::c_int,
    pub pos_x: core::ffi::c_int,
    pub pos_y: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct build_prediction_ctxt {
    pub cm: *const AV1_COMMON,
    pub tmp_buf: *mut *mut uint8_t,
    pub tmp_width: *mut core::ffi::c_int,
    pub tmp_height: *mut core::ffi::c_int,
    pub tmp_stride: *mut core::ffi::c_int,
    pub mb_to_far_edge: core::ffi::c_int,
    pub dcb: *mut core::ffi::c_void,
}
pub type InterPredMode = core::ffi::c_uint;
pub const WARP_PRED: InterPredMode = 1;
pub const TRANSLATION_PRED: InterPredMode = 0;
pub type InterCompMode = core::ffi::c_uint;
pub const MASK_COMP: InterCompMode = 2;
pub const UNIFORM_COMP: InterCompMode = 1;
pub const UNIFORM_SINGLE: InterCompMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterPredParams {
    pub mode: InterPredMode,
    pub comp_mode: InterCompMode,
    pub warp_params: WarpedMotionParams,
    pub conv_params: ConvolveParams,
    pub interp_filter_params: [*const InterpFilterParams; 2],
    pub block_width: core::ffi::c_int,
    pub block_height: core::ffi::c_int,
    pub pix_row: core::ffi::c_int,
    pub pix_col: core::ffi::c_int,
    pub ref_frame_buf: buf_2d,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub scale_factors: *const scale_factors,
    pub bit_depth: core::ffi::c_int,
    pub use_hbd_buf: core::ffi::c_int,
    pub mask_comp: INTERINTER_COMPOUND_DATA,
    pub sb_type: BLOCK_SIZE,
    pub is_intrabc: core::ffi::c_int,
    pub top: core::ffi::c_int,
    pub left: core::ffi::c_int,
}
pub type AV1LfSync = AV1LfSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Decoder {
    pub dcb: DecoderCodingBlock,
    pub __aom_align_common: [crate::platform::Align32; 0],
    pub common: AV1_COMMON,
    pub lf_worker: AVxWorker,
    pub lf_row_sync: AV1LfSync,
    pub lr_row_sync: AV1LrSync,
    pub lr_ctxt: AV1LrStruct,
    pub cdef_sync: AV1CdefSync,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub tile_workers: *mut AVxWorker,
    pub num_workers: core::ffi::c_int,
    pub thread_data: *mut DecWorkerData,
    pub td: ThreadData,
    pub tile_data: *mut TileDataDec,
    pub allocated_tiles: core::ffi::c_int,
    pub tile_buffers: [[TileBufferDec; 64]; 64],
    pub tile_mt_info: AV1DecTileMT,
    pub output_all_layers: core::ffi::c_int,
    pub output_frames: [*mut RefCntBuffer; 4],
    pub num_output_frames: size_t,
    pub decoding_first_frame: core::ffi::c_int,
    pub allow_lowbitdepth: core::ffi::c_int,
    pub max_threads: core::ffi::c_int,
    pub inv_tile_order: core::ffi::c_int,
    pub need_resync: core::ffi::c_int,
    pub reset_decoder_state: core::ffi::c_int,
    pub tile_size_bytes: core::ffi::c_int,
    pub tile_col_size_bytes: core::ffi::c_int,
    pub dec_tile_row: core::ffi::c_int,
    pub dec_tile_col: core::ffi::c_int,
    pub sequence_header_ready: core::ffi::c_int,
    pub sequence_header_changed: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub current_operating_point: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub seen_frame_header: core::ffi::c_int,
    pub next_start_tile: core::ffi::c_int,
    pub camera_frame_header_ready: core::ffi::c_int,
    pub frame_header_size: size_t,
    pub obu_size_hdr: DataBuffer,
    pub output_frame_width_in_tiles_minus_1: core::ffi::c_int,
    pub output_frame_height_in_tiles_minus_1: core::ffi::c_int,
    pub tile_count_minus_1: core::ffi::c_int,
    pub coded_tile_data_size: uint32_t,
    pub ext_tile_debug: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub ext_refs: EXTERNAL_REFERENCES,
    pub tile_list_outbuf: YV12_BUFFER_CONFIG,
    pub cb_buffer_base: *mut CB_BUFFER,
    pub cb_buffer_alloc_size: core::ffi::c_int,
    pub allocated_row_mt_sync_rows: core::ffi::c_int,
    pub row_mt_mutex_: *mut pthread_mutex_t,
    pub row_mt_cond_: *mut pthread_cond_t,
    pub frame_row_mt_info: AV1DecRowMTInfo,
    pub metadata: *mut aom_metadata_array_t,
    pub context_update_tile_id: core::ffi::c_int,
    pub skip_loop_filter: core::ffi::c_int,
    pub skip_film_grain: core::ffi::c_int,
    pub is_annexb: core::ffi::c_int,
    pub valid_for_referencing: [core::ffi::c_int; 8],
    pub is_fwd_kf_present: core::ffi::c_int,
    pub is_arf_frame_present: core::ffi::c_int,
    pub num_tile_groups: core::ffi::c_int,
    pub sframe_info: aom_s_frame_info,
    pub seq_params: SequenceHeader,
    pub buffer_removal_time_present: bool,
    pub error: aom_internal_error_info,
    pub number_temporal_layers: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTInfo {
    pub tile_rows_start: core::ffi::c_int,
    pub tile_rows_end: core::ffi::c_int,
    pub tile_cols_start: core::ffi::c_int,
    pub tile_cols_end: core::ffi::c_int,
    pub start_tile: core::ffi::c_int,
    pub end_tile: core::ffi::c_int,
    pub mi_rows_to_decode: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub row_mt_exit: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXTERNAL_REFERENCES {
    pub refs: [YV12_BUFFER_CONFIG; 128],
    pub num: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DataBuffer {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type AV1DecTileMT = AV1DecTileMTData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecTileMTData {
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut TileJobsDec,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub alloc_tile_rows: core::ffi::c_int,
    pub alloc_tile_cols: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileJobsDec {
    pub tile_buffer: *mut TileBufferDec,
    pub tile_data: *mut TileDataDec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileDataDec {
    pub tile_info: TileInfo,
    pub bit_reader: aom_reader,
    pub __aom_align_tctx: [crate::platform::Align16; 0],
    pub tctx: FRAME_CONTEXT,
    pub dec_row_mt_sync: AV1DecRowMTSync,
}
pub type AV1DecRowMTSync = AV1DecRowMTSyncData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub allocated_sb_rows: core::ffi::c_int,
    pub cur_sb_col: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub intrabc_extra_top_right_sb_delay: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileBufferDec {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadData {
    pub dcb: DecoderCodingBlock,
    pub cb_buffer_base: CB_BUFFER,
    pub bit_reader: *mut aom_reader,
    pub mc_buf: [*mut uint8_t; 2],
    pub seg_mask: *mut uint8_t,
    pub mc_buf_size: int32_t,
    pub mc_buf_use_highbd: core::ffi::c_int,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub read_coeffs_tx_intra_block_visit: decode_block_visitor_fn_t,
    pub predict_and_recon_intra_block_visit: decode_block_visitor_fn_t,
    pub read_coeffs_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub inverse_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub predict_inter_block_visit: predict_inter_block_visitor_fn_t,
    pub cfl_store_inter_block_visit: cfl_store_inter_block_visitor_fn_t,
}
pub type cfl_store_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
>;
pub type predict_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut DecoderCodingBlock, BLOCK_SIZE) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderCodingBlock {
    pub __aom_align_xd: [crate::platform::Align32; 0],

    pub xd: MACROBLOCKD,
    pub corrupted: core::ffi::c_int,
    pub mc_buf: [*mut uint8_t; 2],
    pub dqcoeff_block: [*mut tran_low_t; 3],
    pub cb_offset: [uint16_t; 3],
    pub eob_data: [*mut eob_info; 3],
    pub txb_offset: [uint16_t; 3],
    pub ref_mv_count: [uint8_t; 29],
}
pub type decode_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *mut DecoderCodingBlock,
        *mut aom_reader,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        TX_SIZE,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecWorkerData {
    pub td: *mut ThreadData,
    pub data_end: *const uint8_t,
    pub error_info: aom_internal_error_info,
}
pub type block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut AV1Decoder,
        *mut ThreadData,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut aom_reader,
        PARTITION_TYPE,
        BLOCK_SIZE,
    ) -> (),
>;
pub type palette_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut MACROBLOCKD, core::ffi::c_int, *mut aom_reader) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PadBlock {
    pub x0: core::ffi::c_int,
    pub x1: core::ffi::c_int,
    pub y0: core::ffi::c_int,
    pub y1: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTJobInfo {
    pub tile_row: core::ffi::c_int,
    pub tile_col: core::ffi::c_int,
    pub mi_row: core::ffi::c_int,
}
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const CHAR_BIT: core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
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
#[inline]
unsafe extern "C-unwind" fn aom_read_(
    mut r: *mut aom_reader,
    mut prob: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut p: core::ffi::c_int = 0x7fffff as core::ffi::c_int
        - (prob << 15 as core::ffi::c_int) + prob >> 8 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = od_ec_decode_bool_q15(
        &raw mut (*r).ec,
        p as core::ffi::c_uint,
    );
    return bit;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_bit_(mut r: *mut aom_reader) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    ret = aom_read_(r, 128 as core::ffi::c_int);
    return ret;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_literal_(
    mut r: *mut aom_reader,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut literal: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        literal |= aom_read_bit_(r) << bit;
        bit -= 1;
    }
    return literal;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_cdf_(
    mut r: *mut aom_reader,
    mut cdf: *const aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut symb: core::ffi::c_int = 0;
    if cdf.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_read_cdf_\0"))
                .as_ptr(),
            b"bitreader.h\0" as *const u8 as *const core::ffi::c_char,
            171 as core::ffi::c_int,
            b"cdf != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    symb = od_ec_decode_cdf_q15(&raw mut (*r).ec, cdf as *const uint16_t, nsymbs);
    return symb;
}
#[inline]
unsafe extern "C-unwind" fn aom_read_symbol_(
    mut r: *mut aom_reader,
    mut cdf: *mut aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    ret = aom_read_cdf_(r, cdf, nsymbs);
    if (*r).allow_update_cdf != 0 {
        update_cdf(cdf, ret as int8_t, nsymbs);
    }
    return ret;
}
#[inline]
unsafe extern "C-unwind" fn aom_memset16(
    mut dest: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
    mut length: size_t,
) -> *mut core::ffi::c_void {
    let mut i: size_t = 0;
    let mut dest16: *mut uint16_t = dest as *mut uint16_t;
    i = 0 as size_t;
    while i < length {
        let fresh3 = dest16;
        dest16 = dest16.offset(1);
        *fresh3 = val as uint16_t;
        i = i.wrapping_add(1);
    }
    return dest;
}
unsafe extern "C-unwind" fn mem_get_le16_as_int(
    mut vmem: *const core::ffi::c_void,
) -> core::ffi::c_uint {
    let mut val: core::ffi::c_uint = 0;
    let mut mem: *const core::ffi::c_uchar = vmem as *const core::ffi::c_uchar;
    val = ((*mem.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 8 as core::ffi::c_int) as core::ffi::c_uint;
    val |= *mem.offset(0 as core::ffi::c_int as isize) as core::ffi::c_uint;
    return val;
}
unsafe extern "C-unwind" fn mem_get_le24_as_int(
    mut vmem: *const core::ffi::c_void,
) -> core::ffi::c_uint {
    let mut val: core::ffi::c_uint = 0;
    let mut mem: *const core::ffi::c_uchar = vmem as *const core::ffi::c_uchar;
    val = ((*mem.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 16 as core::ffi::c_int) as core::ffi::c_uint;
    val
        |= ((*mem.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 8 as core::ffi::c_int) as core::ffi::c_uint;
    val |= *mem.offset(0 as core::ffi::c_int as isize) as core::ffi::c_uint;
    return val;
}
unsafe extern "C-unwind" fn mem_get_le32_as_int(
    mut vmem: *const core::ffi::c_void,
) -> core::ffi::c_uint {
    let mut val: core::ffi::c_uint = 0;
    let mut mem: *const core::ffi::c_uchar = vmem as *const core::ffi::c_uchar;
    val = (*mem.offset(3 as core::ffi::c_int as isize) as core::ffi::c_uint)
        << 24 as core::ffi::c_int;
    val
        |= ((*mem.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 16 as core::ffi::c_int) as core::ffi::c_uint;
    val
        |= ((*mem.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 8 as core::ffi::c_int) as core::ffi::c_uint;
    val |= *mem.offset(0 as core::ffi::c_int as isize) as core::ffi::c_uint;
    return val;
}
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const AOM_DEC_BORDER_IN_PIXELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const __CHAR_BIT__: core::ffi::c_int = 8 as core::ffi::c_int;
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_SB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MAX_MIB_MASK: core::ffi::c_int = MAX_MIB_SIZE - 1 as core::ffi::c_int;
pub const MAX_TILE_ROWS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_TILE_COLS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_VARTX_DEPTH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_MODE_LF_DELTAS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PROFILE_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SQR_BLOCK_SIZES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PARTITION_PLOFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const REF_FRAMES_LOG2: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUPERRES_SCALE_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUPERRES_SCALE_DENOMINATOR_MIN: core::ffi::c_int = SCALE_NUMERATOR
    + 1 as core::ffi::c_int;
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
pub const TX_SIZE_W_MIN: core::ffi::c_int = 4 as core::ffi::c_int;
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
pub const TX_SIZE_H_MIN: core::ffi::c_int = 4 as core::ffi::c_int;
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
static mut tx_size_wide_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
];
static mut tx_size_high_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    3 as core::ffi::c_int,
    6 as core::ffi::c_int,
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
static mut partition_context_lookup: [C2RustUnnamed_24; 22] = [
    {
        let mut init = C2RustUnnamed_24 {
            above: 31 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 31 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 30 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 30 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 30 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 28 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 28 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 28 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 24 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 24 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 24 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 16 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 16 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 16 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 0 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 0 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 31 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 28 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 30 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 24 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 28 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_24 {
            above: 16 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
];
pub const MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn segfeatures_copy(
    mut dst: *mut segmentation,
    mut src: *const segmentation,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < MAX_SEGMENTS {
        (*dst).feature_mask[i as usize] = (*src).feature_mask[i as usize];
        j = 0 as core::ffi::c_int;
        while j < SEG_LVL_MAX as core::ffi::c_int {
            (*dst).feature_data[i as usize][j as usize] = (*src)
                .feature_data[i as usize][j as usize];
            j += 1;
        }
        i += 1;
    }
    (*dst).segid_preskip = (*src).segid_preskip;
    (*dst).last_active_segid = (*src).last_active_segid;
}
#[inline]
unsafe extern "C-unwind" fn get_unsigned_bits(
    mut num_values: core::ffi::c_uint,
) -> core::ffi::c_int {
    return if num_values > 0 as core::ffi::c_uint {
        get_msb(num_values) + 1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
pub const AV1_MIN_TILE_SIZE_BYTES: core::ffi::c_int = 1 as core::ffi::c_int;
pub const QINDEX_BITS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const QM_LEVEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NUM_QM_LEVELS: core::ffi::c_int = (1 as core::ffi::c_int) << QM_LEVEL_BITS;
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << SUBPEL_BITS)
    - 1 as core::ffi::c_int;
pub const SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int) << SUBPEL_BITS;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int)
    << SCALE_SUBPEL_BITS;
pub const SCALE_SUBPEL_MASK: core::ffi::c_int = SCALE_SUBPEL_SHIFTS
    - 1 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const SCALE_EXTRA_OFF: core::ffi::c_int = ((1 as core::ffi::c_int)
    << SCALE_EXTRA_BITS) / 2 as core::ffi::c_int;
pub const WARPEDMODEL_PREC_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
static mut default_warp_params: WarpedMotionParams = {
    let mut init = WarpedMotionParams {
        wmmat: [
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS,
        ],
        alpha: 0 as int16_t,
        beta: 0 as int16_t,
        gamma: 0 as int16_t,
        delta: 0 as int16_t,
        wmtype: IDENTITY as core::ffi::c_int as TransformationType,
        invalid: 0 as int8_t,
    };
    init
};
pub const SUBEXPFIN_K: core::ffi::c_int = 3 as core::ffi::c_int;
pub const GM_TRANS_PREC_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const GM_ABS_TRANS_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const GM_ABS_TRANS_ONLY_BITS: core::ffi::c_int = GM_ABS_TRANS_BITS
    - GM_TRANS_PREC_BITS + 3 as core::ffi::c_int;
pub const GM_TRANS_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - GM_TRANS_PREC_BITS;
pub const GM_TRANS_ONLY_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - 3 as core::ffi::c_int;
pub const GM_TRANS_DECODE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << GM_TRANS_PREC_DIFF;
pub const GM_TRANS_ONLY_DECODE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << GM_TRANS_ONLY_PREC_DIFF;
pub const GM_ALPHA_PREC_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const GM_ABS_ALPHA_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const GM_ALPHA_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - GM_ALPHA_PREC_BITS;
pub const GM_ALPHA_DECODE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << GM_ALPHA_PREC_DIFF;
pub const GM_ALPHA_MAX: core::ffi::c_int = (1 as core::ffi::c_int) << GM_ABS_ALPHA_BITS;
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
pub const LOG_SWITCHABLE_FILTERS: core::ffi::c_int = 2 as core::ffi::c_int;
static mut av1_bilinear_filters: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        104 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        104 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        122 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        110 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        102 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        94 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        102 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        122 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8sharp: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        124 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        120 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        100 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        90 as core::ffi::c_int as int16_t,
        70 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        80 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        70 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        60 as core::ffi::c_int as int16_t,
        100 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        124 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8smooth: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_12sharp: crate::platform::Aligned256<[[int16_t; 12]; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        127 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        124 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        120 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        115 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        100 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(26 as core::ffi::c_int) as int16_t,
        91 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        81 as core::ffi::c_int as int16_t,
        81 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        71 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        -(26 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        60 as core::ffi::c_int as int16_t,
        100 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        49 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        115 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        124 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        127 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_interp_filter_params_list: [InterpFilterParams; 5] = unsafe {
    [
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8smooth.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_SMOOTH,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8sharp.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: MULTITAP_SHARP,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_bilinear_filters.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: BILINEAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_12sharp.0).cast::<[int16_t; 12]>() as *const int16_t,
                taps: 12 as uint16_t,
                interp_filter: MULTITAP_SHARP2,
            };
            init
        },
    ]
};
static mut av1_intrabc_bilinear_filter: crate::platform::Aligned256<[int16_t; 32]> = crate::platform::Aligned256([
    128 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    64 as core::ffi::c_int as int16_t,
    64 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
]);
static mut av1_intrabc_filter_params: InterpFilterParams = unsafe {
    {
        let mut init = InterpFilterParams {
            filter_ptr: core::ptr::addr_of!(av1_intrabc_bilinear_filter.0).cast::<int16_t>(),
            taps: 2 as uint16_t,
            interp_filter: BILINEAR,
        };
        init
    }
};
static mut av1_sub_pel_filters_4: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        122 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        110 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        102 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        94 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        102 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        122 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_4smooth: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_interp_4tap: [InterpFilterParams; 4] = unsafe {
    [
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4smooth.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_SMOOTH,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_bilinear_filters.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: BILINEAR,
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C-unwind" fn av1_get_interp_filter_params_with_block_size(
    interp_filter: InterpFilter,
    w: core::ffi::c_int,
) -> *const InterpFilterParams {
    if w <= 4 as core::ffi::c_int
        && interp_filter as core::ffi::c_int != MULTITAP_SHARP2 as core::ffi::c_int
    {
        return &*core::ptr::addr_of!(av1_interp_4tap).cast::<InterpFilterParams>().offset(interp_filter as isize)
            as *const InterpFilterParams;
    }
    return &*core::ptr::addr_of!(av1_interp_filter_params_list).cast::<InterpFilterParams>().offset(interp_filter as isize)
        as *const InterpFilterParams;
}
pub const ROUND0_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const COMPOUND_ROUND1_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_conv_params_no_round(
    mut cmp_index: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut dst: *mut CONV_BUF_TYPE,
    mut dst_stride: core::ffi::c_int,
    mut is_compound: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    let mut conv_params: ConvolveParams = ConvolveParams {
        do_average: 0,
        dst: 0 as *mut CONV_BUF_TYPE,
        dst_stride: 0,
        round_0: 0,
        round_1: 0,
        plane: 0,
        is_compound: 0,
        use_dist_wtd_comp_avg: 0,
        fwd_offset: 0,
        bck_offset: 0,
    };
    if !(cmp_index == 0 || is_compound != 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"get_conv_params_no_round\0"))
                .as_ptr(),
            b"convolve.h\0" as *const u8 as *const core::ffi::c_char,
            73 as core::ffi::c_int,
            b"IMPLIES(cmp_index, is_compound)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    conv_params.is_compound = is_compound;
    conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    conv_params.round_0 = ROUND0_BITS;
    conv_params.round_1 = if is_compound != 0 {
        COMPOUND_ROUND1_BITS
    } else {
        2 as core::ffi::c_int * FILTER_BITS - conv_params.round_0
    };
    let intbufrange: core::ffi::c_int = bd + FILTER_BITS - conv_params.round_0
        + 2 as core::ffi::c_int;
    if !(!(bd < 12 as core::ffi::c_int) || intbufrange <= 16 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"get_conv_params_no_round\0"))
                .as_ptr(),
            b"convolve.h\0" as *const u8 as *const core::ffi::c_char,
            82 as core::ffi::c_int,
            b"IMPLIES(bd < 12, intbufrange <= 16)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if intbufrange > 16 as core::ffi::c_int {
        conv_params.round_0 += intbufrange - 16 as core::ffi::c_int;
        if is_compound == 0 {
            conv_params.round_1 -= intbufrange - 16 as core::ffi::c_int;
        }
    }
    conv_params.dst = dst;
    conv_params.dst_stride = dst_stride;
    conv_params.plane = plane;
    conv_params.do_average = cmp_index;
    return conv_params;
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
unsafe extern "C-unwind" fn has_second_ref(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn intra_mode_to_tx_type(
    mut mbmi: *const MB_MODE_INFO,
    mut plane_type: PLANE_TYPE,
) -> TX_TYPE {
    static mut _intra_mode_to_tx_type: [TX_TYPE; 13] = [
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        DCT_DCT as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
        ADST_DCT as core::ffi::c_int as TX_TYPE,
        DCT_ADST as core::ffi::c_int as TX_TYPE,
        ADST_ADST as core::ffi::c_int as TX_TYPE,
    ];
    let mode: PREDICTION_MODE = (if plane_type as core::ffi::c_int
        == PLANE_TYPE_Y as core::ffi::c_int
    {
        (*mbmi).mode as core::ffi::c_int
    } else {
        get_uv_mode((*mbmi).uv_mode) as core::ffi::c_int
    }) as PREDICTION_MODE;
    if !((mode as core::ffi::c_int) < INTRA_MODES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"intra_mode_to_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1026 as core::ffi::c_int,
            b"mode < INTRA_MODES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return _intra_mode_to_tx_type[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn block_signals_txsize(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int > BLOCK_4X4 as core::ffi::c_int)
        as core::ffi::c_int;
}
static mut av1_ext_tx_used: [[core::ffi::c_int; 16]; 6] = [
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
];
static mut av1_ext_tx_set_lookup: [[TxSetType; 2]; 2] = [
    [
        EXT_TX_SET_DTT4_IDTX_1DDCT as core::ffi::c_int as TxSetType,
        EXT_TX_SET_DTT4_IDTX as core::ffi::c_int as TxSetType,
    ],
    [
        EXT_TX_SET_ALL16 as core::ffi::c_int as TxSetType,
        EXT_TX_SET_DTT9_IDTX_1DDCT as core::ffi::c_int as TxSetType,
    ],
];
#[inline]
unsafe extern "C-unwind" fn av1_get_ext_tx_set_type(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> TxSetType {
    let tx_size_sqr_up: TX_SIZE = txsize_sqr_up_map[tx_size as usize];
    if tx_size_sqr_up as core::ffi::c_int > TX_32X32 as core::ffi::c_int {
        return EXT_TX_SET_DCTONLY as core::ffi::c_int as TxSetType;
    }
    if tx_size_sqr_up as core::ffi::c_int == TX_32X32 as core::ffi::c_int {
        return (if is_inter != 0 {
            EXT_TX_SET_DCT_IDTX as core::ffi::c_int
        } else {
            EXT_TX_SET_DCTONLY as core::ffi::c_int
        }) as TxSetType;
    }
    if use_reduced_set != 0 {
        return (if is_inter != 0 {
            EXT_TX_SET_DCT_IDTX as core::ffi::c_int
        } else {
            EXT_TX_SET_DTT4_IDTX as core::ffi::c_int
        }) as TxSetType;
    }
    let tx_size_sqr: TX_SIZE = txsize_sqr_map[tx_size as usize];
    return av1_ext_tx_set_lookup[is_inter
        as usize][(tx_size_sqr as core::ffi::c_int == TX_16X16 as core::ffi::c_int)
        as core::ffi::c_int as usize];
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
unsafe extern "C-unwind" fn av1_get_tx_type(
    mut xd: *const MACROBLOCKD,
    mut plane_type: PLANE_TYPE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut reduced_tx_set: core::ffi::c_int,
) -> TX_TYPE {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0
        || txsize_sqr_up_map[tx_size as usize] as core::ffi::c_int
            > TX_32X32 as core::ffi::c_int
    {
        return DCT_DCT as core::ffi::c_int as TX_TYPE;
    }
    let mut tx_type: TX_TYPE = 0;
    if plane_type as core::ffi::c_int == PLANE_TYPE_Y as core::ffi::c_int {
        tx_type = *((*xd).tx_type_map)
            .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize) as TX_TYPE;
    } else {
        if is_inter_block(mbmi) != 0 {
            let pd: *const macroblockd_plane = &*((*xd).plane)
                .as_ptr()
                .offset(plane_type as isize) as *const macroblockd_plane;
            blk_row <<= (*pd).subsampling_y;
            blk_col <<= (*pd).subsampling_x;
            tx_type = *((*xd).tx_type_map)
                .offset((blk_row * (*xd).tx_type_map_stride + blk_col) as isize)
                as TX_TYPE;
        } else {
            tx_type = intra_mode_to_tx_type(
                mbmi,
                PLANE_TYPE_UV as core::ffi::c_int as PLANE_TYPE,
            );
        }
        let tx_set_type: TxSetType = av1_get_ext_tx_set_type(
            tx_size,
            is_inter_block(mbmi),
            reduced_tx_set,
        ) as TxSetType;
        if av1_ext_tx_used[tx_set_type as usize][tx_type as usize] == 0 {
            tx_type = DCT_DCT as core::ffi::c_int as TX_TYPE;
        }
    }
    if !((tx_type as core::ffi::c_int) < TX_TYPES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_get_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1311 as core::ffi::c_int,
            b"tx_type < TX_TYPES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (av1_ext_tx_used[av1_get_ext_tx_set_type(
        tx_size,
        is_inter_block(mbmi),
        reduced_tx_set,
    ) as usize][tx_type as usize] == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"av1_get_tx_type\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1313 as core::ffi::c_int,
            b"av1_ext_tx_used[av1_get_ext_tx_set_type(tx_size, is_inter_block(mbmi), reduced_tx_set)][tx_type]\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return tx_type;
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
unsafe extern "C-unwind" fn depth_to_tx_size(
    mut depth: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> TX_SIZE {
    let mut max_tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    let mut tx_size: TX_SIZE = max_tx_size;
    let mut d: core::ffi::c_int = 0 as core::ffi::c_int;
    while d < depth {
        tx_size = sub_tx_size_map[tx_size as usize];
        d += 1;
    }
    return tx_size;
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
unsafe extern "C-unwind" fn av1_get_max_uv_txsize(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> TX_SIZE {
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        subsampling_x,
        subsampling_y,
    ) as BLOCK_SIZE;
    if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_get_max_uv_txsize\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1381 as core::ffi::c_int,
            b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let uv_tx: TX_SIZE = max_txsize_rect_lookup[plane_bsize as usize];
    return av1_get_adjusted_tx_size(uv_tx);
}
#[inline]
unsafe extern "C-unwind" fn av1_get_tx_size(
    mut plane: core::ffi::c_int,
    mut xd: *const MACROBLOCKD,
) -> TX_SIZE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        return TX_4X4 as core::ffi::c_int as TX_SIZE;
    }
    if plane == 0 as core::ffi::c_int {
        return (*mbmi).tx_size;
    }
    let mut pd: *const MACROBLOCKD_PLANE = &*((*xd).plane)
        .as_ptr()
        .offset(plane as isize) as *const MACROBLOCKD_PLANE;
    return av1_get_max_uv_txsize(
        (*mbmi).bsize,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
    );
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
unsafe extern "C-unwind" fn is_interintra_pred(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == INTRA_FRAME as core::ffi::c_int && is_interintra_allowed(mbmi) != 0)
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
static mut max_neighbor_obmc: [core::ffi::c_int; 6] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
#[inline]
unsafe extern "C-unwind" fn is_neighbor_overlappable(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return is_inter_block(mbmi);
}
#[inline]
unsafe extern "C-unwind" fn av1_get_block_dimensions(
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut xd: *const MACROBLOCKD,
    mut width: *mut core::ffi::c_int,
    mut height: *mut core::ffi::c_int,
    mut rows_within_bounds: *mut core::ffi::c_int,
    mut cols_within_bounds: *mut core::ffi::c_int,
) {
    let block_height: core::ffi::c_int = block_size_high[bsize as usize]
        as core::ffi::c_int;
    let block_width: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    let block_rows: core::ffi::c_int = if (*xd).mb_to_bottom_edge
        >= 0 as core::ffi::c_int
    {
        block_height
    } else {
        ((*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int) + block_height
    };
    let block_cols: core::ffi::c_int = if (*xd).mb_to_right_edge >= 0 as core::ffi::c_int
    {
        block_width
    } else {
        ((*xd).mb_to_right_edge >> 3 as core::ffi::c_int) + block_width
    };
    let pd: *const macroblockd_plane = &*((*xd).plane).as_ptr().offset(plane as isize)
        as *const macroblockd_plane;
    if !(!(plane == PLANE_TYPE_Y as core::ffi::c_int)
        || (*pd).subsampling_x == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_block_dimensions\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1531 as core::ffi::c_int,
            b"IMPLIES(plane == PLANE_TYPE_Y, pd->subsampling_x == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(!(plane == PLANE_TYPE_Y as core::ffi::c_int)
        || (*pd).subsampling_y == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_block_dimensions\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1532 as core::ffi::c_int,
            b"IMPLIES(plane == PLANE_TYPE_Y, pd->subsampling_y == 0)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(block_width >= block_cols) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_block_dimensions\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1533 as core::ffi::c_int,
            b"block_width >= block_cols\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(block_height >= block_rows) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_block_dimensions\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1534 as core::ffi::c_int,
            b"block_height >= block_rows\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let plane_block_width: core::ffi::c_int = block_width >> (*pd).subsampling_x;
    let plane_block_height: core::ffi::c_int = block_height >> (*pd).subsampling_y;
    let is_chroma_sub8_x: core::ffi::c_int = (plane > 0 as core::ffi::c_int
        && plane_block_width < 4 as core::ffi::c_int) as core::ffi::c_int;
    let is_chroma_sub8_y: core::ffi::c_int = (plane > 0 as core::ffi::c_int
        && plane_block_height < 4 as core::ffi::c_int) as core::ffi::c_int;
    if !width.is_null() {
        *width = plane_block_width + 2 as core::ffi::c_int * is_chroma_sub8_x;
        if !(*width >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"av1_get_block_dimensions\0"))
                    .as_ptr(),
                b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
                1542 as core::ffi::c_int,
                b"*width >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if !height.is_null() {
        *height = plane_block_height + 2 as core::ffi::c_int * is_chroma_sub8_y;
        if !(*height >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"av1_get_block_dimensions\0"))
                    .as_ptr(),
                b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
                1546 as core::ffi::c_int,
                b"*height >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if !rows_within_bounds.is_null() {
        *rows_within_bounds = (block_rows >> (*pd).subsampling_y)
            + 2 as core::ffi::c_int * is_chroma_sub8_y;
        if !(*rows_within_bounds >= 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"av1_get_block_dimensions\0"))
                    .as_ptr(),
                b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
                1551 as core::ffi::c_int,
                b"*rows_within_bounds >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if !cols_within_bounds.is_null() {
        *cols_within_bounds = (block_cols >> (*pd).subsampling_x)
            + 2 as core::ffi::c_int * is_chroma_sub8_x;
        if !(*cols_within_bounds >= 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"av1_get_block_dimensions\0"))
                    .as_ptr(),
                b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
                1556 as core::ffi::c_int,
                b"*cols_within_bounds >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
}
#[inline]
unsafe extern "C-unwind" fn get_plane_type(mut plane: core::ffi::c_int) -> PLANE_TYPE {
    return (if plane == 0 as core::ffi::c_int {
        PLANE_TYPE_Y as core::ffi::c_int
    } else {
        PLANE_TYPE_UV as core::ffi::c_int
    }) as PLANE_TYPE;
}
pub const WIENER_HALFWIN: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RESTORATION_UNITSIZE_MAX: core::ffi::c_int = 256 as core::ffi::c_int;
pub const SGRPROJ_PRJ_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN0: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) * 3 as core::ffi::c_int / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN1: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MAX1: core::ffi::c_int = SGRPROJ_PRJ_MIN1
    + ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS) - 1 as core::ffi::c_int;
pub const WIENER_WIN: core::ffi::c_int = 2 as core::ffi::c_int * WIENER_HALFWIN
    + 1 as core::ffi::c_int;
pub const WIENER_WIN_CHROMA: core::ffi::c_int = WIENER_WIN - 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP0_MIDV: core::ffi::c_int = 3 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_MIDV: core::ffi::c_int = -(7 as core::ffi::c_int);
pub const WIENER_FILT_TAP2_MIDV: core::ffi::c_int = 15 as core::ffi::c_int;
pub const WIENER_FILT_TAP0_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const WIENER_FILT_TAP2_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const WIENER_FILT_TAP0_MINV: core::ffi::c_int = WIENER_FILT_TAP0_MIDV
    - ((1 as core::ffi::c_int) << WIENER_FILT_TAP0_BITS) / 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_MINV: core::ffi::c_int = WIENER_FILT_TAP1_MIDV
    - ((1 as core::ffi::c_int) << WIENER_FILT_TAP1_BITS) / 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP2_MINV: core::ffi::c_int = WIENER_FILT_TAP2_MIDV
    - ((1 as core::ffi::c_int) << WIENER_FILT_TAP2_BITS) / 2 as core::ffi::c_int;
pub const PRIMARY_REF_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const PRIMARY_REF_NONE: core::ffi::c_int = 7 as core::ffi::c_int;
unsafe extern "C-unwind" fn lock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_lock(&raw mut (*pool).pool_mutex);
}
unsafe extern "C-unwind" fn unlock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_unlock(&raw mut (*pool).pool_mutex);
}
#[inline]
unsafe extern "C-unwind" fn get_free_fb(mut cm: *mut AV1_COMMON) -> core::ffi::c_int {
    let frame_bufs: *mut RefCntBuffer = (*(*cm).buffer_pool).frame_bufs;
    let mut i: core::ffi::c_int = 0;
    lock_buffer_pool((*cm).buffer_pool);
    let num_frame_bufs: core::ffi::c_int = (*(*cm).buffer_pool).num_frame_bufs
        as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < num_frame_bufs {
        if (*frame_bufs.offset(i as isize)).ref_count == 0 as core::ffi::c_int {
            break;
        }
        i += 1;
    }
    if i != num_frame_bufs {
        if (*frame_bufs.offset(i as isize)).buf.use_external_reference_buffers != 0 {
            let mut ybf: *mut YV12_BUFFER_CONFIG = &raw mut (*frame_bufs.offset(i as isize))
                .buf;
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.y_buffer = (*ybf)
                .store_buf_adr[0 as core::ffi::c_int as usize];
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.u_buffer = (*ybf)
                .store_buf_adr[1 as core::ffi::c_int as usize];
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.v_buffer = (*ybf)
                .store_buf_adr[2 as core::ffi::c_int as usize];
            (*ybf).use_external_reference_buffers = 0 as core::ffi::c_int;
        }
        (*frame_bufs.offset(i as isize)).ref_count = 1 as core::ffi::c_int;
    } else {
        if !(0 as core::ffi::c_int != 0
            && !(b"Ran out of free frame buffers. Likely a reference leak.\0"
                as *const u8 as *const core::ffi::c_char)
                .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"get_free_fb\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1131 as core::ffi::c_int,
                b"0 && \"Ran out of free frame buffers. Likely a reference leak.\"\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        i = INVALID_IDX;
    }
    unlock_buffer_pool((*cm).buffer_pool);
    return i;
}
#[inline]
unsafe extern "C-unwind" fn assign_frame_buffer_p(
    mut lhs_ptr: *mut *mut RefCntBuffer,
    mut rhs_ptr: *mut RefCntBuffer,
) {
    let old_ptr: *mut RefCntBuffer = *lhs_ptr;
    if !old_ptr.is_null() {
        if !((*old_ptr).ref_count > 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"assign_frame_buffer_p\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1166 as core::ffi::c_int,
                b"old_ptr->ref_count > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*old_ptr).ref_count -= 1;
    }
    *lhs_ptr = rhs_ptr;
    (*rhs_ptr).ref_count += 1;
}
#[inline]
unsafe extern "C-unwind" fn frame_is_intra_only(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn frame_is_sframe(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == S_FRAME as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_primary_ref_frame_buf(
    cm: *const AV1_COMMON,
) -> *mut RefCntBuffer {
    let primary_ref_frame: core::ffi::c_int = (*cm).features.primary_ref_frame;
    if primary_ref_frame == PRIMARY_REF_NONE {
        return 0 as *mut RefCntBuffer;
    }
    let map_idx: core::ffi::c_int = get_ref_frame_map_idx(
        cm,
        (primary_ref_frame + 1 as core::ffi::c_int) as MV_REFERENCE_FRAME,
    ) as core::ffi::c_int;
    return if map_idx != INVALID_IDX {
        (*cm).ref_frame_map[map_idx as usize]
    } else {
        0 as *mut RefCntBuffer
    };
}
#[inline]
unsafe extern "C-unwind" fn frame_might_allow_ref_frame_mvs(
    mut cm: *const AV1_COMMON,
) -> core::ffi::c_int {
    return (!(*cm).features.error_resilient_mode
        && (*(*cm).seq_params).order_hint_info.enable_ref_frame_mvs != 0
        && (*(*cm).seq_params).order_hint_info.enable_order_hint != 0
        && frame_is_intra_only(cm) == 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn frame_might_allow_warped_motion(
    mut cm: *const AV1_COMMON,
) -> core::ffi::c_int {
    return (!(*cm).features.error_resilient_mode && frame_is_intra_only(cm) == 0
        && (*(*cm).seq_params).enable_warped_motion as core::ffi::c_int != 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn ensure_mv_buffer(
    mut buf: *mut RefCntBuffer,
    mut cm: *mut AV1_COMMON,
) {
    let buf_rows: core::ffi::c_int = (*buf).mi_rows;
    let buf_cols: core::ffi::c_int = (*buf).mi_cols;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    if ((*buf).mvs).is_null() || buf_rows != (*mi_params).mi_rows
        || buf_cols != (*mi_params).mi_cols
    {
        aom_free((*buf).mvs as *mut core::ffi::c_void);
        (*buf).mi_rows = (*mi_params).mi_rows;
        (*buf).mi_cols = (*mi_params).mi_cols;
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ensure_mv_buffer\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1250 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*buf).mvs = aom_calloc(
            (((*mi_params).mi_rows + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
                * ((*mi_params).mi_cols + 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)) as size_t,
            ::core::mem::size_of::<MV_REF>() as size_t,
        ) as *mut MV_REF;
        if ((*buf).mvs).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate buf->mvs at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_common_int.h\0" as *const u8
                    as *const core::ffi::c_char,
                1250 as core::ffi::c_int,
            );
        }
        aom_free((*buf).seg_map as *mut core::ffi::c_void);
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ensure_mv_buffer\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1255 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*buf).seg_map = aom_calloc(
            ((*mi_params).mi_rows * (*mi_params).mi_cols) as size_t,
            ::core::mem::size_of::<uint8_t>() as size_t,
        ) as *mut uint8_t;
        if ((*buf).seg_map).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate buf->seg_map at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_common_int.h\0" as *const u8
                    as *const core::ffi::c_char,
                1255 as core::ffi::c_int,
            );
        }
    }
    let mem_size: core::ffi::c_int = ((*mi_params).mi_rows + MAX_MIB_SIZE
        >> 1 as core::ffi::c_int) * ((*mi_params).mi_stride >> 1 as core::ffi::c_int);
    if ((*cm).tpl_mvs).is_null() || (*cm).tpl_mvs_mem_size < mem_size {
        aom_free((*cm).tpl_mvs as *mut core::ffi::c_void);
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ensure_mv_buffer\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1264 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*cm).tpl_mvs = aom_calloc(
            mem_size as size_t,
            ::core::mem::size_of::<TPL_MV_REF>() as size_t,
        ) as *mut TPL_MV_REF;
        if ((*cm).tpl_mvs).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate cm->tpl_mvs at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_common_int.h\0" as *const u8
                    as *const core::ffi::c_char,
                1264 as core::ffi::c_int,
            );
        }
        (*cm).tpl_mvs_mem_size = mem_size;
    }
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
unsafe extern "C-unwind" fn av1_init_macroblockd(
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let quant_params: *const CommonQuantParams = &raw mut (*cm).quant_params;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        if (*xd).plane[i as usize].plane_type as core::ffi::c_int
            == PLANE_TYPE_Y as core::ffi::c_int
        {
            memcpy(
                ((*xd).plane[i as usize].seg_dequant_QTX).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).y_dequant_QTX).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[int16_t; 2]; 8]>() as size_t,
            );
            memcpy(
                ((*xd).plane[i as usize].seg_iqmatrix).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).y_iqmatrix).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[*const qm_val_t; 19]; 8]>() as size_t,
            );
        } else if i == AOM_PLANE_U {
            memcpy(
                ((*xd).plane[i as usize].seg_dequant_QTX).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).u_dequant_QTX).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[int16_t; 2]; 8]>() as size_t,
            );
            memcpy(
                ((*xd).plane[i as usize].seg_iqmatrix).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).u_iqmatrix).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[*const qm_val_t; 19]; 8]>() as size_t,
            );
        } else {
            memcpy(
                ((*xd).plane[i as usize].seg_dequant_QTX).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).v_dequant_QTX).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[int16_t; 2]; 8]>() as size_t,
            );
            memcpy(
                ((*xd).plane[i as usize].seg_iqmatrix).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*quant_params).v_iqmatrix).as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[*const qm_val_t; 19]; 8]>() as size_t,
            );
        }
        i += 1;
    }
    (*xd).mi_stride = (*cm).mi_params.mi_stride;
    (*xd).error_info = (*cm).error;
    cfl_init(&raw mut (*xd).cfl, (*cm).seq_params);
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
    let fresh4 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh4 = (*fresh4 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh5 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh5 = (*fresh5 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_SPLIT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh6 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh6 = (*fresh6 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh7 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh7 = (*fresh7 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_B as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh8 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh8 = (*fresh8 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    if bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int {
        let fresh9 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
        *fresh9 = (*fresh9 as core::ffi::c_int
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
    let fresh10 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh10 = (*fresh10 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh11 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh11 = (*fresh11 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_SPLIT as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh12 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh12 = (*fresh12 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_HORZ_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh13 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh13 = (*fresh13 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_A as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    let fresh14 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
    *fresh14 = (*fresh14 as core::ffi::c_int
        - cdf_element_prob(in_0, PARTITION_VERT_B as core::ffi::c_int as size_t)
            as core::ffi::c_int) as aom_cdf_prob;
    if bsize as core::ffi::c_int != BLOCK_128X128 as core::ffi::c_int {
        let fresh15 = &raw mut *out.offset(0 as core::ffi::c_int as isize);
        *fresh15 = (*fresh15 as core::ffi::c_int
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
    let fresh16 = &raw mut *((*mi_params).mi_grid_base).offset(mi_grid_idx as isize);
    *fresh16 = &raw mut *((*mi_params).mi_alloc).offset(mi_alloc_idx as isize)
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
unsafe extern "C-unwind" fn set_sb_size(
    seq_params: *mut SequenceHeader,
    mut sb_size: BLOCK_SIZE,
) {
    (*seq_params).sb_size = sb_size;
    (*seq_params).mib_size = mi_size_wide[(*seq_params).sb_size as usize]
        as core::ffi::c_int;
    (*seq_params).mib_size_log2 = mi_size_wide_log2[(*seq_params).sb_size as usize]
        as core::ffi::c_int;
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
pub const CDEF_STRENGTH_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
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
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn foreach_overlappable_nb_above(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut nb_max: core::ffi::c_int,
    mut fun: overlappable_nb_visitor_t,
    mut fun_ctxt: *mut core::ffi::c_void,
) {
    if !(*xd).up_available {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut nb_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut prev_row_mi: *mut *mut MB_MODE_INFO = ((*xd).mi)
        .offset(-(mi_col as isize))
        .offset(-((1 as core::ffi::c_int * (*xd).mi_stride) as isize));
    let end_col: core::ffi::c_int = if (mi_col + (*xd).width as core::ffi::c_int)
        < (*cm).mi_params.mi_cols
    {
        mi_col + (*xd).width as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_cols
    };
    let mut mi_step: uint8_t = 0;
    let mut above_mi_col: core::ffi::c_int = mi_col;
    while above_mi_col < end_col && nb_count < nb_max {
        let mut above_mi: *mut *mut MB_MODE_INFO = prev_row_mi
            .offset(above_mi_col as isize);
        mi_step = (if (mi_size_wide[(**above_mi.offset(0 as core::ffi::c_int as isize))
            .bsize as usize] as core::ffi::c_int)
            < mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_wide[(**above_mi.offset(0 as core::ffi::c_int as isize)).bsize
                as usize] as core::ffi::c_int
        } else {
            mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        }) as uint8_t;
        if mi_step as core::ffi::c_int == 1 as core::ffi::c_int {
            above_mi_col &= !(1 as core::ffi::c_int);
            above_mi = prev_row_mi
                .offset(above_mi_col as isize)
                .offset(1 as core::ffi::c_int as isize);
            mi_step = 2 as uint8_t;
        }
        if is_neighbor_overlappable(*above_mi) != 0 {
            nb_count += 1;
            fun
                .expect(
                    "non-null function pointer",
                )(
                xd,
                0 as core::ffi::c_int,
                above_mi_col - mi_col,
                (if ((*xd).width as core::ffi::c_int) < mi_step as core::ffi::c_int {
                    (*xd).width as core::ffi::c_int
                } else {
                    mi_step as core::ffi::c_int
                }) as uint8_t,
                0 as core::ffi::c_int,
                *above_mi,
                fun_ctxt,
                num_planes,
            );
        }
        above_mi_col += mi_step as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn foreach_overlappable_nb_left(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut nb_max: core::ffi::c_int,
    mut fun: overlappable_nb_visitor_t,
    mut fun_ctxt: *mut core::ffi::c_void,
) {
    if !(*xd).left_available {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut nb_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mut prev_col_mi: *mut *mut MB_MODE_INFO = ((*xd).mi)
        .offset(-(1 as core::ffi::c_int as isize))
        .offset(-((mi_row * (*xd).mi_stride) as isize));
    let end_row: core::ffi::c_int = if (mi_row + (*xd).height as core::ffi::c_int)
        < (*cm).mi_params.mi_rows
    {
        mi_row + (*xd).height as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_rows
    };
    let mut mi_step: uint8_t = 0;
    let mut left_mi_row: core::ffi::c_int = mi_row;
    while left_mi_row < end_row && nb_count < nb_max {
        let mut left_mi: *mut *mut MB_MODE_INFO = prev_col_mi
            .offset((left_mi_row * (*xd).mi_stride) as isize);
        mi_step = (if (mi_size_high[(**left_mi.offset(0 as core::ffi::c_int as isize))
            .bsize as usize] as core::ffi::c_int)
            < mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_high[(**left_mi.offset(0 as core::ffi::c_int as isize)).bsize
                as usize] as core::ffi::c_int
        } else {
            mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        }) as uint8_t;
        if mi_step as core::ffi::c_int == 1 as core::ffi::c_int {
            left_mi_row &= !(1 as core::ffi::c_int);
            left_mi = prev_col_mi
                .offset(
                    ((left_mi_row + 1 as core::ffi::c_int) * (*xd).mi_stride) as isize,
                );
            mi_step = 2 as uint8_t;
        }
        if is_neighbor_overlappable(*left_mi) != 0 {
            nb_count += 1;
            fun
                .expect(
                    "non-null function pointer",
                )(
                xd,
                left_mi_row - mi_row,
                0 as core::ffi::c_int,
                (if ((*xd).height as core::ffi::c_int) < mi_step as core::ffi::c_int {
                    (*xd).height as core::ffi::c_int
                } else {
                    mi_step as core::ffi::c_int
                }) as uint8_t,
                1 as core::ffi::c_int,
                *left_mi,
                fun_ctxt,
                num_planes,
            );
        }
        left_mi_row += mi_step as core::ffi::c_int;
    }
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
#[inline]
unsafe extern "C-unwind" fn init_interp_filter_params(
    mut interp_filter_params: *mut *const InterpFilterParams,
    mut filter: *const InterpFilters,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
) {
    if is_intrabc as core::ffi::c_long != 0 {
        let fresh17 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh17 = &raw const av1_intrabc_filter_params;
        let fresh18 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh18 = &raw const av1_intrabc_filter_params;
    } else {
        let fresh19 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh19 = av1_get_interp_filter_params_with_block_size(
            (*filter).x_filter as InterpFilter,
            block_width,
        );
        let fresh20 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh20 = av1_get_interp_filter_params_with_block_size(
            (*filter).y_filter as InterpFilter,
            block_height,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn init_inter_block_params(
    mut inter_pred_params: *mut InterPredParams,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut pix_row: core::ffi::c_int,
    mut pix_col: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_hbd_buf: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
) {
    (*inter_pred_params).block_width = block_width;
    (*inter_pred_params).block_height = block_height;
    (*inter_pred_params).pix_row = pix_row;
    (*inter_pred_params).pix_col = pix_col;
    (*inter_pred_params).subsampling_x = subsampling_x;
    (*inter_pred_params).subsampling_y = subsampling_y;
    (*inter_pred_params).bit_depth = bit_depth;
    (*inter_pred_params).use_hbd_buf = use_hbd_buf;
    (*inter_pred_params).is_intrabc = is_intrabc;
    (*inter_pred_params).mode = TRANSLATION_PRED;
    (*inter_pred_params).comp_mode = UNIFORM_SINGLE;
    (*inter_pred_params).top = -((AOM_BORDER_IN_PIXELS >> subsampling_y)
        - AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS);
    (*inter_pred_params).left = -((AOM_BORDER_IN_PIXELS >> subsampling_x)
        - AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS);
}
#[inline]
unsafe extern "C-unwind" fn av1_init_inter_params(
    mut inter_pred_params: *mut InterPredParams,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut pix_row: core::ffi::c_int,
    mut pix_col: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_hbd_buf: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
    mut sf: *const scale_factors,
    mut ref_buf: *const buf_2d,
    mut interp_filters: int_interpfilters,
) {
    init_inter_block_params(
        inter_pred_params,
        block_width,
        block_height,
        pix_row,
        pix_col,
        subsampling_x,
        subsampling_y,
        bit_depth,
        use_hbd_buf,
        is_intrabc,
    );
    init_interp_filter_params(
        ((*inter_pred_params).interp_filter_params).as_mut_ptr(),
        &raw mut interp_filters.as_filters,
        block_width,
        block_height,
        is_intrabc,
    );
    (*inter_pred_params).scale_factors = sf;
    (*inter_pred_params).ref_frame_buf = *ref_buf;
}
#[inline]
unsafe extern "C-unwind" fn av1_init_comp_mode(mut inter_pred_params: *mut InterPredParams) {
    (*inter_pred_params).comp_mode = UNIFORM_COMP;
}
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
    if !(ss_x <= 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"clamp_mv_to_umv_border_sb\0"))
                .as_ptr(),
            b"reconinter.h\0" as *const u8 as *const core::ffi::c_char,
            360 as core::ffi::c_int,
            b"ss_x <= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(ss_y <= 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"clamp_mv_to_umv_border_sb\0"))
                .as_ptr(),
            b"reconinter.h\0" as *const u8 as *const core::ffi::c_char,
            361 as core::ffi::c_int,
            b"ss_y <= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn decrease_ref_count(buf: *mut RefCntBuffer, pool: *mut BufferPool) {
    if !buf.is_null() {
        (*buf).ref_count -= 1;
        if !((*buf).ref_count >= 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"decrease_ref_count\0"))
                    .as_ptr(),
                b"decoder.h\0" as *const u8 as *const core::ffi::c_char,
                473 as core::ffi::c_int,
                b"buf->ref_count >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (*buf).ref_count == 0 as core::ffi::c_int
            && !((*buf).raw_frame_buffer.data).is_null()
        {
            ((*pool).release_fb_cb)
                .expect(
                    "non-null function pointer",
                )((*pool).cb_priv, &raw mut (*buf).raw_frame_buffer);
            (*buf).raw_frame_buffer.data = 0 as *mut uint8_t;
            (*buf).raw_frame_buffer.size = 0 as size_t;
            (*buf).raw_frame_buffer.priv_0 = NULL;
        }
    }
}
pub const AOM_MIN_THREADS_PER_TILE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_MAX_THREADS_PER_TILE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MC_TEMP_BUF_PELS: core::ffi::c_int = (((1 as core::ffi::c_int)
    << MAX_SB_SIZE_LOG2) * 2 as core::ffi::c_int
    + 4 as core::ffi::c_int * 2 as core::ffi::c_int)
    * (((1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2) * 2 as core::ffi::c_int
        + 4 as core::ffi::c_int * 2 as core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_check_trailing_bits(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
) -> core::ffi::c_int {
    let mut bits_before_alignment: core::ffi::c_int = (8 as uint32_t)
        .wrapping_sub(((*rb).bit_offset).wrapping_rem(8 as uint32_t))
        as core::ffi::c_int;
    let mut trailing: core::ffi::c_int = aom_rb_read_literal(rb, bits_before_alignment);
    if trailing
        != (1 as core::ffi::c_int) << bits_before_alignment - 1 as core::ffi::c_int
    {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return -(1 as core::ffi::c_int);
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_planes_to_neutral_grey(
    seq_params: *const SequenceHeader,
    buf: *const YV12_BUFFER_CONFIG,
    mut only_chroma: core::ffi::c_int,
) {
    if (*seq_params).use_highbitdepth != 0 {
        let val: core::ffi::c_int = (1 as core::ffi::c_int)
            << ((*seq_params).bit_depth as core::ffi::c_uint)
                .wrapping_sub(1 as core::ffi::c_uint);
        let mut plane: core::ffi::c_int = only_chroma;
        while plane < MAX_MB_PLANE {
            let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let base: *mut uint16_t = (((*buf).c2rust_unnamed_4.buffers[plane as usize]
                as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t;
            if (*buf).c2rust_unnamed_2.crop_heights[is_uv as usize]
                > 0 as core::ffi::c_int
            {
                aom_memset16(
                    base as *mut core::ffi::c_void,
                    val,
                    (*buf).c2rust_unnamed_1.crop_widths[is_uv as usize] as size_t,
                );
                let mut row_idx: core::ffi::c_int = 1 as core::ffi::c_int;
                while row_idx < (*buf).c2rust_unnamed_2.crop_heights[is_uv as usize] {
                    memcpy(
                        &raw mut *base
                            .offset(
                                (row_idx
                                    * *((*buf).c2rust_unnamed_3.strides)
                                        .as_ptr()
                                        .offset(is_uv as isize)) as isize,
                            ) as *mut uint16_t as *mut core::ffi::c_void,
                        base as *const core::ffi::c_void,
                        (::core::mem::size_of::<uint16_t>() as size_t)
                            .wrapping_mul(
                                (*buf).c2rust_unnamed_1.crop_widths[is_uv as usize]
                                    as size_t,
                            ),
                    );
                    row_idx += 1;
                }
            }
            plane += 1;
        }
    } else {
        let mut plane_0: core::ffi::c_int = only_chroma;
        while plane_0 < MAX_MB_PLANE {
            let is_uv_0: core::ffi::c_int = (plane_0 > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let mut row_idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while row_idx_0 < (*buf).c2rust_unnamed_2.crop_heights[is_uv_0 as usize] {
                memset(
                    &raw mut *(*((*buf).c2rust_unnamed_4.buffers)
                        .as_ptr()
                        .offset(plane_0 as isize))
                        .offset(
                            (row_idx_0
                                * *((*buf).c2rust_unnamed_3.strides)
                                    .as_ptr()
                                    .offset(is_uv_0 as isize)) as isize,
                        ) as *mut uint8_t as *mut core::ffi::c_void,
                    (1 as core::ffi::c_int) << 7 as core::ffi::c_int,
                    (*buf).c2rust_unnamed_1.crop_widths[is_uv_0 as usize] as size_t,
                );
                row_idx_0 += 1;
            }
            plane_0 += 1;
        }
    };
}
unsafe extern "C-unwind" fn read_is_valid(
    mut start: *const uint8_t,
    mut len: size_t,
    mut end: *const uint8_t,
) -> core::ffi::c_int {
    return (len != 0 as size_t
        && len <= end.offset_from(start) as core::ffi::c_long as size_t)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_tx_mode(
    mut rb: *mut aom_read_bit_buffer,
    mut coded_lossless: core::ffi::c_int,
) -> TX_MODE {
    if coded_lossless != 0 {
        return ONLY_4X4 as core::ffi::c_int as TX_MODE;
    }
    return (if aom_rb_read_bit(rb) != 0 {
        TX_MODE_SELECT as core::ffi::c_int
    } else {
        TX_MODE_LARGEST as core::ffi::c_int
    }) as TX_MODE;
}
unsafe extern "C-unwind" fn read_frame_reference_mode(
    mut cm: *const AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) -> REFERENCE_MODE {
    if frame_is_intra_only(cm) != 0 {
        return SINGLE_REFERENCE as core::ffi::c_int as REFERENCE_MODE
    } else {
        return (if aom_rb_read_bit(rb) != 0 {
            REFERENCE_MODE_SELECT as core::ffi::c_int
        } else {
            SINGLE_REFERENCE as core::ffi::c_int
        }) as REFERENCE_MODE
    };
}
#[inline]
unsafe extern "C-unwind" fn inverse_transform_block(
    mut dcb: *mut DecoderCodingBlock,
    mut plane: core::ffi::c_int,
    tx_type: TX_TYPE,
    tx_size: TX_SIZE,
    mut dst: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut reduced_tx_set: core::ffi::c_int,
) {
    let dqcoeff: *mut tran_low_t = ((*dcb).dqcoeff_block[plane as usize])
        .offset((*dcb).cb_offset[plane as usize] as core::ffi::c_int as isize);
    let mut eob_data: *mut eob_info = ((*dcb).eob_data[plane as usize])
        .offset((*dcb).txb_offset[plane as usize] as core::ffi::c_int as isize);
    let mut scan_line: uint16_t = (*eob_data).max_scan_line;
    let mut eob: uint16_t = (*eob_data).eob;
    av1_inverse_transform_block(
        &raw mut (*dcb).xd,
        dqcoeff,
        plane,
        tx_type,
        tx_size,
        dst,
        stride,
        eob as core::ffi::c_int,
        reduced_tx_set,
    );
    memset(
        dqcoeff as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((scan_line as core::ffi::c_int + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
}
#[inline]
unsafe extern "C-unwind" fn read_coeffs_tx_intra_block(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    plane: core::ffi::c_int,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    tx_size: TX_SIZE,
) {
    let mut mbmi: *mut MB_MODE_INFO = *((*dcb).xd.mi)
        .offset(0 as core::ffi::c_int as isize);
    if (*mbmi).skip_txfm == 0 {
        av1_read_coeffs_txb(
            cm as *const AV1Common,
            dcb as *mut DecoderCodingBlock,
            r as *mut aom_reader,
            plane,
            row,
            col,
            tx_size,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn decode_block_void(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    plane: core::ffi::c_int,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    tx_size: TX_SIZE,
) {}
#[inline]
unsafe extern "C-unwind" fn predict_inter_block_void(
    cm: *mut AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut bsize: BLOCK_SIZE,
) {}
#[inline]
unsafe extern "C-unwind" fn cfl_store_inter_block_void(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
) {}
#[inline]
unsafe extern "C-unwind" fn predict_and_reconstruct_intra_block(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    plane: core::ffi::c_int,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    tx_size: TX_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut plane_type: PLANE_TYPE = get_plane_type(plane);
    av1_predict_intra_block_facade(cm, xd, plane, col, row, tx_size);
    if (*mbmi).skip_txfm == 0 {
        let mut eob_data: *mut eob_info = ((*dcb).eob_data[plane as usize])
            .offset((*dcb).txb_offset[plane as usize] as core::ffi::c_int as isize);
        if (*eob_data).eob != 0 {
            let reduced_tx_set_used: bool = (*cm).features.reduced_tx_set_used;
            let tx_type: TX_TYPE = av1_get_tx_type(
                xd,
                plane_type,
                row,
                col,
                tx_size,
                reduced_tx_set_used as core::ffi::c_int,
            ) as TX_TYPE;
            let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
                .as_mut_ptr()
                .offset(plane as isize) as *mut macroblockd_plane;
            let mut dst: *mut uint8_t = &raw mut *((*pd).dst.buf)
                .offset((row * (*pd).dst.stride + col << MI_SIZE_LOG2) as isize)
                as *mut uint8_t;
            inverse_transform_block(
                dcb,
                plane,
                tx_type,
                tx_size,
                dst,
                (*pd).dst.stride,
                reduced_tx_set_used as core::ffi::c_int,
            );
        }
    }
    if plane == AOM_PLANE_Y && store_cfl_required(cm, xd) as core::ffi::c_int != 0 {
        cfl_store_tx(xd, row, col, tx_size, (*mbmi).bsize);
    }
}
#[inline]
unsafe extern "C-unwind" fn inverse_transform_inter_block(
    cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    r: *mut aom_reader,
    plane: core::ffi::c_int,
    blk_row: core::ffi::c_int,
    blk_col: core::ffi::c_int,
    tx_size: TX_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mut plane_type: PLANE_TYPE = get_plane_type(plane);
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let reduced_tx_set_used: bool = (*cm).features.reduced_tx_set_used;
    let tx_type: TX_TYPE = av1_get_tx_type(
        xd,
        plane_type,
        blk_row,
        blk_col,
        tx_size,
        reduced_tx_set_used as core::ffi::c_int,
    ) as TX_TYPE;
    let mut dst: *mut uint8_t = &raw mut *((*pd).dst.buf)
        .offset((blk_row * (*pd).dst.stride + blk_col << MI_SIZE_LOG2) as isize)
        as *mut uint8_t;
    inverse_transform_block(
        dcb,
        plane,
        tx_type,
        tx_size,
        dst,
        (*pd).dst.stride,
        reduced_tx_set_used as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn set_cb_buffer_offsets(
    mut dcb: *mut DecoderCodingBlock,
    mut tx_size: TX_SIZE,
    mut plane: core::ffi::c_int,
) {
    (*dcb).cb_offset[plane as usize] = ((*dcb).cb_offset[plane as usize]
        as core::ffi::c_int
        + tx_size_wide[tx_size as usize] * tx_size_high[tx_size as usize]) as uint16_t;
    (*dcb).txb_offset[plane as usize] = ((*dcb).cb_offset[plane as usize]
        as core::ffi::c_int / (TX_SIZE_W_MIN * TX_SIZE_H_MIN)) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn decode_reconstruct_tx(
    mut cm: *mut AV1_COMMON,
    td: *mut ThreadData,
    mut r: *mut aom_reader,
    mbmi: *mut MB_MODE_INFO,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut block: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut eob_total: *mut core::ffi::c_int,
) {
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let plane_tx_size: TX_SIZE = (if plane != 0 {
        av1_get_max_uv_txsize((*mbmi).bsize, (*pd).subsampling_x, (*pd).subsampling_y)
            as core::ffi::c_int
    } else {
        (*mbmi)
            .inter_tx_size[av1_get_txb_size_index(plane_bsize, blk_row, blk_col)
            as usize] as core::ffi::c_int
    }) as TX_SIZE;
    let max_blocks_high: core::ffi::c_int = max_block_high(xd, plane_bsize, plane)
        as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(xd, plane_bsize, plane)
        as core::ffi::c_int;
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    if tx_size as core::ffi::c_int == plane_tx_size as core::ffi::c_int || plane != 0 {
        ((*td).read_coeffs_tx_inter_block_visit)
            .expect(
                "non-null function pointer",
            )(cm, dcb, r, plane, blk_row, blk_col, tx_size);
        ((*td).inverse_tx_inter_block_visit)
            .expect(
                "non-null function pointer",
            )(cm, dcb, r, plane, blk_row, blk_col, tx_size);
        let mut eob_data: *mut eob_info = ((*dcb).eob_data[plane as usize])
            .offset((*dcb).txb_offset[plane as usize] as core::ffi::c_int as isize);
        *eob_total += (*eob_data).eob as core::ffi::c_int;
        set_cb_buffer_offsets(dcb, tx_size, plane);
    } else {
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        if !(!(tx_size as core::ffi::c_int <= TX_4X4 as core::ffi::c_int)
            || sub_txs as core::ffi::c_int == tx_size as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"decode_reconstruct_tx\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                312 as core::ffi::c_int,
                b"IMPLIES(tx_size <= TX_4X4, sub_txs == tx_size)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !(!(tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int)
            || (sub_txs as core::ffi::c_int) < tx_size as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"decode_reconstruct_tx\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                313 as core::ffi::c_int,
                b"IMPLIES(tx_size > TX_4X4, sub_txs < tx_size)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let bsw: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let bsh: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        let sub_step: core::ffi::c_int = bsw * bsh;
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
        if !(bsw > 0 as core::ffi::c_int && bsh > 0 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"decode_reconstruct_tx\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                322 as core::ffi::c_int,
                b"bsw > 0 && bsh > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < row_end {
            let offsetr: core::ffi::c_int = blk_row + row;
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < col_end {
                let offsetc: core::ffi::c_int = blk_col + col;
                decode_reconstruct_tx(
                    cm,
                    td,
                    r,
                    mbmi,
                    plane,
                    plane_bsize,
                    offsetr,
                    offsetc,
                    block,
                    sub_txs,
                    eob_total,
                );
                block += sub_step;
                col += bsw;
            }
            row += bsh;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn set_offsets(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let tile: *const TileInfo = &raw mut (*xd).tile;
    set_mi_offsets(mi_params, xd, mi_row, mi_col);
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize = bsize;
    if !(x_mis != 0 && y_mis != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"set_offsets\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            351 as core::ffi::c_int,
            b"x_mis && y_mis\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut x: core::ffi::c_int = 1 as core::ffi::c_int;
    while x < x_mis {
        let fresh21 = &raw mut *((*xd).mi).offset(x as isize);
        *fresh21 = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
        x += 1;
    }
    let mut idx: core::ffi::c_int = (*mi_params).mi_stride;
    let mut y: core::ffi::c_int = 1 as core::ffi::c_int;
    while y < y_mis {
        memcpy(
            &raw mut *((*xd).mi).offset(idx as isize) as *mut *mut MB_MODE_INFO
                as *mut core::ffi::c_void,
            &raw mut *((*xd).mi).offset(0 as core::ffi::c_int as isize)
                as *mut *mut MB_MODE_INFO as *const core::ffi::c_void,
            (x_mis as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut MB_MODE_INFO>() as size_t),
        );
        idx += (*mi_params).mi_stride;
        y += 1;
    }
    set_plane_n4(xd, bw, bh, num_planes);
    set_entropy_context(xd, mi_row, mi_col, num_planes);
    set_mi_row_col(
        xd,
        tile,
        mi_row,
        bh,
        mi_col,
        bw,
        (*mi_params).mi_rows,
        (*mi_params).mi_cols,
    );
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        bsize,
        &raw mut (*(*cm).cur_frame).buf,
        mi_row,
        mi_col,
        0 as core::ffi::c_int,
        num_planes,
    );
}
#[inline]
unsafe extern "C-unwind" fn decode_mbmi_block(
    pbi: *mut AV1Decoder,
    mut dcb: *mut DecoderCodingBlock,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut r: *mut aom_reader,
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let x_mis: core::ffi::c_int = if bw < (*cm).mi_params.mi_cols - mi_col {
        bw
    } else {
        (*cm).mi_params.mi_cols - mi_col
    };
    let y_mis: core::ffi::c_int = if bh < (*cm).mi_params.mi_rows - mi_row {
        bh
    } else {
        (*cm).mi_params.mi_rows - mi_row
    };
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    set_offsets(cm, xd, bsize, mi_row, mi_col, bw, bh, x_mis, y_mis);
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).partition = partition;
    av1_read_mode_info(pbi, dcb, r, x_mis, y_mis);
    if bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && ((*seq_params).subsampling_x != 0 || (*seq_params).subsampling_y != 0)
    {
        let uv_subsize: BLOCK_SIZE = av1_ss_size_lookup[bsize
            as usize][(*seq_params).subsampling_x
            as usize][(*seq_params).subsampling_y as usize];
        if uv_subsize as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int {
            aom_internal_error(
                (*xd).error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid block size.\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_build_mc_border(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut b_w: core::ffi::c_int,
    mut b_h: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut src: *const uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut ref_row: *const uint16_t = src
        .offset(-(x as isize))
        .offset(-((y * src_stride) as isize));
    if y >= h {
        ref_row = ref_row.offset(((h - 1 as core::ffi::c_int) * src_stride) as isize);
    } else if y > 0 as core::ffi::c_int {
        ref_row = ref_row.offset((y * src_stride) as isize);
    }
    loop {
        let mut right: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut copy: core::ffi::c_int = 0;
        let mut left: core::ffi::c_int = if x < 0 as core::ffi::c_int {
            -x
        } else {
            0 as core::ffi::c_int
        };
        if left > b_w {
            left = b_w;
        }
        if x + b_w > w {
            right = x + b_w - w;
        }
        if right > b_w {
            right = b_w;
        }
        copy = b_w - left - right;
        if left != 0 {
            aom_memset16(
                dst as *mut core::ffi::c_void,
                *ref_row.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                left as size_t,
            );
        }
        if copy != 0 {
            memcpy(
                dst.offset(left as isize) as *mut core::ffi::c_void,
                ref_row.offset(x as isize).offset(left as isize)
                    as *const core::ffi::c_void,
                (copy as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
        }
        if right != 0 {
            aom_memset16(
                dst.offset(left as isize).offset(copy as isize)
                    as *mut core::ffi::c_void,
                *ref_row.offset((w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                right as size_t,
            );
        }
        dst = dst.offset(dst_stride as isize);
        y += 1;
        if y > 0 as core::ffi::c_int && y < h {
            ref_row = ref_row.offset(src_stride as isize);
        }
        b_h -= 1;
        if !(b_h != 0) {
            break;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn build_mc_border(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut b_w: core::ffi::c_int,
    mut b_h: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut ref_row: *const uint8_t = src
        .offset(-(x as isize))
        .offset(-((y * src_stride) as isize));
    if y >= h {
        ref_row = ref_row.offset(((h - 1 as core::ffi::c_int) * src_stride) as isize);
    } else if y > 0 as core::ffi::c_int {
        ref_row = ref_row.offset((y * src_stride) as isize);
    }
    loop {
        let mut right: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut copy: core::ffi::c_int = 0;
        let mut left: core::ffi::c_int = if x < 0 as core::ffi::c_int {
            -x
        } else {
            0 as core::ffi::c_int
        };
        if left > b_w {
            left = b_w;
        }
        if x + b_w > w {
            right = x + b_w - w;
        }
        if right > b_w {
            right = b_w;
        }
        copy = b_w - left - right;
        if left != 0 {
            memset(
                dst as *mut core::ffi::c_void,
                *ref_row.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                left as size_t,
            );
        }
        if copy != 0 {
            memcpy(
                dst.offset(left as isize) as *mut core::ffi::c_void,
                ref_row.offset(x as isize).offset(left as isize)
                    as *const core::ffi::c_void,
                copy as size_t,
            );
        }
        if right != 0 {
            memset(
                dst.offset(left as isize).offset(copy as isize)
                    as *mut core::ffi::c_void,
                *ref_row.offset((w - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                right as size_t,
            );
        }
        dst = dst.offset(dst_stride as isize);
        y += 1;
        if y > 0 as core::ffi::c_int && y < h {
            ref_row = ref_row.offset(src_stride as isize);
        }
        b_h -= 1;
        if !(b_h != 0) {
            break;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn update_extend_mc_border_params(
    sf: *const scale_factors,
    pre_buf: *mut buf_2d,
    mut scaled_mv: MV32,
    mut block: *mut PadBlock,
    mut subpel_x_mv: core::ffi::c_int,
    mut subpel_y_mv: core::ffi::c_int,
    mut do_warp: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
    mut x_pad: *mut core::ffi::c_int,
    mut y_pad: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
    let mut frame_width: core::ffi::c_int = (*pre_buf).width;
    let mut frame_height: core::ffi::c_int = (*pre_buf).height;
    if is_intrabc == 0 && do_warp == 0
        && (is_scaled != 0 || scaled_mv.col != 0 || scaled_mv.row != 0
            || frame_width & 0x7 as core::ffi::c_int != 0
            || frame_height & 0x7 as core::ffi::c_int != 0)
    {
        if subpel_x_mv != 0 || (*sf).x_step_q4 != SUBPEL_SHIFTS {
            (*block).x0 -= AOM_INTERP_EXTEND - 1 as core::ffi::c_int;
            (*block).x1 += AOM_INTERP_EXTEND;
            *x_pad = 1 as core::ffi::c_int;
        }
        if subpel_y_mv != 0 || (*sf).y_step_q4 != SUBPEL_SHIFTS {
            (*block).y0 -= AOM_INTERP_EXTEND - 1 as core::ffi::c_int;
            (*block).y1 += AOM_INTERP_EXTEND;
            *y_pad = 1 as core::ffi::c_int;
        }
        if (*block).x0 < 0 as core::ffi::c_int
            || (*block).x1 > frame_width - 1 as core::ffi::c_int
            || (*block).y0 < 0 as core::ffi::c_int
            || (*block).y1 > frame_height - 1 as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn extend_mc_border(
    sf: *const scale_factors,
    pre_buf: *mut buf_2d,
    mut scaled_mv: MV32,
    mut block: PadBlock,
    mut subpel_x_mv: core::ffi::c_int,
    mut subpel_y_mv: core::ffi::c_int,
    mut do_warp: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
    mut mc_buf: *mut uint8_t,
    mut pre: *mut *mut uint8_t,
    mut src_stride: *mut core::ffi::c_int,
) {
    let mut x_pad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_pad: core::ffi::c_int = 0 as core::ffi::c_int;
    if update_extend_mc_border_params(
        sf,
        pre_buf,
        scaled_mv,
        &raw mut block,
        subpel_x_mv,
        subpel_y_mv,
        do_warp,
        is_intrabc,
        &raw mut x_pad,
        &raw mut y_pad,
    ) != 0
    {
        let buf_ptr: *const uint8_t = ((*pre_buf).buf0)
            .offset((block.y0 * (*pre_buf).stride) as isize)
            .offset(block.x0 as isize);
        let mut buf_stride: core::ffi::c_int = (*pre_buf).stride;
        let b_w: core::ffi::c_int = block.x1 - block.x0;
        let b_h: core::ffi::c_int = block.y1 - block.y0;
        if highbd != 0 {
            highbd_build_mc_border(
                buf_ptr,
                buf_stride,
                mc_buf,
                b_w,
                block.x0,
                block.y0,
                b_w,
                b_h,
                (*pre_buf).width,
                (*pre_buf).height,
            );
        } else {
            build_mc_border(
                buf_ptr,
                buf_stride,
                mc_buf,
                b_w,
                block.x0,
                block.y0,
                b_w,
                b_h,
                (*pre_buf).width,
                (*pre_buf).height,
            );
        }
        *src_stride = b_w;
        *pre = mc_buf
            .offset((y_pad * (AOM_INTERP_EXTEND - 1 as core::ffi::c_int) * b_w) as isize)
            .offset((x_pad * (AOM_INTERP_EXTEND - 1 as core::ffi::c_int)) as isize);
    }
}
#[inline]
unsafe extern "C-unwind" fn dec_calc_subpel_params(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    xd: *const MACROBLOCKD,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut pre: *mut *mut uint8_t,
    mut subpel_params: *mut SubpelParams,
    mut src_stride: *mut core::ffi::c_int,
    mut block: *mut PadBlock,
    mut scaled_mv: *mut MV32,
    mut subpel_x_mv: *mut core::ffi::c_int,
    mut subpel_y_mv: *mut core::ffi::c_int,
) {
    let mut sf: *const scale_factors = (*inter_pred_params).scale_factors;
    let mut pre_buf: *mut buf_2d = &raw mut (*inter_pred_params).ref_frame_buf;
    let bw: core::ffi::c_int = (*inter_pred_params).block_width;
    let bh: core::ffi::c_int = (*inter_pred_params).block_height;
    let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
    if is_scaled != 0 {
        let mut ssx: core::ffi::c_int = (*inter_pred_params).subsampling_x;
        let mut ssy: core::ffi::c_int = (*inter_pred_params).subsampling_y;
        let mut orig_pos_y: core::ffi::c_int = (*inter_pred_params).pix_row
            << SUBPEL_BITS;
        orig_pos_y
            += (*src_mv).row as core::ffi::c_int
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssy);
        let mut orig_pos_x: core::ffi::c_int = (*inter_pred_params).pix_col
            << SUBPEL_BITS;
        orig_pos_x
            += (*src_mv).col as core::ffi::c_int
                * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssx);
        let mut pos_y: core::ffi::c_int = av1_scaled_y(orig_pos_y, sf);
        let mut pos_x: core::ffi::c_int = av1_scaled_x(orig_pos_x, sf);
        pos_x += SCALE_EXTRA_OFF;
        pos_y += SCALE_EXTRA_OFF;
        let top: core::ffi::c_int = -((AOM_BORDER_IN_PIXELS >> ssy) - AOM_INTERP_EXTEND
            << SCALE_SUBPEL_BITS);
        let left: core::ffi::c_int = -((AOM_BORDER_IN_PIXELS >> ssx) - AOM_INTERP_EXTEND
            << SCALE_SUBPEL_BITS);
        let bottom: core::ffi::c_int = (*pre_buf).height + AOM_INTERP_EXTEND
            << SCALE_SUBPEL_BITS;
        let right: core::ffi::c_int = (*pre_buf).width + AOM_INTERP_EXTEND
            << SCALE_SUBPEL_BITS;
        pos_y = clamp(pos_y, top, bottom);
        pos_x = clamp(pos_x, left, right);
        (*subpel_params).subpel_x = pos_x & SCALE_SUBPEL_MASK;
        (*subpel_params).subpel_y = pos_y & SCALE_SUBPEL_MASK;
        (*subpel_params).xs = (*sf).x_step_q4;
        (*subpel_params).ys = (*sf).y_step_q4;
        (*block).x0 = pos_x >> SCALE_SUBPEL_BITS;
        (*block).y0 = pos_y >> SCALE_SUBPEL_BITS;
        (*block).x1 = (pos_x + (bw - 1 as core::ffi::c_int) * (*subpel_params).xs
            >> SCALE_SUBPEL_BITS) + 1 as core::ffi::c_int;
        (*block).y1 = (pos_y + (bh - 1 as core::ffi::c_int) * (*subpel_params).ys
            >> SCALE_SUBPEL_BITS) + 1 as core::ffi::c_int;
        let mut temp_mv: MV = mv { row: 0, col: 0 };
        temp_mv = clamp_mv_to_umv_border_sb(
            xd,
            src_mv,
            bw,
            bh,
            (*inter_pred_params).subsampling_x,
            (*inter_pred_params).subsampling_y,
        );
        *scaled_mv = av1_scale_mv(&raw mut temp_mv, mi_x, mi_y, sf);
        (*scaled_mv).row = ((*scaled_mv).row as core::ffi::c_int + SCALE_EXTRA_OFF)
            as int32_t;
        (*scaled_mv).col = ((*scaled_mv).col as core::ffi::c_int + SCALE_EXTRA_OFF)
            as int32_t;
        *subpel_x_mv = ((*scaled_mv).col & SCALE_SUBPEL_MASK as int32_t)
            as core::ffi::c_int;
        *subpel_y_mv = ((*scaled_mv).row & SCALE_SUBPEL_MASK as int32_t)
            as core::ffi::c_int;
    } else {
        let mut pos_x_0: core::ffi::c_int = (*inter_pred_params).pix_col << SUBPEL_BITS;
        let mut pos_y_0: core::ffi::c_int = (*inter_pred_params).pix_row << SUBPEL_BITS;
        let mv_q4: MV = clamp_mv_to_umv_border_sb(
            xd,
            src_mv,
            bw,
            bh,
            (*inter_pred_params).subsampling_x,
            (*inter_pred_params).subsampling_y,
        ) as MV;
        (*subpel_params).ys = SCALE_SUBPEL_SHIFTS;
        (*subpel_params).xs = (*subpel_params).ys;
        (*subpel_params).subpel_x = (mv_q4.col as core::ffi::c_int & SUBPEL_MASK)
            << SCALE_EXTRA_BITS;
        (*subpel_params).subpel_y = (mv_q4.row as core::ffi::c_int & SUBPEL_MASK)
            << SCALE_EXTRA_BITS;
        pos_x_0 += mv_q4.col as core::ffi::c_int;
        pos_y_0 += mv_q4.row as core::ffi::c_int;
        (*block).x0 = pos_x_0 >> SUBPEL_BITS;
        (*block).y0 = pos_y_0 >> SUBPEL_BITS;
        (*block).x1 = (pos_x_0 >> SUBPEL_BITS) + (bw - 1 as core::ffi::c_int)
            + 1 as core::ffi::c_int;
        (*block).y1 = (pos_y_0 >> SUBPEL_BITS) + (bh - 1 as core::ffi::c_int)
            + 1 as core::ffi::c_int;
        (*scaled_mv).row = mv_q4.row as int32_t;
        (*scaled_mv).col = mv_q4.col as int32_t;
        *subpel_x_mv = ((*scaled_mv).col & SUBPEL_MASK as int32_t) as core::ffi::c_int;
        *subpel_y_mv = ((*scaled_mv).row & SUBPEL_MASK as int32_t) as core::ffi::c_int;
    }
    *pre = ((*pre_buf).buf0)
        .offset(((*block).y0 * (*pre_buf).stride) as isize)
        .offset((*block).x0 as isize);
    *src_stride = (*pre_buf).stride;
}
#[inline]
unsafe extern "C-unwind" fn dec_calc_subpel_params_and_extend(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    xd: *mut MACROBLOCKD,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut mc_buf: *mut *mut uint8_t,
    mut pre: *mut *mut uint8_t,
    mut subpel_params: *mut SubpelParams,
    mut src_stride: *mut core::ffi::c_int,
) {
    let mut block: PadBlock = PadBlock {
        x0: 0,
        x1: 0,
        y0: 0,
        y1: 0,
    };
    let mut scaled_mv: MV32 = mv32 { row: 0, col: 0 };
    let mut subpel_x_mv: core::ffi::c_int = 0;
    let mut subpel_y_mv: core::ffi::c_int = 0;
    dec_calc_subpel_params(
        src_mv,
        inter_pred_params,
        xd,
        mi_x,
        mi_y,
        pre,
        subpel_params,
        src_stride,
        &raw mut block,
        &raw mut scaled_mv,
        &raw mut subpel_x_mv,
        &raw mut subpel_y_mv,
    );
    extend_mc_border(
        (*inter_pred_params).scale_factors,
        &raw mut (*inter_pred_params).ref_frame_buf,
        scaled_mv,
        block,
        subpel_x_mv,
        subpel_y_mv,
        ((*inter_pred_params).mode as core::ffi::c_uint
            == WARP_PRED as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int,
        (*inter_pred_params).is_intrabc,
        (*inter_pred_params).use_hbd_buf,
        *mc_buf.offset(ref_0 as isize),
        pre,
        src_stride,
    );
}
#[inline]
unsafe extern "C-unwind" fn build_one_inter_predictor(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut src_mv: *const MV,
    mut inter_pred_params: *mut InterPredParams,
    mut xd: *mut MACROBLOCKD,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut mc_buf: *mut *mut uint8_t,
) {
    let mut subpel_params: SubpelParams = SubpelParams {
        xs: 0,
        ys: 0,
        subpel_x: 0,
        subpel_y: 0,
        pos_x: 0,
        pos_y: 0,
    };
    let mut src: *mut uint8_t = 0 as *mut uint8_t;
    let mut src_stride: core::ffi::c_int = 0;
    dec_calc_subpel_params_and_extend(
        src_mv,
        inter_pred_params,
        xd,
        mi_x,
        mi_y,
        ref_0,
        mc_buf,
        &raw mut src,
        &raw mut subpel_params,
        &raw mut src_stride,
    );
    if (*inter_pred_params).comp_mode as core::ffi::c_uint
        == UNIFORM_SINGLE as core::ffi::c_int as core::ffi::c_uint
        || (*inter_pred_params).comp_mode as core::ffi::c_uint
            == UNIFORM_COMP as core::ffi::c_int as core::ffi::c_uint
    {
        av1_make_inter_predictor(
            src,
            src_stride,
            dst,
            dst_stride,
            inter_pred_params,
            &raw mut subpel_params,
        );
    } else {
        av1_make_masked_inter_predictor(
            src,
            src_stride,
            dst,
            dst_stride,
            inter_pred_params,
            &raw mut subpel_params,
        );
    };
}
unsafe extern "C-unwind" fn is_sub8x8_inter(
    mut xd: *const MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut is_intrabc: core::ffi::c_int,
    mut build_for_obmc: core::ffi::c_int,
) -> bool {
    if is_intrabc != 0 || build_for_obmc != 0 {
        return false_0 != 0;
    }
    let pd: *const macroblockd_plane = &*((*xd).plane).as_ptr().offset(plane as isize)
        as *const macroblockd_plane;
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let is_sub4_x: core::ffi::c_int = (block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x != 0) as core::ffi::c_int;
    let is_sub4_y: core::ffi::c_int = (block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y != 0) as core::ffi::c_int;
    if is_sub4_x == 0 && is_sub4_y == 0 {
        return false_0 != 0;
    }
    let row_start: core::ffi::c_int = if is_sub4_y != 0 {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if is_sub4_x != 0 {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let mut row: core::ffi::c_int = row_start;
    while row <= 0 as core::ffi::c_int {
        let mut col: core::ffi::c_int = col_start;
        while col <= 0 as core::ffi::c_int {
            let mut this_mbmi: *const MB_MODE_INFO = *((*xd).mi)
                .offset((row * (*xd).mi_stride + col) as isize);
            if is_inter_block(this_mbmi) == 0 {
                return false_0 != 0;
            }
            if is_intrabc_block(this_mbmi) != 0 {
                return false_0 != 0;
            }
            col += 1;
        }
        row += 1;
    }
    return true_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors_sub8x8(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut mc_buf: *mut *mut uint8_t,
) {
    let bsize: BLOCK_SIZE = (*mi).bsize;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let ss_x: bool = (*pd).subsampling_x != 0;
    let ss_y: bool = (*pd).subsampling_y != 0;
    let b4_w: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int
        >> ss_x as core::ffi::c_int;
    let b4_h: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int
        >> ss_y as core::ffi::c_int;
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        ss_x as core::ffi::c_int,
        ss_y as core::ffi::c_int,
    ) as BLOCK_SIZE;
    let b8_w: core::ffi::c_int = block_size_wide[plane_bsize as usize]
        as core::ffi::c_int;
    let b8_h: core::ffi::c_int = block_size_high[plane_bsize as usize]
        as core::ffi::c_int;
    let is_compound: core::ffi::c_int = has_second_ref(mi) as core::ffi::c_int;
    if (is_compound != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"build_inter_predictors_sub8x8\0"))
                .as_ptr(),
            b"reconinter_template.inc\0" as *const u8 as *const core::ffi::c_char,
            108 as core::ffi::c_int,
            b"!is_compound\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (is_intrabc_block(mi) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"build_inter_predictors_sub8x8\0"))
                .as_ptr(),
            b"reconinter_template.inc\0" as *const u8 as *const core::ffi::c_char,
            109 as core::ffi::c_int,
            b"!is_intrabc_block(mi)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let row_start: core::ffi::c_int = if block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y as core::ffi::c_int != 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x as core::ffi::c_int != 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let pre_x: core::ffi::c_int = mi_x + MI_SIZE * col_start >> ss_x as core::ffi::c_int;
    let pre_y: core::ffi::c_int = mi_y + MI_SIZE * row_start >> ss_y as core::ffi::c_int;
    let mut row: core::ffi::c_int = row_start;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < b8_h {
        let mut col: core::ffi::c_int = col_start;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < b8_w {
            let mut this_mbmi: *mut MB_MODE_INFO = *((*xd).mi)
                .offset((row * (*xd).mi_stride + col) as isize);
            let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
            let mut dst: *mut uint8_t = ((*dst_buf).buf)
                .offset(((*dst_buf).stride * y) as isize)
                .offset(x as isize);
            let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut ref_buf: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                (*this_mbmi).ref_frame[ref_0 as usize],
            );
            let mut ref_scale_factors: *const scale_factors = get_ref_scale_factors_const(
                cm,
                (*this_mbmi).ref_frame[ref_0 as usize],
            );
            let sf: *const scale_factors = ref_scale_factors;
            let pre_buf: buf_2d = {
                let mut init = buf_2d {
                    buf: 0 as *mut uint8_t,
                    buf0: if plane == 1 as core::ffi::c_int {
                        (*ref_buf).buf.c2rust_unnamed_4.c2rust_unnamed.u_buffer
                    } else {
                        (*ref_buf).buf.c2rust_unnamed_4.c2rust_unnamed.v_buffer
                    },
                    width: (*ref_buf).buf.c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                    height: (*ref_buf)
                        .buf
                        .c2rust_unnamed_2
                        .c2rust_unnamed
                        .uv_crop_height,
                    stride: (*ref_buf).buf.c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                };
                init
            };
            let mv: MV = (*this_mbmi).mv[ref_0 as usize].as_mv;
            let mut inter_pred_params: InterPredParams = InterPredParams {
                mode: TRANSLATION_PRED,
                comp_mode: UNIFORM_SINGLE,
                warp_params: WarpedMotionParams {
                    wmmat: [0; 6],
                    alpha: 0,
                    beta: 0,
                    gamma: 0,
                    delta: 0,
                    wmtype: 0,
                    invalid: 0,
                },
                conv_params: ConvolveParams {
                    do_average: 0,
                    dst: 0 as *mut CONV_BUF_TYPE,
                    dst_stride: 0,
                    round_0: 0,
                    round_1: 0,
                    plane: 0,
                    is_compound: 0,
                    use_dist_wtd_comp_avg: 0,
                    fwd_offset: 0,
                    bck_offset: 0,
                },
                interp_filter_params: [0 as *const InterpFilterParams; 2],
                block_width: 0,
                block_height: 0,
                pix_row: 0,
                pix_col: 0,
                ref_frame_buf: buf_2d {
                    buf: 0 as *mut uint8_t,
                    buf0: 0 as *mut uint8_t,
                    width: 0,
                    height: 0,
                    stride: 0,
                },
                subsampling_x: 0,
                subsampling_y: 0,
                scale_factors: 0 as *const scale_factors,
                bit_depth: 0,
                use_hbd_buf: 0,
                mask_comp: INTERINTER_COMPOUND_DATA {
                    seg_mask: 0 as *mut uint8_t,
                    wedge_index: 0,
                    wedge_sign: 0,
                    mask_type: 0,
                    type_0: 0,
                },
                sb_type: BLOCK_4X4,
                is_intrabc: 0,
                top: 0,
                left: 0,
            };
            av1_init_inter_params(
                &raw mut inter_pred_params,
                b4_w,
                b4_h,
                pre_y + y,
                pre_x + x,
                (*pd).subsampling_x,
                (*pd).subsampling_y,
                (*xd).bd,
                is_cur_buf_hbd(xd),
                (*mi).use_intrabc() as core::ffi::c_int,
                sf,
                &pre_buf,
                (*this_mbmi).interp_filters,
            );
            inter_pred_params.conv_params = get_conv_params_no_round(
                ref_0,
                plane,
                0 as *mut CONV_BUF_TYPE,
                0 as core::ffi::c_int,
                is_compound,
                (*xd).bd,
            );
            build_one_inter_predictor(
                dst,
                (*dst_buf).stride,
                &mv,
                &raw mut inter_pred_params,
                xd,
                mi_x + x,
                mi_y + y,
                ref_0,
                mc_buf,
            );
            col += 1;
            x += b4_w;
        }
        row += 1;
        y += b4_h;
    }
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors_8x8_and_bigger(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut build_for_obmc: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut mc_buf: *mut *mut uint8_t,
) {
    let is_compound: core::ffi::c_int = has_second_ref(mi) as core::ffi::c_int;
    let is_intrabc: core::ffi::c_int = is_intrabc_block(mi) as core::ffi::c_int;
    if !(is_intrabc == 0 || is_compound == 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 38],
                [core::ffi::c_char; 38],
            >(*b"build_inter_predictors_8x8_and_bigger\0"))
                .as_ptr(),
            b"reconinter_template.inc\0" as *const u8 as *const core::ffi::c_char,
            175 as core::ffi::c_int,
            b"IMPLIES(is_intrabc, !is_compound)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
    let dst: *mut uint8_t = (*dst_buf).buf;
    let mut is_global: [core::ffi::c_int; 2] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + is_compound {
        let wm: *const WarpedMotionParams = &*((*xd).global_motion)
            .offset(*((*mi).ref_frame).as_ptr().offset(ref_0 as isize) as isize)
            as *const WarpedMotionParams;
        is_global[ref_0 as usize] = is_global_mv_block(mi, (*wm).wmtype);
        ref_0 += 1;
    }
    let bsize: BLOCK_SIZE = (*mi).bsize;
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let row_start: core::ffi::c_int = if block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y != 0 && build_for_obmc == 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x != 0 && build_for_obmc == 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let pre_x: core::ffi::c_int = mi_x + MI_SIZE * col_start >> ss_x;
    let pre_y: core::ffi::c_int = mi_y + MI_SIZE * row_start >> ss_y;
    let mut ref_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_1 < 1 as core::ffi::c_int + is_compound {
        let sf: *const scale_factors = if is_intrabc != 0 {
            &(*cm).sf_identity
        } else {
            (*xd).block_ref_scale_factors[ref_1 as usize]
        };
        let pre_buf: *mut buf_2d = if is_intrabc != 0 {
            dst_buf
        } else {
            &raw mut *((*pd).pre).as_mut_ptr().offset(ref_1 as isize) as *mut buf_2d
        };
        let mv: MV = (*mi).mv[ref_1 as usize].as_mv;
        let warp_types: WarpTypesAllowed = {
            let mut init = WarpTypesAllowed {
                global_warp_allowed: is_global[ref_1 as usize],
                local_warp_allowed: ((*mi).motion_mode as core::ffi::c_int
                    == WARPED_CAUSAL as core::ffi::c_int) as core::ffi::c_int,
            };
            init
        };
        let mut inter_pred_params: InterPredParams = InterPredParams {
            mode: TRANSLATION_PRED,
            comp_mode: UNIFORM_SINGLE,
            warp_params: WarpedMotionParams {
                wmmat: [0; 6],
                alpha: 0,
                beta: 0,
                gamma: 0,
                delta: 0,
                wmtype: 0,
                invalid: 0,
            },
            conv_params: ConvolveParams {
                do_average: 0,
                dst: 0 as *mut CONV_BUF_TYPE,
                dst_stride: 0,
                round_0: 0,
                round_1: 0,
                plane: 0,
                is_compound: 0,
                use_dist_wtd_comp_avg: 0,
                fwd_offset: 0,
                bck_offset: 0,
            },
            interp_filter_params: [0 as *const InterpFilterParams; 2],
            block_width: 0,
            block_height: 0,
            pix_row: 0,
            pix_col: 0,
            ref_frame_buf: buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0,
                height: 0,
                stride: 0,
            },
            subsampling_x: 0,
            subsampling_y: 0,
            scale_factors: 0 as *const scale_factors,
            bit_depth: 0,
            use_hbd_buf: 0,
            mask_comp: INTERINTER_COMPOUND_DATA {
                seg_mask: 0 as *mut uint8_t,
                wedge_index: 0,
                wedge_sign: 0,
                mask_type: 0,
                type_0: 0,
            },
            sb_type: BLOCK_4X4,
            is_intrabc: 0,
            top: 0,
            left: 0,
        };
        av1_init_inter_params(
            &raw mut inter_pred_params,
            bw,
            bh,
            pre_y,
            pre_x,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
            (*xd).bd,
            is_cur_buf_hbd(xd),
            (*mi).use_intrabc() as core::ffi::c_int,
            sf,
            pre_buf,
            (*mi).interp_filters,
        );
        if is_compound != 0 {
            av1_init_comp_mode(&raw mut inter_pred_params);
        }
        inter_pred_params.conv_params = get_conv_params_no_round(
            ref_1,
            plane,
            (*xd).tmp_conv_dst,
            MAX_SB_SIZE,
            is_compound,
            (*xd).bd,
        );
        av1_dist_wtd_comp_weight_assign(
            cm,
            mi,
            &raw mut inter_pred_params.conv_params.fwd_offset,
            &raw mut inter_pred_params.conv_params.bck_offset,
            &raw mut inter_pred_params.conv_params.use_dist_wtd_comp_avg,
            is_compound,
        );
        if build_for_obmc == 0 {
            av1_init_warp_params(&raw mut inter_pred_params, &warp_types, ref_1, xd, mi);
        }
        if is_masked_compound_type((*mi).interinter_comp.type_0) != 0 {
            inter_pred_params.sb_type = (*mi).bsize;
            inter_pred_params.mask_comp = (*mi).interinter_comp;
            if ref_1 == 1 as core::ffi::c_int {
                inter_pred_params.conv_params.do_average = 0 as core::ffi::c_int;
                inter_pred_params.comp_mode = MASK_COMP;
            }
            inter_pred_params.mask_comp.seg_mask = (*xd).seg_mask;
        }
        build_one_inter_predictor(
            dst,
            (*dst_buf).stride,
            &mv,
            &raw mut inter_pred_params,
            xd,
            mi_x,
            mi_y,
            ref_1,
            mc_buf,
        );
        ref_1 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut build_for_obmc: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
    mut mc_buf: *mut *mut uint8_t,
) {
    if is_sub8x8_inter(xd, plane, (*mi).bsize, is_intrabc_block(mi), build_for_obmc) {
        if !(bw < 8 as core::ffi::c_int || bh < 8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"build_inter_predictors\0"))
                    .as_ptr(),
                b"reconinter_template.inc\0" as *const u8 as *const core::ffi::c_char,
                249 as core::ffi::c_int,
                b"bw < 8 || bh < 8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        build_inter_predictors_sub8x8(cm, xd, plane, mi, mi_x, mi_y, mc_buf);
    } else {
        build_inter_predictors_8x8_and_bigger(
            cm,
            xd,
            plane,
            mi,
            build_for_obmc,
            bw,
            bh,
            mi_x,
            mi_y,
            mc_buf,
        );
    };
}
unsafe extern "C-unwind" fn dec_build_inter_predictors(
    mut cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut build_for_obmc: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
) {
    build_inter_predictors(
        cm,
        &raw mut (*dcb).xd,
        plane,
        mi,
        build_for_obmc,
        bw,
        bh,
        mi_x,
        mi_y,
        ((*dcb).mc_buf).as_mut_ptr(),
    );
}
#[inline]
unsafe extern "C-unwind" fn dec_build_inter_predictor(
    mut cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if plane != 0 && !(*xd).is_chroma_ref {
            break;
        }
        let mi_x: core::ffi::c_int = mi_col * MI_SIZE;
        let mi_y: core::ffi::c_int = mi_row * MI_SIZE;
        dec_build_inter_predictors(
            cm,
            dcb,
            plane,
            *((*xd).mi).offset(0 as core::ffi::c_int as isize),
            0 as core::ffi::c_int,
            (*xd).plane[plane as usize].width as core::ffi::c_int,
            (*xd).plane[plane as usize].height as core::ffi::c_int,
            mi_x,
            mi_y,
        );
        if is_interintra_pred(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) != 0 {
            let mut ctx: BUFFER_SET = {
                let mut init = BUFFER_SET {
                    plane: [
                        (*xd).plane[0 as core::ffi::c_int as usize].dst.buf,
                        (*xd).plane[1 as core::ffi::c_int as usize].dst.buf,
                        (*xd).plane[2 as core::ffi::c_int as usize].dst.buf,
                    ],
                    stride: [
                        (*xd).plane[0 as core::ffi::c_int as usize].dst.stride,
                        (*xd).plane[1 as core::ffi::c_int as usize].dst.stride,
                        (*xd).plane[2 as core::ffi::c_int as usize].dst.stride,
                    ],
                };
                init
            };
            av1_build_interintra_predictor(
                cm,
                xd,
                (*xd).plane[plane as usize].dst.buf,
                (*xd).plane[plane as usize].dst.stride,
                &raw mut ctx,
                plane,
                bsize,
            );
        }
        plane += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dec_build_prediction_by_above_pred(
    xd: *mut MACROBLOCKD,
    mut rel_mi_row: core::ffi::c_int,
    mut rel_mi_col: core::ffi::c_int,
    mut op_mi_size: uint8_t,
    mut dir: core::ffi::c_int,
    mut above_mbmi: *mut MB_MODE_INFO,
    mut fun_ctxt: *mut core::ffi::c_void,
    num_planes: core::ffi::c_int,
) {
    let mut ctxt: *mut build_prediction_ctxt = fun_ctxt as *mut build_prediction_ctxt;
    let above_mi_col: core::ffi::c_int = (*xd).mi_col + rel_mi_col;
    let mut mi_x: core::ffi::c_int = 0;
    let mut mi_y: core::ffi::c_int = 0;
    let mut backup_mbmi: MB_MODE_INFO = *above_mbmi;
    av1_setup_build_prediction_by_above_pred(
        xd,
        rel_mi_col,
        op_mi_size,
        &raw mut backup_mbmi,
        ctxt,
        num_planes,
    );
    mi_x = above_mi_col << MI_SIZE_LOG2;
    mi_y = (*xd).mi_row << MI_SIZE_LOG2;
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < num_planes {
        let mut pd: *const macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(j as isize) as *mut macroblockd_plane;
        let mut bw: core::ffi::c_int = op_mi_size as core::ffi::c_int * MI_SIZE
            >> (*pd).subsampling_x;
        let mut bh: core::ffi::c_int = clamp(
            block_size_high[bsize as usize] as core::ffi::c_int
                >> (*pd).subsampling_y + 1 as core::ffi::c_int,
            4 as core::ffi::c_int,
            block_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
                >> (*pd).subsampling_y + 1 as core::ffi::c_int,
        );
        if !(av1_skip_u4x4_pred_in_obmc(bsize, pd, 0 as core::ffi::c_int) != 0) {
            dec_build_inter_predictors(
                (*ctxt).cm,
                (*ctxt).dcb as *mut DecoderCodingBlock,
                j,
                &raw mut backup_mbmi,
                1 as core::ffi::c_int,
                bw,
                bh,
                mi_x,
                mi_y,
            );
        }
        j += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dec_build_prediction_by_above_preds(
    mut cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut tmp_buf: *mut *mut uint8_t,
    mut tmp_width: *mut core::ffi::c_int,
    mut tmp_height: *mut core::ffi::c_int,
    mut tmp_stride: *mut core::ffi::c_int,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    if !(*xd).up_available {
        return;
    }
    let this_height: core::ffi::c_int = (*xd).height as core::ffi::c_int * MI_SIZE;
    let pred_height: core::ffi::c_int = if (this_height / 2 as core::ffi::c_int)
        < 32 as core::ffi::c_int
    {
        this_height / 2 as core::ffi::c_int
    } else {
        32 as core::ffi::c_int
    };
    (*xd).mb_to_bottom_edge += (this_height - pred_height) * 8 as core::ffi::c_int;
    let mut ctxt: build_prediction_ctxt = {
        let mut init = build_prediction_ctxt {
            cm: cm,
            tmp_buf: tmp_buf as *mut *mut uint8_t,
            tmp_width: tmp_width as *mut core::ffi::c_int,
            tmp_height: tmp_height as *mut core::ffi::c_int,
            tmp_stride: tmp_stride as *mut core::ffi::c_int,
            mb_to_far_edge: (*xd).mb_to_right_edge,
            dcb: dcb as *mut core::ffi::c_void,
        };
        init
    };
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    foreach_overlappable_nb_above(
        cm,
        xd,
        max_neighbor_obmc[mi_size_wide_log2[bsize as usize] as usize],
        Some(
            dec_build_prediction_by_above_pred
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    uint8_t,
                    core::ffi::c_int,
                    *mut MB_MODE_INFO,
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                ) -> (),
        ),
        &raw mut ctxt as *mut build_prediction_ctxt as *mut core::ffi::c_void,
    );
    (*xd).mb_to_left_edge = -((*xd).mi_col
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int);
    (*xd).mb_to_right_edge = ctxt.mb_to_far_edge;
    (*xd).mb_to_bottom_edge -= (this_height - pred_height) * 8 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn dec_build_prediction_by_left_pred(
    xd: *mut MACROBLOCKD,
    mut rel_mi_row: core::ffi::c_int,
    mut rel_mi_col: core::ffi::c_int,
    mut op_mi_size: uint8_t,
    mut dir: core::ffi::c_int,
    mut left_mbmi: *mut MB_MODE_INFO,
    mut fun_ctxt: *mut core::ffi::c_void,
    num_planes: core::ffi::c_int,
) {
    let mut ctxt: *mut build_prediction_ctxt = fun_ctxt as *mut build_prediction_ctxt;
    let left_mi_row: core::ffi::c_int = (*xd).mi_row + rel_mi_row;
    let mut mi_x: core::ffi::c_int = 0;
    let mut mi_y: core::ffi::c_int = 0;
    let mut backup_mbmi: MB_MODE_INFO = *left_mbmi;
    av1_setup_build_prediction_by_left_pred(
        xd,
        rel_mi_row,
        op_mi_size,
        &raw mut backup_mbmi,
        ctxt,
        num_planes,
    );
    mi_x = (*xd).mi_col << MI_SIZE_LOG2;
    mi_y = left_mi_row << MI_SIZE_LOG2;
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < num_planes {
        let mut pd: *const macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(j as isize) as *mut macroblockd_plane;
        let mut bw: core::ffi::c_int = clamp(
            block_size_wide[bsize as usize] as core::ffi::c_int
                >> (*pd).subsampling_x + 1 as core::ffi::c_int,
            4 as core::ffi::c_int,
            block_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
                >> (*pd).subsampling_x + 1 as core::ffi::c_int,
        );
        let mut bh: core::ffi::c_int = (op_mi_size as core::ffi::c_int) << MI_SIZE_LOG2
            >> (*pd).subsampling_y;
        if !(av1_skip_u4x4_pred_in_obmc(bsize, pd, 1 as core::ffi::c_int) != 0) {
            dec_build_inter_predictors(
                (*ctxt).cm,
                (*ctxt).dcb as *mut DecoderCodingBlock,
                j,
                &raw mut backup_mbmi,
                1 as core::ffi::c_int,
                bw,
                bh,
                mi_x,
                mi_y,
            );
        }
        j += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn dec_build_prediction_by_left_preds(
    mut cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut tmp_buf: *mut *mut uint8_t,
    mut tmp_width: *mut core::ffi::c_int,
    mut tmp_height: *mut core::ffi::c_int,
    mut tmp_stride: *mut core::ffi::c_int,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    if !(*xd).left_available {
        return;
    }
    let this_width: core::ffi::c_int = (*xd).width as core::ffi::c_int * MI_SIZE;
    let pred_width: core::ffi::c_int = if (this_width / 2 as core::ffi::c_int)
        < 32 as core::ffi::c_int
    {
        this_width / 2 as core::ffi::c_int
    } else {
        32 as core::ffi::c_int
    };
    (*xd).mb_to_right_edge += (this_width - pred_width) * 8 as core::ffi::c_int;
    let mut ctxt: build_prediction_ctxt = {
        let mut init = build_prediction_ctxt {
            cm: cm,
            tmp_buf: tmp_buf as *mut *mut uint8_t,
            tmp_width: tmp_width as *mut core::ffi::c_int,
            tmp_height: tmp_height as *mut core::ffi::c_int,
            tmp_stride: tmp_stride as *mut core::ffi::c_int,
            mb_to_far_edge: (*xd).mb_to_bottom_edge,
            dcb: dcb as *mut core::ffi::c_void,
        };
        init
    };
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    foreach_overlappable_nb_left(
        cm,
        xd,
        max_neighbor_obmc[mi_size_high_log2[bsize as usize] as usize],
        Some(
            dec_build_prediction_by_left_pred
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    uint8_t,
                    core::ffi::c_int,
                    *mut MB_MODE_INFO,
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                ) -> (),
        ),
        &raw mut ctxt as *mut build_prediction_ctxt as *mut core::ffi::c_void,
    );
    (*xd).mb_to_top_edge = -((*xd).mi_row
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int);
    (*xd).mb_to_right_edge -= (this_width - pred_width) * 8 as core::ffi::c_int;
    (*xd).mb_to_bottom_edge = ctxt.mb_to_far_edge;
}
#[inline]
unsafe extern "C-unwind" fn dec_build_obmc_inter_predictors_sb(
    mut cm: *const AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut dst_buf1: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut dst_buf2: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut dst_stride1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_stride2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_width1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_width2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_height1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_height2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    av1_setup_obmc_dst_bufs(xd, dst_buf1.as_mut_ptr(), dst_buf2.as_mut_ptr());
    dec_build_prediction_by_above_preds(
        cm,
        dcb,
        dst_buf1.as_mut_ptr(),
        dst_width1.as_mut_ptr(),
        dst_height1.as_mut_ptr(),
        dst_stride1.as_mut_ptr(),
    );
    dec_build_prediction_by_left_preds(
        cm,
        dcb,
        dst_buf2.as_mut_ptr(),
        dst_width2.as_mut_ptr(),
        dst_height2.as_mut_ptr(),
        dst_stride2.as_mut_ptr(),
    );
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize,
        &raw mut (*(*cm).cur_frame).buf,
        mi_row,
        mi_col,
        0 as core::ffi::c_int,
        num_planes,
    );
    av1_build_obmc_inter_prediction(
        cm,
        xd,
        dst_buf1.as_mut_ptr(),
        dst_stride1.as_mut_ptr(),
        dst_buf2.as_mut_ptr(),
        dst_stride2.as_mut_ptr(),
    );
}
#[inline]
unsafe extern "C-unwind" fn cfl_store_inter_block(cm: *mut AV1_COMMON, xd: *mut MACROBLOCKD) {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if store_cfl_required(cm, xd) != 0 {
        cfl_store_block(xd, (*mbmi).bsize, (*mbmi).tx_size);
    }
}
#[inline]
unsafe extern "C-unwind" fn predict_inter_block(
    cm: *mut AV1_COMMON,
    mut dcb: *mut DecoderCodingBlock,
    mut bsize: BLOCK_SIZE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + has_second_ref(mbmi) {
        let frame: MV_REFERENCE_FRAME = (*mbmi).ref_frame[ref_0 as usize];
        if (frame as core::ffi::c_int) < LAST_FRAME as core::ffi::c_int {
            if (is_intrabc_block(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"predict_inter_block\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    863 as core::ffi::c_int,
                    b"is_intrabc_block(mbmi)\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(frame as core::ffi::c_int == INTRA_FRAME as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"predict_inter_block\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    864 as core::ffi::c_int,
                    b"frame == INTRA_FRAME\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(ref_0 == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"predict_inter_block\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    865 as core::ffi::c_int,
                    b"ref == 0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else {
            let mut ref_buf: *const RefCntBuffer = get_ref_frame_buf(cm, frame);
            let mut ref_scale_factors: *const scale_factors = get_ref_scale_factors_const(
                cm,
                frame,
            );
            (*xd).block_ref_scale_factors[ref_0 as usize] = ref_scale_factors;
            av1_setup_pre_planes(
                xd,
                ref_0,
                &(*ref_buf).buf,
                mi_row,
                mi_col,
                ref_scale_factors,
                num_planes,
            );
        }
        ref_0 += 1;
    }
    dec_build_inter_predictor(cm, dcb, mi_row, mi_col, bsize);
    if (*mbmi).motion_mode as core::ffi::c_int == OBMC_CAUSAL as core::ffi::c_int {
        dec_build_obmc_inter_predictors_sb(cm, dcb);
    }
}
#[inline]
unsafe extern "C-unwind" fn set_color_index_map_offset(
    xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut r: *mut aom_reader,
) {
    let mut params: Av1ColorMapParam = Av1ColorMapParam {
        rows: 0,
        cols: 0,
        n_colors: 0,
        plane_width: 0,
        plane_height: 0,
        color_map: 0 as *mut uint8_t,
        map_cdf: 0 as *mut [[aom_cdf_prob; 9]; 5],
        color_cost: 0 as *const [[core::ffi::c_int; 8]; 5],
    };
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    av1_get_block_dimensions(
        (*mbmi).bsize,
        plane,
        xd,
        &raw mut params.plane_width,
        &raw mut params.plane_height,
        0 as *mut core::ffi::c_int,
        0 as *mut core::ffi::c_int,
    );
    (*xd).color_index_map_offset[plane as usize] = ((*xd)
        .color_index_map_offset[plane as usize] as core::ffi::c_int
        + params.plane_width * params.plane_height) as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn decode_token_recon_block(
    pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut r: *mut aom_reader,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if is_inter_block(mbmi) == 0 {
        let mut row: core::ffi::c_int = 0;
        let mut col: core::ffi::c_int = 0;
        if !(bsize as core::ffi::c_int
            == get_plane_block_size(
                bsize,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_x,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_y,
            ) as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"decode_token_recon_block\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                926 as core::ffi::c_int,
                b"bsize == get_plane_block_size(bsize, xd->plane[0].subsampling_x, xd->plane[0].subsampling_y)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let max_blocks_wide: core::ffi::c_int = max_block_wide(
            xd,
            bsize,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let max_blocks_high: core::ffi::c_int = max_block_high(
            xd,
            bsize,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let max_unit_bsize: BLOCK_SIZE = BLOCK_64X64;
        let mut mu_blocks_wide: core::ffi::c_int = mi_size_wide[max_unit_bsize as usize]
            as core::ffi::c_int;
        let mut mu_blocks_high: core::ffi::c_int = mi_size_high[max_unit_bsize as usize]
            as core::ffi::c_int;
        mu_blocks_wide = if max_blocks_wide < mu_blocks_wide {
            max_blocks_wide
        } else {
            mu_blocks_wide
        };
        mu_blocks_high = if max_blocks_high < mu_blocks_high {
            max_blocks_high
        } else {
            mu_blocks_high
        };
        row = 0 as core::ffi::c_int;
        while row < max_blocks_high {
            col = 0 as core::ffi::c_int;
            while col < max_blocks_wide {
                let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
                while plane < num_planes {
                    if plane != 0 && !(*xd).is_chroma_ref {
                        break;
                    }
                    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
                        .as_mut_ptr()
                        .offset(plane as isize) as *mut macroblockd_plane;
                    let tx_size: TX_SIZE = av1_get_tx_size(plane, xd) as TX_SIZE;
                    let stepr: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
                    let stepc: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
                    let unit_height: core::ffi::c_int = (if mu_blocks_high + row
                        < max_blocks_high
                    {
                        mu_blocks_high + row
                    } else {
                        max_blocks_high
                    })
                        + ((1 as core::ffi::c_int) << (*pd).subsampling_y
                            >> 1 as core::ffi::c_int) >> (*pd).subsampling_y;
                    let unit_width: core::ffi::c_int = (if mu_blocks_wide + col
                        < max_blocks_wide
                    {
                        mu_blocks_wide + col
                    } else {
                        max_blocks_wide
                    })
                        + ((1 as core::ffi::c_int) << (*pd).subsampling_x
                            >> 1 as core::ffi::c_int) >> (*pd).subsampling_x;
                    let mut blk_row: core::ffi::c_int = row >> (*pd).subsampling_y;
                    while blk_row < unit_height {
                        let mut blk_col: core::ffi::c_int = col >> (*pd).subsampling_x;
                        while blk_col < unit_width {
                            ((*td).read_coeffs_tx_intra_block_visit)
                                .expect(
                                    "non-null function pointer",
                                )(cm, dcb, r, plane, blk_row, blk_col, tx_size);
                            ((*td).predict_and_recon_intra_block_visit)
                                .expect(
                                    "non-null function pointer",
                                )(cm, dcb, r, plane, blk_row, blk_col, tx_size);
                            set_cb_buffer_offsets(dcb, tx_size, plane);
                            blk_col += stepc;
                        }
                        blk_row += stepr;
                    }
                    plane += 1;
                }
                col += mu_blocks_wide;
            }
            row += mu_blocks_high;
        }
    } else {
        ((*td).predict_inter_block_visit)
            .expect("non-null function pointer")(cm, dcb, bsize);
        if (*mbmi).skip_txfm == 0 {
            let mut eobtotal: core::ffi::c_int = 0 as core::ffi::c_int;
            let max_blocks_wide_0: core::ffi::c_int = max_block_wide(
                xd,
                bsize,
                0 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let max_blocks_high_0: core::ffi::c_int = max_block_high(
                xd,
                bsize,
                0 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let mut row_0: core::ffi::c_int = 0;
            let mut col_0: core::ffi::c_int = 0;
            let max_unit_bsize_0: BLOCK_SIZE = BLOCK_64X64;
            if !(max_unit_bsize_0 as core::ffi::c_int
                == get_plane_block_size(
                    BLOCK_64X64,
                    (*xd).plane[0 as core::ffi::c_int as usize].subsampling_x,
                    (*xd).plane[0 as core::ffi::c_int as usize].subsampling_y,
                ) as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"decode_token_recon_block\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    988 as core::ffi::c_int,
                    b"max_unit_bsize == get_plane_block_size(BLOCK_64X64, xd->plane[0].subsampling_x, xd->plane[0].subsampling_y)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut mu_blocks_wide_0: core::ffi::c_int = mi_size_wide[max_unit_bsize_0
                as usize] as core::ffi::c_int;
            let mut mu_blocks_high_0: core::ffi::c_int = mi_size_high[max_unit_bsize_0
                as usize] as core::ffi::c_int;
            mu_blocks_wide_0 = if max_blocks_wide_0 < mu_blocks_wide_0 {
                max_blocks_wide_0
            } else {
                mu_blocks_wide_0
            };
            mu_blocks_high_0 = if max_blocks_high_0 < mu_blocks_high_0 {
                max_blocks_high_0
            } else {
                mu_blocks_high_0
            };
            row_0 = 0 as core::ffi::c_int;
            while row_0 < max_blocks_high_0 {
                col_0 = 0 as core::ffi::c_int;
                while col_0 < max_blocks_wide_0 {
                    let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while plane_0 < num_planes {
                        if plane_0 != 0 && !(*xd).is_chroma_ref {
                            break;
                        }
                        let pd_0: *const macroblockd_plane = &raw mut *((*xd).plane)
                            .as_mut_ptr()
                            .offset(plane_0 as isize) as *mut macroblockd_plane;
                        let ss_x: core::ffi::c_int = (*pd_0).subsampling_x;
                        let ss_y: core::ffi::c_int = (*pd_0).subsampling_y;
                        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
                            bsize,
                            ss_x,
                            ss_y,
                        ) as BLOCK_SIZE;
                        let max_tx_size: TX_SIZE = get_vartx_max_txsize(
                            xd,
                            plane_bsize,
                            plane_0,
                        ) as TX_SIZE;
                        let bh_var_tx: core::ffi::c_int = tx_size_high_unit[max_tx_size
                            as usize];
                        let bw_var_tx: core::ffi::c_int = tx_size_wide_unit[max_tx_size
                            as usize];
                        let mut block: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut step: core::ffi::c_int = tx_size_wide_unit[max_tx_size
                            as usize] * tx_size_high_unit[max_tx_size as usize];
                        let mut blk_row_0: core::ffi::c_int = 0;
                        let mut blk_col_0: core::ffi::c_int = 0;
                        let unit_height_0: core::ffi::c_int = (if mu_blocks_high_0
                            + row_0 < max_blocks_high_0
                        {
                            mu_blocks_high_0 + row_0
                        } else {
                            max_blocks_high_0
                        }) + ((1 as core::ffi::c_int) << ss_y >> 1 as core::ffi::c_int)
                            >> ss_y;
                        let unit_width_0: core::ffi::c_int = (if mu_blocks_wide_0 + col_0
                            < max_blocks_wide_0
                        {
                            mu_blocks_wide_0 + col_0
                        } else {
                            max_blocks_wide_0
                        }) + ((1 as core::ffi::c_int) << ss_x >> 1 as core::ffi::c_int)
                            >> ss_x;
                        blk_row_0 = row_0 >> ss_y;
                        while blk_row_0 < unit_height_0 {
                            blk_col_0 = col_0 >> ss_x;
                            while blk_col_0 < unit_width_0 {
                                decode_reconstruct_tx(
                                    cm,
                                    td,
                                    r,
                                    mbmi,
                                    plane_0,
                                    plane_bsize,
                                    blk_row_0,
                                    blk_col_0,
                                    block,
                                    max_tx_size,
                                    &raw mut eobtotal,
                                );
                                block += step;
                                blk_col_0 += bw_var_tx;
                            }
                            blk_row_0 += bh_var_tx;
                        }
                        plane_0 += 1;
                    }
                    col_0 += mu_blocks_wide_0;
                }
                row_0 += mu_blocks_high_0;
            }
        }
        ((*td).cfl_store_inter_block_visit).expect("non-null function pointer")(cm, xd);
    }
    av1_visit_palette(
        pbi,
        xd,
        r,
        Some(
            set_color_index_map_offset
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    *mut aom_reader,
                ) -> (),
        ),
    );
}
#[inline]
unsafe extern "C-unwind" fn set_inter_tx_size(
    mut mbmi: *mut MB_MODE_INFO,
    mut stride_log2: core::ffi::c_int,
    mut tx_w_log2: core::ffi::c_int,
    mut tx_h_log2: core::ffi::c_int,
    mut min_txs: core::ffi::c_int,
    mut split_size: core::ffi::c_int,
    mut txs: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
) {
    let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy < tx_size_high_unit[split_size as usize] {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < tx_size_wide_unit[split_size as usize] {
            let index: core::ffi::c_int = (blk_row + idy >> tx_h_log2 << stride_log2)
                + (blk_col + idx >> tx_w_log2);
            (*mbmi).inter_tx_size[index as usize] = txs as TX_SIZE;
            idx += tx_size_wide_unit[min_txs as usize];
        }
        idy += tx_size_high_unit[min_txs as usize];
    }
}
#[inline]
unsafe extern "C-unwind" fn read_tx_size_vartx(
    mut xd: *mut MACROBLOCKD,
    mut mbmi: *mut MB_MODE_INFO,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut r: *mut aom_reader,
) {
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let mut is_split: core::ffi::c_int = 0 as core::ffi::c_int;
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
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    if !(tx_size as core::ffi::c_int > TX_4X4 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"read_tx_size_vartx\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1073 as core::ffi::c_int,
            b"tx_size > TX_4X4\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut txs: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
    let mut level: core::ffi::c_int = 0 as core::ffi::c_int;
    while level < MAX_VARTX_DEPTH - 1 as core::ffi::c_int {
        txs = sub_tx_size_map[txs as usize];
        level += 1;
    }
    let tx_w_log2: core::ffi::c_int = tx_size_wide_log2[txs as usize] - MI_SIZE_LOG2;
    let tx_h_log2: core::ffi::c_int = tx_size_high_log2[txs as usize] - MI_SIZE_LOG2;
    let bw_log2: core::ffi::c_int = mi_size_wide_log2[bsize as usize]
        as core::ffi::c_int;
    let stride_log2: core::ffi::c_int = bw_log2 - tx_w_log2;
    if depth == MAX_VARTX_DEPTH {
        set_inter_tx_size(
            mbmi,
            stride_log2,
            tx_w_log2,
            tx_h_log2,
            txs as core::ffi::c_int,
            tx_size as core::ffi::c_int,
            tx_size as core::ffi::c_int,
            blk_row,
            blk_col,
        );
        (*mbmi).tx_size = tx_size;
        txfm_partition_update(
            ((*xd).above_txfm_context).offset(blk_col as isize),
            ((*xd).left_txfm_context).offset(blk_row as isize),
            tx_size,
            tx_size,
        );
        return;
    }
    let ctx: core::ffi::c_int = txfm_partition_context(
        ((*xd).above_txfm_context).offset(blk_col as isize),
        ((*xd).left_txfm_context).offset(blk_row as isize),
        (*mbmi).bsize,
        tx_size,
    ) as core::ffi::c_int;
    is_split = aom_read_symbol_(
        r,
        ((*ec_ctx).txfm_partition_cdf[ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    );
    if is_split != 0 {
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        let bsw: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let bsh: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        if sub_txs as core::ffi::c_int == TX_4X4 as core::ffi::c_int {
            set_inter_tx_size(
                mbmi,
                stride_log2,
                tx_w_log2,
                tx_h_log2,
                txs as core::ffi::c_int,
                tx_size as core::ffi::c_int,
                sub_txs as core::ffi::c_int,
                blk_row,
                blk_col,
            );
            (*mbmi).tx_size = sub_txs;
            txfm_partition_update(
                ((*xd).above_txfm_context).offset(blk_col as isize),
                ((*xd).left_txfm_context).offset(blk_row as isize),
                sub_txs,
                tx_size,
            );
            return;
        }
        if !(bsw > 0 as core::ffi::c_int && bsh > 0 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"read_tx_size_vartx\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1110 as core::ffi::c_int,
                b"bsw > 0 && bsh > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < tx_size_high_unit[tx_size as usize] {
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < tx_size_wide_unit[tx_size as usize] {
                let mut offsetr: core::ffi::c_int = blk_row + row;
                let mut offsetc: core::ffi::c_int = blk_col + col;
                read_tx_size_vartx(
                    xd,
                    mbmi,
                    sub_txs,
                    depth + 1 as core::ffi::c_int,
                    offsetr,
                    offsetc,
                    r,
                );
                col += bsw;
            }
            row += bsh;
        }
    } else {
        set_inter_tx_size(
            mbmi,
            stride_log2,
            tx_w_log2,
            tx_h_log2,
            txs as core::ffi::c_int,
            tx_size as core::ffi::c_int,
            tx_size as core::ffi::c_int,
            blk_row,
            blk_col,
        );
        (*mbmi).tx_size = tx_size;
        txfm_partition_update(
            ((*xd).above_txfm_context).offset(blk_col as isize),
            ((*xd).left_txfm_context).offset(blk_row as isize),
            tx_size,
            tx_size,
        );
    };
}
unsafe extern "C-unwind" fn read_selected_tx_size(
    xd: *const MACROBLOCKD,
    mut r: *mut aom_reader,
) -> TX_SIZE {
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    let tx_size_cat: int32_t = bsize_to_tx_size_cat(bsize) as int32_t;
    let max_depths: core::ffi::c_int = bsize_to_max_depth(bsize) as core::ffi::c_int;
    let ctx: core::ffi::c_int = get_tx_size_context(xd) as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let depth: core::ffi::c_int = aom_read_symbol_(
        r,
        ((*ec_ctx).tx_size_cdf[tx_size_cat as usize][ctx as usize]).as_mut_ptr(),
        max_depths + 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if !(depth >= 0 as core::ffi::c_int && depth <= max_depths) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"read_selected_tx_size\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1138 as core::ffi::c_int,
            b"depth >= 0 && depth <= max_depths\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let tx_size: TX_SIZE = depth_to_tx_size(depth, bsize) as TX_SIZE;
    return tx_size;
}
unsafe extern "C-unwind" fn read_tx_size(
    xd: *const MACROBLOCKD,
    mut tx_mode: TX_MODE,
    mut is_inter: core::ffi::c_int,
    mut allow_select_inter: core::ffi::c_int,
    mut r: *mut aom_reader,
) -> TX_SIZE {
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    if (*xd)
        .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
        as usize] != 0
    {
        return TX_4X4 as core::ffi::c_int as TX_SIZE;
    }
    if block_signals_txsize(bsize) != 0 {
        if (is_inter == 0 || allow_select_inter != 0)
            && tx_mode as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int
        {
            let coded_tx_size: TX_SIZE = read_selected_tx_size(xd, r) as TX_SIZE;
            return coded_tx_size;
        } else {
            return tx_size_from_tx_mode(bsize, tx_mode)
        }
    } else {
        if !(!(tx_mode as core::ffi::c_int == ONLY_4X4 as core::ffi::c_int)
            || bsize as core::ffi::c_int == BLOCK_4X4 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"read_tx_size\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1157 as core::ffi::c_int,
                b"IMPLIES(tx_mode == ONLY_4X4, bsize == BLOCK_4X4)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return max_txsize_rect_lookup[bsize as usize];
    };
}
#[inline]
unsafe extern "C-unwind" fn parse_decode_block(
    pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut r: *mut aom_reader,
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) {
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    decode_mbmi_block(pbi, dcb, mi_row, mi_col, r, partition, bsize);
    av1_visit_palette(
        pbi,
        xd,
        r,
        Some(
            av1_decode_palette_tokens
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    *mut aom_reader,
                ) -> (),
        ),
    );
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut inter_block_tx: core::ffi::c_int = (is_inter_block(mbmi) != 0
        || is_intrabc_block(mbmi) != 0) as core::ffi::c_int;
    if (*cm).features.tx_mode as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int
        && block_signals_txsize(bsize) != 0 && (*mbmi).skip_txfm == 0
        && inter_block_tx != 0 && (*xd).lossless[(*mbmi).segment_id() as usize] == 0
    {
        let max_tx_size: TX_SIZE = max_txsize_rect_lookup[bsize as usize];
        let bh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
        let bw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
        let width: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
        let height: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
        let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
        while idy < height {
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < width {
                read_tx_size_vartx(
                    xd,
                    mbmi,
                    max_tx_size,
                    0 as core::ffi::c_int,
                    idy,
                    idx,
                    r,
                );
                idx += bw;
            }
            idy += bh;
        }
    } else {
        (*mbmi).tx_size = read_tx_size(
            xd,
            (*cm).features.tx_mode,
            inter_block_tx,
            ((*mbmi).skip_txfm == 0) as core::ffi::c_int,
            r,
        );
        if inter_block_tx != 0 {
            memset(
                ((*mbmi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
                (*mbmi).tx_size as core::ffi::c_int,
                ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
            );
        }
        set_txfm_ctxs(
            (*mbmi).tx_size,
            (*xd).width as core::ffi::c_int,
            (*xd).height as core::ffi::c_int,
            ((*mbmi).skip_txfm as core::ffi::c_int != 0 && is_inter_block(mbmi) != 0)
                as core::ffi::c_int,
            xd,
        );
    }
    if (*cm).delta_q_info.delta_q_present_flag != 0 {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < MAX_SEGMENTS {
            let current_qindex: core::ffi::c_int = av1_get_qindex(
                &raw mut (*cm).seg,
                i,
                (*xd).current_base_qindex,
            ) as core::ffi::c_int;
            let quant_params: *const CommonQuantParams = &raw mut (*cm).quant_params;
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < num_planes {
                let dc_delta_q: core::ffi::c_int = if j == 0 as core::ffi::c_int {
                    (*quant_params).y_dc_delta_q
                } else if j == 1 as core::ffi::c_int {
                    (*quant_params).u_dc_delta_q
                } else {
                    (*quant_params).v_dc_delta_q
                };
                let ac_delta_q: core::ffi::c_int = if j == 0 as core::ffi::c_int {
                    0 as core::ffi::c_int
                } else if j == 1 as core::ffi::c_int {
                    (*quant_params).u_ac_delta_q
                } else {
                    (*quant_params).v_ac_delta_q
                };
                (*xd)
                    .plane[j as usize]
                    .seg_dequant_QTX[i as usize][0 as core::ffi::c_int as usize] = av1_dc_quant_QTX(
                    current_qindex,
                    dc_delta_q,
                    (*(*cm).seq_params).bit_depth,
                );
                (*xd)
                    .plane[j as usize]
                    .seg_dequant_QTX[i as usize][1 as core::ffi::c_int as usize] = av1_ac_quant_QTX(
                    current_qindex,
                    ac_delta_q,
                    (*(*cm).seq_params).bit_depth,
                );
                j += 1;
            }
            i += 1;
        }
    }
    if (*mbmi).skip_txfm != 0 {
        av1_reset_entropy_context(xd, bsize, num_planes);
    }
    decode_token_recon_block(pbi, td, r, bsize);
}
#[inline]
unsafe extern "C-unwind" fn set_offsets_for_pred_and_recon(
    pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let offset: core::ffi::c_int = mi_row * (*mi_params).mi_stride + mi_col;
    let tile: *const TileInfo = &raw mut (*xd).tile;
    (*xd).mi = ((*mi_params).mi_grid_base).offset(offset as isize);
    (*xd).tx_type_map = &raw mut *((*mi_params).tx_type_map)
        .offset((mi_row * (*mi_params).mi_stride + mi_col) as isize) as *mut TX_TYPE
        as *mut uint8_t;
    (*xd).tx_type_map_stride = (*mi_params).mi_stride;
    set_plane_n4(xd, bw, bh, num_planes);
    set_mi_row_col(
        xd,
        tile,
        mi_row,
        bh,
        mi_col,
        bw,
        (*mi_params).mi_rows,
        (*mi_params).mi_cols,
    );
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        bsize,
        &raw mut (*(*cm).cur_frame).buf,
        mi_row,
        mi_col,
        0 as core::ffi::c_int,
        num_planes,
    );
}
#[inline]
unsafe extern "C-unwind" fn decode_block(
    pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut r: *mut aom_reader,
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) {
    set_offsets_for_pred_and_recon(pbi, td, mi_row, mi_col, bsize);
    decode_token_recon_block(pbi, td, r, bsize);
}
unsafe extern "C-unwind" fn read_partition(
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut r: *mut aom_reader,
    mut has_rows: core::ffi::c_int,
    mut has_cols: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> PARTITION_TYPE {
    let ctx: core::ffi::c_int = partition_plane_context(xd, mi_row, mi_col, bsize)
        as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if has_rows == 0 && has_cols == 0 {
        return PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE;
    }
    if !(ctx >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"read_partition\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1274 as core::ffi::c_int,
            b"ctx >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut partition_cdf: *mut aom_cdf_prob = ((*ec_ctx).partition_cdf[ctx as usize])
        .as_mut_ptr();
    if has_rows != 0 && has_cols != 0 {
        return aom_read_symbol_(r, partition_cdf, partition_cdf_length(bsize))
            as PARTITION_TYPE
    } else if has_rows == 0 && has_cols != 0 {
        if !(bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"read_partition\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1280 as core::ffi::c_int,
                b"bsize > BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut cdf: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_vert_alike(cdf.as_mut_ptr(), partition_cdf, bsize);
        if !(cdf[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"read_partition\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1283 as core::ffi::c_int,
                b"cdf[1] == AOM_ICDF(CDF_PROB_TOP)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return (if aom_read_cdf_(r, cdf.as_mut_ptr(), 2 as core::ffi::c_int) != 0 {
            PARTITION_SPLIT as core::ffi::c_int
        } else {
            PARTITION_HORZ as core::ffi::c_int
        }) as PARTITION_TYPE;
    } else {
        if !(has_rows != 0 && has_cols == 0) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"read_partition\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1286 as core::ffi::c_int,
                b"has_rows && !has_cols\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"read_partition\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1287 as core::ffi::c_int,
                b"bsize > BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut cdf_0: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_horz_alike(cdf_0.as_mut_ptr(), partition_cdf, bsize);
        if !(cdf_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"read_partition\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1290 as core::ffi::c_int,
                b"cdf[1] == AOM_ICDF(CDF_PROB_TOP)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return (if aom_read_cdf_(r, cdf_0.as_mut_ptr(), 2 as core::ffi::c_int) != 0 {
            PARTITION_SPLIT as core::ffi::c_int
        } else {
            PARTITION_VERT as core::ffi::c_int
        }) as PARTITION_TYPE;
    };
}
#[inline]
unsafe extern "C-unwind" fn decode_partition(
    pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut reader: *mut aom_reader,
    mut bsize: BLOCK_SIZE,
    mut parse_decode_flag: core::ffi::c_int,
) {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"decode_partition\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1299 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let hbs: core::ffi::c_int = bw >> 1 as core::ffi::c_int;
    let mut partition: PARTITION_TYPE = 0;
    let mut subsize: BLOCK_SIZE = BLOCK_4X4;
    let quarter_step: core::ffi::c_int = bw / 4 as core::ffi::c_int;
    let mut bsize2: BLOCK_SIZE = get_partition_subsize(
        bsize,
        PARTITION_SPLIT as core::ffi::c_int as PARTITION_TYPE,
    );
    let has_rows: core::ffi::c_int = (mi_row + hbs < (*cm).mi_params.mi_rows)
        as core::ffi::c_int;
    let has_cols: core::ffi::c_int = (mi_col + hbs < (*cm).mi_params.mi_cols)
        as core::ffi::c_int;
    if mi_row >= (*cm).mi_params.mi_rows || mi_col >= (*cm).mi_params.mi_cols {
        return;
    }
    static mut block_visit: [block_visitor_fn_t; 4] = unsafe {
        [
            None,
            Some(
                parse_decode_block
                    as unsafe extern "C-unwind" fn(
                        *mut AV1Decoder,
                        *mut ThreadData,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        *mut aom_reader,
                        PARTITION_TYPE,
                        BLOCK_SIZE,
                    ) -> (),
            ),
            Some(
                decode_block
                    as unsafe extern "C-unwind" fn(
                        *mut AV1Decoder,
                        *mut ThreadData,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        *mut aom_reader,
                        PARTITION_TYPE,
                        BLOCK_SIZE,
                    ) -> (),
            ),
            Some(
                parse_decode_block
                    as unsafe extern "C-unwind" fn(
                        *mut AV1Decoder,
                        *mut ThreadData,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        *mut aom_reader,
                        PARTITION_TYPE,
                        BLOCK_SIZE,
                    ) -> (),
            ),
        ]
    };
    if parse_decode_flag & 1 as core::ffi::c_int != 0 {
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < num_planes {
            let mut rcol0: core::ffi::c_int = 0;
            let mut rcol1: core::ffi::c_int = 0;
            let mut rrow0: core::ffi::c_int = 0;
            let mut rrow1: core::ffi::c_int = 0;
            if !((*cm).rst_info[plane as usize].frame_restoration_type
                as core::ffi::c_uint
                == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
            {
                if av1_loop_restoration_corners_in_sb(
                    cm,
                    plane,
                    mi_row,
                    mi_col,
                    bsize,
                    &raw mut rcol0,
                    &raw mut rcol1,
                    &raw mut rrow0,
                    &raw mut rrow1,
                ) != 0
                {
                    let rstride: core::ffi::c_int = (*cm)
                        .rst_info[plane as usize]
                        .horz_units;
                    let mut rrow: core::ffi::c_int = rrow0;
                    while rrow < rrow1 {
                        let mut rcol: core::ffi::c_int = rcol0;
                        while rcol < rcol1 {
                            let runit_idx: core::ffi::c_int = rcol + rrow * rstride;
                            loop_restoration_read_sb_coeffs(
                                cm,
                                xd,
                                reader,
                                plane,
                                runit_idx,
                            );
                            rcol += 1;
                        }
                        rrow += 1;
                    }
                }
            }
            plane += 1;
        }
        partition = (if (bsize as core::ffi::c_int) < BLOCK_8X8 as core::ffi::c_int {
            PARTITION_NONE as core::ffi::c_int
        } else {
            read_partition(xd, mi_row, mi_col, reader, has_rows, has_cols, bsize)
                as core::ffi::c_int
        }) as PARTITION_TYPE;
    } else {
        partition = get_partition(cm, mi_row, mi_col, bsize);
    }
    subsize = get_partition_subsize(bsize, partition);
    if subsize as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int {
        (*xd).mi_row = mi_row;
        aom_internal_error(
            (*xd).error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Partition is invalid for block size %dx%d\0" as *const u8
                as *const core::ffi::c_char,
            block_size_wide[bsize as usize] as core::ffi::c_int,
            block_size_high[bsize as usize] as core::ffi::c_int,
        );
    }
    let pd_u: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    if get_plane_block_size(subsize, (*pd_u).subsampling_x, (*pd_u).subsampling_y)
        as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int
    {
        (*xd).mi_row = mi_row;
        aom_internal_error(
            (*xd).error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Block size %dx%d invalid with this subsampling mode\0" as *const u8
                as *const core::ffi::c_char,
            block_size_wide[subsize as usize] as core::ffi::c_int,
            block_size_high[subsize as usize] as core::ffi::c_int,
        );
    }
    match partition as core::ffi::c_int {
        0 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, subsize);
        }
        1 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, subsize);
            if has_rows != 0 {
                (block_visit[parse_decode_flag as usize])
                    .expect(
                        "non-null function pointer",
                    )(pbi, td, mi_row + hbs, mi_col, reader, partition, subsize);
            }
        }
        2 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, subsize);
            if has_cols != 0 {
                (block_visit[parse_decode_flag as usize])
                    .expect(
                        "non-null function pointer",
                    )(pbi, td, mi_row, mi_col + hbs, reader, partition, subsize);
            }
        }
        3 => {
            decode_partition(
                pbi,
                td,
                mi_row,
                mi_col,
                reader,
                subsize,
                parse_decode_flag,
            );
            decode_partition(
                pbi,
                td,
                mi_row,
                mi_col + hbs,
                reader,
                subsize,
                parse_decode_flag,
            );
            decode_partition(
                pbi,
                td,
                mi_row + hbs,
                mi_col,
                reader,
                subsize,
                parse_decode_flag,
            );
            decode_partition(
                pbi,
                td,
                mi_row + hbs,
                mi_col + hbs,
                reader,
                subsize,
                parse_decode_flag,
            );
        }
        4 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col + hbs, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row + hbs, mi_col, reader, partition, subsize);
        }
        5 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, subsize);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row + hbs, mi_col, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row + hbs, mi_col + hbs, reader, partition, bsize2);
        }
        6 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row + hbs, mi_col, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col + hbs, reader, partition, subsize);
        }
        7 => {
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col, reader, partition, subsize);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row, mi_col + hbs, reader, partition, bsize2);
            (block_visit[parse_decode_flag as usize])
                .expect(
                    "non-null function pointer",
                )(pbi, td, mi_row + hbs, mi_col + hbs, reader, partition, bsize2);
        }
        8 => {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                let mut this_mi_row: core::ffi::c_int = mi_row + i * quarter_step;
                if i > 0 as core::ffi::c_int && this_mi_row >= (*cm).mi_params.mi_rows {
                    break;
                }
                (block_visit[parse_decode_flag as usize])
                    .expect(
                        "non-null function pointer",
                    )(pbi, td, this_mi_row, mi_col, reader, partition, subsize);
                i += 1;
            }
        }
        9 => {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < 4 as core::ffi::c_int {
                let mut this_mi_col: core::ffi::c_int = mi_col + i_0 * quarter_step;
                if i_0 > 0 as core::ffi::c_int && this_mi_col >= (*cm).mi_params.mi_cols
                {
                    break;
                }
                (block_visit[parse_decode_flag as usize])
                    .expect(
                        "non-null function pointer",
                    )(pbi, td, mi_row, this_mi_col, reader, partition, subsize);
                i_0 += 1;
            }
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid partition type\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"decode_partition\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    1432 as core::ffi::c_int,
                    b"0 && \"Invalid partition type\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    if parse_decode_flag & 1 as core::ffi::c_int != 0 {
        update_ext_partition_context(xd, mi_row, mi_col, subsize, bsize, partition);
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_bool_decoder(
    xd: *mut MACROBLOCKD,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    read_size: size_t,
    mut error_info: *mut aom_internal_error_info,
    mut r: *mut aom_reader,
    mut allow_update_cdf: uint8_t,
) {
    if read_is_valid(data, read_size, data_end) == 0 {
        (*xd).mi_row = (*xd).tile.mi_row_start;
        aom_internal_error(
            error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt tile length\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if aom_reader_init(r, data, read_size) != 0 {
        (*xd).mi_row = (*xd).tile.mi_row_start;
        aom_internal_error(
            error_info,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate bool decoder %d\0" as *const u8
                as *const core::ffi::c_char,
            1 as core::ffi::c_int,
        );
    }
    (*r).allow_update_cdf = allow_update_cdf;
}
#[inline]
unsafe extern "C-unwind" fn setup_segmentation(
    cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    let seg: *mut segmentation = &raw mut (*cm).seg;
    (*seg).update_map = 0 as uint8_t;
    (*seg).update_data = 0 as uint8_t;
    (*seg).temporal_update = 0 as uint8_t;
    (*seg).enabled = aom_rb_read_bit(rb) as uint8_t;
    if (*seg).enabled == 0 {
        if !((*(*cm).cur_frame).seg_map).is_null() {
            memset(
                (*(*cm).cur_frame).seg_map as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ((*(*cm).cur_frame).mi_rows * (*(*cm).cur_frame).mi_cols) as size_t,
            );
        }
        memset(
            seg as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<segmentation>() as size_t,
        );
        segfeatures_copy(&raw mut (*(*cm).cur_frame).seg, seg);
        return;
    }
    if (*cm).seg.enabled as core::ffi::c_int != 0 && !((*cm).prev_frame).is_null()
        && (*cm).mi_params.mi_rows == (*(*cm).prev_frame).mi_rows
        && (*cm).mi_params.mi_cols == (*(*cm).prev_frame).mi_cols
    {
        (*cm).last_frame_seg_map = (*(*cm).prev_frame).seg_map;
    } else {
        (*cm).last_frame_seg_map = 0 as *mut uint8_t;
    }
    if (*cm).features.primary_ref_frame == PRIMARY_REF_NONE {
        (*seg).update_map = 1 as uint8_t;
        (*seg).temporal_update = 0 as uint8_t;
        (*seg).update_data = 1 as uint8_t;
    } else {
        (*seg).update_map = aom_rb_read_bit(rb) as uint8_t;
        if (*seg).update_map != 0 {
            (*seg).temporal_update = aom_rb_read_bit(rb) as uint8_t;
        } else {
            (*seg).temporal_update = 0 as uint8_t;
        }
        (*seg).update_data = aom_rb_read_bit(rb) as uint8_t;
    }
    if (*seg).update_data != 0 {
        av1_clearall_segfeatures(seg);
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < MAX_SEGMENTS {
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < SEG_LVL_MAX as core::ffi::c_int {
                let mut data: core::ffi::c_int = 0 as core::ffi::c_int;
                let feature_enabled: core::ffi::c_int = aom_rb_read_bit(rb)
                    as core::ffi::c_int;
                if feature_enabled != 0 {
                    av1_enable_segfeature(seg, i, j as SEG_LVL_FEATURES);
                    let data_max: core::ffi::c_int = av1_seg_feature_data_max(
                        j as SEG_LVL_FEATURES,
                    ) as core::ffi::c_int;
                    let data_min: core::ffi::c_int = -data_max;
                    let ubits: core::ffi::c_int = get_unsigned_bits(
                        data_max as core::ffi::c_uint,
                    ) as core::ffi::c_int;
                    if av1_is_segfeature_signed(j as SEG_LVL_FEATURES) != 0 {
                        data = aom_rb_read_inv_signed_literal(rb, ubits);
                    } else {
                        data = aom_rb_read_literal(rb, ubits);
                    }
                    data = clamp(data, data_min, data_max);
                }
                av1_set_segdata(seg, i, j as SEG_LVL_FEATURES, data);
                j += 1;
            }
            i += 1;
        }
        av1_calculate_segdata(seg);
    } else if !((*cm).prev_frame).is_null() {
        segfeatures_copy(seg, &raw mut (*(*cm).prev_frame).seg);
    }
    segfeatures_copy(&raw mut (*(*cm).cur_frame).seg, seg);
}
#[inline]
unsafe extern "C-unwind" fn decode_restoration_mode(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    if (*cm).features.all_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"decode_restoration_mode\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1550 as core::ffi::c_int,
            b"!cm->features.all_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if (*cm).features.allow_intrabc {
        return;
    }
    let mut all_none: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut chroma_none: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < num_planes {
        let mut rsi: *mut RestorationInfo = &raw mut *((*cm).rst_info)
            .as_mut_ptr()
            .offset(p as isize) as *mut RestorationInfo;
        if aom_rb_read_bit(rb) != 0 {
            (*rsi).frame_restoration_type = (if aom_rb_read_bit(rb) != 0 {
                RESTORE_SGRPROJ as core::ffi::c_int
            } else {
                RESTORE_WIENER as core::ffi::c_int
            }) as RestorationType;
        } else {
            (*rsi).frame_restoration_type = (if aom_rb_read_bit(rb) != 0 {
                RESTORE_SWITCHABLE as core::ffi::c_int
            } else {
                RESTORE_NONE as core::ffi::c_int
            }) as RestorationType;
        }
        if (*rsi).frame_restoration_type as core::ffi::c_uint
            != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            all_none = 0 as core::ffi::c_int;
            chroma_none &= (p == 0 as core::ffi::c_int) as core::ffi::c_int;
        }
        p += 1;
    }
    if all_none == 0 {
        if !((*(*cm).seq_params).sb_size as core::ffi::c_int
            == BLOCK_64X64 as core::ffi::c_int
            || (*(*cm).seq_params).sb_size as core::ffi::c_int
                == BLOCK_128X128 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"decode_restoration_mode\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                1570 as core::ffi::c_int,
                b"cm->seq_params->sb_size == BLOCK_64X64 || cm->seq_params->sb_size == BLOCK_128X128\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let sb_size: core::ffi::c_int = if (*(*cm).seq_params).sb_size
            as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        {
            128 as core::ffi::c_int
        } else {
            64 as core::ffi::c_int
        };
        let mut p_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while p_0 < num_planes {
            (*cm).rst_info[p_0 as usize].restoration_unit_size = sb_size;
            p_0 += 1;
        }
        let mut rsi_0: *mut RestorationInfo = &raw mut *((*cm).rst_info)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut RestorationInfo;
        if sb_size == 64 as core::ffi::c_int {
            (*rsi_0).restoration_unit_size <<= aom_rb_read_bit(rb);
        }
        if (*rsi_0).restoration_unit_size > 64 as core::ffi::c_int {
            (*rsi_0).restoration_unit_size <<= aom_rb_read_bit(rb);
        }
    } else {
        let size: core::ffi::c_int = RESTORATION_UNITSIZE_MAX;
        let mut p_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while p_1 < num_planes {
            (*cm).rst_info[p_1 as usize].restoration_unit_size = size;
            p_1 += 1;
        }
    }
    if num_planes > 1 as core::ffi::c_int {
        let mut s: core::ffi::c_int = if (*(*cm).seq_params).subsampling_x
            < (*(*cm).seq_params).subsampling_y
        {
            (*(*cm).seq_params).subsampling_x
        } else {
            (*(*cm).seq_params).subsampling_y
        };
        if s != 0 && chroma_none == 0 {
            (*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size = (*cm)
                .rst_info[0 as core::ffi::c_int as usize]
                .restoration_unit_size >> aom_rb_read_bit(rb) * s;
        } else {
            (*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size = (*cm)
                .rst_info[0 as core::ffi::c_int as usize]
                .restoration_unit_size;
        }
        (*cm).rst_info[2 as core::ffi::c_int as usize].restoration_unit_size = (*cm)
            .rst_info[1 as core::ffi::c_int as usize]
            .restoration_unit_size;
    }
}
#[inline]
unsafe extern "C-unwind" fn read_wiener_filter(
    mut wiener_win: core::ffi::c_int,
    mut wiener_info: *mut WienerInfo,
    mut ref_wiener_info: *mut WienerInfo,
    mut rb: *mut aom_reader,
) {
    memset(
        ((*wiener_info).vfilter).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<InterpKernel>() as size_t,
    );
    memset(
        ((*wiener_info).hfilter).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<InterpKernel>() as size_t,
    );
    if wiener_win == WIENER_WIN {
        (*wiener_info).vfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
            rb,
            (3 as core::ffi::c_int - 1 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                    / 2 as core::ffi::c_int
                - (3 as core::ffi::c_int
                    - ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                        / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
            1 as uint16_t,
            ((*ref_wiener_info).vfilter[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                - (3 as core::ffi::c_int
                    - ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                        / 2 as core::ffi::c_int)) as uint16_t,
        ) as core::ffi::c_int + WIENER_FILT_TAP0_MINV) as int16_t;
        (*wiener_info).vfilter[0 as core::ffi::c_int as usize] = (*wiener_info)
            .vfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize];
    } else {
        (*wiener_info).vfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize] = 0
            as int16_t;
        (*wiener_info).vfilter[0 as core::ffi::c_int as usize] = (*wiener_info)
            .vfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize];
    }
    (*wiener_info).vfilter[(WIENER_WIN - 2 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
        rb,
        (-(7 as core::ffi::c_int) - 1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) / 2 as core::ffi::c_int
            - (-(7 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                    / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
        2 as uint16_t,
        ((*ref_wiener_info).vfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - (-(7 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)) as uint16_t,
    ) as core::ffi::c_int + WIENER_FILT_TAP1_MINV) as int16_t;
    (*wiener_info).vfilter[1 as core::ffi::c_int as usize] = (*wiener_info)
        .vfilter[(WIENER_WIN - 2 as core::ffi::c_int) as usize];
    (*wiener_info).vfilter[(WIENER_WIN - 3 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
        rb,
        (15 as core::ffi::c_int - 1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int) / 2 as core::ffi::c_int
            - (15 as core::ffi::c_int
                - ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
        3 as uint16_t,
        ((*ref_wiener_info).vfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - (15 as core::ffi::c_int
                - ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)) as uint16_t,
    ) as core::ffi::c_int + WIENER_FILT_TAP2_MINV) as int16_t;
    (*wiener_info).vfilter[2 as core::ffi::c_int as usize] = (*wiener_info)
        .vfilter[(WIENER_WIN - 3 as core::ffi::c_int) as usize];
    (*wiener_info).vfilter[WIENER_HALFWIN as usize] = (-(2 as core::ffi::c_int)
        * ((*wiener_info).vfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
            + (*wiener_info).vfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            + (*wiener_info).vfilter[2 as core::ffi::c_int as usize]
                as core::ffi::c_int)) as int16_t;
    if wiener_win == WIENER_WIN {
        (*wiener_info).hfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
            rb,
            (3 as core::ffi::c_int - 1 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                    / 2 as core::ffi::c_int
                - (3 as core::ffi::c_int
                    - ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                        / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
            1 as uint16_t,
            ((*ref_wiener_info).hfilter[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                - (3 as core::ffi::c_int
                    - ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                        / 2 as core::ffi::c_int)) as uint16_t,
        ) as core::ffi::c_int + WIENER_FILT_TAP0_MINV) as int16_t;
        (*wiener_info).hfilter[0 as core::ffi::c_int as usize] = (*wiener_info)
            .hfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize];
    } else {
        (*wiener_info).hfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize] = 0
            as int16_t;
        (*wiener_info).hfilter[0 as core::ffi::c_int as usize] = (*wiener_info)
            .hfilter[(WIENER_WIN - 1 as core::ffi::c_int) as usize];
    }
    (*wiener_info).hfilter[(WIENER_WIN - 2 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
        rb,
        (-(7 as core::ffi::c_int) - 1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) / 2 as core::ffi::c_int
            - (-(7 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                    / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
        2 as uint16_t,
        ((*ref_wiener_info).hfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - (-(7 as core::ffi::c_int)
                - ((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)) as uint16_t,
    ) as core::ffi::c_int + WIENER_FILT_TAP1_MINV) as int16_t;
    (*wiener_info).hfilter[1 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[(WIENER_WIN - 2 as core::ffi::c_int) as usize];
    (*wiener_info).hfilter[(WIENER_WIN - 3 as core::ffi::c_int) as usize] = (aom_read_primitive_refsubexpfin_(
        rb,
        (15 as core::ffi::c_int - 1 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int) / 2 as core::ffi::c_int
            - (15 as core::ffi::c_int
                - ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    / 2 as core::ffi::c_int) + 1 as core::ffi::c_int) as uint16_t,
        3 as uint16_t,
        ((*ref_wiener_info).hfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - (15 as core::ffi::c_int
                - ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)) as uint16_t,
    ) as core::ffi::c_int + WIENER_FILT_TAP2_MINV) as int16_t;
    (*wiener_info).hfilter[2 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[(WIENER_WIN - 3 as core::ffi::c_int) as usize];
    (*wiener_info).hfilter[WIENER_HALFWIN as usize] = (-(2 as core::ffi::c_int)
        * ((*wiener_info).hfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
            + (*wiener_info).hfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            + (*wiener_info).hfilter[2 as core::ffi::c_int as usize]
                as core::ffi::c_int)) as int16_t;
    *ref_wiener_info = *wiener_info;
}
#[inline]
unsafe extern "C-unwind" fn read_sgrproj_filter(
    mut sgrproj_info: *mut SgrprojInfo,
    mut ref_sgrproj_info: *mut SgrprojInfo,
    mut rb: *mut aom_reader,
) {
    (*sgrproj_info).ep = aom_read_literal_(rb, 4 as core::ffi::c_int);
    let mut params: *const sgr_params_type = &*av1_sgr_params
        .as_ptr()
        .offset((*sgrproj_info).ep as isize) as *const sgr_params_type;
    if (*params).r[0 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        (*sgrproj_info).xqd[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
        (*sgrproj_info).xqd[1 as core::ffi::c_int as usize] = aom_read_primitive_refsubexpfin_(
            rb,
            (-((1 as core::ffi::c_int) << 7 as core::ffi::c_int) / 4 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                - 1 as core::ffi::c_int
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    / 4 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
            4 as uint16_t,
            ((*ref_sgrproj_info).xqd[1 as core::ffi::c_int as usize]
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    / 4 as core::ffi::c_int) as uint16_t,
        ) as core::ffi::c_int + SGRPROJ_PRJ_MIN1;
    } else if (*params).r[1 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        (*sgrproj_info).xqd[0 as core::ffi::c_int as usize] = aom_read_primitive_refsubexpfin_(
            rb,
            (-((1 as core::ffi::c_int) << 7 as core::ffi::c_int) * 3 as core::ffi::c_int
                / 4 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                - 1 as core::ffi::c_int
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * 3 as core::ffi::c_int / 4 as core::ffi::c_int
                + 1 as core::ffi::c_int) as uint16_t,
            4 as uint16_t,
            ((*ref_sgrproj_info).xqd[0 as core::ffi::c_int as usize]
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * 3 as core::ffi::c_int / 4 as core::ffi::c_int) as uint16_t,
        ) as core::ffi::c_int + SGRPROJ_PRJ_MIN0;
        (*sgrproj_info).xqd[1 as core::ffi::c_int as usize] = clamp(
            ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS)
                - (*sgrproj_info).xqd[0 as core::ffi::c_int as usize],
            SGRPROJ_PRJ_MIN1,
            SGRPROJ_PRJ_MAX1,
        );
    } else {
        (*sgrproj_info).xqd[0 as core::ffi::c_int as usize] = aom_read_primitive_refsubexpfin_(
            rb,
            (-((1 as core::ffi::c_int) << 7 as core::ffi::c_int) * 3 as core::ffi::c_int
                / 4 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                - 1 as core::ffi::c_int
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * 3 as core::ffi::c_int / 4 as core::ffi::c_int
                + 1 as core::ffi::c_int) as uint16_t,
            4 as uint16_t,
            ((*ref_sgrproj_info).xqd[0 as core::ffi::c_int as usize]
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * 3 as core::ffi::c_int / 4 as core::ffi::c_int) as uint16_t,
        ) as core::ffi::c_int + SGRPROJ_PRJ_MIN0;
        (*sgrproj_info).xqd[1 as core::ffi::c_int as usize] = aom_read_primitive_refsubexpfin_(
            rb,
            (-((1 as core::ffi::c_int) << 7 as core::ffi::c_int) / 4 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                - 1 as core::ffi::c_int
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    / 4 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
            4 as uint16_t,
            ((*ref_sgrproj_info).xqd[1 as core::ffi::c_int as usize]
                - -((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    / 4 as core::ffi::c_int) as uint16_t,
        ) as core::ffi::c_int + SGRPROJ_PRJ_MIN1;
    }
    *ref_sgrproj_info = *sgrproj_info;
}
#[inline]
unsafe extern "C-unwind" fn loop_restoration_read_sb_coeffs(
    cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    r: *mut aom_reader,
    mut plane: core::ffi::c_int,
    mut runit_idx: core::ffi::c_int,
) {
    let mut rsi: *const RestorationInfo = &*((*cm).rst_info)
        .as_ptr()
        .offset(plane as isize) as *const RestorationInfo;
    let mut rui: *mut RestorationUnitInfo = &raw mut *((*rsi).unit_info)
        .offset(runit_idx as isize) as *mut RestorationUnitInfo;
    if !((*rsi).frame_restoration_type as core::ffi::c_uint
        != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"loop_restoration_read_sb_coeffs\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1708 as core::ffi::c_int,
            b"rsi->frame_restoration_type != RESTORE_NONE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*cm).features.all_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"loop_restoration_read_sb_coeffs\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1710 as core::ffi::c_int,
            b"!cm->features.all_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let wiener_win: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
        WIENER_WIN_CHROMA
    } else {
        WIENER_WIN
    };
    let mut wiener_info: *mut WienerInfo = ((*xd).wiener_info)
        .as_mut_ptr()
        .offset(plane as isize);
    let mut sgrproj_info: *mut SgrprojInfo = ((*xd).sgrproj_info)
        .as_mut_ptr()
        .offset(plane as isize);
    if (*rsi).frame_restoration_type as core::ffi::c_uint
        == RESTORE_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
    {
        (*rui).restoration_type = aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).switchable_restore_cdf).as_mut_ptr(),
            RESTORE_SWITCHABLE_TYPES as core::ffi::c_int,
        ) as RestorationType;
        match (*rui).restoration_type as core::ffi::c_uint {
            1 => {
                read_wiener_filter(wiener_win, &raw mut (*rui).wiener_info, wiener_info, r);
            }
            2 => {
                read_sgrproj_filter(&raw mut (*rui).sgrproj_info, sgrproj_info, r);
            }
            _ => {
                if !((*rui).restoration_type as core::ffi::c_uint
                    == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 32],
                            [core::ffi::c_char; 32],
                        >(*b"loop_restoration_read_sb_coeffs\0"))
                            .as_ptr(),
                        b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                        1727 as core::ffi::c_int,
                        b"rui->restoration_type == RESTORE_NONE\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
    } else if (*rsi).frame_restoration_type as core::ffi::c_uint
        == RESTORE_WIENER as core::ffi::c_int as core::ffi::c_uint
    {
        if aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).wiener_restore_cdf).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) != 0
        {
            (*rui).restoration_type = RESTORE_WIENER;
            read_wiener_filter(wiener_win, &raw mut (*rui).wiener_info, wiener_info, r);
        } else {
            (*rui).restoration_type = RESTORE_NONE;
        }
    } else if (*rsi).frame_restoration_type as core::ffi::c_uint
        == RESTORE_SGRPROJ as core::ffi::c_int as core::ffi::c_uint
    {
        if aom_read_symbol_(
            r,
            ((*(*xd).tile_ctx).sgrproj_restore_cdf).as_mut_ptr(),
            2 as core::ffi::c_int,
        ) != 0
        {
            (*rui).restoration_type = RESTORE_SGRPROJ;
            read_sgrproj_filter(&raw mut (*rui).sgrproj_info, sgrproj_info, r);
        } else {
            (*rui).restoration_type = RESTORE_NONE;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_loopfilter(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut lf: *mut loopfilter = &raw mut (*cm).lf;
    if (*cm).features.allow_intrabc as core::ffi::c_int != 0
        || (*cm).features.coded_lossless as core::ffi::c_int != 0
    {
        av1_set_default_ref_deltas(((*(*cm).cur_frame).ref_deltas).as_mut_ptr());
        av1_set_default_mode_deltas(((*(*cm).cur_frame).mode_deltas).as_mut_ptr());
        return;
    }
    if (*cm).features.coded_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"setup_loopfilter\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1757 as core::ffi::c_int,
            b"!cm->features.coded_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*cm).prev_frame).is_null() {
        memcpy(
            ((*lf).ref_deltas).as_mut_ptr() as *mut core::ffi::c_void,
            ((*(*cm).prev_frame).ref_deltas).as_mut_ptr() as *const core::ffi::c_void,
            REF_FRAMES as core::ffi::c_int as size_t,
        );
        memcpy(
            ((*lf).mode_deltas).as_mut_ptr() as *mut core::ffi::c_void,
            ((*(*cm).prev_frame).mode_deltas).as_mut_ptr() as *const core::ffi::c_void,
            MAX_MODE_LF_DELTAS as size_t,
        );
    } else {
        av1_set_default_ref_deltas(((*lf).ref_deltas).as_mut_ptr());
        av1_set_default_mode_deltas(((*lf).mode_deltas).as_mut_ptr());
    }
    (*lf).filter_level[0 as core::ffi::c_int as usize] = aom_rb_read_literal(
        rb,
        6 as core::ffi::c_int,
    );
    (*lf).filter_level[1 as core::ffi::c_int as usize] = aom_rb_read_literal(
        rb,
        6 as core::ffi::c_int,
    );
    if num_planes > 1 as core::ffi::c_int {
        if (*lf).filter_level[0 as core::ffi::c_int as usize] != 0
            || (*lf).filter_level[1 as core::ffi::c_int as usize] != 0
        {
            (*lf).filter_level_u = aom_rb_read_literal(rb, 6 as core::ffi::c_int);
            (*lf).filter_level_v = aom_rb_read_literal(rb, 6 as core::ffi::c_int);
        }
    }
    (*lf).sharpness_level = aom_rb_read_literal(rb, 3 as core::ffi::c_int);
    (*lf).mode_ref_delta_update = 0 as uint8_t;
    (*lf).mode_ref_delta_enabled = aom_rb_read_bit(rb) as uint8_t;
    if (*lf).mode_ref_delta_enabled != 0 {
        (*lf).mode_ref_delta_update = aom_rb_read_bit(rb) as uint8_t;
        if (*lf).mode_ref_delta_update != 0 {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < REF_FRAMES as core::ffi::c_int {
                if aom_rb_read_bit(rb) != 0 {
                    (*lf).ref_deltas[i as usize] = aom_rb_read_inv_signed_literal(
                        rb,
                        6 as core::ffi::c_int,
                    ) as int8_t;
                }
                i += 1;
            }
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < MAX_MODE_LF_DELTAS {
                if aom_rb_read_bit(rb) != 0 {
                    (*lf).mode_deltas[i_0 as usize] = aom_rb_read_inv_signed_literal(
                        rb,
                        6 as core::ffi::c_int,
                    ) as int8_t;
                }
                i_0 += 1;
            }
        }
    }
    memcpy(
        ((*(*cm).cur_frame).ref_deltas).as_mut_ptr() as *mut core::ffi::c_void,
        ((*lf).ref_deltas).as_mut_ptr() as *const core::ffi::c_void,
        REF_FRAMES as core::ffi::c_int as size_t,
    );
    memcpy(
        ((*(*cm).cur_frame).mode_deltas).as_mut_ptr() as *mut core::ffi::c_void,
        ((*lf).mode_deltas).as_mut_ptr() as *const core::ffi::c_void,
        MAX_MODE_LF_DELTAS as size_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn setup_cdef(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let cdef_info: *mut CdefInfo = &raw mut (*cm).cdef_info;
    if (*cm).features.allow_intrabc {
        return;
    }
    (*cdef_info).cdef_damping = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
        + 3 as core::ffi::c_int;
    (*cdef_info).cdef_bits = aom_rb_read_literal(rb, 2 as core::ffi::c_int);
    (*cdef_info).nb_cdef_strengths = (1 as core::ffi::c_int) << (*cdef_info).cdef_bits;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*cdef_info).nb_cdef_strengths {
        (*cdef_info).cdef_strengths[i as usize] = aom_rb_read_literal(
            rb,
            CDEF_STRENGTH_BITS,
        );
        (*cdef_info).cdef_uv_strengths[i as usize] = if num_planes
            > 1 as core::ffi::c_int
        {
            aom_rb_read_literal(rb, CDEF_STRENGTH_BITS)
        } else {
            0 as core::ffi::c_int
        };
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn read_delta_q(mut rb: *mut aom_read_bit_buffer) -> core::ffi::c_int {
    return if aom_rb_read_bit(rb) != 0 {
        aom_rb_read_inv_signed_literal(rb, 6 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn setup_quantization(
    mut quant_params: *mut CommonQuantParams,
    mut num_planes: core::ffi::c_int,
    mut separate_uv_delta_q: bool,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*quant_params).base_qindex = aom_rb_read_literal(rb, QINDEX_BITS);
    (*quant_params).y_dc_delta_q = read_delta_q(rb);
    if num_planes > 1 as core::ffi::c_int {
        let mut diff_uv_delta: core::ffi::c_int = 0 as core::ffi::c_int;
        if separate_uv_delta_q {
            diff_uv_delta = aom_rb_read_bit(rb);
        }
        (*quant_params).u_dc_delta_q = read_delta_q(rb);
        (*quant_params).u_ac_delta_q = read_delta_q(rb);
        if diff_uv_delta != 0 {
            (*quant_params).v_dc_delta_q = read_delta_q(rb);
            (*quant_params).v_ac_delta_q = read_delta_q(rb);
        } else {
            (*quant_params).v_dc_delta_q = (*quant_params).u_dc_delta_q;
            (*quant_params).v_ac_delta_q = (*quant_params).u_ac_delta_q;
        }
    } else {
        (*quant_params).u_dc_delta_q = 0 as core::ffi::c_int;
        (*quant_params).u_ac_delta_q = 0 as core::ffi::c_int;
        (*quant_params).v_dc_delta_q = 0 as core::ffi::c_int;
        (*quant_params).v_ac_delta_q = 0 as core::ffi::c_int;
    }
    (*quant_params).using_qmatrix = aom_rb_read_bit(rb) != 0;
    if (*quant_params).using_qmatrix {
        (*quant_params).qmatrix_level_y = aom_rb_read_literal(rb, QM_LEVEL_BITS);
        (*quant_params).qmatrix_level_u = aom_rb_read_literal(rb, QM_LEVEL_BITS);
        if !separate_uv_delta_q {
            (*quant_params).qmatrix_level_v = (*quant_params).qmatrix_level_u;
        } else {
            (*quant_params).qmatrix_level_v = aom_rb_read_literal(rb, QM_LEVEL_BITS);
        }
    } else {
        (*quant_params).qmatrix_level_y = 0 as core::ffi::c_int;
        (*quant_params).qmatrix_level_u = 0 as core::ffi::c_int;
        (*quant_params).qmatrix_level_v = 0 as core::ffi::c_int;
    };
}
unsafe extern "C-unwind" fn get_iqmatrix(
    mut quant_params: *const CommonQuantParams,
    mut qmlevel: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> *const qm_val_t {
    if !(!((*quant_params).giqmatrix[qmlevel as usize][plane as usize][tx_size as usize])
        .is_null()
        || qmlevel
            == ((1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"get_iqmatrix\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            1860 as core::ffi::c_int,
            b"quant_params->giqmatrix[qmlevel][plane][tx_size] != NULL || qmlevel == NUM_QM_LEVELS - 1\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return (*quant_params).giqmatrix[qmlevel as usize][plane as usize][tx_size as usize];
}
#[inline]
unsafe extern "C-unwind" fn setup_segmentation_dequant(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
) {
    let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth as core::ffi::c_int;
    let max_segments: core::ffi::c_int = if (*cm).seg.enabled as core::ffi::c_int != 0 {
        MAX_SEGMENTS
    } else {
        1 as core::ffi::c_int
    };
    let quant_params: *mut CommonQuantParams = &raw mut (*cm).quant_params;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < max_segments {
        let qindex: core::ffi::c_int = (*xd).qindex[i as usize];
        (*quant_params).y_dequant_QTX[i as usize][0 as core::ffi::c_int as usize] = av1_dc_quant_QTX(
            qindex,
            (*quant_params).y_dc_delta_q,
            bit_depth as aom_bit_depth_t,
        );
        (*quant_params).y_dequant_QTX[i as usize][1 as core::ffi::c_int as usize] = av1_ac_quant_QTX(
            qindex,
            0 as core::ffi::c_int,
            bit_depth as aom_bit_depth_t,
        );
        (*quant_params).u_dequant_QTX[i as usize][0 as core::ffi::c_int as usize] = av1_dc_quant_QTX(
            qindex,
            (*quant_params).u_dc_delta_q,
            bit_depth as aom_bit_depth_t,
        );
        (*quant_params).u_dequant_QTX[i as usize][1 as core::ffi::c_int as usize] = av1_ac_quant_QTX(
            qindex,
            (*quant_params).u_ac_delta_q,
            bit_depth as aom_bit_depth_t,
        );
        (*quant_params).v_dequant_QTX[i as usize][0 as core::ffi::c_int as usize] = av1_dc_quant_QTX(
            qindex,
            (*quant_params).v_dc_delta_q,
            bit_depth as aom_bit_depth_t,
        );
        (*quant_params).v_dequant_QTX[i as usize][1 as core::ffi::c_int as usize] = av1_ac_quant_QTX(
            qindex,
            (*quant_params).v_ac_delta_q,
            bit_depth as aom_bit_depth_t,
        );
        let use_qmatrix: core::ffi::c_int = av1_use_qmatrix(quant_params, xd, i)
            as core::ffi::c_int;
        let qmlevel_y: core::ffi::c_int = if use_qmatrix != 0 {
            (*quant_params).qmatrix_level_y
        } else {
            NUM_QM_LEVELS - 1 as core::ffi::c_int
        };
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < TX_SIZES_ALL as core::ffi::c_int {
            (*quant_params).y_iqmatrix[i as usize][j as usize] = get_iqmatrix(
                quant_params,
                qmlevel_y,
                AOM_PLANE_Y,
                j as TX_SIZE,
            );
            j += 1;
        }
        let qmlevel_u: core::ffi::c_int = if use_qmatrix != 0 {
            (*quant_params).qmatrix_level_u
        } else {
            NUM_QM_LEVELS - 1 as core::ffi::c_int
        };
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < TX_SIZES_ALL as core::ffi::c_int {
            (*quant_params).u_iqmatrix[i as usize][j_0 as usize] = get_iqmatrix(
                quant_params,
                qmlevel_u,
                AOM_PLANE_U,
                j_0 as TX_SIZE,
            );
            j_0 += 1;
        }
        let qmlevel_v: core::ffi::c_int = if use_qmatrix != 0 {
            (*quant_params).qmatrix_level_v
        } else {
            NUM_QM_LEVELS - 1 as core::ffi::c_int
        };
        let mut j_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_1 < TX_SIZES_ALL as core::ffi::c_int {
            (*quant_params).v_iqmatrix[i as usize][j_1 as usize] = get_iqmatrix(
                quant_params,
                qmlevel_v,
                AOM_PLANE_V,
                j_1 as TX_SIZE,
            );
            j_1 += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn read_frame_interp_filter(
    mut rb: *mut aom_read_bit_buffer,
) -> InterpFilter {
    return (if aom_rb_read_bit(rb) != 0 {
        SWITCHABLE as core::ffi::c_int
    } else {
        aom_rb_read_literal(rb, LOG_SWITCHABLE_FILTERS)
    }) as InterpFilter;
}
unsafe extern "C-unwind" fn read_frame_size(
    mut rb: *mut aom_read_bit_buffer,
    mut num_bits_width: core::ffi::c_int,
    mut num_bits_height: core::ffi::c_int,
    mut width: *mut core::ffi::c_int,
    mut height: *mut core::ffi::c_int,
) {
    *width = aom_rb_read_literal(rb, num_bits_width) + 1 as core::ffi::c_int;
    *height = aom_rb_read_literal(rb, num_bits_height) + 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn setup_render_size(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*cm).render_width = (*cm).superres_upscaled_width;
    (*cm).render_height = (*cm).superres_upscaled_height;
    if aom_rb_read_bit(rb) != 0 {
        read_frame_size(
            rb,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
            &raw mut (*cm).render_width,
            &raw mut (*cm).render_height,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_superres(
    cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
    mut width: *mut core::ffi::c_int,
    mut height: *mut core::ffi::c_int,
) {
    (*cm).superres_upscaled_width = *width;
    (*cm).superres_upscaled_height = *height;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    if (*seq_params).enable_superres == 0 {
        return;
    }
    if aom_rb_read_bit(rb) != 0 {
        (*cm).superres_scale_denominator = aom_rb_read_literal(rb, SUPERRES_SCALE_BITS)
            as uint8_t;
        (*cm).superres_scale_denominator = ((*cm).superres_scale_denominator
            as core::ffi::c_int + SUPERRES_SCALE_DENOMINATOR_MIN) as uint8_t;
        av1_calculate_scaled_superres_size(
            width,
            height,
            (*cm).superres_scale_denominator as core::ffi::c_int,
        );
    } else {
        (*cm).superres_scale_denominator = SCALE_NUMERATOR as uint8_t;
    };
}
#[inline]
unsafe extern "C-unwind" fn resize_context_buffers(
    mut cm: *mut AV1_COMMON,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut frame_size_limit: core::ffi::c_uint,
) {
    if frame_size_limit != 0
        && (width as uint64_t).wrapping_mul(height as uint64_t)
            > frame_size_limit as uint64_t
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_CORRUPT_FRAME,
            b"Dimensions of %dx%d beyond allowed size of %u.\0" as *const u8
                as *const core::ffi::c_char,
            width,
            height,
            frame_size_limit,
        );
    }
    if (*cm).width != width || (*cm).height != height {
        let new_mi_rows: core::ffi::c_int = height
            + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
            >> 2 as core::ffi::c_int;
        let new_mi_cols: core::ffi::c_int = width
            + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
            >> 2 as core::ffi::c_int;
        if new_mi_cols > (*cm).mi_params.mi_cols || new_mi_rows > (*cm).mi_params.mi_rows
        {
            if av1_alloc_context_buffers(cm as *mut AV1Common, width, height, BLOCK_4X4)
                != 0
            {
                (*cm).width = 0 as core::ffi::c_int;
                (*cm).height = 0 as core::ffi::c_int;
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate context buffers\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else {
            ((*cm).mi_params.set_mb_mi)
                .expect(
                    "non-null function pointer",
                )(&raw mut (*cm).mi_params, width, height, BLOCK_4X4);
        }
        av1_init_mi_buffers(&raw mut (*cm).mi_params);
        (*cm).width = width;
        (*cm).height = height;
    }
    ensure_mv_buffer((*cm).cur_frame, cm);
    (*(*cm).cur_frame).width = (*cm).width;
    (*(*cm).cur_frame).height = (*cm).height;
}
#[inline]
unsafe extern "C-unwind" fn setup_buffer_pool(mut cm: *mut AV1_COMMON) {
    let pool: *mut BufferPool = (*cm).buffer_pool;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    lock_buffer_pool(pool);
    if aom_realloc_frame_buffer(
        &raw mut (*(*cm).cur_frame).buf,
        (*cm).width,
        (*cm).height,
        (*seq_params).subsampling_x,
        (*seq_params).subsampling_y,
        (*seq_params).use_highbitdepth as core::ffi::c_int,
        AOM_DEC_BORDER_IN_PIXELS,
        (*cm).features.byte_alignment,
        &raw mut (*(*cm).cur_frame).raw_frame_buffer,
        (*pool).get_fb_cb,
        (*pool).cb_priv,
        false_0 != 0,
        0 as core::ffi::c_int,
    ) != 0
    {
        unlock_buffer_pool(pool);
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate frame buffer\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    unlock_buffer_pool(pool);
    (*(*cm).cur_frame).buf.bit_depth = (*seq_params).bit_depth as core::ffi::c_uint;
    (*(*cm).cur_frame).buf.color_primaries = (*seq_params).color_primaries;
    (*(*cm).cur_frame).buf.transfer_characteristics = (*seq_params)
        .transfer_characteristics;
    (*(*cm).cur_frame).buf.matrix_coefficients = (*seq_params).matrix_coefficients;
    (*(*cm).cur_frame).buf.monochrome = (*seq_params).monochrome;
    (*(*cm).cur_frame).buf.chroma_sample_position = (*seq_params).chroma_sample_position;
    (*(*cm).cur_frame).buf.color_range = (*seq_params).color_range as aom_color_range_t;
    (*(*cm).cur_frame).buf.render_width = (*cm).render_width;
    (*(*cm).cur_frame).buf.render_height = (*cm).render_height;
}
#[inline]
unsafe extern "C-unwind" fn setup_frame_size(
    mut cm: *mut AV1_COMMON,
    mut frame_size_override_flag: core::ffi::c_int,
    mut frame_size_limit: core::ffi::c_uint,
    mut rb: *mut aom_read_bit_buffer,
) {
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let mut width: core::ffi::c_int = 0;
    let mut height: core::ffi::c_int = 0;
    if frame_size_override_flag != 0 {
        let mut num_bits_width: core::ffi::c_int = (*seq_params).num_bits_width;
        let mut num_bits_height: core::ffi::c_int = (*seq_params).num_bits_height;
        read_frame_size(rb, num_bits_width, num_bits_height, &raw mut width, &raw mut height);
        if width > (*seq_params).max_frame_width
            || height > (*seq_params).max_frame_height
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_CORRUPT_FRAME,
                b"Frame dimensions are larger than the maximum values\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        width = (*seq_params).max_frame_width;
        height = (*seq_params).max_frame_height;
    }
    setup_superres(cm, rb, &raw mut width, &raw mut height);
    resize_context_buffers(cm, width, height, frame_size_limit);
    setup_render_size(cm, rb);
    setup_buffer_pool(cm);
}
#[inline]
unsafe extern "C-unwind" fn setup_sb_size(
    mut seq_params: *mut SequenceHeader,
    mut rb: *mut aom_read_bit_buffer,
) {
    set_sb_size(
        seq_params,
        (if aom_rb_read_bit(rb) != 0 {
            BLOCK_128X128 as core::ffi::c_int
        } else {
            BLOCK_64X64 as core::ffi::c_int
        }) as BLOCK_SIZE,
    );
}
#[inline]
unsafe extern "C-unwind" fn valid_ref_frame_img_fmt(
    mut ref_bit_depth: aom_bit_depth_t,
    mut ref_xss: core::ffi::c_int,
    mut ref_yss: core::ffi::c_int,
    mut this_bit_depth: aom_bit_depth_t,
    mut this_xss: core::ffi::c_int,
    mut this_yss: core::ffi::c_int,
) -> core::ffi::c_int {
    return (ref_bit_depth as core::ffi::c_uint == this_bit_depth as core::ffi::c_uint
        && ref_xss == this_xss && ref_yss == this_yss) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn setup_frame_size_with_refs(
    mut cm: *mut AV1_COMMON,
    mut frame_size_limit: core::ffi::c_uint,
    mut rb: *mut aom_read_bit_buffer,
) {
    let mut width: core::ffi::c_int = 0;
    let mut height: core::ffi::c_int = 0;
    let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut has_valid_ref_frame: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while i <= ALTREF_FRAME as core::ffi::c_int {
        if aom_rb_read_bit(rb) != 0 {
            let ref_buf: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                i as MV_REFERENCE_FRAME,
            );
            if ref_buf.is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_CORRUPT_FRAME,
                    b"Invalid condition: invalid reference buffer\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {
                let buf: *const YV12_BUFFER_CONFIG = &(*ref_buf).buf;
                width = (*buf).c2rust_unnamed_1.c2rust_unnamed.y_crop_width;
                height = (*buf).c2rust_unnamed_2.c2rust_unnamed.y_crop_height;
                (*cm).render_width = (*buf).render_width;
                (*cm).render_height = (*buf).render_height;
                setup_superres(cm, rb, &raw mut width, &raw mut height);
                resize_context_buffers(cm, width, height, frame_size_limit);
                found = 1 as core::ffi::c_int;
                break;
            }
        }
        i += 1;
    }
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    if found == 0 {
        let mut num_bits_width: core::ffi::c_int = (*seq_params).num_bits_width;
        let mut num_bits_height: core::ffi::c_int = (*seq_params).num_bits_height;
        read_frame_size(rb, num_bits_width, num_bits_height, &raw mut width, &raw mut height);
        setup_superres(cm, rb, &raw mut width, &raw mut height);
        resize_context_buffers(cm, width, height, frame_size_limit);
        setup_render_size(cm, rb);
    }
    if width <= 0 as core::ffi::c_int || height <= 0 as core::ffi::c_int {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_CORRUPT_FRAME,
            b"Invalid frame size\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    let mut i_0: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while i_0 <= ALTREF_FRAME as core::ffi::c_int {
        let ref_frame: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            i_0 as MV_REFERENCE_FRAME,
        );
        has_valid_ref_frame
            |= valid_ref_frame_size(
                (*ref_frame).buf.c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
                (*ref_frame).buf.c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
                width,
                height,
            );
        i_0 += 1;
    }
    if has_valid_ref_frame == 0 {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_CORRUPT_FRAME,
            b"Referenced frame has invalid size\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    let mut i_1: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while i_1 <= ALTREF_FRAME as core::ffi::c_int {
        let ref_frame_0: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            i_1 as MV_REFERENCE_FRAME,
        );
        if valid_ref_frame_img_fmt(
            (*ref_frame_0).buf.bit_depth as aom_bit_depth_t,
            (*ref_frame_0).buf.subsampling_x,
            (*ref_frame_0).buf.subsampling_y,
            (*seq_params).bit_depth,
            (*seq_params).subsampling_x,
            (*seq_params).subsampling_y,
        ) == 0
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_CORRUPT_FRAME,
                b"Referenced frame has incompatible color format\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        i_1 += 1;
    }
    setup_buffer_pool(cm);
}
unsafe extern "C-unwind" fn rb_read_uniform(
    rb: *mut aom_read_bit_buffer,
    mut n: core::ffi::c_int,
) -> core::ffi::c_int {
    let l: core::ffi::c_int = get_unsigned_bits(n as core::ffi::c_uint)
        as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n;
    let v: core::ffi::c_int = aom_rb_read_literal(rb, l - 1 as core::ffi::c_int)
        as core::ffi::c_int;
    if !(l != 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"rb_read_uniform\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2139 as core::ffi::c_int,
            b"l != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if v < m {
        return v
    } else {
        return (v << 1 as core::ffi::c_int) - m + aom_rb_read_bit(rb)
    };
}
#[inline]
unsafe extern "C-unwind" fn read_tile_info_max_tile(
    cm: *mut AV1_COMMON,
    rb: *mut aom_read_bit_buffer,
) {
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let mut width_sb: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << (*seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*seq_params).mib_size_log2;
    let mut height_sb: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << (*seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*seq_params).mib_size_log2;
    av1_get_tile_limits(cm as *mut AV1Common);
    (*tiles).uniform_spacing = aom_rb_read_bit(rb);
    if (*tiles).uniform_spacing != 0 {
        (*tiles).log2_cols = (*tiles).min_log2_cols;
        while (*tiles).log2_cols < (*tiles).max_log2_cols {
            if aom_rb_read_bit(rb) == 0 {
                break;
            }
            (*tiles).log2_cols += 1;
        }
    } else {
        let mut i: core::ffi::c_int = 0;
        let mut start_sb: core::ffi::c_int = 0;
        i = 0 as core::ffi::c_int;
        start_sb = 0 as core::ffi::c_int;
        while width_sb > 0 as core::ffi::c_int && i < MAX_TILE_COLS {
            let size_sb: core::ffi::c_int = 1 as core::ffi::c_int
                + rb_read_uniform(
                    rb,
                    (if width_sb < (*tiles).max_width_sb {
                        width_sb
                    } else {
                        (*tiles).max_width_sb
                    }),
                ) as core::ffi::c_int;
            (*tiles).col_start_sb[i as usize] = start_sb;
            start_sb += size_sb;
            width_sb -= size_sb;
            i += 1;
        }
        (*tiles).cols = i;
        (*tiles).col_start_sb[i as usize] = start_sb + width_sb;
    }
    av1_calculate_tile_cols(
        seq_params as *const SequenceHeader,
        (*cm).mi_params.mi_rows,
        (*cm).mi_params.mi_cols,
        tiles as *mut CommonTileParams,
    );
    if (*tiles).uniform_spacing != 0 {
        (*tiles).log2_rows = (*tiles).min_log2_rows;
        while (*tiles).log2_rows < (*tiles).max_log2_rows {
            if aom_rb_read_bit(rb) == 0 {
                break;
            }
            (*tiles).log2_rows += 1;
        }
    } else {
        let mut i_0: core::ffi::c_int = 0;
        let mut start_sb_0: core::ffi::c_int = 0;
        i_0 = 0 as core::ffi::c_int;
        start_sb_0 = 0 as core::ffi::c_int;
        while height_sb > 0 as core::ffi::c_int && i_0 < MAX_TILE_ROWS {
            let size_sb_0: core::ffi::c_int = 1 as core::ffi::c_int
                + rb_read_uniform(
                    rb,
                    (if height_sb < (*tiles).max_height_sb {
                        height_sb
                    } else {
                        (*tiles).max_height_sb
                    }),
                ) as core::ffi::c_int;
            (*tiles).row_start_sb[i_0 as usize] = start_sb_0;
            start_sb_0 += size_sb_0;
            height_sb -= size_sb_0;
            i_0 += 1;
        }
        (*tiles).rows = i_0;
        (*tiles).row_start_sb[i_0 as usize] = start_sb_0 + height_sb;
    }
    av1_calculate_tile_rows(
        seq_params as *const SequenceHeader,
        (*cm).mi_params.mi_rows,
        tiles as *mut CommonTileParams,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_single_tile_decoding_mode(cm: *mut AV1_COMMON) {
    (*cm).tiles.single_tile_decoding = 0 as core::ffi::c_uint;
    if (*cm).tiles.large_scale != 0 {
        let mut lf: *mut loopfilter = &raw mut (*cm).lf;
        let rst_info: *mut RestorationInfo = ((*cm).rst_info).as_mut_ptr();
        let cdef_info: *const CdefInfo = &raw mut (*cm).cdef_info;
        let no_loopfilter: core::ffi::c_int = !((*lf)
            .filter_level[0 as core::ffi::c_int as usize] != 0
            || (*lf).filter_level[1 as core::ffi::c_int as usize] != 0)
            as core::ffi::c_int;
        let no_cdef: core::ffi::c_int = ((*cdef_info).cdef_bits == 0 as core::ffi::c_int
            && (*cdef_info).cdef_strengths[0 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
            && (*cdef_info).cdef_uv_strengths[0 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int) as core::ffi::c_int;
        let no_restoration: core::ffi::c_int = ((*rst_info
            .offset(0 as core::ffi::c_int as isize))
            .frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
            && (*rst_info.offset(1 as core::ffi::c_int as isize)).frame_restoration_type
                as core::ffi::c_uint
                == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
            && (*rst_info.offset(2 as core::ffi::c_int as isize)).frame_restoration_type
                as core::ffi::c_uint
                == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        if !(!(*cm).features.coded_lossless || no_loopfilter != 0 && no_cdef != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 34],
                    [core::ffi::c_char; 34],
                >(*b"av1_set_single_tile_decoding_mode\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                2224 as core::ffi::c_int,
                b"IMPLIES(cm->features.coded_lossless, no_loopfilter && no_cdef)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(!(*cm).features.all_lossless || no_restoration != 0) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 34],
                    [core::ffi::c_char; 34],
                >(*b"av1_set_single_tile_decoding_mode\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                2225 as core::ffi::c_int,
                b"IMPLIES(cm->features.all_lossless, no_restoration)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        (*cm).tiles.single_tile_decoding = (no_loopfilter != 0 && no_cdef != 0
            && no_restoration != 0) as core::ffi::c_int as core::ffi::c_uint;
    }
}
#[inline]
unsafe extern "C-unwind" fn read_tile_info(pbi: *mut AV1Decoder, rb: *mut aom_read_bit_buffer) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    read_tile_info_max_tile(cm, rb);
    (*pbi).context_update_tile_id = 0 as core::ffi::c_int;
    if (*cm).tiles.rows * (*cm).tiles.cols > 1 as core::ffi::c_int {
        (*pbi).context_update_tile_id = aom_rb_read_literal(
            rb,
            (*cm).tiles.log2_rows + (*cm).tiles.log2_cols,
        );
        if (*pbi).context_update_tile_id >= (*cm).tiles.rows * (*cm).tiles.cols {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid context_update_tile_id\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*pbi).tile_size_bytes = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
            + 1 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn read_ext_tile_info(
    pbi: *mut AV1Decoder,
    rb: *mut aom_read_bit_buffer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut mod_0: core::ffi::c_int = ((*rb).bit_offset)
        .wrapping_rem(CHAR_BIT as uint32_t) as core::ffi::c_int;
    if mod_0 > 0 as core::ffi::c_int {
        aom_rb_read_literal(rb, CHAR_BIT - mod_0);
    }
    if !(((*rb).bit_offset).wrapping_rem(8 as uint32_t) == 0 as uint32_t)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"read_ext_tile_info\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2258 as core::ffi::c_int,
            b"rb->bit_offset % CHAR_BIT == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*cm).tiles.cols * (*cm).tiles.rows > 1 as core::ffi::c_int {
        (*pbi).tile_col_size_bytes = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
            + 1 as core::ffi::c_int;
        (*pbi).tile_size_bytes = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
            + 1 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn mem_get_varsize(
    mut src: *const uint8_t,
    mut sz: core::ffi::c_int,
) -> size_t {
    match sz {
        1 => return *src.offset(0 as core::ffi::c_int as isize) as size_t,
        2 => return mem_get_le16_as_int(src as *const core::ffi::c_void) as size_t,
        3 => return mem_get_le24_as_int(src as *const core::ffi::c_void) as size_t,
        4 => return mem_get_le32_as_int(src as *const core::ffi::c_void) as size_t,
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid size\0" as *const u8 as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 16],
                        [core::ffi::c_char; 16],
                    >(*b"mem_get_varsize\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    2274 as core::ffi::c_int,
                    b"0 && \"Invalid size\"\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            return -(1 as core::ffi::c_int) as size_t;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn get_ls_tile_buffer(
    data_end: *const uint8_t,
    mut error_info: *mut aom_internal_error_info,
    mut data: *mut *const uint8_t,
    tile_buffers: *mut [TileBufferDec; 64],
    mut tile_size_bytes: core::ffi::c_int,
    mut col: core::ffi::c_int,
    mut row: core::ffi::c_int,
    mut tile_copy_mode: core::ffi::c_int,
) {
    let mut size: size_t = 0;
    let mut copy_size: size_t = 0 as size_t;
    let mut copy_data: *const uint8_t = 0 as *const uint8_t;
    if read_is_valid(*data, tile_size_bytes as size_t, data_end) == 0 {
        aom_internal_error(
            error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt tile length\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    size = mem_get_varsize(*data, tile_size_bytes);
    if tile_copy_mode != 0
        && size >> tile_size_bytes * 8 as core::ffi::c_int - 1 as core::ffi::c_int
            == 1 as size_t
    {
        let mut offset: core::ffi::c_int = (size
            >> (tile_size_bytes - 1 as core::ffi::c_int) * 8 as core::ffi::c_int
            & 0x7f as size_t) as core::ffi::c_int;
        if offset > row {
            aom_internal_error(
                error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid row offset in tile copy mode: row=%d offset=%d\0" as *const u8
                    as *const core::ffi::c_char,
                row,
                offset,
            );
        }
        copy_data = (*tile_buffers.offset((row - offset) as isize))[col as usize].data;
        copy_size = (*tile_buffers.offset((row - offset) as isize))[col as usize].size;
        size = 0 as size_t;
    } else {
        size = size.wrapping_add(AV1_MIN_TILE_SIZE_BYTES as size_t);
    }
    *data = (*data).offset(tile_size_bytes as isize);
    if size > data_end.offset_from(*data) as core::ffi::c_long as size_t {
        aom_internal_error(
            error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Truncated packet or corrupt tile size\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if size > 0 as size_t {
        let fresh23 = &raw mut (*tile_buffers.offset(row as isize))[col as usize].data;
        *fresh23 = *data;
        (*tile_buffers.offset(row as isize))[col as usize].size = size;
    } else {
        let fresh24 = &raw mut (*tile_buffers.offset(row as isize))[col as usize].data;
        *fresh24 = copy_data;
        (*tile_buffers.offset(row as isize))[col as usize].size = copy_size;
    }
    *data = (*data).offset(size as isize);
}
unsafe extern "C-unwind" fn get_ls_tile_buffers(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    tile_buffers: *mut [TileBufferDec; 64],
) -> *const uint8_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let have_tiles: core::ffi::c_int = (tile_cols * tile_rows > 1 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut raw_data_end: *const uint8_t = 0 as *const uint8_t;
    if have_tiles == 0 {
        let tile_size: size_t = data_end.offset_from(data) as core::ffi::c_long
            as size_t;
        let fresh22 = &raw mut (*tile_buffers
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            .data;
        *fresh22 = data;
        (*tile_buffers
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            .size = tile_size;
        raw_data_end = 0 as *const uint8_t;
    } else {
        let mut tile_col_data_end: [*const uint8_t; 64] = [
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
            0 as *const uint8_t,
        ];
        let data_start: *const uint8_t = data;
        let dec_tile_row: core::ffi::c_int = if (*pbi).dec_tile_row < tile_rows {
            (*pbi).dec_tile_row
        } else {
            tile_rows
        };
        let single_row: core::ffi::c_int = ((*pbi).dec_tile_row >= 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let tile_rows_start: core::ffi::c_int = if single_row != 0 {
            dec_tile_row
        } else {
            0 as core::ffi::c_int
        };
        let tile_rows_end: core::ffi::c_int = if single_row != 0 {
            tile_rows_start + 1 as core::ffi::c_int
        } else {
            tile_rows
        };
        let dec_tile_col: core::ffi::c_int = if (*pbi).dec_tile_col < tile_cols {
            (*pbi).dec_tile_col
        } else {
            tile_cols
        };
        let single_col: core::ffi::c_int = ((*pbi).dec_tile_col >= 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let tile_cols_start: core::ffi::c_int = if single_col != 0 {
            dec_tile_col
        } else {
            0 as core::ffi::c_int
        };
        let tile_cols_end: core::ffi::c_int = if single_col != 0 {
            tile_cols_start + 1 as core::ffi::c_int
        } else {
            tile_cols
        };
        let tile_col_size_bytes: core::ffi::c_int = (*pbi).tile_col_size_bytes;
        let tile_size_bytes: core::ffi::c_int = (*pbi).tile_size_bytes;
        let mut tile_width: core::ffi::c_int = 0;
        let mut tile_height: core::ffi::c_int = 0;
        if !av1_get_uniform_tile_size(cm, &raw mut tile_width, &raw mut tile_height) {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Not all the tiles in the tile list have the same size.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let tile_copy_mode: core::ffi::c_int = if (if tile_width > tile_height {
            tile_width
        } else {
            tile_height
        }) << MI_SIZE_LOG2 <= 256 as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < tile_cols {
            let is_last: core::ffi::c_int = (c == tile_cols - 1 as core::ffi::c_int)
                as core::ffi::c_int;
            let mut tile_col_size: size_t = 0;
            if is_last == 0 {
                if tile_col_size_bytes as core::ffi::c_long
                    > data_end.offset_from(data) as core::ffi::c_long
                {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Not enough data to read tile_col_size\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                tile_col_size = mem_get_varsize(data, tile_col_size_bytes);
                data = data.offset(tile_col_size_bytes as isize);
                if tile_col_size
                    > data_end.offset_from(data) as core::ffi::c_long as size_t
                {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"tile_col_data_end[%d] is out of bound\0" as *const u8
                            as *const core::ffi::c_char,
                        c,
                    );
                }
                tile_col_data_end[c as usize] = data.offset(tile_col_size as isize);
            } else {
                tile_col_size = data_end.offset_from(data) as core::ffi::c_long
                    as size_t;
                tile_col_data_end[c as usize] = data_end;
            }
            data = data.offset(tile_col_size as isize);
            c += 1;
        }
        data = data_start;
        let mut c_0: core::ffi::c_int = tile_cols_start;
        while c_0 < tile_cols_end {
            let is_last_0: core::ffi::c_int = (c_0 == tile_cols - 1 as core::ffi::c_int)
                as core::ffi::c_int;
            if c_0 > 0 as core::ffi::c_int {
                data = tile_col_data_end[(c_0 - 1 as core::ffi::c_int) as usize];
            }
            if is_last_0 == 0 {
                data = data.offset(tile_col_size_bytes as isize);
            }
            let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
            while r < (if is_last_0 != 0 { tile_rows } else { tile_rows_end }) {
                get_ls_tile_buffer(
                    tile_col_data_end[c_0 as usize],
                    &raw mut (*pbi).error,
                    &raw mut data,
                    tile_buffers,
                    tile_size_bytes,
                    c_0,
                    r,
                    tile_copy_mode,
                );
                r += 1;
            }
            c_0 += 1;
        }
        if tile_cols_end != tile_cols {
            let c_1: core::ffi::c_int = tile_cols - 1 as core::ffi::c_int;
            data = tile_col_data_end[(c_1 - 1 as core::ffi::c_int) as usize];
            let mut r_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while r_0 < tile_rows {
                get_ls_tile_buffer(
                    tile_col_data_end[c_1 as usize],
                    &raw mut (*pbi).error,
                    &raw mut data,
                    tile_buffers,
                    tile_size_bytes,
                    c_1,
                    r_0,
                    tile_copy_mode,
                );
                r_0 += 1;
            }
        }
        raw_data_end = data;
    }
    return raw_data_end;
}
unsafe extern "C-unwind" fn get_ls_single_tile_buffer(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    tile_buffers: *mut [TileBufferDec; 64],
) -> *const uint8_t {
    if !((*pbi).dec_tile_row >= 0 as core::ffi::c_int
        && (*pbi).dec_tile_col >= 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"get_ls_single_tile_buffer\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2438 as core::ffi::c_int,
            b"pbi->dec_tile_row >= 0 && pbi->dec_tile_col >= 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let fresh25 = &raw mut (*tile_buffers
        .offset((*pbi).dec_tile_row as isize))[(*pbi).dec_tile_col as usize]
        .data;
    *fresh25 = data;
    (*tile_buffers.offset((*pbi).dec_tile_row as isize))[(*pbi).dec_tile_col as usize]
        .size = (*pbi).coded_tile_data_size as size_t;
    return data.offset((*pbi).coded_tile_data_size as isize);
}
#[inline]
unsafe extern "C-unwind" fn get_tile_buffer(
    data_end: *const uint8_t,
    tile_size_bytes: core::ffi::c_int,
    mut is_last: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
    mut data: *mut *const uint8_t,
    buf: *mut TileBufferDec,
) {
    let mut size: size_t = 0;
    if is_last == 0 {
        if read_is_valid(*data, tile_size_bytes as size_t, data_end) == 0 {
            aom_internal_error(
                error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Not enough data to read tile size\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        size = (mem_get_varsize(*data, tile_size_bytes))
            .wrapping_add(AV1_MIN_TILE_SIZE_BYTES as size_t);
        *data = (*data).offset(tile_size_bytes as isize);
        if size > data_end.offset_from(*data) as core::ffi::c_long as size_t {
            aom_internal_error(
                error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Truncated packet or corrupt tile size\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        size = data_end.offset_from(*data) as core::ffi::c_long as size_t;
    }
    (*buf).data = *data;
    (*buf).size = size;
    *data = (*data).offset(size as isize);
}
#[inline]
unsafe extern "C-unwind" fn get_tile_buffers(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    tile_buffers: *mut [TileBufferDec; 64],
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tile_cols: core::ffi::c_int = (*cm).tiles.cols;
    let tile_rows: core::ffi::c_int = (*cm).tiles.rows;
    let mut tc: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < tile_rows {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < tile_cols {
            let buf: *mut TileBufferDec = &raw mut *(*tile_buffers.offset(r as isize))
                .as_mut_ptr()
                .offset(c as isize) as *mut TileBufferDec;
            let is_last: core::ffi::c_int = (tc == end_tile) as core::ffi::c_int;
            let hdr_offset: size_t = 0 as size_t;
            if !(tc < start_tile || tc > end_tile) {
                if data.offset(hdr_offset as isize) >= data_end {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Data ended before all tiles were read.\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                data = data.offset(hdr_offset as isize);
                get_tile_buffer(
                    data_end,
                    (*pbi).tile_size_bytes,
                    is_last,
                    &raw mut (*pbi).error,
                    &raw mut data,
                    buf,
                );
            }
            c += 1;
            tc += 1;
        }
        r += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_cb_buffer(
    mut pbi: *mut AV1Decoder,
    mut dcb: *mut DecoderCodingBlock,
    mut cb_buffer_base: *mut CB_BUFFER,
    num_planes: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut mib_size_log2: core::ffi::c_int = (*(*cm).seq_params).mib_size_log2;
    let mut stride: core::ffi::c_int = ((*cm).mi_params.mi_cols >> mib_size_log2)
        + 1 as core::ffi::c_int;
    let mut offset: core::ffi::c_int = (mi_row >> mib_size_log2) * stride
        + (mi_col >> mib_size_log2);
    let mut cb_buffer: *mut CB_BUFFER = cb_buffer_base.offset(offset as isize);
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        (*dcb).dqcoeff_block[plane as usize] = ((*cb_buffer).dqcoeff[plane as usize])
            .as_mut_ptr();
        (*dcb).eob_data[plane as usize] = ((*cb_buffer).eob_data[plane as usize])
            .as_mut_ptr();
        (*dcb).cb_offset[plane as usize] = 0 as uint16_t;
        (*dcb).txb_offset[plane as usize] = 0 as uint16_t;
        plane += 1;
    }
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    (*xd).plane[0 as core::ffi::c_int as usize].color_index_map = ((*cb_buffer)
        .color_index_map[0 as core::ffi::c_int as usize])
        .as_mut_ptr();
    (*xd).plane[1 as core::ffi::c_int as usize].color_index_map = ((*cb_buffer)
        .color_index_map[1 as core::ffi::c_int as usize])
        .as_mut_ptr();
    (*xd).color_index_map_offset[0 as core::ffi::c_int as usize] = 0 as uint16_t;
    (*xd).color_index_map_offset[1 as core::ffi::c_int as usize] = 0 as uint16_t;
}
#[inline]
unsafe extern "C-unwind" fn decoder_alloc_tile_data(
    mut pbi: *mut AV1Decoder,
    n_tiles: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    aom_free((*pbi).tile_data as *mut core::ffi::c_void);
    (*pbi).allocated_tiles = 0 as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"decoder_alloc_tile_data\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2530 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*pbi).tile_data = aom_memalign(
        32 as size_t,
        (n_tiles as size_t).wrapping_mul(::core::mem::size_of::<TileDataDec>() as size_t),
    ) as *mut TileDataDec;
    if ((*pbi).tile_data).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate pbi->tile_data at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            2530 as core::ffi::c_int,
        );
    }
    (*pbi).allocated_tiles = n_tiles;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_tiles {
        let tile_data: *mut TileDataDec = ((*pbi).tile_data).offset(i as isize);
        memset(
            &raw mut (*tile_data).dec_row_mt_sync as *mut AV1DecRowMTSync
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1DecRowMTSync>() as size_t,
        );
        i += 1;
    }
    (*pbi).allocated_row_mt_sync_rows = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_sync_range(mut width: core::ffi::c_int) -> core::ffi::c_int {
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn dec_row_mt_alloc(
    mut dec_row_mt_sync: *mut AV1DecRowMTSync,
    mut cm: *mut AV1_COMMON,
    mut rows: core::ffi::c_int,
) {
    (*dec_row_mt_sync).allocated_sb_rows = rows;
    let mut i: core::ffi::c_int = 0;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"dec_row_mt_alloc\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2566 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*dec_row_mt_sync).mutex_ = aom_malloc(
        (::core::mem::size_of::<pthread_mutex_t>() as size_t)
            .wrapping_mul(rows as size_t),
    ) as *mut pthread_mutex_t;
    if ((*dec_row_mt_sync).mutex_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate dec_row_mt_sync->mutex_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            2566 as core::ffi::c_int,
        );
    }
    if !((*dec_row_mt_sync).mutex_).is_null() {
        i = 0 as core::ffi::c_int;
        while i < rows {
            pthread_mutex_init(
                &raw mut *((*dec_row_mt_sync).mutex_).offset(i as isize),
                0 as *const pthread_mutexattr_t,
            );
            i += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"dec_row_mt_alloc\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2574 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*dec_row_mt_sync).cond_ = aom_malloc(
        (::core::mem::size_of::<pthread_cond_t>() as size_t).wrapping_mul(rows as size_t),
    ) as *mut pthread_cond_t;
    if ((*dec_row_mt_sync).cond_).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate dec_row_mt_sync->cond_ at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            2574 as core::ffi::c_int,
        );
    }
    if !((*dec_row_mt_sync).cond_).is_null() {
        i = 0 as core::ffi::c_int;
        while i < rows {
            pthread_cond_init(
                &raw mut *((*dec_row_mt_sync).cond_).offset(i as isize),
                0 as *const pthread_condattr_t,
            );
            i += 1;
        }
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"dec_row_mt_alloc\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2584 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*dec_row_mt_sync).cur_sb_col = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(rows as size_t),
    ) as *mut core::ffi::c_int;
    if ((*dec_row_mt_sync).cur_sb_col).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate dec_row_mt_sync->cur_sb_col at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            2584 as core::ffi::c_int,
        );
    }
    (*dec_row_mt_sync).sync_range = get_sync_range((*cm).width);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dec_row_mt_dealloc(
    mut dec_row_mt_sync: *mut AV1DecRowMTSync,
) {
    if !dec_row_mt_sync.is_null() {
        let mut i: core::ffi::c_int = 0;
        if !((*dec_row_mt_sync).mutex_).is_null() {
            i = 0 as core::ffi::c_int;
            while i < (*dec_row_mt_sync).allocated_sb_rows {
                pthread_mutex_destroy(
                    &raw mut *((*dec_row_mt_sync).mutex_).offset(i as isize),
                );
                i += 1;
            }
            aom_free((*dec_row_mt_sync).mutex_ as *mut core::ffi::c_void);
        }
        if !((*dec_row_mt_sync).cond_).is_null() {
            i = 0 as core::ffi::c_int;
            while i < (*dec_row_mt_sync).allocated_sb_rows {
                pthread_cond_destroy(
                    &raw mut *((*dec_row_mt_sync).cond_).offset(i as isize),
                );
                i += 1;
            }
            aom_free((*dec_row_mt_sync).cond_ as *mut core::ffi::c_void);
        }
        aom_free((*dec_row_mt_sync).cur_sb_col as *mut core::ffi::c_void);
        memset(
            dec_row_mt_sync as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1DecRowMTSync>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_read(
    dec_row_mt_sync: *mut AV1DecRowMTSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*dec_row_mt_sync).sync_range;
    if r != 0 && c & nsync - 1 as core::ffi::c_int == 0 {
        let mutex: *mut pthread_mutex_t = &raw mut *((*dec_row_mt_sync).mutex_)
            .offset((r - 1 as core::ffi::c_int) as isize) as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex as *mut pthread_mutex_t);
        while c
            > *((*dec_row_mt_sync).cur_sb_col)
                .offset((r - 1 as core::ffi::c_int) as isize) - nsync
                - (*dec_row_mt_sync).intrabc_extra_top_right_sb_delay
        {
            pthread_cond_wait(
                &raw mut *((*dec_row_mt_sync).cond_)
                    .offset((r - 1 as core::ffi::c_int) as isize),
                mutex as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock(mutex as *mut pthread_mutex_t);
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_write(
    dec_row_mt_sync: *mut AV1DecRowMTSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    sb_cols: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*dec_row_mt_sync).sync_range;
    let mut cur: core::ffi::c_int = 0;
    let mut sig: core::ffi::c_int = 1 as core::ffi::c_int;
    if c < sb_cols - 1 as core::ffi::c_int {
        cur = c;
        if c % nsync != 0 {
            sig = 0 as core::ffi::c_int;
        }
    } else {
        cur = sb_cols + nsync + (*dec_row_mt_sync).intrabc_extra_top_right_sb_delay;
    }
    if sig != 0 {
        pthread_mutex_lock(&raw mut *((*dec_row_mt_sync).mutex_).offset(r as isize));
        *((*dec_row_mt_sync).cur_sb_col).offset(r as isize) = cur;
        pthread_cond_signal(&raw mut *((*dec_row_mt_sync).cond_).offset(r as isize));
        pthread_mutex_unlock(&raw mut *((*dec_row_mt_sync).mutex_).offset(r as isize));
    }
}
#[inline]
unsafe extern "C-unwind" fn signal_decoding_done_for_erroneous_row(
    pbi: *mut AV1Decoder,
    xd: *const MACROBLOCKD,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tile: *const TileInfo = &(*xd).tile;
    let sb_row_in_tile: core::ffi::c_int = (*xd).mi_row - (*tile).mi_row_start
        >> (*(*cm).seq_params).mib_size_log2;
    let sb_cols_in_tile: core::ffi::c_int = av1_get_sb_cols_in_tile(cm, tile)
        as core::ffi::c_int;
    let tile_data: *mut TileDataDec = ((*pbi).tile_data)
        .offset(((*tile).tile_row * (*cm).tiles.cols) as isize)
        .offset((*tile).tile_col as isize);
    let mut dec_row_mt_sync: *mut AV1DecRowMTSync = &raw mut (*tile_data).dec_row_mt_sync;
    sync_write(
        dec_row_mt_sync,
        sb_row_in_tile,
        sb_cols_in_tile - 1 as core::ffi::c_int,
        sb_cols_in_tile,
    );
}
#[inline]
unsafe extern "C-unwind" fn decode_tile_sb_row(
    mut pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut tile_info: *const TileInfo,
    mi_row: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tile_data: *mut TileDataDec = ((*pbi).tile_data)
        .offset(((*tile_info).tile_row * (*cm).tiles.cols) as isize)
        .offset((*tile_info).tile_col as isize);
    let sb_cols_in_tile: core::ffi::c_int = av1_get_sb_cols_in_tile(cm, tile_info)
        as core::ffi::c_int;
    let sb_row_in_tile: core::ffi::c_int = mi_row - (*tile_info).mi_row_start
        >> (*(*cm).seq_params).mib_size_log2;
    let mut sb_col_in_tile: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row_mt_exit: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mi_col: core::ffi::c_int = (*tile_info).mi_col_start;
    while mi_col < (*tile_info).mi_col_end {
        set_cb_buffer(
            pbi,
            &raw mut (*td).dcb,
            (*pbi).cb_buffer_base,
            num_planes,
            mi_row,
            mi_col,
        );
        sync_read(&raw mut (*tile_data).dec_row_mt_sync, sb_row_in_tile, sb_col_in_tile);
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        row_mt_exit = (*pbi).frame_row_mt_info.row_mt_exit;
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        if row_mt_exit == 0 {
            decode_partition(
                pbi,
                td,
                mi_row,
                mi_col,
                (*td).bit_reader,
                (*(*cm).seq_params).sb_size,
                0x2 as core::ffi::c_int,
            );
        }
        sync_write(
            &raw mut (*tile_data).dec_row_mt_sync,
            sb_row_in_tile,
            sb_col_in_tile,
            sb_cols_in_tile,
        );
        mi_col += (*(*cm).seq_params).mib_size;
        sb_col_in_tile += 1;
    }
}
unsafe extern "C-unwind" fn check_trailing_bits_after_symbol_coder(
    mut r: *mut aom_reader,
) -> core::ffi::c_int {
    if aom_reader_has_overflowed(r) != 0 {
        return -(1 as core::ffi::c_int);
    }
    let mut nb_bits: uint32_t = aom_reader_tell(r);
    let mut nb_bytes: uint32_t = nb_bits.wrapping_add(7 as uint32_t)
        >> 3 as core::ffi::c_int;
    let mut p: *const uint8_t = (aom_reader_find_begin(r)).offset(nb_bytes as isize);
    let mut last_byte: uint8_t = *p.offset(-(1 as core::ffi::c_int) as isize);
    let mut pattern: uint8_t = (128 as core::ffi::c_int
        >> (nb_bits.wrapping_sub(1 as uint32_t) & 7 as uint32_t)) as uint8_t;
    if last_byte as core::ffi::c_int
        & 2 as core::ffi::c_int * pattern as core::ffi::c_int - 1 as core::ffi::c_int
        != pattern as core::ffi::c_int
    {
        return -(1 as core::ffi::c_int);
    }
    let mut p_end: *const uint8_t = aom_reader_find_end(r);
    while p < p_end {
        if *p as core::ffi::c_int != 0 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        p = p.offset(1);
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_decode_func_pointers(
    mut td: *mut ThreadData,
    mut parse_decode_flag: core::ffi::c_int,
) {
    (*td).read_coeffs_tx_intra_block_visit = Some(
        decode_block_void
            as unsafe extern "C-unwind" fn(
                *const AV1_COMMON,
                *mut DecoderCodingBlock,
                *mut aom_reader,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                TX_SIZE,
            ) -> (),
    ) as decode_block_visitor_fn_t;
    (*td).predict_and_recon_intra_block_visit = Some(
        decode_block_void
            as unsafe extern "C-unwind" fn(
                *const AV1_COMMON,
                *mut DecoderCodingBlock,
                *mut aom_reader,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                TX_SIZE,
            ) -> (),
    ) as decode_block_visitor_fn_t;
    (*td).read_coeffs_tx_inter_block_visit = Some(
        decode_block_void
            as unsafe extern "C-unwind" fn(
                *const AV1_COMMON,
                *mut DecoderCodingBlock,
                *mut aom_reader,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                TX_SIZE,
            ) -> (),
    ) as decode_block_visitor_fn_t;
    (*td).inverse_tx_inter_block_visit = Some(
        decode_block_void
            as unsafe extern "C-unwind" fn(
                *const AV1_COMMON,
                *mut DecoderCodingBlock,
                *mut aom_reader,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                TX_SIZE,
            ) -> (),
    ) as decode_block_visitor_fn_t;
    (*td).predict_inter_block_visit = Some(
        predict_inter_block_void
            as unsafe extern "C-unwind" fn(
                *mut AV1_COMMON,
                *mut DecoderCodingBlock,
                BLOCK_SIZE,
            ) -> (),
    ) as predict_inter_block_visitor_fn_t;
    (*td).cfl_store_inter_block_visit = Some(
        cfl_store_inter_block_void
            as unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
    ) as cfl_store_inter_block_visitor_fn_t;
    if parse_decode_flag & 0x1 as core::ffi::c_int != 0 {
        (*td).read_coeffs_tx_intra_block_visit = Some(
            read_coeffs_tx_intra_block
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMMON,
                    *mut DecoderCodingBlock,
                    *mut aom_reader,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    TX_SIZE,
                ) -> (),
        ) as decode_block_visitor_fn_t;
        (*td).read_coeffs_tx_inter_block_visit = Some(
            av1_read_coeffs_txb
                as unsafe extern "C-unwind" fn(
                    *const AV1Common,
                    *mut DecoderCodingBlock,
                    *mut aom_reader,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    TX_SIZE,
                ) -> (),
        ) as decode_block_visitor_fn_t;
    }
    if parse_decode_flag & 0x2 as core::ffi::c_int != 0 {
        (*td).predict_and_recon_intra_block_visit = Some(
            predict_and_reconstruct_intra_block
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMMON,
                    *mut DecoderCodingBlock,
                    *mut aom_reader,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    TX_SIZE,
                ) -> (),
        ) as decode_block_visitor_fn_t;
        (*td).inverse_tx_inter_block_visit = Some(
            inverse_transform_inter_block
                as unsafe extern "C-unwind" fn(
                    *const AV1_COMMON,
                    *mut DecoderCodingBlock,
                    *mut aom_reader,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    TX_SIZE,
                ) -> (),
        ) as decode_block_visitor_fn_t;
        (*td).predict_inter_block_visit = Some(
            predict_inter_block
                as unsafe extern "C-unwind" fn(
                    *mut AV1_COMMON,
                    *mut DecoderCodingBlock,
                    BLOCK_SIZE,
                ) -> (),
        ) as predict_inter_block_visitor_fn_t;
        (*td).cfl_store_inter_block_visit = Some(
            cfl_store_inter_block
                as unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
        ) as cfl_store_inter_block_visitor_fn_t;
    }
}
#[inline]
unsafe extern "C-unwind" fn decode_tile(
    mut pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
) {
    let mut tile_info: TileInfo = TileInfo {
        mi_row_start: 0,
        mi_row_end: 0,
        mi_col_start: 0,
        mi_col_end: 0,
        tile_row: 0,
        tile_col: 0,
    };
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    av1_tile_set_row(&raw mut tile_info, cm, tile_row);
    av1_tile_set_col(&raw mut tile_info, cm, tile_col);
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    av1_zero_above_context(
        cm,
        xd,
        tile_info.mi_col_start,
        tile_info.mi_col_end,
        tile_row,
    );
    av1_reset_loop_filter_delta(xd, num_planes);
    av1_reset_loop_restoration(xd, num_planes);
    let mut mi_row: core::ffi::c_int = tile_info.mi_row_start;
    while mi_row < tile_info.mi_row_end {
        av1_zero_left_context(xd);
        let mut mi_col: core::ffi::c_int = tile_info.mi_col_start;
        while mi_col < tile_info.mi_col_end {
            set_cb_buffer(
                pbi,
                dcb,
                &raw mut (*td).cb_buffer_base,
                num_planes,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            decode_partition(
                pbi,
                td,
                mi_row,
                mi_col,
                (*td).bit_reader,
                (*(*cm).seq_params).sb_size,
                0x3 as core::ffi::c_int,
            );
            if aom_reader_has_overflowed((*td).bit_reader) != 0 {
                aom_merge_corrupted_flag(&raw mut (*dcb).corrupted, 1 as core::ffi::c_int);
                return;
            }
            mi_col += (*(*cm).seq_params).mib_size;
        }
        mi_row += (*(*cm).seq_params).mib_size;
    }
    let mut corrupted: core::ffi::c_int = if check_trailing_bits_after_symbol_coder(
        (*td).bit_reader,
    ) != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    aom_merge_corrupted_flag(&raw mut (*dcb).corrupted, corrupted);
}
unsafe extern "C-unwind" fn decode_tiles(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) -> *const uint8_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let td: *mut ThreadData = &raw mut (*pbi).td;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let n_tiles: core::ffi::c_int = tile_cols * tile_rows;
    let tile_buffers: *mut [TileBufferDec; 64] = ((*pbi).tile_buffers).as_mut_ptr()
        as *mut [TileBufferDec; 64];
    let dec_tile_row: core::ffi::c_int = if (*pbi).dec_tile_row < tile_rows {
        (*pbi).dec_tile_row
    } else {
        tile_rows
    };
    let single_row: core::ffi::c_int = ((*pbi).dec_tile_row >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let dec_tile_col: core::ffi::c_int = if (*pbi).dec_tile_col < tile_cols {
        (*pbi).dec_tile_col
    } else {
        tile_cols
    };
    let single_col: core::ffi::c_int = ((*pbi).dec_tile_col >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut tile_rows_start: core::ffi::c_int = 0;
    let mut tile_rows_end: core::ffi::c_int = 0;
    let mut tile_cols_start: core::ffi::c_int = 0;
    let mut tile_cols_end: core::ffi::c_int = 0;
    let mut inv_col_order: core::ffi::c_int = 0;
    let mut inv_row_order: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = 0;
    let mut tile_col: core::ffi::c_int = 0;
    let mut allow_update_cdf: uint8_t = 0;
    let mut raw_data_end: *const uint8_t = 0 as *const uint8_t;
    if (*tiles).large_scale != 0 {
        tile_rows_start = if single_row != 0 {
            dec_tile_row
        } else {
            0 as core::ffi::c_int
        };
        tile_rows_end = if single_row != 0 {
            dec_tile_row + 1 as core::ffi::c_int
        } else {
            tile_rows
        };
        tile_cols_start = if single_col != 0 {
            dec_tile_col
        } else {
            0 as core::ffi::c_int
        };
        tile_cols_end = if single_col != 0 {
            tile_cols_start + 1 as core::ffi::c_int
        } else {
            tile_cols
        };
        inv_col_order = ((*pbi).inv_tile_order != 0 && single_col == 0)
            as core::ffi::c_int;
        inv_row_order = ((*pbi).inv_tile_order != 0 && single_row == 0)
            as core::ffi::c_int;
        allow_update_cdf = 0 as uint8_t;
    } else {
        tile_rows_start = 0 as core::ffi::c_int;
        tile_rows_end = tile_rows;
        tile_cols_start = 0 as core::ffi::c_int;
        tile_cols_end = tile_cols;
        inv_col_order = (*pbi).inv_tile_order;
        inv_row_order = (*pbi).inv_tile_order;
        allow_update_cdf = 1 as uint8_t;
    }
    if tile_rows_end <= tile_rows_start || tile_cols_end <= tile_cols_start
        || tile_rows_start * (*tiles).cols + tile_cols_start > end_tile
        || ((tile_rows_end - 1 as core::ffi::c_int) * (*tiles).cols + tile_cols_end
            - 1 as core::ffi::c_int) < start_tile
    {
        return data;
    }
    allow_update_cdf = (allow_update_cdf as core::ffi::c_int != 0
        && !(*cm).features.disable_cdf_update) as core::ffi::c_int as uint8_t;
    if !(tile_rows <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"decode_tiles\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2895 as core::ffi::c_int,
            b"tile_rows <= MAX_TILE_ROWS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tile_cols <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"decode_tiles\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            2896 as core::ffi::c_int,
            b"tile_cols <= MAX_TILE_COLS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*tiles).large_scale != 0 && (*pbi).ext_tile_debug == 0 {
        raw_data_end = get_ls_single_tile_buffer(pbi, data, tile_buffers);
    } else if (*tiles).large_scale != 0 && (*pbi).ext_tile_debug != 0 {
        raw_data_end = get_ls_tile_buffers(pbi, data, data_end, tile_buffers);
    } else {
        get_tile_buffers(pbi, data, data_end, tile_buffers, start_tile, end_tile);
    }
    if ((*pbi).tile_data).is_null() || n_tiles != (*pbi).allocated_tiles {
        decoder_alloc_tile_data(pbi, n_tiles);
    }
    if ((*pbi).dcb.xd.seg_mask).is_null() {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"decode_tiles\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                2913 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).dcb.xd.seg_mask = aom_memalign(
            16 as size_t,
            ((2 as core::ffi::c_int
                * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
        if ((*pbi).dcb.xd.seg_mask).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->dcb.xd.seg_mask at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                2913 as core::ffi::c_int,
            );
        }
    }
    set_decode_func_pointers(&raw mut (*pbi).td, 0x3 as core::ffi::c_int);
    (*td).dcb = (*pbi).dcb;
    (*td).dcb.corrupted = 0 as core::ffi::c_int;
    (*td).dcb.mc_buf[0 as core::ffi::c_int as usize] = (*td)
        .mc_buf[0 as core::ffi::c_int as usize];
    (*td).dcb.mc_buf[1 as core::ffi::c_int as usize] = (*td)
        .mc_buf[1 as core::ffi::c_int as usize];
    (*td).dcb.xd.tmp_conv_dst = (*td).tmp_conv_dst;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < 2 as core::ffi::c_int {
        (*td).dcb.xd.tmp_obmc_bufs[j as usize] = (*td).tmp_obmc_bufs[j as usize];
        j += 1;
    }
    tile_row = tile_rows_start;
    while tile_row < tile_rows_end {
        let row: core::ffi::c_int = if inv_row_order != 0 {
            tile_rows - 1 as core::ffi::c_int - tile_row
        } else {
            tile_row
        };
        tile_col = tile_cols_start;
        while tile_col < tile_cols_end {
            let col: core::ffi::c_int = if inv_col_order != 0 {
                tile_cols - 1 as core::ffi::c_int - tile_col
            } else {
                tile_col
            };
            let tile_data: *mut TileDataDec = ((*pbi).tile_data)
                .offset((row * (*tiles).cols) as isize)
                .offset(col as isize);
            let tile_bs_buf: *const TileBufferDec = &raw mut *(*tile_buffers
                .offset(row as isize))
                .as_mut_ptr()
                .offset(col as isize) as *mut TileBufferDec;
            if !(row * (*tiles).cols + col < start_tile
                || row * (*tiles).cols + col > end_tile)
            {
                (*td).bit_reader = &raw mut (*tile_data).bit_reader;
                memset(
                    &raw mut (*td).cb_buffer_base.dqcoeff as *mut [[tran_low_t; 16384]; 3]
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[tran_low_t; 16384]; 3]>() as size_t,
                );
                av1_tile_init(&raw mut (*td).dcb.xd.tile, cm, row, col);
                (*td).dcb.xd.current_base_qindex = (*cm).quant_params.base_qindex;
                setup_bool_decoder(
                    &raw mut (*td).dcb.xd,
                    (*tile_bs_buf).data,
                    data_end,
                    (*tile_bs_buf).size,
                    &raw mut (*pbi).error,
                    (*td).bit_reader,
                    allow_update_cdf,
                );
                av1_init_macroblockd(cm, &raw mut (*td).dcb.xd);
                av1_init_above_context(
                    &raw mut (*cm).above_contexts,
                    av1_num_planes(cm),
                    row,
                    &raw mut (*td).dcb.xd,
                );
                (*tile_data).tctx = *(*cm).fc;
                (*td).dcb.xd.tile_ctx = &raw mut (*tile_data).tctx;
                decode_tile(pbi, td, row, col);
                aom_merge_corrupted_flag(&raw mut (*pbi).dcb.corrupted, (*td).dcb.corrupted);
                if (*pbi).dcb.corrupted != 0 {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Failed to decode tile data\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
    if (*tiles).large_scale != 0 {
        if n_tiles == 1 as core::ffi::c_int {
            return aom_reader_find_end(&raw mut (*(*pbi).tile_data).bit_reader);
        }
        return raw_data_end;
    }
    let tile_data_0: *mut TileDataDec = ((*pbi).tile_data).offset(end_tile as isize);
    return aom_reader_find_end(&raw mut (*tile_data_0).bit_reader);
}
unsafe extern "C-unwind" fn get_dec_job_info(
    mut tile_mt_info: *mut AV1DecTileMT,
) -> *mut TileJobsDec {
    let mut cur_job_info: *mut TileJobsDec = 0 as *mut TileJobsDec;
    pthread_mutex_lock((*tile_mt_info).job_mutex as *mut pthread_mutex_t);
    if (*tile_mt_info).jobs_dequeued < (*tile_mt_info).jobs_enqueued {
        cur_job_info = ((*tile_mt_info).job_queue)
            .offset((*tile_mt_info).jobs_dequeued as isize);
        (*tile_mt_info).jobs_dequeued += 1;
    }
    pthread_mutex_unlock((*tile_mt_info).job_mutex as *mut pthread_mutex_t);
    return cur_job_info;
}
#[inline]
unsafe extern "C-unwind" fn tile_worker_hook_init(
    pbi: *mut AV1Decoder,
    thread_data: *mut DecWorkerData,
    tile_buffer: *const TileBufferDec,
    tile_data: *mut TileDataDec,
    mut allow_update_cdf: uint8_t,
) {
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
    let mut tile_row: core::ffi::c_int = (*tile_data).tile_info.tile_row;
    let mut tile_col: core::ffi::c_int = (*tile_data).tile_info.tile_col;
    (*td).bit_reader = &raw mut (*tile_data).bit_reader;
    memset(
        &raw mut (*td).cb_buffer_base.dqcoeff as *mut [[tran_low_t; 16384]; 3]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[tran_low_t; 16384]; 3]>() as size_t,
    );
    let xd: *mut MACROBLOCKD = &raw mut (*td).dcb.xd;
    av1_tile_init(&raw mut (*xd).tile, cm, tile_row, tile_col);
    (*xd).current_base_qindex = (*cm).quant_params.base_qindex;
    setup_bool_decoder(
        xd,
        (*tile_buffer).data,
        (*thread_data).data_end,
        (*tile_buffer).size,
        &raw mut (*thread_data).error_info,
        (*td).bit_reader,
        allow_update_cdf,
    );
    av1_init_macroblockd(cm, xd);
    (*xd).error_info = &raw mut (*thread_data).error_info;
    av1_init_above_context(&raw mut (*cm).above_contexts, av1_num_planes(cm), tile_row, xd);
    (*tile_data).tctx = *(*cm).fc;
    (*xd).tile_ctx = &raw mut (*tile_data).tctx;
}
unsafe extern "C-unwind" fn tile_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut DecWorkerData = arg1 as *mut DecWorkerData;
    let pbi: *mut AV1Decoder = arg2 as *mut AV1Decoder;
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
    let mut allow_update_cdf: uint8_t = 0;
    loop {
match crate::jump::value_scope((((*thread_data).error_info.jmp).as_mut_ptr()).cast(), || {
    (*thread_data).error_info.setjmp = 1 as core::ffi::c_int;
    allow_update_cdf = (if (*cm).tiles.large_scale != 0 {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint8_t;
    allow_update_cdf = (allow_update_cdf as core::ffi::c_int != 0
        && !(*cm).features.disable_cdf_update) as core::ffi::c_int as uint8_t;
    set_decode_func_pointers(td, 0x3 as core::ffi::c_int);
    if !((*cm).tiles.cols > 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"tile_worker_hook\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3074 as core::ffi::c_int,
            b"cm->tiles.cols > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    while (*td).dcb.corrupted == 0 {
        let mut cur_job_info: *mut TileJobsDec = get_dec_job_info(
            &raw mut (*pbi).tile_mt_info,
        );
        if cur_job_info.is_null() {
            break;
        }
        let tile_buffer: *const TileBufferDec = (*cur_job_info).tile_buffer;
        let tile_data: *mut TileDataDec = (*cur_job_info).tile_data;
        tile_worker_hook_init(
            pbi,
            thread_data,
            tile_buffer,
            tile_data,
            allow_update_cdf,
        );
        let mut tile_row: core::ffi::c_int = (*tile_data).tile_info.tile_row;
        let mut tile_col: core::ffi::c_int = (*tile_data).tile_info.tile_col;
        decode_tile(pbi, td, tile_row, tile_col);
    }
    (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
    return ((*td).dcb.corrupted == 0) as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
        (*(*thread_data).td).dcb.corrupted = 1 as core::ffi::c_int;
        return 0 as core::ffi::c_int;}
}
}}
#[inline]
unsafe extern "C-unwind" fn get_max_row_mt_workers_per_tile(
    mut cm: *mut AV1_COMMON,
    mut tile: *const TileInfo,
) -> core::ffi::c_int {
    let mut sb_rows: core::ffi::c_int = av1_get_sb_rows_in_tile(cm, tile);
    let mut max_workers: core::ffi::c_int = if sb_rows == 1 as core::ffi::c_int {
        AOM_MIN_THREADS_PER_TILE
    } else {
        AOM_MAX_THREADS_PER_TILE
    };
    return max_workers;
}
unsafe extern "C-unwind" fn get_next_job_info(
    pbi: *mut AV1Decoder,
    mut next_job_info: *mut AV1DecRowMTJobInfo,
    mut end_of_frame: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut tile_data: *mut TileDataDec = 0 as *mut TileDataDec;
    let mut dec_row_mt_sync: *mut AV1DecRowMTSync = 0 as *mut AV1DecRowMTSync;
    let mut frame_row_mt_info: *mut AV1DecRowMTInfo = &raw mut (*pbi).frame_row_mt_info;
    let tile_rows_start: core::ffi::c_int = (*frame_row_mt_info).tile_rows_start;
    let tile_rows_end: core::ffi::c_int = (*frame_row_mt_info).tile_rows_end;
    let tile_cols_start: core::ffi::c_int = (*frame_row_mt_info).tile_cols_start;
    let tile_cols_end: core::ffi::c_int = (*frame_row_mt_info).tile_cols_end;
    let start_tile: core::ffi::c_int = (*frame_row_mt_info).start_tile;
    let end_tile: core::ffi::c_int = (*frame_row_mt_info).end_tile;
    let sb_mi_size: core::ffi::c_int = mi_size_wide[(*(*cm).seq_params).sb_size as usize]
        as core::ffi::c_int;
    let mut num_mis_to_decode: core::ffi::c_int = 0;
    let mut num_threads_working: core::ffi::c_int = 0;
    let mut num_mis_waiting_for_decode: core::ffi::c_int = 0;
    let mut min_threads_working: core::ffi::c_int = INT_MAX;
    let mut max_mis_to_decode: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_row_idx: core::ffi::c_int = 0;
    let mut tile_col_idx: core::ffi::c_int = 0;
    let mut tile_row: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut tile_col: core::ffi::c_int = -(1 as core::ffi::c_int);
    memset(
        next_job_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AV1DecRowMTJobInfo>() as size_t,
    );
    *end_of_frame = ((*frame_row_mt_info).mi_rows_decode_started
        == (*frame_row_mt_info).mi_rows_to_decode
        || (*frame_row_mt_info).row_mt_exit == 1 as core::ffi::c_int)
        as core::ffi::c_int;
    if *end_of_frame != 0 {
        return 1 as core::ffi::c_int;
    }
    if !((*frame_row_mt_info).mi_rows_parse_done
        >= (*frame_row_mt_info).mi_rows_decode_started) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_next_job_info\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3153 as core::ffi::c_int,
            b"frame_row_mt_info->mi_rows_parse_done >= frame_row_mt_info->mi_rows_decode_started\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*frame_row_mt_info).mi_rows_parse_done
        == (*frame_row_mt_info).mi_rows_decode_started
    {
        return 0 as core::ffi::c_int;
    }
    tile_row_idx = tile_rows_start;
    while tile_row_idx < tile_rows_end {
        tile_col_idx = tile_cols_start;
        while tile_col_idx < tile_cols_end {
            if !(tile_row_idx * (*cm).tiles.cols + tile_col_idx < start_tile
                || tile_row_idx * (*cm).tiles.cols + tile_col_idx > end_tile)
            {
                tile_data = ((*pbi).tile_data)
                    .offset((tile_row_idx * (*cm).tiles.cols) as isize)
                    .offset(tile_col_idx as isize);
                dec_row_mt_sync = &raw mut (*tile_data).dec_row_mt_sync;
                num_threads_working = (*dec_row_mt_sync).num_threads_working;
                num_mis_waiting_for_decode = ((*dec_row_mt_sync).mi_rows_parse_done
                    - (*dec_row_mt_sync).mi_rows_decode_started)
                    * (*dec_row_mt_sync).mi_cols;
                num_mis_to_decode = ((*dec_row_mt_sync).mi_rows
                    - (*dec_row_mt_sync).mi_rows_decode_started)
                    * (*dec_row_mt_sync).mi_cols;
                if !(num_mis_to_decode >= num_mis_waiting_for_decode) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 18],
                            [core::ffi::c_char; 18],
                        >(*b"get_next_job_info\0"))
                            .as_ptr(),
                        b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                        3178 as core::ffi::c_int,
                        b"num_mis_to_decode >= num_mis_waiting_for_decode\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                if num_mis_waiting_for_decode > 0 as core::ffi::c_int {
                    if num_threads_working < min_threads_working {
                        min_threads_working = num_threads_working;
                        max_mis_to_decode = 0 as core::ffi::c_int;
                    }
                    if num_threads_working == min_threads_working
                        && num_mis_to_decode > max_mis_to_decode
                        && num_threads_working
                            < get_max_row_mt_workers_per_tile(
                                cm,
                                &raw mut (*tile_data).tile_info,
                            )
                    {
                        max_mis_to_decode = num_mis_to_decode;
                        tile_row = tile_row_idx;
                        tile_col = tile_col_idx;
                    }
                }
            }
            tile_col_idx += 1;
        }
        tile_row_idx += 1;
    }
    if tile_row == -(1 as core::ffi::c_int) || tile_col == -(1 as core::ffi::c_int) {
        return 0 as core::ffi::c_int;
    }
    tile_data = ((*pbi).tile_data)
        .offset((tile_row * (*cm).tiles.cols) as isize)
        .offset(tile_col as isize);
    dec_row_mt_sync = &raw mut (*tile_data).dec_row_mt_sync;
    (*next_job_info).tile_row = tile_row;
    (*next_job_info).tile_col = tile_col;
    (*next_job_info).mi_row = (*dec_row_mt_sync).mi_rows_decode_started
        + (*tile_data).tile_info.mi_row_start;
    (*dec_row_mt_sync).num_threads_working += 1;
    (*dec_row_mt_sync).mi_rows_decode_started += sb_mi_size;
    (*frame_row_mt_info).mi_rows_decode_started += sb_mi_size;
    if !((*frame_row_mt_info).mi_rows_parse_done
        >= (*frame_row_mt_info).mi_rows_decode_started) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_next_job_info\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3212 as core::ffi::c_int,
            b"frame_row_mt_info->mi_rows_parse_done >= frame_row_mt_info->mi_rows_decode_started\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*frame_row_mt_info).mi_rows_decode_started
        == (*frame_row_mt_info).mi_rows_to_decode
    {
        pthread_cond_broadcast((*pbi).row_mt_cond_ as *mut pthread_cond_t);
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn signal_parse_sb_row_done(
    pbi: *mut AV1Decoder,
    tile_data: *mut TileDataDec,
    sb_mi_size: core::ffi::c_int,
) {
    let mut frame_row_mt_info: *mut AV1DecRowMTInfo = &raw mut (*pbi).frame_row_mt_info;
    pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
    if !((*frame_row_mt_info).mi_rows_parse_done
        >= (*frame_row_mt_info).mi_rows_decode_started) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"signal_parse_sb_row_done\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3231 as core::ffi::c_int,
            b"frame_row_mt_info->mi_rows_parse_done >= frame_row_mt_info->mi_rows_decode_started\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tile_data).dec_row_mt_sync.mi_rows_parse_done += sb_mi_size;
    (*frame_row_mt_info).mi_rows_parse_done += sb_mi_size;
    pthread_cond_signal((*pbi).row_mt_cond_ as *mut pthread_cond_t);
    pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
}
#[inline]
unsafe extern "C-unwind" fn parse_tile_row_mt(
    mut pbi: *mut AV1Decoder,
    td: *mut ThreadData,
    tile_data: *mut TileDataDec,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let sb_mi_size: core::ffi::c_int = mi_size_wide[(*(*cm).seq_params).sb_size as usize]
        as core::ffi::c_int;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
    let mut tile_row: core::ffi::c_int = (*tile_info).tile_row;
    let dcb: *mut DecoderCodingBlock = &raw mut (*td).dcb;
    let xd: *mut MACROBLOCKD = &raw mut (*dcb).xd;
    av1_zero_above_context(
        cm,
        xd,
        (*tile_info).mi_col_start,
        (*tile_info).mi_col_end,
        tile_row,
    );
    av1_reset_loop_filter_delta(xd, num_planes);
    av1_reset_loop_restoration(xd, num_planes);
    let mut mi_row: core::ffi::c_int = (*tile_info).mi_row_start;
    while mi_row < (*tile_info).mi_row_end {
        av1_zero_left_context(xd);
        let mut mi_col: core::ffi::c_int = (*tile_info).mi_col_start;
        while mi_col < (*tile_info).mi_col_end {
            set_cb_buffer(pbi, dcb, (*pbi).cb_buffer_base, num_planes, mi_row, mi_col);
            decode_partition(
                pbi,
                td,
                mi_row,
                mi_col,
                (*td).bit_reader,
                (*(*cm).seq_params).sb_size,
                0x1 as core::ffi::c_int,
            );
            if aom_reader_has_overflowed((*td).bit_reader) != 0 {
                aom_merge_corrupted_flag(&raw mut (*dcb).corrupted, 1 as core::ffi::c_int);
                return;
            }
            mi_col += (*(*cm).seq_params).mib_size;
        }
        signal_parse_sb_row_done(pbi, tile_data, sb_mi_size);
        mi_row += (*(*cm).seq_params).mib_size;
    }
    let mut corrupted: core::ffi::c_int = if check_trailing_bits_after_symbol_coder(
        (*td).bit_reader,
    ) != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    aom_merge_corrupted_flag(&raw mut (*dcb).corrupted, corrupted);
}
unsafe extern "C-unwind" fn row_mt_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let thread_data: *mut DecWorkerData = arg1 as *mut DecWorkerData;
    let pbi: *mut AV1Decoder = arg2 as *mut AV1Decoder;
    let td: *mut ThreadData = (*thread_data).td as *mut ThreadData;
    let mut allow_update_cdf: uint8_t = 0;
    let mut frame_row_mt_info: *mut AV1DecRowMTInfo = &raw mut (*pbi).frame_row_mt_info;
    (*td).dcb.corrupted = 0 as core::ffi::c_int;
    loop {
match crate::jump::value_scope((((*thread_data).error_info.jmp).as_mut_ptr()).cast(), || {
    (*thread_data).error_info.setjmp = 1 as core::ffi::c_int;
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    allow_update_cdf = (if (*cm).tiles.large_scale != 0 {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint8_t;
    allow_update_cdf = (allow_update_cdf as core::ffi::c_int != 0
        && !(*cm).features.disable_cdf_update) as core::ffi::c_int as uint8_t;
    set_decode_func_pointers(td, 0x1 as core::ffi::c_int);
    if !((*cm).tiles.cols > 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"row_mt_worker_hook\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3324 as core::ffi::c_int,
            b"cm->tiles.cols > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    while (*td).dcb.corrupted == 0 {
        let mut cur_job_info: *mut TileJobsDec = get_dec_job_info(
            &raw mut (*pbi).tile_mt_info,
        );
        if cur_job_info.is_null() {
            break;
        }
        let tile_buffer: *const TileBufferDec = (*cur_job_info).tile_buffer;
        let tile_data: *mut TileDataDec = (*cur_job_info).tile_data;
        tile_worker_hook_init(
            pbi,
            thread_data,
            tile_buffer,
            tile_data,
            allow_update_cdf,
        );
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        (*tile_data).dec_row_mt_sync.num_threads_working += 1;
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        parse_tile_row_mt(pbi, td, tile_data);
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        (*tile_data).dec_row_mt_sync.num_threads_working -= 1;
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
    }
    if (*td).dcb.corrupted != 0 {
        (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        (*frame_row_mt_info).row_mt_exit = 1 as core::ffi::c_int;
        pthread_cond_broadcast((*pbi).row_mt_cond_ as *mut pthread_cond_t);
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        return 0 as core::ffi::c_int;
    }
    set_decode_func_pointers(td, 0x2 as core::ffi::c_int);
    loop {
        let mut next_job_info: AV1DecRowMTJobInfo = AV1DecRowMTJobInfo {
            tile_row: 0,
            tile_col: 0,
            mi_row: 0,
        };
        let mut end_of_frame: core::ffi::c_int = 0 as core::ffi::c_int;
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        while get_next_job_info(pbi, &raw mut next_job_info, &raw mut end_of_frame) == 0 {
            pthread_cond_wait(
                (*pbi).row_mt_cond_ as *mut pthread_cond_t,
                (*pbi).row_mt_mutex_ as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        if end_of_frame != 0 {
            break;
        }
        let mut tile_row: core::ffi::c_int = next_job_info.tile_row;
        let mut tile_col: core::ffi::c_int = next_job_info.tile_col;
        let mut mi_row: core::ffi::c_int = next_job_info.mi_row;
        let mut tile_data_0: *mut TileDataDec = ((*pbi).tile_data)
            .offset((tile_row * (*cm).tiles.cols) as isize)
            .offset(tile_col as isize);
        let mut dec_row_mt_sync: *mut AV1DecRowMTSync = &raw mut (*tile_data_0)
            .dec_row_mt_sync;
        av1_tile_init(&raw mut (*td).dcb.xd.tile, cm, tile_row, tile_col);
        av1_init_macroblockd(cm, &raw mut (*td).dcb.xd);
        (*td).dcb.xd.error_info = &raw mut (*thread_data).error_info;
        decode_tile_sb_row(pbi, td, &raw mut (*tile_data_0).tile_info, mi_row);
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        (*dec_row_mt_sync).num_threads_working -= 1;
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
    }
    (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
    return ((*td).dcb.corrupted == 0) as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
        (*(*thread_data).td).dcb.corrupted = 1 as core::ffi::c_int;
        pthread_mutex_lock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        (*frame_row_mt_info).row_mt_exit = 1 as core::ffi::c_int;
        pthread_cond_broadcast((*pbi).row_mt_cond_ as *mut pthread_cond_t);
        pthread_mutex_unlock((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        signal_decoding_done_for_erroneous_row(pbi, &raw mut (*(*thread_data).td).dcb.xd);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn compare_tile_buffers(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let buf1: *const TileJobsDec = a as *const TileJobsDec;
    let buf2: *const TileJobsDec = b as *const TileJobsDec;
    return (*(*buf2).tile_buffer).size as core::ffi::c_int
        - (*(*buf1).tile_buffer).size as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn enqueue_tile_jobs(
    mut pbi: *mut AV1Decoder,
    mut cm: *mut AV1_COMMON,
    mut tile_rows_start: core::ffi::c_int,
    mut tile_rows_end: core::ffi::c_int,
    mut tile_cols_start: core::ffi::c_int,
    mut tile_cols_end: core::ffi::c_int,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) {
    let mut tile_mt_info: *mut AV1DecTileMT = &raw mut (*pbi).tile_mt_info;
    let mut tile_job_queue: *mut TileJobsDec = (*tile_mt_info).job_queue;
    (*tile_mt_info).jobs_enqueued = 0 as core::ffi::c_int;
    (*tile_mt_info).jobs_dequeued = 0 as core::ffi::c_int;
    let mut row: core::ffi::c_int = tile_rows_start;
    while row < tile_rows_end {
        let mut col: core::ffi::c_int = tile_cols_start;
        while col < tile_cols_end {
            if !(row * (*cm).tiles.cols + col < start_tile
                || row * (*cm).tiles.cols + col > end_tile)
            {
                (*tile_job_queue).tile_buffer = &raw mut *(*((*pbi).tile_buffers)
                    .as_mut_ptr()
                    .offset(row as isize))
                    .as_mut_ptr()
                    .offset(col as isize) as *mut TileBufferDec;
                (*tile_job_queue).tile_data = ((*pbi).tile_data)
                    .offset((row * (*cm).tiles.cols) as isize)
                    .offset(col as isize);
                tile_job_queue = tile_job_queue.offset(1);
                (*tile_mt_info).jobs_enqueued += 1;
            }
            col += 1;
        }
        row += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn alloc_dec_jobs(
    mut tile_mt_info: *mut AV1DecTileMT,
    mut cm: *mut AV1_COMMON,
    mut tile_rows: core::ffi::c_int,
    mut tile_cols: core::ffi::c_int,
) {
    (*tile_mt_info).alloc_tile_rows = tile_rows;
    (*tile_mt_info).alloc_tile_cols = tile_cols;
    let mut num_tiles: core::ffi::c_int = tile_rows * tile_cols;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"alloc_dec_jobs\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3450 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tile_mt_info).job_mutex = aom_malloc(
        (::core::mem::size_of::<pthread_mutex_t>() as size_t)
            .wrapping_mul(num_tiles as size_t),
    ) as *mut pthread_mutex_t;
    if ((*tile_mt_info).job_mutex).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate tile_mt_info->job_mutex at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            3450 as core::ffi::c_int,
        );
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_tiles {
        pthread_mutex_init(
            &raw mut *((*tile_mt_info).job_mutex).offset(i as isize),
            0 as *const pthread_mutexattr_t,
        );
        i += 1;
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"alloc_dec_jobs\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3458 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*tile_mt_info).job_queue = aom_malloc(
        (::core::mem::size_of::<TileJobsDec>() as size_t)
            .wrapping_mul(num_tiles as size_t),
    ) as *mut TileJobsDec;
    if ((*tile_mt_info).job_queue).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate tile_mt_info->job_queue at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            3458 as core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_free_mc_tmp_buf(mut thread_data: *mut ThreadData) {
    let mut ref_0: core::ffi::c_int = 0;
    ref_0 = 0 as core::ffi::c_int;
    while ref_0 < 2 as core::ffi::c_int {
        if (*thread_data).mc_buf_use_highbd != 0 {
            aom_free(
                (((*thread_data).mc_buf[ref_0 as usize] as uintptr_t)
                    << 1 as core::ffi::c_int) as *mut uint16_t as *mut core::ffi::c_void,
            );
        } else {
            aom_free((*thread_data).mc_buf[ref_0 as usize] as *mut core::ffi::c_void);
        }
        (*thread_data).mc_buf[ref_0 as usize] = 0 as *mut uint8_t;
        ref_0 += 1;
    }
    (*thread_data).mc_buf_size = 0 as core::ffi::c_int as int32_t;
    (*thread_data).mc_buf_use_highbd = 0 as core::ffi::c_int;
    aom_free((*thread_data).tmp_conv_dst as *mut core::ffi::c_void);
    (*thread_data).tmp_conv_dst = 0 as *mut CONV_BUF_TYPE;
    aom_free((*thread_data).seg_mask as *mut core::ffi::c_void);
    (*thread_data).seg_mask = 0 as *mut uint8_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        aom_free((*thread_data).tmp_obmc_bufs[i as usize] as *mut core::ffi::c_void);
        (*thread_data).tmp_obmc_bufs[i as usize] = 0 as *mut uint8_t;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn allocate_mc_tmp_buf(
    cm: *mut AV1_COMMON,
    mut thread_data: *mut ThreadData,
    mut buf_size: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
) {
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 2 as core::ffi::c_int {
        if use_highbd != 0 {
            let mut hbd_mc_buf: *mut uint16_t = 0 as *mut uint16_t;
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"allocate_mc_tmp_buf\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    3494 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            hbd_mc_buf = aom_memalign(16 as size_t, buf_size as size_t) as *mut uint16_t;
            if hbd_mc_buf.is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate hbd_mc_buf at %s:%d\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                        as *const core::ffi::c_char,
                    3494 as core::ffi::c_int,
                );
            }
            memset(
                hbd_mc_buf as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                buf_size as size_t,
            );
            (*thread_data).mc_buf[ref_0 as usize] = (hbd_mc_buf as uintptr_t
                >> 1 as core::ffi::c_int) as *mut uint8_t;
        } else {
            if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"allocate_mc_tmp_buf\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    3499 as core::ffi::c_int,
                    b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*thread_data).mc_buf[ref_0 as usize] = aom_memalign(
                16 as size_t,
                buf_size as size_t,
            ) as *mut uint8_t;
            if ((*thread_data).mc_buf[ref_0 as usize]).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate thread_data->mc_buf[ref] at %s:%d\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                        as *const core::ffi::c_char,
                    3499 as core::ffi::c_int,
                );
            }
            memset(
                (*thread_data).mc_buf[ref_0 as usize] as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                buf_size as size_t,
            );
        }
        ref_0 += 1;
    }
    (*thread_data).mc_buf_size = buf_size as int32_t;
    (*thread_data).mc_buf_use_highbd = use_highbd;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"allocate_mc_tmp_buf\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3508 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*thread_data).tmp_conv_dst = aom_memalign(
        32 as size_t,
        ((((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
            * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<CONV_BUF_TYPE>() as size_t),
    ) as *mut CONV_BUF_TYPE;
    if ((*thread_data).tmp_conv_dst).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate thread_data->tmp_conv_dst at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            3508 as core::ffi::c_int,
        );
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"allocate_mc_tmp_buf\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3511 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*thread_data).seg_mask = aom_memalign(
        16 as size_t,
        ((2 as core::ffi::c_int
            * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*thread_data).seg_mask).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate thread_data->seg_mask at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                as *const core::ffi::c_char,
            3511 as core::ffi::c_int,
        );
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"allocate_mc_tmp_buf\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3517 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*thread_data).tmp_obmc_bufs[i as usize] = aom_memalign(
            16 as size_t,
            ((2 as core::ffi::c_int * 3 as core::ffi::c_int
                * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
        if ((*thread_data).tmp_obmc_bufs[i as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate thread_data->tmp_obmc_bufs[i] at %s:%d\0"
                    as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3517 as core::ffi::c_int,
            );
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn reset_dec_workers(
    mut pbi: *mut AV1Decoder,
    mut worker_hook: AVxWorkerHook,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut worker_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while worker_idx < num_workers {
        let worker: *mut AVxWorker = &raw mut *((*pbi).tile_workers)
            .offset(worker_idx as isize) as *mut AVxWorker;
        let thread_data: *mut DecWorkerData = ((*pbi).thread_data)
            .offset(worker_idx as isize);
        (*(*thread_data).td).dcb = (*pbi).dcb;
        (*(*thread_data).td).dcb.corrupted = 0 as core::ffi::c_int;
        (*(*thread_data).td).dcb.mc_buf[0 as core::ffi::c_int as usize] = (*(*thread_data)
            .td)
            .mc_buf[0 as core::ffi::c_int as usize];
        (*(*thread_data).td).dcb.mc_buf[1 as core::ffi::c_int as usize] = (*(*thread_data)
            .td)
            .mc_buf[1 as core::ffi::c_int as usize];
        (*(*thread_data).td).dcb.xd.tmp_conv_dst = (*(*thread_data).td).tmp_conv_dst;
        if worker_idx != 0 {
            (*(*thread_data).td).dcb.xd.seg_mask = (*(*thread_data).td).seg_mask;
        }
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 2 as core::ffi::c_int {
            (*(*thread_data).td).dcb.xd.tmp_obmc_bufs[j as usize] = (*(*thread_data).td)
                .tmp_obmc_bufs[j as usize];
            j += 1;
        }
        ((*winterface).sync).expect("non-null function pointer")(worker);
        (*worker).hook = worker_hook;
        (*worker).data1 = thread_data as *mut core::ffi::c_void;
        (*worker).data2 = pbi as *mut core::ffi::c_void;
        worker_idx += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn launch_dec_workers(
    mut pbi: *mut AV1Decoder,
    mut data_end: *const uint8_t,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut worker_idx: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while worker_idx >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*pbi).tile_workers)
            .offset(worker_idx as isize) as *mut AVxWorker;
        let thread_data: *mut DecWorkerData = (*worker).data1 as *mut DecWorkerData;
        (*thread_data).data_end = data_end;
        (*worker).had_error = 0 as core::ffi::c_int;
        if worker_idx == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        worker_idx -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_dec_workers(
    mut pbi: *mut AV1Decoder,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut corrupted: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut worker_idx: core::ffi::c_int = num_workers;
    while worker_idx > 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *((*pbi).tile_workers)
            .offset((worker_idx - 1 as core::ffi::c_int) as isize) as *mut AVxWorker;
        aom_merge_corrupted_flag(
            &raw mut corrupted,
            (((*winterface).sync).expect("non-null function pointer")(worker) == 0)
                as core::ffi::c_int,
        );
        worker_idx -= 1;
    }
    (*pbi).dcb.corrupted = corrupted;
}
#[inline]
unsafe extern "C-unwind" fn decode_mt_init(mut pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut worker_idx: core::ffi::c_int = 0;
    if (*pbi).num_workers == 0 as core::ffi::c_int {
        let num_threads: core::ffi::c_int = (*pbi).max_threads;
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"decode_mt_init\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3593 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).tile_workers = aom_malloc(
            (num_threads as size_t)
                .wrapping_mul(::core::mem::size_of::<AVxWorker>() as size_t),
        ) as *mut AVxWorker;
        if ((*pbi).tile_workers).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->tile_workers at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3593 as core::ffi::c_int,
            );
        }
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"decode_mt_init\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3595 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).thread_data = aom_calloc(
            num_threads as size_t,
            ::core::mem::size_of::<DecWorkerData>() as size_t,
        ) as *mut DecWorkerData;
        if ((*pbi).thread_data).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->thread_data at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3595 as core::ffi::c_int,
            );
        }
        worker_idx = 0 as core::ffi::c_int;
        while worker_idx < num_threads {
            let worker: *mut AVxWorker = &raw mut *((*pbi).tile_workers)
                .offset(worker_idx as isize) as *mut AVxWorker;
            let thread_data: *mut DecWorkerData = ((*pbi).thread_data)
                .offset(worker_idx as isize);
            ((*winterface).init).expect("non-null function pointer")(worker);
            (*worker).thread_name = b"aom tile worker\0" as *const u8
                as *const core::ffi::c_char;
            if worker_idx != 0 as core::ffi::c_int
                && ((*winterface).reset).expect("non-null function pointer")(worker) == 0
            {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_ERROR,
                    b"Tile decoder thread creation failed\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            (*pbi).num_workers += 1;
            if worker_idx != 0 as core::ffi::c_int {
                if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 15],
                            [core::ffi::c_char; 15],
                        >(*b"decode_mt_init\0"))
                            .as_ptr(),
                        b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                        3612 as core::ffi::c_int,
                        b"((cm)->error)->setjmp\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*thread_data).td = aom_memalign(
                    32 as size_t,
                    ::core::mem::size_of::<ThreadData>() as size_t,
                ) as *mut ThreadData;
                if ((*thread_data).td).is_null() {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_MEM_ERROR,
                        b"Failed to allocate thread_data->td at %s:%d\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                            as *const core::ffi::c_char,
                        3612 as core::ffi::c_int,
                    );
                }
                memset(
                    (*thread_data).td as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<ThreadData>() as size_t,
                );
            } else {
                (*thread_data).td = &raw mut (*pbi).td as *mut ThreadData;
            }
            (*thread_data).error_info.error_code = AOM_CODEC_OK;
            (*thread_data).error_info.setjmp = 0 as core::ffi::c_int;
            worker_idx += 1;
        }
    }
    let use_highbd: core::ffi::c_int = (*(*cm).seq_params).use_highbitdepth
        as core::ffi::c_int;
    let buf_size: core::ffi::c_int = MC_TEMP_BUF_PELS << use_highbd;
    worker_idx = 1 as core::ffi::c_int;
    while worker_idx < (*pbi).max_threads {
        let thread_data_0: *mut DecWorkerData = ((*pbi).thread_data)
            .offset(worker_idx as isize);
        if (*(*thread_data_0).td).mc_buf_size != buf_size as int32_t {
            av1_free_mc_tmp_buf((*thread_data_0).td as *mut ThreadData);
            allocate_mc_tmp_buf(
                cm,
                (*thread_data_0).td as *mut ThreadData,
                buf_size,
                use_highbd,
            );
        }
        worker_idx += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn tile_mt_queue(
    mut pbi: *mut AV1Decoder,
    mut tile_cols: core::ffi::c_int,
    mut tile_rows: core::ffi::c_int,
    mut tile_rows_start: core::ffi::c_int,
    mut tile_rows_end: core::ffi::c_int,
    mut tile_cols_start: core::ffi::c_int,
    mut tile_cols_end: core::ffi::c_int,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    if (*pbi).tile_mt_info.alloc_tile_cols != tile_cols
        || (*pbi).tile_mt_info.alloc_tile_rows != tile_rows
    {
        av1_dealloc_dec_jobs(&raw mut (*pbi).tile_mt_info);
        alloc_dec_jobs(&raw mut (*pbi).tile_mt_info, cm, tile_rows, tile_cols);
    }
    enqueue_tile_jobs(
        pbi,
        cm,
        tile_rows_start,
        tile_rows_end,
        tile_cols_start,
        tile_cols_end,
        start_tile,
        end_tile,
    );
    qsort(
        (*pbi).tile_mt_info.job_queue as *mut core::ffi::c_void,
        (*pbi).tile_mt_info.jobs_enqueued as size_t,
        ::core::mem::size_of::<TileJobsDec>() as size_t,
        Some(
            compare_tile_buffers
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
}
unsafe extern "C-unwind" fn decode_tiles_mt(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) -> *const uint8_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let n_tiles: core::ffi::c_int = tile_cols * tile_rows;
    let tile_buffers: *mut [TileBufferDec; 64] = ((*pbi).tile_buffers).as_mut_ptr()
        as *mut [TileBufferDec; 64];
    let dec_tile_row: core::ffi::c_int = if (*pbi).dec_tile_row < tile_rows {
        (*pbi).dec_tile_row
    } else {
        tile_rows
    };
    let single_row: core::ffi::c_int = ((*pbi).dec_tile_row >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let dec_tile_col: core::ffi::c_int = if (*pbi).dec_tile_col < tile_cols {
        (*pbi).dec_tile_col
    } else {
        tile_cols
    };
    let single_col: core::ffi::c_int = ((*pbi).dec_tile_col >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut tile_rows_start: core::ffi::c_int = 0;
    let mut tile_rows_end: core::ffi::c_int = 0;
    let mut tile_cols_start: core::ffi::c_int = 0;
    let mut tile_cols_end: core::ffi::c_int = 0;
    let mut tile_count_tg: core::ffi::c_int = 0;
    let mut num_workers: core::ffi::c_int = 0;
    let mut raw_data_end: *const uint8_t = 0 as *const uint8_t;
    if (*tiles).large_scale != 0 {
        tile_rows_start = if single_row != 0 {
            dec_tile_row
        } else {
            0 as core::ffi::c_int
        };
        tile_rows_end = if single_row != 0 {
            dec_tile_row + 1 as core::ffi::c_int
        } else {
            tile_rows
        };
        tile_cols_start = if single_col != 0 {
            dec_tile_col
        } else {
            0 as core::ffi::c_int
        };
        tile_cols_end = if single_col != 0 {
            tile_cols_start + 1 as core::ffi::c_int
        } else {
            tile_cols
        };
    } else {
        tile_rows_start = 0 as core::ffi::c_int;
        tile_rows_end = tile_rows;
        tile_cols_start = 0 as core::ffi::c_int;
        tile_cols_end = tile_cols;
    }
    tile_count_tg = end_tile - start_tile + 1 as core::ffi::c_int;
    num_workers = if (*pbi).max_threads < tile_count_tg {
        (*pbi).max_threads
    } else {
        tile_count_tg
    };
    if tile_rows_end <= tile_rows_start || tile_cols_end <= tile_cols_start
        || tile_rows_start * tile_cols + tile_cols_start > end_tile
        || ((tile_rows_end - 1 as core::ffi::c_int) * tile_cols + tile_cols_end
            - 1 as core::ffi::c_int) < start_tile
    {
        return data;
    }
    if !(tile_rows <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3692 as core::ffi::c_int,
            b"tile_rows <= MAX_TILE_ROWS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tile_cols <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3693 as core::ffi::c_int,
            b"tile_cols <= MAX_TILE_COLS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tile_count_tg > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3694 as core::ffi::c_int,
            b"tile_count_tg > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(num_workers > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3695 as core::ffi::c_int,
            b"num_workers > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(start_tile <= end_tile) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3696 as core::ffi::c_int,
            b"start_tile <= end_tile\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(start_tile >= 0 as core::ffi::c_int && end_tile < n_tiles) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"decode_tiles_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3697 as core::ffi::c_int,
            b"start_tile >= 0 && end_tile < n_tiles\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    decode_mt_init(pbi);
    if (*tiles).large_scale != 0 {
        if !((*pbi).ext_tile_debug == 1 as core::ffi::c_uint) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"decode_tiles_mt\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3703 as core::ffi::c_int,
                b"pbi->ext_tile_debug == 1\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*tiles).large_scale != 0 {
        raw_data_end = get_ls_tile_buffers(pbi, data, data_end, tile_buffers);
    } else {
        get_tile_buffers(pbi, data, data_end, tile_buffers, start_tile, end_tile);
    }
    if ((*pbi).tile_data).is_null() || n_tiles != (*pbi).allocated_tiles {
        decoder_alloc_tile_data(pbi, n_tiles);
    }
    if ((*pbi).dcb.xd.seg_mask).is_null() {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"decode_tiles_mt\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3716 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).dcb.xd.seg_mask = aom_memalign(
            16 as size_t,
            ((2 as core::ffi::c_int
                * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
        if ((*pbi).dcb.xd.seg_mask).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->dcb.xd.seg_mask at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3716 as core::ffi::c_int,
            );
        }
    }
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < tile_cols {
            let mut tile_data: *mut TileDataDec = ((*pbi).tile_data)
                .offset((row * (*tiles).cols) as isize)
                .offset(col as isize);
            av1_tile_init(&raw mut (*tile_data).tile_info, cm, row, col);
            col += 1;
        }
        row += 1;
    }
    tile_mt_queue(
        pbi,
        tile_cols,
        tile_rows,
        tile_rows_start,
        tile_rows_end,
        tile_cols_start,
        tile_cols_end,
        start_tile,
        end_tile,
    );
    reset_dec_workers(
        pbi,
        Some(
            tile_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_dec_workers(pbi, data_end, num_workers);
    sync_dec_workers(pbi, num_workers);
    if (*pbi).dcb.corrupted != 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Failed to decode tile data\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if (*tiles).large_scale != 0 {
        if n_tiles == 1 as core::ffi::c_int {
            return aom_reader_find_end(&raw mut (*(*pbi).tile_data).bit_reader);
        }
        return raw_data_end;
    }
    let tile_data_0: *mut TileDataDec = ((*pbi).tile_data).offset(end_tile as isize);
    return aom_reader_find_end(&raw mut (*tile_data_0).bit_reader);
}
#[inline]
unsafe extern "C-unwind" fn dec_alloc_cb_buf(mut pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut size: core::ffi::c_int = (((*cm).mi_params.mi_rows
        >> (*(*cm).seq_params).mib_size_log2) + 1 as core::ffi::c_int)
        * (((*cm).mi_params.mi_cols >> (*(*cm).seq_params).mib_size_log2)
            + 1 as core::ffi::c_int);
    if (*pbi).cb_buffer_alloc_size < size {
        av1_dec_free_cb_buf(pbi);
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"dec_alloc_cb_buf\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3757 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).cb_buffer_base = aom_memalign(
            32 as size_t,
            (::core::mem::size_of::<CB_BUFFER>() as size_t).wrapping_mul(size as size_t),
        ) as *mut CB_BUFFER;
        if ((*pbi).cb_buffer_base).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->cb_buffer_base at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3757 as core::ffi::c_int,
            );
        }
        memset(
            (*pbi).cb_buffer_base as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<CB_BUFFER>() as size_t).wrapping_mul(size as size_t),
        );
        (*pbi).cb_buffer_alloc_size = size;
    }
}
#[inline]
unsafe extern "C-unwind" fn row_mt_frame_init(
    mut pbi: *mut AV1Decoder,
    mut tile_rows_start: core::ffi::c_int,
    mut tile_rows_end: core::ffi::c_int,
    mut tile_cols_start: core::ffi::c_int,
    mut tile_cols_end: core::ffi::c_int,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
    mut max_sb_rows: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut frame_row_mt_info: *mut AV1DecRowMTInfo = &raw mut (*pbi).frame_row_mt_info;
    (*frame_row_mt_info).tile_rows_start = tile_rows_start;
    (*frame_row_mt_info).tile_rows_end = tile_rows_end;
    (*frame_row_mt_info).tile_cols_start = tile_cols_start;
    (*frame_row_mt_info).tile_cols_end = tile_cols_end;
    (*frame_row_mt_info).start_tile = start_tile;
    (*frame_row_mt_info).end_tile = end_tile;
    (*frame_row_mt_info).mi_rows_to_decode = 0 as core::ffi::c_int;
    (*frame_row_mt_info).mi_rows_parse_done = 0 as core::ffi::c_int;
    (*frame_row_mt_info).mi_rows_decode_started = 0 as core::ffi::c_int;
    (*frame_row_mt_info).row_mt_exit = 0 as core::ffi::c_int;
    let mut tile_row: core::ffi::c_int = tile_rows_start;
    while tile_row < tile_rows_end {
        let mut tile_col: core::ffi::c_int = tile_cols_start;
        while tile_col < tile_cols_end {
            if !(tile_row * (*cm).tiles.cols + tile_col < start_tile
                || tile_row * (*cm).tiles.cols + tile_col > end_tile)
            {
                let tile_data: *mut TileDataDec = ((*pbi).tile_data)
                    .offset((tile_row * (*cm).tiles.cols) as isize)
                    .offset(tile_col as isize);
                let tile_info: *const TileInfo = &raw mut (*tile_data).tile_info;
                (*tile_data).dec_row_mt_sync.mi_rows_parse_done = 0 as core::ffi::c_int;
                (*tile_data).dec_row_mt_sync.mi_rows_decode_started = 0
                    as core::ffi::c_int;
                (*tile_data).dec_row_mt_sync.num_threads_working = 0 as core::ffi::c_int;
                (*tile_data).dec_row_mt_sync.mi_rows = (*tile_info).mi_row_end
                    - (*tile_info).mi_row_start
                    + (((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
                        - 1 as core::ffi::c_int)
                    & !(((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
                        - 1 as core::ffi::c_int);
                (*tile_data).dec_row_mt_sync.mi_cols = (*tile_info).mi_col_end
                    - (*tile_info).mi_col_start
                    + (((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
                        - 1 as core::ffi::c_int)
                    & !(((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
                        - 1 as core::ffi::c_int);
                (*tile_data).dec_row_mt_sync.intrabc_extra_top_right_sb_delay = av1_get_intrabc_extra_top_right_sb_delay(
                    cm,
                );
                (*frame_row_mt_info).mi_rows_to_decode
                    += (*tile_data).dec_row_mt_sync.mi_rows;
                memset(
                    (*tile_data).dec_row_mt_sync.cur_sb_col as *mut core::ffi::c_void,
                    -(1 as core::ffi::c_int),
                    (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                        .wrapping_mul(max_sb_rows as size_t),
                );
            }
            tile_col += 1;
        }
        tile_row += 1;
    }
    if ((*pbi).row_mt_mutex_).is_null() {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"row_mt_frame_init\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3815 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).row_mt_mutex_ = aom_malloc(
            ::core::mem::size_of::<pthread_mutex_t>() as size_t,
        ) as *mut pthread_mutex_t;
        if ((*pbi).row_mt_mutex_).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->row_mt_mutex_ at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3815 as core::ffi::c_int,
            );
        }
        if !((*pbi).row_mt_mutex_).is_null() {
            pthread_mutex_init(
                (*pbi).row_mt_mutex_ as *mut pthread_mutex_t,
                0 as *const pthread_mutexattr_t,
            );
        }
    }
    if ((*pbi).row_mt_cond_).is_null() {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"row_mt_frame_init\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3823 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).row_mt_cond_ = aom_malloc(
            ::core::mem::size_of::<pthread_cond_t>() as size_t,
        ) as *mut pthread_cond_t;
        if ((*pbi).row_mt_cond_).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->row_mt_cond_ at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3823 as core::ffi::c_int,
            );
        }
        if !((*pbi).row_mt_cond_).is_null() {
            pthread_cond_init(
                (*pbi).row_mt_cond_ as *mut pthread_cond_t,
                0 as *const pthread_condattr_t,
            );
        }
    }
}
unsafe extern "C-unwind" fn decode_tiles_row_mt(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
) -> *const uint8_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let n_tiles: core::ffi::c_int = tile_cols * tile_rows;
    let tile_buffers: *mut [TileBufferDec; 64] = ((*pbi).tile_buffers).as_mut_ptr()
        as *mut [TileBufferDec; 64];
    let dec_tile_row: core::ffi::c_int = if (*pbi).dec_tile_row < tile_rows {
        (*pbi).dec_tile_row
    } else {
        tile_rows
    };
    let single_row: core::ffi::c_int = ((*pbi).dec_tile_row >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let dec_tile_col: core::ffi::c_int = if (*pbi).dec_tile_col < tile_cols {
        (*pbi).dec_tile_col
    } else {
        tile_cols
    };
    let single_col: core::ffi::c_int = ((*pbi).dec_tile_col >= 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut tile_rows_start: core::ffi::c_int = 0;
    let mut tile_rows_end: core::ffi::c_int = 0;
    let mut tile_cols_start: core::ffi::c_int = 0;
    let mut tile_cols_end: core::ffi::c_int = 0;
    let mut tile_count_tg: core::ffi::c_int = 0;
    let mut num_workers: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_threads: core::ffi::c_int = 0;
    let mut raw_data_end: *const uint8_t = 0 as *const uint8_t;
    let mut max_sb_rows: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*tiles).large_scale != 0 {
        tile_rows_start = if single_row != 0 {
            dec_tile_row
        } else {
            0 as core::ffi::c_int
        };
        tile_rows_end = if single_row != 0 {
            dec_tile_row + 1 as core::ffi::c_int
        } else {
            tile_rows
        };
        tile_cols_start = if single_col != 0 {
            dec_tile_col
        } else {
            0 as core::ffi::c_int
        };
        tile_cols_end = if single_col != 0 {
            tile_cols_start + 1 as core::ffi::c_int
        } else {
            tile_cols
        };
    } else {
        tile_rows_start = 0 as core::ffi::c_int;
        tile_rows_end = tile_rows;
        tile_cols_start = 0 as core::ffi::c_int;
        tile_cols_end = tile_cols;
    }
    tile_count_tg = end_tile - start_tile + 1 as core::ffi::c_int;
    max_threads = (*pbi).max_threads;
    if tile_rows_end <= tile_rows_start || tile_cols_end <= tile_cols_start
        || tile_rows_start * tile_cols + tile_cols_start > end_tile
        || ((tile_rows_end - 1 as core::ffi::c_int) * tile_cols + tile_cols_end
            - 1 as core::ffi::c_int) < start_tile
    {
        return data;
    }
    if !(tile_rows <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3876 as core::ffi::c_int,
            b"tile_rows <= MAX_TILE_ROWS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tile_cols <= 64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3877 as core::ffi::c_int,
            b"tile_cols <= MAX_TILE_COLS\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tile_count_tg > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3878 as core::ffi::c_int,
            b"tile_count_tg > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(max_threads > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3879 as core::ffi::c_int,
            b"max_threads > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(start_tile <= end_tile) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3880 as core::ffi::c_int,
            b"start_tile <= end_tile\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(start_tile >= 0 as core::ffi::c_int && end_tile < n_tiles) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"decode_tiles_row_mt\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            3881 as core::ffi::c_int,
            b"start_tile >= 0 && end_tile < n_tiles\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    decode_mt_init(pbi);
    if (*tiles).large_scale != 0 {
        if !((*pbi).ext_tile_debug == 1 as core::ffi::c_uint) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"decode_tiles_row_mt\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3889 as core::ffi::c_int,
                b"pbi->ext_tile_debug == 1\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*tiles).large_scale != 0 {
        raw_data_end = get_ls_tile_buffers(pbi, data, data_end, tile_buffers);
    } else {
        get_tile_buffers(pbi, data, data_end, tile_buffers, start_tile, end_tile);
    }
    if ((*pbi).tile_data).is_null() || n_tiles != (*pbi).allocated_tiles {
        if !((*pbi).tile_data).is_null() {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < (*pbi).allocated_tiles {
                let tile_data: *mut TileDataDec = ((*pbi).tile_data).offset(i as isize);
                av1_dec_row_mt_dealloc(&raw mut (*tile_data).dec_row_mt_sync);
                i += 1;
            }
        }
        decoder_alloc_tile_data(pbi, n_tiles);
    }
    if ((*pbi).dcb.xd.seg_mask).is_null() {
        if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"decode_tiles_row_mt\0"))
                    .as_ptr(),
                b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                3908 as core::ffi::c_int,
                b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        (*pbi).dcb.xd.seg_mask = aom_memalign(
            16 as size_t,
            ((2 as core::ffi::c_int
                * (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                    * ((1 as core::ffi::c_int) << 7 as core::ffi::c_int))) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
        if ((*pbi).dcb.xd.seg_mask).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate pbi->dcb.xd.seg_mask at %s:%d\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/decodeframe.c\0" as *const u8
                    as *const core::ffi::c_char,
                3908 as core::ffi::c_int,
            );
        }
    }
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < tile_rows {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < tile_cols {
            let mut tile_data_0: *mut TileDataDec = ((*pbi).tile_data)
                .offset((row * (*tiles).cols) as isize)
                .offset(col as isize);
            av1_tile_init(&raw mut (*tile_data_0).tile_info, cm, row, col);
            max_sb_rows = if max_sb_rows
                > av1_get_sb_rows_in_tile(cm, &raw mut (*tile_data_0).tile_info)
            {
                max_sb_rows
            } else {
                av1_get_sb_rows_in_tile(cm, &raw mut (*tile_data_0).tile_info)
            };
            num_workers
                += get_max_row_mt_workers_per_tile(cm, &raw mut (*tile_data_0).tile_info);
            col += 1;
        }
        row += 1;
    }
    num_workers = if num_workers < max_threads { num_workers } else { max_threads };
    if (*pbi).allocated_row_mt_sync_rows != max_sb_rows {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < n_tiles {
            let tile_data_1: *mut TileDataDec = ((*pbi).tile_data).offset(i_0 as isize);
            av1_dec_row_mt_dealloc(&raw mut (*tile_data_1).dec_row_mt_sync);
            dec_row_mt_alloc(&raw mut (*tile_data_1).dec_row_mt_sync, cm, max_sb_rows);
            i_0 += 1;
        }
        (*pbi).allocated_row_mt_sync_rows = max_sb_rows;
    }
    tile_mt_queue(
        pbi,
        tile_cols,
        tile_rows,
        tile_rows_start,
        tile_rows_end,
        tile_cols_start,
        tile_cols_end,
        start_tile,
        end_tile,
    );
    dec_alloc_cb_buf(pbi);
    row_mt_frame_init(
        pbi,
        tile_rows_start,
        tile_rows_end,
        tile_cols_start,
        tile_cols_end,
        start_tile,
        end_tile,
        max_sb_rows,
    );
    reset_dec_workers(
        pbi,
        Some(
            row_mt_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        num_workers,
    );
    launch_dec_workers(pbi, data_end, num_workers);
    sync_dec_workers(pbi, num_workers);
    if (*pbi).dcb.corrupted != 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Failed to decode tile data\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if (*tiles).large_scale != 0 {
        if n_tiles == 1 as core::ffi::c_int {
            return aom_reader_find_end(&raw mut (*(*pbi).tile_data).bit_reader);
        }
        return raw_data_end;
    }
    let tile_data_2: *mut TileDataDec = ((*pbi).tile_data).offset(end_tile as isize);
    return aom_reader_find_end(&raw mut (*tile_data_2).bit_reader);
}
#[inline]
unsafe extern "C-unwind" fn error_handler(mut data: *mut core::ffi::c_void) {
    let cm: *mut AV1_COMMON = data as *mut AV1_COMMON;
    aom_internal_error(
        (*cm).error,
        AOM_CODEC_CORRUPT_FRAME,
        b"Truncated packet\0" as *const u8 as *const core::ffi::c_char,
    );
}
#[inline]
unsafe extern "C-unwind" fn read_bitdepth(
    mut rb: *mut aom_read_bit_buffer,
    mut seq_params: *mut SequenceHeader,
    mut error_info: *mut aom_internal_error_info,
) {
    let high_bitdepth: core::ffi::c_int = aom_rb_read_bit(rb) as core::ffi::c_int;
    if (*seq_params).profile as core::ffi::c_int == PROFILE_2 as core::ffi::c_int
        && high_bitdepth != 0
    {
        let twelve_bit: core::ffi::c_int = aom_rb_read_bit(rb) as core::ffi::c_int;
        (*seq_params).bit_depth = (if twelve_bit != 0 {
            AOM_BITS_12 as core::ffi::c_int
        } else {
            AOM_BITS_10 as core::ffi::c_int
        }) as aom_bit_depth_t;
    } else if (*seq_params).profile as core::ffi::c_int <= PROFILE_2 as core::ffi::c_int
    {
        (*seq_params).bit_depth = (if high_bitdepth != 0 {
            AOM_BITS_10 as core::ffi::c_int
        } else {
            AOM_BITS_8 as core::ffi::c_int
        }) as aom_bit_depth_t;
    } else {
        aom_internal_error(
            error_info,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"Unsupported profile/bit-depth combination\0" as *const u8
                as *const core::ffi::c_char,
        );
    };
}
unsafe extern "C-unwind" fn read_film_grain_params(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    let mut pars: *mut aom_film_grain_t = &raw mut (*cm).film_grain_params;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    (*pars).apply_grain = aom_rb_read_bit(rb);
    if (*pars).apply_grain == 0 {
        memset(
            pars as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_film_grain_t>() as size_t,
        );
        return;
    }
    (*pars).random_seed = aom_rb_read_literal(rb, 16 as core::ffi::c_int) as uint16_t;
    if (*cm).current_frame.frame_type as core::ffi::c_int
        == INTER_FRAME as core::ffi::c_int
    {
        (*pars).update_parameters = aom_rb_read_bit(rb);
    } else {
        (*pars).update_parameters = 1 as core::ffi::c_int;
    }
    (*pars).bit_depth = (*seq_params).bit_depth as core::ffi::c_uint;
    if (*pars).update_parameters == 0 {
        let mut film_grain_params_ref_idx: core::ffi::c_int = aom_rb_read_literal(
            rb,
            3 as core::ffi::c_int,
        );
        let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
            if film_grain_params_ref_idx == (*cm).remapped_ref_idx[i as usize] {
                found = 1 as core::ffi::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if found == 0 {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Invalid film grain reference idx %d. ref_frame_idx = {%d, %d, %d, %d, %d, %d, %d}\0"
                    as *const u8 as *const core::ffi::c_char,
                film_grain_params_ref_idx,
                (*cm).remapped_ref_idx[0 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[1 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[2 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[3 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[4 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[5 as core::ffi::c_int as usize],
                (*cm).remapped_ref_idx[6 as core::ffi::c_int as usize],
            );
        }
        let buf: *mut RefCntBuffer = (*cm)
            .ref_frame_map[film_grain_params_ref_idx as usize];
        if buf.is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Invalid Film grain reference idx\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if (*buf).film_grain_params_present == 0 {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Film grain reference parameters not available\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut random_seed: uint16_t = (*pars).random_seed;
        *pars = (*buf).film_grain_params;
        (*pars).random_seed = random_seed;
        return;
    }
    (*pars).num_y_points = aom_rb_read_literal(rb, 4 as core::ffi::c_int);
    if (*pars).num_y_points > 14 as core::ffi::c_int {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"Number of points for film grain luma scaling function exceeds the maximum value.\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*pars).num_y_points {
        (*pars).scaling_points_y[i_0 as usize][0 as core::ffi::c_int as usize] = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        );
        if i_0 != 0
            && (*pars)
                .scaling_points_y[(i_0 - 1 as core::ffi::c_int)
                as usize][0 as core::ffi::c_int as usize]
                >= (*pars).scaling_points_y[i_0 as usize][0 as core::ffi::c_int as usize]
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"First coordinate of the scaling function points shall be increasing.\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        (*pars).scaling_points_y[i_0 as usize][1 as core::ffi::c_int as usize] = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        );
        i_0 += 1;
    }
    if (*seq_params).monochrome == 0 {
        (*pars).chroma_scaling_from_luma = aom_rb_read_bit(rb);
    } else {
        (*pars).chroma_scaling_from_luma = 0 as core::ffi::c_int;
    }
    if (*seq_params).monochrome as core::ffi::c_int != 0
        || (*pars).chroma_scaling_from_luma != 0
        || (*seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*seq_params).subsampling_y == 1 as core::ffi::c_int
            && (*pars).num_y_points == 0 as core::ffi::c_int
    {
        (*pars).num_cb_points = 0 as core::ffi::c_int;
        (*pars).num_cr_points = 0 as core::ffi::c_int;
    } else {
        (*pars).num_cb_points = aom_rb_read_literal(rb, 4 as core::ffi::c_int);
        if (*pars).num_cb_points > 10 as core::ffi::c_int {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Number of points for film grain cb scaling function exceeds the maximum value.\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < (*pars).num_cb_points {
            (*pars).scaling_points_cb[i_1 as usize][0 as core::ffi::c_int as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            );
            if i_1 != 0
                && (*pars)
                    .scaling_points_cb[(i_1 - 1 as core::ffi::c_int)
                    as usize][0 as core::ffi::c_int as usize]
                    >= (*pars)
                        .scaling_points_cb[i_1 as usize][0 as core::ffi::c_int as usize]
            {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_UNSUP_BITSTREAM,
                    b"First coordinate of the scaling function points shall be increasing.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            (*pars).scaling_points_cb[i_1 as usize][1 as core::ffi::c_int as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            );
            i_1 += 1;
        }
        (*pars).num_cr_points = aom_rb_read_literal(rb, 4 as core::ffi::c_int);
        if (*pars).num_cr_points > 10 as core::ffi::c_int {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Number of points for film grain cr scaling function exceeds the maximum value.\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < (*pars).num_cr_points {
            (*pars).scaling_points_cr[i_2 as usize][0 as core::ffi::c_int as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            );
            if i_2 != 0
                && (*pars)
                    .scaling_points_cr[(i_2 - 1 as core::ffi::c_int)
                    as usize][0 as core::ffi::c_int as usize]
                    >= (*pars)
                        .scaling_points_cr[i_2 as usize][0 as core::ffi::c_int as usize]
            {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_UNSUP_BITSTREAM,
                    b"First coordinate of the scaling function points shall be increasing.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            (*pars).scaling_points_cr[i_2 as usize][1 as core::ffi::c_int as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            );
            i_2 += 1;
        }
        if (*seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*seq_params).subsampling_y == 1 as core::ffi::c_int
            && ((*pars).num_cb_points == 0 as core::ffi::c_int
                && (*pars).num_cr_points != 0 as core::ffi::c_int
                || (*pars).num_cb_points != 0 as core::ffi::c_int
                    && (*pars).num_cr_points == 0 as core::ffi::c_int)
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"In YCbCr 4:2:0, film grain shall be applied to both chroma components or neither.\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    (*pars).scaling_shift = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
        + 8 as core::ffi::c_int;
    (*pars).ar_coeff_lag = aom_rb_read_literal(rb, 2 as core::ffi::c_int);
    let mut num_pos_luma: core::ffi::c_int = 2 as core::ffi::c_int * (*pars).ar_coeff_lag
        * ((*pars).ar_coeff_lag + 1 as core::ffi::c_int);
    let mut num_pos_chroma: core::ffi::c_int = num_pos_luma;
    if (*pars).num_y_points > 0 as core::ffi::c_int {
        num_pos_chroma += 1;
    }
    if (*pars).num_y_points != 0 {
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 < num_pos_luma {
            (*pars).ar_coeffs_y[i_3 as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            ) - 128 as core::ffi::c_int;
            i_3 += 1;
        }
    }
    if (*pars).num_cb_points != 0 || (*pars).chroma_scaling_from_luma != 0 {
        let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_4 < num_pos_chroma {
            (*pars).ar_coeffs_cb[i_4 as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            ) - 128 as core::ffi::c_int;
            i_4 += 1;
        }
    }
    if (*pars).num_cr_points != 0 || (*pars).chroma_scaling_from_luma != 0 {
        let mut i_5: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_5 < num_pos_chroma {
            (*pars).ar_coeffs_cr[i_5 as usize] = aom_rb_read_literal(
                rb,
                8 as core::ffi::c_int,
            ) - 128 as core::ffi::c_int;
            i_5 += 1;
        }
    }
    (*pars).ar_coeff_shift = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
        + 6 as core::ffi::c_int;
    (*pars).grain_scale_shift = aom_rb_read_literal(rb, 2 as core::ffi::c_int);
    if (*pars).num_cb_points != 0 {
        (*pars).cb_mult = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        (*pars).cb_luma_mult = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        (*pars).cb_offset = aom_rb_read_literal(rb, 9 as core::ffi::c_int);
    }
    if (*pars).num_cr_points != 0 {
        (*pars).cr_mult = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        (*pars).cr_luma_mult = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        (*pars).cr_offset = aom_rb_read_literal(rb, 9 as core::ffi::c_int);
    }
    (*pars).overlap_flag = aom_rb_read_bit(rb);
    (*pars).clip_to_restricted_range = aom_rb_read_bit(rb);
}
#[inline]
unsafe extern "C-unwind" fn read_film_grain(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    if (*(*cm).seq_params).film_grain_params_present as core::ffi::c_int != 0
        && ((*cm).show_frame != 0 || (*cm).showable_frame != 0)
    {
        read_film_grain_params(cm, rb);
    } else {
        memset(
            &raw mut (*cm).film_grain_params as *mut aom_film_grain_t
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_film_grain_t>() as size_t,
        );
    }
    (*cm).film_grain_params.bit_depth = (*(*cm).seq_params).bit_depth
        as core::ffi::c_uint;
    (*(*cm).cur_frame).film_grain_params = (*cm).film_grain_params;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_color_config(
    mut rb: *mut aom_read_bit_buffer,
    mut allow_lowbitdepth: core::ffi::c_int,
    mut seq_params: *mut SequenceHeader,
    mut error_info: *mut aom_internal_error_info,
) {
    read_bitdepth(rb, seq_params, error_info);
    (*seq_params).use_highbitdepth = ((*seq_params).bit_depth as core::ffi::c_uint
        > AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint || allow_lowbitdepth == 0)
        as core::ffi::c_int as uint8_t;
    let is_monochrome: core::ffi::c_int = if (*seq_params).profile as core::ffi::c_int
        != PROFILE_1 as core::ffi::c_int
    {
        aom_rb_read_bit(rb) as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    (*seq_params).monochrome = is_monochrome as uint8_t;
    let mut color_description_present_flag: core::ffi::c_int = aom_rb_read_bit(rb);
    if color_description_present_flag != 0 {
        (*seq_params).color_primaries = aom_rb_read_literal(rb, 8 as core::ffi::c_int)
            as aom_color_primaries_t;
        (*seq_params).transfer_characteristics = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        ) as aom_transfer_characteristics_t;
        (*seq_params).matrix_coefficients = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        ) as aom_matrix_coefficients_t;
    } else {
        (*seq_params).color_primaries = AOM_CICP_CP_UNSPECIFIED;
        (*seq_params).transfer_characteristics = AOM_CICP_TC_UNSPECIFIED;
        (*seq_params).matrix_coefficients = AOM_CICP_MC_UNSPECIFIED;
    }
    if is_monochrome != 0 {
        (*seq_params).color_range = aom_rb_read_bit(rb);
        (*seq_params).subsampling_x = 1 as core::ffi::c_int;
        (*seq_params).subsampling_y = (*seq_params).subsampling_x;
        (*seq_params).chroma_sample_position = AOM_CSP_UNKNOWN;
        (*seq_params).separate_uv_delta_q = 0 as uint8_t;
        return;
    }
    if (*seq_params).color_primaries as core::ffi::c_uint
        == AOM_CICP_CP_BT_709 as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).transfer_characteristics as core::ffi::c_uint
            == AOM_CICP_TC_SRGB as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).matrix_coefficients as core::ffi::c_uint
            == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
    {
        (*seq_params).subsampling_x = 0 as core::ffi::c_int;
        (*seq_params).subsampling_y = (*seq_params).subsampling_x;
        (*seq_params).color_range = 1 as core::ffi::c_int;
        if !((*seq_params).profile as core::ffi::c_int == PROFILE_1 as core::ffi::c_int
            || (*seq_params).profile as core::ffi::c_int == PROFILE_2 as core::ffi::c_int
                && (*seq_params).bit_depth as core::ffi::c_uint
                    == AOM_BITS_12 as core::ffi::c_int as core::ffi::c_uint)
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"sRGB colorspace not compatible with specified profile\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    } else {
        (*seq_params).color_range = aom_rb_read_bit(rb);
        if (*seq_params).profile as core::ffi::c_int == PROFILE_0 as core::ffi::c_int {
            (*seq_params).subsampling_y = 1 as core::ffi::c_int;
            (*seq_params).subsampling_x = (*seq_params).subsampling_y;
        } else if (*seq_params).profile as core::ffi::c_int
            == PROFILE_1 as core::ffi::c_int
        {
            (*seq_params).subsampling_y = 0 as core::ffi::c_int;
            (*seq_params).subsampling_x = (*seq_params).subsampling_y;
        } else {
            if !((*seq_params).profile as core::ffi::c_int
                == PROFILE_2 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"av1_read_color_config\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    4218 as core::ffi::c_int,
                    b"seq_params->profile == PROFILE_2\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if (*seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_12 as core::ffi::c_int as core::ffi::c_uint
            {
                (*seq_params).subsampling_x = aom_rb_read_bit(rb);
                if (*seq_params).subsampling_x != 0 {
                    (*seq_params).subsampling_y = aom_rb_read_bit(rb);
                } else {
                    (*seq_params).subsampling_y = 0 as core::ffi::c_int;
                }
            } else {
                (*seq_params).subsampling_x = 1 as core::ffi::c_int;
                (*seq_params).subsampling_y = 0 as core::ffi::c_int;
            }
        }
        if (*seq_params).matrix_coefficients as core::ffi::c_uint
            == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
            && ((*seq_params).subsampling_x != 0 || (*seq_params).subsampling_y != 0)
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Identity CICP Matrix incompatible with non 4:4:4 color sampling\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if (*seq_params).subsampling_x != 0 && (*seq_params).subsampling_y != 0 {
            (*seq_params).chroma_sample_position = aom_rb_read_literal(
                rb,
                2 as core::ffi::c_int,
            ) as aom_chroma_sample_position_t;
        }
    }
    (*seq_params).separate_uv_delta_q = aom_rb_read_bit(rb) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_timing_info_header(
    mut timing_info: *mut aom_timing_info_t,
    mut error: *mut aom_internal_error_info,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*timing_info).num_units_in_display_tick = aom_rb_read_unsigned_literal(
        rb,
        32 as core::ffi::c_int,
    );
    (*timing_info).time_scale = aom_rb_read_unsigned_literal(rb, 32 as core::ffi::c_int);
    if (*timing_info).num_units_in_display_tick == 0 as uint32_t
        || (*timing_info).time_scale == 0 as uint32_t
    {
        aom_internal_error(
            error,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"num_units_in_display_tick and time_scale must be greater than 0.\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    (*timing_info).equal_picture_interval = aom_rb_read_bit(rb);
    if (*timing_info).equal_picture_interval != 0 {
        let num_ticks_per_picture_minus_1: uint32_t = aom_rb_read_uvlc(rb) as uint32_t;
        if num_ticks_per_picture_minus_1 == UINT32_MAX as uint32_t {
            aom_internal_error(
                error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"num_ticks_per_picture_minus_1 cannot be (1 << 32) - 1.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*timing_info).num_ticks_per_picture = num_ticks_per_picture_minus_1
            .wrapping_add(1 as uint32_t);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_decoder_model_info(
    mut decoder_model_info: *mut aom_dec_model_info_t,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*decoder_model_info).encoder_decoder_buffer_delay_length = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) + 1 as core::ffi::c_int;
    (*decoder_model_info).num_units_in_decoding_tick = aom_rb_read_unsigned_literal(
        rb,
        32 as core::ffi::c_int,
    );
    (*decoder_model_info).buffer_removal_time_length = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) + 1 as core::ffi::c_int;
    (*decoder_model_info).frame_presentation_time_length = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) + 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_op_parameters_info(
    mut op_params: *mut aom_dec_model_op_parameters_t,
    mut buffer_delay_length: core::ffi::c_int,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*op_params).decoder_buffer_delay = aom_rb_read_unsigned_literal(
        rb,
        buffer_delay_length,
    );
    (*op_params).encoder_buffer_delay = aom_rb_read_unsigned_literal(
        rb,
        buffer_delay_length,
    );
    (*op_params).low_delay_mode_flag = aom_rb_read_bit(rb);
}
#[inline]
unsafe extern "C-unwind" fn read_temporal_point_info(
    cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    (*cm).frame_presentation_time = aom_rb_read_unsigned_literal(
        rb,
        (*(*cm).seq_params).decoder_model_info.frame_presentation_time_length,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_sequence_header(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
    mut seq_params: *mut SequenceHeader,
) {
    let num_bits_width: core::ffi::c_int = aom_rb_read_literal(rb, 4 as core::ffi::c_int)
        as core::ffi::c_int + 1 as core::ffi::c_int;
    let num_bits_height: core::ffi::c_int = aom_rb_read_literal(
        rb,
        4 as core::ffi::c_int,
    ) as core::ffi::c_int + 1 as core::ffi::c_int;
    let max_frame_width: core::ffi::c_int = aom_rb_read_literal(rb, num_bits_width)
        as core::ffi::c_int + 1 as core::ffi::c_int;
    let max_frame_height: core::ffi::c_int = aom_rb_read_literal(rb, num_bits_height)
        as core::ffi::c_int + 1 as core::ffi::c_int;
    (*seq_params).num_bits_width = num_bits_width;
    (*seq_params).num_bits_height = num_bits_height;
    (*seq_params).max_frame_width = max_frame_width;
    (*seq_params).max_frame_height = max_frame_height;
    if (*seq_params).reduced_still_picture_hdr != 0 {
        (*seq_params).frame_id_numbers_present_flag = 0 as uint8_t;
    } else {
        (*seq_params).frame_id_numbers_present_flag = aom_rb_read_bit(rb) as uint8_t;
    }
    if (*seq_params).frame_id_numbers_present_flag != 0 {
        (*seq_params).delta_frame_id_length = aom_rb_read_literal(
            rb,
            4 as core::ffi::c_int,
        ) + 2 as core::ffi::c_int;
        (*seq_params).frame_id_length = aom_rb_read_literal(rb, 3 as core::ffi::c_int)
            + (*seq_params).delta_frame_id_length + 1 as core::ffi::c_int;
        if (*seq_params).frame_id_length > 16 as core::ffi::c_int {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_CORRUPT_FRAME,
                b"Invalid frame_id_length\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    setup_sb_size(seq_params, rb);
    (*seq_params).enable_filter_intra = aom_rb_read_bit(rb) as uint8_t;
    (*seq_params).enable_intra_edge_filter = aom_rb_read_bit(rb) as uint8_t;
    if (*seq_params).reduced_still_picture_hdr != 0 {
        (*seq_params).enable_interintra_compound = 0 as uint8_t;
        (*seq_params).enable_masked_compound = 0 as uint8_t;
        (*seq_params).enable_warped_motion = 0 as uint8_t;
        (*seq_params).enable_dual_filter = 0 as uint8_t;
        (*seq_params).order_hint_info.enable_order_hint = 0 as core::ffi::c_int;
        (*seq_params).order_hint_info.enable_dist_wtd_comp = 0 as core::ffi::c_int;
        (*seq_params).order_hint_info.enable_ref_frame_mvs = 0 as core::ffi::c_int;
        (*seq_params).force_screen_content_tools = 2 as uint8_t;
        (*seq_params).force_integer_mv = 2 as uint8_t;
        (*seq_params).order_hint_info.order_hint_bits_minus_1 = -(1 as core::ffi::c_int);
    } else {
        (*seq_params).enable_interintra_compound = aom_rb_read_bit(rb) as uint8_t;
        (*seq_params).enable_masked_compound = aom_rb_read_bit(rb) as uint8_t;
        (*seq_params).enable_warped_motion = aom_rb_read_bit(rb) as uint8_t;
        (*seq_params).enable_dual_filter = aom_rb_read_bit(rb) as uint8_t;
        (*seq_params).order_hint_info.enable_order_hint = aom_rb_read_bit(rb);
        (*seq_params).order_hint_info.enable_dist_wtd_comp = if (*seq_params)
            .order_hint_info
            .enable_order_hint != 0
        {
            aom_rb_read_bit(rb)
        } else {
            0 as core::ffi::c_int
        };
        (*seq_params).order_hint_info.enable_ref_frame_mvs = if (*seq_params)
            .order_hint_info
            .enable_order_hint != 0
        {
            aom_rb_read_bit(rb)
        } else {
            0 as core::ffi::c_int
        };
        if aom_rb_read_bit(rb) != 0 {
            (*seq_params).force_screen_content_tools = 2 as uint8_t;
        } else {
            (*seq_params).force_screen_content_tools = aom_rb_read_bit(rb) as uint8_t;
        }
        if (*seq_params).force_screen_content_tools as core::ffi::c_int
            > 0 as core::ffi::c_int
        {
            if aom_rb_read_bit(rb) != 0 {
                (*seq_params).force_integer_mv = 2 as uint8_t;
            } else {
                (*seq_params).force_integer_mv = aom_rb_read_bit(rb) as uint8_t;
            }
        } else {
            (*seq_params).force_integer_mv = 2 as uint8_t;
        }
        (*seq_params).order_hint_info.order_hint_bits_minus_1 = if (*seq_params)
            .order_hint_info
            .enable_order_hint != 0
        {
            aom_rb_read_literal(rb, 3 as core::ffi::c_int)
        } else {
            -(1 as core::ffi::c_int)
        };
    }
    (*seq_params).enable_superres = aom_rb_read_bit(rb) as uint8_t;
    (*seq_params).enable_cdef = aom_rb_read_bit(rb) as uint8_t;
    (*seq_params).enable_restoration = aom_rb_read_bit(rb) as uint8_t;
}
unsafe extern "C-unwind" fn read_global_motion_params(
    mut params: *mut WarpedMotionParams,
    mut ref_params: *const WarpedMotionParams,
    mut rb: *mut aom_read_bit_buffer,
    mut allow_hp: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut type_0: TransformationType = aom_rb_read_bit(rb) as TransformationType;
    if type_0 as core::ffi::c_int != IDENTITY as core::ffi::c_int {
        if aom_rb_read_bit(rb) != 0 {
            type_0 = ROTZOOM as core::ffi::c_int as TransformationType;
        } else {
            type_0 = (if aom_rb_read_bit(rb) != 0 {
                TRANSLATION as core::ffi::c_int
            } else {
                AFFINE as core::ffi::c_int
            }) as TransformationType;
        }
    }
    *params = default_warp_params;
    (*params).wmtype = type_0;
    if type_0 as core::ffi::c_int >= ROTZOOM as core::ffi::c_int {
        (*params).wmmat[2 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            (((*ref_params).wmmat[2 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
        ) as core::ffi::c_int * GM_ALPHA_DECODE_FACTOR
            + ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)) as int32_t;
        (*params).wmmat[3 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[3 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
        ) as core::ffi::c_int * GM_ALPHA_DECODE_FACTOR) as int32_t;
    }
    if type_0 as core::ffi::c_int >= AFFINE as core::ffi::c_int {
        (*params).wmmat[4 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[4 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
        ) as core::ffi::c_int * GM_ALPHA_DECODE_FACTOR) as int32_t;
        (*params).wmmat[5 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            (((*ref_params).wmmat[5 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
        ) as core::ffi::c_int * GM_ALPHA_DECODE_FACTOR
            + ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)) as int32_t;
    } else {
        (*params).wmmat[4 as core::ffi::c_int as usize] = -(*params)
            .wmmat[3 as core::ffi::c_int as usize];
        (*params).wmmat[5 as core::ffi::c_int as usize] = (*params)
            .wmmat[2 as core::ffi::c_int as usize];
    }
    if type_0 as core::ffi::c_int >= TRANSLATION as core::ffi::c_int {
        let trans_bits: core::ffi::c_int = if type_0 as core::ffi::c_int
            == TRANSLATION as core::ffi::c_int
        {
            GM_ABS_TRANS_ONLY_BITS - (allow_hp == 0) as core::ffi::c_int
        } else {
            GM_ABS_TRANS_BITS
        };
        let trans_dec_factor: core::ffi::c_int = if type_0 as core::ffi::c_int
            == TRANSLATION as core::ffi::c_int
        {
            GM_TRANS_ONLY_DECODE_FACTOR
                * ((1 as core::ffi::c_int) << (allow_hp == 0) as core::ffi::c_int)
        } else {
            GM_TRANS_DECODE_FACTOR
        };
        let trans_prec_diff: core::ffi::c_int = if type_0 as core::ffi::c_int
            == TRANSLATION as core::ffi::c_int
        {
            GM_TRANS_ONLY_PREC_DIFF + (allow_hp == 0) as core::ffi::c_int
        } else {
            GM_TRANS_PREC_DIFF
        };
        (*params).wmmat[0 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (((1 as core::ffi::c_int) << trans_bits) + 1 as core::ffi::c_int)
                as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[0 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
        ) as core::ffi::c_int * trans_dec_factor) as int32_t;
        (*params).wmmat[1 as core::ffi::c_int as usize] = (aom_rb_read_signed_primitive_refsubexpfin(
            rb,
            (((1 as core::ffi::c_int) << trans_bits) + 1 as core::ffi::c_int)
                as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[1 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
        ) as core::ffi::c_int * trans_dec_factor) as int32_t;
    }
    let mut good_shear_params: core::ffi::c_int = av1_get_shear_params(params);
    if good_shear_params == 0 {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn read_global_motion(
    mut cm: *mut AV1_COMMON,
    mut rb: *mut aom_read_bit_buffer,
) {
    let mut frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while frame <= ALTREF_FRAME as core::ffi::c_int {
        let mut ref_params: *const WarpedMotionParams = if !((*cm).prev_frame).is_null()
        {
            &raw mut *((*(*cm).prev_frame).global_motion).as_mut_ptr().offset(frame as isize)
                as *mut WarpedMotionParams as *const WarpedMotionParams
        } else {
            &raw const default_warp_params
        };
        let mut good_params: core::ffi::c_int = read_global_motion_params(
            &raw mut *((*cm).global_motion).as_mut_ptr().offset(frame as isize),
            ref_params,
            rb,
            (*cm).features.allow_high_precision_mv as core::ffi::c_int,
        );
        if good_params == 0 {
            (*cm).global_motion[frame as usize].invalid = 1 as int8_t;
        }
        frame += 1;
    }
    memcpy(
        ((*(*cm).cur_frame).global_motion).as_mut_ptr() as *mut core::ffi::c_void,
        ((*cm).global_motion).as_mut_ptr() as *const core::ffi::c_void,
        (REF_FRAMES as core::ffi::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<WarpedMotionParams>() as size_t),
    );
}
#[inline]
unsafe extern "C-unwind" fn reset_ref_frame_map(cm: *mut AV1_COMMON) {
    let pool: *mut BufferPool = (*cm).buffer_pool;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < REF_FRAMES as core::ffi::c_int {
        decrease_ref_count((*cm).ref_frame_map[i as usize], pool);
        (*cm).ref_frame_map[i as usize] = 0 as *mut RefCntBuffer;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn update_ref_frame_id(pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut refresh_frame_flags: core::ffi::c_int = (*cm)
        .current_frame
        .refresh_frame_flags;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < REF_FRAMES as core::ffi::c_int {
        if refresh_frame_flags >> i & 1 as core::ffi::c_int != 0 {
            (*cm).ref_frame_id[i as usize] = (*cm).current_frame_id;
            (*pbi).valid_for_referencing[i as usize] = 1 as core::ffi::c_int;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn show_existing_frame_reset(
    pbi: *mut AV1Decoder,
    mut existing_frame_idx: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    if ((*cm).show_existing_frame == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"show_existing_frame_reset\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            4524 as core::ffi::c_int,
            b"cm->show_existing_frame\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*cm).current_frame.frame_type = KEY_FRAME as core::ffi::c_int as FRAME_TYPE;
    (*cm).current_frame.refresh_frame_flags = ((1 as core::ffi::c_int)
        << REF_FRAMES as core::ffi::c_int) - 1 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
        (*cm).remapped_ref_idx[i as usize] = INVALID_IDX;
        i += 1;
    }
    if (*pbi).need_resync != 0 {
        reset_ref_frame_map(cm);
        (*pbi).need_resync = 0 as core::ffi::c_int;
    }
    (*cm).current_frame_id = (*cm).ref_frame_id[existing_frame_idx as usize];
    update_ref_frame_id(pbi);
    (*cm).features.refresh_frame_context = REFRESH_FRAME_CONTEXT_DISABLED
        as core::ffi::c_int as REFRESH_FRAME_CONTEXT_MODE;
}
#[inline]
unsafe extern "C-unwind" fn reset_frame_buffers(mut cm: *mut AV1_COMMON) {
    let frame_bufs: *mut RefCntBuffer = (*(*cm).buffer_pool).frame_bufs;
    let mut i: core::ffi::c_int = 0;
    lock_buffer_pool((*cm).buffer_pool);
    reset_ref_frame_map(cm);
    if !((*(*cm).cur_frame).ref_count == 1 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"reset_frame_buffers\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            4553 as core::ffi::c_int,
            b"cm->cur_frame->ref_count == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    i = 0 as core::ffi::c_int;
    while i < (*(*cm).buffer_pool).num_frame_bufs as core::ffi::c_int {
        if !((*frame_bufs.offset(i as isize)).ref_count > 0 as core::ffi::c_int
            && &raw mut *frame_bufs.offset(i as isize) as *mut RefCntBuffer
                != (*cm).cur_frame)
        {
            (*frame_bufs.offset(i as isize)).order_hint = 0 as core::ffi::c_uint;
            memset(
                &raw mut (*frame_bufs.offset(i as isize)).ref_order_hints
                    as *mut [core::ffi::c_uint; 7] as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[core::ffi::c_uint; 7]>() as size_t,
            );
        }
        i += 1;
    }
    av1_zero_unused_internal_frame_buffers(&raw mut (*(*cm).buffer_pool).int_frame_buffers);
    unlock_buffer_pool((*cm).buffer_pool);
}
unsafe extern "C-unwind" fn read_uncompressed_header(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).dcb.xd;
    let pool: *mut BufferPool = (*cm).buffer_pool;
    let frame_bufs: *mut RefCntBuffer = (*pool).frame_bufs;
    let mut sframe_info: *mut aom_s_frame_info = &raw mut (*pbi).sframe_info;
    (*sframe_info).is_s_frame = 0 as core::ffi::c_int;
    (*sframe_info).is_s_frame_at_altref = 0 as core::ffi::c_int;
    if (*pbi).sequence_header_ready == 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"No sequence header\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if (*seq_params).reduced_still_picture_hdr != 0 {
        (*cm).show_existing_frame = 0 as core::ffi::c_int;
        (*cm).show_frame = 1 as core::ffi::c_int;
        (*current_frame).frame_type = KEY_FRAME as core::ffi::c_int as FRAME_TYPE;
        if (*pbi).sequence_header_changed != 0 {
            (*pbi).sequence_header_changed = 0 as core::ffi::c_int;
            (*pbi).decoding_first_frame = 1 as core::ffi::c_int;
            reset_frame_buffers(cm);
        }
        (*features).error_resilient_mode = 1 as core::ffi::c_int != 0;
    } else {
        (*cm).show_existing_frame = aom_rb_read_bit(rb);
        (*pbi).reset_decoder_state = 0 as core::ffi::c_int;
        if (*cm).show_existing_frame != 0 {
            if (*pbi).sequence_header_changed != 0 {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_CORRUPT_FRAME,
                    b"New sequence header starts with a show_existing_frame.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            let existing_frame_idx: core::ffi::c_int = aom_rb_read_literal(
                rb,
                3 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let frame_to_show: *mut RefCntBuffer = (*cm)
                .ref_frame_map[existing_frame_idx as usize];
            if frame_to_show.is_null() {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_UNSUP_BITSTREAM,
                    b"Buffer does not contain a decoded frame\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if (*seq_params).decoder_model_info_present_flag as core::ffi::c_int != 0
                && (*seq_params).timing_info.equal_picture_interval
                    == 0 as core::ffi::c_int
            {
                read_temporal_point_info(cm, rb);
            }
            if (*seq_params).frame_id_numbers_present_flag != 0 {
                let mut frame_id_length: core::ffi::c_int = (*seq_params)
                    .frame_id_length;
                let mut display_frame_id: core::ffi::c_int = aom_rb_read_literal(
                    rb,
                    frame_id_length,
                );
                if display_frame_id != (*cm).ref_frame_id[existing_frame_idx as usize]
                    || (*pbi).valid_for_referencing[existing_frame_idx as usize]
                        == 0 as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Reference buffer frame ID mismatch\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            lock_buffer_pool(pool);
            if !((*frame_to_show).ref_count > 0 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"read_uncompressed_header\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    4630 as core::ffi::c_int,
                    b"frame_to_show->ref_count > 0\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if !((*(*cm).cur_frame).ref_count == 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"read_uncompressed_header\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    4634 as core::ffi::c_int,
                    b"cm->cur_frame->ref_count == 1\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if !((*(*cm).cur_frame).raw_frame_buffer.data).is_null() as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"read_uncompressed_header\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    4639 as core::ffi::c_int,
                    b"!cm->cur_frame->raw_frame_buffer.data\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            assign_frame_buffer_p(&raw mut (*cm).cur_frame, frame_to_show);
            (*pbi).reset_decoder_state = ((*frame_to_show).frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int) as core::ffi::c_int;
            unlock_buffer_pool(pool);
            (*cm).lf.filter_level[0 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int;
            (*cm).lf.filter_level[1 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int;
            (*cm).show_frame = 1 as core::ffi::c_int;
            (*current_frame).order_hint = (*frame_to_show).order_hint;
            if (*frame_to_show).showable_frame == 0 {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_UNSUP_BITSTREAM,
                    b"Buffer does not contain a showable frame\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if (*pbi).reset_decoder_state != 0 {
                (*frame_to_show).showable_frame = 0 as core::ffi::c_int;
            }
            (*cm).film_grain_params = (*frame_to_show).film_grain_params;
            if (*pbi).reset_decoder_state != 0 {
                show_existing_frame_reset(pbi, existing_frame_idx);
            } else {
                (*current_frame).refresh_frame_flags = 0 as core::ffi::c_int;
            }
            return 0 as core::ffi::c_int;
        }
        (*current_frame).frame_type = aom_rb_read_literal(rb, 2 as core::ffi::c_int)
            as FRAME_TYPE;
        if (*pbi).sequence_header_changed != 0 {
            if (*current_frame).frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int
            {
                (*pbi).sequence_header_changed = 0 as core::ffi::c_int;
                (*pbi).decoding_first_frame = 1 as core::ffi::c_int;
                reset_frame_buffers(cm);
            } else {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_CORRUPT_FRAME,
                    b"Sequence header has changed without a keyframe.\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        (*cm).show_frame = aom_rb_read_bit(rb);
        if (*cm).show_frame == 0 as core::ffi::c_int {
            (*pbi).is_arf_frame_present = 1 as core::ffi::c_int;
        }
        if (*cm).show_frame == 0 as core::ffi::c_int
            && (*cm).current_frame.frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int
        {
            (*pbi).is_fwd_kf_present = 1 as core::ffi::c_int;
        }
        if (*cm).current_frame.frame_type as core::ffi::c_int
            == S_FRAME as core::ffi::c_int
        {
            (*sframe_info).is_s_frame = 1 as core::ffi::c_int;
            (*sframe_info).is_s_frame_at_altref = if (*cm).show_frame != 0 {
                0 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            };
        }
        if (*seq_params).still_picture as core::ffi::c_int != 0
            && ((*current_frame).frame_type as core::ffi::c_int
                != KEY_FRAME as core::ffi::c_int || (*cm).show_frame == 0)
        {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Still pictures must be coded as shown keyframes\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*cm).showable_frame = ((*current_frame).frame_type as core::ffi::c_int
            != KEY_FRAME as core::ffi::c_int) as core::ffi::c_int;
        if (*cm).show_frame != 0 {
            if (*seq_params).decoder_model_info_present_flag as core::ffi::c_int != 0
                && (*seq_params).timing_info.equal_picture_interval
                    == 0 as core::ffi::c_int
            {
                read_temporal_point_info(cm, rb);
            }
        } else {
            (*cm).showable_frame = aom_rb_read_bit(rb);
        }
        (*(*cm).cur_frame).showable_frame = (*cm).showable_frame;
        (*features).error_resilient_mode = if frame_is_sframe(cm) != 0
            || (*current_frame).frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int && (*cm).show_frame != 0
        {
            1 as core::ffi::c_int
        } else {
            aom_rb_read_bit(rb)
        } != 0;
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        && (*cm).show_frame != 0
    {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < REF_FRAMES as core::ffi::c_int {
            (*pbi).valid_for_referencing[i as usize] = 0 as core::ffi::c_int;
            i += 1;
        }
    }
    (*features).disable_cdf_update = aom_rb_read_bit(rb) != 0;
    if (*seq_params).force_screen_content_tools as core::ffi::c_int
        == 2 as core::ffi::c_int
    {
        (*features).allow_screen_content_tools = aom_rb_read_bit(rb) != 0;
    } else {
        (*features).allow_screen_content_tools = (*seq_params).force_screen_content_tools
            != 0;
    }
    if (*features).allow_screen_content_tools {
        if (*seq_params).force_integer_mv as core::ffi::c_int == 2 as core::ffi::c_int {
            (*features).cur_frame_force_integer_mv = aom_rb_read_bit(rb) != 0;
        } else {
            (*features).cur_frame_force_integer_mv = (*seq_params).force_integer_mv != 0;
        }
    } else {
        (*features).cur_frame_force_integer_mv = 0 as core::ffi::c_int != 0;
    }
    let mut frame_size_override_flag: core::ffi::c_int = 0 as core::ffi::c_int;
    (*features).allow_intrabc = 0 as core::ffi::c_int != 0;
    (*features).primary_ref_frame = PRIMARY_REF_NONE;
    if (*seq_params).reduced_still_picture_hdr == 0 {
        if (*seq_params).frame_id_numbers_present_flag != 0 {
            let mut frame_id_length_0: core::ffi::c_int = (*seq_params).frame_id_length;
            let mut diff_len: core::ffi::c_int = (*seq_params).delta_frame_id_length;
            let mut prev_frame_id: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut have_prev_frame_id: core::ffi::c_int = ((*pbi).decoding_first_frame
                == 0
                && !((*current_frame).frame_type as core::ffi::c_int
                    == KEY_FRAME as core::ffi::c_int && (*cm).show_frame != 0))
                as core::ffi::c_int;
            if have_prev_frame_id != 0 {
                prev_frame_id = (*cm).current_frame_id;
            }
            (*cm).current_frame_id = aom_rb_read_literal(rb, frame_id_length_0);
            if have_prev_frame_id != 0 {
                let mut diff_frame_id: core::ffi::c_int = 0;
                if (*cm).current_frame_id > prev_frame_id {
                    diff_frame_id = (*cm).current_frame_id - prev_frame_id;
                } else {
                    diff_frame_id = ((1 as core::ffi::c_int) << frame_id_length_0)
                        + (*cm).current_frame_id - prev_frame_id;
                }
                if prev_frame_id == (*cm).current_frame_id
                    || diff_frame_id
                        >= (1 as core::ffi::c_int)
                            << frame_id_length_0 - 1 as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Invalid value of current_frame_id\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < REF_FRAMES as core::ffi::c_int {
                if (*cm).current_frame_id - ((1 as core::ffi::c_int) << diff_len)
                    > 0 as core::ffi::c_int
                {
                    if (*cm).ref_frame_id[i_0 as usize] > (*cm).current_frame_id
                        || (*cm).ref_frame_id[i_0 as usize]
                            < (*cm).current_frame_id
                                - ((1 as core::ffi::c_int) << diff_len)
                    {
                        (*pbi).valid_for_referencing[i_0 as usize] = 0
                            as core::ffi::c_int;
                    }
                } else if (*cm).ref_frame_id[i_0 as usize] > (*cm).current_frame_id
                    && (*cm).ref_frame_id[i_0 as usize]
                        < ((1 as core::ffi::c_int) << frame_id_length_0)
                            + (*cm).current_frame_id
                            - ((1 as core::ffi::c_int) << diff_len)
                {
                    (*pbi).valid_for_referencing[i_0 as usize] = 0 as core::ffi::c_int;
                }
                i_0 += 1;
            }
        }
        frame_size_override_flag = if frame_is_sframe(cm) != 0 {
            1 as core::ffi::c_int
        } else {
            aom_rb_read_bit(rb)
        };
        (*current_frame).order_hint = aom_rb_read_literal(
            rb,
            (*seq_params).order_hint_info.order_hint_bits_minus_1 + 1 as core::ffi::c_int,
        ) as core::ffi::c_uint;
        if (*seq_params).order_hint_info.enable_order_hint != 0 {
            (*current_frame).frame_number = (*current_frame).order_hint;
        }
        if !(*features).error_resilient_mode && frame_is_intra_only(cm) == 0 {
            (*features).primary_ref_frame = aom_rb_read_literal(rb, PRIMARY_REF_BITS);
        }
    }
    if (*seq_params).decoder_model_info_present_flag != 0 {
        (*pbi).buffer_removal_time_present = aom_rb_read_bit(rb) != 0;
        if (*pbi).buffer_removal_time_present {
            let mut op_num: core::ffi::c_int = 0 as core::ffi::c_int;
            while op_num
                < (*seq_params).operating_points_cnt_minus_1 + 1 as core::ffi::c_int
            {
                if (*seq_params)
                    .op_params[op_num as usize]
                    .decoder_model_param_present_flag != 0
                {
                    if (*seq_params).operating_point_idc[op_num as usize]
                        == 0 as core::ffi::c_int
                        || (*seq_params).operating_point_idc[op_num as usize]
                            >> (*cm).temporal_layer_id & 0x1 as core::ffi::c_int != 0
                            && (*seq_params).operating_point_idc[op_num as usize]
                                >> (*cm).spatial_layer_id + 8 as core::ffi::c_int
                                & 0x1 as core::ffi::c_int != 0
                    {
                        (*cm).buffer_removal_times[op_num as usize] = aom_rb_read_unsigned_literal(
                            rb,
                            (*seq_params).decoder_model_info.buffer_removal_time_length,
                        );
                    } else {
                        (*cm).buffer_removal_times[op_num as usize] = 0 as uint32_t;
                    }
                } else {
                    (*cm).buffer_removal_times[op_num as usize] = 0 as uint32_t;
                }
                op_num += 1;
            }
        }
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        if (*cm).show_frame == 0 {
            (*current_frame).refresh_frame_flags = aom_rb_read_literal(
                rb,
                REF_FRAMES as core::ffi::c_int,
            );
        } else {
            (*current_frame).refresh_frame_flags = ((1 as core::ffi::c_int)
                << REF_FRAMES as core::ffi::c_int) - 1 as core::ffi::c_int;
        }
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < INTER_REFS_PER_FRAME as core::ffi::c_int {
            (*cm).remapped_ref_idx[i_1 as usize] = INVALID_IDX;
            i_1 += 1;
        }
        if (*pbi).need_resync != 0 {
            reset_ref_frame_map(cm);
            (*pbi).need_resync = 0 as core::ffi::c_int;
        }
    } else if (*current_frame).frame_type as core::ffi::c_int
        == INTRA_ONLY_FRAME as core::ffi::c_int
    {
        (*current_frame).refresh_frame_flags = aom_rb_read_literal(
            rb,
            REF_FRAMES as core::ffi::c_int,
        );
        if (*current_frame).refresh_frame_flags == 0xff as core::ffi::c_int {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Intra only frames cannot have refresh flags 0xFF\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if (*pbi).need_resync != 0 {
            reset_ref_frame_map(cm);
            (*pbi).need_resync = 0 as core::ffi::c_int;
        }
    } else if (*pbi).need_resync != 1 as core::ffi::c_int {
        (*current_frame).refresh_frame_flags = if frame_is_sframe(cm) != 0 {
            0xff as core::ffi::c_int
        } else {
            aom_rb_read_literal(rb, REF_FRAMES as core::ffi::c_int)
        };
    }
    if frame_is_intra_only(cm) == 0
        || (*current_frame).refresh_frame_flags != 0xff as core::ffi::c_int
    {
        if (*features).error_resilient_mode as core::ffi::c_int != 0
            && (*seq_params).order_hint_info.enable_order_hint != 0
        {
            let mut ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while ref_idx < REF_FRAMES as core::ffi::c_int {
                let mut order_hint: core::ffi::c_uint = aom_rb_read_literal(
                    rb,
                    (*seq_params).order_hint_info.order_hint_bits_minus_1
                        + 1 as core::ffi::c_int,
                ) as core::ffi::c_uint;
                let mut buf: *mut RefCntBuffer = (*cm).ref_frame_map[ref_idx as usize];
                if buf.is_null() || order_hint != (*buf).order_hint {
                    if !buf.is_null() {
                        lock_buffer_pool(pool);
                        decrease_ref_count(buf, pool);
                        unlock_buffer_pool(pool);
                        (*cm).ref_frame_map[ref_idx as usize] = 0 as *mut RefCntBuffer;
                    }
                    let mut buf_idx: core::ffi::c_int = get_free_fb(cm);
                    if buf_idx == INVALID_IDX {
                        aom_internal_error(
                            &raw mut (*pbi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Unable to find free frame buffer\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    buf = &raw mut *frame_bufs.offset(buf_idx as isize) as *mut RefCntBuffer;
                    lock_buffer_pool(pool);
                    if aom_realloc_frame_buffer(
                        &raw mut (*buf).buf,
                        (*seq_params).max_frame_width,
                        (*seq_params).max_frame_height,
                        (*seq_params).subsampling_x,
                        (*seq_params).subsampling_y,
                        (*seq_params).use_highbitdepth as core::ffi::c_int,
                        AOM_BORDER_IN_PIXELS,
                        (*features).byte_alignment,
                        &raw mut (*buf).raw_frame_buffer,
                        (*pool).get_fb_cb,
                        (*pool).cb_priv,
                        false_0 != 0,
                        0 as core::ffi::c_int,
                    ) != 0
                    {
                        decrease_ref_count(buf, pool);
                        unlock_buffer_pool(pool);
                        aom_internal_error(
                            &raw mut (*pbi).error as *mut aom_internal_error_info,
                            AOM_CODEC_MEM_ERROR,
                            b"Failed to allocate frame buffer\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    unlock_buffer_pool(pool);
                    set_planes_to_neutral_grey(
                        seq_params,
                        &raw mut (*buf).buf,
                        0 as core::ffi::c_int,
                    );
                    (*pbi).valid_for_referencing[ref_idx as usize] = 1
                        as core::ffi::c_int;
                    (*cm).ref_frame_map[ref_idx as usize] = buf;
                    (*buf).order_hint = order_hint;
                }
                ref_idx += 1;
            }
        }
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        setup_frame_size(cm, frame_size_override_flag, (*pbi).frame_size_limit, rb);
        if (*features).allow_screen_content_tools as core::ffi::c_int != 0
            && av1_superres_scaled(cm) == 0
        {
            (*features).allow_intrabc = aom_rb_read_bit(rb) != 0;
        }
        (*features).allow_ref_frame_mvs = 0 as core::ffi::c_int != 0;
        (*cm).prev_frame = 0 as *mut RefCntBuffer;
    } else {
        (*features).allow_ref_frame_mvs = 0 as core::ffi::c_int != 0;
        if (*current_frame).frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int
        {
            (*(*cm).cur_frame).film_grain_params_present = (*seq_params)
                .film_grain_params_present;
            setup_frame_size(cm, frame_size_override_flag, (*pbi).frame_size_limit, rb);
            if (*features).allow_screen_content_tools as core::ffi::c_int != 0
                && av1_superres_scaled(cm) == 0
            {
                (*features).allow_intrabc = aom_rb_read_bit(rb) != 0;
            }
        } else if (*pbi).need_resync != 1 as core::ffi::c_int {
            let mut frame_refs_short_signaling: core::ffi::c_int = 0 as core::ffi::c_int;
            if (*seq_params).order_hint_info.enable_order_hint != 0 {
                frame_refs_short_signaling = aom_rb_read_bit(rb);
            }
            if frame_refs_short_signaling != 0 {
                let lst_ref: core::ffi::c_int = aom_rb_read_literal(rb, REF_FRAMES_LOG2)
                    as core::ffi::c_int;
                let lst_buf: *const RefCntBuffer = (*cm).ref_frame_map[lst_ref as usize];
                let gld_ref: core::ffi::c_int = aom_rb_read_literal(rb, REF_FRAMES_LOG2)
                    as core::ffi::c_int;
                let gld_buf: *const RefCntBuffer = (*cm).ref_frame_map[gld_ref as usize];
                if lst_buf.is_null() {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Inter frame requests nonexistent reference\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if gld_buf.is_null() {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Inter frame requests nonexistent reference\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                av1_set_frame_refs(
                    cm,
                    ((*cm).remapped_ref_idx).as_mut_ptr(),
                    lst_ref,
                    gld_ref,
                );
            }
            let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_2 < INTER_REFS_PER_FRAME as core::ffi::c_int {
                let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
                if frame_refs_short_signaling == 0 {
                    ref_0 = aom_rb_read_literal(rb, REF_FRAMES_LOG2);
                    if ((*cm).ref_frame_map[ref_0 as usize]).is_null() {
                        aom_internal_error(
                            &raw mut (*pbi).error as *mut aom_internal_error_info,
                            AOM_CODEC_CORRUPT_FRAME,
                            b"Inter frame requests nonexistent reference\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    (*cm).remapped_ref_idx[i_2 as usize] = ref_0;
                } else {
                    ref_0 = (*cm).remapped_ref_idx[i_2 as usize];
                }
                if (*pbi).valid_for_referencing[ref_0 as usize] == 0 as core::ffi::c_int
                {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_CORRUPT_FRAME,
                        b"Reference frame not valid for referencing\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                (*cm)
                    .ref_frame_sign_bias[(LAST_FRAME as core::ffi::c_int + i_2)
                    as usize] = 0 as core::ffi::c_int;
                if (*seq_params).frame_id_numbers_present_flag != 0 {
                    let mut frame_id_length_1: core::ffi::c_int = (*seq_params)
                        .frame_id_length;
                    let mut diff_len_0: core::ffi::c_int = (*seq_params)
                        .delta_frame_id_length;
                    let mut delta_frame_id_minus_1: core::ffi::c_int = aom_rb_read_literal(
                        rb,
                        diff_len_0,
                    );
                    let mut ref_frame_id: core::ffi::c_int = ((*cm).current_frame_id
                        - (delta_frame_id_minus_1 + 1 as core::ffi::c_int)
                        + ((1 as core::ffi::c_int) << frame_id_length_1))
                        % ((1 as core::ffi::c_int) << frame_id_length_1);
                    if ref_frame_id != (*cm).ref_frame_id[ref_0 as usize] {
                        aom_internal_error(
                            &raw mut (*pbi).error as *mut aom_internal_error_info,
                            AOM_CODEC_CORRUPT_FRAME,
                            b"Reference buffer frame ID mismatch\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                }
                i_2 += 1;
            }
            if !(*features).error_resilient_mode && frame_size_override_flag != 0 {
                setup_frame_size_with_refs(cm, (*pbi).frame_size_limit, rb);
            } else {
                setup_frame_size(
                    cm,
                    frame_size_override_flag,
                    (*pbi).frame_size_limit,
                    rb,
                );
            }
            if (*features).cur_frame_force_integer_mv {
                (*features).allow_high_precision_mv = 0 as core::ffi::c_int != 0;
            } else {
                (*features).allow_high_precision_mv = aom_rb_read_bit(rb) != 0;
            }
            (*features).interp_filter = read_frame_interp_filter(rb);
            (*features).switchable_motion_mode = aom_rb_read_bit(rb) != 0;
        }
        (*cm).prev_frame = get_primary_ref_frame_buf(cm);
        if (*features).primary_ref_frame != PRIMARY_REF_NONE
            && (get_primary_ref_frame_buf(cm)).is_null()
        {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"Reference frame containing this frame's initial frame context is unavailable.\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if !((*current_frame).frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int)
            && (*pbi).need_resync != 1 as core::ffi::c_int
        {
            if frame_might_allow_ref_frame_mvs(cm) != 0 {
                (*features).allow_ref_frame_mvs = aom_rb_read_bit(rb) != 0;
            } else {
                (*features).allow_ref_frame_mvs = 0 as core::ffi::c_int != 0;
            }
            let mut i_3: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
            while i_3 <= ALTREF_FRAME as core::ffi::c_int {
                let ref_buf: *const RefCntBuffer = get_ref_frame_buf(
                    cm,
                    i_3 as MV_REFERENCE_FRAME,
                );
                let ref_scale_factors: *mut scale_factors = get_ref_scale_factors(
                    cm,
                    i_3 as MV_REFERENCE_FRAME,
                ) as *mut scale_factors;
                av1_setup_scale_factors_for_frame(
                    ref_scale_factors,
                    (*ref_buf).buf.c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
                    (*ref_buf).buf.c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
                    (*cm).width,
                    (*cm).height,
                );
                if av1_is_valid_scale(ref_scale_factors) == 0 {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_UNSUP_BITSTREAM,
                        b"Reference frame has invalid dimensions\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                i_3 += 1;
            }
        }
    }
    av1_setup_frame_buf_refs(cm);
    av1_setup_frame_sign_bias(cm);
    (*(*cm).cur_frame).frame_type = (*current_frame).frame_type;
    update_ref_frame_id(pbi);
    let might_bwd_adapt: core::ffi::c_int = ((*seq_params).reduced_still_picture_hdr == 0
        && !(*features).disable_cdf_update) as core::ffi::c_int;
    if might_bwd_adapt != 0 {
        (*features).refresh_frame_context = (if aom_rb_read_bit(rb) != 0 {
            REFRESH_FRAME_CONTEXT_DISABLED as core::ffi::c_int
        } else {
            REFRESH_FRAME_CONTEXT_BACKWARD as core::ffi::c_int
        }) as REFRESH_FRAME_CONTEXT_MODE;
    } else {
        (*features).refresh_frame_context = REFRESH_FRAME_CONTEXT_DISABLED
            as core::ffi::c_int as REFRESH_FRAME_CONTEXT_MODE;
    }
    (*(*cm).cur_frame).buf.bit_depth = (*seq_params).bit_depth as core::ffi::c_uint;
    (*(*cm).cur_frame).buf.color_primaries = (*seq_params).color_primaries;
    (*(*cm).cur_frame).buf.transfer_characteristics = (*seq_params)
        .transfer_characteristics;
    (*(*cm).cur_frame).buf.matrix_coefficients = (*seq_params).matrix_coefficients;
    (*(*cm).cur_frame).buf.monochrome = (*seq_params).monochrome;
    (*(*cm).cur_frame).buf.chroma_sample_position = (*seq_params).chroma_sample_position;
    (*(*cm).cur_frame).buf.color_range = (*seq_params).color_range as aom_color_range_t;
    (*(*cm).cur_frame).buf.render_width = (*cm).render_width;
    (*(*cm).cur_frame).buf.render_height = (*cm).render_height;
    if (*pbi).need_resync != 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Keyframe / intra-only frame required to reset decoder state\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if (*features).allow_intrabc {
        let mut lf: *mut loopfilter = &raw mut (*cm).lf;
        (*lf).filter_level[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
        (*lf).filter_level[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
        (*cm).cdef_info.cdef_bits = 0 as core::ffi::c_int;
        (*cm).cdef_info.cdef_strengths[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
        (*cm).cdef_info.nb_cdef_strengths = 1 as core::ffi::c_int;
        (*cm).cdef_info.cdef_uv_strengths[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
        (*cm).rst_info[0 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
        (*cm).rst_info[1 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
        (*cm).rst_info[2 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
    }
    read_tile_info(pbi, rb);
    if av1_is_min_tile_width_satisfied(cm) == 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Minimum tile width requirement not satisfied\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    let quant_params: *mut CommonQuantParams = &raw mut (*cm).quant_params;
    setup_quantization(
        quant_params,
        av1_num_planes(cm),
        (*(*cm).seq_params).separate_uv_delta_q != 0,
        rb,
    );
    (*xd).bd = (*seq_params).bit_depth as core::ffi::c_int;
    let above_contexts: *mut CommonContexts = &raw mut (*cm).above_contexts;
    if (*above_contexts).num_planes < av1_num_planes(cm)
        || (*above_contexts).num_mi_cols < (*cm).mi_params.mi_cols
        || (*above_contexts).num_tile_rows < (*cm).tiles.rows
    {
        av1_free_above_context_buffers(above_contexts as *mut CommonContexts);
        if av1_alloc_above_context_buffers(
            above_contexts as *mut CommonContexts,
            (*cm).tiles.rows,
            (*cm).mi_params.mi_cols,
            av1_num_planes(cm),
        ) != 0
        {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate context buffers\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    if (*features).primary_ref_frame == PRIMARY_REF_NONE {
        av1_setup_past_independence(cm as *mut AV1Common);
    }
    setup_segmentation(cm, rb);
    (*cm).delta_q_info.delta_q_res = 1 as core::ffi::c_int;
    (*cm).delta_q_info.delta_lf_res = 1 as core::ffi::c_int;
    (*cm).delta_q_info.delta_lf_present_flag = 0 as core::ffi::c_int;
    (*cm).delta_q_info.delta_lf_multi = 0 as core::ffi::c_int;
    (*cm).delta_q_info.delta_q_present_flag = if (*quant_params).base_qindex
        > 0 as core::ffi::c_int
    {
        aom_rb_read_bit(rb)
    } else {
        0 as core::ffi::c_int
    };
    if (*cm).delta_q_info.delta_q_present_flag != 0 {
        (*xd).current_base_qindex = (*quant_params).base_qindex;
        (*cm).delta_q_info.delta_q_res = (1 as core::ffi::c_int)
            << aom_rb_read_literal(rb, 2 as core::ffi::c_int);
        if !(*features).allow_intrabc {
            (*cm).delta_q_info.delta_lf_present_flag = aom_rb_read_bit(rb);
        }
        if (*cm).delta_q_info.delta_lf_present_flag != 0 {
            (*cm).delta_q_info.delta_lf_res = (1 as core::ffi::c_int)
                << aom_rb_read_literal(rb, 2 as core::ffi::c_int);
            (*cm).delta_q_info.delta_lf_multi = aom_rb_read_bit(rb);
            av1_reset_loop_filter_delta(xd, av1_num_planes(cm));
        }
    }
    (*xd).cur_frame_force_integer_mv = (*features).cur_frame_force_integer_mv
        as core::ffi::c_int;
    let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_4 < MAX_SEGMENTS {
        let qindex: core::ffi::c_int = av1_get_qindex(
            &raw mut (*cm).seg,
            i_4,
            (*quant_params).base_qindex,
        ) as core::ffi::c_int;
        (*xd).lossless[i_4 as usize] = (qindex == 0 as core::ffi::c_int
            && (*quant_params).y_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).u_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).u_ac_delta_q == 0 as core::ffi::c_int
            && (*quant_params).v_dc_delta_q == 0 as core::ffi::c_int
            && (*quant_params).v_ac_delta_q == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        (*xd).qindex[i_4 as usize] = qindex;
        i_4 += 1;
    }
    (*features).coded_lossless = is_coded_lossless(cm, xd) != 0;
    (*features).all_lossless = (*features).coded_lossless as core::ffi::c_int != 0
        && av1_superres_scaled(cm) == 0;
    setup_segmentation_dequant(cm, xd);
    if (*features).coded_lossless {
        (*cm).lf.filter_level[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
        (*cm).lf.filter_level[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
    }
    if (*features).coded_lossless as core::ffi::c_int != 0
        || (*seq_params).enable_cdef == 0
    {
        (*cm).cdef_info.cdef_bits = 0 as core::ffi::c_int;
        (*cm).cdef_info.cdef_strengths[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
        (*cm).cdef_info.cdef_uv_strengths[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
    }
    if (*features).all_lossless as core::ffi::c_int != 0
        || (*seq_params).enable_restoration == 0
    {
        (*cm).rst_info[0 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
        (*cm).rst_info[1 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
        (*cm).rst_info[2 as core::ffi::c_int as usize].frame_restoration_type = RESTORE_NONE;
    }
    setup_loopfilter(cm, rb);
    if !(*features).coded_lossless && (*seq_params).enable_cdef as core::ffi::c_int != 0
    {
        setup_cdef(cm, rb);
    }
    if !(*features).all_lossless
        && (*seq_params).enable_restoration as core::ffi::c_int != 0
    {
        decode_restoration_mode(cm, rb);
    }
    (*features).tx_mode = read_tx_mode(
        rb,
        (*features).coded_lossless as core::ffi::c_int,
    );
    (*current_frame).reference_mode = read_frame_reference_mode(cm, rb);
    av1_setup_skip_mode_allowed(cm);
    (*current_frame).skip_mode_info.skip_mode_flag = if (*current_frame)
        .skip_mode_info
        .skip_mode_allowed != 0
    {
        aom_rb_read_bit(rb)
    } else {
        0 as core::ffi::c_int
    };
    if frame_might_allow_warped_motion(cm) != 0 {
        (*features).allow_warped_motion = aom_rb_read_bit(rb) != 0;
    } else {
        (*features).allow_warped_motion = 0 as core::ffi::c_int != 0;
    }
    (*features).reduced_tx_set_used = aom_rb_read_bit(rb) != 0;
    if (*features).allow_ref_frame_mvs as core::ffi::c_int != 0
        && frame_might_allow_ref_frame_mvs(cm) == 0
    {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Frame wrongly requests reference frame MVs\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if frame_is_intra_only(cm) == 0 {
        read_global_motion(cm, rb);
    }
    (*(*cm).cur_frame).film_grain_params_present = (*seq_params)
        .film_grain_params_present;
    read_film_grain(cm, rb);
    if (*pbi).ext_tile_debug != 0 && (*cm).tiles.large_scale != 0 {
        read_ext_tile_info(pbi, rb);
        av1_set_single_tile_decoding_mode(cm);
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_read_bit_buffer(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
) -> *mut aom_read_bit_buffer {
    (*rb).bit_offset = 0 as uint32_t;
    (*rb).error_handler = Some(
        error_handler as unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> (),
    ) as aom_rb_error_handler;
    (*rb).error_handler_data = &raw mut (*pbi).common as *mut AV1_COMMON
        as *mut core::ffi::c_void;
    (*rb).bit_buffer = data;
    (*rb).bit_buffer_end = data_end;
    return rb;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_read_profile(
    mut rb: *mut aom_read_bit_buffer,
) -> BITSTREAM_PROFILE {
    let mut profile: core::ffi::c_int = aom_rb_read_literal(rb, PROFILE_BITS);
    return profile as BITSTREAM_PROFILE;
}
#[inline]
unsafe extern "C-unwind" fn superres_post_decode(mut pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let pool: *mut BufferPool = (*cm).buffer_pool;
    if av1_superres_scaled(cm) == 0 {
        return;
    }
    if (*cm).features.all_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"superres_post_decode\0"))
                .as_ptr(),
            b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
            5253 as core::ffi::c_int,
            b"!cm->features.all_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_superres_upscale(cm, pool, 0 as core::ffi::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_decode_frame_headers_and_setup(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut trailing_bits_present: core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).dcb.xd;
    let mut i: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while i <= ALTREF_FRAME as core::ffi::c_int {
        (*cm).global_motion[i as usize] = default_warp_params;
        (*(*cm).cur_frame).global_motion[i as usize] = default_warp_params;
        i += 1;
    }
    (*xd).global_motion = ((*cm).global_motion).as_mut_ptr();
    read_uncompressed_header(pbi, rb);
    if trailing_bits_present != 0 {
        av1_check_trailing_bits(pbi, rb);
    }
    if (*cm).tiles.single_tile_decoding == 0
        && ((*pbi).dec_tile_row >= 0 as core::ffi::c_int
            || (*pbi).dec_tile_col >= 0 as core::ffi::c_int)
    {
        (*pbi).dec_tile_row = -(1 as core::ffi::c_int);
        (*pbi).dec_tile_col = -(1 as core::ffi::c_int);
    }
    let uncomp_hdr_size: uint32_t = aom_rb_bytes_read(rb) as uint32_t;
    let mut new_fb: *mut YV12_BUFFER_CONFIG = &raw mut (*(*cm).cur_frame).buf;
    (*xd).cur_buf = new_fb;
    if av1_allow_intrabc(cm) != 0 {
        av1_setup_scale_factors_for_frame(
            &raw mut (*cm).sf_identity,
            (*(*xd).cur_buf).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
            (*(*xd).cur_buf).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
            (*(*xd).cur_buf).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
            (*(*xd).cur_buf).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
        );
    }
    if (*cm).show_existing_frame != 0 {
        if (*pbi).reset_decoder_state != 0 {
            *(*cm).fc = *(*cm).default_frame_context;
            if (*(*cm).fc).initialized == 0 {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_CORRUPT_FRAME,
                    b"Uninitialized entropy context.\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        return uncomp_hdr_size;
    }
    ((*cm).mi_params.setup_mi).expect("non-null function pointer")(&raw mut (*cm).mi_params);
    av1_calculate_ref_frame_side(cm);
    if (*cm).features.allow_ref_frame_mvs {
        av1_setup_motion_field(cm);
    }
    av1_setup_block_planes(
        xd,
        (*(*cm).seq_params).subsampling_x,
        (*(*cm).seq_params).subsampling_y,
        num_planes,
    );
    if (*cm).features.primary_ref_frame == PRIMARY_REF_NONE {
        *(*cm).fc = *(*cm).default_frame_context;
    } else {
        *(*cm).fc = (*get_primary_ref_frame_buf(cm)).frame_context;
    }
    if (*(*cm).fc).initialized == 0 {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Uninitialized entropy context.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    (*pbi).dcb.corrupted = 0 as core::ffi::c_int;
    return uncomp_hdr_size;
}
#[inline]
unsafe extern "C-unwind" fn setup_frame_info(mut pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    if (*cm).rst_info[0 as core::ffi::c_int as usize].frame_restoration_type
        as core::ffi::c_uint != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        || (*cm).rst_info[1 as core::ffi::c_int as usize].frame_restoration_type
            as core::ffi::c_uint != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        || (*cm).rst_info[2 as core::ffi::c_int as usize].frame_restoration_type
            as core::ffi::c_uint != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        av1_alloc_restoration_buffers(cm as *mut AV1Common, true_0 != 0);
        let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
        while p < av1_num_planes(cm) {
            av1_alloc_restoration_struct(
                cm as *mut AV1Common,
                &raw mut *((*cm).rst_info).as_mut_ptr().offset(p as isize),
                (p > 0 as core::ffi::c_int) as core::ffi::c_int,
            );
            p += 1;
        }
    }
    let use_highbd: core::ffi::c_int = (*(*cm).seq_params).use_highbitdepth
        as core::ffi::c_int;
    let buf_size: core::ffi::c_int = MC_TEMP_BUF_PELS << use_highbd;
    if (*pbi).td.mc_buf_size != buf_size as int32_t {
        av1_free_mc_tmp_buf(&raw mut (*pbi).td);
        allocate_mc_tmp_buf(cm, &raw mut (*pbi).td, buf_size, use_highbd);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_decode_tg_tiles_and_wrapup(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut p_data_end: *mut *const uint8_t,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
    mut initialize_flag: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let xd: *mut MACROBLOCKD = &raw mut (*pbi).dcb.xd;
    let tile_count_tg: core::ffi::c_int = end_tile - start_tile + 1 as core::ffi::c_int;
    (*xd).error_info = (*cm).error;
    if initialize_flag != 0 {
        setup_frame_info(pbi);
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if (*pbi).max_threads > 1 as core::ffi::c_int
        && !((*tiles).large_scale != 0 && (*pbi).ext_tile_debug == 0)
        && (*pbi).row_mt != 0
    {
        *p_data_end = decode_tiles_row_mt(pbi, data, data_end, start_tile, end_tile);
    } else if (*pbi).max_threads > 1 as core::ffi::c_int
        && tile_count_tg > 1 as core::ffi::c_int
        && !((*tiles).large_scale != 0 && (*pbi).ext_tile_debug == 0)
    {
        *p_data_end = decode_tiles_mt(pbi, data, data_end, start_tile, end_tile);
    } else {
        *p_data_end = decode_tiles(pbi, data, data_end, start_tile, end_tile);
    }
    if num_planes < 3 as core::ffi::c_int {
        set_planes_to_neutral_grey(
            (*cm).seq_params,
            (*xd).cur_buf,
            1 as core::ffi::c_int,
        );
    }
    if end_tile != (*tiles).rows * (*tiles).cols - 1 as core::ffi::c_int {
        return;
    }
    av1_alloc_cdef_buffers(
        cm as *mut AV1Common,
        &raw mut (*pbi).cdef_worker,
        &raw mut (*pbi).cdef_sync,
        (*pbi).num_workers,
        1 as core::ffi::c_int,
    );
    av1_alloc_cdef_sync(cm, &raw mut (*pbi).cdef_sync, (*pbi).num_workers);
    if !(*cm).features.allow_intrabc && (*tiles).single_tile_decoding == 0 {
        if (*cm).lf.filter_level[0 as core::ffi::c_int as usize] != 0
            || (*cm).lf.filter_level[1 as core::ffi::c_int as usize] != 0
        {
            av1_loop_filter_frame_mt(
                &raw mut (*(*cm).cur_frame).buf,
                cm as *mut AV1Common,
                &raw mut (*pbi).dcb.xd,
                0 as core::ffi::c_int,
                num_planes,
                0 as core::ffi::c_int,
                (*pbi).tile_workers,
                (*pbi).num_workers,
                &raw mut (*pbi).lf_row_sync,
                0 as core::ffi::c_int,
            );
        }
        let do_cdef: core::ffi::c_int = ((*pbi).skip_loop_filter == 0
            && !(*cm).features.coded_lossless
            && ((*cm).cdef_info.cdef_bits != 0
                || (*cm).cdef_info.cdef_strengths[0 as core::ffi::c_int as usize] != 0
                || (*cm).cdef_info.cdef_uv_strengths[0 as core::ffi::c_int as usize]
                    != 0)) as core::ffi::c_int;
        let do_superres: core::ffi::c_int = av1_superres_scaled(cm) as core::ffi::c_int;
        let optimized_loop_restoration: core::ffi::c_int = (do_cdef == 0
            && do_superres == 0) as core::ffi::c_int;
        let do_loop_restoration: core::ffi::c_int = ((*cm)
            .rst_info[0 as core::ffi::c_int as usize]
            .frame_restoration_type as core::ffi::c_uint
            != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
            || (*cm).rst_info[1 as core::ffi::c_int as usize].frame_restoration_type
                as core::ffi::c_uint
                != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
            || (*cm).rst_info[2 as core::ffi::c_int as usize].frame_restoration_type
                as core::ffi::c_uint
                != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let mut do_extend_border_mt: core::ffi::c_int = 0 as core::ffi::c_int;
        if optimized_loop_restoration == 0 {
            if do_loop_restoration != 0 {
                av1_loop_restoration_save_boundary_lines(
                    &raw mut (*(*pbi).common.cur_frame).buf,
                    cm as *mut AV1Common,
                    0 as core::ffi::c_int,
                );
            }
            if do_cdef != 0 {
                if (*pbi).num_workers > 1 as core::ffi::c_int {
                    av1_cdef_frame_mt(
                        cm,
                        &raw mut (*pbi).dcb.xd,
                        (*pbi).cdef_worker,
                        (*pbi).tile_workers,
                        &raw mut (*pbi).cdef_sync,
                        (*pbi).num_workers,
                        Some(
                            av1_cdef_init_fb_row_mt
                                as unsafe extern "C-unwind" fn(
                                    *const AV1_COMMON,
                                    *const MACROBLOCKD,
                                    *mut CdefBlockInfo,
                                    *mut *mut uint16_t,
                                    *mut uint16_t,
                                    *mut AV1CdefSyncData,
                                    core::ffi::c_int,
                                ) -> (),
                        ),
                        do_extend_border_mt,
                    );
                } else {
                    av1_cdef_frame(
                        &raw mut (*(*pbi).common.cur_frame).buf,
                        cm,
                        &raw mut (*pbi).dcb.xd,
                        Some(
                            av1_cdef_init_fb_row
                                as unsafe extern "C-unwind" fn(
                                    *const AV1_COMMON,
                                    *const MACROBLOCKD,
                                    *mut CdefBlockInfo,
                                    *mut *mut uint16_t,
                                    *mut uint16_t,
                                    *mut AV1CdefSyncData,
                                    core::ffi::c_int,
                                ) -> (),
                        ),
                    );
                }
            }
            superres_post_decode(pbi);
            if do_loop_restoration != 0 {
                av1_loop_restoration_save_boundary_lines(
                    &raw mut (*(*pbi).common.cur_frame).buf,
                    cm as *mut AV1Common,
                    1 as core::ffi::c_int,
                );
                if (*pbi).num_workers > 1 as core::ffi::c_int {
                    av1_loop_restoration_filter_frame_mt(
                        (*xd).cur_buf as *mut YV12_BUFFER_CONFIG,
                        cm as *mut AV1Common,
                        optimized_loop_restoration,
                        (*pbi).tile_workers,
                        (*pbi).num_workers,
                        &raw mut (*pbi).lr_row_sync,
                        &raw mut (*pbi).lr_ctxt as *mut AV1LrStruct
                            as *mut core::ffi::c_void,
                        do_extend_border_mt,
                    );
                } else {
                    av1_loop_restoration_filter_frame(
                        (*xd).cur_buf as *mut YV12_BUFFER_CONFIG,
                        cm as *mut AV1Common,
                        optimized_loop_restoration,
                        &raw mut (*pbi).lr_ctxt as *mut AV1LrStruct as *mut core::ffi::c_void,
                    );
                }
            }
        } else if do_loop_restoration != 0 {
            if (*pbi).num_workers > 1 as core::ffi::c_int {
                av1_loop_restoration_filter_frame_mt(
                    (*xd).cur_buf as *mut YV12_BUFFER_CONFIG,
                    cm as *mut AV1Common,
                    optimized_loop_restoration,
                    (*pbi).tile_workers,
                    (*pbi).num_workers,
                    &raw mut (*pbi).lr_row_sync,
                    &raw mut (*pbi).lr_ctxt as *mut AV1LrStruct as *mut core::ffi::c_void,
                    do_extend_border_mt,
                );
            } else {
                av1_loop_restoration_filter_frame(
                    (*xd).cur_buf as *mut YV12_BUFFER_CONFIG,
                    cm as *mut AV1Common,
                    optimized_loop_restoration,
                    &raw mut (*pbi).lr_ctxt as *mut AV1LrStruct as *mut core::ffi::c_void,
                );
            }
        }
    }
    if (*pbi).dcb.corrupted == 0 {
        if (*cm).features.refresh_frame_context as core::ffi::c_int
            == REFRESH_FRAME_CONTEXT_BACKWARD as core::ffi::c_int
        {
            if !((*pbi).context_update_tile_id < (*pbi).allocated_tiles)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_decode_tg_tiles_and_wrapup\0"))
                        .as_ptr(),
                    b"decodeframe.c\0" as *const u8 as *const core::ffi::c_char,
                    5490 as core::ffi::c_int,
                    b"pbi->context_update_tile_id < pbi->allocated_tiles\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            *(*cm).fc = (*((*pbi).tile_data)
                .offset((*pbi).context_update_tile_id as isize))
                .tctx;
            av1_reset_cdf_symbol_counters((*cm).fc as *mut frame_contexts);
        }
    } else {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Decode failed. Frame data is corrupted.\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if (*tiles).large_scale == 0 {
        (*(*cm).cur_frame).frame_context = *(*cm).fc;
    }
    if (*cm).show_frame != 0
        && (*(*cm).seq_params).order_hint_info.enable_order_hint == 0
    {
        (*cm).current_frame.frame_number = ((*cm).current_frame.frame_number)
            .wrapping_add(1);
    }
}

extern "C" {
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
}

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_get_partition_subsize(
    bsize: BLOCK_SIZE,
    partition: PARTITION_TYPE,
) -> BLOCK_SIZE {
    get_partition_subsize(bsize, partition)
}
