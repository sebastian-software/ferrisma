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

// Mechanically translated from upstream/aom/av1/encoder/nonrd_pickmode.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_setup_scale_factors_for_frame(
        sf: *mut scale_factors,
        other_w: core::ffi::c_int,
        other_h: core::ffi::c_int,
        this_w: core::ffi::c_int,
        this_h: core::ffi::c_int,
    );
    fn av1_left_block_mode(left_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_above_block_mode(above_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
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
    fn av1_selectSamples(
        mv: *mut MV,
        pts: *mut core::ffi::c_int,
        pts_inref: *mut core::ffi::c_int,
        len: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    ) -> uint8_t;
    fn av1_findSamples(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        pts: *mut core::ffi::c_int,
        pts_inref: *mut core::ffi::c_int,
    ) -> uint8_t;
    fn av1_find_projection(
        np: core::ffi::c_int,
        pts1: *const core::ffi::c_int,
        pts2: *const core::ffi::c_int,
        bsize: BLOCK_SIZE,
        mvy: core::ffi::c_int,
        mvx: core::ffi::c_int,
        wm_params: *mut WarpedMotionParams,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_count_overlappable_neighbors(cm: *const AV1_COMMON, xd: *mut MACROBLOCKD);
    fn av1_model_rd_curvfit(
        bsize: BLOCK_SIZE,
        sse_norm: core::ffi::c_double,
        xqr: core::ffi::c_double,
        rate_dist_f: *mut core::ffi::c_double,
    );
    fn av1_get_switchable_rate(
        x: *const MACROBLOCK,
        xd: *const MACROBLOCKD,
        interp_filter: InterpFilter,
        dual_filter: core::ffi::c_int,
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
    fn av1_mv_bit_cost(
        mv: *const MV,
        ref_mv: *const MV,
        mvjcost: *const core::ffi::c_int,
        mvcost: *const *mut core::ffi::c_int,
        weight: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_make_default_fullpel_ms_params(
        ms_params: *mut FULLPEL_MOTION_SEARCH_PARAMS,
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        ref_mv: *const MV,
        start_mv: FULLPEL_MV,
        search_sites: *const search_site_config,
        search_method: SEARCH_METHODS,
        fine_search_interval: core::ffi::c_int,
    );
    fn av1_set_ms_to_intra_mode(
        ms_params: *mut FULLPEL_MOTION_SEARCH_PARAMS,
        dv_costs: *const IntraBCMVCosts,
    );
    static av1_init_motion_compensation: [av1_init_search_site_config; 6];
    fn av1_int_pro_motion_estimation(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        ref_mv: *const MV,
        y_sad_zero: *mut core::ffi::c_uint,
        me_search_size_col: core::ffi::c_int,
        me_search_size_row: core::ffi::c_int,
        is_var_part: core::ffi::c_int,
        use_larger_search: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn av1_full_pixel_search(
        start_mv: FULLPEL_MV,
        ms_params: *const FULLPEL_MOTION_SEARCH_PARAMS,
        step_param: core::ffi::c_int,
        cost_list: *mut core::ffi::c_int,
        best_mv: *mut FULLPEL_MV,
        best_mv_stats: *mut FULLPEL_MV_STATS,
        second_best_mv: *mut FULLPEL_MV,
    ) -> core::ffi::c_int;
    fn av1_intrabc_hash_search(
        cpi: *const AV1_COMP,
        xd: *const MACROBLOCKD,
        ms_params: *const FULLPEL_MOTION_SEARCH_PARAMS,
        intrabc_hash_info: *mut IntraBCHashInfo,
        best_mv: *mut FULLPEL_MV,
    ) -> core::ffi::c_int;
    fn av1_make_default_subpel_ms_params(
        ms_params: *mut SUBPEL_MOTION_SEARCH_PARAMS,
        cpi: *const AV1_COMP,
        x: *const MACROBLOCK,
        bsize: BLOCK_SIZE,
        ref_mv: *const MV,
        cost_list: *const core::ffi::c_int,
    );
    fn av1_find_best_sub_pixel_tree_pruned_more(
        _: *mut MACROBLOCKD,
        _: *const AV1_COMMON,
        _: *const SUBPEL_MOTION_SEARCH_PARAMS,
        _: MV,
        _: *const FULLPEL_MV_STATS,
        _: *mut MV,
        _: *mut core::ffi::c_int,
        _: *mut core::ffi::c_uint,
        _: *mut int_mv,
    ) -> core::ffi::c_int;
    fn av1_refine_warped_mv(
        xd: *mut MACROBLOCKD,
        cm: *const AV1_COMMON,
        ms_params: *const SUBPEL_MOTION_SEARCH_PARAMS,
        bsize: BLOCK_SIZE,
        pts0: *const core::ffi::c_int,
        pts_inref0: *const core::ffi::c_int,
        total_samples: core::ffi::c_int,
        search_method: WARP_SEARCH_METHOD,
        num_iterations: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn av1_foreach_transformed_block_in_plane(
        xd: *const MACROBLOCKD,
        plane_bsize: BLOCK_SIZE,
        plane: core::ffi::c_int,
        visit: foreach_transformed_block_visitor,
        arg: *mut core::ffi::c_void,
    );
    fn av1_noise_estimate_extract_level(ne: *mut NOISE_ESTIMATE) -> NOISE_LEVEL;
    fn av1_get_ref_mv(x: *const MACROBLOCK, ref_idx: core::ffi::c_int) -> int_mv;
    fn av1_find_best_ref_mvs_from_stack(
        allow_hp: core::ffi::c_int,
        mbmi_ext: *const MB_MODE_INFO_EXT,
        ref_frame: MV_REFERENCE_FRAME,
        nearest_mv: *mut int_mv,
        near_mv: *mut int_mv,
        is_integer: core::ffi::c_int,
    );
    fn av1_search_palette_mode_luma(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        ref_frame_cost: core::ffi::c_uint,
        ctx: *mut PICK_MODE_CONTEXT,
        this_rd_cost: *mut RD_STATS,
        best_rd: int64_t,
    );
    fn av1_get_intra_inter_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn av1_get_comp_reference_type_context(xd: *const MACROBLOCKD) -> core::ffi::c_int;
    fn aom_convolve_copy_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn aom_get_var_sse_sum_16x16_dual_c(
        src_ptr: *const uint8_t,
        source_stride: core::ffi::c_int,
        ref_ptr: *const uint8_t,
        ref_stride: core::ffi::c_int,
        sse16x16: *mut uint32_t,
        tot_sse: *mut core::ffi::c_uint,
        tot_sum: *mut core::ffi::c_int,
        var16x16: *mut uint32_t,
    );
    fn aom_get_var_sse_sum_8x8_quad_c(
        src_ptr: *const uint8_t,
        source_stride: core::ffi::c_int,
        ref_ptr: *const uint8_t,
        ref_stride: core::ffi::c_int,
        sse8x8: *mut uint32_t,
        sum8x8: *mut core::ffi::c_int,
        tot_sse: *mut core::ffi::c_uint,
        tot_sum: *mut core::ffi::c_int,
        var8x8: *mut uint32_t,
    );
    fn av1_block_yrd(
        x: *mut MACROBLOCK,
        this_rdc: *mut RD_STATS,
        skippable: *mut core::ffi::c_int,
        bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
    );
    fn av1_block_yrd_idtx(
        x: *mut MACROBLOCK,
        pred_buf: *const uint8_t,
        pred_stride: core::ffi::c_int,
        this_rdc: *mut RD_STATS,
        skippable: *mut core::ffi::c_int,
        bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
    );
    fn av1_model_rd_for_sb_uv(
        cpi: *mut AV1_COMP,
        plane_bsize: BLOCK_SIZE,
        x: *mut MACROBLOCK,
        xd: *mut MACROBLOCKD,
        this_rdc: *mut RD_STATS,
        start_plane: core::ffi::c_int,
        stop_plane: core::ffi::c_int,
    ) -> int64_t;
    fn av1_estimate_block_intra(
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        row: core::ffi::c_int,
        col: core::ffi::c_int,
        plane_bsize: BLOCK_SIZE,
        tx_size: TX_SIZE,
        arg: *mut core::ffi::c_void,
    );
    fn av1_estimate_intra_mode(
        cpi: *mut AV1_COMP,
        x: *mut MACROBLOCK,
        bsize: BLOCK_SIZE,
        best_early_term: core::ffi::c_int,
        ref_cost_intra: core::ffi::c_uint,
        reuse_prediction: core::ffi::c_int,
        orig_dst: *mut buf_2d,
        tmp_buffers: *mut PRED_BUFFER,
        this_mode_pred: *mut *mut PRED_BUFFER,
        best_rdc: *mut RD_STATS,
        best_pickmode: *mut BEST_PICKMODE,
        best_sad_norm: *mut core::ffi::c_uint,
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
    fn av1_enc_build_inter_predictor_y_nonrd(
        xd: *mut MACROBLOCKD,
        inter_pred_params: *mut InterPredParams,
        subpel_params: *const SubpelParams,
    );
    fn av1_get_force_skip_low_temp_var_small_sb(
        variance_low: *const uint8_t,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    ) -> core::ffi::c_int;
    fn av1_get_force_skip_low_temp_var(
        variance_low: *const uint8_t,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        bsize: BLOCK_SIZE,
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
pub type ptrdiff_t = isize;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_0 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_0 = 15;
pub const TX_TYPES: C2RustUnnamed_0 = 16;
pub const H_FLIPADST: C2RustUnnamed_0 = 15;
pub const V_FLIPADST: C2RustUnnamed_0 = 14;
pub const H_ADST: C2RustUnnamed_0 = 13;
pub const V_ADST: C2RustUnnamed_0 = 12;
pub const H_DCT: C2RustUnnamed_0 = 11;
pub const V_DCT: C2RustUnnamed_0 = 10;
pub const IDTX: C2RustUnnamed_0 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_0 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_0 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_0 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_0 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_0 = 4;
pub const ADST_ADST: C2RustUnnamed_0 = 3;
pub const DCT_ADST: C2RustUnnamed_0 = 2;
pub const ADST_DCT: C2RustUnnamed_0 = 1;
pub const DCT_DCT: C2RustUnnamed_0 = 0;
pub type TX_TYPE = uint8_t;
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
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub frame: C2RustUnnamed_2,
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
pub struct C2RustUnnamed_2 {
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
pub const PLANE_TYPES: C2RustUnnamed_5 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_5 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_5 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_6 = 25;
pub const INTRA_MODES: C2RustUnnamed_6 = 13;
pub const INTER_MODE_END: C2RustUnnamed_6 = 25;
pub const INTER_MODE_START: C2RustUnnamed_6 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_6 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_6 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_6 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_6 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_6 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_6 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_6 = 13;
pub const DIR_MODE_END: C2RustUnnamed_6 = 9;
pub const DIR_MODE_START: C2RustUnnamed_6 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_6 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_6 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_6 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_6 = 25;
pub const NEW_NEWMV: C2RustUnnamed_6 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_6 = 23;
pub const NEW_NEARMV: C2RustUnnamed_6 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_6 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_6 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_6 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_6 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_6 = 17;
pub const NEWMV: C2RustUnnamed_6 = 16;
pub const GLOBALMV: C2RustUnnamed_6 = 15;
pub const NEARMV: C2RustUnnamed_6 = 14;
pub const NEARESTMV: C2RustUnnamed_6 = 13;
pub const PAETH_PRED: C2RustUnnamed_6 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_6 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_6 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_6 = 9;
pub const D67_PRED: C2RustUnnamed_6 = 8;
pub const D203_PRED: C2RustUnnamed_6 = 7;
pub const D157_PRED: C2RustUnnamed_6 = 6;
pub const D113_PRED: C2RustUnnamed_6 = 5;
pub const D135_PRED: C2RustUnnamed_6 = 4;
pub const D45_PRED: C2RustUnnamed_6 = 3;
pub const H_PRED: C2RustUnnamed_6 = 2;
pub const V_PRED: C2RustUnnamed_6 = 1;
pub const DC_PRED: C2RustUnnamed_6 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_7 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_7 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_7 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_7 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_7 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_7 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_7 = 9;
pub const UV_D67_PRED: C2RustUnnamed_7 = 8;
pub const UV_D203_PRED: C2RustUnnamed_7 = 7;
pub const UV_D157_PRED: C2RustUnnamed_7 = 6;
pub const UV_D113_PRED: C2RustUnnamed_7 = 5;
pub const UV_D135_PRED: C2RustUnnamed_7 = 4;
pub const UV_D45_PRED: C2RustUnnamed_7 = 3;
pub const UV_H_PRED: C2RustUnnamed_7 = 2;
pub const UV_V_PRED: C2RustUnnamed_7 = 1;
pub const UV_DC_PRED: C2RustUnnamed_7 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_8 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_8 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_8 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_8 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_8 = 0;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_9 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_9 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_9 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_9 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_9 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_9 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_9 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_10 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_10 = 7;
pub const BWD_REFS: C2RustUnnamed_10 = 3;
pub const FWD_REFS: C2RustUnnamed_10 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_10 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_10 = 8;
pub const REF_FRAMES: C2RustUnnamed_10 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_10 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_10 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_10 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_10 = 4;
pub const LAST3_FRAME: C2RustUnnamed_10 = 3;
pub const LAST2_FRAME: C2RustUnnamed_10 = 2;
pub const LAST_FRAME: C2RustUnnamed_10 = 1;
pub const INTRA_FRAME: C2RustUnnamed_10 = 0;
pub const NONE_FRAME: C2RustUnnamed_10 = -1;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const UNIDIR_COMP_REFS: C2RustUnnamed_11 = 4;
pub const TOTAL_UNIDIR_COMP_REFS: C2RustUnnamed_11 = 9;
pub const ALTREF2_ALTREF_FRAMES: C2RustUnnamed_11 = 8;
pub const BWDREF_ALTREF2_FRAMES: C2RustUnnamed_11 = 7;
pub const LAST3_GOLDEN_FRAMES: C2RustUnnamed_11 = 6;
pub const LAST2_GOLDEN_FRAMES: C2RustUnnamed_11 = 5;
pub const LAST2_LAST3_FRAMES: C2RustUnnamed_11 = 4;
pub const BWDREF_ALTREF_FRAMES: C2RustUnnamed_11 = 3;
pub const LAST_GOLDEN_FRAMES: C2RustUnnamed_11 = 2;
pub const LAST_LAST3_FRAMES: C2RustUnnamed_11 = 1;
pub const LAST_LAST2_FRAMES: C2RustUnnamed_11 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_22,
    pub c2rust_unnamed_0: C2RustUnnamed_20,
    pub c2rust_unnamed_1: C2RustUnnamed_18,
    pub c2rust_unnamed_2: C2RustUnnamed_16,
    pub c2rust_unnamed_3: C2RustUnnamed_14,
    pub c2rust_unnamed_4: C2RustUnnamed_12,
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
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_17,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub c2rust_unnamed: C2RustUnnamed_19,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub c2rust_unnamed: C2RustUnnamed_21,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub c2rust_unnamed: C2RustUnnamed_23,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type aom_cdf_prob = uint16_t;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_24 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_24 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_24 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_24 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_24 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_24 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_24 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_24 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_24 = 0;
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
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_25 = 4;
pub const AFFINE: C2RustUnnamed_25 = 3;
pub const ROTZOOM: C2RustUnnamed_25 = 2;
pub const TRANSLATION: C2RustUnnamed_25 = 1;
pub const IDENTITY: C2RustUnnamed_25 = 0;
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
pub type C2RustUnnamed_26 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_26 = 4;
pub const S_FRAME: C2RustUnnamed_26 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_26 = 2;
pub const INTER_FRAME: C2RustUnnamed_26 = 1;
pub const KEY_FRAME: C2RustUnnamed_26 = 0;
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
pub type foreach_transformed_block_visitor = Option<
    unsafe extern "C-unwind" fn(
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        BLOCK_SIZE,
        TX_SIZE,
        *mut core::ffi::c_void,
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
pub type C2RustUnnamed_27 = core::ffi::c_uint;
pub const SIGMOID: C2RustUnnamed_27 = 3;
pub const SOFTSIGN: C2RustUnnamed_27 = 2;
pub const RELU: C2RustUnnamed_27 = 1;
pub const NONE: C2RustUnnamed_27 = 0;
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
pub type C2RustUnnamed_28 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_28 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_28 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_28 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_28 = 0;
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
pub type C2RustUnnamed_29 = core::ffi::c_uint;
pub const THR_INVALID: C2RustUnnamed_29 = 255;
pub const THR_INTER_MODE_END: C2RustUnnamed_29 = 156;
pub const THR_INTER_MODE_START: C2RustUnnamed_29 = 0;
pub const THR_MODE_END: C2RustUnnamed_29 = 169;
pub const THR_MODE_START: C2RustUnnamed_29 = 0;
pub const NUM_SINGLE_REF_MODES: C2RustUnnamed_29 = 28;
pub const SINGLE_REF_MODE_END: C2RustUnnamed_29 = 28;
pub const SINGLE_REF_MODE_START: C2RustUnnamed_29 = 0;
pub const MAX_MODES: C2RustUnnamed_29 = 169;
pub const THR_D45_PRED: C2RustUnnamed_29 = 168;
pub const THR_D113_PRED: C2RustUnnamed_29 = 167;
pub const THR_D67_PRED: C2RustUnnamed_29 = 166;
pub const THR_D157_PRED: C2RustUnnamed_29 = 165;
pub const THR_D203_PRED: C2RustUnnamed_29 = 164;
pub const THR_D135_PRED: C2RustUnnamed_29 = 163;
pub const THR_V_PRED: C2RustUnnamed_29 = 162;
pub const THR_H_PRED: C2RustUnnamed_29 = 161;
pub const THR_SMOOTH_H: C2RustUnnamed_29 = 160;
pub const THR_SMOOTH_V: C2RustUnnamed_29 = 159;
pub const THR_SMOOTH: C2RustUnnamed_29 = 158;
pub const THR_PAETH: C2RustUnnamed_29 = 157;
pub const THR_DC: C2RustUnnamed_29 = 156;
pub const THR_COMP_GLOBAL_GLOBALBA: C2RustUnnamed_29 = 155;
pub const THR_COMP_NEAR_NEWBA: C2RustUnnamed_29 = 154;
pub const THR_COMP_NEW_NEARBA: C2RustUnnamed_29 = 153;
pub const THR_COMP_NEAREST_NEWBA: C2RustUnnamed_29 = 152;
pub const THR_COMP_NEW_NEARESTBA: C2RustUnnamed_29 = 151;
pub const THR_COMP_NEW_NEWBA: C2RustUnnamed_29 = 150;
pub const THR_COMP_NEAR_NEARBA: C2RustUnnamed_29 = 149;
pub const THR_COMP_GLOBAL_GLOBALLG: C2RustUnnamed_29 = 148;
pub const THR_COMP_NEAR_NEWLG: C2RustUnnamed_29 = 147;
pub const THR_COMP_NEW_NEARLG: C2RustUnnamed_29 = 146;
pub const THR_COMP_NEAREST_NEWLG: C2RustUnnamed_29 = 145;
pub const THR_COMP_NEW_NEARESTLG: C2RustUnnamed_29 = 144;
pub const THR_COMP_NEW_NEWLG: C2RustUnnamed_29 = 143;
pub const THR_COMP_NEAR_NEARLG: C2RustUnnamed_29 = 142;
pub const THR_COMP_GLOBAL_GLOBALLL3: C2RustUnnamed_29 = 141;
pub const THR_COMP_NEAR_NEWLL3: C2RustUnnamed_29 = 140;
pub const THR_COMP_NEW_NEARLL3: C2RustUnnamed_29 = 139;
pub const THR_COMP_NEAREST_NEWLL3: C2RustUnnamed_29 = 138;
pub const THR_COMP_NEW_NEARESTLL3: C2RustUnnamed_29 = 137;
pub const THR_COMP_NEW_NEWLL3: C2RustUnnamed_29 = 136;
pub const THR_COMP_NEAR_NEARLL3: C2RustUnnamed_29 = 135;
pub const THR_COMP_GLOBAL_GLOBALLL2: C2RustUnnamed_29 = 134;
pub const THR_COMP_NEAR_NEWLL2: C2RustUnnamed_29 = 133;
pub const THR_COMP_NEW_NEARLL2: C2RustUnnamed_29 = 132;
pub const THR_COMP_NEAREST_NEWLL2: C2RustUnnamed_29 = 131;
pub const THR_COMP_NEW_NEARESTLL2: C2RustUnnamed_29 = 130;
pub const THR_COMP_NEW_NEWLL2: C2RustUnnamed_29 = 129;
pub const THR_COMP_NEAR_NEARLL2: C2RustUnnamed_29 = 128;
pub const THR_COMP_GLOBAL_GLOBALGA2: C2RustUnnamed_29 = 127;
pub const THR_COMP_NEAR_NEWGA2: C2RustUnnamed_29 = 126;
pub const THR_COMP_NEW_NEARGA2: C2RustUnnamed_29 = 125;
pub const THR_COMP_NEAREST_NEWGA2: C2RustUnnamed_29 = 124;
pub const THR_COMP_NEW_NEARESTGA2: C2RustUnnamed_29 = 123;
pub const THR_COMP_NEW_NEWGA2: C2RustUnnamed_29 = 122;
pub const THR_COMP_NEAR_NEARGA2: C2RustUnnamed_29 = 121;
pub const THR_COMP_GLOBAL_GLOBALL3A2: C2RustUnnamed_29 = 120;
pub const THR_COMP_NEAR_NEWL3A2: C2RustUnnamed_29 = 119;
pub const THR_COMP_NEW_NEARL3A2: C2RustUnnamed_29 = 118;
pub const THR_COMP_NEAREST_NEWL3A2: C2RustUnnamed_29 = 117;
pub const THR_COMP_NEW_NEARESTL3A2: C2RustUnnamed_29 = 116;
pub const THR_COMP_NEW_NEWL3A2: C2RustUnnamed_29 = 115;
pub const THR_COMP_NEAR_NEARL3A2: C2RustUnnamed_29 = 114;
pub const THR_COMP_GLOBAL_GLOBALL2A2: C2RustUnnamed_29 = 113;
pub const THR_COMP_NEAR_NEWL2A2: C2RustUnnamed_29 = 112;
pub const THR_COMP_NEW_NEARL2A2: C2RustUnnamed_29 = 111;
pub const THR_COMP_NEAREST_NEWL2A2: C2RustUnnamed_29 = 110;
pub const THR_COMP_NEW_NEARESTL2A2: C2RustUnnamed_29 = 109;
pub const THR_COMP_NEW_NEWL2A2: C2RustUnnamed_29 = 108;
pub const THR_COMP_NEAR_NEARL2A2: C2RustUnnamed_29 = 107;
pub const THR_COMP_GLOBAL_GLOBALLA2: C2RustUnnamed_29 = 106;
pub const THR_COMP_NEAR_NEWLA2: C2RustUnnamed_29 = 105;
pub const THR_COMP_NEW_NEARLA2: C2RustUnnamed_29 = 104;
pub const THR_COMP_NEAREST_NEWLA2: C2RustUnnamed_29 = 103;
pub const THR_COMP_NEW_NEARESTLA2: C2RustUnnamed_29 = 102;
pub const THR_COMP_NEW_NEWLA2: C2RustUnnamed_29 = 101;
pub const THR_COMP_NEAR_NEARLA2: C2RustUnnamed_29 = 100;
pub const THR_COMP_GLOBAL_GLOBALGB: C2RustUnnamed_29 = 99;
pub const THR_COMP_NEAR_NEWGB: C2RustUnnamed_29 = 98;
pub const THR_COMP_NEW_NEARGB: C2RustUnnamed_29 = 97;
pub const THR_COMP_NEAREST_NEWGB: C2RustUnnamed_29 = 96;
pub const THR_COMP_NEW_NEARESTGB: C2RustUnnamed_29 = 95;
pub const THR_COMP_NEW_NEWGB: C2RustUnnamed_29 = 94;
pub const THR_COMP_NEAR_NEARGB: C2RustUnnamed_29 = 93;
pub const THR_COMP_GLOBAL_GLOBALL3B: C2RustUnnamed_29 = 92;
pub const THR_COMP_NEAR_NEWL3B: C2RustUnnamed_29 = 91;
pub const THR_COMP_NEW_NEARL3B: C2RustUnnamed_29 = 90;
pub const THR_COMP_NEAREST_NEWL3B: C2RustUnnamed_29 = 89;
pub const THR_COMP_NEW_NEARESTL3B: C2RustUnnamed_29 = 88;
pub const THR_COMP_NEW_NEWL3B: C2RustUnnamed_29 = 87;
pub const THR_COMP_NEAR_NEARL3B: C2RustUnnamed_29 = 86;
pub const THR_COMP_GLOBAL_GLOBALL2B: C2RustUnnamed_29 = 85;
pub const THR_COMP_NEAR_NEWL2B: C2RustUnnamed_29 = 84;
pub const THR_COMP_NEW_NEARL2B: C2RustUnnamed_29 = 83;
pub const THR_COMP_NEAREST_NEWL2B: C2RustUnnamed_29 = 82;
pub const THR_COMP_NEW_NEARESTL2B: C2RustUnnamed_29 = 81;
pub const THR_COMP_NEW_NEWL2B: C2RustUnnamed_29 = 80;
pub const THR_COMP_NEAR_NEARL2B: C2RustUnnamed_29 = 79;
pub const THR_COMP_GLOBAL_GLOBALGA: C2RustUnnamed_29 = 78;
pub const THR_COMP_NEAR_NEWGA: C2RustUnnamed_29 = 77;
pub const THR_COMP_NEW_NEARGA: C2RustUnnamed_29 = 76;
pub const THR_COMP_NEAREST_NEWGA: C2RustUnnamed_29 = 75;
pub const THR_COMP_NEW_NEARESTGA: C2RustUnnamed_29 = 74;
pub const THR_COMP_NEW_NEWGA: C2RustUnnamed_29 = 73;
pub const THR_COMP_NEAR_NEARGA: C2RustUnnamed_29 = 72;
pub const THR_COMP_GLOBAL_GLOBALL3A: C2RustUnnamed_29 = 71;
pub const THR_COMP_NEAR_NEWL3A: C2RustUnnamed_29 = 70;
pub const THR_COMP_NEW_NEARL3A: C2RustUnnamed_29 = 69;
pub const THR_COMP_NEAREST_NEWL3A: C2RustUnnamed_29 = 68;
pub const THR_COMP_NEW_NEARESTL3A: C2RustUnnamed_29 = 67;
pub const THR_COMP_NEW_NEWL3A: C2RustUnnamed_29 = 66;
pub const THR_COMP_NEAR_NEARL3A: C2RustUnnamed_29 = 65;
pub const THR_COMP_GLOBAL_GLOBALL2A: C2RustUnnamed_29 = 64;
pub const THR_COMP_NEAR_NEWL2A: C2RustUnnamed_29 = 63;
pub const THR_COMP_NEW_NEARL2A: C2RustUnnamed_29 = 62;
pub const THR_COMP_NEAREST_NEWL2A: C2RustUnnamed_29 = 61;
pub const THR_COMP_NEW_NEARESTL2A: C2RustUnnamed_29 = 60;
pub const THR_COMP_NEW_NEWL2A: C2RustUnnamed_29 = 59;
pub const THR_COMP_NEAR_NEARL2A: C2RustUnnamed_29 = 58;
pub const THR_COMP_GLOBAL_GLOBALLA: C2RustUnnamed_29 = 57;
pub const THR_COMP_NEAR_NEWLA: C2RustUnnamed_29 = 56;
pub const THR_COMP_NEW_NEARLA: C2RustUnnamed_29 = 55;
pub const THR_COMP_NEAREST_NEWLA: C2RustUnnamed_29 = 54;
pub const THR_COMP_NEW_NEARESTLA: C2RustUnnamed_29 = 53;
pub const THR_COMP_NEW_NEWLA: C2RustUnnamed_29 = 52;
pub const THR_COMP_NEAR_NEARLA: C2RustUnnamed_29 = 51;
pub const THR_COMP_GLOBAL_GLOBALLB: C2RustUnnamed_29 = 50;
pub const THR_COMP_NEAR_NEWLB: C2RustUnnamed_29 = 49;
pub const THR_COMP_NEW_NEARLB: C2RustUnnamed_29 = 48;
pub const THR_COMP_NEAREST_NEWLB: C2RustUnnamed_29 = 47;
pub const THR_COMP_NEW_NEARESTLB: C2RustUnnamed_29 = 46;
pub const THR_COMP_NEW_NEWLB: C2RustUnnamed_29 = 45;
pub const THR_COMP_NEAR_NEARLB: C2RustUnnamed_29 = 44;
pub const THR_COMP_NEAREST_NEARESTBA: C2RustUnnamed_29 = 43;
pub const THR_COMP_NEAREST_NEARESTLG: C2RustUnnamed_29 = 42;
pub const THR_COMP_NEAREST_NEARESTLL3: C2RustUnnamed_29 = 41;
pub const THR_COMP_NEAREST_NEARESTLL2: C2RustUnnamed_29 = 40;
pub const THR_COMP_NEAREST_NEARESTGA2: C2RustUnnamed_29 = 39;
pub const THR_COMP_NEAREST_NEARESTL3A2: C2RustUnnamed_29 = 38;
pub const THR_COMP_NEAREST_NEARESTL2A2: C2RustUnnamed_29 = 37;
pub const THR_COMP_NEAREST_NEARESTLA2: C2RustUnnamed_29 = 36;
pub const THR_COMP_NEAREST_NEARESTGB: C2RustUnnamed_29 = 35;
pub const THR_COMP_NEAREST_NEARESTL3B: C2RustUnnamed_29 = 34;
pub const THR_COMP_NEAREST_NEARESTL2B: C2RustUnnamed_29 = 33;
pub const THR_COMP_NEAREST_NEARESTLB: C2RustUnnamed_29 = 32;
pub const THR_COMP_NEAREST_NEARESTGA: C2RustUnnamed_29 = 31;
pub const THR_COMP_NEAREST_NEARESTL3A: C2RustUnnamed_29 = 30;
pub const THR_COMP_NEAREST_NEARESTL2A: C2RustUnnamed_29 = 29;
pub const THR_COMP_NEAREST_NEARESTLA: C2RustUnnamed_29 = 28;
pub const THR_GLOBALG: C2RustUnnamed_29 = 27;
pub const THR_GLOBALA: C2RustUnnamed_29 = 26;
pub const THR_GLOBALA2: C2RustUnnamed_29 = 25;
pub const THR_GLOBALB: C2RustUnnamed_29 = 24;
pub const THR_GLOBALL3: C2RustUnnamed_29 = 23;
pub const THR_GLOBALL2: C2RustUnnamed_29 = 22;
pub const THR_GLOBALMV: C2RustUnnamed_29 = 21;
pub const THR_NEARG: C2RustUnnamed_29 = 20;
pub const THR_NEARA: C2RustUnnamed_29 = 19;
pub const THR_NEARA2: C2RustUnnamed_29 = 18;
pub const THR_NEARB: C2RustUnnamed_29 = 17;
pub const THR_NEARL3: C2RustUnnamed_29 = 16;
pub const THR_NEARL2: C2RustUnnamed_29 = 15;
pub const THR_NEARMV: C2RustUnnamed_29 = 14;
pub const THR_NEWG: C2RustUnnamed_29 = 13;
pub const THR_NEWA: C2RustUnnamed_29 = 12;
pub const THR_NEWA2: C2RustUnnamed_29 = 11;
pub const THR_NEWB: C2RustUnnamed_29 = 10;
pub const THR_NEWL3: C2RustUnnamed_29 = 9;
pub const THR_NEWL2: C2RustUnnamed_29 = 8;
pub const THR_NEWMV: C2RustUnnamed_29 = 7;
pub const THR_NEARESTG: C2RustUnnamed_29 = 6;
pub const THR_NEARESTA: C2RustUnnamed_29 = 5;
pub const THR_NEARESTA2: C2RustUnnamed_29 = 4;
pub const THR_NEARESTB: C2RustUnnamed_29 = 3;
pub const THR_NEARESTL3: C2RustUnnamed_29 = 2;
pub const THR_NEARESTL2: C2RustUnnamed_29 = 1;
pub const THR_NEARESTMV: C2RustUnnamed_29 = 0;
pub type C2RustUnnamed_30 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_30 = 2;
pub const REALTIME: C2RustUnnamed_30 = 1;
pub const GOOD: C2RustUnnamed_30 = 0;
pub type MODE = uint8_t;
pub type C2RustUnnamed_31 = core::ffi::c_uint;
pub const NUM_DISTINCT_SEARCH_METHODS: C2RustUnnamed_31 = 6;
pub const NUM_SEARCH_METHODS: C2RustUnnamed_31 = 9;
pub const VFAST_DIAMOND: C2RustUnnamed_31 = 8;
pub const FAST_BIGDIA: C2RustUnnamed_31 = 7;
pub const FAST_DIAMOND: C2RustUnnamed_31 = 6;
pub const BIGDIA: C2RustUnnamed_31 = 5;
pub const HEX: C2RustUnnamed_31 = 4;
pub const CLAMPED_DIAMOND: C2RustUnnamed_31 = 3;
pub const NSTEP_8PT: C2RustUnnamed_31 = 2;
pub const NSTEP: C2RustUnnamed_31 = 1;
pub const DIAMOND: C2RustUnnamed_31 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FULLPEL_MOTION_SEARCH_PARAMS {
    pub bsize: BLOCK_SIZE,
    pub vfp: *const aom_variance_fn_ptr_t,
    pub ms_buffers: MSBuffers,
    pub search_method: SEARCH_METHODS,
    pub search_sites: *const search_site_config,
    pub mv_limits: FullMvLimits,
    pub run_mesh_search: core::ffi::c_int,
    pub prune_mesh_search: core::ffi::c_int,
    pub mesh_search_mv_diff_threshold: core::ffi::c_int,
    pub force_mesh_thresh: core::ffi::c_int,
    pub mesh_patterns: [*const MESH_PATTERN; 2],
    pub fine_search_interval: core::ffi::c_int,
    pub is_intra_mode: core::ffi::c_int,
    pub fast_obmc_search: core::ffi::c_int,
    pub mv_cost_params: MV_COST_PARAMS,
    pub sdf: aom_sad_fn_t,
    pub sdx4df: aom_sad_multi_d_fn_t,
    pub sdx3df: aom_sad_multi_d_fn_t,
}
pub type av1_init_search_site_config = Option<
    unsafe extern "C-unwind" fn(
        *mut search_site_config,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type C2RustUnnamed_32 = core::ffi::c_uint;
pub const FULL_PEL: C2RustUnnamed_32 = 3;
pub const HALF_PEL: C2RustUnnamed_32 = 2;
pub const QUARTER_PEL: C2RustUnnamed_32 = 1;
pub const EIGHTH_PEL: C2RustUnnamed_32 = 0;
pub type C2RustUnnamed_33 = core::ffi::c_uint;
pub const SUBPEL_SEARCH_METHODS: C2RustUnnamed_33 = 3;
pub const SUBPEL_TREE_PRUNED_MORE: C2RustUnnamed_33 = 2;
pub const SUBPEL_TREE_PRUNED: C2RustUnnamed_33 = 1;
pub const SUBPEL_TREE: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_34 = core::ffi::c_uint;
pub const AQ_MODE_COUNT: C2RustUnnamed_34 = 4;
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_34 = 3;
pub const COMPLEXITY_AQ: C2RustUnnamed_34 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_34 = 1;
pub const NO_AQ: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = core::ffi::c_uint;
pub const SS_CFG_TOTAL: C2RustUnnamed_35 = 3;
pub const SS_CFG_FPF: C2RustUnnamed_35 = 2;
pub const SS_CFG_LOOKAHEAD: C2RustUnnamed_35 = 1;
pub const SS_CFG_SRC: C2RustUnnamed_35 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estimate_block_intra_args {
    pub cpi: *mut AV1_COMP,
    pub x: *mut MACROBLOCK,
    pub mode: PREDICTION_MODE,
    pub skippable: core::ffi::c_int,
    pub rdc: *mut RD_STATS,
    pub best_sad: core::ffi::c_uint,
    pub prune_mode_based_on_sad: bool,
    pub prune_palette_sad: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterModeSearchStateNonrd {
    pub best_pickmode: BEST_PICKMODE,
    pub this_rdc: RD_STATS,
    pub best_rdc: RD_STATS,
    pub uv_dist: [[int64_t; 8]; 4],
    pub yv12_mb: [[buf_2d; 3]; 8],
    pub vars: [[core::ffi::c_uint; 8]; 4],
    pub ref_costs_single: [core::ffi::c_uint; 8],
    pub frame_mv: [[int_mv; 8]; 25],
    pub frame_mv_best: [[int_mv; 8]; 25],
    pub single_inter_mode_costs: [[core::ffi::c_int; 8]; 4],
    pub use_ref_frame_mask: [core::ffi::c_int; 8],
    pub mode_checked: [[uint8_t; 8]; 25],
    pub use_scaled_ref_frame: [bool; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BEST_PICKMODE {
    pub best_pred: *mut PRED_BUFFER,
    pub best_mode: PREDICTION_MODE,
    pub best_tx_size: TX_SIZE,
    pub tx_type: TX_TYPE,
    pub best_ref_frame: MV_REFERENCE_FRAME,
    pub best_second_ref_frame: MV_REFERENCE_FRAME,
    pub best_mode_skip_txfm: uint8_t,
    pub best_mode_initial_skip_flag: uint8_t,
    pub best_pred_filter: int_interpfilters,
    pub best_motion_mode: MOTION_MODE,
    pub wm_params: WarpedMotionParams,
    pub num_proj_ref: core::ffi::c_int,
    pub pmi: PALETTE_MODE_INFO,
    pub best_sse: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRED_BUFFER {
    pub data: *mut uint8_t,
    pub stride: core::ffi::c_int,
    pub in_use: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REF_MODE {
    pub ref_frame: MV_REFERENCE_FRAME,
    pub pred_mode: PREDICTION_MODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMP_REF_MODE {
    pub ref_frame: [MV_REFERENCE_FRAME; 2],
    pub pred_mode: PREDICTION_MODE,
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const UINT8_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NEWMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << GLOBALMV_OFFSET)
    - 1 as core::ffi::c_int;
pub const GLOBALMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << REFMV_OFFSET - GLOBALMV_OFFSET) - 1 as core::ffi::c_int;
pub const REFMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 8 as core::ffi::c_int - REFMV_OFFSET) - 1 as core::ffi::c_int;
pub const USABLE_REF_MV_STACK_SIZE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const REF_CAT_LEVEL: core::ffi::c_int = 640 as core::ffi::c_int;
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
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
static mut tx_mode_to_biggest_tx_size: [TX_SIZE; 3] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
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
#[inline]
unsafe extern "C-unwind" fn get_segdata(
    mut seg: *const segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return (*seg).feature_data[segment_id as usize][feature_id as usize]
        as core::ffi::c_int;
}
pub const QINDEX_BITS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int)
    << SCALE_SUBPEL_BITS;
pub const SCALE_SUBPEL_MASK: core::ffi::c_int = SCALE_SUBPEL_SHIFTS
    - 1 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const SCALE_EXTRA_OFF: core::ffi::c_int = ((1 as core::ffi::c_int)
    << SCALE_EXTRA_BITS) / 2 as core::ffi::c_int;
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
pub const INVALID_MV_ROW_COL: core::ffi::c_int = -(32768 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn get_fullmv_from_mv(mut subpel_mv: *const MV) -> FULLPEL_MV {
    let full_mv: FULLPEL_MV = {
        let mut init = fullpel_mv {
            row: ((*subpel_mv).row as core::ffi::c_int + 3 as core::ffi::c_int
                + ((*subpel_mv).row as core::ffi::c_int >= 0 as core::ffi::c_int)
                    as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t,
            col: ((*subpel_mv).col as core::ffi::c_int + 3 as core::ffi::c_int
                + ((*subpel_mv).col as core::ffi::c_int >= 0 as core::ffi::c_int)
                    as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t,
        };
        init
    };
    return full_mv;
}
#[inline]
unsafe extern "C-unwind" fn get_mv_from_fullmv(mut full_mv: *const FULLPEL_MV) -> MV {
    let subpel_mv: MV = {
        let mut init = mv {
            row: ((*full_mv).row as core::ffi::c_int * 8 as core::ffi::c_int) as int16_t,
            col: ((*full_mv).col as core::ffi::c_int * 8 as core::ffi::c_int) as int16_t,
        };
        init
    };
    return subpel_mv;
}
#[inline]
unsafe extern "C-unwind" fn convert_fullmv_to_mv(mut mv: *mut int_mv) {
    (*mv).as_mv = get_mv_from_fullmv(&raw mut (*mv).as_fullmv);
}
#[inline]
unsafe extern "C-unwind" fn av1_broadcast_interp_filter(
    mut filter: InterpFilter,
) -> int_interpfilters {
    let mut filters: int_interpfilters = int_interpfilters { as_int: 0 };
    filters.as_filters.x_filter = filter as uint16_t;
    filters.as_filters.y_filter = filter as uint16_t;
    return filters;
}
#[inline]
unsafe extern "C-unwind" fn av1_unswitchable_filter(mut filter: InterpFilter) -> InterpFilter {
    return (if filter as core::ffi::c_int == SWITCHABLE as core::ffi::c_int {
        EIGHTTAP_REGULAR as core::ffi::c_int
    } else {
        filter as core::ffi::c_int
    }) as InterpFilter;
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
#[inline]
unsafe extern "C-unwind" fn get_conv_params(
    mut do_average: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    return get_conv_params_no_round(
        do_average,
        plane,
        0 as *mut CONV_BUF_TYPE,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bd,
    );
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
unsafe extern "C-unwind" fn is_inter_singleref_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= SINGLE_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < SINGLE_INTER_MODE_END as core::ffi::c_int)
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
unsafe extern "C-unwind" fn compound_ref0_mode(mut mode: PREDICTION_MODE) -> PREDICTION_MODE {
    static mut lut: [PREDICTION_MODE; 25] = [
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
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
    ];
    if !((::core::mem::size_of::<[PREDICTION_MODE; 25]>() as usize)
        .wrapping_div(::core::mem::size_of::<PREDICTION_MODE>() as usize)
        as core::ffi::c_int == MB_MODE_COUNT as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"compound_ref0_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            113 as core::ffi::c_int,
            b"NELEMENTS(lut) == MB_MODE_COUNT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(is_inter_compound_mode(mode) != 0 || is_inter_singleref_mode(mode) != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"compound_ref0_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            114 as core::ffi::c_int,
            b"is_inter_compound_mode(mode) || is_inter_singleref_mode(mode)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return lut[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn compound_ref1_mode(mut mode: PREDICTION_MODE) -> PREDICTION_MODE {
    static mut lut: [PREDICTION_MODE; 25] = [
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        MB_MODE_COUNT as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
        NEARMV as core::ffi::c_int as PREDICTION_MODE,
        GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        NEWMV as core::ffi::c_int as PREDICTION_MODE,
    ];
    if !((::core::mem::size_of::<[PREDICTION_MODE; 25]>() as usize)
        .wrapping_div(::core::mem::size_of::<PREDICTION_MODE>() as usize)
        as core::ffi::c_int == MB_MODE_COUNT as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"compound_ref1_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            146 as core::ffi::c_int,
            b"NELEMENTS(lut) == MB_MODE_COUNT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (is_inter_compound_mode(mode) == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"compound_ref1_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            147 as core::ffi::c_int,
            b"is_inter_compound_mode(mode)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return lut[mode as usize];
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
pub const INTRABC_DELAY_PIXELS: core::ffi::c_int = 256 as core::ffi::c_int;
pub const INTRABC_DELAY_SB64: core::ffi::c_int = INTRABC_DELAY_PIXELS
    / 64 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_find_ref_dv(
    mut ref_dv: *mut int_mv,
    tile: *const TileInfo,
    mut mib_size: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
) {
    if mi_row - mib_size < (*tile).mi_row_start {
        (*ref_dv).as_fullmv.row = 0 as int16_t;
        (*ref_dv).as_fullmv.col = (-MI_SIZE * mib_size - INTRABC_DELAY_PIXELS)
            as int16_t;
    } else {
        (*ref_dv).as_fullmv.row = (-MI_SIZE * mib_size) as int16_t;
        (*ref_dv).as_fullmv.col = 0 as int16_t;
    }
    convert_fullmv_to_mv(ref_dv);
}
#[inline]
unsafe extern "C-unwind" fn av1_is_dv_valid(
    dv: MV,
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mib_size_log2: core::ffi::c_int,
) -> core::ffi::c_int {
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let SCALE_PX_TO_MV: core::ffi::c_int = 8 as core::ffi::c_int;
    if dv.row as core::ffi::c_int & SCALE_PX_TO_MV - 1 as core::ffi::c_int != 0
        || dv.col as core::ffi::c_int & SCALE_PX_TO_MV - 1 as core::ffi::c_int != 0
    {
        return 0 as core::ffi::c_int;
    }
    let tile: *const TileInfo = &(*xd).tile;
    let src_top_edge: core::ffi::c_int = mi_row * MI_SIZE * SCALE_PX_TO_MV
        + dv.row as core::ffi::c_int;
    let tile_top_edge: core::ffi::c_int = (*tile).mi_row_start * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_top_edge < tile_top_edge {
        return 0 as core::ffi::c_int;
    }
    let src_left_edge: core::ffi::c_int = mi_col * MI_SIZE * SCALE_PX_TO_MV
        + dv.col as core::ffi::c_int;
    let tile_left_edge: core::ffi::c_int = (*tile).mi_col_start * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_left_edge < tile_left_edge {
        return 0 as core::ffi::c_int;
    }
    let src_bottom_edge: core::ffi::c_int = (mi_row * MI_SIZE + bh) * SCALE_PX_TO_MV
        + dv.row as core::ffi::c_int;
    let tile_bottom_edge: core::ffi::c_int = (*tile).mi_row_end * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_bottom_edge > tile_bottom_edge {
        return 0 as core::ffi::c_int;
    }
    let src_right_edge: core::ffi::c_int = (mi_col * MI_SIZE + bw) * SCALE_PX_TO_MV
        + dv.col as core::ffi::c_int;
    let tile_right_edge: core::ffi::c_int = (*tile).mi_col_end * MI_SIZE
        * SCALE_PX_TO_MV;
    if src_right_edge > tile_right_edge {
        return 0 as core::ffi::c_int;
    }
    if (*xd).is_chroma_ref as core::ffi::c_int != 0
        && av1_num_planes(cm) > 1 as core::ffi::c_int
    {
        let pd: *const macroblockd_plane = &*((*xd).plane)
            .as_ptr()
            .offset(1 as core::ffi::c_int as isize) as *const macroblockd_plane;
        if bw < 8 as core::ffi::c_int && (*pd).subsampling_x != 0 {
            if src_left_edge < tile_left_edge + 4 as core::ffi::c_int * SCALE_PX_TO_MV {
                return 0 as core::ffi::c_int;
            }
        }
        if bh < 8 as core::ffi::c_int && (*pd).subsampling_y != 0 {
            if src_top_edge < tile_top_edge + 4 as core::ffi::c_int * SCALE_PX_TO_MV {
                return 0 as core::ffi::c_int;
            }
        }
    }
    let max_mib_size: core::ffi::c_int = (1 as core::ffi::c_int) << mib_size_log2;
    let active_sb_row: core::ffi::c_int = mi_row >> mib_size_log2;
    let active_sb64_col: core::ffi::c_int = mi_col * MI_SIZE >> 6 as core::ffi::c_int;
    let sb_size: core::ffi::c_int = max_mib_size * MI_SIZE;
    let src_sb_row: core::ffi::c_int = ((src_bottom_edge >> 3 as core::ffi::c_int)
        - 1 as core::ffi::c_int) / sb_size;
    let src_sb64_col: core::ffi::c_int = (src_right_edge >> 3 as core::ffi::c_int)
        - 1 as core::ffi::c_int >> 6 as core::ffi::c_int;
    let total_sb64_per_row: core::ffi::c_int = ((*tile).mi_col_end - (*tile).mi_col_start
        - 1 as core::ffi::c_int >> 4 as core::ffi::c_int) + 1 as core::ffi::c_int;
    let active_sb64: core::ffi::c_int = active_sb_row * total_sb64_per_row
        + active_sb64_col;
    let src_sb64: core::ffi::c_int = src_sb_row * total_sb64_per_row + src_sb64_col;
    if src_sb64 >= active_sb64 - INTRABC_DELAY_SB64 {
        return 0 as core::ffi::c_int;
    }
    let gradient: core::ffi::c_int = 1 as core::ffi::c_int + INTRABC_DELAY_SB64
        + (sb_size > 64 as core::ffi::c_int) as core::ffi::c_int;
    let wf_offset: core::ffi::c_int = gradient * (active_sb_row - src_sb_row);
    if src_sb_row > active_sb_row
        || src_sb64_col >= active_sb64_col - INTRABC_DELAY_SB64 + wf_offset
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn init_subpel_params(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    mut subpel_params: *mut SubpelParams,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut sf: *const scale_factors = (*inter_pred_params).scale_factors;
    let mut ssx: core::ffi::c_int = (*inter_pred_params).subsampling_x;
    let mut ssy: core::ffi::c_int = (*inter_pred_params).subsampling_y;
    let mut orig_pos_y: core::ffi::c_int = (*inter_pred_params).pix_row << SUBPEL_BITS;
    orig_pos_y
        += (*src_mv).row as core::ffi::c_int
            * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssy);
    let mut orig_pos_x: core::ffi::c_int = (*inter_pred_params).pix_col << SUBPEL_BITS;
    orig_pos_x
        += (*src_mv).col as core::ffi::c_int
            * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssx);
    let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
    let mut pos_x: core::ffi::c_int = 0;
    let mut pos_y: core::ffi::c_int = 0;
    if (is_scaled == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        pos_y = av1_unscaled_value(orig_pos_y, sf);
        pos_x = av1_unscaled_value(orig_pos_x, sf);
    } else {
        pos_y = av1_scaled_y(orig_pos_y, sf);
        pos_x = av1_scaled_x(orig_pos_x, sf);
    }
    pos_x += SCALE_EXTRA_OFF;
    pos_y += SCALE_EXTRA_OFF;
    let bottom: core::ffi::c_int = height + AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS;
    let right: core::ffi::c_int = width + AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS;
    pos_y = clamp(pos_y, (*inter_pred_params).top, bottom);
    pos_x = clamp(pos_x, (*inter_pred_params).left, right);
    (*subpel_params).pos_x = pos_x;
    (*subpel_params).pos_y = pos_y;
    (*subpel_params).subpel_x = pos_x & SCALE_SUBPEL_MASK;
    (*subpel_params).subpel_y = pos_y & SCALE_SUBPEL_MASK;
    (*subpel_params).xs = (*sf).x_step_q4;
    (*subpel_params).ys = (*sf).y_step_q4;
}
#[inline]
unsafe extern "C-unwind" fn init_inter_mode_params(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    mut subpel_params: *mut SubpelParams,
    mut sf: *const scale_factors,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    (*inter_pred_params).scale_factors = sf;
    init_subpel_params(src_mv, inter_pred_params, subpel_params, width, height);
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
unsafe extern "C-unwind" fn set_default_interp_filters(
    mbmi: *mut MB_MODE_INFO,
    mut frame_interp_filter: InterpFilter,
) {
    (*mbmi).interp_filters = av1_broadcast_interp_filter(
        av1_unswitchable_filter(frame_interp_filter),
    );
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
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
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MV_COST_WEIGHT: core::ffi::c_int = 108 as core::ffi::c_int;
pub const RESOLUTION_288P: core::ffi::c_int = 352 as core::ffi::c_int
    * 288 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn mode_offset(mode: PREDICTION_MODE) -> core::ffi::c_int {
    if mode as core::ffi::c_int >= NEARESTMV as core::ffi::c_int {
        return mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int
    } else {
        match mode as core::ffi::c_int {
            0 => return 0 as core::ffi::c_int,
            1 => return 1 as core::ffi::c_int,
            2 => return 2 as core::ffi::c_int,
            9 => return 3 as core::ffi::c_int,
            _ => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 12],
                            [core::ffi::c_char; 12],
                        >(*b"mode_offset\0"))
                            .as_ptr(),
                        b"rd.h\0" as *const u8 as *const core::ffi::c_char,
                        87 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                return -(1 as core::ffi::c_int);
            }
        }
    };
}
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
unsafe extern "C-unwind" fn rd_less_than_thresh(
    mut best_rd: int64_t,
    mut thresh: int64_t,
    mut thresh_fact: core::ffi::c_int,
) -> core::ffi::c_int {
    return (best_rd < thresh * thresh_fact as int64_t >> 5 as core::ffi::c_int
        || thresh == INT_MAX as int64_t) as core::ffi::c_int;
}
static mut search_method_lookup: [SEARCH_METHODS; 9] = [
    DIAMOND as core::ffi::c_int as SEARCH_METHODS,
    NSTEP as core::ffi::c_int as SEARCH_METHODS,
    NSTEP_8PT as core::ffi::c_int as SEARCH_METHODS,
    CLAMPED_DIAMOND as core::ffi::c_int as SEARCH_METHODS,
    HEX as core::ffi::c_int as SEARCH_METHODS,
    BIGDIA as core::ffi::c_int as SEARCH_METHODS,
    BIGDIA as core::ffi::c_int as SEARCH_METHODS,
    BIGDIA as core::ffi::c_int as SEARCH_METHODS,
    BIGDIA as core::ffi::c_int as SEARCH_METHODS,
];
#[inline]
unsafe extern "C-unwind" fn av1_refresh_search_site_config(
    mut ss_cfg_buf: *mut search_site_config,
    mut search_method: SEARCH_METHODS,
    ref_stride: core::ffi::c_int,
) {
    let level: core::ffi::c_int = (search_method as core::ffi::c_int
        == NSTEP_8PT as core::ffi::c_int
        || search_method as core::ffi::c_int == CLAMPED_DIAMOND as core::ffi::c_int)
        as core::ffi::c_int;
    search_method = search_method_lookup[search_method as usize];
    (av1_init_motion_compensation[search_method as usize])
        .expect(
            "non-null function pointer",
        )(&raw mut *ss_cfg_buf.offset(search_method as isize), ref_stride, level);
}
#[inline]
unsafe extern "C-unwind" fn av1_is_subpelmv_in_range(
    mut mv_limits: *const SubpelMvLimits,
    mut mv: MV,
) -> core::ffi::c_int {
    return (mv.col as core::ffi::c_int >= (*mv_limits).col_min
        && mv.col as core::ffi::c_int <= (*mv_limits).col_max
        && mv.row as core::ffi::c_int >= (*mv_limits).row_min
        && mv.row as core::ffi::c_int <= (*mv_limits).row_max) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_offset_from_fullmv(
    mut mv: *const FULLPEL_MV,
    mut stride: core::ffi::c_int,
) -> core::ffi::c_int {
    return (*mv).row as core::ffi::c_int * stride + (*mv).col as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_buf_from_fullmv(
    mut buf: *const buf_2d,
    mut mv: *const FULLPEL_MV,
) -> *const uint8_t {
    return &raw mut *((*buf).buf)
        .offset(
            (get_offset_from_fullmv
                as unsafe extern "C-unwind" fn(
                    *const FULLPEL_MV,
                    core::ffi::c_int,
                ) -> core::ffi::c_int)(mv, (*buf).stride) as isize,
        ) as *mut uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn is_lossless_requested(
    rc_cfg: *const RateControlCfg,
) -> core::ffi::c_int {
    return ((*rc_cfg).best_allowed_q == 0 as core::ffi::c_int
        && (*rc_cfg).worst_allowed_q == 0 as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_chessboard_index(
    mut frame_index: core::ffi::c_int,
) -> core::ffi::c_int {
    return frame_index & 0x1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn cond_cost_list(
    mut cpi: *const AV1_COMP,
    mut cost_list: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let use_cost_list: core::ffi::c_int = ((*cpi).sf.mv_sf.subpel_search_method
        as core::ffi::c_int != SUBPEL_TREE as core::ffi::c_int
        && (*cpi).sf.mv_sf.use_fullpel_costlist != 0) as core::ffi::c_int;
    return if use_cost_list != 0 { cost_list } else { 0 as *mut core::ffi::c_int };
}
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
unsafe extern "C-unwind" fn is_mode_intra(mut mode: PREDICTION_MODE) -> core::ffi::c_int {
    return ((mode as core::ffi::c_int) < INTRA_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
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
unsafe extern "C-unwind" fn model_rd_with_curvfit(
    cpi: *const AV1_COMP,
    x: *const MACROBLOCK,
    mut plane_bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut sse: int64_t,
    mut num_samples: core::ffi::c_int,
    mut rate: *mut core::ffi::c_int,
    mut dist: *mut int64_t,
) {
    let xd: *const MACROBLOCKD = &(*x).e_mbd;
    let p: *const macroblock_plane = &*((*x).plane).as_ptr().offset(plane as isize)
        as *const macroblock_plane;
    let dequant_shift: core::ffi::c_int = if is_cur_buf_hbd(xd) != 0 {
        (*xd).bd - 5 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    let qstep: core::ffi::c_int = if *((*p).dequant_QTX)
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int >> dequant_shift
        > 1 as core::ffi::c_int
    {
        *((*p).dequant_QTX).offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            >> dequant_shift
    } else {
        1 as core::ffi::c_int
    };
    if sse == 0 as int64_t {
        if !rate.is_null() {
            *rate = 0 as core::ffi::c_int;
        }
        if !dist.is_null() {
            *dist = 0 as int64_t;
        }
        return;
    }
    let sse_norm: core::ffi::c_double = sse as core::ffi::c_double
        / num_samples as core::ffi::c_double;
    let qstepsqr: core::ffi::c_double = qstep as core::ffi::c_double
        * qstep as core::ffi::c_double;
    let xqr: core::ffi::c_double = log2(sse_norm / qstepsqr) as core::ffi::c_double;
    let mut rate_dist_f: [core::ffi::c_double; 2] = [0.; 2];
    av1_model_rd_curvfit(plane_bsize, sse_norm, xqr, rate_dist_f.as_mut_ptr());
    let dist_f: core::ffi::c_double = rate_dist_f[1 as core::ffi::c_int as usize]
        * sse_norm;
    let mut rate_i: core::ffi::c_int = ((if 0.0f64
        > rate_dist_f[0 as core::ffi::c_int as usize]
            * num_samples as core::ffi::c_double
    {
        0.0f64
    } else {
        rate_dist_f[0 as core::ffi::c_int as usize] * num_samples as core::ffi::c_double
    }) + 0.5f64) as core::ffi::c_int;
    let mut dist_i: int64_t = ((if 0.0f64 > dist_f * num_samples as core::ffi::c_double {
        0.0f64
    } else {
        dist_f * num_samples as core::ffi::c_double
    }) + 0.5f64) as int64_t;
    if rate_i == 0 as core::ffi::c_int {
        dist_i = sse << 4 as core::ffi::c_int;
    } else if (rate_i as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + dist_i * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
        >= (0 as core::ffi::c_int as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + (sse << 4 as core::ffi::c_int)
                * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
    {
        rate_i = 0 as core::ffi::c_int;
        dist_i = sse << 4 as core::ffi::c_int;
    }
    if !rate.is_null() {
        *rate = rate_i;
    }
    if !dist.is_null() {
        *dist = dist_i;
    }
}
#[inline]
unsafe extern "C-unwind" fn av1_get_search_site_config(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut search_method: SEARCH_METHODS,
) -> *const search_site_config {
    let ref_stride: core::ffi::c_int = (*x)
        .e_mbd
        .plane[0 as core::ffi::c_int as usize]
        .pre[0 as core::ffi::c_int as usize]
        .stride;
    let mut mv_search_params: *const MotionVectorSearchParams = &(*cpi).mv_search_params;
    if ref_stride
        == (*((*mv_search_params)
            .search_site_cfg[SS_CFG_SRC as core::ffi::c_int as usize])
            .as_ptr())
            .stride
    {
        return ((*mv_search_params)
            .search_site_cfg[SS_CFG_SRC as core::ffi::c_int as usize])
            .as_ptr()
    } else if ref_stride
        == (*((*mv_search_params)
            .search_site_cfg[SS_CFG_LOOKAHEAD as core::ffi::c_int as usize])
            .as_ptr())
            .stride
    {
        return ((*mv_search_params)
            .search_site_cfg[SS_CFG_LOOKAHEAD as core::ffi::c_int as usize])
            .as_ptr()
    }
    search_method = search_method_lookup[search_method as usize];
    if !(search_method_lookup[search_method as usize] as core::ffi::c_int
        == search_method as core::ffi::c_int
        && !(b"The search_method_lookup table should be idempotent.\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_get_search_site_config\0"))
                .as_ptr(),
            b"motion_search_facade.h\0" as *const u8 as *const core::ffi::c_char,
            88 as core::ffi::c_int,
            b"search_method_lookup[search_method] == search_method && \"The search_method_lookup table should be idempotent.\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if ref_stride != (*x).search_site_cfg_buf[search_method as usize].stride {
        av1_refresh_search_site_config(
            ((*x).search_site_cfg_buf).as_mut_ptr(),
            search_method,
            ref_stride,
        );
    }
    return ((*x).search_site_cfg_buf).as_mut_ptr();
}
#[inline]
unsafe extern "C-unwind" fn av1_get_faster_search_method(
    mut search_method: SEARCH_METHODS,
) -> SEARCH_METHODS {
    match search_method as core::ffi::c_int {
        1 => return DIAMOND as core::ffi::c_int as SEARCH_METHODS,
        2 => return DIAMOND as core::ffi::c_int as SEARCH_METHODS,
        0 => return BIGDIA as core::ffi::c_int as SEARCH_METHODS,
        3 => return BIGDIA as core::ffi::c_int as SEARCH_METHODS,
        5 => return HEX as core::ffi::c_int as SEARCH_METHODS,
        4 => return FAST_DIAMOND as core::ffi::c_int as SEARCH_METHODS,
        6 => return VFAST_DIAMOND as core::ffi::c_int as SEARCH_METHODS,
        7 => return FAST_BIGDIA as core::ffi::c_int as SEARCH_METHODS,
        8 => return VFAST_DIAMOND as core::ffi::c_int as SEARCH_METHODS,
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid search method!\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"av1_get_faster_search_method\0"))
                        .as_ptr(),
                    b"motion_search_facade.h\0" as *const u8 as *const core::ffi::c_char,
                    115 as core::ffi::c_int,
                    b"0 && \"Invalid search method!\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            return DIAMOND as core::ffi::c_int as SEARCH_METHODS;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_default_mv_search_method(
    mut x: *const MACROBLOCK,
    mut mv_sf: *const MV_SPEED_FEATURES,
    mut bsize: BLOCK_SIZE,
) -> SEARCH_METHODS {
    let mut search_method: SEARCH_METHODS = (*mv_sf).search_method;
    let sf_blk_search_method: core::ffi::c_int = (*mv_sf)
        .use_bsize_dependent_search_method;
    let min_dim: core::ffi::c_int = if (block_size_wide[bsize as usize]
        as core::ffi::c_int) < block_size_high[bsize as usize] as core::ffi::c_int
    {
        block_size_wide[bsize as usize] as core::ffi::c_int
    } else {
        block_size_high[bsize as usize] as core::ffi::c_int
    };
    let qband: core::ffi::c_int = (*x).qindex >> QINDEX_BITS - 2 as core::ffi::c_int;
    let min_dim_th: [core::ffi::c_int; 4] = [
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    ];
    let mut use_faster_search_method: bool = false_0 != 0;
    if sf_blk_search_method >= 1 as core::ffi::c_int
        && sf_blk_search_method <= 3 as core::ffi::c_int
    {
        use_faster_search_method = min_dim
            >= min_dim_th[(sf_blk_search_method - 1 as core::ffi::c_int) as usize];
    } else if sf_blk_search_method == 4 as core::ffi::c_int {
        use_faster_search_method = min_dim
            >= min_dim_th[(sf_blk_search_method - 1 as core::ffi::c_int) as usize]
            && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
                <= kMedSad as core::ffi::c_int as core::ffi::c_uint
            && qband < 3 as core::ffi::c_int;
    }
    if use_faster_search_method {
        search_method = av1_get_faster_search_method(search_method);
    }
    return search_method;
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
pub const RTC_INTER_MODES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const RTC_INTRA_MODES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NUM_COMP_INTER_MODES_RT: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NUM_COMP_INTER_MODES_RT_FULL: core::ffi::c_int = 10 as core::ffi::c_int;
pub const NUM_INTER_MODES: core::ffi::c_int = 12 as core::ffi::c_int;
pub const NUM_INTER_MODES_FULL: core::ffi::c_int = 28 as core::ffi::c_int;
pub const FILTER_SEARCH_SIZE: core::ffi::c_int = 2 as core::ffi::c_int;
static mut b_width_log2_lookup: [uint8_t; 16] = [
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
];
static mut b_height_log2_lookup: [uint8_t; 16] = [
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
];
static mut intra_mode_list: [PREDICTION_MODE; 4] = [
    DC_PRED as core::ffi::c_int as PREDICTION_MODE,
    V_PRED as core::ffi::c_int as PREDICTION_MODE,
    H_PRED as core::ffi::c_int as PREDICTION_MODE,
    SMOOTH_PRED as core::ffi::c_int as PREDICTION_MODE,
];
static mut mode_idx: [[THR_MODES; 4]; 8] = [
    [
        THR_DC as core::ffi::c_int as THR_MODES,
        THR_V_PRED as core::ffi::c_int as THR_MODES,
        THR_H_PRED as core::ffi::c_int as THR_MODES,
        THR_SMOOTH as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTMV as core::ffi::c_int as THR_MODES,
        THR_NEARMV as core::ffi::c_int as THR_MODES,
        THR_GLOBALMV as core::ffi::c_int as THR_MODES,
        THR_NEWMV as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTL2 as core::ffi::c_int as THR_MODES,
        THR_NEARL2 as core::ffi::c_int as THR_MODES,
        THR_GLOBALL2 as core::ffi::c_int as THR_MODES,
        THR_NEWL2 as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTL3 as core::ffi::c_int as THR_MODES,
        THR_NEARL3 as core::ffi::c_int as THR_MODES,
        THR_GLOBALL3 as core::ffi::c_int as THR_MODES,
        THR_NEWL3 as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTG as core::ffi::c_int as THR_MODES,
        THR_NEARG as core::ffi::c_int as THR_MODES,
        THR_GLOBALG as core::ffi::c_int as THR_MODES,
        THR_NEWG as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTB as core::ffi::c_int as THR_MODES,
        THR_NEARB as core::ffi::c_int as THR_MODES,
        THR_GLOBALB as core::ffi::c_int as THR_MODES,
        THR_NEWB as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTA2 as core::ffi::c_int as THR_MODES,
        THR_NEARA2 as core::ffi::c_int as THR_MODES,
        THR_GLOBALA2 as core::ffi::c_int as THR_MODES,
        THR_NEWA2 as core::ffi::c_int as THR_MODES,
    ],
    [
        THR_NEARESTA as core::ffi::c_int as THR_MODES,
        THR_NEARA as core::ffi::c_int as THR_MODES,
        THR_GLOBALA as core::ffi::c_int as THR_MODES,
        THR_NEWA as core::ffi::c_int as THR_MODES,
    ],
];
static mut ref_mode_set: [REF_MODE; 12] = [
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
];
static mut ref_mode_set_full: [REF_MODE; 28] = [
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEARMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = REF_MODE {
            ref_frame: ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            pred_mode: NEWMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
];
static mut comp_ref_mode_set: [COMP_REF_MODE; 6] = [
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
];
static mut comp_ref_mode_set_full: [COMP_REF_MODE; 10] = [
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: GLOBAL_GLOBALMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
    {
        let mut init = COMP_REF_MODE {
            ref_frame: [
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ],
            pred_mode: NEAREST_NEARESTMV as core::ffi::c_int as PREDICTION_MODE,
        };
        init
    },
];
static mut filters_ref_set: [int_interpfilters; 9] = [
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
                x_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
                x_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_REGULAR as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
                x_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
    int_interpfilters {
        as_filters: {
            let mut init = InterpFilters {
                y_filter: MULTITAP_SHARP as core::ffi::c_int as uint16_t,
                x_filter: EIGHTTAP_SMOOTH as core::ffi::c_int as uint16_t,
            };
            init
        },
    },
];
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
unsafe extern "C-unwind" fn init_mbmi_nonrd(
    mut mbmi: *mut MB_MODE_INFO,
    mut pred_mode: PREDICTION_MODE,
    mut ref_frame0: MV_REFERENCE_FRAME,
    mut ref_frame1: MV_REFERENCE_FRAME,
    mut cm: *const AV1_COMMON,
) {
    let pmi: *mut PALETTE_MODE_INFO = &raw mut (*mbmi).palette_mode_info;
    (*mbmi).set_ref_mv_idx(0 as uint8_t as uint8_t);
    (*mbmi).mode = pred_mode;
    (*mbmi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
    (*mbmi).ref_frame[0 as core::ffi::c_int as usize] = ref_frame0;
    (*mbmi).ref_frame[1 as core::ffi::c_int as usize] = ref_frame1;
    (*pmi).palette_size[PLANE_TYPE_Y as core::ffi::c_int as usize] = 0 as uint8_t;
    (*pmi).palette_size[PLANE_TYPE_UV as core::ffi::c_int as usize] = 0 as uint8_t;
    (*mbmi).set_use_intrabc(0 as uint8_t as uint8_t);
    (*mbmi).filter_intra_mode_info.use_filter_intra = 0 as uint8_t;
    (*mbmi).mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
    (*mbmi).mv[0 as core::ffi::c_int as usize].as_int = (*mbmi)
        .mv[1 as core::ffi::c_int as usize]
        .as_int;
    (*mbmi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    (*mbmi).num_proj_ref = 1 as uint8_t;
    (*mbmi).interintra_mode = 0 as INTERINTRA_MODE;
    set_default_interp_filters(mbmi, (*cm).features.interp_filter);
}
#[inline]
unsafe extern "C-unwind" fn init_estimate_block_intra_args(
    mut args: *mut estimate_block_intra_args,
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
) {
    (*args).cpi = cpi;
    (*args).x = x;
    (*args).mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
    (*args).skippable = 1 as core::ffi::c_int;
    (*args).rdc = 0 as *mut RD_STATS;
    (*args).best_sad = UINT_MAX;
    (*args).prune_mode_based_on_sad = false_0 != 0;
    (*args).prune_palette_sad = false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn get_pred_buffer(
    mut p: *mut PRED_BUFFER,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut buf_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while buf_idx < len {
        if (*p.offset(buf_idx as isize)).in_use == 0 {
            (*p.offset(buf_idx as isize)).in_use = 1 as core::ffi::c_int;
            return buf_idx;
        }
        buf_idx += 1;
    }
    return -(1 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn prune_palette_testing_inter(
    mut cpi: *mut AV1_COMP,
    mut source_variance: core::ffi::c_uint,
) -> bool {
    return (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).oxcf.speed >= 11 as core::ffi::c_int && (*cpi).rc.high_source_sad != 0
        && ((*cpi).sf.rt_sf.prune_palette_search_nonrd > 2 as core::ffi::c_int
            || (*cpi).sf.rt_sf.rc_compute_spatial_var_sc_kf != 0
                && (*cpi).rc.frame_spatial_variance < 1200 as uint64_t
                && (*cpi).rc.perc_spatial_flat_blocks < 5 as core::ffi::c_int
                && (*cpi).rc.percent_blocks_with_motion > 98 as core::ffi::c_int
                && source_variance < 4000 as core::ffi::c_uint);
}
#[inline]
unsafe extern "C-unwind" fn free_pred_buffer(mut p: *mut PRED_BUFFER) {
    if !p.is_null() {
        (*p).in_use = 0 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn store_coding_context_nonrd(
    mut x: *mut MACROBLOCK,
    mut ctx: *mut PICK_MODE_CONTEXT,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    (*ctx).rd_stats.skip_txfm = (*txfm_info).skip_txfm;
    (*ctx).skippable = (*txfm_info).skip_txfm as core::ffi::c_int;
    (*ctx).mic = **((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*ctx).skippable = (*txfm_info).skip_txfm as core::ffi::c_int;
    av1_copy_mbmi_ext_to_mbmi_ext_frame(
        &raw mut (*ctx).mbmi_ext_best,
        &raw mut (*x).mbmi_ext,
        av1_ref_frame_type(
            ((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).ref_frame)
                .as_mut_ptr(),
        ) as uint8_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn early_term_inter_search_with_sse(
    mut early_term_idx: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut this_sse: int64_t,
    mut best_sse: int64_t,
    mut this_mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    static mut early_term_thresh: [[core::ffi::c_double; 4]; 4] = [
        [0.65f64, 0.65f64, 0.65f64, 0.7f64],
        [0.6f64, 0.65f64, 0.85f64, 0.9f64],
        [0.5f64, 0.5f64, 0.55f64, 0.6f64],
        [0.6f64, 0.75f64, 0.85f64, 0.85f64],
    ];
    static mut early_term_thresh_newmv_nearestmv: [core::ffi::c_double; 4] = [
        0.3f64,
        0.3f64,
        0.3f64,
        0.3f64,
    ];
    let size_group: core::ffi::c_int = size_group_lookup[bsize as usize]
        as core::ffi::c_int;
    if !(size_group < 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"early_term_inter_search_with_sse\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            48 as core::ffi::c_int,
            b"size_group < 4\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(early_term_idx > 0 as core::ffi::c_int
        && early_term_idx < EARLY_TERM_INDICES as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"early_term_inter_search_with_sse\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            49 as core::ffi::c_int,
            b"(early_term_idx > 0) && (early_term_idx < EARLY_TERM_INDICES)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let threshold: core::ffi::c_double = if early_term_idx
        == EARLY_TERM_IDX_4 as core::ffi::c_int
        && (this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
            || this_mode as core::ffi::c_int == NEARESTMV as core::ffi::c_int)
    {
        early_term_thresh_newmv_nearestmv[size_group as usize]
    } else {
        early_term_thresh[(early_term_idx - 1 as core::ffi::c_int)
            as usize][size_group as usize]
    };
    if early_term_idx > 0 as core::ffi::c_int
        && threshold * this_sse as core::ffi::c_double > best_sse as core::ffi::c_double
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn init_best_pickmode(mut bp: *mut BEST_PICKMODE) {
    (*bp).best_sse = INT64_MAX as int64_t;
    (*bp).best_mode = NEARESTMV as core::ffi::c_int as PREDICTION_MODE;
    (*bp).best_ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    (*bp).best_second_ref_frame = NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    (*bp).best_tx_size = TX_8X8 as core::ffi::c_int as TX_SIZE;
    (*bp).tx_type = DCT_DCT as core::ffi::c_int as TX_TYPE;
    (*bp).best_pred_filter = av1_broadcast_interp_filter(EIGHTTAP_REGULAR);
    (*bp).best_mode_skip_txfm = 0 as uint8_t;
    (*bp).best_mode_initial_skip_flag = 0 as uint8_t;
    (*bp).best_pred = 0 as *mut PRED_BUFFER;
    (*bp).best_motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    (*bp).num_proj_ref = 0 as core::ffi::c_int;
    memset(
        &raw mut (*bp).wm_params as *mut WarpedMotionParams as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<WarpedMotionParams>() as size_t,
    );
    memset(
        &raw mut (*bp).pmi as *mut PALETTE_MODE_INFO as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<PALETTE_MODE_INFO>() as size_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn update_search_state_nonrd(
    mut search_state: *mut InterModeSearchStateNonrd,
    mi: *mut MB_MODE_INFO,
    mut txfm_info: *mut TxfmSearchInfo,
    mut nonskip_rdc: *mut RD_STATS,
    mut this_best_mode: PREDICTION_MODE,
    sse_y: int64_t,
) {
    let best_pickmode: *mut BEST_PICKMODE = &raw mut (*search_state).best_pickmode;
    (*best_pickmode).best_sse = sse_y;
    (*best_pickmode).best_mode = this_best_mode;
    (*best_pickmode).best_motion_mode = (*mi).motion_mode;
    (*best_pickmode).wm_params = (*mi).wm_params;
    (*best_pickmode).num_proj_ref = (*mi).num_proj_ref as core::ffi::c_int;
    (*best_pickmode).best_pred_filter = (*mi).interp_filters;
    (*best_pickmode).best_tx_size = (*mi).tx_size;
    (*best_pickmode).best_ref_frame = (*mi).ref_frame[0 as core::ffi::c_int as usize];
    (*best_pickmode).best_second_ref_frame = (*mi)
        .ref_frame[1 as core::ffi::c_int as usize];
    (*best_pickmode).best_mode_skip_txfm = (*search_state).this_rdc.skip_txfm;
    (*best_pickmode).best_mode_initial_skip_flag = ((*nonskip_rdc).rate == INT_MAX
        && (*search_state).this_rdc.skip_txfm as core::ffi::c_int != 0)
        as core::ffi::c_int as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn subpel_select(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut mv: *mut int_mv,
    mut ref_mv: MV,
    mut start_mv: FULLPEL_MV,
    mut fullpel_performed_well: bool,
) -> core::ffi::c_int {
    let frame_lowmotion: core::ffi::c_int = (*cpi).rc.avg_frame_low_motion;
    let reduce_mv_pel_precision_highmotion: core::ffi::c_int = (*cpi)
        .sf
        .rt_sf
        .reduce_mv_pel_precision_highmotion;
    if reduce_mv_pel_precision_highmotion >= 3 as core::ffi::c_int {
        let mut mv_thresh: core::ffi::c_int = 4 as core::ffi::c_int;
        let is_low_resoln: core::ffi::c_int = ((*cpi).common.width * (*cpi).common.height
            <= 320 as core::ffi::c_int * 240 as core::ffi::c_int) as core::ffi::c_int;
        mv_thresh = if bsize as core::ffi::c_int > BLOCK_32X32 as core::ffi::c_int {
            2 as core::ffi::c_int
        } else if bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int {
            4 as core::ffi::c_int
        } else {
            6 as core::ffi::c_int
        };
        if frame_lowmotion > 0 as core::ffi::c_int
            && frame_lowmotion < 40 as core::ffi::c_int
        {
            mv_thresh = 12 as core::ffi::c_int;
        }
        mv_thresh = if is_low_resoln != 0 {
            mv_thresh >> 1 as core::ffi::c_int
        } else {
            mv_thresh
        };
        if abs((*mv).as_fullmv.row as core::ffi::c_int) >= mv_thresh
            || abs((*mv).as_fullmv.col as core::ffi::c_int) >= mv_thresh
        {
            return HALF_PEL as core::ffi::c_int;
        }
    } else if reduce_mv_pel_precision_highmotion >= 1 as core::ffi::c_int {
        let mut mv_thresh_0: core::ffi::c_int = 0;
        let th_vals: [[core::ffi::c_int; 3]; 2] = [
            [4 as core::ffi::c_int, 8 as core::ffi::c_int, 10 as core::ffi::c_int],
            [4 as core::ffi::c_int, 6 as core::ffi::c_int, 8 as core::ffi::c_int],
        ];
        let th_idx: core::ffi::c_int = reduce_mv_pel_precision_highmotion
            - 1 as core::ffi::c_int;
        if !(th_idx >= 0 as core::ffi::c_int && th_idx < 2 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"subpel_select\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                124 as core::ffi::c_int,
                b"th_idx >= 0 && th_idx < 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if frame_lowmotion > 0 as core::ffi::c_int
            && frame_lowmotion < 40 as core::ffi::c_int
        {
            mv_thresh_0 = 12 as core::ffi::c_int;
        } else {
            mv_thresh_0 = if bsize as core::ffi::c_int >= BLOCK_32X32 as core::ffi::c_int
            {
                th_vals[th_idx as usize][0 as core::ffi::c_int as usize]
            } else if bsize as core::ffi::c_int >= BLOCK_16X16 as core::ffi::c_int {
                th_vals[th_idx as usize][1 as core::ffi::c_int as usize]
            } else {
                th_vals[th_idx as usize][2 as core::ffi::c_int as usize]
            };
        }
        if abs((*mv).as_fullmv.row as core::ffi::c_int)
            >= mv_thresh_0 << 1 as core::ffi::c_int
            || abs((*mv).as_fullmv.col as core::ffi::c_int)
                >= mv_thresh_0 << 1 as core::ffi::c_int
        {
            return FULL_PEL as core::ffi::c_int
        } else if abs((*mv).as_fullmv.row as core::ffi::c_int) >= mv_thresh_0
            || abs((*mv).as_fullmv.col as core::ffi::c_int) >= mv_thresh_0
        {
            return HALF_PEL as core::ffi::c_int
        }
    }
    if (*cpi).sf.rt_sf.reduce_mv_pel_precision_lowcomplex >= 2 as core::ffi::c_int {
        let qband: core::ffi::c_int = (*x).qindex >> QINDEX_BITS - 2 as core::ffi::c_int;
        if !(qband < 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"subpel_select\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                142 as core::ffi::c_int,
                b"qband < 4\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            <= kVeryLowSad as core::ffi::c_int as core::ffi::c_uint
            && bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int
            && qband != 0 as core::ffi::c_int
        {
            if (*x).source_variance < 500 as core::ffi::c_uint {
                return FULL_PEL as core::ffi::c_int
            } else if (*x).source_variance < 5000 as core::ffi::c_uint {
                return HALF_PEL as core::ffi::c_int
            }
        }
    } else if (*cpi).sf.rt_sf.reduce_mv_pel_precision_lowcomplex >= 1 as core::ffi::c_int
    {
        if fullpel_performed_well as core::ffi::c_int != 0
            && ref_mv.row as core::ffi::c_int == 0 as core::ffi::c_int
            && ref_mv.col as core::ffi::c_int == 0 as core::ffi::c_int
            && start_mv.row as core::ffi::c_int == 0 as core::ffi::c_int
            && start_mv.col as core::ffi::c_int == 0 as core::ffi::c_int
        {
            return HALF_PEL as core::ffi::c_int;
        }
    }
    return (*cpi).sf.mv_sf.subpel_force_stop as core::ffi::c_int;
}
unsafe extern "C-unwind" fn use_aggressive_subpel_search_method(
    mut x: *mut MACROBLOCK,
    mut use_adaptive_subpel_search: bool,
    mut fullpel_performed_well: bool,
) -> bool {
    if !use_adaptive_subpel_search {
        return false_0 != 0;
    }
    let qband: core::ffi::c_int = (*x).qindex >> QINDEX_BITS - 2 as core::ffi::c_int;
    if !(qband < 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"use_aggressive_subpel_search_method\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            163 as core::ffi::c_int,
            b"qband < 4\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if qband > 0 as core::ffi::c_int
        && (fullpel_performed_well as core::ffi::c_int != 0
            || (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
                <= kLowSad as core::ffi::c_int as core::ffi::c_uint
            || (*x).source_variance < 100 as core::ffi::c_uint)
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn combined_motion_search(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut tmp_mv: *mut int_mv,
    mut rate_mv: *mut core::ffi::c_int,
    mut best_rd_sofar: int64_t,
    mut use_base_mv: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut sf: *const SPEED_FEATURES = &raw mut (*cpi).sf;
    let mut mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut step_param: core::ffi::c_int = if (*sf).rt_sf.fullpel_search_step_param != 0
    {
        (*sf).rt_sf.fullpel_search_step_param
    } else {
        (*cpi).mv_search_params.mv_step_param
    };
    let mut start_mv: FULLPEL_MV = fullpel_mv { row: 0, col: 0 };
    let ref_0: core::ffi::c_int = (*mi).ref_frame[0 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let ref_mv: MV = (av1_get_ref_mv(x, (*mi).ref_mv_idx() as core::ffi::c_int)).as_mv
        as MV;
    let mut center_mv: MV = mv { row: 0, col: 0 };
    let mut dis: core::ffi::c_int = 0;
    let mut rv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut cost_list: [core::ffi::c_int; 5] = [0; 5];
    let mut search_subpel: core::ffi::c_int = 1 as core::ffi::c_int;
    if av1_is_scaled(get_ref_scale_factors_const(cm, ref_0 as MV_REFERENCE_FRAME)) != 0 {
        let mut scaled_ref: *const YV12_BUFFER_CONFIG = av1_get_scaled_ref_frame(
            cpi,
            ref_0,
        );
        if scaled_ref.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"combined_motion_search\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                216 as core::ffi::c_int,
                b"scaled_ref != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*scaled_ref).c2rust_unnamed_1.c2rust_unnamed.y_crop_width == (*cm).width
            && (*scaled_ref).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
                == (*cm).height) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"combined_motion_search\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                218 as core::ffi::c_int,
                b"scaled_ref->y_crop_width == cm->width && scaled_ref->y_crop_height == cm->height\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    start_mv = get_fullmv_from_mv(&ref_mv);
    if use_base_mv == 0 {
        center_mv = ref_mv;
    } else {
        center_mv = (*tmp_mv).as_mv;
    }
    let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(
        x,
        &raw mut (*cpi).sf.mv_sf,
        bsize,
    ) as SEARCH_METHODS;
    let mut src_search_sites: *const search_site_config = av1_get_search_site_config(
        cpi,
        x,
        search_method,
    );
    let mut full_ms_params: FULLPEL_MOTION_SEARCH_PARAMS = FULLPEL_MOTION_SEARCH_PARAMS {
        bsize: BLOCK_4X4,
        vfp: 0 as *const aom_variance_fn_ptr_t,
        ms_buffers: MSBuffers {
            ref_0: 0 as *const buf_2d,
            src: 0 as *const buf_2d,
            second_pred: 0 as *const uint8_t,
            mask: 0 as *const uint8_t,
            mask_stride: 0,
            inv_mask: 0,
            wsrc: 0 as *const int32_t,
            obmc_mask: 0 as *const int32_t,
        },
        search_method: 0,
        search_sites: 0 as *const search_site_config,
        mv_limits: FullMvLimits {
            col_min: 0,
            col_max: 0,
            row_min: 0,
            row_max: 0,
        },
        run_mesh_search: 0,
        prune_mesh_search: 0,
        mesh_search_mv_diff_threshold: 0,
        force_mesh_thresh: 0,
        mesh_patterns: [0 as *const MESH_PATTERN; 2],
        fine_search_interval: 0,
        is_intra_mode: 0,
        fast_obmc_search: 0,
        mv_cost_params: MV_COST_PARAMS {
            ref_mv: 0 as *const MV,
            full_ref_mv: fullpel_mv { row: 0, col: 0 },
            mv_cost_type: 0,
            mvjcost: 0 as *const core::ffi::c_int,
            mvcost: [0 as *const core::ffi::c_int; 2],
            error_per_bit: 0,
            sad_per_bit: 0,
        },
        sdf: None,
        sdx4df: None,
        sdx3df: None,
    };
    let mut best_mv_stats: FULLPEL_MV_STATS = FULLPEL_MV_STATS {
        err_cost: 0,
        distortion: 0,
        sse: 0,
    };
    av1_make_default_fullpel_ms_params(
        &raw mut full_ms_params,
        cpi,
        x,
        bsize,
        &raw mut center_mv,
        start_mv,
        src_search_sites as *const search_site_config,
        search_method,
        0 as core::ffi::c_int,
    );
    let full_var_rd: core::ffi::c_uint = av1_full_pixel_search(
        start_mv,
        &raw mut full_ms_params,
        step_param,
        cond_cost_list(cpi, cost_list.as_mut_ptr()),
        &raw mut (*tmp_mv).as_fullmv,
        &raw mut best_mv_stats,
        0 as *mut FULLPEL_MV,
    ) as core::ffi::c_uint;
    let mut mvp_full: MV = get_mv_from_fullmv(&raw mut (*tmp_mv).as_fullmv);
    *rate_mv = av1_mv_bit_cost(
        &raw mut mvp_full,
        &ref_mv,
        ((*(*x).mv_costs).nmv_joint_cost).as_mut_ptr(),
        (*(*x).mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
        MV_COST_WEIGHT,
    );
    rv = !((*rate_mv as int64_t * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (0 as core::ffi::c_int * ((1 as core::ffi::c_int) << RDDIV_BITS)) as int64_t
        > best_rd_sofar) as core::ffi::c_int;
    if rv != 0 && search_subpel != 0 {
        let mut ms_params: SUBPEL_MOTION_SEARCH_PARAMS = SUBPEL_MOTION_SEARCH_PARAMS {
            allow_hp: 0,
            cost_list: 0 as *const core::ffi::c_int,
            forced_stop: 0,
            iters_per_step: 0,
            mv_limits: SubpelMvLimits {
                col_min: 0,
                col_max: 0,
                row_min: 0,
                row_max: 0,
            },
            mv_cost_params: MV_COST_PARAMS {
                ref_mv: 0 as *const MV,
                full_ref_mv: fullpel_mv { row: 0, col: 0 },
                mv_cost_type: 0,
                mvjcost: 0 as *const core::ffi::c_int,
                mvcost: [0 as *const core::ffi::c_int; 2],
                error_per_bit: 0,
                sad_per_bit: 0,
            },
            var_params: SUBPEL_SEARCH_VAR_PARAMS {
                vfp: 0 as *const aom_variance_fn_ptr_t,
                subpel_search_type: 0,
                ms_buffers: MSBuffers {
                    ref_0: 0 as *const buf_2d,
                    src: 0 as *const buf_2d,
                    second_pred: 0 as *const uint8_t,
                    mask: 0 as *const uint8_t,
                    mask_stride: 0,
                    inv_mask: 0,
                    wsrc: 0 as *const int32_t,
                    obmc_mask: 0 as *const int32_t,
                },
                w: 0,
                h: 0,
            },
        };
        av1_make_default_subpel_ms_params(
            &raw mut ms_params,
            cpi,
            x,
            bsize,
            &ref_mv,
            cost_list.as_mut_ptr(),
        );
        let fullpel_performed_well: bool = bsize as core::ffi::c_int
            == BLOCK_64X64 as core::ffi::c_int
            && full_var_rd.wrapping_mul(40 as core::ffi::c_uint)
                < (62267 as core::ffi::c_int * 7 as core::ffi::c_int)
                    as core::ffi::c_uint
            || bsize as core::ffi::c_int == BLOCK_32X32 as core::ffi::c_int
                && full_var_rd.wrapping_mul(8 as core::ffi::c_uint)
                    < 42380 as core::ffi::c_uint
            || bsize as core::ffi::c_int == BLOCK_16X16 as core::ffi::c_int
                && full_var_rd.wrapping_mul(8 as core::ffi::c_uint)
                    < 10127 as core::ffi::c_uint;
        if (*sf).rt_sf.reduce_mv_pel_precision_highmotion != 0
            || (*sf).rt_sf.reduce_mv_pel_precision_lowcomplex != 0
        {
            ms_params.forced_stop = subpel_select(
                cpi,
                x,
                bsize,
                tmp_mv,
                ref_mv,
                start_mv,
                fullpel_performed_well,
            ) as SUBPEL_FORCE_STOP;
        }
        let mut subpel_start_mv: MV = get_mv_from_fullmv(&raw mut (*tmp_mv).as_fullmv);
        if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, subpel_start_mv) == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"combined_motion_search\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                265 as core::ffi::c_int,
                b"av1_is_subpelmv_in_range(&ms_params.mv_limits, subpel_start_mv)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if use_aggressive_subpel_search_method(
            x,
            (*sf).rt_sf.use_adaptive_subpel_search,
            fullpel_performed_well,
        ) {
            av1_find_best_sub_pixel_tree_pruned_more(
                xd,
                cm,
                &raw mut ms_params,
                subpel_start_mv,
                &raw mut best_mv_stats,
                &raw mut (*tmp_mv).as_mv,
                &raw mut dis,
                &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
                0 as *mut int_mv,
            );
        } else {
            ((*cpi).mv_search_params.find_fractional_mv_step)
                .expect(
                    "non-null function pointer",
                )(
                xd,
                cm,
                &raw mut ms_params,
                subpel_start_mv,
                &raw mut best_mv_stats,
                &raw mut (*tmp_mv).as_mv,
                &raw mut dis,
                &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
                0 as *mut int_mv,
            );
        }
        *rate_mv = av1_mv_bit_cost(
            &raw mut (*tmp_mv).as_mv,
            &ref_mv,
            ((*(*x).mv_costs).nmv_joint_cost).as_mut_ptr(),
            (*(*x).mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
            MV_COST_WEIGHT,
        );
    }
    rv = ((*tmp_mv).as_mv.col as core::ffi::c_int != ref_mv.col as core::ffi::c_int
        || (*tmp_mv).as_mv.row as core::ffi::c_int != ref_mv.row as core::ffi::c_int)
        as core::ffi::c_int;
    return rv;
}
unsafe extern "C-unwind" fn search_new_mv(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut frame_mv: *mut [int_mv; 8],
    mut ref_frame: MV_REFERENCE_FRAME,
    mut gf_temporal_ref: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut best_rdc: *mut RD_STATS,
) -> core::ffi::c_int {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut this_ref_frm_newmv: *mut int_mv = &raw mut *(*frame_mv
        .offset(NEWMV as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(ref_frame as isize) as *mut int_mv;
    let mut y_sad_zero: core::ffi::c_uint = 0;
    if ref_frame as core::ffi::c_int > LAST_FRAME as core::ffi::c_int
        && (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).ref_frame_flags & AOM_LAST_FLAG as core::ffi::c_int != 0
        && gf_temporal_ref != 0 && is_one_pass_rt_lag_params(cpi) == 0
    {
        let mut tmp_sad: core::ffi::c_int = 0;
        let mut dis: core::ffi::c_int = 0;
        if (bsize as core::ffi::c_int) < BLOCK_16X16 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        let mut me_search_size_col: core::ffi::c_int = block_size_wide[bsize as usize]
            as core::ffi::c_int >> 1 as core::ffi::c_int;
        let mut me_search_size_row: core::ffi::c_int = block_size_high[bsize as usize]
            as core::ffi::c_int >> 1 as core::ffi::c_int;
        let mut ref_mv: MV = (av1_get_ref_mv(x, 0 as core::ffi::c_int)).as_mv;
        tmp_sad = av1_int_pro_motion_estimation(
            cpi,
            x,
            bsize,
            mi_row,
            mi_col,
            &raw mut ref_mv,
            &raw mut y_sad_zero,
            me_search_size_col,
            me_search_size_row,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if tmp_sad > (*x).pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize] {
            return -(1 as core::ffi::c_int);
        }
        (*this_ref_frm_newmv).as_int = (*mi).mv[0 as core::ffi::c_int as usize].as_int;
        let mut best_mv: int_mv = (*mi).mv[0 as core::ffi::c_int as usize];
        best_mv.as_mv.row = (best_mv.as_mv.row as core::ffi::c_int
            >> 3 as core::ffi::c_int) as int16_t;
        best_mv.as_mv.col = (best_mv.as_mv.col as core::ffi::c_int
            >> 3 as core::ffi::c_int) as int16_t;
        (*this_ref_frm_newmv).as_mv.row = ((*this_ref_frm_newmv).as_mv.row
            as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
        (*this_ref_frm_newmv).as_mv.col = ((*this_ref_frm_newmv).as_mv.col
            as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
        let mut ms_params: SUBPEL_MOTION_SEARCH_PARAMS = SUBPEL_MOTION_SEARCH_PARAMS {
            allow_hp: 0,
            cost_list: 0 as *const core::ffi::c_int,
            forced_stop: 0,
            iters_per_step: 0,
            mv_limits: SubpelMvLimits {
                col_min: 0,
                col_max: 0,
                row_min: 0,
                row_max: 0,
            },
            mv_cost_params: MV_COST_PARAMS {
                ref_mv: 0 as *const MV,
                full_ref_mv: fullpel_mv { row: 0, col: 0 },
                mv_cost_type: 0,
                mvjcost: 0 as *const core::ffi::c_int,
                mvcost: [0 as *const core::ffi::c_int; 2],
                error_per_bit: 0,
                sad_per_bit: 0,
            },
            var_params: SUBPEL_SEARCH_VAR_PARAMS {
                vfp: 0 as *const aom_variance_fn_ptr_t,
                subpel_search_type: 0,
                ms_buffers: MSBuffers {
                    ref_0: 0 as *const buf_2d,
                    src: 0 as *const buf_2d,
                    second_pred: 0 as *const uint8_t,
                    mask: 0 as *const uint8_t,
                    mask_stride: 0,
                    inv_mask: 0,
                    wsrc: 0 as *const int32_t,
                    obmc_mask: 0 as *const int32_t,
                },
                w: 0,
                h: 0,
            },
        };
        av1_make_default_subpel_ms_params(
            &raw mut ms_params,
            cpi,
            x,
            bsize,
            &raw mut ref_mv,
            0 as *const core::ffi::c_int,
        );
        if (*cpi).sf.rt_sf.reduce_mv_pel_precision_highmotion != 0
            || (*cpi).sf.rt_sf.reduce_mv_pel_precision_lowcomplex != 0
        {
            let mut start_mv: FULLPEL_MV = {
                let mut init = fullpel_mv {
                    row: 0 as int16_t,
                    col: 0 as int16_t,
                };
                init
            };
            ms_params.forced_stop = subpel_select(
                cpi,
                x,
                bsize,
                &raw mut best_mv,
                ref_mv,
                start_mv,
                false_0 != 0,
            ) as SUBPEL_FORCE_STOP;
        }
        let mut start_mv_0: MV = get_mv_from_fullmv(&raw mut best_mv.as_fullmv);
        if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, start_mv_0) == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"search_new_mv\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                357 as core::ffi::c_int,
                b"av1_is_subpelmv_in_range(&ms_params.mv_limits, start_mv)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        ((*cpi).mv_search_params.find_fractional_mv_step)
            .expect(
                "non-null function pointer",
            )(
            xd,
            cm,
            &raw mut ms_params,
            start_mv_0,
            0 as *const FULLPEL_MV_STATS,
            &raw mut best_mv.as_mv,
            &raw mut dis,
            &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_frame as isize),
            0 as *mut int_mv,
        );
        (*this_ref_frm_newmv).as_int = best_mv.as_int;
        if (*this_ref_frm_newmv).as_mv.col as core::ffi::c_int
            == ref_mv.col as core::ffi::c_int
            && (*this_ref_frm_newmv).as_mv.row as core::ffi::c_int
                == ref_mv.row as core::ffi::c_int
        {
            return -(1 as core::ffi::c_int);
        }
        *rate_mv = av1_mv_bit_cost(
            &raw mut (*this_ref_frm_newmv).as_mv,
            &raw mut ref_mv,
            ((*(*x).mv_costs).nmv_joint_cost).as_mut_ptr(),
            (*(*x).mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
            MV_COST_WEIGHT,
        );
    } else if combined_motion_search(
        cpi,
        x,
        bsize,
        &raw mut *(*frame_mv.offset(NEWMV as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(ref_frame as isize),
        rate_mv,
        (*best_rdc).rdcost,
        0 as core::ffi::c_int,
    ) == 0
    {
        return -(1 as core::ffi::c_int)
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn estimate_single_ref_frame_costs(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mode_costs: *const ModeCosts,
    mut segment_id: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut ref_costs_single: *mut core::ffi::c_uint,
    mut is_one_pass_rt_lag: core::ffi::c_int,
) {
    let mut seg_ref_active: core::ffi::c_int = segfeature_active(
        &(*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    );
    if seg_ref_active != 0 {
        memset(
            ref_costs_single as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (REF_FRAMES as core::ffi::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as size_t),
        );
    } else {
        let mut intra_inter_ctx: core::ffi::c_int = av1_get_intra_inter_context(xd);
        *ref_costs_single.offset(INTRA_FRAME as core::ffi::c_int as isize) = (*mode_costs)
            .intra_inter_cost[intra_inter_ctx as usize][0 as core::ffi::c_int as usize]
            as core::ffi::c_uint;
        let mut base_cost: core::ffi::c_uint = (*mode_costs)
            .intra_inter_cost[intra_inter_ctx as usize][1 as core::ffi::c_int as usize]
            as core::ffi::c_uint;
        if (*cm).current_frame.reference_mode as core::ffi::c_int
            == REFERENCE_MODE_SELECT as core::ffi::c_int
            && is_comp_ref_allowed(bsize) != 0
        {
            let comp_ref_type_ctx: core::ffi::c_int = av1_get_comp_reference_type_context(
                xd,
            ) as core::ffi::c_int;
            base_cost = base_cost
                .wrapping_add(
                    (*mode_costs)
                        .comp_ref_type_cost[comp_ref_type_ctx
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
        }
        *ref_costs_single.offset(LAST_FRAME as core::ffi::c_int as isize) = base_cost;
        *ref_costs_single.offset(GOLDEN_FRAME as core::ffi::c_int as isize) = base_cost;
        *ref_costs_single.offset(ALTREF_FRAME as core::ffi::c_int as isize) = base_cost;
        let fresh0 = &raw mut *ref_costs_single
            .offset(LAST_FRAME as core::ffi::c_int as isize);
        *fresh0 = (*fresh0)
            .wrapping_add(
                (*mode_costs)
                    .single_ref_cost[0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
            );
        let fresh1 = &raw mut *ref_costs_single
            .offset(GOLDEN_FRAME as core::ffi::c_int as isize);
        *fresh1 = (*fresh1)
            .wrapping_add(
                (*mode_costs)
                    .single_ref_cost[0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint,
            );
        let fresh2 = &raw mut *ref_costs_single
            .offset(GOLDEN_FRAME as core::ffi::c_int as isize);
        *fresh2 = (*fresh2)
            .wrapping_add(
                (*mode_costs)
                    .single_ref_cost[0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
            );
        let fresh3 = &raw mut *ref_costs_single
            .offset(ALTREF_FRAME as core::ffi::c_int as isize);
        *fresh3 = (*fresh3)
            .wrapping_add(
                (*mode_costs)
                    .single_ref_cost[0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint,
            );
        let fresh4 = &raw mut *ref_costs_single
            .offset(ALTREF_FRAME as core::ffi::c_int as isize);
        *fresh4 = (*fresh4)
            .wrapping_add(
                (*mode_costs)
                    .single_ref_cost[0 as core::ffi::c_int
                    as usize][2 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
            );
        if is_one_pass_rt_lag != 0 {
            *ref_costs_single.offset(ALTREF2_FRAME as core::ffi::c_int as isize) = base_cost;
            *ref_costs_single.offset(BWDREF_FRAME as core::ffi::c_int as isize) = base_cost;
            *ref_costs_single.offset(LAST2_FRAME as core::ffi::c_int as isize) = base_cost;
            *ref_costs_single.offset(LAST3_FRAME as core::ffi::c_int as isize) = base_cost;
            let fresh5 = &raw mut *ref_costs_single
                .offset(LAST2_FRAME as core::ffi::c_int as isize);
            *fresh5 = (*fresh5)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
            let fresh6 = &raw mut *ref_costs_single
                .offset(LAST3_FRAME as core::ffi::c_int as isize);
            *fresh6 = (*fresh6)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
            let fresh7 = &raw mut *ref_costs_single
                .offset(BWDREF_FRAME as core::ffi::c_int as isize);
            *fresh7 = (*fresh7)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
            let fresh8 = &raw mut *ref_costs_single
                .offset(BWDREF_FRAME as core::ffi::c_int as isize);
            *fresh8 = (*fresh8)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
            let fresh9 = &raw mut *ref_costs_single
                .offset(ALTREF2_FRAME as core::ffi::c_int as isize);
            *fresh9 = (*fresh9)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
            let fresh10 = &raw mut *ref_costs_single
                .offset(ALTREF2_FRAME as core::ffi::c_int as isize);
            *fresh10 = (*fresh10)
                .wrapping_add(
                    (*mode_costs)
                        .single_ref_cost[0 as core::ffi::c_int
                        as usize][2 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] as core::ffi::c_uint,
                );
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn set_force_skip_flag(
    cpi: *const AV1_COMP,
    x: *mut MACROBLOCK,
    mut sse: core::ffi::c_uint,
    mut force_skip: *mut core::ffi::c_int,
) {
    if (*x).txfm_search_params.tx_mode_search_type as core::ffi::c_int
        == TX_MODE_SELECT as core::ffi::c_int
        && (*cpi).sf.rt_sf.tx_size_level_based_on_qstep != 0
        && (*cpi).sf.rt_sf.tx_size_level_based_on_qstep >= 2 as core::ffi::c_int
    {
        let qstep: core::ffi::c_int = *((*x).plane[AOM_PLANE_Y as usize].dequant_QTX)
            .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            >> (*x).e_mbd.bd - 5 as core::ffi::c_int;
        let qstep_sq: core::ffi::c_uint = (qstep * qstep) as core::ffi::c_uint;
        if sse < qstep_sq && (*x).source_variance < qstep_sq
            && (*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 0 as core::ffi::c_int
            && (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 0 as core::ffi::c_int
        {
            *force_skip = 1 as core::ffi::c_int;
        }
    }
}
unsafe extern "C-unwind" fn calculate_tx_size(
    cpi: *const AV1_COMP,
    mut bsize: BLOCK_SIZE,
    x: *mut MACROBLOCK,
    mut var: core::ffi::c_uint,
    mut sse: core::ffi::c_uint,
    mut force_skip: *mut core::ffi::c_int,
) -> TX_SIZE {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut tx_size: TX_SIZE = 0;
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if (*txfm_params).tx_mode_search_type as core::ffi::c_int
        == TX_MODE_SELECT as core::ffi::c_int
    {
        let mut multiplier: core::ffi::c_int = 8 as core::ffi::c_int;
        let mut var_thresh: core::ffi::c_uint = 0 as core::ffi::c_uint;
        let mut is_high_var: core::ffi::c_uint = 1 as core::ffi::c_uint;
        if (*cpi).sf.rt_sf.tx_size_level_based_on_qstep != 0 {
            let qband: core::ffi::c_int = (*x).qindex
                >> QINDEX_BITS - 2 as core::ffi::c_int;
            let mult: [core::ffi::c_int; 4] = [
                8 as core::ffi::c_int,
                7 as core::ffi::c_int,
                6 as core::ffi::c_int,
                5 as core::ffi::c_int,
            ];
            if !(qband < 4 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"calculate_tx_size\0"))
                        .as_ptr(),
                    b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                    464 as core::ffi::c_int,
                    b"qband < 4\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            multiplier = mult[qband as usize];
            let qstep: core::ffi::c_int = *((*x).plane[AOM_PLANE_Y as usize].dequant_QTX)
                .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                >> (*xd).bd - 5 as core::ffi::c_int;
            let qstep_sq: core::ffi::c_uint = (qstep * qstep) as core::ffi::c_uint;
            var_thresh = qstep_sq.wrapping_mul(2 as core::ffi::c_uint);
            if (*cpi).sf.rt_sf.tx_size_level_based_on_qstep >= 2 as core::ffi::c_int {
                if sse < qstep_sq && (*x).source_variance < qstep_sq
                    && (*x)
                        .color_sensitivity[(1 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                        == 0 as core::ffi::c_int
                    && (*x)
                        .color_sensitivity[(2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                        == 0 as core::ffi::c_int
                {
                    *force_skip = 1 as core::ffi::c_int;
                }
                is_high_var = (var >= var_thresh) as core::ffi::c_int
                    as core::ffi::c_uint;
            }
        }
        if sse
            > var.wrapping_mul(multiplier as core::ffi::c_uint) >> 2 as core::ffi::c_int
            || var < var_thresh
        {
            tx_size = (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
                < tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                    as core::ffi::c_int
            {
                max_txsize_lookup[bsize as usize] as core::ffi::c_int
            } else {
                tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                    as core::ffi::c_int
            }) as TX_SIZE;
        } else {
            tx_size = TX_8X8 as core::ffi::c_int as TX_SIZE;
        }
        if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
            == CYCLIC_REFRESH_AQ as core::ffi::c_int
            && cyclic_refresh_segment_id_boosted(
                (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
                    as core::ffi::c_int,
            ) != 0 && is_high_var != 0
        {
            tx_size = TX_8X8 as core::ffi::c_int as TX_SIZE;
        } else if tx_size as core::ffi::c_int > TX_16X16 as core::ffi::c_int {
            tx_size = TX_16X16 as core::ffi::c_int as TX_SIZE;
        }
    } else {
        tx_size = (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
            < tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                as core::ffi::c_int
        {
            max_txsize_lookup[bsize as usize] as core::ffi::c_int
        } else {
            tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                as core::ffi::c_int
        }) as TX_SIZE;
    }
    if if (*txfm_params).tx_mode_search_type as core::ffi::c_int
        != ONLY_4X4 as core::ffi::c_int
        && bsize as core::ffi::c_int > BLOCK_32X32 as core::ffi::c_int
    {
        true_0
    } else {
        false_0
    } != 0
    {
        tx_size = TX_16X16 as core::ffi::c_int as TX_SIZE;
    }
    return (if (tx_size as core::ffi::c_int) < TX_16X16 as core::ffi::c_int {
        tx_size as core::ffi::c_int
    } else {
        TX_16X16 as core::ffi::c_int
    }) as TX_SIZE;
}
unsafe extern "C-unwind" fn block_variance(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut sse8x8: *mut uint32_t,
    mut sum8x8: *mut core::ffi::c_int,
    mut var8x8: *mut uint32_t,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    *sse = 0 as core::ffi::c_uint;
    *sum = 0 as core::ffi::c_int;
    if !(w >= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"block_variance\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            522 as core::ffi::c_int,
            b"w >= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h >= 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"block_variance\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            523 as core::ffi::c_int,
            b"h >= 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < h {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < w {
            aom_get_var_sse_sum_8x8_quad_c(
                src.offset((src_stride * row) as isize).offset(col as isize),
                src_stride,
                ref_0.offset((ref_stride * row) as isize).offset(col as isize),
                ref_stride,
                &raw mut *sse8x8.offset(k as isize),
                &raw mut *sum8x8.offset(k as isize),
                sse,
                sum,
                &raw mut *var8x8.offset(k as isize),
            );
            k += 4 as core::ffi::c_int;
            col += 32 as core::ffi::c_int;
        }
        row += block_size;
    }
}
unsafe extern "C-unwind" fn block_variance_16x16_dual(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut sse16x16: *mut uint32_t,
    mut var16x16: *mut uint32_t,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    *sse = 0 as core::ffi::c_uint;
    *sum = 0 as core::ffi::c_int;
    if !(w >= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"block_variance_16x16_dual\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            547 as core::ffi::c_int,
            b"w >= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h >= 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"block_variance_16x16_dual\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            548 as core::ffi::c_int,
            b"h >= 16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < h {
        let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
        while col < w {
            aom_get_var_sse_sum_16x16_dual_c(
                src.offset((src_stride * row) as isize).offset(col as isize),
                src_stride,
                ref_0.offset((ref_stride * row) as isize).offset(col as isize),
                ref_stride,
                &raw mut *sse16x16.offset(k as isize),
                sse,
                sum,
                &raw mut *var16x16.offset(k as isize),
            );
            k += 2 as core::ffi::c_int;
            col += 32 as core::ffi::c_int;
        }
        row += block_size;
    }
}
unsafe extern "C-unwind" fn calculate_variance(
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut sse_i: *mut core::ffi::c_uint,
    mut sum_i: *mut core::ffi::c_int,
    mut var_o: *mut core::ffi::c_uint,
    mut sse_o: *mut core::ffi::c_uint,
    mut sum_o: *mut core::ffi::c_int,
) {
    let unit_size: BLOCK_SIZE = txsize_to_bsize[tx_size as usize];
    let nw: core::ffi::c_int = (1 as core::ffi::c_int)
        << bw - b_width_log2_lookup[unit_size as usize] as core::ffi::c_int;
    let nh: core::ffi::c_int = (1 as core::ffi::c_int)
        << bh - b_height_log2_lookup[unit_size as usize] as core::ffi::c_int;
    let mut row: core::ffi::c_int = 0;
    let mut col: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    row = 0 as core::ffi::c_int;
    while row < nh {
        col = 0 as core::ffi::c_int;
        while col < nw {
            *sse_o.offset(k as isize) = (*sse_i.offset((row * nw + col) as isize))
                .wrapping_add(
                    *sse_i.offset((row * nw + col + 1 as core::ffi::c_int) as isize),
                )
                .wrapping_add(
                    *sse_i.offset(((row + 1 as core::ffi::c_int) * nw + col) as isize),
                )
                .wrapping_add(
                    *sse_i
                        .offset(
                            ((row + 1 as core::ffi::c_int) * nw + col
                                + 1 as core::ffi::c_int) as isize,
                        ),
                );
            *sum_o.offset(k as isize) = *sum_i.offset((row * nw + col) as isize)
                + *sum_i.offset((row * nw + col + 1 as core::ffi::c_int) as isize)
                + *sum_i.offset(((row + 1 as core::ffi::c_int) * nw + col) as isize)
                + *sum_i
                    .offset(
                        ((row + 1 as core::ffi::c_int) * nw + col
                            + 1 as core::ffi::c_int) as isize,
                    );
            *var_o.offset(k as isize) = (*sse_o.offset(k as isize))
                .wrapping_sub(
                    (*sum_o.offset(k as isize) as int64_t
                        * *sum_o.offset(k as isize) as int64_t
                        >> b_width_log2_lookup[unit_size as usize] as core::ffi::c_int
                            + b_height_log2_lookup[unit_size as usize]
                                as core::ffi::c_int + 6 as core::ffi::c_int) as uint32_t,
                ) as core::ffi::c_uint;
            k += 1;
            col += 2 as core::ffi::c_int;
        }
        row += 2 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn ac_thr_factor(
    mut speed: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut norm_sum: core::ffi::c_int,
) -> core::ffi::c_int {
    if speed >= 8 as core::ffi::c_int && norm_sum < 5 as core::ffi::c_int {
        if width <= 640 as core::ffi::c_int && height <= 480 as core::ffi::c_int {
            return 4 as core::ffi::c_int
        } else {
            return 2 as core::ffi::c_int
        }
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_early_term_based_on_uv_plane(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut early_term: *mut core::ffi::c_int,
    mut num_blk: core::ffi::c_int,
    mut sse_tx: *const core::ffi::c_uint,
    mut var_tx: *const core::ffi::c_uint,
    mut sum: core::ffi::c_int,
    mut var: core::ffi::c_uint,
    mut sse: core::ffi::c_uint,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblock_plane;
    let dc_quant: uint32_t = *((*p).dequant_QTX).offset(0 as core::ffi::c_int as isize)
        as uint32_t;
    let ac_quant: uint32_t = *((*p).dequant_QTX).offset(1 as core::ffi::c_int as isize)
        as uint32_t;
    let mut dc_thr: int64_t = (dc_quant.wrapping_mul(dc_quant) >> 6 as core::ffi::c_int)
        as int64_t;
    let mut ac_thr: int64_t = (ac_quant.wrapping_mul(ac_quant) >> 6 as core::ffi::c_int)
        as int64_t;
    let bw: core::ffi::c_int = b_width_log2_lookup[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
    let mut ac_test: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut dc_test: core::ffi::c_int = 1 as core::ffi::c_int;
    let norm_sum: core::ffi::c_int = abs(sum) as core::ffi::c_int >> bw + bh;
    ac_thr
        *= ac_thr_factor((*cpi).oxcf.speed, (*cm).width, (*cm).height, norm_sum)
            as int64_t;
    if (*cpi).sf.rt_sf.increase_source_sad_thresh != 0 {
        dc_thr = dc_thr << 1 as core::ffi::c_int;
        ac_thr = ac_thr << 2 as core::ffi::c_int;
    }
    if (*cpi).common.width * (*cpi).common.height
        >= 1280 as core::ffi::c_int * 720 as core::ffi::c_int
        && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
            != AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            > kLowSad as core::ffi::c_int as core::ffi::c_uint
        && sse >> bw + bh > 1000 as core::ffi::c_uint
    {
        dc_thr = dc_thr >> 4 as core::ffi::c_int;
        ac_thr = ac_thr >> 4 as core::ffi::c_int;
    }
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < num_blk {
        if !((*var_tx.offset(k as isize) as int64_t) < ac_thr
            || var == 0 as core::ffi::c_uint)
        {
            ac_test = 0 as core::ffi::c_int;
            break;
        } else if !(((*sse_tx.offset(k as isize))
            .wrapping_sub(*var_tx.offset(k as isize)) as int64_t) < dc_thr || sse == var)
        {
            dc_test = 0 as core::ffi::c_int;
            break;
        } else {
            k += 1;
        }
    }
    if ac_test != 0 && dc_test != 0 {
        let mut skip_uv: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
        let mut var_uv: [core::ffi::c_uint; 2] = [0; 2];
        let mut sse_uv: [core::ffi::c_uint; 2] = [0; 2];
        let mut plane: core::ffi::c_int = AOM_PLANE_U;
        while plane <= AOM_PLANE_V {
            let mut j: core::ffi::c_int = plane - 1 as core::ffi::c_int;
            skip_uv[j as usize] = 1 as core::ffi::c_int;
            if (*x).color_sensitivity[(plane - 1 as core::ffi::c_int) as usize] != 0 {
                skip_uv[j as usize] = 0 as core::ffi::c_int;
                let puv: *mut macroblock_plane = &raw mut *((*x).plane)
                    .as_mut_ptr()
                    .offset(plane as isize) as *mut macroblock_plane;
                let puvd: *mut macroblockd_plane = &raw mut *((*xd).plane)
                    .as_mut_ptr()
                    .offset(plane as isize) as *mut macroblockd_plane;
                let uv_bsize: BLOCK_SIZE = get_plane_block_size(
                    bsize,
                    (*puvd).subsampling_x,
                    (*puvd).subsampling_y,
                ) as BLOCK_SIZE;
                let shift_ac: core::ffi::c_int = if (*cpi)
                    .sf
                    .rt_sf
                    .increase_source_sad_thresh != 0
                {
                    5 as core::ffi::c_int
                } else {
                    3 as core::ffi::c_int
                };
                let shift_dc: core::ffi::c_int = if (*cpi)
                    .sf
                    .rt_sf
                    .increase_source_sad_thresh != 0
                {
                    4 as core::ffi::c_int
                } else {
                    3 as core::ffi::c_int
                };
                let uv_dc_thr: int64_t = (*((*puv).dequant_QTX)
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *((*puv).dequant_QTX).offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int >> shift_dc) as int64_t;
                let uv_ac_thr: int64_t = (*((*puv).dequant_QTX)
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    * *((*puv).dequant_QTX).offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int >> shift_ac) as int64_t;
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    plane,
                    plane,
                );
                var_uv[j as usize] = ((*(*cpi).ppi).fn_ptr[uv_bsize as usize].vf)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*puv).src.buf,
                    (*puv).src.stride,
                    (*puvd).dst.buf,
                    (*puvd).dst.stride,
                    &raw mut *sse_uv.as_mut_ptr().offset(j as isize),
                );
                if !(((var_uv[j as usize] as int64_t) < uv_ac_thr
                    || var_uv[j as usize] == 0 as core::ffi::c_uint)
                    && (((sse_uv[j as usize]).wrapping_sub(var_uv[j as usize])
                        as int64_t) < uv_dc_thr
                        || sse_uv[j as usize] == var_uv[j as usize]))
                {
                    break;
                }
                skip_uv[j as usize] = 1 as core::ffi::c_int;
            }
            plane += 1;
        }
        if skip_uv[0 as core::ffi::c_int as usize]
            & skip_uv[1 as core::ffi::c_int as usize] != 0
        {
            *early_term = 1 as core::ffi::c_int;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn calc_rate_dist_block_param(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_stats: *mut RD_STATS,
    mut calculate_rd: core::ffi::c_int,
    mut early_term: *mut core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut sse: core::ffi::c_uint,
) {
    if calculate_rd != 0 {
        if *early_term == 0 {
            let bw: core::ffi::c_int = block_size_wide[bsize as usize]
                as core::ffi::c_int;
            let bh: core::ffi::c_int = block_size_high[bsize as usize]
                as core::ffi::c_int;
            model_rd_with_curvfit(
                cpi,
                x,
                bsize,
                AOM_PLANE_Y,
                (*rd_stats).sse,
                bw * bh,
                &raw mut (*rd_stats).rate,
                &raw mut (*rd_stats).dist,
            );
        }
        if *early_term != 0 {
            (*rd_stats).rate = 0 as core::ffi::c_int;
            (*rd_stats).dist = (sse << 4 as core::ffi::c_int) as int64_t;
        }
    }
}
unsafe extern "C-unwind" fn model_skip_for_sb_y_large_64(
    mut cpi: *mut AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut rd_stats: *mut RD_STATS,
    mut early_term: *mut core::ffi::c_int,
    mut calculate_rd: core::ffi::c_int,
    mut best_sse: int64_t,
    mut var_output: *mut core::ffi::c_uint,
    mut var_prune_threshold: core::ffi::c_uint,
) {
    let mut sse: core::ffi::c_uint = 0;
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblock_plane;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut test_skip: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut var: core::ffi::c_uint = 0;
    let mut sum: core::ffi::c_int = 0;
    let bw: core::ffi::c_int = b_width_log2_lookup[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
    let mut sse16x16: [core::ffi::c_uint; 64] = [
        0 as core::ffi::c_int as core::ffi::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut var16x16: [core::ffi::c_uint; 64] = [
        0 as core::ffi::c_int as core::ffi::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    if !((**((*xd).mi).offset(0 as core::ffi::c_int as isize)).tx_size
        as core::ffi::c_int == TX_16X16 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"model_skip_for_sb_y_large_64\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            729 as core::ffi::c_int,
            b"xd->mi[0]->tx_size == TX_16X16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(bsize as core::ffi::c_int > BLOCK_32X32 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"model_skip_for_sb_y_large_64\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            730 as core::ffi::c_int,
            b"bsize > BLOCK_32X32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    block_variance_16x16_dual(
        (*p).src.buf,
        (*p).src.stride,
        (*pd).dst.buf,
        (*pd).dst.stride,
        (4 as core::ffi::c_int) << bw,
        (4 as core::ffi::c_int) << bh,
        &raw mut sse,
        &raw mut sum,
        16 as core::ffi::c_int,
        sse16x16.as_mut_ptr() as *mut uint32_t,
        var16x16.as_mut_ptr() as *mut uint32_t,
    );
    var = sse
        .wrapping_sub(
            (sum as int64_t * sum as int64_t >> bw + bh + 4 as core::ffi::c_int)
                as core::ffi::c_uint,
        );
    if !var_output.is_null() {
        *var_output = var;
        if *var_output > var_prune_threshold {
            return;
        }
    }
    (*rd_stats).sse = sse as int64_t;
    *early_term = 0 as core::ffi::c_int;
    set_force_skip_flag(cpi, x, sse, early_term);
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if calculate_rd == 0
        && (*cpi).sf.rt_sf.sse_early_term_inter_search as core::ffi::c_uint != 0
        && early_term_inter_search_with_sse(
            (*cpi).sf.rt_sf.sse_early_term_inter_search as core::ffi::c_int,
            bsize,
            sse as int64_t,
            best_sse,
            (*mi).mode,
        ) != 0
    {
        test_skip = 0 as core::ffi::c_int;
    }
    if *early_term != 0 {
        test_skip = 0 as core::ffi::c_int;
    }
    if test_skip != 0 {
        let mut sse_tx: *const core::ffi::c_uint = sse16x16.as_mut_ptr();
        let mut var_tx: *const core::ffi::c_uint = var16x16.as_mut_ptr();
        let num_block: core::ffi::c_uint = ((1 as core::ffi::c_int)
            << bw + bh - 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
            as core::ffi::c_uint;
        set_early_term_based_on_uv_plane(
            cpi,
            x,
            bsize,
            xd,
            mi_row,
            mi_col,
            early_term,
            num_block as core::ffi::c_int,
            sse_tx,
            var_tx,
            sum,
            var,
            sse,
        );
    }
    calc_rate_dist_block_param(cpi, x, rd_stats, calculate_rd, early_term, bsize, sse);
}
unsafe extern "C-unwind" fn model_skip_for_sb_y_large(
    mut cpi: *mut AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut rd_stats: *mut RD_STATS,
    mut early_term: *mut core::ffi::c_int,
    mut calculate_rd: core::ffi::c_int,
    mut best_sse: int64_t,
    mut var_output: *mut core::ffi::c_uint,
    mut var_prune_threshold: core::ffi::c_uint,
) {
    if (*x).force_zeromv_skip_for_blk != 0 {
        *early_term = 1 as core::ffi::c_int;
        (*rd_stats).rate = 0 as core::ffi::c_int;
        (*rd_stats).dist = 0 as int64_t;
        (*rd_stats).sse = 0 as int64_t;
        return;
    }
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    if if (*txfm_params).tx_mode_search_type as core::ffi::c_int
        != ONLY_4X4 as core::ffi::c_int
        && bsize as core::ffi::c_int > BLOCK_32X32 as core::ffi::c_int
    {
        true_0
    } else {
        false_0
    } != 0
    {
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).tx_size = TX_16X16
            as core::ffi::c_int as TX_SIZE;
        model_skip_for_sb_y_large_64(
            cpi,
            bsize,
            mi_row,
            mi_col,
            x,
            xd,
            rd_stats,
            early_term,
            calculate_rd,
            best_sse,
            var_output,
            var_prune_threshold,
        );
        return;
    }
    let mut sse: core::ffi::c_uint = 0;
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblock_plane;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut test_skip: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut var: core::ffi::c_uint = 0;
    let mut sum: core::ffi::c_int = 0;
    let bw: core::ffi::c_int = b_width_log2_lookup[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
    let mut sse8x8: [core::ffi::c_uint; 256] = [
        0 as core::ffi::c_int as core::ffi::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut sum8x8: [core::ffi::c_int; 256] = [0 as core::ffi::c_int; 256];
    let mut var8x8: [core::ffi::c_uint; 256] = [
        0 as core::ffi::c_int as core::ffi::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut tx_size: TX_SIZE = 0;
    block_variance(
        (*p).src.buf,
        (*p).src.stride,
        (*pd).dst.buf,
        (*pd).dst.stride,
        (4 as core::ffi::c_int) << bw,
        (4 as core::ffi::c_int) << bh,
        &raw mut sse,
        &raw mut sum,
        8 as core::ffi::c_int,
        sse8x8.as_mut_ptr() as *mut uint32_t,
        sum8x8.as_mut_ptr(),
        var8x8.as_mut_ptr() as *mut uint32_t,
    );
    var = sse
        .wrapping_sub(
            (sum as int64_t * sum as int64_t >> bw + bh + 4 as core::ffi::c_int)
                as core::ffi::c_uint,
        );
    if !var_output.is_null() {
        *var_output = var;
        if *var_output > var_prune_threshold {
            return;
        }
    }
    (*rd_stats).sse = sse as int64_t;
    *early_term = 0 as core::ffi::c_int;
    tx_size = calculate_tx_size(cpi, bsize, x, var, sse, early_term);
    if !(tx_size as core::ffi::c_int <= TX_16X16 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"model_skip_for_sb_y_large\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            835 as core::ffi::c_int,
            b"tx_size <= TX_16X16\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (tx_size as core::ffi::c_int) < TX_8X8 as core::ffi::c_int {
        tx_size = TX_8X8 as core::ffi::c_int as TX_SIZE;
    }
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).tx_size = tx_size;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if calculate_rd == 0
        && (*cpi).sf.rt_sf.sse_early_term_inter_search as core::ffi::c_uint != 0
        && early_term_inter_search_with_sse(
            (*cpi).sf.rt_sf.sse_early_term_inter_search as core::ffi::c_int,
            bsize,
            sse as int64_t,
            best_sse,
            (*mi).mode,
        ) != 0
    {
        test_skip = 0 as core::ffi::c_int;
    }
    if *early_term != 0 {
        test_skip = 0 as core::ffi::c_int;
    }
    if test_skip != 0 {
        let mut sse16x16: [core::ffi::c_uint; 64] = [
            0 as core::ffi::c_int as core::ffi::c_uint,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let mut sum16x16: [core::ffi::c_int; 64] = [0 as core::ffi::c_int; 64];
        let mut var16x16: [core::ffi::c_uint; 64] = [
            0 as core::ffi::c_int as core::ffi::c_uint,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let mut sse_tx: *const core::ffi::c_uint = sse8x8.as_mut_ptr();
        let mut var_tx: *const core::ffi::c_uint = var8x8.as_mut_ptr();
        let mut num_blks: core::ffi::c_uint = ((1 as core::ffi::c_int)
            << bw + bh - 2 as core::ffi::c_int) as core::ffi::c_uint;
        if tx_size as core::ffi::c_int >= TX_16X16 as core::ffi::c_int {
            calculate_variance(
                bw,
                bh,
                TX_8X8 as core::ffi::c_int as TX_SIZE,
                sse8x8.as_mut_ptr(),
                sum8x8.as_mut_ptr(),
                var16x16.as_mut_ptr(),
                sse16x16.as_mut_ptr(),
                sum16x16.as_mut_ptr(),
            );
            sse_tx = sse16x16.as_mut_ptr();
            var_tx = var16x16.as_mut_ptr();
            num_blks = num_blks >> 2 as core::ffi::c_int;
        }
        set_early_term_based_on_uv_plane(
            cpi,
            x,
            bsize,
            xd,
            mi_row,
            mi_col,
            early_term,
            num_blks as core::ffi::c_int,
            sse_tx,
            var_tx,
            sum,
            var,
            sse,
        );
    }
    calc_rate_dist_block_param(cpi, x, rd_stats, calculate_rd, early_term, bsize, sse);
}
unsafe extern "C-unwind" fn model_rd_for_sb_y(
    cpi: *const AV1_COMP,
    mut bsize: BLOCK_SIZE,
    mut x: *mut MACROBLOCK,
    mut xd: *mut MACROBLOCKD,
    mut rd_stats: *mut RD_STATS,
    mut var_out: *mut core::ffi::c_uint,
    mut calculate_rd: core::ffi::c_int,
    mut early_term: *mut core::ffi::c_int,
) {
    if (*x).force_zeromv_skip_for_blk != 0 && !early_term.is_null() {
        *early_term = 1 as core::ffi::c_int;
        (*rd_stats).rate = 0 as core::ffi::c_int;
        (*rd_stats).dist = 0 as int64_t;
        (*rd_stats).sse = 0 as int64_t;
    }
    let ref_0: core::ffi::c_int = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"model_rd_for_sb_y\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            890 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblock_plane;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut sse: core::ffi::c_uint = 0;
    let mut rate: core::ffi::c_int = 0;
    let mut dist: int64_t = 0;
    let mut var: core::ffi::c_uint = ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
        .expect(
            "non-null function pointer",
        )((*p).src.buf, (*p).src.stride, (*pd).dst.buf, (*pd).dst.stride, &raw mut sse);
    let mut force_skip: core::ffi::c_int = 0 as core::ffi::c_int;
    (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).tx_size = calculate_tx_size(
        cpi,
        bsize,
        x,
        var,
        sse,
        &raw mut force_skip,
    );
    if !var_out.is_null() {
        *var_out = var;
    }
    if calculate_rd != 0 && (force_skip == 0 || ref_0 == INTRA_FRAME as core::ffi::c_int)
    {
        let bwide: core::ffi::c_int = block_size_wide[bsize as usize]
            as core::ffi::c_int;
        let bhigh: core::ffi::c_int = block_size_high[bsize as usize]
            as core::ffi::c_int;
        model_rd_with_curvfit(
            cpi,
            x,
            bsize,
            AOM_PLANE_Y,
            sse as int64_t,
            bwide * bhigh,
            &raw mut rate,
            &raw mut dist,
        );
    } else {
        rate = INT_MAX;
        dist = INT_MAX as int64_t;
    }
    (*rd_stats).sse = sse as int64_t;
    (*x).pred_sse[ref_0 as usize] = if sse
        < (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
    {
        sse
    } else {
        (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
    };
    if force_skip != 0 && ref_0 > INTRA_FRAME as core::ffi::c_int {
        rate = 0 as core::ffi::c_int;
        dist = (sse as int64_t) << 4 as core::ffi::c_int;
    }
    if !(rate >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"model_rd_for_sb_y\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            923 as core::ffi::c_int,
            b"rate >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*rd_stats).skip_txfm = (rate == 0 as core::ffi::c_int) as core::ffi::c_int
        as uint8_t;
    rate = if rate < 2147483647 as core::ffi::c_int {
        rate
    } else {
        2147483647 as core::ffi::c_int
    };
    (*rd_stats).rate = rate;
    (*rd_stats).dist = dist;
}
#[inline]
unsafe extern "C-unwind" fn get_drl_cost(
    mut this_mode: PREDICTION_MODE,
    mut ref_mv_idx: core::ffi::c_int,
    mut mbmi_ext: *const MB_MODE_INFO_EXT,
    drl_mode_cost0: *const [core::ffi::c_int; 2],
    mut ref_frame_type: int8_t,
) -> core::ffi::c_int {
    let mut cost: core::ffi::c_int = 0 as core::ffi::c_int;
    if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        || this_mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
    {
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < 2 as core::ffi::c_int {
            if (*mbmi_ext).ref_mv_count[ref_frame_type as usize] as core::ffi::c_int
                > idx + 1 as core::ffi::c_int
            {
                let mut drl_ctx: uint8_t = av1_drl_ctx(
                    ((*mbmi_ext).weight[ref_frame_type as usize]).as_ptr(),
                    idx,
                );
                cost
                    += (*drl_mode_cost0
                        .offset(
                            drl_ctx as isize,
                        ))[(ref_mv_idx != idx) as core::ffi::c_int as usize];
                if ref_mv_idx == idx {
                    return cost;
                }
            }
            idx += 1;
        }
        return cost;
    }
    if have_nearmv_in_inter_mode(this_mode) != 0 {
        let mut idx_0: core::ffi::c_int = 1 as core::ffi::c_int;
        while idx_0 < 3 as core::ffi::c_int {
            if (*mbmi_ext).ref_mv_count[ref_frame_type as usize] as core::ffi::c_int
                > idx_0 + 1 as core::ffi::c_int
            {
                let mut drl_ctx_0: uint8_t = av1_drl_ctx(
                    ((*mbmi_ext).weight[ref_frame_type as usize]).as_ptr(),
                    idx_0,
                );
                cost
                    += (*drl_mode_cost0
                        .offset(
                            drl_ctx_0 as isize,
                        ))[(ref_mv_idx != idx_0 - 1 as core::ffi::c_int)
                        as core::ffi::c_int as usize];
                if ref_mv_idx == idx_0 - 1 as core::ffi::c_int {
                    return cost;
                }
            }
            idx_0 += 1;
        }
        return cost;
    }
    return cost;
}
unsafe extern "C-unwind" fn cost_mv_ref(
    mode_costs: *const ModeCosts,
    mut mode: PREDICTION_MODE,
    mut mode_context: int16_t,
) -> core::ffi::c_int {
    if is_inter_compound_mode(mode) != 0 {
        return (*mode_costs)
            .inter_compound_mode_cost[mode_context
            as usize][(mode as core::ffi::c_int - NEAREST_NEARESTMV as core::ffi::c_int)
            as uint8_t as usize];
    }
    let mut mode_cost: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mode_ctx: int16_t = (mode_context as core::ffi::c_int & NEWMV_CTX_MASK)
        as int16_t;
    if (is_inter_mode(mode) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"cost_mv_ref\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            970 as core::ffi::c_int,
            b"is_inter_mode(mode)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
        mode_cost = (*mode_costs)
            .newmv_mode_cost[mode_ctx as usize][0 as core::ffi::c_int as usize];
        return mode_cost;
    } else {
        mode_cost = (*mode_costs)
            .newmv_mode_cost[mode_ctx as usize][1 as core::ffi::c_int as usize];
        mode_ctx = (mode_context as core::ffi::c_int >> GLOBALMV_OFFSET
            & GLOBALMV_CTX_MASK) as int16_t;
        if mode as core::ffi::c_int == GLOBALMV as core::ffi::c_int {
            mode_cost
                += (*mode_costs)
                    .zeromv_mode_cost[mode_ctx as usize][0 as core::ffi::c_int as usize];
            return mode_cost;
        } else {
            mode_cost
                += (*mode_costs)
                    .zeromv_mode_cost[mode_ctx as usize][1 as core::ffi::c_int as usize];
            mode_ctx = (mode_context as core::ffi::c_int >> REFMV_OFFSET
                & REFMV_CTX_MASK) as int16_t;
            mode_cost
                += (*mode_costs)
                    .refmv_mode_cost[mode_ctx
                    as usize][(mode as core::ffi::c_int != NEARESTMV as core::ffi::c_int)
                    as core::ffi::c_int as usize];
            return mode_cost;
        }
    };
}
unsafe extern "C-unwind" fn newmv_diff_bias(
    mut xd: *mut MACROBLOCKD,
    mut this_mode: PREDICTION_MODE,
    mut this_rdc: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut mv_row: core::ffi::c_int,
    mut mv_col: core::ffi::c_int,
    mut speed: core::ffi::c_int,
    mut spatial_variance: uint32_t,
    mut content_state_sb: CONTENT_STATE_SB,
) {
    if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
        let mut al_mv_average_row: core::ffi::c_int = 0;
        let mut al_mv_average_col: core::ffi::c_int = 0;
        let mut row_diff: core::ffi::c_int = 0;
        let mut col_diff: core::ffi::c_int = 0;
        let mut above_mv_valid: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut left_mv_valid: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut above_row: core::ffi::c_int = INVALID_MV_ROW_COL;
        let mut above_col: core::ffi::c_int = INVALID_MV_ROW_COL;
        let mut left_row: core::ffi::c_int = INVALID_MV_ROW_COL;
        let mut left_col: core::ffi::c_int = INVALID_MV_ROW_COL;
        if bsize as core::ffi::c_int >= BLOCK_64X64 as core::ffi::c_int
            && content_state_sb.source_sad_nonrd as core::ffi::c_uint
                != kHighSad as core::ffi::c_int as core::ffi::c_uint
            && spatial_variance < 300 as uint32_t
            && (mv_row > 16 as core::ffi::c_int || mv_row < -(16 as core::ffi::c_int)
                || mv_col > 16 as core::ffi::c_int || mv_col < -(16 as core::ffi::c_int))
        {
            (*this_rdc).rdcost = (*this_rdc).rdcost << 2 as core::ffi::c_int;
            return;
        }
        if !((*xd).above_mbmi).is_null() {
            above_mv_valid = ((*(*xd).above_mbmi)
                .mv[0 as core::ffi::c_int as usize]
                .as_int != INVALID_MV as uint32_t) as core::ffi::c_int;
            above_row = (*(*xd).above_mbmi).mv[0 as core::ffi::c_int as usize].as_mv.row
                as core::ffi::c_int;
            above_col = (*(*xd).above_mbmi).mv[0 as core::ffi::c_int as usize].as_mv.col
                as core::ffi::c_int;
        }
        if !((*xd).left_mbmi).is_null() {
            left_mv_valid = ((*(*xd).left_mbmi).mv[0 as core::ffi::c_int as usize].as_int
                != INVALID_MV as uint32_t) as core::ffi::c_int;
            left_row = (*(*xd).left_mbmi).mv[0 as core::ffi::c_int as usize].as_mv.row
                as core::ffi::c_int;
            left_col = (*(*xd).left_mbmi).mv[0 as core::ffi::c_int as usize].as_mv.col
                as core::ffi::c_int;
        }
        if above_mv_valid != 0 && left_mv_valid != 0 {
            al_mv_average_row = above_row + left_row + 1 as core::ffi::c_int
                >> 1 as core::ffi::c_int;
            al_mv_average_col = above_col + left_col + 1 as core::ffi::c_int
                >> 1 as core::ffi::c_int;
        } else if above_mv_valid != 0 {
            al_mv_average_row = above_row;
            al_mv_average_col = above_col;
        } else if left_mv_valid != 0 {
            al_mv_average_row = left_row;
            al_mv_average_col = left_col;
        } else {
            al_mv_average_col = 0 as core::ffi::c_int;
            al_mv_average_row = al_mv_average_col;
        }
        row_diff = al_mv_average_row - mv_row;
        col_diff = al_mv_average_col - mv_col;
        if row_diff > 80 as core::ffi::c_int || row_diff < -(80 as core::ffi::c_int)
            || col_diff > 80 as core::ffi::c_int || col_diff < -(80 as core::ffi::c_int)
        {
            if bsize as core::ffi::c_int >= BLOCK_32X32 as core::ffi::c_int {
                (*this_rdc).rdcost = (*this_rdc).rdcost << 1 as core::ffi::c_int;
            } else {
                (*this_rdc).rdcost = 5 as int64_t * (*this_rdc).rdcost
                    >> 2 as core::ffi::c_int;
            }
        }
    } else if speed >= 8 as core::ffi::c_int && spatial_variance < 150 as uint32_t
        && (mv_row > 64 as core::ffi::c_int || mv_row < -(64 as core::ffi::c_int)
            || mv_col > 64 as core::ffi::c_int || mv_col < -(64 as core::ffi::c_int))
    {
        (*this_rdc).rdcost = 5 as int64_t * (*this_rdc).rdcost >> 2 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn update_thresh_freq_fact(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut best_mode_idx: THR_MODES,
    mut mode: PREDICTION_MODE,
) {
    let thr_mode_idx: THR_MODES = mode_idx[ref_frame
        as usize][mode_offset(mode) as usize];
    let min_size: BLOCK_SIZE = (if bsize as core::ffi::c_int - 3 as core::ffi::c_int
        > BLOCK_4X4 as core::ffi::c_int
    {
        bsize as core::ffi::c_int - 3 as core::ffi::c_int
    } else {
        BLOCK_4X4 as core::ffi::c_int
    }) as BLOCK_SIZE;
    let max_size: BLOCK_SIZE = (if (bsize as core::ffi::c_int + 6 as core::ffi::c_int)
        < BLOCK_128X128 as core::ffi::c_int
    {
        bsize as core::ffi::c_int + 6 as core::ffi::c_int
    } else {
        BLOCK_128X128 as core::ffi::c_int
    }) as BLOCK_SIZE;
    let mut bs: BLOCK_SIZE = min_size;
    while bs as core::ffi::c_int <= max_size as core::ffi::c_int {
        let mut freq_fact: *mut core::ffi::c_int = &raw mut *(*((*x).thresh_freq_fact)
            .as_mut_ptr()
            .offset(bs as isize))
            .as_mut_ptr()
            .offset(thr_mode_idx as isize) as *mut core::ffi::c_int;
        if thr_mode_idx as core::ffi::c_int == best_mode_idx as core::ffi::c_int {
            *freq_fact -= *freq_fact >> 4 as core::ffi::c_int;
        } else {
            *freq_fact = if (*freq_fact + 1 as core::ffi::c_int)
                < (*cpi).sf.inter_sf.adaptive_rd_thresh
                    * (((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                        << 1 as core::ffi::c_int)
            {
                *freq_fact + 1 as core::ffi::c_int
            } else {
                (*cpi).sf.inter_sf.adaptive_rd_thresh
                    * (((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                        << 1 as core::ffi::c_int)
            };
        }
        bs = (bs as core::ffi::c_int + 3 as core::ffi::c_int) as BLOCK_SIZE;
    }
}
unsafe extern "C-unwind" fn search_filter_ref(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut this_rdc: *mut RD_STATS,
    mut inter_pred_params_sr: *mut InterPredParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut tmp_buffer: *mut PRED_BUFFER,
    mut bsize: BLOCK_SIZE,
    mut reuse_inter_pred: core::ffi::c_int,
    mut this_mode_pred: *mut *mut PRED_BUFFER,
    mut this_early_term: *mut core::ffi::c_int,
    mut var: *mut core::ffi::c_uint,
    mut use_model_yrd_large: core::ffi::c_int,
    mut best_sse: int64_t,
    mut is_single_pred: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let mut dim_factor: core::ffi::c_int = if (*cpi).sf.interp_sf.disable_dual_filter
        == 0 as core::ffi::c_int
    {
        FILTER_SEARCH_SIZE
    } else {
        1 as core::ffi::c_int
    };
    let mut pf_rd_stats: [RD_STATS; 4] = [
        {
            let mut init = RD_STATS {
                rate: 0 as core::ffi::c_int,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            init
        },
        RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
    ];
    let mut pf_tx_size: [TX_SIZE; 4] = [0 as core::ffi::c_int as TX_SIZE, 0, 0, 0];
    let mut current_pred: *mut PRED_BUFFER = *this_mode_pred;
    let mut best_skip: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_early_term: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_cost: int64_t = INT64_MAX as int64_t;
    let mut best_filter_index: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut subpel_params: SubpelParams = SubpelParams {
        xs: 0,
        ys: 0,
        subpel_x: 0,
        subpel_y: 0,
        pos_x: 0,
        pos_y: 0,
    };
    if is_single_pred != 0 {
        init_inter_mode_params(
            &raw mut (*((*mi).mv).as_mut_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
            inter_pred_params_sr,
            &raw mut subpel_params,
            (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
            (*((*pd).pre).as_mut_ptr()).width,
            (*((*pd).pre).as_mut_ptr()).height,
        );
    }
    let mut filter_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while filter_idx < FILTER_SEARCH_SIZE * FILTER_SEARCH_SIZE {
        let mut cost: int64_t = 0;
        if !((*cpi).sf.interp_sf.disable_dual_filter != 0
            && filters_ref_set[filter_idx as usize].as_filters.x_filter
                as core::ffi::c_int
                != filters_ref_set[filter_idx as usize].as_filters.y_filter
                    as core::ffi::c_int)
        {
            (*mi).interp_filters.as_int = filters_ref_set[filter_idx as usize].as_int;
            if is_single_pred != 0 {
                av1_enc_build_inter_predictor_y_nonrd(
                    xd,
                    inter_pred_params_sr,
                    &raw mut subpel_params,
                );
            } else {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    AOM_PLANE_Y,
                    AOM_PLANE_Y,
                );
            }
            let mut curr_var: core::ffi::c_uint = UINT_MAX;
            if use_model_yrd_large != 0 {
                model_skip_for_sb_y_large(
                    cpi,
                    bsize,
                    mi_row,
                    mi_col,
                    x,
                    xd,
                    &raw mut *pf_rd_stats.as_mut_ptr().offset(filter_idx as isize),
                    this_early_term,
                    1 as core::ffi::c_int,
                    best_sse,
                    &raw mut curr_var,
                    UINT_MAX,
                );
            } else {
                model_rd_for_sb_y(
                    cpi,
                    bsize,
                    x,
                    xd,
                    &raw mut *pf_rd_stats.as_mut_ptr().offset(filter_idx as isize),
                    &raw mut curr_var,
                    1 as core::ffi::c_int,
                    0 as *mut core::ffi::c_int,
                );
            }
            pf_rd_stats[filter_idx as usize].rate
                += av1_get_switchable_rate(
                    x,
                    xd,
                    (*cm).features.interp_filter,
                    (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int,
                );
            cost = (pf_rd_stats[filter_idx as usize].rate as int64_t
                * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + pf_rd_stats[filter_idx as usize].dist
                    * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            pf_tx_size[filter_idx as usize] = (*mi).tx_size;
            if cost < best_cost {
                *var = curr_var;
                best_filter_index = filter_idx;
                best_cost = cost;
                best_skip = pf_rd_stats[filter_idx as usize].skip_txfm
                    as core::ffi::c_int;
                best_early_term = *this_early_term;
                if reuse_inter_pred != 0 {
                    if *this_mode_pred != current_pred {
                        free_pred_buffer(*this_mode_pred);
                        *this_mode_pred = current_pred;
                    }
                    current_pred = &raw mut *tmp_buffer
                        .offset(
                            (get_pred_buffer
                                as unsafe extern "C-unwind" fn(
                                    *mut PRED_BUFFER,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int)(tmp_buffer, 3 as core::ffi::c_int)
                                as isize,
                        ) as *mut PRED_BUFFER;
                    (*pd).dst.buf = (*current_pred).data;
                    (*pd).dst.stride = bw;
                }
            }
        }
        filter_idx += 1;
    }
    if !(best_filter_index >= 0 as core::ffi::c_int
        && best_filter_index < dim_factor * 2 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"search_filter_ref\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            1272 as core::ffi::c_int,
            b"best_filter_index >= 0 && best_filter_index < dim_factor * FILTER_SEARCH_SIZE\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if reuse_inter_pred != 0 && *this_mode_pred != current_pred {
        free_pred_buffer(current_pred);
    }
    (*mi).interp_filters.as_int = filters_ref_set[best_filter_index as usize].as_int;
    (*mi).tx_size = pf_tx_size[best_filter_index as usize];
    (*this_rdc).rate = pf_rd_stats[best_filter_index as usize].rate;
    (*this_rdc).dist = pf_rd_stats[best_filter_index as usize].dist;
    (*this_rdc).sse = pf_rd_stats[best_filter_index as usize].sse;
    (*this_rdc).skip_txfm = (best_skip != 0 || best_early_term != 0) as core::ffi::c_int
        as uint8_t;
    *this_early_term = best_early_term;
    if reuse_inter_pred != 0 {
        (*pd).dst.buf = (**this_mode_pred).data;
        (*pd).dst.stride = (**this_mode_pred).stride;
    } else if best_filter_index < dim_factor * FILTER_SEARCH_SIZE - 1 as core::ffi::c_int
    {
        if is_single_pred != 0 {
            av1_enc_build_inter_predictor_y_nonrd(
                xd,
                inter_pred_params_sr,
                &raw mut subpel_params,
            );
        } else {
            av1_enc_build_inter_predictor(
                cm,
                xd,
                mi_row,
                mi_col,
                0 as *const BUFFER_SET,
                bsize,
                AOM_PLANE_Y,
                AOM_PLANE_Y,
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn is_warped_mode_allowed(
    mut cpi: *const AV1_COMP,
    x: *mut MACROBLOCK,
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    let features: *const FeatureFlags = &(*cpi).common.features;
    let mut xd: *const MACROBLOCKD = &raw mut (*x).e_mbd;
    if (*cpi).sf.inter_sf.extra_prune_warped != 0 {
        return 0 as core::ffi::c_int;
    }
    if has_second_ref(mbmi) != 0 {
        return 0 as core::ffi::c_int;
    }
    let mut last_motion_mode_allowed: MOTION_MODE = SIMPLE_TRANSLATION
        as core::ffi::c_int as MOTION_MODE;
    if (*features).switchable_motion_mode {
        last_motion_mode_allowed = motion_mode_allowed(
            (*xd).global_motion,
            xd,
            mbmi,
            (*features).allow_warped_motion as core::ffi::c_int,
        );
    }
    if last_motion_mode_allowed as core::ffi::c_int == WARPED_CAUSAL as core::ffi::c_int
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn calc_num_proj_ref(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mi: *mut MB_MODE_INFO,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let features: *const FeatureFlags = &raw mut (*cm).features;
    (*mi).num_proj_ref = 1 as uint8_t;
    let warp_sample_info: *mut WARP_SAMPLE_INFO = &raw mut *((*x).warp_sample_info)
        .as_mut_ptr()
        .offset(
            *((*mi).ref_frame).as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                as isize,
        ) as *mut WARP_SAMPLE_INFO;
    let mut pts0: *mut core::ffi::c_int = ((*warp_sample_info).pts).as_mut_ptr();
    let mut pts_inref0: *mut core::ffi::c_int = ((*warp_sample_info).pts_inref)
        .as_mut_ptr();
    let mut last_motion_mode_allowed: MOTION_MODE = SIMPLE_TRANSLATION
        as core::ffi::c_int as MOTION_MODE;
    if (*features).switchable_motion_mode {
        last_motion_mode_allowed = motion_mode_allowed(
            (*xd).global_motion,
            xd,
            mi,
            (*features).allow_warped_motion as core::ffi::c_int,
        );
    }
    if last_motion_mode_allowed as core::ffi::c_int == WARPED_CAUSAL as core::ffi::c_int
    {
        if (*warp_sample_info).num < 0 as core::ffi::c_int {
            (*warp_sample_info).num = av1_findSamples(cm, xd, pts0, pts_inref0)
                as core::ffi::c_int;
        }
        (*mi).num_proj_ref = (*warp_sample_info).num as uint8_t;
    }
}
unsafe extern "C-unwind" fn search_motion_mode(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut this_rdc: *mut RD_STATS,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut this_early_term: *mut core::ffi::c_int,
    mut use_model_yrd_large: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut best_sse: int64_t,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let features: *const FeatureFlags = &raw mut (*cm).features;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut pf_rd_stats: [RD_STATS; 2] = [
        {
            let mut init = RD_STATS {
                rate: 0 as core::ffi::c_int,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            init
        },
        RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
    ];
    let mut best_skip: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_early_term: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut best_cost: int64_t = INT64_MAX as int64_t;
    let mut best_mode_index: core::ffi::c_int = -(1 as core::ffi::c_int);
    let interp_filter: core::ffi::c_int = (*features).interp_filter as core::ffi::c_int;
    let motion_modes: [MOTION_MODE; 2] = [
        SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE,
        WARPED_CAUSAL as core::ffi::c_int as MOTION_MODE,
    ];
    let mut mode_search_size: core::ffi::c_int = if is_warped_mode_allowed(cpi, x, mi)
        != 0
    {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let warp_sample_info: *mut WARP_SAMPLE_INFO = &raw mut *((*x).warp_sample_info)
        .as_mut_ptr()
        .offset(
            *((*mi).ref_frame).as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                as isize,
        ) as *mut WARP_SAMPLE_INFO;
    let mut pts0: *mut core::ffi::c_int = ((*warp_sample_info).pts).as_mut_ptr();
    let mut pts_inref0: *mut core::ffi::c_int = ((*warp_sample_info).pts_inref)
        .as_mut_ptr();
    let total_samples: core::ffi::c_int = (*mi).num_proj_ref as core::ffi::c_int;
    if total_samples == 0 as core::ffi::c_int {
        mode_search_size = 1 as core::ffi::c_int;
    }
    let base_mbmi: MB_MODE_INFO = *mi;
    let mut best_mbmi: MB_MODE_INFO = MB_MODE_INFO {
        bsize: BLOCK_4X4,
        partition: 0,
        mode: 0,
        uv_mode: 0,
        current_qindex: 0,
        mv: [int_mv { as_int: 0 }; 2],
        ref_frame: [0; 2],
        interp_filters: int_interpfilters { as_int: 0 },
        motion_mode: 0,
        num_proj_ref: 0,
        overlappable_neighbors: 0,
        wm_params: WarpedMotionParams {
            wmmat: [0; 6],
            alpha: 0,
            beta: 0,
            gamma: 0,
            delta: 0,
            wmtype: 0,
            invalid: 0,
        },
        interintra_mode: 0,
        interintra_wedge_index: 0,
        interinter_comp: INTERINTER_COMPOUND_DATA {
            seg_mask: 0 as *mut uint8_t,
            wedge_index: 0,
            wedge_sign: 0,
            mask_type: 0,
            type_0: 0,
        },
        angle_delta: [0; 2],
        filter_intra_mode_info: FILTER_INTRA_MODE_INFO {
            filter_intra_mode: 0,
            use_filter_intra: 0,
        },
        cfl_alpha_signs: 0,
        cfl_alpha_idx: 0,
        palette_mode_info: PALETTE_MODE_INFO {
            palette_colors: [0; 24],
            palette_size: [0; 2],
        },
        skip_txfm: 0,
        tx_size: 0,
        inter_tx_size: [0; 16],
        delta_lf_from_base: 0,
        delta_lf: [0; 4],
        segment_id_seg_id_predicted_ref_mv_idx_skip_mode_use_intrabc_comp_group_idx_compound_idx_use_wedge_interintra_cdef_strength: [0; 2],
        c2rust_padding: [0; 7],
    };
    let mut current_block_43: u64;
    let mut mode_index: core::ffi::c_int = 0 as core::ffi::c_int;
    while mode_index < mode_search_size {
        let mut cost: int64_t = INT64_MAX as int64_t;
        let mut motion_mode: MOTION_MODE = motion_modes[mode_index as usize];
        *mi = base_mbmi;
        (*mi).motion_mode = motion_mode;
        if motion_mode as core::ffi::c_int == SIMPLE_TRANSLATION as core::ffi::c_int {
            (*mi).interp_filters = av1_broadcast_interp_filter(EIGHTTAP_REGULAR);
            av1_enc_build_inter_predictor(
                cm,
                xd,
                mi_row,
                mi_col,
                0 as *const BUFFER_SET,
                bsize,
                AOM_PLANE_Y,
                AOM_PLANE_Y,
            );
            if use_model_yrd_large != 0 {
                model_skip_for_sb_y_large(
                    cpi,
                    bsize,
                    mi_row,
                    mi_col,
                    x,
                    xd,
                    &raw mut *pf_rd_stats.as_mut_ptr().offset(mode_index as isize),
                    this_early_term,
                    1 as core::ffi::c_int,
                    best_sse,
                    0 as *mut core::ffi::c_uint,
                    UINT_MAX,
                );
            } else {
                model_rd_for_sb_y(
                    cpi,
                    bsize,
                    x,
                    xd,
                    &raw mut *pf_rd_stats.as_mut_ptr().offset(mode_index as isize),
                    0 as *mut core::ffi::c_uint,
                    1 as core::ffi::c_int,
                    0 as *mut core::ffi::c_int,
                );
            }
            pf_rd_stats[mode_index as usize].rate
                += av1_get_switchable_rate(
                    x,
                    xd,
                    (*cm).features.interp_filter,
                    (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int,
                );
            cost = (pf_rd_stats[mode_index as usize].rate as int64_t
                * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + pf_rd_stats[mode_index as usize].dist
                    * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            current_block_43 = 0;
        } else if motion_mode as core::ffi::c_int == WARPED_CAUSAL as core::ffi::c_int {
            let mut pts: [core::ffi::c_int; 16] = [0; 16];
            let mut pts_inref: [core::ffi::c_int; 16] = [0; 16];
            let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
            (*mi).wm_params.wmtype = AFFINE as core::ffi::c_int as TransformationType;
            (*mi).interp_filters = av1_broadcast_interp_filter(
                av1_unswitchable_filter(interp_filter as InterpFilter),
            );
            memcpy(
                pts.as_mut_ptr() as *mut core::ffi::c_void,
                pts0 as *const core::ffi::c_void,
                ((total_samples * 2 as core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
            );
            memcpy(
                pts_inref.as_mut_ptr() as *mut core::ffi::c_void,
                pts_inref0 as *const core::ffi::c_void,
                ((total_samples * 2 as core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
            );
            if (*mi).num_proj_ref as core::ffi::c_int > 1 as core::ffi::c_int {
                (*mi).num_proj_ref = av1_selectSamples(
                    &raw mut (*((*mi).mv)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mv,
                    pts.as_mut_ptr(),
                    pts_inref.as_mut_ptr(),
                    (*mi).num_proj_ref as core::ffi::c_int,
                    bsize,
                );
            }
            if av1_find_projection(
                (*mi).num_proj_ref as core::ffi::c_int,
                pts.as_mut_ptr(),
                pts_inref.as_mut_ptr(),
                bsize,
                (*mi).mv[0 as core::ffi::c_int as usize].as_mv.row as core::ffi::c_int,
                (*mi).mv[0 as core::ffi::c_int as usize].as_mv.col as core::ffi::c_int,
                &raw mut (*mi).wm_params,
                mi_row,
                mi_col,
            ) == 0
            {
                if (*mi).mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
                    let mv0: int_mv = (*mi).mv[0 as core::ffi::c_int as usize];
                    let wm_params0: WarpedMotionParams = (*mi).wm_params;
                    let num_proj_ref0: core::ffi::c_int = (*mi).num_proj_ref
                        as core::ffi::c_int;
                    let ref_mv: int_mv = av1_get_ref_mv(x, 0 as core::ffi::c_int)
                        as int_mv;
                    let mut ms_params: SUBPEL_MOTION_SEARCH_PARAMS = SUBPEL_MOTION_SEARCH_PARAMS {
                        allow_hp: 0,
                        cost_list: 0 as *const core::ffi::c_int,
                        forced_stop: 0,
                        iters_per_step: 0,
                        mv_limits: SubpelMvLimits {
                            col_min: 0,
                            col_max: 0,
                            row_min: 0,
                            row_max: 0,
                        },
                        mv_cost_params: MV_COST_PARAMS {
                            ref_mv: 0 as *const MV,
                            full_ref_mv: fullpel_mv { row: 0, col: 0 },
                            mv_cost_type: 0,
                            mvjcost: 0 as *const core::ffi::c_int,
                            mvcost: [0 as *const core::ffi::c_int; 2],
                            error_per_bit: 0,
                            sad_per_bit: 0,
                        },
                        var_params: SUBPEL_SEARCH_VAR_PARAMS {
                            vfp: 0 as *const aom_variance_fn_ptr_t,
                            subpel_search_type: 0,
                            ms_buffers: MSBuffers {
                                ref_0: 0 as *const buf_2d,
                                src: 0 as *const buf_2d,
                                second_pred: 0 as *const uint8_t,
                                mask: 0 as *const uint8_t,
                                mask_stride: 0,
                                inv_mask: 0,
                                wsrc: 0 as *const int32_t,
                                obmc_mask: 0 as *const int32_t,
                            },
                            w: 0,
                            h: 0,
                        },
                    };
                    av1_make_default_subpel_ms_params(
                        &raw mut ms_params,
                        cpi,
                        x,
                        bsize,
                        &ref_mv.as_mv,
                        0 as *const core::ffi::c_int,
                    );
                    av1_refine_warped_mv(
                        xd,
                        cm,
                        &raw mut ms_params,
                        bsize,
                        pts0,
                        pts_inref0,
                        total_samples,
                        (*cpi).sf.mv_sf.warp_search_method,
                        (*cpi).sf.mv_sf.warp_search_iters,
                    );
                    if (*mi).mv[0 as core::ffi::c_int as usize].as_int == ref_mv.as_int {
                        current_block_43 = 1;
                    } else {
                        if mv0.as_int != (*mi).mv[0 as core::ffi::c_int as usize].as_int
                        {
                            let mut tmp_rate_mv: core::ffi::c_int = av1_mv_bit_cost(
                                &raw mut (*((*mi).mv)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize))
                                    .as_mv,
                                &ref_mv.as_mv,
                                ((*(*x).mv_costs).nmv_joint_cost).as_mut_ptr(),
                                (*(*x).mv_costs).mv_cost_stack
                                    as *const *mut core::ffi::c_int,
                                MV_COST_WEIGHT,
                            );
                            *rate_mv = tmp_rate_mv;
                        } else {
                            (*mi).mv[0 as core::ffi::c_int as usize] = mv0;
                            (*mi).wm_params = wm_params0;
                            (*mi).num_proj_ref = num_proj_ref0 as uint8_t;
                        }
                        current_block_43 = 2;
                    }
                } else {
                    current_block_43 = 2;
                }
                match current_block_43 {
                    1 => {}
                    _ => {
                        av1_enc_build_inter_predictor(
                            cm,
                            xd,
                            mi_row,
                            mi_col,
                            0 as *const BUFFER_SET,
                            bsize,
                            AOM_PLANE_Y,
                            av1_num_planes(cm) - 1 as core::ffi::c_int,
                        );
                        if use_model_yrd_large != 0 {
                            model_skip_for_sb_y_large(
                                cpi,
                                bsize,
                                mi_row,
                                mi_col,
                                x,
                                xd,
                                &raw mut *pf_rd_stats.as_mut_ptr().offset(mode_index as isize),
                                this_early_term,
                                1 as core::ffi::c_int,
                                best_sse,
                                0 as *mut core::ffi::c_uint,
                                UINT_MAX,
                            );
                        } else {
                            model_rd_for_sb_y(
                                cpi,
                                bsize,
                                x,
                                xd,
                                &raw mut *pf_rd_stats.as_mut_ptr().offset(mode_index as isize),
                                0 as *mut core::ffi::c_uint,
                                1 as core::ffi::c_int,
                                0 as *mut core::ffi::c_int,
                            );
                        }
                        pf_rd_stats[mode_index as usize].rate
                            += (*mode_costs)
                                .motion_mode_cost[bsize
                                as usize][(*mi).motion_mode as usize];
                        cost = (pf_rd_stats[mode_index as usize].rate as int64_t
                            * (*x).rdmult as int64_t
                            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                                >> 1 as core::ffi::c_int) as int64_t
                            >> 9 as core::ffi::c_int)
                            + pf_rd_stats[mode_index as usize].dist
                                * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                        current_block_43 = 0;
                    }
                }
            } else {
                cost = INT64_MAX as int64_t;
                current_block_43 = 0;
            }
        } else {
            current_block_43 = 0;
        }
        match current_block_43 {
            0 => {
                if cost < best_cost {
                    best_mode_index = mode_index;
                    best_cost = cost;
                    best_skip = pf_rd_stats[mode_index as usize].skip_txfm
                        as core::ffi::c_int;
                    best_early_term = *this_early_term;
                    best_mbmi = *mi;
                }
            }
            _ => {}
        }
        mode_index += 1;
    }
    if !(best_mode_index >= 0 as core::ffi::c_int
        && best_mode_index < 2 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"search_motion_mode\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            1483 as core::ffi::c_int,
            b"best_mode_index >= 0 && best_mode_index < FILTER_SEARCH_SIZE\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *mi = best_mbmi;
    (*this_rdc).rate = pf_rd_stats[best_mode_index as usize].rate;
    (*this_rdc).dist = pf_rd_stats[best_mode_index as usize].dist;
    (*this_rdc).sse = pf_rd_stats[best_mode_index as usize].sse;
    (*this_rdc).skip_txfm = (best_skip != 0 || best_early_term != 0) as core::ffi::c_int
        as uint8_t;
    *this_early_term = best_early_term;
    if best_mode_index < FILTER_SEARCH_SIZE - 1 as core::ffi::c_int {
        av1_enc_build_inter_predictor(
            cm,
            xd,
            mi_row,
            mi_col,
            0 as *const BUFFER_SET,
            bsize,
            AOM_PLANE_Y,
            AOM_PLANE_Y,
        );
    }
}
unsafe extern "C-unwind" fn should_prune_intra_modes_using_neighbors(
    mut xd: *const MACROBLOCKD,
    mut enable_intra_mode_pruning_using_neighbors: bool,
    mut this_mode: PREDICTION_MODE,
    mut above_mode: PREDICTION_MODE,
    mut left_mode: PREDICTION_MODE,
) -> bool {
    if !enable_intra_mode_pruning_using_neighbors {
        return false_0 != 0;
    }
    if this_mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        return false_0 != 0;
    }
    return (*xd).up_available as core::ffi::c_int != 0
        && this_mode as core::ffi::c_int != above_mode as core::ffi::c_int
        && (*xd).left_available as core::ffi::c_int != 0
        && this_mode as core::ffi::c_int != left_mode as core::ffi::c_int;
}
unsafe extern "C-unwind" fn av1_search_intrabc_nonrd(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut this_rdc: *mut RD_STATS,
    mut best_dv: *mut int_mv,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    (*this_rdc).rdcost = INT64_MAX as int64_t;
    let mut ori_pre: [buf_2d; 3] = [buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        ori_pre[i as usize] = (*xd)
            .plane[i as usize]
            .pre[0 as core::ffi::c_int as usize];
        i += 1;
    }
    let mut yv12_mb: [buf_2d; 3] = [buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3];
    av1_setup_pred_block(
        xd,
        yv12_mb.as_mut_ptr(),
        (*xd).cur_buf,
        0 as *const scale_factors,
        0 as *const scale_factors,
        num_planes,
    );
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < num_planes {
        (*xd).plane[i_0 as usize].pre[0 as core::ffi::c_int as usize] = yv12_mb[i_0
            as usize];
        i_0 += 1;
    }
    let mbmi_ext: *mut MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let ref_frame: MV_REFERENCE_FRAME = INTRA_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    av1_find_mv_refs(
        cm,
        xd,
        mi,
        ref_frame,
        ((*mbmi_ext).ref_mv_count).as_mut_ptr(),
        ((*xd).ref_mv_stack).as_mut_ptr(),
        ((*xd).weight).as_mut_ptr(),
        0 as *mut [int_mv; 2],
        ((*mbmi_ext).global_mvs).as_mut_ptr(),
        ((*mbmi_ext).mode_context).as_mut_ptr(),
    );
    av1_copy_usable_ref_mv_stack_and_weight(xd, mbmi_ext, ref_frame);
    let mut nearestmv: int_mv = int_mv { as_int: 0 };
    let mut nearmv: int_mv = int_mv { as_int: 0 };
    av1_find_best_ref_mvs_from_stack(
        0 as core::ffi::c_int,
        mbmi_ext,
        ref_frame,
        &raw mut nearestmv,
        &raw mut nearmv,
        0 as core::ffi::c_int,
    );
    if nearestmv.as_int == INVALID_MV as uint32_t {
        nearestmv.as_int = 0 as uint32_t;
    }
    if nearmv.as_int == INVALID_MV as uint32_t {
        nearmv.as_int = 0 as uint32_t;
    }
    let mut dv_ref: int_mv = if nearestmv.as_int == 0 as uint32_t {
        nearmv
    } else {
        nearestmv
    };
    if dv_ref.as_int == 0 as uint32_t {
        av1_find_ref_dv(
            &raw mut dv_ref,
            &raw mut (*xd).tile,
            (*(*cm).seq_params).mib_size,
            mi_row,
        );
    }
    if !(dv_ref.as_mv.col as core::ffi::c_int & 7 as core::ffi::c_int
        == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_search_intrabc_nonrd\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            1629 as core::ffi::c_int,
            b"(dv_ref.as_mv.col & 7) == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dv_ref.as_mv.row as core::ffi::c_int & 7 as core::ffi::c_int
        == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_search_intrabc_nonrd\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            1630 as core::ffi::c_int,
            b"(dv_ref.as_mv.row & 7) == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*mbmi_ext)
        .ref_mv_stack[INTRA_FRAME as core::ffi::c_int
            as usize][0 as core::ffi::c_int as usize]
        .this_mv = dv_ref;
    let mut fullms_params: FULLPEL_MOTION_SEARCH_PARAMS = FULLPEL_MOTION_SEARCH_PARAMS {
        bsize: BLOCK_4X4,
        vfp: 0 as *const aom_variance_fn_ptr_t,
        ms_buffers: MSBuffers {
            ref_0: 0 as *const buf_2d,
            src: 0 as *const buf_2d,
            second_pred: 0 as *const uint8_t,
            mask: 0 as *const uint8_t,
            mask_stride: 0,
            inv_mask: 0,
            wsrc: 0 as *const int32_t,
            obmc_mask: 0 as *const int32_t,
        },
        search_method: 0,
        search_sites: 0 as *const search_site_config,
        mv_limits: FullMvLimits {
            col_min: 0,
            col_max: 0,
            row_min: 0,
            row_max: 0,
        },
        run_mesh_search: 0,
        prune_mesh_search: 0,
        mesh_search_mv_diff_threshold: 0,
        force_mesh_thresh: 0,
        mesh_patterns: [0 as *const MESH_PATTERN; 2],
        fine_search_interval: 0,
        is_intra_mode: 0,
        fast_obmc_search: 0,
        mv_cost_params: MV_COST_PARAMS {
            ref_mv: 0 as *const MV,
            full_ref_mv: fullpel_mv { row: 0, col: 0 },
            mv_cost_type: 0,
            mvjcost: 0 as *const core::ffi::c_int,
            mvcost: [0 as *const core::ffi::c_int; 2],
            error_per_bit: 0,
            sad_per_bit: 0,
        },
        sdf: None,
        sdx4df: None,
        sdx3df: None,
    };
    let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(
        x,
        &raw mut (*cpi).sf.mv_sf,
        bsize,
    ) as SEARCH_METHODS;
    let mut src_search_sites: *const search_site_config = av1_get_search_site_config(
        cpi,
        x,
        search_method,
    );
    let start_mv: FULLPEL_MV = get_fullmv_from_mv(&raw mut dv_ref.as_mv) as FULLPEL_MV;
    av1_make_default_fullpel_ms_params(
        &raw mut fullms_params,
        cpi,
        x,
        bsize,
        &raw mut dv_ref.as_mv,
        start_mv,
        src_search_sites as *const search_site_config,
        search_method,
        0 as core::ffi::c_int,
    );
    av1_set_ms_to_intra_mode(&raw mut fullms_params, (*x).dv_costs);
    fullms_params.mv_limits.col_min = ((*xd).tile.mi_col_start - mi_col) * MI_SIZE;
    fullms_params.mv_limits.col_max = ((*xd).tile.mi_col_end - mi_col) * MI_SIZE
        - block_size_wide[bsize as usize] as core::ffi::c_int;
    fullms_params.mv_limits.row_min = ((*xd).tile.mi_row_start - mi_row) * MI_SIZE;
    fullms_params.mv_limits.row_max = ((*xd).tile.mi_row_end - mi_row) * MI_SIZE
        - block_size_high[bsize as usize] as core::ffi::c_int;
    let mut best_mv: int_mv = int_mv { as_int: 0 };
    let mut bestsme: core::ffi::c_int = INT_MAX;
    if (*cpi).sf.mv_sf.hash_max_8x8_intrabc_blocks == 0
        || bsize as core::ffi::c_int <= BLOCK_8X8 as core::ffi::c_int
    {
        bestsme = av1_intrabc_hash_search(
            cpi,
            xd,
            &raw mut fullms_params,
            &raw mut (*x).intrabc_hash_info,
            &raw mut best_mv.as_fullmv,
        );
    }
    if bestsme == INT_MAX {
        let miss_mode: core::ffi::c_int = (*cpi).sf.rt_sf.rt_intrabc_miss_mode;
        if miss_mode == 0 as core::ffi::c_int {
            let mut best_mv_stats: FULLPEL_MV_STATS = FULLPEL_MV_STATS {
                err_cost: 0,
                distortion: 0,
                sse: 0,
            };
            bestsme = av1_full_pixel_search(
                start_mv,
                &raw mut fullms_params,
                (*cpi).mv_search_params.mv_step_param,
                0 as *mut core::ffi::c_int,
                &raw mut best_mv.as_fullmv,
                &raw mut best_mv_stats,
                0 as *mut FULLPEL_MV,
            );
        } else if miss_mode == 1 as core::ffi::c_int
            || miss_mode == 2 as core::ffi::c_int
        {
            let mut cand_mv: FULLPEL_MV = start_mv;
            if cand_mv.col as core::ffi::c_int >= fullms_params.mv_limits.col_min
                && cand_mv.col as core::ffi::c_int <= fullms_params.mv_limits.col_max
                && cand_mv.row as core::ffi::c_int >= fullms_params.mv_limits.row_min
                && cand_mv.row as core::ffi::c_int <= fullms_params.mv_limits.row_max
            {
                let mut dv: MV = get_mv_from_fullmv(&raw mut cand_mv);
                if av1_is_dv_valid(
                    dv,
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    bsize,
                    (*(*cm).seq_params).mib_size_log2,
                ) != 0
                {
                    let mut src: *const buf_2d = fullms_params.ms_buffers.src;
                    let mut ref_0: *const buf_2d = fullms_params.ms_buffers.ref_0;
                    let mut sad: core::ffi::c_uint = ((*fullms_params.vfp).sdf)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*src).buf,
                        (*src).stride,
                        get_buf_from_fullmv(ref_0, &raw mut cand_mv),
                        (*ref_0).stride,
                    );
                    let mut rate: core::ffi::c_uint = av1_mv_bit_cost(
                        &raw mut dv,
                        &raw mut dv_ref.as_mv,
                        ((*(*x).dv_costs).joint_mv).as_mut_ptr(),
                        ((*(*x).dv_costs).dv_costs).as_mut_ptr()
                            as *const *mut core::ffi::c_int,
                        MV_COST_WEIGHT,
                    ) as core::ffi::c_uint;
                    best_mv.as_fullmv = cand_mv;
                    bestsme = sad.wrapping_add(rate) as core::ffi::c_int;
                }
            }
            if miss_mode == 2 as core::ffi::c_int {
                let row_offsets: [core::ffi::c_int; 12] = [
                    -(1 as core::ffi::c_int),
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    -(2 as core::ffi::c_int),
                    2 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    -(1 as core::ffi::c_int),
                    1 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                ];
                let col_offsets: [core::ffi::c_int; 12] = [
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    -(2 as core::ffi::c_int),
                    2 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    1 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    1 as core::ffi::c_int,
                ];
                let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                while k < 12 as core::ffi::c_int {
                    let mut probe_mv: FULLPEL_MV = fullpel_mv { row: 0, col: 0 };
                    probe_mv.row = (start_mv.row as core::ffi::c_int
                        + row_offsets[k as usize]) as int16_t;
                    probe_mv.col = (start_mv.col as core::ffi::c_int
                        + col_offsets[k as usize]) as int16_t;
                    if probe_mv.col as core::ffi::c_int
                        >= fullms_params.mv_limits.col_min
                        && probe_mv.col as core::ffi::c_int
                            <= fullms_params.mv_limits.col_max
                        && probe_mv.row as core::ffi::c_int
                            >= fullms_params.mv_limits.row_min
                        && probe_mv.row as core::ffi::c_int
                            <= fullms_params.mv_limits.row_max
                    {
                        let mut dv_0: MV = get_mv_from_fullmv(&raw mut probe_mv);
                        if av1_is_dv_valid(
                            dv_0,
                            cm,
                            xd,
                            mi_row,
                            mi_col,
                            bsize,
                            (*(*cm).seq_params).mib_size_log2,
                        ) != 0
                        {
                            let mut src_0: *const buf_2d = fullms_params.ms_buffers.src;
                            let mut ref_1: *const buf_2d = fullms_params
                                .ms_buffers
                                .ref_0;
                            let mut sad_0: core::ffi::c_uint = ((*fullms_params.vfp).sdf)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*src_0).buf,
                                (*src_0).stride,
                                get_buf_from_fullmv(ref_1, &raw mut probe_mv),
                                (*ref_1).stride,
                            );
                            let mut rate_0: core::ffi::c_uint = av1_mv_bit_cost(
                                &raw mut dv_0,
                                &raw mut dv_ref.as_mv,
                                ((*(*x).dv_costs).joint_mv).as_mut_ptr(),
                                ((*(*x).dv_costs).dv_costs).as_mut_ptr()
                                    as *const *mut core::ffi::c_int,
                                MV_COST_WEIGHT,
                            ) as core::ffi::c_uint;
                            let mut cost: core::ffi::c_int = sad_0.wrapping_add(rate_0)
                                as core::ffi::c_int;
                            if cost < bestsme {
                                bestsme = cost;
                                best_mv.as_fullmv = probe_mv;
                            }
                        }
                    }
                    k += 1;
                }
            }
        } else {
            bestsme = INT_MAX;
        }
    }
    if bestsme != INT_MAX {
        let mut dv_1: MV = get_mv_from_fullmv(&raw mut best_mv.as_fullmv);
        if av1_is_dv_valid(
            dv_1,
            cm,
            xd,
            mi_row,
            mi_col,
            bsize,
            (*(*cm).seq_params).mib_size_log2,
        ) != 0
        {
            let ori_mi_use_intrabc: int8_t = (*mi).use_intrabc() as int8_t;
            let ori_mi_mv0: int_mv = (*mi).mv[0 as core::ffi::c_int as usize];
            let ori_mi_mode: PREDICTION_MODE = (*mi).mode;
            let ori_ref0: MV_REFERENCE_FRAME = (*mi)
                .ref_frame[0 as core::ffi::c_int as usize];
            (*mi).set_use_intrabc(1 as uint8_t as uint8_t);
            (*mi).mv[0 as core::ffi::c_int as usize].as_mv = dv_1;
            (*mi).mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
            (*mi).ref_frame[0 as core::ffi::c_int as usize] = INTRA_FRAME
                as core::ffi::c_int as MV_REFERENCE_FRAME;
            av1_enc_build_inter_predictor(
                cm,
                xd,
                mi_row,
                mi_col,
                0 as *const BUFFER_SET,
                bsize,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            if num_planes > 1 as core::ffi::c_int {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    1 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                );
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    2 as core::ffi::c_int,
                    2 as core::ffi::c_int,
                );
            }
            (*this_rdc).rate = av1_mv_bit_cost(
                &raw mut dv_1,
                &raw mut dv_ref.as_mv,
                ((*(*x).dv_costs).joint_mv).as_mut_ptr(),
                ((*(*x).dv_costs).dv_costs).as_mut_ptr() as *const *mut core::ffi::c_int,
                MV_COST_WEIGHT,
            );
            (*this_rdc).rate
                += (*x).mode_costs.intrabc_cost[1 as core::ffi::c_int as usize];
            (*this_rdc).dist = 0 as int64_t;
            let mut skippable: core::ffi::c_int = 0;
            av1_block_yrd(
                x,
                this_rdc,
                &raw mut skippable,
                bsize,
                (if ((*mi).tx_size as core::ffi::c_int) < TX_16X16 as core::ffi::c_int {
                    (*mi).tx_size as core::ffi::c_int
                } else {
                    TX_16X16 as core::ffi::c_int
                }) as TX_SIZE,
            );
            if num_planes > 1 as core::ffi::c_int {
                let mut rdc_uv: RD_STATS = RD_STATS {
                    rate: 0,
                    zero_rate: 0,
                    dist: 0,
                    rdcost: 0,
                    sse: 0,
                    skip_txfm: 0,
                };
                av1_invalid_rd_stats(&raw mut rdc_uv);
                av1_model_rd_for_sb_uv(
                    cpi,
                    bsize,
                    x,
                    xd,
                    &raw mut rdc_uv,
                    1 as core::ffi::c_int,
                    2 as core::ffi::c_int,
                );
                (*this_rdc).rate += rdc_uv.rate;
                (*this_rdc).dist += rdc_uv.dist;
            }
            (*this_rdc).rdcost = ((*this_rdc).rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*this_rdc).dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
            *best_dv = (*mi).mv[0 as core::ffi::c_int as usize];
            (*mi).set_use_intrabc(ori_mi_use_intrabc as uint8_t as uint8_t);
            (*mi).mv[0 as core::ffi::c_int as usize] = ori_mi_mv0;
            (*mi).mode = ori_mi_mode;
            (*mi).ref_frame[0 as core::ffi::c_int as usize] = ori_ref0;
        }
    }
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < num_planes {
        (*xd).plane[i_1 as usize].pre[0 as core::ffi::c_int as usize] = ori_pre[i_1
            as usize];
        i_1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nonrd_pick_intra_mode(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut rd_cost: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ctx: *mut PICK_MODE_CONTEXT,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut this_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut best_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    let mut args: estimate_block_intra_args = estimate_block_intra_args {
        cpi: 0 as *mut AV1_COMP,
        x: 0 as *mut MACROBLOCK,
        mode: 0,
        skippable: 0,
        rdc: 0 as *mut RD_STATS,
        best_sad: 0,
        prune_mode_based_on_sad: false,
        prune_palette_sad: false,
    };
    init_estimate_block_intra_args(&raw mut args, cpi, x);
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    (*mi).tx_size = (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
        < tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
            as core::ffi::c_int
    {
        max_txsize_lookup[bsize as usize] as core::ffi::c_int
    } else {
        tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
            as core::ffi::c_int
    }) as TX_SIZE;
    if (*cm).quant_params.base_qindex > 150 as core::ffi::c_int
        && (*x).source_variance == 0 as core::ffi::c_uint
        && (mi_row == 0 as core::ffi::c_int || mi_col == 0 as core::ffi::c_int)
        && (*mi).tx_size as core::ffi::c_int > TX_16X16 as core::ffi::c_int
    {
        (*mi).tx_size = TX_16X16 as core::ffi::c_int as TX_SIZE;
    }
    if !((*xd).lossless[(*mi).segment_id() as usize] == 0
        || (*mi).tx_size as core::ffi::c_int == TX_4X4 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_nonrd_pick_intra_mode\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            1796 as core::ffi::c_int,
            b"IMPLIES(xd->lossless[mi->segment_id], mi->tx_size == TX_4X4)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let tx_bsize: BLOCK_SIZE = txsize_to_bsize[(*mi).tx_size as usize];
    if (*cpi).sf.rt_sf.prune_intra_mode_using_best_sad_so_far as core::ffi::c_int != 0
        && bsize as core::ffi::c_int == tx_bsize as core::ffi::c_int
    {
        args.prune_mode_based_on_sad = true_0 != 0;
    }
    let mut bmode_costs: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut best_mode: PREDICTION_MODE = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
    let mut above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let mut left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let A: PREDICTION_MODE = av1_above_block_mode(above_mi) as PREDICTION_MODE;
    let L: PREDICTION_MODE = av1_left_block_mode(left_mi) as PREDICTION_MODE;
    let above_ctx: core::ffi::c_int = intra_mode_context[A as usize];
    let left_ctx: core::ffi::c_int = intra_mode_context[L as usize];
    let source_variance: core::ffi::c_uint = (*x).source_variance;
    bmode_costs = ((*x).mode_costs.y_mode_costs[above_ctx as usize][left_ctx as usize])
        .as_mut_ptr();
    let flat_blocks_screen: bool = (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).oxcf.mode as core::ffi::c_int == REALTIME as core::ffi::c_int
        && (*x).source_variance == 0 as core::ffi::c_uint
        && bsize as core::ffi::c_int >= BLOCK_32X32 as core::ffi::c_int;
    av1_invalid_rd_stats(&raw mut best_rdc);
    av1_invalid_rd_stats(&raw mut this_rdc);
    init_mbmi_nonrd(
        mi,
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        INTRA_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        cm,
    );
    (*mi).mv[1 as core::ffi::c_int as usize].as_int = INVALID_MV as uint32_t;
    (*mi).mv[0 as core::ffi::c_int as usize].as_int = (*mi)
        .mv[1 as core::ffi::c_int as usize]
        .as_int;
    let mut allow_skip_nondc: bool = true_0 != 0;
    let mut palette_selected: bool = false_0 != 0;
    let mut current_block_33: u64;
    let mut mode_index: core::ffi::c_int = 0 as core::ffi::c_int;
    while mode_index < RTC_INTRA_MODES {
        let mut this_mode: PREDICTION_MODE = intra_mode_list[mode_index as usize];
        if !((*x).source_variance == 0 as core::ffi::c_uint
            && mi_col == 0 as core::ffi::c_int && mi_row == 0 as core::ffi::c_int
            && bsize as core::ffi::c_int >= BLOCK_32X32 as core::ffi::c_int
            && this_mode as core::ffi::c_int > 0 as core::ffi::c_int)
        {
            if !((*cpi).sf.rt_sf.prune_h_pred_using_best_mode_so_far as core::ffi::c_int
                != 0 && this_mode as core::ffi::c_int == H_PRED as core::ffi::c_int
                && best_mode as core::ffi::c_int == V_PRED as core::ffi::c_int
                && allow_skip_nondc as core::ffi::c_int != 0)
            {
                if should_prune_intra_modes_using_neighbors(
                    xd,
                    (*cpi).sf.rt_sf.enable_intra_mode_pruning_using_neighbors,
                    this_mode,
                    A,
                    L,
                ) {
                    if (this_mode as core::ffi::c_int == V_PRED as core::ffi::c_int
                        || this_mode as core::ffi::c_int == H_PRED as core::ffi::c_int)
                        && source_variance <= 50 as core::ffi::c_uint
                        && allow_skip_nondc as core::ffi::c_int != 0
                    {
                        current_block_33 = 3;
                    } else if best_mode as core::ffi::c_int
                        == DC_PRED as core::ffi::c_int
                        && this_mode as core::ffi::c_int
                            == SMOOTH_PRED as core::ffi::c_int
                        && allow_skip_nondc as core::ffi::c_int != 0
                    {
                        current_block_33 = 3;
                    } else {
                        current_block_33 = 4;
                    }
                } else {
                    current_block_33 = 4;
                }
                match current_block_33 {
                    3 => {}
                    _ => {
                        this_rdc.rate = 0 as core::ffi::c_int;
                        this_rdc.dist = this_rdc.rate as int64_t;
                        args.mode = this_mode;
                        args.skippable = 1 as core::ffi::c_int;
                        args.rdc = &raw mut this_rdc;
                        (*mi).mode = this_mode;
                        av1_foreach_transformed_block_in_plane(
                            xd,
                            bsize,
                            AOM_PLANE_Y,
                            Some(
                                av1_estimate_block_intra
                                    as unsafe extern "C-unwind" fn(
                                        core::ffi::c_int,
                                        core::ffi::c_int,
                                        core::ffi::c_int,
                                        core::ffi::c_int,
                                        BLOCK_SIZE,
                                        TX_SIZE,
                                        *mut core::ffi::c_void,
                                    ) -> (),
                            ),
                            &raw mut args as *mut estimate_block_intra_args
                                as *mut core::ffi::c_void,
                        );
                        if !(this_rdc.rate == INT_MAX) {
                            let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(
                                xd,
                            ) as core::ffi::c_int;
                            if args.skippable != 0 {
                                this_rdc.rate = (*x)
                                    .mode_costs
                                    .skip_txfm_cost[skip_ctx
                                    as usize][1 as core::ffi::c_int as usize];
                            } else {
                                this_rdc.rate
                                    += (*x)
                                        .mode_costs
                                        .skip_txfm_cost[skip_ctx
                                        as usize][0 as core::ffi::c_int as usize];
                            }
                            this_rdc.rate += *bmode_costs.offset(this_mode as isize);
                            this_rdc.rdcost = (this_rdc.rate as int64_t
                                * (*x).rdmult as int64_t
                                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                                    >> 1 as core::ffi::c_int) as int64_t
                                >> 9 as core::ffi::c_int)
                                + this_rdc.dist
                                    * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                            if this_rdc.rdcost < best_rdc.rdcost {
                                best_rdc = this_rdc;
                                best_mode = this_mode;
                            }
                            if this_mode as core::ffi::c_int
                                == DC_PRED as core::ffi::c_int
                            {
                                if flat_blocks_screen as core::ffi::c_int != 0
                                    && args.skippable != 0 && this_rdc.dist > 0 as int64_t
                                {
                                    allow_skip_nondc = false_0 != 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        mode_index += 1;
    }
    let thresh_sad: core::ffi::c_uint = (if (*cpi).sf.rt_sf.prune_palette_search_nonrd
        > 1 as core::ffi::c_int
    {
        100 as core::ffi::c_int
    } else {
        20 as core::ffi::c_int
    }) as core::ffi::c_uint;
    let best_sad_norm: core::ffi::c_uint = args.best_sad
        >> b_width_log2_lookup[bsize as usize] as core::ffi::c_int
            + b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
    let mut try_palette: bool = (*cpi).oxcf.tool_cfg.enable_palette as core::ffi::c_int
        != 0
        && av1_allow_palette(
            (*cpi).common.features.allow_screen_content_tools as core::ffi::c_int,
            (*mi).bsize,
        ) != 0;
    if (*cpi).sf.rt_sf.prune_palette_search_nonrd > 0 as core::ffi::c_int {
        let mut prune: bool = (!args.prune_mode_based_on_sad
            || best_sad_norm > thresh_sad)
            && bsize as core::ffi::c_int <= BLOCK_16X16 as core::ffi::c_int
            && (*x).source_variance > 200 as core::ffi::c_uint;
        try_palette = (try_palette as core::ffi::c_int & prune as core::ffi::c_int) != 0;
    }
    if try_palette {
        let intra_ref_frame_cost: core::ffi::c_uint = 0 as core::ffi::c_uint;
        (*x).color_palette_thresh = if best_sad_norm < 500 as core::ffi::c_uint {
            32 as core::ffi::c_int
        } else {
            64 as core::ffi::c_int
        };
        av1_search_palette_mode_luma(
            cpi,
            x,
            bsize,
            intra_ref_frame_cost,
            ctx,
            &raw mut this_rdc,
            best_rdc.rdcost,
        );
        if this_rdc.rdcost < best_rdc.rdcost {
            palette_selected = true_0 != 0;
            best_mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
            (*mi).mv[0 as core::ffi::c_int as usize].as_int = INVALID_MV as uint32_t;
            (*mi).mv[1 as core::ffi::c_int as usize].as_int = INVALID_MV as uint32_t;
            best_rdc.rate = this_rdc.rate;
            best_rdc.dist = this_rdc.dist;
            best_rdc.rdcost = this_rdc.rdcost;
            if *((*xd).tx_type_map).offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int != DCT_DCT as core::ffi::c_int
            {
                memcpy(
                    (*ctx).tx_type_map as *mut core::ffi::c_void,
                    (*xd).tx_type_map as *const core::ffi::c_void,
                    ((*ctx).num_4x4_blk as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
            }
        } else {
            memset(
                &raw mut (*mi).palette_mode_info as *mut PALETTE_MODE_INFO
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<PALETTE_MODE_INFO>() as size_t,
            );
        }
    }
    let mut try_intrabc: bool = (*cpi).sf.rt_sf.rt_use_intrabc != 0
        && av1_allow_intrabc(cm) != 0
        && bsize as core::ffi::c_int <= BLOCK_16X16 as core::ffi::c_int
        && ((*cpi).sf.rt_sf.rt_prune_intrabc_nonrd == 0
            || palette_selected as core::ffi::c_int != 0);
    if try_intrabc {
        let mut best_dv: int_mv = int_mv { as_int: 0 };
        av1_search_intrabc_nonrd(cpi, x, bsize, &raw mut this_rdc, &raw mut best_dv);
        if this_rdc.rdcost < best_rdc.rdcost {
            best_rdc = this_rdc;
            best_mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
            (*mi).set_use_intrabc(1 as uint8_t as uint8_t);
            (*mi).mv[0 as core::ffi::c_int as usize] = best_dv;
            (*mi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
            (*mi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
            (*mi).interp_filters = av1_broadcast_interp_filter(BILINEAR);
            memset(
                &raw mut (*mi).palette_mode_info as *mut PALETTE_MODE_INFO
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<PALETTE_MODE_INFO>() as size_t,
            );
            memset(
                ((*mi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
                (*mi).tx_size as core::ffi::c_int,
                ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
            );
        } else {
            (*mi).set_use_intrabc(0 as uint8_t as uint8_t);
        }
    }
    (*mi).mode = best_mode;
    (*mi).uv_mode = UV_DC_PRED as core::ffi::c_int as UV_PREDICTION_MODE;
    *rd_cost = best_rdc;
    if is_lossless_requested(&raw mut (*cpi).oxcf.rc_cfg) != 0 {
        (*x).txfm_search_info.skip_txfm = 0 as uint8_t;
    }
    store_coding_context_nonrd(x, ctx);
}
#[inline]
unsafe extern "C-unwind" fn is_same_gf_and_last_scale(
    mut cm: *mut AV1_COMMON,
) -> core::ffi::c_int {
    let sf_last: *mut scale_factors = get_ref_scale_factors(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *mut scale_factors;
    let sf_golden: *mut scale_factors = get_ref_scale_factors(
        cm,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as *mut scale_factors;
    return ((*sf_last).x_scale_fp == (*sf_golden).x_scale_fp
        && (*sf_last).y_scale_fp == (*sf_golden).y_scale_fp) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame_use_mask(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mi: *mut MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut gf_temporal_ref: core::ffi::c_int,
    mut use_ref_frame: *mut core::ffi::c_int,
    mut force_skip_low_temp_var: *mut core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let seg: *const segmentation = &raw mut (*cm).seg;
    let is_small_sb: core::ffi::c_int = ((*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_64X64 as core::ffi::c_int) as core::ffi::c_int;
    let mut use_alt_ref_frame: core::ffi::c_int = ((*(*cpi).ppi)
        .rtc_ref
        .set_ref_frame_config != 0 || (*cpi).sf.rt_sf.use_nonrd_altref_frame != 0)
        as core::ffi::c_int;
    let mut use_golden_ref_frame: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut use_last_ref_frame: core::ffi::c_int = 1 as core::ffi::c_int;
    if (*(*cpi).ppi).rtc_ref.set_ref_frame_config != 0 {
        use_last_ref_frame = if (*cpi).ref_frame_flags
            & AOM_LAST_FLAG as core::ffi::c_int != 0
        {
            use_last_ref_frame
        } else {
            0 as core::ffi::c_int
        };
    }
    if (*(*cpi).ppi).rtc_ref.set_ref_frame_config == 0 && use_last_ref_frame != 0
        && (*cpi).rc.frames_since_golden == 0 as core::ffi::c_int && gf_temporal_ref != 0
    {
        use_golden_ref_frame = 0 as core::ffi::c_int;
    }
    if use_last_ref_frame != 0 && (*cpi).sf.rt_sf.short_circuit_low_temp_var != 0
        && (*x).nonrd_prune_ref_frame_search != 0
    {
        if is_small_sb != 0 {
            *force_skip_low_temp_var = av1_get_force_skip_low_temp_var_small_sb(
                &raw mut *((*x).part_search_info.variance_low)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                mi_row,
                mi_col,
                bsize,
            );
        } else {
            *force_skip_low_temp_var = av1_get_force_skip_low_temp_var(
                &raw mut *((*x).part_search_info.variance_low)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                mi_row,
                mi_col,
                bsize,
            );
        }
        if *force_skip_low_temp_var != 0 {
            use_golden_ref_frame = 0 as core::ffi::c_int;
            use_alt_ref_frame = 0 as core::ffi::c_int;
        }
    }
    if use_last_ref_frame != 0
        && ((*x).nonrd_prune_ref_frame_search > 2 as core::ffi::c_int
            || (*x).force_zeromv_skip_for_blk != 0
            || (*x).nonrd_prune_ref_frame_search > 1 as core::ffi::c_int
                && bsize as core::ffi::c_int > BLOCK_64X64 as core::ffi::c_int)
    {
        use_golden_ref_frame = 0 as core::ffi::c_int;
        use_alt_ref_frame = 0 as core::ffi::c_int;
    }
    if segfeature_active(
        seg,
        (*mi).segment_id(),
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        && get_segdata(
            seg,
            (*mi).segment_id() as core::ffi::c_int,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) == GOLDEN_FRAME as core::ffi::c_int
    {
        *use_ref_frame.offset(GOLDEN_FRAME as core::ffi::c_int as isize) = 1
            as core::ffi::c_int;
        *use_ref_frame.offset(ALTREF_FRAME as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
        return;
    } else if segfeature_active(
        seg,
        (*mi).segment_id(),
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        && get_segdata(
            seg,
            (*mi).segment_id() as core::ffi::c_int,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) == ALTREF_FRAME as core::ffi::c_int
    {
        *use_ref_frame.offset(GOLDEN_FRAME as core::ffi::c_int as isize) = 0
            as core::ffi::c_int;
        *use_ref_frame.offset(ALTREF_FRAME as core::ffi::c_int as isize) = 1
            as core::ffi::c_int;
        return;
    }
    if use_last_ref_frame != 0
        && ((*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
            == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
            && (*x).nonrd_prune_ref_frame_search != 0 as core::ffi::c_int
            || (*x).source_variance < 200 as core::ffi::c_uint
                && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
                    >= kLowSad as core::ffi::c_int as core::ffi::c_uint)
    {
        if (*x)
            .color_sensitivity_sb_g[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 1 as core::ffi::c_int
            || (*x)
                .color_sensitivity_sb_g[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            use_golden_ref_frame = 0 as core::ffi::c_int;
        }
        if (*x)
            .color_sensitivity_sb_alt[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 1 as core::ffi::c_int
            || (*x)
                .color_sensitivity_sb_alt[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            use_alt_ref_frame = 0 as core::ffi::c_int;
        }
    }
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        != AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).ref_frame_flags & AOM_LAST_FLAG as core::ffi::c_int != 0
        && use_golden_ref_frame == 0 && use_alt_ref_frame == 0
        && (*x).pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize] != INT_MAX
        && (*x).nonrd_prune_ref_frame_search > 2 as core::ffi::c_int
        && (*x)
            .color_sensitivity_sb_g[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 0 as core::ffi::c_int
        && (*x)
            .color_sensitivity_sb_g[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 0 as core::ffi::c_int
    {
        let mut thr: core::ffi::c_int = if (*cm).width * (*cm).height > RESOLUTION_288P {
            100 as core::ffi::c_int
        } else {
            150 as core::ffi::c_int
        };
        let mut pred: core::ffi::c_int = (*x)
            .pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize]
            >> b_width_log2_lookup[bsize as usize] as core::ffi::c_int
                + b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
        if pred > thr {
            use_golden_ref_frame = 1 as core::ffi::c_int;
        }
    }
    use_alt_ref_frame = if (*cpi).ref_frame_flags & AOM_ALT_FLAG as core::ffi::c_int != 0
    {
        use_alt_ref_frame
    } else {
        0 as core::ffi::c_int
    };
    use_golden_ref_frame = if (*cpi).ref_frame_flags & AOM_GOLD_FLAG as core::ffi::c_int
        != 0
    {
        use_golden_ref_frame
    } else {
        0 as core::ffi::c_int
    };
    if (*cpi).svc.spatial_layer_id > 0 as core::ffi::c_int
        && (*cpi).ref_frame_flags & AOM_GOLD_FLAG as core::ffi::c_int != 0
        && ((*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint)
            < kHighSad as core::ffi::c_int as core::ffi::c_uint
    {
        let buffslot_golden: core::ffi::c_int = (*(*cpi).ppi)
            .rtc_ref
            .ref_idx[(GOLDEN_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
            as usize];
        if (*(*cpi).ppi).rtc_ref.buffer_time_index[buffslot_golden as usize]
            == (*cpi).svc.current_superframe
        {
            use_golden_ref_frame = 1 as core::ffi::c_int;
        }
    }
    *use_ref_frame.offset(ALTREF_FRAME as core::ffi::c_int as isize) = use_alt_ref_frame;
    *use_ref_frame.offset(GOLDEN_FRAME as core::ffi::c_int as isize) = use_golden_ref_frame;
    *use_ref_frame.offset(LAST_FRAME as core::ffi::c_int as isize) = use_last_ref_frame;
    if !(use_last_ref_frame != 0 || use_golden_ref_frame != 0 || use_alt_ref_frame != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"get_ref_frame_use_mask\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            2113 as core::ffi::c_int,
            b"use_last_ref_frame || use_golden_ref_frame || use_alt_ref_frame\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
}
#[inline]
unsafe extern "C-unwind" fn is_filter_search_enabled_blk(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut segment_id: core::ffi::c_int,
    mut cb_pred_filter_search: core::ffi::c_int,
    mut filt_select: *mut InterpFilter,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    if (*cpi).sf.rt_sf.use_nonrd_filter_search == 0 {
        return 0 as core::ffi::c_int;
    }
    if cb_pred_filter_search == 0 {
        return 1 as core::ffi::c_int;
    }
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut enable_interp_search: core::ffi::c_int = 0 as core::ffi::c_int;
    if !(!((*xd).left_mbmi).is_null() && !((*xd).above_mbmi).is_null()) {
        enable_interp_search = 2 as core::ffi::c_int;
    } else if !(is_inter_block((*xd).left_mbmi) != 0
        && is_inter_block((*xd).above_mbmi) != 0)
    {
        enable_interp_search = 2 as core::ffi::c_int;
    } else if (*(*xd).left_mbmi).interp_filters.as_int
        != (*(*xd).above_mbmi).interp_filters.as_int
    {
        enable_interp_search = 2 as core::ffi::c_int;
    } else if cb_pred_filter_search == 1 as core::ffi::c_int
        && (*(*xd).left_mbmi).interp_filters.as_filters.x_filter as core::ffi::c_int
            != EIGHTTAP_REGULAR as core::ffi::c_int
    {
        enable_interp_search = 2 as core::ffi::c_int;
    } else {
        if (*(*xd).left_mbmi).interp_filters.as_filters.x_filter as core::ffi::c_int
            == EIGHTTAP_SMOOTH as core::ffi::c_int
        {
            *filt_select = EIGHTTAP_SMOOTH;
        }
        let bsl: core::ffi::c_int = mi_size_wide_log2[bsize as usize]
            as core::ffi::c_int;
        enable_interp_search = ((mi_row + mi_col >> bsl)
            + get_chessboard_index((*cm).current_frame.frame_number as core::ffi::c_int)
            & 0x1 as core::ffi::c_int != 0) as core::ffi::c_int;
        if cyclic_refresh_segment_id_boosted(segment_id) != 0 {
            enable_interp_search = 1 as core::ffi::c_int;
        }
    }
    return enable_interp_search;
}
#[inline]
unsafe extern "C-unwind" fn skip_mode_by_threshold(
    mut mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut mv: int_mv,
    mut frames_since_golden: core::ffi::c_int,
    rd_threshes: *const core::ffi::c_int,
    rd_thresh_freq_fact: *const core::ffi::c_int,
    mut best_cost: int64_t,
    mut best_skip: core::ffi::c_int,
    mut extra_shift: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut skip_this_mode: core::ffi::c_int = 0 as core::ffi::c_int;
    let mode_index: THR_MODES = mode_idx[ref_frame
        as usize][(mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as usize];
    let mut mode_rd_thresh: int64_t = if best_skip != 0 {
        (*rd_threshes.offset(mode_index as isize) as int64_t)
            << extra_shift + 1 as core::ffi::c_int
    } else {
        (*rd_threshes.offset(mode_index as isize) as int64_t) << extra_shift
    };
    if ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int {
        mode_rd_thresh = mode_rd_thresh << 1 as core::ffi::c_int;
        if ref_frame as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int
            && frames_since_golden > 4 as core::ffi::c_int
        {
            mode_rd_thresh = mode_rd_thresh << extra_shift + 1 as core::ffi::c_int;
        }
    }
    if rd_less_than_thresh(
        best_cost,
        mode_rd_thresh,
        *rd_thresh_freq_fact.offset(mode_index as isize),
    ) != 0
    {
        if mv.as_int != 0 as uint32_t {
            skip_this_mode = 1 as core::ffi::c_int;
        }
    }
    return skip_this_mode;
}
#[inline]
unsafe extern "C-unwind" fn skip_mode_by_low_temp(
    mut mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut bsize: BLOCK_SIZE,
    mut content_state_sb: CONTENT_STATE_SB,
    mut mv: int_mv,
    mut force_skip_low_temp_var: core::ffi::c_int,
) -> core::ffi::c_int {
    if force_skip_low_temp_var != 0
        && ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
        && mv.as_int != 0 as uint32_t
    {
        return 1 as core::ffi::c_int;
    }
    if content_state_sb.source_sad_nonrd as core::ffi::c_uint
        != kHighSad as core::ffi::c_int as core::ffi::c_uint
        && bsize as core::ffi::c_int >= BLOCK_64X64 as core::ffi::c_int
        && force_skip_low_temp_var != 0
        && mode as core::ffi::c_int == NEWMV as core::ffi::c_int
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn skip_mode_by_bsize_and_ref_frame(
    mut mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut bsize: BLOCK_SIZE,
    mut extra_prune: core::ffi::c_int,
    mut sse_zeromv_norm: core::ffi::c_uint,
    mut more_prune: core::ffi::c_int,
    mut skip_nearmv: core::ffi::c_int,
) -> core::ffi::c_int {
    let thresh_skip_golden: core::ffi::c_uint = 500 as core::ffi::c_uint;
    if ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
        && sse_zeromv_norm < thresh_skip_golden
        && mode as core::ffi::c_int == NEWMV as core::ffi::c_int
    {
        return 1 as core::ffi::c_int;
    }
    if bsize as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        && mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        || skip_nearmv != 0 && mode as core::ffi::c_int == NEARMV as core::ffi::c_int
    {
        return 1 as core::ffi::c_int;
    }
    if extra_prune != 0 {
        if extra_prune > 1 as core::ffi::c_int
            && ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
            && (bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int
                && mode as core::ffi::c_int == NEWMV as core::ffi::c_int)
        {
            return 1 as core::ffi::c_int;
        }
        if ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
            && mode as core::ffi::c_int == NEARMV as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
        if more_prune != 0
            && bsize as core::ffi::c_int >= BLOCK_32X32 as core::ffi::c_int
            && mode as core::ffi::c_int == NEARMV as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_block_source_sad(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut yv12_mb: *mut buf_2d,
) {
    let p: *mut macroblock_plane = &raw mut *((*x).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblock_plane;
    let y_sad: core::ffi::c_int = ((*(*cpi).ppi).fn_ptr[bsize as usize].sdf)
        .expect(
            "non-null function pointer",
        )((*p).src.buf, (*p).src.stride, (*yv12_mb).buf, (*yv12_mb).stride)
        as core::ffi::c_int;
    if y_sad == 0 as core::ffi::c_int {
        (*x).block_is_zero_sad = 1 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn set_color_sensitivity(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut y_sad: core::ffi::c_int,
    mut source_variance: core::ffi::c_uint,
    mut yv12_mb: *mut buf_2d,
) {
    let subsampling_x: core::ffi::c_int = (*(*cpi).common.seq_params).subsampling_x;
    let subsampling_y: core::ffi::c_int = (*(*cpi).common.seq_params).subsampling_y;
    let source_sad_nonrd: core::ffi::c_int = (*x).content_state_sb.source_sad_nonrd
        as core::ffi::c_int;
    let high_res: core::ffi::c_int = ((*cpi).common.width * (*cpi).common.height
        >= 640 as core::ffi::c_int * 360 as core::ffi::c_int) as core::ffi::c_int;
    if bsize as core::ffi::c_int
        == (*(*cpi).common.seq_params).sb_size as core::ffi::c_int
        && (*x).force_color_check_block_level == 0
    {
        if (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int == 2 as core::ffi::c_int
        {
            (*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] = (if source_sad_nonrd >= kMedSad as core::ffi::c_int {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint8_t;
        }
        if (*x)
            .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int == 2 as core::ffi::c_int
        {
            (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] = (if source_sad_nonrd >= kMedSad as core::ffi::c_int {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint8_t;
        }
        return;
    }
    let mut shift: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut source_var_thr: core::ffi::c_uint = 50 as core::ffi::c_uint;
    let mut norm_uv_sad_thresh: core::ffi::c_int = 100 as core::ffi::c_int;
    let mut norm_uv_sad_thresh2: core::ffi::c_int = 40 as core::ffi::c_int;
    if source_sad_nonrd >= kMedSad as core::ffi::c_int
        && (*x).source_variance > 0 as core::ffi::c_uint && high_res != 0
    {
        shift = 4 as core::ffi::c_int;
    }
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
    {
        if (*cpi).rc.high_source_sad != 0 {
            shift = 6 as core::ffi::c_int;
        }
        if source_sad_nonrd > kMedSad as core::ffi::c_int {
            source_var_thr = 1200 as core::ffi::c_uint;
            norm_uv_sad_thresh = 10 as core::ffi::c_int;
        }
        if (*cpi).rc.percent_blocks_with_motion > 90 as core::ffi::c_int
            && (*cpi).rc.frame_source_sad > 10000 as uint64_t
            && source_sad_nonrd > kLowSad as core::ffi::c_int
        {
            shift = 10 as core::ffi::c_int;
            norm_uv_sad_thresh = 0 as core::ffi::c_int;
            norm_uv_sad_thresh2 = 0 as core::ffi::c_int;
        }
    }
    let mut noise_level: NOISE_LEVEL = kLow;
    let mut norm_sad: core::ffi::c_int = y_sad
        >> b_width_log2_lookup[bsize as usize] as core::ffi::c_int
            + b_height_log2_lookup[bsize as usize] as core::ffi::c_int;
    let mut thresh_spatial: core::ffi::c_uint = (if (*cpi).common.width
        > 1920 as core::ffi::c_int
    {
        5000 as core::ffi::c_int
    } else {
        1000 as core::ffi::c_int
    }) as core::ffi::c_uint;
    if (*cpi).noise_estimate.enabled != 0 {
        noise_level = av1_noise_estimate_extract_level(&raw mut (*cpi).noise_estimate);
    }
    if noise_level as core::ffi::c_uint == kLow as core::ffi::c_int as core::ffi::c_uint
        && source_variance > thresh_spatial
        && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
            != AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && norm_sad < 50 as core::ffi::c_int
    {
        (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] = 0 as uint8_t;
        (*x)
            .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] = 0 as uint8_t;
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(&raw mut (*cpi).common)
        as core::ffi::c_int;
    let mut plane: core::ffi::c_int = AOM_PLANE_U;
    while plane < num_planes {
        if (*x).color_sensitivity[(plane - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int == 2 as core::ffi::c_int
            || (*x).force_color_check_block_level != 0
            || (*x).color_sensitivity[(plane - 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int == 0 as core::ffi::c_int
                && source_sad_nonrd >= kMedSad as core::ffi::c_int && high_res != 0
        {
            let p: *mut macroblock_plane = &raw mut *((*x).plane)
                .as_mut_ptr()
                .offset(plane as isize) as *mut macroblock_plane;
            let bs: BLOCK_SIZE = get_plane_block_size(
                bsize,
                subsampling_x,
                subsampling_y,
            ) as BLOCK_SIZE;
            let uv_sad: core::ffi::c_int = ((*(*cpi).ppi).fn_ptr[bs as usize].sdf)
                .expect(
                    "non-null function pointer",
                )(
                (*p).src.buf,
                (*p).src.stride,
                (*yv12_mb.offset(plane as isize)).buf,
                (*yv12_mb.offset(plane as isize)).stride,
            ) as core::ffi::c_int;
            let norm_uv_sad: core::ffi::c_int = uv_sad
                >> b_width_log2_lookup[bs as usize] as core::ffi::c_int
                    + b_height_log2_lookup[bs as usize] as core::ffi::c_int;
            (*x).color_sensitivity[(plane - 1 as core::ffi::c_int) as usize] = (uv_sad
                > y_sad >> shift && norm_uv_sad > norm_uv_sad_thresh2)
                as core::ffi::c_int as uint8_t;
            if source_variance < source_var_thr && norm_uv_sad > norm_uv_sad_thresh {
                (*x).color_sensitivity[(plane - 1 as core::ffi::c_int) as usize] = 1
                    as uint8_t;
            }
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn setup_compound_prediction(
    mut cm: *const AV1_COMMON,
    mut x: *mut MACROBLOCK,
    mut yv12_mb: *mut [buf_2d; 3],
    mut use_ref_frame_mask: *const core::ffi::c_int,
    mut rf: *const MV_REFERENCE_FRAME,
    mut ref_mv_idx: *mut core::ffi::c_int,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mbmi_ext: *mut MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let mut ref_frame_comp: MV_REFERENCE_FRAME = 0;
    if *use_ref_frame_mask.offset(*rf.offset(1 as core::ffi::c_int as isize) as isize)
        == 0
    {
        let mut yv12: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
            cm,
            *rf.offset(1 as core::ffi::c_int as isize),
        );
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        if !yv12.is_null() {
            let sf: *const scale_factors = get_ref_scale_factors_const(
                cm,
                *rf.offset(1 as core::ffi::c_int as isize),
            ) as *const scale_factors;
            av1_setup_pred_block(
                xd,
                (*yv12_mb.offset(*rf.offset(1 as core::ffi::c_int as isize) as isize))
                    .as_mut_ptr(),
                yv12,
                sf,
                sf,
                num_planes,
            );
        }
    }
    ref_frame_comp = av1_ref_frame_type(rf) as MV_REFERENCE_FRAME;
    (*mbmi_ext).mode_context[ref_frame_comp as usize] = 0 as int16_t;
    (*mbmi_ext).ref_mv_count[ref_frame_comp as usize] = UINT8_MAX as uint8_t;
    av1_find_mv_refs(
        cm,
        xd,
        mbmi,
        ref_frame_comp,
        ((*mbmi_ext).ref_mv_count).as_mut_ptr(),
        ((*xd).ref_mv_stack).as_mut_ptr(),
        ((*xd).weight).as_mut_ptr(),
        0 as *mut [int_mv; 2],
        ((*mbmi_ext).global_mvs).as_mut_ptr(),
        ((*mbmi_ext).mode_context).as_mut_ptr(),
    );
    av1_copy_usable_ref_mv_stack_and_weight(xd, mbmi_ext, ref_frame_comp);
    *ref_mv_idx = (*mbmi).ref_mv_idx() as core::ffi::c_int + 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_compound_mode(
    mut x: *mut MACROBLOCK,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut ref_frame2: MV_REFERENCE_FRAME,
    mut ref_mv_idx: core::ffi::c_int,
    mut frame_mv: *mut [int_mv; 8],
    mut this_mode: PREDICTION_MODE,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mi).ref_frame[0 as core::ffi::c_int as usize] = ref_frame;
    (*mi).ref_frame[1 as core::ffi::c_int as usize] = ref_frame2;
    (*mi).set_compound_idx(1 as uint8_t as uint8_t);
    (*mi).set_comp_group_idx(0 as uint8_t as uint8_t);
    (*mi).interinter_comp.type_0 = COMPOUND_AVERAGE as core::ffi::c_int as COMPOUND_TYPE;
    let mut ref_frame_comp: MV_REFERENCE_FRAME = av1_ref_frame_type(
        ((*mi).ref_frame).as_mut_ptr(),
    ) as MV_REFERENCE_FRAME;
    if this_mode as core::ffi::c_int == GLOBAL_GLOBALMV as core::ffi::c_int {
        (*frame_mv.offset(this_mode as isize))[ref_frame as usize].as_int = 0
            as uint32_t;
        (*frame_mv.offset(this_mode as isize))[ref_frame2 as usize].as_int = 0
            as uint32_t;
    } else if this_mode as core::ffi::c_int == NEAREST_NEARESTMV as core::ffi::c_int {
        (*frame_mv.offset(this_mode as isize))[ref_frame as usize].as_int = (*xd)
            .ref_mv_stack[ref_frame_comp as usize][0 as core::ffi::c_int as usize]
            .this_mv
            .as_int;
        (*frame_mv.offset(this_mode as isize))[ref_frame2 as usize].as_int = (*xd)
            .ref_mv_stack[ref_frame_comp as usize][0 as core::ffi::c_int as usize]
            .comp_mv
            .as_int;
    } else if this_mode as core::ffi::c_int == NEAR_NEARMV as core::ffi::c_int {
        (*frame_mv.offset(this_mode as isize))[ref_frame as usize].as_int = (*xd)
            .ref_mv_stack[ref_frame_comp as usize][ref_mv_idx as usize]
            .this_mv
            .as_int;
        (*frame_mv.offset(this_mode as isize))[ref_frame2 as usize].as_int = (*xd)
            .ref_mv_stack[ref_frame_comp as usize][ref_mv_idx as usize]
            .comp_mv
            .as_int;
    }
}
unsafe extern "C-unwind" fn skip_comp_based_on_var(
    mut single_vars: *const [core::ffi::c_uint; 8],
    mut bsize: BLOCK_SIZE,
) -> bool {
    let mut best_var: core::ffi::c_uint = UINT_MAX;
    let mut cur_mode_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while cur_mode_idx < RTC_INTER_MODES {
        let mut ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while ref_idx < REF_FRAMES as core::ffi::c_int {
            best_var = if best_var
                < (*single_vars.offset(cur_mode_idx as isize))[ref_idx as usize]
            {
                best_var
            } else {
                (*single_vars.offset(cur_mode_idx as isize))[ref_idx as usize]
            };
            ref_idx += 1;
        }
        cur_mode_idx += 1;
    }
    let thresh_64: core::ffi::c_uint = (0.57356805f32
        * 8659 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_uint;
    let thresh_32: core::ffi::c_uint = (0.23964763f32
        * 4281 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_uint;
    match bsize as core::ffi::c_int {
        15 => return best_var < (4 as core::ffi::c_uint).wrapping_mul(thresh_64),
        12 => return best_var < thresh_64,
        9 => return best_var < thresh_32,
        6 => return best_var < thresh_32.wrapping_div(4 as core::ffi::c_uint),
        _ => return false_0 != 0,
    };
}
#[inline(always)]
unsafe extern "C-unwind" fn fill_single_inter_mode_costs(
    mut single_inter_mode_costs: *mut [core::ffi::c_int; 8],
    mut num_inter_modes: core::ffi::c_int,
    mut reference_mode_set: *const REF_MODE,
    mut mode_costs: *const ModeCosts,
    mut mode_context: *const int16_t,
) {
    let mut ref_frame_used: [bool; 8] = [
        false_0 != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < num_inter_modes {
        ref_frame_used[(*reference_mode_set.offset(idx as isize)).ref_frame as usize] = true_0
            != 0;
        idx += 1;
    }
    let mut this_ref_frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while this_ref_frame < REF_FRAMES as core::ffi::c_int {
        if ref_frame_used[this_ref_frame as usize] {
            let rf: [MV_REFERENCE_FRAME; 2] = [
                this_ref_frame as MV_REFERENCE_FRAME,
                NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ];
            let mode_ctx: int16_t = av1_mode_context_analyzer(mode_context, rf.as_ptr())
                as int16_t;
            let mut this_mode: PREDICTION_MODE = NEARESTMV as core::ffi::c_int
                as PREDICTION_MODE;
            while this_mode as core::ffi::c_int <= NEWMV as core::ffi::c_int {
                (*single_inter_mode_costs
                    .offset(
                        (this_mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                            as isize,
                    ))[this_ref_frame as usize] = cost_mv_ref(
                    mode_costs,
                    this_mode,
                    mode_ctx,
                );
                this_mode = this_mode.wrapping_add(1);
            }
        }
        this_ref_frame += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn is_globalmv_better(
    mut this_mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut rate_mv: core::ffi::c_int,
    mut mode_costs: *const ModeCosts,
    mut single_inter_mode_costs: *const [core::ffi::c_int; 8],
    mut mbmi_ext: *const MB_MODE_INFO_EXT,
) -> bool {
    let globalmv_mode_cost: core::ffi::c_int = (*single_inter_mode_costs
        .offset(
            (GLOBALMV as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as isize,
        ))[ref_frame as usize];
    let mut this_mode_cost: core::ffi::c_int = rate_mv
        + (*single_inter_mode_costs
            .offset(
                (this_mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as isize,
            ))[ref_frame as usize];
    if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        || this_mode as core::ffi::c_int == NEARMV as core::ffi::c_int
    {
        let rf: [MV_REFERENCE_FRAME; 2] = [
            ref_frame,
            NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ];
        this_mode_cost
            += get_drl_cost(
                NEWMV as core::ffi::c_int as PREDICTION_MODE,
                0 as core::ffi::c_int,
                mbmi_ext,
                ((*mode_costs).drl_mode_cost0).as_ptr() as *const [core::ffi::c_int; 2],
                av1_ref_frame_type(rf.as_ptr()),
            );
    }
    return this_mode_cost > globalmv_mode_cost;
}
#[inline]
unsafe extern "C-unwind" fn setup_compound_params_from_comp_idx(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut yv12_mb: *mut [buf_2d; 3],
    mut this_mode: *mut PREDICTION_MODE,
    mut ref_frame: *mut MV_REFERENCE_FRAME,
    mut ref_frame2: *mut MV_REFERENCE_FRAME,
    mut frame_mv: *mut [int_mv; 8],
    mut use_ref_frame_mask: *const core::ffi::c_int,
    mut comp_index: core::ffi::c_int,
    mut comp_use_zero_zeromv_only: bool,
    mut last_comp_ref_frame: *mut MV_REFERENCE_FRAME,
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut rf: *const MV_REFERENCE_FRAME = if is_one_pass_rt_lag_params(cpi) != 0 {
        (comp_ref_mode_set_full[comp_index as usize].ref_frame).as_ptr()
    } else {
        (comp_ref_mode_set[comp_index as usize].ref_frame).as_ptr()
    };
    let mut skip_gf: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut skip_alt: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*x).source_variance < 50 as core::ffi::c_uint
        && bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int
    {
        if (*x)
            .color_sensitivity_sb_g[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 1 as core::ffi::c_int
            || (*x)
                .color_sensitivity_sb_g[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            skip_gf = 1 as core::ffi::c_int;
        }
        if (*x)
            .color_sensitivity_sb_alt[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] as core::ffi::c_int == 1 as core::ffi::c_int
            || (*x)
                .color_sensitivity_sb_alt[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 1 as core::ffi::c_int
        {
            skip_alt = 1 as core::ffi::c_int;
        }
    }
    if comp_use_zero_zeromv_only as core::ffi::c_int != 0
        && *this_mode as core::ffi::c_int != GLOBAL_GLOBALMV as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if *ref_frame2 as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int
        && ((*cpi).sf.rt_sf.ref_frame_comp_nonrd[0 as core::ffi::c_int as usize]
            == 0 as core::ffi::c_int || skip_gf != 0
            || (*cpi).ref_frame_flags & AOM_GOLD_FLAG as core::ffi::c_int == 0)
    {
        return 0 as core::ffi::c_int
    } else if *ref_frame2 as core::ffi::c_int == LAST2_FRAME as core::ffi::c_int
        && ((*cpi).sf.rt_sf.ref_frame_comp_nonrd[1 as core::ffi::c_int as usize]
            == 0 as core::ffi::c_int
            || (*cpi).ref_frame_flags & AOM_LAST2_FLAG as core::ffi::c_int == 0)
    {
        return 0 as core::ffi::c_int
    } else if *ref_frame2 as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
        && ((*cpi).sf.rt_sf.ref_frame_comp_nonrd[2 as core::ffi::c_int as usize]
            == 0 as core::ffi::c_int || skip_alt != 0
            || (*cpi).ref_frame_flags & AOM_ALT_FLAG as core::ffi::c_int == 0)
    {
        return 0 as core::ffi::c_int
    }
    let mut ref_mv_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    if *last_comp_ref_frame as core::ffi::c_int
        != *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
    {
        setup_compound_prediction(
            &(*cpi).common,
            x,
            yv12_mb,
            use_ref_frame_mask,
            rf,
            &raw mut ref_mv_idx,
        );
        *last_comp_ref_frame = *rf.offset(1 as core::ffi::c_int as isize);
    }
    set_compound_mode(x, *ref_frame, *ref_frame2, ref_mv_idx, frame_mv, *this_mode);
    if *this_mode as core::ffi::c_int != GLOBAL_GLOBALMV as core::ffi::c_int
        && (*frame_mv.offset(*this_mode as isize))[*ref_frame as usize].as_int
            == 0 as uint32_t
        && (*frame_mv.offset(*this_mode as isize))[*ref_frame2 as usize].as_int
            == 0 as uint32_t
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn previous_mode_performed_poorly(
    mut mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut vars: *const [core::ffi::c_uint; 8],
    mut uv_dist: *const [int64_t; 8],
) -> bool {
    let mut best_var: core::ffi::c_uint = UINT_MAX;
    let mut best_uv_dist: int64_t = INT64_MAX as int64_t;
    let mut midx: core::ffi::c_int = 0 as core::ffi::c_int;
    while midx < RTC_INTER_MODES {
        best_var = if best_var < (*vars.offset(midx as isize))[ref_frame as usize] {
            best_var
        } else {
            (*vars.offset(midx as isize))[ref_frame as usize]
        };
        best_uv_dist = if best_uv_dist
            < (*uv_dist.offset(midx as isize))[ref_frame as usize]
        {
            best_uv_dist
        } else {
            (*uv_dist.offset(midx as isize))[ref_frame as usize]
        };
        midx += 1;
    }
    if !(best_var
        != (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
        && !(b"Invalid variance data.\0" as *const u8 as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"previous_mode_performed_poorly\0"))
                .as_ptr(),
            b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
            2521 as core::ffi::c_int,
            b"best_var != UINT_MAX && \"Invalid variance data.\"\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mult: core::ffi::c_float = 1.125f32;
    let mut var_bad: bool = (mult * best_var as core::ffi::c_float)
        < (*vars
            .offset(
                (mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as isize,
            ))[ref_frame as usize] as core::ffi::c_float;
    if (*uv_dist
        .offset(
            (mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as isize,
        ))[ref_frame as usize] < INT64_MAX as int64_t
        && best_uv_dist
            != (*uv_dist
                .offset(
                    (mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int) as isize,
                ))[ref_frame as usize]
    {
        var_bad = (var_bad as core::ffi::c_int
            & ((mult * best_uv_dist as core::ffi::c_float)
                < (*uv_dist
                    .offset(
                        (mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                            as isize,
                    ))[ref_frame as usize] as core::ffi::c_float) as core::ffi::c_int) != 0;
    }
    return var_bad;
}
#[inline]
unsafe extern "C-unwind" fn prune_compoundmode_with_singlemode_var(
    mut compound_mode: PREDICTION_MODE,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut ref_frame2: MV_REFERENCE_FRAME,
    mut frame_mv: *const [int_mv; 8],
    mut mode_checked: *const [uint8_t; 8],
    mut vars: *const [core::ffi::c_uint; 8],
    mut uv_dist: *const [int64_t; 8],
) -> bool {
    let single_mode0: PREDICTION_MODE = compound_ref0_mode(compound_mode)
        as PREDICTION_MODE;
    let single_mode1: PREDICTION_MODE = compound_ref1_mode(compound_mode)
        as PREDICTION_MODE;
    let mut first_ref_valid: bool = false_0 != 0;
    let mut second_ref_valid: bool = false_0 != 0;
    let mut first_ref_bad: bool = false_0 != 0;
    let mut second_ref_bad: bool = false_0 != 0;
    if (*mode_checked.offset(single_mode0 as isize))[ref_frame as usize]
        as core::ffi::c_int != 0
        && (*frame_mv.offset(single_mode0 as isize))[ref_frame as usize].as_int
            == (*frame_mv.offset(compound_mode as isize))[ref_frame as usize].as_int
        && (*vars
            .offset(
                (single_mode0 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                    as isize,
            ))[ref_frame as usize] < UINT_MAX
    {
        first_ref_valid = true_0 != 0;
        first_ref_bad = previous_mode_performed_poorly(
            single_mode0,
            ref_frame,
            vars,
            uv_dist,
        );
    }
    if (*mode_checked.offset(single_mode1 as isize))[ref_frame2 as usize]
        as core::ffi::c_int != 0
        && (*frame_mv.offset(single_mode1 as isize))[ref_frame2 as usize].as_int
            == (*frame_mv.offset(compound_mode as isize))[ref_frame2 as usize].as_int
        && (*vars
            .offset(
                (single_mode1 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                    as isize,
            ))[ref_frame2 as usize] < UINT_MAX
    {
        second_ref_valid = true_0 != 0;
        second_ref_bad = previous_mode_performed_poorly(
            single_mode1,
            ref_frame2,
            vars,
            uv_dist,
        );
    }
    if first_ref_valid as core::ffi::c_int != 0
        && second_ref_valid as core::ffi::c_int != 0
    {
        return first_ref_bad as core::ffi::c_int != 0
            && second_ref_bad as core::ffi::c_int != 0
    } else if first_ref_valid as core::ffi::c_int != 0
        || second_ref_valid as core::ffi::c_int != 0
    {
        return first_ref_bad as core::ffi::c_int != 0
            || second_ref_bad as core::ffi::c_int != 0
    }
    return false_0 != 0;
}
#[inline(always)]
unsafe extern "C-unwind" fn set_params_nonrd_pick_inter_mode(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut search_state: *mut InterModeSearchStateNonrd,
    mut rd_cost: *mut RD_STATS,
    mut force_skip_low_temp_var: *mut core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut gf_temporal_ref: core::ffi::c_int,
    mut segment_id: core::ffi::c_uchar,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let mut skip_pred_mv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < RTC_INTER_MODES {
        let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while ref_0 < REF_FRAMES as core::ffi::c_int {
            (*search_state).vars[idx as usize][ref_0 as usize] = UINT_MAX;
            (*search_state).uv_dist[idx as usize][ref_0 as usize] = INT64_MAX as int64_t;
            ref_0 += 1;
        }
        idx += 1;
    }
    memcpy(
        ((*x).color_sensitivity).as_mut_ptr() as *mut core::ffi::c_void,
        ((*x).color_sensitivity_sb).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
    );
    init_best_pickmode(&raw mut (*search_state).best_pickmode);
    estimate_single_ref_frame_costs(
        cm,
        xd,
        mode_costs,
        segment_id as core::ffi::c_int,
        bsize,
        ((*search_state).ref_costs_single).as_mut_ptr(),
        is_one_pass_rt_lag_params(cpi),
    );
    memset(
        &raw mut (*search_state).mode_checked as *mut [[uint8_t; 8]; 25]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[uint8_t; 8]; 25]>() as size_t,
    );
    (*txfm_info).skip_txfm = 0 as uint8_t;
    av1_invalid_rd_stats(&raw mut (*search_state).best_rdc);
    av1_invalid_rd_stats(&raw mut (*search_state).this_rdc);
    av1_invalid_rd_stats(rd_cost);
    let mut ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_idx < REF_FRAMES as core::ffi::c_int {
        (*x).warp_sample_info[ref_idx as usize].num = -(1 as core::ffi::c_int);
        ref_idx += 1;
    }
    (*mi).bsize = bsize;
    (*mi).ref_frame[0 as core::ffi::c_int as usize] = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    (*mi).ref_frame[1 as core::ffi::c_int as usize] = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    if (*cpi).ref_frame_flags & AOM_LAST_FLAG as core::ffi::c_int != 0 {
        find_predictors(
            cpi,
            x,
            LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ((*search_state).frame_mv).as_mut_ptr(),
            ((*search_state).yv12_mb).as_mut_ptr(),
            bsize,
            *force_skip_low_temp_var,
            (*x).force_zeromv_skip_for_blk,
            &raw mut *((*search_state).use_scaled_ref_frame)
                .as_mut_ptr()
                .offset(LAST_FRAME as core::ffi::c_int as isize),
        );
    }
    if is_one_pass_rt_lag_params(cpi) == 0 {
        get_ref_frame_use_mask(
            cpi,
            x,
            mi,
            mi_row,
            mi_col,
            bsize,
            gf_temporal_ref,
            ((*search_state).use_ref_frame_mask).as_mut_ptr(),
            force_skip_low_temp_var,
        );
    } else {
        let mut ref_frame_iter: MV_REFERENCE_FRAME = LAST_FRAME as core::ffi::c_int
            as MV_REFERENCE_FRAME;
        while ref_frame_iter as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
            (*search_state).use_ref_frame_mask[ref_frame_iter as usize] = (*cpi)
                .ref_frame_flags
                & (1 as core::ffi::c_int)
                    << ref_frame_iter as core::ffi::c_int - 1 as core::ffi::c_int;
            ref_frame_iter += 1;
        }
    }
    skip_pred_mv = ((*x).force_zeromv_skip_for_blk != 0
        || (*x).nonrd_prune_ref_frame_search > 2 as core::ffi::c_int
            && (*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 2 as core::ffi::c_int
            && (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 2 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut ref_frame_iter_0: MV_REFERENCE_FRAME = (LAST_FRAME as core::ffi::c_int
        + 1 as core::ffi::c_int) as MV_REFERENCE_FRAME;
    while ref_frame_iter_0 as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        if (*search_state).use_ref_frame_mask[ref_frame_iter_0 as usize] != 0 {
            find_predictors(
                cpi,
                x,
                ref_frame_iter_0,
                ((*search_state).frame_mv).as_mut_ptr(),
                ((*search_state).yv12_mb).as_mut_ptr(),
                bsize,
                *force_skip_low_temp_var,
                skip_pred_mv,
                &raw mut *((*search_state).use_scaled_ref_frame)
                    .as_mut_ptr()
                    .offset(ref_frame_iter_0 as isize),
            );
        }
        ref_frame_iter_0 += 1;
    }
}
#[inline(always)]
unsafe extern "C-unwind" fn skip_inter_mode_nonrd(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut search_state: *mut InterModeSearchStateNonrd,
    mut thresh_sad_pred: *mut int64_t,
    mut force_mv_inter_layer: *mut core::ffi::c_int,
    mut is_single_pred: *mut core::ffi::c_int,
    mut this_mode: *mut PREDICTION_MODE,
    mut last_comp_ref_frame: *mut MV_REFERENCE_FRAME,
    mut ref_frame: *mut MV_REFERENCE_FRAME,
    mut ref_frame2: *mut MV_REFERENCE_FRAME,
    mut idx: core::ffi::c_int,
    mut svc_mv: int_mv,
    mut force_skip_low_temp_var: core::ffi::c_int,
    mut sse_zeromv_norm: core::ffi::c_uint,
    mut num_inter_modes: core::ffi::c_int,
    mut segment_id: core::ffi::c_uchar,
    mut bsize: BLOCK_SIZE,
    mut comp_use_zero_zeromv_only: bool,
    mut check_globalmv: bool,
) -> bool {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let seg: *const segmentation = &raw mut (*cm).seg;
    let svc: *const SVC = &raw mut (*cpi).svc;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let rt_sf: *const REAL_TIME_SPEED_FEATURES = &raw mut (*cpi).sf.rt_sf;
    if idx >= num_inter_modes {
        let comp_index: core::ffi::c_int = idx - num_inter_modes;
        let comp_modes: *const COMP_REF_MODE = if is_one_pass_rt_lag_params(cpi) != 0 {
            core::ptr::addr_of!(comp_ref_mode_set_full).cast::<COMP_REF_MODE>()
        } else {
            core::ptr::addr_of!(comp_ref_mode_set).cast::<COMP_REF_MODE>()
        };
        let mut rf: *const MV_REFERENCE_FRAME = ((*comp_modes
            .offset(comp_index as isize))
            .ref_frame)
            .as_ptr();
        *this_mode = (*comp_modes.offset(comp_index as isize)).pred_mode;
        *ref_frame = *rf.offset(0 as core::ffi::c_int as isize);
        *ref_frame2 = *rf.offset(1 as core::ffi::c_int as isize);
        if !(*ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"skip_inter_mode_nonrd\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                2695 as core::ffi::c_int,
                b"*ref_frame == LAST_FRAME\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(*this_mode as core::ffi::c_int == GLOBAL_GLOBALMV as core::ffi::c_int
            || *this_mode as core::ffi::c_int == NEAREST_NEARESTMV as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"skip_inter_mode_nonrd\0"))
                    .as_ptr(),
                b"nonrd_pickmode.c\0" as *const u8 as *const core::ffi::c_char,
                2696 as core::ffi::c_int,
                b"*this_mode == GLOBAL_GLOBALMV || *this_mode == NEAREST_NEARESTMV\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if setup_compound_params_from_comp_idx(
            cpi,
            x,
            ((*search_state).yv12_mb).as_mut_ptr(),
            this_mode,
            ref_frame,
            ref_frame2,
            ((*search_state).frame_mv).as_mut_ptr(),
            ((*search_state).use_ref_frame_mask).as_mut_ptr(),
            comp_index,
            comp_use_zero_zeromv_only,
            last_comp_ref_frame,
            bsize,
        ) == 0
        {
            return true_0 != 0;
        }
        *is_single_pred = 0 as core::ffi::c_int;
    } else {
        *ref_frame2 = NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
        let single_modes: *const REF_MODE = if is_one_pass_rt_lag_params(cpi) != 0 {
            core::ptr::addr_of!(ref_mode_set_full).cast::<REF_MODE>()
        } else {
            core::ptr::addr_of!(ref_mode_set).cast::<REF_MODE>()
        };
        *this_mode = (*single_modes.offset(idx as isize)).pred_mode;
        *ref_frame = (*single_modes.offset(idx as isize)).ref_frame;
    }
    if is_one_pass_rt_lag_params(cpi) != 0 && (*cpi).rc.is_src_frame_alt_ref != 0 {
        if *this_mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
            || *ref_frame as core::ffi::c_int != ALTREF_FRAME as core::ffi::c_int
        {
            return true_0 != 0;
        }
    }
    if (*cpi).sf.rt_sf.skip_newmv_mode_sad_screen != 0 && (*cpi).rc.high_source_sad != 0
        && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            >= kMedSad as core::ffi::c_int as core::ffi::c_uint
        && bsize as core::ffi::c_int <= BLOCK_16X16 as core::ffi::c_int
        && (*x).sb_me_block == 0
        && (*ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
            || *this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if segfeature_active(
        &raw mut (*cm).seg,
        segment_id as uint8_t,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        && (*this_mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
            || *ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if (*search_state).use_ref_frame_mask[*ref_frame as usize] == 0 {
        return true_0 != 0;
    }
    *force_mv_inter_layer = 0 as core::ffi::c_int;
    if (*(*cpi).ppi).use_svc != 0 && (*svc).spatial_layer_id > 0 as core::ffi::c_int
        && (*ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
            && (*svc).skip_mvsearch_last != 0
            || *ref_frame as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int
                && (*svc).skip_mvsearch_gf != 0
            || *ref_frame as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int
                && (*svc).skip_mvsearch_altref != 0)
    {
        *force_mv_inter_layer = 1 as core::ffi::c_int;
        if *this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
            (*search_state).frame_mv[*this_mode as usize][*ref_frame as usize] = svc_mv;
        } else if (*search_state)
            .frame_mv[*this_mode as usize][*ref_frame as usize]
            .as_int != svc_mv.as_int
        {
            return true_0 != 0
        }
    }
    if segfeature_active(
        seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        if get_segdata(
            seg,
            segment_id as core::ffi::c_int,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != *ref_frame as core::ffi::c_int
        {
            return true_0 != 0;
        }
        return false_0 != 0;
    }
    if (*cpi).ref_frame_flags & AOM_LAST_FLAG as core::ffi::c_int == 0
        && (*ref_frame as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int
            || *ref_frame as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int)
    {
        return false_0 != 0;
    }
    if (*x).force_zeromv_skip_for_blk != 0
        && (!(*this_mode as core::ffi::c_int == NEARESTMV as core::ffi::c_int
            && (*search_state).frame_mv[*this_mode as usize][*ref_frame as usize].as_int
                == 0 as uint32_t)
            && *this_mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
            || *ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int)
    {
        return true_0 != 0;
    }
    if (*x).sb_me_block != 0
        && *ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
    {
        if *this_mode as core::ffi::c_int == NEARESTMV as core::ffi::c_int
            && (*search_state)
                .frame_mv[NEARESTMV as core::ffi::c_int
                    as usize][LAST_FRAME as core::ffi::c_int as usize]
                .as_int == (*x).sb_me_mv.as_int
        {
            return false_0 != 0;
        }
        if *this_mode as core::ffi::c_int == NEARMV as core::ffi::c_int
            && (*search_state)
                .frame_mv[NEARMV as core::ffi::c_int
                    as usize][LAST_FRAME as core::ffi::c_int as usize]
                .as_int == (*x).sb_me_mv.as_int
        {
            return false_0 != 0;
        }
        if *this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
            return false_0 != 0;
        }
    }
    if *is_single_pred != 0
        && (*search_state).mode_checked[*this_mode as usize][*ref_frame as usize]
            as core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if !check_globalmv && *this_mode as core::ffi::c_int == GLOBALMV as core::ffi::c_int
    {
        return true_0 != 0;
    }
    (*mi).mode = *this_mode;
    (*mi).ref_frame[0 as core::ffi::c_int as usize] = *ref_frame;
    (*mi).ref_frame[1 as core::ffi::c_int as usize] = *ref_frame2;
    if (*rt_sf).prune_compoundmode_with_singlemode_var as core::ffi::c_int != 0
        && *is_single_pred == 0 && is_one_pass_rt_lag_params(cpi) == 0
        && prune_compoundmode_with_singlemode_var(
            *this_mode,
            *ref_frame,
            *ref_frame2,
            ((*search_state).frame_mv).as_mut_ptr() as *const [int_mv; 8],
            ((*search_state).mode_checked).as_mut_ptr() as *const [uint8_t; 8],
            ((*search_state).vars).as_mut_ptr() as *const [core::ffi::c_uint; 8],
            ((*search_state).uv_dist).as_mut_ptr() as *const [int64_t; 8],
        ) as core::ffi::c_int != 0
    {
        return true_0 != 0;
    }
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*x).force_zeromv_skip_for_blk == 0
    {
        if (*rt_sf).source_metrics_sb_nonrd != 0 {
            if (*search_state).frame_mv[*this_mode as usize][*ref_frame as usize].as_int
                != 0 as uint32_t
                && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
                    == kZeroSad as core::ffi::c_int as core::ffi::c_uint
                || (*search_state)
                    .frame_mv[*this_mode as usize][*ref_frame as usize]
                    .as_int == 0 as uint32_t
                    && (*x).block_is_zero_sad == 0 as core::ffi::c_int
                    && *ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
                    && ((*x)
                        .color_sensitivity_sb[(1 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                        == 0 as core::ffi::c_int
                        && (*x)
                            .color_sensitivity_sb[(2 as core::ffi::c_int
                            - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                            == 0 as core::ffi::c_int || (*cpi).rc.high_source_sad != 0)
                    && (*x).source_variance == 0 as core::ffi::c_uint
            {
                return true_0 != 0;
            }
        }
        if (*rt_sf).skip_newmv_flat_blocks_screen != 0
            && *this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
            && (*x).source_variance < 100 as core::ffi::c_uint
        {
            return true_0 != 0;
        }
        if *ref_frame as core::ffi::c_int > LAST_FRAME as core::ffi::c_int
            && (*x).source_variance == 0 as core::ffi::c_uint
            && ((*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 1 as core::ffi::c_int
                || (*x)
                    .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int == 1 as core::ffi::c_int)
        {
            return true_0 != 0;
        }
    }
    if is_one_pass_rt_lag_params(cpi) != 0 {
        return false_0 != 0;
    }
    if skip_mode_by_bsize_and_ref_frame(
        *this_mode,
        *ref_frame,
        bsize,
        (*x).nonrd_prune_ref_frame_search,
        sse_zeromv_norm,
        (*rt_sf).nonrd_aggressive_skip,
        (*rt_sf).increase_source_sad_thresh,
    ) != 0
    {
        return true_0 != 0;
    }
    if skip_mode_by_low_temp(
        *this_mode,
        *ref_frame,
        bsize,
        (*x).content_state_sb,
        (*search_state).frame_mv[*this_mode as usize][*ref_frame as usize],
        force_skip_low_temp_var,
    ) != 0
    {
        return true_0 != 0;
    }
    if segfeature_active(
        seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) == 0
    {
        if (*rt_sf).nonrd_prune_ref_frame_search > 0 as core::ffi::c_int
            && (*x).pred_mv_sad[*ref_frame as usize] != INT_MAX
            && *ref_frame as core::ffi::c_int != LAST_FRAME as core::ffi::c_int
        {
            if (*x).pred_mv_sad[*ref_frame as usize] as int64_t > *thresh_sad_pred {
                return true_0 != 0;
            }
        }
    }
    if *this_mode as core::ffi::c_int == NEARMV as core::ffi::c_int
        && (*x).pred_mv1_sad[*ref_frame as usize] != INT_MAX
        && (*x).pred_mv1_sad[*ref_frame as usize]
            > (*x).pred_mv0_sad[*ref_frame as usize] << 1 as core::ffi::c_int
    {
        return true_0 != 0;
    }
    if *is_single_pred != 0 {
        if skip_mode_by_threshold(
            *this_mode,
            *ref_frame,
            (*search_state).frame_mv[*this_mode as usize][*ref_frame as usize],
            (*cpi).rc.frames_since_golden,
            ((*cpi).rd.threshes[segment_id as usize][bsize as usize]).as_mut_ptr(),
            ((*x).thresh_freq_fact[bsize as usize]).as_mut_ptr(),
            (*search_state).best_rdc.rdcost,
            (*search_state).best_pickmode.best_mode_skip_txfm as core::ffi::c_int,
            if (*rt_sf).nonrd_aggressive_skip != 0 {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            },
        ) != 0
        {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[inline(always)]
unsafe extern "C-unwind" fn handle_inter_mode_nonrd(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut search_state: *mut InterModeSearchStateNonrd,
    mut ctx: *mut PICK_MODE_CONTEXT,
    mut this_mode_pred: *mut *mut PRED_BUFFER,
    mut tmp_buffer: *mut PRED_BUFFER,
    mut inter_pred_params_sr: InterPredParams,
    mut best_early_term: *mut core::ffi::c_int,
    mut sse_zeromv_norm: *mut core::ffi::c_uint,
    mut check_globalmv: *mut bool,
    mut idx: core::ffi::c_int,
    mut force_mv_inter_layer: core::ffi::c_int,
    mut is_single_pred: core::ffi::c_int,
    mut gf_temporal_ref: core::ffi::c_int,
    mut use_model_yrd_large: core::ffi::c_int,
    mut filter_search_enabled_blk: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut this_mode: PREDICTION_MODE,
    mut filt_select: InterpFilter,
    mut cb_pred_filter_search: core::ffi::c_int,
    mut reuse_inter_pred: core::ffi::c_int,
    mut sb_me_has_been_tested: *mut core::ffi::c_int,
) -> bool {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mbmi_ext: *const MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let filter_ref: InterpFilter = (*cm).features.interp_filter;
    let default_interp_filter: InterpFilter = EIGHTTAP_REGULAR;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let rt_sf: *const REAL_TIME_SPEED_FEATURES = &raw mut (*cpi).sf.rt_sf;
    let best_pickmode: *mut BEST_PICKMODE = &raw mut (*search_state).best_pickmode;
    let mut ref_frame: MV_REFERENCE_FRAME = (*mi)
        .ref_frame[0 as core::ffi::c_int as usize];
    let mut ref_frame2: MV_REFERENCE_FRAME = (*mi)
        .ref_frame[1 as core::ffi::c_int as usize];
    let this_mv: *mut int_mv = &raw mut *(*((*search_state).frame_mv)
        .as_mut_ptr()
        .offset(this_mode as isize))
        .as_mut_ptr()
        .offset(ref_frame as isize) as *mut int_mv;
    let mut var: core::ffi::c_uint = UINT_MAX;
    let mut this_early_term: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut rate_mv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut is_skippable: core::ffi::c_int = 0;
    let mut skip_this_mv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut var_threshold: core::ffi::c_uint = UINT_MAX;
    let mut this_best_mode: PREDICTION_MODE = 0;
    let mut nonskip_rdc: RD_STATS = RD_STATS {
        rate: 0,
        zero_rate: 0,
        dist: 0,
        rdcost: 0,
        sse: 0,
        skip_txfm: 0,
    };
    av1_invalid_rd_stats(&raw mut nonskip_rdc);
    if (*x).sb_me_block != 0
        && this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        && ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
    {
        (*search_state)
            .frame_mv[NEWMV as core::ffi::c_int
                as usize][LAST_FRAME as core::ffi::c_int as usize]
            .as_int = (*x).sb_me_mv.as_int;
    } else if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        && force_mv_inter_layer == 0
    {
        let skip_newmv: bool = search_new_mv(
            cpi,
            x,
            ((*search_state).frame_mv).as_mut_ptr(),
            ref_frame,
            gf_temporal_ref,
            bsize,
            mi_row,
            mi_col,
            &raw mut rate_mv,
            &raw mut (*search_state).best_rdc,
        ) != 0;
        if skip_newmv {
            return true_0 != 0;
        }
    }
    let mut inter_mv_mode: PREDICTION_MODE = NEARESTMV as core::ffi::c_int
        as PREDICTION_MODE;
    while inter_mv_mode as core::ffi::c_int <= NEWMV as core::ffi::c_int {
        if !(inter_mv_mode as core::ffi::c_int == this_mode as core::ffi::c_int) {
            if is_single_pred != 0
                && (*search_state)
                    .mode_checked[inter_mv_mode as usize][ref_frame as usize]
                    as core::ffi::c_int != 0
                && (*this_mv).as_int
                    == (*search_state)
                        .frame_mv[inter_mv_mode as usize][ref_frame as usize]
                        .as_int
            {
                skip_this_mv = 1 as core::ffi::c_int;
                break;
            }
        }
        inter_mv_mode = inter_mv_mode.wrapping_add(1);
    }
    if skip_this_mv != 0 && is_single_pred != 0 {
        return true_0 != 0;
    }
    if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
        && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
            == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).svc.spatial_layer_id == 0 as core::ffi::c_int
        && (*rt_sf).source_metrics_sb_nonrd != 0
    {
        if (*this_mv).as_int == 0 as uint32_t
            && ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
            && (*x).block_is_zero_sad == 0 as core::ffi::c_int
            && ((*x)
                .color_sensitivity_sb[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int == 0 as core::ffi::c_int
                && (*x)
                    .color_sensitivity_sb[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int == 0 as core::ffi::c_int
                || (*cpi).rc.high_source_sad != 0)
            && (*x).source_variance == 0 as core::ffi::c_uint
        {
            return true_0 != 0;
        }
    }
    (*mi).mode = this_mode;
    (*mi).mv[0 as core::ffi::c_int as usize].as_int = (*this_mv).as_int;
    (*mi).mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
    if is_single_pred == 0 {
        (*mi).mv[1 as core::ffi::c_int as usize].as_int = (*search_state)
            .frame_mv[this_mode as usize][ref_frame2 as usize]
            .as_int;
    }
    if reuse_inter_pred != 0 {
        if (*this_mode_pred).is_null() {
            *this_mode_pred = &raw mut *tmp_buffer.offset(3 as core::ffi::c_int as isize)
                as *mut PRED_BUFFER;
        } else {
            *this_mode_pred = &raw mut *tmp_buffer
                .offset(
                    (get_pred_buffer
                        as unsafe extern "C-unwind" fn(
                            *mut PRED_BUFFER,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int)(tmp_buffer, 3 as core::ffi::c_int)
                        as isize,
                ) as *mut PRED_BUFFER;
            (*pd).dst.buf = (**this_mode_pred).data;
            (*pd).dst.stride = bw;
        }
    }
    (*mi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    if (*cpi).oxcf.motion_mode_cfg.allow_warped_motion {
        calc_num_proj_ref(cpi, x, mi);
    }
    if (*rt_sf).prune_compoundmode_with_singlecompound_var as core::ffi::c_int != 0
        && is_single_pred == 0 && use_model_yrd_large != 0
    {
        let single_mode0: PREDICTION_MODE = compound_ref0_mode(this_mode)
            as PREDICTION_MODE;
        let single_mode1: PREDICTION_MODE = compound_ref1_mode(this_mode)
            as PREDICTION_MODE;
        var_threshold = if var_threshold
            < (*search_state)
                .vars[(single_mode0 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                as usize][ref_frame as usize]
        {
            var_threshold
        } else {
            (*search_state)
                .vars[(single_mode0 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                as usize][ref_frame as usize]
        };
        var_threshold = if var_threshold
            < (*search_state)
                .vars[(single_mode1 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                as usize][ref_frame2 as usize]
        {
            var_threshold
        } else {
            (*search_state)
                .vars[(single_mode1 as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                as usize][ref_frame2 as usize]
        };
    }
    let is_mv_subpel: bool = (*mi).mv[0 as core::ffi::c_int as usize].as_mv.row
        as core::ffi::c_int & 0x7 as core::ffi::c_int != 0
        || (*mi).mv[0 as core::ffi::c_int as usize].as_mv.col as core::ffi::c_int
            & 0x7 as core::ffi::c_int != 0;
    let enable_filt_search_this_mode: bool = if filter_search_enabled_blk
        == 2 as core::ffi::c_int
    {
        true_0
    } else {
        (filter_search_enabled_blk != 0 && force_mv_inter_layer == 0
            && is_single_pred != 0
            && (ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
                || (*x).nonrd_prune_ref_frame_search == 0)) as core::ffi::c_int
    } != 0;
    if is_mv_subpel as core::ffi::c_int != 0
        && enable_filt_search_this_mode as core::ffi::c_int != 0
    {
        search_filter_ref(
            cpi,
            x,
            &raw mut (*search_state).this_rdc,
            &raw mut inter_pred_params_sr,
            mi_row,
            mi_col,
            tmp_buffer,
            bsize,
            reuse_inter_pred,
            this_mode_pred,
            &raw mut this_early_term,
            &raw mut var,
            use_model_yrd_large,
            (*best_pickmode).best_sse,
            is_single_pred,
        );
    } else if (*cpi).oxcf.motion_mode_cfg.allow_warped_motion as core::ffi::c_int != 0
        && this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int
    {
        search_motion_mode(
            cpi,
            x,
            &raw mut (*search_state).this_rdc,
            mi_row,
            mi_col,
            bsize,
            &raw mut this_early_term,
            use_model_yrd_large,
            &raw mut rate_mv,
            (*best_pickmode).best_sse,
        );
        if this_mode as core::ffi::c_int == NEWMV as core::ffi::c_int {
            *this_mv.offset(0 as core::ffi::c_int as isize) = (*mi)
                .mv[0 as core::ffi::c_int as usize];
        }
    } else {
        (*mi).interp_filters = if filter_ref as core::ffi::c_int
            == SWITCHABLE as core::ffi::c_int
        {
            av1_broadcast_interp_filter(default_interp_filter)
        } else {
            av1_broadcast_interp_filter(filter_ref)
        };
        if force_mv_inter_layer != 0 {
            (*mi).interp_filters = av1_broadcast_interp_filter(EIGHTTAP_REGULAR);
        }
        if is_mv_subpel as core::ffi::c_int != 0 && cb_pred_filter_search != 0 {
            (*mi).interp_filters = av1_broadcast_interp_filter(filt_select);
        }
        if is_single_pred != 0 {
            let mut subpel_params: SubpelParams = SubpelParams {
                xs: 0,
                ys: 0,
                subpel_x: 0,
                subpel_y: 0,
                pos_x: 0,
                pos_y: 0,
            };
            init_inter_mode_params(
                &raw mut (*((*mi).mv).as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                    .as_mv,
                &raw mut inter_pred_params_sr,
                &raw mut subpel_params,
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
                (*((*pd).pre).as_mut_ptr()).width,
                (*((*pd).pre).as_mut_ptr()).height,
            );
            av1_enc_build_inter_predictor_y_nonrd(
                xd,
                &raw mut inter_pred_params_sr,
                &raw mut subpel_params,
            );
        } else {
            av1_enc_build_inter_predictor(
                cm,
                xd,
                mi_row,
                mi_col,
                0 as *const BUFFER_SET,
                bsize,
                AOM_PLANE_Y,
                AOM_PLANE_Y,
            );
        }
        if use_model_yrd_large != 0 {
            model_skip_for_sb_y_large(
                cpi,
                bsize,
                mi_row,
                mi_col,
                x,
                xd,
                &raw mut (*search_state).this_rdc,
                &raw mut this_early_term,
                0 as core::ffi::c_int,
                (*best_pickmode).best_sse,
                &raw mut var,
                var_threshold,
            );
        } else {
            model_rd_for_sb_y(
                cpi,
                bsize,
                x,
                xd,
                &raw mut (*search_state).this_rdc,
                &raw mut var,
                0 as core::ffi::c_int,
                &raw mut this_early_term,
            );
        }
    }
    if is_single_pred != 0 {
        (*search_state)
            .vars[(this_mode as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
            as usize][ref_frame as usize] = var;
        if (*this_mv).as_int == 0 as uint32_t {
            (*search_state)
                .vars[(GLOBALMV as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
                as usize][ref_frame as usize] = var;
        }
    }
    if is_single_pred == 0 && var > var_threshold {
        if reuse_inter_pred != 0 {
            free_pred_buffer(*this_mode_pred);
        }
        return true_0 != 0;
    }
    if ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
        && (*this_mv).as_int == 0 as uint32_t
    {
        *sse_zeromv_norm = ((*search_state).this_rdc.sse
            >> b_width_log2_lookup[bsize as usize] as core::ffi::c_int
                + b_height_log2_lookup[bsize as usize] as core::ffi::c_int)
            as core::ffi::c_uint;
    }
    if (*rt_sf).sse_early_term_inter_search as core::ffi::c_uint != 0
        && early_term_inter_search_with_sse(
            (*rt_sf).sse_early_term_inter_search as core::ffi::c_int,
            bsize,
            (*search_state).this_rdc.sse,
            (*best_pickmode).best_sse,
            this_mode,
        ) != 0
    {
        if reuse_inter_pred != 0 {
            free_pred_buffer(*this_mode_pred);
        }
        return true_0 != 0;
    }
    let skip_ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
    let skip_txfm_cost: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][1 as core::ffi::c_int as usize];
    let no_skip_txfm_cost: core::ffi::c_int = (*mode_costs)
        .skip_txfm_cost[skip_ctx as usize][0 as core::ffi::c_int as usize];
    let sse_y: int64_t = (*search_state).this_rdc.sse;
    if this_early_term != 0 {
        (*search_state).this_rdc.skip_txfm = 1 as uint8_t;
        (*search_state).this_rdc.rate = skip_txfm_cost;
        (*search_state).this_rdc.dist = (*search_state).this_rdc.sse
            << 4 as core::ffi::c_int;
    } else {
        av1_block_yrd(
            x,
            &raw mut (*search_state).this_rdc,
            &raw mut is_skippable,
            bsize,
            (*mi).tx_size,
        );
        if (*search_state).this_rdc.skip_txfm as core::ffi::c_int != 0
            || ((*search_state).this_rdc.rate as int64_t * (*x).rdmult as int64_t
                + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                + (*search_state).this_rdc.dist
                    * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
                >= (0 as core::ffi::c_int as int64_t * (*x).rdmult as int64_t
                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as int64_t >> 9 as core::ffi::c_int)
                    + (*search_state).this_rdc.sse
                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t
        {
            if (*search_state).this_rdc.skip_txfm == 0 {
                nonskip_rdc = (*search_state).this_rdc;
                nonskip_rdc.rate += no_skip_txfm_cost;
            }
            (*search_state).this_rdc.rate = skip_txfm_cost;
            (*search_state).this_rdc.skip_txfm = 1 as uint8_t;
            (*search_state).this_rdc.dist = (*search_state).this_rdc.sse;
        } else {
            (*search_state).this_rdc.rate += no_skip_txfm_cost;
        }
        if (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 0
            || (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 0
        {
            let mut rdc_uv: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            let uv_bsize: BLOCK_SIZE = get_plane_block_size(
                bsize,
                (*xd).plane[AOM_PLANE_U as usize].subsampling_x,
                (*xd).plane[AOM_PLANE_U as usize].subsampling_y,
            ) as BLOCK_SIZE;
            if (*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] != 0
            {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    AOM_PLANE_U,
                    AOM_PLANE_U,
                );
            }
            if (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] != 0
            {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    AOM_PLANE_V,
                    AOM_PLANE_V,
                );
            }
            let sse_uv: int64_t = av1_model_rd_for_sb_uv(
                cpi,
                uv_bsize,
                x,
                xd,
                &raw mut rdc_uv,
                AOM_PLANE_U,
                AOM_PLANE_V,
            ) as int64_t;
            if rdc_uv.dist < (*x).min_dist_inter_uv {
                (*x).min_dist_inter_uv = rdc_uv.dist;
            }
            (*search_state).this_rdc.sse += sse_uv;
            if (*search_state).this_rdc.skip_txfm as core::ffi::c_int != 0
                && rdc_uv.skip_txfm == 0 && nonskip_rdc.rate != INT_MAX
            {
                (*search_state).this_rdc = nonskip_rdc;
            }
            if is_single_pred != 0 {
                (*search_state)
                    .uv_dist[(this_mode as core::ffi::c_int
                    - NEARESTMV as core::ffi::c_int) as usize][ref_frame as usize] = rdc_uv
                    .dist;
            }
            (*search_state).this_rdc.rate += rdc_uv.rate;
            (*search_state).this_rdc.dist += rdc_uv.dist;
            (*search_state).this_rdc.skip_txfm = ((*search_state).this_rdc.skip_txfm
                as core::ffi::c_int != 0 && rdc_uv.skip_txfm as core::ffi::c_int != 0)
                as core::ffi::c_int as uint8_t;
        }
    }
    this_best_mode = this_mode;
    (*search_state).this_rdc.rate += rate_mv;
    if is_single_pred == 0 {
        let mode_ctx: int16_t = av1_mode_context_analyzer(
            ((*mbmi_ext).mode_context).as_ptr(),
            ((*mi).ref_frame).as_mut_ptr(),
        ) as int16_t;
        (*search_state).this_rdc.rate += cost_mv_ref(mode_costs, this_mode, mode_ctx);
    } else {
        if this_mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int
            && (*this_mv).as_int
                == (*search_state)
                    .frame_mv[GLOBALMV as core::ffi::c_int as usize][ref_frame as usize]
                    .as_int
        {
            if is_globalmv_better(
                this_mode,
                ref_frame,
                rate_mv,
                mode_costs,
                ((*search_state).single_inter_mode_costs).as_mut_ptr()
                    as *const [core::ffi::c_int; 8],
                mbmi_ext,
            ) {
                this_best_mode = GLOBALMV as core::ffi::c_int as PREDICTION_MODE;
            }
        }
        (*search_state).this_rdc.rate
            += (*search_state)
                .single_inter_mode_costs[(this_best_mode as core::ffi::c_int
                - NEARESTMV as core::ffi::c_int) as usize][ref_frame as usize];
    }
    if is_single_pred != 0 && (*this_mv).as_int == 0 as uint32_t && var < UINT_MAX {
        (*search_state)
            .vars[(GLOBALMV as core::ffi::c_int - NEARESTMV as core::ffi::c_int)
            as usize][ref_frame as usize] = var;
    }
    (*search_state).this_rdc.rate = ((*search_state).this_rdc.rate as core::ffi::c_uint)
        .wrapping_add((*search_state).ref_costs_single[ref_frame as usize])
        as core::ffi::c_int as core::ffi::c_int;
    (*search_state).this_rdc.rdcost = ((*search_state).this_rdc.rate as int64_t
        * (*x).rdmult as int64_t
        + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 9 as core::ffi::c_int)
        + (*search_state).this_rdc.dist
            * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint && is_single_pred != 0
    {
        newmv_diff_bias(
            xd,
            this_best_mode,
            &raw mut (*search_state).this_rdc,
            bsize,
            (*search_state)
                .frame_mv[this_best_mode as usize][ref_frame as usize]
                .as_mv
                .row as core::ffi::c_int,
            (*search_state)
                .frame_mv[this_best_mode as usize][ref_frame as usize]
                .as_mv
                .col as core::ffi::c_int,
            (*cpi).speed,
            (*x).source_variance as uint32_t,
            (*x).content_state_sb,
        );
    }
    (*search_state).mode_checked[this_mode as usize][ref_frame as usize] = 1 as uint8_t;
    (*search_state).mode_checked[this_best_mode as usize][ref_frame as usize] = 1
        as uint8_t;
    if *check_globalmv {
        let mut abs_mv: int32_t = abs(
            (*search_state)
                .frame_mv[this_best_mode as usize][ref_frame as usize]
                .as_mv
                .row as core::ffi::c_int,
        ) as int32_t
            + abs(
                (*search_state)
                    .frame_mv[this_best_mode as usize][ref_frame as usize]
                    .as_mv
                    .col as core::ffi::c_int,
            ) as int32_t;
        if abs_mv < 2 as int32_t {
            *check_globalmv = false_0 != 0;
        }
    }
    if (*x).sb_me_block != 0
        && ref_frame as core::ffi::c_int == LAST_FRAME as core::ffi::c_int
        && (*search_state).frame_mv[this_best_mode as usize][ref_frame as usize].as_int
            == (*x).sb_me_mv.as_int
    {
        *sb_me_has_been_tested = 1 as core::ffi::c_int;
    }
    if (*search_state).this_rdc.rdcost < (*search_state).best_rdc.rdcost {
        (*search_state).best_rdc = (*search_state).this_rdc;
        *best_early_term = this_early_term;
        update_search_state_nonrd(
            search_state,
            mi,
            txfm_info,
            &raw mut nonskip_rdc,
            this_best_mode,
            sse_y,
        );
        (*search_state)
            .frame_mv_best[this_best_mode as usize][ref_frame as usize]
            .as_int = (*search_state)
            .frame_mv[this_best_mode as usize][ref_frame as usize]
            .as_int;
        if ref_frame2 as core::ffi::c_int > NONE_FRAME as core::ffi::c_int {
            (*search_state)
                .frame_mv_best[this_best_mode as usize][ref_frame2 as usize]
                .as_int = (*search_state)
                .frame_mv[this_best_mode as usize][ref_frame2 as usize]
                .as_int;
        }
        if reuse_inter_pred != 0 {
            free_pred_buffer((*best_pickmode).best_pred);
            (*best_pickmode).best_pred = *this_mode_pred;
        }
    } else if reuse_inter_pred != 0 {
        free_pred_buffer(*this_mode_pred);
    }
    if *best_early_term != 0
        && (idx > 0 as core::ffi::c_int || (*rt_sf).nonrd_aggressive_skip != 0)
    {
        (*txfm_info).skip_txfm = 1 as uint8_t;
        if (*x).sb_me_block == 0 || *sb_me_has_been_tested != 0 {
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[inline(always)]
unsafe extern "C-unwind" fn handle_screen_content_mode_nonrd(
    mut cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut search_state: *mut InterModeSearchStateNonrd,
    mut this_mode_pred: *mut PRED_BUFFER,
    mut ctx: *mut PICK_MODE_CONTEXT,
    mut tmp_buffer: *mut PRED_BUFFER,
    mut orig_dst: *mut buf_2d,
    mut skip_idtx_palette: core::ffi::c_int,
    mut try_palette: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut reuse_inter_pred: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let rt_sf: *const REAL_TIME_SPEED_FEATURES = &raw mut (*cpi).sf.rt_sf;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let best_pickmode: *mut BEST_PICKMODE = &raw mut (*search_state).best_pickmode;
    if (*(*cm).seq_params).bit_depth as core::ffi::c_uint == 8 as core::ffi::c_uint
        && (*rt_sf).use_idtx_nonrd != 0 && skip_idtx_palette == 0
        && !(*cpi).oxcf.txfm_cfg.use_inter_dct_only
        && (*x).force_zeromv_skip_for_blk == 0
        && is_inter_mode((*best_pickmode).best_mode) != 0
        && !((*best_pickmode).best_pred).is_null()
        && ((*rt_sf).prune_idtx_nonrd == 0
            || (*rt_sf).prune_idtx_nonrd != 0
                && bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int
                && (*best_pickmode).best_mode_skip_txfm as core::ffi::c_int
                    != 1 as core::ffi::c_int
                && (*x).source_variance > 200 as core::ffi::c_uint)
    {
        let mut idtx_rdc: RD_STATS = RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        };
        av1_init_rd_stats(&raw mut idtx_rdc);
        let mut is_skippable: core::ffi::c_int = 0;
        this_mode_pred = &raw mut *tmp_buffer
            .offset(
                (get_pred_buffer
                    as unsafe extern "C-unwind" fn(
                        *mut PRED_BUFFER,
                        core::ffi::c_int,
                    ) -> core::ffi::c_int)(tmp_buffer, 3 as core::ffi::c_int) as isize,
            ) as *mut PRED_BUFFER;
        (*pd).dst.buf = (*this_mode_pred).data;
        (*pd).dst.stride = bw;
        let best_pred: *const PRED_BUFFER = (*best_pickmode).best_pred;
        av1_block_yrd_idtx(
            x,
            (*best_pred).data,
            (*best_pred).stride,
            &raw mut idtx_rdc,
            &raw mut is_skippable,
            bsize,
            (*mi).tx_size,
        );
        let mut idx_rdcost_y: int64_t = (idtx_rdc.rate as int64_t
            * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + idtx_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        let mut allow_idtx: core::ffi::c_int = 1 as core::ffi::c_int;
        if (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 0
            || (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 0
        {
            let mut rdc_uv: RD_STATS = RD_STATS {
                rate: 0,
                zero_rate: 0,
                dist: 0,
                rdcost: 0,
                sse: 0,
                skip_txfm: 0,
            };
            let uv_bsize: BLOCK_SIZE = get_plane_block_size(
                bsize,
                (*xd).plane[AOM_PLANE_U as usize].subsampling_x,
                (*xd).plane[AOM_PLANE_U as usize].subsampling_y,
            ) as BLOCK_SIZE;
            if (*x)
                .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] != 0
            {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    AOM_PLANE_U,
                    AOM_PLANE_U,
                );
            }
            if (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] != 0
            {
                av1_enc_build_inter_predictor(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    0 as *const BUFFER_SET,
                    bsize,
                    AOM_PLANE_V,
                    AOM_PLANE_V,
                );
            }
            av1_model_rd_for_sb_uv(
                cpi,
                uv_bsize,
                x,
                xd,
                &raw mut rdc_uv,
                AOM_PLANE_U,
                AOM_PLANE_V,
            );
            if rdc_uv.dist < (*x).min_dist_inter_uv {
                (*x).min_dist_inter_uv = rdc_uv.dist;
            }
            idtx_rdc.rate += rdc_uv.rate;
            idtx_rdc.dist += rdc_uv.dist;
            idtx_rdc.skip_txfm = (idtx_rdc.skip_txfm as core::ffi::c_int != 0
                && rdc_uv.skip_txfm as core::ffi::c_int != 0) as core::ffi::c_int
                as uint8_t;
            if idx_rdcost_y == 0 as int64_t && rdc_uv.dist > 0 as int64_t
                && (*x).source_variance < 3000 as core::ffi::c_uint
                && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
                    > kMedSad as core::ffi::c_int as core::ffi::c_uint
            {
                allow_idtx = 0 as core::ffi::c_int;
            }
        }
        let mut idx_rdcost: int64_t = (idtx_rdc.rate as int64_t * (*x).rdmult as int64_t
            + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as int64_t >> 9 as core::ffi::c_int)
            + idtx_rdc.dist * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
        if allow_idtx != 0 && idx_rdcost < (*search_state).best_rdc.rdcost {
            (*best_pickmode).tx_type = IDTX as core::ffi::c_int as TX_TYPE;
            (*search_state).best_rdc.rdcost = idx_rdcost;
            (*best_pickmode).best_mode_skip_txfm = idtx_rdc.skip_txfm;
            *((*xd).tx_type_map).offset(0 as core::ffi::c_int as isize) = (*best_pickmode)
                .tx_type as uint8_t;
            memset(
                (*ctx).tx_type_map as *mut core::ffi::c_void,
                (*best_pickmode).tx_type as core::ffi::c_int,
                (*ctx).num_4x4_blk as size_t,
            );
            memset(
                (*xd).tx_type_map as *mut core::ffi::c_void,
                (*best_pickmode).tx_type as core::ffi::c_int,
                (*ctx).num_4x4_blk as size_t,
            );
        }
        (*pd).dst = *orig_dst;
    }
    if try_palette == 0 {
        return;
    }
    let intra_ref_frame_cost: core::ffi::c_uint = (*search_state)
        .ref_costs_single[INTRA_FRAME as core::ffi::c_int as usize];
    if is_mode_intra((*best_pickmode).best_mode) == 0 {
        let best_pred_0: *mut PRED_BUFFER = (*best_pickmode).best_pred;
        if reuse_inter_pred != 0 && !best_pred_0.is_null() {
            if (*best_pred_0).data == (*orig_dst).buf {
                this_mode_pred = &raw mut *tmp_buffer
                    .offset(
                        (get_pred_buffer
                            as unsafe extern "C-unwind" fn(
                                *mut PRED_BUFFER,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int)(tmp_buffer, 3 as core::ffi::c_int)
                            as isize,
                    ) as *mut PRED_BUFFER;
                aom_convolve_copy_c(
                    (*best_pred_0).data,
                    (*best_pred_0).stride as ptrdiff_t,
                    (*this_mode_pred).data,
                    (*this_mode_pred).stride as ptrdiff_t,
                    bw,
                    bh,
                );
                (*best_pickmode).best_pred = this_mode_pred;
            }
        }
        (*pd).dst = *orig_dst;
    }
    av1_search_palette_mode_luma(
        cpi,
        x,
        bsize,
        intra_ref_frame_cost,
        ctx,
        &raw mut (*search_state).this_rdc,
        (*search_state).best_rdc.rdcost,
    );
    if (*search_state).this_rdc.rdcost < (*search_state).best_rdc.rdcost {
        (*best_pickmode).pmi = (*mi).palette_mode_info;
        (*best_pickmode).best_mode = DC_PRED as core::ffi::c_int as PREDICTION_MODE;
        (*mi).mv[0 as core::ffi::c_int as usize].as_int = INVALID_MV as uint32_t;
        (*mi).mv[1 as core::ffi::c_int as usize].as_int = INVALID_MV as uint32_t;
        (*best_pickmode).best_ref_frame = INTRA_FRAME as core::ffi::c_int
            as MV_REFERENCE_FRAME;
        (*best_pickmode).best_second_ref_frame = NONE as core::ffi::c_int
            as MV_REFERENCE_FRAME;
        (*search_state).best_rdc.rate = (*search_state).this_rdc.rate;
        (*search_state).best_rdc.dist = (*search_state).this_rdc.dist;
        (*search_state).best_rdc.rdcost = (*search_state).this_rdc.rdcost;
        (*best_pickmode).best_mode_skip_txfm = (*search_state).this_rdc.skip_txfm;
        if (*x)
            .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 0
            || (*x)
                .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 0
        {
            (*search_state).this_rdc.skip_txfm = 0 as uint8_t;
        }
        if *((*xd).tx_type_map).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int != DCT_DCT as core::ffi::c_int
        {
            memcpy(
                (*ctx).tx_type_map as *mut core::ffi::c_void,
                (*xd).tx_type_map as *const core::ffi::c_void,
                ((*ctx).num_4x4_blk as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn enable_palette(
    mut cpi: *mut AV1_COMP,
    mut is_mode_intra_0: bool,
    mut bsize: BLOCK_SIZE,
    mut source_variance: core::ffi::c_uint,
    mut force_zeromv_skip: core::ffi::c_int,
    mut skip_idtx_palette: core::ffi::c_int,
    mut force_palette_test: core::ffi::c_int,
    mut best_intra_sad_norm: core::ffi::c_uint,
) -> bool {
    let sad_thresh: core::ffi::c_uint = (if (*cpi).sf.rt_sf.prune_palette_search_nonrd
        > 2 as core::ffi::c_int
    {
        if (*cpi).oxcf.frm_dim_cfg.width * (*cpi).oxcf.frm_dim_cfg.height
            <= 1280 as core::ffi::c_int * 720 as core::ffi::c_int
        {
            6 as core::ffi::c_int
        } else {
            12 as core::ffi::c_int
        }
    } else {
        10 as core::ffi::c_int
    }) as core::ffi::c_uint;
    if !(*cpi).oxcf.tool_cfg.enable_palette {
        return false_0 != 0;
    }
    if av1_allow_palette(
        (*cpi).common.features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) == 0
    {
        return false_0 != 0;
    }
    if skip_idtx_palette != 0 {
        return false_0 != 0;
    }
    if (*cpi).sf.rt_sf.prune_palette_search_nonrd > 1 as core::ffi::c_int
        && ((*cpi).rc.high_source_sad != 0
            && (*(*cpi).ppi).rtc_ref.non_reference_frame != 0
            || bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int)
    {
        return false_0 != 0;
    }
    if prune_palette_testing_inter(cpi, source_variance) as core::ffi::c_int != 0
        && best_intra_sad_norm < sad_thresh
    {
        return false_0 != 0;
    }
    if (is_mode_intra_0 as core::ffi::c_int != 0 || force_palette_test != 0)
        && source_variance > 0 as core::ffi::c_uint && force_zeromv_skip == 0
        && ((*cpi).rc.high_source_sad != 0 || source_variance > 300 as core::ffi::c_uint)
    {
        return true_0 != 0
    } else {
        return false_0 != 0
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nonrd_pick_inter_mode_sb(
    mut cpi: *mut AV1_COMP,
    mut tile_data: *mut TileDataEnc,
    mut x: *mut MACROBLOCK,
    mut rd_cost: *mut RD_STATS,
    mut bsize: BLOCK_SIZE,
    mut ctx: *mut PICK_MODE_CONTEXT,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let svc: *mut SVC = &raw mut (*cpi).svc;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let seg: *const segmentation = &raw mut (*cm).seg;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mbmi_ext: *const MB_MODE_INFO_EXT = &raw mut (*x).mbmi_ext;
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    let mut ref_frame2: MV_REFERENCE_FRAME = 0;
    let segment_id: core::ffi::c_uchar = (*mi).segment_id() as core::ffi::c_uchar;
    let mut best_early_term: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut force_skip_low_temp_var: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut sse_zeromv_norm: core::ffi::c_uint = UINT_MAX;
    let num_inter_modes: core::ffi::c_int = if is_one_pass_rt_lag_params(cpi) != 0 {
        NUM_INTER_MODES_FULL
    } else {
        NUM_INTER_MODES
    };
    let rt_sf: *const REAL_TIME_SPEED_FEATURES = &raw mut (*cpi).sf.rt_sf;
    let mut check_globalmv: bool = (*rt_sf).check_globalmv_on_single_ref;
    let mut tmp_buffer: [PRED_BUFFER; 4] = [PRED_BUFFER {
        data: 0 as *mut uint8_t,
        stride: 0,
        in_use: 0,
    }; 4];
    let mut pred_buf: crate::platform::Aligned16<[uint8_t; 49152]> = crate::platform::Aligned16([0; 49152]);
    let mut this_mode_pred: *mut PRED_BUFFER = 0 as *mut PRED_BUFFER;
    let reuse_inter_pred: core::ffi::c_int = ((*rt_sf).reuse_inter_pred_nonrd != 0
        && (*(*cm).seq_params).bit_depth as core::ffi::c_uint
            == AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int;
    let mut search_state: InterModeSearchStateNonrd = InterModeSearchStateNonrd {
        best_pickmode: BEST_PICKMODE {
            best_pred: 0 as *mut PRED_BUFFER,
            best_mode: 0,
            best_tx_size: 0,
            tx_type: 0,
            best_ref_frame: 0,
            best_second_ref_frame: 0,
            best_mode_skip_txfm: 0,
            best_mode_initial_skip_flag: 0,
            best_pred_filter: int_interpfilters { as_int: 0 },
            best_motion_mode: 0,
            wm_params: WarpedMotionParams {
                wmmat: [0; 6],
                alpha: 0,
                beta: 0,
                gamma: 0,
                delta: 0,
                wmtype: 0,
                invalid: 0,
            },
            num_proj_ref: 0,
            pmi: PALETTE_MODE_INFO {
                palette_colors: [0; 24],
                palette_size: [0; 2],
            },
            best_sse: 0,
        },
        this_rdc: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        best_rdc: RD_STATS {
            rate: 0,
            zero_rate: 0,
            dist: 0,
            rdcost: 0,
            sse: 0,
            skip_txfm: 0,
        },
        uv_dist: [[0; 8]; 4],
        yv12_mb: [[buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        }; 3]; 8],
        vars: [[0; 8]; 4],
        ref_costs_single: [0; 8],
        frame_mv: [[int_mv { as_int: 0 }; 8]; 25],
        frame_mv_best: [[int_mv { as_int: 0 }; 8]; 25],
        single_inter_mode_costs: [[0; 8]; 4],
        use_ref_frame_mask: [0; 8],
        mode_checked: [[0; 8]; 25],
        use_scaled_ref_frame: [false; 8],
    };
    memset(
        &raw mut search_state.use_ref_frame_mask as *mut [core::ffi::c_int; 8]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 8]>() as size_t,
    );
    memset(
        &raw mut search_state.use_scaled_ref_frame as *mut [bool; 8]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[bool; 8]>() as size_t,
    );
    let best_pickmode: *mut BEST_PICKMODE = &raw mut search_state.best_pickmode;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let pixels_in_block: core::ffi::c_int = bh * bw;
    let mut orig_dst: buf_2d = (*pd).dst;
    let mut txfm_params: *const TxfmSearchParams = &raw mut (*x).txfm_search_params;
    let mut txfm_info: *mut TxfmSearchInfo = &raw mut (*x).txfm_search_info;
    let mut thresh_sad_pred: int64_t = INT64_MAX as int64_t;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut svc_mv: int_mv = int_mv {
        as_int: 0 as core::ffi::c_int as uint32_t,
    };
    let mut force_mv_inter_layer: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut comp_use_zero_zeromv_only: bool = 0 as core::ffi::c_int != 0;
    let mut tot_num_comp_modes: core::ffi::c_int = if is_one_pass_rt_lag_params(cpi) != 0
    {
        NUM_COMP_INTER_MODES_RT_FULL
    } else {
        NUM_COMP_INTER_MODES_RT
    };
    let mut mode_costs: *const ModeCosts = &raw mut (*x).mode_costs;
    let mut sf_no_scale: scale_factors = scale_factors {
        x_scale_fp: 0,
        y_scale_fp: 0,
        x_step_q4: 0,
        y_step_q4: 0,
    };
    av1_setup_scale_factors_for_frame(
        &raw mut sf_no_scale,
        (*cm).width,
        (*cm).height,
        (*cm).width,
        (*cm).height,
    );
    if reuse_inter_pred != 0 {
        let mut buf_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while buf_idx < 3 as core::ffi::c_int {
            tmp_buffer[buf_idx as usize].data = &raw mut *pred_buf.0
                .as_mut_ptr()
                .offset((pixels_in_block * buf_idx) as isize) as *mut uint8_t;
            tmp_buffer[buf_idx as usize].stride = bw;
            tmp_buffer[buf_idx as usize].in_use = 0 as core::ffi::c_int;
            buf_idx += 1;
        }
        tmp_buffer[3 as core::ffi::c_int as usize].data = (*pd).dst.buf;
        tmp_buffer[3 as core::ffi::c_int as usize].stride = (*pd).dst.stride;
        tmp_buffer[3 as core::ffi::c_int as usize].in_use = 0 as core::ffi::c_int;
    }
    let gf_temporal_ref: core::ffi::c_int = is_same_gf_and_last_scale(cm)
        as core::ffi::c_int;
    if (*(*cpi).ppi).use_svc != 0 && (*svc).spatial_layer_id > 0 as core::ffi::c_int
        && (*svc).has_lower_quality_layer == 0
        && (*svc)
            .downsample_filter_phase[((*svc).spatial_layer_id - 1 as core::ffi::c_int)
            as usize] == 8 as core::ffi::c_int
        && (*cm).width * (*cm).height > 640 as core::ffi::c_int * 480 as core::ffi::c_int
    {
        svc_mv.as_mv.row = -(4 as core::ffi::c_int) as int16_t;
        svc_mv.as_mv.col = -(4 as core::ffi::c_int) as int16_t;
    }
    set_params_nonrd_pick_inter_mode(
        cpi,
        x,
        &raw mut search_state,
        rd_cost,
        &raw mut force_skip_low_temp_var,
        mi_row,
        mi_col,
        gf_temporal_ref,
        segment_id,
        bsize,
    );
    if (*rt_sf).use_comp_ref_nonrd != 0 && is_comp_ref_allowed(bsize) != 0 {
        if bsize as core::ffi::c_int > BLOCK_16X16 as core::ffi::c_int {
            comp_use_zero_zeromv_only = (*rt_sf).check_only_zero_zeromv_on_large_blocks;
        } else {
            tot_num_comp_modes = 0 as core::ffi::c_int;
        }
    } else {
        tot_num_comp_modes = 0 as core::ffi::c_int;
    }
    if segfeature_active(
        seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        tot_num_comp_modes = 0 as core::ffi::c_int;
    }
    if (*x).pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize] != INT_MAX {
        thresh_sad_pred = ((*x).pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize]
            as int64_t) << 1 as core::ffi::c_int;
        if (*rt_sf).nonrd_prune_ref_frame_search == 1 as core::ffi::c_int {
            thresh_sad_pred
                += ((*x).pred_mv_sad[LAST_FRAME as core::ffi::c_int as usize]
                    >> 2 as core::ffi::c_int) as int64_t;
        }
    }
    let use_model_yrd_large: core::ffi::c_int = get_model_rd_flag(cpi, xd, bsize)
        as core::ffi::c_int;
    let mut filt_select: InterpFilter = EIGHTTAP_REGULAR;
    let cb_pred_filter_search: core::ffi::c_int = if (*x)
        .content_state_sb
        .source_sad_nonrd as core::ffi::c_uint
        > kVeryLowSad as core::ffi::c_int as core::ffi::c_uint
    {
        (*cpi).sf.interp_sf.cb_pred_filter_search
    } else {
        0 as core::ffi::c_int
    };
    let filter_search_enabled_blk: core::ffi::c_int = is_filter_search_enabled_blk(
        cpi,
        x,
        mi_row,
        mi_col,
        bsize,
        segment_id as core::ffi::c_int,
        cb_pred_filter_search,
        &raw mut filt_select,
    ) as core::ffi::c_int;
    init_mbmi_nonrd(
        mi,
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        cm,
    );
    (*mi).tx_size = (if (if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
        < tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
            as core::ffi::c_int
    {
        max_txsize_lookup[bsize as usize] as core::ffi::c_int
    } else {
        tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
            as core::ffi::c_int
    }) < TX_16X16 as core::ffi::c_int
    {
        if (max_txsize_lookup[bsize as usize] as core::ffi::c_int)
            < tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                as core::ffi::c_int
        {
            max_txsize_lookup[bsize as usize] as core::ffi::c_int
        } else {
            tx_mode_to_biggest_tx_size[(*txfm_params).tx_mode_search_type as usize]
                as core::ffi::c_int
        }
    } else {
        TX_16X16 as core::ffi::c_int
    }) as TX_SIZE;
    if is_one_pass_rt_lag_params(cpi) == 0 {
        fill_single_inter_mode_costs(
            (search_state.single_inter_mode_costs).as_mut_ptr()
                as *mut [core::ffi::c_int; 8],
            num_inter_modes,
            core::ptr::addr_of!(ref_mode_set).cast::<REF_MODE>(),
            mode_costs,
            ((*mbmi_ext).mode_context).as_ptr(),
        );
    } else {
        fill_single_inter_mode_costs(
            (search_state.single_inter_mode_costs).as_mut_ptr()
                as *mut [core::ffi::c_int; 8],
            num_inter_modes,
            core::ptr::addr_of!(ref_mode_set_full).cast::<REF_MODE>(),
            mode_costs,
            ((*mbmi_ext).mode_context).as_ptr(),
        );
    }
    let mut last_comp_ref_frame: MV_REFERENCE_FRAME = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    let mut inter_pred_params_sr: InterPredParams = InterPredParams {
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
    init_inter_block_params(
        &raw mut inter_pred_params_sr,
        (*pd).width as core::ffi::c_int,
        (*pd).height as core::ffi::c_int,
        mi_row * MI_SIZE,
        mi_col * MI_SIZE,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
        (*xd).bd,
        is_cur_buf_hbd(xd),
        0 as core::ffi::c_int,
    );
    inter_pred_params_sr.conv_params = get_conv_params(
        0 as core::ffi::c_int,
        AOM_PLANE_Y,
        (*xd).bd,
    );
    (*x).block_is_zero_sad = ((*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
        == kZeroSad as core::ffi::c_int as core::ffi::c_uint
        || segfeature_active(
            &raw mut (*cm).seg,
            segment_id as uint8_t,
            SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0) as core::ffi::c_int;
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*x).force_zeromv_skip_for_blk == 0
        && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            != kZeroSad as core::ffi::c_int as core::ffi::c_uint
        && (*x).source_variance == 0 as core::ffi::c_uint
        && (bsize as core::ffi::c_int) < (*(*cm).seq_params).sb_size as core::ffi::c_int
        && search_state.use_ref_frame_mask[LAST_FRAME as core::ffi::c_int as usize] != 0
        && search_state
            .yv12_mb[LAST_FRAME as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize]
            .width == (*cm).width
        && search_state
            .yv12_mb[LAST_FRAME as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize]
            .height == (*cm).height
    {
        set_block_source_sad(
            cpi,
            x,
            bsize,
            &raw mut *(*(search_state.yv12_mb)
                .as_mut_ptr()
                .offset(LAST_FRAME as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
        );
    }
    let mut sb_me_has_been_tested: core::ffi::c_int = 0 as core::ffi::c_int;
    (*x).sb_me_block = (*x).sb_me_partition;
    if (*x).sb_me_block != 0 {
        if (*(*cm).seq_params).sb_size as core::ffi::c_int
            == BLOCK_128X128 as core::ffi::c_int
            && (bsize as core::ffi::c_int) < BLOCK_64X64 as core::ffi::c_int
        {
            (*x).sb_me_block = 0 as core::ffi::c_int;
        } else if (*(*cm).seq_params).sb_size as core::ffi::c_int
            == BLOCK_64X64 as core::ffi::c_int
            && (bsize as core::ffi::c_int) < BLOCK_32X32 as core::ffi::c_int
        {
            (*x).sb_me_block = 0 as core::ffi::c_int;
        }
    }
    (*x).min_dist_inter_uv = INT64_MAX as int64_t;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < num_inter_modes + tot_num_comp_modes {
        if (*rt_sf).skip_compound_based_on_var as core::ffi::c_int != 0
            && idx == num_inter_modes
            && skip_comp_based_on_var(
                (search_state.vars).as_mut_ptr() as *const [core::ffi::c_uint; 8],
                bsize,
            ) as core::ffi::c_int != 0
        {
            break;
        }
        let mut is_single_pred: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut this_mode: PREDICTION_MODE = 0;
        if idx == 0 as core::ffi::c_int && (*x).force_zeromv_skip_for_blk == 0 {
            if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
                == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
                && search_state
                    .use_ref_frame_mask[GOLDEN_FRAME as core::ffi::c_int as usize] != 0
            {
                if (*x)
                    .color_sensitivity_sb_g[(1 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                    == 1 as core::ffi::c_int
                {
                    (*x)
                        .color_sensitivity[(1 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] = 1 as uint8_t;
                }
                if (*x)
                    .color_sensitivity_sb_g[(2 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize] as core::ffi::c_int
                    == 1 as core::ffi::c_int
                {
                    (*x)
                        .color_sensitivity[(2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] = 1 as uint8_t;
                }
            }
            if search_state.use_ref_frame_mask[LAST_FRAME as core::ffi::c_int as usize]
                != 0
                && (*x).pred_mv0_sad[LAST_FRAME as core::ffi::c_int as usize] != INT_MAX
            {
                let mut y_sad: core::ffi::c_int = (*x)
                    .pred_mv0_sad[LAST_FRAME as core::ffi::c_int as usize];
                if (*x).pred_mv1_sad[LAST_FRAME as core::ffi::c_int as usize] != INT_MAX
                    && abs(
                        search_state
                            .frame_mv[NEARMV as core::ffi::c_int
                                as usize][LAST_FRAME as core::ffi::c_int as usize]
                            .as_mv
                            .col as core::ffi::c_int,
                    )
                        + abs(
                            search_state
                                .frame_mv[NEARMV as core::ffi::c_int
                                    as usize][LAST_FRAME as core::ffi::c_int as usize]
                                .as_mv
                                .row as core::ffi::c_int,
                        )
                        < abs(
                            search_state
                                .frame_mv[NEARESTMV as core::ffi::c_int
                                    as usize][LAST_FRAME as core::ffi::c_int as usize]
                                .as_mv
                                .col as core::ffi::c_int,
                        )
                            + abs(
                                search_state
                                    .frame_mv[NEARESTMV as core::ffi::c_int
                                        as usize][LAST_FRAME as core::ffi::c_int as usize]
                                    .as_mv
                                    .row as core::ffi::c_int,
                            )
                {
                    y_sad = (*x).pred_mv1_sad[LAST_FRAME as core::ffi::c_int as usize];
                }
                set_color_sensitivity(
                    cpi,
                    x,
                    bsize,
                    y_sad,
                    (*x).source_variance,
                    (search_state.yv12_mb[LAST_FRAME as core::ffi::c_int as usize])
                        .as_mut_ptr(),
                );
            }
        }
        if !skip_inter_mode_nonrd(
            cpi,
            x,
            &raw mut search_state,
            &raw mut thresh_sad_pred,
            &raw mut force_mv_inter_layer,
            &raw mut is_single_pred,
            &raw mut this_mode,
            &raw mut last_comp_ref_frame,
            &raw mut ref_frame,
            &raw mut ref_frame2,
            idx,
            svc_mv,
            force_skip_low_temp_var,
            sse_zeromv_norm,
            num_inter_modes,
            segment_id,
            bsize,
            comp_use_zero_zeromv_only,
            check_globalmv,
        ) {
            let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane < MAX_MB_PLANE {
                (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize] = search_state
                    .yv12_mb[ref_frame as usize][plane as usize];
                if is_single_pred == 0 {
                    (*xd).plane[plane as usize].pre[1 as core::ffi::c_int as usize] = search_state
                        .yv12_mb[ref_frame2 as usize][plane as usize];
                }
                plane += 1;
            }
            (*mi).ref_frame[0 as core::ffi::c_int as usize] = ref_frame;
            (*mi).ref_frame[1 as core::ffi::c_int as usize] = ref_frame2;
            set_ref_ptrs(cm, xd, ref_frame, ref_frame2);
            if search_state.use_scaled_ref_frame[ref_frame as usize] {
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = &raw mut sf_no_scale;
            }
            if is_single_pred == 0
                && search_state.use_scaled_ref_frame[ref_frame2 as usize]
                    as core::ffi::c_int != 0
            {
                (*xd).block_ref_scale_factors[1 as core::ffi::c_int as usize] = &raw mut sf_no_scale;
            }
            if !handle_inter_mode_nonrd(
                cpi,
                x,
                &raw mut search_state,
                ctx,
                &raw mut this_mode_pred,
                tmp_buffer.as_mut_ptr(),
                inter_pred_params_sr,
                &raw mut best_early_term,
                &raw mut sse_zeromv_norm,
                &raw mut check_globalmv,
                idx,
                force_mv_inter_layer,
                is_single_pred,
                gf_temporal_ref,
                use_model_yrd_large,
                filter_search_enabled_blk,
                bsize,
                this_mode,
                filt_select,
                cb_pred_filter_search,
                reuse_inter_pred,
                &raw mut sb_me_has_been_tested,
            ) {
                break;
            }
        }
        idx += 1;
    }
    (*mi).mode = (*best_pickmode).best_mode;
    (*mi).motion_mode = (*best_pickmode).best_motion_mode;
    (*mi).wm_params = (*best_pickmode).wm_params;
    (*mi).num_proj_ref = (*best_pickmode).num_proj_ref as uint8_t;
    (*mi).interp_filters = (*best_pickmode).best_pred_filter;
    (*mi).tx_size = (*best_pickmode).best_tx_size;
    memset(
        ((*mi).inter_tx_size).as_mut_ptr() as *mut core::ffi::c_void,
        (*mi).tx_size as core::ffi::c_int,
        ::core::mem::size_of::<[TX_SIZE; 16]>() as size_t,
    );
    (*mi).ref_frame[0 as core::ffi::c_int as usize] = (*best_pickmode).best_ref_frame;
    (*mi).mv[0 as core::ffi::c_int as usize].as_int = search_state
        .frame_mv_best[(*best_pickmode).best_mode
            as usize][(*best_pickmode).best_ref_frame as usize]
        .as_int;
    (*mi).mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
    if (*best_pickmode).best_second_ref_frame as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
    {
        (*mi).ref_frame[1 as core::ffi::c_int as usize] = (*best_pickmode)
            .best_second_ref_frame;
        (*mi).mv[1 as core::ffi::c_int as usize].as_int = search_state
            .frame_mv_best[(*best_pickmode).best_mode
                as usize][(*best_pickmode).best_second_ref_frame as usize]
            .as_int;
    }
    (*mi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize] = 0 as int8_t;
    (*mi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize] = 0 as int8_t;
    (*mi).filter_intra_mode_info.use_filter_intra = 0 as uint8_t;
    let mut force_palette_test: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            != kZeroSad as core::ffi::c_int as core::ffi::c_uint
        && bsize as core::ffi::c_int <= BLOCK_16X16 as core::ffi::c_int
    {
        let mut thresh_sse: core::ffi::c_uint = (if (*cpi).rc.high_source_sad != 0 {
            15000 as core::ffi::c_int
        } else {
            100000 as core::ffi::c_int
        }) as core::ffi::c_uint;
        let mut thresh_source_var: core::ffi::c_uint = (if (*cpi).rc.high_source_sad != 0
        {
            50 as core::ffi::c_int
        } else {
            200 as core::ffi::c_int
        }) as core::ffi::c_uint;
        let mut best_sse_inter_motion: core::ffi::c_uint = (search_state.best_rdc.sse
            >> b_width_log2_lookup[bsize as usize] as core::ffi::c_int
                + b_height_log2_lookup[bsize as usize] as core::ffi::c_int)
            as core::ffi::c_uint;
        if best_sse_inter_motion > thresh_sse && (*x).source_variance > thresh_source_var
        {
            force_palette_test = 1 as core::ffi::c_int;
        }
    }
    let inter_forced_on_segment: core::ffi::c_int = (segfeature_active(
        seg,
        segment_id as uint8_t,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        && get_segdata(
            seg,
            segment_id as core::ffi::c_int,
            SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
    let mut best_intra_sad_norm: core::ffi::c_uint = UINT_MAX;
    if (*x).force_zeromv_skip_for_blk == 0 && inter_forced_on_segment == 0 {
        av1_estimate_intra_mode(
            cpi,
            x,
            bsize,
            best_early_term,
            search_state.ref_costs_single[INTRA_FRAME as core::ffi::c_int as usize],
            reuse_inter_pred,
            &raw mut orig_dst,
            tmp_buffer.as_mut_ptr(),
            &raw mut this_mode_pred,
            &raw mut search_state.best_rdc,
            best_pickmode,
            &raw mut best_intra_sad_norm,
        );
    }
    let mut skip_idtx_palette: core::ffi::c_int = (((*x)
        .color_sensitivity[(1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
        as core::ffi::c_int != 0
        || (*x)
            .color_sensitivity[(2 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int != 0)
        && (*x).content_state_sb.source_sad_nonrd as core::ffi::c_uint
            != kZeroSad as core::ffi::c_int as core::ffi::c_uint
        && (*cpi).rc.high_source_sad == 0
        && ((*cpi).rc.high_motion_content_screen_rtc != 0
            || (*cpi).rc.frame_source_sad < 1000 as uint64_t)) as core::ffi::c_int;
    let mut try_palette: bool = enable_palette(
        cpi,
        is_mode_intra((*best_pickmode).best_mode) != 0,
        bsize,
        (*x).source_variance,
        (*x).force_zeromv_skip_for_blk,
        skip_idtx_palette,
        force_palette_test,
        best_intra_sad_norm,
    );
    if try_palette as core::ffi::c_int != 0
        && prune_palette_testing_inter(cpi, (*x).source_variance) as core::ffi::c_int
            != 0
    {
        (*x).color_palette_thresh = if (*cpi).sf.rt_sf.prune_palette_search_nonrd
            > 2 as core::ffi::c_int
        {
            20 as core::ffi::c_int
        } else {
            32 as core::ffi::c_int
        };
    }
    if inter_forced_on_segment == 0 {
        handle_screen_content_mode_nonrd(
            cpi,
            x,
            &raw mut search_state,
            this_mode_pred,
            ctx,
            tmp_buffer.as_mut_ptr(),
            &raw mut orig_dst,
            skip_idtx_palette,
            try_palette as core::ffi::c_int,
            bsize,
            reuse_inter_pred,
            mi_col,
            mi_row,
        );
    }
    (*pd).dst = orig_dst;
    if try_palette {
        (*mi).palette_mode_info = (*best_pickmode).pmi;
    }
    (*mi).mode = (*best_pickmode).best_mode;
    (*mi).ref_frame[0 as core::ffi::c_int as usize] = (*best_pickmode).best_ref_frame;
    (*mi).ref_frame[1 as core::ffi::c_int as usize] = (*best_pickmode)
        .best_second_ref_frame;
    if is_lossless_requested(&raw mut (*cpi).oxcf.rc_cfg) != 0 {
        (*txfm_info).skip_txfm = 0 as uint8_t;
    } else {
        (*txfm_info).skip_txfm = (*best_pickmode).best_mode_skip_txfm;
    }
    if has_second_ref(mi) != 0 {
        (*mi).set_comp_group_idx(0 as uint8_t as uint8_t);
        (*mi).set_compound_idx(1 as uint8_t as uint8_t);
        (*mi).interinter_comp.type_0 = COMPOUND_AVERAGE as core::ffi::c_int
            as COMPOUND_TYPE;
    }
    if is_inter_block(mi) == 0 {
        (*mi).interp_filters = av1_broadcast_interp_filter(SWITCHABLE_FILTERS);
    } else if search_state.use_scaled_ref_frame[(*best_pickmode).best_ref_frame as usize]
        as core::ffi::c_int != 0
        || has_second_ref(mi) != 0
            && search_state
                .use_scaled_ref_frame[(*best_pickmode).best_second_ref_frame as usize]
                as core::ffi::c_int != 0
    {
        (*x).reuse_inter_pred = 0 as core::ffi::c_int != 0;
    }
    if reuse_inter_pred != 0 && !((*best_pickmode).best_pred).is_null() {
        let best_pred: *mut PRED_BUFFER = (*best_pickmode).best_pred;
        if (*best_pred).data != orig_dst.buf && is_inter_mode((*mi).mode) != 0 {
            aom_convolve_copy_c(
                (*best_pred).data,
                (*best_pred).stride as ptrdiff_t,
                (*pd).dst.buf,
                (*pd).dst.stride as ptrdiff_t,
                bw,
                bh,
            );
        }
    }
    if (*cpi).sf.inter_sf.adaptive_rd_thresh != 0 && has_second_ref(mi) == 0 {
        let mut best_mode_idx: THR_MODES = mode_idx[(*best_pickmode).best_ref_frame
            as usize][mode_offset((*mi).mode) as usize];
        if (*best_pickmode).best_ref_frame as core::ffi::c_int
            == INTRA_FRAME as core::ffi::c_int
        {
            let mut intra_modes: core::ffi::c_int = (::core::mem::size_of::<
                [PREDICTION_MODE; 4],
            >() as usize)
                .wrapping_div(::core::mem::size_of::<PREDICTION_MODE>() as usize)
                as core::ffi::c_int;
            let mut mode_index: core::ffi::c_int = 0 as core::ffi::c_int;
            while mode_index < intra_modes {
                update_thresh_freq_fact(
                    cpi,
                    x,
                    bsize,
                    INTRA_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                    best_mode_idx,
                    intra_mode_list[mode_index as usize],
                );
                mode_index += 1;
            }
        } else {
            let mut this_mode_0: PREDICTION_MODE = 0;
            this_mode_0 = NEARESTMV as core::ffi::c_int as PREDICTION_MODE;
            while this_mode_0 as core::ffi::c_int <= NEWMV as core::ffi::c_int {
                update_thresh_freq_fact(
                    cpi,
                    x,
                    bsize,
                    (*best_pickmode).best_ref_frame,
                    best_mode_idx,
                    this_mode_0,
                );
                this_mode_0 = this_mode_0.wrapping_add(1);
            }
        }
    }
    store_coding_context_nonrd(x, ctx);
    *rd_cost = search_state.best_rdc;
    (*mi).num_proj_ref = 0 as uint8_t;
    if warped_motion_update_num_proj_ref(cpi, mi) != 0 {
        let mut pts: [core::ffi::c_int; 16] = [0; 16];
        let mut pts_inref: [core::ffi::c_int; 16] = [0; 16];
        (*mi).num_proj_ref = av1_findSamples(
            cm,
            xd,
            pts.as_mut_ptr(),
            pts_inref.as_mut_ptr(),
        );
    }
    set_ref_ptrs(
        cm,
        xd,
        (*mi).ref_frame[0 as core::ffi::c_int as usize],
        (*mi).ref_frame[1 as core::ffi::c_int as usize],
    );
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);

extern "C" {
    fn log2(_: core::ffi::c_double) -> core::ffi::c_double;
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
}
