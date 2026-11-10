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

// Mechanically translated from upstream/aom/av1/encoder/motion_search_facade.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    fn av1_setup_scale_factors_for_frame(
        sf: *mut scale_factors,
        other_w: core::ffi::c_int,
        other_h: core::ffi::c_int,
        this_w: core::ffi::c_int,
        this_h: core::ffi::c_int,
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
    fn av1_get_compound_type_mask(
        comp_data: *const INTERINTER_COMPOUND_DATA,
        sb_type: BLOCK_SIZE,
    ) -> *const uint8_t;
    fn av1_get_scaled_ref_frame(
        cpi: *const AV1_COMP,
        ref_frame: core::ffi::c_int,
    ) -> *mut YV12_BUFFER_CONFIG;
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
    static av1_init_motion_compensation: [av1_init_search_site_config; 6];
    fn av1_init_search_range(size: core::ffi::c_int) -> core::ffi::c_int;
    fn av1_refining_search_8p_c(
        ms_params: *const FULLPEL_MOTION_SEARCH_PARAMS,
        start_mv: FULLPEL_MV,
        best_mv: *mut FULLPEL_MV,
    ) -> core::ffi::c_int;
    fn av1_full_pixel_search(
        start_mv: FULLPEL_MV,
        ms_params: *const FULLPEL_MOTION_SEARCH_PARAMS,
        step_param: core::ffi::c_int,
        cost_list: *mut core::ffi::c_int,
        best_mv: *mut FULLPEL_MV,
        best_mv_stats: *mut FULLPEL_MV_STATS,
        second_best_mv: *mut FULLPEL_MV,
    ) -> core::ffi::c_int;
    fn av1_obmc_full_pixel_search(
        start_mv: FULLPEL_MV,
        ms_params: *const FULLPEL_MOTION_SEARCH_PARAMS,
        step_param: core::ffi::c_int,
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
    fn av1_find_best_obmc_sub_pixel_tree_up(
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
    fn av1_get_ref_mv(x: *const MACROBLOCK, ref_idx: core::ffi::c_int) -> int_mv;
    fn av1_get_ref_mv_from_stack(
        ref_idx: core::ffi::c_int,
        ref_frame: *const MV_REFERENCE_FRAME,
        ref_mv_idx: core::ffi::c_int,
        mbmi_ext: *const MB_MODE_INFO_EXT,
    ) -> int_mv;
    fn av1_setup_src_planes(
        x: *mut macroblock,
        src: *const yv12_buffer_config,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        num_planes: core::ffi::c_int,
        bsize: BLOCK_SIZE,
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
    fn av1_enc_build_one_inter_predictor(
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        src_mv: *const MV,
        inter_pred_params: *mut InterPredParams,
    );
    fn av1_estimate_txfm_yrd(
        cpi: *const AV1_COMP,
        x: *mut MACROBLOCK,
        rd_stats: *mut RD_STATS,
        ref_best_rd: int64_t,
        bs: BLOCK_SIZE,
        tx_size: TX_SIZE,
    ) -> int64_t;
}
pub type size_t = usize;
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
pub type uintptr_t = usize;
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
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_2 = 25;
pub const INTRA_MODES: C2RustUnnamed_2 = 13;
pub const INTER_MODE_END: C2RustUnnamed_2 = 25;
pub const INTER_MODE_START: C2RustUnnamed_2 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_2 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_2 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_2 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_2 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_2 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_2 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_2 = 13;
pub const DIR_MODE_END: C2RustUnnamed_2 = 9;
pub const DIR_MODE_START: C2RustUnnamed_2 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_2 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_2 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_2 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_2 = 25;
pub const NEW_NEWMV: C2RustUnnamed_2 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_2 = 23;
pub const NEW_NEARMV: C2RustUnnamed_2 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_2 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_2 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_2 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_2 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_2 = 17;
pub const NEWMV: C2RustUnnamed_2 = 16;
pub const GLOBALMV: C2RustUnnamed_2 = 15;
pub const NEARMV: C2RustUnnamed_2 = 14;
pub const NEARESTMV: C2RustUnnamed_2 = 13;
pub const PAETH_PRED: C2RustUnnamed_2 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_2 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_2 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_2 = 9;
pub const D67_PRED: C2RustUnnamed_2 = 8;
pub const D203_PRED: C2RustUnnamed_2 = 7;
pub const D157_PRED: C2RustUnnamed_2 = 6;
pub const D113_PRED: C2RustUnnamed_2 = 5;
pub const D135_PRED: C2RustUnnamed_2 = 4;
pub const D45_PRED: C2RustUnnamed_2 = 3;
pub const H_PRED: C2RustUnnamed_2 = 2;
pub const V_PRED: C2RustUnnamed_2 = 1;
pub const DC_PRED: C2RustUnnamed_2 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_3 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_3 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_3 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_3 = 0;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_4 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_4 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_4 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_4 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_4 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_4 = 0;
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
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_18 = 4;
pub const S_FRAME: C2RustUnnamed_18 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_18 = 2;
pub const INTER_FRAME: C2RustUnnamed_18 = 1;
pub const KEY_FRAME: C2RustUnnamed_18 = 0;
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
pub type MODE = uint8_t;
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const NUM_DISTINCT_SEARCH_METHODS: C2RustUnnamed_19 = 6;
pub const NUM_SEARCH_METHODS: C2RustUnnamed_19 = 9;
pub const VFAST_DIAMOND: C2RustUnnamed_19 = 8;
pub const FAST_BIGDIA: C2RustUnnamed_19 = 7;
pub const FAST_DIAMOND: C2RustUnnamed_19 = 6;
pub const BIGDIA: C2RustUnnamed_19 = 5;
pub const HEX: C2RustUnnamed_19 = 4;
pub const CLAMPED_DIAMOND: C2RustUnnamed_19 = 3;
pub const NSTEP_8PT: C2RustUnnamed_19 = 2;
pub const NSTEP: C2RustUnnamed_19 = 1;
pub const DIAMOND: C2RustUnnamed_19 = 0;
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
pub type C2RustUnnamed_20 = core::ffi::c_uint;
pub const FRAME_UPDATE_TYPES: C2RustUnnamed_20 = 7;
pub const INTNL_ARF_UPDATE: C2RustUnnamed_20 = 6;
pub const INTNL_OVERLAY_UPDATE: C2RustUnnamed_20 = 5;
pub const OVERLAY_UPDATE: C2RustUnnamed_20 = 4;
pub const ARF_UPDATE: C2RustUnnamed_20 = 3;
pub const GF_UPDATE: C2RustUnnamed_20 = 2;
pub const LF_UPDATE: C2RustUnnamed_20 = 1;
pub const KF_UPDATE: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = core::ffi::c_uint;
pub const MV_COST_NONE: C2RustUnnamed_21 = 4;
pub const MV_COST_L1_HDRES: C2RustUnnamed_21 = 3;
pub const MV_COST_L1_MIDRES: C2RustUnnamed_21 = 2;
pub const MV_COST_L1_LOWRES: C2RustUnnamed_21 = 1;
pub const MV_COST_ENTROPY: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const FULL_PEL: C2RustUnnamed_22 = 3;
pub const HALF_PEL: C2RustUnnamed_22 = 2;
pub const QUARTER_PEL: C2RustUnnamed_22 = 1;
pub const EIGHTH_PEL: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const SUBPEL_SEARCH_METHODS: C2RustUnnamed_23 = 3;
pub const SUBPEL_TREE_PRUNED_MORE: C2RustUnnamed_23 = 2;
pub const SUBPEL_TREE_PRUNED: C2RustUnnamed_23 = 1;
pub const SUBPEL_TREE: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const SS_CFG_TOTAL: C2RustUnnamed_24 = 3;
pub const SS_CFG_FPF: C2RustUnnamed_24 = 2;
pub const SS_CFG_LOOKAHEAD: C2RustUnnamed_24 = 1;
pub const SS_CFG_SRC: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERPOLATION_FILTER_STATS {
    pub filters: int_interpfilters,
    pub mv: [int_mv; 2],
    pub ref_frames: [int8_t; 2],
    pub comp_type: COMPOUND_TYPE,
    pub compound_idx: core::ffi::c_int,
    pub rd: int64_t,
    pub pred_sse: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HandleInterModeArgs {
    pub above_pred_buf: [*mut uint8_t; 3],
    pub above_pred_stride: [core::ffi::c_int; 3],
    pub left_pred_buf: [*mut uint8_t; 3],
    pub left_pred_stride: [core::ffi::c_int; 3],
    pub single_newmv: *mut [int_mv; 8],
    pub single_newmv_rate: *mut [core::ffi::c_int; 8],
    pub single_newmv_valid: *mut [core::ffi::c_int; 8],
    pub modelled_rd: *mut [[int64_t; 8]; 3],
    pub ref_frame_cost: core::ffi::c_int,
    pub single_comp_cost: core::ffi::c_int,
    pub simple_rd: *mut [[int64_t; 8]; 3],
    pub skip_motion_mode: core::ffi::c_int,
    pub skip_ifs: bool,
    pub inter_intra_mode: *mut INTERINTRA_MODE,
    pub interp_filter_stats: [INTERPOLATION_FILTER_STATS; 128],
    pub start_mv_stack: [FULLPEL_MV; 6],
    pub ref_mv_idx_stack: [uint8_t; 6],
    pub start_mv_cnt: core::ffi::c_int,
    pub interp_filter_stats_idx: core::ffi::c_int,
    pub wedge_index: core::ffi::c_int,
    pub wedge_sign: core::ffi::c_int,
    pub diffwtd_index: core::ffi::c_int,
    pub cmp_mode: [core::ffi::c_int; 29],
    pub best_single_sse_in_refs: [core::ffi::c_uint; 8],
    pub best_pred_sse: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inter_mode_info {
    pub drl_cost: core::ffi::c_int,
    pub full_search_mv: int_mv,
    pub full_mv_rate: core::ffi::c_int,
    pub full_mv_bestsme: core::ffi::c_int,
    pub skip: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cand_mv_t {
    pub fmv: int_mv,
    pub weight: core::ffi::c_int,
}
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
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
pub const QINDEX_BITS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
static mut kZeroMv: MV = {
    let mut init = mv {
        row: 0 as int16_t,
        col: 0 as int16_t,
    };
    init
};
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
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_inter_compound_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= COMP_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < COMP_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
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
unsafe extern "C-unwind" fn has_second_ref(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_buf_by_bd(
    mut xd: *const MACROBLOCKD,
    mut buf16: *mut uint8_t,
) -> *mut uint8_t {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        (buf16 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t
    } else {
        buf16
    };
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_mode(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn init_interp_filter_params(
    mut interp_filter_params: *mut *const InterpFilterParams,
    mut filter: *const InterpFilters,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
) {
    if is_intrabc as core::ffi::c_long != 0 {
        let fresh1 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh1 = &raw const av1_intrabc_filter_params;
        let fresh2 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh2 = &raw const av1_intrabc_filter_params;
    } else {
        let fresh3 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh3 = av1_get_interp_filter_params_with_block_size(
            (*filter).x_filter as InterpFilter,
            block_width,
        );
        let fresh4 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh4 = av1_get_interp_filter_params_with_block_size(
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
pub const RDDIV_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MV_COST_WEIGHT: core::ffi::c_int = 108 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_set_ms_compound_refs(
    mut ms_buffers: *mut MSBuffers,
    mut second_pred: *const uint8_t,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) {
    (*ms_buffers).second_pred = second_pred;
    (*ms_buffers).mask = mask;
    (*ms_buffers).mask_stride = mask_stride;
    (*ms_buffers).inv_mask = invert_mask;
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
unsafe extern "C-unwind" fn av1_set_fractional_mv(mut fractional_best_mv: *mut int_mv) {
    let mut z: core::ffi::c_int = 0 as core::ffi::c_int;
    while z < 3 as core::ffi::c_int {
        (*fractional_best_mv.offset(z as isize)).as_int = INVALID_MV as uint32_t;
        z += 1;
    }
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
unsafe extern "C-unwind" fn get_ref_frame_yv12_buf(
    cm: *const AV1_COMMON,
    mut ref_frame: MV_REFERENCE_FRAME,
) -> *const YV12_BUFFER_CONFIG {
    let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
    return if !buf.is_null() { &(*buf).buf } else { 0 as *const YV12_BUFFER_CONFIG };
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
pub const NUM_JOINT_ME_REFINE_ITER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const REDUCED_JOINT_ME_REFINE_ITER: core::ffi::c_int = 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn set_offsets_for_motion_search(
    cpi: *const AV1_COMP,
    x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
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
        mi_params,
        &raw mut (*x).mv_limits,
        mi_row,
        mi_col,
        mi_height,
        mi_width,
        (*cpi).oxcf.border_in_pixels,
    );
    set_plane_n4(xd, mi_width, mi_height, num_planes);
    (*xd).mi_row = mi_row;
    (*xd).mi_col = mi_col;
    if !(mi_col & mi_width - 1 as core::ffi::c_int == 0
        && mi_row & mi_height - 1 as core::ffi::c_int == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"set_offsets_for_motion_search\0"))
                .as_ptr(),
            b"partition_strategy.h\0" as *const u8 as *const core::ffi::c_char,
            154 as core::ffi::c_int,
            b"!(mi_col & (mi_width - 1)) && !(mi_row & (mi_height - 1))\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    (*xd).mb_to_top_edge = -(mi_row * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int)
        * 8 as core::ffi::c_int);
    (*xd).mb_to_bottom_edge = ((*mi_params).mi_rows - mi_height - mi_row)
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int;
    (*xd).mb_to_left_edge = -(mi_col * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int)
        * 8 as core::ffi::c_int);
    (*xd).mb_to_right_edge = ((*mi_params).mi_cols - mi_width - mi_col)
        * ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) * 8 as core::ffi::c_int;
    av1_setup_src_planes(
        x as *mut macroblock,
        (*cpi).source,
        mi_row,
        mi_col,
        num_planes,
        bsize,
    );
}
unsafe extern "C-unwind" fn compare_weight(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let diff: core::ffi::c_int = (*(a as *mut cand_mv_t)).weight
        - (*(b as *mut cand_mv_t)).weight;
    if diff < 0 as core::ffi::c_int {
        return 1 as core::ffi::c_int
    } else if diff > 0 as core::ffi::c_int {
        return -(1 as core::ffi::c_int)
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn use_fine_search_interval(cpi: *const AV1_COMP) -> core::ffi::c_int {
    return ((*cpi).is_screen_content_type != 0
        && (*(*cpi).ppi).gf_group.update_type[(*cpi).gf_frame_index as usize]
            as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
        && (*cpi).oxcf.speed <= 2 as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_mv_candidate_from_tpl(
    cpi: *const AV1_COMP,
    mut x: *const MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut ref_0: core::ffi::c_int,
    mut cand: *mut cand_mv_t,
    mut cand_count: *mut core::ffi::c_int,
    mut total_cand_weight: *mut core::ffi::c_int,
) {
    let mut sb_enc: *const SuperBlockEnc = &(*x).sb_enc;
    if (*sb_enc).tpl_data_count == 0 {
        return;
    }
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut xd: *const MACROBLOCKD = &(*x).e_mbd;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let tpl_bsize: BLOCK_SIZE = convert_length_to_bsize(
        (*(*cpi).ppi).tpl_data.tpl_bsize_1d as core::ffi::c_int,
    ) as BLOCK_SIZE;
    let tplw: core::ffi::c_int = mi_size_wide[tpl_bsize as usize] as core::ffi::c_int;
    let tplh: core::ffi::c_int = mi_size_high[tpl_bsize as usize] as core::ffi::c_int;
    let nw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int / tplw;
    let nh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int / tplh;
    if nw >= 1 as core::ffi::c_int && nh >= 1 as core::ffi::c_int {
        let of_h: core::ffi::c_int = mi_row
            % mi_size_high[(*(*cm).seq_params).sb_size as usize] as core::ffi::c_int;
        let of_w: core::ffi::c_int = mi_col
            % mi_size_wide[(*(*cm).seq_params).sb_size as usize] as core::ffi::c_int;
        let start: core::ffi::c_int = of_h / tplh * (*sb_enc).tpl_stride + of_w / tplw;
        let mut valid: core::ffi::c_int = 1 as core::ffi::c_int;
        (*cand.offset(0 as core::ffi::c_int as isize)).weight = nw * nh;
        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
        while k < nh {
            let mut l: core::ffi::c_int = 0 as core::ffi::c_int;
            while l < nw {
                let mv: int_mv = (*sb_enc)
                    .tpl_mv[(start + k * (*sb_enc).tpl_stride + l)
                    as usize][(ref_0 - LAST_FRAME as core::ffi::c_int) as usize];
                if mv.as_int == INVALID_MV as uint32_t {
                    valid = 0 as core::ffi::c_int;
                    break;
                } else {
                    let fmv: FULLPEL_MV = {
                        let mut init = fullpel_mv {
                            row: (mv.as_mv.row as core::ffi::c_int
                                + 3 as core::ffi::c_int
                                + (mv.as_mv.row as core::ffi::c_int
                                    >= 0 as core::ffi::c_int) as core::ffi::c_int
                                >> 3 as core::ffi::c_int) as int16_t,
                            col: (mv.as_mv.col as core::ffi::c_int
                                + 3 as core::ffi::c_int
                                + (mv.as_mv.col as core::ffi::c_int
                                    >= 0 as core::ffi::c_int) as core::ffi::c_int
                                >> 3 as core::ffi::c_int) as int16_t,
                        };
                        init
                    };
                    let mut unique: core::ffi::c_int = 1 as core::ffi::c_int;
                    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m < *cand_count {
                        if fmv.row as core::ffi::c_int + 3 as core::ffi::c_int
                            + (fmv.row as core::ffi::c_int >= 0 as core::ffi::c_int)
                                as core::ffi::c_int >> 3 as core::ffi::c_int
                            == (*cand.offset(m as isize)).fmv.as_fullmv.row
                                as core::ffi::c_int + 3 as core::ffi::c_int
                                + ((*cand.offset(m as isize)).fmv.as_fullmv.row
                                    as core::ffi::c_int >= 0 as core::ffi::c_int)
                                    as core::ffi::c_int >> 3 as core::ffi::c_int
                            && fmv.col as core::ffi::c_int + 3 as core::ffi::c_int
                                + (fmv.col as core::ffi::c_int >= 0 as core::ffi::c_int)
                                    as core::ffi::c_int >> 3 as core::ffi::c_int
                                == (*cand.offset(m as isize)).fmv.as_fullmv.col
                                    as core::ffi::c_int + 3 as core::ffi::c_int
                                    + ((*cand.offset(m as isize)).fmv.as_fullmv.col
                                        as core::ffi::c_int >= 0 as core::ffi::c_int)
                                        as core::ffi::c_int >> 3 as core::ffi::c_int
                        {
                            unique = 0 as core::ffi::c_int;
                            let fresh5 = &raw mut (*cand.offset(m as isize)).weight;
                            *fresh5 += 1;
                            break;
                        } else {
                            m += 1;
                        }
                    }
                    if unique != 0 {
                        (*cand.offset(*cand_count as isize)).fmv.as_fullmv = fmv;
                        (*cand.offset(*cand_count as isize)).weight = 1
                            as core::ffi::c_int;
                        *cand_count += 1;
                    }
                    l += 1;
                }
            }
            if valid == 0 {
                break;
            }
            k += 1;
        }
        if valid != 0 {
            *total_cand_weight = 2 as core::ffi::c_int * nh * nw;
            if *cand_count > 2 as core::ffi::c_int {
                qsort(
                    cand as *mut core::ffi::c_void,
                    *cand_count as size_t,
                    ::core::mem::size_of::<cand_mv_t>() as size_t,
                    Some(
                        compare_weight
                            as unsafe extern "C-unwind" fn(
                                *const core::ffi::c_void,
                                *const core::ffi::c_void,
                            ) -> core::ffi::c_int,
                    ),
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_single_motion_search(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut ref_idx: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut search_range: core::ffi::c_int,
    mut mode_info: *mut inter_mode_info,
    mut best_mv: *mut int_mv,
    args: *mut HandleInterModeArgs,
) {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut cm: *const AV1_COMMON = &(*cpi).common;
    let mut mv_search_params: *const MotionVectorSearchParams = &(*cpi).mv_search_params;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut backup_yv12: [buf_2d; 3] = [
        {
            let mut init = buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0 as core::ffi::c_int,
                height: 0 as core::ffi::c_int,
                stride: 0 as core::ffi::c_int,
            };
            init
        },
        buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        },
        buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        },
    ];
    let mut bestsme: core::ffi::c_int = INT_MAX;
    let ref_0: core::ffi::c_int = (*mbmi).ref_frame[ref_idx as usize]
        as core::ffi::c_int;
    let mut scaled_ref_frame: *const YV12_BUFFER_CONFIG = av1_get_scaled_ref_frame(
        cpi as *const AV1_COMP,
        ref_0,
    );
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut mv_costs: *const MvCosts = (*x).mv_costs;
    if !scaled_ref_frame.is_null() {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < num_planes {
            backup_yv12[i as usize] = (*xd).plane[i as usize].pre[ref_idx as usize];
            i += 1;
        }
        av1_setup_pre_planes(
            xd,
            ref_idx,
            scaled_ref_frame,
            mi_row,
            mi_col,
            0 as *const scale_factors,
            num_planes,
        );
    }
    let mut step_param: core::ffi::c_int = 0;
    if (*cpi).sf.mv_sf.auto_mv_step_size != 0 && (*cm).show_frame != 0 {
        step_param = (av1_init_search_range(
            (*x).max_mv_context[ref_0 as usize] as core::ffi::c_int,
        ) + (*mv_search_params).mv_step_param) / 2 as core::ffi::c_int;
    } else {
        step_param = (*mv_search_params).mv_step_param;
    }
    let ref_mv: MV = (av1_get_ref_mv(x, ref_idx)).as_mv as MV;
    let mut start_mv: FULLPEL_MV = fullpel_mv { row: 0, col: 0 };
    if (*mbmi).motion_mode as core::ffi::c_int != SIMPLE_TRANSLATION as core::ffi::c_int
    {
        start_mv = get_fullmv_from_mv(
            &raw mut (*((*mbmi).mv).as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                .as_mv,
        );
    } else {
        start_mv = get_fullmv_from_mv(&ref_mv);
    }
    let fullpel_ref_mv: FULLPEL_MV = start_mv;
    let mut cand: [cand_mv_t; 65] = [cand_mv_t {
        fmv: int_mv { as_int: 0 },
        weight: 0,
    }; 65];
    memset(
        &raw mut cand as *mut [cand_mv_t; 65] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[cand_mv_t; 65]>() as size_t,
    );
    cand[0 as core::ffi::c_int as usize].fmv.as_fullmv = start_mv;
    let mut cnt: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut total_weight: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*cpi).sf.mv_sf.full_pixel_search_level == 0
        && (*mbmi).motion_mode as core::ffi::c_int
            == SIMPLE_TRANSLATION as core::ffi::c_int
    {
        get_mv_candidate_from_tpl(
            cpi,
            x,
            bsize,
            ref_0,
            cand.as_mut_ptr(),
            &raw mut cnt,
            &raw mut total_weight,
        );
    }
    let cand_cnt: core::ffi::c_int = if (2 as core::ffi::c_int) < cnt {
        2 as core::ffi::c_int
    } else {
        cnt
    };
    if (*cpi).sf.mv_sf.skip_fullpel_search_using_startmv_refmv != 0
        && (*mbmi).motion_mode as core::ffi::c_int
            == SIMPLE_TRANSLATION as core::ffi::c_int
    {
        let mut cand_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while cand_idx < cand_cnt {
            let mut fmv_cand: *mut int_mv = &raw mut (*cand
                .as_mut_ptr()
                .offset(cand_idx as isize))
                .fmv;
            let mut skip_cand_mv: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut stack_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while stack_idx < (*args).start_mv_cnt {
                let mut this_ref_mv_idx: uint8_t = (*args)
                    .ref_mv_idx_stack[stack_idx as usize];
                let this_newmv_valid: core::ffi::c_int = (*((*args).single_newmv_valid)
                    .offset(this_ref_mv_idx as isize))[ref_0 as usize];
                if !(this_newmv_valid == 0
                    && this_ref_mv_idx as core::ffi::c_int
                        != (*mbmi).ref_mv_idx() as core::ffi::c_int)
                {
                    let mut fmv_stack: *const FULLPEL_MV = &raw mut *((*args).start_mv_stack)
                        .as_mut_ptr()
                        .offset(stack_idx as isize) as *mut FULLPEL_MV;
                    let start_mv_row_diff: core::ffi::c_int = abs(
                        (*fmv_stack).row as core::ffi::c_int
                            - (*fmv_cand).as_fullmv.row as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    let start_mv_col_diff: core::ffi::c_int = abs(
                        (*fmv_stack).col as core::ffi::c_int
                            - (*fmv_cand).as_fullmv.col as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    if (*mbmi).mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
                        || (*mbmi).mode as core::ffi::c_int
                            == NEW_NEARMV as core::ffi::c_int
                    {
                        if (has_second_ref(mbmi) == 0) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 25],
                                    [core::ffi::c_char; 25],
                                >(*b"av1_single_motion_search\0"))
                                    .as_ptr(),
                                b"motion_search_facade.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                209 as core::ffi::c_int,
                                b"has_second_ref(mbmi)\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        this_ref_mv_idx = (this_ref_mv_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as uint8_t;
                    }
                    let this_ref_mv: MV = (av1_get_ref_mv_from_stack(
                        ref_idx,
                        ((*mbmi).ref_frame).as_mut_ptr(),
                        this_ref_mv_idx as core::ffi::c_int,
                        &raw mut (*x).mbmi_ext,
                    ))
                        .as_mv as MV;
                    if !(!((*args).ref_mv_idx_stack[stack_idx as usize]
                        as core::ffi::c_int == (*mbmi).ref_mv_idx() as core::ffi::c_int)
                        || this_ref_mv.row as core::ffi::c_int
                            == ref_mv.row as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 25],
                                [core::ffi::c_char; 25],
                            >(*b"av1_single_motion_search\0"))
                                .as_ptr(),
                            b"motion_search_facade.c\0" as *const u8
                                as *const core::ffi::c_char,
                            218 as core::ffi::c_int,
                            b"IMPLIES(args->ref_mv_idx_stack[stack_idx] == mbmi->ref_mv_idx, this_ref_mv.row == ref_mv.row)\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    if !(!((*args).ref_mv_idx_stack[stack_idx as usize]
                        as core::ffi::c_int == (*mbmi).ref_mv_idx() as core::ffi::c_int)
                        || this_ref_mv.col as core::ffi::c_int
                            == ref_mv.col as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 25],
                                [core::ffi::c_char; 25],
                            >(*b"av1_single_motion_search\0"))
                                .as_ptr(),
                            b"motion_search_facade.c\0" as *const u8
                                as *const core::ffi::c_char,
                            220 as core::ffi::c_int,
                            b"IMPLIES(args->ref_mv_idx_stack[stack_idx] == mbmi->ref_mv_idx, this_ref_mv.col == ref_mv.col)\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    let this_fullpel_ref_mv: FULLPEL_MV = get_fullmv_from_mv(
                        &this_ref_mv,
                    ) as FULLPEL_MV;
                    let ref_mv_row_diff: core::ffi::c_int = abs(
                        this_fullpel_ref_mv.row as core::ffi::c_int
                            - fullpel_ref_mv.row as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    let ref_mv_col_diff: core::ffi::c_int = abs(
                        this_fullpel_ref_mv.col as core::ffi::c_int
                            - fullpel_ref_mv.col as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    if (*cpi).sf.mv_sf.skip_fullpel_search_using_startmv_refmv
                        >= 2 as core::ffi::c_int
                    {
                        if start_mv_row_diff <= 1 as core::ffi::c_int
                            && start_mv_col_diff <= 1 as core::ffi::c_int
                            && ref_mv_row_diff <= 1 as core::ffi::c_int
                            && ref_mv_col_diff <= 1 as core::ffi::c_int
                        {
                            skip_cand_mv = 1 as core::ffi::c_int;
                            break;
                        }
                    } else if (*cpi).sf.mv_sf.skip_fullpel_search_using_startmv_refmv
                        >= 1 as core::ffi::c_int
                    {
                        if start_mv_row_diff + start_mv_col_diff <= 1 as core::ffi::c_int
                            && ref_mv_row_diff + ref_mv_col_diff <= 1 as core::ffi::c_int
                        {
                            skip_cand_mv = 1 as core::ffi::c_int;
                            break;
                        }
                    }
                }
                stack_idx += 1;
            }
            if skip_cand_mv != 0 {
                cand[cand_idx as usize].fmv.as_int = INVALID_MV as uint32_t;
            } else {
                if !((*args).start_mv_cnt
                    < 3 as core::ffi::c_int * 2 as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 25],
                            [core::ffi::c_char; 25],
                        >(*b"av1_single_motion_search\0"))
                            .as_ptr(),
                        b"motion_search_facade.c\0" as *const u8
                            as *const core::ffi::c_char,
                        252 as core::ffi::c_int,
                        b"args->start_mv_cnt < MAX_REF_MV_SEARCH * 2\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                (*args).start_mv_stack[(*args).start_mv_cnt as usize] = (*fmv_cand)
                    .as_fullmv;
                (*args).ref_mv_idx_stack[(*args).start_mv_cnt as usize] = (*mbmi)
                    .ref_mv_idx();
                (*args).start_mv_cnt += 1;
            }
            cand_idx += 1;
        }
    }
    let mut mv_sf: *const MV_SPEED_FEATURES = &(*cpi).sf.mv_sf;
    let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(x, mv_sf, bsize)
        as SEARCH_METHODS;
    let mut src_search_site_cfg: *const search_site_config = av1_get_search_site_config(
        cpi,
        x,
        search_method,
    );
    if search_range < INT_MAX {
        let mut search_site_cfg: *const search_site_config = &*src_search_site_cfg
            .offset(
                *core::ptr::addr_of!(search_method_lookup).cast::<SEARCH_METHODS>().offset(search_method as isize) as isize,
            ) as *const search_site_config;
        if search_range < 1 as core::ffi::c_int {
            step_param = (*search_site_cfg).num_search_steps;
        } else {
            while (*search_site_cfg)
                .radius[((*search_site_cfg).num_search_steps - step_param
                - 1 as core::ffi::c_int) as usize]
                > search_range << 1 as core::ffi::c_int
                && (*search_site_cfg).num_search_steps - step_param
                    - 1 as core::ffi::c_int > 0 as core::ffi::c_int
            {
                step_param += 1;
            }
        }
    }
    let mut cost_list: [core::ffi::c_int; 5] = [0; 5];
    let mut best_mv_stats: FULLPEL_MV_STATS = FULLPEL_MV_STATS {
        err_cost: 0,
        distortion: 0,
        sse: 0,
    };
    let mut second_best_mv: int_mv = int_mv { as_int: 0 };
    second_best_mv.as_int = INVALID_MV as uint32_t;
    (*best_mv).as_int = second_best_mv.as_int;
    let fine_search_interval: core::ffi::c_int = use_fine_search_interval(cpi)
        as core::ffi::c_int;
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
    match (*mbmi).motion_mode as core::ffi::c_int {
        0 => {
            let mut sum_weight: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
            while m < cand_cnt {
                let mut smv: int_mv = cand[m as usize].fmv;
                let mut this_best_mv: FULLPEL_MV = fullpel_mv { row: 0, col: 0 };
                let mut this_second_best_mv: FULLPEL_MV = fullpel_mv { row: 0, col: 0 };
                let mut this_mv_stats: FULLPEL_MV_STATS = FULLPEL_MV_STATS {
                    err_cost: 0,
                    distortion: 0,
                    sse: 0,
                };
                if !(smv.as_int == INVALID_MV as uint32_t) {
                    av1_make_default_fullpel_ms_params(
                        &raw mut full_ms_params,
                        cpi as *const AV1_COMP,
                        x,
                        bsize,
                        &ref_mv,
                        smv.as_fullmv,
                        src_search_site_cfg as *const search_site_config,
                        search_method,
                        fine_search_interval,
                    );
                    let thissme: core::ffi::c_int = av1_full_pixel_search(
                        smv.as_fullmv,
                        &raw mut full_ms_params,
                        step_param,
                        cond_cost_list(cpi as *const AV1_COMP, cost_list.as_mut_ptr()),
                        &raw mut this_best_mv,
                        &raw mut this_mv_stats,
                        &raw mut this_second_best_mv,
                    ) as core::ffi::c_int;
                    if thissme < bestsme {
                        bestsme = thissme;
                        (*best_mv).as_fullmv = this_best_mv;
                        best_mv_stats = this_mv_stats;
                        second_best_mv.as_fullmv = this_second_best_mv;
                    }
                    sum_weight += cand[m as usize].weight;
                    if 4 as core::ffi::c_int * sum_weight
                        > 3 as core::ffi::c_int * total_weight
                    {
                        break;
                    }
                }
                m += 1;
            }
        }
        1 => {
            av1_make_default_fullpel_ms_params(
                &raw mut full_ms_params,
                cpi as *const AV1_COMP,
                x,
                bsize,
                &ref_mv,
                start_mv,
                src_search_site_cfg as *const search_site_config,
                search_method,
                fine_search_interval,
            );
            bestsme = av1_obmc_full_pixel_search(
                start_mv,
                &raw mut full_ms_params,
                step_param,
                &raw mut (*best_mv).as_fullmv,
            );
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid motion mode!\n\0" as *const u8
                    as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"av1_single_motion_search\0"))
                        .as_ptr(),
                    b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                    333 as core::ffi::c_int,
                    b"0 && \"Invalid motion mode!\\n\"\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    if (*best_mv).as_int == INVALID_MV as uint32_t {
        return;
    }
    if !scaled_ref_frame.is_null() {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < num_planes {
            (*xd).plane[i_0 as usize].pre[ref_idx as usize] = backup_yv12[i_0 as usize];
            i_0 += 1;
        }
    }
    if (*cpi).sf.inter_sf.skip_newmv_in_drl >= 2 as core::ffi::c_int
        && (*mbmi).motion_mode as core::ffi::c_int
            == SIMPLE_TRANSLATION as core::ffi::c_int
        && (*best_mv).as_int != INVALID_MV as uint32_t
    {
        let mut this_mv: int_mv = int_mv { as_int: 0 };
        this_mv.as_mv = get_mv_from_fullmv(&raw mut (*best_mv).as_fullmv);
        let ref_mv_idx: core::ffi::c_int = (*mbmi).ref_mv_idx() as core::ffi::c_int;
        let this_mv_rate: core::ffi::c_int = av1_mv_bit_cost(
            &raw mut this_mv.as_mv,
            &ref_mv,
            ((*mv_costs).nmv_joint_cost).as_ptr(),
            (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
            MV_COST_WEIGHT,
        ) as core::ffi::c_int;
        (*mode_info.offset(ref_mv_idx as isize)).full_search_mv.as_int = this_mv.as_int;
        (*mode_info.offset(ref_mv_idx as isize)).full_mv_rate = this_mv_rate;
        (*mode_info.offset(ref_mv_idx as isize)).full_mv_bestsme = bestsme;
        let mut prev_ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while prev_ref_idx < ref_mv_idx {
            if this_mv.as_int
                == (*mode_info.offset(prev_ref_idx as isize)).full_search_mv.as_int
            {
                let prev_rate_cost: core::ffi::c_int = (*mode_info
                    .offset(prev_ref_idx as isize))
                    .full_mv_rate + (*mode_info.offset(prev_ref_idx as isize)).drl_cost;
                let this_rate_cost: core::ffi::c_int = this_mv_rate
                    + (*mode_info.offset(ref_mv_idx as isize)).drl_cost;
                if prev_rate_cost <= this_rate_cost {
                    (*best_mv).as_int = INVALID_MV as uint32_t;
                    return;
                }
            }
            let psme: core::ffi::c_int = (*mode_info.offset(prev_ref_idx as isize))
                .full_mv_bestsme;
            if !(psme == INT_MAX) {
                let thr: core::ffi::c_int = if (*cpi).sf.inter_sf.skip_newmv_in_drl
                    == 3 as core::ffi::c_int
                {
                    psme + (psme >> 2 as core::ffi::c_int)
                } else {
                    psme
                };
                if (*cpi).sf.inter_sf.skip_newmv_in_drl >= 3 as core::ffi::c_int
                    && (*mode_info.offset(ref_mv_idx as isize)).full_mv_bestsme > thr
                    && (*mode_info.offset(prev_ref_idx as isize)).drl_cost
                        < (*mode_info.offset(ref_mv_idx as isize)).drl_cost
                {
                    (*best_mv).as_int = INVALID_MV as uint32_t;
                    return;
                }
            }
            prev_ref_idx += 1;
        }
    }
    if (*cpi).common.features.cur_frame_force_integer_mv {
        convert_fullmv_to_mv(best_mv);
    }
    let use_fractional_mv: core::ffi::c_int = (bestsme < INT_MAX
        && (*cpi).common.features.cur_frame_force_integer_mv as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut best_mv_rate: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mv_rate_calculated: core::ffi::c_int = 0 as core::ffi::c_int;
    if use_fractional_mv != 0 {
        let mut fractional_ms_list: [int_mv; 3] = [int_mv { as_int: 0 }; 3];
        av1_set_fractional_mv(fractional_ms_list.as_mut_ptr());
        let mut dis: core::ffi::c_int = 0;
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
            cpi as *const AV1_COMP,
            x,
            bsize,
            &ref_mv,
            cost_list.as_mut_ptr(),
        );
        let mut subpel_start_mv: MV = get_mv_from_fullmv(&raw mut (*best_mv).as_fullmv);
        if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, subpel_start_mv) == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"av1_single_motion_search\0"))
                    .as_ptr(),
                b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                411 as core::ffi::c_int,
                b"av1_is_subpelmv_in_range(&ms_params.mv_limits, subpel_start_mv)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        match (*mbmi).motion_mode as core::ffi::c_int {
            0 => {
                if (*mv_sf).use_accurate_subpel_search != 0 {
                    let try_second: core::ffi::c_int = (second_best_mv.as_int
                        != INVALID_MV as uint32_t
                        && second_best_mv.as_int != (*best_mv).as_int
                        && (*mv_sf).disable_second_mv <= 1 as core::ffi::c_int)
                        as core::ffi::c_int;
                    let best_mv_var: core::ffi::c_int = ((*mv_search_params)
                        .find_fractional_mv_step)
                        .expect(
                            "non-null function pointer",
                        )(
                        xd,
                        cm,
                        &raw mut ms_params,
                        subpel_start_mv,
                        &raw mut best_mv_stats,
                        &raw mut (*best_mv).as_mv,
                        &raw mut dis,
                        &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
                        fractional_ms_list.as_mut_ptr(),
                    ) as core::ffi::c_int;
                    if try_second != 0 {
                        let mut p: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
                        let orig_dst: BUFFER_SET = {
                            let mut init = BUFFER_SET {
                                plane: [
                                    (*p.offset(0 as core::ffi::c_int as isize)).dst.buf,
                                    (*p.offset(1 as core::ffi::c_int as isize)).dst.buf,
                                    (*p.offset(2 as core::ffi::c_int as isize)).dst.buf,
                                ],
                                stride: [
                                    (*p.offset(0 as core::ffi::c_int as isize)).dst.stride,
                                    (*p.offset(1 as core::ffi::c_int as isize)).dst.stride,
                                    (*p.offset(2 as core::ffi::c_int as isize)).dst.stride,
                                ],
                            };
                            init
                        };
                        let mut this_best_mv_0: MV = mv { row: 0, col: 0 };
                        subpel_start_mv = get_mv_from_fullmv(
                            &raw mut second_best_mv.as_fullmv,
                        );
                        if av1_is_subpelmv_in_range(
                            &raw mut ms_params.mv_limits,
                            subpel_start_mv,
                        ) != 0
                        {
                            let mut sse: core::ffi::c_uint = 0;
                            let this_var: core::ffi::c_int = ((*mv_search_params)
                                .find_fractional_mv_step)
                                .expect(
                                    "non-null function pointer",
                                )(
                                xd,
                                cm,
                                &raw mut ms_params,
                                subpel_start_mv,
                                0 as *const FULLPEL_MV_STATS,
                                &raw mut this_best_mv_0,
                                &raw mut dis,
                                &raw mut sse,
                                fractional_ms_list.as_mut_ptr(),
                            ) as core::ffi::c_int;
                            if (*mv_sf).disable_second_mv == 0 && this_var != INT32_MAX {
                                (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv = (*best_mv)
                                    .as_mv;
                                av1_enc_build_inter_predictor(
                                    cm,
                                    xd,
                                    mi_row,
                                    mi_col,
                                    &orig_dst,
                                    bsize,
                                    0 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                );
                                let mut this_rd_stats: RD_STATS = RD_STATS {
                                    rate: 0,
                                    zero_rate: 0,
                                    dist: 0,
                                    rdcost: 0,
                                    sse: 0,
                                    skip_txfm: 0,
                                };
                                av1_init_rd_stats(&raw mut this_rd_stats);
                                av1_estimate_txfm_yrd(
                                    cpi,
                                    x,
                                    &raw mut this_rd_stats,
                                    INT64_MAX as int64_t,
                                    bsize,
                                    max_txsize_rect_lookup[bsize as usize],
                                );
                                let mut this_mv_rate_0: core::ffi::c_int = av1_mv_bit_cost(
                                    &raw mut (*best_mv).as_mv,
                                    &ref_mv,
                                    ((*mv_costs).nmv_joint_cost).as_ptr(),
                                    (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
                                    MV_COST_WEIGHT,
                                );
                                let rd: int64_t = ((this_mv_rate_0 + this_rd_stats.rate)
                                    as int64_t * (*x).rdmult as int64_t
                                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                                        >> 1 as core::ffi::c_int) as int64_t
                                    >> 9 as core::ffi::c_int)
                                    + this_rd_stats.dist
                                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                                (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv = this_best_mv_0;
                                av1_enc_build_inter_predictor(
                                    cm,
                                    xd,
                                    mi_row,
                                    mi_col,
                                    &orig_dst,
                                    bsize,
                                    0 as core::ffi::c_int,
                                    0 as core::ffi::c_int,
                                );
                                let mut tmp_rd_stats: RD_STATS = RD_STATS {
                                    rate: 0,
                                    zero_rate: 0,
                                    dist: 0,
                                    rdcost: 0,
                                    sse: 0,
                                    skip_txfm: 0,
                                };
                                av1_init_rd_stats(&raw mut tmp_rd_stats);
                                av1_estimate_txfm_yrd(
                                    cpi,
                                    x,
                                    &raw mut tmp_rd_stats,
                                    INT64_MAX as int64_t,
                                    bsize,
                                    max_txsize_rect_lookup[bsize as usize],
                                );
                                let mut tmp_mv_rate: core::ffi::c_int = av1_mv_bit_cost(
                                    &raw mut this_best_mv_0,
                                    &ref_mv,
                                    ((*mv_costs).nmv_joint_cost).as_ptr(),
                                    (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
                                    MV_COST_WEIGHT,
                                );
                                let mut tmp_rd: int64_t = ((tmp_rd_stats.rate + tmp_mv_rate)
                                    as int64_t * (*x).rdmult as int64_t
                                    + ((1 as core::ffi::c_int) << 9 as core::ffi::c_int
                                        >> 1 as core::ffi::c_int) as int64_t
                                    >> 9 as core::ffi::c_int)
                                    + tmp_rd_stats.dist
                                        * ((1 as core::ffi::c_int) << RDDIV_BITS) as int64_t;
                                if tmp_rd < rd {
                                    (*best_mv).as_mv = this_best_mv_0;
                                    (*x).pred_sse[ref_0 as usize] = sse;
                                }
                            } else if this_var < best_mv_var {
                                (*best_mv).as_mv = this_best_mv_0;
                                (*x).pred_sse[ref_0 as usize] = sse;
                            }
                        }
                    }
                } else {
                    ((*mv_search_params).find_fractional_mv_step)
                        .expect(
                            "non-null function pointer",
                        )(
                        xd,
                        cm,
                        &raw mut ms_params,
                        subpel_start_mv,
                        &raw mut best_mv_stats,
                        &raw mut (*best_mv).as_mv,
                        &raw mut dis,
                        &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
                        0 as *mut int_mv,
                    );
                }
            }
            1 => {
                av1_find_best_obmc_sub_pixel_tree_up(
                    xd,
                    cm,
                    &raw mut ms_params,
                    subpel_start_mv,
                    0 as *const FULLPEL_MV_STATS,
                    &raw mut (*best_mv).as_mv,
                    &raw mut dis,
                    &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
                    0 as *mut int_mv,
                );
            }
            _ => {
                if !(0 as core::ffi::c_int != 0
                    && !(b"Invalid motion mode!\n\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 25],
                            [core::ffi::c_char; 25],
                        >(*b"av1_single_motion_search\0"))
                            .as_ptr(),
                        b"motion_search_facade.c\0" as *const u8
                            as *const core::ffi::c_char,
                        496 as core::ffi::c_int,
                        b"0 && \"Invalid motion mode!\\n\"\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
        if (*cpi).sf.inter_sf.skip_newmv_in_drl >= 1 as core::ffi::c_int
            && !args.is_null()
            && (*mbmi).motion_mode as core::ffi::c_int
                == SIMPLE_TRANSLATION as core::ffi::c_int
            && (*best_mv).as_int != INVALID_MV as uint32_t
        {
            let ref_mv_idx_0: core::ffi::c_int = (*mbmi).ref_mv_idx()
                as core::ffi::c_int;
            best_mv_rate = av1_mv_bit_cost(
                &raw mut (*best_mv).as_mv,
                &ref_mv,
                ((*mv_costs).nmv_joint_cost).as_ptr(),
                (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
                MV_COST_WEIGHT,
            );
            mv_rate_calculated = 1 as core::ffi::c_int;
            let mut prev_ref_idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while prev_ref_idx_0 < ref_mv_idx_0 {
                if !((*((*args).single_newmv_valid)
                    .offset(prev_ref_idx_0 as isize))[ref_0 as usize] == 0)
                {
                    if (*best_mv).as_int
                        == (*((*args).single_newmv)
                            .offset(prev_ref_idx_0 as isize))[ref_0 as usize]
                            .as_int
                    {
                        if (*mode_info.offset(prev_ref_idx_0 as isize)).skip != 0 {
                            (*mode_info.offset(ref_mv_idx_0 as isize)).skip = 1
                                as core::ffi::c_int;
                            break;
                        } else {
                            let prev_rate_cost_0: core::ffi::c_int = (*((*args)
                                .single_newmv_rate)
                                .offset(prev_ref_idx_0 as isize))[ref_0 as usize]
                                + (*mode_info.offset(prev_ref_idx_0 as isize)).drl_cost;
                            let this_rate_cost_0: core::ffi::c_int = best_mv_rate
                                + (*mode_info.offset(ref_mv_idx_0 as isize)).drl_cost;
                            if prev_rate_cost_0 <= this_rate_cost_0 {
                                (*mode_info.offset(ref_mv_idx_0 as isize)).skip = 1
                                    as core::ffi::c_int;
                                break;
                            }
                        }
                    }
                }
                prev_ref_idx_0 += 1;
            }
        }
    }
    if mv_rate_calculated != 0 {
        *rate_mv = best_mv_rate;
    } else {
        *rate_mv = av1_mv_bit_cost(
            &raw mut (*best_mv).as_mv,
            &ref_mv,
            ((*mv_costs).nmv_joint_cost).as_ptr(),
            (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
            MV_COST_WEIGHT,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_joint_motion_search(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut cur_mv: *mut int_mv,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut allow_second_mv: core::ffi::c_int,
    mut joint_me_num_refine_iter: core::ffi::c_int,
    mut use_subpel_mv_cost_none: bool,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let pw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let ph: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let plane: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (has_second_ref(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_joint_motion_search\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            559 as core::ffi::c_int,
            b"has_second_ref(mbmi)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let init_mv: [int_mv; 2] = [
        *cur_mv.offset(0 as core::ffi::c_int as isize),
        *cur_mv.offset(1 as core::ffi::c_int as isize),
    ];
    let refs: [core::ffi::c_int; 2] = [
        (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int,
        (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int,
    ];
    let mut mv_costs: *const MvCosts = (*x).mv_costs;
    let mut ref_mv: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
    let mut ite: core::ffi::c_int = 0;
    let mut ref_0: core::ffi::c_int = 0;
    let interp_filters: int_interpfilters = av1_broadcast_interp_filter(EIGHTTAP_REGULAR)
        as int_interpfilters;
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
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut backup_yv12: [[buf_2d; 3]; 2] = [[buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3]; 2];
    let mut last_besterr: [core::ffi::c_int; 2] = [INT_MAX, INT_MAX];
    let scaled_ref_frame: [*const YV12_BUFFER_CONFIG; 2] = [
        av1_get_scaled_ref_frame(
            cpi as *const AV1_COMP,
            refs[0 as core::ffi::c_int as usize],
        ) as *const YV12_BUFFER_CONFIG,
        av1_get_scaled_ref_frame(
            cpi as *const AV1_COMP,
            refs[1 as core::ffi::c_int as usize],
        ) as *const YV12_BUFFER_CONFIG,
    ];
    let mut second_pred16: crate::platform::Aligned16<[uint8_t; 32768]> = crate::platform::Aligned16([0; 32768]);
    let mut second_pred: *mut uint8_t = get_buf_by_bd(xd, second_pred16.0.as_mut_ptr());
    let mut best_mv: int_mv = int_mv { as_int: 0 };
    let mut second_best_mv: int_mv = int_mv { as_int: 0 };
    ite = 0 as core::ffi::c_int;
    while ite < 2 as core::ffi::c_int * joint_me_num_refine_iter {
        let mut bestsme: core::ffi::c_int = INT_MAX;
        let mut id: core::ffi::c_int = ite % 2 as core::ffi::c_int;
        if ite >= 2 as core::ffi::c_int
            && (*cur_mv.offset((id == 0) as core::ffi::c_int as isize)).as_int
                == init_mv[(id == 0) as core::ffi::c_int as usize].as_int
        {
            if (*cur_mv.offset(id as isize)).as_int == init_mv[id as usize].as_int {
                break;
            }
            let mut cur_int_mv: int_mv = int_mv { as_int: 0 };
            let mut init_int_mv: int_mv = int_mv { as_int: 0 };
            cur_int_mv.as_mv.col = ((*cur_mv.offset(id as isize)).as_mv.col
                as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
            cur_int_mv.as_mv.row = ((*cur_mv.offset(id as isize)).as_mv.row
                as core::ffi::c_int >> 3 as core::ffi::c_int) as int16_t;
            init_int_mv.as_mv.row = (init_mv[id as usize].as_mv.row as core::ffi::c_int
                >> 3 as core::ffi::c_int) as int16_t;
            init_int_mv.as_mv.col = (init_mv[id as usize].as_mv.col as core::ffi::c_int
                >> 3 as core::ffi::c_int) as int16_t;
            if cur_int_mv.as_int == init_int_mv.as_int {
                break;
            }
        }
        ref_0 = 0 as core::ffi::c_int;
        while ref_0 < 2 as core::ffi::c_int {
            ref_mv[ref_0 as usize] = av1_get_ref_mv(x, ref_0);
            if !(scaled_ref_frame[ref_0 as usize]).is_null() {
                let mut i: core::ffi::c_int = 0;
                i = 0 as core::ffi::c_int;
                while i < num_planes {
                    backup_yv12[ref_0 as usize][i as usize] = (*xd)
                        .plane[i as usize]
                        .pre[ref_0 as usize];
                    i += 1;
                }
                av1_setup_pre_planes(
                    xd,
                    ref_0,
                    scaled_ref_frame[ref_0 as usize],
                    mi_row,
                    mi_col,
                    0 as *const scale_factors,
                    num_planes,
                );
            }
            ref_0 += 1;
        }
        if !((scaled_ref_frame[0 as core::ffi::c_int as usize]).is_null()
            || (*cm).width
                == (*scaled_ref_frame[0 as core::ffi::c_int as usize])
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .y_crop_width
                && (*cm).height
                    == (*scaled_ref_frame[0 as core::ffi::c_int as usize])
                        .c2rust_unnamed_2
                        .c2rust_unnamed
                        .y_crop_height) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"av1_joint_motion_search\0"))
                    .as_ptr(),
                b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                625 as core::ffi::c_int,
                b"IMPLIES(scaled_ref_frame[0] != NULL, cm->width == scaled_ref_frame[0]->y_crop_width && cm->height == scaled_ref_frame[0]->y_crop_height)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((scaled_ref_frame[1 as core::ffi::c_int as usize]).is_null()
            || (*cm).width
                == (*scaled_ref_frame[1 as core::ffi::c_int as usize])
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .y_crop_width
                && (*cm).height
                    == (*scaled_ref_frame[1 as core::ffi::c_int as usize])
                        .c2rust_unnamed_2
                        .c2rust_unnamed
                        .y_crop_height) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"av1_joint_motion_search\0"))
                    .as_ptr(),
                b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                628 as core::ffi::c_int,
                b"IMPLIES(scaled_ref_frame[1] != NULL, cm->width == scaled_ref_frame[1]->y_crop_width && cm->height == scaled_ref_frame[1]->y_crop_height)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_init_inter_params(
            &raw mut inter_pred_params,
            pw,
            ph,
            mi_row * MI_SIZE,
            mi_col * MI_SIZE,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (*xd).bd,
            is_cur_buf_hbd(xd),
            0 as core::ffi::c_int,
            &(*cm).sf_identity,
            &raw mut *((*((*xd).plane).as_mut_ptr().offset(plane as isize)).pre)
                .as_mut_ptr()
                .offset((id == 0) as core::ffi::c_int as isize),
            interp_filters,
        );
        inter_pred_params.conv_params = get_conv_params(
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            (*xd).bd,
        );
        av1_enc_build_one_inter_predictor(
            second_pred,
            pw,
            &raw mut (*cur_mv.offset((id == 0) as core::ffi::c_int as isize)).as_mv,
            &raw mut inter_pred_params,
        );
        let mut orig_yv12: buf_2d = buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        };
        if id != 0 {
            orig_yv12 = (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize];
            (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize] = (*xd)
                .plane[plane as usize]
                .pre[id as usize];
        }
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
        let mut mv_sf: *const MV_SPEED_FEATURES = &(*cpi).sf.mv_sf;
        let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(
            x,
            mv_sf,
            bsize,
        ) as SEARCH_METHODS;
        let mut src_search_sites: *const search_site_config = av1_get_search_site_config(
            cpi,
            x,
            search_method,
        );
        let start_fullmv: FULLPEL_MV = get_fullmv_from_mv(
            &raw mut (*cur_mv.offset(id as isize)).as_mv,
        ) as FULLPEL_MV;
        av1_make_default_fullpel_ms_params(
            &raw mut full_ms_params,
            cpi as *const AV1_COMP,
            x,
            bsize,
            &raw mut (*ref_mv.as_mut_ptr().offset(id as isize)).as_mv,
            start_fullmv,
            src_search_sites as *const search_site_config,
            search_method,
            0 as core::ffi::c_int,
        );
        av1_set_ms_compound_refs(
            &raw mut full_ms_params.ms_buffers,
            second_pred,
            mask,
            mask_stride,
            id,
        );
        if (*mv_sf).disable_extensive_joint_motion_search == 0
            && (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                != COMPOUND_WEDGE as core::ffi::c_int
        {
            bestsme = av1_full_pixel_search(
                start_fullmv,
                &raw mut full_ms_params,
                5 as core::ffi::c_int,
                0 as *mut core::ffi::c_int,
                &raw mut best_mv.as_fullmv,
                &raw mut best_mv_stats,
                &raw mut second_best_mv.as_fullmv,
            );
        } else {
            bestsme = av1_refining_search_8p_c(
                &raw mut full_ms_params,
                start_fullmv,
                &raw mut best_mv.as_fullmv,
            );
            second_best_mv = best_mv;
        }
        let try_second: core::ffi::c_int = (second_best_mv.as_int
            != INVALID_MV as uint32_t && second_best_mv.as_int != best_mv.as_int
            && allow_second_mv != 0) as core::ffi::c_int;
        if id != 0 {
            (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize] = orig_yv12;
        }
        ref_0 = 0 as core::ffi::c_int;
        while ref_0 < 2 as core::ffi::c_int {
            if !(scaled_ref_frame[ref_0 as usize]).is_null() {
                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_0 < num_planes {
                    (*xd).plane[i_0 as usize].pre[ref_0 as usize] = backup_yv12[ref_0
                        as usize][i_0 as usize];
                    i_0 += 1;
                }
            }
            ref_0 += 1;
        }
        let mut orig_sf: *const scale_factors = 0 as *const scale_factors;
        if id != 0 {
            orig_yv12 = (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize];
            (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize] = (*xd)
                .plane[plane as usize]
                .pre[id as usize];
            orig_sf = (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize];
            (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = (*xd)
                .block_ref_scale_factors[id as usize];
        }
        if (*cpi).common.features.cur_frame_force_integer_mv {
            convert_fullmv_to_mv(&raw mut best_mv);
        }
        if bestsme < INT_MAX
            && (*cpi).common.features.cur_frame_force_integer_mv as core::ffi::c_int
                == 0 as core::ffi::c_int
        {
            let mut dis: core::ffi::c_int = 0;
            let mut sse: core::ffi::c_uint = 0;
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
                cpi as *const AV1_COMP,
                x,
                bsize,
                &raw mut (*ref_mv.as_mut_ptr().offset(id as isize)).as_mv,
                0 as *const core::ffi::c_int,
            );
            av1_set_ms_compound_refs(
                &raw mut ms_params.var_params.ms_buffers,
                second_pred,
                mask,
                mask_stride,
                id,
            );
            ms_params.forced_stop = EIGHTH_PEL as core::ffi::c_int as SUBPEL_FORCE_STOP;
            if use_subpel_mv_cost_none {
                ms_params.mv_cost_params.mv_cost_type = MV_COST_NONE as core::ffi::c_int
                    as MV_COST_TYPE;
            }
            let mut start_mv: MV = get_mv_from_fullmv(&raw mut best_mv.as_fullmv);
            if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, start_mv) == 0)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_joint_motion_search\0"))
                        .as_ptr(),
                    b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                    727 as core::ffi::c_int,
                    b"av1_is_subpelmv_in_range(&ms_params.mv_limits, start_mv)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            bestsme = ((*cpi).mv_search_params.find_fractional_mv_step)
                .expect(
                    "non-null function pointer",
                )(
                xd,
                cm,
                &raw mut ms_params,
                start_mv,
                0 as *const FULLPEL_MV_STATS,
                &raw mut best_mv.as_mv,
                &raw mut dis,
                &raw mut sse,
                0 as *mut int_mv,
            );
            if try_second != 0 {
                let mut this_best_mv: MV = mv { row: 0, col: 0 };
                let mut subpel_start_mv: MV = get_mv_from_fullmv(
                    &raw mut second_best_mv.as_fullmv,
                );
                if av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, subpel_start_mv)
                    != 0
                {
                    let thissme: core::ffi::c_int = ((*cpi)
                        .mv_search_params
                        .find_fractional_mv_step)
                        .expect(
                            "non-null function pointer",
                        )(
                        xd,
                        cm,
                        &raw mut ms_params,
                        subpel_start_mv,
                        0 as *const FULLPEL_MV_STATS,
                        &raw mut this_best_mv,
                        &raw mut dis,
                        &raw mut sse,
                        0 as *mut int_mv,
                    ) as core::ffi::c_int;
                    if thissme < bestsme {
                        best_mv.as_mv = this_best_mv;
                        bestsme = thissme;
                    }
                }
            }
        }
        if id != 0 {
            (*xd).plane[plane as usize].pre[0 as core::ffi::c_int as usize] = orig_yv12;
            (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = orig_sf;
        }
        if !(bestsme < last_besterr[id as usize]) {
            break;
        }
        *cur_mv.offset(id as isize) = best_mv;
        last_besterr[id as usize] = bestsme;
        ite += 1;
    }
    *rate_mv = 0 as core::ffi::c_int;
    ref_0 = 0 as core::ffi::c_int;
    while ref_0 < 2 as core::ffi::c_int {
        let curr_ref_mv: int_mv = av1_get_ref_mv(x, ref_0) as int_mv;
        *rate_mv
            += av1_mv_bit_cost(
                &raw mut (*cur_mv.offset(ref_0 as isize)).as_mv,
                &curr_ref_mv.as_mv,
                ((*mv_costs).nmv_joint_cost).as_ptr(),
                (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
                MV_COST_WEIGHT,
            );
        ref_0 += 1;
    }
    return if last_besterr[0 as core::ffi::c_int as usize]
        < last_besterr[1 as core::ffi::c_int as usize]
    {
        last_besterr[0 as core::ffi::c_int as usize]
    } else {
        last_besterr[1 as core::ffi::c_int as usize]
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compound_single_motion_search(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut this_mv: *mut MV,
    mut second_pred: *const uint8_t,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut ref_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let ref_0: core::ffi::c_int = (*mbmi).ref_frame[ref_idx as usize]
        as core::ffi::c_int;
    let ref_mv: int_mv = av1_get_ref_mv(x, ref_idx) as int_mv;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    let mut mv_costs: *const MvCosts = (*x).mv_costs;
    let mut backup_yv12: [buf_2d; 3] = [buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    }; 3];
    let scaled_ref_frame: *const YV12_BUFFER_CONFIG = av1_get_scaled_ref_frame(
        cpi as *const AV1_COMP,
        ref_0,
    );
    if !(has_second_ref(mbmi) != 0
        || ref_idx == 0 as core::ffi::c_int && is_interintra_mode(mbmi) != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_compound_single_motion_search\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            792 as core::ffi::c_int,
            b"has_second_ref(mbmi) || (ref_idx == 0 && is_interintra_mode(mbmi))\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut orig_yv12: buf_2d = buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    };
    if ref_idx != 0 {
        orig_yv12 = (*pd).pre[0 as core::ffi::c_int as usize];
        (*pd).pre[0 as core::ffi::c_int as usize] = (*pd).pre[ref_idx as usize];
    }
    if !scaled_ref_frame.is_null() {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < num_planes {
            backup_yv12[i as usize] = (*xd).plane[i as usize].pre[ref_idx as usize];
            i += 1;
        }
        let mi_row: core::ffi::c_int = (*xd).mi_row;
        let mi_col: core::ffi::c_int = (*xd).mi_col;
        av1_setup_pre_planes(
            xd,
            0 as core::ffi::c_int,
            scaled_ref_frame,
            mi_row,
            mi_col,
            0 as *const scale_factors,
            num_planes,
        );
    }
    let mut bestsme: core::ffi::c_int = INT_MAX;
    let mut best_mv: int_mv = int_mv { as_int: 0 };
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
    let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(
        x,
        &(*cpi).sf.mv_sf,
        bsize,
    ) as SEARCH_METHODS;
    let mut src_search_sites: *const search_site_config = av1_get_search_site_config(
        cpi,
        x,
        search_method,
    );
    let start_fullmv: FULLPEL_MV = get_fullmv_from_mv(this_mv) as FULLPEL_MV;
    av1_make_default_fullpel_ms_params(
        &raw mut full_ms_params,
        cpi as *const AV1_COMP,
        x,
        bsize,
        &ref_mv.as_mv,
        start_fullmv,
        src_search_sites as *const search_site_config,
        search_method,
        0 as core::ffi::c_int,
    );
    av1_set_ms_compound_refs(
        &raw mut full_ms_params.ms_buffers,
        second_pred,
        mask,
        mask_stride,
        ref_idx,
    );
    bestsme = av1_full_pixel_search(
        start_fullmv,
        &raw mut full_ms_params,
        5 as core::ffi::c_int,
        0 as *mut core::ffi::c_int,
        &raw mut best_mv.as_fullmv,
        &raw mut best_mv_stats,
        0 as *mut FULLPEL_MV,
    );
    if !scaled_ref_frame.is_null() {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < num_planes {
            (*xd).plane[i_0 as usize].pre[0 as core::ffi::c_int as usize] = backup_yv12[i_0
                as usize];
            i_0 += 1;
        }
    }
    let mut orig_sf: *const scale_factors = 0 as *const scale_factors;
    if ref_idx != 0 {
        orig_sf = (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize];
        (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = (*xd)
            .block_ref_scale_factors[ref_idx as usize];
    }
    if (*cpi).common.features.cur_frame_force_integer_mv {
        convert_fullmv_to_mv(&raw mut best_mv);
    }
    let use_fractional_mv: core::ffi::c_int = (bestsme < INT_MAX
        && (*cpi).common.features.cur_frame_force_integer_mv as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int;
    if use_fractional_mv != 0 {
        let mut dis: core::ffi::c_int = 0;
        let mut sse: core::ffi::c_uint = 0;
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
            cpi as *const AV1_COMP,
            x,
            bsize,
            &ref_mv.as_mv,
            0 as *const core::ffi::c_int,
        );
        av1_set_ms_compound_refs(
            &raw mut ms_params.var_params.ms_buffers,
            second_pred,
            mask,
            mask_stride,
            ref_idx,
        );
        ms_params.forced_stop = EIGHTH_PEL as core::ffi::c_int as SUBPEL_FORCE_STOP;
        let mut start_mv: MV = get_mv_from_fullmv(&raw mut best_mv.as_fullmv);
        if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, start_mv) == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 34],
                    [core::ffi::c_char; 34],
                >(*b"av1_compound_single_motion_search\0"))
                    .as_ptr(),
                b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                871 as core::ffi::c_int,
                b"av1_is_subpelmv_in_range(&ms_params.mv_limits, start_mv)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        bestsme = ((*cpi).mv_search_params.find_fractional_mv_step)
            .expect(
                "non-null function pointer",
            )(
            xd,
            cm,
            &raw mut ms_params,
            start_mv,
            &raw mut best_mv_stats,
            &raw mut best_mv.as_mv,
            &raw mut dis,
            &raw mut sse,
            0 as *mut int_mv,
        );
    }
    if ref_idx != 0 {
        (*pd).pre[0 as core::ffi::c_int as usize] = orig_yv12;
        (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = orig_sf;
    }
    if bestsme < INT_MAX {
        *this_mv = best_mv.as_mv;
    }
    *rate_mv = 0 as core::ffi::c_int;
    *rate_mv
        += av1_mv_bit_cost(
            this_mv,
            &ref_mv.as_mv,
            ((*mv_costs).nmv_joint_cost).as_ptr(),
            (*mv_costs).mv_cost_stack as *const *mut core::ffi::c_int,
            MV_COST_WEIGHT,
        );
    return bestsme;
}
#[inline]
unsafe extern "C-unwind" fn build_second_inter_pred(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut other_mv: *const MV,
    mut ref_idx: core::ffi::c_int,
    mut second_pred: *mut uint8_t,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let pw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let ph: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut macroblockd_plane;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let p_col: core::ffi::c_int = mi_col * MI_SIZE >> (*pd).subsampling_x;
    let p_row: core::ffi::c_int = mi_row * MI_SIZE >> (*pd).subsampling_y;
    if (has_second_ref(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"build_second_inter_pred\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            907 as core::ffi::c_int,
            b"has_second_ref(mbmi)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let plane: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ref_yv12: buf_2d = (*xd)
        .plane[plane as usize]
        .pre[(ref_idx == 0) as core::ffi::c_int as usize];
    let mut sf: scale_factors = scale_factors {
        x_scale_fp: 0,
        y_scale_fp: 0,
        x_step_q4: 0,
        y_step_q4: 0,
    };
    av1_setup_scale_factors_for_frame(
        &raw mut sf,
        ref_yv12.width,
        ref_yv12.height,
        (*cm).width,
        (*cm).height,
    );
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
        pw,
        ph,
        p_row,
        p_col,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
        (*xd).bd,
        is_cur_buf_hbd(xd),
        0 as core::ffi::c_int,
        &raw mut sf,
        &raw mut ref_yv12,
        (*mbmi).interp_filters,
    );
    inter_pred_params.conv_params = get_conv_params(
        0 as core::ffi::c_int,
        plane,
        (*xd).bd,
    );
    av1_enc_build_one_inter_predictor(second_pred, pw, other_mv, &raw mut inter_pred_params);
}
unsafe extern "C-unwind" fn compound_single_motion_search_interinter(
    mut cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut bsize: BLOCK_SIZE,
    mut cur_mv: *mut int_mv,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut rate_mv: *mut core::ffi::c_int,
    mut ref_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    if (has_second_ref(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 41],
                [core::ffi::c_char; 41],
            >(*b"compound_single_motion_search_interinter\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            936 as core::ffi::c_int,
            b"has_second_ref(xd->mi[0])\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut second_pred_alloc_16: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    let mut second_pred: *mut uint8_t = 0 as *mut uint8_t;
    if is_cur_buf_hbd(xd) != 0 {
        second_pred = (second_pred_alloc_16.0.as_mut_ptr() as uintptr_t
            >> 1 as core::ffi::c_int) as *mut uint8_t;
    } else {
        second_pred = second_pred_alloc_16.0.as_mut_ptr() as *mut uint8_t;
    }
    let mut this_mv: *mut MV = &raw mut (*cur_mv.offset(ref_idx as isize)).as_mv;
    let mut other_mv: *const MV = &raw mut (*cur_mv
        .offset((ref_idx == 0) as core::ffi::c_int as isize))
        .as_mv;
    build_second_inter_pred(cpi, x, bsize, other_mv, ref_idx, second_pred);
    return av1_compound_single_motion_search(
        cpi,
        x,
        bsize,
        this_mv,
        second_pred,
        mask,
        mask_stride,
        rate_mv,
        ref_idx,
    );
}
#[inline]
unsafe extern "C-unwind" fn do_masked_motion_search_indexed(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    cur_mv: *const int_mv,
    comp_data: *const INTERINTER_COMPOUND_DATA,
    mut bsize: BLOCK_SIZE,
    mut tmp_mv: *mut int_mv,
    mut rate_mv: *mut core::ffi::c_int,
    mut which: core::ffi::c_int,
) {
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut sb_type: BLOCK_SIZE = (*mbmi).bsize;
    let mut mask: *const uint8_t = 0 as *const uint8_t;
    let mask_stride: core::ffi::c_int = block_size_wide[bsize as usize]
        as core::ffi::c_int;
    mask = av1_get_compound_type_mask(comp_data, sb_type);
    (*tmp_mv.offset(0 as core::ffi::c_int as isize)).as_int = (*cur_mv
        .offset(0 as core::ffi::c_int as isize))
        .as_int;
    (*tmp_mv.offset(1 as core::ffi::c_int as isize)).as_int = (*cur_mv
        .offset(1 as core::ffi::c_int as isize))
        .as_int;
    if which == 0 as core::ffi::c_int || which == 1 as core::ffi::c_int {
        compound_single_motion_search_interinter(
            cpi,
            x,
            bsize,
            tmp_mv,
            mask,
            mask_stride,
            rate_mv,
            which,
        );
    } else if which == 2 as core::ffi::c_int {
        let joint_me_num_refine_iter: core::ffi::c_int = if (*cpi)
            .sf
            .inter_sf
            .enable_fast_compound_mode_search == 2 as core::ffi::c_int
        {
            REDUCED_JOINT_ME_REFINE_ITER
        } else {
            NUM_JOINT_ME_REFINE_ITER
        };
        av1_joint_motion_search(
            cpi,
            x,
            bsize,
            tmp_mv,
            mask,
            mask_stride,
            rate_mv,
            ((*cpi).sf.mv_sf.disable_second_mv == 0) as core::ffi::c_int,
            joint_me_num_refine_iter,
            false_0 != 0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_interinter_compound_motion_search(
    cpi: *const AV1_COMP,
    mut x: *mut MACROBLOCK,
    cur_mv: *const int_mv,
    bsize: BLOCK_SIZE,
    this_mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut tmp_mv: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
    let mut tmp_rate_mv: core::ffi::c_int = 0 as core::ffi::c_int;
    (*mbmi).interinter_comp.seg_mask = if (*mbmi).interinter_comp.type_0
        as core::ffi::c_int == COMPOUND_AVERAGE as core::ffi::c_int
    {
        0 as *mut uint8_t
    } else {
        (*xd).seg_mask
    };
    let mut compound_data: *const INTERINTER_COMPOUND_DATA = &raw mut (*mbmi)
        .interinter_comp;
    if this_mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int {
        do_masked_motion_search_indexed(
            cpi,
            x,
            cur_mv,
            compound_data,
            bsize,
            tmp_mv.as_mut_ptr(),
            &raw mut tmp_rate_mv,
            2 as core::ffi::c_int,
        );
        (*mbmi).mv[0 as core::ffi::c_int as usize].as_int = tmp_mv[0 as core::ffi::c_int
                as usize]
            .as_int;
        (*mbmi).mv[1 as core::ffi::c_int as usize].as_int = tmp_mv[1 as core::ffi::c_int
                as usize]
            .as_int;
    } else if this_mode as core::ffi::c_int >= NEAREST_NEWMV as core::ffi::c_int
        && this_mode as core::ffi::c_int <= NEW_NEARMV as core::ffi::c_int
    {
        let mut which: core::ffi::c_int = (NEWMV as core::ffi::c_int
            == compound_ref1_mode(this_mode) as core::ffi::c_int) as core::ffi::c_int;
        do_masked_motion_search_indexed(
            cpi,
            x,
            cur_mv,
            compound_data,
            bsize,
            tmp_mv.as_mut_ptr(),
            &raw mut tmp_rate_mv,
            which,
        );
        (*mbmi).mv[which as usize].as_int = tmp_mv[which as usize].as_int;
    }
    return tmp_rate_mv;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_simple_motion_search_sse_var(
    cpi: *mut AV1_COMP,
    mut x: *mut MACROBLOCK,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut ref_0: core::ffi::c_int,
    mut start_mv: FULLPEL_MV,
    mut num_planes: core::ffi::c_int,
    mut use_subpixel: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut var: *mut core::ffi::c_uint,
) -> int_mv {
    if !(num_planes == 1 as core::ffi::c_int
        && !(b"Currently simple_motion_search only supports luma plane\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_simple_motion_search_sse_var\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            1022 as core::ffi::c_int,
            b"num_planes == 1 && \"Currently simple_motion_search only supports luma plane\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(frame_is_intra_only(&raw mut (*cpi).common) == 0
        && !(b"Simple motion search only enabled for non-key frames\0" as *const u8
            as *const core::ffi::c_char)
            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_simple_motion_search_sse_var\0"))
                .as_ptr(),
            b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
            1024 as core::ffi::c_int,
            b"!frame_is_intra_only(&cpi->common) && \"Simple motion search only enabled for non-key frames\"\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    set_offsets_for_motion_search(cpi, x, mi_row, mi_col, bsize);
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    (*mbmi).bsize = bsize;
    (*mbmi).ref_frame[0 as core::ffi::c_int as usize] = ref_0 as MV_REFERENCE_FRAME;
    (*mbmi).ref_frame[1 as core::ffi::c_int as usize] = NONE_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    (*mbmi).motion_mode = SIMPLE_TRANSLATION as core::ffi::c_int as MOTION_MODE;
    (*mbmi).interp_filters = av1_broadcast_interp_filter(EIGHTTAP_REGULAR);
    let mut yv12: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
        cm,
        ref_0 as MV_REFERENCE_FRAME,
    );
    let mut scaled_ref_frame: *const YV12_BUFFER_CONFIG = av1_get_scaled_ref_frame(
        cpi,
        ref_0,
    );
    let mut backup_yv12: buf_2d = buf_2d {
        buf: 0 as *mut uint8_t,
        buf0: 0 as *mut uint8_t,
        width: 0,
        height: 0,
        stride: 0,
    };
    let ref_mv: MV = kZeroMv;
    let step_param: core::ffi::c_int = if (*cpi).mv_search_params.mv_step_param
        + (*cpi).sf.part_sf.simple_motion_search_reduce_search_steps
        < 11 as core::ffi::c_int - 2 as core::ffi::c_int
    {
        (*cpi).mv_search_params.mv_step_param
            + (*cpi).sf.part_sf.simple_motion_search_reduce_search_steps
    } else {
        11 as core::ffi::c_int - 2 as core::ffi::c_int
    };
    let mut cost_list: [core::ffi::c_int; 5] = [0; 5];
    let ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bestsme: core::ffi::c_int = 0;
    let mut best_mv: int_mv = int_mv { as_int: 0 };
    let mut best_mv_stats: FULLPEL_MV_STATS = FULLPEL_MV_STATS {
        err_cost: 0,
        distortion: 0,
        sse: 0,
    };
    av1_setup_pre_planes(
        xd,
        ref_idx,
        yv12,
        mi_row,
        mi_col,
        get_ref_scale_factors(cm, ref_0 as MV_REFERENCE_FRAME),
        num_planes,
    );
    set_ref_ptrs(
        cm,
        xd,
        (*mbmi).ref_frame[0 as core::ffi::c_int as usize],
        (*mbmi).ref_frame[1 as core::ffi::c_int as usize],
    );
    if !scaled_ref_frame.is_null() {
        backup_yv12 = (*xd).plane[AOM_PLANE_Y as usize].pre[ref_idx as usize];
        av1_setup_pre_planes(
            xd,
            ref_idx,
            scaled_ref_frame,
            mi_row,
            mi_col,
            0 as *const scale_factors,
            num_planes,
        );
    }
    let fine_search_interval: core::ffi::c_int = use_fine_search_interval(cpi)
        as core::ffi::c_int;
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
    let mut mv_sf: *const MV_SPEED_FEATURES = &raw mut (*cpi).sf.mv_sf;
    let search_method: SEARCH_METHODS = av1_get_default_mv_search_method(x, mv_sf, bsize)
        as SEARCH_METHODS;
    let mut src_search_sites: *const search_site_config = av1_get_search_site_config(
        cpi,
        x,
        search_method,
    );
    av1_make_default_fullpel_ms_params(
        &raw mut full_ms_params,
        cpi,
        x,
        bsize,
        &ref_mv,
        start_mv,
        src_search_sites as *const search_site_config,
        search_method,
        fine_search_interval,
    );
    bestsme = av1_full_pixel_search(
        start_mv,
        &raw mut full_ms_params,
        step_param,
        cond_cost_list(cpi, cost_list.as_mut_ptr()),
        &raw mut best_mv.as_fullmv,
        &raw mut best_mv_stats,
        0 as *mut FULLPEL_MV,
    );
    let use_subpel_search: core::ffi::c_int = (bestsme < INT_MAX
        && !(*cpi).common.features.cur_frame_force_integer_mv && use_subpixel != 0
        && (*cpi).sf.mv_sf.simple_motion_subpel_force_stop as core::ffi::c_int
            != FULL_PEL as core::ffi::c_int) as core::ffi::c_int;
    if !scaled_ref_frame.is_null() {
        (*xd).plane[AOM_PLANE_Y as usize].pre[ref_idx as usize] = backup_yv12;
    }
    if use_subpel_search != 0 {
        let mut not_used: core::ffi::c_int = 0 as core::ffi::c_int;
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
        ms_params.forced_stop = (*mv_sf).simple_motion_subpel_force_stop;
        let mut subpel_start_mv: MV = get_mv_from_fullmv(&raw mut best_mv.as_fullmv);
        if (av1_is_subpelmv_in_range(&raw mut ms_params.mv_limits, subpel_start_mv) == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 33],
                    [core::ffi::c_char; 33],
                >(*b"av1_simple_motion_search_sse_var\0"))
                    .as_ptr(),
                b"motion_search_facade.c\0" as *const u8 as *const core::ffi::c_char,
                1095 as core::ffi::c_int,
                b"av1_is_subpelmv_in_range(&ms_params.mv_limits, subpel_start_mv)\0"
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
            subpel_start_mv,
            &raw mut best_mv_stats,
            &raw mut best_mv.as_mv,
            &raw mut not_used,
            &raw mut *((*x).pred_sse).as_mut_ptr().offset(ref_0 as isize),
            0 as *mut int_mv,
        );
        (*mbmi).mv[0 as core::ffi::c_int as usize] = best_mv;
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
        *var = ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
            .expect(
                "non-null function pointer",
            )(
            (*x).plane[0 as core::ffi::c_int as usize].src.buf,
            (*x).plane[0 as core::ffi::c_int as usize].src.stride,
            (*xd).plane[0 as core::ffi::c_int as usize].dst.buf,
            (*xd).plane[0 as core::ffi::c_int as usize].dst.stride,
            sse,
        );
    } else {
        convert_fullmv_to_mv(&raw mut best_mv);
        *var = best_mv_stats.distortion;
        *sse = best_mv_stats.sse;
    }
    return best_mv;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
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
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
