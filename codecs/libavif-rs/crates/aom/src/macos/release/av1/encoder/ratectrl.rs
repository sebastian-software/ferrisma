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

// Mechanically translated from upstream/aom/av1/encoder/ratectrl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type __sFILEX;
    pub type SB_FIRST_PASS_STATS;
    pub type AVxWorkerImpl;
    pub type aom_denoise_and_model_t;
    pub type EncWorkerData;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_get_MBs(
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_ac_quant_QTX(
        qindex: core::ffi::c_int,
        delta: core::ffi::c_int,
        bit_depth: aom_bit_depth_t,
    ) -> int16_t;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_cyclic_refresh_estimate_bits_at_q(
        cpi: *const AV1_COMP,
        correction_factor: core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn av1_cyclic_refresh_rc_bits_per_mb(
        cpi: *const AV1_COMP,
        i: core::ffi::c_int,
        correction_factor: core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn av1_vector_match(
        ref_0: *const int16_t,
        src: *const int16_t,
        bwl: core::ffi::c_int,
        search_size_top: core::ffi::c_int,
        search_size_bottom: core::ffi::c_int,
        full_search: core::ffi::c_int,
        sad: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_update_temporal_layer_framerate(cpi: *mut AV1_COMP);
    fn av1_check_ref_is_low_spatial_res_super_frame(
        cpi: *mut AV1_COMP,
        ref_frame: core::ffi::c_int,
    ) -> bool;
    fn av1_restore_layer_context(cpi: *mut AV1_COMP);
    fn av1_svc_reset_temporal_layers(cpi: *mut AV1_COMP, is_key: core::ffi::c_int);
    fn av1_svc_get_min_ref_dist(cpi: *const AV1_COMP) -> core::ffi::c_int;
    fn aom_avg_4x4_c(_: *const uint8_t, p: core::ffi::c_int) -> core::ffi::c_uint;
    fn aom_int_pro_col_c(
        vbuf: *mut int16_t,
        ref_0: *const uint8_t,
        ref_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        norm_factor: core::ffi::c_int,
    );
    fn aom_int_pro_row_c(
        hbuf: *mut int16_t,
        ref_0: *const uint8_t,
        ref_stride: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        norm_factor: core::ffi::c_int,
    );
}
pub type float_t = core::ffi::c_float;
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
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
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
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub type pthread_cond_t = libc::pthread_cond_t;
pub type pthread_mutex_t = libc::pthread_mutex_t;
pub type pthread_once_t = libc::pthread_once_t;
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
pub type TX_SIZE = uint8_t;
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
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const AOM_REFFRAME_ALL: C2RustUnnamed_1 = 127;
pub const AOM_ALT_FLAG: C2RustUnnamed_1 = 64;
pub const AOM_ALT2_FLAG: C2RustUnnamed_1 = 32;
pub const AOM_BWD_FLAG: C2RustUnnamed_1 = 16;
pub const AOM_GOLD_FLAG: C2RustUnnamed_1 = 8;
pub const AOM_LAST3_FLAG: C2RustUnnamed_1 = 4;
pub const AOM_LAST2_FLAG: C2RustUnnamed_1 = 2;
pub const AOM_LAST_FLAG: C2RustUnnamed_1 = 1;
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const SEQ_LEVEL_KEEP_STATS: C2RustUnnamed_2 = 32;
pub const SEQ_LEVEL_MAX: C2RustUnnamed_2 = 31;
pub const SEQ_LEVELS: C2RustUnnamed_2 = 28;
pub const SEQ_LEVEL_8_3: C2RustUnnamed_2 = 27;
pub const SEQ_LEVEL_8_2: C2RustUnnamed_2 = 26;
pub const SEQ_LEVEL_8_1: C2RustUnnamed_2 = 25;
pub const SEQ_LEVEL_8_0: C2RustUnnamed_2 = 24;
pub const SEQ_LEVEL_7_3: C2RustUnnamed_2 = 23;
pub const SEQ_LEVEL_7_2: C2RustUnnamed_2 = 22;
pub const SEQ_LEVEL_7_1: C2RustUnnamed_2 = 21;
pub const SEQ_LEVEL_7_0: C2RustUnnamed_2 = 20;
pub const SEQ_LEVEL_6_3: C2RustUnnamed_2 = 19;
pub const SEQ_LEVEL_6_2: C2RustUnnamed_2 = 18;
pub const SEQ_LEVEL_6_1: C2RustUnnamed_2 = 17;
pub const SEQ_LEVEL_6_0: C2RustUnnamed_2 = 16;
pub const SEQ_LEVEL_5_3: C2RustUnnamed_2 = 15;
pub const SEQ_LEVEL_5_2: C2RustUnnamed_2 = 14;
pub const SEQ_LEVEL_5_1: C2RustUnnamed_2 = 13;
pub const SEQ_LEVEL_5_0: C2RustUnnamed_2 = 12;
pub const SEQ_LEVEL_4_3: C2RustUnnamed_2 = 11;
pub const SEQ_LEVEL_4_2: C2RustUnnamed_2 = 10;
pub const SEQ_LEVEL_4_1: C2RustUnnamed_2 = 9;
pub const SEQ_LEVEL_4_0: C2RustUnnamed_2 = 8;
pub const SEQ_LEVEL_3_3: C2RustUnnamed_2 = 7;
pub const SEQ_LEVEL_3_2: C2RustUnnamed_2 = 6;
pub const SEQ_LEVEL_3_1: C2RustUnnamed_2 = 5;
pub const SEQ_LEVEL_3_0: C2RustUnnamed_2 = 4;
pub const SEQ_LEVEL_2_3: C2RustUnnamed_2 = 3;
pub const SEQ_LEVEL_2_2: C2RustUnnamed_2 = 2;
pub const SEQ_LEVEL_2_1: C2RustUnnamed_2 = 1;
pub const SEQ_LEVEL_2_0: C2RustUnnamed_2 = 0;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_3 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_3 = 7;
pub const BWD_REFS: C2RustUnnamed_3 = 3;
pub const FWD_REFS: C2RustUnnamed_3 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_3 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_3 = 8;
pub const REF_FRAMES: C2RustUnnamed_3 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_3 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_3 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_3 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_3 = 4;
pub const LAST3_FRAME: C2RustUnnamed_3 = 3;
pub const LAST2_FRAME: C2RustUnnamed_3 = 2;
pub const LAST_FRAME: C2RustUnnamed_3 = 1;
pub const INTRA_FRAME: C2RustUnnamed_3 = 0;
pub const NONE_FRAME: C2RustUnnamed_3 = -1;
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
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_8,
    pub c2rust_unnamed_3: C2RustUnnamed_6,
    pub c2rust_unnamed_4: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
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
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_16 = 4;
pub const S_FRAME: C2RustUnnamed_16 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_16 = 2;
pub const INTER_FRAME: C2RustUnnamed_16 = 1;
pub const KEY_FRAME: C2RustUnnamed_16 = 0;
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
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const ALLINTRA: C2RustUnnamed_17 = 2;
pub const REALTIME: C2RustUnnamed_17 = 1;
pub const GOOD: C2RustUnnamed_17 = 0;
pub type MODE = uint8_t;
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
pub type C2RustUnnamed_18 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_18 = 2;
pub const LAP_STAGE: C2RustUnnamed_18 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = core::ffi::c_uint;
pub const RATE_FACTOR_LEVELS: C2RustUnnamed_19 = 4;
pub const KF_STD: C2RustUnnamed_19 = 3;
pub const GF_ARF_STD: C2RustUnnamed_19 = 2;
pub const GF_ARF_LOW: C2RustUnnamed_19 = 1;
pub const INTER_NORMAL: C2RustUnnamed_19 = 0;
pub type RATE_FACTOR_LEVEL = uint8_t;
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
pub const REFBUF_STATES: C2RustUnnamed_21 = 2;
pub const REFBUF_UPDATE: C2RustUnnamed_21 = 1;
pub const REFBUF_RESET: C2RustUnnamed_21 = 0;
pub type RESIZE_ACTION = core::ffi::c_int;
pub const UP_ORIG: RESIZE_ACTION = -2;
pub const UP_THREEFOUR: RESIZE_ACTION = -1;
pub const DOWN_ONEHALF: RESIZE_ACTION = 2;
pub const DOWN_THREEFOUR: RESIZE_ACTION = 1;
pub const NO_RESIZE: RESIZE_ACTION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodeFrameInput {
    pub source: *mut YV12_BUFFER_CONFIG,
    pub last_source: *mut YV12_BUFFER_CONFIG,
    pub ts_duration: int64_t,
}
pub const CYCLIC_REFRESH_AQ: C2RustUnnamed_24 = 3;
pub const FAST_DETECTION_MAXQ: C2RustUnnamed_22 = 1;
pub const RESIZE_DYNAMIC: C2RustUnnamed_25 = 3;
pub const FRAMEFLAGS_KEY: C2RustUnnamed_23 = 1;
pub type C2RustUnnamed_22 = core::ffi::c_uint;
pub const NO_DETECTION: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = core::ffi::c_uint;
pub const FRAMEFLAGS_ERROR_RESILIENT: C2RustUnnamed_23 = 64;
pub const FRAMEFLAGS_SWITCH: C2RustUnnamed_23 = 32;
pub const FRAMEFLAGS_INTRAONLY: C2RustUnnamed_23 = 16;
pub const FRAMEFLAGS_ALTREF: C2RustUnnamed_23 = 8;
pub const FRAMEFLAGS_BWDREF: C2RustUnnamed_23 = 4;
pub const FRAMEFLAGS_GOLDEN: C2RustUnnamed_23 = 2;
pub type C2RustUnnamed_24 = core::ffi::c_uint;
pub const AQ_MODE_COUNT: C2RustUnnamed_24 = 4;
pub const COMPLEXITY_AQ: C2RustUnnamed_24 = 2;
pub const VARIANCE_AQ: C2RustUnnamed_24 = 1;
pub const NO_AQ: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = core::ffi::c_uint;
pub const RESIZE_MODES: C2RustUnnamed_25 = 4;
pub const RESIZE_RANDOM: C2RustUnnamed_25 = 2;
pub const RESIZE_FIXED: C2RustUnnamed_25 = 1;
pub const RESIZE_NONE: C2RustUnnamed_25 = 0;
pub const UINT64_MAX: core::ffi::c_ulonglong = 18446744073709551615
    as core::ffi::c_ulonglong;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn fclamp(
    mut value: core::ffi::c_double,
    mut low: core::ffi::c_double,
    mut high: core::ffi::c_double,
) -> core::ffi::c_double {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn saturate_cast_double_to_int(
    mut d: core::ffi::c_double,
) -> core::ffi::c_int {
    if d > INT_MAX as core::ffi::c_double {
        return INT_MAX;
    }
    return d as core::ffi::c_int;
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func as Option<unsafe extern "C-unwind" fn() -> ()>);
}
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const MINQ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAXQ: core::ffi::c_int = 255 as core::ffi::c_int;
pub const QINDEX_RANGE: core::ffi::c_int = MAXQ - MINQ + 1 as core::ffi::c_int;
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
pub const BPER_MB_NORMBITS: core::ffi::c_int = 9 as core::ffi::c_int;
pub const STATIC_KF_GROUP_THRESH: core::ffi::c_int = 99 as core::ffi::c_int;
pub const MAX_STATIC_GF_GROUP_LENGTH: core::ffi::c_int = 250 as core::ffi::c_int;
pub const MIN_GF_INTERVAL: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_GF_INTERVAL: core::ffi::c_int = 32 as core::ffi::c_int;
pub const FIXED_GF_INTERVAL: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FIXED_GF_INTERVAL_RT: core::ffi::c_int = 80 as core::ffi::c_int;
pub const MAX_ARF_LAYERS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const DEFAULT_KF_BOOST_RT: core::ffi::c_int = 2300 as core::ffi::c_int;
pub const DEFAULT_GF_BOOST_RT: core::ffi::c_int = 2000 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn frame_is_kf_gf_arf(mut cpi: *const AV1_COMP) -> core::ffi::c_int {
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let update_type: FRAME_UPDATE_TYPE = (*gf_group)
        .update_type[(*cpi).gf_frame_index as usize];
    return (frame_is_intra_only(&(*cpi).common) != 0
        || update_type as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame_yv12_buf(
    cm: *const AV1_COMMON,
    mut ref_frame: MV_REFERENCE_FRAME,
) -> *const YV12_BUFFER_CONFIG {
    let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
    return if !buf.is_null() { &(*buf).buf } else { 0 as *const YV12_BUFFER_CONFIG };
}
pub const ALT_MIN_LAG: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_altref_enabled(
    mut lag_in_frames: core::ffi::c_int,
    mut enable_auto_arf: bool,
) -> core::ffi::c_int {
    return (lag_in_frames >= ALT_MIN_LAG && enable_auto_arf as core::ffi::c_int != 0)
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
unsafe extern "C-unwind" fn av1_resize_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).superres_upscaled_width != (*cm).render_width
        || (*cm).superres_upscaled_height != (*cm).render_height) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_frame_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (av1_superres_scaled(cm) != 0 || av1_resize_scaled(cm) != 0)
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
#[inline]
unsafe extern "C-unwind" fn is_frame_resize_pending(cpi: *const AV1_COMP) -> core::ffi::c_int {
    let resize_pending_params: *const ResizePendingParams = &(*cpi)
        .resize_pending_params;
    return ((*resize_pending_params).width != 0 && (*resize_pending_params).height != 0
        && ((*cpi).common.width != (*resize_pending_params).width
            || (*cpi).common.height != (*resize_pending_params).height))
        as core::ffi::c_int;
}
pub const AM_SEGMENT_ID_ACTIVE: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn size_in_mi(mut size: core::ffi::c_int) -> core::ffi::c_int {
    let aligned_size: core::ffi::c_int = size
        + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
    return aligned_size >> MI_SIZE_LOG2;
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
pub const RES_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MODE_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MIN_BPB_FACTOR: core::ffi::c_double = 0.005f64;
pub const MAX_BPB_FACTOR: core::ffi::c_int = 50 as core::ffi::c_int;
pub const SUPERRES_QADJ_PER_DENOM_KEYFRAME_SOLO: core::ffi::c_int = 0
    as core::ffi::c_int;
pub const SUPERRES_QADJ_PER_DENOM_KEYFRAME: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SUPERRES_QADJ_PER_DENOM_ARFFRAME: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FRAME_OVERHEAD_BITS: core::ffi::c_int = 200 as core::ffi::c_int;
static mut kf_low_motion_minq_8: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut kf_high_motion_minq_8: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_low_motion_minq_8: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_high_motion_minq_8: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut inter_minq_8: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut rtc_minq_8: [core::ffi::c_int; 256] = [0; 256];
static mut kf_low_motion_minq_10: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut kf_high_motion_minq_10: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_low_motion_minq_10: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_high_motion_minq_10: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut inter_minq_10: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut rtc_minq_10: [core::ffi::c_int; 256] = [0; 256];
static mut kf_low_motion_minq_12: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut kf_high_motion_minq_12: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_low_motion_minq_12: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut arfgf_high_motion_minq_12: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut inter_minq_12: [[[core::ffi::c_int; 256]; 2]; 2] = [[[0; 256]; 2]; 2];
static mut rtc_minq_12: [core::ffi::c_int; 256] = [0; 256];
static mut gf_high_1: core::ffi::c_int = 2875 as core::ffi::c_int;
static mut gf_low_1: core::ffi::c_int = 562 as core::ffi::c_int;
static mut gf_high_2: core::ffi::c_int = 4994 as core::ffi::c_int;
static mut gf_low_2: core::ffi::c_int = 100 as core::ffi::c_int;
static mut kf_high: core::ffi::c_int = 8000 as core::ffi::c_int;
static mut kf_low: core::ffi::c_int = 553 as core::ffi::c_int;
static mut gf_high_rtc: core::ffi::c_int = 2400 as core::ffi::c_int;
static mut gf_low_rtc: core::ffi::c_int = 300 as core::ffi::c_int;
static mut kf_high_rtc: core::ffi::c_int = 5000 as core::ffi::c_int;
static mut kf_low_rtc: core::ffi::c_int = 400 as core::ffi::c_int;
unsafe extern "C-unwind" fn resize_rate_factor(
    frm_dim_cfg: *const FrameDimensionCfg,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_double {
    return ((*frm_dim_cfg).width * (*frm_dim_cfg).height) as core::ffi::c_double
        / (width * height) as core::ffi::c_double;
}
unsafe extern "C-unwind" fn get_minq_index(
    mut maxq: core::ffi::c_double,
    mut x3: core::ffi::c_double,
    mut x2: core::ffi::c_double,
    mut x1_0: core::ffi::c_double,
    mut bit_depth: aom_bit_depth_t,
) -> core::ffi::c_int {
    let minqtarget: core::ffi::c_double = if ((x3 * maxq + x2) * maxq + x1_0) * maxq
        < maxq
    {
        ((x3 * maxq + x2) * maxq + x1_0) * maxq
    } else {
        maxq
    };
    if minqtarget <= 2.0f64 {
        return 0 as core::ffi::c_int;
    }
    return av1_find_qindex(
        minqtarget,
        bit_depth,
        0 as core::ffi::c_int,
        QINDEX_RANGE - 1 as core::ffi::c_int,
    );
}
static mut x1: [[[core::ffi::c_double; 5]; 2]; 2] = [
    [
        [0.1771f64, 0.379f64, 0.3279f64, 0.6634f64, 1.385f64],
        [0.1917f64, 0.3760f64, 0.34570f64, 0.6916f64, 1.14820f64],
    ],
    [
        [0.15f64, 0.45f64, 0.30f64, 0.55f64, 0.90f64],
        [0.15f64, 0.45f64, 0.30f64, 0.55f64, 0.90f64],
    ],
];
unsafe extern "C-unwind" fn init_minq_luts(
    mut kf_low_m: *mut [[core::ffi::c_int; 256]; 2],
    mut kf_high_m: *mut [[core::ffi::c_int; 256]; 2],
    mut arfgf_low: *mut [[core::ffi::c_int; 256]; 2],
    mut arfgf_high: *mut [[core::ffi::c_int; 256]; 2],
    mut inter: *mut [[core::ffi::c_int; 256]; 2],
    mut rtc: *mut core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut mode: core::ffi::c_int = 0 as core::ffi::c_int;
    while mode < MODE_NUM {
        let mut res: core::ffi::c_int = 0 as core::ffi::c_int;
        while res < RES_NUM {
            i = 0 as core::ffi::c_int;
            while i < QINDEX_RANGE {
                let maxq: core::ffi::c_double = av1_convert_qindex_to_q(i, bit_depth)
                    as core::ffi::c_double;
                (*kf_low_m.offset(mode as isize))[res as usize][i as usize] = get_minq_index(
                    maxq,
                    0.000001f64,
                    -0.0004f64,
                    x1[mode as usize][res as usize][0 as core::ffi::c_int as usize],
                    bit_depth,
                );
                (*kf_high_m.offset(mode as isize))[res as usize][i as usize] = get_minq_index(
                    maxq,
                    0.0000021f64,
                    -0.00125f64,
                    x1[mode as usize][res as usize][1 as core::ffi::c_int as usize],
                    bit_depth,
                );
                (*arfgf_low.offset(mode as isize))[res as usize][i as usize] = get_minq_index(
                    maxq,
                    0.0000015f64,
                    -0.0009f64,
                    x1[mode as usize][res as usize][2 as core::ffi::c_int as usize],
                    bit_depth,
                );
                (*arfgf_high.offset(mode as isize))[res as usize][i as usize] = get_minq_index(
                    maxq,
                    0.0000021f64,
                    -0.00125f64,
                    x1[mode as usize][res as usize][3 as core::ffi::c_int as usize],
                    bit_depth,
                );
                (*inter.offset(mode as isize))[res as usize][i as usize] = get_minq_index(
                    maxq,
                    0.00000271f64,
                    -0.00113f64,
                    x1[mode as usize][res as usize][4 as core::ffi::c_int as usize],
                    bit_depth,
                );
                *rtc.offset(i as isize) = get_minq_index(
                    maxq,
                    0.00000271f64,
                    -0.00113f64,
                    0.70f64,
                    bit_depth,
                );
                i += 1;
            }
            res += 1;
        }
        mode += 1;
    }
}
unsafe extern "C-unwind" fn rc_init_minq_luts() {
    init_minq_luts(
        core::ptr::addr_of_mut!(kf_low_motion_minq_8).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(kf_high_motion_minq_8).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_low_motion_minq_8).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_high_motion_minq_8).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(inter_minq_8).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(rtc_minq_8).cast::<core::ffi::c_int>(),
        AOM_BITS_8,
    );
    init_minq_luts(
        core::ptr::addr_of_mut!(kf_low_motion_minq_10).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(kf_high_motion_minq_10).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_low_motion_minq_10).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_high_motion_minq_10).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(inter_minq_10).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(rtc_minq_10).cast::<core::ffi::c_int>(),
        AOM_BITS_10,
    );
    init_minq_luts(
        core::ptr::addr_of_mut!(kf_low_motion_minq_12).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(kf_high_motion_minq_12).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_low_motion_minq_12).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(arfgf_high_motion_minq_12).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(inter_minq_12).cast::<[[core::ffi::c_int; 256]; 2]>(),
        core::ptr::addr_of_mut!(rtc_minq_12).cast::<core::ffi::c_int>(),
        AOM_BITS_12,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_init_minq_luts() {
    aom_once(Some(rc_init_minq_luts as unsafe extern "C-unwind" fn() -> ()));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convert_qindex_to_q(
    mut qindex: core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
) -> core::ffi::c_double {
    match bit_depth as core::ffi::c_uint {
        8 => {
            return av1_ac_quant_QTX(qindex, 0 as core::ffi::c_int, bit_depth)
                as core::ffi::c_int as core::ffi::c_double / 4.0f64;
        }
        10 => {
            return av1_ac_quant_QTX(qindex, 0 as core::ffi::c_int, bit_depth)
                as core::ffi::c_int as core::ffi::c_double / 16.0f64;
        }
        12 => {
            return av1_ac_quant_QTX(qindex, 0 as core::ffi::c_int, bit_depth)
                as core::ffi::c_int as core::ffi::c_double / 64.0f64;
        }
        _ => return -1.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convert_q_to_qindex(
    mut q: core::ffi::c_double,
    mut bit_depth: aom_bit_depth_t,
) -> core::ffi::c_int {
    let mut qindex: core::ffi::c_int = MINQ;
    while qindex < MAXQ && av1_convert_qindex_to_q(qindex, bit_depth) < q {
        qindex += 1;
    }
    return qindex;
}
unsafe extern "C-unwind" fn get_bpmb_enumerator(
    mut frame_type: FRAME_TYPE,
    is_screen_content_type: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut enumerator: core::ffi::c_int = 0;
    if is_screen_content_type != 0 {
        enumerator = if frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
            1000000 as core::ffi::c_int
        } else {
            750000 as core::ffi::c_int
        };
    } else {
        enumerator = if frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
            2000000 as core::ffi::c_int
        } else {
            1500000 as core::ffi::c_int
        };
    }
    return enumerator;
}
unsafe extern "C-unwind" fn get_init_ratio(mut sse: core::ffi::c_double) -> core::ffi::c_int {
    return (300000 as core::ffi::c_int as core::ffi::c_double / sse) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn adjust_rtc_keyframe(
    mut rc: *const RATE_CONTROL,
    mut enumerator: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*rc).perc_spatial_flat_blocks > 70 as core::ffi::c_int {
        return enumerator;
    }
    if (*rc).last_encoded_size_keyframe == 0 as core::ffi::c_int
        || (*rc).frames_since_scene_change < (*rc).frames_since_key
    {
        if (*rc).frame_spatial_variance > 1000 as uint64_t
            || (*rc).frame_spatial_variance > 500 as uint64_t
                && (*rc).perc_spatial_flat_blocks == 0 as core::ffi::c_int
        {
            return enumerator << 3 as core::ffi::c_int
        } else if (*rc).frame_spatial_variance > 500 as uint64_t
            && (*rc).perc_spatial_flat_blocks < 10 as core::ffi::c_int
        {
            return enumerator << 2 as core::ffi::c_int
        } else if (*rc).frame_spatial_variance > 400 as uint64_t {
            return enumerator << 1 as core::ffi::c_int
        }
    } else if (*rc).frames_since_scene_change >= (*rc).frames_since_key {
        if (*rc).last_encoded_size_keyframe
            > 4 as core::ffi::c_int * (*rc).last_target_size_keyframe
            && (*rc).frame_spatial_variance > 500 as uint64_t
        {
            return enumerator << 3 as core::ffi::c_int
        } else if (*rc).last_encoded_size_keyframe
            > 2 as core::ffi::c_int * (*rc).last_target_size_keyframe
            && (*rc).frame_spatial_variance > 200 as uint64_t
        {
            return enumerator << 2 as core::ffi::c_int
        } else if (*rc).last_encoded_size_keyframe > (*rc).last_target_size_keyframe {
            return enumerator << 1 as core::ffi::c_int
        }
    }
    return enumerator;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_bits_per_mb(
    mut cpi: *const AV1_COMP,
    mut frame_type: FRAME_TYPE,
    mut qindex: core::ffi::c_int,
    mut correction_factor: core::ffi::c_double,
    mut accurate_estimate: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let is_screen_content_type: core::ffi::c_int = (*cpi).is_screen_content_type;
    let bit_depth: aom_bit_depth_t = (*(*cm).seq_params).bit_depth;
    let q: core::ffi::c_double = av1_convert_qindex_to_q(qindex, bit_depth)
        as core::ffi::c_double;
    let mut enumerator: core::ffi::c_int = get_bpmb_enumerator(
        frame_type,
        is_screen_content_type,
    );
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        && frame_type as core::ffi::c_int != KEY_FRAME as core::ffi::c_int
        && accurate_estimate != 0 && (*cpi).rec_sse != UINT64_MAX as uint64_t
    {
        let mbs: core::ffi::c_int = (*cm).mi_params.MBs;
        let sse_sqrt: core::ffi::c_double = ((sqrt((*cpi).rec_sse as core::ffi::c_double)
            as core::ffi::c_int) << BPER_MB_NORMBITS) as core::ffi::c_double
            / mbs as core::ffi::c_double;
        let ratio: core::ffi::c_int = if (*cpi).rc.bit_est_ratio == 0 as core::ffi::c_int
        {
            get_init_ratio(sse_sqrt) as core::ffi::c_int
        } else {
            (*cpi).rc.bit_est_ratio
        };
        enumerator = clamp(
            (ratio as core::ffi::c_double * sse_sqrt) as core::ffi::c_int,
            20000 as core::ffi::c_int,
            170000 as core::ffi::c_int,
        );
    } else if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        && frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        && (*cpi).sf.rt_sf.rc_adjust_keyframe != 0
        && bit_depth as core::ffi::c_uint == 8 as core::ffi::c_uint
        && (*cpi).oxcf.rc_cfg.max_intra_bitrate_pct > 0 as core::ffi::c_uint
        && (*cpi).svc.spatial_layer_id == 0 as core::ffi::c_int
    {
        enumerator = adjust_rtc_keyframe(&(*cpi).rc, enumerator);
    }
    return (enumerator as core::ffi::c_double * correction_factor / q)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_estimate_bits_at_q(
    mut cpi: *const AV1_COMP,
    mut q: core::ffi::c_int,
    mut correction_factor: core::ffi::c_double,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let frame_type: FRAME_TYPE = (*cm).current_frame.frame_type;
    let mbs: core::ffi::c_int = (*cm).mi_params.MBs;
    let bpm: core::ffi::c_int = av1_rc_bits_per_mb(
        cpi,
        frame_type,
        q,
        correction_factor,
        (*cpi).sf.hl_sf.accurate_bit_estimate,
    );
    return if 200 as core::ffi::c_int
        > (bpm as uint64_t).wrapping_mul(mbs as uint64_t) as core::ffi::c_int
            >> 9 as core::ffi::c_int
    {
        200 as core::ffi::c_int
    } else {
        (bpm as uint64_t).wrapping_mul(mbs as uint64_t) as core::ffi::c_int
            >> 9 as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn clamp_pframe_target_size(
    cpi: *const AV1_COMP,
    mut target: int64_t,
    mut frame_update_type: FRAME_UPDATE_TYPE,
) -> core::ffi::c_int {
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let rc_cfg: *const RateControlCfg = &(*cpi).oxcf.rc_cfg;
    let min_frame_target: core::ffi::c_int = if (*rc).min_frame_bandwidth
        > (*rc).avg_frame_bandwidth >> 5 as core::ffi::c_int
    {
        (*rc).min_frame_bandwidth
    } else {
        (*rc).avg_frame_bandwidth >> 5 as core::ffi::c_int
    };
    if frame_update_type as core::ffi::c_int == OVERLAY_UPDATE as core::ffi::c_int
        || frame_update_type as core::ffi::c_int
            == INTNL_OVERLAY_UPDATE as core::ffi::c_int
    {
        target = min_frame_target as int64_t;
    } else if target < min_frame_target as int64_t {
        target = min_frame_target as int64_t;
    }
    if target > (*rc).max_frame_bandwidth as int64_t {
        target = (*rc).max_frame_bandwidth as int64_t;
    }
    if (*rc_cfg).max_inter_bitrate_pct != 0 {
        let max_rate: int64_t = (*rc).avg_frame_bandwidth as int64_t
            * (*rc_cfg).max_inter_bitrate_pct as int64_t / 100 as int64_t;
        target = if target < max_rate { target } else { max_rate };
    }
    return target as core::ffi::c_int;
}
unsafe extern "C-unwind" fn clamp_iframe_target_size(
    cpi: *const AV1_COMP,
    mut target: int64_t,
) -> core::ffi::c_int {
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let rc_cfg: *const RateControlCfg = &(*cpi).oxcf.rc_cfg;
    if (*rc_cfg).max_intra_bitrate_pct != 0 {
        let max_rate: int64_t = (*rc).avg_frame_bandwidth as int64_t
            * (*rc_cfg).max_intra_bitrate_pct as int64_t / 100 as int64_t;
        target = if target < max_rate { target } else { max_rate };
    }
    if target > (*rc).max_frame_bandwidth as int64_t {
        target = (*rc).max_frame_bandwidth as int64_t;
    }
    return target as core::ffi::c_int;
}
unsafe extern "C-unwind" fn update_layer_buffer_level(
    mut svc: *mut SVC,
    mut encoded_frame_size: core::ffi::c_int,
    mut is_screen: bool,
) {
    let current_temporal_layer: core::ffi::c_int = (*svc).temporal_layer_id;
    let mut i: core::ffi::c_int = current_temporal_layer + 1 as core::ffi::c_int;
    while i < (*svc).number_temporal_layers {
        let layer: core::ffi::c_int = (*svc).spatial_layer_id
            * (*svc).number_temporal_layers + i;
        let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
            .offset(layer as isize) as *mut LAYER_CONTEXT;
        let mut lp_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*lc).p_rc;
        (*lp_rc).bits_off_target
            += (round((*lc).target_bandwidth as core::ffi::c_double / (*lc).framerate)
                as core::ffi::c_int - encoded_frame_size) as int64_t;
        (*lp_rc).bits_off_target = if (*lp_rc).bits_off_target
            < (*lp_rc).maximum_buffer_size
        {
            (*lp_rc).bits_off_target
        } else {
            (*lp_rc).maximum_buffer_size
        };
        (*lp_rc).buffer_level = (*lp_rc).bits_off_target;
        if is_screen {
            (*lp_rc).bits_off_target = if (*lp_rc).bits_off_target
                > -(*lp_rc).maximum_buffer_size
            {
                (*lp_rc).bits_off_target
            } else {
                -(*lp_rc).maximum_buffer_size
            };
            (*lp_rc).buffer_level = (*lp_rc).bits_off_target;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn update_buffer_level(
    mut cpi: *mut AV1_COMP,
    mut encoded_frame_size: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    if (*cm).show_frame == 0 {
        (*p_rc).bits_off_target -= encoded_frame_size as int64_t;
    } else {
        (*p_rc).bits_off_target
            += ((*rc).avg_frame_bandwidth - encoded_frame_size) as int64_t;
    }
    (*p_rc).bits_off_target = if (*p_rc).bits_off_target < (*p_rc).maximum_buffer_size {
        (*p_rc).bits_off_target
    } else {
        (*p_rc).maximum_buffer_size
    };
    if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
    {
        (*p_rc).bits_off_target = if (*p_rc).bits_off_target
            > -(*p_rc).maximum_buffer_size
        {
            (*p_rc).bits_off_target
        } else {
            -(*p_rc).maximum_buffer_size
        };
    }
    (*p_rc).buffer_level = (*p_rc).bits_off_target;
    if (*(*cpi).ppi).use_svc != 0 {
        update_layer_buffer_level(
            &raw mut (*cpi).svc,
            encoded_frame_size,
            (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
                == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_get_default_min_gf_interval(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut framerate: core::ffi::c_double,
) -> core::ffi::c_int {
    static mut factor_safe: core::ffi::c_double = (3840 as core::ffi::c_int
        * 2160 as core::ffi::c_int) as core::ffi::c_double * 20.0f64;
    let factor: core::ffi::c_double = width as core::ffi::c_double
        * height as core::ffi::c_double * framerate;
    let default_interval: core::ffi::c_int = clamp(
        (framerate * 0.125f64) as core::ffi::c_int,
        MIN_GF_INTERVAL,
        MAX_GF_INTERVAL,
    ) as core::ffi::c_int;
    if factor <= factor_safe {
        return default_interval
    } else {
        return if default_interval
            > (4 as core::ffi::c_int as core::ffi::c_double * factor / factor_safe
                + 0.5f64) as core::ffi::c_int
        {
            default_interval
        } else {
            (4 as core::ffi::c_int as core::ffi::c_double * factor / factor_safe
                + 0.5f64) as core::ffi::c_int
        }
    };
}
unsafe extern "C-unwind" fn get_default_max_gf_interval(
    mut framerate: core::ffi::c_double,
    mut min_gf_interval: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut interval: core::ffi::c_int = if (32 as core::ffi::c_int)
        < (framerate * 0.75f64) as core::ffi::c_int
    {
        32 as core::ffi::c_int
    } else {
        (framerate * 0.75f64) as core::ffi::c_int
    };
    interval += interval & 0x1 as core::ffi::c_int;
    interval = if 32 as core::ffi::c_int > interval {
        32 as core::ffi::c_int
    } else {
        interval
    };
    return if interval > min_gf_interval { interval } else { min_gf_interval };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_primary_rc_init(
    mut oxcf: *const AV1EncoderConfig,
    mut p_rc: *mut PRIMARY_RATE_CONTROL,
) {
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    let mut worst_allowed_q: core::ffi::c_int = (*rc_cfg).worst_allowed_q;
    let mut min_gf_interval: core::ffi::c_int = (*oxcf).gf_cfg.min_gf_interval;
    let mut max_gf_interval: core::ffi::c_int = (*oxcf).gf_cfg.max_gf_interval;
    if min_gf_interval == 0 as core::ffi::c_int {
        min_gf_interval = av1_rc_get_default_min_gf_interval(
            (*oxcf).frm_dim_cfg.width,
            (*oxcf).frm_dim_cfg.height,
            (*oxcf).input_cfg.init_framerate,
        );
    }
    if max_gf_interval == 0 as core::ffi::c_int {
        max_gf_interval = get_default_max_gf_interval(
            (*oxcf).input_cfg.init_framerate,
            min_gf_interval,
        );
    }
    (*p_rc).baseline_gf_interval = (min_gf_interval + max_gf_interval)
        / 2 as core::ffi::c_int;
    (*p_rc).this_key_frame_forced = 0 as core::ffi::c_int;
    (*p_rc).next_key_frame_forced = 0 as core::ffi::c_int;
    (*p_rc).ni_frames = 0 as core::ffi::c_int;
    (*p_rc).tot_q = 0.0f64;
    (*p_rc).total_actual_bits = 0 as int64_t;
    (*p_rc).total_target_bits = 0 as int64_t;
    (*p_rc).buffer_level = (*p_rc).starting_buffer_level;
    if ((*oxcf).target_seq_level_idx[0 as core::ffi::c_int as usize] as core::ffi::c_int)
        < SEQ_LEVELS as core::ffi::c_int
    {
        worst_allowed_q = 255 as core::ffi::c_int;
    }
    if (*oxcf).pass as core::ffi::c_uint
        == AOM_RC_ONE_PASS as core::ffi::c_int as core::ffi::c_uint
        && (*rc_cfg).mode as core::ffi::c_uint
            == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
    {
        (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize] = worst_allowed_q;
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = worst_allowed_q;
    } else {
        (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize] = (worst_allowed_q
            + (*rc_cfg).best_allowed_q) / 2 as core::ffi::c_int;
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (worst_allowed_q
            + (*rc_cfg).best_allowed_q) / 2 as core::ffi::c_int;
    }
    (*p_rc).avg_q = av1_convert_qindex_to_q(
        (*rc_cfg).worst_allowed_q,
        (*oxcf).tool_cfg.bit_depth,
    );
    (*p_rc).last_q[KEY_FRAME as core::ffi::c_int as usize] = (*rc_cfg).best_allowed_q;
    (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize] = (*rc_cfg).worst_allowed_q;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < RATE_FACTOR_LEVELS as core::ffi::c_int {
        (*p_rc).rate_correction_factors[i as usize] = 0.7f64;
        i += 1;
    }
    (*p_rc).rate_correction_factors[KF_STD as core::ffi::c_int as usize] = 1.0f64;
    (*p_rc).bits_off_target = (*p_rc).starting_buffer_level;
    let bits_per_frame: core::ffi::c_double = (*oxcf).rc_cfg.target_bandwidth
        as core::ffi::c_double / (*oxcf).input_cfg.init_framerate;
    (*p_rc).rolling_target_bits = if 1 as core::ffi::c_int
        > (if bits_per_frame > 2147483647 as core::ffi::c_int as core::ffi::c_double {
            2147483647 as core::ffi::c_int
        } else {
            bits_per_frame as core::ffi::c_int
        })
    {
        1 as core::ffi::c_int
    } else if bits_per_frame > 2147483647 as core::ffi::c_int as core::ffi::c_double {
        2147483647 as core::ffi::c_int
    } else {
        bits_per_frame as core::ffi::c_int
    };
    (*p_rc).rolling_actual_bits = (*p_rc).rolling_target_bits;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_init(
    mut oxcf: *const AV1EncoderConfig,
    mut rc: *mut RATE_CONTROL,
) {
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    (*rc).frames_since_key = 8 as core::ffi::c_int;
    (*rc).frames_to_fwd_kf = (*oxcf).kf_cfg.fwd_kf_dist;
    (*rc).frames_till_gf_update_due = 0 as core::ffi::c_int;
    (*rc).ni_av_qi = (*rc_cfg).worst_allowed_q;
    (*rc).ni_tot_qi = 0 as core::ffi::c_int;
    (*rc).min_gf_interval = (*oxcf).gf_cfg.min_gf_interval;
    (*rc).max_gf_interval = (*oxcf).gf_cfg.max_gf_interval;
    if (*rc).min_gf_interval == 0 as core::ffi::c_int {
        (*rc).min_gf_interval = av1_rc_get_default_min_gf_interval(
            (*oxcf).frm_dim_cfg.width,
            (*oxcf).frm_dim_cfg.height,
            (*oxcf).input_cfg.init_framerate,
        );
    }
    if (*rc).max_gf_interval == 0 as core::ffi::c_int {
        (*rc).max_gf_interval = get_default_max_gf_interval(
            (*oxcf).input_cfg.init_framerate,
            (*rc).min_gf_interval,
        );
    }
    (*rc).avg_frame_low_motion = 0 as core::ffi::c_int;
    (*rc).resize_state = ORIG;
    (*rc).resize_avg_qp = 0 as core::ffi::c_int;
    (*rc).resize_buffer_underflow = 0 as core::ffi::c_int;
    (*rc).resize_count = 0 as core::ffi::c_int;
    (*rc).rtc_external_ratectrl = 0 as core::ffi::c_int;
    (*rc).frame_level_fast_extra_bits = 0 as core::ffi::c_int;
    (*rc).use_external_qp_one_pass = 0 as core::ffi::c_int;
    (*rc).percent_blocks_inactive = 0 as core::ffi::c_int;
    (*rc).force_max_q = 0 as core::ffi::c_int;
    (*rc).postencode_drop = 0 as core::ffi::c_int;
    (*rc).frames_since_scene_change = 0 as core::ffi::c_int;
    (*rc).last_frame_low_source_sad = 0 as core::ffi::c_uint;
}
unsafe extern "C-unwind" fn check_buffer_below_thresh(
    mut cpi: *mut AV1_COMP,
    mut buffer_level: int64_t,
    mut drop_mark: core::ffi::c_int,
) -> bool {
    let mut svc: *mut SVC = &raw mut (*cpi).svc;
    if (*(*cpi).ppi).use_svc == 0
        || (*cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
        || (*cpi).svc.framedrop_mode as core::ffi::c_uint
            == AOM_LAYER_DROP as core::ffi::c_int as core::ffi::c_uint
    {
        return buffer_level <= drop_mark as int64_t
    } else {
        let mut i: core::ffi::c_int = (*svc).spatial_layer_id;
        while i < (*svc).number_spatial_layers {
            let layer: core::ffi::c_int = i * (*svc).number_temporal_layers
                + (*svc).temporal_layer_id;
            let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                .offset(layer as isize) as *mut LAYER_CONTEXT;
            let mut lrc: *mut PRIMARY_RATE_CONTROL = &raw mut (*lc).p_rc;
            if (*lc).target_bandwidth > 0 as int64_t {
                let drop_thresh: core::ffi::c_int = (*cpi)
                    .oxcf
                    .rc_cfg
                    .drop_frames_water_mark;
                let drop_mark_layer: core::ffi::c_int = (drop_thresh as int64_t
                    * (*lrc).optimal_buffer_level / 100 as int64_t) as core::ffi::c_int;
                if (*lrc).buffer_level <= drop_mark_layer as int64_t {
                    return true_0 != 0;
                }
            }
            i += 1;
        }
        return false_0 != 0;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_drop_frame(mut cpi: *mut AV1_COMP) -> core::ffi::c_int {
    let mut oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut buffer_level: int64_t = (*p_rc).buffer_level;
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*(*cpi).ppi).use_svc != 0
            && (*((*cpi).svc.layer_context)
                .offset((*cpi).svc.temporal_layer_id as isize))
                .is_key_frame != 0 || (*oxcf).rc_cfg.drop_frames_water_mark == 0
        || (*rc).max_consec_drop > 0 as core::ffi::c_int
            && (*rc).drop_count_consec >= (*rc).max_consec_drop
    {
        return 0 as core::ffi::c_int
    } else {
        let mut svc: *mut SVC = &raw mut (*cpi).svc;
        if (*(*cpi).ppi).use_svc != 0 && (*svc).spatial_layer_id > 0 as core::ffi::c_int
            && (*svc)
                .drop_spatial_layer[((*svc).spatial_layer_id - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int != 0
            && (*svc).framedrop_mode as core::ffi::c_uint
                == AOM_FULL_SUPERFRAME_DROP as core::ffi::c_int as core::ffi::c_uint
        {
            return 1 as core::ffi::c_int;
        }
        if check_buffer_below_thresh(cpi, buffer_level, -(1 as core::ffi::c_int)) {
            (*rc).drop_count_consec += 1;
            return 1 as core::ffi::c_int;
        } else {
            let drop_mark: core::ffi::c_int = ((*oxcf).rc_cfg.drop_frames_water_mark
                as int64_t * (*p_rc).optimal_buffer_level / 100 as int64_t)
                as core::ffi::c_int;
            let buffer_below_thresh: bool = check_buffer_below_thresh(
                cpi,
                buffer_level,
                drop_mark,
            ) as bool;
            if !buffer_below_thresh && (*rc).decimation_factor > 0 as core::ffi::c_int {
                (*rc).decimation_factor -= 1;
            } else if buffer_below_thresh as core::ffi::c_int != 0
                && (*rc).decimation_factor == 0 as core::ffi::c_int
            {
                (*rc).decimation_factor = 1 as core::ffi::c_int;
            }
            if (*rc).decimation_factor > 0 as core::ffi::c_int {
                if (*rc).decimation_count > 0 as core::ffi::c_int {
                    (*rc).decimation_count -= 1;
                    (*rc).drop_count_consec += 1;
                    return 1 as core::ffi::c_int;
                } else {
                    (*rc).decimation_count = (*rc).decimation_factor;
                    return 0 as core::ffi::c_int;
                }
            } else {
                (*rc).decimation_count = 0 as core::ffi::c_int;
                return 0 as core::ffi::c_int;
            }
        }
    };
}
unsafe extern "C-unwind" fn adjust_q_cbr(
    mut cpi: *const AV1_COMP,
    mut q: core::ffi::c_int,
    mut active_worst_quality: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let cm: *const AV1_COMMON = &(*cpi).common;
    let svc: *const SVC = &(*cpi).svc;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let overshoot_buffer_low: bool = (*cpi).rc.rc_1_frame == -(1 as core::ffi::c_int)
        && (*rc).frame_source_sad > 1000 as uint64_t
        && (*p_rc).buffer_level < (*p_rc).optimal_buffer_level >> 1 as core::ffi::c_int
        && (*rc).frames_since_key > 4 as core::ffi::c_int;
    let mut max_delta_down: core::ffi::c_int = 0;
    let mut max_delta_up: core::ffi::c_int = if overshoot_buffer_low as core::ffi::c_int
        != 0
    {
        120 as core::ffi::c_int
    } else {
        20 as core::ffi::c_int
    };
    let change_avg_frame_bandwidth: core::ffi::c_int = (abs(
        (*rc).avg_frame_bandwidth - (*rc).prev_avg_frame_bandwidth,
    ) as core::ffi::c_double > 0.1f64 * (*rc).avg_frame_bandwidth as core::ffi::c_double)
        as core::ffi::c_int;
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        == CYCLIC_REFRESH_AQ as core::ffi::c_int
        && (*(*cpi).cyclic_refresh).apply_cyclic_refresh != 0
    {
        if (*cpi).is_screen_content_type != 0
            && (*(*cpi).cyclic_refresh).sb_index > (*(*cpi).cyclic_refresh).last_sb_index
        {
            max_delta_down = clamp(
                (*rc).q_1_frame / 32 as core::ffi::c_int,
                1 as core::ffi::c_int,
                8 as core::ffi::c_int,
            );
        } else {
            max_delta_down = clamp(
                (*rc).q_1_frame / 8 as core::ffi::c_int,
                1 as core::ffi::c_int,
                16 as core::ffi::c_int,
            );
        }
        if (*(*cpi).ppi).use_svc == 0 && (*cpi).is_screen_content_type != 0 {
            if (*p_rc).buffer_level > (*p_rc).optimal_buffer_level {
                max_delta_up = if 4 as core::ffi::c_int > max_delta_down {
                    4 as core::ffi::c_int
                } else {
                    max_delta_down
                };
            } else if !overshoot_buffer_low {
                max_delta_up = if 8 as core::ffi::c_int > max_delta_down {
                    8 as core::ffi::c_int
                } else {
                    max_delta_down
                };
            }
        }
    } else {
        max_delta_down = if (*cpi).is_screen_content_type != 0 {
            clamp(
                (*rc).q_1_frame / 16 as core::ffi::c_int,
                1 as core::ffi::c_int,
                8 as core::ffi::c_int,
            )
        } else {
            clamp(
                (*rc).q_1_frame / 8 as core::ffi::c_int,
                1 as core::ffi::c_int,
                16 as core::ffi::c_int,
            )
        };
    }
    if (*cpi).sf.rt_sf.rc_faster_convergence_static == 1 as core::ffi::c_int
        && (*cpi).sf.rt_sf.check_scene_detection != 0
        && (*rc).frame_source_sad == 0 as uint64_t
        && (*rc).static_since_last_scene_change != 0
        && (*p_rc).buffer_level > (*p_rc).optimal_buffer_level >> 1 as core::ffi::c_int
        && (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
            == CYCLIC_REFRESH_AQ as core::ffi::c_int
        && (*(*cpi).cyclic_refresh).counter_encode_maxq_scene_change
            > 4 as core::ffi::c_int
    {
        let mut qp_delta: core::ffi::c_int = 32 as core::ffi::c_int;
        let mut qp_bias: core::ffi::c_int = 16 as core::ffi::c_int;
        if (*p_rc).buffer_level > (*p_rc).optimal_buffer_level {
            qp_delta = 60 as core::ffi::c_int;
            qp_bias = 32 as core::ffi::c_int;
        }
        if (*cpi).rc.rc_1_frame == 1 as core::ffi::c_int {
            q = q - qp_bias;
        }
        max_delta_down = if max_delta_down > qp_delta {
            max_delta_down
        } else {
            qp_delta
        };
        max_delta_up = if max_delta_up < 4 as core::ffi::c_int {
            max_delta_up
        } else {
            4 as core::ffi::c_int
        };
    }
    let change_target_bits_mb: core::ffi::c_int = (!((*cm).prev_frame).is_null()
        && (width != (*(*cm).prev_frame).width || height != (*(*cm).prev_frame).height
            || change_avg_frame_bandwidth != 0)) as core::ffi::c_int;
    if frame_is_intra_only(cm) == 0 && (*rc).frames_since_key > 1 as core::ffi::c_int
        && (*rc).q_1_frame > 0 as core::ffi::c_int
        && (*rc).q_2_frame > 0 as core::ffi::c_int
        && ((*(*cpi).ppi).use_svc == 0
            || (*svc).current_superframe
                > (*svc).number_temporal_layers as core::ffi::c_uint)
        && change_target_bits_mb == 0 && (*cpi).rc.rtc_external_ratectrl == 0
        && ((*cpi).oxcf.rc_cfg.gf_cbr_boost_pct == 0
            || !((*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
                || (*refresh_frame).golden_frame as core::ffi::c_int != 0))
    {
        if (*rc).rc_1_frame * (*rc).rc_2_frame == -(1 as core::ffi::c_int)
            && (*rc).q_1_frame != (*rc).q_2_frame && !overshoot_buffer_low
        {
            let mut qclamp: core::ffi::c_int = clamp(
                q,
                if (*rc).q_1_frame < (*rc).q_2_frame {
                    (*rc).q_1_frame
                } else {
                    (*rc).q_2_frame
                },
                if (*rc).q_1_frame > (*rc).q_2_frame {
                    (*rc).q_1_frame
                } else {
                    (*rc).q_2_frame
                },
            );
            if (*cpi).rc.rc_1_frame == -(1 as core::ffi::c_int) && q > qclamp
                && (*rc).frames_since_key > 10 as core::ffi::c_int
            {
                q = q + qclamp >> 1 as core::ffi::c_int;
            } else {
                q = qclamp;
            }
        }
        if (*cpi).sf.rt_sf.check_scene_detection != 0
            && (*rc).prev_avg_source_sad > 0 as uint64_t
            && (*rc).frames_since_key > 10 as core::ffi::c_int
            && (*rc).frame_source_sad > 0 as uint64_t
            && (*cpi).rc.rtc_external_ratectrl == 0
        {
            let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth
                as core::ffi::c_int;
            let mut delta: core::ffi::c_double = (*rc).avg_source_sad
                as core::ffi::c_double / (*rc).prev_avg_source_sad as core::ffi::c_double
                - 1.0f64;
            if delta < 0.0f64
                && (*p_rc).buffer_level
                    > (*p_rc).optimal_buffer_level >> 2 as core::ffi::c_int
                && q > (*rc).worst_quality >> 1 as core::ffi::c_int
            {
                let mut q_adj_factor: core::ffi::c_double = 1.0f64
                    + 0.5f64 * tanh(4.0f64 * delta);
                let mut q_val: core::ffi::c_double = av1_convert_qindex_to_q(
                    q,
                    bit_depth as aom_bit_depth_t,
                );
                q
                    += av1_compute_qdelta(
                        rc,
                        q_val,
                        q_val * q_adj_factor,
                        bit_depth as aom_bit_depth_t,
                    );
            } else if (*rc).q_1_frame - q > 0 as core::ffi::c_int && delta > 0.1f64
                && (*p_rc).buffer_level
                    < (if (*p_rc).maximum_buffer_size
                        < (*p_rc).optimal_buffer_level << 1 as core::ffi::c_int
                    {
                        (*p_rc).maximum_buffer_size
                    } else {
                        (*p_rc).optimal_buffer_level << 1 as core::ffi::c_int
                    })
            {
                q = 3 as core::ffi::c_int * q + (*rc).q_1_frame >> 2 as core::ffi::c_int;
            }
        }
        if (*rc).q_1_frame - q > max_delta_down {
            q = (*rc).q_1_frame - max_delta_down;
        } else if q - (*rc).q_1_frame > max_delta_up {
            q = (*rc).q_1_frame + max_delta_up;
        }
    }
    if (*svc).number_temporal_layers > 1 as core::ffi::c_int
        && (*svc).spatial_layer_id == 0 as core::ffi::c_int && change_target_bits_mb == 0
        && (*cpi).rc.rtc_external_ratectrl == 0
        && (*cpi).oxcf.resize_cfg.resize_mode as core::ffi::c_int
            != RESIZE_DYNAMIC as core::ffi::c_int
    {
        if (*svc).temporal_layer_id > 0 as core::ffi::c_int {
            let layer: core::ffi::c_int = 0 as core::ffi::c_int
                * (*svc).number_temporal_layers + 0 as core::ffi::c_int;
            let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                .offset(layer as isize) as *mut LAYER_CONTEXT;
            let last_qindex_tl0: core::ffi::c_int = if (*rc).frames_since_key
                < (*svc).number_temporal_layers
            {
                (*lc).p_rc.last_q[KEY_FRAME as core::ffi::c_int as usize]
            } else {
                (*lc).p_rc.last_q[INTER_FRAME as core::ffi::c_int as usize]
            };
            if (*rc).avg_frame_bandwidth < (*lc).rc.avg_frame_bandwidth
                && q < last_qindex_tl0 - 4 as core::ffi::c_int
            {
                q = last_qindex_tl0 - 4 as core::ffi::c_int;
            }
        } else if (*cpi).svc.temporal_layer_id == 0 as core::ffi::c_int
            && frame_is_intra_only(cm) == 0
            && (*p_rc).buffer_level
                > (*p_rc).optimal_buffer_level >> 2 as core::ffi::c_int
            && (*rc).frame_source_sad < 100000 as uint64_t
        {
            let mut delta_0: core::ffi::c_int = if (*svc).number_temporal_layers
                == 2 as core::ffi::c_int
            {
                4 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            };
            q = q - delta_0;
        }
    }
    if (*(*cpi).ppi).use_svc == 0 && !((*cm).prev_frame).is_null()
        && (width * height) as core::ffi::c_double
            > 1.5f64 * (*(*cm).prev_frame).width as core::ffi::c_double
                * (*(*cm).prev_frame).height as core::ffi::c_double
    {
        q = q + active_worst_quality >> 1 as core::ffi::c_int;
    }
    if (*(*cpi).ppi).rtc_ref.bias_recovery_frame {
        let min_dist: core::ffi::c_int = av1_svc_get_min_ref_dist(cpi as *const AV1_COMP)
            as core::ffi::c_int;
        q = q
            - (if min_dist < 20 as core::ffi::c_int {
                min_dist
            } else {
                20 as core::ffi::c_int
            });
    }
    return clamp(q, (*cpi).rc.best_quality, (*cpi).rc.worst_quality);
}
static mut rate_factor_levels: [RATE_FACTOR_LEVEL; 7] = [
    KF_STD as core::ffi::c_int as RATE_FACTOR_LEVEL,
    INTER_NORMAL as core::ffi::c_int as RATE_FACTOR_LEVEL,
    GF_ARF_STD as core::ffi::c_int as RATE_FACTOR_LEVEL,
    GF_ARF_STD as core::ffi::c_int as RATE_FACTOR_LEVEL,
    INTER_NORMAL as core::ffi::c_int as RATE_FACTOR_LEVEL,
    INTER_NORMAL as core::ffi::c_int as RATE_FACTOR_LEVEL,
    GF_ARF_LOW as core::ffi::c_int as RATE_FACTOR_LEVEL,
];
unsafe extern "C-unwind" fn get_rate_factor_level(
    gf_group: *const GF_GROUP,
    mut gf_frame_index: core::ffi::c_int,
) -> RATE_FACTOR_LEVEL {
    let update_type: FRAME_UPDATE_TYPE = (*gf_group)
        .update_type[gf_frame_index as usize];
    return rate_factor_levels[update_type as usize];
}
unsafe extern "C-unwind" fn get_rate_correction_factor(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_double {
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let mut rcf: core::ffi::c_double = 0.;
    let mut rate_correction_factors_kfstd: core::ffi::c_double = 0.;
    let mut rate_correction_factors_gfarfstd: core::ffi::c_double = 0.;
    let mut rate_correction_factors_internormal: core::ffi::c_double = 0.;
    rate_correction_factors_kfstd = if (*(*cpi).ppi)
        .gf_group
        .frame_parallel_level[(*cpi).gf_frame_index as usize] > 0 as core::ffi::c_int
    {
        (*rc).frame_level_rate_correction_factors[KF_STD as core::ffi::c_int as usize]
    } else {
        (*p_rc).rate_correction_factors[KF_STD as core::ffi::c_int as usize]
    };
    rate_correction_factors_gfarfstd = if (*(*cpi).ppi)
        .gf_group
        .frame_parallel_level[(*cpi).gf_frame_index as usize] > 0 as core::ffi::c_int
    {
        (*rc)
            .frame_level_rate_correction_factors[GF_ARF_STD as core::ffi::c_int as usize]
    } else {
        (*p_rc).rate_correction_factors[GF_ARF_STD as core::ffi::c_int as usize]
    };
    rate_correction_factors_internormal = if (*(*cpi).ppi)
        .gf_group
        .frame_parallel_level[(*cpi).gf_frame_index as usize] > 0 as core::ffi::c_int
    {
        (*rc)
            .frame_level_rate_correction_factors[INTER_NORMAL as core::ffi::c_int
            as usize]
    } else {
        (*p_rc).rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize]
    };
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        rcf = rate_correction_factors_kfstd;
    } else if is_stat_consumption_stage(cpi) != 0 {
        let rf_lvl: RATE_FACTOR_LEVEL = get_rate_factor_level(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as RATE_FACTOR_LEVEL;
        let mut rate_correction_factors_rflvl: core::ffi::c_double = if (*(*cpi).ppi)
            .gf_group
            .frame_parallel_level[(*cpi).gf_frame_index as usize] > 0 as core::ffi::c_int
        {
            (*rc).frame_level_rate_correction_factors[rf_lvl as usize]
        } else {
            (*p_rc).rate_correction_factors[rf_lvl as usize]
        };
        rcf = rate_correction_factors_rflvl;
    } else if ((*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
        || (*refresh_frame).golden_frame as core::ffi::c_int != 0)
        && (*rc).is_src_frame_alt_ref == 0 && (*(*cpi).ppi).use_svc == 0
        && ((*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            != AOM_CBR as core::ffi::c_int as core::ffi::c_uint
            || (*cpi).oxcf.rc_cfg.gf_cbr_boost_pct > 20 as core::ffi::c_uint)
    {
        rcf = rate_correction_factors_gfarfstd;
    } else {
        rcf = rate_correction_factors_internormal;
    }
    rcf *= resize_rate_factor(&(*cpi).oxcf.frm_dim_cfg, width, height);
    return fclamp(rcf, MIN_BPB_FACTOR, MAX_BPB_FACTOR as core::ffi::c_double);
}
unsafe extern "C-unwind" fn set_rate_correction_factor(
    mut cpi: *mut AV1_COMP,
    mut is_encode_stage: core::ffi::c_int,
    mut factor: core::ffi::c_double,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let refresh_frame: *const RefreshFrameInfo = &raw mut (*cpi).refresh_frame;
    let mut update_default_rcf: core::ffi::c_int = 1 as core::ffi::c_int;
    factor /= resize_rate_factor(&raw mut (*cpi).oxcf.frm_dim_cfg, width, height);
    factor = fclamp(factor, MIN_BPB_FACTOR, MAX_BPB_FACTOR as core::ffi::c_double);
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        (*p_rc).rate_correction_factors[KF_STD as core::ffi::c_int as usize] = factor;
    } else if is_stat_consumption_stage(cpi) != 0 {
        let rf_lvl: RATE_FACTOR_LEVEL = get_rate_factor_level(
            &raw mut (*(*cpi).ppi).gf_group,
            (*cpi).gf_frame_index as core::ffi::c_int,
        ) as RATE_FACTOR_LEVEL;
        if is_encode_stage != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            (*rc).frame_level_rate_correction_factors[rf_lvl as usize] = factor;
            update_default_rcf = 0 as core::ffi::c_int;
        }
        if update_default_rcf != 0 {
            (*p_rc).rate_correction_factors[rf_lvl as usize] = factor;
        }
    } else if ((*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
        || (*refresh_frame).golden_frame as core::ffi::c_int != 0)
        && (*rc).is_src_frame_alt_ref == 0 && (*(*cpi).ppi).use_svc == 0
        && ((*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            != AOM_CBR as core::ffi::c_int as core::ffi::c_uint
            || (*cpi).oxcf.rc_cfg.gf_cbr_boost_pct > 20 as core::ffi::c_uint)
    {
        (*p_rc).rate_correction_factors[GF_ARF_STD as core::ffi::c_int as usize] = factor;
    } else {
        if is_encode_stage != 0
            && (*(*cpi).ppi)
                .gf_group
                .frame_parallel_level[(*cpi).gf_frame_index as usize]
                > 0 as core::ffi::c_int
        {
            (*rc)
                .frame_level_rate_correction_factors[INTER_NORMAL as core::ffi::c_int
                as usize] = factor;
            update_default_rcf = 0 as core::ffi::c_int;
        }
        if update_default_rcf != 0 {
            (*p_rc).rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize] = factor;
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_update_rate_correction_factors(
    mut cpi: *mut AV1_COMP,
    mut is_encode_stage: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let mut correction_factor: core::ffi::c_double = 1.0f64;
    let mut rate_correction_factor: core::ffi::c_double = get_rate_correction_factor(
        cpi,
        width,
        height,
    );
    let mut adjustment_limit: core::ffi::c_double = 0.;
    let mut projected_size_based_on_q: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut cyclic_refresh_active: core::ffi::c_int = ((*cpi).oxcf.q_cfg.aq_mode
        as core::ffi::c_int == CYCLIC_REFRESH_AQ as core::ffi::c_int
        && (*cpi).common.seg.enabled as core::ffi::c_int != 0) as core::ffi::c_int;
    if (*cpi).rc.is_src_frame_alt_ref != 0 {
        return;
    }
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        == CYCLIC_REFRESH_AQ as core::ffi::c_int
        && (*cpi).sf.rt_sf.overshoot_detection_cbr as core::ffi::c_int
            == FAST_DETECTION_MAXQ as core::ffi::c_int && (*cpi).rc.high_source_sad != 0
        && (*(*cpi).cyclic_refresh).counter_encode_maxq_scene_change
            == 0 as core::ffi::c_int && frame_is_intra_only(cm) == 0
        && (*(*cpi).ppi).use_svc == 0
    {
        (*cpi).rc.q_2_frame = (*cm).quant_params.base_qindex;
        (*cpi).rc.q_1_frame = (*cm).quant_params.base_qindex;
        (*cpi).rc.rc_2_frame = 0 as core::ffi::c_int;
        (*cpi).rc.rc_1_frame = 0 as core::ffi::c_int;
        return;
    }
    if cyclic_refresh_active != 0 {
        projected_size_based_on_q = av1_cyclic_refresh_estimate_bits_at_q(
            cpi,
            rate_correction_factor,
        );
    } else {
        projected_size_based_on_q = av1_estimate_bits_at_q(
            cpi,
            (*cm).quant_params.base_qindex,
            rate_correction_factor,
        );
    }
    if projected_size_based_on_q > FRAME_OVERHEAD_BITS {
        correction_factor = (*cpi).rc.projected_frame_size as core::ffi::c_double
            / projected_size_based_on_q as core::ffi::c_double;
    }
    correction_factor = if correction_factor > 0.25f64 {
        correction_factor
    } else {
        0.25f64
    };
    (*cpi).rc.q_2_frame = (*cpi).rc.q_1_frame;
    (*cpi).rc.q_1_frame = (*cm).quant_params.base_qindex;
    (*cpi).rc.rc_2_frame = (*cpi).rc.rc_1_frame;
    if correction_factor > 1.1f64 {
        (*cpi).rc.rc_1_frame = -(1 as core::ffi::c_int);
    } else if correction_factor < 0.9f64 {
        (*cpi).rc.rc_1_frame = 1 as core::ffi::c_int;
    } else {
        (*cpi).rc.rc_1_frame = 0 as core::ffi::c_int;
    }
    if correction_factor > 0.0f64 {
        if (*cpi).is_screen_content_type != 0 {
            adjustment_limit = 0.25f64
                + 0.5f64
                    * (if 0.5f64 < fabs(log10(correction_factor)) {
                        0.5f64
                    } else {
                        fabs(log10(correction_factor))
                    });
        } else {
            adjustment_limit = 0.25f64
                + 0.75f64
                    * (if 0.5f64 < fabs(log10(correction_factor)) {
                        0.5f64
                    } else {
                        fabs(log10(correction_factor))
                    });
        }
    } else {
        adjustment_limit = 0.75f64;
    }
    if cyclic_refresh_active != 0 && (*cpi).rc.this_frame_target > 0 as core::ffi::c_int
    {
        let cr: *mut CYCLIC_REFRESH = (*cpi).cyclic_refresh;
        if correction_factor > 1.25f64 {
            (*cr).percent_refresh_adjustment = if (*cr).percent_refresh_adjustment
                - 1 as core::ffi::c_int > -(5 as core::ffi::c_int)
            {
                (*cr).percent_refresh_adjustment - 1 as core::ffi::c_int
            } else {
                -(5 as core::ffi::c_int)
            };
            (*cr).rate_ratio_qdelta_adjustment = if (*cr).rate_ratio_qdelta_adjustment
                - 0.05f64 > -0.0f64
            {
                (*cr).rate_ratio_qdelta_adjustment - 0.05f64
            } else {
                -0.0f64
            };
        } else if correction_factor < 0.5f64 {
            (*cr).percent_refresh_adjustment = if ((*cr).percent_refresh_adjustment
                + 1 as core::ffi::c_int) < 5 as core::ffi::c_int
            {
                (*cr).percent_refresh_adjustment + 1 as core::ffi::c_int
            } else {
                5 as core::ffi::c_int
            };
            (*cr).rate_ratio_qdelta_adjustment = if (*cr).rate_ratio_qdelta_adjustment
                + 0.05f64 < 0.25f64
            {
                (*cr).rate_ratio_qdelta_adjustment + 0.05f64
            } else {
                0.25f64
            };
        }
    }
    if correction_factor > 1.01f64 {
        correction_factor = 1.0f64 + (correction_factor - 1.0f64) * adjustment_limit;
        rate_correction_factor = rate_correction_factor * correction_factor;
        if rate_correction_factor > MAX_BPB_FACTOR as core::ffi::c_double {
            rate_correction_factor = MAX_BPB_FACTOR as core::ffi::c_double;
        }
    } else if correction_factor < 0.99f64 {
        correction_factor = 1.0f64 / correction_factor;
        correction_factor = 1.0f64 + (correction_factor - 1.0f64) * adjustment_limit;
        correction_factor = 1.0f64 / correction_factor;
        rate_correction_factor = rate_correction_factor * correction_factor;
        if rate_correction_factor < MIN_BPB_FACTOR {
            rate_correction_factor = MIN_BPB_FACTOR;
        }
    }
    set_rate_correction_factor(
        cpi,
        is_encode_stage,
        rate_correction_factor,
        width,
        height,
    );
}
unsafe extern "C-unwind" fn get_bits_per_mb(
    mut cpi: *const AV1_COMP,
    mut use_cyclic_refresh: core::ffi::c_int,
    mut correction_factor: core::ffi::c_double,
    mut q: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    return if use_cyclic_refresh != 0 {
        av1_cyclic_refresh_rc_bits_per_mb(cpi as *const AV1_COMP, q, correction_factor)
    } else {
        av1_rc_bits_per_mb(
            cpi,
            (*cm).current_frame.frame_type,
            q,
            correction_factor,
            (*cpi).sf.hl_sf.accurate_bit_estimate,
        )
    };
}
unsafe extern "C-unwind" fn find_closest_qindex_by_rate(
    mut desired_bits_per_mb: core::ffi::c_int,
    mut cpi: *const AV1_COMP,
    mut correction_factor: core::ffi::c_double,
    mut best_qindex: core::ffi::c_int,
    mut worst_qindex: core::ffi::c_int,
) -> core::ffi::c_int {
    let use_cyclic_refresh: core::ffi::c_int = ((*cpi).oxcf.q_cfg.aq_mode
        as core::ffi::c_int == CYCLIC_REFRESH_AQ as core::ffi::c_int
        && (*(*cpi).cyclic_refresh).apply_cyclic_refresh != 0) as core::ffi::c_int;
    let mut low: core::ffi::c_int = best_qindex;
    let mut high: core::ffi::c_int = worst_qindex;
    while low < high {
        let mid: core::ffi::c_int = low + high >> 1 as core::ffi::c_int;
        let mid_bits_per_mb: core::ffi::c_int = get_bits_per_mb(
            cpi,
            use_cyclic_refresh,
            correction_factor,
            mid,
        ) as core::ffi::c_int;
        if mid_bits_per_mb > desired_bits_per_mb {
            low = mid + 1 as core::ffi::c_int;
        } else {
            high = mid;
        }
    }
    let curr_q: core::ffi::c_int = low;
    let curr_bits_per_mb: core::ffi::c_int = get_bits_per_mb(
        cpi,
        use_cyclic_refresh,
        correction_factor,
        curr_q,
    ) as core::ffi::c_int;
    let curr_bit_diff: core::ffi::c_int = if curr_bits_per_mb <= desired_bits_per_mb {
        desired_bits_per_mb - curr_bits_per_mb
    } else {
        INT_MAX
    };
    let prev_q: core::ffi::c_int = curr_q - 1 as core::ffi::c_int;
    let mut prev_bit_diff: core::ffi::c_int = 0;
    if curr_bit_diff == INT_MAX || curr_q == best_qindex {
        prev_bit_diff = INT_MAX;
    } else {
        let prev_bits_per_mb: core::ffi::c_int = get_bits_per_mb(
            cpi,
            use_cyclic_refresh,
            correction_factor,
            prev_q,
        ) as core::ffi::c_int;
        prev_bit_diff = prev_bits_per_mb - desired_bits_per_mb;
    }
    return if curr_bit_diff <= prev_bit_diff { curr_q } else { prev_q };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_regulate_q(
    mut cpi: *const AV1_COMP,
    mut target_bits_per_frame: core::ffi::c_int,
    mut active_best_quality: core::ffi::c_int,
    mut active_worst_quality: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    let MBs: core::ffi::c_int = av1_get_MBs(width, height) as core::ffi::c_int;
    let correction_factor: core::ffi::c_double = get_rate_correction_factor(
        cpi,
        width,
        height,
    ) as core::ffi::c_double;
    let target_bits_per_mb: core::ffi::c_int = ((target_bits_per_frame as uint64_t)
        << BPER_MB_NORMBITS)
        .wrapping_div(MBs as uint64_t) as core::ffi::c_int;
    let mut q: core::ffi::c_int = find_closest_qindex_by_rate(
        target_bits_per_mb,
        cpi,
        correction_factor,
        active_best_quality,
        active_worst_quality,
    );
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        && has_no_stats_stage(cpi) != 0
    {
        return adjust_q_cbr(cpi, q, active_worst_quality, width, height);
    }
    return q;
}
unsafe extern "C-unwind" fn get_active_quality(
    mut q: core::ffi::c_int,
    mut gfu_boost: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
    mut low_motion_minq: *mut core::ffi::c_int,
    mut high_motion_minq: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    if gfu_boost > high {
        return *low_motion_minq.offset(q as isize)
    } else if gfu_boost < low {
        return *high_motion_minq.offset(q as isize)
    } else {
        let gap: core::ffi::c_int = high - low;
        let offset: core::ffi::c_int = high - gfu_boost;
        let qdiff: core::ffi::c_int = *high_motion_minq.offset(q as isize)
            - *low_motion_minq.offset(q as isize);
        let adjustment: core::ffi::c_int = (offset * qdiff
            + (gap >> 1 as core::ffi::c_int)) / gap;
        return *low_motion_minq.offset(q as isize) + adjustment;
    };
}
static mut gfboost_thresh: [core::ffi::c_int; 3] = [
    4000 as core::ffi::c_int,
    4000 as core::ffi::c_int,
    3000 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn get_kf_active_quality(
    p_rc: *const PRIMARY_RATE_CONTROL,
    mut q: core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
    res_idx: core::ffi::c_int,
    rtc_mode: bool,
) -> core::ffi::c_int {
    let mut kf_low_motion_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut kf_high_motion_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    match bit_depth as core::ffi::c_uint {
        8 => {
            kf_low_motion_minq = (kf_low_motion_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            kf_low_motion_minq = (kf_low_motion_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            kf_low_motion_minq = (kf_low_motion_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            kf_low_motion_minq = 0 as *mut core::ffi::c_int;
        }
    }
    match bit_depth as core::ffi::c_uint {
        8 => {
            kf_high_motion_minq = (kf_high_motion_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            kf_high_motion_minq = (kf_high_motion_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            kf_high_motion_minq = (kf_high_motion_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            kf_high_motion_minq = 0 as *mut core::ffi::c_int;
        }
    }
    let mut kf_low_local: core::ffi::c_int = if rtc_mode as core::ffi::c_int != 0 {
        kf_low_rtc
    } else {
        kf_low
    };
    let mut kf_high_local: core::ffi::c_int = if rtc_mode as core::ffi::c_int != 0 {
        kf_high_rtc
    } else {
        kf_high
    };
    return get_active_quality(
        q,
        (*p_rc).kf_boost,
        kf_low_local,
        kf_high_local,
        kf_low_motion_minq,
        kf_high_motion_minq,
    );
}
unsafe extern "C-unwind" fn get_gf_active_quality_no_rc(
    p_rc: *const PRIMARY_RATE_CONTROL,
    mut q: core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
    res_idx: core::ffi::c_int,
    rtc_mode: bool,
) -> core::ffi::c_int {
    let mut arfgf_low_motion_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut arfgf_high_motion_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    match bit_depth as core::ffi::c_uint {
        8 => {
            arfgf_low_motion_minq = (arfgf_low_motion_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            arfgf_low_motion_minq = (arfgf_low_motion_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            arfgf_low_motion_minq = (arfgf_low_motion_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            arfgf_low_motion_minq = 0 as *mut core::ffi::c_int;
        }
    }
    match bit_depth as core::ffi::c_uint {
        8 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            arfgf_high_motion_minq = 0 as *mut core::ffi::c_int;
        }
    }
    let mut gf_low_local: core::ffi::c_int = 0;
    let mut gf_high_local: core::ffi::c_int = 0;
    if !rtc_mode {
        gf_low_local = if (*p_rc).gfu_boost_average < gfboost_thresh[res_idx as usize] {
            gf_low_1
        } else {
            gf_low_2
        };
        gf_high_local = if (*p_rc).gfu_boost_average < gfboost_thresh[res_idx as usize] {
            gf_high_1
        } else {
            gf_high_2
        };
    } else {
        gf_low_local = gf_low_rtc;
        gf_high_local = gf_high_rtc;
    }
    return get_active_quality(
        q,
        (*p_rc).gfu_boost,
        gf_low_local,
        gf_high_local,
        arfgf_low_motion_minq,
        arfgf_high_motion_minq,
    );
}
unsafe extern "C-unwind" fn get_gf_active_quality(
    p_rc: *const PRIMARY_RATE_CONTROL,
    mut q: core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
    res_idx: core::ffi::c_int,
    rtc_mode: bool,
) -> core::ffi::c_int {
    return get_gf_active_quality_no_rc(p_rc, q, bit_depth, res_idx, rtc_mode);
}
unsafe extern "C-unwind" fn get_gf_high_motion_quality(
    mut q: core::ffi::c_int,
    mut bit_depth: aom_bit_depth_t,
    res_idx: core::ffi::c_int,
    rtc_mode: bool,
) -> core::ffi::c_int {
    let mut arfgf_high_motion_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    match bit_depth as core::ffi::c_uint {
        8 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            arfgf_high_motion_minq = (arfgf_high_motion_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            arfgf_high_motion_minq = 0 as *mut core::ffi::c_int;
        }
    }
    return *arfgf_high_motion_minq.offset(q as isize);
}
unsafe extern "C-unwind" fn calc_active_worst_quality_no_stats_vbr(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let curr_frame: core::ffi::c_uint = (*cpi).common.current_frame.frame_number;
    let mut active_worst_quality: core::ffi::c_int = 0;
    let mut last_q_key_frame: core::ffi::c_int = 0;
    let mut last_q_inter_frame: core::ffi::c_int = 0;
    last_q_key_frame = (*p_rc).last_q[KEY_FRAME as core::ffi::c_int as usize];
    last_q_inter_frame = (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize];
    if (*cpi).common.current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        active_worst_quality = if curr_frame == 0 as core::ffi::c_uint {
            (*rc).worst_quality
        } else {
            last_q_key_frame * 2 as core::ffi::c_int
        };
    } else if (*rc).is_src_frame_alt_ref == 0
        && ((*refresh_frame).golden_frame as core::ffi::c_int != 0
            || (*refresh_frame).bwd_ref_frame as core::ffi::c_int != 0
            || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0)
    {
        active_worst_quality = if curr_frame == 1 as core::ffi::c_uint {
            last_q_key_frame * 5 as core::ffi::c_int / 4 as core::ffi::c_int
        } else {
            last_q_inter_frame
        };
    } else {
        active_worst_quality = if curr_frame == 1 as core::ffi::c_uint {
            last_q_key_frame * 2 as core::ffi::c_int
        } else {
            last_q_inter_frame * 2 as core::ffi::c_int
        };
    }
    return if active_worst_quality < (*rc).worst_quality {
        active_worst_quality
    } else {
        (*rc).worst_quality
    };
}
unsafe extern "C-unwind" fn calc_active_worst_quality_no_stats_cbr(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let mut p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let svc: *const SVC = &(*cpi).svc;
    let mut num_frames_weight_key: core::ffi::c_uint = (5 as core::ffi::c_int
        * (*cpi).svc.number_temporal_layers) as core::ffi::c_uint;
    let mut critical_level: int64_t = (*p_rc).optimal_buffer_level
        >> 3 as core::ffi::c_int;
    let mut buff_lvl_step: int64_t = 0 as int64_t;
    let mut adjustment: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut active_worst_quality: core::ffi::c_int = 0;
    let mut ambient_qp: core::ffi::c_int = 0;
    if frame_is_intra_only(cm) != 0 {
        if (*cpi).sf.rt_sf.rc_compute_spatial_var_sc_kf != 0
            && (*svc).number_spatial_layers == 1 as core::ffi::c_int
            && (*rc).frame_spatial_variance < 1000 as uint64_t
            && (*p_rc).buffer_level > (*p_rc).optimal_buffer_level
            && (*p_rc).optimal_buffer_level
                > ((*rc).avg_frame_bandwidth << 3 as core::ffi::c_int) as int64_t
            && (*rc).last_encoded_size_keyframe
                < (*rc).last_target_size_keyframe << 3 as core::ffi::c_int
        {
            if (*p_rc).buffer_level
                > 9 as int64_t * (*p_rc).optimal_buffer_level >> 3 as core::ffi::c_int
            {
                ambient_qp = (*p_rc)
                    .avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize];
            } else {
                ambient_qp = (*rc).worst_quality
                    + (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
                    >> 1 as core::ffi::c_int;
            }
            return if (*rc).worst_quality
                < (if ambient_qp > (*rc).best_quality {
                    ambient_qp
                } else {
                    (*rc).best_quality
                })
            {
                (*rc).worst_quality
            } else if ambient_qp > (*rc).best_quality {
                ambient_qp
            } else {
                (*rc).best_quality
            };
        } else {
            return (*rc).worst_quality
        }
    }
    let mut avg_qindex_key: core::ffi::c_int = (*p_rc)
        .avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize];
    if (*svc).number_temporal_layers > 1 as core::ffi::c_int {
        let mut layer: core::ffi::c_int = 0 as core::ffi::c_int
            * (*svc).number_temporal_layers + 0 as core::ffi::c_int;
        let mut lc: *const LAYER_CONTEXT = &raw mut *((*svc).layer_context)
            .offset(layer as isize) as *mut LAYER_CONTEXT;
        let lp_rc: *const PRIMARY_RATE_CONTROL = &(*lc).p_rc;
        avg_qindex_key = if (*lp_rc)
            .avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize]
            < (*lp_rc).last_q[KEY_FRAME as core::ffi::c_int as usize]
        {
            (*lp_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize]
        } else {
            (*lp_rc).last_q[KEY_FRAME as core::ffi::c_int as usize]
        };
    }
    if (*svc).temporal_layer_id > 0 as core::ffi::c_int
        && (*rc).frames_since_key < 2 as core::ffi::c_int * (*svc).number_temporal_layers
    {
        ambient_qp = avg_qindex_key;
    } else {
        ambient_qp = if (*cm).current_frame.frame_number < num_frames_weight_key {
            if (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
                < avg_qindex_key
            {
                (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
            } else {
                avg_qindex_key
            }
        } else {
            (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
        };
    }
    ambient_qp = if (*rc).worst_quality < ambient_qp {
        (*rc).worst_quality
    } else {
        ambient_qp
    };
    if (*p_rc).buffer_level > (*p_rc).optimal_buffer_level {
        let mut max_adjustment_down: core::ffi::c_int = 0;
        if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
            == CYCLIC_REFRESH_AQ as core::ffi::c_int && (*(*cpi).ppi).use_svc == 0
            && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
                == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
        {
            active_worst_quality = if (*rc).worst_quality < ambient_qp {
                (*rc).worst_quality
            } else {
                ambient_qp
            };
            max_adjustment_down = if (4 as core::ffi::c_int)
                < active_worst_quality / 16 as core::ffi::c_int
            {
                4 as core::ffi::c_int
            } else {
                active_worst_quality / 16 as core::ffi::c_int
            };
        } else {
            active_worst_quality = if (*rc).worst_quality
                < ambient_qp * 5 as core::ffi::c_int / 4 as core::ffi::c_int
            {
                (*rc).worst_quality
            } else {
                ambient_qp * 5 as core::ffi::c_int / 4 as core::ffi::c_int
            };
            max_adjustment_down = active_worst_quality / 3 as core::ffi::c_int;
        }
        if max_adjustment_down != 0 {
            buff_lvl_step = ((*p_rc).maximum_buffer_size - (*p_rc).optimal_buffer_level)
                / max_adjustment_down as int64_t;
            if buff_lvl_step != 0 {
                adjustment = (((*p_rc).buffer_level - (*p_rc).optimal_buffer_level)
                    / buff_lvl_step) as core::ffi::c_int;
            }
            active_worst_quality -= adjustment;
        }
    } else if (*p_rc).buffer_level > critical_level {
        active_worst_quality = if (*rc).worst_quality < ambient_qp {
            (*rc).worst_quality
        } else {
            ambient_qp
        };
        if critical_level != 0 {
            buff_lvl_step = (*p_rc).optimal_buffer_level - critical_level;
            if buff_lvl_step != 0 {
                adjustment = (((*rc).worst_quality - ambient_qp) as int64_t
                    * ((*p_rc).optimal_buffer_level - (*p_rc).buffer_level)
                    / buff_lvl_step) as core::ffi::c_int;
            }
            active_worst_quality += adjustment;
        }
    } else {
        active_worst_quality = (*rc).worst_quality;
    }
    return active_worst_quality;
}
unsafe extern "C-unwind" fn calc_active_best_quality_no_stats_cbr(
    mut cpi: *const AV1_COMP,
    mut active_worst_quality: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let current_frame: *const CurrentFrame = &(*cm).current_frame;
    let mut rtc_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth as core::ffi::c_int;
    let mut active_best_quality: core::ffi::c_int = (*rc).best_quality;
    match bit_depth {
        8 => {
            rtc_minq = core::ptr::addr_of_mut!(rtc_minq_8).cast::<core::ffi::c_int>();
        }
        10 => {
            rtc_minq = core::ptr::addr_of_mut!(rtc_minq_10).cast::<core::ffi::c_int>();
        }
        12 => {
            rtc_minq = core::ptr::addr_of_mut!(rtc_minq_12).cast::<core::ffi::c_int>();
        }
        _ => {
            rtc_minq = 0 as *mut core::ffi::c_int;
        }
    }
    let is_608p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 608 as core::ffi::c_int) as core::ffi::c_int;
    let is_480p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 480 as core::ffi::c_int) as core::ffi::c_int;
    let res_idx: core::ffi::c_int = is_480p_or_larger + is_608p_or_larger;
    let rtc_mode: bool = (*cpi).oxcf.mode as core::ffi::c_int
        == REALTIME as core::ffi::c_int;
    if frame_is_intra_only(cm) != 0 {
        if (*p_rc).this_key_frame_forced != 0 {
            let mut qindex: core::ffi::c_int = (*p_rc).last_boosted_qindex;
            let mut last_boosted_q: core::ffi::c_double = av1_convert_qindex_to_q(
                qindex,
                bit_depth as aom_bit_depth_t,
            );
            let mut delta_qindex: core::ffi::c_int = av1_compute_qdelta(
                rc,
                last_boosted_q,
                last_boosted_q * 0.75f64,
                bit_depth as aom_bit_depth_t,
            );
            active_best_quality = if qindex + delta_qindex > (*rc).best_quality {
                qindex + delta_qindex
            } else {
                (*rc).best_quality
            };
        } else if (*current_frame).frame_number > 0 as core::ffi::c_uint {
            let mut q_adj_factor: core::ffi::c_double = 1.0f64;
            let mut q_val: core::ffi::c_double = 0.;
            active_best_quality = get_kf_active_quality(
                p_rc,
                (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize],
                bit_depth as aom_bit_depth_t,
                res_idx,
                rtc_mode,
            );
            if width * height <= 352 as core::ffi::c_int * 288 as core::ffi::c_int {
                q_adj_factor -= 0.25f64;
            }
            q_val = av1_convert_qindex_to_q(
                active_best_quality,
                bit_depth as aom_bit_depth_t,
            );
            active_best_quality
                += av1_compute_qdelta(
                    rc,
                    q_val,
                    q_val * q_adj_factor,
                    bit_depth as aom_bit_depth_t,
                );
        }
    } else if (*rc).is_src_frame_alt_ref == 0 && (*(*cpi).ppi).use_svc == 0
        && (*cpi).oxcf.rc_cfg.gf_cbr_boost_pct != 0
        && ((*refresh_frame).golden_frame as core::ffi::c_int != 0
            || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0)
    {
        let mut q: core::ffi::c_int = active_worst_quality;
        if (*rc).frames_since_key > 1 as core::ffi::c_int
            && (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
                < active_worst_quality
        {
            q = (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize];
        }
        active_best_quality = get_gf_active_quality(
            p_rc,
            q,
            bit_depth as aom_bit_depth_t,
            res_idx,
            rtc_mode,
        );
    } else {
        let mut frame_type: FRAME_TYPE = (if (*current_frame).frame_number
            > 1 as core::ffi::c_uint
        {
            INTER_FRAME as core::ffi::c_int
        } else {
            KEY_FRAME as core::ffi::c_int
        }) as FRAME_TYPE;
        if (*p_rc).avg_frame_qindex[frame_type as usize] < active_worst_quality {
            active_best_quality = *rtc_minq
                .offset((*p_rc).avg_frame_qindex[frame_type as usize] as isize);
        } else {
            active_best_quality = *rtc_minq.offset(active_worst_quality as isize);
        }
    }
    return active_best_quality;
}
unsafe extern "C-unwind" fn rc_pick_q_and_bounds_no_stats_cbr(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut bottom_index: *mut core::ffi::c_int,
    mut top_index: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let current_frame: *const CurrentFrame = &(*cm).current_frame;
    let mut q: core::ffi::c_int = 0;
    let mut active_worst_quality: core::ffi::c_int = calc_active_worst_quality_no_stats_cbr(
        cpi,
    );
    let mut active_best_quality: core::ffi::c_int = calc_active_best_quality_no_stats_cbr(
        cpi,
        active_worst_quality,
        width,
        height,
    );
    active_best_quality = clamp(
        active_best_quality,
        (*rc).best_quality,
        (*rc).worst_quality,
    );
    active_worst_quality = clamp(
        active_worst_quality,
        active_best_quality,
        (*rc).worst_quality,
    );
    *top_index = active_worst_quality;
    *bottom_index = active_best_quality;
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        && (*p_rc).this_key_frame_forced == 0
        && (*current_frame).frame_number != 0 as core::ffi::c_uint
    {
        let mut qdelta: core::ffi::c_int = 0 as core::ffi::c_int;
        qdelta = av1_compute_qdelta_by_rate(
            cpi,
            (*current_frame).frame_type,
            active_worst_quality,
            2.0f64,
        );
        *top_index = active_worst_quality + qdelta;
        *top_index = if *top_index > *bottom_index { *top_index } else { *bottom_index };
    }
    q = av1_rc_regulate_q(
        cpi,
        (*rc).this_frame_target,
        active_best_quality,
        active_worst_quality,
        width,
        height,
    );
    if q > *top_index {
        if (*rc).this_frame_target >= (*rc).max_frame_bandwidth {
            *top_index = q;
        } else {
            q = *top_index;
        }
    }
    return q;
}
unsafe extern "C-unwind" fn gf_group_pyramid_level(
    mut gf_group: *const GF_GROUP,
    mut gf_index: core::ffi::c_int,
) -> core::ffi::c_int {
    return (*gf_group).layer_depth[gf_index as usize];
}
unsafe extern "C-unwind" fn get_active_cq_level(
    mut rc: *const RATE_CONTROL,
    mut p_rc: *const PRIMARY_RATE_CONTROL,
    oxcf: *const AV1EncoderConfig,
    mut intra_only: core::ffi::c_int,
    mut superres_mode: aom_superres_mode,
    mut superres_denom: core::ffi::c_int,
) -> core::ffi::c_int {
    let rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    static mut cq_adjust_threshold: core::ffi::c_double = 0.1f64;
    let mut active_cq_level: core::ffi::c_int = (*rc_cfg).cq_level;
    if (*rc_cfg).mode as core::ffi::c_uint
        == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        || (*rc_cfg).mode as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        if (superres_mode as core::ffi::c_uint
            == AOM_SUPERRES_QTHRESH as core::ffi::c_int as core::ffi::c_uint
            || superres_mode as core::ffi::c_uint
                == AOM_SUPERRES_AUTO as core::ffi::c_int as core::ffi::c_uint)
            && superres_denom != SCALE_NUMERATOR
        {
            let mut mult: core::ffi::c_int = SUPERRES_QADJ_PER_DENOM_KEYFRAME_SOLO;
            if intra_only != 0 && (*rc).frames_to_key <= 1 as core::ffi::c_int {
                mult = 0 as core::ffi::c_int;
            } else if intra_only != 0 {
                mult = SUPERRES_QADJ_PER_DENOM_KEYFRAME;
            } else {
                mult = SUPERRES_QADJ_PER_DENOM_ARFFRAME;
            }
            active_cq_level = if active_cq_level
                - (superres_denom - 8 as core::ffi::c_int) * mult > 0 as core::ffi::c_int
            {
                active_cq_level - (superres_denom - 8 as core::ffi::c_int) * mult
            } else {
                0 as core::ffi::c_int
            };
        }
    }
    if (*rc_cfg).mode as core::ffi::c_uint
        == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        && (*p_rc).total_target_bits > 0 as int64_t
    {
        let x: core::ffi::c_double = (*p_rc).total_actual_bits as core::ffi::c_double
            / (*p_rc).total_target_bits as core::ffi::c_double;
        if x < cq_adjust_threshold {
            active_cq_level = (active_cq_level as core::ffi::c_double * x
                / cq_adjust_threshold) as core::ffi::c_int;
        }
    }
    return active_cq_level;
}
unsafe extern "C-unwind" fn rc_pick_q_and_bounds_no_stats(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut bottom_index: *mut core::ffi::c_int,
    mut top_index: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let current_frame: *const CurrentFrame = &(*cm).current_frame;
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let rc_mode: aom_rc_mode = (*oxcf).rc_cfg.mode;
    let is_608p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 608 as core::ffi::c_int) as core::ffi::c_int;
    let is_480p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 480 as core::ffi::c_int) as core::ffi::c_int;
    let res_idx: core::ffi::c_int = is_480p_or_larger + is_608p_or_larger;
    let rtc_mode: bool = (*cpi).oxcf.mode as core::ffi::c_int
        == REALTIME as core::ffi::c_int;
    let cq_level: core::ffi::c_int = get_active_cq_level(
        rc,
        p_rc,
        oxcf,
        frame_is_intra_only(cm),
        (*cpi).superres_mode,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth as core::ffi::c_int;
    let mut active_best_quality: core::ffi::c_int = 0;
    let mut active_worst_quality: core::ffi::c_int = calc_active_worst_quality_no_stats_vbr(
        cpi,
    );
    let mut q: core::ffi::c_int = 0;
    let mut inter_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    match bit_depth {
        8 => {
            inter_minq = (inter_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            inter_minq = (inter_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            inter_minq = (inter_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            inter_minq = 0 as *mut core::ffi::c_int;
        }
    }
    if frame_is_intra_only(cm) != 0 {
        if rc_mode as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            let qindex: core::ffi::c_int = cq_level;
            let q_val: core::ffi::c_double = av1_convert_qindex_to_q(
                qindex,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_double;
            let delta_qindex: core::ffi::c_int = av1_compute_qdelta(
                rc,
                q_val,
                q_val * 0.25f64,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_int;
            active_best_quality = if qindex + delta_qindex > (*rc).best_quality {
                qindex + delta_qindex
            } else {
                (*rc).best_quality
            };
        } else if (*p_rc).this_key_frame_forced != 0 {
            let mut qindex_0: core::ffi::c_int = (*p_rc).last_boosted_qindex;
            let last_boosted_q: core::ffi::c_double = av1_convert_qindex_to_q(
                qindex_0,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_double;
            let delta_qindex_0: core::ffi::c_int = av1_compute_qdelta(
                rc,
                last_boosted_q,
                last_boosted_q * 0.75f64,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_int;
            active_best_quality = if qindex_0 + delta_qindex_0 > (*rc).best_quality {
                qindex_0 + delta_qindex_0
            } else {
                (*rc).best_quality
            };
        } else {
            let mut q_adj_factor: core::ffi::c_double = 1.0f64;
            active_best_quality = get_kf_active_quality(
                p_rc,
                (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize],
                bit_depth as aom_bit_depth_t,
                res_idx,
                rtc_mode,
            );
            if width * height <= 352 as core::ffi::c_int * 288 as core::ffi::c_int {
                q_adj_factor -= 0.25f64;
            }
            let q_val_0: core::ffi::c_double = av1_convert_qindex_to_q(
                active_best_quality,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_double;
            active_best_quality
                += av1_compute_qdelta(
                    rc,
                    q_val_0,
                    q_val_0 * q_adj_factor,
                    bit_depth as aom_bit_depth_t,
                );
        }
    } else if (*rc).is_src_frame_alt_ref == 0
        && ((*refresh_frame).golden_frame as core::ffi::c_int != 0
            || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0)
    {
        q = if (*rc).frames_since_key > 1 as core::ffi::c_int
            && (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
                < active_worst_quality
        {
            (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
        } else {
            (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize]
        };
        if rc_mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        {
            if q < cq_level {
                q = cq_level;
            }
            active_best_quality = get_gf_active_quality(
                p_rc,
                q,
                bit_depth as aom_bit_depth_t,
                res_idx,
                rtc_mode,
            );
            active_best_quality = active_best_quality * 15 as core::ffi::c_int
                / 16 as core::ffi::c_int;
        } else if rc_mode as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            let qindex_1: core::ffi::c_int = cq_level;
            let q_val_1: core::ffi::c_double = av1_convert_qindex_to_q(
                qindex_1,
                bit_depth as aom_bit_depth_t,
            ) as core::ffi::c_double;
            let delta_qindex_1: core::ffi::c_int = if (*refresh_frame).alt_ref_frame
                as core::ffi::c_int != 0
            {
                av1_compute_qdelta(
                    rc,
                    q_val_1,
                    q_val_1 * 0.40f64,
                    bit_depth as aom_bit_depth_t,
                ) as core::ffi::c_int
            } else {
                av1_compute_qdelta(
                    rc,
                    q_val_1,
                    q_val_1 * 0.50f64,
                    bit_depth as aom_bit_depth_t,
                ) as core::ffi::c_int
            };
            active_best_quality = if qindex_1 + delta_qindex_1 > (*rc).best_quality {
                qindex_1 + delta_qindex_1
            } else {
                (*rc).best_quality
            };
        } else {
            active_best_quality = get_gf_active_quality(
                p_rc,
                q,
                bit_depth as aom_bit_depth_t,
                res_idx,
                rtc_mode,
            );
        }
    } else if rc_mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        let qindex_2: core::ffi::c_int = cq_level;
        let q_val_2: core::ffi::c_double = av1_convert_qindex_to_q(
            qindex_2,
            bit_depth as aom_bit_depth_t,
        ) as core::ffi::c_double;
        let delta_rate: [core::ffi::c_double; 16] = [
            0.50f64,
            1.0f64,
            0.85f64,
            1.0f64,
            0.70f64,
            1.0f64,
            0.85f64,
            1.0f64,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        let delta_qindex_2: core::ffi::c_int = av1_compute_qdelta(
            rc,
            q_val_2,
            q_val_2
                * delta_rate[((*current_frame).frame_number)
                    .wrapping_rem(FIXED_GF_INTERVAL as core::ffi::c_uint) as usize],
            bit_depth as aom_bit_depth_t,
        ) as core::ffi::c_int;
        active_best_quality = if qindex_2 + delta_qindex_2 > (*rc).best_quality {
            qindex_2 + delta_qindex_2
        } else {
            (*rc).best_quality
        };
    } else {
        active_best_quality = if (*current_frame).frame_number > 1 as core::ffi::c_uint {
            *inter_minq
                .offset(
                    (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
                        as isize,
                )
        } else {
            *inter_minq
                .offset(
                    (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize]
                        as isize,
                )
        };
        if rc_mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
            && active_best_quality < cq_level
        {
            active_best_quality = cq_level;
        }
    }
    active_best_quality = clamp(
        active_best_quality,
        (*rc).best_quality,
        (*rc).worst_quality,
    );
    active_worst_quality = clamp(
        active_worst_quality,
        active_best_quality,
        (*rc).worst_quality,
    );
    *top_index = active_worst_quality;
    *bottom_index = active_best_quality;
    let mut qdelta: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        && (*p_rc).this_key_frame_forced == 0
        && (*current_frame).frame_number != 0 as core::ffi::c_uint
    {
        qdelta = av1_compute_qdelta_by_rate(
            cpi,
            (*current_frame).frame_type,
            active_worst_quality,
            2.0f64,
        );
    } else if (*rc).is_src_frame_alt_ref == 0
        && ((*refresh_frame).golden_frame as core::ffi::c_int != 0
            || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0)
    {
        qdelta = av1_compute_qdelta_by_rate(
            cpi,
            (*current_frame).frame_type,
            active_worst_quality,
            1.75f64,
        );
    }
    *top_index = active_worst_quality + qdelta;
    *top_index = if *top_index > *bottom_index { *top_index } else { *bottom_index };
    if rc_mode as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint {
        q = active_best_quality;
    } else if (*current_frame).frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int && (*p_rc).this_key_frame_forced != 0
    {
        q = (*p_rc).last_boosted_qindex;
    } else {
        q = av1_rc_regulate_q(
            cpi,
            (*rc).this_frame_target,
            active_best_quality,
            active_worst_quality,
            width,
            height,
        );
        if q > *top_index {
            if (*rc).this_frame_target >= (*rc).max_frame_bandwidth {
                *top_index = q;
            } else {
                q = *top_index;
            }
        }
    }
    return q;
}
static mut arf_layer_deltas: [core::ffi::c_double; 7] = [
    2.50f64,
    2.00f64,
    1.75f64,
    1.50f64,
    1.25f64,
    1.15f64,
    1.0f64,
];
unsafe extern "C-unwind" fn frame_type_qdelta(
    mut cpi: *const AV1_COMP,
    mut q: core::ffi::c_int,
) -> core::ffi::c_int {
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let rf_lvl: RATE_FACTOR_LEVEL = get_rate_factor_level(
        gf_group,
        (*cpi).gf_frame_index as core::ffi::c_int,
    ) as RATE_FACTOR_LEVEL;
    let frame_type: FRAME_TYPE = (*gf_group).frame_type[(*cpi).gf_frame_index as usize];
    let arf_layer: core::ffi::c_int = if (*gf_group)
        .layer_depth[(*cpi).gf_frame_index as usize] < 6 as core::ffi::c_int
    {
        (*gf_group).layer_depth[(*cpi).gf_frame_index as usize]
    } else {
        6 as core::ffi::c_int
    };
    let rate_factor: core::ffi::c_double = if rf_lvl as core::ffi::c_int
        == INTER_NORMAL as core::ffi::c_int
    {
        1.0f64
    } else {
        arf_layer_deltas[arf_layer as usize]
    };
    return av1_compute_qdelta_by_rate(cpi, frame_type, q, rate_factor);
}
pub const STATIC_MOTION_THRESH: core::ffi::c_int = 95 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_intra_q_and_bounds(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut active_best: *mut core::ffi::c_int,
    mut active_worst: *mut core::ffi::c_int,
    mut cq_level: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let mut active_best_quality: core::ffi::c_int = 0;
    let mut active_worst_quality: core::ffi::c_int = *active_worst;
    let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth as core::ffi::c_int;
    let is_608p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 608 as core::ffi::c_int) as core::ffi::c_int;
    let is_480p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 480 as core::ffi::c_int) as core::ffi::c_int;
    let res_idx: core::ffi::c_int = is_480p_or_larger + is_608p_or_larger;
    let rtc_mode: bool = (*cpi).oxcf.mode as core::ffi::c_int
        == REALTIME as core::ffi::c_int;
    if (*rc).frames_to_key <= 1 as core::ffi::c_int
        && (*oxcf).rc_cfg.mode as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        active_best_quality = cq_level;
        active_worst_quality = cq_level;
    } else if (*p_rc).this_key_frame_forced != 0 {
        let mut last_boosted_q: core::ffi::c_double = 0.;
        let mut delta_qindex: core::ffi::c_int = 0;
        let mut qindex: core::ffi::c_int = 0;
        let mut last_boosted_qindex: core::ffi::c_int = (*p_rc).last_boosted_qindex;
        if is_stat_consumption_stage_twopass(cpi) != 0
            && (*(*cpi).ppi).twopass.last_kfgroup_zeromotion_pct >= STATIC_MOTION_THRESH
        {
            qindex = if (*p_rc).last_kf_qindex < last_boosted_qindex {
                (*p_rc).last_kf_qindex
            } else {
                last_boosted_qindex
            };
            active_best_quality = qindex;
            last_boosted_q = av1_convert_qindex_to_q(
                qindex,
                bit_depth as aom_bit_depth_t,
            );
            delta_qindex = av1_compute_qdelta(
                rc,
                last_boosted_q,
                last_boosted_q * 1.25f64,
                bit_depth as aom_bit_depth_t,
            );
            active_worst_quality = if qindex + delta_qindex < active_worst_quality {
                qindex + delta_qindex
            } else {
                active_worst_quality
            };
        } else {
            qindex = last_boosted_qindex;
            last_boosted_q = av1_convert_qindex_to_q(
                qindex,
                bit_depth as aom_bit_depth_t,
            );
            delta_qindex = av1_compute_qdelta(
                rc,
                last_boosted_q,
                last_boosted_q * 0.50f64,
                bit_depth as aom_bit_depth_t,
            );
            active_best_quality = if qindex + delta_qindex > (*rc).best_quality {
                qindex + delta_qindex
            } else {
                (*rc).best_quality
            };
        }
    } else {
        let mut q_adj_factor: core::ffi::c_double = 1.0f64;
        let mut q_val: core::ffi::c_double = 0.;
        active_best_quality = get_kf_active_quality(
            p_rc,
            active_worst_quality,
            bit_depth as aom_bit_depth_t,
            res_idx,
            rtc_mode,
        );
        if (*cpi).is_screen_content_type != 0 {
            active_best_quality /= 2 as core::ffi::c_int;
        }
        if is_stat_consumption_stage_twopass(cpi) != 0
            && (*(*cpi).ppi).twopass.kf_zeromotion_pct >= STATIC_KF_GROUP_THRESH
        {
            active_best_quality /= 3 as core::ffi::c_int;
        }
        if width * height <= 352 as core::ffi::c_int * 288 as core::ffi::c_int {
            q_adj_factor -= 0.25f64;
        }
        if is_stat_consumption_stage_twopass(cpi) != 0 {
            q_adj_factor
                += 0.05f64
                    - 0.001f64
                        * (*(*cpi).ppi).twopass.kf_zeromotion_pct as core::ffi::c_double;
        }
        q_val = av1_convert_qindex_to_q(
            active_best_quality,
            bit_depth as aom_bit_depth_t,
        );
        active_best_quality
            += av1_compute_qdelta(
                rc,
                q_val,
                q_val * q_adj_factor,
                bit_depth as aom_bit_depth_t,
            );
        if (*oxcf).rc_cfg.mode as core::ffi::c_uint
            == AOM_Q as core::ffi::c_int as core::ffi::c_uint
            && ((*cpi).superres_mode as core::ffi::c_uint
                == AOM_SUPERRES_QTHRESH as core::ffi::c_int as core::ffi::c_uint
                || (*cpi).superres_mode as core::ffi::c_uint
                    == AOM_SUPERRES_AUTO as core::ffi::c_int as core::ffi::c_uint)
            && (*cm).superres_scale_denominator as core::ffi::c_int != SCALE_NUMERATOR
        {
            active_best_quality = if active_best_quality
                - ((*cm).superres_scale_denominator as core::ffi::c_int
                    - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
                > 0 as core::ffi::c_int
            {
                active_best_quality
                    - ((*cm).superres_scale_denominator as core::ffi::c_int
                        - 8 as core::ffi::c_int) * 2 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            };
        }
    }
    *active_best = active_best_quality;
    *active_worst = active_worst_quality;
}
unsafe extern "C-unwind" fn adjust_active_best_and_worst_quality(
    mut cpi: *const AV1_COMP,
    is_intrl_arf_boost: core::ffi::c_int,
    mut active_worst: *mut core::ffi::c_int,
    mut active_best: *mut core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut active_best_quality: core::ffi::c_int = *active_best;
    let mut active_worst_quality: core::ffi::c_int = *active_worst;
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        != AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        active_best_quality -= (*(*cpi).ppi).twopass.extend_minq / 4 as core::ffi::c_int;
        active_worst_quality += (*(*cpi).ppi).twopass.extend_maxq;
    }
    if frame_is_intra_only(cm) == 0 || (*p_rc).this_key_frame_forced == 0
        || (*(*cpi).ppi).twopass.last_kfgroup_zeromotion_pct < STATIC_MOTION_THRESH
    {
        let qdelta: core::ffi::c_int = frame_type_qdelta(cpi, active_worst_quality)
            as core::ffi::c_int;
        active_worst_quality = if active_worst_quality + qdelta > active_best_quality {
            active_worst_quality + qdelta
        } else {
            active_best_quality
        };
    }
    if av1_frame_scaled(cm) != 0 && frame_is_kf_gf_arf(cpi) == 0 {
        let mut qdelta_0: core::ffi::c_int = av1_compute_qdelta_by_rate(
            cpi,
            (*cm).current_frame.frame_type,
            active_best_quality,
            2.0f64,
        );
        active_best_quality = if active_best_quality + qdelta_0 > (*rc).best_quality {
            active_best_quality + qdelta_0
        } else {
            (*rc).best_quality
        };
    }
    active_best_quality = clamp(
        active_best_quality,
        (*rc).best_quality,
        (*rc).worst_quality,
    );
    active_worst_quality = clamp(
        active_worst_quality,
        active_best_quality,
        (*rc).worst_quality,
    );
    *active_best = active_best_quality;
    *active_worst = active_worst_quality;
}
unsafe extern "C-unwind" fn get_q(
    mut cpi: *const AV1_COMP,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    active_worst_quality: core::ffi::c_int,
    active_best_quality: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut q: core::ffi::c_int = 0;
    let mut last_boosted_qindex: core::ffi::c_int = (*p_rc).last_boosted_qindex;
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        || frame_is_intra_only(cm) != 0 && (*p_rc).this_key_frame_forced == 0
            && (*(*cpi).ppi).twopass.kf_zeromotion_pct >= STATIC_KF_GROUP_THRESH
            && (*rc).frames_to_key > 1 as core::ffi::c_int
    {
        q = active_best_quality;
    } else if frame_is_intra_only(cm) != 0 && (*p_rc).this_key_frame_forced != 0 {
        if (*(*cpi).ppi).twopass.last_kfgroup_zeromotion_pct >= STATIC_MOTION_THRESH {
            q = if (*p_rc).last_kf_qindex < last_boosted_qindex {
                (*p_rc).last_kf_qindex
            } else {
                last_boosted_qindex
            };
        } else {
            q = if last_boosted_qindex
                < (active_best_quality + active_worst_quality) / 2 as core::ffi::c_int
            {
                last_boosted_qindex
            } else {
                (active_best_quality + active_worst_quality) / 2 as core::ffi::c_int
            };
        }
        q = clamp(q, active_best_quality, active_worst_quality);
    } else {
        q = av1_rc_regulate_q(
            cpi,
            (*rc).this_frame_target,
            active_best_quality,
            active_worst_quality,
            width,
            height,
        );
        if q > active_worst_quality {
            if (*rc).this_frame_target < (*rc).max_frame_bandwidth {
                q = active_worst_quality;
            }
        }
        q = if q > active_best_quality { q } else { active_best_quality };
    }
    return q;
}
unsafe extern "C-unwind" fn get_active_best_quality(
    cpi: *const AV1_COMP,
    active_worst_quality: core::ffi::c_int,
    cq_level: core::ffi::c_int,
    gf_index: core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let bit_depth: core::ffi::c_int = (*(*cm).seq_params).bit_depth as core::ffi::c_int;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let mut gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let rc_mode: aom_rc_mode = (*oxcf).rc_cfg.mode;
    let mut active_best_quality: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut update_type: FRAME_UPDATE_TYPE = (*gf_group).update_type[gf_index as usize];
    let is_intrl_arf_boost: core::ffi::c_int = (update_type as core::ffi::c_int
        == INTNL_ARF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
    let mut is_leaf_frame: core::ffi::c_int = !(update_type as core::ffi::c_int
        == ARF_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int
        || is_intrl_arf_boost != 0) as core::ffi::c_int;
    let is_608p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 608 as core::ffi::c_int) as core::ffi::c_int;
    let is_480p_or_larger: core::ffi::c_int = ((if (*cm).width < (*cm).height {
        (*cm).width
    } else {
        (*cm).height
    }) >= 480 as core::ffi::c_int) as core::ffi::c_int;
    let res_idx: core::ffi::c_int = is_480p_or_larger + is_608p_or_larger;
    let rtc_mode: bool = (*cpi).oxcf.mode as core::ffi::c_int
        == REALTIME as core::ffi::c_int;
    let mut inter_minq: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    match bit_depth {
        8 => {
            inter_minq = (inter_minq_8[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        10 => {
            inter_minq = (inter_minq_10[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        12 => {
            inter_minq = (inter_minq_12[rtc_mode
                as usize][(res_idx > 1 as core::ffi::c_int) as core::ffi::c_int
                as usize])
                .as_mut_ptr();
        }
        _ => {
            inter_minq = 0 as *mut core::ffi::c_int;
        }
    }
    if (*cm).tiles.large_scale != 0 {
        is_leaf_frame = !((*refresh_frame).golden_frame as core::ffi::c_int != 0
            || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
            || is_intrl_arf_boost != 0) as core::ffi::c_int;
    }
    let is_overlay_frame: core::ffi::c_int = (update_type as core::ffi::c_int
        == OVERLAY_UPDATE as core::ffi::c_int
        || update_type as core::ffi::c_int == INTNL_OVERLAY_UPDATE as core::ffi::c_int)
        as core::ffi::c_int;
    if is_leaf_frame != 0 || is_overlay_frame != 0 {
        if rc_mode as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            return cq_level;
        }
        active_best_quality = *inter_minq.offset(active_worst_quality as isize);
        if rc_mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
            && active_best_quality < cq_level
        {
            active_best_quality = cq_level;
        }
        return active_best_quality;
    }
    let mut q: core::ffi::c_int = active_worst_quality;
    if (*rc).frames_since_key > 1 as core::ffi::c_int
        && (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
            < active_worst_quality
    {
        q = (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize];
    }
    if rc_mode as core::ffi::c_uint == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
        && q < cq_level
    {
        q = cq_level;
    }
    active_best_quality = get_gf_active_quality(
        p_rc,
        q,
        bit_depth as aom_bit_depth_t,
        res_idx,
        rtc_mode,
    );
    if rc_mode as core::ffi::c_uint == AOM_CQ as core::ffi::c_int as core::ffi::c_uint {
        active_best_quality = active_best_quality * 15 as core::ffi::c_int
            / 16 as core::ffi::c_int;
    }
    let min_boost: core::ffi::c_int = get_gf_high_motion_quality(
        q,
        bit_depth as aom_bit_depth_t,
        res_idx,
        rtc_mode,
    ) as core::ffi::c_int;
    let boost: core::ffi::c_int = min_boost - active_best_quality;
    active_best_quality = min_boost
        - (boost as float_t * (*p_rc).arf_boost_factor) as core::ffi::c_int;
    if is_intrl_arf_boost == 0 {
        return active_best_quality;
    }
    if rc_mode as core::ffi::c_uint == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        || rc_mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
    {
        active_best_quality = (*p_rc).arf_q;
    }
    let mut this_height: core::ffi::c_int = gf_group_pyramid_level(gf_group, gf_index);
    while this_height > 1 as core::ffi::c_int {
        active_best_quality = (active_best_quality + active_worst_quality
            + 1 as core::ffi::c_int) / 2 as core::ffi::c_int;
        this_height -= 1;
    }
    return active_best_quality;
}
unsafe extern "C-unwind" fn rc_pick_q_and_bounds_q_mode(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut gf_index: core::ffi::c_int,
    mut bottom_index: *mut core::ffi::c_int,
    mut top_index: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let cq_level: core::ffi::c_int = get_active_cq_level(
        rc,
        p_rc,
        oxcf,
        frame_is_intra_only(cm),
        (*cpi).superres_mode,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut active_best_quality: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut active_worst_quality: core::ffi::c_int = (*rc).active_worst_quality;
    let mut q: core::ffi::c_int = 0;
    if frame_is_intra_only(cm) != 0 {
        get_intra_q_and_bounds(
            cpi,
            width,
            height,
            &raw mut active_best_quality,
            &raw mut active_worst_quality,
            cq_level,
        );
    } else {
        active_best_quality = get_active_best_quality(
            cpi,
            active_worst_quality,
            cq_level,
            gf_index,
        );
    }
    if cq_level > 0 as core::ffi::c_int {
        active_best_quality = if 1 as core::ffi::c_int > active_best_quality {
            1 as core::ffi::c_int
        } else {
            active_best_quality
        };
    }
    *top_index = clamp(active_worst_quality, (*rc).best_quality, (*rc).worst_quality);
    *bottom_index = clamp(active_best_quality, (*rc).best_quality, (*rc).worst_quality);
    q = *bottom_index;
    return q;
}
unsafe extern "C-unwind" fn rc_pick_q_and_bounds(
    mut cpi: *const AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut gf_index: core::ffi::c_int,
    mut bottom_index: *mut core::ffi::c_int,
    mut top_index: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let mut gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let cq_level: core::ffi::c_int = get_active_cq_level(
        rc,
        p_rc,
        oxcf,
        frame_is_intra_only(cm),
        (*cpi).superres_mode,
        (*cm).superres_scale_denominator as core::ffi::c_int,
    ) as core::ffi::c_int;
    if (*oxcf).rc_cfg.mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        return rc_pick_q_and_bounds_q_mode(
            cpi,
            width,
            height,
            gf_index,
            bottom_index,
            top_index,
        );
    }
    let mut active_best_quality: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut active_worst_quality: core::ffi::c_int = (*rc).active_worst_quality;
    let mut q: core::ffi::c_int = 0;
    let is_intrl_arf_boost: core::ffi::c_int = ((*gf_group)
        .update_type[gf_index as usize] as core::ffi::c_int
        == INTNL_ARF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
    if frame_is_intra_only(cm) != 0 {
        get_intra_q_and_bounds(
            cpi,
            width,
            height,
            &raw mut active_best_quality,
            &raw mut active_worst_quality,
            cq_level,
        );
    } else {
        let pyramid_level: core::ffi::c_int = gf_group_pyramid_level(gf_group, gf_index)
            as core::ffi::c_int;
        if pyramid_level <= 1 as core::ffi::c_int || pyramid_level > MAX_ARF_LAYERS {
            active_best_quality = get_active_best_quality(
                cpi,
                active_worst_quality,
                cq_level,
                gf_index,
            );
        } else {
            active_best_quality = (*p_rc)
                .active_best_quality[(pyramid_level - 1 as core::ffi::c_int) as usize]
                + 1 as core::ffi::c_int;
            active_best_quality = if active_best_quality < active_worst_quality {
                active_best_quality
            } else {
                active_worst_quality
            };
            active_best_quality
                += (active_worst_quality - active_best_quality) / 2 as core::ffi::c_int;
        }
        if (*rc).is_src_frame_alt_ref == 0
            && ((*refresh_frame).golden_frame as core::ffi::c_int != 0
                || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
                || is_intrl_arf_boost != 0)
        {
            active_worst_quality = (active_best_quality
                + 3 as core::ffi::c_int * active_worst_quality + 2 as core::ffi::c_int)
                / 4 as core::ffi::c_int;
        }
    }
    adjust_active_best_and_worst_quality(
        cpi,
        is_intrl_arf_boost,
        &raw mut active_worst_quality,
        &raw mut active_best_quality,
    );
    q = get_q(cpi, width, height, active_worst_quality, active_best_quality);
    if (*rc).this_frame_target >= (*rc).max_frame_bandwidth && q > active_worst_quality {
        active_worst_quality = q;
    }
    *top_index = active_worst_quality;
    *bottom_index = active_best_quality;
    return q;
}
unsafe extern "C-unwind" fn rc_compute_variance_onepass_rt(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let unscaled_src: *const YV12_BUFFER_CONFIG = (*cpi).unscaled_source;
    if unscaled_src.is_null() {
        return;
    }
    let mut src_y: *const uint8_t = (*unscaled_src)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .y_buffer;
    let src_ystride: core::ffi::c_int = (*unscaled_src)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    let mut yv12: *const YV12_BUFFER_CONFIG = get_ref_frame_yv12_buf(
        cm,
        LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    );
    let mut pre_y: *const uint8_t = (*yv12)
        .c2rust_unnamed_4
        .buffers[0 as core::ffi::c_int as usize];
    let pre_ystride: core::ffi::c_int = (*yv12)
        .c2rust_unnamed_3
        .strides[0 as core::ffi::c_int as usize];
    if !((*cpi)
        .scaled_ref_buf[(LAST_FRAME as core::ffi::c_int - 1 as core::ffi::c_int)
        as usize])
        .is_null()
    {
        return;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        if (*unscaled_src).c2rust_unnamed.widths[i as usize]
            != (*yv12).c2rust_unnamed.widths[i as usize]
            || (*unscaled_src).c2rust_unnamed_0.heights[i as usize]
                != (*yv12).c2rust_unnamed_0.heights[i as usize]
        {
            return;
        }
        i += 1;
    }
    let num_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let num_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    let bsize: BLOCK_SIZE = BLOCK_64X64;
    let mut num_samples: core::ffi::c_int = 0 as core::ffi::c_int;
    let sb_size_by_mb: core::ffi::c_int = if (*(*cm).seq_params).sb_size
        as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
    {
        (*(*cm).seq_params).mib_size >> 1 as core::ffi::c_int
    } else {
        (*(*cm).seq_params).mib_size
    };
    let sb_cols: core::ffi::c_int = (num_mi_cols + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let sb_rows: core::ffi::c_int = (num_mi_rows + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let mut fsse: uint64_t = 0 as uint64_t;
    (*cpi).rec_sse = 0 as uint64_t;
    let mut sbi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while sbi_row < sb_rows {
        let mut sbi_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while sbi_col < sb_cols {
            let mut sse: core::ffi::c_uint = 0;
            let mut src: [uint8_t; 4096] = [
                0 as core::ffi::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
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
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < 64 as core::ffi::c_int {
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < 64 as core::ffi::c_int {
                    let avg: core::ffi::c_uint = aom_avg_4x4_c(
                        src_y.offset((i_0 * src_ystride) as isize).offset(j as isize),
                        src_ystride,
                    ) as core::ffi::c_uint;
                    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m < 4 as core::ffi::c_int {
                        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
                        while n < 4 as core::ffi::c_int {
                            src[(i_0 * 64 as core::ffi::c_int + j
                                + m * 64 as core::ffi::c_int + n) as usize] = avg
                                as uint8_t;
                            n += 1;
                        }
                        m += 1;
                    }
                    j += 4 as core::ffi::c_int;
                }
                i_0 += 4 as core::ffi::c_int;
            }
            ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
                .expect(
                    "non-null function pointer",
                )(
                src.as_mut_ptr(),
                64 as core::ffi::c_int,
                pre_y,
                pre_ystride,
                &raw mut sse,
            );
            fsse = fsse.wrapping_add(sse as uint64_t);
            num_samples += 1;
            src_y = src_y.offset(64 as core::ffi::c_int as isize);
            pre_y = pre_y.offset(64 as core::ffi::c_int as isize);
            sbi_col += 1;
        }
        src_y = src_y
            .offset(
                ((src_ystride << 6 as core::ffi::c_int)
                    - (sb_cols << 6 as core::ffi::c_int)) as isize,
            );
        pre_y = pre_y
            .offset(
                ((pre_ystride << 6 as core::ffi::c_int)
                    - (sb_cols << 6 as core::ffi::c_int)) as isize,
            );
        sbi_row += 1;
    }
    if num_samples > 0 as core::ffi::c_int {
        (*cpi).rec_sse = if fsse > 0 as uint64_t { fsse } else { 1 as uint64_t };
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_pick_q_and_bounds(
    mut cpi: *mut AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut gf_index: core::ffi::c_int,
    mut bottom_index: *mut core::ffi::c_int,
    mut top_index: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut q: core::ffi::c_int = 0;
    let mut gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    if ((*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        != AOM_Q as core::ffi::c_int as core::ffi::c_uint
        || (*gf_group).update_type[gf_index as usize] as core::ffi::c_int
            == ARF_UPDATE as core::ffi::c_int) && has_no_stats_stage(cpi) != 0
    {
        if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        {
            (*cpi).rec_sse = UINT64_MAX as uint64_t;
            if (*cpi).sf.hl_sf.accurate_bit_estimate != 0
                && (*cpi).common.current_frame.frame_type as core::ffi::c_int
                    != KEY_FRAME as core::ffi::c_int
            {
                rc_compute_variance_onepass_rt(cpi);
            }
            q = rc_pick_q_and_bounds_no_stats_cbr(
                cpi,
                width,
                height,
                bottom_index,
                top_index,
            );
            (*cpi).rc.active_worst_quality = *top_index;
        } else {
            q = rc_pick_q_and_bounds_no_stats(
                cpi,
                width,
                height,
                bottom_index,
                top_index,
            );
        }
    } else {
        q = rc_pick_q_and_bounds(cpi, width, height, gf_index, bottom_index, top_index);
    }
    if (*gf_group).update_type[gf_index as usize] as core::ffi::c_int
        == ARF_UPDATE as core::ffi::c_int
    {
        (*p_rc).arf_q = q;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_compute_frame_size_bounds(
    mut cpi: *const AV1_COMP,
    mut frame_target: core::ffi::c_int,
    mut frame_under_shoot_limit: *mut core::ffi::c_int,
    mut frame_over_shoot_limit: *mut core::ffi::c_int,
) {
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        *frame_under_shoot_limit = 0 as core::ffi::c_int;
        *frame_over_shoot_limit = INT_MAX;
    } else {
        let tolerance: core::ffi::c_int = (if 100 as int64_t
            > (*cpi).sf.hl_sf.recode_tolerance as int64_t * frame_target as int64_t
                / 100 as int64_t
        {
            100 as int64_t
        } else {
            (*cpi).sf.hl_sf.recode_tolerance as int64_t * frame_target as int64_t
                / 100 as int64_t
        }) as core::ffi::c_int;
        *frame_under_shoot_limit = if frame_target - tolerance > 0 as core::ffi::c_int {
            frame_target - tolerance
        } else {
            0 as core::ffi::c_int
        };
        *frame_over_shoot_limit = (if (frame_target as int64_t + tolerance as int64_t)
            < (*cpi).rc.max_frame_bandwidth as int64_t
        {
            frame_target as int64_t + tolerance as int64_t
        } else {
            (*cpi).rc.max_frame_bandwidth as int64_t
        }) as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_set_frame_target(
    mut cpi: *mut AV1_COMP,
    mut target: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    (*rc).this_frame_target = target;
    if av1_frame_scaled(cm) != 0
        && (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            != AOM_CBR as core::ffi::c_int as core::ffi::c_uint
    {
        (*rc).this_frame_target = saturate_cast_double_to_int(
            (*rc).this_frame_target as core::ffi::c_double
                * resize_rate_factor(&raw mut (*cpi).oxcf.frm_dim_cfg, width, height),
        );
    }
    let sb64_target_rate: int64_t = (((*rc).this_frame_target as int64_t)
        << 12 as core::ffi::c_int) / (width * height) as int64_t;
    (*rc).sb64_target_rate = (if sb64_target_rate < 2147483647 as int64_t {
        sb64_target_rate
    } else {
        2147483647 as int64_t
    }) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn update_alt_ref_frame_stats(mut cpi: *mut AV1_COMP) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    (*rc).frames_since_golden = 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn update_golden_frame_stats(mut cpi: *mut AV1_COMP) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    if (*cpi).refresh_frame.golden_frame as core::ffi::c_int != 0
        || (*rc).is_src_frame_alt_ref != 0
    {
        (*rc).frames_since_golden = 0 as core::ffi::c_int;
    } else if (*cpi).common.show_frame != 0 {
        (*rc).frames_since_golden += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_postencode_update(
    mut cpi: *mut AV1_COMP,
    mut bytes_used: uint64_t,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let current_frame: *const CurrentFrame = &(*cm).current_frame;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let gf_group: *const GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let refresh_frame: *const RefreshFrameInfo = &raw mut (*cpi).refresh_frame;
    let is_intrnl_arf: core::ffi::c_int = ((*gf_group)
        .update_type[(*cpi).gf_frame_index as usize] as core::ffi::c_int
        == INTNL_ARF_UPDATE as core::ffi::c_int) as core::ffi::c_int;
    let qindex: core::ffi::c_int = (*cm).quant_params.base_qindex;
    (*rc).projected_frame_size = (bytes_used << 3 as core::ffi::c_int)
        as core::ffi::c_int;
    av1_rc_update_rate_correction_factors(
        cpi,
        0 as core::ffi::c_int,
        (*cm).width,
        (*cm).height,
    );
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        && (*cm).current_frame.frame_type as core::ffi::c_int
            != KEY_FRAME as core::ffi::c_int
        && (*cpi).sf.hl_sf.accurate_bit_estimate != 0
    {
        let q: core::ffi::c_double = av1_convert_qindex_to_q(
            (*cm).quant_params.base_qindex,
            (*(*cm).seq_params).bit_depth,
        ) as core::ffi::c_double;
        let this_bit_est_ratio: core::ffi::c_int = ((*rc).projected_frame_size
            as core::ffi::c_double * q / sqrt((*cpi).rec_sse as core::ffi::c_double))
            as core::ffi::c_int;
        (*cpi).rc.bit_est_ratio = if (*cpi).rc.bit_est_ratio == 0 as core::ffi::c_int {
            this_bit_est_ratio
        } else {
            (7 as core::ffi::c_int * (*cpi).rc.bit_est_ratio + this_bit_est_ratio)
                / 8 as core::ffi::c_int
        };
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        (*p_rc).last_q[KEY_FRAME as core::ffi::c_int as usize] = qindex;
        (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize] = 3
            as core::ffi::c_int
            * (*p_rc).avg_frame_qindex[KEY_FRAME as core::ffi::c_int as usize] + qindex
            + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 2 as core::ffi::c_int;
        if (*cpi).svc.spatial_layer_id == 0 as core::ffi::c_int {
            (*rc).last_encoded_size_keyframe = (*rc).projected_frame_size;
            (*rc).last_target_size_keyframe = (*rc).this_frame_target;
        }
    } else if (*(*cpi).ppi).use_svc != 0
        && (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).rc.rtc_external_ratectrl != 0
        || (*rc).is_src_frame_alt_ref == 0
            && !((*refresh_frame).golden_frame as core::ffi::c_int != 0
                || is_intrnl_arf != 0
                || (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0)
    {
        (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize] = qindex;
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = 3
            as core::ffi::c_int
            * (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] + qindex
            + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 2 as core::ffi::c_int;
        (*p_rc).ni_frames += 1;
        (*p_rc).tot_q += av1_convert_qindex_to_q(qindex, (*(*cm).seq_params).bit_depth);
        (*p_rc).avg_q = (*p_rc).tot_q / (*p_rc).ni_frames as core::ffi::c_double;
        (*rc).ni_tot_qi += qindex;
        (*rc).ni_av_qi = (*rc).ni_tot_qi / (*p_rc).ni_frames;
    }
    if qindex < (*p_rc).last_boosted_qindex
        || (*current_frame).frame_type as core::ffi::c_int
            == KEY_FRAME as core::ffi::c_int
        || (*p_rc).constrained_gf_group == 0
            && ((*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
                || is_intrnl_arf != 0
                || (*refresh_frame).golden_frame as core::ffi::c_int != 0
                    && (*rc).is_src_frame_alt_ref == 0)
    {
        (*p_rc).last_boosted_qindex = qindex;
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        (*p_rc).last_kf_qindex = qindex;
    }
    update_buffer_level(cpi, (*rc).projected_frame_size);
    (*rc).prev_avg_frame_bandwidth = (*rc).avg_frame_bandwidth;
    if av1_frame_scaled(cm) != 0 {
        (*rc).this_frame_target = saturate_cast_double_to_int(
            (*rc).this_frame_target as core::ffi::c_double
                / resize_rate_factor(
                    &raw mut (*cpi).oxcf.frm_dim_cfg,
                    (*cm).width,
                    (*cm).height,
                ),
        );
    }
    if (*current_frame).frame_type as core::ffi::c_int != KEY_FRAME as core::ffi::c_int {
        (*p_rc).rolling_target_bits = ((*p_rc).rolling_target_bits as int64_t
            * 3 as int64_t + (*rc).this_frame_target as int64_t
            + ((1 as core::ffi::c_int as int64_t) << 2 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int) as core::ffi::c_int;
        (*p_rc).rolling_actual_bits = ((*p_rc).rolling_actual_bits as int64_t
            * 3 as int64_t + (*rc).projected_frame_size as int64_t
            + ((1 as core::ffi::c_int as int64_t) << 2 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int) as core::ffi::c_int;
    }
    (*p_rc).total_actual_bits += (*rc).projected_frame_size as int64_t;
    (*p_rc).total_target_bits
        += (if (*cm).show_frame != 0 {
            (*rc).avg_frame_bandwidth
        } else {
            0 as core::ffi::c_int
        }) as int64_t;
    if is_altref_enabled(
        (*cpi).oxcf.gf_cfg.lag_in_frames,
        (*cpi).oxcf.gf_cfg.enable_auto_arf,
    ) != 0 && (*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
        && ((*current_frame).frame_type as core::ffi::c_int
            != KEY_FRAME as core::ffi::c_int && frame_is_sframe(cm) == 0)
    {
        update_alt_ref_frame_stats(cpi);
    } else {
        update_golden_frame_stats(cpi);
    }
    if (*current_frame).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
        (*rc).frames_since_key = 0 as core::ffi::c_int;
        (*rc).frames_since_scene_change = 0 as core::ffi::c_int;
    }
    if (*cpi).refresh_frame.golden_frame {
        (*rc).frame_num_last_gf_refresh = (*current_frame).frame_number
            as core::ffi::c_int;
    }
    if (*rc).frame_source_sad < 10000 as uint64_t {
        (*rc).last_frame_low_source_sad = (*rc).frame_number_encoded;
    }
    (*rc).prev_coded_width = (*cm).width;
    (*rc).prev_coded_height = (*cm).height;
    (*rc).frame_number_encoded = ((*rc).frame_number_encoded).wrapping_add(1);
    (*rc).prev_frame_is_dropped = 0 as core::ffi::c_int;
    (*rc).drop_count_consec = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_postencode_update_drop_frame(mut cpi: *mut AV1_COMP) {
    update_buffer_level(cpi, 0 as core::ffi::c_int);
    (*cpi).rc.rc_2_frame = 0 as core::ffi::c_int;
    (*cpi).rc.rc_1_frame = 0 as core::ffi::c_int;
    (*cpi).rc.prev_avg_frame_bandwidth = (*cpi).rc.avg_frame_bandwidth;
    (*cpi).rc.prev_coded_width = (*cpi).common.width;
    (*cpi).rc.prev_coded_height = (*cpi).common.height;
    (*cpi).rc.prev_frame_is_dropped = 1 as core::ffi::c_int;
    if (*cpi).rc.high_source_sad != 0 {
        (*cpi).rc.avg_source_sad = 0 as uint64_t;
    }
    if (*(*cpi).ppi).use_svc != 0
        && (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int
    {
        (*cpi).svc.last_layer_dropped[(*cpi).svc.spatial_layer_id as usize] = true_0
            != 0;
        (*cpi).svc.drop_spatial_layer[(*cpi).svc.spatial_layer_id as usize] = true_0
            != 0;
    }
    if (*cpi).svc.spatial_layer_id
        == (*cpi).svc.number_spatial_layers - 1 as core::ffi::c_int
    {
        (*cpi).svc.prev_number_spatial_layers = (*cpi).svc.number_spatial_layers;
    }
    (*cpi).svc.prev_number_temporal_layers = (*cpi).svc.number_temporal_layers;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_qindex(
    mut desired_q: core::ffi::c_double,
    mut bit_depth: aom_bit_depth_t,
    mut best_qindex: core::ffi::c_int,
    mut worst_qindex: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut low: core::ffi::c_int = best_qindex;
    let mut high: core::ffi::c_int = worst_qindex;
    while low < high {
        let mid: core::ffi::c_int = low + high >> 1 as core::ffi::c_int;
        let mid_q: core::ffi::c_double = av1_convert_qindex_to_q(mid, bit_depth)
            as core::ffi::c_double;
        if mid_q < desired_q {
            low = mid + 1 as core::ffi::c_int;
        } else {
            high = mid;
        }
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_qdelta(
    mut rc: *const RATE_CONTROL,
    mut qstart: core::ffi::c_double,
    mut qtarget: core::ffi::c_double,
    mut bit_depth: aom_bit_depth_t,
) -> core::ffi::c_int {
    let start_index: core::ffi::c_int = av1_find_qindex(
        qstart,
        bit_depth,
        (*rc).best_quality,
        (*rc).worst_quality,
    ) as core::ffi::c_int;
    let target_index: core::ffi::c_int = av1_find_qindex(
        qtarget,
        bit_depth,
        (*rc).best_quality,
        (*rc).worst_quality,
    ) as core::ffi::c_int;
    return target_index - start_index;
}
unsafe extern "C-unwind" fn find_qindex_by_rate(
    cpi: *const AV1_COMP,
    mut desired_bits_per_mb: core::ffi::c_int,
    mut frame_type: FRAME_TYPE,
    mut best_qindex: core::ffi::c_int,
    mut worst_qindex: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut low: core::ffi::c_int = best_qindex;
    let mut high: core::ffi::c_int = worst_qindex;
    while low < high {
        let mid: core::ffi::c_int = low + high >> 1 as core::ffi::c_int;
        let mid_bits_per_mb: core::ffi::c_int = av1_rc_bits_per_mb(
            cpi,
            frame_type,
            mid,
            1.0f64,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if mid_bits_per_mb > desired_bits_per_mb {
            low = mid + 1 as core::ffi::c_int;
        } else {
            high = mid;
        }
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_qdelta_by_rate(
    mut cpi: *const AV1_COMP,
    mut frame_type: FRAME_TYPE,
    mut qindex: core::ffi::c_int,
    mut rate_target_ratio: core::ffi::c_double,
) -> core::ffi::c_int {
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let base_bits_per_mb: core::ffi::c_int = av1_rc_bits_per_mb(
        cpi,
        frame_type,
        qindex,
        1.0f64,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let target_bits_per_mb: core::ffi::c_int = (rate_target_ratio
        * base_bits_per_mb as core::ffi::c_double) as core::ffi::c_int;
    let target_index: core::ffi::c_int = find_qindex_by_rate(
        cpi,
        target_bits_per_mb,
        frame_type,
        (*rc).best_quality,
        (*rc).worst_quality,
    ) as core::ffi::c_int;
    return target_index - qindex;
}
unsafe extern "C-unwind" fn set_gf_interval_range(cpi: *const AV1_COMP, rc: *mut RATE_CONTROL) {
    let oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    (*rc).max_gf_interval = (*oxcf).gf_cfg.max_gf_interval;
    (*rc).min_gf_interval = (*oxcf).gf_cfg.min_gf_interval;
    if (*rc).min_gf_interval == 0 as core::ffi::c_int {
        (*rc).min_gf_interval = av1_rc_get_default_min_gf_interval(
            (*oxcf).frm_dim_cfg.width,
            (*oxcf).frm_dim_cfg.height,
            (*cpi).framerate,
        );
    }
    if (*rc).max_gf_interval == 0 as core::ffi::c_int {
        (*rc).max_gf_interval = get_default_max_gf_interval(
            (*cpi).framerate,
            (*rc).min_gf_interval,
        );
    }
    if (*(*cpi).ppi).lap_enabled != 0 {
        (*rc).static_scene_max_gf_interval = (*rc).max_gf_interval
            + 1 as core::ffi::c_int;
    } else {
        (*rc).static_scene_max_gf_interval = MAX_STATIC_GF_GROUP_LENGTH;
    }
    if (*rc).max_gf_interval > (*rc).static_scene_max_gf_interval {
        (*rc).max_gf_interval = (*rc).static_scene_max_gf_interval;
    }
    (*rc).min_gf_interval = if (*rc).min_gf_interval < (*rc).max_gf_interval {
        (*rc).min_gf_interval
    } else {
        (*rc).max_gf_interval
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_update_framerate(
    mut cpi: *mut AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let oxcf: *const AV1EncoderConfig = &raw mut (*cpi).oxcf;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let MBs: core::ffi::c_int = av1_get_MBs(width, height) as core::ffi::c_int;
    (*rc).avg_frame_bandwidth = saturate_cast_double_to_int(
        round((*oxcf).rc_cfg.target_bandwidth as core::ffi::c_double / (*cpi).framerate),
    );
    let mut vbr_min_bits: int64_t = (*rc).avg_frame_bandwidth as int64_t
        * (*oxcf).rc_cfg.vbrmin_section as int64_t / 100 as int64_t;
    vbr_min_bits = if vbr_min_bits < 2147483647 as int64_t {
        vbr_min_bits
    } else {
        2147483647 as int64_t
    };
    (*rc).min_frame_bandwidth = if vbr_min_bits as core::ffi::c_int
        > 200 as core::ffi::c_int
    {
        vbr_min_bits as core::ffi::c_int
    } else {
        200 as core::ffi::c_int
    };
    let mut vbr_max_bits: int64_t = (*rc).avg_frame_bandwidth as int64_t
        * (*oxcf).rc_cfg.vbrmax_section as int64_t / 100 as int64_t;
    vbr_max_bits = if vbr_max_bits < 2147483647 as int64_t {
        vbr_max_bits
    } else {
        2147483647 as int64_t
    };
    (*rc).max_frame_bandwidth = if (if MBs * 250 as core::ffi::c_int
        > 2025000 as core::ffi::c_int
    {
        MBs * 250 as core::ffi::c_int
    } else {
        2025000 as core::ffi::c_int
    }) > vbr_max_bits as core::ffi::c_int
    {
        if MBs * 250 as core::ffi::c_int > 2025000 as core::ffi::c_int {
            MBs * 250 as core::ffi::c_int
        } else {
            2025000 as core::ffi::c_int
        }
    } else {
        vbr_max_bits as core::ffi::c_int
    };
    set_gf_interval_range(cpi, rc);
}
unsafe extern "C-unwind" fn vbr_rate_correction(
    mut cpi: *mut AV1_COMP,
    mut this_frame_target: *mut core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut vbr_bits_off_target: int64_t = (*p_rc).vbr_bits_off_target;
    let mut frame_target: int64_t = *this_frame_target as int64_t;
    let stats_count: core::ffi::c_double = if !((*(*(*cpi).ppi).twopass.stats_buf_ctx)
        .total_stats)
        .is_null()
    {
        (*(*(*(*cpi).ppi).twopass.stats_buf_ctx).total_stats).count
    } else {
        0.0f64
    };
    let frame_window: core::ffi::c_int = (if (16 as core::ffi::c_int
        as core::ffi::c_double)
        < stats_count - (*cpi).common.current_frame.frame_number as core::ffi::c_double
    {
        16 as core::ffi::c_int as core::ffi::c_double
    } else {
        stats_count - (*cpi).common.current_frame.frame_number as core::ffi::c_double
    }) as core::ffi::c_int;
    if frame_window > 0 as core::ffi::c_int {
        let max_delta: int64_t = if (llabs(
            vbr_bits_off_target as core::ffi::c_longlong
                / frame_window as core::ffi::c_longlong,
        ) as int64_t) < frame_target * 50 as int64_t / 100 as int64_t
        {
            llabs(
                vbr_bits_off_target as core::ffi::c_longlong
                    / frame_window as core::ffi::c_longlong,
            ) as int64_t
        } else {
            frame_target * 50 as int64_t / 100 as int64_t
        };
        frame_target
            += if vbr_bits_off_target >= 0 as int64_t { max_delta } else { -max_delta };
    }
    if frame_is_kf_gf_arf(cpi) == 0 && (*p_rc).vbr_bits_off_target_fast != 0
        && (*rc).is_src_frame_alt_ref == 0
    {
        let mut one_frame_bits: int64_t = if (*rc).avg_frame_bandwidth as int64_t
            > frame_target
        {
            (*rc).avg_frame_bandwidth as int64_t
        } else {
            frame_target
        };
        let mut fast_extra_bits: int64_t = 0;
        fast_extra_bits = if (*p_rc).vbr_bits_off_target_fast < one_frame_bits {
            (*p_rc).vbr_bits_off_target_fast
        } else {
            one_frame_bits
        };
        fast_extra_bits = if fast_extra_bits
            < (if one_frame_bits / 8 as int64_t
                > (*p_rc).vbr_bits_off_target_fast / 8 as int64_t
            {
                one_frame_bits / 8 as int64_t
            } else {
                (*p_rc).vbr_bits_off_target_fast / 8 as int64_t
            })
        {
            fast_extra_bits
        } else if one_frame_bits / 8 as int64_t
            > (*p_rc).vbr_bits_off_target_fast / 8 as int64_t
        {
            one_frame_bits / 8 as int64_t
        } else {
            (*p_rc).vbr_bits_off_target_fast / 8 as int64_t
        };
        fast_extra_bits = if fast_extra_bits < 2147483647 as int64_t {
            fast_extra_bits
        } else {
            2147483647 as int64_t
        };
        if fast_extra_bits > 0 as int64_t {
            frame_target += fast_extra_bits;
        }
        (*rc).frame_level_fast_extra_bits = fast_extra_bits as core::ffi::c_int;
        (*cpi).do_update_vbr_bits_off_target_fast = 1 as core::ffi::c_int;
    }
    *this_frame_target = (if frame_target < 2147483647 as int64_t {
        frame_target
    } else {
        2147483647 as int64_t
    }) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_target_rate(
    mut cpi: *mut AV1_COMP,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let mut target_rate: core::ffi::c_int = (*rc).base_frame_target;
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_VBR as core::ffi::c_int as core::ffi::c_uint
        || (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
    {
        vbr_rate_correction(cpi, &raw mut target_rate);
    }
    av1_rc_set_frame_target(cpi, target_rate, width, height);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_pframe_target_size_one_pass_vbr(
    cpi: *const AV1_COMP,
    mut frame_update_type: FRAME_UPDATE_TYPE,
) -> core::ffi::c_int {
    let af_ratio: core::ffi::c_int = if is_one_pass_rt_lag_params(cpi) != 0 {
        6 as core::ffi::c_int
    } else {
        10 as core::ffi::c_int
    };
    let rc: *const RATE_CONTROL = &(*cpi).rc;
    let p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut target: int64_t = 0;
    if frame_update_type as core::ffi::c_int == KF_UPDATE as core::ffi::c_int
        || frame_update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int
        || frame_update_type as core::ffi::c_int == ARF_UPDATE as core::ffi::c_int
    {
        target = (*rc).avg_frame_bandwidth as int64_t
            * (*p_rc).baseline_gf_interval as int64_t * af_ratio as int64_t
            / ((*p_rc).baseline_gf_interval + af_ratio - 1 as core::ffi::c_int)
                as int64_t;
    } else {
        target = (*rc).avg_frame_bandwidth as int64_t
            * (*p_rc).baseline_gf_interval as int64_t
            / ((*p_rc).baseline_gf_interval + af_ratio - 1 as core::ffi::c_int)
                as int64_t;
    }
    return clamp_pframe_target_size(cpi, target, frame_update_type);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_iframe_target_size_one_pass_vbr(
    cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    static mut kf_ratio: core::ffi::c_int = 25 as core::ffi::c_int;
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let target: int64_t = (*rc).avg_frame_bandwidth as int64_t * kf_ratio as int64_t;
    return clamp_iframe_target_size(cpi, target);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_pframe_target_size_one_pass_cbr(
    mut cpi: *const AV1_COMP,
    mut frame_update_type: FRAME_UPDATE_TYPE,
) -> core::ffi::c_int {
    let mut oxcf: *const AV1EncoderConfig = &(*cpi).oxcf;
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let mut p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut rc_cfg: *const RateControlCfg = &(*oxcf).rc_cfg;
    let refresh_frame: *const RefreshFrameInfo = &(*cpi).refresh_frame;
    let mut diff: int64_t = (*p_rc).optimal_buffer_level - (*p_rc).buffer_level;
    if is_one_pass_rt_lag_params(cpi) != 0
        && ((*refresh_frame).alt_ref_frame as core::ffi::c_int != 0
            || (*refresh_frame).golden_frame as core::ffi::c_int != 0)
    {
        diff = if diff < -(*p_rc).optimal_buffer_level {
            diff
        } else {
            -(*p_rc).optimal_buffer_level
        };
    }
    let one_pct_bits: int64_t = 1 as int64_t
        + (*p_rc).optimal_buffer_level / 100 as int64_t;
    let mut min_frame_target: core::ffi::c_int = if (*rc).avg_frame_bandwidth
        >> 4 as core::ffi::c_int > 200 as core::ffi::c_int
    {
        (*rc).avg_frame_bandwidth >> 4 as core::ffi::c_int
    } else {
        200 as core::ffi::c_int
    };
    let mut target: int64_t = 0;
    if (*rc_cfg).gf_cbr_boost_pct != 0 {
        let af_ratio_pct: core::ffi::c_int = ((*rc_cfg).gf_cbr_boost_pct)
            .wrapping_add(100 as core::ffi::c_uint) as core::ffi::c_int;
        if frame_update_type as core::ffi::c_int == GF_UPDATE as core::ffi::c_int
            || frame_update_type as core::ffi::c_int
                == OVERLAY_UPDATE as core::ffi::c_int
        {
            target = (*rc).avg_frame_bandwidth as int64_t
                * (*p_rc).baseline_gf_interval as int64_t * af_ratio_pct as int64_t
                / ((*p_rc).baseline_gf_interval * 100 as core::ffi::c_int + af_ratio_pct
                    - 100 as core::ffi::c_int) as int64_t;
        } else {
            target = (*rc).avg_frame_bandwidth as int64_t
                * (*p_rc).baseline_gf_interval as int64_t * 100 as int64_t
                / ((*p_rc).baseline_gf_interval * 100 as core::ffi::c_int + af_ratio_pct
                    - 100 as core::ffi::c_int) as int64_t;
        }
    } else {
        target = (*rc).avg_frame_bandwidth as int64_t;
    }
    if (*(*cpi).ppi).use_svc != 0 {
        let mut layer: core::ffi::c_int = (*cpi).svc.spatial_layer_id
            * (*cpi).svc.number_temporal_layers + (*cpi).svc.temporal_layer_id;
        let mut lc: *const LAYER_CONTEXT = &raw mut *((*cpi).svc.layer_context)
            .offset(layer as isize) as *mut LAYER_CONTEXT;
        target = (*lc).avg_frame_size as int64_t;
        min_frame_target = if (*lc).avg_frame_size >> 4 as core::ffi::c_int
            > 200 as core::ffi::c_int
        {
            (*lc).avg_frame_size >> 4 as core::ffi::c_int
        } else {
            200 as core::ffi::c_int
        };
    }
    if diff > 0 as int64_t {
        let pct_low: core::ffi::c_int = (if diff / one_pct_bits
            < (*rc_cfg).under_shoot_pct as int64_t
        {
            diff / one_pct_bits
        } else {
            (*rc_cfg).under_shoot_pct as int64_t
        }) as core::ffi::c_int;
        target -= target * pct_low as int64_t / 200 as int64_t;
    } else if diff < 0 as int64_t {
        let pct_high: core::ffi::c_int = (if -diff / one_pct_bits
            < (*rc_cfg).over_shoot_pct as int64_t
        {
            -diff / one_pct_bits
        } else {
            (*rc_cfg).over_shoot_pct as int64_t
        }) as core::ffi::c_int;
        target += target * pct_high as int64_t / 200 as int64_t;
    }
    if (*rc_cfg).max_inter_bitrate_pct != 0 {
        let max_rate: int64_t = (*rc).avg_frame_bandwidth as int64_t
            * (*rc_cfg).max_inter_bitrate_pct as int64_t / 100 as int64_t;
        target = if target < max_rate { target } else { max_rate };
    }
    if target > INT_MAX as int64_t {
        target = INT_MAX as int64_t;
    }
    return if min_frame_target > target as core::ffi::c_int {
        min_frame_target
    } else {
        target as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calc_iframe_target_size_one_pass_cbr(
    mut cpi: *const AV1_COMP,
) -> core::ffi::c_int {
    let mut rc: *const RATE_CONTROL = &(*cpi).rc;
    let mut p_rc: *const PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut target: int64_t = 0;
    if (*cpi).common.current_frame.frame_number == 0 as core::ffi::c_uint {
        target = (if (*p_rc).starting_buffer_level / 2 as int64_t > INT_MAX as int64_t {
            INT_MAX
        } else {
            ((*p_rc).starting_buffer_level / 2 as int64_t) as core::ffi::c_int
        }) as int64_t;
        if (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int
            && target < (INT_MAX >> 2 as core::ffi::c_int) as int64_t
        {
            target = target
                << (if (2 as core::ffi::c_int)
                    < (*cpi).svc.number_temporal_layers - 1 as core::ffi::c_int
                {
                    2 as core::ffi::c_int
                } else {
                    (*cpi).svc.number_temporal_layers - 1 as core::ffi::c_int
                });
        }
    } else {
        let mut kf_boost: core::ffi::c_int = 32 as core::ffi::c_int;
        let mut framerate: core::ffi::c_double = (*cpi).framerate;
        kf_boost = if kf_boost
            > round(
                2 as core::ffi::c_int as core::ffi::c_double * framerate
                    - 16 as core::ffi::c_int as core::ffi::c_double,
            ) as core::ffi::c_int
        {
            kf_boost
        } else {
            round(
                2 as core::ffi::c_int as core::ffi::c_double * framerate
                    - 16 as core::ffi::c_int as core::ffi::c_double,
            ) as core::ffi::c_int
        };
        if ((*rc).frames_since_key as core::ffi::c_double)
            < framerate / 2 as core::ffi::c_int as core::ffi::c_double
        {
            kf_boost = ((kf_boost * (*rc).frames_since_key) as core::ffi::c_double
                / (framerate / 2 as core::ffi::c_int as core::ffi::c_double))
                as core::ffi::c_int;
        }
        target = (16 as core::ffi::c_int + kf_boost) as int64_t
            * (*rc).avg_frame_bandwidth as int64_t >> 4 as core::ffi::c_int;
    }
    return clamp_iframe_target_size(cpi, target);
}
unsafe extern "C-unwind" fn set_golden_update(cpi: *mut AV1_COMP) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut divisor: core::ffi::c_int = 10 as core::ffi::c_int;
    if (*cpi).oxcf.q_cfg.aq_mode as core::ffi::c_int
        == CYCLIC_REFRESH_AQ as core::ffi::c_int
    {
        divisor = (*(*cpi).cyclic_refresh).percent_refresh;
    }
    let gf_length_mult: [core::ffi::c_int; 2] = [
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    ];
    if divisor > 0 as core::ffi::c_int {
        (*p_rc).baseline_gf_interval = if gf_length_mult[(*cpi).sf.rt_sf.gf_length_lvl
            as usize] * (100 as core::ffi::c_int / divisor) < 160 as core::ffi::c_int
        {
            gf_length_mult[(*cpi).sf.rt_sf.gf_length_lvl as usize]
                * (100 as core::ffi::c_int / divisor)
        } else {
            160 as core::ffi::c_int
        };
    } else {
        (*p_rc).baseline_gf_interval = FIXED_GF_INTERVAL_RT;
    }
    if (*rc).avg_frame_low_motion != 0
        && (*rc).avg_frame_low_motion < 40 as core::ffi::c_int
    {
        (*p_rc).baseline_gf_interval = 16 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn set_baseline_gf_interval(
    mut cpi: *mut AV1_COMP,
    mut frame_type: FRAME_TYPE,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    set_golden_update(cpi);
    if (*p_rc).baseline_gf_interval > (*rc).frames_to_key
        && (*cpi).oxcf.kf_cfg.auto_key as core::ffi::c_int != 0
    {
        (*p_rc).baseline_gf_interval = (*rc).frames_to_key;
    }
    (*p_rc).gfu_boost = DEFAULT_GF_BOOST_RT;
    (*p_rc).constrained_gf_group = if (*p_rc).baseline_gf_interval >= (*rc).frames_to_key
        && (*cpi).oxcf.kf_cfg.auto_key as core::ffi::c_int != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    (*rc).frames_till_gf_update_due = (*p_rc).baseline_gf_interval;
    (*cpi).gf_frame_index = 0 as core::ffi::c_uchar;
    if (*(*cpi).ppi).use_svc != 0 {
        let svc: *mut SVC = &raw mut (*cpi).svc;
        (*p_rc).baseline_gf_interval = MAX_STATIC_GF_GROUP_LENGTH
            - 1 as core::ffi::c_int;
        (*p_rc).gfu_boost = 1 as core::ffi::c_int;
        (*p_rc).constrained_gf_group = 0 as core::ffi::c_int;
        (*rc).frames_till_gf_update_due = (*p_rc).baseline_gf_interval;
        let mut layer: core::ffi::c_int = 0 as core::ffi::c_int;
        while layer < (*svc).number_spatial_layers * (*svc).number_temporal_layers {
            let lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                .offset(layer as isize) as *mut LAYER_CONTEXT;
            (*lc).p_rc.baseline_gf_interval = (*p_rc).baseline_gf_interval;
            (*lc).p_rc.gfu_boost = (*p_rc).gfu_boost;
            (*lc).p_rc.constrained_gf_group = (*p_rc).constrained_gf_group;
            (*lc).rc.frames_till_gf_update_due = (*rc).frames_till_gf_update_due;
            (*lc).group_index = 0 as core::ffi::c_uchar;
            layer += 1;
        }
    }
    (*gf_group).size = (*p_rc).baseline_gf_interval;
    (*gf_group).update_type[0 as core::ffi::c_int as usize] = (if frame_type
        as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
    {
        KF_UPDATE as core::ffi::c_int
    } else {
        GF_UPDATE as core::ffi::c_int
    }) as FRAME_UPDATE_TYPE;
    (*gf_group).refbuf_state[(*cpi).gf_frame_index as usize] = (if frame_type
        as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
    {
        REFBUF_RESET as core::ffi::c_int
    } else {
        REFBUF_UPDATE as core::ffi::c_int
    }) as REFBUF_STATE;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_adjust_gf_refresh_qp_one_pass_rt(mut cpi: *mut AV1_COMP) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let rtc_ref: *mut RTC_REF = &raw mut (*(*cpi).ppi).rtc_ref;
    let resize_pending: core::ffi::c_int = is_frame_resize_pending(cpi)
        as core::ffi::c_int;
    if resize_pending == 0 && (*rc).high_source_sad == 0 {
        let mut p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
        let avg_qp: core::ffi::c_int = (*p_rc)
            .avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize];
        let allow_gf_update: core::ffi::c_int = ((*rc).frames_till_gf_update_due
            <= (*p_rc).baseline_gf_interval - 10 as core::ffi::c_int)
            as core::ffi::c_int;
        let mut gf_update_changed: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut thresh: core::ffi::c_int = 87 as core::ffi::c_int;
        if ((*cm).current_frame.frame_number)
            .wrapping_sub((*cpi).rc.frame_num_last_gf_refresh as core::ffi::c_uint)
            < FIXED_GF_INTERVAL_RT as core::ffi::c_uint
            && (*rc).frames_till_gf_update_due == 1 as core::ffi::c_int
            && (*cm).quant_params.base_qindex > avg_qp
        {
            (*rtc_ref).refresh[(*rtc_ref).gld_idx_1layer as usize] = 0
                as core::ffi::c_int;
            gf_update_changed = 1 as core::ffi::c_int;
            (*cpi).refresh_frame.golden_frame = 0 as core::ffi::c_int != 0;
        } else if allow_gf_update != 0
            && ((*cm).quant_params.base_qindex
                < thresh * avg_qp / 100 as core::ffi::c_int
                || (*rc).avg_frame_low_motion != 0
                    && (*rc).avg_frame_low_motion < 20 as core::ffi::c_int)
        {
            (*rtc_ref).refresh[(*rtc_ref).gld_idx_1layer as usize] = 1
                as core::ffi::c_int;
            gf_update_changed = 1 as core::ffi::c_int;
            (*cpi).refresh_frame.golden_frame = 1 as core::ffi::c_int != 0;
        }
        if gf_update_changed != 0 {
            set_baseline_gf_interval(cpi, INTER_FRAME as core::ffi::c_int as FRAME_TYPE);
            let mut refresh_mask: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while i < INTER_REFS_PER_FRAME as core::ffi::c_int as core::ffi::c_uint {
                let mut ref_frame_map_idx: core::ffi::c_int = (*rtc_ref)
                    .ref_idx[i as usize];
                refresh_mask
                    |= (*rtc_ref).refresh[ref_frame_map_idx as usize]
                        << ref_frame_map_idx;
                i = i.wrapping_add(1);
            }
            (*cm).current_frame.refresh_frame_flags = refresh_mask;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_rtc_reference_structure_one_layer(
    mut cpi: *mut AV1_COMP,
    mut gf_update: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let ext_flags: *mut ExternalFlags = &raw mut (*cpi).ext_flags;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let ext_refresh_frame_flags: *mut ExtRefreshFrameFlagsInfo = &raw mut (*ext_flags)
        .refresh_frame;
    let rtc_ref: *mut RTC_REF = &raw mut (*(*cpi).ppi).rtc_ref;
    let mut frame_number: core::ffi::c_uint = if (*cpi)
        .oxcf
        .rc_cfg
        .drop_frames_water_mark != 0
    {
        (*rc).frame_number_encoded
    } else {
        (*cm).current_frame.frame_number
    };
    let mut lag_alt: core::ffi::c_uint = 4 as core::ffi::c_uint;
    let mut last_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut last_idx_refresh: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut gld_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut alt_ref_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut last2_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    (*ext_refresh_frame_flags).update_pending = 1 as core::ffi::c_int != 0;
    (*ext_flags).ref_frame_flags = 0 as core::ffi::c_int;
    (*ext_refresh_frame_flags).last_frame = 1 as core::ffi::c_int != 0;
    (*ext_refresh_frame_flags).golden_frame = 0 as core::ffi::c_int != 0;
    (*ext_refresh_frame_flags).alt_ref_frame = 0 as core::ffi::c_int != 0;
    if (*cpi).sf.rt_sf.sad_based_adp_altref_lag != 0 {
        lag_alt = 6 as core::ffi::c_uint;
        let th_frame_sad: [[uint64_t; 3]; 4] = [
            [
                18000 as core::ffi::c_int as uint64_t,
                18000 as core::ffi::c_int as uint64_t,
                18000 as core::ffi::c_int as uint64_t,
            ],
            [
                25000 as core::ffi::c_int as uint64_t,
                25000 as core::ffi::c_int as uint64_t,
                25000 as core::ffi::c_int as uint64_t,
            ],
            [
                40000 as core::ffi::c_int as uint64_t,
                30000 as core::ffi::c_int as uint64_t,
                20000 as core::ffi::c_int as uint64_t,
            ],
            [
                30000 as core::ffi::c_int as uint64_t,
                25000 as core::ffi::c_int as uint64_t,
                20000 as core::ffi::c_int as uint64_t,
            ],
        ];
        let mut th_idx: core::ffi::c_int = (*cpi).sf.rt_sf.sad_based_adp_altref_lag
            - 1 as core::ffi::c_int;
        if (*rc).avg_source_sad
            > th_frame_sad[th_idx as usize][0 as core::ffi::c_int as usize]
        {
            lag_alt = 3 as core::ffi::c_uint;
        } else if (*rc).avg_source_sad
            > th_frame_sad[th_idx as usize][1 as core::ffi::c_int as usize]
        {
            lag_alt = 4 as core::ffi::c_uint;
        } else if (*rc).avg_source_sad
            > th_frame_sad[th_idx as usize][2 as core::ffi::c_int as usize]
        {
            lag_alt = 5 as core::ffi::c_uint;
        }
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
        (*rtc_ref).ref_idx[i as usize] = 7 as core::ffi::c_int;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < REF_FRAMES as core::ffi::c_int {
        (*rtc_ref).refresh[i_0 as usize] = 0 as core::ffi::c_int;
        i_0 += 1;
    }
    (*ext_flags).ref_frame_flags ^= AOM_LAST_FLAG as core::ffi::c_int;
    if (*cpi).sf.rt_sf.force_only_last_ref == 0 {
        (*ext_flags).ref_frame_flags ^= AOM_ALT_FLAG as core::ffi::c_int;
        (*ext_flags).ref_frame_flags ^= AOM_GOLD_FLAG as core::ffi::c_int;
        if (*cpi).sf.rt_sf.ref_frame_comp_nonrd[1 as core::ffi::c_int as usize] != 0 {
            (*ext_flags).ref_frame_flags ^= AOM_LAST2_FLAG as core::ffi::c_int;
        }
    }
    let sh: core::ffi::c_int = 6 as core::ffi::c_int;
    if frame_number > 1 as core::ffi::c_uint {
        last_idx = frame_number
            .wrapping_sub(1 as core::ffi::c_uint)
            .wrapping_rem(sh as core::ffi::c_uint) as core::ffi::c_int;
    }
    last_idx_refresh = frame_number.wrapping_rem(sh as core::ffi::c_uint)
        as core::ffi::c_int;
    gld_idx = 6 as core::ffi::c_int;
    if frame_number > lag_alt {
        alt_ref_idx = frame_number
            .wrapping_sub(lag_alt)
            .wrapping_rem(sh as core::ffi::c_uint) as core::ffi::c_int;
    }
    if (*cpi).sf.rt_sf.ref_frame_comp_nonrd[1 as core::ffi::c_int as usize] != 0 {
        if frame_number > 2 as core::ffi::c_uint {
            last2_idx = frame_number
                .wrapping_sub(2 as core::ffi::c_uint)
                .wrapping_rem(sh as core::ffi::c_uint) as core::ffi::c_int;
        }
    }
    (*rtc_ref).ref_idx[0 as core::ffi::c_int as usize] = last_idx;
    (*rtc_ref).ref_idx[1 as core::ffi::c_int as usize] = last_idx_refresh;
    if (*cpi).sf.rt_sf.ref_frame_comp_nonrd[1 as core::ffi::c_int as usize] != 0 {
        (*rtc_ref).ref_idx[1 as core::ffi::c_int as usize] = last2_idx;
        (*rtc_ref).ref_idx[2 as core::ffi::c_int as usize] = last_idx_refresh;
    }
    (*rtc_ref).ref_idx[3 as core::ffi::c_int as usize] = gld_idx;
    (*rtc_ref).ref_idx[6 as core::ffi::c_int as usize] = alt_ref_idx;
    (*rtc_ref).refresh[last_idx_refresh as usize] = 1 as core::ffi::c_int;
    if gf_update != 0
        && (*cm).current_frame.frame_type as core::ffi::c_int
            != KEY_FRAME as core::ffi::c_int
    {
        (*ext_refresh_frame_flags).golden_frame = 1 as core::ffi::c_int != 0;
        (*rtc_ref).refresh[gld_idx as usize] = 1 as core::ffi::c_int;
    }
    (*rtc_ref).gld_idx_1layer = gld_idx;
    (*cpi).rt_reduce_num_ref_buffers = 1 as core::ffi::c_int;
    (*cpi).rt_reduce_num_ref_buffers
        &= ((*rtc_ref).ref_idx[0 as core::ffi::c_int as usize] < 7 as core::ffi::c_int)
            as core::ffi::c_int;
    (*cpi).rt_reduce_num_ref_buffers
        &= ((*rtc_ref).ref_idx[1 as core::ffi::c_int as usize] < 7 as core::ffi::c_int)
            as core::ffi::c_int;
    (*cpi).rt_reduce_num_ref_buffers
        &= ((*rtc_ref).ref_idx[3 as core::ffi::c_int as usize] < 7 as core::ffi::c_int)
            as core::ffi::c_int;
    (*cpi).rt_reduce_num_ref_buffers
        &= ((*rtc_ref).ref_idx[6 as core::ffi::c_int as usize] < 7 as core::ffi::c_int)
            as core::ffi::c_int;
    if (*cpi).sf.rt_sf.ref_frame_comp_nonrd[1 as core::ffi::c_int as usize] != 0 {
        (*cpi).rt_reduce_num_ref_buffers
            &= ((*rtc_ref).ref_idx[2 as core::ffi::c_int as usize]
                < 7 as core::ffi::c_int) as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn set_block_is_active(
    active_map_4x4: *mut core::ffi::c_uchar,
    mut mi_cols: core::ffi::c_int,
    mut mi_rows: core::ffi::c_int,
    mut sbi_col: core::ffi::c_int,
    mut sbi_row: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut num_4x4: core::ffi::c_int = 16 as core::ffi::c_int;
    let mut r: core::ffi::c_int = sbi_row << 4 as core::ffi::c_int;
    let mut c: core::ffi::c_int = sbi_col << 4 as core::ffi::c_int;
    let row_max: core::ffi::c_int = if num_4x4 < mi_rows - r {
        num_4x4
    } else {
        mi_rows - r
    };
    let col_max: core::ffi::c_int = if num_4x4 < mi_cols - c {
        num_4x4
    } else {
        mi_cols - c
    };
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < row_max {
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < col_max {
            if *active_map_4x4.offset(((r + x) * mi_cols + (c + y)) as isize)
                as core::ffi::c_int == AM_SEGMENT_ID_ACTIVE
            {
                return 1 as core::ffi::c_int;
            }
            y += 4 as core::ffi::c_int;
        }
        x += 4 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn estimate_scroll_motion(
    mut cpi: *const AV1_COMP,
    mut src_buf: *mut uint8_t,
    mut last_src_buf: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_stride: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut pos_col: core::ffi::c_int,
    mut pos_row: core::ffi::c_int,
    mut best_intmv_col: *mut core::ffi::c_int,
    mut best_intmv_row: *mut core::ffi::c_int,
    mut sw_col: core::ffi::c_int,
    mut sw_row: core::ffi::c_int,
) -> core::ffi::c_uint {
    let cm: *const AV1_COMMON = &(*cpi).common;
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let full_search: core::ffi::c_int = 1 as core::ffi::c_int;
    let border: core::ffi::c_int = ((*cpi).oxcf.border_in_pixels
        >> 4 as core::ffi::c_int) << 4 as core::ffi::c_int;
    let mut search_size_width: core::ffi::c_int = sw_col;
    let mut search_size_height: core::ffi::c_int = sw_row;
    if pos_col - search_size_width < -border
        || pos_col + search_size_width > (*cm).width + border
    {
        search_size_width = border;
    }
    if pos_row - search_size_height < -border
        || pos_row + search_size_height > (*cm).height + border
    {
        search_size_height = border;
    }
    let mut ref_buf: *const uint8_t = 0 as *const uint8_t;
    let row_norm_factor: core::ffi::c_int = mi_size_high_log2[bsize as usize]
        as core::ffi::c_int + 1 as core::ffi::c_int;
    let col_norm_factor: core::ffi::c_int = 3 as core::ffi::c_int
        + (bw >> 5 as core::ffi::c_int);
    let ref_buf_width: core::ffi::c_int = (search_size_width << 1 as core::ffi::c_int)
        + bw;
    let ref_buf_height: core::ffi::c_int = (search_size_height << 1 as core::ffi::c_int)
        + bh;
    let mut hbuf: *mut int16_t = aom_malloc(
        (ref_buf_width as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    let mut vbuf: *mut int16_t = aom_malloc(
        (ref_buf_height as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    let mut src_hbuf: *mut int16_t = aom_malloc(
        (bw as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    let mut src_vbuf: *mut int16_t = aom_malloc(
        (bh as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    if hbuf.is_null() || vbuf.is_null() || src_hbuf.is_null() || src_vbuf.is_null() {
        aom_free(hbuf as *mut core::ffi::c_void);
        aom_free(vbuf as *mut core::ffi::c_void);
        aom_free(src_hbuf as *mut core::ffi::c_void);
        aom_free(src_vbuf as *mut core::ffi::c_void);
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate hbuf, vbuf, src_hbuf, or src_vbuf\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    ref_buf = last_src_buf.offset(-(search_size_width as isize));
    aom_int_pro_row_c(hbuf, ref_buf, ref_stride, ref_buf_width, bh, row_norm_factor);
    ref_buf = last_src_buf.offset(-((search_size_height * ref_stride) as isize));
    aom_int_pro_col_c(vbuf, ref_buf, ref_stride, bw, ref_buf_height, col_norm_factor);
    aom_int_pro_row_c(src_hbuf, src_buf, src_stride, bw, bh, row_norm_factor);
    aom_int_pro_col_c(src_vbuf, src_buf, src_stride, bw, bh, col_norm_factor);
    let mut best_sad: core::ffi::c_uint = 0;
    let mut best_sad_col: core::ffi::c_int = 0;
    let mut best_sad_row: core::ffi::c_int = 0;
    *best_intmv_col = av1_vector_match(
        hbuf,
        src_hbuf,
        mi_size_wide_log2[bsize as usize] as core::ffi::c_int,
        search_size_width,
        search_size_width,
        full_search,
        &raw mut best_sad_col,
    );
    *best_intmv_row = av1_vector_match(
        vbuf,
        src_vbuf,
        mi_size_high_log2[bsize as usize] as core::ffi::c_int,
        search_size_height,
        search_size_height,
        full_search,
        &raw mut best_sad_row,
    );
    if best_sad_col < best_sad_row {
        *best_intmv_row = 0 as core::ffi::c_int;
        best_sad = best_sad_col as core::ffi::c_uint;
    } else {
        *best_intmv_col = 0 as core::ffi::c_int;
        best_sad = best_sad_row as core::ffi::c_uint;
    }
    aom_free(hbuf as *mut core::ffi::c_void);
    aom_free(vbuf as *mut core::ffi::c_void);
    aom_free(src_hbuf as *mut core::ffi::c_void);
    aom_free(src_vbuf as *mut core::ffi::c_void);
    return best_sad;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rc_scene_detection_onepass_rt(
    mut cpi: *mut AV1_COMP,
    mut frame_input: *const EncodeFrameInput,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let unscaled_src: *const YV12_BUFFER_CONFIG = (*frame_input).source;
    let unscaled_last_src: *const YV12_BUFFER_CONFIG = (*frame_input).last_source;
    let mut src_y: *mut uint8_t = 0 as *mut uint8_t;
    let mut src_ystride: core::ffi::c_int = 0;
    let mut src_width: core::ffi::c_int = 0;
    let mut src_height: core::ffi::c_int = 0;
    let mut last_src_y: *mut uint8_t = 0 as *mut uint8_t;
    let mut last_src_ystride: core::ffi::c_int = 0;
    let mut last_src_width: core::ffi::c_int = 0;
    let mut last_src_height: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = (*cm).width;
    let mut height: core::ffi::c_int = (*cm).height;
    if (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int {
        width = (*cpi).oxcf.frm_dim_cfg.width;
        height = (*cpi).oxcf.frm_dim_cfg.height;
    }
    if width != (*cm).render_width || height != (*cm).render_height
        || (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int
        || unscaled_src.is_null() || unscaled_last_src.is_null()
        || is_one_pass_rt_lag_params(cpi) != 0
    {
        aom_free((*cpi).src_sad_blk_64x64 as *mut core::ffi::c_void);
        (*cpi).src_sad_blk_64x64 = 0 as *mut uint64_t;
        (*cpi).src_sad_blk_alloc_size = 0 as core::ffi::c_int;
    }
    if unscaled_src.is_null() || unscaled_last_src.is_null() {
        return;
    }
    src_y = (*unscaled_src).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    src_ystride = (*unscaled_src).c2rust_unnamed_3.c2rust_unnamed.y_stride;
    src_width = (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width;
    src_height = (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height;
    last_src_y = (*unscaled_last_src).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    last_src_ystride = (*unscaled_last_src).c2rust_unnamed_3.c2rust_unnamed.y_stride;
    last_src_width = (*unscaled_last_src).c2rust_unnamed.c2rust_unnamed.y_width;
    last_src_height = (*unscaled_last_src).c2rust_unnamed_0.c2rust_unnamed.y_height;
    if src_width != last_src_width || src_height != last_src_height {
        aom_free((*cpi).src_sad_blk_64x64 as *mut core::ffi::c_void);
        (*cpi).src_sad_blk_64x64 = 0 as *mut uint64_t;
        (*cpi).src_sad_blk_alloc_size = 0 as core::ffi::c_int;
        return;
    }
    (*rc).high_source_sad = 0 as core::ffi::c_int;
    (*rc).percent_blocks_with_motion = 0 as core::ffi::c_int;
    (*rc).max_block_source_sad = 0 as uint64_t;
    (*rc).prev_avg_source_sad = (*rc).avg_source_sad;
    let mut num_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let mut num_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    if (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int {
        num_mi_cols = size_in_mi(src_width);
        num_mi_rows = size_in_mi(src_height);
    }
    let mut num_zero_temp_sad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut min_thresh: uint32_t = (if (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
    {
        8000 as core::ffi::c_int
    } else {
        10000 as core::ffi::c_int
    }) as uint32_t;
    if (*cpi).sf.rt_sf.higher_thresh_scene_detection != 0 {
        min_thresh = (if (*cm).width * (*cm).height
            <= 320 as core::ffi::c_int * 240 as core::ffi::c_int
            && (*cpi).framerate < 10.0f64
        {
            50000 as core::ffi::c_int
        } else {
            100000 as core::ffi::c_int
        }) as uint32_t;
    }
    let bsize: BLOCK_SIZE = BLOCK_64X64;
    let mut avg_sad: uint64_t = 0 as uint64_t;
    let mut tmp_sad: uint64_t = 0 as uint64_t;
    let mut num_samples: core::ffi::c_int = 0 as core::ffi::c_int;
    let thresh: core::ffi::c_int = if (*cm).width * (*cm).height
        <= 320 as core::ffi::c_int * 240 as core::ffi::c_int
        && (*cpi).framerate < 10.0f64
        || (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
            == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
    {
        5 as core::ffi::c_int
    } else {
        6 as core::ffi::c_int
    };
    let sb_size_by_mb: core::ffi::c_int = if (*(*cm).seq_params).sb_size
        as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
    {
        (*(*cm).seq_params).mib_size >> 1 as core::ffi::c_int
    } else {
        (*(*cm).seq_params).mib_size
    };
    let sb_cols: core::ffi::c_int = (num_mi_cols + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let sb_rows: core::ffi::c_int = (num_mi_rows + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let mut sum_sq_thresh: uint64_t = 10000 as uint64_t;
    let mut num_low_var_high_sumdiff: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut light_change: core::ffi::c_int = 0 as core::ffi::c_int;
    let check_light_change: core::ffi::c_int = 0 as core::ffi::c_int;
    let border: core::ffi::c_int = ((*rc).prev_frame_is_dropped != 0
        || (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int)
        as core::ffi::c_int;
    if width == (*cm).render_width && height == (*cm).render_height
        && (*cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
        && is_one_pass_rt_lag_params(cpi) == 0
    {
        if (*cpi).src_sad_blk_alloc_size != sb_cols * sb_rows {
            aom_free((*cpi).src_sad_blk_64x64 as *mut core::ffi::c_void);
            (*cpi).src_sad_blk_64x64 = aom_calloc(
                (sb_cols * sb_rows) as size_t,
                ::core::mem::size_of::<uint64_t>() as size_t,
            ) as *mut uint64_t;
            if ((*cpi).src_sad_blk_64x64).is_null() {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate cpi->src_sad_blk_64x64\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            (*cpi).src_sad_blk_alloc_size = sb_cols * sb_rows;
        }
    }
    let mi_params: *const CommonModeInfoParams = &raw mut (*cpi).common.mi_params;
    let mi_cols: core::ffi::c_int = (*mi_params).mi_cols;
    let mi_rows: core::ffi::c_int = (*mi_params).mi_rows;
    let active_map_4x4: *mut core::ffi::c_uchar = (*cpi).active_map.map;
    let mut sbi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while sbi_row < sb_rows - border {
        let mut sbi_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while sbi_col < sb_cols {
            let mut block_is_active: core::ffi::c_int = 1 as core::ffi::c_int;
            if (*cpi).active_map.enabled != 0
                && (*rc).percent_blocks_inactive > 0 as core::ffi::c_int
            {
                block_is_active = set_block_is_active(
                    active_map_4x4,
                    mi_cols,
                    mi_rows,
                    sbi_col,
                    sbi_row,
                );
            }
            if block_is_active != 0 {
                tmp_sad = ((*(*cpi).ppi).fn_ptr[bsize as usize].sdf)
                    .expect(
                        "non-null function pointer",
                    )(src_y, src_ystride, last_src_y, last_src_ystride) as uint64_t;
            } else {
                tmp_sad = 0 as uint64_t;
            }
            if !((*cpi).src_sad_blk_64x64).is_null() {
                *((*cpi).src_sad_blk_64x64)
                    .offset((sbi_col + sbi_row * sb_cols) as isize) = tmp_sad;
            }
            if check_light_change != 0 {
                let mut sse: core::ffi::c_uint = 0;
                let mut variance: core::ffi::c_uint = 0;
                variance = ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
                    .expect(
                        "non-null function pointer",
                    )(src_y, src_ystride, last_src_y, last_src_ystride, &raw mut sse);
                if variance < sse >> 1 as core::ffi::c_int
                    && sse.wrapping_sub(variance) as uint64_t > sum_sq_thresh
                {
                    num_low_var_high_sumdiff += 1;
                }
            }
            avg_sad = avg_sad.wrapping_add(tmp_sad);
            num_samples += 1;
            if tmp_sad == 0 as uint64_t {
                num_zero_temp_sad += 1;
            }
            if tmp_sad > (*rc).max_block_source_sad {
                (*rc).max_block_source_sad = tmp_sad;
            }
            src_y = src_y.offset(64 as core::ffi::c_int as isize);
            last_src_y = last_src_y.offset(64 as core::ffi::c_int as isize);
            sbi_col += 1;
        }
        src_y = src_y
            .offset(
                ((src_ystride << 6 as core::ffi::c_int)
                    - (sb_cols << 6 as core::ffi::c_int)) as isize,
            );
        last_src_y = last_src_y
            .offset(
                ((last_src_ystride << 6 as core::ffi::c_int)
                    - (sb_cols << 6 as core::ffi::c_int)) as isize,
            );
        sbi_row += 1;
    }
    if check_light_change != 0 && num_samples > 0 as core::ffi::c_int
        && num_low_var_high_sumdiff > num_samples >> 1 as core::ffi::c_int
    {
        light_change = 1 as core::ffi::c_int;
    }
    if num_samples > 0 as core::ffi::c_int {
        avg_sad = avg_sad.wrapping_div(num_samples as uint64_t);
    }
    let thresh_zero_sad_samples: core::ffi::c_int = if avg_sad
        > (8 as uint32_t).wrapping_mul(min_thresh) as uint64_t
    {
        3 as core::ffi::c_int * (num_samples >> 2 as core::ffi::c_int)
    } else {
        num_samples >> 1 as core::ffi::c_int
    };
    if light_change == 0
        && avg_sad
            > (if min_thresh
                > ((*rc).avg_source_sad).wrapping_mul(thresh as uint64_t) as uint32_t
            {
                min_thresh
            } else {
                ((*rc).avg_source_sad).wrapping_mul(thresh as uint64_t) as uint32_t
            }) as uint64_t
        && (*rc).frames_since_key
            > 1 as core::ffi::c_int + (*cpi).svc.number_spatial_layers
        && num_zero_temp_sad < thresh_zero_sad_samples
    {
        (*rc).high_source_sad = 1 as core::ffi::c_int;
    } else {
        (*rc).high_source_sad = 0 as core::ffi::c_int;
    }
    (*rc).avg_source_sad = (3 as uint64_t)
        .wrapping_mul((*rc).avg_source_sad)
        .wrapping_add(avg_sad) >> 2 as core::ffi::c_int;
    (*rc).frame_source_sad = avg_sad;
    if num_samples > 0 as core::ffi::c_int {
        (*rc).percent_blocks_with_motion = (num_samples - num_zero_temp_sad)
            * 100 as core::ffi::c_int / num_samples;
    }
    if (*rc).frame_source_sad > 0 as uint64_t {
        (*rc).static_since_last_scene_change = 0 as core::ffi::c_int;
    }
    if (*rc).high_source_sad != 0 {
        (*cpi).rc.frames_since_scene_change = 0 as core::ffi::c_int;
        (*rc).static_since_last_scene_change = 1 as core::ffi::c_int;
    }
    let scale: core::ffi::c_int = if (*unscaled_src)
        .c2rust_unnamed
        .c2rust_unnamed
        .y_width * (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
        > 1920 as core::ffi::c_int * 1080 as core::ffi::c_int
    {
        30 as core::ffi::c_int
    } else {
        10 as core::ffi::c_int
    };
    let thresh_high_motion: uint64_t = (scale * 64 as core::ffi::c_int
        * 64 as core::ffi::c_int) as uint64_t;
    if (*cpi).svc.temporal_layer_id == 0 as core::ffi::c_int
        && (*rc).drop_count_consec < 3 as core::ffi::c_int
    {
        (*cpi).rc.high_motion_content_screen_rtc = 0 as core::ffi::c_int;
        if (*cpi).oxcf.speed >= 11 as core::ffi::c_int
            && is_one_pass_rt_lag_params(cpi) == 0
            && (*cpi).oxcf.tune_cfg.content as core::ffi::c_uint
                == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint
            && (*rc).num_col_blscroll_last_tl0 < 5 as core::ffi::c_int
            && (*rc).num_row_blscroll_last_tl0 < 5 as core::ffi::c_int
            && (*rc).percent_blocks_with_motion > 40 as core::ffi::c_int
            && (*rc).prev_avg_source_sad > thresh_high_motion
            && (*rc).avg_source_sad > thresh_high_motion
            && ((*rc).frame_number_encoded).wrapping_sub((*rc).last_frame_low_source_sad)
                > 12 as core::ffi::c_uint
            && (*rc).avg_frame_low_motion < 60 as core::ffi::c_int
            && (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width
                >= 1280 as core::ffi::c_int
            && (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                >= 720 as core::ffi::c_int
        {
            (*cpi).rc.high_motion_content_screen_rtc = 1 as core::ffi::c_int;
            if (*(*cm).seq_params).bit_depth as core::ffi::c_uint
                == 8 as core::ffi::c_uint
            {
                let mut sw_row: core::ffi::c_int = if (*cpi).rc.frame_source_sad
                    > 20000 as uint64_t
                {
                    512 as core::ffi::c_int
                } else {
                    192 as core::ffi::c_int
                };
                let mut sw_col: core::ffi::c_int = if (*cpi).rc.frame_source_sad
                    > 20000 as uint64_t
                {
                    512 as core::ffi::c_int
                } else {
                    160 as core::ffi::c_int
                };
                if (*cm).width * (*cm).height
                    >= 3840 as core::ffi::c_int * 2160 as core::ffi::c_int
                    && (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int
                {
                    sw_row = sw_row << 1 as core::ffi::c_int;
                    sw_col = sw_col << 1 as core::ffi::c_int;
                }
                let num_pts: core::ffi::c_int = if (*unscaled_src)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .y_width * (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                    >= 1920 as core::ffi::c_int * 1080 as core::ffi::c_int
                {
                    3 as core::ffi::c_int
                } else {
                    1 as core::ffi::c_int
                };
                let mut pts: core::ffi::c_int = 0 as core::ffi::c_int;
                while pts < num_pts {
                    let mut fac: core::ffi::c_int = 1 as core::ffi::c_int;
                    let mut shift: core::ffi::c_int = 1 as core::ffi::c_int;
                    if pts == 1 as core::ffi::c_int {
                        fac = 1 as core::ffi::c_int;
                        shift = 2 as core::ffi::c_int;
                    } else if pts == 2 as core::ffi::c_int {
                        fac = 3 as core::ffi::c_int;
                        shift = 2 as core::ffi::c_int;
                    }
                    let mut pos_col: core::ffi::c_int = (fac
                        * (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width >> shift)
                        - 64 as core::ffi::c_int;
                    let mut pos_row: core::ffi::c_int = (fac
                        * (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                        >> shift) - 64 as core::ffi::c_int;
                    pos_col = if sw_col
                        > (if ((*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width
                            - sw_col - 1 as core::ffi::c_int) < pos_col
                        {
                            (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width
                                - sw_col - 1 as core::ffi::c_int
                        } else {
                            pos_col
                        })
                    {
                        sw_col
                    } else if ((*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width
                        - sw_col - 1 as core::ffi::c_int) < pos_col
                    {
                        (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width - sw_col
                            - 1 as core::ffi::c_int
                    } else {
                        pos_col
                    };
                    pos_row = if sw_row
                        > (if ((*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                            - sw_row - 1 as core::ffi::c_int) < pos_row
                        {
                            (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                                - sw_row - 1 as core::ffi::c_int
                        } else {
                            pos_row
                        })
                    {
                        sw_row
                    } else if ((*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                        - sw_row - 1 as core::ffi::c_int) < pos_row
                    {
                        (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height - sw_row
                            - 1 as core::ffi::c_int
                    } else {
                        pos_row
                    };
                    if pos_col >= 0 as core::ffi::c_int
                        && pos_col
                            < (*unscaled_src).c2rust_unnamed.c2rust_unnamed.y_width
                                - 64 as core::ffi::c_int && pos_row >= 0 as core::ffi::c_int
                        && pos_row
                            < (*unscaled_src).c2rust_unnamed_0.c2rust_unnamed.y_height
                                - 64 as core::ffi::c_int
                    {
                        src_y = ((*unscaled_src)
                            .c2rust_unnamed_4
                            .c2rust_unnamed
                            .y_buffer)
                            .offset((pos_row * src_ystride) as isize)
                            .offset(pos_col as isize);
                        last_src_y = ((*unscaled_last_src)
                            .c2rust_unnamed_4
                            .c2rust_unnamed
                            .y_buffer)
                            .offset((pos_row * last_src_ystride) as isize)
                            .offset(pos_col as isize);
                        let mut best_intmv_col: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut best_intmv_row: core::ffi::c_int = 0 as core::ffi::c_int;
                        let mut y_sad: core::ffi::c_uint = estimate_scroll_motion(
                            cpi,
                            src_y,
                            last_src_y,
                            src_ystride,
                            last_src_ystride,
                            BLOCK_128X128,
                            pos_col,
                            pos_row,
                            &raw mut best_intmv_col,
                            &raw mut best_intmv_row,
                            sw_col,
                            sw_row,
                        );
                        let mut sad_thresh: core::ffi::c_uint = (if abs(best_intmv_col)
                            > 150 as core::ffi::c_int
                            || abs(best_intmv_row) > 150 as core::ffi::c_int
                        {
                            300 as core::ffi::c_int
                        } else {
                            150 as core::ffi::c_int
                        }) as core::ffi::c_uint;
                        if y_sad < sad_thresh
                            && (abs(best_intmv_col) > 16 as core::ffi::c_int
                                || abs(best_intmv_row) > 16 as core::ffi::c_int)
                        {
                            (*cpi).rc.high_motion_content_screen_rtc = 0
                                as core::ffi::c_int;
                            break;
                        }
                    }
                    pts += 1;
                }
            }
        }
        if (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int
            || (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int
        {
            let mut svc: *mut SVC = &raw mut (*cpi).svc;
            let mut sl: core::ffi::c_int = 0 as core::ffi::c_int;
            while sl < (*svc).number_spatial_layers {
                let mut tl: core::ffi::c_int = 1 as core::ffi::c_int;
                while tl < (*svc).number_temporal_layers {
                    let layer: core::ffi::c_int = sl * (*svc).number_temporal_layers
                        + tl;
                    let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                        .offset(layer as isize) as *mut LAYER_CONTEXT;
                    let mut lrc: *mut RATE_CONTROL = &raw mut (*lc).rc;
                    (*lrc).high_motion_content_screen_rtc = (*rc)
                        .high_motion_content_screen_rtc;
                    tl += 1;
                }
                sl += 1;
            }
        }
    }
    if (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int {
        let mut svc_0: *mut SVC = &raw mut (*cpi).svc;
        let mut sl_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while sl_0 < (*svc_0).number_spatial_layers {
            let mut tl_0: core::ffi::c_int = (*svc_0).temporal_layer_id;
            let layer_0: core::ffi::c_int = sl_0 * (*svc_0).number_temporal_layers
                + tl_0;
            let mut lc_0: *mut LAYER_CONTEXT = &raw mut *((*svc_0).layer_context)
                .offset(layer_0 as isize) as *mut LAYER_CONTEXT;
            let mut lrc_0: *mut RATE_CONTROL = &raw mut (*lc_0).rc;
            (*lrc_0).high_source_sad = (*rc).high_source_sad;
            (*lrc_0).frame_source_sad = (*rc).frame_source_sad;
            (*lrc_0).avg_source_sad = (*rc).avg_source_sad;
            (*lrc_0).percent_blocks_with_motion = (*rc).percent_blocks_with_motion;
            (*lrc_0).max_block_source_sad = (*rc).max_block_source_sad;
            sl_0 += 1;
        }
    }
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
unsafe extern "C-unwind" fn rc_spatial_act_onepass_rt(
    mut cpi: *mut AV1_COMP,
    mut src_y: *mut uint8_t,
    mut src_ystride: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let mut num_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let mut num_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    let bsize: BLOCK_SIZE = BLOCK_64X64;
    let mut avg_variance: uint64_t = 0 as uint64_t;
    let mut num_samples: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_zero_var_blocks: core::ffi::c_int = 0 as core::ffi::c_int;
    (*cpi).rc.perc_spatial_flat_blocks = 0 as core::ffi::c_int;
    let sb_size_by_mb: core::ffi::c_int = if (*(*cm).seq_params).sb_size
        as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
    {
        (*(*cm).seq_params).mib_size >> 1 as core::ffi::c_int
    } else {
        (*(*cm).seq_params).mib_size
    };
    let sb_cols: core::ffi::c_int = (num_mi_cols + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let sb_rows: core::ffi::c_int = (num_mi_rows + sb_size_by_mb - 1 as core::ffi::c_int)
        / sb_size_by_mb;
    let mut sbi_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while sbi_row < sb_rows {
        let mut sbi_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while sbi_col < sb_cols {
            let mut sse: core::ffi::c_uint = 0;
            let var: core::ffi::c_uint = ((*(*cpi).ppi).fn_ptr[bsize as usize].vf)
                .expect(
                    "non-null function pointer",
                )(
                src_y,
                src_ystride,
                core::ptr::addr_of!(AV1_VAR_OFFS).cast::<uint8_t>(),
                0 as core::ffi::c_int,
                &raw mut sse,
            ) as core::ffi::c_uint;
            avg_variance = avg_variance.wrapping_add(var as uint64_t);
            num_samples += 1;
            if var == 0 as core::ffi::c_uint {
                num_zero_var_blocks += 1;
            }
            src_y = src_y.offset(64 as core::ffi::c_int as isize);
            sbi_col += 1;
        }
        src_y = src_y
            .offset(
                ((src_ystride << 6 as core::ffi::c_int)
                    - (sb_cols << 6 as core::ffi::c_int)) as isize,
            );
        sbi_row += 1;
    }
    if num_samples > 0 as core::ffi::c_int {
        (*cpi).rc.perc_spatial_flat_blocks = 100 as core::ffi::c_int
            * num_zero_var_blocks / num_samples;
        avg_variance = avg_variance.wrapping_div(num_samples as uint64_t);
    }
    (*cpi).rc.frame_spatial_variance = avg_variance >> 12 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_gf_interval_update_onepass_rt(
    mut cpi: *mut AV1_COMP,
    mut frame_type: FRAME_TYPE,
) -> core::ffi::c_int {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let mut gf_update: core::ffi::c_int = 0 as core::ffi::c_int;
    let resize_pending: core::ffi::c_int = is_frame_resize_pending(cpi)
        as core::ffi::c_int;
    if (resize_pending != 0 || (*rc).high_source_sad != 0
        || (*rc).frames_till_gf_update_due == 0 as core::ffi::c_int)
        && (*cpi).svc.temporal_layer_id == 0 as core::ffi::c_int
        && (*cpi).svc.spatial_layer_id == 0 as core::ffi::c_int
    {
        set_baseline_gf_interval(cpi, frame_type);
        gf_update = 1 as core::ffi::c_int;
    }
    return gf_update;
}
unsafe extern "C-unwind" fn resize_reset_rc(
    mut cpi: *mut AV1_COMP,
    mut resize_width: core::ffi::c_int,
    mut resize_height: core::ffi::c_int,
    mut prev_width: core::ffi::c_int,
    mut prev_height: core::ffi::c_int,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let svc: *mut SVC = &raw mut (*cpi).svc;
    let mut target_bits_per_frame: core::ffi::c_int = 0;
    let mut active_worst_quality: core::ffi::c_int = 0;
    let mut qindex: core::ffi::c_int = 0;
    let mut tot_scale_change: core::ffi::c_double = (resize_width * resize_height)
        as core::ffi::c_double / (prev_width * prev_height) as core::ffi::c_double;
    (*svc).skip_mvsearch_last = 0 as core::ffi::c_int;
    (*svc).skip_mvsearch_gf = 0 as core::ffi::c_int;
    (*svc).skip_mvsearch_altref = 0 as core::ffi::c_int;
    (*p_rc).buffer_level = (*p_rc).optimal_buffer_level;
    (*p_rc).bits_off_target = (*p_rc).optimal_buffer_level;
    (*rc).this_frame_target = av1_calc_pframe_target_size_one_pass_cbr(
        cpi,
        INTER_FRAME as core::ffi::c_int as FRAME_UPDATE_TYPE,
    );
    target_bits_per_frame = (*rc).this_frame_target;
    if tot_scale_change > 4.0f64 {
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (*rc)
            .worst_quality;
    } else if tot_scale_change > 1.0f64 {
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (*p_rc)
            .avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize]
            + (*rc).worst_quality >> 1 as core::ffi::c_int;
    }
    active_worst_quality = calc_active_worst_quality_no_stats_cbr(cpi);
    qindex = av1_rc_regulate_q(
        cpi,
        target_bits_per_frame,
        (*rc).best_quality,
        active_worst_quality,
        resize_width,
        resize_height,
    );
    if tot_scale_change < 1.0f64
        && qindex
            > 90 as core::ffi::c_int * (*rc).worst_quality / 100 as core::ffi::c_int
    {
        (*p_rc).rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize]
            *= 0.85f64;
    }
    if tot_scale_change >= 1.0f64 {
        if tot_scale_change < 4.0f64
            && qindex
                > 130 as core::ffi::c_int
                    * (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize]
                    / 100 as core::ffi::c_int
        {
            (*p_rc).rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize]
                *= 0.8f64;
        }
        if qindex
            <= 120 as core::ffi::c_int
                * (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize]
                / 100 as core::ffi::c_int
        {
            (*p_rc).rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize]
                *= 1.5f64;
        }
    }
    if (*svc).number_temporal_layers > 1 as core::ffi::c_int {
        let mut tl: core::ffi::c_int = 0 as core::ffi::c_int;
        while tl < (*svc).number_temporal_layers {
            let mut lc: *mut LAYER_CONTEXT = 0 as *mut LAYER_CONTEXT;
            lc = &raw mut *((*svc).layer_context)
                .offset(
                    ((*svc).spatial_layer_id * (*svc).number_temporal_layers + tl)
                        as isize,
                ) as *mut LAYER_CONTEXT;
            (*lc).rc.resize_state = (*rc).resize_state;
            (*lc).p_rc.buffer_level = (*lc).p_rc.optimal_buffer_level;
            (*lc).p_rc.bits_off_target = (*lc).p_rc.optimal_buffer_level;
            (*lc)
                .p_rc
                .rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize] = (*p_rc)
                .rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize];
            (*lc).p_rc.avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (*p_rc)
                .avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize];
            tl += 1;
        }
    }
}
unsafe extern "C-unwind" fn dynamic_resize_one_pass_cbr(
    mut cpi: *mut AV1_COMP,
    mut one_half_only: core::ffi::c_int,
) {
    let cm: *const AV1_COMMON = &raw mut (*cpi).common;
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut resize_action: RESIZE_ACTION = NO_RESIZE;
    let avg_qp_thr1: core::ffi::c_int = 70 as core::ffi::c_int;
    let avg_qp_thr2: core::ffi::c_int = 50 as core::ffi::c_int;
    let min_width: core::ffi::c_int = 160 as core::ffi::c_int * 4 as core::ffi::c_int
        / 3 as core::ffi::c_int;
    let min_height: core::ffi::c_int = 90 as core::ffi::c_int * 4 as core::ffi::c_int
        / 3 as core::ffi::c_int;
    let mut down_size_on: core::ffi::c_int = 1 as core::ffi::c_int;
    if (*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
    {
        (*rc).resize_avg_qp = 0 as core::ffi::c_int;
        (*rc).resize_count = 0 as core::ffi::c_int;
        (*rc).resize_buffer_underflow = 0 as core::ffi::c_int;
        return;
    }
    if (*cm).width * (*cm).height < min_width * min_height {
        down_size_on = 0 as core::ffi::c_int;
    }
    if (*rc).frames_since_key as core::ffi::c_double > (*cpi).framerate
        && (*rc).high_source_sad == 0
    {
        let window: core::ffi::c_int = if 60 as core::ffi::c_int
            > (3 as core::ffi::c_int as core::ffi::c_double * (*cpi).framerate)
                as core::ffi::c_int
        {
            60 as core::ffi::c_int
        } else {
            (3 as core::ffi::c_int as core::ffi::c_double * (*cpi).framerate)
                as core::ffi::c_int
        };
        (*rc).resize_avg_qp += (*p_rc).last_q[INTER_FRAME as core::ffi::c_int as usize];
        if (*(*cpi).ppi).p_rc.buffer_level
            < (30 as int64_t * (*p_rc).optimal_buffer_level / 100 as int64_t)
                as core::ffi::c_int as int64_t
        {
            (*rc).resize_buffer_underflow += 1;
        }
        (*rc).resize_count += 1;
        if (*rc).resize_count >= window {
            let mut avg_qp: core::ffi::c_int = (*rc).resize_avg_qp / (*rc).resize_count;
            if (*rc).resize_buffer_underflow
                > (*rc).resize_count >> 2 as core::ffi::c_int && down_size_on != 0
            {
                if (*rc).resize_state as core::ffi::c_uint
                    == THREE_QUARTER as core::ffi::c_int as core::ffi::c_uint
                {
                    resize_action = DOWN_ONEHALF;
                    (*rc).resize_state = ONE_HALF;
                } else if (*rc).resize_state as core::ffi::c_uint
                    == ORIG as core::ffi::c_int as core::ffi::c_uint
                {
                    resize_action = (if one_half_only != 0 {
                        DOWN_ONEHALF as core::ffi::c_int
                    } else {
                        DOWN_THREEFOUR as core::ffi::c_int
                    }) as RESIZE_ACTION;
                    (*rc).resize_state = (if one_half_only != 0 {
                        ONE_HALF as core::ffi::c_int
                    } else {
                        THREE_QUARTER as core::ffi::c_int
                    }) as RESIZE_STATE;
                }
            } else if (*rc).resize_state as core::ffi::c_uint
                != ORIG as core::ffi::c_int as core::ffi::c_uint
                && avg_qp
                    < avg_qp_thr1 * (*cpi).rc.worst_quality / 100 as core::ffi::c_int
            {
                if (*rc).resize_state as core::ffi::c_uint
                    == THREE_QUARTER as core::ffi::c_int as core::ffi::c_uint
                    || avg_qp
                        < avg_qp_thr2 * (*cpi).rc.worst_quality / 100 as core::ffi::c_int
                    || one_half_only != 0
                {
                    resize_action = UP_ORIG;
                    (*rc).resize_state = ORIG;
                } else if (*rc).resize_state as core::ffi::c_uint
                    == ONE_HALF as core::ffi::c_int as core::ffi::c_uint
                {
                    resize_action = UP_THREEFOUR;
                    (*rc).resize_state = THREE_QUARTER;
                }
            }
            (*rc).resize_avg_qp = 0 as core::ffi::c_int;
            (*rc).resize_count = 0 as core::ffi::c_int;
            (*rc).resize_buffer_underflow = 0 as core::ffi::c_int;
        }
    }
    if resize_action as core::ffi::c_int != NO_RESIZE as core::ffi::c_int {
        let mut resize_width: core::ffi::c_int = (*cpi).oxcf.frm_dim_cfg.width;
        let mut resize_height: core::ffi::c_int = (*cpi).oxcf.frm_dim_cfg.height;
        let mut resize_scale_num: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut resize_scale_den: core::ffi::c_int = 1 as core::ffi::c_int;
        if resize_action as core::ffi::c_int == DOWN_THREEFOUR as core::ffi::c_int
            || resize_action as core::ffi::c_int == UP_THREEFOUR as core::ffi::c_int
        {
            resize_scale_num = 3 as core::ffi::c_int;
            resize_scale_den = 4 as core::ffi::c_int;
        } else if resize_action as core::ffi::c_int == DOWN_ONEHALF as core::ffi::c_int {
            resize_scale_num = 1 as core::ffi::c_int;
            resize_scale_den = 2 as core::ffi::c_int;
        }
        resize_width = resize_width * resize_scale_num / resize_scale_den;
        resize_height = resize_height * resize_scale_num / resize_scale_den;
        resize_reset_rc(cpi, resize_width, resize_height, (*cm).width, (*cm).height);
    }
}
#[inline]
unsafe extern "C-unwind" fn set_key_frame(
    mut cpi: *mut AV1_COMP,
    mut frame_flags: core::ffi::c_uint,
) -> core::ffi::c_int {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let svc: *mut SVC = &raw mut (*cpi).svc;
    if (*cm).current_frame.frame_number == 0 as core::ffi::c_uint {
        return 1 as core::ffi::c_int;
    }
    if frame_flags & FRAMEFLAGS_KEY as core::ffi::c_int as core::ffi::c_uint != 0 {
        return 1 as core::ffi::c_int;
    }
    if (*(*cpi).ppi).use_svc == 0 {
        if (*cpi).oxcf.kf_cfg.auto_key as core::ffi::c_int != 0
            && (*rc).frames_to_key == 0 as core::ffi::c_int
        {
            return 1 as core::ffi::c_int;
        }
    } else if (*svc).spatial_layer_id == 0 as core::ffi::c_int
        && ((*cpi).oxcf.kf_cfg.auto_key as core::ffi::c_int != 0
            && ((*cpi).oxcf.kf_cfg.key_freq_max == 0 as core::ffi::c_int
                || ((*svc).current_superframe)
                    .wrapping_rem((*cpi).oxcf.kf_cfg.key_freq_max as core::ffi::c_uint)
                    == 0 as core::ffi::c_uint))
    {
        return 1 as core::ffi::c_int
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_flag_rps_bias_recovery_frame(cpi: *const AV1_COMP) -> bool {
    if (*(*cpi).ppi).rtc_ref.set_ref_frame_config != 0
        && (*cpi).svc.number_temporal_layers == 1 as core::ffi::c_int
        && (*cpi).svc.number_spatial_layers == 1 as core::ffi::c_int
        && (*(*cpi).ppi).rtc_ref.reference_was_previous_frame as core::ffi::c_int != 0
    {
        let mut min_dist: core::ffi::c_int = av1_svc_get_min_ref_dist(
            cpi as *const AV1_COMP,
        );
        if min_dist != INT_MAX && min_dist >= 4 as core::ffi::c_int {
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_one_pass_rt_params(
    mut cpi: *mut AV1_COMP,
    frame_type: *mut FRAME_TYPE,
    mut frame_input: *const EncodeFrameInput,
    mut frame_flags: core::ffi::c_uint,
) {
    let rc: *mut RATE_CONTROL = &raw mut (*cpi).rc;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let gf_group: *mut GF_GROUP = &raw mut (*(*cpi).ppi).gf_group;
    let svc: *mut SVC = &raw mut (*cpi).svc;
    let resize_pending_params: *mut ResizePendingParams = &raw mut (*cpi)
        .resize_pending_params;
    let mut target: core::ffi::c_int = 0;
    let layer: core::ffi::c_int = (*svc).spatial_layer_id * (*svc).number_temporal_layers
        + (*svc).temporal_layer_id;
    if (*cpi).oxcf.rc_cfg.max_consec_drop_ms > 0 as core::ffi::c_int {
        let mut framerate: core::ffi::c_double = if (*cpi).framerate
            > 1 as core::ffi::c_int as core::ffi::c_double
        {
            round((*cpi).framerate)
        } else {
            (*cpi).framerate
        };
        (*rc).max_consec_drop = saturate_cast_double_to_int(
            ceil(
                (*cpi).oxcf.rc_cfg.max_consec_drop_ms as core::ffi::c_double * framerate
                    / 1000 as core::ffi::c_int as core::ffi::c_double,
            ),
        );
    }
    if (*(*cpi).ppi).use_svc != 0 {
        av1_update_temporal_layer_framerate(cpi as *mut AV1_COMP);
        av1_restore_layer_context(cpi as *mut AV1_COMP);
    }
    (*(*cpi).ppi).rtc_ref.bias_recovery_frame = set_flag_rps_bias_recovery_frame(cpi);
    if set_key_frame(cpi, frame_flags) != 0 {
        *frame_type = KEY_FRAME as core::ffi::c_int as FRAME_TYPE;
        (*p_rc).this_key_frame_forced = ((*cm).current_frame.frame_number
            != 0 as core::ffi::c_uint && (*rc).frames_to_key == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        (*rc).frames_to_key = (*cpi).oxcf.kf_cfg.key_freq_max;
        (*p_rc).kf_boost = DEFAULT_KF_BOOST_RT;
        (*gf_group).update_type[(*cpi).gf_frame_index as usize] = KF_UPDATE
            as core::ffi::c_int as FRAME_UPDATE_TYPE;
        (*gf_group).frame_type[(*cpi).gf_frame_index as usize] = KEY_FRAME
            as core::ffi::c_int as FRAME_TYPE;
        (*gf_group).refbuf_state[(*cpi).gf_frame_index as usize] = REFBUF_RESET
            as core::ffi::c_int as REFBUF_STATE;
        if (*(*cpi).ppi).use_svc != 0 {
            if (*cm).current_frame.frame_number > 0 as core::ffi::c_uint {
                av1_svc_reset_temporal_layers(
                    cpi as *mut AV1_COMP,
                    1 as core::ffi::c_int,
                );
            }
            (*((*svc).layer_context).offset(layer as isize)).is_key_frame = 1
                as core::ffi::c_int;
        }
        (*rc).frame_number_encoded = 0 as core::ffi::c_uint;
        (*(*cpi).ppi).rtc_ref.non_reference_frame = 0 as core::ffi::c_int;
        (*rc).static_since_last_scene_change = 0 as core::ffi::c_int;
    } else {
        *frame_type = INTER_FRAME as core::ffi::c_int as FRAME_TYPE;
        (*gf_group).update_type[(*cpi).gf_frame_index as usize] = LF_UPDATE
            as core::ffi::c_int as FRAME_UPDATE_TYPE;
        (*gf_group).frame_type[(*cpi).gf_frame_index as usize] = INTER_FRAME
            as core::ffi::c_int as FRAME_TYPE;
        (*gf_group).refbuf_state[(*cpi).gf_frame_index as usize] = REFBUF_UPDATE
            as core::ffi::c_int as REFBUF_STATE;
        if (*(*cpi).ppi).use_svc != 0 {
            let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                .offset(layer as isize) as *mut LAYER_CONTEXT;
            (*lc).is_key_frame = if (*svc).spatial_layer_id == 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else {
                (*((*svc).layer_context).offset((*svc).temporal_layer_id as isize))
                    .is_key_frame
            };
        }
        if (*(*cpi).ppi).rtc_ref.set_ref_frame_config != 0 {
            let mut no_references_set: core::ffi::c_int = 1 as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
                if (*(*cpi).ppi).rtc_ref.reference[i as usize] != 0 {
                    no_references_set = 0 as core::ffi::c_int;
                    break;
                } else {
                    i += 1;
                }
            }
            if no_references_set != 0 {
                *frame_type = INTRA_ONLY_FRAME as core::ffi::c_int as FRAME_TYPE;
            }
        }
    }
    if (*cpi).active_map.enabled != 0
        && (*cpi).rc.percent_blocks_inactive == 100 as core::ffi::c_int
    {
        (*rc).frame_source_sad = 0 as uint64_t;
        (*rc).avg_source_sad = (3 as uint64_t)
            .wrapping_mul((*rc).avg_source_sad)
            .wrapping_add((*rc).frame_source_sad) >> 2 as core::ffi::c_int;
        (*rc).percent_blocks_with_motion = 0 as core::ffi::c_int;
        (*rc).high_source_sad = 0 as core::ffi::c_int;
    } else if (*cpi).sf.rt_sf.check_scene_detection != 0
        && (*svc).spatial_layer_id == 0 as core::ffi::c_int
    {
        if (*rc).prev_coded_width == (*cm).width
            && (*rc).prev_coded_height == (*cm).height
        {
            av1_rc_scene_detection_onepass_rt(cpi, frame_input);
        } else {
            aom_free((*cpi).src_sad_blk_64x64 as *mut core::ffi::c_void);
            (*cpi).src_sad_blk_64x64 = 0 as *mut uint64_t;
            (*cpi).src_sad_blk_alloc_size = 0 as core::ffi::c_int;
        }
    }
    if (*cpi).sf.rt_sf.rc_compute_spatial_var_sc_kf != 0
        && (*frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
            || (*rc).high_source_sad != 0)
        && (*svc).spatial_layer_id == 0 as core::ffi::c_int
        && (*(*cm).seq_params).use_highbitdepth == 0
        && (*cpi).oxcf.rc_cfg.max_intra_bitrate_pct > 0 as core::ffi::c_uint
    {
        rc_spatial_act_onepass_rt(
            cpi,
            (*(*frame_input).source).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
            (*(*frame_input).source).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        );
    }
    if (*cpi).oxcf.resize_cfg.resize_mode as core::ffi::c_int
        == RESIZE_DYNAMIC as core::ffi::c_int
    {
        if (*svc).number_spatial_layers == 1 as core::ffi::c_int
            && (*svc).temporal_layer_id == 0 as core::ffi::c_int
        {
            dynamic_resize_one_pass_cbr(cpi, 1 as core::ffi::c_int);
        }
        if (*rc).resize_state as core::ffi::c_uint
            == THREE_QUARTER as core::ffi::c_int as core::ffi::c_uint
        {
            (*resize_pending_params).width = 3 as core::ffi::c_int
                + (*cpi).oxcf.frm_dim_cfg.width * 3 as core::ffi::c_int
                >> 2 as core::ffi::c_int;
            (*resize_pending_params).height = 3 as core::ffi::c_int
                + (*cpi).oxcf.frm_dim_cfg.height * 3 as core::ffi::c_int
                >> 2 as core::ffi::c_int;
        } else if (*rc).resize_state as core::ffi::c_uint
            == ONE_HALF as core::ffi::c_int as core::ffi::c_uint
        {
            (*resize_pending_params).width = 1 as core::ffi::c_int
                + (*cpi).oxcf.frm_dim_cfg.width >> 1 as core::ffi::c_int;
            (*resize_pending_params).height = 1 as core::ffi::c_int
                + (*cpi).oxcf.frm_dim_cfg.height >> 1 as core::ffi::c_int;
        } else {
            (*resize_pending_params).width = (*cpi).oxcf.frm_dim_cfg.width;
            (*resize_pending_params).height = (*cpi).oxcf.frm_dim_cfg.height;
        }
    } else if is_frame_resize_pending(cpi) != 0 {
        resize_reset_rc(
            cpi,
            (*resize_pending_params).width,
            (*resize_pending_params).height,
            (*cm).width,
            (*cm).height,
        );
    }
    if (*rc).rtc_external_ratectrl == 0 {
        set_gf_interval_update_onepass_rt(cpi, *frame_type);
    }
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
    {
        if *frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
            || *frame_type as core::ffi::c_int == INTRA_ONLY_FRAME as core::ffi::c_int
        {
            target = av1_calc_iframe_target_size_one_pass_cbr(cpi);
        } else {
            target = av1_calc_pframe_target_size_one_pass_cbr(
                cpi,
                (*gf_group).update_type[(*cpi).gf_frame_index as usize],
            );
        }
    } else if *frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        || *frame_type as core::ffi::c_int == INTRA_ONLY_FRAME as core::ffi::c_int
    {
        target = av1_calc_iframe_target_size_one_pass_vbr(cpi);
    } else {
        target = av1_calc_pframe_target_size_one_pass_vbr(
            cpi,
            (*gf_group).update_type[(*cpi).gf_frame_index as usize],
        );
    }
    if (*cpi).oxcf.rc_cfg.mode as core::ffi::c_uint
        == AOM_Q as core::ffi::c_int as core::ffi::c_uint
    {
        (*rc).active_worst_quality = (*cpi).oxcf.rc_cfg.cq_level;
    }
    av1_rc_set_frame_target(cpi, target, (*cm).width, (*cm).height);
    (*rc).base_frame_target = target;
    (*cm).current_frame.frame_type = *frame_type;
    if (*(*cpi).ppi).use_svc != 0 && (*svc).use_flexible_mode == 0
        && (*svc).ksvc_fixed_mode != 0
        && (*svc).number_spatial_layers > 1 as core::ffi::c_int
        && (*((*svc).layer_context).offset(layer as isize)).is_key_frame == 0
    {
        let ext_flags: *mut ExternalFlags = &raw mut (*cpi).ext_flags;
        (*ext_flags).ref_frame_flags ^= AOM_GOLD_FLAG as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_encodedframe_overshoot_cbr(
    mut cpi: *mut AV1_COMP,
    mut q: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*cpi).common;
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut rate_correction_factor: core::ffi::c_double = (*(*cpi).ppi)
        .p_rc
        .rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize];
    let target_size: core::ffi::c_int = (*cpi).rc.avg_frame_bandwidth;
    let mut new_correction_factor: core::ffi::c_double = 0.;
    let mut target_bits_per_mb: core::ffi::c_int = 0;
    let mut q2: core::ffi::c_double = 0.;
    let mut enumerator: core::ffi::c_int = 0;
    let mut inter_layer_pred_on: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut is_screen_content: core::ffi::c_int = ((*cpi).oxcf.tune_cfg.content
        as core::ffi::c_uint
        == AOM_CONTENT_SCREEN as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    (*(*cpi).cyclic_refresh).counter_encode_maxq_scene_change = 0 as core::ffi::c_int;
    if (*cpi).svc.spatial_layer_id > 0 as core::ffi::c_int {
        inter_layer_pred_on = ((*cpi).ref_frame_flags
            & av1_ref_frame_flag_list[LAST_FRAME as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
            && av1_check_ref_is_low_spatial_res_super_frame(
                cpi as *mut AV1_COMP,
                LAST_FRAME as core::ffi::c_int,
            ) as core::ffi::c_int != 0
            || (*cpi).ref_frame_flags
                & av1_ref_frame_flag_list[GOLDEN_FRAME as core::ffi::c_int as usize]
                    as core::ffi::c_int != 0
                && av1_check_ref_is_low_spatial_res_super_frame(
                    cpi as *mut AV1_COMP,
                    GOLDEN_FRAME as core::ffi::c_int,
                ) as core::ffi::c_int != 0
            || (*cpi).ref_frame_flags
                & av1_ref_frame_flag_list[ALTREF_FRAME as core::ffi::c_int as usize]
                    as core::ffi::c_int != 0
                && av1_check_ref_is_low_spatial_res_super_frame(
                    cpi as *mut AV1_COMP,
                    ALTREF_FRAME as core::ffi::c_int,
                ) as core::ffi::c_int != 0) as core::ffi::c_int;
    }
    if (*cpi).svc.spatial_layer_id > 0 as core::ffi::c_int && inter_layer_pred_on != 0 {
        *q = (*cpi).rc.worst_quality + *q >> 1 as core::ffi::c_int;
    } else if (*cpi).sf.rt_sf.rc_compute_spatial_var_sc_kf != 0 {
        if (*cpi).rc.frame_spatial_variance < 100 as uint64_t {
            *q = (*cpi).rc.worst_quality + *q >> 1 as core::ffi::c_int;
        } else if (*cpi).rc.frame_spatial_variance < 400 as uint64_t
            || (*cpi).rc.frame_source_sad < 80000 as uint64_t
                && (*cpi).rc.frame_spatial_variance < 1000 as uint64_t
        {
            *q = 3 as core::ffi::c_int * (*cpi).rc.worst_quality + *q
                >> 2 as core::ffi::c_int;
        } else {
            *q = (*cpi).rc.worst_quality;
        }
    } else {
        let sad_thr: uint64_t = (64 as core::ffi::c_int * 64 as core::ffi::c_int
            * 32 as core::ffi::c_int) as uint64_t;
        if (*cm).width * (*cm).height
            >= 1280 as core::ffi::c_int * 720 as core::ffi::c_int
            && (*p_rc).buffer_level
                > (*p_rc).optimal_buffer_level >> 1 as core::ffi::c_int
            && (*cpi).rc.avg_source_sad < sad_thr
        {
            *q = *q + (*cpi).rc.worst_quality >> 1 as core::ffi::c_int;
        } else {
            *q = 3 as core::ffi::c_int * (*cpi).rc.worst_quality + *q
                >> 2 as core::ffi::c_int;
        }
        if is_screen_content != 0 {
            *q = (*cpi).rc.worst_quality;
        }
    }
    (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = *q;
    (*p_rc).buffer_level = (*p_rc).optimal_buffer_level;
    (*p_rc).bits_off_target = (*p_rc).optimal_buffer_level;
    (*cpi).rc.rc_1_frame = 0 as core::ffi::c_int;
    (*cpi).rc.rc_2_frame = 0 as core::ffi::c_int;
    target_bits_per_mb = ((target_size as uint64_t) << BPER_MB_NORMBITS)
        .wrapping_div((*cm).mi_params.MBs as uint64_t) as core::ffi::c_int;
    q2 = av1_convert_qindex_to_q(*q, (*(*cm).seq_params).bit_depth);
    enumerator = get_bpmb_enumerator(
        INTER_NORMAL as core::ffi::c_int as FRAME_TYPE,
        is_screen_content,
    );
    new_correction_factor = target_bits_per_mb as core::ffi::c_double * q2
        / enumerator as core::ffi::c_double;
    if new_correction_factor > rate_correction_factor {
        rate_correction_factor = (new_correction_factor + rate_correction_factor)
            / 2.0f64;
        if rate_correction_factor > MAX_BPB_FACTOR as core::ffi::c_double {
            rate_correction_factor = MAX_BPB_FACTOR as core::ffi::c_double;
        }
        (*(*cpi).ppi)
            .p_rc
            .rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize] = rate_correction_factor;
    }
    if (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int
        && ((*cpi).svc.spatial_layer_id == 0 as core::ffi::c_int
            || inter_layer_pred_on == 0 as core::ffi::c_int)
    {
        let mut svc: *mut SVC = &raw mut (*cpi).svc;
        let mut tl: core::ffi::c_int = 0 as core::ffi::c_int;
        while tl < (*svc).number_temporal_layers {
            let mut sl: core::ffi::c_int = (*svc).spatial_layer_id;
            let layer: core::ffi::c_int = sl * (*svc).number_temporal_layers + tl;
            let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                .offset(layer as isize) as *mut LAYER_CONTEXT;
            let mut lrc: *mut RATE_CONTROL = &raw mut (*lc).rc;
            let mut lp_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*lc).p_rc;
            (*lp_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = *q;
            (*lp_rc).buffer_level = (*lp_rc).optimal_buffer_level;
            (*lp_rc).bits_off_target = (*lp_rc).optimal_buffer_level;
            (*lrc).rc_1_frame = 0 as core::ffi::c_int;
            (*lrc).rc_2_frame = 0 as core::ffi::c_int;
            (*lp_rc)
                .rate_correction_factors[INTER_NORMAL as core::ffi::c_int as usize] = rate_correction_factor;
            tl += 1;
        }
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_postencode_drop_cbr(
    mut cpi: *mut AV1_COMP,
    mut size: *mut size_t,
) -> core::ffi::c_int {
    let p_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*(*cpi).ppi).p_rc;
    let mut frame_size: size_t = *size << 3 as core::ffi::c_int;
    let new_buffer_level: int64_t = (*p_rc).buffer_level
        + (*cpi).rc.avg_frame_bandwidth as int64_t - frame_size as int64_t;
    let qp_thresh: core::ffi::c_int = 3 as core::ffi::c_int
        * ((*cpi).rc.worst_quality >> 2 as core::ffi::c_int);
    let buffer_thresh: int64_t = (*p_rc).optimal_buffer_level >> 2 as core::ffi::c_int;
    if (*cpi).rc.high_source_sad == 0 && new_buffer_level < buffer_thresh
        && frame_size
            > (8 as core::ffi::c_uint)
                .wrapping_mul((*cpi).rc.avg_frame_bandwidth as core::ffi::c_uint)
                as size_t && (*cpi).common.quant_params.base_qindex < qp_thresh
    {
        *size = 0 as size_t;
        (*cpi).is_dropped_frame = true_0 != 0;
        restore_all_coding_context(cpi);
        av1_rc_postencode_update_drop_frame(cpi);
        (*cpi).rc.force_max_q = 1 as core::ffi::c_int;
        (*p_rc).avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (*cpi)
            .rc
            .worst_quality;
        (*p_rc).buffer_level = (*p_rc).optimal_buffer_level;
        (*p_rc).bits_off_target = (*p_rc).optimal_buffer_level;
        (*cpi).rc.rc_1_frame = 0 as core::ffi::c_int;
        (*cpi).rc.rc_2_frame = 0 as core::ffi::c_int;
        if (*cpi).svc.number_spatial_layers > 1 as core::ffi::c_int
            || (*cpi).svc.number_temporal_layers > 1 as core::ffi::c_int
        {
            let mut svc: *mut SVC = &raw mut (*cpi).svc;
            let mut sl: core::ffi::c_int = 0 as core::ffi::c_int;
            while sl < (*svc).number_spatial_layers {
                let mut tl: core::ffi::c_int = 0 as core::ffi::c_int;
                while tl < (*svc).number_temporal_layers {
                    let layer: core::ffi::c_int = sl * (*svc).number_temporal_layers
                        + tl;
                    let mut lc: *mut LAYER_CONTEXT = &raw mut *((*svc).layer_context)
                        .offset(layer as isize) as *mut LAYER_CONTEXT;
                    let mut lrc: *mut RATE_CONTROL = &raw mut (*lc).rc;
                    let mut lp_rc: *mut PRIMARY_RATE_CONTROL = &raw mut (*lc).p_rc;
                    (*lrc).force_max_q = 1 as core::ffi::c_int;
                    (*lp_rc)
                        .avg_frame_qindex[INTER_FRAME as core::ffi::c_int as usize] = (*cpi)
                        .rc
                        .worst_quality;
                    (*lp_rc).buffer_level = (*lp_rc).optimal_buffer_level;
                    (*lp_rc).bits_off_target = (*lp_rc).optimal_buffer_level;
                    (*lrc).rc_1_frame = 0 as core::ffi::c_int;
                    (*lrc).rc_2_frame = 0 as core::ffi::c_int;
                    tl += 1;
                }
                sl += 1;
            }
        }
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;

extern "C" {
    fn tanh(_: core::ffi::c_double) -> core::ffi::c_double;
    fn log10(_: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(_: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(_: core::ffi::c_double) -> core::ffi::c_double;
    fn ceil(_: core::ffi::c_double) -> core::ffi::c_double;
    fn round(_: core::ffi::c_double) -> core::ffi::c_double;
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn llabs(_: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
