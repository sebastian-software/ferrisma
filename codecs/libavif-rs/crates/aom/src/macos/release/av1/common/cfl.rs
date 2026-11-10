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

// Mechanically translated from upstream/aom/av1/common/cfl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type uintptr_t = usize;
pub type qm_val_t = uint8_t;
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
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
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
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const CFL_PRED_PLANES: C2RustUnnamed_0 = 2;
pub const CFL_PRED_V: C2RustUnnamed_0 = 1;
pub const CFL_PRED_U: C2RustUnnamed_0 = 0;
pub type CFL_PRED_TYPE = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const CFL_SIGNS: C2RustUnnamed_1 = 3;
pub const CFL_SIGN_POS: C2RustUnnamed_1 = 2;
pub const CFL_SIGN_NEG: C2RustUnnamed_1 = 1;
pub const CFL_SIGN_ZERO: C2RustUnnamed_1 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type MV_REFERENCE_FRAME = int8_t;
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
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_10,
    pub c2rust_unnamed_1: C2RustUnnamed_8,
    pub c2rust_unnamed_2: C2RustUnnamed_6,
    pub c2rust_unnamed_3: C2RustUnnamed_4,
    pub c2rust_unnamed_4: C2RustUnnamed_2,
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
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
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
pub type ENTROPY_CONTEXT = core::ffi::c_char;
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
pub type MACROBLOCKD = macroblockd;
pub type cfl_subsample_lbd_fn = Option<
    unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *mut uint16_t) -> (),
>;
pub type cfl_subsample_hbd_fn = Option<
    unsafe extern "C-unwind" fn(*const uint16_t, core::ffi::c_int, *mut uint16_t) -> (),
>;
pub type cfl_predict_hbd_fn = Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type cfl_subtract_average_fn = Option<
    unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
