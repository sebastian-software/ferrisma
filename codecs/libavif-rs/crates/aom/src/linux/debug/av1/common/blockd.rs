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

// Mechanically translated from upstream/aom/av1/common/blockd.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type size_t = usize;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type qm_val_t = uint8_t;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed = 0;
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_0 = 25;
pub const INTRA_MODES: C2RustUnnamed_0 = 13;
pub const INTER_MODE_END: C2RustUnnamed_0 = 25;
pub const INTER_MODE_START: C2RustUnnamed_0 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_0 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_0 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_0 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_0 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_0 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_0 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_0 = 13;
pub const DIR_MODE_END: C2RustUnnamed_0 = 9;
pub const DIR_MODE_START: C2RustUnnamed_0 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_0 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_0 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_0 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_0 = 25;
pub const NEW_NEWMV: C2RustUnnamed_0 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_0 = 23;
pub const NEW_NEARMV: C2RustUnnamed_0 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_0 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_0 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_0 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_0 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_0 = 17;
pub const NEWMV: C2RustUnnamed_0 = 16;
pub const GLOBALMV: C2RustUnnamed_0 = 15;
pub const NEARMV: C2RustUnnamed_0 = 14;
pub const NEARESTMV: C2RustUnnamed_0 = 13;
pub const PAETH_PRED: C2RustUnnamed_0 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_0 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_0 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_0 = 9;
pub const D67_PRED: C2RustUnnamed_0 = 8;
pub const D203_PRED: C2RustUnnamed_0 = 7;
pub const D157_PRED: C2RustUnnamed_0 = 6;
pub const D113_PRED: C2RustUnnamed_0 = 5;
pub const D135_PRED: C2RustUnnamed_0 = 4;
pub const D45_PRED: C2RustUnnamed_0 = 3;
pub const H_PRED: C2RustUnnamed_0 = 2;
pub const V_PRED: C2RustUnnamed_0 = 1;
pub const DC_PRED: C2RustUnnamed_0 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_1 = 7;
pub const BWD_REFS: C2RustUnnamed_1 = 3;
pub const FWD_REFS: C2RustUnnamed_1 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_1 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_1 = 8;
pub const REF_FRAMES: C2RustUnnamed_1 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_1 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_1 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_1 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_1 = 4;
pub const LAST3_FRAME: C2RustUnnamed_1 = 3;
pub const LAST2_FRAME: C2RustUnnamed_1 = 2;
pub const LAST_FRAME: C2RustUnnamed_1 = 1;
pub const INTRA_FRAME: C2RustUnnamed_1 = 0;
pub const NONE_FRAME: C2RustUnnamed_1 = -1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [core::ffi::c_char; 40],
    pub __align: core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: core::ffi::c_int,
    pub __count: core::ffi::c_uint,
    pub __owner: core::ffi::c_int,
    pub __nusers: core::ffi::c_uint,
    pub __kind: core::ffi::c_int,
    pub __spins: core::ffi::c_short,
    pub __elision: core::ffi::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
