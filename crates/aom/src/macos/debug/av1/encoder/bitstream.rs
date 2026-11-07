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

// Mechanically translated from upstream/aom/av1/encoder/bitstream.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    fn aom_uleb_size_in_bytes(value: uint64_t) -> size_t;
    fn aom_uleb_decode(
        buffer: *const uint8_t,
        available: size_t,
        value: *mut uint64_t,
        length: *mut size_t,
    ) -> core::ffi::c_int;
    fn aom_uleb_encode(
        value: uint64_t,
        available: size_t,
        coded_value: *mut uint8_t,
        coded_size: *mut size_t,
    ) -> core::ffi::c_int;
    fn od_ec_encode_bool_q15(
        enc: *mut od_ec_enc,
        val: core::ffi::c_int,
        f_q15: core::ffi::c_uint,
    );
    fn od_ec_encode_cdf_q15(
        enc: *mut od_ec_enc,
        s: core::ffi::c_int,
        cdf: *const uint16_t,
        nsyms: core::ffi::c_int,
    );
    fn aom_start_encode(w: *mut aom_writer, buffer: *mut uint8_t, size: size_t);
    fn aom_stop_encode(w: *mut aom_writer) -> core::ffi::c_int;
    fn aom_tell_size(w: *mut aom_writer) -> core::ffi::c_int;
    fn aom_wb_is_byte_aligned(wb: *const aom_write_bit_buffer) -> core::ffi::c_int;
    fn aom_wb_bytes_written(wb: *const aom_write_bit_buffer) -> uint32_t;
    fn aom_wb_write_bit(wb: *mut aom_write_bit_buffer, bit: core::ffi::c_int);
    fn aom_wb_write_literal(
        wb: *mut aom_write_bit_buffer,
        data: core::ffi::c_int,
        bits: core::ffi::c_int,
    );
    fn aom_wb_write_unsigned_literal(
        wb: *mut aom_write_bit_buffer,
        data: uint32_t,
        bits: core::ffi::c_int,
    );
    fn aom_wb_overwrite_literal(
        wb: *mut aom_write_bit_buffer,
        data: core::ffi::c_int,
        bits: core::ffi::c_int,
    );
    fn aom_wb_write_inv_signed_literal(
        wb: *mut aom_write_bit_buffer,
        data: core::ffi::c_int,
        bits: core::ffi::c_int,
    );
    fn aom_wb_write_uvlc(wb: *mut aom_write_bit_buffer, v: uint32_t);
    fn aom_wb_write_signed_primitive_refsubexpfin(
        wb: *mut aom_write_bit_buffer,
        n: uint16_t,
        k: uint16_t,
        ref_0: int16_t,
        v: int16_t,
    );
    fn aom_write_primitive_refsubexpfin(
        w: *mut aom_writer,
        n: uint16_t,
        k: uint16_t,
        ref_0: uint16_t,
        v: uint16_t,
    );
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn av1_seg_feature_data_max(feature_id: SEG_LVL_FEATURES) -> core::ffi::c_int;
    fn av1_is_segfeature_signed(feature_id: SEG_LVL_FEATURES) -> core::ffi::c_int;
    fn av1_set_default_ref_deltas(ref_deltas: *mut int8_t);
    fn av1_set_default_mode_deltas(mode_deltas: *mut int8_t);
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
    fn av1_left_block_mode(left_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_above_block_mode(above_mi: *const MB_MODE_INFO) -> PREDICTION_MODE;
    fn av1_reset_loop_filter_delta(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    fn av1_reset_loop_restoration(xd: *mut MACROBLOCKD, num_planes: core::ffi::c_int);
    static av1_sgr_params: [sgr_params_type; 16];
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
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_print_uncompressed_frame_header(
        data: *const uint8_t,
        size: core::ffi::c_int,
        filename: *const core::ffi::c_char,
    );
    fn av1_set_frame_refs(
        cm: *mut AV1_COMMON,
        remapped_ref_idx: *mut core::ffi::c_int,
        lst_map_idx: core::ffi::c_int,
        gld_map_idx: core::ffi::c_int,
    );
    fn av1_get_pred_context_switchable_interp(
        xd: *const MACROBLOCKD,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_palette_cache(
        xd: *const MACROBLOCKD,
        plane: core::ffi::c_int,
        cache: *mut uint16_t,
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
    static av1_wedge_params_lookup: [wedge_params_type; 22];
    fn av1_encode_mv(
        cpi: *mut AV1_COMP,
        w: *mut aom_writer,
        td: *mut ThreadData,
        mv: *const MV,
        ref_0: *const MV,
        mvctx: *mut nmv_context,
        usehp: core::ffi::c_int,
    );
    fn av1_encode_dv(
        w: *mut aom_writer,
        mv: *const MV,
        ref_0: *const MV,
        mvctx: *mut nmv_context,
    );
    fn av1_write_coeffs_txb(
        cm: *const AV1_COMMON,
        x: *mut MACROBLOCK,
        w: *mut aom_writer,
        blk_row: core::ffi::c_int,
        blk_col: core::ffi::c_int,
        plane: core::ffi::c_int,
        block: core::ffi::c_int,
        tx_size: TX_SIZE,
    );
    fn av1_write_intra_coeffs_mb(
        cm: *const AV1_COMMON,
        x: *mut MACROBLOCK,
        w: *mut aom_writer,
        bsize: BLOCK_SIZE,
    );
    fn av1_get_cb_coeff_buffer(
        cpi: *const AV1_COMP,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
    ) -> *mut CB_COEFF_BUFFER;
    fn av1_write_tile_obu_mt(
        cpi: *mut AV1_COMP,
        dst: *mut uint8_t,
        dst_size: size_t,
        total_size: *mut uint32_t,
        saved_wb: *mut aom_write_bit_buffer,
        obu_extn_header: uint8_t,
        fh_info: *const FrameHeaderInfo,
        largest_tile_id: *mut core::ffi::c_int,
        max_tile_size: *mut core::ffi::c_uint,
        obu_header_size: *mut uint32_t,
        tile_data_start: *mut *mut uint8_t,
        num_workers: core::ffi::c_int,
    );
    fn av1_index_color_cache(
        color_cache: *const uint16_t,
        n_cache: core::ffi::c_int,
        colors: *const uint16_t,
        n_colors: core::ffi::c_int,
        cache_color_found: *mut uint8_t,
        out_cache_colors: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_palette_delta_bits_v(
        pmi: *const PALETTE_MODE_INFO,
        bit_depth: core::ffi::c_int,
        zero_count: *mut core::ffi::c_int,
        min_bits: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
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
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
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
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub type aom_cdf_prob = uint16_t;
pub type od_ec_enc_window = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_enc {
    pub buf: *mut core::ffi::c_uchar,
    pub storage: uint32_t,
    pub offs: uint32_t,
    pub low: od_ec_enc_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
    pub error: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_writer {
    pub pos: core::ffi::c_uint,
    pub buffer: *mut uint8_t,
    pub size: size_t,
    pub ec: od_ec_enc,
    pub allow_update_cdf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOKEN_STATS {
    pub cost: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_write_bit_buffer {
    pub bit_buffer: *mut uint8_t,
    pub bit_offset: uint32_t,
}
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
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const EXT_TX_SET_TYPES: C2RustUnnamed_2 = 6;
pub const EXT_TX_SET_ALL16: C2RustUnnamed_2 = 5;
pub const EXT_TX_SET_DTT9_IDTX_1DDCT: C2RustUnnamed_2 = 4;
pub const EXT_TX_SET_DTT4_IDTX_1DDCT: C2RustUnnamed_2 = 3;
pub const EXT_TX_SET_DTT4_IDTX: C2RustUnnamed_2 = 2;
pub const EXT_TX_SET_DCT_IDTX: C2RustUnnamed_2 = 1;
pub const EXT_TX_SET_DCTONLY: C2RustUnnamed_2 = 0;
pub type TxSetType = uint8_t;
pub type float_t = core::ffi::c_float;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const MAX_PROFILES: C2RustUnnamed_3 = 3;
pub const PROFILE_2: C2RustUnnamed_3 = 2;
pub const PROFILE_1: C2RustUnnamed_3 = 1;
pub const PROFILE_0: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const PARTITION_INVALID: C2RustUnnamed_4 = 255;
pub const PARTITION_TYPES: C2RustUnnamed_4 = 4;
pub const EXT_PARTITION_TYPES: C2RustUnnamed_4 = 10;
pub const PARTITION_VERT_4: C2RustUnnamed_4 = 9;
pub const PARTITION_HORZ_4: C2RustUnnamed_4 = 8;
pub const PARTITION_VERT_B: C2RustUnnamed_4 = 7;
pub const PARTITION_VERT_A: C2RustUnnamed_4 = 6;
pub const PARTITION_HORZ_B: C2RustUnnamed_4 = 5;
pub const PARTITION_HORZ_A: C2RustUnnamed_4 = 4;
pub const PARTITION_SPLIT: C2RustUnnamed_4 = 3;
pub const PARTITION_VERT: C2RustUnnamed_4 = 2;
pub const PARTITION_HORZ: C2RustUnnamed_4 = 1;
pub const PARTITION_NONE: C2RustUnnamed_4 = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const TX_MODES: C2RustUnnamed_5 = 3;
pub const TX_MODE_SELECT: C2RustUnnamed_5 = 2;
pub const TX_MODE_LARGEST: C2RustUnnamed_5 = 1;
pub const ONLY_4X4: C2RustUnnamed_5 = 0;
pub type TX_MODE = uint8_t;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const COMP_REFERENCE_TYPES: C2RustUnnamed_6 = 2;
pub const BIDIR_COMP_REFERENCE: C2RustUnnamed_6 = 1;
pub const UNIDIR_COMP_REFERENCE: C2RustUnnamed_6 = 0;
pub type COMP_REFERENCE_TYPE = uint8_t;
pub type C2RustUnnamed_7 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_7 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_7 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_7 = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_8 = core::ffi::c_uint;
pub const CFL_SIGNS: C2RustUnnamed_8 = 3;
pub const CFL_SIGN_POS: C2RustUnnamed_8 = 2;
pub const CFL_SIGN_NEG: C2RustUnnamed_8 = 1;
pub const CFL_SIGN_ZERO: C2RustUnnamed_8 = 0;
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_9 = core::ffi::c_uint;
pub const PALETTE_SIZES: C2RustUnnamed_9 = 7;
pub const EIGHT_COLORS: C2RustUnnamed_9 = 6;
pub const SEVEN_COLORS: C2RustUnnamed_9 = 5;
pub const SIX_COLORS: C2RustUnnamed_9 = 4;
pub const FIVE_COLORS: C2RustUnnamed_9 = 3;
pub const FOUR_COLORS: C2RustUnnamed_9 = 2;
pub const THREE_COLORS: C2RustUnnamed_9 = 1;
pub const TWO_COLORS: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_10 = 25;
pub const INTRA_MODES: C2RustUnnamed_10 = 13;
pub const INTER_MODE_END: C2RustUnnamed_10 = 25;
pub const INTER_MODE_START: C2RustUnnamed_10 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_10 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_10 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_10 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_10 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_10 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_10 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_10 = 13;
pub const DIR_MODE_END: C2RustUnnamed_10 = 9;
pub const DIR_MODE_START: C2RustUnnamed_10 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_10 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_10 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_10 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_10 = 25;
pub const NEW_NEWMV: C2RustUnnamed_10 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_10 = 23;
pub const NEW_NEARMV: C2RustUnnamed_10 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_10 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_10 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_10 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_10 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_10 = 17;
pub const NEWMV: C2RustUnnamed_10 = 16;
pub const GLOBALMV: C2RustUnnamed_10 = 15;
pub const NEARMV: C2RustUnnamed_10 = 14;
pub const NEARESTMV: C2RustUnnamed_10 = 13;
pub const PAETH_PRED: C2RustUnnamed_10 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_10 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_10 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_10 = 9;
pub const D67_PRED: C2RustUnnamed_10 = 8;
pub const D203_PRED: C2RustUnnamed_10 = 7;
pub const D157_PRED: C2RustUnnamed_10 = 6;
pub const D113_PRED: C2RustUnnamed_10 = 5;
pub const D135_PRED: C2RustUnnamed_10 = 4;
pub const D45_PRED: C2RustUnnamed_10 = 3;
pub const H_PRED: C2RustUnnamed_10 = 2;
pub const V_PRED: C2RustUnnamed_10 = 1;
pub const DC_PRED: C2RustUnnamed_10 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_11 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_11 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_11 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_11 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_11 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_11 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_11 = 9;
pub const UV_D67_PRED: C2RustUnnamed_11 = 8;
pub const UV_D203_PRED: C2RustUnnamed_11 = 7;
pub const UV_D157_PRED: C2RustUnnamed_11 = 6;
pub const UV_D113_PRED: C2RustUnnamed_11 = 5;
pub const UV_D135_PRED: C2RustUnnamed_11 = 4;
pub const UV_D45_PRED: C2RustUnnamed_11 = 3;
pub const UV_H_PRED: C2RustUnnamed_11 = 2;
pub const UV_V_PRED: C2RustUnnamed_11 = 1;
pub const UV_DC_PRED: C2RustUnnamed_11 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_12 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_12 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_12 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_12 = 0;
pub type MOTION_MODE = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const INTERINTRA_MODES: C2RustUnnamed_13 = 4;
pub const II_SMOOTH_PRED: C2RustUnnamed_13 = 3;
pub const II_H_PRED: C2RustUnnamed_13 = 2;
pub const II_V_PRED: C2RustUnnamed_13 = 1;
pub const II_DC_PRED: C2RustUnnamed_13 = 0;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_14 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_14 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_14 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_14 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_14 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_14 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const FILTER_INTRA_MODES: C2RustUnnamed_15 = 5;
pub const FILTER_PAETH_PRED: C2RustUnnamed_15 = 4;
pub const FILTER_D157_PRED: C2RustUnnamed_15 = 3;
pub const FILTER_H_PRED: C2RustUnnamed_15 = 2;
pub const FILTER_V_PRED: C2RustUnnamed_15 = 1;
pub const FILTER_DC_PRED: C2RustUnnamed_15 = 0;
pub type FILTER_INTRA_MODE = uint8_t;
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const SEQ_LEVEL_KEEP_STATS: C2RustUnnamed_16 = 32;
pub const SEQ_LEVEL_MAX: C2RustUnnamed_16 = 31;
pub const SEQ_LEVELS: C2RustUnnamed_16 = 28;
pub const SEQ_LEVEL_8_3: C2RustUnnamed_16 = 27;
pub const SEQ_LEVEL_8_2: C2RustUnnamed_16 = 26;
pub const SEQ_LEVEL_8_1: C2RustUnnamed_16 = 25;
pub const SEQ_LEVEL_8_0: C2RustUnnamed_16 = 24;
pub const SEQ_LEVEL_7_3: C2RustUnnamed_16 = 23;
pub const SEQ_LEVEL_7_2: C2RustUnnamed_16 = 22;
pub const SEQ_LEVEL_7_1: C2RustUnnamed_16 = 21;
pub const SEQ_LEVEL_7_0: C2RustUnnamed_16 = 20;
pub const SEQ_LEVEL_6_3: C2RustUnnamed_16 = 19;
pub const SEQ_LEVEL_6_2: C2RustUnnamed_16 = 18;
pub const SEQ_LEVEL_6_1: C2RustUnnamed_16 = 17;
pub const SEQ_LEVEL_6_0: C2RustUnnamed_16 = 16;
pub const SEQ_LEVEL_5_3: C2RustUnnamed_16 = 15;
pub const SEQ_LEVEL_5_2: C2RustUnnamed_16 = 14;
pub const SEQ_LEVEL_5_1: C2RustUnnamed_16 = 13;
pub const SEQ_LEVEL_5_0: C2RustUnnamed_16 = 12;
pub const SEQ_LEVEL_4_3: C2RustUnnamed_16 = 11;
pub const SEQ_LEVEL_4_2: C2RustUnnamed_16 = 10;
pub const SEQ_LEVEL_4_1: C2RustUnnamed_16 = 9;
pub const SEQ_LEVEL_4_0: C2RustUnnamed_16 = 8;
pub const SEQ_LEVEL_3_3: C2RustUnnamed_16 = 7;
pub const SEQ_LEVEL_3_2: C2RustUnnamed_16 = 6;
pub const SEQ_LEVEL_3_1: C2RustUnnamed_16 = 5;
pub const SEQ_LEVEL_3_0: C2RustUnnamed_16 = 4;
pub const SEQ_LEVEL_2_3: C2RustUnnamed_16 = 3;
pub const SEQ_LEVEL_2_2: C2RustUnnamed_16 = 2;
pub const SEQ_LEVEL_2_1: C2RustUnnamed_16 = 1;
pub const SEQ_LEVEL_2_0: C2RustUnnamed_16 = 0;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_17 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_17 = 7;
pub const BWD_REFS: C2RustUnnamed_17 = 3;
pub const FWD_REFS: C2RustUnnamed_17 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_17 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_17 = 8;
pub const REF_FRAMES: C2RustUnnamed_17 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_17 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_17 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_17 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_17 = 4;
pub const LAST3_FRAME: C2RustUnnamed_17 = 3;
pub const LAST2_FRAME: C2RustUnnamed_17 = 2;
pub const LAST_FRAME: C2RustUnnamed_17 = 1;
pub const INTRA_FRAME: C2RustUnnamed_17 = 0;
pub const NONE_FRAME: C2RustUnnamed_17 = -1;
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const UNIDIR_COMP_REFS: C2RustUnnamed_18 = 4;
pub const TOTAL_UNIDIR_COMP_REFS: C2RustUnnamed_18 = 9;
pub const ALTREF2_ALTREF_FRAMES: C2RustUnnamed_18 = 8;
pub const BWDREF_ALTREF2_FRAMES: C2RustUnnamed_18 = 7;
pub const LAST3_GOLDEN_FRAMES: C2RustUnnamed_18 = 6;
pub const LAST2_GOLDEN_FRAMES: C2RustUnnamed_18 = 5;
pub const LAST2_LAST3_FRAMES: C2RustUnnamed_18 = 4;
pub const BWDREF_ALTREF_FRAMES: C2RustUnnamed_18 = 3;
pub const LAST_GOLDEN_FRAMES: C2RustUnnamed_18 = 2;
pub const LAST_LAST3_FRAMES: C2RustUnnamed_18 = 1;
pub const LAST_LAST2_FRAMES: C2RustUnnamed_18 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_29,
    pub c2rust_unnamed_0: C2RustUnnamed_27,
    pub c2rust_unnamed_1: C2RustUnnamed_25,
    pub c2rust_unnamed_2: C2RustUnnamed_23,
    pub c2rust_unnamed_3: C2RustUnnamed_21,
    pub c2rust_unnamed_4: C2RustUnnamed_19,
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
pub union C2RustUnnamed_19 {
    pub c2rust_unnamed: C2RustUnnamed_20,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_22,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_24,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub c2rust_unnamed: C2RustUnnamed_28,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub c2rust_unnamed: C2RustUnnamed_30,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub above: PARTITION_CONTEXT,
    pub left: PARTITION_CONTEXT,
}
pub type C2RustUnnamed_32 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_32 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_32 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_32 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_32 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_32 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_32 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_32 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_32 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_32 = 0;
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
pub type C2RustUnnamed_33 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_33 = 4;
pub const AFFINE: C2RustUnnamed_33 = 3;
pub const ROTZOOM: C2RustUnnamed_33 = 2;
pub const TRANSLATION: C2RustUnnamed_33 = 1;
pub const IDENTITY: C2RustUnnamed_33 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv32 {
    pub row: int32_t,
    pub col: int32_t,
}
pub type MV32 = mv32;
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
pub type C2RustUnnamed_34 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_34 = 4;
pub const S_FRAME: C2RustUnnamed_34 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_34 = 2;
pub const INTER_FRAME: C2RustUnnamed_34 = 1;
pub const KEY_FRAME: C2RustUnnamed_34 = 0;
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
pub type MapCdf = *mut [[aom_cdf_prob; 9]; 5];
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
pub type C2RustUnnamed_35 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_35 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_35 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_35 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_36 = core::ffi::c_uint;
pub const REFRESH_FRAME_CONTEXT_BACKWARD: C2RustUnnamed_36 = 1;
pub const REFRESH_FRAME_CONTEXT_DISABLED: C2RustUnnamed_36 = 0;
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
pub type MACROBLOCK = macroblock;
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
pub const MOD_PACK_BS: C2RustUnnamed_37 = 9;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LargeTileFrameOBU {
    pub tg_hdr_size: uint32_t,
    pub frame_header_size: uint32_t,
}
pub type C2RustUnnamed_37 = core::ffi::c_uint;
pub const NUM_MT_MODULES: C2RustUnnamed_37 = 12;
pub const MOD_AI: C2RustUnnamed_37 = 11;
pub const MOD_FRAME_ENC: C2RustUnnamed_37 = 10;
pub const MOD_LR: C2RustUnnamed_37 = 8;
pub const MOD_CDEF: C2RustUnnamed_37 = 7;
pub const MOD_CDEF_SEARCH: C2RustUnnamed_37 = 6;
pub const MOD_LPF: C2RustUnnamed_37 = 5;
pub const MOD_ENC: C2RustUnnamed_37 = 4;
pub const MOD_GME: C2RustUnnamed_37 = 3;
pub const MOD_TPL: C2RustUnnamed_37 = 2;
pub const MOD_TF: C2RustUnnamed_37 = 1;
pub const MOD_FP: C2RustUnnamed_37 = 0;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const UINT8_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
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
#[inline]
unsafe extern "C-unwind" fn aom_ceil_log2(mut n: core::ffi::c_int) -> core::ffi::c_int {
    if n < 2 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    return get_msb((n - 1 as core::ffi::c_int) as core::ffi::c_uint)
        + 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn init_token_stats(mut token_stats: *mut TOKEN_STATS) {
    (*token_stats).cost = 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn aom_write(
    mut w: *mut aom_writer,
    mut bit: core::ffi::c_int,
    mut probability: core::ffi::c_int,
) {
    let mut p: core::ffi::c_int = 0x7fffff as core::ffi::c_int
        - (probability << 15 as core::ffi::c_int) + probability >> 8 as core::ffi::c_int;
    od_ec_encode_bool_q15(&raw mut (*w).ec, bit, p as core::ffi::c_uint);
}
#[inline]
unsafe extern "C-unwind" fn aom_write_bit(mut w: *mut aom_writer, mut bit: core::ffi::c_int) {
    aom_write(w, bit, 128 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn aom_write_literal(
    mut w: *mut aom_writer,
    mut data: core::ffi::c_int,
    mut bits: core::ffi::c_int,
) {
    let mut bit: core::ffi::c_int = 0;
    bit = bits - 1 as core::ffi::c_int;
    while bit >= 0 as core::ffi::c_int {
        aom_write_bit(w, 1 as core::ffi::c_int & data >> bit);
        bit -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn aom_write_cdf(
    mut w: *mut aom_writer,
    mut symb: core::ffi::c_int,
    mut cdf: *const aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) {
    od_ec_encode_cdf_q15(&raw mut (*w).ec, symb, cdf as *const uint16_t, nsymbs);
}
#[inline]
unsafe extern "C-unwind" fn aom_write_symbol(
    mut w: *mut aom_writer,
    mut symb: core::ffi::c_int,
    mut cdf: *mut aom_cdf_prob,
    mut nsymbs: core::ffi::c_int,
) {
    aom_write_cdf(w, symb, cdf, nsymbs);
    if (*w).allow_update_cdf != 0 {
        update_cdf(cdf, symb as int8_t, nsymbs);
    }
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
#[inline]
unsafe extern "C-unwind" fn mem_put_le16_as_int(
    mut vmem: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
) {
    let mut mem: *mut core::ffi::c_uchar = vmem as *mut core::ffi::c_uchar;
    *mem.offset(0 as core::ffi::c_int as isize) = (val >> 0 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(1 as core::ffi::c_int as isize) = (val >> 8 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
}
#[inline]
unsafe extern "C-unwind" fn mem_put_le24_as_int(
    mut vmem: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
) {
    let mut mem: *mut core::ffi::c_uchar = vmem as *mut core::ffi::c_uchar;
    *mem.offset(0 as core::ffi::c_int as isize) = (val >> 0 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(1 as core::ffi::c_int as isize) = (val >> 8 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(2 as core::ffi::c_int as isize) = (val >> 16 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
}
#[inline]
unsafe extern "C-unwind" fn mem_put_le32_as_int(
    mut vmem: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
) {
    let mut mem: *mut core::ffi::c_uchar = vmem as *mut core::ffi::c_uchar;
    *mem.offset(0 as core::ffi::c_int as isize) = (val >> 0 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(1 as core::ffi::c_int as isize) = (val >> 8 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(2 as core::ffi::c_int as isize) = (val >> 16 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
    *mem.offset(3 as core::ffi::c_int as isize) = (val >> 24 as core::ffi::c_int
        & 0xff as core::ffi::c_int) as core::ffi::c_uchar;
}
pub const CDEF_STRENGTH_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MAX_MIB_MASK: core::ffi::c_int = MAX_MIB_SIZE - 1 as core::ffi::c_int;
pub const MAX_VARTX_DEPTH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PALETTE_MAX_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const PALETTE_MIN_SIZE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_MODE_LF_DELTAS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PROFILE_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const OP_POINTS_CNT_MINUS_1_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const OP_POINTS_IDC_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const SQR_BLOCK_SIZES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PARTITION_PLOFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE_LOG2: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE: core::ffi::c_int = (1 as core::ffi::c_int)
    << CFL_ALPHABET_SIZE_LOG2;
pub const LEVEL_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_ANGLE_DELTA: core::ffi::c_int = 3 as core::ffi::c_int;
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NEWMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << GLOBALMV_OFFSET)
    - 1 as core::ffi::c_int;
pub const GLOBALMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << REFMV_OFFSET - GLOBALMV_OFFSET) - 1 as core::ffi::c_int;
pub const REFMV_CTX_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 8 as core::ffi::c_int - REFMV_OFFSET) - 1 as core::ffi::c_int;
pub const DELTA_Q_SMALL: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DELTA_Q_PROBS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DELTA_LF_SMALL: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DELTA_LF_PROBS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REF_CAT_LEVEL: core::ffi::c_int = 640 as core::ffi::c_int;
pub const REF_FRAMES_LOG2: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUPERRES_SCALE_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUPERRES_SCALE_DENOMINATOR_MIN: core::ffi::c_int = SCALE_NUMERATOR
    + 1 as core::ffi::c_int;
pub const AOM_MIF_LAYER_SPECIFIC: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const AOM_MIF_INSERT_LOCATION_MASK: core::ffi::c_int = 0xf as core::ffi::c_int;
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
static mut partition_context_lookup: [C2RustUnnamed_31; 22] = [
    {
        let mut init = C2RustUnnamed_31 {
            above: 31 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 31 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 30 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 30 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 30 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 28 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 28 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 28 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 24 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 24 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 24 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 16 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 16 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 16 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 0 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 0 as PARTITION_CONTEXT,
            left: 0 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 31 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 28 as PARTITION_CONTEXT,
            left: 31 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 30 as PARTITION_CONTEXT,
            left: 24 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 24 as PARTITION_CONTEXT,
            left: 30 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 28 as PARTITION_CONTEXT,
            left: 16 as PARTITION_CONTEXT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_31 {
            above: 16 as PARTITION_CONTEXT,
            left: 28 as PARTITION_CONTEXT,
        };
        init
    },
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
#[inline]
unsafe extern "C-unwind" fn get_segdata(
    mut seg: *const segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return (*seg).feature_data[segment_id as usize][feature_id as usize]
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_segment_id(
    mut segment_ids: *mut uint8_t,
    mut mi_offset: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
    mut mi_stride: core::ffi::c_int,
    mut segment_id: uint8_t,
) {
    segment_ids = segment_ids.offset(mi_offset as isize);
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_mis {
        memset(
            &raw mut *segment_ids.offset((y * mi_stride) as isize) as *mut uint8_t
                as *mut core::ffi::c_void,
            segment_id as core::ffi::c_int,
            (x_mis as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        y += 1;
    }
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
pub const GM_ALPHA_PREC_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const GM_ABS_ALPHA_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const GM_ALPHA_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - GM_ALPHA_PREC_BITS;
pub const GM_ALPHA_MAX: core::ffi::c_int = (1 as core::ffi::c_int) << GM_ABS_ALPHA_BITS;
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
pub const LOG_SWITCHABLE_FILTERS: core::ffi::c_int = 2 as core::ffi::c_int;
static mut av1_ext_tx_ind: [[core::ffi::c_int; 16]; 6] = [
    [
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
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        2 as core::ffi::c_int,
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
        5 as core::ffi::c_int,
        6 as core::ffi::c_int,
        4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        5 as core::ffi::c_int,
        8 as core::ffi::c_int,
        6 as core::ffi::c_int,
        7 as core::ffi::c_int,
        9 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        7 as core::ffi::c_int,
        8 as core::ffi::c_int,
        9 as core::ffi::c_int,
        12 as core::ffi::c_int,
        10 as core::ffi::c_int,
        11 as core::ffi::c_int,
        13 as core::ffi::c_int,
        14 as core::ffi::c_int,
        15 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        5 as core::ffi::c_int,
        6 as core::ffi::c_int,
    ],
];
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
pub const REF_SCALE_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const REF_NO_SCALE: core::ffi::c_int = (1 as core::ffi::c_int) << REF_SCALE_SHIFT;
pub const REF_INVALID_SCALE: core::ffi::c_int = -(1 as core::ffi::c_int);
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
pub const MAX_DIFFWTD_MASK_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn is_masked_compound_type(
    mut type_0: COMPOUND_TYPE,
) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == COMPOUND_WEDGE as core::ffi::c_int
        || type_0 as core::ffi::c_int == COMPOUND_DIFFWTD as core::ffi::c_int)
        as core::ffi::c_int;
}
static mut fimode_to_intradir: [PREDICTION_MODE; 5] = [
    DC_PRED as core::ffi::c_int as PREDICTION_MODE,
    V_PRED as core::ffi::c_int as PREDICTION_MODE,
    H_PRED as core::ffi::c_int as PREDICTION_MODE,
    D157_PRED as core::ffi::c_int as PREDICTION_MODE,
    DC_PRED as core::ffi::c_int as PREDICTION_MODE,
];
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
static mut av1_num_ext_tx_set: [core::ffi::c_int; 6] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    7 as core::ffi::c_int,
    12 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
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
static mut ext_tx_set_index: [[core::ffi::c_int; 6]; 2] = [
    [
        0 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        -(1 as core::ffi::c_int),
    ],
    [
        0 as core::ffi::c_int,
        3 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        -(1 as core::ffi::c_int),
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
];
#[inline]
unsafe extern "C-unwind" fn get_ext_tx_set(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> core::ffi::c_int {
    let set_type: TxSetType = av1_get_ext_tx_set_type(tx_size, is_inter, use_reduced_set)
        as TxSetType;
    return ext_tx_set_index[is_inter as usize][set_type as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_ext_tx_types(
    mut tx_size: TX_SIZE,
    mut is_inter: core::ffi::c_int,
    mut use_reduced_set: core::ffi::c_int,
) -> core::ffi::c_int {
    let set_type: core::ffi::c_int = av1_get_ext_tx_set_type(
        tx_size,
        is_inter,
        use_reduced_set,
    ) as core::ffi::c_int;
    return av1_num_ext_tx_set[set_type as usize];
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
pub const WIENER_HALFWIN: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SGRPROJ_PARAMS_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN0: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) * 3 as core::ffi::c_int / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MAX0: core::ffi::c_int = SGRPROJ_PRJ_MIN0
    + ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS) - 1 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN1: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MAX1: core::ffi::c_int = SGRPROJ_PRJ_MIN1
    + ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS) - 1 as core::ffi::c_int;
pub const SGRPROJ_PRJ_SUBEXP_K: core::ffi::c_int = 4 as core::ffi::c_int;
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
pub const WIENER_FILT_TAP0_MAXV: core::ffi::c_int = WIENER_FILT_TAP0_MIDV
    - 1 as core::ffi::c_int
    + ((1 as core::ffi::c_int) << WIENER_FILT_TAP0_BITS) / 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_MAXV: core::ffi::c_int = WIENER_FILT_TAP1_MIDV
    - 1 as core::ffi::c_int
    + ((1 as core::ffi::c_int) << WIENER_FILT_TAP1_BITS) / 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP2_MAXV: core::ffi::c_int = WIENER_FILT_TAP2_MIDV
    - 1 as core::ffi::c_int
    + ((1 as core::ffi::c_int) << WIENER_FILT_TAP2_BITS) / 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP0_SUBEXP_K: core::ffi::c_int = 1 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_SUBEXP_K: core::ffi::c_int = 2 as core::ffi::c_int;
pub const WIENER_FILT_TAP2_SUBEXP_K: core::ffi::c_int = 3 as core::ffi::c_int;
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn aom_check_grain_params_equiv(
    pa: *const aom_film_grain_t,
    pb: *const aom_film_grain_t,
) -> core::ffi::c_int {
    if (*pa).apply_grain != (*pb).apply_grain {
        return 0 as core::ffi::c_int;
    }
    if (*pa).num_y_points != (*pb).num_y_points {
        return 0 as core::ffi::c_int;
    }
    if memcmp(
        ((*pa).scaling_points_y).as_ptr() as *const core::ffi::c_void,
        ((*pb).scaling_points_y).as_ptr() as *const core::ffi::c_void,
        (((*pa).num_y_points * 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*pa).num_cb_points != (*pb).num_cb_points {
        return 0 as core::ffi::c_int;
    }
    if memcmp(
        ((*pa).scaling_points_cb).as_ptr() as *const core::ffi::c_void,
        ((*pb).scaling_points_cb).as_ptr() as *const core::ffi::c_void,
        (((*pa).num_cb_points * 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*pa).num_cr_points != (*pb).num_cr_points {
        return 0 as core::ffi::c_int;
    }
    if memcmp(
        ((*pa).scaling_points_cr).as_ptr() as *const core::ffi::c_void,
        ((*pb).scaling_points_cr).as_ptr() as *const core::ffi::c_void,
        (((*pa).num_cr_points * 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*pa).scaling_shift != (*pb).scaling_shift {
        return 0 as core::ffi::c_int;
    }
    if (*pa).ar_coeff_lag != (*pb).ar_coeff_lag {
        return 0 as core::ffi::c_int;
    }
    let num_pos: core::ffi::c_int = 2 as core::ffi::c_int * (*pa).ar_coeff_lag
        * ((*pa).ar_coeff_lag + 1 as core::ffi::c_int);
    if memcmp(
        ((*pa).ar_coeffs_y).as_ptr() as *const core::ffi::c_void,
        ((*pb).ar_coeffs_y).as_ptr() as *const core::ffi::c_void,
        (num_pos as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if memcmp(
        ((*pa).ar_coeffs_cb).as_ptr() as *const core::ffi::c_void,
        ((*pb).ar_coeffs_cb).as_ptr() as *const core::ffi::c_void,
        (num_pos as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if memcmp(
        ((*pa).ar_coeffs_cr).as_ptr() as *const core::ffi::c_void,
        ((*pb).ar_coeffs_cr).as_ptr() as *const core::ffi::c_void,
        (num_pos as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) != 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*pa).ar_coeff_shift != (*pb).ar_coeff_shift {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cb_mult != (*pb).cb_mult {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cb_luma_mult != (*pb).cb_luma_mult {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cb_offset != (*pb).cb_offset {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cr_mult != (*pb).cr_mult {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cr_luma_mult != (*pb).cr_luma_mult {
        return 0 as core::ffi::c_int;
    }
    if (*pa).cr_offset != (*pb).cr_offset {
        return 0 as core::ffi::c_int;
    }
    if (*pa).overlap_flag != (*pb).overlap_flag {
        return 0 as core::ffi::c_int;
    }
    if (*pa).clip_to_restricted_range != (*pb).clip_to_restricted_range {
        return 0 as core::ffi::c_int;
    }
    if (*pa).bit_depth != (*pb).bit_depth {
        return 0 as core::ffi::c_int;
    }
    if (*pa).chroma_scaling_from_luma != (*pb).chroma_scaling_from_luma {
        return 0 as core::ffi::c_int;
    }
    if (*pa).grain_scale_shift != (*pb).grain_scale_shift {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
pub const PRIMARY_REF_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const PRIMARY_REF_NONE: core::ffi::c_int = 7 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_y_mode_cdf(
    mut tile_ctx: *mut FRAME_CONTEXT,
    mut above_mi: *const MB_MODE_INFO,
    mut left_mi: *const MB_MODE_INFO,
) -> *mut aom_cdf_prob {
    let above: PREDICTION_MODE = av1_above_block_mode(above_mi) as PREDICTION_MODE;
    let left: PREDICTION_MODE = av1_left_block_mode(left_mi) as PREDICTION_MODE;
    let above_ctx: core::ffi::c_int = intra_mode_context[above as usize];
    let left_ctx: core::ffi::c_int = intra_mode_context[left as usize];
    return ((*tile_ctx).kf_y_cdf[above_ctx as usize][left_ctx as usize]).as_mut_ptr();
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
pub const __CHAR_BIT__: core::ffi::c_int = 8 as core::ffi::c_int;
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
        let fresh15 = &raw mut *ref_counts
            .offset((*above_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh15 = (*fresh15).wrapping_add(1);
        if has_second_ref(above_mbmi) != 0 {
            let fresh16 = &raw mut *ref_counts
                .offset(
                    (*above_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize,
                );
            *fresh16 = (*fresh16).wrapping_add(1);
        }
    }
    if left_in_image != 0 && is_inter_block(left_mbmi) != 0 {
        let fresh17 = &raw mut *ref_counts
            .offset((*left_mbmi).ref_frame[0 as core::ffi::c_int as usize] as isize);
        *fresh17 = (*fresh17).wrapping_add(1);
        if has_second_ref(left_mbmi) != 0 {
            let fresh18 = &raw mut *ref_counts
                .offset((*left_mbmi).ref_frame[1 as core::ffi::c_int as usize] as isize);
            *fresh18 = (*fresh18).wrapping_add(1);
        }
    }
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
unsafe extern "C-unwind" fn av1_get_spatial_seg_pred(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    mut cdf_index: *mut core::ffi::c_int,
    mut skip_over4x4: core::ffi::c_int,
) -> uint8_t {
    let step_size: core::ffi::c_int = if skip_over4x4 != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut prev_ul: uint8_t = UINT8_MAX as uint8_t;
    let mut prev_l: uint8_t = UINT8_MAX as uint8_t;
    let mut prev_u: uint8_t = UINT8_MAX as uint8_t;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mut seg_map: *const uint8_t = (*(*cm).cur_frame).seg_map;
    if (*xd).up_available as core::ffi::c_int != 0
        && (*xd).left_available as core::ffi::c_int != 0
    {
        prev_ul = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - step_size,
            mi_col - step_size,
        );
    }
    if (*xd).up_available {
        prev_u = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - step_size,
            mi_col - 0 as core::ffi::c_int,
        );
    }
    if (*xd).left_available {
        prev_l = get_segment_id(
            mi_params,
            seg_map,
            BLOCK_4X4,
            mi_row - 0 as core::ffi::c_int,
            mi_col - step_size,
        );
    }
    if !(!(prev_ul as core::ffi::c_int != 255 as core::ffi::c_int)
        || prev_u as core::ffi::c_int != 255 as core::ffi::c_int
            && prev_l as core::ffi::c_int != 255 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"av1_get_spatial_seg_pred\0"))
                .as_ptr(),
            b"pred_common.h\0" as *const u8 as *const core::ffi::c_char,
            73 as core::ffi::c_int,
            b"IMPLIES(prev_ul != UINT8_MAX, prev_u != UINT8_MAX && prev_l != UINT8_MAX)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if prev_ul as core::ffi::c_int == UINT8_MAX {
        *cdf_index = 0 as core::ffi::c_int;
    } else if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int
        && prev_ul as core::ffi::c_int == prev_l as core::ffi::c_int
    {
        *cdf_index = 2 as core::ffi::c_int;
    } else if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int
        || prev_ul as core::ffi::c_int == prev_l as core::ffi::c_int
        || prev_u as core::ffi::c_int == prev_l as core::ffi::c_int
    {
        *cdf_index = 1 as core::ffi::c_int;
    } else {
        *cdf_index = 0 as core::ffi::c_int;
    }
    if prev_u as core::ffi::c_int == UINT8_MAX {
        return (if prev_l as core::ffi::c_int == UINT8_MAX {
            0 as core::ffi::c_int
        } else {
            prev_l as core::ffi::c_int
        }) as uint8_t;
    }
    if prev_l as core::ffi::c_int == UINT8_MAX {
        return prev_u;
    }
    return (if prev_ul as core::ffi::c_int == prev_u as core::ffi::c_int {
        prev_u as core::ffi::c_int
    } else {
        prev_l as core::ffi::c_int
    }) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_pred_context_seg_id(mut xd: *const MACROBLOCKD) -> uint8_t {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_sip: core::ffi::c_int = if !above_mi.is_null() {
        (*above_mi).seg_id_predicted() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let left_sip: core::ffi::c_int = if !left_mi.is_null() {
        (*left_mi).seg_id_predicted() as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return (above_sip + left_sip) as uint8_t;
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
unsafe extern "C-unwind" fn av1_get_pred_cdf_seg_id(
    mut segp: *mut segmentation_probs,
    mut xd: *const MACROBLOCKD,
) -> *mut aom_cdf_prob {
    return ((*segp).pred_cdf[av1_get_pred_context_seg_id(xd) as usize]).as_mut_ptr();
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
unsafe extern "C-unwind" fn av1_get_palette_bsize_ctx(
    mut bsize: BLOCK_SIZE,
) -> core::ffi::c_int {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_get_palette_bsize_ctx\0"))
                .as_ptr(),
            b"pred_common.h\0" as *const u8 as *const core::ffi::c_char,
            193 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return num_pels_log2_lookup[bsize as usize] as core::ffi::c_int
        - num_pels_log2_lookup[BLOCK_8X8 as core::ffi::c_int as usize]
            as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_palette_mode_ctx(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut ctx: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mi.is_null() {
        ctx
            += ((*above_mi)
                .palette_mode_info
                .palette_size[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    if !left_mi.is_null() {
        ctx
            += ((*left_mi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize]
                as core::ffi::c_int > 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    return ctx;
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
pub const MAX_WEDGE_TYPES: core::ffi::c_int = 16 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_is_directional_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= V_PRED as core::ffi::c_int
        && mode as core::ffi::c_int <= D67_PRED as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_use_angle_delta(mut bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_filter_intra_allowed_bsize(
    cm: *const AV1_COMMON,
    mut bs: BLOCK_SIZE,
) -> core::ffi::c_int {
    if (*(*cm).seq_params).enable_filter_intra == 0
        || bs as core::ffi::c_int == BLOCK_INVALID as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    return (block_size_wide[bs as usize] as core::ffi::c_int <= 32 as core::ffi::c_int
        && block_size_high[bs as usize] as core::ffi::c_int <= 32 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_filter_intra_allowed(
    cm: *const AV1_COMMON,
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int
        && (*mbmi).palette_mode_info.palette_size[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == 0 as core::ffi::c_int
        && av1_filter_intra_allowed_bsize(cm, (*mbmi).bsize) != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_resize_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).superres_upscaled_width != (*cm).render_width
        || (*cm).superres_upscaled_height != (*cm).render_height) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn encode_show_existing_frame(
    mut cm: *const AV1_COMMON,
) -> core::ffi::c_int {
    return ((*cm).show_existing_frame != 0
        && (!(*cm).features.error_resilient_mode
            || (*cm).current_frame.frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int)) as core::ffi::c_int;
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
pub const SETUP_TIME_OH_CONST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const JOB_DISP_TIME_OH_CONST: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn write_uniform(
    mut w: *mut aom_writer,
    mut n: core::ffi::c_int,
    mut v: core::ffi::c_int,
) {
    let l: core::ffi::c_int = get_unsigned_bits(n as core::ffi::c_uint)
        as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n;
    if l == 0 as core::ffi::c_int {
        return;
    }
    if v < m {
        aom_write_literal(w, v, l - 1 as core::ffi::c_int);
    } else {
        aom_write_literal(
            w,
            m + (v - m >> 1 as core::ffi::c_int),
            l - 1 as core::ffi::c_int,
        );
        aom_write_literal(w, v - m & 1 as core::ffi::c_int, 1 as core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C-unwind" fn write_intra_y_mode_kf(
    mut frame_ctx: *mut FRAME_CONTEXT,
    mut mi: *const MB_MODE_INFO,
    mut above_mi: *const MB_MODE_INFO,
    mut left_mi: *const MB_MODE_INFO,
    mut mode: PREDICTION_MODE,
    mut w: *mut aom_writer,
) {
    if (is_intrabc_block(mi) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"write_intra_y_mode_kf\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            81 as core::ffi::c_int,
            b"!is_intrabc_block(mi)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_write_symbol(
        w,
        mode as core::ffi::c_int,
        get_y_mode_cdf(frame_ctx, above_mi, left_mi),
        INTRA_MODES as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_inter_mode(
    mut w: *mut aom_writer,
    mut mode: PREDICTION_MODE,
    mut ec_ctx: *mut FRAME_CONTEXT,
    mode_ctx: int16_t,
) {
    let newmv_ctx: int16_t = (mode_ctx as core::ffi::c_int & NEWMV_CTX_MASK) as int16_t;
    aom_write_symbol(
        w,
        (mode as core::ffi::c_int != NEWMV as core::ffi::c_int) as core::ffi::c_int,
        ((*ec_ctx).newmv_cdf[newmv_ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    );
    if mode as core::ffi::c_int != NEWMV as core::ffi::c_int {
        let zeromv_ctx: int16_t = (mode_ctx as core::ffi::c_int >> GLOBALMV_OFFSET
            & GLOBALMV_CTX_MASK) as int16_t;
        aom_write_symbol(
            w,
            (mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int)
                as core::ffi::c_int,
            ((*ec_ctx).zeromv_cdf[zeromv_ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if mode as core::ffi::c_int != GLOBALMV as core::ffi::c_int {
            let mut refmv_ctx: int16_t = (mode_ctx as core::ffi::c_int >> REFMV_OFFSET
                & REFMV_CTX_MASK) as int16_t;
            aom_write_symbol(
                w,
                (mode as core::ffi::c_int != NEARESTMV as core::ffi::c_int)
                    as core::ffi::c_int,
                ((*ec_ctx).refmv_cdf[refmv_ctx as usize]).as_mut_ptr(),
                2 as core::ffi::c_int,
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_drl_idx(
    mut ec_ctx: *mut FRAME_CONTEXT,
    mut mbmi: *const MB_MODE_INFO,
    mut mbmi_ext_frame: *const MB_MODE_INFO_EXT_FRAME,
    mut w: *mut aom_writer,
) {
    if !(((*mbmi).ref_mv_idx() as core::ffi::c_int) < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_drl_idx\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            110 as core::ffi::c_int,
            b"mbmi->ref_mv_idx < 3\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let new_mv: core::ffi::c_int = ((*mbmi).mode as core::ffi::c_int
        == NEWMV as core::ffi::c_int
        || (*mbmi).mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int)
        as core::ffi::c_int;
    if new_mv != 0 {
        let mut idx: core::ffi::c_int = 0;
        idx = 0 as core::ffi::c_int;
        while idx < 2 as core::ffi::c_int {
            if (*mbmi_ext_frame).ref_mv_count as core::ffi::c_int
                > idx + 1 as core::ffi::c_int
            {
                let mut drl_ctx: uint8_t = av1_drl_ctx(
                    ((*mbmi_ext_frame).weight).as_ptr(),
                    idx,
                );
                aom_write_symbol(
                    w,
                    ((*mbmi).ref_mv_idx() as core::ffi::c_int != idx)
                        as core::ffi::c_int,
                    ((*ec_ctx).drl_cdf[drl_ctx as usize]).as_mut_ptr(),
                    2 as core::ffi::c_int,
                );
                if (*mbmi).ref_mv_idx() as core::ffi::c_int == idx {
                    return;
                }
            }
            idx += 1;
        }
        return;
    }
    if have_nearmv_in_inter_mode((*mbmi).mode) != 0 {
        let mut idx_0: core::ffi::c_int = 0;
        idx_0 = 1 as core::ffi::c_int;
        while idx_0 < 3 as core::ffi::c_int {
            if (*mbmi_ext_frame).ref_mv_count as core::ffi::c_int
                > idx_0 + 1 as core::ffi::c_int
            {
                let mut drl_ctx_0: uint8_t = av1_drl_ctx(
                    ((*mbmi_ext_frame).weight).as_ptr(),
                    idx_0,
                );
                aom_write_symbol(
                    w,
                    ((*mbmi).ref_mv_idx() as core::ffi::c_int
                        != idx_0 - 1 as core::ffi::c_int) as core::ffi::c_int,
                    ((*ec_ctx).drl_cdf[drl_ctx_0 as usize]).as_mut_ptr(),
                    2 as core::ffi::c_int,
                );
                if (*mbmi).ref_mv_idx() as core::ffi::c_int
                    == idx_0 - 1 as core::ffi::c_int
                {
                    return;
                }
            }
            idx_0 += 1;
        }
        return;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_inter_compound_mode(
    mut xd: *mut MACROBLOCKD,
    mut w: *mut aom_writer,
    mut mode: PREDICTION_MODE,
    mode_ctx: int16_t,
) {
    if (is_inter_compound_mode(mode) == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"write_inter_compound_mode\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            145 as core::ffi::c_int,
            b"is_inter_compound_mode(mode)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_write_symbol(
        w,
        (mode as core::ffi::c_int - NEAREST_NEARESTMV as core::ffi::c_int) as uint8_t
            as core::ffi::c_int,
        ((*(*xd).tile_ctx).inter_compound_mode_cdf[mode_ctx as usize]).as_mut_ptr(),
        1 as core::ffi::c_int + NEW_NEWMV as core::ffi::c_int
            - NEAREST_NEARESTMV as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_tx_size_vartx(
    mut xd: *mut MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
    mut tx_size: TX_SIZE,
    mut depth: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let max_blocks_high: core::ffi::c_int = max_block_high(
        xd,
        (*mbmi).bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(
        xd,
        (*mbmi).bsize,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
    if depth == MAX_VARTX_DEPTH {
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
    let txb_size_index: core::ffi::c_int = av1_get_txb_size_index(
        (*mbmi).bsize,
        blk_row,
        blk_col,
    ) as core::ffi::c_int;
    let write_txfm_partition: core::ffi::c_int = (tx_size as core::ffi::c_int
        == (*mbmi).inter_tx_size[txb_size_index as usize] as core::ffi::c_int)
        as core::ffi::c_int;
    if write_txfm_partition != 0 {
        aom_write_symbol(
            w,
            0 as core::ffi::c_int,
            ((*ec_ctx).txfm_partition_cdf[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
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
        aom_write_symbol(
            w,
            1 as core::ffi::c_int,
            ((*ec_ctx).txfm_partition_cdf[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if sub_txs as core::ffi::c_int == TX_4X4 as core::ffi::c_int {
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
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"write_tx_size_vartx\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                193 as core::ffi::c_int,
                b"bsw > 0 && bsh > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < tx_size_high_unit[tx_size as usize] {
            let offsetr: core::ffi::c_int = blk_row + row;
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < tx_size_wide_unit[tx_size as usize] {
                let offsetc: core::ffi::c_int = blk_col + col;
                write_tx_size_vartx(
                    xd,
                    mbmi,
                    sub_txs,
                    depth + 1 as core::ffi::c_int,
                    offsetr,
                    offsetc,
                    w,
                );
                col += bsw;
            }
            row += bsh;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_selected_tx_size(
    mut xd: *const MACROBLOCKD,
    mut w: *mut aom_writer,
) {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if block_signals_txsize(bsize) != 0 {
        let tx_size: TX_SIZE = (*mbmi).tx_size;
        let tx_size_ctx: core::ffi::c_int = get_tx_size_context(xd) as core::ffi::c_int;
        let depth: core::ffi::c_int = tx_size_to_depth(tx_size, bsize)
            as core::ffi::c_int;
        let max_depths: core::ffi::c_int = bsize_to_max_depth(bsize) as core::ffi::c_int;
        let tx_size_cat: int32_t = bsize_to_tx_size_cat(bsize) as int32_t;
        if !(depth >= 0 as core::ffi::c_int && depth <= max_depths) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"write_selected_tx_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                216 as core::ffi::c_int,
                b"depth >= 0 && depth <= max_depths\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if (is_inter_block(mbmi) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"write_selected_tx_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                217 as core::ffi::c_int,
                b"!is_inter_block(mbmi)\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(is_rect_tx(tx_size) == 0 || is_rect_tx_allowed(xd, mbmi) != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"write_selected_tx_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                218 as core::ffi::c_int,
                b"IMPLIES(is_rect_tx(tx_size), is_rect_tx_allowed(xd, mbmi))\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        aom_write_symbol(
            w,
            depth,
            ((*ec_ctx).tx_size_cdf[tx_size_cat as usize][tx_size_ctx as usize])
                .as_mut_ptr(),
            max_depths + 1 as core::ffi::c_int,
        );
    }
}
unsafe extern "C-unwind" fn write_skip(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut segment_id: uint8_t,
    mut mi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
) -> core::ffi::c_int {
    if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        return 1 as core::ffi::c_int
    } else {
        let skip_txfm: core::ffi::c_int = (*mi).skip_txfm as core::ffi::c_int;
        let ctx: core::ffi::c_int = av1_get_skip_txfm_context(xd) as core::ffi::c_int;
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        aom_write_symbol(
            w,
            skip_txfm,
            ((*ec_ctx).skip_txfm_cdfs[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        return skip_txfm;
    };
}
unsafe extern "C-unwind" fn write_skip_mode(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut segment_id: uint8_t,
    mut mi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
) -> core::ffi::c_int {
    if (*cm).current_frame.skip_mode_info.skip_mode_flag == 0 {
        return 0 as core::ffi::c_int;
    }
    if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        return 0 as core::ffi::c_int;
    }
    let skip_mode: core::ffi::c_int = (*mi).skip_mode() as core::ffi::c_int;
    if is_comp_ref_allowed((*mi).bsize) == 0 {
        if (skip_mode != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_skip_mode\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                248 as core::ffi::c_int,
                b"!skip_mode\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return 0 as core::ffi::c_int;
    }
    if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        || segfeature_active(
            &(*cm).seg,
            segment_id,
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        if (skip_mode != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_skip_mode\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                256 as core::ffi::c_int,
                b"!skip_mode\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return 0 as core::ffi::c_int;
    }
    let ctx: core::ffi::c_int = av1_get_skip_mode_context(xd) as core::ffi::c_int;
    aom_write_symbol(
        w,
        skip_mode,
        ((*(*xd).tile_ctx).skip_mode_cdfs[ctx as usize]).as_mut_ptr(),
        2 as core::ffi::c_int,
    );
    return skip_mode;
}
#[inline]
unsafe extern "C-unwind" fn write_is_inter(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut segment_id: uint8_t,
    mut w: *mut aom_writer,
    is_inter: core::ffi::c_int,
) {
    if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) == 0
    {
        if segfeature_active(
            &(*cm).seg,
            segment_id,
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
        {
            if (is_inter == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 15],
                        [core::ffi::c_char; 15],
                    >(*b"write_is_inter\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    269 as core::ffi::c_int,
                    b"is_inter\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            return;
        }
        let ctx: core::ffi::c_int = av1_get_intra_inter_context(xd) as core::ffi::c_int;
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        aom_write_symbol(
            w,
            is_inter,
            ((*ec_ctx).intra_inter_cdf[ctx as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_motion_mode(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut mbmi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
) {
    let mut last_motion_mode_allowed: MOTION_MODE = (if (*cm)
        .features
        .switchable_motion_mode as core::ffi::c_int != 0
    {
        motion_mode_allowed(
            ((*cm).global_motion).as_ptr(),
            xd,
            mbmi,
            (*cm).features.allow_warped_motion as core::ffi::c_int,
        ) as core::ffi::c_int
    } else {
        SIMPLE_TRANSLATION as core::ffi::c_int
    }) as MOTION_MODE;
    if !((*mbmi).motion_mode as core::ffi::c_int
        <= last_motion_mode_allowed as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"write_motion_mode\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            285 as core::ffi::c_int,
            b"mbmi->motion_mode <= last_motion_mode_allowed\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    match last_motion_mode_allowed as core::ffi::c_int {
        0 => {}
        1 => {
            aom_write_symbol(
                w,
                ((*mbmi).motion_mode as core::ffi::c_int
                    == OBMC_CAUSAL as core::ffi::c_int) as core::ffi::c_int,
                ((*(*xd).tile_ctx).obmc_cdf[(*mbmi).bsize as usize]).as_mut_ptr(),
                2 as core::ffi::c_int,
            );
        }
        _ => {
            aom_write_symbol(
                w,
                (*mbmi).motion_mode as core::ffi::c_int,
                ((*(*xd).tile_ctx).motion_mode_cdf[(*mbmi).bsize as usize]).as_mut_ptr(),
                MOTION_MODES as core::ffi::c_int,
            );
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_delta_qindex(
    mut xd: *const MACROBLOCKD,
    mut delta_qindex: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let mut sign: core::ffi::c_int = (delta_qindex < 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut abs_0: core::ffi::c_int = if sign != 0 {
        -delta_qindex
    } else {
        delta_qindex
    };
    let mut rem_bits: core::ffi::c_int = 0;
    let mut thr: core::ffi::c_int = 0;
    let mut smallval: core::ffi::c_int = if abs_0 < DELTA_Q_SMALL {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    aom_write_symbol(
        w,
        if abs_0 < 3 as core::ffi::c_int { abs_0 } else { 3 as core::ffi::c_int },
        ((*ec_ctx).delta_q_cdf).as_mut_ptr(),
        DELTA_Q_PROBS + 1 as core::ffi::c_int,
    );
    if smallval == 0 {
        rem_bits = get_msb((abs_0 - 1 as core::ffi::c_int) as core::ffi::c_uint);
        thr = ((1 as core::ffi::c_int) << rem_bits) + 1 as core::ffi::c_int;
        aom_write_literal(w, rem_bits - 1 as core::ffi::c_int, 3 as core::ffi::c_int);
        aom_write_literal(w, abs_0 - thr, rem_bits);
    }
    if abs_0 > 0 as core::ffi::c_int {
        aom_write_bit(w, sign);
    }
}
#[inline]
unsafe extern "C-unwind" fn write_delta_lflevel(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut lf_id: core::ffi::c_int,
    mut delta_lflevel: core::ffi::c_int,
    mut delta_lf_multi: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let mut sign: core::ffi::c_int = (delta_lflevel < 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut abs_0: core::ffi::c_int = if sign != 0 {
        -delta_lflevel
    } else {
        delta_lflevel
    };
    let mut rem_bits: core::ffi::c_int = 0;
    let mut thr: core::ffi::c_int = 0;
    let mut smallval: core::ffi::c_int = if abs_0 < DELTA_LF_SMALL {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if delta_lf_multi != 0 {
        if !(lf_id >= 0 as core::ffi::c_int
            && lf_id
                < (if av1_num_planes(cm) > 1 as core::ffi::c_int {
                    4 as core::ffi::c_int
                } else {
                    4 as core::ffi::c_int - 2 as core::ffi::c_int
                })) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"write_delta_lflevel\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                334 as core::ffi::c_int,
                b"lf_id >= 0 && lf_id < (av1_num_planes(cm) > 1 ? FRAME_LF_COUNT : FRAME_LF_COUNT - 2)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        aom_write_symbol(
            w,
            if abs_0 < 3 as core::ffi::c_int { abs_0 } else { 3 as core::ffi::c_int },
            ((*ec_ctx).delta_lf_multi_cdf[lf_id as usize]).as_mut_ptr(),
            DELTA_LF_PROBS + 1 as core::ffi::c_int,
        );
    } else {
        aom_write_symbol(
            w,
            if abs_0 < 3 as core::ffi::c_int { abs_0 } else { 3 as core::ffi::c_int },
            ((*ec_ctx).delta_lf_cdf).as_mut_ptr(),
            DELTA_LF_PROBS + 1 as core::ffi::c_int,
        );
    }
    if smallval == 0 {
        rem_bits = get_msb((abs_0 - 1 as core::ffi::c_int) as core::ffi::c_uint);
        thr = ((1 as core::ffi::c_int) << rem_bits) + 1 as core::ffi::c_int;
        aom_write_literal(w, rem_bits - 1 as core::ffi::c_int, 3 as core::ffi::c_int);
        aom_write_literal(w, abs_0 - thr, rem_bits);
    }
    if abs_0 > 0 as core::ffi::c_int {
        aom_write_bit(w, sign);
    }
}
#[inline]
unsafe extern "C-unwind" fn pack_map_tokens(
    mut w: *mut aom_writer,
    mut tp: *mut *const TokenExtra,
    mut n: core::ffi::c_int,
    mut num: core::ffi::c_int,
    mut map_pb_cdf: MapCdf,
) {
    let mut p: *const TokenExtra = *tp;
    let palette_size_idx: core::ffi::c_int = n - PALETTE_MIN_SIZE;
    write_uniform(w, n, (*p).token() as core::ffi::c_int);
    p = p.offset(1);
    num -= 1;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num {
        if !((*p).color_ctx() as core::ffi::c_int >= 0 as core::ffi::c_int
            && ((*p).color_ctx() as core::ffi::c_int) < 5 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"pack_map_tokens\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                362 as core::ffi::c_int,
                b"(p->color_ctx >= 0) && (p->color_ctx < PALETTE_COLOR_INDEX_CONTEXTS)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut color_map_cdf: *mut aom_cdf_prob = ((*map_pb_cdf
            .offset(palette_size_idx as isize))[(*p).color_ctx() as usize])
            .as_mut_ptr();
        aom_write_symbol(w, (*p).token() as core::ffi::c_int, color_map_cdf, n);
        p = p.offset(1);
        i += 1;
    }
    *tp = p;
}
#[inline]
unsafe extern "C-unwind" fn pack_txb_tokens(
    mut w: *mut aom_writer,
    mut cm: *mut AV1_COMMON,
    x: *mut MACROBLOCK,
    mut tp: *mut *const TokenExtra,
    tok_end: *const TokenExtra,
    mut xd: *mut MACROBLOCKD,
    mut mbmi: *mut MB_MODE_INFO,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut bit_depth: aom_bit_depth_t,
    mut block: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut token_stats: *mut TOKEN_STATS,
) {
    let max_blocks_high: core::ffi::c_int = max_block_high(xd, plane_bsize, plane)
        as core::ffi::c_int;
    let max_blocks_wide: core::ffi::c_int = max_block_wide(xd, plane_bsize, plane)
        as core::ffi::c_int;
    if blk_row >= max_blocks_high || blk_col >= max_blocks_wide {
        return;
    }
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
    if tx_size as core::ffi::c_int == plane_tx_size as core::ffi::c_int || plane != 0 {
        av1_write_coeffs_txb(cm, x, w, blk_row, blk_col, plane, block, tx_size);
    } else {
        let sub_txs: TX_SIZE = sub_tx_size_map[tx_size as usize];
        let bsw: core::ffi::c_int = tx_size_wide_unit[sub_txs as usize];
        let bsh: core::ffi::c_int = tx_size_high_unit[sub_txs as usize];
        let step: core::ffi::c_int = bsh * bsw;
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
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"pack_txb_tokens\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                404 as core::ffi::c_int,
                b"bsw > 0 && bsh > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
        while r < row_end {
            let offsetr: core::ffi::c_int = blk_row + r;
            let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
            while c < col_end {
                let offsetc: core::ffi::c_int = blk_col + c;
                pack_txb_tokens(
                    w,
                    cm,
                    x,
                    tp,
                    tok_end,
                    xd,
                    mbmi,
                    plane,
                    plane_bsize,
                    bit_depth,
                    block,
                    offsetr,
                    offsetc,
                    sub_txs,
                    token_stats,
                );
                block += step;
                c += bsw;
            }
            r += bsh;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn set_spatial_segment_id(
    mi_params: *const CommonModeInfoParams,
    mut segment_ids: *mut uint8_t,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut segment_id: uint8_t,
) {
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
    let mi_stride: core::ffi::c_int = (*mi_params).mi_cols;
    set_segment_id(segment_ids, mi_offset, xmis, ymis, mi_stride, segment_id);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_neg_interleave(
    mut x: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut max: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(x < max) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_neg_interleave\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            434 as core::ffi::c_int,
            b"x < max\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let diff: core::ffi::c_int = x - ref_0;
    if ref_0 == 0 {
        return x;
    }
    if ref_0 >= max - 1 as core::ffi::c_int {
        return -x + max - 1 as core::ffi::c_int;
    }
    if 2 as core::ffi::c_int * ref_0 < max {
        if abs(diff) <= ref_0 {
            if diff > 0 as core::ffi::c_int {
                return (diff << 1 as core::ffi::c_int) - 1 as core::ffi::c_int
            } else {
                return -diff << 1 as core::ffi::c_int
            }
        }
        return x;
    } else {
        if abs(diff) < max - ref_0 {
            if diff > 0 as core::ffi::c_int {
                return (diff << 1 as core::ffi::c_int) - 1 as core::ffi::c_int
            } else {
                return -diff << 1 as core::ffi::c_int
            }
        }
        return max - x - 1 as core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C-unwind" fn write_segment_id(
    mut cpi: *mut AV1_COMP,
    xd: *mut MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
    mut seg: *const segmentation,
    mut segp: *mut segmentation_probs,
    mut skip_txfm: core::ffi::c_int,
) {
    if (*seg).enabled == 0 || (*seg).update_map == 0 {
        return;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut cdf_num: core::ffi::c_int = 0;
    let pred: uint8_t = av1_get_spatial_seg_pred(
        cm,
        xd,
        &raw mut cdf_num,
        (*(*cpi).cyclic_refresh).skip_over4x4,
    ) as uint8_t;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    if skip_txfm != 0 {
        if !(is_inter_block(mbmi) != 0 || !(*cpi).enc_seg.has_lossless_segment)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"write_segment_id\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                476 as core::ffi::c_int,
                b"is_inter_block(mbmi) || !cpi->enc_seg.has_lossless_segment\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        set_spatial_segment_id(
            &raw mut (*cm).mi_params,
            (*(*cm).cur_frame).seg_map,
            (*mbmi).bsize,
            mi_row,
            mi_col,
            pred,
        );
        set_spatial_segment_id(
            &raw mut (*cm).mi_params,
            (*cpi).enc_seg.map,
            (*mbmi).bsize,
            mi_row,
            mi_col,
            pred,
        );
        let fresh19 = &raw mut *(mbmi as *mut MB_MODE_INFO);
        (*fresh19).set_segment_id(pred as uint8_t);
        return;
    }
    let coded_id: core::ffi::c_int = av1_neg_interleave(
        (*mbmi).segment_id() as core::ffi::c_int,
        pred as core::ffi::c_int,
        (*seg).last_active_segid + 1 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut pred_cdf: *mut aom_cdf_prob = ((*segp)
        .spatial_pred_seg_cdf[cdf_num as usize])
        .as_mut_ptr();
    aom_write_symbol(w, coded_id, pred_cdf, MAX_SEGMENTS);
    set_spatial_segment_id(
        &raw mut (*cm).mi_params,
        (*(*cm).cur_frame).seg_map,
        (*mbmi).bsize,
        mi_row,
        mi_col,
        (*mbmi).segment_id(),
    );
}
#[inline]
unsafe extern "C-unwind" fn write_ref_frames(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut w: *mut aom_writer,
) {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let segment_id: uint8_t = (*mbmi).segment_id();
    if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
    {
        if (is_compound != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"write_ref_frames\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                508 as core::ffi::c_int,
                b"!is_compound\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == get_segdata(
                &(*cm).seg,
                segment_id as core::ffi::c_int,
                SEG_LVL_REF_FRAME as core::ffi::c_int as SEG_LVL_FEATURES,
            )) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"write_ref_frames\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                510 as core::ffi::c_int,
                b"mbmi->ref_frame[0] == get_segdata(&cm->seg, segment_id, SEG_LVL_REF_FRAME)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    } else if segfeature_active(
        &(*cm).seg,
        segment_id,
        SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
    ) != 0
        || segfeature_active(
            &(*cm).seg,
            segment_id,
            SEG_LVL_GLOBALMV as core::ffi::c_int as SEG_LVL_FEATURES,
        ) != 0
    {
        if (is_compound != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"write_ref_frames\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                513 as core::ffi::c_int,
                b"!is_compound\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == LAST_FRAME as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"write_ref_frames\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                514 as core::ffi::c_int,
                b"mbmi->ref_frame[0] == LAST_FRAME\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    } else {
        if (*cm).current_frame.reference_mode as core::ffi::c_int
            == REFERENCE_MODE_SELECT as core::ffi::c_int
        {
            if is_comp_ref_allowed((*mbmi).bsize) != 0 {
                aom_write_symbol(
                    w,
                    is_compound,
                    av1_get_reference_mode_cdf(xd),
                    2 as core::ffi::c_int,
                );
            }
        } else {
            if !((is_compound == 0) as core::ffi::c_int
                == ((*cm).current_frame.reference_mode as core::ffi::c_int
                    == SINGLE_REFERENCE as core::ffi::c_int) as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"write_ref_frames\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    523 as core::ffi::c_int,
                    b"(!is_compound) == (cm->current_frame.reference_mode == SINGLE_REFERENCE)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        if is_compound != 0 {
            let comp_ref_type: COMP_REFERENCE_TYPE = (if has_uni_comp_refs(mbmi) != 0 {
                UNIDIR_COMP_REFERENCE as core::ffi::c_int
            } else {
                BIDIR_COMP_REFERENCE as core::ffi::c_int
            }) as COMP_REFERENCE_TYPE;
            aom_write_symbol(
                w,
                comp_ref_type as core::ffi::c_int,
                av1_get_comp_reference_type_cdf(xd),
                2 as core::ffi::c_int,
            );
            if comp_ref_type as core::ffi::c_int
                == UNIDIR_COMP_REFERENCE as core::ffi::c_int
            {
                let bit: core::ffi::c_int = ((*mbmi)
                    .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    bit,
                    av1_get_pred_cdf_uni_comp_ref_p(xd),
                    2 as core::ffi::c_int,
                );
                if bit == 0 {
                    if !((*mbmi).ref_frame[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int == LAST_FRAME as core::ffi::c_int)
                        as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 17],
                                [core::ffi::c_char; 17],
                            >(*b"write_ref_frames\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            538 as core::ffi::c_int,
                            b"mbmi->ref_frame[0] == LAST_FRAME\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                    let bit1: core::ffi::c_int = ((*mbmi)
                        .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        == LAST3_FRAME as core::ffi::c_int
                        || (*mbmi).ref_frame[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int)
                        as core::ffi::c_int;
                    aom_write_symbol(
                        w,
                        bit1,
                        av1_get_pred_cdf_uni_comp_ref_p1(xd),
                        2 as core::ffi::c_int,
                    );
                    if bit1 != 0 {
                        let bit2: core::ffi::c_int = ((*mbmi)
                            .ref_frame[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int)
                            as core::ffi::c_int;
                        aom_write_symbol(
                            w,
                            bit2,
                            av1_get_pred_cdf_uni_comp_ref_p2(xd),
                            2 as core::ffi::c_int,
                        );
                    }
                } else {
                    if !((*mbmi).ref_frame[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int == ALTREF_FRAME as core::ffi::c_int)
                        as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 17],
                                [core::ffi::c_char; 17],
                            >(*b"write_ref_frames\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            547 as core::ffi::c_int,
                            b"mbmi->ref_frame[1] == ALTREF_FRAME\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                }
                return;
            }
            if !(comp_ref_type as core::ffi::c_int
                == BIDIR_COMP_REFERENCE as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"write_ref_frames\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    553 as core::ffi::c_int,
                    b"comp_ref_type == BIDIR_COMP_REFERENCE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let bit_0: core::ffi::c_int = ((*mbmi)
                .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == GOLDEN_FRAME as core::ffi::c_int
                || (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == LAST3_FRAME as core::ffi::c_int) as core::ffi::c_int;
            aom_write_symbol(
                w,
                bit_0,
                av1_get_pred_cdf_comp_ref_p(xd),
                2 as core::ffi::c_int,
            );
            if bit_0 == 0 {
                let bit1_0: core::ffi::c_int = ((*mbmi)
                    .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == LAST2_FRAME as core::ffi::c_int) as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    bit1_0,
                    av1_get_pred_cdf_comp_ref_p1(xd),
                    2 as core::ffi::c_int,
                );
            } else {
                let bit2_0: core::ffi::c_int = ((*mbmi)
                    .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == GOLDEN_FRAME as core::ffi::c_int) as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    bit2_0,
                    av1_get_pred_cdf_comp_ref_p2(xd),
                    2 as core::ffi::c_int,
                );
            }
            let bit_bwd: core::ffi::c_int = ((*mbmi)
                .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
            aom_write_symbol(
                w,
                bit_bwd,
                av1_get_pred_cdf_comp_bwdref_p(xd),
                2 as core::ffi::c_int,
            );
            if bit_bwd == 0 {
                aom_write_symbol(
                    w,
                    ((*mbmi).ref_frame[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int == ALTREF2_FRAME as core::ffi::c_int)
                        as core::ffi::c_int,
                    av1_get_pred_cdf_comp_bwdref_p1(xd),
                    2 as core::ffi::c_int,
                );
            }
        } else {
            let bit0: core::ffi::c_int = ((*mbmi)
                .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                <= ALTREF_FRAME as core::ffi::c_int
                && (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
            aom_write_symbol(
                w,
                bit0,
                av1_get_pred_cdf_single_ref_p1(xd),
                2 as core::ffi::c_int,
            );
            if bit0 != 0 {
                let bit1_1: core::ffi::c_int = ((*mbmi)
                    .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    bit1_1,
                    av1_get_pred_cdf_single_ref_p2(xd),
                    2 as core::ffi::c_int,
                );
                if bit1_1 == 0 {
                    aom_write_symbol(
                        w,
                        ((*mbmi).ref_frame[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int == ALTREF2_FRAME as core::ffi::c_int)
                            as core::ffi::c_int,
                        av1_get_pred_cdf_single_ref_p6(xd),
                        2 as core::ffi::c_int,
                    );
                }
            } else {
                let bit2_1: core::ffi::c_int = ((*mbmi)
                    .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == LAST3_FRAME as core::ffi::c_int
                    || (*mbmi).ref_frame[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int == GOLDEN_FRAME as core::ffi::c_int)
                    as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    bit2_1,
                    av1_get_pred_cdf_single_ref_p3(xd),
                    2 as core::ffi::c_int,
                );
                if bit2_1 == 0 {
                    let bit3: core::ffi::c_int = ((*mbmi)
                        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        != LAST_FRAME as core::ffi::c_int) as core::ffi::c_int;
                    aom_write_symbol(
                        w,
                        bit3,
                        av1_get_pred_cdf_single_ref_p4(xd),
                        2 as core::ffi::c_int,
                    );
                } else {
                    let bit4: core::ffi::c_int = ((*mbmi)
                        .ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        != LAST3_FRAME as core::ffi::c_int) as core::ffi::c_int;
                    aom_write_symbol(
                        w,
                        bit4,
                        av1_get_pred_cdf_single_ref_p5(xd),
                        2 as core::ffi::c_int,
                    );
                }
            }
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_filter_intra_mode_info(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
) {
    if av1_filter_intra_allowed(cm, mbmi) != 0 {
        aom_write_symbol(
            w,
            (*mbmi).filter_intra_mode_info.use_filter_intra as core::ffi::c_int,
            ((*(*xd).tile_ctx).filter_intra_cdfs[(*mbmi).bsize as usize]).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if (*mbmi).filter_intra_mode_info.use_filter_intra != 0 {
            let mode: FILTER_INTRA_MODE = (*mbmi)
                .filter_intra_mode_info
                .filter_intra_mode;
            aom_write_symbol(
                w,
                mode as core::ffi::c_int,
                ((*(*xd).tile_ctx).filter_intra_mode_cdf).as_mut_ptr(),
                FILTER_INTRA_MODES as core::ffi::c_int,
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_angle_delta(
    mut w: *mut aom_writer,
    mut angle_delta: core::ffi::c_int,
    mut cdf: *mut aom_cdf_prob,
) {
    aom_write_symbol(
        w,
        angle_delta + MAX_ANGLE_DELTA,
        cdf,
        2 as core::ffi::c_int * MAX_ANGLE_DELTA + 1 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_mb_interp_filter(
    cm: *mut AV1_COMMON,
    mut td: *mut ThreadData,
    mut w: *mut aom_writer,
) {
    let mut xd: *const MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if av1_is_interp_needed(xd) == 0 {
        let mut filters: int_interpfilters = av1_broadcast_interp_filter(
            av1_unswitchable_filter((*cm).features.interp_filter),
        );
        if !((*mbmi).interp_filters.as_int == filters.as_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"write_mb_interp_filter\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                634 as core::ffi::c_int,
                b"mbmi->interp_filters.as_int == filters.as_int\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return;
    }
    if (*cm).features.interp_filter as core::ffi::c_int == SWITCHABLE as core::ffi::c_int
    {
        let mut dir: core::ffi::c_int = 0;
        dir = 0 as core::ffi::c_int;
        while dir < 2 as core::ffi::c_int {
            let ctx: core::ffi::c_int = av1_get_pred_context_switchable_interp(xd, dir)
                as core::ffi::c_int;
            let mut filter: InterpFilter = av1_extract_interp_filter(
                (*mbmi).interp_filters,
                dir,
            );
            aom_write_symbol(
                w,
                filter as core::ffi::c_int,
                ((*ec_ctx).switchable_interp_cdf[ctx as usize]).as_mut_ptr(),
                SWITCHABLE_FILTERS as core::ffi::c_int,
            );
            (*td).interp_filter_selected[filter as usize] += 1;
            if (*(*cm).seq_params).enable_dual_filter as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                return;
            }
            dir += 1;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn delta_encode_palette_colors(
    mut colors: *const core::ffi::c_int,
    mut num: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut min_val: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    if num <= 0 as core::ffi::c_int {
        return;
    }
    if !(*colors.offset(0 as core::ffi::c_int as isize)
        < (1 as core::ffi::c_int) << bit_depth) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"delta_encode_palette_colors\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            659 as core::ffi::c_int,
            b"colors[0] < (1 << bit_depth)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_write_literal(w, *colors.offset(0 as core::ffi::c_int as isize), bit_depth);
    if num == 1 as core::ffi::c_int {
        return;
    }
    let mut max_delta: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut deltas: [core::ffi::c_int; 8] = [0; 8];
    memset(
        deltas.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 8]>() as size_t,
    );
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < num {
        if !(*colors.offset(i as isize) < (1 as core::ffi::c_int) << bit_depth)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"delta_encode_palette_colors\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                666 as core::ffi::c_int,
                b"colors[i] < (1 << bit_depth)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let delta: core::ffi::c_int = *colors.offset(i as isize)
            - *colors.offset((i - 1 as core::ffi::c_int) as isize);
        deltas[(i - 1 as core::ffi::c_int) as usize] = delta;
        if !(delta >= min_val) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"delta_encode_palette_colors\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                669 as core::ffi::c_int,
                b"delta >= min_val\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if delta > max_delta {
            max_delta = delta;
        }
        i += 1;
    }
    let min_bits: core::ffi::c_int = bit_depth - 3 as core::ffi::c_int;
    let mut bits: core::ffi::c_int = if aom_ceil_log2(
        max_delta + 1 as core::ffi::c_int - min_val,
    ) > min_bits
    {
        aom_ceil_log2(max_delta + 1 as core::ffi::c_int - min_val)
    } else {
        min_bits
    };
    if !(bits <= bit_depth) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"delta_encode_palette_colors\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            674 as core::ffi::c_int,
            b"bits <= bit_depth\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut range: core::ffi::c_int = ((1 as core::ffi::c_int) << bit_depth)
        - *colors.offset(0 as core::ffi::c_int as isize) - min_val;
    aom_write_literal(w, bits - min_bits, 2 as core::ffi::c_int);
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < num - 1 as core::ffi::c_int {
        aom_write_literal(w, deltas[i_0 as usize] - min_val, bits);
        range -= deltas[i_0 as usize];
        bits = if bits < aom_ceil_log2(range) { bits } else { aom_ceil_log2(range) };
        i_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_palette_colors_y(
    xd: *const MACROBLOCKD,
    pmi: *const PALETTE_MODE_INFO,
    mut bit_depth: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let n: core::ffi::c_int = (*pmi).palette_size[0 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let mut color_cache: [uint16_t; 16] = [0; 16];
    let n_cache: core::ffi::c_int = av1_get_palette_cache(
        xd,
        0 as core::ffi::c_int,
        color_cache.as_mut_ptr(),
    ) as core::ffi::c_int;
    let mut out_cache_colors: [core::ffi::c_int; 8] = [0; 8];
    let mut cache_color_found: [uint8_t; 16] = [0; 16];
    let n_out_cache: core::ffi::c_int = av1_index_color_cache(
        color_cache.as_mut_ptr(),
        n_cache,
        ((*pmi).palette_colors).as_ptr(),
        n,
        cache_color_found.as_mut_ptr(),
        out_cache_colors.as_mut_ptr(),
    ) as core::ffi::c_int;
    let mut n_in_cache: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_cache && n_in_cache < n {
        let found: core::ffi::c_int = cache_color_found[i as usize] as core::ffi::c_int;
        aom_write_bit(w, found);
        n_in_cache += found;
        i += 1;
    }
    if !(n_in_cache + n_out_cache == n) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"write_palette_colors_y\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            704 as core::ffi::c_int,
            b"n_in_cache + n_out_cache == n\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    delta_encode_palette_colors(
        out_cache_colors.as_mut_ptr(),
        n_out_cache,
        bit_depth,
        1 as core::ffi::c_int,
        w,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_palette_colors_uv(
    xd: *const MACROBLOCKD,
    pmi: *const PALETTE_MODE_INFO,
    mut bit_depth: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let n: core::ffi::c_int = (*pmi).palette_size[1 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let mut colors_u: *const uint16_t = ((*pmi).palette_colors)
        .as_ptr()
        .offset(PALETTE_MAX_SIZE as isize);
    let mut colors_v: *const uint16_t = ((*pmi).palette_colors)
        .as_ptr()
        .offset((2 as core::ffi::c_int * PALETTE_MAX_SIZE) as isize);
    let mut color_cache: [uint16_t; 16] = [0; 16];
    let n_cache: core::ffi::c_int = av1_get_palette_cache(
        xd,
        1 as core::ffi::c_int,
        color_cache.as_mut_ptr(),
    ) as core::ffi::c_int;
    let mut out_cache_colors: [core::ffi::c_int; 8] = [0; 8];
    let mut cache_color_found: [uint8_t; 16] = [0; 16];
    let n_out_cache: core::ffi::c_int = av1_index_color_cache(
        color_cache.as_mut_ptr(),
        n_cache,
        colors_u,
        n,
        cache_color_found.as_mut_ptr(),
        out_cache_colors.as_mut_ptr(),
    ) as core::ffi::c_int;
    let mut n_in_cache: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n_cache && n_in_cache < n {
        let found: core::ffi::c_int = cache_color_found[i as usize] as core::ffi::c_int;
        aom_write_bit(w, found);
        n_in_cache += found;
        i += 1;
    }
    delta_encode_palette_colors(
        out_cache_colors.as_mut_ptr(),
        n_out_cache,
        bit_depth,
        0 as core::ffi::c_int,
        w,
    );
    let max_val: core::ffi::c_int = (1 as core::ffi::c_int) << bit_depth;
    let mut zero_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut min_bits_v: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bits_v: core::ffi::c_int = av1_get_palette_delta_bits_v(
        pmi,
        bit_depth,
        &raw mut zero_count,
        &raw mut min_bits_v,
    );
    let rate_using_delta: core::ffi::c_int = 2 as core::ffi::c_int + bit_depth
        + (bits_v + 1 as core::ffi::c_int) * (n - 1 as core::ffi::c_int) - zero_count;
    let rate_using_raw: core::ffi::c_int = bit_depth * n;
    if rate_using_delta < rate_using_raw {
        if !((*colors_v.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            < (1 as core::ffi::c_int) << bit_depth) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"write_palette_colors_uv\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                741 as core::ffi::c_int,
                b"colors_v[0] < (1 << bit_depth)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        aom_write_bit(w, 1 as core::ffi::c_int);
        aom_write_literal(w, bits_v - min_bits_v, 2 as core::ffi::c_int);
        aom_write_literal(
            w,
            *colors_v.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            bit_depth,
        );
        let mut i_0: core::ffi::c_int = 1 as core::ffi::c_int;
        while i_0 < n {
            if !((*colors_v.offset(i_0 as isize) as core::ffi::c_int)
                < (1 as core::ffi::c_int) << bit_depth) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"write_palette_colors_uv\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    746 as core::ffi::c_int,
                    b"colors_v[i] < (1 << bit_depth)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if *colors_v.offset(i_0 as isize) as core::ffi::c_int
                == *colors_v.offset((i_0 - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
            {
                aom_write_literal(w, 0 as core::ffi::c_int, bits_v);
            } else {
                let delta: core::ffi::c_int = abs(
                    *colors_v.offset(i_0 as isize) as core::ffi::c_int
                        - *colors_v.offset((i_0 - 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int,
                ) as core::ffi::c_int;
                let sign_bit: core::ffi::c_int = ((*colors_v.offset(i_0 as isize)
                    as core::ffi::c_int)
                    < *colors_v.offset((i_0 - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int) as core::ffi::c_int;
                if delta <= max_val - delta {
                    aom_write_literal(w, delta, bits_v);
                    aom_write_bit(w, sign_bit);
                } else {
                    aom_write_literal(w, max_val - delta, bits_v);
                    aom_write_bit(w, (sign_bit == 0) as core::ffi::c_int);
                }
            }
            i_0 += 1;
        }
    } else {
        aom_write_bit(w, 0 as core::ffi::c_int);
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < n {
            if !((*colors_v.offset(i_1 as isize) as core::ffi::c_int)
                < (1 as core::ffi::c_int) << bit_depth) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"write_palette_colors_uv\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    764 as core::ffi::c_int,
                    b"colors_v[i] < (1 << bit_depth)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            aom_write_literal(
                w,
                *colors_v.offset(i_1 as isize) as core::ffi::c_int,
                bit_depth,
            );
            i_1 += 1;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_palette_mode_info(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mut w: *mut aom_writer,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if (av1_allow_palette(
        (*cm).features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"write_palette_mode_info\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            776 as core::ffi::c_int,
            b"av1_allow_palette(cm->features.allow_screen_content_tools, bsize)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let pmi: *const PALETTE_MODE_INFO = &(*mbmi).palette_mode_info;
    let bsize_ctx: core::ffi::c_int = av1_get_palette_bsize_ctx(bsize)
        as core::ffi::c_int;
    if (*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        let n: core::ffi::c_int = (*pmi).palette_size[0 as core::ffi::c_int as usize]
            as core::ffi::c_int;
        let palette_y_mode_ctx: core::ffi::c_int = av1_get_palette_mode_ctx(xd)
            as core::ffi::c_int;
        aom_write_symbol(
            w,
            (n > 0 as core::ffi::c_int) as core::ffi::c_int,
            ((*(*xd).tile_ctx)
                .palette_y_mode_cdf[bsize_ctx as usize][palette_y_mode_ctx as usize])
                .as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if n > 0 as core::ffi::c_int {
            aom_write_symbol(
                w,
                n - PALETTE_MIN_SIZE,
                ((*(*xd).tile_ctx).palette_y_size_cdf[bsize_ctx as usize]).as_mut_ptr(),
                PALETTE_SIZES as core::ffi::c_int,
            );
            write_palette_colors_y(
                xd,
                pmi,
                (*(*cm).seq_params).bit_depth as core::ffi::c_int,
                w,
            );
        }
    }
    let uv_dc_pred: core::ffi::c_int = (num_planes > 1 as core::ffi::c_int
        && (*mbmi).uv_mode as core::ffi::c_int == UV_DC_PRED as core::ffi::c_int
        && (*xd).is_chroma_ref as core::ffi::c_int != 0) as core::ffi::c_int;
    if uv_dc_pred != 0 {
        let n_0: core::ffi::c_int = (*pmi).palette_size[1 as core::ffi::c_int as usize]
            as core::ffi::c_int;
        let palette_uv_mode_ctx: core::ffi::c_int = ((*pmi)
            .palette_size[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int) as core::ffi::c_int;
        aom_write_symbol(
            w,
            (n_0 > 0 as core::ffi::c_int) as core::ffi::c_int,
            ((*(*xd).tile_ctx).palette_uv_mode_cdf[palette_uv_mode_ctx as usize])
                .as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if n_0 > 0 as core::ffi::c_int {
            aom_write_symbol(
                w,
                n_0 - PALETTE_MIN_SIZE,
                ((*(*xd).tile_ctx).palette_uv_size_cdf[bsize_ctx as usize]).as_mut_ptr(),
                PALETTE_SIZES as core::ffi::c_int,
            );
            write_palette_colors_uv(
                xd,
                pmi,
                (*(*cm).seq_params).bit_depth as core::ffi::c_int,
                w,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_tx_type(
    cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut w: *mut aom_writer,
) {
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let features: *const FeatureFlags = &(*cm).features;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    if get_ext_tx_types(
        tx_size,
        is_inter,
        (*features).reduced_tx_set_used as core::ffi::c_int,
    ) > 1 as core::ffi::c_int
        && ((*cm).seg.enabled == 0
            && (*cm).quant_params.base_qindex > 0 as core::ffi::c_int
            || (*cm).seg.enabled as core::ffi::c_int != 0
                && (*xd).qindex[(*mbmi).segment_id() as usize] > 0 as core::ffi::c_int)
        && (*mbmi).skip_txfm == 0
        && segfeature_active(
            &(*cm).seg,
            (*mbmi).segment_id(),
            SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
        ) == 0
    {
        let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
        let square_tx_size: TX_SIZE = txsize_sqr_map[tx_size as usize];
        let tx_set_type: TxSetType = av1_get_ext_tx_set_type(
            tx_size,
            is_inter,
            (*features).reduced_tx_set_used as core::ffi::c_int,
        ) as TxSetType;
        let eset: core::ffi::c_int = get_ext_tx_set(
            tx_size,
            is_inter,
            (*features).reduced_tx_set_used as core::ffi::c_int,
        ) as core::ffi::c_int;
        if !(eset > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"av1_write_tx_type\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                828 as core::ffi::c_int,
                b"eset > 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (av1_ext_tx_used[tx_set_type as usize][tx_type as usize] == 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"av1_write_tx_type\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                829 as core::ffi::c_int,
                b"av1_ext_tx_used[tx_set_type][tx_type]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if is_inter != 0 {
            aom_write_symbol(
                w,
                av1_ext_tx_ind[tx_set_type as usize][tx_type as usize],
                ((*ec_ctx).inter_ext_tx_cdf[eset as usize][square_tx_size as usize])
                    .as_mut_ptr(),
                av1_num_ext_tx_set[tx_set_type as usize],
            );
        } else {
            let mut intra_dir: PREDICTION_MODE = 0;
            if (*mbmi).filter_intra_mode_info.use_filter_intra != 0 {
                intra_dir = fimode_to_intradir[(*mbmi)
                    .filter_intra_mode_info
                    .filter_intra_mode as usize];
            } else {
                intra_dir = (*mbmi).mode;
            }
            aom_write_symbol(
                w,
                av1_ext_tx_ind[tx_set_type as usize][tx_type as usize],
                ((*ec_ctx)
                    .intra_ext_tx_cdf[eset
                    as usize][square_tx_size as usize][intra_dir as usize])
                    .as_mut_ptr(),
                av1_num_ext_tx_set[tx_set_type as usize],
            );
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_intra_y_mode_nonkf(
    mut frame_ctx: *mut FRAME_CONTEXT,
    mut bsize: BLOCK_SIZE,
    mut mode: PREDICTION_MODE,
    mut w: *mut aom_writer,
) {
    aom_write_symbol(
        w,
        mode as core::ffi::c_int,
        ((*frame_ctx).y_mode_cdf[size_group_lookup[bsize as usize] as usize])
            .as_mut_ptr(),
        INTRA_MODES as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_intra_uv_mode(
    mut frame_ctx: *mut FRAME_CONTEXT,
    mut uv_mode: UV_PREDICTION_MODE,
    mut y_mode: PREDICTION_MODE,
    mut cfl_allowed: CFL_ALLOWED_TYPE,
    mut w: *mut aom_writer,
) {
    aom_write_symbol(
        w,
        uv_mode as core::ffi::c_int,
        ((*frame_ctx).uv_mode_cdf[cfl_allowed as usize][y_mode as usize]).as_mut_ptr(),
        UV_INTRA_MODES as core::ffi::c_int - (cfl_allowed == 0) as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_cfl_alphas(
    ec_ctx: *mut FRAME_CONTEXT,
    mut idx: uint8_t,
    mut joint_sign: int8_t,
    mut w: *mut aom_writer,
) {
    aom_write_symbol(
        w,
        joint_sign as core::ffi::c_int,
        ((*ec_ctx).cfl_sign_cdf).as_mut_ptr(),
        CFL_SIGNS as core::ffi::c_int * CFL_SIGNS as core::ffi::c_int
            - 1 as core::ffi::c_int,
    );
    if (joint_sign as core::ffi::c_int + 1 as core::ffi::c_int) * 11 as core::ffi::c_int
        >> 5 as core::ffi::c_int != CFL_SIGN_ZERO as core::ffi::c_int
    {
        let mut cdf_u: *mut aom_cdf_prob = ((*ec_ctx)
            .cfl_alpha_cdf[(joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int) as usize])
            .as_mut_ptr();
        aom_write_symbol(
            w,
            idx as core::ffi::c_int >> CFL_ALPHABET_SIZE_LOG2,
            cdf_u,
            CFL_ALPHABET_SIZE,
        );
    }
    if joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
        - CFL_SIGNS as core::ffi::c_int
            * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
        != CFL_SIGN_ZERO as core::ffi::c_int
    {
        let mut cdf_v: *mut aom_cdf_prob = ((*ec_ctx)
            .cfl_alpha_cdf[((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int
                * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 11 as core::ffi::c_int >> 5 as core::ffi::c_int))
            * CFL_SIGNS as core::ffi::c_int
            + ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
            - CFL_SIGNS as core::ffi::c_int) as usize])
            .as_mut_ptr();
        aom_write_symbol(
            w,
            idx as core::ffi::c_int & CFL_ALPHABET_SIZE - 1 as core::ffi::c_int,
            cdf_v,
            CFL_ALPHABET_SIZE,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_cdef(
    mut cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut w: *mut aom_writer,
    mut skip: core::ffi::c_int,
) {
    if (*cm).features.coded_lossless as core::ffi::c_int != 0
        || (*cm).features.allow_intrabc as core::ffi::c_int != 0
    {
        return;
    }
    let sb_mask: core::ffi::c_int = (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int;
    let mi_row_in_sb: core::ffi::c_int = (*xd).mi_row & sb_mask;
    let mi_col_in_sb: core::ffi::c_int = (*xd).mi_col & sb_mask;
    if mi_row_in_sb == 0 as core::ffi::c_int && mi_col_in_sb == 0 as core::ffi::c_int {
        (*xd).cdef_transmitted[3 as core::ffi::c_int as usize] = false_0 != 0;
        (*xd).cdef_transmitted[2 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[3 as core::ffi::c_int as usize];
        (*xd).cdef_transmitted[1 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[2 as core::ffi::c_int as usize];
        (*xd).cdef_transmitted[0 as core::ffi::c_int as usize] = (*xd)
            .cdef_transmitted[1 as core::ffi::c_int as usize];
    }
    let cdef_size: core::ffi::c_int = (1 as core::ffi::c_int)
        << 6 as core::ffi::c_int - MI_SIZE_LOG2;
    let index_mask: core::ffi::c_int = cdef_size;
    let cdef_unit_row_in_sb: core::ffi::c_int = ((*xd).mi_row & index_mask
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let cdef_unit_col_in_sb: core::ffi::c_int = ((*xd).mi_col & index_mask
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let index: core::ffi::c_int = if (*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_128X128 as core::ffi::c_int
    {
        cdef_unit_col_in_sb + 2 as core::ffi::c_int * cdef_unit_row_in_sb
    } else {
        0 as core::ffi::c_int
    };
    if !(*xd).cdef_transmitted[index as usize] && skip == 0 {
        let first_block_mask: core::ffi::c_int = !(cdef_size - 1 as core::ffi::c_int);
        let mi_params: *const CommonModeInfoParams = &raw mut (*cm).mi_params;
        let grid_idx: core::ffi::c_int = get_mi_grid_idx(
            mi_params,
            (*xd).mi_row & first_block_mask,
            (*xd).mi_col & first_block_mask,
        ) as core::ffi::c_int;
        let mbmi: *const MB_MODE_INFO = *((*mi_params).mi_grid_base)
            .offset(grid_idx as isize);
        aom_write_literal(
            w,
            (*mbmi).cdef_strength() as core::ffi::c_int,
            (*cm).cdef_info.cdef_bits,
        );
        (*xd).cdef_transmitted[index as usize] = true_0 != 0;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_inter_segment_id(
    mut cpi: *mut AV1_COMP,
    xd: *mut MACROBLOCKD,
    mut w: *mut aom_writer,
    seg: *const segmentation,
    segp: *mut segmentation_probs,
    mut skip: core::ffi::c_int,
    mut preskip: core::ffi::c_int,
) {
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    if (*seg).update_map != 0 {
        if preskip != 0 {
            if (*seg).segid_preskip == 0 {
                return;
            }
        } else {
            if (*seg).segid_preskip != 0 {
                return;
            }
            if skip != 0 {
                write_segment_id(cpi, xd, mbmi, w, seg, segp, 1 as core::ffi::c_int);
                if (*seg).temporal_update != 0 {
                    (*mbmi).set_seg_id_predicted(0 as uint8_t as uint8_t);
                }
                return;
            }
        }
        if (*seg).temporal_update != 0 {
            let pred_flag: core::ffi::c_int = (*mbmi).seg_id_predicted()
                as core::ffi::c_int;
            let mut pred_cdf: *mut aom_cdf_prob = av1_get_pred_cdf_seg_id(segp, xd);
            aom_write_symbol(w, pred_flag, pred_cdf, 2 as core::ffi::c_int);
            if pred_flag == 0 {
                write_segment_id(cpi, xd, mbmi, w, seg, segp, 0 as core::ffi::c_int);
            }
            if pred_flag != 0 {
                set_spatial_segment_id(
                    &raw mut (*cm).mi_params,
                    (*(*cm).cur_frame).seg_map,
                    (*mbmi).bsize,
                    mi_row,
                    mi_col,
                    (*mbmi).segment_id(),
                );
            }
        } else {
            write_segment_id(cpi, xd, mbmi, w, seg, segp, 0 as core::ffi::c_int);
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_delta_q_params(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut skip: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    if (*delta_q_info).delta_q_present_flag != 0 {
        let mbmi: *const MB_MODE_INFO = *((*xd).mi)
            .offset(0 as core::ffi::c_int as isize);
        let bsize: BLOCK_SIZE = (*mbmi).bsize;
        let super_block_upper_left: core::ffi::c_int = ((*xd).mi_row
            & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*xd).mi_col & (*(*cm).seq_params).mib_size - 1 as core::ffi::c_int
                == 0 as core::ffi::c_int) as core::ffi::c_int;
        if (bsize as core::ffi::c_int != (*(*cm).seq_params).sb_size as core::ffi::c_int
            || skip == 0 as core::ffi::c_int) && super_block_upper_left != 0
        {
            if !((*mbmi).current_qindex > 0 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"write_delta_q_params\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    974 as core::ffi::c_int,
                    b"mbmi->current_qindex > 0\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let reduced_delta_qindex: core::ffi::c_int = ((*mbmi).current_qindex
                - (*xd).current_base_qindex) / (*delta_q_info).delta_q_res;
            write_delta_qindex(xd, reduced_delta_qindex, w);
            (*xd).current_base_qindex = (*mbmi).current_qindex;
            if (*delta_q_info).delta_lf_present_flag != 0 {
                if (*delta_q_info).delta_lf_multi != 0 {
                    let frame_lf_count: core::ffi::c_int = if av1_num_planes(cm)
                        > 1 as core::ffi::c_int
                    {
                        FRAME_LF_COUNT
                    } else {
                        FRAME_LF_COUNT - 2 as core::ffi::c_int
                    };
                    let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
                    while lf_id < frame_lf_count {
                        let mut reduced_delta_lflevel: core::ffi::c_int = ((*mbmi)
                            .delta_lf[lf_id as usize] as core::ffi::c_int
                            - (*xd).delta_lf[lf_id as usize] as core::ffi::c_int)
                            / (*delta_q_info).delta_lf_res;
                        write_delta_lflevel(
                            cm,
                            xd,
                            lf_id,
                            reduced_delta_lflevel,
                            1 as core::ffi::c_int,
                            w,
                        );
                        (*xd).delta_lf[lf_id as usize] = (*mbmi)
                            .delta_lf[lf_id as usize];
                        lf_id += 1;
                    }
                } else {
                    let mut reduced_delta_lflevel_0: core::ffi::c_int = ((*mbmi)
                        .delta_lf_from_base as core::ffi::c_int
                        - (*xd).delta_lf_from_base as core::ffi::c_int)
                        / (*delta_q_info).delta_lf_res;
                    write_delta_lflevel(
                        cm,
                        xd,
                        -(1 as core::ffi::c_int),
                        reduced_delta_lflevel_0,
                        0 as core::ffi::c_int,
                        w,
                    );
                    (*xd).delta_lf_from_base = (*mbmi).delta_lf_from_base;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_intra_prediction_modes(
    mut cm: *const AV1_COMMON,
    xd: *mut MACROBLOCKD,
    mut is_keyframe: core::ffi::c_int,
    mut w: *mut aom_writer,
) {
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mode: PREDICTION_MODE = (*mbmi).mode;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if is_keyframe != 0 {
        let above_mi: *const MB_MODE_INFO = (*xd).above_mbmi;
        let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
        write_intra_y_mode_kf(ec_ctx, mbmi, above_mi, left_mi, mode, w);
    } else {
        write_intra_y_mode_nonkf(ec_ctx, bsize, mode, w);
    }
    let use_angle_delta: core::ffi::c_int = av1_use_angle_delta(bsize)
        as core::ffi::c_int;
    if use_angle_delta != 0 && av1_is_directional_mode(mode) != 0 {
        write_angle_delta(
            w,
            (*mbmi).angle_delta[PLANE_TYPE_Y as core::ffi::c_int as usize]
                as core::ffi::c_int,
            ((*ec_ctx)
                .angle_delta_cdf[(mode as core::ffi::c_int - V_PRED as core::ffi::c_int)
                as usize])
                .as_mut_ptr(),
        );
    }
    if (*(*cm).seq_params).monochrome == 0
        && (*xd).is_chroma_ref as core::ffi::c_int != 0
    {
        let uv_mode: UV_PREDICTION_MODE = (*mbmi).uv_mode;
        write_intra_uv_mode(ec_ctx, uv_mode, mode, is_cfl_allowed(xd), w);
        if uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int {
            write_cfl_alphas(ec_ctx, (*mbmi).cfl_alpha_idx, (*mbmi).cfl_alpha_signs, w);
        }
        let intra_mode: PREDICTION_MODE = get_uv_mode(uv_mode) as PREDICTION_MODE;
        if use_angle_delta != 0 && av1_is_directional_mode(intra_mode) != 0 {
            write_angle_delta(
                w,
                (*mbmi).angle_delta[PLANE_TYPE_UV as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                ((*ec_ctx)
                    .angle_delta_cdf[(intra_mode as core::ffi::c_int
                    - V_PRED as core::ffi::c_int) as usize])
                    .as_mut_ptr(),
            );
        }
    }
    if av1_allow_palette(
        (*cm).features.allow_screen_content_tools as core::ffi::c_int,
        bsize,
    ) != 0
    {
        write_palette_mode_info(cm, xd, mbmi, w);
    }
    write_filter_intra_mode_info(cm, xd, mbmi, w);
}
#[inline]
unsafe extern "C-unwind" fn mode_context_analyzer(
    mode_context: int16_t,
    rf: *const MV_REFERENCE_FRAME,
) -> int16_t {
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        <= INTRA_FRAME as core::ffi::c_int
    {
        return mode_context;
    }
    let newmv_ctx: int16_t = (mode_context as core::ffi::c_int & NEWMV_CTX_MASK)
        as int16_t;
    let refmv_ctx: int16_t = (mode_context as core::ffi::c_int >> REFMV_OFFSET
        & REFMV_CTX_MASK) as int16_t;
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
unsafe extern "C-unwind" fn get_ref_mv_from_stack(
    mut ref_idx: core::ffi::c_int,
    mut ref_frame: *const MV_REFERENCE_FRAME,
    mut ref_mv_idx: core::ffi::c_int,
    mut mbmi_ext_frame: *const MB_MODE_INFO_EXT_FRAME,
) -> int_mv {
    let ref_frame_type: int8_t = av1_ref_frame_type(ref_frame) as int8_t;
    let mut curr_ref_mv_stack: *const CANDIDATE_MV = ((*mbmi_ext_frame).ref_mv_stack)
        .as_ptr();
    if *ref_frame.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
    {
        if !(ref_idx == 0 as core::ffi::c_int || ref_idx == 1 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"get_ref_mv_from_stack\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1069 as core::ffi::c_int,
                b"ref_idx == 0 || ref_idx == 1\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return if ref_idx != 0 {
            (*curr_ref_mv_stack.offset(ref_mv_idx as isize)).comp_mv
        } else {
            (*curr_ref_mv_stack.offset(ref_mv_idx as isize)).this_mv
        };
    }
    if !(ref_idx == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"get_ref_mv_from_stack\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1074 as core::ffi::c_int,
            b"ref_idx == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return if ref_mv_idx < (*mbmi_ext_frame).ref_mv_count as core::ffi::c_int {
        (*curr_ref_mv_stack.offset(ref_mv_idx as isize)).this_mv
    } else {
        (*mbmi_ext_frame).global_mvs[ref_frame_type as usize]
    };
}
#[inline]
unsafe extern "C-unwind" fn get_ref_mv(
    mut x: *const MACROBLOCK,
    mut ref_idx: core::ffi::c_int,
) -> int_mv {
    let mut xd: *const MACROBLOCKD = &(*x).e_mbd;
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let mut ref_mv_idx: core::ffi::c_int = (*mbmi).ref_mv_idx() as core::ffi::c_int;
    if (*mbmi).mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
        || (*mbmi).mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int
    {
        if (has_second_ref(mbmi) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 11],
                    [core::ffi::c_char; 11],
                >(*b"get_ref_mv\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1085 as core::ffi::c_int,
                b"has_second_ref(mbmi)\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        ref_mv_idx += 1 as core::ffi::c_int;
    }
    return get_ref_mv_from_stack(
        ref_idx,
        ((*mbmi).ref_frame).as_ptr(),
        ref_mv_idx,
        (*x).mbmi_ext_frame,
    );
}
#[inline]
unsafe extern "C-unwind" fn pack_inter_mode_mvs(
    mut cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    mut w: *mut aom_writer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let x: *mut MACROBLOCK = &raw mut (*td).mb;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let seg: *const segmentation = &raw mut (*cm).seg;
    let segp: *mut segmentation_probs = &raw mut (*ec_ctx).seg;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mbmi_ext_frame: *const MB_MODE_INFO_EXT_FRAME = (*x).mbmi_ext_frame;
    let mode: PREDICTION_MODE = (*mbmi).mode;
    let segment_id: uint8_t = (*mbmi).segment_id();
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    let allow_hp: core::ffi::c_int = (*cm).features.allow_high_precision_mv
        as core::ffi::c_int;
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let mut ref_0: core::ffi::c_int = 0;
    write_inter_segment_id(
        cpi,
        xd,
        w,
        seg,
        segp,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
    write_skip_mode(cm, xd, segment_id, mbmi, w);
    if !((*mbmi).skip_mode() == 0 || (*mbmi).skip_txfm as core::ffi::c_int != 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"pack_inter_mode_mvs\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1114 as core::ffi::c_int,
            b"IMPLIES(mbmi->skip_mode, mbmi->skip_txfm)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let skip: core::ffi::c_int = if (*mbmi).skip_mode() as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        write_skip(cm, xd, segment_id, mbmi, w) as core::ffi::c_int
    };
    write_inter_segment_id(cpi, xd, w, seg, segp, skip, 0 as core::ffi::c_int);
    write_cdef(cm, xd, w, skip);
    write_delta_q_params(cm, xd, skip, w);
    if (*mbmi).skip_mode() == 0 {
        write_is_inter(cm, xd, (*mbmi).segment_id(), w, is_inter);
    }
    if (*mbmi).skip_mode() != 0 {
        return;
    }
    if is_inter == 0 {
        write_intra_prediction_modes(cm, xd, 0 as core::ffi::c_int, w);
    } else {
        let mut mode_ctx: int16_t = 0;
        av1_collect_neighbors_ref_counts(xd);
        write_ref_frames(cm, xd, w);
        mode_ctx = mode_context_analyzer(
            (*mbmi_ext_frame).mode_context,
            ((*mbmi).ref_frame).as_ptr(),
        );
        if segfeature_active(
            seg,
            segment_id,
            SEG_LVL_SKIP as core::ffi::c_int as SEG_LVL_FEATURES,
        ) == 0
        {
            if is_inter_compound_mode(mode) != 0 {
                write_inter_compound_mode(xd, w, mode, mode_ctx);
            } else if is_inter_singleref_mode(mode) != 0 {
                write_inter_mode(w, mode, ec_ctx, mode_ctx);
            }
            if mode as core::ffi::c_int == NEWMV as core::ffi::c_int
                || mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
                || have_nearmv_in_inter_mode(mode) != 0
            {
                write_drl_idx(ec_ctx, mbmi, mbmi_ext_frame, w);
            } else {
                if !((*mbmi).ref_mv_idx() as core::ffi::c_int == 0 as core::ffi::c_int)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"pack_inter_mode_mvs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1150 as core::ffi::c_int,
                        b"mbmi->ref_mv_idx == 0\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
        if mode as core::ffi::c_int == NEWMV as core::ffi::c_int
            || mode as core::ffi::c_int == NEW_NEWMV as core::ffi::c_int
        {
            ref_0 = 0 as core::ffi::c_int;
            while ref_0 < 1 as core::ffi::c_int + is_compound {
                let mut nmvc: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
                let ref_mv: int_mv = get_ref_mv(x, ref_0) as int_mv;
                av1_encode_mv(
                    cpi,
                    w,
                    td,
                    &(*((*mbmi).mv).as_ptr().offset(ref_0 as isize)).as_mv,
                    &ref_mv.as_mv,
                    nmvc,
                    allow_hp,
                );
                ref_0 += 1;
            }
        } else if mode as core::ffi::c_int == NEAREST_NEWMV as core::ffi::c_int
            || mode as core::ffi::c_int == NEAR_NEWMV as core::ffi::c_int
        {
            let mut nmvc_0: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            let ref_mv_0: int_mv = get_ref_mv(x, 1 as core::ffi::c_int) as int_mv;
            av1_encode_mv(
                cpi,
                w,
                td,
                &(*((*mbmi).mv).as_ptr().offset(1 as core::ffi::c_int as isize)).as_mv,
                &ref_mv_0.as_mv,
                nmvc_0,
                allow_hp,
            );
        } else if mode as core::ffi::c_int == NEW_NEARESTMV as core::ffi::c_int
            || mode as core::ffi::c_int == NEW_NEARMV as core::ffi::c_int
        {
            let mut nmvc_1: *mut nmv_context = &raw mut (*ec_ctx).nmvc;
            let ref_mv_1: int_mv = get_ref_mv(x, 0 as core::ffi::c_int) as int_mv;
            av1_encode_mv(
                cpi,
                w,
                td,
                &(*((*mbmi).mv).as_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
                &ref_mv_1.as_mv,
                nmvc_1,
                allow_hp,
            );
        }
        if (*cpi).common.current_frame.reference_mode as core::ffi::c_int
            != COMPOUND_REFERENCE as core::ffi::c_int
            && (*(*cpi).common.seq_params).enable_interintra_compound as core::ffi::c_int
                != 0 && is_interintra_allowed(mbmi) != 0
        {
            let interintra: core::ffi::c_int = ((*mbmi)
                .ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
            let bsize_group: core::ffi::c_int = size_group_lookup[bsize as usize]
                as core::ffi::c_int;
            aom_write_symbol(
                w,
                interintra,
                ((*ec_ctx).interintra_cdf[bsize_group as usize]).as_mut_ptr(),
                2 as core::ffi::c_int,
            );
            if interintra != 0 {
                aom_write_symbol(
                    w,
                    (*mbmi).interintra_mode as core::ffi::c_int,
                    ((*ec_ctx).interintra_mode_cdf[bsize_group as usize]).as_mut_ptr(),
                    INTERINTRA_MODES as core::ffi::c_int,
                );
                if av1_is_wedge_used(bsize) != 0 {
                    aom_write_symbol(
                        w,
                        (*mbmi).use_wedge_interintra() as core::ffi::c_int,
                        ((*ec_ctx).wedge_interintra_cdf[bsize as usize]).as_mut_ptr(),
                        2 as core::ffi::c_int,
                    );
                    if (*mbmi).use_wedge_interintra() != 0 {
                        aom_write_symbol(
                            w,
                            (*mbmi).interintra_wedge_index as core::ffi::c_int,
                            ((*ec_ctx).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                            MAX_WEDGE_TYPES,
                        );
                    }
                }
            }
        }
        if (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            != INTRA_FRAME as core::ffi::c_int
        {
            write_motion_mode(cm, xd, mbmi, w);
        }
        if has_second_ref(mbmi) != 0 {
            let masked_compound_used: core::ffi::c_int = (is_any_masked_compound_used(
                bsize,
            ) != 0
                && (*(*cm).seq_params).enable_masked_compound as core::ffi::c_int != 0)
                as core::ffi::c_int;
            if masked_compound_used != 0 {
                let ctx_comp_group_idx: core::ffi::c_int = get_comp_group_idx_context(xd)
                    as core::ffi::c_int;
                aom_write_symbol(
                    w,
                    (*mbmi).comp_group_idx() as core::ffi::c_int,
                    ((*ec_ctx).comp_group_idx_cdf[ctx_comp_group_idx as usize])
                        .as_mut_ptr(),
                    2 as core::ffi::c_int,
                );
            } else {
                if !((*mbmi).comp_group_idx() as core::ffi::c_int
                    == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"pack_inter_mode_mvs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1207 as core::ffi::c_int,
                        b"mbmi->comp_group_idx == 0\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            }
            if (*mbmi).comp_group_idx() as core::ffi::c_int == 0 as core::ffi::c_int {
                if (*mbmi).compound_idx() != 0 {
                    if !((*mbmi).interinter_comp.type_0 as core::ffi::c_int
                        == COMPOUND_AVERAGE as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"pack_inter_mode_mvs\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            1212 as core::ffi::c_int,
                            b"mbmi->interinter_comp.type == COMPOUND_AVERAGE\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                if (*(*cm).seq_params).order_hint_info.enable_dist_wtd_comp != 0 {
                    let comp_index_ctx: core::ffi::c_int = get_comp_index_context(cm, xd)
                        as core::ffi::c_int;
                    aom_write_symbol(
                        w,
                        (*mbmi).compound_idx() as core::ffi::c_int,
                        ((*ec_ctx).compound_index_cdf[comp_index_ctx as usize])
                            .as_mut_ptr(),
                        2 as core::ffi::c_int,
                    );
                } else {
                    if !((*mbmi).compound_idx() as core::ffi::c_int
                        == 1 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"pack_inter_mode_mvs\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            1219 as core::ffi::c_int,
                            b"mbmi->compound_idx == 1\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else {};
                }
            } else {
                if !((*cpi).common.current_frame.reference_mode as core::ffi::c_int
                    != SINGLE_REFERENCE as core::ffi::c_int
                    && is_inter_compound_mode((*mbmi).mode) != 0
                    && (*mbmi).motion_mode as core::ffi::c_int
                        == SIMPLE_TRANSLATION as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"pack_inter_mode_mvs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1224 as core::ffi::c_int,
                        b"cpi->common.current_frame.reference_mode != SINGLE_REFERENCE && is_inter_compound_mode(mbmi->mode) && mbmi->motion_mode == SIMPLE_TRANSLATION\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if (masked_compound_used == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"pack_inter_mode_mvs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1225 as core::ffi::c_int,
                        b"masked_compound_used\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                if !((*mbmi).interinter_comp.type_0 as core::ffi::c_int
                    == COMPOUND_WEDGE as core::ffi::c_int
                    || (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                        == COMPOUND_DIFFWTD as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 20],
                            [core::ffi::c_char; 20],
                        >(*b"pack_inter_mode_mvs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1228 as core::ffi::c_int,
                        b"mbmi->interinter_comp.type == COMPOUND_WEDGE || mbmi->interinter_comp.type == COMPOUND_DIFFWTD\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if is_interinter_compound_used(
                    COMPOUND_WEDGE as core::ffi::c_int as COMPOUND_TYPE,
                    bsize,
                ) != 0
                {
                    aom_write_symbol(
                        w,
                        (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                            - COMPOUND_WEDGE as core::ffi::c_int,
                        ((*ec_ctx).compound_type_cdf[bsize as usize]).as_mut_ptr(),
                        MASKED_COMPOUND_TYPES as core::ffi::c_int,
                    );
                }
                if (*mbmi).interinter_comp.type_0 as core::ffi::c_int
                    == COMPOUND_WEDGE as core::ffi::c_int
                {
                    if (is_interinter_compound_used(
                        COMPOUND_WEDGE as core::ffi::c_int as COMPOUND_TYPE,
                        bsize,
                    ) == 0) as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"pack_inter_mode_mvs\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            1236 as core::ffi::c_int,
                            b"is_interinter_compound_used(COMPOUND_WEDGE, bsize)\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    aom_write_symbol(
                        w,
                        (*mbmi).interinter_comp.wedge_index as core::ffi::c_int,
                        ((*ec_ctx).wedge_idx_cdf[bsize as usize]).as_mut_ptr(),
                        MAX_WEDGE_TYPES,
                    );
                    aom_write_bit(
                        w,
                        (*mbmi).interinter_comp.wedge_sign as core::ffi::c_int,
                    );
                } else {
                    if !((*mbmi).interinter_comp.type_0 as core::ffi::c_int
                        == COMPOUND_DIFFWTD as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"pack_inter_mode_mvs\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            1241 as core::ffi::c_int,
                            b"mbmi->interinter_comp.type == COMPOUND_DIFFWTD\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    aom_write_literal(
                        w,
                        (*mbmi).interinter_comp.mask_type as core::ffi::c_int,
                        MAX_DIFFWTD_MASK_BITS,
                    );
                }
            }
        }
        write_mb_interp_filter(cm, td, w);
    };
}
#[inline]
unsafe extern "C-unwind" fn write_intrabc_info(
    mut xd: *mut MACROBLOCKD,
    mut mbmi_ext_frame: *const MB_MODE_INFO_EXT_FRAME,
    mut w: *mut aom_writer,
) {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mut use_intrabc: core::ffi::c_int = is_intrabc_block(mbmi);
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    aom_write_symbol(
        w,
        use_intrabc,
        ((*ec_ctx).intrabc_cdf).as_mut_ptr(),
        2 as core::ffi::c_int,
    );
    if use_intrabc != 0 {
        if !((*mbmi).mode as core::ffi::c_int == DC_PRED as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_intrabc_info\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1259 as core::ffi::c_int,
                b"mbmi->mode == DC_PRED\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*mbmi).uv_mode as core::ffi::c_int == UV_DC_PRED as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_intrabc_info\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1260 as core::ffi::c_int,
                b"mbmi->uv_mode == UV_DC_PRED\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*mbmi).motion_mode as core::ffi::c_int
            == SIMPLE_TRANSLATION as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_intrabc_info\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1261 as core::ffi::c_int,
                b"mbmi->motion_mode == SIMPLE_TRANSLATION\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let mut dv_ref: int_mv = (*mbmi_ext_frame)
            .ref_mv_stack[0 as core::ffi::c_int as usize]
            .this_mv;
        av1_encode_dv(
            w,
            &(*((*mbmi).mv).as_ptr().offset(0 as core::ffi::c_int as isize)).as_mv,
            &raw mut dv_ref.as_mv,
            &raw mut (*ec_ctx).ndvc,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_mb_modes_kf(
    mut cpi: *mut AV1_COMP,
    mut xd: *mut MACROBLOCKD,
    mut mbmi_ext_frame: *const MB_MODE_INFO_EXT_FRAME,
    mut w: *mut aom_writer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let seg: *const segmentation = &raw mut (*cm).seg;
    let segp: *mut segmentation_probs = &raw mut (*ec_ctx).seg;
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*seg).segid_preskip as core::ffi::c_int != 0
        && (*seg).update_map as core::ffi::c_int != 0
    {
        write_segment_id(cpi, xd, mbmi, w, seg, segp, 0 as core::ffi::c_int);
    }
    let skip: core::ffi::c_int = write_skip(cm, xd, (*mbmi).segment_id(), mbmi, w)
        as core::ffi::c_int;
    if (*seg).segid_preskip == 0 && (*seg).update_map as core::ffi::c_int != 0 {
        write_segment_id(cpi, xd, mbmi, w, seg, segp, skip);
    }
    write_cdef(cm, xd, w, skip);
    write_delta_q_params(cm, xd, skip, w);
    if av1_allow_intrabc(cm) != 0 {
        write_intrabc_info(xd, mbmi_ext_frame, w);
        if is_intrabc_block(mbmi) != 0 {
            return;
        }
    }
    write_intra_prediction_modes(cm, xd, 1 as core::ffi::c_int, w);
}
#[inline]
unsafe extern "C-unwind" fn write_mbmi_b(
    mut cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    mut w: *mut aom_writer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    let mut m: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if frame_is_intra_only(cm) != 0 {
        write_mb_modes_kf(cpi, xd, (*td).mb.mbmi_ext_frame, w);
    } else {
        set_ref_ptrs(
            cm,
            xd,
            (*m).ref_frame[0 as core::ffi::c_int as usize],
            (*m).ref_frame[1 as core::ffi::c_int as usize],
        );
        pack_inter_mode_mvs(cpi, td, w);
    };
}
#[inline]
unsafe extern "C-unwind" fn write_inter_txb_coeff(
    cm: *mut AV1_COMMON,
    x: *mut MACROBLOCK,
    mbmi: *mut MB_MODE_INFO,
    mut w: *mut aom_writer,
    mut tok: *mut *const TokenExtra,
    tok_end: *const TokenExtra,
    mut token_stats: *mut TOKEN_STATS,
    row: core::ffi::c_int,
    col: core::ffi::c_int,
    mut block: *mut core::ffi::c_int,
    plane: core::ffi::c_int,
) {
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"write_inter_txb_coeff\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1403 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(bsize, ss_x, ss_y) as BLOCK_SIZE;
    if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"write_inter_txb_coeff\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1407 as core::ffi::c_int,
            b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_tx_size: TX_SIZE = get_vartx_max_txsize(xd, plane_bsize, plane) as TX_SIZE;
    let step: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize]
        * tx_size_high_unit[max_tx_size as usize];
    let bkw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
    let bkh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
    let max_unit_bsize: BLOCK_SIZE = get_plane_block_size(BLOCK_64X64, ss_x, ss_y)
        as BLOCK_SIZE;
    let num_4x4_w: core::ffi::c_int = mi_size_wide[plane_bsize as usize]
        as core::ffi::c_int;
    let num_4x4_h: core::ffi::c_int = mi_size_high[plane_bsize as usize]
        as core::ffi::c_int;
    let mu_blocks_wide: core::ffi::c_int = mi_size_wide[max_unit_bsize as usize]
        as core::ffi::c_int;
    let mu_blocks_high: core::ffi::c_int = mi_size_high[max_unit_bsize as usize]
        as core::ffi::c_int;
    let unit_height: core::ffi::c_int = if mu_blocks_high + (row >> ss_y) < num_4x4_h {
        mu_blocks_high + (row >> ss_y)
    } else {
        num_4x4_h
    };
    let unit_width: core::ffi::c_int = if mu_blocks_wide + (col >> ss_x) < num_4x4_w {
        mu_blocks_wide + (col >> ss_x)
    } else {
        num_4x4_w
    };
    let mut blk_row: core::ffi::c_int = row >> ss_y;
    while blk_row < unit_height {
        let mut blk_col: core::ffi::c_int = col >> ss_x;
        while blk_col < unit_width {
            pack_txb_tokens(
                w,
                cm,
                x,
                tok,
                tok_end,
                xd,
                mbmi,
                plane,
                plane_bsize,
                (*(*cm).seq_params).bit_depth,
                *block,
                blk_row,
                blk_col,
                max_tx_size,
                token_stats,
            );
            *block += step;
            blk_col += bkw;
        }
        blk_row += bkh;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_tokens_b(
    mut cpi: *mut AV1_COMP,
    x: *mut MACROBLOCK,
    mut w: *mut aom_writer,
    mut tok: *mut *const TokenExtra,
    tok_end: *const TokenExtra,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*x).e_mbd;
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if ((*mbmi).skip_txfm != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"write_tokens_b\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1439 as core::ffi::c_int,
            b"!mbmi->skip_txfm\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let is_inter: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    if is_inter == 0 {
        av1_write_intra_coeffs_mb(cm, x, w, bsize);
    } else {
        let mut block: [core::ffi::c_int; 3] = [0 as core::ffi::c_int; 3];
        if !(bsize as core::ffi::c_int
            == get_plane_block_size(
                bsize,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_x,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_y,
            ) as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"write_tokens_b\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1447 as core::ffi::c_int,
                b"bsize == get_plane_block_size(bsize, xd->plane[0].subsampling_x, xd->plane[0].subsampling_y)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let num_4x4_w: core::ffi::c_int = mi_size_wide[bsize as usize]
            as core::ffi::c_int;
        let num_4x4_h: core::ffi::c_int = mi_size_high[bsize as usize]
            as core::ffi::c_int;
        let mut token_stats: TOKEN_STATS = TOKEN_STATS { cost: 0 };
        init_token_stats(&raw mut token_stats);
        let max_unit_bsize: BLOCK_SIZE = BLOCK_64X64;
        if !(max_unit_bsize as core::ffi::c_int
            == get_plane_block_size(
                BLOCK_64X64,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_x,
                (*xd).plane[0 as core::ffi::c_int as usize].subsampling_y,
            ) as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"write_tokens_b\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1456 as core::ffi::c_int,
                b"max_unit_bsize == get_plane_block_size(BLOCK_64X64, xd->plane[0].subsampling_x, xd->plane[0].subsampling_y)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut mu_blocks_wide: core::ffi::c_int = mi_size_wide[max_unit_bsize as usize]
            as core::ffi::c_int;
        let mut mu_blocks_high: core::ffi::c_int = mi_size_high[max_unit_bsize as usize]
            as core::ffi::c_int;
        mu_blocks_wide = if num_4x4_w < mu_blocks_wide {
            num_4x4_w
        } else {
            mu_blocks_wide
        };
        mu_blocks_high = if num_4x4_h < mu_blocks_high {
            num_4x4_h
        } else {
            mu_blocks_high
        };
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < num_4x4_h {
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < num_4x4_w {
                let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
                while plane < num_planes {
                    if plane != 0 && !(*xd).is_chroma_ref {
                        break;
                    }
                    write_inter_txb_coeff(
                        cm,
                        x,
                        mbmi,
                        w,
                        tok,
                        tok_end,
                        &raw mut token_stats,
                        row,
                        col,
                        &raw mut *block.as_mut_ptr().offset(plane as isize),
                        plane,
                    );
                    plane += 1;
                }
                col += mu_blocks_wide;
            }
            row += mu_blocks_high;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn write_modes_b(
    mut cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    tile: *const TileInfo,
    mut w: *mut aom_writer,
    mut tok: *mut *const TokenExtra,
    tok_end: *const TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mut cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mut xd: *mut MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    let mut tile_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    let grid_idx: core::ffi::c_int = mi_row * (*mi_params).mi_stride + mi_col;
    (*xd).mi = ((*mi_params).mi_grid_base).offset(grid_idx as isize);
    (*td).mb.mbmi_ext_frame = ((*cpi).mbmi_ext_info.frame_base)
        .offset(
            get_mi_ext_idx(
                mi_row,
                mi_col,
                (*cm).mi_params.mi_alloc_bsize,
                (*cpi).mbmi_ext_info.stride,
            ) as isize,
        );
    (*xd).tx_type_map = ((*mi_params).tx_type_map).offset(grid_idx as isize)
        as *mut uint8_t;
    (*xd).tx_type_map_stride = (*mi_params).mi_stride;
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if !(bsize as core::ffi::c_int <= (*(*cm).seq_params).sb_size as core::ffi::c_int
        || bsize as core::ffi::c_int >= BLOCK_SIZES as core::ffi::c_int
            && (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_modes_b\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1505 as core::ffi::c_int,
            b"bsize <= cm->seq_params->sb_size || (bsize >= BLOCK_SIZES && bsize < BLOCK_SIZES_ALL)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let bh: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
    let bw: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
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
    (*xd).above_txfm_context = (*((*cm).above_contexts.txfm)
        .offset((*tile).tile_row as isize))
        .offset(mi_col as isize);
    (*xd).left_txfm_context = ((*xd).left_txfm_context_buffer)
        .as_mut_ptr()
        .offset((mi_row & MAX_MIB_MASK) as isize);
    write_mbmi_b(cpi, td, w);
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane
        < (if (2 as core::ffi::c_int) < av1_num_planes(cm) {
            2 as core::ffi::c_int
        } else {
            av1_num_planes(cm)
        })
    {
        let palette_size_plane: uint8_t = (*mbmi)
            .palette_mode_info
            .palette_size[plane as usize];
        if !((*mbmi).skip_mode() == 0 || palette_size_plane == 0) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"write_modes_b\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1521 as core::ffi::c_int,
                b"!mbmi->skip_mode || !palette_size_plane\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if palette_size_plane as core::ffi::c_int > 0 as core::ffi::c_int {
            if !((*mbmi).use_intrabc() as core::ffi::c_int == 0 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"write_modes_b\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1523 as core::ffi::c_int,
                    b"mbmi->use_intrabc == 0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if (av1_allow_palette(
                (*cm).features.allow_screen_content_tools as core::ffi::c_int,
                (*mbmi).bsize,
            ) == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"write_modes_b\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1525 as core::ffi::c_int,
                    b"av1_allow_palette(cm->features.allow_screen_content_tools, mbmi->bsize)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(plane == 0 || (*xd).is_chroma_ref as core::ffi::c_int != 0)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"write_modes_b\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1526 as core::ffi::c_int,
                    b"!plane || xd->is_chroma_ref\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let mut rows: core::ffi::c_int = 0;
            let mut cols: core::ffi::c_int = 0;
            av1_get_block_dimensions(
                (*mbmi).bsize,
                plane,
                xd,
                0 as *mut core::ffi::c_int,
                0 as *mut core::ffi::c_int,
                &raw mut rows,
                &raw mut cols,
            );
            if !(*tok < tok_end) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"write_modes_b\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1530 as core::ffi::c_int,
                    b"*tok < tok_end\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut map_pb_cdf: MapCdf = if plane != 0 {
                ((*tile_ctx).palette_uv_color_index_cdf).as_mut_ptr() as MapCdf
            } else {
                ((*tile_ctx).palette_y_color_index_cdf).as_mut_ptr() as MapCdf
            };
            pack_map_tokens(
                w,
                tok,
                palette_size_plane as core::ffi::c_int,
                rows * cols,
                map_pb_cdf,
            );
        }
        plane += 1;
    }
    let is_inter_tx: core::ffi::c_int = is_inter_block(mbmi) as core::ffi::c_int;
    let skip_txfm: core::ffi::c_int = (*mbmi).skip_txfm as core::ffi::c_int;
    let segment_id: uint8_t = (*mbmi).segment_id();
    if (*cm).features.tx_mode as core::ffi::c_int == TX_MODE_SELECT as core::ffi::c_int
        && block_signals_txsize(bsize) != 0 && !(is_inter_tx != 0 && skip_txfm != 0)
        && (*xd).lossless[segment_id as usize] == 0
    {
        if is_inter_tx != 0 {
            let max_tx_size: TX_SIZE = get_vartx_max_txsize(
                xd,
                bsize,
                0 as core::ffi::c_int,
            ) as TX_SIZE;
            let txbh: core::ffi::c_int = tx_size_high_unit[max_tx_size as usize];
            let txbw: core::ffi::c_int = tx_size_wide_unit[max_tx_size as usize];
            let width: core::ffi::c_int = mi_size_wide[bsize as usize]
                as core::ffi::c_int;
            let height: core::ffi::c_int = mi_size_high[bsize as usize]
                as core::ffi::c_int;
            let mut idy: core::ffi::c_int = 0 as core::ffi::c_int;
            while idy < height {
                let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
                while idx < width {
                    write_tx_size_vartx(
                        xd,
                        mbmi,
                        max_tx_size,
                        0 as core::ffi::c_int,
                        idy,
                        idx,
                        w,
                    );
                    idx += txbw;
                }
                idy += txbh;
            }
        } else {
            write_selected_tx_size(xd, w);
            set_txfm_ctxs(
                (*mbmi).tx_size,
                (*xd).width as core::ffi::c_int,
                (*xd).height as core::ffi::c_int,
                0 as core::ffi::c_int,
                xd,
            );
        }
    } else {
        set_txfm_ctxs(
            (*mbmi).tx_size,
            (*xd).width as core::ffi::c_int,
            (*xd).height as core::ffi::c_int,
            (skip_txfm != 0 && is_inter_tx != 0) as core::ffi::c_int,
            xd,
        );
    }
    if (*mbmi).skip_txfm == 0 {
        let mut start: core::ffi::c_int = aom_tell_size(w);
        write_tokens_b(cpi, &raw mut (*td).mb, w, tok, tok_end);
        let end: core::ffi::c_int = aom_tell_size(w) as core::ffi::c_int;
        (*td).coefficient_size += end - start;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_partition(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    mut hbs: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut p: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
    mut w: *mut aom_writer,
) {
    let is_partition_point: core::ffi::c_int = (bsize as core::ffi::c_int
        >= BLOCK_8X8 as core::ffi::c_int) as core::ffi::c_int;
    if is_partition_point == 0 {
        return;
    }
    let has_rows: core::ffi::c_int = (mi_row + hbs < (*cm).mi_params.mi_rows)
        as core::ffi::c_int;
    let has_cols: core::ffi::c_int = (mi_col + hbs < (*cm).mi_params.mi_cols)
        as core::ffi::c_int;
    let ctx: core::ffi::c_int = partition_plane_context(xd, mi_row, mi_col, bsize)
        as core::ffi::c_int;
    let mut ec_ctx: *mut FRAME_CONTEXT = (*xd).tile_ctx;
    if has_rows == 0 && has_cols == 0 {
        if !(p as core::ffi::c_int == PARTITION_SPLIT as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1586 as core::ffi::c_int,
                b"p == PARTITION_SPLIT\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return;
    }
    if has_rows != 0 && has_cols != 0 {
        aom_write_symbol(
            w,
            p as core::ffi::c_int,
            ((*ec_ctx).partition_cdf[ctx as usize]).as_mut_ptr(),
            partition_cdf_length(bsize),
        );
    } else if has_rows == 0 && has_cols != 0 {
        if !(p as core::ffi::c_int == PARTITION_SPLIT as core::ffi::c_int
            || p as core::ffi::c_int == PARTITION_HORZ as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1594 as core::ffi::c_int,
                b"p == PARTITION_SPLIT || p == PARTITION_HORZ\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !(bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1595 as core::ffi::c_int,
                b"bsize > BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut cdf: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_vert_alike(
            cdf.as_mut_ptr(),
            ((*ec_ctx).partition_cdf[ctx as usize]).as_mut_ptr(),
            bsize,
        );
        aom_write_cdf(
            w,
            (p as core::ffi::c_int == PARTITION_SPLIT as core::ffi::c_int)
                as core::ffi::c_int,
            cdf.as_mut_ptr(),
            2 as core::ffi::c_int,
        );
    } else {
        if !(has_rows != 0 && has_cols == 0) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1600 as core::ffi::c_int,
                b"has_rows && !has_cols\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(p as core::ffi::c_int == PARTITION_SPLIT as core::ffi::c_int
            || p as core::ffi::c_int == PARTITION_VERT as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1601 as core::ffi::c_int,
                b"p == PARTITION_SPLIT || p == PARTITION_VERT\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !(bsize as core::ffi::c_int > BLOCK_8X8 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"write_partition\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1602 as core::ffi::c_int,
                b"bsize > BLOCK_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut cdf_0: [aom_cdf_prob; 2] = [0; 2];
        partition_gather_horz_alike(
            cdf_0.as_mut_ptr(),
            ((*ec_ctx).partition_cdf[ctx as usize]).as_mut_ptr(),
            bsize,
        );
        aom_write_cdf(
            w,
            (p as core::ffi::c_int == PARTITION_SPLIT as core::ffi::c_int)
                as core::ffi::c_int,
            cdf_0.as_mut_ptr(),
            2 as core::ffi::c_int,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn write_modes_sb(
    cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    tile: *const TileInfo,
    w: *mut aom_writer,
    mut tok: *mut *const TokenExtra,
    tok_end: *const TokenExtra,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let xd: *mut MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"write_modes_sb\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1617 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let hbs: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
        / 2 as core::ffi::c_int;
    let quarter_step: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int
        / 4 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let partition: PARTITION_TYPE = get_partition(cm, mi_row, mi_col, bsize)
        as PARTITION_TYPE;
    let subsize: BLOCK_SIZE = get_partition_subsize(bsize, partition) as BLOCK_SIZE;
    if mi_row >= (*mi_params).mi_rows || mi_col >= (*mi_params).mi_cols {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let mut rcol0: core::ffi::c_int = 0;
        let mut rcol1: core::ffi::c_int = 0;
        let mut rrow0: core::ffi::c_int = 0;
        let mut rrow1: core::ffi::c_int = 0;
        if !((*cm).rst_info[plane as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            if av1_loop_restoration_corners_in_sb(
                cm as *const AV1Common,
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
                        loop_restoration_write_sb_coeffs(
                            cm,
                            xd,
                            runit_idx,
                            w,
                            plane,
                            (*td).counts,
                        );
                        rcol += 1;
                    }
                    rrow += 1;
                }
            }
        }
        plane += 1;
    }
    write_partition(cm, xd, hbs, mi_row, mi_col, partition, bsize, w);
    match partition as core::ffi::c_int {
        0 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
        }
        1 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            if mi_row + hbs < (*mi_params).mi_rows {
                write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col);
            }
        }
        2 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            if mi_col + hbs < (*mi_params).mi_cols {
                write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col + hbs);
            }
        }
        3 => {
            write_modes_sb(cpi, td, tile, w, tok, tok_end, mi_row, mi_col, subsize);
            write_modes_sb(
                cpi,
                td,
                tile,
                w,
                tok,
                tok_end,
                mi_row,
                mi_col + hbs,
                subsize,
            );
            write_modes_sb(
                cpi,
                td,
                tile,
                w,
                tok,
                tok_end,
                mi_row + hbs,
                mi_col,
                subsize,
            );
            write_modes_sb(
                cpi,
                td,
                tile,
                w,
                tok,
                tok_end,
                mi_row + hbs,
                mi_col + hbs,
                subsize,
            );
        }
        4 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col + hbs);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col);
        }
        5 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col + hbs);
        }
        6 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col + hbs);
        }
        7 => {
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, mi_col + hbs);
            write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row + hbs, mi_col + hbs);
        }
        8 => {
            i = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                let mut this_mi_row: core::ffi::c_int = mi_row + i * quarter_step;
                if i > 0 as core::ffi::c_int && this_mi_row >= (*mi_params).mi_rows {
                    break;
                }
                write_modes_b(cpi, td, tile, w, tok, tok_end, this_mi_row, mi_col);
                i += 1;
            }
        }
        9 => {
            i = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                let mut this_mi_col: core::ffi::c_int = mi_col + i * quarter_step;
                if i > 0 as core::ffi::c_int && this_mi_col >= (*mi_params).mi_cols {
                    break;
                }
                write_modes_b(cpi, td, tile, w, tok, tok_end, mi_row, this_mi_col);
                i += 1;
            }
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 15],
                        [core::ffi::c_char; 15],
                    >(*b"write_modes_sb\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1708 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    update_ext_partition_context(xd, mi_row, mi_col, subsize, bsize, partition);
}
#[inline]
unsafe extern "C-unwind" fn get_token_pointers(
    mut token_info: *const TokenInfo,
    tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
    sb_row_in_tile: core::ffi::c_int,
    mut tok: *mut *const TokenExtra,
    mut tok_end: *mut *const TokenExtra,
) {
    if !is_token_info_allocated(token_info) {
        *tok = 0 as *const TokenExtra;
        *tok_end = 0 as *const TokenExtra;
        return;
    }
    *tok = (*((*token_info).tplist[tile_row as usize][tile_col as usize])
        .offset(sb_row_in_tile as isize))
        .start;
    *tok_end = (*tok)
        .offset(
            (*((*token_info).tplist[tile_row as usize][tile_col as usize])
                .offset(sb_row_in_tile as isize))
                .count as isize,
        );
}
#[inline]
unsafe extern "C-unwind" fn write_modes(
    cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    tile: *const TileInfo,
    w: *mut aom_writer,
    mut tile_row: core::ffi::c_int,
    mut tile_col: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*td).mb.e_mbd;
    let mi_row_start: core::ffi::c_int = (*tile).mi_row_start;
    let mi_row_end: core::ffi::c_int = (*tile).mi_row_end;
    let mi_col_start: core::ffi::c_int = (*tile).mi_col_start;
    let mi_col_end: core::ffi::c_int = (*tile).mi_col_end;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    av1_zero_above_context(cm, xd, mi_col_start, mi_col_end, (*tile).tile_row);
    av1_init_above_context(&raw mut (*cm).above_contexts, num_planes, (*tile).tile_row, xd);
    if (*cpi).common.delta_q_info.delta_q_present_flag != 0 {
        (*xd).current_base_qindex = (*cpi).common.quant_params.base_qindex;
        if (*cpi).common.delta_q_info.delta_lf_present_flag != 0 {
            av1_reset_loop_filter_delta(xd, num_planes);
        }
    }
    let mut mi_row: core::ffi::c_int = mi_row_start;
    while mi_row < mi_row_end {
        let sb_row_in_tile: core::ffi::c_int = mi_row - (*tile).mi_row_start
            >> (*(*cm).seq_params).mib_size_log2;
        let mut token_info: *const TokenInfo = &raw mut (*cpi).token_info;
        let mut tok: *const TokenExtra = 0 as *const TokenExtra;
        let mut tok_end: *const TokenExtra = 0 as *const TokenExtra;
        get_token_pointers(
            token_info,
            tile_row,
            tile_col,
            sb_row_in_tile,
            &raw mut tok,
            &raw mut tok_end,
        );
        av1_zero_left_context(xd);
        let mut mi_col: core::ffi::c_int = mi_col_start;
        while mi_col < mi_col_end {
            (*td).mb.cb_coef_buff = av1_get_cb_coeff_buffer(cpi, mi_row, mi_col);
            write_modes_sb(
                cpi,
                td,
                tile,
                w,
                &raw mut tok,
                tok_end,
                mi_row,
                mi_col,
                (*(*cm).seq_params).sb_size,
            );
            mi_col += (*(*cm).seq_params).mib_size;
        }
        if !(tok == tok_end) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"write_modes\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1770 as core::ffi::c_int,
                b"tok == tok_end\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        mi_row += (*(*cm).seq_params).mib_size;
    }
}
#[inline]
unsafe extern "C-unwind" fn encode_restoration_mode(
    mut cm: *mut AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    if (*cm).features.all_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"encode_restoration_mode\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1776 as core::ffi::c_int,
            b"!cm->features.all_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*(*cm).seq_params).enable_restoration == 0 {
        return;
    }
    if (*cm).features.allow_intrabc {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut all_none: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut chroma_none: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < num_planes {
        let mut rsi: *mut RestorationInfo = &raw mut *((*cm).rst_info)
            .as_mut_ptr()
            .offset(p as isize) as *mut RestorationInfo;
        if (*rsi).frame_restoration_type as core::ffi::c_uint
            != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            all_none = 0 as core::ffi::c_int;
            chroma_none &= (p == 0 as core::ffi::c_int) as core::ffi::c_int;
        }
        match (*rsi).frame_restoration_type as core::ffi::c_uint {
            0 => {
                aom_wb_write_bit(wb, 0 as core::ffi::c_int);
                aom_wb_write_bit(wb, 0 as core::ffi::c_int);
            }
            1 => {
                aom_wb_write_bit(wb, 1 as core::ffi::c_int);
                aom_wb_write_bit(wb, 0 as core::ffi::c_int);
            }
            2 => {
                aom_wb_write_bit(wb, 1 as core::ffi::c_int);
                aom_wb_write_bit(wb, 1 as core::ffi::c_int);
            }
            3 => {
                aom_wb_write_bit(wb, 0 as core::ffi::c_int);
                aom_wb_write_bit(wb, 1 as core::ffi::c_int);
            }
            _ => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 24],
                            [core::ffi::c_char; 24],
                        >(*b"encode_restoration_mode\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1804 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
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
                >(*b"encode_restoration_mode\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1809 as core::ffi::c_int,
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
        let mut rsi_0: *mut RestorationInfo = &raw mut *((*cm).rst_info)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut RestorationInfo;
        if !((*rsi_0).restoration_unit_size >= sb_size) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"encode_restoration_mode\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1814 as core::ffi::c_int,
                b"rsi->restoration_unit_size >= sb_size\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !(256 as core::ffi::c_int == 256 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"encode_restoration_mode\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1815 as core::ffi::c_int,
                b"RESTORATION_UNITSIZE_MAX == 256\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if sb_size == 64 as core::ffi::c_int {
            aom_wb_write_bit(
                wb,
                ((*rsi_0).restoration_unit_size > 64 as core::ffi::c_int)
                    as core::ffi::c_int,
            );
        }
        if (*rsi_0).restoration_unit_size > 64 as core::ffi::c_int {
            aom_wb_write_bit(
                wb,
                ((*rsi_0).restoration_unit_size > 128 as core::ffi::c_int)
                    as core::ffi::c_int,
            );
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
            aom_wb_write_bit(
                wb,
                ((*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size
                    != (*cm)
                        .rst_info[0 as core::ffi::c_int as usize]
                        .restoration_unit_size) as core::ffi::c_int,
            );
            if !((*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size
                == (*cm).rst_info[0 as core::ffi::c_int as usize].restoration_unit_size
                || (*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size
                    == (*cm)
                        .rst_info[0 as core::ffi::c_int as usize]
                        .restoration_unit_size >> s) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"encode_restoration_mode\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1834 as core::ffi::c_int,
                    b"cm->rst_info[1].restoration_unit_size == cm->rst_info[0].restoration_unit_size || cm->rst_info[1].restoration_unit_size == (cm->rst_info[0].restoration_unit_size >> s)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !((*cm).rst_info[2 as core::ffi::c_int as usize].restoration_unit_size
                == (*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"encode_restoration_mode\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1836 as core::ffi::c_int,
                    b"cm->rst_info[2].restoration_unit_size == cm->rst_info[1].restoration_unit_size\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else if s == 0 {
            if !((*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size
                == (*cm).rst_info[0 as core::ffi::c_int as usize].restoration_unit_size)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"encode_restoration_mode\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1839 as core::ffi::c_int,
                    b"cm->rst_info[1].restoration_unit_size == cm->rst_info[0].restoration_unit_size\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !((*cm).rst_info[2 as core::ffi::c_int as usize].restoration_unit_size
                == (*cm).rst_info[1 as core::ffi::c_int as usize].restoration_unit_size)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"encode_restoration_mode\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    1841 as core::ffi::c_int,
                    b"cm->rst_info[2].restoration_unit_size == cm->rst_info[1].restoration_unit_size\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_wiener_filter(
    mut wiener_win: core::ffi::c_int,
    mut wiener_info: *const WienerInfo,
    mut ref_wiener_info: *mut WienerInfo,
    mut wb: *mut aom_writer,
) {
    if wiener_win == WIENER_WIN {
        aom_write_primitive_refsubexpfin(
            wb,
            (WIENER_FILT_TAP0_MAXV - WIENER_FILT_TAP0_MINV + 1 as core::ffi::c_int)
                as uint16_t,
            WIENER_FILT_TAP0_SUBEXP_K as uint16_t,
            ((*ref_wiener_info).vfilter[0 as core::ffi::c_int as usize]
                as core::ffi::c_int - WIENER_FILT_TAP0_MINV) as uint16_t,
            ((*wiener_info).vfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
                - WIENER_FILT_TAP0_MINV) as uint16_t,
        );
    } else {
        if !((*wiener_info).vfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*wiener_info)
                .vfilter[(2 as core::ffi::c_int * 3 as core::ffi::c_int
                + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int == 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"write_wiener_filter\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1859 as core::ffi::c_int,
                b"wiener_info->vfilter[0] == 0 && wiener_info->vfilter[WIENER_WIN - 1] == 0\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    aom_write_primitive_refsubexpfin(
        wb,
        (WIENER_FILT_TAP1_MAXV - WIENER_FILT_TAP1_MINV + 1 as core::ffi::c_int)
            as uint16_t,
        WIENER_FILT_TAP1_SUBEXP_K as uint16_t,
        ((*ref_wiener_info).vfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP1_MINV) as uint16_t,
        ((*wiener_info).vfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP1_MINV) as uint16_t,
    );
    aom_write_primitive_refsubexpfin(
        wb,
        (WIENER_FILT_TAP2_MAXV - WIENER_FILT_TAP2_MINV + 1 as core::ffi::c_int)
            as uint16_t,
        WIENER_FILT_TAP2_SUBEXP_K as uint16_t,
        ((*ref_wiener_info).vfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP2_MINV) as uint16_t,
        ((*wiener_info).vfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP2_MINV) as uint16_t,
    );
    if wiener_win == WIENER_WIN {
        aom_write_primitive_refsubexpfin(
            wb,
            (WIENER_FILT_TAP0_MAXV - WIENER_FILT_TAP0_MINV + 1 as core::ffi::c_int)
                as uint16_t,
            WIENER_FILT_TAP0_SUBEXP_K as uint16_t,
            ((*ref_wiener_info).hfilter[0 as core::ffi::c_int as usize]
                as core::ffi::c_int - WIENER_FILT_TAP0_MINV) as uint16_t,
            ((*wiener_info).hfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
                - WIENER_FILT_TAP0_MINV) as uint16_t,
        );
    } else {
        if !((*wiener_info).hfilter[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*wiener_info)
                .hfilter[(2 as core::ffi::c_int * 3 as core::ffi::c_int
                + 1 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int == 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"write_wiener_filter\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1878 as core::ffi::c_int,
                b"wiener_info->hfilter[0] == 0 && wiener_info->hfilter[WIENER_WIN - 1] == 0\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    aom_write_primitive_refsubexpfin(
        wb,
        (WIENER_FILT_TAP1_MAXV - WIENER_FILT_TAP1_MINV + 1 as core::ffi::c_int)
            as uint16_t,
        WIENER_FILT_TAP1_SUBEXP_K as uint16_t,
        ((*ref_wiener_info).hfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP1_MINV) as uint16_t,
        ((*wiener_info).hfilter[1 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP1_MINV) as uint16_t,
    );
    aom_write_primitive_refsubexpfin(
        wb,
        (WIENER_FILT_TAP2_MAXV - WIENER_FILT_TAP2_MINV + 1 as core::ffi::c_int)
            as uint16_t,
        WIENER_FILT_TAP2_SUBEXP_K as uint16_t,
        ((*ref_wiener_info).hfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP2_MINV) as uint16_t,
        ((*wiener_info).hfilter[2 as core::ffi::c_int as usize] as core::ffi::c_int
            - WIENER_FILT_TAP2_MINV) as uint16_t,
    );
    *ref_wiener_info = *wiener_info;
}
#[inline]
unsafe extern "C-unwind" fn write_sgrproj_filter(
    mut sgrproj_info: *const SgrprojInfo,
    mut ref_sgrproj_info: *mut SgrprojInfo,
    mut wb: *mut aom_writer,
) {
    aom_write_literal(wb, (*sgrproj_info).ep, SGRPROJ_PARAMS_BITS);
    let mut params: *const sgr_params_type = &*av1_sgr_params
        .as_ptr()
        .offset((*sgrproj_info).ep as isize) as *const sgr_params_type;
    if (*params).r[0 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        if !((*sgrproj_info).xqd[0 as core::ffi::c_int as usize]
            == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"write_sgrproj_filter\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                1899 as core::ffi::c_int,
                b"sgrproj_info->xqd[0] == 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        aom_write_primitive_refsubexpfin(
            wb,
            (SGRPROJ_PRJ_MAX1 - SGRPROJ_PRJ_MIN1 + 1 as core::ffi::c_int) as uint16_t,
            SGRPROJ_PRJ_SUBEXP_K as uint16_t,
            ((*ref_sgrproj_info).xqd[1 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN1)
                as uint16_t,
            ((*sgrproj_info).xqd[1 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN1)
                as uint16_t,
        );
    } else if (*params).r[1 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        aom_write_primitive_refsubexpfin(
            wb,
            (SGRPROJ_PRJ_MAX0 - SGRPROJ_PRJ_MIN0 + 1 as core::ffi::c_int) as uint16_t,
            SGRPROJ_PRJ_SUBEXP_K as uint16_t,
            ((*ref_sgrproj_info).xqd[0 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN0)
                as uint16_t,
            ((*sgrproj_info).xqd[0 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN0)
                as uint16_t,
        );
    } else {
        aom_write_primitive_refsubexpfin(
            wb,
            (SGRPROJ_PRJ_MAX0 - SGRPROJ_PRJ_MIN0 + 1 as core::ffi::c_int) as uint16_t,
            SGRPROJ_PRJ_SUBEXP_K as uint16_t,
            ((*ref_sgrproj_info).xqd[0 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN0)
                as uint16_t,
            ((*sgrproj_info).xqd[0 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN0)
                as uint16_t,
        );
        aom_write_primitive_refsubexpfin(
            wb,
            (SGRPROJ_PRJ_MAX1 - SGRPROJ_PRJ_MIN1 + 1 as core::ffi::c_int) as uint16_t,
            SGRPROJ_PRJ_SUBEXP_K as uint16_t,
            ((*ref_sgrproj_info).xqd[1 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN1)
                as uint16_t,
            ((*sgrproj_info).xqd[1 as core::ffi::c_int as usize] - SGRPROJ_PRJ_MIN1)
                as uint16_t,
        );
    }
    *ref_sgrproj_info = *sgrproj_info;
}
#[inline]
unsafe extern "C-unwind" fn loop_restoration_write_sb_coeffs(
    cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut runit_idx: core::ffi::c_int,
    w: *mut aom_writer,
    mut plane: core::ffi::c_int,
    mut counts: *mut FRAME_COUNTS,
) {
    let mut rui: *const RestorationUnitInfo = &raw mut *((*((*cm).rst_info)
        .as_ptr()
        .offset(plane as isize))
        .unit_info)
        .offset(runit_idx as isize) as *mut RestorationUnitInfo;
    let mut rsi: *const RestorationInfo = ((*cm).rst_info)
        .as_ptr()
        .offset(plane as isize);
    let mut frame_rtype: RestorationType = (*rsi).frame_restoration_type;
    if !(frame_rtype as core::ffi::c_uint
        != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"loop_restoration_write_sb_coeffs\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1929 as core::ffi::c_int,
            b"frame_rtype != RESTORE_NONE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*cm).features.all_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"loop_restoration_write_sb_coeffs\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            1932 as core::ffi::c_int,
            b"!cm->features.all_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let wiener_win: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
        WIENER_WIN_CHROMA
    } else {
        WIENER_WIN
    };
    let mut ref_wiener_info: *mut WienerInfo = &raw mut *((*xd).wiener_info)
        .as_mut_ptr()
        .offset(plane as isize) as *mut WienerInfo;
    let mut ref_sgrproj_info: *mut SgrprojInfo = &raw mut *((*xd).sgrproj_info)
        .as_mut_ptr()
        .offset(plane as isize) as *mut SgrprojInfo;
    let mut unit_rtype: RestorationType = (*rui).restoration_type;
    if frame_rtype as core::ffi::c_uint
        == RESTORE_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
    {
        aom_write_symbol(
            w,
            unit_rtype as core::ffi::c_int,
            ((*(*xd).tile_ctx).switchable_restore_cdf).as_mut_ptr(),
            RESTORE_SWITCHABLE_TYPES as core::ffi::c_int,
        );
        match unit_rtype as core::ffi::c_uint {
            1 => {
                write_wiener_filter(wiener_win, &(*rui).wiener_info, ref_wiener_info, w);
            }
            2 => {
                write_sgrproj_filter(&(*rui).sgrproj_info, ref_sgrproj_info, w);
            }
            _ => {
                if !(unit_rtype as core::ffi::c_uint
                    == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 33],
                            [core::ffi::c_char; 33],
                        >(*b"loop_restoration_write_sb_coeffs\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        1964 as core::ffi::c_int,
                        b"unit_rtype == RESTORE_NONE\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
    } else if frame_rtype as core::ffi::c_uint
        == RESTORE_WIENER as core::ffi::c_int as core::ffi::c_uint
    {
        aom_write_symbol(
            w,
            (unit_rtype as core::ffi::c_uint
                != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int,
            ((*(*xd).tile_ctx).wiener_restore_cdf).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if unit_rtype as core::ffi::c_uint
            != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            write_wiener_filter(wiener_win, &(*rui).wiener_info, ref_wiener_info, w);
        }
    } else if frame_rtype as core::ffi::c_uint
        == RESTORE_SGRPROJ as core::ffi::c_int as core::ffi::c_uint
    {
        aom_write_symbol(
            w,
            (unit_rtype as core::ffi::c_uint
                != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int,
            ((*(*xd).tile_ctx).sgrproj_restore_cdf).as_mut_ptr(),
            2 as core::ffi::c_int,
        );
        if unit_rtype as core::ffi::c_uint
            != RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            write_sgrproj_filter(&(*rui).sgrproj_info, ref_sgrproj_info, w);
        }
    }
}
unsafe extern "C-unwind" fn is_mode_ref_delta_meaningful(mut cm: *mut AV1_COMMON) -> bool {
    let mut lf: *mut loopfilter = &raw mut (*cm).lf;
    if (*lf).mode_ref_delta_update == 0 {
        return 0 as core::ffi::c_int != 0;
    }
    let mut buf: *const RefCntBuffer = get_primary_ref_frame_buf(cm);
    let mut last_ref_deltas: [int8_t; 8] = [0; 8];
    let mut last_mode_deltas: [int8_t; 2] = [0; 2];
    if buf.is_null() {
        av1_set_default_ref_deltas(last_ref_deltas.as_mut_ptr());
        av1_set_default_mode_deltas(last_mode_deltas.as_mut_ptr());
    } else {
        memcpy(
            last_ref_deltas.as_mut_ptr() as *mut core::ffi::c_void,
            ((*buf).ref_deltas).as_ptr() as *const core::ffi::c_void,
            REF_FRAMES as core::ffi::c_int as size_t,
        );
        memcpy(
            last_mode_deltas.as_mut_ptr() as *mut core::ffi::c_void,
            ((*buf).mode_deltas).as_ptr() as *const core::ffi::c_void,
            MAX_MODE_LF_DELTAS as size_t,
        );
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < REF_FRAMES as core::ffi::c_int {
        if (*lf).ref_deltas[i as usize] as core::ffi::c_int
            != last_ref_deltas[i as usize] as core::ffi::c_int
        {
            return true_0 != 0;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < MAX_MODE_LF_DELTAS {
        if (*lf).mode_deltas[i_0 as usize] as core::ffi::c_int
            != last_mode_deltas[i_0 as usize] as core::ffi::c_int
        {
            return true_0 != 0;
        }
        i_0 += 1;
    }
    return false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn encode_loopfilter(
    mut cm: *mut AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    if (*cm).features.coded_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"encode_loopfilter\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2032 as core::ffi::c_int,
            b"!cm->features.coded_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*cm).features.allow_intrabc {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut lf: *mut loopfilter = &raw mut (*cm).lf;
    aom_wb_write_literal(
        wb,
        (*lf).filter_level[0 as core::ffi::c_int as usize],
        6 as core::ffi::c_int,
    );
    aom_wb_write_literal(
        wb,
        (*lf).filter_level[1 as core::ffi::c_int as usize],
        6 as core::ffi::c_int,
    );
    if num_planes > 1 as core::ffi::c_int {
        if (*lf).filter_level[0 as core::ffi::c_int as usize] != 0
            || (*lf).filter_level[1 as core::ffi::c_int as usize] != 0
        {
            aom_wb_write_literal(wb, (*lf).filter_level_u, 6 as core::ffi::c_int);
            aom_wb_write_literal(wb, (*lf).filter_level_v, 6 as core::ffi::c_int);
        }
    }
    aom_wb_write_literal(wb, (*lf).sharpness_level, 3 as core::ffi::c_int);
    aom_wb_write_bit(wb, (*lf).mode_ref_delta_enabled as core::ffi::c_int);
    if (*lf).mode_ref_delta_enabled != 0 {
        let mut mode_ref_delta_update: core::ffi::c_int = is_mode_ref_delta_meaningful(
            cm,
        ) as core::ffi::c_int;
        aom_wb_write_bit(wb, mode_ref_delta_update);
        if mode_ref_delta_update == 0 {
            return;
        }
        let mut buf: *const RefCntBuffer = get_primary_ref_frame_buf(cm);
        let mut last_ref_deltas: [int8_t; 8] = [0; 8];
        let mut last_mode_deltas: [int8_t; 2] = [0; 2];
        if buf.is_null() {
            av1_set_default_ref_deltas(last_ref_deltas.as_mut_ptr());
            av1_set_default_mode_deltas(last_mode_deltas.as_mut_ptr());
        } else {
            memcpy(
                last_ref_deltas.as_mut_ptr() as *mut core::ffi::c_void,
                ((*buf).ref_deltas).as_ptr() as *const core::ffi::c_void,
                REF_FRAMES as core::ffi::c_int as size_t,
            );
            memcpy(
                last_mode_deltas.as_mut_ptr() as *mut core::ffi::c_void,
                ((*buf).mode_deltas).as_ptr() as *const core::ffi::c_void,
                MAX_MODE_LF_DELTAS as size_t,
            );
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < REF_FRAMES as core::ffi::c_int {
            let delta: core::ffi::c_int = (*lf).ref_deltas[i as usize]
                as core::ffi::c_int;
            let changed: core::ffi::c_int = (delta
                != last_ref_deltas[i as usize] as core::ffi::c_int) as core::ffi::c_int;
            aom_wb_write_bit(wb, changed);
            if changed != 0 {
                aom_wb_write_inv_signed_literal(wb, delta, 6 as core::ffi::c_int);
            }
            i += 1;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < MAX_MODE_LF_DELTAS {
            let delta_0: core::ffi::c_int = (*lf).mode_deltas[i_0 as usize]
                as core::ffi::c_int;
            let changed_0: core::ffi::c_int = (delta_0
                != last_mode_deltas[i_0 as usize] as core::ffi::c_int)
                as core::ffi::c_int;
            aom_wb_write_bit(wb, changed_0);
            if changed_0 != 0 {
                aom_wb_write_inv_signed_literal(wb, delta_0, 6 as core::ffi::c_int);
            }
            i_0 += 1;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn encode_cdef(
    mut cm: *const AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    if (*cm).features.coded_lossless as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"encode_cdef\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2088 as core::ffi::c_int,
            b"!cm->features.coded_lossless\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*(*cm).seq_params).enable_cdef == 0 {
        return;
    }
    if (*cm).features.allow_intrabc {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    aom_wb_write_literal(
        wb,
        (*cm).cdef_info.cdef_damping - 3 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    aom_wb_write_literal(wb, (*cm).cdef_info.cdef_bits, 2 as core::ffi::c_int);
    i = 0 as core::ffi::c_int;
    while i < (*cm).cdef_info.nb_cdef_strengths {
        aom_wb_write_literal(
            wb,
            (*cm).cdef_info.cdef_strengths[i as usize],
            CDEF_STRENGTH_BITS,
        );
        if num_planes > 1 as core::ffi::c_int {
            aom_wb_write_literal(
                wb,
                (*cm).cdef_info.cdef_uv_strengths[i as usize],
                CDEF_STRENGTH_BITS,
            );
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn write_delta_q(
    mut wb: *mut aom_write_bit_buffer,
    mut delta_q: core::ffi::c_int,
) {
    if delta_q != 0 as core::ffi::c_int {
        aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        aom_wb_write_inv_signed_literal(wb, delta_q, 6 as core::ffi::c_int);
    } else {
        aom_wb_write_bit(wb, 0 as core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C-unwind" fn encode_quantization(
    quant_params: *const CommonQuantParams,
    mut num_planes: core::ffi::c_int,
    mut separate_uv_delta_q: bool,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_literal(wb, (*quant_params).base_qindex, QINDEX_BITS);
    write_delta_q(wb, (*quant_params).y_dc_delta_q);
    if num_planes > 1 as core::ffi::c_int {
        let mut diff_uv_delta: core::ffi::c_int = ((*quant_params).u_dc_delta_q
            != (*quant_params).v_dc_delta_q
            || (*quant_params).u_ac_delta_q != (*quant_params).v_ac_delta_q)
            as core::ffi::c_int;
        if separate_uv_delta_q {
            aom_wb_write_bit(wb, diff_uv_delta);
        }
        write_delta_q(wb, (*quant_params).u_dc_delta_q);
        write_delta_q(wb, (*quant_params).u_ac_delta_q);
        if diff_uv_delta != 0 {
            write_delta_q(wb, (*quant_params).v_dc_delta_q);
            write_delta_q(wb, (*quant_params).v_ac_delta_q);
        }
    }
    aom_wb_write_bit(wb, (*quant_params).using_qmatrix as core::ffi::c_int);
    if (*quant_params).using_qmatrix {
        aom_wb_write_literal(wb, (*quant_params).qmatrix_level_y, QM_LEVEL_BITS);
        aom_wb_write_literal(wb, (*quant_params).qmatrix_level_u, QM_LEVEL_BITS);
        if !separate_uv_delta_q {
            if !((*quant_params).qmatrix_level_u == (*quant_params).qmatrix_level_v)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"encode_quantization\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2135 as core::ffi::c_int,
                    b"quant_params->qmatrix_level_u == quant_params->qmatrix_level_v\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else {
            aom_wb_write_literal(wb, (*quant_params).qmatrix_level_v, QM_LEVEL_BITS);
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn encode_segmentation(
    mut cm: *mut AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut seg: *mut segmentation = &raw mut (*cm).seg;
    aom_wb_write_bit(wb, (*seg).enabled as core::ffi::c_int);
    if (*seg).enabled == 0 {
        return;
    }
    if (*cm).features.primary_ref_frame != PRIMARY_REF_NONE {
        aom_wb_write_bit(wb, (*seg).update_map as core::ffi::c_int);
        if (*seg).update_map != 0 {
            aom_wb_write_bit(wb, (*seg).temporal_update as core::ffi::c_int);
        }
        aom_wb_write_bit(wb, (*seg).update_data as core::ffi::c_int);
    }
    if (*seg).update_data != 0 {
        i = 0 as core::ffi::c_int;
        while i < MAX_SEGMENTS {
            j = 0 as core::ffi::c_int;
            while j < SEG_LVL_MAX as core::ffi::c_int {
                let active: core::ffi::c_int = segfeature_active(
                    seg,
                    i as uint8_t,
                    j as SEG_LVL_FEATURES,
                ) as core::ffi::c_int;
                aom_wb_write_bit(wb, active);
                if active != 0 {
                    let data_max: core::ffi::c_int = av1_seg_feature_data_max(
                        j as SEG_LVL_FEATURES,
                    ) as core::ffi::c_int;
                    let data_min: core::ffi::c_int = -data_max;
                    let ubits: core::ffi::c_int = get_unsigned_bits(
                        data_max as core::ffi::c_uint,
                    ) as core::ffi::c_int;
                    let data: core::ffi::c_int = clamp(
                        get_segdata(seg, i, j as SEG_LVL_FEATURES),
                        data_min,
                        data_max,
                    ) as core::ffi::c_int;
                    if av1_is_segfeature_signed(j as SEG_LVL_FEATURES) != 0 {
                        aom_wb_write_inv_signed_literal(wb, data, ubits);
                    } else {
                        aom_wb_write_literal(wb, data, ubits);
                    }
                }
                j += 1;
            }
            i += 1;
        }
    }
}
#[inline]
unsafe extern "C-unwind" fn write_frame_interp_filter(
    mut filter: InterpFilter,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_bit(
        wb,
        (filter as core::ffi::c_int == SWITCHABLE as core::ffi::c_int)
            as core::ffi::c_int,
    );
    if filter as core::ffi::c_int != SWITCHABLE as core::ffi::c_int {
        aom_wb_write_literal(wb, filter as core::ffi::c_int, LOG_SWITCHABLE_FILTERS);
    }
}
#[inline]
unsafe extern "C-unwind" fn wb_write_uniform(
    mut wb: *mut aom_write_bit_buffer,
    mut n: core::ffi::c_int,
    mut v: core::ffi::c_int,
) {
    let l: core::ffi::c_int = get_unsigned_bits(n as core::ffi::c_uint)
        as core::ffi::c_int;
    let m: core::ffi::c_int = ((1 as core::ffi::c_int) << l) - n;
    if l == 0 as core::ffi::c_int {
        return;
    }
    if v < m {
        aom_wb_write_literal(wb, v, l - 1 as core::ffi::c_int);
    } else {
        aom_wb_write_literal(
            wb,
            m + (v - m >> 1 as core::ffi::c_int),
            l - 1 as core::ffi::c_int,
        );
        aom_wb_write_literal(wb, v - m & 1 as core::ffi::c_int, 1 as core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C-unwind" fn write_tile_info_max_tile(
    cm: *const AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    let mut width_sb: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*(*cm).seq_params).mib_size_log2;
    let mut height_sb: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << (*(*cm).seq_params).mib_size_log2)
        - 1 as core::ffi::c_int >> (*(*cm).seq_params).mib_size_log2;
    let mut size_sb: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let tiles: *const CommonTileParams = &(*cm).tiles;
    aom_wb_write_bit(wb, (*tiles).uniform_spacing);
    if (*tiles).uniform_spacing != 0 {
        let mut ones: core::ffi::c_int = (*tiles).log2_cols - (*tiles).min_log2_cols;
        loop {
            let fresh20 = ones;
            ones = ones - 1;
            if !(fresh20 != 0) {
                break;
            }
            aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        }
        if (*tiles).log2_cols < (*tiles).max_log2_cols {
            aom_wb_write_bit(wb, 0 as core::ffi::c_int);
        }
        ones = (*tiles).log2_rows - (*tiles).min_log2_rows;
        loop {
            let fresh21 = ones;
            ones = ones - 1;
            if !(fresh21 != 0) {
                break;
            }
            aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        }
        if (*tiles).log2_rows < (*tiles).max_log2_rows {
            aom_wb_write_bit(wb, 0 as core::ffi::c_int);
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < (*tiles).cols {
            size_sb = (*tiles).col_start_sb[(i + 1 as core::ffi::c_int) as usize]
                - (*tiles).col_start_sb[i as usize];
            wb_write_uniform(
                wb,
                if width_sb < (*tiles).max_width_sb {
                    width_sb
                } else {
                    (*tiles).max_width_sb
                },
                size_sb - 1 as core::ffi::c_int,
            );
            width_sb -= size_sb;
            i += 1;
        }
        if !(width_sb == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"write_tile_info_max_tile\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2236 as core::ffi::c_int,
                b"width_sb == 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        i = 0 as core::ffi::c_int;
        while i < (*tiles).rows {
            size_sb = (*tiles).row_start_sb[(i + 1 as core::ffi::c_int) as usize]
                - (*tiles).row_start_sb[i as usize];
            wb_write_uniform(
                wb,
                if height_sb < (*tiles).max_height_sb {
                    height_sb
                } else {
                    (*tiles).max_height_sb
                },
                size_sb - 1 as core::ffi::c_int,
            );
            height_sb -= size_sb;
            i += 1;
        }
        if !(height_sb == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"write_tile_info_max_tile\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2245 as core::ffi::c_int,
                b"height_sb == 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    };
}
#[inline]
unsafe extern "C-unwind" fn write_tile_info(
    cm: *const AV1_COMMON,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut wb: *mut aom_write_bit_buffer,
) {
    write_tile_info_max_tile(cm, wb);
    *saved_wb = *wb;
    if (*cm).tiles.rows * (*cm).tiles.cols > 1 as core::ffi::c_int {
        aom_wb_write_literal(
            wb,
            0 as core::ffi::c_int,
            (*cm).tiles.log2_cols + (*cm).tiles.log2_rows,
        );
        aom_wb_write_literal(wb, 3 as core::ffi::c_int, 2 as core::ffi::c_int);
    }
}
#[inline]
unsafe extern "C-unwind" fn write_ext_tile_info(
    cm: *const AV1_COMMON,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut wb: *mut aom_write_bit_buffer,
) {
    let mut mod_0: core::ffi::c_int = ((*wb).bit_offset)
        .wrapping_rem(CHAR_BIT as uint32_t) as core::ffi::c_int;
    if mod_0 > 0 as core::ffi::c_int {
        aom_wb_write_literal(wb, 0 as core::ffi::c_int, CHAR_BIT - mod_0);
    }
    if (aom_wb_is_byte_aligned(wb) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"write_ext_tile_info\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2269 as core::ffi::c_int,
            b"aom_wb_is_byte_aligned(wb)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *saved_wb = *wb;
    if (*cm).tiles.rows * (*cm).tiles.cols > 1 as core::ffi::c_int {
        aom_wb_write_literal(wb, 0 as core::ffi::c_int, 2 as core::ffi::c_int);
        aom_wb_write_literal(wb, 0 as core::ffi::c_int, 2 as core::ffi::c_int);
    }
}
#[inline]
unsafe extern "C-unwind" fn find_identical_tile(
    tile_row: core::ffi::c_int,
    tile_col: core::ffi::c_int,
    tile_buffers: *mut [TileBufferEnc; 64],
) -> core::ffi::c_int {
    let candidate_offset: [MV32; 1] = [
        {
            let mut init = mv32 {
                row: 1 as int32_t,
                col: 0 as int32_t,
            };
            init
        },
    ];
    let cur_tile_data: *const uint8_t = ((*tile_buffers
        .offset(tile_row as isize))[tile_col as usize]
        .data)
        .offset(4 as core::ffi::c_int as isize);
    let cur_tile_size: size_t = (*tile_buffers
        .offset(tile_row as isize))[tile_col as usize]
        .size;
    let mut i: core::ffi::c_int = 0;
    if tile_row == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < 1 as core::ffi::c_int {
        let mut row_offset: core::ffi::c_int = candidate_offset[0 as core::ffi::c_int
                as usize]
            .row as core::ffi::c_int;
        let mut col_offset: core::ffi::c_int = candidate_offset[0 as core::ffi::c_int
                as usize]
            .col as core::ffi::c_int;
        let mut row: core::ffi::c_int = tile_row - row_offset;
        let mut col: core::ffi::c_int = tile_col - col_offset;
        let mut tile_data: *const uint8_t = 0 as *const uint8_t;
        let mut candidate: *mut TileBufferEnc = 0 as *mut TileBufferEnc;
        if !(row < 0 as core::ffi::c_int || col < 0 as core::ffi::c_int) {
            let tile_hdr: uint32_t = mem_get_le32_as_int(
                (*tile_buffers.offset(row as isize))[col as usize].data
                    as *const core::ffi::c_void,
            ) as uint32_t;
            if tile_hdr >> 31 as core::ffi::c_int == 1 as uint32_t {
                row_offset = (row_offset as uint32_t)
                    .wrapping_add(tile_hdr >> 24 as core::ffi::c_int & 0x7f as uint32_t)
                    as core::ffi::c_int as core::ffi::c_int;
                row = tile_row - row_offset;
            }
            candidate = &raw mut *(*tile_buffers.offset(row as isize))
                .as_mut_ptr()
                .offset(col as isize) as *mut TileBufferEnc;
            if !(row_offset >= 128 as core::ffi::c_int
                || (*candidate).size != cur_tile_size)
            {
                tile_data = ((*candidate).data).offset(4 as core::ffi::c_int as isize);
                if !(memcmp(
                    tile_data as *const core::ffi::c_void,
                    cur_tile_data as *const core::ffi::c_void,
                    cur_tile_size,
                ) != 0 as core::ffi::c_int)
                {
                    if !(row_offset > 0 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 20],
                                [core::ffi::c_char; 20],
                            >(*b"find_identical_tile\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            2325 as core::ffi::c_int,
                            b"row_offset > 0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    return row_offset;
                }
            }
        }
        i += 1;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn write_render_size(
    mut cm: *const AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    let scaling_active: core::ffi::c_int = av1_resize_scaled(cm) as core::ffi::c_int;
    aom_wb_write_bit(wb, scaling_active);
    if scaling_active != 0 {
        aom_wb_write_literal(
            wb,
            (*cm).render_width - 1 as core::ffi::c_int,
            16 as core::ffi::c_int,
        );
        aom_wb_write_literal(
            wb,
            (*cm).render_height - 1 as core::ffi::c_int,
            16 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_superres_scale(
    cm: *const AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    if (*seq_params).enable_superres == 0 {
        if !((*cm).superres_scale_denominator as core::ffi::c_int
            == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"write_superres_scale\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2347 as core::ffi::c_int,
                b"cm->superres_scale_denominator == SCALE_NUMERATOR\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return;
    }
    if (*cm).superres_scale_denominator as core::ffi::c_int == SCALE_NUMERATOR {
        aom_wb_write_bit(wb, 0 as core::ffi::c_int);
    } else {
        aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        if !((*cm).superres_scale_denominator as core::ffi::c_int
            >= 8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"write_superres_scale\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2356 as core::ffi::c_int,
                b"cm->superres_scale_denominator >= SUPERRES_SCALE_DENOMINATOR_MIN\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(((*cm).superres_scale_denominator as core::ffi::c_int)
            < 8 as core::ffi::c_int + 1 as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 3 as core::ffi::c_int)) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"write_superres_scale\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2358 as core::ffi::c_int,
                b"cm->superres_scale_denominator < SUPERRES_SCALE_DENOMINATOR_MIN + (1 << SUPERRES_SCALE_BITS)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        aom_wb_write_literal(
            wb,
            (*cm).superres_scale_denominator as core::ffi::c_int
                - SUPERRES_SCALE_DENOMINATOR_MIN,
            SUPERRES_SCALE_BITS,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn write_frame_size(
    mut cm: *const AV1_COMMON,
    mut frame_size_override: core::ffi::c_int,
    mut wb: *mut aom_write_bit_buffer,
) {
    let coded_width: core::ffi::c_int = (*cm).superres_upscaled_width
        - 1 as core::ffi::c_int;
    let coded_height: core::ffi::c_int = (*cm).superres_upscaled_height
        - 1 as core::ffi::c_int;
    let mut seq_params: *const SequenceHeader = (*cm).seq_params;
    if frame_size_override != 0 {
        let mut num_bits_width: core::ffi::c_int = (*seq_params).num_bits_width;
        let mut num_bits_height: core::ffi::c_int = (*seq_params).num_bits_height;
        aom_wb_write_literal(wb, coded_width, num_bits_width);
        aom_wb_write_literal(wb, coded_height, num_bits_height);
    } else if (*cm).superres_upscaled_width != (*seq_params).max_frame_width
        || (*cm).superres_upscaled_height != (*seq_params).max_frame_height
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"Frame dimensions are different from the maximum values when frame_size_override_flag is 0\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    write_superres_scale(cm, wb);
    write_render_size(cm, wb);
}
#[inline]
unsafe extern "C-unwind" fn write_frame_size_with_refs(
    cm: *const AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        let mut cfg: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(cm, ref_frame);
        if !cfg.is_null() {
            found = ((*cm).superres_upscaled_width
                == (*cfg).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
                && (*cm).superres_upscaled_height
                    == (*cfg).c2rust_unnamed_2.c2rust_unnamed.y_crop_height)
                as core::ffi::c_int;
            found
                &= ((*cm).render_width == (*cfg).render_width
                    && (*cm).render_height == (*cfg).render_height) as core::ffi::c_int;
        }
        aom_wb_write_bit(wb, found);
        if found != 0 {
            write_superres_scale(cm, wb);
            break;
        } else {
            ref_frame += 1;
        }
    }
    if found == 0 {
        let mut frame_size_override: core::ffi::c_int = 1 as core::ffi::c_int;
        write_frame_size(cm, frame_size_override, wb);
    }
}
#[inline]
unsafe extern "C-unwind" fn write_profile(
    mut profile: BITSTREAM_PROFILE,
    mut wb: *mut aom_write_bit_buffer,
) {
    if !(profile as core::ffi::c_int >= PROFILE_0 as core::ffi::c_int
        && (profile as core::ffi::c_int) < MAX_PROFILES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_profile\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2419 as core::ffi::c_int,
            b"profile >= PROFILE_0 && profile < MAX_PROFILES\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_write_literal(wb, profile as core::ffi::c_int, PROFILE_BITS);
}
#[inline]
unsafe extern "C-unwind" fn write_bitdepth(
    seq_params: *const SequenceHeader,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_bit(
        wb,
        if (*seq_params).bit_depth as core::ffi::c_uint
            == AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
        {
            0 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        },
    );
    if (*seq_params).profile as core::ffi::c_int == PROFILE_2 as core::ffi::c_int
        && (*seq_params).bit_depth as core::ffi::c_uint
            != AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
    {
        aom_wb_write_bit(
            wb,
            if (*seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_10 as core::ffi::c_int as core::ffi::c_uint
            {
                0 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            },
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_color_config(
    seq_params: *const SequenceHeader,
    mut wb: *mut aom_write_bit_buffer,
) {
    write_bitdepth(seq_params, wb);
    let is_monochrome: core::ffi::c_int = (*seq_params).monochrome as core::ffi::c_int;
    if (*seq_params).profile as core::ffi::c_int != PROFILE_1 as core::ffi::c_int {
        aom_wb_write_bit(wb, is_monochrome);
    } else {
        if (is_monochrome != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_color_config\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2441 as core::ffi::c_int,
                b"!is_monochrome\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*seq_params).color_primaries as core::ffi::c_uint
        == AOM_CICP_CP_UNSPECIFIED as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).transfer_characteristics as core::ffi::c_uint
            == AOM_CICP_TC_UNSPECIFIED as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).matrix_coefficients as core::ffi::c_uint
            == AOM_CICP_MC_UNSPECIFIED as core::ffi::c_int as core::ffi::c_uint
    {
        aom_wb_write_bit(wb, 0 as core::ffi::c_int);
    } else {
        aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        aom_wb_write_literal(
            wb,
            (*seq_params).color_primaries as core::ffi::c_int,
            8 as core::ffi::c_int,
        );
        aom_wb_write_literal(
            wb,
            (*seq_params).transfer_characteristics as core::ffi::c_int,
            8 as core::ffi::c_int,
        );
        aom_wb_write_literal(
            wb,
            (*seq_params).matrix_coefficients as core::ffi::c_int,
            8 as core::ffi::c_int,
        );
    }
    if is_monochrome != 0 {
        aom_wb_write_bit(wb, (*seq_params).color_range);
        return;
    }
    if (*seq_params).color_primaries as core::ffi::c_uint
        == AOM_CICP_CP_BT_709 as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).transfer_characteristics as core::ffi::c_uint
            == AOM_CICP_TC_SRGB as core::ffi::c_int as core::ffi::c_uint
        && (*seq_params).matrix_coefficients as core::ffi::c_uint
            == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
    {
        if !((*seq_params).subsampling_x == 0 as core::ffi::c_int
            && (*seq_params).subsampling_y == 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_color_config\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2460 as core::ffi::c_int,
                b"seq_params->subsampling_x == 0 && seq_params->subsampling_y == 0\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*seq_params).profile as core::ffi::c_int == PROFILE_1 as core::ffi::c_int
            || (*seq_params).profile as core::ffi::c_int == PROFILE_2 as core::ffi::c_int
                && (*seq_params).bit_depth as core::ffi::c_uint
                    == AOM_BITS_12 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"write_color_config\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2463 as core::ffi::c_int,
                b"seq_params->profile == PROFILE_1 || (seq_params->profile == PROFILE_2 && seq_params->bit_depth == AOM_BITS_12)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    } else {
        aom_wb_write_bit(wb, (*seq_params).color_range);
        if (*seq_params).profile as core::ffi::c_int == PROFILE_0 as core::ffi::c_int {
            if !((*seq_params).subsampling_x == 1 as core::ffi::c_int
                && (*seq_params).subsampling_y == 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"write_color_config\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2469 as core::ffi::c_int,
                    b"seq_params->subsampling_x == 1 && seq_params->subsampling_y == 1\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else if (*seq_params).profile as core::ffi::c_int
            == PROFILE_1 as core::ffi::c_int
        {
            if !((*seq_params).subsampling_x == 0 as core::ffi::c_int
                && (*seq_params).subsampling_y == 0 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"write_color_config\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2472 as core::ffi::c_int,
                    b"seq_params->subsampling_x == 0 && seq_params->subsampling_y == 0\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        } else if (*seq_params).profile as core::ffi::c_int
            == PROFILE_2 as core::ffi::c_int
        {
            if (*seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_12 as core::ffi::c_int as core::ffi::c_uint
            {
                aom_wb_write_bit(wb, (*seq_params).subsampling_x);
                if (*seq_params).subsampling_x == 0 as core::ffi::c_int {
                    if !((*seq_params).subsampling_y == 0 as core::ffi::c_int
                        && !(b"4:4:0 subsampling not allowed in AV1\0" as *const u8
                            as *const core::ffi::c_char)
                            .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"write_color_config\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            2479 as core::ffi::c_int,
                            b"seq_params->subsampling_y == 0 && \"4:4:0 subsampling not allowed in AV1\"\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                } else {
                    aom_wb_write_bit(wb, (*seq_params).subsampling_y);
                }
            } else {
                if !((*seq_params).subsampling_x == 1 as core::ffi::c_int
                    && (*seq_params).subsampling_y == 0 as core::ffi::c_int)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"write_color_config\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        2486 as core::ffi::c_int,
                        b"seq_params->subsampling_x == 1 && seq_params->subsampling_y == 0\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
        if (*seq_params).matrix_coefficients as core::ffi::c_uint
            == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
        {
            if !((*seq_params).subsampling_x == 0 as core::ffi::c_int
                && (*seq_params).subsampling_y == 0 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"write_color_config\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2490 as core::ffi::c_int,
                    b"seq_params->subsampling_x == 0 && seq_params->subsampling_y == 0\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        if (*seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*seq_params).subsampling_y == 1 as core::ffi::c_int
        {
            aom_wb_write_literal(
                wb,
                (*seq_params).chroma_sample_position as core::ffi::c_int,
                2 as core::ffi::c_int,
            );
        }
    }
    aom_wb_write_bit(wb, (*seq_params).separate_uv_delta_q as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn write_timing_info_header(
    timing_info: *const aom_timing_info_t,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_unsigned_literal(
        wb,
        (*timing_info).num_units_in_display_tick,
        32 as core::ffi::c_int,
    );
    aom_wb_write_unsigned_literal(wb, (*timing_info).time_scale, 32 as core::ffi::c_int);
    aom_wb_write_bit(wb, (*timing_info).equal_picture_interval);
    if (*timing_info).equal_picture_interval != 0 {
        aom_wb_write_uvlc(
            wb,
            ((*timing_info).num_ticks_per_picture).wrapping_sub(1 as uint32_t),
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_decoder_model_info(
    decoder_model_info: *const aom_dec_model_info_t,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_literal(
        wb,
        (*decoder_model_info).encoder_decoder_buffer_delay_length
            - 1 as core::ffi::c_int,
        5 as core::ffi::c_int,
    );
    aom_wb_write_unsigned_literal(
        wb,
        (*decoder_model_info).num_units_in_decoding_tick,
        32 as core::ffi::c_int,
    );
    aom_wb_write_literal(
        wb,
        (*decoder_model_info).buffer_removal_time_length - 1 as core::ffi::c_int,
        5 as core::ffi::c_int,
    );
    aom_wb_write_literal(
        wb,
        (*decoder_model_info).frame_presentation_time_length - 1 as core::ffi::c_int,
        5 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_dec_model_op_parameters(
    mut op_params: *const aom_dec_model_op_parameters_t,
    mut buffer_delay_length: core::ffi::c_int,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_unsigned_literal(
        wb,
        (*op_params).decoder_buffer_delay,
        buffer_delay_length,
    );
    aom_wb_write_unsigned_literal(
        wb,
        (*op_params).encoder_buffer_delay,
        buffer_delay_length,
    );
    aom_wb_write_bit(wb, (*op_params).low_delay_mode_flag);
}
#[inline]
unsafe extern "C-unwind" fn write_tu_pts_info(
    cm: *mut AV1_COMMON,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_unsigned_literal(
        wb,
        (*cm).frame_presentation_time,
        (*(*cm).seq_params).decoder_model_info.frame_presentation_time_length,
    );
}
#[inline]
unsafe extern "C-unwind" fn write_film_grain_params(
    cpi: *const AV1_COMP,
    mut wb: *mut aom_write_bit_buffer,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let pars: *const aom_film_grain_t = &raw mut (*(*cm).cur_frame).film_grain_params;
    aom_wb_write_bit(wb, (*pars).apply_grain);
    if (*pars).apply_grain == 0 {
        return;
    }
    aom_wb_write_literal(
        wb,
        (*pars).random_seed as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
    if (*cm).current_frame.frame_type as core::ffi::c_int
        == INTER_FRAME as core::ffi::c_int
    {
        aom_wb_write_bit(wb, (*pars).update_parameters);
    }
    if (*pars).update_parameters == 0 {
        let mut ref_frame: core::ffi::c_int = 0;
        let mut ref_idx: core::ffi::c_int = 0;
        ref_frame = LAST_FRAME as core::ffi::c_int;
        while ref_frame < REF_FRAMES as core::ffi::c_int {
            ref_idx = get_ref_frame_map_idx(cm, ref_frame as MV_REFERENCE_FRAME);
            if !(ref_idx != -(1 as core::ffi::c_int)) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"write_film_grain_params\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2556 as core::ffi::c_int,
                    b"ref_idx != INVALID_IDX\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let buf: *const RefCntBuffer = (*cm).ref_frame_map[ref_idx as usize];
            if (*buf).film_grain_params_present as core::ffi::c_int != 0
                && aom_check_grain_params_equiv(pars, &(*buf).film_grain_params) != 0
            {
                break;
            }
            ref_frame += 1;
        }
        if !(ref_frame < REF_FRAMES as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"write_film_grain_params\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2563 as core::ffi::c_int,
                b"ref_frame < REF_FRAMES\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        aom_wb_write_literal(wb, ref_idx, 3 as core::ffi::c_int);
        return;
    }
    aom_wb_write_literal(wb, (*pars).num_y_points, 4 as core::ffi::c_int);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*pars).num_y_points {
        aom_wb_write_literal(
            wb,
            (*pars).scaling_points_y[i as usize][0 as core::ffi::c_int as usize],
            8 as core::ffi::c_int,
        );
        aom_wb_write_literal(
            wb,
            (*pars).scaling_points_y[i as usize][1 as core::ffi::c_int as usize],
            8 as core::ffi::c_int,
        );
        i += 1;
    }
    if (*(*cm).seq_params).monochrome == 0 {
        aom_wb_write_bit(wb, (*pars).chroma_scaling_from_luma);
    } else {
        if ((*pars).chroma_scaling_from_luma != 0) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"write_film_grain_params\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2578 as core::ffi::c_int,
                b"!pars->chroma_scaling_from_luma\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0
        || (*pars).chroma_scaling_from_luma != 0
        || (*(*cm).seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*(*cm).seq_params).subsampling_y == 1 as core::ffi::c_int
            && (*pars).num_y_points == 0 as core::ffi::c_int
    {
        if !((*pars).num_cb_points == 0 as core::ffi::c_int
            && (*pars).num_cr_points == 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"write_film_grain_params\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2584 as core::ffi::c_int,
                b"pars->num_cb_points == 0 && pars->num_cr_points == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    } else {
        aom_wb_write_literal(wb, (*pars).num_cb_points, 4 as core::ffi::c_int);
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < (*pars).num_cb_points {
            aom_wb_write_literal(
                wb,
                (*pars).scaling_points_cb[i_0 as usize][0 as core::ffi::c_int as usize],
                8 as core::ffi::c_int,
            );
            aom_wb_write_literal(
                wb,
                (*pars).scaling_points_cb[i_0 as usize][1 as core::ffi::c_int as usize],
                8 as core::ffi::c_int,
            );
            i_0 += 1;
        }
        aom_wb_write_literal(wb, (*pars).num_cr_points, 4 as core::ffi::c_int);
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < (*pars).num_cr_points {
            aom_wb_write_literal(
                wb,
                (*pars).scaling_points_cr[i_1 as usize][0 as core::ffi::c_int as usize],
                8 as core::ffi::c_int,
            );
            aom_wb_write_literal(
                wb,
                (*pars).scaling_points_cr[i_1 as usize][1 as core::ffi::c_int as usize],
                8 as core::ffi::c_int,
            );
            i_1 += 1;
        }
    }
    aom_wb_write_literal(
        wb,
        (*pars).scaling_shift - 8 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    aom_wb_write_literal(wb, (*pars).ar_coeff_lag, 2 as core::ffi::c_int);
    let mut num_pos_luma: core::ffi::c_int = 2 as core::ffi::c_int * (*pars).ar_coeff_lag
        * ((*pars).ar_coeff_lag + 1 as core::ffi::c_int);
    let mut num_pos_chroma: core::ffi::c_int = num_pos_luma;
    if (*pars).num_y_points > 0 as core::ffi::c_int {
        num_pos_chroma += 1;
    }
    if (*pars).num_y_points != 0 {
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < num_pos_luma {
            aom_wb_write_literal(
                wb,
                (*pars).ar_coeffs_y[i_2 as usize] + 128 as core::ffi::c_int,
                8 as core::ffi::c_int,
            );
            i_2 += 1;
        }
    }
    if (*pars).num_cb_points != 0 || (*pars).chroma_scaling_from_luma != 0 {
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 < num_pos_chroma {
            aom_wb_write_literal(
                wb,
                (*pars).ar_coeffs_cb[i_3 as usize] + 128 as core::ffi::c_int,
                8 as core::ffi::c_int,
            );
            i_3 += 1;
        }
    }
    if (*pars).num_cr_points != 0 || (*pars).chroma_scaling_from_luma != 0 {
        let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_4 < num_pos_chroma {
            aom_wb_write_literal(
                wb,
                (*pars).ar_coeffs_cr[i_4 as usize] + 128 as core::ffi::c_int,
                8 as core::ffi::c_int,
            );
            i_4 += 1;
        }
    }
    aom_wb_write_literal(
        wb,
        (*pars).ar_coeff_shift - 6 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    aom_wb_write_literal(wb, (*pars).grain_scale_shift, 2 as core::ffi::c_int);
    if (*pars).num_cb_points != 0 {
        aom_wb_write_literal(wb, (*pars).cb_mult, 8 as core::ffi::c_int);
        aom_wb_write_literal(wb, (*pars).cb_luma_mult, 8 as core::ffi::c_int);
        aom_wb_write_literal(wb, (*pars).cb_offset, 9 as core::ffi::c_int);
    }
    if (*pars).num_cr_points != 0 {
        aom_wb_write_literal(wb, (*pars).cr_mult, 8 as core::ffi::c_int);
        aom_wb_write_literal(wb, (*pars).cr_luma_mult, 8 as core::ffi::c_int);
        aom_wb_write_literal(wb, (*pars).cr_offset, 9 as core::ffi::c_int);
    }
    aom_wb_write_bit(wb, (*pars).overlap_flag);
    aom_wb_write_bit(wb, (*pars).clip_to_restricted_range);
}
#[inline]
unsafe extern "C-unwind" fn write_sb_size(
    seq_params: *const SequenceHeader,
    mut wb: *mut aom_write_bit_buffer,
) {
    if !((*seq_params).mib_size
        == mi_size_wide[(*seq_params).sb_size as usize] as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_sb_size\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2648 as core::ffi::c_int,
            b"seq_params->mib_size == mi_size_wide[seq_params->sb_size]\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*seq_params).mib_size
        == (1 as core::ffi::c_int) << (*seq_params).mib_size_log2) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_sb_size\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2649 as core::ffi::c_int,
            b"seq_params->mib_size == 1 << seq_params->mib_size_log2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*seq_params).sb_size as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        || (*seq_params).sb_size as core::ffi::c_int == BLOCK_64X64 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"write_sb_size\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2651 as core::ffi::c_int,
            b"seq_params->sb_size == BLOCK_128X128 || seq_params->sb_size == BLOCK_64X64\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_write_bit(
        wb,
        if (*seq_params).sb_size as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        },
    );
}
#[inline]
unsafe extern "C-unwind" fn write_sequence_header(
    seq_params: *const SequenceHeader,
    mut wb: *mut aom_write_bit_buffer,
) {
    aom_wb_write_literal(
        wb,
        (*seq_params).num_bits_width - 1 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
    aom_wb_write_literal(
        wb,
        (*seq_params).num_bits_height - 1 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
    aom_wb_write_literal(
        wb,
        (*seq_params).max_frame_width - 1 as core::ffi::c_int,
        (*seq_params).num_bits_width,
    );
    aom_wb_write_literal(
        wb,
        (*seq_params).max_frame_height - 1 as core::ffi::c_int,
        (*seq_params).num_bits_height,
    );
    if (*seq_params).reduced_still_picture_hdr == 0 {
        aom_wb_write_bit(
            wb,
            (*seq_params).frame_id_numbers_present_flag as core::ffi::c_int,
        );
        if (*seq_params).frame_id_numbers_present_flag != 0 {
            aom_wb_write_literal(
                wb,
                (*seq_params).delta_frame_id_length - 2 as core::ffi::c_int,
                4 as core::ffi::c_int,
            );
            aom_wb_write_literal(
                wb,
                (*seq_params).frame_id_length - (*seq_params).delta_frame_id_length
                    - 1 as core::ffi::c_int,
                3 as core::ffi::c_int,
            );
        }
    }
    write_sb_size(seq_params, wb);
    aom_wb_write_bit(wb, (*seq_params).enable_filter_intra as core::ffi::c_int);
    aom_wb_write_bit(wb, (*seq_params).enable_intra_edge_filter as core::ffi::c_int);
    if (*seq_params).reduced_still_picture_hdr == 0 {
        aom_wb_write_bit(
            wb,
            (*seq_params).enable_interintra_compound as core::ffi::c_int,
        );
        aom_wb_write_bit(wb, (*seq_params).enable_masked_compound as core::ffi::c_int);
        aom_wb_write_bit(wb, (*seq_params).enable_warped_motion as core::ffi::c_int);
        aom_wb_write_bit(wb, (*seq_params).enable_dual_filter as core::ffi::c_int);
        aom_wb_write_bit(wb, (*seq_params).order_hint_info.enable_order_hint);
        if (*seq_params).order_hint_info.enable_order_hint != 0 {
            aom_wb_write_bit(wb, (*seq_params).order_hint_info.enable_dist_wtd_comp);
            aom_wb_write_bit(wb, (*seq_params).order_hint_info.enable_ref_frame_mvs);
        }
        if (*seq_params).force_screen_content_tools as core::ffi::c_int
            == 2 as core::ffi::c_int
        {
            aom_wb_write_bit(wb, 1 as core::ffi::c_int);
        } else {
            aom_wb_write_bit(wb, 0 as core::ffi::c_int);
            aom_wb_write_bit(
                wb,
                (*seq_params).force_screen_content_tools as core::ffi::c_int,
            );
        }
        if (*seq_params).force_screen_content_tools as core::ffi::c_int
            > 0 as core::ffi::c_int
        {
            if (*seq_params).force_integer_mv as core::ffi::c_int
                == 2 as core::ffi::c_int
            {
                aom_wb_write_bit(wb, 1 as core::ffi::c_int);
            } else {
                aom_wb_write_bit(wb, 0 as core::ffi::c_int);
                aom_wb_write_bit(wb, (*seq_params).force_integer_mv as core::ffi::c_int);
            }
        } else {
            if !((*seq_params).force_integer_mv as core::ffi::c_int
                == 2 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 22],
                        [core::ffi::c_char; 22],
                    >(*b"write_sequence_header\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2709 as core::ffi::c_int,
                    b"seq_params->force_integer_mv == 2\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
        if (*seq_params).order_hint_info.enable_order_hint != 0 {
            aom_wb_write_literal(
                wb,
                (*seq_params).order_hint_info.order_hint_bits_minus_1,
                3 as core::ffi::c_int,
            );
        }
    }
    aom_wb_write_bit(wb, (*seq_params).enable_superres as core::ffi::c_int);
    aom_wb_write_bit(wb, (*seq_params).enable_cdef as core::ffi::c_int);
    aom_wb_write_bit(wb, (*seq_params).enable_restoration as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn write_global_motion_params(
    mut params: *const WarpedMotionParams,
    mut ref_params: *const WarpedMotionParams,
    mut wb: *mut aom_write_bit_buffer,
    mut allow_hp: core::ffi::c_int,
) {
    let type_0: TransformationType = (*params).wmtype;
    if !(type_0 as core::ffi::c_int != TRANSLATION as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"write_global_motion_params\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            2730 as core::ffi::c_int,
            b"type != TRANSLATION\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_write_bit(
        wb,
        (type_0 as core::ffi::c_int != IDENTITY as core::ffi::c_int) as core::ffi::c_int,
    );
    if type_0 as core::ffi::c_int != IDENTITY as core::ffi::c_int {
        aom_wb_write_bit(
            wb,
            (type_0 as core::ffi::c_int == ROTZOOM as core::ffi::c_int)
                as core::ffi::c_int,
        );
        if type_0 as core::ffi::c_int != ROTZOOM as core::ffi::c_int {
            aom_wb_write_bit(
                wb,
                (type_0 as core::ffi::c_int == TRANSLATION as core::ffi::c_int)
                    as core::ffi::c_int,
            );
        }
    }
    if type_0 as core::ffi::c_int >= ROTZOOM as core::ffi::c_int {
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            (((*ref_params).wmmat[2 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
            (((*params).wmmat[2 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
        );
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[3 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
            ((*params).wmmat[3 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
        );
    }
    if type_0 as core::ffi::c_int >= AFFINE as core::ffi::c_int {
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[4 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
            ((*params).wmmat[4 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                as int16_t,
        );
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (GM_ALPHA_MAX + 1 as core::ffi::c_int) as uint16_t,
            SUBEXPFIN_K as uint16_t,
            (((*ref_params).wmmat[5 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
            (((*params).wmmat[5 as core::ffi::c_int as usize] >> GM_ALPHA_PREC_DIFF)
                - ((1 as int32_t) << GM_ALPHA_PREC_BITS)) as int16_t,
        );
    }
    if type_0 as core::ffi::c_int >= TRANSLATION as core::ffi::c_int {
        let trans_bits: core::ffi::c_int = if type_0 as core::ffi::c_int
            == TRANSLATION as core::ffi::c_int
        {
            GM_ABS_TRANS_ONLY_BITS - (allow_hp == 0) as core::ffi::c_int
        } else {
            GM_ABS_TRANS_BITS
        };
        let trans_prec_diff: core::ffi::c_int = if type_0 as core::ffi::c_int
            == TRANSLATION as core::ffi::c_int
        {
            GM_TRANS_ONLY_PREC_DIFF + (allow_hp == 0) as core::ffi::c_int
        } else {
            GM_TRANS_PREC_DIFF
        };
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (((1 as core::ffi::c_int) << trans_bits) + 1 as core::ffi::c_int)
                as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[0 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
            ((*params).wmmat[0 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
        );
        aom_wb_write_signed_primitive_refsubexpfin(
            wb,
            (((1 as core::ffi::c_int) << trans_bits) + 1 as core::ffi::c_int)
                as uint16_t,
            SUBEXPFIN_K as uint16_t,
            ((*ref_params).wmmat[1 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
            ((*params).wmmat[1 as core::ffi::c_int as usize] >> trans_prec_diff)
                as int16_t,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn write_global_motion(
    mut cpi: *mut AV1_COMP,
    mut wb: *mut aom_write_bit_buffer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut frame: core::ffi::c_int = 0;
    frame = LAST_FRAME as core::ffi::c_int;
    while frame <= ALTREF_FRAME as core::ffi::c_int {
        let mut ref_params: *const WarpedMotionParams = if !((*cm).prev_frame).is_null()
        {
            &raw mut *((*(*cm).prev_frame).global_motion).as_mut_ptr().offset(frame as isize)
                as *mut WarpedMotionParams as *const WarpedMotionParams
        } else {
            &raw const default_warp_params
        };
        write_global_motion_params(
            &raw mut *((*cm).global_motion).as_mut_ptr().offset(frame as isize),
            ref_params,
            wb,
            (*cm).features.allow_high_precision_mv as core::ffi::c_int,
        );
        frame += 1;
    }
}
unsafe extern "C-unwind" fn check_frame_refs_short_signaling(
    cm: *mut AV1_COMMON,
    mut enable_ref_short_signaling: bool,
) -> core::ffi::c_int {
    if enable_ref_short_signaling {
        let gld_map_idx: core::ffi::c_int = get_ref_frame_map_idx(
            cm,
            GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ) as core::ffi::c_int;
        let base: core::ffi::c_int = (1 as core::ffi::c_int)
            << (*(*cm).seq_params).order_hint_info.order_hint_bits_minus_1
                + 1 as core::ffi::c_int;
        let order_hint_group_cur: core::ffi::c_int = ((*cm)
            .current_frame
            .display_order_hint)
            .wrapping_div(base as core::ffi::c_uint) as core::ffi::c_int;
        let order_hint_group_gld: core::ffi::c_int = ((*(*cm)
            .ref_frame_map[gld_map_idx as usize])
            .display_order_hint)
            .wrapping_div(base as core::ffi::c_uint) as core::ffi::c_int;
        let relative_dist: core::ffi::c_int = ((*cm).current_frame.order_hint)
            .wrapping_sub((*(*cm).ref_frame_map[gld_map_idx as usize]).order_hint)
            as core::ffi::c_int;
        if order_hint_group_cur == order_hint_group_gld
            && relative_dist >= 0 as core::ffi::c_int
            && relative_dist <= 64 as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
        return 0 as core::ffi::c_int;
    }
    let mut seen_bufs: [*const RefCntBuffer; 7] = [
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
        0 as *const RefCntBuffer,
    ];
    let mut num_refs: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ref_frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while ref_frame <= ALTREF_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            ref_frame as MV_REFERENCE_FRAME,
        );
        if !buf.is_null() {
            let mut seen: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < num_refs {
                if seen_bufs[i as usize] == buf {
                    seen = 1 as core::ffi::c_int;
                    break;
                } else {
                    i += 1;
                }
            }
            if seen == 0 {
                let fresh22 = num_refs;
                num_refs = num_refs + 1;
                seen_bufs[fresh22 as usize] = buf;
            }
        }
        ref_frame += 1;
    }
    if num_refs < INTER_REFS_PER_FRAME as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut remapped_ref_idx_decoder: [core::ffi::c_int; 8] = [0; 8];
    let lst_map_idx: core::ffi::c_int = get_ref_frame_map_idx(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as core::ffi::c_int;
    let gld_map_idx_0: core::ffi::c_int = get_ref_frame_map_idx(
        cm,
        GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ) as core::ffi::c_int;
    av1_set_frame_refs(
        cm,
        remapped_ref_idx_decoder.as_mut_ptr(),
        lst_map_idx,
        gld_map_idx_0,
    );
    let mut frame_refs_short_signaling: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_idx < INTER_REFS_PER_FRAME as core::ffi::c_int {
        let mut ref_frame_buf_new: *mut RefCntBuffer = 0 as *mut RefCntBuffer;
        if remapped_ref_idx_decoder[ref_idx as usize] != INVALID_IDX {
            ref_frame_buf_new = (*cm)
                .ref_frame_map[remapped_ref_idx_decoder[ref_idx as usize] as usize];
        }
        if get_ref_frame_buf(
            cm,
            (LAST_FRAME as core::ffi::c_int + ref_idx) as MV_REFERENCE_FRAME,
        ) != ref_frame_buf_new
        {
            frame_refs_short_signaling = 0 as core::ffi::c_int;
            break;
        } else {
            ref_idx += 1;
        }
    }
    return frame_refs_short_signaling;
}
#[inline]
unsafe extern "C-unwind" fn write_uncompressed_header_obu(
    mut cpi: *mut AV1_COMP,
    xd: *mut MACROBLOCKD,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut wb: *mut aom_write_bit_buffer,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let mut quant_params: *const CommonQuantParams = &raw mut (*cm).quant_params;
    let current_frame: *mut CurrentFrame = &raw mut (*cm).current_frame;
    let features: *mut FeatureFlags = &raw mut (*cm).features;
    if !(*cpi).sf.rt_sf.enable_ref_short_signaling
        || (*seq_params).order_hint_info.enable_order_hint == 0
        || (*seq_params).order_hint_info.enable_ref_frame_mvs != 0
    {
        (*current_frame).frame_refs_short_signaling = 0 as core::ffi::c_int;
    } else {
        (*current_frame).frame_refs_short_signaling = 1 as core::ffi::c_int;
    }
    if (*seq_params).still_picture != 0 {
        if !((*cm).show_existing_frame == 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2926 as core::ffi::c_int,
                b"cm->show_existing_frame == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !((*cm).show_frame == 1 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2927 as core::ffi::c_int,
                b"cm->show_frame == 1\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !((*current_frame).frame_type as core::ffi::c_int
            == KEY_FRAME as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2928 as core::ffi::c_int,
                b"current_frame->frame_type == KEY_FRAME\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*seq_params).reduced_still_picture_hdr == 0 {
        if encode_show_existing_frame(cm) != 0 {
            aom_wb_write_bit(wb, 1 as core::ffi::c_int);
            aom_wb_write_literal(
                wb,
                (*cpi).existing_fb_idx_to_show,
                3 as core::ffi::c_int,
            );
            if (*seq_params).decoder_model_info_present_flag as core::ffi::c_int != 0
                && (*seq_params).timing_info.equal_picture_interval
                    == 0 as core::ffi::c_int
            {
                write_tu_pts_info(cm, wb);
            }
            if (*seq_params).frame_id_numbers_present_flag != 0 {
                let mut frame_id_len: core::ffi::c_int = (*seq_params).frame_id_length;
                let mut display_frame_id: core::ffi::c_int = (*cm)
                    .ref_frame_id[(*cpi).existing_fb_idx_to_show as usize];
                aom_wb_write_literal(wb, display_frame_id, frame_id_len);
            }
            return;
        } else {
            aom_wb_write_bit(wb, 0 as core::ffi::c_int);
        }
        aom_wb_write_literal(
            wb,
            (*current_frame).frame_type as core::ffi::c_int,
            2 as core::ffi::c_int,
        );
        aom_wb_write_bit(wb, (*cm).show_frame);
        if (*cm).show_frame != 0 {
            if (*seq_params).decoder_model_info_present_flag as core::ffi::c_int != 0
                && (*seq_params).timing_info.equal_picture_interval
                    == 0 as core::ffi::c_int
            {
                write_tu_pts_info(cm, wb);
            }
        } else {
            aom_wb_write_bit(wb, (*cm).showable_frame);
        }
        if frame_is_sframe(cm) != 0 {
            if !(*features).error_resilient_mode as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"write_uncompressed_header_obu\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2960 as core::ffi::c_int,
                    b"features->error_resilient_mode\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        } else if !((*current_frame).frame_type as core::ffi::c_int
            == KEY_FRAME as core::ffi::c_int && (*cm).show_frame != 0)
        {
            aom_wb_write_bit(wb, (*features).error_resilient_mode as core::ffi::c_int);
        }
    }
    aom_wb_write_bit(wb, (*features).disable_cdf_update as core::ffi::c_int);
    if (*seq_params).force_screen_content_tools as core::ffi::c_int
        == 2 as core::ffi::c_int
    {
        aom_wb_write_bit(wb, (*features).allow_screen_content_tools as core::ffi::c_int);
    } else {
        if !((*features).allow_screen_content_tools as core::ffi::c_int
            == (*seq_params).force_screen_content_tools as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2971 as core::ffi::c_int,
                b"features->allow_screen_content_tools == seq_params->force_screen_content_tools\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*features).allow_screen_content_tools {
        if (*seq_params).force_integer_mv as core::ffi::c_int == 2 as core::ffi::c_int {
            aom_wb_write_bit(
                wb,
                (*features).cur_frame_force_integer_mv as core::ffi::c_int,
            );
        } else {
            if !((*features).cur_frame_force_integer_mv as core::ffi::c_int
                == (*seq_params).force_integer_mv as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"write_uncompressed_header_obu\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    2979 as core::ffi::c_int,
                    b"features->cur_frame_force_integer_mv == seq_params->force_integer_mv\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    } else {
        if !((*features).cur_frame_force_integer_mv as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                2982 as core::ffi::c_int,
                b"features->cur_frame_force_integer_mv == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    let mut frame_size_override_flag: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*seq_params).reduced_still_picture_hdr == 0 {
        if (*seq_params).frame_id_numbers_present_flag != 0 {
            let mut frame_id_len_0: core::ffi::c_int = (*seq_params).frame_id_length;
            aom_wb_write_literal(wb, (*cm).current_frame_id, frame_id_len_0);
        }
        if (*cm).superres_upscaled_width > (*seq_params).max_frame_width
            || (*cm).superres_upscaled_height > (*seq_params).max_frame_height
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Frame dimensions are larger than the maximum values\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        frame_size_override_flag = if frame_is_sframe(cm) != 0 {
            1 as core::ffi::c_int
        } else {
            ((*cm).superres_upscaled_width != (*seq_params).max_frame_width
                || (*cm).superres_upscaled_height != (*seq_params).max_frame_height)
                as core::ffi::c_int
        };
        if frame_is_sframe(cm) == 0 {
            aom_wb_write_bit(wb, frame_size_override_flag);
        }
        if (*seq_params).order_hint_info.enable_order_hint != 0 {
            aom_wb_write_literal(
                wb,
                (*current_frame).order_hint as core::ffi::c_int,
                (*seq_params).order_hint_info.order_hint_bits_minus_1
                    + 1 as core::ffi::c_int,
            );
        }
        if !(*features).error_resilient_mode && frame_is_intra_only(cm) == 0 {
            aom_wb_write_literal(wb, (*features).primary_ref_frame, PRIMARY_REF_BITS);
        }
    }
    if (*seq_params).decoder_model_info_present_flag != 0 {
        aom_wb_write_bit(
            wb,
            (*(*cpi).ppi).buffer_removal_time_present as core::ffi::c_int,
        );
        if (*(*cpi).ppi).buffer_removal_time_present {
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
                        aom_wb_write_unsigned_literal(
                            wb,
                            (*cm).buffer_removal_times[op_num as usize],
                            (*seq_params).decoder_model_info.buffer_removal_time_length,
                        );
                        (*cm).buffer_removal_times[op_num as usize] = ((*cm)
                            .buffer_removal_times[op_num as usize])
                            .wrapping_add(1);
                        if (*cm).buffer_removal_times[op_num as usize] == 0 as uint32_t {
                            aom_internal_error(
                                (*cm).error,
                                AOM_CODEC_UNSUP_BITSTREAM,
                                b"buffer_removal_time overflowed\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                    }
                }
                op_num += 1;
            }
        }
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        && (*cm).show_frame == 0
        || (*current_frame).frame_type as core::ffi::c_int
            == INTER_FRAME as core::ffi::c_int
        || (*current_frame).frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int
    {
        aom_wb_write_literal(
            wb,
            (*current_frame).refresh_frame_flags,
            REF_FRAMES as core::ffi::c_int,
        );
    }
    if frame_is_intra_only(cm) == 0
        || (*current_frame).refresh_frame_flags != 0xff as core::ffi::c_int
    {
        if (*features).error_resilient_mode as core::ffi::c_int != 0
            && (*seq_params).order_hint_info.enable_order_hint != 0
        {
            let mut ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while ref_idx < REF_FRAMES as core::ffi::c_int {
                aom_wb_write_literal(
                    wb,
                    (*(*cm).ref_frame_map[ref_idx as usize]).order_hint
                        as core::ffi::c_int,
                    (*seq_params).order_hint_info.order_hint_bits_minus_1
                        + 1 as core::ffi::c_int,
                );
                ref_idx += 1;
            }
        }
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        write_frame_size(cm, frame_size_override_flag, wb);
        if !(av1_superres_scaled(cm) == 0 || !(*features).allow_intrabc)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3064 as core::ffi::c_int,
                b"!av1_superres_scaled(cm) || !features->allow_intrabc\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if (*features).allow_screen_content_tools as core::ffi::c_int != 0
            && av1_superres_scaled(cm) == 0
        {
            aom_wb_write_bit(wb, (*features).allow_intrabc as core::ffi::c_int);
        }
    } else if (*current_frame).frame_type as core::ffi::c_int
        == INTRA_ONLY_FRAME as core::ffi::c_int
    {
        write_frame_size(cm, frame_size_override_flag, wb);
        if !(av1_superres_scaled(cm) == 0 || !(*features).allow_intrabc)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3070 as core::ffi::c_int,
                b"!av1_superres_scaled(cm) || !features->allow_intrabc\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if (*features).allow_screen_content_tools as core::ffi::c_int != 0
            && av1_superres_scaled(cm) == 0
        {
            aom_wb_write_bit(wb, (*features).allow_intrabc as core::ffi::c_int);
        }
    } else if (*current_frame).frame_type as core::ffi::c_int
        == INTER_FRAME as core::ffi::c_int || frame_is_sframe(cm) != 0
    {
        let mut ref_frame: MV_REFERENCE_FRAME = 0;
        if (*current_frame).frame_refs_short_signaling != 0 {
            (*current_frame).frame_refs_short_signaling = check_frame_refs_short_signaling(
                cm,
                (*cpi).sf.rt_sf.enable_ref_short_signaling,
            );
        }
        if (*seq_params).order_hint_info.enable_order_hint != 0 {
            aom_wb_write_bit(wb, (*current_frame).frame_refs_short_signaling);
        }
        if (*current_frame).frame_refs_short_signaling != 0 {
            let lst_ref: core::ffi::c_int = get_ref_frame_map_idx(
                cm,
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ) as core::ffi::c_int;
            aom_wb_write_literal(wb, lst_ref, REF_FRAMES_LOG2);
            let gld_ref: core::ffi::c_int = get_ref_frame_map_idx(
                cm,
                GOLDEN_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            ) as core::ffi::c_int;
            aom_wb_write_literal(wb, gld_ref, REF_FRAMES_LOG2);
        }
        let mut first_ref_map_idx: core::ffi::c_int = INVALID_IDX;
        if (*(*cpi).ppi).rtc_ref.set_ref_frame_config != 0 {
            ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
            while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
                if (*(*cpi).ppi)
                    .rtc_ref
                    .reference[(ref_frame as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] == 1 as core::ffi::c_int
                {
                    first_ref_map_idx = (*(*cpi).ppi)
                        .rtc_ref
                        .ref_idx[(ref_frame as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize];
                    break;
                } else {
                    ref_frame += 1;
                }
            }
        }
        ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
        while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
            if !(get_ref_frame_map_idx(cm, ref_frame) != -(1 as core::ffi::c_int))
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"write_uncompressed_header_obu\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    3122 as core::ffi::c_int,
                    b"get_ref_frame_map_idx(cm, ref_frame) != INVALID_IDX\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if (*current_frame).frame_refs_short_signaling == 0 {
                if (*(*cpi).ppi).rtc_ref.set_ref_frame_config != 0
                    && first_ref_map_idx != INVALID_IDX
                    && (*cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
                    && (*seq_params).order_hint_info.enable_order_hint == 0
                {
                    let map_idx: core::ffi::c_int = if (*(*cpi).ppi)
                        .rtc_ref
                        .reference[(ref_frame as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] != 0
                    {
                        get_ref_frame_map_idx(cm, ref_frame) as core::ffi::c_int
                    } else {
                        first_ref_map_idx
                    };
                    aom_wb_write_literal(wb, map_idx, REF_FRAMES_LOG2);
                } else {
                    aom_wb_write_literal(
                        wb,
                        get_ref_frame_map_idx(cm, ref_frame),
                        REF_FRAMES_LOG2,
                    );
                }
            }
            if (*seq_params).frame_id_numbers_present_flag != 0 {
                let mut i: core::ffi::c_int = get_ref_frame_map_idx(cm, ref_frame);
                let mut frame_id_len_1: core::ffi::c_int = (*seq_params).frame_id_length;
                let mut diff_len: core::ffi::c_int = (*seq_params).delta_frame_id_length;
                let mut delta_frame_id_minus_1: core::ffi::c_int = ((*cm)
                    .current_frame_id - (*cm).ref_frame_id[i as usize]
                    + ((1 as core::ffi::c_int) << frame_id_len_1))
                    % ((1 as core::ffi::c_int) << frame_id_len_1)
                    - 1 as core::ffi::c_int;
                if delta_frame_id_minus_1 < 0 as core::ffi::c_int
                    || delta_frame_id_minus_1 >= (1 as core::ffi::c_int) << diff_len
                {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_ERROR,
                        b"Invalid delta_frame_id_minus_1\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                aom_wb_write_literal(wb, delta_frame_id_minus_1, diff_len);
            }
            ref_frame += 1;
        }
        if !(*features).error_resilient_mode && frame_size_override_flag != 0 {
            write_frame_size_with_refs(cm, wb);
        } else {
            write_frame_size(cm, frame_size_override_flag, wb);
        }
        if !(*features).cur_frame_force_integer_mv {
            aom_wb_write_bit(
                wb,
                (*features).allow_high_precision_mv as core::ffi::c_int,
            );
        }
        write_frame_interp_filter((*features).interp_filter, wb);
        aom_wb_write_bit(wb, (*features).switchable_motion_mode as core::ffi::c_int);
        if frame_might_allow_ref_frame_mvs(cm) != 0 {
            aom_wb_write_bit(wb, (*features).allow_ref_frame_mvs as core::ffi::c_int);
        } else {
            if !((*features).allow_ref_frame_mvs as core::ffi::c_int
                == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"write_uncompressed_header_obu\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    3171 as core::ffi::c_int,
                    b"features->allow_ref_frame_mvs == 0\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
    }
    let might_bwd_adapt: core::ffi::c_int = ((*seq_params).reduced_still_picture_hdr == 0
        && !(*features).disable_cdf_update) as core::ffi::c_int;
    if (*cm).tiles.large_scale != 0 {
        if !((*features).refresh_frame_context as core::ffi::c_int
            == REFRESH_FRAME_CONTEXT_DISABLED as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3179 as core::ffi::c_int,
                b"features->refresh_frame_context == REFRESH_FRAME_CONTEXT_DISABLED\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    if might_bwd_adapt != 0 {
        aom_wb_write_bit(
            wb,
            ((*features).refresh_frame_context as core::ffi::c_int
                == REFRESH_FRAME_CONTEXT_DISABLED as core::ffi::c_int)
                as core::ffi::c_int,
        );
    }
    write_tile_info(cm, saved_wb, wb);
    encode_quantization(
        quant_params,
        av1_num_planes(cm),
        (*(*cm).seq_params).separate_uv_delta_q != 0,
        wb,
    );
    encode_segmentation(cm, wb);
    let delta_q_info: *const DeltaQInfo = &raw mut (*cm).delta_q_info;
    if (*delta_q_info).delta_q_present_flag != 0 {
        if !((*quant_params).base_qindex > 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3192 as core::ffi::c_int,
                b"quant_params->base_qindex > 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    if (*quant_params).base_qindex > 0 as core::ffi::c_int {
        aom_wb_write_bit(wb, (*delta_q_info).delta_q_present_flag);
        if (*delta_q_info).delta_q_present_flag != 0 {
            aom_wb_write_literal(
                wb,
                get_msb((*delta_q_info).delta_q_res as core::ffi::c_uint),
                2 as core::ffi::c_int,
            );
            (*xd).current_base_qindex = (*quant_params).base_qindex;
            if (*features).allow_intrabc {
                if !((*delta_q_info).delta_lf_present_flag == 0 as core::ffi::c_int)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 30],
                            [core::ffi::c_char; 30],
                        >(*b"write_uncompressed_header_obu\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        3199 as core::ffi::c_int,
                        b"delta_q_info->delta_lf_present_flag == 0\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
            } else {
                aom_wb_write_bit(wb, (*delta_q_info).delta_lf_present_flag);
            }
            if (*delta_q_info).delta_lf_present_flag != 0 {
                aom_wb_write_literal(
                    wb,
                    get_msb((*delta_q_info).delta_lf_res as core::ffi::c_uint),
                    2 as core::ffi::c_int,
                );
                aom_wb_write_bit(wb, (*delta_q_info).delta_lf_multi);
                av1_reset_loop_filter_delta(xd, av1_num_planes(cm));
            }
        }
    }
    if (*features).all_lossless {
        if (av1_superres_scaled(cm) != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3211 as core::ffi::c_int,
                b"!av1_superres_scaled(cm)\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    } else {
        if !(*features).coded_lossless {
            encode_loopfilter(cm, wb);
            encode_cdef(cm, wb);
        }
        encode_restoration_mode(cm, wb);
    }
    if (*features).coded_lossless {
        if !((*features).tx_mode as core::ffi::c_int == ONLY_4X4 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3222 as core::ffi::c_int,
                b"features->tx_mode == ONLY_4X4\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    } else {
        aom_wb_write_bit(
            wb,
            ((*features).tx_mode as core::ffi::c_int
                == TX_MODE_SELECT as core::ffi::c_int) as core::ffi::c_int,
        );
    }
    if frame_is_intra_only(cm) == 0 {
        let use_hybrid_pred: core::ffi::c_int = ((*current_frame).reference_mode
            as core::ffi::c_int == REFERENCE_MODE_SELECT as core::ffi::c_int)
            as core::ffi::c_int;
        aom_wb_write_bit(wb, use_hybrid_pred);
    }
    if (*current_frame).skip_mode_info.skip_mode_allowed != 0 {
        aom_wb_write_bit(wb, (*current_frame).skip_mode_info.skip_mode_flag);
    }
    if frame_might_allow_warped_motion(cm) != 0 {
        aom_wb_write_bit(wb, (*features).allow_warped_motion as core::ffi::c_int);
    } else {
        if (*features).allow_warped_motion as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"write_uncompressed_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3239 as core::ffi::c_int,
                b"!features->allow_warped_motion\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
    }
    aom_wb_write_bit(wb, (*features).reduced_tx_set_used as core::ffi::c_int);
    if frame_is_intra_only(cm) == 0 {
        write_global_motion(cpi, wb);
    }
    if (*seq_params).film_grain_params_present as core::ffi::c_int != 0
        && ((*cm).show_frame != 0 || (*cm).showable_frame != 0)
    {
        write_film_grain_params(cpi, wb);
    }
    if (*cm).tiles.large_scale != 0 {
        write_ext_tile_info(cm, saved_wb, wb);
    }
}
unsafe extern "C-unwind" fn choose_size_bytes(
    mut size: uint32_t,
    mut spare_msbs: core::ffi::c_int,
) -> core::ffi::c_int {
    if spare_msbs > 0 as core::ffi::c_int
        && size >> 32 as core::ffi::c_int - spare_msbs != 0 as uint32_t
    {
        return -(1 as core::ffi::c_int);
    }
    size <<= spare_msbs;
    if size >> 24 as core::ffi::c_int != 0 as uint32_t {
        return 4 as core::ffi::c_int
    } else if size >> 16 as core::ffi::c_int != 0 as uint32_t {
        return 3 as core::ffi::c_int
    } else if size >> 8 as core::ffi::c_int != 0 as uint32_t {
        return 2 as core::ffi::c_int
    } else {
        return 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn mem_put_varsize(
    dst: *mut uint8_t,
    sz: core::ffi::c_int,
    val: core::ffi::c_int,
) {
    match sz {
        1 => {
            *dst.offset(0 as core::ffi::c_int as isize) = (val
                & 0xff as core::ffi::c_int) as uint8_t;
        }
        2 => {
            mem_put_le16_as_int(dst as *mut core::ffi::c_void, val);
        }
        3 => {
            mem_put_le24_as_int(dst as *mut core::ffi::c_void, val);
        }
        4 => {
            mem_put_le32_as_int(dst as *mut core::ffi::c_void, val);
        }
        _ => {
            if !(0 as core::ffi::c_int != 0
                && !(b"Invalid size\0" as *const u8 as *const core::ffi::c_char)
                    .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 16],
                        [core::ffi::c_char; 16],
                    >(*b"mem_put_varsize\0"))
                        .as_ptr(),
                    b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                    3279 as core::ffi::c_int,
                    b"0 && \"Invalid size\"\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    };
}
unsafe extern "C-unwind" fn remux_tiles(
    tiles: *const CommonTileParams,
    mut dst: *mut uint8_t,
    data_size: uint32_t,
    max_tile_size: uint32_t,
    max_tile_col_size: uint32_t,
    tile_size_bytes: *mut core::ffi::c_int,
    tile_col_size_bytes: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut tsb: core::ffi::c_int = 0;
    let mut tcsb: core::ffi::c_int = 0;
    if (*tiles).large_scale != 0 {
        tsb = choose_size_bytes(max_tile_size, 1 as core::ffi::c_int);
        tcsb = choose_size_bytes(max_tile_col_size, 0 as core::ffi::c_int);
    } else {
        tsb = choose_size_bytes(max_tile_size, 0 as core::ffi::c_int);
        tcsb = 4 as core::ffi::c_int;
    }
    if !(tsb > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"remux_tiles\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3303 as core::ffi::c_int,
            b"tsb > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(tcsb > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"remux_tiles\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3304 as core::ffi::c_int,
            b"tcsb > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *tile_size_bytes = tsb;
    *tile_col_size_bytes = tcsb;
    if tsb == 4 as core::ffi::c_int && tcsb == 4 as core::ffi::c_int {
        return data_size as core::ffi::c_int;
    }
    let mut wpos: uint32_t = 0 as uint32_t;
    let mut rpos: uint32_t = 0 as uint32_t;
    if (*tiles).large_scale != 0 {
        let mut tile_row: core::ffi::c_int = 0;
        let mut tile_col: core::ffi::c_int = 0;
        tile_col = 0 as core::ffi::c_int;
        while tile_col < (*tiles).cols {
            if tile_col < (*tiles).cols - 1 as core::ffi::c_int {
                let mut tile_col_size: uint32_t = mem_get_le32_as_int(
                    dst.offset(rpos as isize) as *const core::ffi::c_void,
                ) as uint32_t;
                rpos = rpos.wrapping_add(4 as uint32_t);
                tile_col_size = tile_col_size
                    .wrapping_sub(
                        ((4 as core::ffi::c_int - tsb) * (*tiles).rows) as uint32_t,
                    );
                mem_put_varsize(
                    dst.offset(wpos as isize),
                    tcsb,
                    tile_col_size as core::ffi::c_int,
                );
                wpos = wpos.wrapping_add(tcsb as uint32_t);
            }
            tile_row = 0 as core::ffi::c_int;
            while tile_row < (*tiles).rows {
                let mut tile_header: uint32_t = mem_get_le32_as_int(
                    dst.offset(rpos as isize) as *const core::ffi::c_void,
                ) as uint32_t;
                rpos = rpos.wrapping_add(4 as uint32_t);
                if tile_header >> 31 as core::ffi::c_int != 0 as uint32_t {
                    if tsb < 4 as core::ffi::c_int {
                        tile_header
                            >>= 32 as core::ffi::c_int - 8 as core::ffi::c_int * tsb;
                    }
                    mem_put_varsize(
                        dst.offset(wpos as isize),
                        tsb,
                        tile_header as core::ffi::c_int,
                    );
                    wpos = wpos.wrapping_add(tsb as uint32_t);
                } else {
                    mem_put_varsize(
                        dst.offset(wpos as isize),
                        tsb,
                        tile_header as core::ffi::c_int,
                    );
                    wpos = wpos.wrapping_add(tsb as uint32_t);
                    tile_header = tile_header
                        .wrapping_add(AV1_MIN_TILE_SIZE_BYTES as uint32_t);
                    memmove(
                        dst.offset(wpos as isize) as *mut core::ffi::c_void,
                        dst.offset(rpos as isize) as *const core::ffi::c_void,
                        tile_header as size_t,
                    );
                    rpos = rpos.wrapping_add(tile_header);
                    wpos = wpos.wrapping_add(tile_header);
                }
                tile_row += 1;
            }
            tile_col += 1;
        }
        if !(rpos > wpos) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"remux_tiles\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3354 as core::ffi::c_int,
                b"rpos > wpos\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if !(rpos == data_size) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"remux_tiles\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3355 as core::ffi::c_int,
                b"rpos == data_size\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return wpos as core::ffi::c_int;
    }
    let n_tiles: core::ffi::c_int = (*tiles).cols * (*tiles).rows;
    let mut n: core::ffi::c_int = 0;
    n = 0 as core::ffi::c_int;
    while n < n_tiles {
        let mut tile_size: core::ffi::c_int = 0;
        if n == n_tiles - 1 as core::ffi::c_int {
            tile_size = data_size.wrapping_sub(rpos) as core::ffi::c_int;
        } else {
            tile_size = mem_get_le32_as_int(
                dst.offset(rpos as isize) as *const core::ffi::c_void,
            ) as core::ffi::c_int;
            rpos = rpos.wrapping_add(4 as uint32_t);
            mem_put_varsize(dst.offset(wpos as isize), tsb, tile_size);
            tile_size += AV1_MIN_TILE_SIZE_BYTES;
            wpos = wpos.wrapping_add(tsb as uint32_t);
        }
        memmove(
            dst.offset(wpos as isize) as *mut core::ffi::c_void,
            dst.offset(rpos as isize) as *const core::ffi::c_void,
            tile_size as size_t,
        );
        rpos = rpos.wrapping_add(tile_size as uint32_t);
        wpos = wpos.wrapping_add(tile_size as uint32_t);
        n += 1;
    }
    if !(rpos > wpos) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"remux_tiles\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3381 as core::ffi::c_int,
            b"rpos > wpos\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(rpos == data_size) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"remux_tiles\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3382 as core::ffi::c_int,
            b"rpos == data_size\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return wpos as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_obu_header(
    level_params: *mut AV1LevelParams,
    mut frame_header_count: *mut core::ffi::c_int,
    mut obu_type: OBU_TYPE,
    mut has_nonzero_operating_point_idc: bool,
    mut is_layer_specific_obu: bool,
    mut obu_extension: core::ffi::c_int,
    dst: *mut uint8_t,
) -> uint32_t {
    if !(has_nonzero_operating_point_idc || obu_extension == 0 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_write_obu_header\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3392 as core::ffi::c_int,
            b"IMPLIES(!has_nonzero_operating_point_idc, obu_extension == 0)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*level_params).keep_level_stats != 0
        && (obu_type as core::ffi::c_int == OBU_FRAME as core::ffi::c_int
            || obu_type as core::ffi::c_int == OBU_FRAME_HEADER as core::ffi::c_int)
    {
        *frame_header_count += 1;
    }
    let mut size: uint32_t = 0 as uint32_t;
    let obu_extension_flag: core::ffi::c_int = (has_nonzero_operating_point_idc
        as core::ffi::c_int != 0 && is_layer_specific_obu as core::ffi::c_int != 0)
        as core::ffi::c_int;
    let obu_has_size_field: core::ffi::c_int = 1 as core::ffi::c_int;
    *dst.offset(0 as core::ffi::c_int as isize) = ((obu_type as core::ffi::c_int)
        << 3 as core::ffi::c_int | obu_extension_flag << 2 as core::ffi::c_int
        | obu_has_size_field << 1 as core::ffi::c_int) as uint8_t;
    size = size.wrapping_add(1);
    if obu_extension_flag != 0 {
        *dst.offset(1 as core::ffi::c_int as isize) = (obu_extension
            & 0xff as core::ffi::c_int) as uint8_t;
        size = size.wrapping_add(1);
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_uleb_obu_size(
    mut obu_payload_size: size_t,
    mut dest: *mut uint8_t,
    mut dest_size: size_t,
) -> core::ffi::c_int {
    let mut coded_obu_size: size_t = 0 as size_t;
    if aom_uleb_encode(
        obu_payload_size as uint64_t,
        dest_size,
        dest,
        &raw mut coded_obu_size,
    ) != 0 as core::ffi::c_int
    {
        return AOM_CODEC_ERROR as core::ffi::c_int;
    }
    if coded_obu_size != dest_size {
        return AOM_CODEC_ERROR as core::ffi::c_int;
    }
    return AOM_CODEC_OK as core::ffi::c_int;
}
unsafe extern "C-unwind" fn av1_write_uleb_obu_size_unsafe(
    mut obu_payload_size: size_t,
    mut dest: *mut uint8_t,
) -> core::ffi::c_int {
    let mut coded_obu_size: size_t = 0 as size_t;
    if aom_uleb_encode(
        obu_payload_size as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
        dest,
        &raw mut coded_obu_size,
    ) != 0 as core::ffi::c_int
    {
        return AOM_CODEC_ERROR as core::ffi::c_int;
    }
    return AOM_CODEC_OK as core::ffi::c_int;
}
unsafe extern "C-unwind" fn obu_memmove(
    mut obu_header_size: size_t,
    mut obu_payload_size: size_t,
    mut data: *mut uint8_t,
    mut data_size: size_t,
) -> size_t {
    let length_field_size: size_t = aom_uleb_size_in_bytes(obu_payload_size as uint64_t)
        as size_t;
    let move_dst_offset: size_t = obu_header_size.wrapping_add(length_field_size);
    let move_src_offset: size_t = obu_header_size;
    let move_size: size_t = obu_payload_size;
    if move_size > data_size || move_src_offset > data_size.wrapping_sub(move_size) {
        if !(0 as core::ffi::c_int != 0
            && !(b"obu_memmove: output buffer overflow\0" as *const u8
                as *const core::ffi::c_char)
                .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"obu_memmove\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3466 as core::ffi::c_int,
                b"0 && \"obu_memmove: output buffer overflow\"\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        return 0 as size_t;
    }
    if move_dst_offset > data_size.wrapping_sub(move_size) {
        return 0 as size_t;
    }
    memmove(
        data.offset(move_dst_offset as isize) as *mut core::ffi::c_void,
        data.offset(move_src_offset as isize) as *const core::ffi::c_void,
        move_size,
    );
    return length_field_size;
}
unsafe extern "C-unwind" fn obu_memmove_unsafe(
    mut obu_header_size: size_t,
    mut obu_payload_size: size_t,
    mut data: *mut uint8_t,
) -> size_t {
    let length_field_size: size_t = aom_uleb_size_in_bytes(obu_payload_size as uint64_t)
        as size_t;
    let move_dst_offset: size_t = obu_header_size.wrapping_add(length_field_size);
    let move_src_offset: size_t = obu_header_size;
    let move_size: size_t = obu_payload_size;
    memmove(
        data.offset(move_dst_offset as isize) as *mut core::ffi::c_void,
        data.offset(move_src_offset as isize) as *const core::ffi::c_void,
        move_size,
    );
    return length_field_size;
}
#[inline]
unsafe extern "C-unwind" fn add_trailing_bits(mut wb: *mut aom_write_bit_buffer) {
    if aom_wb_is_byte_aligned(wb) != 0 {
        aom_wb_write_literal(wb, 0x80 as core::ffi::c_int, 8 as core::ffi::c_int);
    } else {
        aom_wb_write_bit(wb, 1 as core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C-unwind" fn write_bitstream_level(
    mut seq_level_idx: AV1_LEVEL,
    mut wb: *mut aom_write_bit_buffer,
) {
    if (is_valid_seq_level_idx(seq_level_idx) == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"write_bitstream_level\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3499 as core::ffi::c_int,
            b"is_valid_seq_level_idx(seq_level_idx)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_write_literal(wb, seq_level_idx as core::ffi::c_int, LEVEL_BITS);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_sequence_header_obu(
    mut seq_params: *const SequenceHeader,
    dst: *mut uint8_t,
    mut dst_size: size_t,
) -> uint32_t {
    let mut wb: aom_write_bit_buffer = {
        let mut init = aom_write_bit_buffer {
            bit_buffer: dst,
            bit_offset: 0 as uint32_t,
        };
        init
    };
    let mut size: uint32_t = 0 as uint32_t;
    write_profile((*seq_params).profile, &raw mut wb);
    aom_wb_write_bit(&raw mut wb, (*seq_params).still_picture as core::ffi::c_int);
    if !((*seq_params).still_picture != 0
        || (*seq_params).reduced_still_picture_hdr == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_write_sequence_header_obu\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3515 as core::ffi::c_int,
            b"IMPLIES(!seq_params->still_picture, !seq_params->reduced_still_picture_hdr)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_write_bit(
        &raw mut wb,
        (*seq_params).reduced_still_picture_hdr as core::ffi::c_int,
    );
    if (*seq_params).reduced_still_picture_hdr != 0 {
        if !((*seq_params).timing_info_present == 0 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"av1_write_sequence_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3520 as core::ffi::c_int,
                b"seq_params->timing_info_present == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !((*seq_params).decoder_model_info_present_flag as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"av1_write_sequence_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3521 as core::ffi::c_int,
                b"seq_params->decoder_model_info_present_flag == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if !((*seq_params).display_model_info_present_flag as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"av1_write_sequence_header_obu\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3522 as core::ffi::c_int,
                b"seq_params->display_model_info_present_flag == 0\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        write_bitstream_level(
            (*seq_params).seq_level_idx[0 as core::ffi::c_int as usize],
            &raw mut wb,
        );
    } else {
        aom_wb_write_bit(&raw mut wb, (*seq_params).timing_info_present);
        if (*seq_params).timing_info_present != 0 {
            write_timing_info_header(&(*seq_params).timing_info, &raw mut wb);
            aom_wb_write_bit(
                &raw mut wb,
                (*seq_params).decoder_model_info_present_flag as core::ffi::c_int,
            );
            if (*seq_params).decoder_model_info_present_flag != 0 {
                write_decoder_model_info(&(*seq_params).decoder_model_info, &raw mut wb);
            }
        }
        aom_wb_write_bit(
            &raw mut wb,
            (*seq_params).display_model_info_present_flag as core::ffi::c_int,
        );
        aom_wb_write_literal(
            &raw mut wb,
            (*seq_params).operating_points_cnt_minus_1,
            OP_POINTS_CNT_MINUS_1_BITS,
        );
        let mut i: core::ffi::c_int = 0;
        i = 0 as core::ffi::c_int;
        while i < (*seq_params).operating_points_cnt_minus_1 + 1 as core::ffi::c_int {
            aom_wb_write_literal(
                &raw mut wb,
                (*seq_params).operating_point_idc[i as usize],
                OP_POINTS_IDC_BITS,
            );
            write_bitstream_level((*seq_params).seq_level_idx[i as usize], &raw mut wb);
            if (*seq_params).seq_level_idx[i as usize] as core::ffi::c_int
                >= SEQ_LEVEL_4_0 as core::ffi::c_int
            {
                aom_wb_write_bit(
                    &raw mut wb,
                    (*seq_params).tier[i as usize] as core::ffi::c_int,
                );
            }
            if (*seq_params).decoder_model_info_present_flag != 0 {
                aom_wb_write_bit(
                    &raw mut wb,
                    (*seq_params).op_params[i as usize].decoder_model_param_present_flag,
                );
                if (*seq_params).op_params[i as usize].decoder_model_param_present_flag
                    != 0
                {
                    write_dec_model_op_parameters(
                        &*((*seq_params).op_params).as_ptr().offset(i as isize),
                        (*seq_params)
                            .decoder_model_info
                            .encoder_decoder_buffer_delay_length,
                        &raw mut wb,
                    );
                }
            }
            if (*seq_params).display_model_info_present_flag != 0 {
                aom_wb_write_bit(
                    &raw mut wb,
                    (*seq_params).op_params[i as usize].display_model_param_present_flag,
                );
                if (*seq_params).op_params[i as usize].display_model_param_present_flag
                    != 0
                {
                    if !((*seq_params).op_params[i as usize].initial_display_delay
                        >= 1 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 30],
                                [core::ffi::c_char; 30],
                            >(*b"av1_write_sequence_header_obu\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            3561 as core::ffi::c_int,
                            b"seq_params->op_params[i].initial_display_delay >= 1\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    if !((*seq_params).op_params[i as usize].initial_display_delay
                        <= 10 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 30],
                                [core::ffi::c_char; 30],
                            >(*b"av1_write_sequence_header_obu\0"))
                                .as_ptr(),
                            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                            3562 as core::ffi::c_int,
                            b"seq_params->op_params[i].initial_display_delay <= 10\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    aom_wb_write_literal(
                        &raw mut wb,
                        (*seq_params).op_params[i as usize].initial_display_delay
                            - 1 as core::ffi::c_int,
                        4 as core::ffi::c_int,
                    );
                }
            }
            i += 1;
        }
    }
    write_sequence_header(seq_params, &raw mut wb);
    write_color_config(seq_params, &raw mut wb);
    aom_wb_write_bit(
        &raw mut wb,
        (*seq_params).film_grain_params_present as core::ffi::c_int,
    );
    add_trailing_bits(&raw mut wb);
    size = aom_wb_bytes_written(&raw mut wb);
    return size;
}
unsafe extern "C-unwind" fn write_frame_header_obu(
    mut cpi: *mut AV1_COMP,
    xd: *mut MACROBLOCKD,
    mut saved_wb: *mut aom_write_bit_buffer,
    dst: *mut uint8_t,
    mut append_trailing_bits: core::ffi::c_int,
) -> uint32_t {
    let mut wb: aom_write_bit_buffer = {
        let mut init = aom_write_bit_buffer {
            bit_buffer: dst,
            bit_offset: 0 as uint32_t,
        };
        init
    };
    write_uncompressed_header_obu(cpi, xd, saved_wb, &raw mut wb);
    if append_trailing_bits != 0 {
        add_trailing_bits(&raw mut wb);
    }
    return aom_wb_bytes_written(&raw mut wb);
}
unsafe extern "C-unwind" fn write_tile_group_header(
    dst: *mut uint8_t,
    mut start_tile: core::ffi::c_int,
    mut end_tile: core::ffi::c_int,
    mut tiles_log2: core::ffi::c_int,
    mut tile_start_and_end_present_flag: core::ffi::c_int,
) -> uint32_t {
    let mut wb: aom_write_bit_buffer = {
        let mut init = aom_write_bit_buffer {
            bit_buffer: dst,
            bit_offset: 0 as uint32_t,
        };
        init
    };
    let mut size: uint32_t = 0 as uint32_t;
    if tiles_log2 == 0 {
        return size;
    }
    aom_wb_write_bit(&raw mut wb, tile_start_and_end_present_flag);
    if tile_start_and_end_present_flag != 0 {
        aom_wb_write_literal(&raw mut wb, start_tile, tiles_log2);
        aom_wb_write_literal(&raw mut wb, end_tile, tiles_log2);
    }
    size = aom_wb_bytes_written(&raw mut wb);
    return size;
}
unsafe extern "C-unwind" fn init_large_scale_tile_obu_header(
    cpi: *mut AV1_COMP,
    mut data: *mut *mut uint8_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut obu_extension_header: uint8_t,
    mut lst_obu: *mut LargeTileFrameOBU,
) -> uint32_t {
    let level_params: *mut AV1LevelParams = &raw mut (*(*cpi).ppi).level_params;
    let current_frame: *mut CurrentFrame = &raw mut (*cpi).common.current_frame;
    let obu_type: OBU_TYPE = OBU_FRAME;
    (*lst_obu).tg_hdr_size = av1_write_obu_header(
        level_params,
        &raw mut (*cpi).frame_header_count,
        obu_type,
        (*(*cpi).common.seq_params).has_nonzero_operating_point_idc,
        true_0 != 0,
        obu_extension_header as core::ffi::c_int,
        *data,
    );
    *data = (*data).offset((*lst_obu).tg_hdr_size as isize);
    let frame_header_size: uint32_t = write_frame_header_obu(
        cpi,
        &raw mut (*cpi).td.mb.e_mbd,
        saved_wb,
        *data,
        0 as core::ffi::c_int,
    ) as uint32_t;
    *data = (*data).offset(frame_header_size as isize);
    (*lst_obu).frame_header_size = frame_header_size;
    if (*cpi).oxcf.tile_cfg.enable_ext_tile_debug {
        let mut fn_0: [core::ffi::c_char; 20] = ::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"./fh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        fn_0[4 as core::ffi::c_int as usize] = ((*current_frame).frame_number)
            .wrapping_div(100 as core::ffi::c_uint)
            .wrapping_add('0' as i32 as core::ffi::c_uint) as core::ffi::c_char;
        fn_0[5 as core::ffi::c_int as usize] = ((*current_frame).frame_number)
            .wrapping_rem(100 as core::ffi::c_uint)
            .wrapping_div(10 as core::ffi::c_uint)
            .wrapping_add('0' as i32 as core::ffi::c_uint) as core::ffi::c_char;
        fn_0[6 as core::ffi::c_int as usize] = ((*current_frame).frame_number)
            .wrapping_rem(10 as core::ffi::c_uint)
            .wrapping_add('0' as i32 as core::ffi::c_uint) as core::ffi::c_char;
        fn_0[7 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
        av1_print_uncompressed_frame_header(
            (*data).offset(-(frame_header_size as isize)),
            frame_header_size as core::ffi::c_int,
            fn_0.as_mut_ptr(),
        );
    }
    return frame_header_size;
}
unsafe extern "C-unwind" fn write_large_scale_tile_obu_size(
    tiles: *const CommonTileParams,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut data: *mut uint8_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    lst_obu: *mut LargeTileFrameOBU,
    mut have_tiles: core::ffi::c_int,
    mut total_size: *mut uint32_t,
    mut max_tile_size: core::ffi::c_int,
    mut max_tile_col_size: core::ffi::c_int,
) {
    let mut tile_size_bytes: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_col_size_bytes: core::ffi::c_int = 0 as core::ffi::c_int;
    if have_tiles != 0 {
        *total_size = remux_tiles(
            tiles,
            data,
            (*total_size).wrapping_sub((*lst_obu).frame_header_size),
            max_tile_size as uint32_t,
            max_tile_col_size as uint32_t,
            &raw mut tile_size_bytes,
            &raw mut tile_col_size_bytes,
        ) as uint32_t;
        *total_size = (*total_size).wrapping_add((*lst_obu).frame_header_size);
    }
    *total_size = (*total_size).wrapping_add((*lst_obu).tg_hdr_size);
    let obu_payload_size: uint32_t = (*total_size).wrapping_sub((*lst_obu).tg_hdr_size);
    let length_field_size: size_t = obu_memmove_unsafe(
        (*lst_obu).tg_hdr_size as size_t,
        obu_payload_size as size_t,
        dst,
    ) as size_t;
    if av1_write_uleb_obu_size_unsafe(
        obu_payload_size as size_t,
        dst.offset((*lst_obu).tg_hdr_size as isize),
    ) != AOM_CODEC_OK as core::ffi::c_int
    {
        if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 32],
                    [core::ffi::c_char; 32],
                >(*b"write_large_scale_tile_obu_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3674 as core::ffi::c_int,
                b"0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    *total_size = (*total_size).wrapping_add(length_field_size as uint32_t);
    (*saved_wb).bit_buffer = ((*saved_wb).bit_buffer).offset(length_field_size as isize);
    if have_tiles != 0 {
        if !(tile_col_size_bytes >= 1 as core::ffi::c_int
            && tile_col_size_bytes <= 4 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 32],
                    [core::ffi::c_char; 32],
                >(*b"write_large_scale_tile_obu_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3681 as core::ffi::c_int,
                b"tile_col_size_bytes >= 1 && tile_col_size_bytes <= 4\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        aom_wb_overwrite_literal(
            saved_wb,
            tile_col_size_bytes - 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
        );
        if !(tile_size_bytes >= 1 as core::ffi::c_int
            && tile_size_bytes <= 4 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 32],
                    [core::ffi::c_char; 32],
                >(*b"write_large_scale_tile_obu_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                3684 as core::ffi::c_int,
                b"tile_size_bytes >= 1 && tile_size_bytes <= 4\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        aom_wb_overwrite_literal(
            saved_wb,
            tile_size_bytes - 1 as core::ffi::c_int,
            2 as core::ffi::c_int,
        );
    }
}
unsafe extern "C-unwind" fn write_large_scale_tile_obu(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    lst_obu: *mut LargeTileFrameOBU,
    largest_tile_id: *mut core::ffi::c_int,
    mut total_size: *mut uint32_t,
    have_tiles: core::ffi::c_int,
    max_tile_size: *mut core::ffi::c_uint,
    max_tile_col_size: *mut core::ffi::c_uint,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let mut tile_buffers: [[TileBufferEnc; 64]; 64] = [[TileBufferEnc {
        data: 0 as *mut uint8_t,
        size: 0,
    }; 64]; 64];
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let mut tile_size: core::ffi::c_uint = 0 as core::ffi::c_uint;
    av1_reset_pack_bs_thread_data(&raw mut (*cpi).td);
    let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_col < tile_cols {
        let mut tile_info: TileInfo = TileInfo {
            mi_row_start: 0,
            mi_row_end: 0,
            mi_col_start: 0,
            mi_col_end: 0,
            tile_row: 0,
            tile_col: 0,
        };
        let is_last_col: core::ffi::c_int = (tile_col
            == tile_cols - 1 as core::ffi::c_int) as core::ffi::c_int;
        let col_offset: uint32_t = *total_size;
        av1_tile_set_col(&raw mut tile_info, cm, tile_col);
        if is_last_col == 0 {
            *total_size = (*total_size).wrapping_add(4 as uint32_t);
        }
        let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_row < tile_rows {
            let buf: *mut TileBufferEnc = &raw mut *(*tile_buffers
                .as_mut_ptr()
                .offset(tile_row as isize))
                .as_mut_ptr()
                .offset(tile_col as isize) as *mut TileBufferEnc;
            let data_offset: uint32_t = (if have_tiles != 0 {
                4 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint32_t;
            let tile_idx: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let mut this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_idx as isize) as *mut TileDataEnc;
            av1_tile_set_row(&raw mut tile_info, cm, tile_row);
            let mut mode_bc: aom_writer = aom_writer {
                pos: 0,
                buffer: 0 as *mut uint8_t,
                size: 0,
                ec: od_ec_enc {
                    buf: 0 as *mut core::ffi::c_uchar,
                    storage: 0,
                    offs: 0,
                    low: 0,
                    rng: 0,
                    cnt: 0,
                    error: 0,
                },
                allow_update_cdf: 0,
            };
            if *total_size as size_t > dst_size
                || (*lst_obu).tg_hdr_size as size_t
                    > dst_size.wrapping_sub(*total_size as size_t)
                || data_offset as size_t
                    >= dst_size
                        .wrapping_sub(*total_size as size_t)
                        .wrapping_sub((*lst_obu).tg_hdr_size as size_t)
            {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_ERROR,
                    b"Error writing modes\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            (*buf).data = dst
                .offset(*total_size as isize)
                .offset((*lst_obu).tg_hdr_size as isize);
            if *total_size > (UINT32_MAX as uint32_t).wrapping_sub(data_offset) {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_ERROR,
                    b"Error writing modes\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            *total_size = (*total_size).wrapping_add(data_offset);
            (*cpi).td.mb.e_mbd.tile_ctx = &raw mut (*this_tile).tctx;
            mode_bc.allow_update_cdf = ((*tiles).large_scale == 0) as core::ffi::c_int
                as uint8_t;
            mode_bc.allow_update_cdf = (mode_bc.allow_update_cdf as core::ffi::c_int != 0
                && !(*cm).features.disable_cdf_update) as core::ffi::c_int as uint8_t;
            aom_start_encode(
                &raw mut mode_bc,
                ((*buf).data).offset(data_offset as isize),
                dst_size
                    .wrapping_sub(*total_size as size_t)
                    .wrapping_sub((*lst_obu).tg_hdr_size as size_t)
                    .wrapping_sub(data_offset as size_t),
            );
            write_modes(
                cpi,
                &raw mut (*cpi).td,
                &raw mut tile_info,
                &raw mut mode_bc,
                tile_row,
                tile_col,
            );
            if aom_stop_encode(&raw mut mode_bc) < 0 as core::ffi::c_int {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_ERROR,
                    b"Error writing modes\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            tile_size = mode_bc.pos;
            (*buf).size = tile_size as size_t;
            if tile_size > *max_tile_size {
                *max_tile_size = tile_size;
                *largest_tile_id = tile_cols * tile_row + tile_col;
            }
            if have_tiles != 0 {
                let mut tile_header: uint32_t = (tile_size as uint32_t)
                    .wrapping_sub(AV1_MIN_TILE_SIZE_BYTES as uint32_t);
                let tile_copy_mode: core::ffi::c_int = if (if (*tiles).width
                    > (*tiles).height
                {
                    (*tiles).width
                } else {
                    (*tiles).height
                }) << MI_SIZE_LOG2 <= 256 as core::ffi::c_int
                {
                    1 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                };
                if (*cm).current_frame.frame_type as core::ffi::c_int
                    != KEY_FRAME as core::ffi::c_int && tile_copy_mode != 0
                {
                    let identical_tile_offset: core::ffi::c_int = find_identical_tile(
                        tile_row,
                        tile_col,
                        tile_buffers.as_mut_ptr() as *mut [TileBufferEnc; 64],
                    ) as core::ffi::c_int;
                    if identical_tile_offset > 0 as core::ffi::c_int {
                        tile_size = 0 as core::ffi::c_uint;
                        tile_header = (identical_tile_offset | 0x80 as core::ffi::c_int)
                            as uint32_t;
                        tile_header <<= 24 as core::ffi::c_int;
                    }
                }
                mem_put_le32_as_int(
                    (*buf).data as *mut core::ffi::c_void,
                    tile_header as core::ffi::c_int,
                );
            }
            *total_size = (*total_size as core::ffi::c_uint).wrapping_add(tile_size)
                as uint32_t as uint32_t;
            tile_row += 1;
        }
        if is_last_col == 0 {
            let mut col_size: uint32_t = (*total_size)
                .wrapping_sub(col_offset)
                .wrapping_sub(4 as uint32_t);
            mem_put_le32_as_int(
                dst.offset(col_offset as isize).offset((*lst_obu).tg_hdr_size as isize)
                    as *mut core::ffi::c_void,
                col_size as core::ffi::c_int,
            );
            *max_tile_col_size = (if *max_tile_col_size > col_size {
                *max_tile_col_size
            } else {
                col_size
            }) as core::ffi::c_uint;
        }
        tile_col += 1;
    }
    av1_accumulate_pack_bs_thread_data(cpi, &raw mut (*cpi).td);
}
#[inline]
unsafe extern "C-unwind" fn pack_large_scale_tiles_in_tg_obus(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut obu_extension_header: uint8_t,
    largest_tile_id: *mut core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let mut total_size: uint32_t = 0 as uint32_t;
    let mut max_tile_size: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut max_tile_col_size: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let have_tiles: core::ffi::c_int = ((*tiles).cols * (*tiles).rows
        > 1 as core::ffi::c_int) as core::ffi::c_int;
    let mut data: *mut uint8_t = dst;
    let mut lst_obu: LargeTileFrameOBU = LargeTileFrameOBU {
        tg_hdr_size: 0,
        frame_header_size: 0,
    };
    total_size = total_size
        .wrapping_add(
            init_large_scale_tile_obu_header(
                cpi,
                &raw mut data,
                saved_wb,
                obu_extension_header,
                &raw mut lst_obu,
            ),
        );
    write_large_scale_tile_obu(
        cpi,
        dst,
        dst_size,
        &raw mut lst_obu,
        largest_tile_id,
        &raw mut total_size,
        have_tiles,
        &raw mut max_tile_size,
        &raw mut max_tile_col_size,
    );
    write_large_scale_tile_obu_size(
        tiles,
        dst,
        dst_size,
        data,
        saved_wb,
        &raw mut lst_obu,
        have_tiles,
        &raw mut total_size,
        max_tile_size as core::ffi::c_int,
        max_tile_col_size as core::ffi::c_int,
    );
    return total_size;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_obu_tg_tile_headers(
    cpi: *mut AV1_COMP,
    xd: *mut MACROBLOCKD,
    pack_bs_params: *mut PackBSParams,
    tile_idx: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let curr_tg_hdr_size: *mut core::ffi::c_int = &raw mut (*pack_bs_params)
        .curr_tg_hdr_size;
    let tg_size: core::ffi::c_int = ((*tiles).rows * (*tiles).cols + (*cpi).num_tg
        - 1 as core::ffi::c_int) / (*cpi).num_tg;
    let obu_type: OBU_TYPE = (if (*cpi).num_tg == 1 as core::ffi::c_int {
        OBU_FRAME as core::ffi::c_int
    } else {
        OBU_TILE_GROUP as core::ffi::c_int
    }) as OBU_TYPE;
    *curr_tg_hdr_size = av1_write_obu_header(
        &raw mut (*(*cpi).ppi).level_params,
        &raw mut (*cpi).frame_header_count,
        obu_type,
        (*(*cm).seq_params).has_nonzero_operating_point_idc,
        true_0 != 0,
        (*pack_bs_params).obu_extn_header as core::ffi::c_int,
        (*pack_bs_params).tile_data_curr,
    ) as core::ffi::c_int;
    (*pack_bs_params).obu_header_size = *curr_tg_hdr_size as uint32_t;
    if (*cpi).num_tg == 1 as core::ffi::c_int {
        *curr_tg_hdr_size = (*curr_tg_hdr_size as uint32_t)
            .wrapping_add(
                write_frame_header_obu(
                    cpi,
                    xd,
                    (*pack_bs_params).saved_wb,
                    ((*pack_bs_params).tile_data_curr)
                        .offset(*curr_tg_hdr_size as isize),
                    0 as core::ffi::c_int,
                ),
            ) as core::ffi::c_int as core::ffi::c_int;
    }
    *curr_tg_hdr_size = (*curr_tg_hdr_size as uint32_t)
        .wrapping_add(
            write_tile_group_header(
                ((*pack_bs_params).tile_data_curr).offset(*curr_tg_hdr_size as isize),
                tile_idx,
                if (tile_idx + tg_size - 1 as core::ffi::c_int)
                    < (*tiles).cols * (*tiles).rows - 1 as core::ffi::c_int
                {
                    tile_idx + tg_size - 1 as core::ffi::c_int
                } else {
                    (*tiles).cols * (*tiles).rows - 1 as core::ffi::c_int
                },
                (*tiles).log2_rows + (*tiles).log2_cols,
                ((*cpi).num_tg > 1 as core::ffi::c_int) as core::ffi::c_int,
            ),
        ) as core::ffi::c_int as core::ffi::c_int;
    *(*pack_bs_params).total_size = (*(*pack_bs_params).total_size)
        .wrapping_add(*curr_tg_hdr_size as uint32_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_pack_tile_info(
    cpi: *mut AV1_COMP,
    td: *mut ThreadData,
    pack_bs_params: *mut PackBSParams,
) {
    let mut mode_bc: aom_writer = aom_writer {
        pos: 0,
        buffer: 0 as *mut uint8_t,
        size: 0,
        ec: od_ec_enc {
            buf: 0 as *mut core::ffi::c_uchar,
            storage: 0,
            offs: 0,
            low: 0,
            rng: 0,
            cnt: 0,
            error: 0,
        },
        allow_update_cdf: 0,
    };
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut tile_row: core::ffi::c_int = (*pack_bs_params).tile_row;
    let mut tile_col: core::ffi::c_int = (*pack_bs_params).tile_col;
    let total_size: *mut uint32_t = (*pack_bs_params).total_size;
    let mut tile_info: TileInfo = TileInfo {
        mi_row_start: 0,
        mi_row_end: 0,
        mi_col_start: 0,
        mi_col_end: 0,
        tile_row: 0,
        tile_col: 0,
    };
    av1_tile_set_col(&raw mut tile_info, cm, tile_col);
    av1_tile_set_row(&raw mut tile_info, cm, tile_row);
    mode_bc.allow_update_cdf = 1 as uint8_t;
    mode_bc.allow_update_cdf = (mode_bc.allow_update_cdf as core::ffi::c_int != 0
        && !(*cm).features.disable_cdf_update) as core::ffi::c_int as uint8_t;
    let mut tile_size: core::ffi::c_uint = 0;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    av1_reset_loop_restoration(&raw mut (*td).mb.e_mbd, num_planes);
    (*pack_bs_params).buf.data = ((*pack_bs_params).dst).offset(*total_size as isize);
    if (*pack_bs_params).is_last_tile_in_tg == 0 {
        *total_size = (*total_size).wrapping_add(4 as uint32_t);
    }
    if (*pack_bs_params).tile_buf_size <= *total_size as size_t {
        aom_internal_error(
            (*td).mb.e_mbd.error_info as *mut aom_internal_error_info,
            AOM_CODEC_ERROR,
            b"Error writing modes\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    aom_start_encode(
        &raw mut mode_bc,
        ((*pack_bs_params).dst).offset(*total_size as isize),
        ((*pack_bs_params).tile_buf_size).wrapping_sub(*total_size as size_t),
    );
    write_modes(cpi, td, &raw mut tile_info, &raw mut mode_bc, tile_row, tile_col);
    if aom_stop_encode(&raw mut mode_bc) < 0 as core::ffi::c_int {
        aom_internal_error(
            (*td).mb.e_mbd.error_info as *mut aom_internal_error_info,
            AOM_CODEC_ERROR,
            b"Error writing modes\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    tile_size = mode_bc.pos;
    if !(tile_size >= 1 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_pack_tile_info\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            3896 as core::ffi::c_int,
            b"tile_size >= AV1_MIN_TILE_SIZE_BYTES\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    (*pack_bs_params).buf.size = tile_size as size_t;
    if (*pack_bs_params).is_last_tile_in_tg == 0 {
        mem_put_le32_as_int(
            (*pack_bs_params).buf.data as *mut core::ffi::c_void,
            tile_size.wrapping_sub(AV1_MIN_TILE_SIZE_BYTES as core::ffi::c_uint)
                as core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_write_last_tile_info(
    cpi: *mut AV1_COMP,
    mut fh_info: *const FrameHeaderInfo,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut curr_tg_data_size: *mut size_t,
    mut curr_tg_start: *mut uint8_t,
    total_size: *mut uint32_t,
    mut tile_data_start: *mut *mut uint8_t,
    largest_tile_id: *mut core::ffi::c_int,
    is_first_tg: *mut core::ffi::c_int,
    mut obu_header_size: uint32_t,
    mut obu_extn_header: uint8_t,
) {
    let obu_payload_size: size_t = (*curr_tg_data_size)
        .wrapping_sub(obu_header_size as size_t);
    let length_field_size: size_t = obu_memmove_unsafe(
        obu_header_size as size_t,
        obu_payload_size,
        curr_tg_start,
    ) as size_t;
    if av1_write_uleb_obu_size_unsafe(
        obu_payload_size,
        curr_tg_start.offset(obu_header_size as isize),
    ) != AOM_CODEC_OK as core::ffi::c_int
    {
        aom_internal_error(
            (*cpi).common.error,
            AOM_CODEC_ERROR,
            b"av1_write_last_tile_info: output buffer full\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    *curr_tg_data_size = (*curr_tg_data_size).wrapping_add(length_field_size);
    *total_size = (*total_size).wrapping_add(length_field_size as uint32_t);
    *tile_data_start = (*tile_data_start).offset(length_field_size as isize);
    if (*cpi).num_tg == 1 as core::ffi::c_int {
        (*saved_wb).bit_buffer = ((*saved_wb).bit_buffer)
            .offset(length_field_size as isize);
    }
    if *is_first_tg == 0
        && (*cpi).common.features.error_resilient_mode as core::ffi::c_int != 0
    {
        memmove(
            curr_tg_start.offset((*fh_info).total_length as isize)
                as *mut core::ffi::c_void,
            curr_tg_start as *const core::ffi::c_void,
            *curr_tg_data_size,
        );
        memcpy(
            curr_tg_start as *mut core::ffi::c_void,
            (*fh_info).frame_header as *const core::ffi::c_void,
            (*fh_info).total_length,
        );
        *largest_tile_id = 0 as core::ffi::c_int;
        av1_write_obu_header(
            &raw mut (*(*cpi).ppi).level_params,
            &raw mut (*cpi).frame_header_count,
            OBU_REDUNDANT_FRAME_HEADER,
            (*(*cpi).common.seq_params).has_nonzero_operating_point_idc,
            true_0 != 0,
            obu_extn_header as core::ffi::c_int,
            &raw mut *curr_tg_start.offset((*fh_info).obu_header_byte_offset as isize),
        );
        *curr_tg_data_size = (*curr_tg_data_size).wrapping_add((*fh_info).total_length);
        *total_size = (*total_size).wrapping_add((*fh_info).total_length as uint32_t);
    }
    *is_first_tg = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_pack_bs_thread_data(td: *mut ThreadData) {
    (*td).coefficient_size = 0 as core::ffi::c_int;
    (*td).max_mv_magnitude = 0 as core::ffi::c_int;
    memset(
        &raw mut (*td).interp_filter_selected as *mut [core::ffi::c_int; 4]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 4]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_accumulate_pack_bs_thread_data(
    cpi: *mut AV1_COMP,
    mut td: *const ThreadData,
) {
    let mut do_max_mv_magnitude_update: core::ffi::c_int = 1 as core::ffi::c_int;
    (*cpi).rc.coefficient_size += (*td).coefficient_size;
    if !(*cpi).do_frame_data_update {
        do_max_mv_magnitude_update = 0 as core::ffi::c_int;
    }
    if (*cpi).sf.mv_sf.auto_mv_step_size != 0 && do_max_mv_magnitude_update != 0 {
        (*cpi).mv_search_params.max_mv_magnitude = if (*cpi)
            .mv_search_params
            .max_mv_magnitude > (*td).max_mv_magnitude
        {
            (*cpi).mv_search_params.max_mv_magnitude
        } else {
            (*td).max_mv_magnitude
        };
    }
    let mut filter: InterpFilter = EIGHTTAP_REGULAR;
    while (filter as core::ffi::c_int) < SWITCHABLE as core::ffi::c_int {
        (*(*cpi).common.cur_frame).interp_filter_selected[filter as usize]
            += (*td).interp_filter_selected[filter as usize];
        filter += 1;
    }
}
unsafe extern "C-unwind" fn write_tile_obu(
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
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let xd: *mut MACROBLOCKD = &raw mut (*cpi).td.mb.e_mbd;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let num_tg_hdrs: core::ffi::c_int = (*cpi).num_tg;
    let tg_size: core::ffi::c_int = (tile_rows * tile_cols + num_tg_hdrs
        - 1 as core::ffi::c_int) / num_tg_hdrs;
    let mut tile_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut curr_tg_data_size: size_t = 0 as size_t;
    let mut tile_data_curr: *mut uint8_t = dst;
    let mut new_tg: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut is_first_tg: core::ffi::c_int = 1 as core::ffi::c_int;
    av1_reset_pack_bs_thread_data(&raw mut (*cpi).td);
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_row < tile_rows {
        let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while tile_col < tile_cols {
            let tile_idx: core::ffi::c_int = tile_row * tile_cols + tile_col;
            let mut this_tile: *mut TileDataEnc = &raw mut *((*cpi).tile_data)
                .offset(tile_idx as isize) as *mut TileDataEnc;
            let mut is_last_tile_in_tg: core::ffi::c_int = 0 as core::ffi::c_int;
            if new_tg != 0 {
                tile_data_curr = dst.offset(*total_size as isize);
                tile_count = 0 as core::ffi::c_int;
            }
            tile_count += 1;
            if tile_count == tg_size
                || tile_idx == tile_cols * tile_rows - 1 as core::ffi::c_int
            {
                is_last_tile_in_tg = 1 as core::ffi::c_int;
            }
            (*xd).tile_ctx = &raw mut (*this_tile).tctx;
            let mut pack_bs_params: PackBSParams = PackBSParams {
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
            };
            pack_bs_params.dst = dst;
            pack_bs_params.tile_buf_size = dst_size;
            pack_bs_params.curr_tg_hdr_size = 0 as core::ffi::c_int;
            pack_bs_params.is_last_tile_in_tg = is_last_tile_in_tg;
            pack_bs_params.new_tg = new_tg;
            pack_bs_params.obu_extn_header = obu_extn_header;
            pack_bs_params.obu_header_size = 0 as uint32_t;
            pack_bs_params.saved_wb = saved_wb;
            pack_bs_params.tile_col = tile_col;
            pack_bs_params.tile_row = tile_row;
            pack_bs_params.tile_data_curr = tile_data_curr;
            pack_bs_params.total_size = total_size;
            if new_tg != 0 {
                av1_write_obu_tg_tile_headers(cpi, xd, &raw mut pack_bs_params, tile_idx);
            }
            av1_pack_tile_info(cpi, &raw mut (*cpi).td, &raw mut pack_bs_params);
            if new_tg != 0 {
                curr_tg_data_size = pack_bs_params.curr_tg_hdr_size as size_t;
                *tile_data_start = (*tile_data_start)
                    .offset(pack_bs_params.curr_tg_hdr_size as isize);
                *obu_header_size = pack_bs_params.obu_header_size;
                new_tg = 0 as core::ffi::c_int;
            }
            if is_last_tile_in_tg != 0 {
                new_tg = 1 as core::ffi::c_int;
            }
            curr_tg_data_size = curr_tg_data_size
                .wrapping_add(
                    (pack_bs_params.buf.size)
                        .wrapping_add(
                            (if is_last_tile_in_tg != 0 {
                                0 as core::ffi::c_int
                            } else {
                                4 as core::ffi::c_int
                            }) as size_t,
                        ),
                );
            if pack_bs_params.buf.size > *max_tile_size as size_t {
                *largest_tile_id = tile_idx;
                *max_tile_size = pack_bs_params.buf.size as core::ffi::c_uint;
            }
            if is_last_tile_in_tg != 0 {
                av1_write_last_tile_info(
                    cpi,
                    fh_info,
                    saved_wb,
                    &raw mut curr_tg_data_size,
                    tile_data_curr,
                    total_size,
                    tile_data_start,
                    largest_tile_id,
                    &raw mut is_first_tg,
                    *obu_header_size,
                    obu_extn_header,
                );
            }
            *total_size = (*total_size)
                .wrapping_add(pack_bs_params.buf.size as uint32_t);
            tile_col += 1;
        }
        tile_row += 1;
    }
    av1_accumulate_pack_bs_thread_data(cpi, &raw mut (*cpi).td);
}
unsafe extern "C-unwind" fn write_tile_obu_size(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut largest_tile_id: core::ffi::c_int,
    total_size: *mut uint32_t,
    mut max_tile_size: core::ffi::c_uint,
    mut obu_header_size: uint32_t,
    mut tile_data_start: *mut uint8_t,
) {
    let tiles: *const CommonTileParams = &raw mut (*cpi).common.tiles;
    aom_wb_overwrite_literal(
        saved_wb,
        largest_tile_id,
        (*tiles).log2_cols + (*tiles).log2_rows,
    );
    if (*cpi).num_tg != 1 as core::ffi::c_int {
        return;
    }
    let mut tile_size_bytes: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut unused: core::ffi::c_int = 0;
    let tile_data_offset: uint32_t = tile_data_start.offset_from(dst)
        as core::ffi::c_long as uint32_t;
    let tile_data_size: uint32_t = (*total_size).wrapping_sub(tile_data_offset);
    *total_size = remux_tiles(
        tiles,
        tile_data_start,
        tile_data_size,
        max_tile_size as uint32_t,
        0 as uint32_t,
        &raw mut tile_size_bytes,
        &raw mut unused,
    ) as uint32_t;
    *total_size = (*total_size).wrapping_add(tile_data_offset);
    if !(tile_size_bytes >= 1 as core::ffi::c_int
        && tile_size_bytes <= 4 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"write_tile_obu_size\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            4098 as core::ffi::c_int,
            b"tile_size_bytes >= 1 && tile_size_bytes <= 4\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    aom_wb_overwrite_literal(
        saved_wb,
        tile_size_bytes - 1 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    let mut payload_size: uint64_t = 0;
    let mut length_field_size: size_t = 0;
    let mut res: core::ffi::c_int = aom_uleb_decode(
        dst.offset(obu_header_size as isize),
        (*total_size).wrapping_sub(obu_header_size) as size_t,
        &raw mut payload_size,
        &raw mut length_field_size,
    );
    if !(res == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"write_tile_obu_size\0"))
                .as_ptr(),
            b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
            4108 as core::ffi::c_int,
            b"res == 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let new_payload_size: uint64_t = ((*total_size).wrapping_sub(obu_header_size)
        as size_t)
        .wrapping_sub(length_field_size) as uint64_t;
    if new_payload_size != payload_size {
        let mut new_length_field_size: size_t = 0;
        res = aom_uleb_encode(
            new_payload_size,
            length_field_size,
            dst.offset(obu_header_size as isize),
            &raw mut new_length_field_size,
        );
        if !(res == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"write_tile_obu_size\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                4117 as core::ffi::c_int,
                b"res == 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if new_length_field_size < length_field_size {
            let src_offset: size_t = (obu_header_size as size_t)
                .wrapping_add(length_field_size);
            let dst_offset: size_t = (obu_header_size as size_t)
                .wrapping_add(new_length_field_size);
            memmove(
                dst.offset(dst_offset as isize) as *mut core::ffi::c_void,
                dst.offset(src_offset as isize) as *const core::ffi::c_void,
                payload_size as size_t,
            );
            *total_size = (*total_size)
                .wrapping_sub(
                    length_field_size.wrapping_sub(new_length_field_size)
                        as core::ffi::c_int as uint32_t,
                );
        }
    }
}
unsafe extern "C-unwind" fn calc_pack_bs_mt_workers(
    mut tile_data: *const TileDataEnc,
    mut num_tiles: core::ffi::c_int,
    mut avail_workers: core::ffi::c_int,
    mut pack_bs_mt_enabled: bool,
) -> core::ffi::c_int {
    if !pack_bs_mt_enabled {
        return 1 as core::ffi::c_int;
    }
    let mut frame_abs_sum_level: uint64_t = 0 as uint64_t;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < num_tiles {
        frame_abs_sum_level = frame_abs_sum_level
            .wrapping_add((*tile_data.offset(idx as isize)).abs_sum_level);
        idx += 1;
    }
    let mut ideal_num_workers: core::ffi::c_int = 1 as core::ffi::c_int;
    let job_disp_time_const: core::ffi::c_float = num_tiles as core::ffi::c_float
        * JOB_DISP_TIME_OH_CONST as core::ffi::c_float;
    let mut max_sum: core::ffi::c_float = 0.0f32;
    let mut num_workers: core::ffi::c_int = avail_workers;
    while num_workers > 1 as core::ffi::c_int {
        let fas_per_worker_const: core::ffi::c_float = (num_workers
            - 1 as core::ffi::c_int) as core::ffi::c_float
            / num_workers as core::ffi::c_float
            * frame_abs_sum_level as core::ffi::c_float;
        let setup_time_const: core::ffi::c_float = num_workers as core::ffi::c_float
            * SETUP_TIME_OH_CONST as core::ffi::c_float;
        let this_sum: core::ffi::c_float = fas_per_worker_const - setup_time_const
            - job_disp_time_const / num_workers as core::ffi::c_float;
        if this_sum > max_sum {
            max_sum = this_sum;
            ideal_num_workers = num_workers;
        }
        num_workers -= 1;
    }
    return ideal_num_workers;
}
#[inline]
unsafe extern "C-unwind" fn pack_tiles_in_tg_obus(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut obu_extension_header: uint8_t,
    mut fh_info: *const FrameHeaderInfo,
    largest_tile_id: *mut core::ffi::c_int,
) -> uint32_t {
    let tiles: *const CommonTileParams = &raw mut (*cpi).common.tiles;
    let mut total_size: uint32_t = 0 as uint32_t;
    let mut max_tile_size: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut obu_header_size: uint32_t = 0 as uint32_t;
    let mut tile_data_start: *mut uint8_t = dst;
    let tile_cols: core::ffi::c_int = (*tiles).cols;
    let tile_rows: core::ffi::c_int = (*tiles).rows;
    let num_tiles: core::ffi::c_int = tile_rows * tile_cols;
    let num_workers: core::ffi::c_int = calc_pack_bs_mt_workers(
        (*cpi).tile_data,
        num_tiles,
        (*cpi).mt_info.num_mod_workers[MOD_PACK_BS as core::ffi::c_int as usize],
        (*cpi).mt_info.pack_bs_mt_enabled,
    ) as core::ffi::c_int;
    if num_workers > 1 as core::ffi::c_int {
        av1_write_tile_obu_mt(
            cpi,
            dst,
            dst_size,
            &raw mut total_size,
            saved_wb,
            obu_extension_header,
            fh_info,
            largest_tile_id,
            &raw mut max_tile_size,
            &raw mut obu_header_size,
            &raw mut tile_data_start,
            num_workers,
        );
    } else {
        write_tile_obu(
            cpi,
            dst,
            dst_size,
            &raw mut total_size,
            saved_wb,
            obu_extension_header,
            fh_info,
            largest_tile_id,
            &raw mut max_tile_size,
            &raw mut obu_header_size,
            &raw mut tile_data_start,
        );
    }
    if num_tiles > 1 as core::ffi::c_int {
        write_tile_obu_size(
            cpi,
            dst,
            dst_size,
            saved_wb,
            *largest_tile_id,
            &raw mut total_size,
            max_tile_size,
            obu_header_size,
            tile_data_start,
        );
    }
    return total_size;
}
unsafe extern "C-unwind" fn write_tiles_in_tg_obus(
    cpi: *mut AV1_COMP,
    dst: *mut uint8_t,
    mut dst_size: size_t,
    mut saved_wb: *mut aom_write_bit_buffer,
    mut obu_extension_header: uint8_t,
    mut fh_info: *const FrameHeaderInfo,
    largest_tile_id: *mut core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let tiles: *const CommonTileParams = &raw mut (*cm).tiles;
    *largest_tile_id = 0 as core::ffi::c_int;
    if (*cm).seg.enabled as core::ffi::c_int != 0
        && (*cm).seg.update_map as core::ffi::c_int != 0
    {
        if (*cm).features.primary_ref_frame == PRIMARY_REF_NONE {
            (*cm).seg.temporal_update = 0 as uint8_t;
        } else {
            (*cm).seg.temporal_update = 1 as uint8_t;
            if (*cpi).td.rd_counts.seg_tmp_pred_cost[0 as core::ffi::c_int as usize]
                < (*cpi).td.rd_counts.seg_tmp_pred_cost[1 as core::ffi::c_int as usize]
            {
                (*cm).seg.temporal_update = 0 as uint8_t;
            }
        }
    }
    if (*tiles).large_scale != 0 {
        return pack_large_scale_tiles_in_tg_obus(
            cpi,
            dst,
            dst_size,
            saved_wb,
            obu_extension_header,
            largest_tile_id,
        );
    }
    return pack_tiles_in_tg_obus(
        cpi,
        dst,
        dst_size,
        saved_wb,
        obu_extension_header,
        fh_info,
        largest_tile_id,
    );
}
unsafe extern "C-unwind" fn av1_write_metadata_obu(
    mut metadata: *const aom_metadata_t,
    dst: *mut uint8_t,
    mut dst_size: size_t,
) -> size_t {
    let mut coded_metadata_size: size_t = 0 as size_t;
    let metadata_type: uint64_t = (*metadata).type_0 as uint64_t;
    if aom_uleb_encode(metadata_type, dst_size, dst, &raw mut coded_metadata_size)
        != 0 as core::ffi::c_int
    {
        return 0 as size_t;
    }
    if coded_metadata_size.wrapping_add((*metadata).sz).wrapping_add(1 as size_t)
        > dst_size
    {
        return 0 as size_t;
    }
    memcpy(
        dst.offset(coded_metadata_size as isize) as *mut core::ffi::c_void,
        (*metadata).payload as *const core::ffi::c_void,
        (*metadata).sz,
    );
    *dst.offset(coded_metadata_size.wrapping_add((*metadata).sz) as isize) = 0x80
        as uint8_t;
    return coded_metadata_size.wrapping_add((*metadata).sz).wrapping_add(1 as size_t);
}
unsafe extern "C-unwind" fn av1_write_metadata_array(
    cpi: *mut AV1_COMP,
    mut dst: *mut uint8_t,
    mut dst_size: size_t,
) -> size_t {
    if ((*cpi).source).is_null() {
        return 0 as size_t;
    }
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut arr: *mut aom_metadata_array_t = (*(*cpi).source).metadata;
    if arr.is_null() {
        return 0 as size_t;
    }
    let mut obu_header_size: size_t = 0 as size_t;
    let mut obu_payload_size: size_t = 0 as size_t;
    let mut total_bytes_written: size_t = 0 as size_t;
    let mut length_field_size: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < (*arr).sz {
        let mut current_metadata: *mut aom_metadata_t = *((*arr).metadata_array)
            .offset(i as isize);
        if !current_metadata.is_null() && !((*current_metadata).payload).is_null() {
            let metadata_insert_location: core::ffi::c_int = ((*current_metadata)
                .insert_flag as core::ffi::c_uint
                & AOM_MIF_INSERT_LOCATION_MASK as core::ffi::c_uint) as core::ffi::c_int;
            if (*cm).current_frame.frame_type as core::ffi::c_int
                == KEY_FRAME as core::ffi::c_int
                && metadata_insert_location == AOM_MIF_KEY_FRAME as core::ffi::c_int
                || (*cm).current_frame.frame_type as core::ffi::c_int
                    != KEY_FRAME as core::ffi::c_int
                    && metadata_insert_location
                        == AOM_MIF_NON_KEY_FRAME as core::ffi::c_int
                || metadata_insert_location == AOM_MIF_ANY_FRAME as core::ffi::c_int
            {
                if dst_size < 2 as size_t {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_ERROR,
                        b"av1_write_metadata_array: output buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                let is_layer_specific_obu: bool = (*current_metadata).insert_flag
                    as core::ffi::c_uint & AOM_MIF_LAYER_SPECIFIC as core::ffi::c_uint
                    != 0 as core::ffi::c_uint;
                obu_header_size = av1_write_obu_header(
                    &raw mut (*(*cpi).ppi).level_params,
                    &raw mut (*cpi).frame_header_count,
                    OBU_METADATA,
                    (*(*cm).seq_params).has_nonzero_operating_point_idc,
                    is_layer_specific_obu,
                    0 as core::ffi::c_int,
                    dst,
                ) as size_t;
                if !(obu_header_size <= 2 as size_t) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 25],
                            [core::ffi::c_char; 25],
                        >(*b"av1_write_metadata_array\0"))
                            .as_ptr(),
                        b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                        4275 as core::ffi::c_int,
                        b"obu_header_size <= 2\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                obu_payload_size = av1_write_metadata_obu(
                    current_metadata,
                    dst.offset(obu_header_size as isize),
                    dst_size.wrapping_sub(obu_header_size),
                );
                if obu_payload_size == 0 as size_t {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_ERROR,
                        b"av1_write_metadata_array: output buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                length_field_size = obu_memmove(
                    obu_header_size,
                    obu_payload_size,
                    dst,
                    dst_size,
                );
                if length_field_size == 0 as size_t {
                    aom_internal_error(
                        (*cm).error,
                        AOM_CODEC_ERROR,
                        b"av1_write_metadata_array: output buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if av1_write_uleb_obu_size(
                    obu_payload_size,
                    dst.offset(obu_header_size as isize),
                    length_field_size,
                ) == AOM_CODEC_OK as core::ffi::c_int
                {
                    let obu_size: size_t = obu_header_size
                        .wrapping_add(length_field_size)
                        .wrapping_add(obu_payload_size);
                    dst = dst.offset(obu_size as isize);
                    dst_size = dst_size.wrapping_sub(obu_size);
                    total_bytes_written = total_bytes_written.wrapping_add(obu_size);
                } else {
                    aom_internal_error(
                        (*cpi).common.error,
                        AOM_CODEC_ERROR,
                        b"av1_write_metadata_array: output buffer full\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return total_bytes_written;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_pack_bitstream(
    cpi: *mut AV1_COMP,
    mut dst: *mut uint8_t,
    mut dst_size: size_t,
    mut size: *mut size_t,
    largest_tile_id: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut data: *mut uint8_t = dst;
    let mut data_size: size_t = dst_size;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let level_params: *mut AV1LevelParams = &raw mut (*(*cpi).ppi).level_params;
    let mut obu_header_size: uint32_t = 0 as uint32_t;
    let mut obu_payload_size: uint32_t = 0 as uint32_t;
    let mut fh_info: FrameHeaderInfo = {
        let mut init = FrameHeaderInfo {
            frame_header: 0 as *mut uint8_t,
            obu_header_byte_offset: 0 as size_t,
            total_length: 0 as size_t,
        };
        init
    };
    let obu_extension_header: uint8_t = ((*cm).temporal_layer_id << 5 as core::ffi::c_int
        | (*cm).spatial_layer_id << 3 as core::ffi::c_int | 0 as core::ffi::c_int)
        as uint8_t;
    if (*cm).delta_q_info.delta_q_present_flag != 0
        && (*cpi).deltaq_used == 0 as core::ffi::c_int
    {
        (*cm).delta_q_info.delta_q_present_flag = 0 as core::ffi::c_int;
    }
    (*cpi).frame_header_count = 0 as core::ffi::c_int;
    if (*cm).current_frame.frame_type as core::ffi::c_int
        == INTRA_ONLY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == KEY_FRAME as core::ffi::c_int
    {
        if data_size < 2 as size_t {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        obu_header_size = av1_write_obu_header(
            level_params,
            &raw mut (*cpi).frame_header_count,
            OBU_SEQUENCE_HEADER,
            (*(*cm).seq_params).has_nonzero_operating_point_idc,
            false_0 != 0,
            0 as core::ffi::c_int,
            data,
        );
        if !(obu_header_size <= 2 as uint32_t) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"av1_pack_bitstream\0"))
                    .as_ptr(),
                b"bitstream.c\0" as *const u8 as *const core::ffi::c_char,
                4343 as core::ffi::c_int,
                b"obu_header_size <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        obu_payload_size = av1_write_sequence_header_obu(
            (*cm).seq_params,
            data.offset(obu_header_size as isize),
            data_size.wrapping_sub(obu_header_size as size_t),
        );
        let length_field_size: size_t = obu_memmove(
            obu_header_size as size_t,
            obu_payload_size as size_t,
            data,
            data_size,
        ) as size_t;
        if length_field_size == 0 as size_t {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        if av1_write_uleb_obu_size(
            obu_payload_size as size_t,
            data.offset(obu_header_size as isize),
            length_field_size,
        ) != AOM_CODEC_OK as core::ffi::c_int
        {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        let bytes_written: size_t = (obu_header_size as size_t)
            .wrapping_add(length_field_size)
            .wrapping_add(obu_payload_size as size_t);
        data = data.offset(bytes_written as isize);
        data_size = data_size.wrapping_sub(bytes_written);
    }
    if (*cm).show_frame != 0 {
        let bytes_written_0: size_t = av1_write_metadata_array(cpi, data, data_size)
            as size_t;
        data = data.offset(bytes_written_0 as isize);
        data_size = data_size.wrapping_sub(bytes_written_0);
    }
    let write_frame_header: core::ffi::c_int = ((*cpi).num_tg > 1 as core::ffi::c_int
        || encode_show_existing_frame(cm) != 0) as core::ffi::c_int;
    let mut saved_wb: aom_write_bit_buffer = {
        let mut init = aom_write_bit_buffer {
            bit_buffer: 0 as *mut uint8_t,
            bit_offset: 0 as uint32_t,
        };
        init
    };
    let mut length_field: size_t = 0 as size_t;
    if write_frame_header != 0 {
        fh_info.frame_header = data;
        if data_size < 2 as size_t {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        obu_header_size = av1_write_obu_header(
            level_params,
            &raw mut (*cpi).frame_header_count,
            OBU_FRAME_HEADER,
            (*(*cm).seq_params).has_nonzero_operating_point_idc,
            true_0 != 0,
            obu_extension_header as core::ffi::c_int,
            data,
        );
        obu_payload_size = write_frame_header_obu(
            cpi,
            &raw mut (*cpi).td.mb.e_mbd,
            &raw mut saved_wb,
            data.offset(obu_header_size as isize),
            1 as core::ffi::c_int,
        );
        length_field = obu_memmove(
            obu_header_size as size_t,
            obu_payload_size as size_t,
            data,
            data_size,
        );
        if length_field == 0 as size_t {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        if av1_write_uleb_obu_size(
            obu_payload_size as size_t,
            data.offset(obu_header_size as isize),
            length_field,
        ) != AOM_CODEC_OK as core::ffi::c_int
        {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        fh_info.obu_header_byte_offset = 0 as size_t;
        fh_info.total_length = (obu_header_size as size_t)
            .wrapping_add(length_field)
            .wrapping_add(obu_payload_size as size_t);
        if fh_info.total_length > UINT32_MAX as size_t {
            return AOM_CODEC_ERROR as core::ffi::c_int;
        }
        data = data.offset(fh_info.total_length as isize);
        data_size = data_size.wrapping_sub(fh_info.total_length);
    }
    if encode_show_existing_frame(cm) == 0 {
        if !(saved_wb.bit_buffer).is_null() {
            saved_wb.bit_buffer = (saved_wb.bit_buffer).offset(length_field as isize);
        }
        let bytes_written_1: size_t = write_tiles_in_tg_obus(
            cpi,
            data,
            data_size,
            &raw mut saved_wb,
            obu_extension_header,
            &raw mut fh_info,
            largest_tile_id,
        ) as size_t;
        data = data.offset(bytes_written_1 as isize);
        data_size = data_size.wrapping_sub(bytes_written_1);
    }
    *size = data.offset_from(dst) as core::ffi::c_long as size_t;
    return AOM_CODEC_OK as core::ffi::c_int;
}
pub const CHAR_BIT: core::ffi::c_int = __CHAR_BIT__;

extern "C" {
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
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
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
}