>;
pub type cfl_predict_lbd_fn = Option<
    unsafe extern "C-unwind" fn(
        *const int16_t,
        *mut uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clip_pixel(mut val: core::ffi::c_int) -> uint8_t {
    return (if val > 255 as core::ffi::c_int {
        255 as core::ffi::c_int
    } else if val < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        val
    }) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn clip_pixel_highbd(
    mut val: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> uint16_t {
    match bd {
        10 => {
            return clamp(val, 0 as core::ffi::c_int, 1023 as core::ffi::c_int)
                as uint16_t;
        }
        12 => {
            return clamp(val, 0 as core::ffi::c_int, 4095 as core::ffi::c_int)
                as uint16_t;
        }
        8 | _ => {
            return clamp(val, 0 as core::ffi::c_int, 255 as core::ffi::c_int) as uint16_t;
        }
    };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE_LOG2: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CFL_ALPHABET_SIZE: core::ffi::c_int = (1 as core::ffi::c_int)
    << CFL_ALPHABET_SIZE_LOG2;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
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
pub const CFL_BUF_LINE: core::ffi::c_int = 32 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn max_block_wide(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
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
unsafe extern "C-unwind" fn get_tx_size(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> TX_SIZE {
    if width == height {
        return get_sqr_tx_size(width);
    }
    if width < height {
        if width + width == height {
            match width {
                4 => return TX_4X8 as core::ffi::c_int as TX_SIZE,
                8 => return TX_8X16 as core::ffi::c_int as TX_SIZE,
                16 => return TX_16X32 as core::ffi::c_int as TX_SIZE,
                32 => return TX_32X64 as core::ffi::c_int as TX_SIZE,
                _ => {}
            }
        } else {
            match width {
                4 => return TX_4X16 as core::ffi::c_int as TX_SIZE,
                8 => return TX_8X32 as core::ffi::c_int as TX_SIZE,
                16 => return TX_16X64 as core::ffi::c_int as TX_SIZE,
                _ => {}
            }
        }
    } else if height + height == width {
        match height {
            4 => return TX_8X4 as core::ffi::c_int as TX_SIZE,
            8 => return TX_16X8 as core::ffi::c_int as TX_SIZE,
            16 => return TX_32X16 as core::ffi::c_int as TX_SIZE,
            32 => return TX_64X32 as core::ffi::c_int as TX_SIZE,
            _ => {}
        }
    } else {
        match height {
            4 => return TX_16X4 as core::ffi::c_int as TX_SIZE,
            8 => return TX_32X8 as core::ffi::c_int as TX_SIZE,
            16 => return TX_64X16 as core::ffi::c_int as TX_SIZE,
            _ => {}
        }
    }
    return TX_4X4 as core::ffi::c_int as TX_SIZE;
}
#[inline]
unsafe extern "C-unwind" fn get_scaled_luma_q0(
    mut alpha_q3: core::ffi::c_int,
    mut pred_buf_q3: int16_t,
) -> core::ffi::c_int {
    let mut scaled_luma_q6: core::ffi::c_int = alpha_q3
        * pred_buf_q3 as core::ffi::c_int;
    return if scaled_luma_q6 < 0 as core::ffi::c_int {
        -(-scaled_luma_q6
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int)
    } else {
        scaled_luma_q6
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn clear_cfl_dc_pred_cache_flags(mut cfl: *mut CFL_CTX) {
    (*cfl).use_dc_pred_cache = false_0 != 0;
    (*cfl).dc_pred_is_cached[CFL_PRED_U as core::ffi::c_int as usize] = false_0 != 0;
    (*cfl).dc_pred_is_cached[CFL_PRED_V as core::ffi::c_int as usize] = false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_init(
    mut cfl: *mut CFL_CTX,
    mut seq_params: *const SequenceHeader,
) {
    memset(
        &raw mut (*cfl).recon_buf_q3 as *mut [uint16_t; 1024] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint16_t; 1024]>() as size_t,
    );
    memset(
        &raw mut (*cfl).ac_buf_q3 as *mut [int16_t; 1024] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[int16_t; 1024]>() as size_t,
    );
    (*cfl).subsampling_x = (*seq_params).subsampling_x;
    (*cfl).subsampling_y = (*seq_params).subsampling_y;
    (*cfl).are_parameters_computed = 0 as core::ffi::c_int;
    (*cfl).store_y = 0 as core::ffi::c_int;
    clear_cfl_dc_pred_cache_flags(cfl);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_store_dc_pred(
    xd: *mut MACROBLOCKD,
    mut input: *const uint8_t,
    mut pred_plane: CFL_PRED_TYPE,
    mut width: core::ffi::c_int,
) {
    if is_cur_buf_hbd(xd) != 0 {
        let input_16: *mut uint16_t = ((input as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        memcpy(
            ((*xd).cfl.dc_pred_cache[pred_plane as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            input_16 as *const core::ffi::c_void,
            (width << 1 as core::ffi::c_int) as size_t,
        );
        return;
    }
    memcpy(
        ((*xd).cfl.dc_pred_cache[pred_plane as usize]).as_mut_ptr()
            as *mut core::ffi::c_void,
        input as *const core::ffi::c_void,
        width as size_t,
    );
}
unsafe extern "C-unwind" fn cfl_load_dc_pred_lbd(
    mut dc_pred_cache: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            dc_pred_cache as *const core::ffi::c_void,
            width as size_t,
        );
        dst = dst.offset(dst_stride as isize);
        j += 1;
    }
}
unsafe extern "C-unwind" fn cfl_load_dc_pred_hbd(
    mut dc_pred_cache: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let num_bytes: size_t = (width << 1 as core::ffi::c_int) as size_t;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            dc_pred_cache as *const core::ffi::c_void,
            num_bytes,
        );
        dst = dst.offset(dst_stride as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_load_dc_pred(
    xd: *mut MACROBLOCKD,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut pred_plane: CFL_PRED_TYPE,
) {
    let width: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let height: core::ffi::c_int = tx_size_high[tx_size as usize];
    if is_cur_buf_hbd(xd) != 0 {
        let mut dst_16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        cfl_load_dc_pred_hbd(
            ((*xd).cfl.dc_pred_cache[pred_plane as usize]).as_mut_ptr(),
            dst_16,
            dst_stride,
            width,
            height,
        );
        return;
    }
    cfl_load_dc_pred_lbd(
        ((*xd).cfl.dc_pred_cache[pred_plane as usize]).as_mut_ptr(),
        dst,
        dst_stride,
        width,
        height,
    );
}
#[inline]
unsafe extern "C-unwind" fn cfl_pad(
    mut cfl: *mut CFL_CTX,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let diff_width: core::ffi::c_int = width - (*cfl).buf_width;
    let diff_height: core::ffi::c_int = height - (*cfl).buf_height;
    if diff_width > 0 as core::ffi::c_int {
        let min_height: core::ffi::c_int = height - diff_height;
        let mut recon_buf_q3: *mut uint16_t = ((*cfl).recon_buf_q3)
            .as_mut_ptr()
            .offset((width - diff_width) as isize);
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < min_height {
            let last_pixel: uint16_t = *recon_buf_q3
                .offset(-(1 as core::ffi::c_int) as isize);
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < diff_width {
                *recon_buf_q3.offset(i as isize) = last_pixel;
                i += 1;
            }
            recon_buf_q3 = recon_buf_q3.offset(CFL_BUF_LINE as isize);
            j += 1;
        }
        (*cfl).buf_width = width;
    }
    if diff_height > 0 as core::ffi::c_int {
        let mut recon_buf_q3_0: *mut uint16_t = ((*cfl).recon_buf_q3)
            .as_mut_ptr()
            .offset(((height - diff_height) * CFL_BUF_LINE) as isize);
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < diff_height {
            let mut last_row_q3: *const uint16_t = recon_buf_q3_0
                .offset(-(CFL_BUF_LINE as isize));
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < width {
                *recon_buf_q3_0.offset(i_0 as isize) = *last_row_q3.offset(i_0 as isize);
                i_0 += 1;
            }
            recon_buf_q3_0 = recon_buf_q3_0.offset(CFL_BUF_LINE as isize);
            j_0 += 1;
        }
        (*cfl).buf_height = height;
    }
}
unsafe extern "C-unwind" fn subtract_average_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut round_offset: core::ffi::c_int,
    mut num_pel_log2: core::ffi::c_int,
) {
    let mut sum: core::ffi::c_int = round_offset;
    let mut recon: *const uint16_t = src;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            sum += *recon.offset(i as isize) as core::ffi::c_int;
            i += 1;
        }
        recon = recon.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
    let avg: core::ffi::c_int = sum >> num_pel_log2;
    let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while j_0 < height {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < width {
            *dst.offset(i_0 as isize) = (*src.offset(i_0 as isize) as core::ffi::c_int
                - avg) as int16_t;
            i_0 += 1;
        }
        src = src.offset(CFL_BUF_LINE as isize);
        dst = dst.offset(CFL_BUF_LINE as isize);
        j_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x32_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        512 as core::ffi::c_int,
        10 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x8_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        5 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x16_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        7 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_subtract_average_fn_c(
    mut tx_size: TX_SIZE,
) -> cfl_subtract_average_fn {
    static mut sub_avg: [cfl_subtract_average_fn; 19] = unsafe {
        [
            Some(
                cfl_subtract_average_4x4_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_8x8_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_16x16_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_32x32_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            None,
            Some(
                cfl_subtract_average_4x8_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_8x4_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_8x16_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_16x8_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_16x32_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_32x16_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            None,
            None,
            Some(
                cfl_subtract_average_4x16_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_16x4_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_8x32_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            Some(
                cfl_subtract_average_32x8_c
                    as unsafe extern "C-unwind" fn(*const uint16_t, *mut int16_t) -> (),
            ),
            None,
            None,
        ]
    };
    return sub_avg[(tx_size as core::ffi::c_int % TX_SIZES_ALL as core::ffi::c_int)
        as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x8_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        128 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x32_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        128 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x4_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        32 as core::ffi::c_int,
        6 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x16_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        6 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x16_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        256 as core::ffi::c_int,
        9 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x32_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        256 as core::ffi::c_int,
        9 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x8_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        64 as core::ffi::c_int,
        7 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x4_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        5 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x16_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        128 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_8x8_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        6 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subtract_average_4x4_c(
    mut src: *const uint16_t,
    mut dst: *mut int16_t,
) {
    subtract_average_c(
        src,
        dst,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn cfl_idx_to_alpha(
    mut alpha_idx: uint8_t,
    mut joint_sign: int8_t,
    mut pred_type: CFL_PRED_TYPE,
) -> core::ffi::c_int {
    let alpha_sign: core::ffi::c_int = if pred_type as core::ffi::c_int
        == CFL_PRED_U as core::ffi::c_int
    {
        (joint_sign as core::ffi::c_int + 1 as core::ffi::c_int) * 11 as core::ffi::c_int
            >> 5 as core::ffi::c_int
    } else {
        joint_sign as core::ffi::c_int + 1 as core::ffi::c_int
            - CFL_SIGNS as core::ffi::c_int as core::ffi::c_int
                * ((joint_sign as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 11 as core::ffi::c_int >> 5 as core::ffi::c_int)
    };
    if alpha_sign == CFL_SIGN_ZERO as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let abs_alpha_q3: core::ffi::c_int = if pred_type as core::ffi::c_int
        == CFL_PRED_U as core::ffi::c_int
    {
        alpha_idx as core::ffi::c_int >> CFL_ALPHABET_SIZE_LOG2
    } else {
        alpha_idx as core::ffi::c_int & CFL_ALPHABET_SIZE - 1 as core::ffi::c_int
    };
    return if alpha_sign == CFL_SIGN_POS as core::ffi::c_int {
        abs_alpha_q3 + 1 as core::ffi::c_int
    } else {
        -abs_alpha_q3 - 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn cfl_predict_lbd_c(
    mut ac_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *dst.offset(i as isize) = clip_pixel(
                get_scaled_luma_q0(alpha_q3, *ac_buf_q3.offset(i as isize))
                    + *dst.offset(i as isize) as core::ffi::c_int,
            );
            i += 1;
        }
        dst = dst.offset(dst_stride as isize);
        ac_buf_q3 = ac_buf_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_predict_lbd_fn_c(
    mut tx_size: TX_SIZE,
) -> cfl_predict_lbd_fn {
    static mut pred: [cfl_predict_lbd_fn; 19] = unsafe {
        [
            Some(
                cfl_predict_lbd_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            Some(
                cfl_predict_lbd_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_predict_lbd_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_lbd_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint8_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return pred[(tx_size as core::ffi::c_int % TX_SIZES_ALL as core::ffi::c_int)
        as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
) {
    cfl_predict_lbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn cfl_predict_hbd_c(
    mut ac_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *dst.offset(i as isize) = clip_pixel_highbd(
                get_scaled_luma_q0(alpha_q3, *ac_buf_q3.offset(i as isize))
                    + *dst.offset(i as isize) as core::ffi::c_int,
                bit_depth,
            );
            i += 1;
        }
        dst = dst.offset(dst_stride as isize);
        ac_buf_q3 = ac_buf_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x4_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x16_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_predict_hbd_fn_c(
    mut tx_size: TX_SIZE,
) -> cfl_predict_hbd_fn {
    static mut pred: [cfl_predict_hbd_fn; 19] = unsafe {
        [
            Some(
                cfl_predict_hbd_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            Some(
                cfl_predict_hbd_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_predict_hbd_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                cfl_predict_hbd_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const int16_t,
                        *mut uint16_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return pred[(tx_size as core::ffi::c_int % TX_SIZES_ALL as core::ffi::c_int)
        as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x8_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x32_c(
    mut pred_buf_q3: *const int16_t,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut alpha_q3: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    cfl_predict_hbd_c(
        pred_buf_q3,
        dst,
        dst_stride,
        alpha_q3,
        bd,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
unsafe extern "C-unwind" fn cfl_compute_parameters(xd: *mut MACROBLOCKD, mut tx_size: TX_SIZE) {
    let cfl: *mut CFL_CTX = &raw mut (*xd).cfl;
    cfl_pad(cfl, tx_size_wide[tx_size as usize], tx_size_high[tx_size as usize]);
    (cfl_get_subtract_average_fn_c(tx_size))
        .expect(
            "non-null function pointer",
        )(((*cfl).recon_buf_q3).as_mut_ptr(), ((*cfl).ac_buf_q3).as_mut_ptr());
    (*cfl).are_parameters_computed = 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cfl_predict_block(
    xd: *mut MACROBLOCKD,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut plane: core::ffi::c_int,
) {
    let cfl: *mut CFL_CTX = &raw mut (*xd).cfl;
    let mut mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    if (*cfl).are_parameters_computed == 0 {
        cfl_compute_parameters(xd, tx_size);
    }
    let alpha_q3: core::ffi::c_int = cfl_idx_to_alpha(
        (*mbmi).cfl_alpha_idx,
        (*mbmi).cfl_alpha_signs,
        (plane - 1 as core::ffi::c_int) as CFL_PRED_TYPE,
    ) as core::ffi::c_int;
    if is_cur_buf_hbd(xd) != 0 {
        let mut dst_16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        (cfl_get_predict_hbd_fn_c(tx_size))
            .expect(
                "non-null function pointer",
            )(((*cfl).ac_buf_q3).as_mut_ptr(), dst_16, dst_stride, alpha_q3, (*xd).bd);
        return;
    }
    (cfl_get_predict_lbd_fn_c(tx_size))
        .expect(
            "non-null function pointer",
        )(((*cfl).ac_buf_q3).as_mut_ptr(), dst, dst_stride, alpha_q3);
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_420_lbd_c(
    mut input: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            let bot: core::ffi::c_int = i + input_stride;
            *output_q3.offset((i >> 1 as core::ffi::c_int) as isize) = ((*input
                .offset(i as isize) as core::ffi::c_int
                + *input.offset((i + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                + *input.offset(bot as isize) as core::ffi::c_int
                + *input.offset((bot + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) << 1 as core::ffi::c_int) as uint16_t;
            i += 2 as core::ffi::c_int;
        }
        input = input.offset((input_stride << 1 as core::ffi::c_int) as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 2 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_422_lbd_c(
    mut input: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *output_q3.offset((i >> 1 as core::ffi::c_int) as isize) = ((*input
                .offset(i as isize) as core::ffi::c_int
                + *input.offset((i + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) << 2 as core::ffi::c_int) as uint16_t;
            i += 2 as core::ffi::c_int;
        }
        input = input.offset(input_stride as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_444_lbd_c(
    mut input: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *output_q3.offset(i as isize) = ((*input.offset(i as isize)
                as core::ffi::c_int) << 3 as core::ffi::c_int) as uint16_t;
            i += 1;
        }
        input = input.offset(input_stride as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_420_hbd_c(
    mut input: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            let bot: core::ffi::c_int = i + input_stride;
            *output_q3.offset((i >> 1 as core::ffi::c_int) as isize) = ((*input
                .offset(i as isize) as core::ffi::c_int
                + *input.offset((i + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                + *input.offset(bot as isize) as core::ffi::c_int
                + *input.offset((bot + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) << 1 as core::ffi::c_int) as uint16_t;
            i += 2 as core::ffi::c_int;
        }
        input = input.offset((input_stride << 1 as core::ffi::c_int) as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 2 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_422_hbd_c(
    mut input: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *output_q3.offset((i >> 1 as core::ffi::c_int) as isize) = ((*input
                .offset(i as isize) as core::ffi::c_int
                + *input.offset((i + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) << 2 as core::ffi::c_int) as uint16_t;
            i += 2 as core::ffi::c_int;
        }
        input = input.offset(input_stride as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
unsafe extern "C-unwind" fn cfl_luma_subsampling_444_hbd_c(
    mut input: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < height {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < width {
            *output_q3.offset(i as isize) = ((*input.offset(i as isize)
                as core::ffi::c_int) << 3 as core::ffi::c_int) as uint16_t;
            i += 1;
        }
        input = input.offset(input_stride as isize);
        output_q3 = output_q3.offset(CFL_BUF_LINE as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_hbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_hbd_fn {
    static mut subfn_422: [cfl_subsample_hbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_hbd_422_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_hbd_422_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_hbd_422_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_422_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_422[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_lbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_lbd_fn {
    static mut subfn_422: [cfl_subsample_lbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_lbd_422_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_lbd_422_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_lbd_422_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_422_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_422[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_422_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_hbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_hbd_fn {
    static mut subfn_444: [cfl_subsample_hbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_hbd_444_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_hbd_444_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_hbd_444_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_444_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_444[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_lbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_lbd_fn {
    static mut subfn_444: [cfl_subsample_lbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_lbd_444_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_lbd_444_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_lbd_444_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_444_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_444[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_hbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_hbd_fn {
    static mut subfn_420: [cfl_subsample_hbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_hbd_420_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_hbd_420_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_hbd_420_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_hbd_420_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint16_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_420[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x32_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x16_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x4_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_lbd_c(
    mut tx_size: TX_SIZE,
) -> cfl_subsample_lbd_fn {
    static mut subfn_420: [cfl_subsample_lbd_fn; 19] = unsafe {
        [
            Some(
                cfl_subsample_lbd_420_4x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_8x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_16x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_32x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            Some(
                cfl_subsample_lbd_420_4x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_8x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_8x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_16x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_16x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_32x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
            Some(
                cfl_subsample_lbd_420_4x16_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_16x4_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_8x32_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            Some(
                cfl_subsample_lbd_420_32x8_c
                    as unsafe extern "C-unwind" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint16_t,
                    ) -> (),
            ),
            None,
            None,
        ]
    };
    return subfn_420[tx_size as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x4_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_420_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x32_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x8_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x16_c(
    mut cfl_type: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_lbd_c(
        cfl_type,
        input_stride,
        output_q3,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x8_c(
    mut cfl_type: *const uint16_t,
    mut input_stride: core::ffi::c_int,
    mut output_q3: *mut uint16_t,
) {
    cfl_luma_subsampling_444_hbd_c(
        cfl_type,
        input_stride,
        output_q3,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn cfl_subsampling_hbd(
    mut tx_size: TX_SIZE,
    mut sub_x: core::ffi::c_int,
    mut sub_y: core::ffi::c_int,
) -> cfl_subsample_hbd_fn {
    if sub_x == 1 as core::ffi::c_int {
        if sub_y == 1 as core::ffi::c_int {
            return cfl_get_luma_subsampling_420_hbd_c(tx_size);
        }
        return cfl_get_luma_subsampling_422_hbd_c(tx_size);
    }
    return cfl_get_luma_subsampling_444_hbd_c(tx_size);
}
#[inline]
unsafe extern "C-unwind" fn cfl_subsampling_lbd(
    mut tx_size: TX_SIZE,
    mut sub_x: core::ffi::c_int,
    mut sub_y: core::ffi::c_int,
) -> cfl_subsample_lbd_fn {
    if sub_x == 1 as core::ffi::c_int {
        if sub_y == 1 as core::ffi::c_int {
            return cfl_get_luma_subsampling_420_lbd_c(tx_size);
        }
        return cfl_get_luma_subsampling_422_lbd_c(tx_size);
    }
    return cfl_get_luma_subsampling_444_lbd_c(tx_size);
}
unsafe extern "C-unwind" fn cfl_store(
    mut cfl: *mut CFL_CTX,
    mut input: *const uint8_t,
    mut input_stride: core::ffi::c_int,
    mut row: core::ffi::c_int,
    mut col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut use_hbd: core::ffi::c_int,
) {
    let width: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let height: core::ffi::c_int = tx_size_high[tx_size as usize];
    let tx_off_log2: core::ffi::c_int = MI_SIZE_LOG2;
    let sub_x: core::ffi::c_int = (*cfl).subsampling_x;
    let sub_y: core::ffi::c_int = (*cfl).subsampling_y;
    let store_row: core::ffi::c_int = row << tx_off_log2 - sub_y;
    let store_col: core::ffi::c_int = col << tx_off_log2 - sub_x;
    let store_height: core::ffi::c_int = height >> sub_y;
    let store_width: core::ffi::c_int = width >> sub_x;
    (*cfl).are_parameters_computed = 0 as core::ffi::c_int;
    if col == 0 as core::ffi::c_int && row == 0 as core::ffi::c_int {
        (*cfl).buf_width = store_width;
        (*cfl).buf_height = store_height;
    } else {
        (*cfl).buf_width = if store_col + store_width > (*cfl).buf_width {
            store_col + store_width
        } else {
            (*cfl).buf_width
        };
        (*cfl).buf_height = if store_row + store_height > (*cfl).buf_height {
            store_row + store_height
        } else {
            (*cfl).buf_height
        };
    }
    let mut recon_buf_q3: *mut uint16_t = ((*cfl).recon_buf_q3)
        .as_mut_ptr()
        .offset((store_row * CFL_BUF_LINE + store_col) as isize);
    if use_hbd != 0 {
        (cfl_subsampling_hbd(tx_size, sub_x, sub_y))
            .expect(
                "non-null function pointer",
            )(
            ((input as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            input_stride,
            recon_buf_q3,
        );
    } else {
        (cfl_subsampling_lbd(tx_size, sub_x, sub_y))
            .expect("non-null function pointer")(input, input_stride, recon_buf_q3);
    };
}
#[inline]
unsafe extern "C-unwind" fn sub8x8_adjust_offset(
    mut cfl: *const CFL_CTX,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut row_out: *mut core::ffi::c_int,
    mut col_out: *mut core::ffi::c_int,
) {
    if mi_row & 0x1 as core::ffi::c_int != 0 && (*cfl).subsampling_y != 0 {
        *row_out += 1;
    }
    if mi_col & 0x1 as core::ffi::c_int != 0 && (*cfl).subsampling_x != 0 {
        *col_out += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_store_tx(
    xd: *mut MACROBLOCKD,
    mut row: core::ffi::c_int,
    mut col: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut bsize: BLOCK_SIZE,
) {
    let cfl: *mut CFL_CTX = &raw mut (*xd).cfl;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut dst: *mut uint8_t = &raw mut *((*pd).dst.buf)
        .offset((row * (*pd).dst.stride + col << MI_SIZE_LOG2) as isize) as *mut uint8_t;
    if block_size_high[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
        || block_size_wide[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
    {
        sub8x8_adjust_offset(cfl, (*xd).mi_row, (*xd).mi_col, &raw mut row, &raw mut col);
    }
    cfl_store(cfl, dst, (*pd).dst.stride, row, col, tx_size, is_cur_buf_hbd(xd));
}
#[inline]
unsafe extern "C-unwind" fn max_intra_block_width(
    mut xd: *const MACROBLOCKD,
    mut plane_bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    let max_blocks_wide: core::ffi::c_int = (max_block_wide(xd, plane_bsize, plane)
        as core::ffi::c_int) << MI_SIZE_LOG2;
    return max_blocks_wide
        + (((1 as core::ffi::c_int) << tx_size_wide_log2[tx_size as usize])
            - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << tx_size_wide_log2[tx_size as usize])
            - 1 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn max_intra_block_height(
    mut xd: *const MACROBLOCKD,
    mut plane_bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) -> core::ffi::c_int {
    let max_blocks_high: core::ffi::c_int = (max_block_high(xd, plane_bsize, plane)
        as core::ffi::c_int) << MI_SIZE_LOG2;
    return max_blocks_high
        + (((1 as core::ffi::c_int) << tx_size_high_log2[tx_size as usize])
            - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << tx_size_high_log2[tx_size as usize])
            - 1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_store_block(
    xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
) {
    let cfl: *mut CFL_CTX = &raw mut (*xd).cfl;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    if block_size_high[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
        || block_size_wide[bsize as usize] as core::ffi::c_int == 4 as core::ffi::c_int
    {
        sub8x8_adjust_offset(cfl, (*xd).mi_row, (*xd).mi_col, &raw mut row, &raw mut col);
    }
    let width: core::ffi::c_int = max_intra_block_width(xd, bsize, AOM_PLANE_Y, tx_size)
        as core::ffi::c_int;
    let height: core::ffi::c_int = max_intra_block_height(
        xd,
        bsize,
        AOM_PLANE_Y,
        tx_size,
    ) as core::ffi::c_int;
    tx_size = get_tx_size(width, height);
    cfl_store(
        cfl,
        (*pd).dst.buf,
        (*pd).dst.stride,
        row,
        col,
        tx_size,
        is_cur_buf_hbd(xd),
    );
}
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
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

#[cfg(feature = "port-test-bridges")]
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_inline_get_tx_size(
    width: core::ffi::c_int,
    height: core::ffi::c_int,
) -> TX_SIZE {
    get_tx_size(width, height)
}