pub type __jmp_buf = [core::ffi::c_long; 8];
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
#[inline]
unsafe extern "C-unwind" fn max_block_wide(
    mut xd: *const MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {} else {
        __assert_fail(
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_common_int.h\0" as *const u8
                as *const core::ffi::c_char,
            1569 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 57],
                [core::ffi::c_char; 57],
            >(*b"int max_block_wide(const MACROBLOCKD *, BLOCK_SIZE, int)\0"))
                .as_ptr(),
        );
    };
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
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_left_block_mode(
    mut left_mi: *const MB_MODE_INFO,
) -> PREDICTION_MODE {
    if left_mi.is_null() {
        return DC_PRED as core::ffi::c_int as PREDICTION_MODE;
    }
    if is_inter_block(left_mi) == 0 || is_intrabc_block(left_mi) != 0 {} else {
        __assert_fail(
            b"!is_inter_block(left_mi) || is_intrabc_block(left_mi)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.c\0" as *const u8
                as *const core::ffi::c_char,
            19 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 58],
                [core::ffi::c_char; 58],
            >(*b"PREDICTION_MODE av1_left_block_mode(const MB_MODE_INFO *)\0"))
                .as_ptr(),
        );
    };
    return (*left_mi).mode;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_above_block_mode(
    mut above_mi: *const MB_MODE_INFO,
) -> PREDICTION_MODE {
    if above_mi.is_null() {
        return DC_PRED as core::ffi::c_int as PREDICTION_MODE;
    }
    if is_inter_block(above_mi) == 0 || is_intrabc_block(above_mi) != 0 {} else {
        __assert_fail(
            b"!is_inter_block(above_mi) || is_intrabc_block(above_mi)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.c\0" as *const u8
                as *const core::ffi::c_char,
            25 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 59],
                [core::ffi::c_char; 59],
            >(*b"PREDICTION_MODE av1_above_block_mode(const MB_MODE_INFO *)\0"))
                .as_ptr(),
        );
    };
    return (*above_mi).mode;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_entropy_contexts(
    mut xd: *const MACROBLOCKD,
    mut pd: *mut macroblockd_plane,
    mut plane: core::ffi::c_int,
    mut plane_bsize: BLOCK_SIZE,
    mut tx_size: TX_SIZE,
    mut has_eob: core::ffi::c_int,
    mut aoff: core::ffi::c_int,
    mut loff: core::ffi::c_int,
) {
    let a: *mut ENTROPY_CONTEXT = ((*pd).above_entropy_context).offset(aoff as isize);
    let l: *mut ENTROPY_CONTEXT = ((*pd).left_entropy_context).offset(loff as isize);
    let txs_wide: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txs_high: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    if has_eob != 0 && (*xd).mb_to_right_edge < 0 as core::ffi::c_int {
        let blocks_wide: core::ffi::c_int = max_block_wide(xd, plane_bsize, plane)
            as core::ffi::c_int;
        let above_contexts: core::ffi::c_int = if txs_wide < blocks_wide - aoff {
            txs_wide
        } else {
            blocks_wide - aoff
        };
        memset(
            a as *mut core::ffi::c_void,
            has_eob,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(above_contexts as size_t),
        );
        memset(
            a.offset(above_contexts as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul((txs_wide - above_contexts) as size_t),
        );
    } else {
        memset(
            a as *mut core::ffi::c_void,
            has_eob,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(txs_wide as size_t),
        );
    }
    if has_eob != 0 && (*xd).mb_to_bottom_edge < 0 as core::ffi::c_int {
        let blocks_high: core::ffi::c_int = max_block_high(xd, plane_bsize, plane)
            as core::ffi::c_int;
        let left_contexts: core::ffi::c_int = if txs_high < blocks_high - loff {
            txs_high
        } else {
            blocks_high - loff
        };
        memset(
            l as *mut core::ffi::c_void,
            has_eob,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(left_contexts as size_t),
        );
        memset(
            l.offset(left_contexts as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul((txs_high - left_contexts) as size_t),
        );
    } else {
        memset(
            l as *mut core::ffi::c_void,
            has_eob,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(txs_high as size_t),
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_entropy_context(
    mut xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    num_planes: core::ffi::c_int,
) {
    if (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {} else {
        __assert_fail(
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.c\0" as *const u8
                as *const core::ffi::c_char,
            60 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 69],
                [core::ffi::c_char; 69],
            >(
                *b"void av1_reset_entropy_context(MACROBLOCKD *, BLOCK_SIZE, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let nplanes: core::ffi::c_int = 1 as core::ffi::c_int
        + (num_planes - 1 as core::ffi::c_int) * (*xd).is_chroma_ref as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < nplanes {
        let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(i as isize) as *mut macroblockd_plane;
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        let txs_wide: core::ffi::c_int = mi_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let txs_high: core::ffi::c_int = mi_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        memset(
            (*pd).above_entropy_context as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(txs_wide as size_t),
        );
        memset(
            (*pd).left_entropy_context as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ENTROPY_CONTEXT>() as size_t)
                .wrapping_mul(txs_high as size_t),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_loop_filter_delta(
    mut xd: *mut MACROBLOCKD,
    mut num_planes: core::ffi::c_int,
) {
    (*xd).delta_lf_from_base = 0 as int8_t;
    let frame_lf_count: core::ffi::c_int = if num_planes > 1 as core::ffi::c_int {
        FRAME_LF_COUNT
    } else {
        FRAME_LF_COUNT - 2 as core::ffi::c_int
    };
    let mut lf_id: core::ffi::c_int = 0 as core::ffi::c_int;
    while lf_id < frame_lf_count {
        (*xd).delta_lf[lf_id as usize] = 0 as int8_t;
        lf_id += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_reset_loop_restoration(
    mut xd: *mut MACROBLOCKD,
    num_planes: core::ffi::c_int,
) {
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < num_planes {
        set_default_wiener(((*xd).wiener_info).as_mut_ptr().offset(p as isize));
        set_default_sgrproj(((*xd).sgrproj_info).as_mut_ptr().offset(p as isize));
        p += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_block_planes(
    mut xd: *mut MACROBLOCKD,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        (*xd).plane[i as usize].plane_type = get_plane_type(i);
        (*xd).plane[i as usize].subsampling_x = if i != 0 {
            ss_x
        } else {
            0 as core::ffi::c_int
        };
        (*xd).plane[i as usize].subsampling_y = if i != 0 {
            ss_y
        } else {
            0 as core::ffi::c_int
        };
        i += 1;
    }
    i = num_planes;
    while i < MAX_MB_PLANE {
        (*xd).plane[i as usize].subsampling_x = 1 as core::ffi::c_int;
        (*xd).plane[i as usize].subsampling_y = 1 as core::ffi::c_int;
        i += 1;
    }
}
pub const WIENER_HALFWIN: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SGRPROJ_PRJ_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN0: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) * 3 as core::ffi::c_int / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MAX0: core::ffi::c_int = SGRPROJ_PRJ_MIN0
    + ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS) - 1 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MIN1: core::ffi::c_int = -((1 as core::ffi::c_int)
    << SGRPROJ_PRJ_BITS) / 4 as core::ffi::c_int;
pub const SGRPROJ_PRJ_MAX1: core::ffi::c_int = SGRPROJ_PRJ_MIN1
    + ((1 as core::ffi::c_int) << SGRPROJ_PRJ_BITS) - 1 as core::ffi::c_int;
pub const WIENER_FILT_TAP0_MIDV: core::ffi::c_int = 3 as core::ffi::c_int;
pub const WIENER_FILT_TAP1_MIDV: core::ffi::c_int = -(7 as core::ffi::c_int);
pub const WIENER_FILT_TAP2_MIDV: core::ffi::c_int = 15 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn set_default_sgrproj(mut sgrproj_info: *mut SgrprojInfo) {
    (*sgrproj_info).xqd[0 as core::ffi::c_int as usize] = (SGRPROJ_PRJ_MIN0
        + SGRPROJ_PRJ_MAX0) / 2 as core::ffi::c_int;
    (*sgrproj_info).xqd[1 as core::ffi::c_int as usize] = (SGRPROJ_PRJ_MIN1
        + SGRPROJ_PRJ_MAX1) / 2 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn set_default_wiener(mut wiener_info: *mut WienerInfo) {
    (*wiener_info).hfilter[0 as core::ffi::c_int as usize] = WIENER_FILT_TAP0_MIDV
        as int16_t;
    (*wiener_info).vfilter[0 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[0 as core::ffi::c_int as usize];
    (*wiener_info).hfilter[1 as core::ffi::c_int as usize] = WIENER_FILT_TAP1_MIDV
        as int16_t;
    (*wiener_info).vfilter[1 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[1 as core::ffi::c_int as usize];
    (*wiener_info).hfilter[2 as core::ffi::c_int as usize] = WIENER_FILT_TAP2_MIDV
        as int16_t;
    (*wiener_info).vfilter[2 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[2 as core::ffi::c_int as usize];
    (*wiener_info).hfilter[WIENER_HALFWIN as usize] = (-(2 as core::ffi::c_int)
        * (WIENER_FILT_TAP2_MIDV + WIENER_FILT_TAP1_MIDV + WIENER_FILT_TAP0_MIDV))
        as int16_t;
    (*wiener_info).vfilter[WIENER_HALFWIN as usize] = (*wiener_info)
        .hfilter[WIENER_HALFWIN as usize];
    (*wiener_info).hfilter[4 as core::ffi::c_int as usize] = WIENER_FILT_TAP2_MIDV
        as int16_t;
    (*wiener_info).vfilter[4 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[4 as core::ffi::c_int as usize];
    (*wiener_info).hfilter[5 as core::ffi::c_int as usize] = WIENER_FILT_TAP1_MIDV
        as int16_t;
    (*wiener_info).vfilter[5 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[5 as core::ffi::c_int as usize];
    (*wiener_info).hfilter[6 as core::ffi::c_int as usize] = WIENER_FILT_TAP0_MIDV
        as int16_t;
    (*wiener_info).vfilter[6 as core::ffi::c_int as usize] = (*wiener_info)
        .hfilter[6 as core::ffi::c_int as usize];
}
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_LF_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_plane_block_size(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    if (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {} else {
        __assert_fail(
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.h\0" as *const u8
                as *const core::ffi::c_char,
            1194 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 54],
                [core::ffi::c_char; 54],
            >(*b"BLOCK_SIZE get_plane_block_size(BLOCK_SIZE, int, int)\0"))
                .as_ptr(),
        );
    };
    if subsampling_x >= 0 as core::ffi::c_int && subsampling_x < 2 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"subsampling_x >= 0 && subsampling_x < 2\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.h\0" as *const u8
                as *const core::ffi::c_char,
            1195 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 54],
                [core::ffi::c_char; 54],
            >(*b"BLOCK_SIZE get_plane_block_size(BLOCK_SIZE, int, int)\0"))
                .as_ptr(),
        );
    };
    if subsampling_y >= 0 as core::ffi::c_int && subsampling_y < 2 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"subsampling_y >= 0 && subsampling_y < 2\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/blockd.h\0" as *const u8
                as *const core::ffi::c_char,
            1196 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 54],
                [core::ffi::c_char; 54],
            >(*b"BLOCK_SIZE get_plane_block_size(BLOCK_SIZE, int, int)\0"))
                .as_ptr(),
        );
    };
    return av1_ss_size_lookup[bsize
        as usize][subsampling_x as usize][subsampling_y as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_plane_type(mut plane: core::ffi::c_int) -> PLANE_TYPE {
    return (if plane == 0 as core::ffi::c_int {
        PLANE_TYPE_Y as core::ffi::c_int
    } else {
        PLANE_TYPE_UV as core::ffi::c_int
    }) as PLANE_TYPE;
}

extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
