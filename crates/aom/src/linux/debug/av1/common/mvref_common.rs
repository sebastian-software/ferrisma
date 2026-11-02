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

// Mechanically translated from upstream/aom/av1/common/mvref_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type qm_val_t = uint8_t;
pub type TX_SIZE = uint8_t;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const PARTITION_INVALID: C2RustUnnamed = 255;
pub const PARTITION_TYPES: C2RustUnnamed = 4;
pub const EXT_PARTITION_TYPES: C2RustUnnamed = 10;
pub const PARTITION_VERT_4: C2RustUnnamed = 9;
pub const PARTITION_HORZ_4: C2RustUnnamed = 8;
pub const PARTITION_VERT_B: C2RustUnnamed = 7;
pub const PARTITION_VERT_A: C2RustUnnamed = 6;
pub const PARTITION_HORZ_B: C2RustUnnamed = 5;
pub const PARTITION_HORZ_A: C2RustUnnamed = 4;
pub const PARTITION_SPLIT: C2RustUnnamed = 3;
pub const PARTITION_VERT: C2RustUnnamed = 2;
pub const PARTITION_HORZ: C2RustUnnamed = 1;
pub const PARTITION_NONE: C2RustUnnamed = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
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
pub type AV1_LEVEL = uint8_t;
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
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_14 = 4;
pub const AFFINE: C2RustUnnamed_14 = 3;
pub const ROTZOOM: C2RustUnnamed_14 = 2;
pub const TRANSLATION: C2RustUnnamed_14 = 1;
pub const IDENTITY: C2RustUnnamed_14 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpelMvLimits {
    pub col_min: core::ffi::c_int,
    pub col_max: core::ffi::c_int,
    pub row_min: core::ffi::c_int,
    pub row_max: core::ffi::c_int,
}
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_15 = 4;
pub const S_FRAME: C2RustUnnamed_15 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_15 = 2;
pub const INTER_FRAME: C2RustUnnamed_15 = 1;
pub const KEY_FRAME: C2RustUnnamed_15 = 0;
pub type MACROBLOCKD = macroblockd;
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const REFERENCE_MODES: C2RustUnnamed_16 = 3;
pub const REFERENCE_MODE_SELECT: C2RustUnnamed_16 = 2;
pub const COMPOUND_REFERENCE: C2RustUnnamed_16 = 1;
pub const SINGLE_REFERENCE: C2RustUnnamed_16 = 0;
pub type AV1_COMMON = AV1Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position {
    pub row: core::ffi::c_int,
    pub col: core::ffi::c_int,
}
pub type POSITION = position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REF_FRAME_INFO {
    pub map_idx: core::ffi::c_int,
    pub buf: *mut RefCntBuffer,
    pub sort_idx: core::ffi::c_int,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MVREF_ROW_COLS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMVS_LIMIT: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 12 as core::ffi::c_int) - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const FRAME_OFFSET_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_FRAME_DISTANCE: core::ffi::c_int = ((1 as core::ffi::c_int)
    << FRAME_OFFSET_BITS) - 1 as core::ffi::c_int;
pub const GLOBALMV_OFFSET: core::ffi::c_int = 3 as core::ffi::c_int;
pub const REFMV_OFFSET: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_MV_REF_CANDIDATES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_REF_MV_STACK_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const REF_CAT_LEVEL: core::ffi::c_int = 640 as core::ffi::c_int;
pub const MFMV_STACK_SIZE: core::ffi::c_int = 3 as core::ffi::c_int;
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
pub const MV_IN_USE_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const MV_UPP: core::ffi::c_int = (1 as core::ffi::c_int) << MV_IN_USE_BITS;
pub const MV_LOW: core::ffi::c_int = -((1 as core::ffi::c_int) << MV_IN_USE_BITS);
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
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
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
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
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
        if 1 as core::ffi::c_int
            & (res.as_mv.row as core::ffi::c_int | res.as_mv.col as core::ffi::c_int)
            == 0 || allow_hp != 0
        {} else {
            __assert_fail(
                b"IMPLIES(1 & (res.as_mv.row | res.as_mv.col), allow_hp)\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mv.h\0" as *const u8
                    as *const core::ffi::c_char,
                268 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 88],
                    [core::ffi::c_char; 88],
                >(
                    *b"int_mv gm_get_motion_vector(const WarpedMotionParams *, int, BLOCK_SIZE, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        if is_integer != 0 {
            integer_mv_precision(&raw mut res.as_mv);
        }
        return res;
    }
    x = block_center_x(mi_col, bsize);
    y = block_center_y(mi_row, bsize);
    if (*gm).wmtype as core::ffi::c_int == ROTZOOM as core::ffi::c_int {
        if (*gm).wmmat[5 as core::ffi::c_int as usize]
            == (*gm).wmmat[2 as core::ffi::c_int as usize]
        {} else {
            __assert_fail(
                b"gm->wmmat[5] == gm->wmmat[2]\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mv.h\0" as *const u8
                    as *const core::ffi::c_char,
                279 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 88],
                    [core::ffi::c_char; 88],
                >(
                    *b"int_mv gm_get_motion_vector(const WarpedMotionParams *, int, BLOCK_SIZE, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        if (*gm).wmmat[4 as core::ffi::c_int as usize]
            == -(*gm).wmmat[3 as core::ffi::c_int as usize]
        {} else {
            __assert_fail(
                b"gm->wmmat[4] == -gm->wmmat[3]\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mv.h\0" as *const u8
                    as *const core::ffi::c_char,
                280 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 88],
                    [core::ffi::c_char; 88],
                >(
                    *b"int_mv gm_get_motion_vector(const WarpedMotionParams *, int, BLOCK_SIZE, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
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
pub const MV_BORDER: core::ffi::c_int = (16 as core::ffi::c_int)
    << 3 as core::ffi::c_int;
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
    if bits >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"bits >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.h\0" as *const u8
                as *const core::ffi::c_char,
            42 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 55],
                [core::ffi::c_char; 55],
            >(*b"int get_relative_dist(const OrderHintInfo *, int, int)\0"))
                .as_ptr(),
        );
    };
    if a >= 0 as core::ffi::c_int && a < (1 as core::ffi::c_int) << bits {} else {
        __assert_fail(
            b"a >= 0 && a < (1 << bits)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.h\0" as *const u8
                as *const core::ffi::c_char,
            43 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 55],
                [core::ffi::c_char; 55],
            >(*b"int get_relative_dist(const OrderHintInfo *, int, int)\0"))
                .as_ptr(),
        );
    };
    if b >= 0 as core::ffi::c_int && b < (1 as core::ffi::c_int) << bits {} else {
        __assert_fail(
            b"b >= 0 && b < (1 << bits)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.h\0" as *const u8
                as *const core::ffi::c_char,
            44 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 55],
                [core::ffi::c_char; 55],
            >(*b"int get_relative_dist(const OrderHintInfo *, int, int)\0"))
                .as_ptr(),
        );
    };
    let mut diff: core::ffi::c_int = a - b;
    let m: core::ffi::c_int = (1 as core::ffi::c_int) << bits - 1 as core::ffi::c_int;
    diff = (diff & m - 1 as core::ffi::c_int) - (diff & m);
    return diff;
}
#[inline]
unsafe extern "C-unwind" fn clamp_mv_ref(
    mut mv: *mut MV,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut xd: *const MACROBLOCKD,
) {
    let mv_limits: SubpelMvLimits = {
        let mut init = SubpelMvLimits {
            col_min: (*xd).mb_to_left_edge - bw * 8 as core::ffi::c_int - MV_BORDER,
            col_max: (*xd).mb_to_right_edge + bw * 8 as core::ffi::c_int + MV_BORDER,
            row_min: (*xd).mb_to_top_edge - bh * 8 as core::ffi::c_int - MV_BORDER,
            row_max: (*xd).mb_to_bottom_edge + bh * 8 as core::ffi::c_int + MV_BORDER,
        };
        init
    };
    clamp_mv(mv, &mv_limits);
}
#[inline]
unsafe extern "C-unwind" fn get_block_mv(
    mut candidate: *const MB_MODE_INFO,
    mut which_mv: core::ffi::c_int,
) -> int_mv {
    return (*candidate).mv[which_mv as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_inside(
    tile: *const TileInfo,
    mut mi_col: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_pos: *const POSITION,
) -> core::ffi::c_int {
    return !(mi_row + (*mi_pos).row < (*tile).mi_row_start
        || mi_col + (*mi_pos).col < (*tile).mi_col_start
        || mi_row + (*mi_pos).row >= (*tile).mi_row_end
        || mi_col + (*mi_pos).col >= (*tile).mi_col_end) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn find_valid_row_offset(
    tile: *const TileInfo,
    mut mi_row: core::ffi::c_int,
    mut row_offset: core::ffi::c_int,
) -> core::ffi::c_int {
    return clamp(
        row_offset,
        (*tile).mi_row_start - mi_row,
        (*tile).mi_row_end - mi_row - 1 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn find_valid_col_offset(
    tile: *const TileInfo,
    mut mi_col: core::ffi::c_int,
    mut col_offset: core::ffi::c_int,
) -> core::ffi::c_int {
    return clamp(
        col_offset,
        (*tile).mi_col_start - mi_col,
        (*tile).mi_col_end - mi_col - 1 as core::ffi::c_int,
    );
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
        if ref_frame_type as core::ffi::c_int > NONE_FRAME as core::ffi::c_int {} else {
            __assert_fail(
                b"ref_frame_type > NONE_FRAME\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mvref_common.h\0" as *const u8
                    as *const core::ffi::c_char,
                158 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 65],
                    [core::ffi::c_char; 65],
                >(
                    *b"void av1_set_ref_frame(MV_REFERENCE_FRAME *, MV_REFERENCE_FRAME)\0",
                ))
                    .as_ptr(),
            );
        };
        *rf.offset(0 as core::ffi::c_int as isize) = ref_frame_type;
        *rf.offset(1 as core::ffi::c_int as isize) = NONE_FRAME as core::ffi::c_int
            as MV_REFERENCE_FRAME;
    };
}
static mut div_mult: [core::ffi::c_int; 32] = [
    0 as core::ffi::c_int,
    16384 as core::ffi::c_int,
    8192 as core::ffi::c_int,
    5461 as core::ffi::c_int,
    4096 as core::ffi::c_int,
    3276 as core::ffi::c_int,
    2730 as core::ffi::c_int,
    2340 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    1820 as core::ffi::c_int,
    1638 as core::ffi::c_int,
    1489 as core::ffi::c_int,
    1365 as core::ffi::c_int,
    1260 as core::ffi::c_int,
    1170 as core::ffi::c_int,
    1092 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    963 as core::ffi::c_int,
    910 as core::ffi::c_int,
    862 as core::ffi::c_int,
    819 as core::ffi::c_int,
    780 as core::ffi::c_int,
    744 as core::ffi::c_int,
    712 as core::ffi::c_int,
    682 as core::ffi::c_int,
    655 as core::ffi::c_int,
    630 as core::ffi::c_int,
    606 as core::ffi::c_int,
    585 as core::ffi::c_int,
    564 as core::ffi::c_int,
    546 as core::ffi::c_int,
    528 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_mv_projection(
    mut output: *mut MV,
    mut ref_0: MV,
    mut num: core::ffi::c_int,
    mut den: core::ffi::c_int,
) {
    den = if den
        < ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int
    {
        den
    } else {
        ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int
    };
    num = if num > 0 as core::ffi::c_int {
        if num
            < ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int
        {
            num
        } else {
            ((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int
        }
    } else if num
        > -(((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int)
    {
        num
    } else {
        -(((1 as core::ffi::c_int) << 5 as core::ffi::c_int) - 1 as core::ffi::c_int)
    };
    let mv_row: core::ffi::c_int = if ref_0.row as core::ffi::c_int * num
        * div_mult[den as usize] < 0 as core::ffi::c_int
    {
        -(-(ref_0.row as core::ffi::c_int * num * div_mult[den as usize])
            + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 14 as core::ffi::c_int)
    } else {
        ref_0.row as core::ffi::c_int * num * div_mult[den as usize]
            + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 14 as core::ffi::c_int
    };
    let mv_col: core::ffi::c_int = if ref_0.col as core::ffi::c_int * num
        * div_mult[den as usize] < 0 as core::ffi::c_int
    {
        -(-(ref_0.col as core::ffi::c_int * num * div_mult[den as usize])
            + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 14 as core::ffi::c_int)
    } else {
        ref_0.col as core::ffi::c_int * num * div_mult[den as usize]
            + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int
                >> 1 as core::ffi::c_int) >> 14 as core::ffi::c_int
    };
    let clamp_max: core::ffi::c_int = MV_UPP - 1 as core::ffi::c_int;
    let clamp_min: core::ffi::c_int = MV_LOW + 1 as core::ffi::c_int;
    (*output).row = clamp(mv_row, clamp_min, clamp_max) as int16_t;
    (*output).col = clamp(mv_col, clamp_min, clamp_max) as int16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_copy_frame_mvs(
    cm: *const AV1_COMMON,
    mi: *const MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut x_mis: core::ffi::c_int,
    mut y_mis: core::ffi::c_int,
) {
    let frame_mvs_stride: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    let mut frame_mvs: *mut MV_REF = ((*(*cm).cur_frame).mvs)
        .offset(((mi_row >> 1 as core::ffi::c_int) * frame_mvs_stride) as isize)
        .offset((mi_col >> 1 as core::ffi::c_int) as isize);
    x_mis = x_mis
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    y_mis = y_mis
        + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    let mut w: core::ffi::c_int = 0;
    let mut h: core::ffi::c_int = 0;
    h = 0 as core::ffi::c_int;
    while h < y_mis {
        let mut mv: *mut MV_REF = frame_mvs;
        w = 0 as core::ffi::c_int;
        while w < x_mis {
            (*mv).ref_frame = NONE_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
            (*mv).mv.as_int = 0 as uint32_t;
            let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx < 2 as core::ffi::c_int {
                let mut ref_frame: MV_REFERENCE_FRAME = (*mi).ref_frame[idx as usize];
                if ref_frame as core::ffi::c_int > INTRA_FRAME as core::ffi::c_int {
                    let mut ref_idx: int8_t = (*cm).ref_frame_side[ref_frame as usize];
                    if !(ref_idx != 0) {
                        if !(abs((*mi).mv[idx as usize].as_mv.row as core::ffi::c_int)
                            > REFMVS_LIMIT
                            || abs((*mi).mv[idx as usize].as_mv.col as core::ffi::c_int)
                                > REFMVS_LIMIT)
                        {
                            (*mv).ref_frame = ref_frame;
                            (*mv).mv.as_int = (*mi).mv[idx as usize].as_int;
                        }
                    }
                }
                idx += 1;
            }
            mv = mv.offset(1);
            w += 1;
        }
        frame_mvs = frame_mvs.offset(frame_mvs_stride as isize);
        h += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn add_ref_mv_candidate(
    candidate: *const MB_MODE_INFO,
    mut rf: *const MV_REFERENCE_FRAME,
    mut refmv_count: *mut uint8_t,
    mut ref_match_count: *mut uint8_t,
    mut newmv_count: *mut uint8_t,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut gm_mv_candidates: *mut int_mv,
    mut gm_params: *const WarpedMotionParams,
    mut weight: uint16_t,
) {
    if is_inter_block(candidate) == 0 {
        return;
    }
    if weight as core::ffi::c_int % 2 as core::ffi::c_int == 0 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"weight % 2 == 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            82 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 190],
                [core::ffi::c_char; 190],
            >(
                *b"void add_ref_mv_candidate(const MB_MODE_INFO *const, const MV_REFERENCE_FRAME *, uint8_t *, uint8_t *, uint8_t *, CANDIDATE_MV *, uint16_t *, int_mv *, const WarpedMotionParams *, uint16_t)\0",
            ))
                .as_ptr(),
        );
    };
    let mut index: core::ffi::c_int = 0;
    let mut ref_0: core::ffi::c_int = 0;
    if *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        == NONE_FRAME as core::ffi::c_int
    {
        ref_0 = 0 as core::ffi::c_int;
        while ref_0 < 2 as core::ffi::c_int {
            if (*candidate).ref_frame[ref_0 as usize] as core::ffi::c_int
                == *rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            {
                let is_gm_block: core::ffi::c_int = is_global_mv_block(
                    candidate,
                    (*gm_params
                        .offset(*rf.offset(0 as core::ffi::c_int as isize) as isize))
                        .wmtype,
                ) as core::ffi::c_int;
                let this_refmv: int_mv = if is_gm_block != 0 {
                    *gm_mv_candidates.offset(0 as core::ffi::c_int as isize)
                } else {
                    get_block_mv(candidate, ref_0) as int_mv
                };
                index = 0 as core::ffi::c_int;
                while index < *refmv_count as core::ffi::c_int {
                    if (*ref_mv_stack.offset(index as isize)).this_mv.as_int
                        == this_refmv.as_int
                    {
                        let fresh13 = &raw mut *ref_mv_weight.offset(index as isize);
                        *fresh13 = (*fresh13 as core::ffi::c_int
                            + weight as core::ffi::c_int) as uint16_t;
                        break;
                    } else {
                        index += 1;
                    }
                }
                if index == *refmv_count as core::ffi::c_int
                    && (*refmv_count as core::ffi::c_int) < MAX_REF_MV_STACK_SIZE
                {
                    (*ref_mv_stack.offset(index as isize)).this_mv = this_refmv;
                    *ref_mv_weight.offset(index as isize) = weight;
                    *refmv_count = (*refmv_count).wrapping_add(1);
                }
                if have_newmv_in_inter_mode((*candidate).mode) != 0 {
                    *newmv_count = (*newmv_count).wrapping_add(1);
                }
                *ref_match_count = (*ref_match_count).wrapping_add(1);
            }
            ref_0 += 1;
        }
    } else if (*candidate).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        == *rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        && (*candidate).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
    {
        let mut this_refmv_0: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
        ref_0 = 0 as core::ffi::c_int;
        while ref_0 < 2 as core::ffi::c_int {
            if is_global_mv_block(
                candidate,
                (*gm_params.offset(*rf.offset(ref_0 as isize) as isize)).wmtype,
            ) != 0
            {
                this_refmv_0[ref_0 as usize] = *gm_mv_candidates.offset(ref_0 as isize);
            } else {
                this_refmv_0[ref_0 as usize] = get_block_mv(candidate, ref_0);
            }
            ref_0 += 1;
        }
        index = 0 as core::ffi::c_int;
        while index < *refmv_count as core::ffi::c_int {
            if (*ref_mv_stack.offset(index as isize)).this_mv.as_int
                == this_refmv_0[0 as core::ffi::c_int as usize].as_int
                && (*ref_mv_stack.offset(index as isize)).comp_mv.as_int
                    == this_refmv_0[1 as core::ffi::c_int as usize].as_int
            {
                let fresh14 = &raw mut *ref_mv_weight.offset(index as isize);
                *fresh14 = (*fresh14 as core::ffi::c_int + weight as core::ffi::c_int)
                    as uint16_t;
                break;
            } else {
                index += 1;
            }
        }
        if index == *refmv_count as core::ffi::c_int
            && (*refmv_count as core::ffi::c_int) < MAX_REF_MV_STACK_SIZE
        {
            (*ref_mv_stack.offset(index as isize)).this_mv = this_refmv_0[0
                as core::ffi::c_int as usize];
            (*ref_mv_stack.offset(index as isize)).comp_mv = this_refmv_0[1
                as core::ffi::c_int as usize];
            *ref_mv_weight.offset(index as isize) = weight;
            *refmv_count = (*refmv_count).wrapping_add(1);
        }
        if have_newmv_in_inter_mode((*candidate).mode) != 0 {
            *newmv_count = (*newmv_count).wrapping_add(1);
        }
        *ref_match_count = (*ref_match_count).wrapping_add(1);
    }
}
#[inline]
unsafe extern "C-unwind" fn scan_row_mbmi(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_col: core::ffi::c_int,
    mut rf: *const MV_REFERENCE_FRAME,
    mut row_offset: core::ffi::c_int,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut refmv_count: *mut uint8_t,
    mut ref_match_count: *mut uint8_t,
    mut newmv_count: *mut uint8_t,
    mut gm_mv_candidates: *mut int_mv,
    mut max_row_offset: core::ffi::c_int,
    mut processed_rows: *mut core::ffi::c_int,
) {
    let mut end_mi: core::ffi::c_int = if ((*xd).width as core::ffi::c_int)
        < (*cm).mi_params.mi_cols - mi_col
    {
        (*xd).width as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_cols - mi_col
    };
    end_mi = if end_mi
        < mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    {
        end_mi
    } else {
        mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    };
    let width_8x8: core::ffi::c_int = mi_size_wide[BLOCK_8X8 as core::ffi::c_int
        as usize] as core::ffi::c_int;
    let width_16x16: core::ffi::c_int = mi_size_wide[BLOCK_16X16 as core::ffi::c_int
        as usize] as core::ffi::c_int;
    let mut col_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    if abs(row_offset) > 1 as core::ffi::c_int {
        col_offset = 1 as core::ffi::c_int;
        if mi_col & 0x1 as core::ffi::c_int != 0
            && ((*xd).width as core::ffi::c_int) < width_8x8
        {
            col_offset -= 1;
        }
    }
    let use_step_16: core::ffi::c_int = ((*xd).width as core::ffi::c_int
        >= 16 as core::ffi::c_int) as core::ffi::c_int;
    let candidate_mi0: *mut *mut MB_MODE_INFO = ((*xd).mi)
        .offset((row_offset * (*xd).mi_stride) as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < end_mi {
        let candidate: *const MB_MODE_INFO = *candidate_mi0
            .offset((col_offset + i) as isize);
        let candidate_bsize: core::ffi::c_int = (*candidate).bsize as core::ffi::c_int;
        let n4_w: core::ffi::c_int = mi_size_wide[candidate_bsize as usize]
            as core::ffi::c_int;
        let mut len: core::ffi::c_int = if ((*xd).width as core::ffi::c_int) < n4_w {
            (*xd).width as core::ffi::c_int
        } else {
            n4_w
        };
        if use_step_16 != 0 {
            len = if width_16x16 > len { width_16x16 } else { len };
        } else if abs(row_offset) > 1 as core::ffi::c_int {
            len = if len > width_8x8 { len } else { width_8x8 };
        }
        let mut weight: uint16_t = 2 as uint16_t;
        if (*xd).width as core::ffi::c_int >= width_8x8
            && (*xd).width as core::ffi::c_int <= n4_w
        {
            let mut inc: uint16_t = (if (-max_row_offset + row_offset
                + 1 as core::ffi::c_int)
                < mi_size_high[candidate_bsize as usize] as core::ffi::c_int
            {
                -max_row_offset + row_offset + 1 as core::ffi::c_int
            } else {
                mi_size_high[candidate_bsize as usize] as core::ffi::c_int
            }) as uint16_t;
            weight = (if weight as core::ffi::c_int > inc as core::ffi::c_int {
                weight as core::ffi::c_int
            } else {
                inc as core::ffi::c_int
            }) as uint16_t;
            *processed_rows = inc as core::ffi::c_int - row_offset
                - 1 as core::ffi::c_int;
        }
        add_ref_mv_candidate(
            candidate,
            rf,
            refmv_count,
            ref_match_count,
            newmv_count,
            ref_mv_stack,
            ref_mv_weight,
            gm_mv_candidates,
            ((*cm).global_motion).as_ptr(),
            (len * weight as core::ffi::c_int) as uint16_t,
        );
        i += len;
    }
}
#[inline]
unsafe extern "C-unwind" fn scan_col_mbmi(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut rf: *const MV_REFERENCE_FRAME,
    mut col_offset: core::ffi::c_int,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut refmv_count: *mut uint8_t,
    mut ref_match_count: *mut uint8_t,
    mut newmv_count: *mut uint8_t,
    mut gm_mv_candidates: *mut int_mv,
    mut max_col_offset: core::ffi::c_int,
    mut processed_cols: *mut core::ffi::c_int,
) {
    let mut end_mi: core::ffi::c_int = if ((*xd).height as core::ffi::c_int)
        < (*cm).mi_params.mi_rows - mi_row
    {
        (*xd).height as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_rows - mi_row
    };
    end_mi = if end_mi
        < mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    {
        end_mi
    } else {
        mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    };
    let n8_h_8: core::ffi::c_int = mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let n8_h_16: core::ffi::c_int = mi_size_high[BLOCK_16X16 as core::ffi::c_int
        as usize] as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut row_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    if abs(col_offset) > 1 as core::ffi::c_int {
        row_offset = 1 as core::ffi::c_int;
        if mi_row & 0x1 as core::ffi::c_int != 0
            && ((*xd).height as core::ffi::c_int) < n8_h_8
        {
            row_offset -= 1;
        }
    }
    let use_step_16: core::ffi::c_int = ((*xd).height as core::ffi::c_int
        >= 16 as core::ffi::c_int) as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < end_mi {
        let candidate: *const MB_MODE_INFO = *((*xd).mi)
            .offset(((row_offset + i) * (*xd).mi_stride + col_offset) as isize);
        let candidate_bsize: core::ffi::c_int = (*candidate).bsize as core::ffi::c_int;
        let n4_h: core::ffi::c_int = mi_size_high[candidate_bsize as usize]
            as core::ffi::c_int;
        let mut len: core::ffi::c_int = if ((*xd).height as core::ffi::c_int) < n4_h {
            (*xd).height as core::ffi::c_int
        } else {
            n4_h
        };
        if use_step_16 != 0 {
            len = if n8_h_16 > len { n8_h_16 } else { len };
        } else if abs(col_offset) > 1 as core::ffi::c_int {
            len = if len > n8_h_8 { len } else { n8_h_8 };
        }
        let mut weight: core::ffi::c_int = 2 as core::ffi::c_int;
        if (*xd).height as core::ffi::c_int >= n8_h_8
            && (*xd).height as core::ffi::c_int <= n4_h
        {
            let mut inc: core::ffi::c_int = if (-max_col_offset + col_offset
                + 1 as core::ffi::c_int)
                < mi_size_wide[candidate_bsize as usize] as core::ffi::c_int
            {
                -max_col_offset + col_offset + 1 as core::ffi::c_int
            } else {
                mi_size_wide[candidate_bsize as usize] as core::ffi::c_int
            };
            weight = if weight > inc { weight } else { inc };
            *processed_cols = inc - col_offset - 1 as core::ffi::c_int;
        }
        add_ref_mv_candidate(
            candidate,
            rf,
            refmv_count,
            ref_match_count,
            newmv_count,
            ref_mv_stack,
            ref_mv_weight,
            gm_mv_candidates,
            ((*cm).global_motion).as_ptr(),
            (len * weight) as uint16_t,
        );
        i += len;
    }
}
#[inline]
unsafe extern "C-unwind" fn scan_blk_mbmi(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    mut rf: *const MV_REFERENCE_FRAME,
    mut row_offset: core::ffi::c_int,
    mut col_offset: core::ffi::c_int,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut ref_match_count: *mut uint8_t,
    mut newmv_count: *mut uint8_t,
    mut gm_mv_candidates: *mut int_mv,
    mut refmv_count: *mut uint8_t,
) {
    let tile: *const TileInfo = &(*xd).tile;
    let mut mi_pos: POSITION = position { row: 0, col: 0 };
    mi_pos.row = row_offset;
    mi_pos.col = col_offset;
    if is_inside(tile, mi_col, mi_row, &raw mut mi_pos) != 0 {
        let candidate: *const MB_MODE_INFO = *((*xd).mi)
            .offset((mi_pos.row * (*xd).mi_stride + mi_pos.col) as isize);
        let len: core::ffi::c_int = mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize]
            as core::ffi::c_int;
        add_ref_mv_candidate(
            candidate,
            rf,
            refmv_count,
            ref_match_count,
            newmv_count,
            ref_mv_stack,
            ref_mv_weight,
            gm_mv_candidates,
            ((*cm).global_motion).as_ptr(),
            (2 as core::ffi::c_int * len) as uint16_t,
        );
    }
}
unsafe extern "C-unwind" fn has_top_right(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bs: core::ffi::c_int,
) -> core::ffi::c_int {
    let sb_mi_size: core::ffi::c_int = mi_size_wide[(*(*cm).seq_params).sb_size as usize]
        as core::ffi::c_int;
    let mask_row: core::ffi::c_int = mi_row & sb_mi_size - 1 as core::ffi::c_int;
    let mask_col: core::ffi::c_int = mi_col & sb_mi_size - 1 as core::ffi::c_int;
    if bs > mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut has_tr: core::ffi::c_int = !(mask_row & bs != 0 && mask_col & bs != 0)
        as core::ffi::c_int;
    if bs > 0 as core::ffi::c_int && bs & bs - 1 as core::ffi::c_int == 0 {} else {
        __assert_fail(
            b"bs > 0 && !(bs & (bs - 1))\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            276 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 74],
                [core::ffi::c_char; 74],
            >(
                *b"int has_top_right(const AV1_COMMON *, const MACROBLOCKD *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    while bs < sb_mi_size {
        if !(mask_col & bs != 0) {
            break;
        }
        if mask_col & 2 as core::ffi::c_int * bs != 0
            && mask_row & 2 as core::ffi::c_int * bs != 0
        {
            has_tr = 0 as core::ffi::c_int;
            break;
        } else {
            bs <<= 1 as core::ffi::c_int;
        }
    }
    if ((*xd).width as core::ffi::c_int) < (*xd).height as core::ffi::c_int {
        if !(*xd).is_last_vertical_rect {
            has_tr = 1 as core::ffi::c_int;
        }
    }
    if (*xd).width as core::ffi::c_int > (*xd).height as core::ffi::c_int {
        if !(*xd).is_first_horizontal_rect {
            has_tr = 0 as core::ffi::c_int;
        }
    }
    if (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).partition
        as core::ffi::c_int == PARTITION_VERT_A as core::ffi::c_int
    {
        if (*xd).width as core::ffi::c_int == (*xd).height as core::ffi::c_int {
            if mask_row & bs != 0 {
                has_tr = 0 as core::ffi::c_int;
            }
        }
    }
    return has_tr;
}
unsafe extern "C-unwind" fn check_sb_border(
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    row_offset: core::ffi::c_int,
    col_offset: core::ffi::c_int,
) -> core::ffi::c_int {
    let sb_mi_size: core::ffi::c_int = mi_size_wide[BLOCK_64X64 as core::ffi::c_int
        as usize] as core::ffi::c_int;
    let row: core::ffi::c_int = mi_row & sb_mi_size - 1 as core::ffi::c_int;
    let col: core::ffi::c_int = mi_col & sb_mi_size - 1 as core::ffi::c_int;
    if row + row_offset < 0 as core::ffi::c_int || row + row_offset >= sb_mi_size
        || col + col_offset < 0 as core::ffi::c_int || col + col_offset >= sb_mi_size
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn add_tpl_ref_mv(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut gm_mv_candidates: *mut int_mv,
    refmv_count: *mut uint8_t,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut mode_context: *mut int16_t,
) -> core::ffi::c_int {
    let mut mi_pos: POSITION = position { row: 0, col: 0 };
    mi_pos.row = if mi_row & 0x1 as core::ffi::c_int != 0 {
        blk_row
    } else {
        blk_row + 1 as core::ffi::c_int
    };
    mi_pos.col = if mi_col & 0x1 as core::ffi::c_int != 0 {
        blk_col
    } else {
        blk_col + 1 as core::ffi::c_int
    };
    if is_inside(&(*xd).tile, mi_col, mi_row, &raw mut mi_pos) == 0 {
        return 0 as core::ffi::c_int;
    }
    let mut prev_frame_mvs: *const TPL_MV_REF = ((*cm).tpl_mvs)
        .offset(
            ((mi_row + mi_pos.row >> 1 as core::ffi::c_int)
                * ((*cm).mi_params.mi_stride >> 1 as core::ffi::c_int)) as isize,
        )
        .offset((mi_col + mi_pos.col >> 1 as core::ffi::c_int) as isize);
    if (*prev_frame_mvs).mfmv0.as_int == INVALID_MV as uint32_t {
        return 0 as core::ffi::c_int;
    }
    let mut rf: [MV_REFERENCE_FRAME; 2] = [0; 2];
    av1_set_ref_frame(rf.as_mut_ptr(), ref_frame);
    let weight_unit: uint16_t = 1 as uint16_t;
    let cur_frame_index: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    let buf_0: *const RefCntBuffer = get_ref_frame_buf(
        cm,
        rf[0 as core::ffi::c_int as usize],
    );
    let frame0_index: core::ffi::c_int = (*buf_0).order_hint as core::ffi::c_int;
    let cur_offset_0: core::ffi::c_int = get_relative_dist(
        &raw mut (*(*cm).seq_params).order_hint_info,
        cur_frame_index,
        frame0_index,
    ) as core::ffi::c_int;
    let mut idx: core::ffi::c_int = 0;
    let allow_high_precision_mv: core::ffi::c_int = (*cm)
        .features
        .allow_high_precision_mv as core::ffi::c_int;
    let force_integer_mv: core::ffi::c_int = (*cm).features.cur_frame_force_integer_mv
        as core::ffi::c_int;
    let mut this_refmv: int_mv = int_mv { as_int: 0 };
    av1_get_mv_projection(
        &raw mut this_refmv.as_mv,
        (*prev_frame_mvs).mfmv0.as_mv,
        cur_offset_0,
        (*prev_frame_mvs).ref_frame_offset as core::ffi::c_int,
    );
    lower_mv_precision(&raw mut this_refmv.as_mv, allow_high_precision_mv, force_integer_mv);
    if rf[1 as core::ffi::c_int as usize] as core::ffi::c_int
        == NONE_FRAME as core::ffi::c_int
    {
        if blk_row == 0 as core::ffi::c_int && blk_col == 0 as core::ffi::c_int {
            if abs(
                this_refmv.as_mv.row as core::ffi::c_int
                    - (*gm_mv_candidates.offset(0 as core::ffi::c_int as isize))
                        .as_mv
                        .row as core::ffi::c_int,
            ) >= 16 as core::ffi::c_int
                || abs(
                    this_refmv.as_mv.col as core::ffi::c_int
                        - (*gm_mv_candidates.offset(0 as core::ffi::c_int as isize))
                            .as_mv
                            .col as core::ffi::c_int,
                ) >= 16 as core::ffi::c_int
            {
                let fresh15 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh15 = (*fresh15 as core::ffi::c_int
                    | (1 as core::ffi::c_int) << GLOBALMV_OFFSET) as int16_t;
            }
        }
        idx = 0 as core::ffi::c_int;
        while idx < *refmv_count as core::ffi::c_int {
            if this_refmv.as_int == (*ref_mv_stack.offset(idx as isize)).this_mv.as_int {
                break;
            }
            idx += 1;
        }
        if idx < *refmv_count as core::ffi::c_int {
            let fresh16 = &raw mut *ref_mv_weight.offset(idx as isize);
            *fresh16 = (*fresh16 as core::ffi::c_int
                + 2 as core::ffi::c_int * weight_unit as core::ffi::c_int) as uint16_t;
        }
        if idx == *refmv_count as core::ffi::c_int
            && (*refmv_count as core::ffi::c_int) < MAX_REF_MV_STACK_SIZE
        {
            (*ref_mv_stack.offset(idx as isize)).this_mv.as_int = this_refmv.as_int;
            *ref_mv_weight.offset(idx as isize) = (2 as core::ffi::c_int
                * weight_unit as core::ffi::c_int) as uint16_t;
            *refmv_count = (*refmv_count).wrapping_add(1);
        }
    } else {
        let buf_1: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            rf[1 as core::ffi::c_int as usize],
        );
        let frame1_index: core::ffi::c_int = (*buf_1).order_hint as core::ffi::c_int;
        let cur_offset_1: core::ffi::c_int = get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            cur_frame_index,
            frame1_index,
        ) as core::ffi::c_int;
        let mut comp_refmv: int_mv = int_mv { as_int: 0 };
        av1_get_mv_projection(
            &raw mut comp_refmv.as_mv,
            (*prev_frame_mvs).mfmv0.as_mv,
            cur_offset_1,
            (*prev_frame_mvs).ref_frame_offset as core::ffi::c_int,
        );
        lower_mv_precision(
            &raw mut comp_refmv.as_mv,
            allow_high_precision_mv,
            force_integer_mv,
        );
        if blk_row == 0 as core::ffi::c_int && blk_col == 0 as core::ffi::c_int {
            if abs(
                this_refmv.as_mv.row as core::ffi::c_int
                    - (*gm_mv_candidates.offset(0 as core::ffi::c_int as isize))
                        .as_mv
                        .row as core::ffi::c_int,
            ) >= 16 as core::ffi::c_int
                || abs(
                    this_refmv.as_mv.col as core::ffi::c_int
                        - (*gm_mv_candidates.offset(0 as core::ffi::c_int as isize))
                            .as_mv
                            .col as core::ffi::c_int,
                ) >= 16 as core::ffi::c_int
                || abs(
                    comp_refmv.as_mv.row as core::ffi::c_int
                        - (*gm_mv_candidates.offset(1 as core::ffi::c_int as isize))
                            .as_mv
                            .row as core::ffi::c_int,
                ) >= 16 as core::ffi::c_int
                || abs(
                    comp_refmv.as_mv.col as core::ffi::c_int
                        - (*gm_mv_candidates.offset(1 as core::ffi::c_int as isize))
                            .as_mv
                            .col as core::ffi::c_int,
                ) >= 16 as core::ffi::c_int
            {
                let fresh17 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh17 = (*fresh17 as core::ffi::c_int
                    | (1 as core::ffi::c_int) << GLOBALMV_OFFSET) as int16_t;
            }
        }
        idx = 0 as core::ffi::c_int;
        while idx < *refmv_count as core::ffi::c_int {
            if this_refmv.as_int == (*ref_mv_stack.offset(idx as isize)).this_mv.as_int
                && comp_refmv.as_int
                    == (*ref_mv_stack.offset(idx as isize)).comp_mv.as_int
            {
                break;
            }
            idx += 1;
        }
        if idx < *refmv_count as core::ffi::c_int {
            let fresh18 = &raw mut *ref_mv_weight.offset(idx as isize);
            *fresh18 = (*fresh18 as core::ffi::c_int
                + 2 as core::ffi::c_int * weight_unit as core::ffi::c_int) as uint16_t;
        }
        if idx == *refmv_count as core::ffi::c_int
            && (*refmv_count as core::ffi::c_int) < MAX_REF_MV_STACK_SIZE
        {
            (*ref_mv_stack.offset(idx as isize)).this_mv.as_int = this_refmv.as_int;
            (*ref_mv_stack.offset(idx as isize)).comp_mv.as_int = comp_refmv.as_int;
            *ref_mv_weight.offset(idx as isize) = (2 as core::ffi::c_int
                * weight_unit as core::ffi::c_int) as uint16_t;
            *refmv_count = (*refmv_count).wrapping_add(1);
        }
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn process_compound_ref_mv_candidate(
    candidate: *const MB_MODE_INFO,
    cm: *const AV1_COMMON,
    rf: *const MV_REFERENCE_FRAME,
    mut ref_id: *mut [int_mv; 2],
    mut ref_id_count: *mut core::ffi::c_int,
    mut ref_diff: *mut [int_mv; 2],
    mut ref_diff_count: *mut core::ffi::c_int,
) {
    let mut rf_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while rf_idx < 2 as core::ffi::c_int {
        let mut can_rf: MV_REFERENCE_FRAME = (*candidate).ref_frame[rf_idx as usize];
        let mut cmp_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while cmp_idx < 2 as core::ffi::c_int {
            if can_rf as core::ffi::c_int
                == *rf.offset(cmp_idx as isize) as core::ffi::c_int
                && *ref_id_count.offset(cmp_idx as isize) < 2 as core::ffi::c_int
            {
                (*ref_id
                    .offset(
                        cmp_idx as isize,
                    ))[*ref_id_count.offset(cmp_idx as isize) as usize] = (*candidate)
                    .mv[rf_idx as usize];
                let fresh11 = &raw mut *ref_id_count.offset(cmp_idx as isize);
                *fresh11 += 1;
            } else if can_rf as core::ffi::c_int > INTRA_FRAME as core::ffi::c_int
                && *ref_diff_count.offset(cmp_idx as isize) < 2 as core::ffi::c_int
            {
                let mut this_mv: int_mv = (*candidate).mv[rf_idx as usize];
                if (*cm).ref_frame_sign_bias[can_rf as usize]
                    != (*cm).ref_frame_sign_bias[*rf.offset(cmp_idx as isize) as usize]
                {
                    this_mv.as_mv.row = -(this_mv.as_mv.row as core::ffi::c_int)
                        as int16_t;
                    this_mv.as_mv.col = -(this_mv.as_mv.col as core::ffi::c_int)
                        as int16_t;
                }
                (*ref_diff
                    .offset(
                        cmp_idx as isize,
                    ))[*ref_diff_count.offset(cmp_idx as isize) as usize] = this_mv;
                let fresh12 = &raw mut *ref_diff_count.offset(cmp_idx as isize);
                *fresh12 += 1;
            }
            cmp_idx += 1;
        }
        rf_idx += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn process_single_ref_mv_candidate(
    candidate: *const MB_MODE_INFO,
    cm: *const AV1_COMMON,
    mut ref_frame: MV_REFERENCE_FRAME,
    refmv_count: *mut uint8_t,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
) {
    let mut rf_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while rf_idx < 2 as core::ffi::c_int {
        if (*candidate).ref_frame[rf_idx as usize] as core::ffi::c_int
            > INTRA_FRAME as core::ffi::c_int
        {
            let mut this_mv: int_mv = (*candidate).mv[rf_idx as usize];
            if (*cm)
                .ref_frame_sign_bias[(*candidate).ref_frame[rf_idx as usize] as usize]
                != (*cm).ref_frame_sign_bias[ref_frame as usize]
            {
                this_mv.as_mv.row = -(this_mv.as_mv.row as core::ffi::c_int) as int16_t;
                this_mv.as_mv.col = -(this_mv.as_mv.col as core::ffi::c_int) as int16_t;
            }
            let mut stack_idx: core::ffi::c_int = 0;
            stack_idx = 0 as core::ffi::c_int;
            while stack_idx < *refmv_count as core::ffi::c_int {
                let stack_mv: int_mv = (*ref_mv_stack.offset(stack_idx as isize))
                    .this_mv;
                if this_mv.as_int == stack_mv.as_int {
                    break;
                }
                stack_idx += 1;
            }
            if stack_idx == *refmv_count as core::ffi::c_int {
                (*ref_mv_stack.offset(stack_idx as isize)).this_mv = this_mv;
                *ref_mv_weight.offset(stack_idx as isize) = 2 as uint16_t;
                *refmv_count = (*refmv_count).wrapping_add(1);
            }
        }
        rf_idx += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn setup_ref_mv_list(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut ref_frame: MV_REFERENCE_FRAME,
    refmv_count: *mut uint8_t,
    mut ref_mv_stack: *mut CANDIDATE_MV,
    mut ref_mv_weight: *mut uint16_t,
    mut mv_ref_list: *mut int_mv,
    mut gm_mv_candidates: *mut int_mv,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut mode_context: *mut int16_t,
) {
    let bs: core::ffi::c_int = if (*xd).width as core::ffi::c_int
        > (*xd).height as core::ffi::c_int
    {
        (*xd).width as core::ffi::c_int
    } else {
        (*xd).height as core::ffi::c_int
    };
    let has_tr: core::ffi::c_int = has_top_right(cm, xd, mi_row, mi_col, bs)
        as core::ffi::c_int;
    let mut rf: [MV_REFERENCE_FRAME; 2] = [0; 2];
    let tile: *const TileInfo = &(*xd).tile;
    let mut max_row_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_col_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let row_adj: core::ffi::c_int = (((*xd).height as core::ffi::c_int)
        < mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        && mi_row & 0x1 as core::ffi::c_int != 0) as core::ffi::c_int;
    let col_adj: core::ffi::c_int = (((*xd).width as core::ffi::c_int)
        < mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        && mi_col & 0x1 as core::ffi::c_int != 0) as core::ffi::c_int;
    let mut processed_rows: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut processed_cols: core::ffi::c_int = 0 as core::ffi::c_int;
    av1_set_ref_frame(rf.as_mut_ptr(), ref_frame);
    *mode_context.offset(ref_frame as isize) = 0 as int16_t;
    *refmv_count = 0 as uint8_t;
    if (*xd).up_available {
        max_row_offset = -(MVREF_ROW_COLS << 1 as core::ffi::c_int) + row_adj;
        if ((*xd).height as core::ffi::c_int)
            < mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            max_row_offset = -((2 as core::ffi::c_int) << 1 as core::ffi::c_int)
                + row_adj;
        }
        max_row_offset = find_valid_row_offset(tile, mi_row, max_row_offset);
    }
    if (*xd).left_available {
        max_col_offset = -(MVREF_ROW_COLS << 1 as core::ffi::c_int) + col_adj;
        if ((*xd).width as core::ffi::c_int)
            < mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            max_col_offset = -((2 as core::ffi::c_int) << 1 as core::ffi::c_int)
                + col_adj;
        }
        max_col_offset = find_valid_col_offset(tile, mi_col, max_col_offset);
    }
    let mut col_match_count: uint8_t = 0 as uint8_t;
    let mut row_match_count: uint8_t = 0 as uint8_t;
    let mut newmv_count: uint8_t = 0 as uint8_t;
    if abs(max_row_offset) >= 1 as core::ffi::c_int {
        scan_row_mbmi(
            cm,
            xd,
            mi_col,
            rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
            -(1 as core::ffi::c_int),
            ref_mv_stack as *mut CANDIDATE_MV,
            ref_mv_weight as *mut uint16_t,
            refmv_count,
            &raw mut row_match_count,
            &raw mut newmv_count,
            gm_mv_candidates,
            max_row_offset,
            &raw mut processed_rows,
        );
    }
    if abs(max_col_offset) >= 1 as core::ffi::c_int {
        scan_col_mbmi(
            cm,
            xd,
            mi_row,
            rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
            -(1 as core::ffi::c_int),
            ref_mv_stack as *mut CANDIDATE_MV,
            ref_mv_weight as *mut uint16_t,
            refmv_count,
            &raw mut col_match_count,
            &raw mut newmv_count,
            gm_mv_candidates,
            max_col_offset,
            &raw mut processed_cols,
        );
    }
    if has_tr != 0 {
        scan_blk_mbmi(
            cm,
            xd,
            mi_row,
            mi_col,
            rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
            -(1 as core::ffi::c_int),
            (*xd).width as core::ffi::c_int,
            ref_mv_stack as *mut CANDIDATE_MV,
            ref_mv_weight as *mut uint16_t,
            &raw mut row_match_count,
            &raw mut newmv_count,
            gm_mv_candidates,
            refmv_count,
        );
    }
    let nearest_match: uint8_t = ((row_match_count as core::ffi::c_int
        > 0 as core::ffi::c_int) as core::ffi::c_int
        + (col_match_count as core::ffi::c_int > 0 as core::ffi::c_int)
            as core::ffi::c_int) as uint8_t;
    let nearest_refmv_count: uint8_t = *refmv_count;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < nearest_refmv_count as core::ffi::c_int {
        let fresh0 = &raw mut *ref_mv_weight.offset(idx as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int + REF_CAT_LEVEL) as uint16_t;
        idx += 1;
    }
    if (*cm).features.allow_ref_frame_mvs {
        let mut is_available: core::ffi::c_int = 0 as core::ffi::c_int;
        let voffset: core::ffi::c_int = if mi_size_high[BLOCK_8X8 as core::ffi::c_int
            as usize] as core::ffi::c_int > (*xd).height as core::ffi::c_int
        {
            mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        } else {
            (*xd).height as core::ffi::c_int
        };
        let hoffset: core::ffi::c_int = if mi_size_wide[BLOCK_8X8 as core::ffi::c_int
            as usize] as core::ffi::c_int > (*xd).width as core::ffi::c_int
        {
            mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        } else {
            (*xd).width as core::ffi::c_int
        };
        let blk_row_end: core::ffi::c_int = if ((*xd).height as core::ffi::c_int)
            < mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            (*xd).height as core::ffi::c_int
        } else {
            mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        };
        let blk_col_end: core::ffi::c_int = if ((*xd).width as core::ffi::c_int)
            < mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            (*xd).width as core::ffi::c_int
        } else {
            mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        };
        let tpl_sample_pos: [[core::ffi::c_int; 2]; 3] = [
            [voffset, -(2 as core::ffi::c_int)],
            [voffset, hoffset],
            [voffset - 2 as core::ffi::c_int, hoffset],
        ];
        let allow_extension: core::ffi::c_int = ((*xd).height as core::ffi::c_int
            >= mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
            && ((*xd).height as core::ffi::c_int)
                < mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int
            && (*xd).width as core::ffi::c_int
                >= mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize]
                    as core::ffi::c_int
            && ((*xd).width as core::ffi::c_int)
                < mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int) as core::ffi::c_int;
        let step_h: core::ffi::c_int = if (*xd).height as core::ffi::c_int
            >= mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_high[BLOCK_16X16 as core::ffi::c_int as usize] as core::ffi::c_int
        } else {
            mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        };
        let step_w: core::ffi::c_int = if (*xd).width as core::ffi::c_int
            >= mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_wide[BLOCK_16X16 as core::ffi::c_int as usize] as core::ffi::c_int
        } else {
            mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        };
        let mut blk_row: core::ffi::c_int = 0 as core::ffi::c_int;
        while blk_row < blk_row_end {
            let mut blk_col: core::ffi::c_int = 0 as core::ffi::c_int;
            while blk_col < blk_col_end {
                let mut ret: core::ffi::c_int = add_tpl_ref_mv(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    ref_frame,
                    blk_row,
                    blk_col,
                    gm_mv_candidates,
                    refmv_count,
                    ref_mv_stack,
                    ref_mv_weight,
                    mode_context,
                );
                if blk_row == 0 as core::ffi::c_int && blk_col == 0 as core::ffi::c_int {
                    is_available = ret;
                }
                blk_col += step_w;
            }
            blk_row += step_h;
        }
        if is_available == 0 as core::ffi::c_int {
            let fresh1 = &raw mut *mode_context.offset(ref_frame as isize);
            *fresh1 = (*fresh1 as core::ffi::c_int
                | (1 as core::ffi::c_int) << GLOBALMV_OFFSET) as int16_t;
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 3 as core::ffi::c_int && allow_extension != 0 {
            let blk_row_0: core::ffi::c_int = tpl_sample_pos[i
                as usize][0 as core::ffi::c_int as usize];
            let blk_col_0: core::ffi::c_int = tpl_sample_pos[i
                as usize][1 as core::ffi::c_int as usize];
            if !(check_sb_border(mi_row, mi_col, blk_row_0, blk_col_0) == 0) {
                add_tpl_ref_mv(
                    cm,
                    xd,
                    mi_row,
                    mi_col,
                    ref_frame,
                    blk_row_0,
                    blk_col_0,
                    gm_mv_candidates,
                    refmv_count,
                    ref_mv_stack,
                    ref_mv_weight,
                    mode_context,
                );
            }
            i += 1;
        }
    }
    let mut dummy_newmv_count: uint8_t = 0 as uint8_t;
    scan_blk_mbmi(
        cm,
        xd,
        mi_row,
        mi_col,
        rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
        -(1 as core::ffi::c_int),
        -(1 as core::ffi::c_int),
        ref_mv_stack as *mut CANDIDATE_MV,
        ref_mv_weight as *mut uint16_t,
        &raw mut row_match_count,
        &raw mut dummy_newmv_count,
        gm_mv_candidates,
        refmv_count,
    );
    let mut idx_0: core::ffi::c_int = 2 as core::ffi::c_int;
    while idx_0 <= MVREF_ROW_COLS {
        let row_offset: core::ffi::c_int = -(idx_0 << 1 as core::ffi::c_int)
            + 1 as core::ffi::c_int + row_adj;
        let col_offset: core::ffi::c_int = -(idx_0 << 1 as core::ffi::c_int)
            + 1 as core::ffi::c_int + col_adj;
        if abs(row_offset) <= abs(max_row_offset) && abs(row_offset) > processed_rows {
            scan_row_mbmi(
                cm,
                xd,
                mi_col,
                rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
                row_offset,
                ref_mv_stack as *mut CANDIDATE_MV,
                ref_mv_weight as *mut uint16_t,
                refmv_count,
                &raw mut row_match_count,
                &raw mut dummy_newmv_count,
                gm_mv_candidates,
                max_row_offset,
                &raw mut processed_rows,
            );
        }
        if abs(col_offset) <= abs(max_col_offset) && abs(col_offset) > processed_cols {
            scan_col_mbmi(
                cm,
                xd,
                mi_row,
                rf.as_mut_ptr() as *const MV_REFERENCE_FRAME,
                col_offset,
                ref_mv_stack as *mut CANDIDATE_MV,
                ref_mv_weight as *mut uint16_t,
                refmv_count,
                &raw mut col_match_count,
                &raw mut dummy_newmv_count,
                gm_mv_candidates,
                max_col_offset,
                &raw mut processed_cols,
            );
        }
        idx_0 += 1;
    }
    let ref_match_count: uint8_t = ((row_match_count as core::ffi::c_int
        > 0 as core::ffi::c_int) as core::ffi::c_int
        + (col_match_count as core::ffi::c_int > 0 as core::ffi::c_int)
            as core::ffi::c_int) as uint8_t;
    match nearest_match as core::ffi::c_int {
        0 => {
            if ref_match_count as core::ffi::c_int >= 1 as core::ffi::c_int {
                let fresh2 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh2 = (*fresh2 as core::ffi::c_int | 1 as core::ffi::c_int)
                    as int16_t;
            }
            if ref_match_count as core::ffi::c_int == 1 as core::ffi::c_int {
                let fresh3 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh3 = (*fresh3 as core::ffi::c_int
                    | (1 as core::ffi::c_int) << REFMV_OFFSET) as int16_t;
            } else if ref_match_count as core::ffi::c_int >= 2 as core::ffi::c_int {
                let fresh4 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh4 = (*fresh4 as core::ffi::c_int
                    | (2 as core::ffi::c_int) << REFMV_OFFSET) as int16_t;
            }
        }
        1 => {
            let fresh5 = &raw mut *mode_context.offset(ref_frame as isize);
            *fresh5 = (*fresh5 as core::ffi::c_int
                | if newmv_count as core::ffi::c_int > 0 as core::ffi::c_int {
                    2 as core::ffi::c_int
                } else {
                    3 as core::ffi::c_int
                }) as int16_t;
            if ref_match_count as core::ffi::c_int == 1 as core::ffi::c_int {
                let fresh6 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh6 = (*fresh6 as core::ffi::c_int
                    | (3 as core::ffi::c_int) << REFMV_OFFSET) as int16_t;
            } else if ref_match_count as core::ffi::c_int >= 2 as core::ffi::c_int {
                let fresh7 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh7 = (*fresh7 as core::ffi::c_int
                    | (4 as core::ffi::c_int) << REFMV_OFFSET) as int16_t;
            }
        }
        2 | _ => {
            if newmv_count as core::ffi::c_int >= 1 as core::ffi::c_int {
                let fresh8 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh8 = (*fresh8 as core::ffi::c_int | 4 as core::ffi::c_int)
                    as int16_t;
            } else {
                let fresh9 = &raw mut *mode_context.offset(ref_frame as isize);
                *fresh9 = (*fresh9 as core::ffi::c_int | 5 as core::ffi::c_int)
                    as int16_t;
            }
            let fresh10 = &raw mut *mode_context.offset(ref_frame as isize);
            *fresh10 = (*fresh10 as core::ffi::c_int
                | (5 as core::ffi::c_int) << REFMV_OFFSET) as int16_t;
        }
    }
    let mut len: core::ffi::c_int = nearest_refmv_count as core::ffi::c_int;
    while len > 0 as core::ffi::c_int {
        let mut nr_len: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut idx_1: core::ffi::c_int = 1 as core::ffi::c_int;
        while idx_1 < len {
            if (*ref_mv_weight.offset((idx_1 - 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int)
                < *ref_mv_weight.offset(idx_1 as isize) as core::ffi::c_int
            {
                let tmp_mv: CANDIDATE_MV = *ref_mv_stack
                    .offset((idx_1 - 1 as core::ffi::c_int) as isize);
                let tmp_ref_mv_weight: uint16_t = *ref_mv_weight
                    .offset((idx_1 - 1 as core::ffi::c_int) as isize);
                *ref_mv_stack.offset((idx_1 - 1 as core::ffi::c_int) as isize) = *ref_mv_stack
                    .offset(idx_1 as isize);
                *ref_mv_stack.offset(idx_1 as isize) = tmp_mv;
                *ref_mv_weight.offset((idx_1 - 1 as core::ffi::c_int) as isize) = *ref_mv_weight
                    .offset(idx_1 as isize);
                *ref_mv_weight.offset(idx_1 as isize) = tmp_ref_mv_weight;
                nr_len = idx_1;
            }
            idx_1 += 1;
        }
        len = nr_len;
    }
    len = *refmv_count as core::ffi::c_int;
    while len > nearest_refmv_count as core::ffi::c_int {
        let mut nr_len_0: core::ffi::c_int = nearest_refmv_count as core::ffi::c_int;
        let mut idx_2: core::ffi::c_int = nearest_refmv_count as core::ffi::c_int
            + 1 as core::ffi::c_int;
        while idx_2 < len {
            if (*ref_mv_weight.offset((idx_2 - 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int)
                < *ref_mv_weight.offset(idx_2 as isize) as core::ffi::c_int
            {
                let tmp_mv_0: CANDIDATE_MV = *ref_mv_stack
                    .offset((idx_2 - 1 as core::ffi::c_int) as isize);
                let tmp_ref_mv_weight_0: uint16_t = *ref_mv_weight
                    .offset((idx_2 - 1 as core::ffi::c_int) as isize);
                *ref_mv_stack.offset((idx_2 - 1 as core::ffi::c_int) as isize) = *ref_mv_stack
                    .offset(idx_2 as isize);
                *ref_mv_stack.offset(idx_2 as isize) = tmp_mv_0;
                *ref_mv_weight.offset((idx_2 - 1 as core::ffi::c_int) as isize) = *ref_mv_weight
                    .offset(idx_2 as isize);
                *ref_mv_weight.offset(idx_2 as isize) = tmp_ref_mv_weight_0;
                nr_len_0 = idx_2;
            }
            idx_2 += 1;
        }
        len = nr_len_0;
    }
    let mut mi_width: core::ffi::c_int = if (mi_size_wide[BLOCK_64X64 as core::ffi::c_int
        as usize] as core::ffi::c_int) < (*xd).width as core::ffi::c_int
    {
        mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    } else {
        (*xd).width as core::ffi::c_int
    };
    mi_width = if mi_width < (*cm).mi_params.mi_cols - mi_col {
        mi_width
    } else {
        (*cm).mi_params.mi_cols - mi_col
    };
    let mut mi_height: core::ffi::c_int = if (mi_size_high[BLOCK_64X64
        as core::ffi::c_int as usize] as core::ffi::c_int)
        < (*xd).height as core::ffi::c_int
    {
        mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
    } else {
        (*xd).height as core::ffi::c_int
    };
    mi_height = if mi_height < (*cm).mi_params.mi_rows - mi_row {
        mi_height
    } else {
        (*cm).mi_params.mi_rows - mi_row
    };
    let mi_size: core::ffi::c_int = if mi_width < mi_height {
        mi_width
    } else {
        mi_height
    };
    if rf[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > NONE_FRAME as core::ffi::c_int
    {
        if (*refmv_count as core::ffi::c_int) < MAX_MV_REF_CANDIDATES {
            let mut ref_id: [[int_mv; 2]; 2] = [[int_mv { as_int: 0 }; 2]; 2];
            let mut ref_diff: [[int_mv; 2]; 2] = [[int_mv { as_int: 0 }; 2]; 2];
            let mut ref_id_count: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
            let mut ref_diff_count: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
            let mut idx_3: core::ffi::c_int = 0 as core::ffi::c_int;
            while abs(max_row_offset) >= 1 as core::ffi::c_int && idx_3 < mi_size {
                let candidate: *const MB_MODE_INFO = *((*xd).mi)
                    .offset((-(*xd).mi_stride + idx_3) as isize);
                process_compound_ref_mv_candidate(
                    candidate,
                    cm,
                    rf.as_mut_ptr(),
                    ref_id.as_mut_ptr(),
                    ref_id_count.as_mut_ptr(),
                    ref_diff.as_mut_ptr(),
                    ref_diff_count.as_mut_ptr(),
                );
                idx_3 += mi_size_wide[(*candidate).bsize as usize] as core::ffi::c_int;
            }
            let mut idx_4: core::ffi::c_int = 0 as core::ffi::c_int;
            while abs(max_col_offset) >= 1 as core::ffi::c_int && idx_4 < mi_size {
                let candidate_0: *const MB_MODE_INFO = *((*xd).mi)
                    .offset((idx_4 * (*xd).mi_stride - 1 as core::ffi::c_int) as isize);
                process_compound_ref_mv_candidate(
                    candidate_0,
                    cm,
                    rf.as_mut_ptr(),
                    ref_id.as_mut_ptr(),
                    ref_id_count.as_mut_ptr(),
                    ref_diff.as_mut_ptr(),
                    ref_diff_count.as_mut_ptr(),
                );
                idx_4 += mi_size_high[(*candidate_0).bsize as usize] as core::ffi::c_int;
            }
            let mut comp_list: [[int_mv; 2]; 2] = [[int_mv { as_int: 0 }; 2]; 2];
            let mut idx_5: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx_5 < 2 as core::ffi::c_int {
                let mut comp_idx: core::ffi::c_int = 0 as core::ffi::c_int;
                let mut list_idx: core::ffi::c_int = 0 as core::ffi::c_int;
                while list_idx < ref_id_count[idx_5 as usize]
                    && comp_idx < MAX_MV_REF_CANDIDATES
                {
                    comp_list[comp_idx as usize][idx_5 as usize] = ref_id[idx_5
                        as usize][list_idx as usize];
                    list_idx += 1;
                    comp_idx += 1;
                }
                let mut list_idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while list_idx_0 < ref_diff_count[idx_5 as usize]
                    && comp_idx < MAX_MV_REF_CANDIDATES
                {
                    comp_list[comp_idx as usize][idx_5 as usize] = ref_diff[idx_5
                        as usize][list_idx_0 as usize];
                    list_idx_0 += 1;
                    comp_idx += 1;
                }
                while comp_idx < MAX_MV_REF_CANDIDATES {
                    comp_list[comp_idx as usize][idx_5 as usize] = *gm_mv_candidates
                        .offset(idx_5 as isize);
                    comp_idx += 1;
                }
                idx_5 += 1;
            }
            if *refmv_count != 0 {
                if *refmv_count as core::ffi::c_int == 1 as core::ffi::c_int {} else {
                    __assert_fail(
                        b"*refmv_count == 1\0" as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                            as *const core::ffi::c_char,
                        724 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 169],
                            [core::ffi::c_char; 169],
                        >(
                            *b"void setup_ref_mv_list(const AV1_COMMON *, const MACROBLOCKD *, MV_REFERENCE_FRAME, uint8_t *const, CANDIDATE_MV *, uint16_t *, int_mv *, int_mv *, int, int, int16_t *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if comp_list[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                    .as_int
                    == (*ref_mv_stack.offset(0 as core::ffi::c_int as isize))
                        .this_mv
                        .as_int
                    && comp_list[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                        .as_int
                        == (*ref_mv_stack.offset(0 as core::ffi::c_int as isize))
                            .comp_mv
                            .as_int
                {
                    (*ref_mv_stack.offset(*refmv_count as isize)).this_mv = comp_list[1
                        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                    (*ref_mv_stack.offset(*refmv_count as isize)).comp_mv = comp_list[1
                        as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
                } else {
                    (*ref_mv_stack.offset(*refmv_count as isize)).this_mv = comp_list[0
                        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                    (*ref_mv_stack.offset(*refmv_count as isize)).comp_mv = comp_list[0
                        as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
                }
                *ref_mv_weight.offset(*refmv_count as isize) = 2 as uint16_t;
                *refmv_count = (*refmv_count).wrapping_add(1);
            } else {
                let mut idx_6: core::ffi::c_int = 0 as core::ffi::c_int;
                while idx_6 < MAX_MV_REF_CANDIDATES {
                    (*ref_mv_stack.offset(*refmv_count as isize)).this_mv = comp_list[idx_6
                        as usize][0 as core::ffi::c_int as usize];
                    (*ref_mv_stack.offset(*refmv_count as isize)).comp_mv = comp_list[idx_6
                        as usize][1 as core::ffi::c_int as usize];
                    *ref_mv_weight.offset(*refmv_count as isize) = 2 as uint16_t;
                    *refmv_count = (*refmv_count).wrapping_add(1);
                    idx_6 += 1;
                }
            }
        }
        if *refmv_count as core::ffi::c_int >= 2 as core::ffi::c_int {} else {
            __assert_fail(
                b"*refmv_count >= 2\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                    as *const core::ffi::c_char,
                745 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void setup_ref_mv_list(const AV1_COMMON *, const MACROBLOCKD *, MV_REFERENCE_FRAME, uint8_t *const, CANDIDATE_MV *, uint16_t *, int_mv *, int_mv *, int, int, int16_t *)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut idx_7: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx_7 < *refmv_count as core::ffi::c_int {
            clamp_mv_ref(
                &raw mut (*ref_mv_stack.offset(idx_7 as isize)).this_mv.as_mv,
                ((*xd).width as core::ffi::c_int) << MI_SIZE_LOG2,
                ((*xd).height as core::ffi::c_int) << MI_SIZE_LOG2,
                xd,
            );
            clamp_mv_ref(
                &raw mut (*ref_mv_stack.offset(idx_7 as isize)).comp_mv.as_mv,
                ((*xd).width as core::ffi::c_int) << MI_SIZE_LOG2,
                ((*xd).height as core::ffi::c_int) << MI_SIZE_LOG2,
                xd,
            );
            idx_7 += 1;
        }
    } else {
        let mut idx_8: core::ffi::c_int = 0 as core::ffi::c_int;
        while abs(max_row_offset) >= 1 as core::ffi::c_int && idx_8 < mi_size
            && (*refmv_count as core::ffi::c_int) < MAX_MV_REF_CANDIDATES
        {
            let candidate_1: *const MB_MODE_INFO = *((*xd).mi)
                .offset((-(*xd).mi_stride + idx_8) as isize);
            process_single_ref_mv_candidate(
                candidate_1,
                cm,
                ref_frame,
                refmv_count,
                ref_mv_stack,
                ref_mv_weight,
            );
            idx_8 += mi_size_wide[(*candidate_1).bsize as usize] as core::ffi::c_int;
        }
        let mut idx_9: core::ffi::c_int = 0 as core::ffi::c_int;
        while abs(max_col_offset) >= 1 as core::ffi::c_int && idx_9 < mi_size
            && (*refmv_count as core::ffi::c_int) < MAX_MV_REF_CANDIDATES
        {
            let candidate_2: *const MB_MODE_INFO = *((*xd).mi)
                .offset((idx_9 * (*xd).mi_stride - 1 as core::ffi::c_int) as isize);
            process_single_ref_mv_candidate(
                candidate_2,
                cm,
                ref_frame,
                refmv_count,
                ref_mv_stack,
                ref_mv_weight,
            );
            idx_9 += mi_size_high[(*candidate_2).bsize as usize] as core::ffi::c_int;
        }
        let mut idx_10: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx_10 < *refmv_count as core::ffi::c_int {
            clamp_mv_ref(
                &raw mut (*ref_mv_stack.offset(idx_10 as isize)).this_mv.as_mv,
                ((*xd).width as core::ffi::c_int) << MI_SIZE_LOG2,
                ((*xd).height as core::ffi::c_int) << MI_SIZE_LOG2,
                xd,
            );
            idx_10 += 1;
        }
        if !mv_ref_list.is_null() {
            let mut idx_11: core::ffi::c_int = *refmv_count as core::ffi::c_int;
            while idx_11 < MAX_MV_REF_CANDIDATES {
                (*mv_ref_list.offset(idx_11 as isize)).as_int = (*gm_mv_candidates
                    .offset(0 as core::ffi::c_int as isize))
                    .as_int;
                idx_11 += 1;
            }
            let mut idx_12: core::ffi::c_int = 0 as core::ffi::c_int;
            while idx_12
                < (if (2 as core::ffi::c_int) < *refmv_count as core::ffi::c_int {
                    2 as core::ffi::c_int
                } else {
                    *refmv_count as core::ffi::c_int
                })
            {
                (*mv_ref_list.offset(idx_12 as isize)).as_int = (*ref_mv_stack
                    .offset(idx_12 as isize))
                    .this_mv
                    .as_int;
                idx_12 += 1;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_mv_refs(
    mut cm: *const AV1_COMMON,
    mut xd: *const MACROBLOCKD,
    mut mi: *mut MB_MODE_INFO,
    mut ref_frame: MV_REFERENCE_FRAME,
    mut ref_mv_count: *mut uint8_t,
    mut ref_mv_stack: *mut [CANDIDATE_MV; 8],
    mut ref_mv_weight: *mut [uint16_t; 8],
    mut mv_ref_list: *mut [int_mv; 2],
    mut global_mvs: *mut int_mv,
    mut mode_context: *mut int16_t,
) {
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut gm_mv: [int_mv; 2] = [int_mv { as_int: 0 }; 2];
    if ref_frame as core::ffi::c_int == INTRA_FRAME as core::ffi::c_int {
        gm_mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
        gm_mv[0 as core::ffi::c_int as usize].as_int = gm_mv[1 as core::ffi::c_int
                as usize]
            .as_int;
        if !global_mvs.is_null() {
            (*global_mvs.offset(ref_frame as isize)).as_int = INVALID_MV as uint32_t;
        }
    } else {
        let bsize: BLOCK_SIZE = (*mi).bsize;
        let allow_high_precision_mv: core::ffi::c_int = (*cm)
            .features
            .allow_high_precision_mv as core::ffi::c_int;
        let force_integer_mv: core::ffi::c_int = (*cm)
            .features
            .cur_frame_force_integer_mv as core::ffi::c_int;
        if (ref_frame as core::ffi::c_int) < REF_FRAMES as core::ffi::c_int {
            gm_mv[0 as core::ffi::c_int as usize] = gm_get_motion_vector(
                &*((*cm).global_motion).as_ptr().offset(ref_frame as isize),
                allow_high_precision_mv,
                bsize,
                mi_col,
                mi_row,
                force_integer_mv,
            );
            gm_mv[1 as core::ffi::c_int as usize].as_int = 0 as uint32_t;
            if !global_mvs.is_null() {
                *global_mvs.offset(ref_frame as isize) = gm_mv[0 as core::ffi::c_int
                    as usize];
            }
        } else {
            let mut rf: [MV_REFERENCE_FRAME; 2] = [0; 2];
            av1_set_ref_frame(rf.as_mut_ptr(), ref_frame);
            gm_mv[0 as core::ffi::c_int as usize] = gm_get_motion_vector(
                &*((*cm).global_motion)
                    .as_ptr()
                    .offset(
                        *rf.as_mut_ptr().offset(0 as core::ffi::c_int as isize) as isize,
                    ),
                allow_high_precision_mv,
                bsize,
                mi_col,
                mi_row,
                force_integer_mv,
            );
            gm_mv[1 as core::ffi::c_int as usize] = gm_get_motion_vector(
                &*((*cm).global_motion)
                    .as_ptr()
                    .offset(
                        *rf.as_mut_ptr().offset(1 as core::ffi::c_int as isize) as isize,
                    ),
                allow_high_precision_mv,
                bsize,
                mi_col,
                mi_row,
                force_integer_mv,
            );
        }
    }
    setup_ref_mv_list(
        cm,
        xd,
        ref_frame,
        &raw mut *ref_mv_count.offset(ref_frame as isize),
        (*ref_mv_stack.offset(ref_frame as isize)).as_mut_ptr(),
        (*ref_mv_weight.offset(ref_frame as isize)).as_mut_ptr(),
        if !mv_ref_list.is_null() {
            (*mv_ref_list.offset(ref_frame as isize)).as_mut_ptr()
        } else {
            0 as *mut int_mv
        },
        gm_mv.as_mut_ptr(),
        mi_row,
        mi_col,
        mode_context,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_best_ref_mvs(
    mut allow_hp: core::ffi::c_int,
    mut mvlist: *mut int_mv,
    mut nearest_mv: *mut int_mv,
    mut near_mv: *mut int_mv,
    mut is_integer: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < MAX_MV_REF_CANDIDATES {
        lower_mv_precision(
            &raw mut (*mvlist.offset(i as isize)).as_mv,
            allow_hp,
            is_integer,
        );
        i += 1;
    }
    *nearest_mv = *mvlist.offset(0 as core::ffi::c_int as isize);
    *near_mv = *mvlist.offset(1 as core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_frame_buf_refs(mut cm: *mut AV1_COMMON) {
    (*(*cm).cur_frame).order_hint = (*cm).current_frame.order_hint;
    (*(*cm).cur_frame).display_order_hint = (*cm).current_frame.display_order_hint;
    (*(*cm).cur_frame).pyramid_level = (*cm).current_frame.pyramid_level;
    (*(*cm).cur_frame).base_qindex = (*cm).quant_params.base_qindex;
    (*(*cm).cur_frame).filter_level[0 as core::ffi::c_int as usize] = -(1
        as core::ffi::c_int);
    (*(*cm).cur_frame).filter_level[1 as core::ffi::c_int as usize] = -(1
        as core::ffi::c_int);
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
        if !buf.is_null() {
            (*(*cm).cur_frame)
                .ref_order_hints[(ref_frame as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = (*buf).order_hint;
            (*(*cm).cur_frame)
                .ref_display_order_hint[(ref_frame as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = (*buf).display_order_hint;
        }
        ref_frame += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_frame_sign_bias(mut cm: *mut AV1_COMMON) {
    let mut ref_frame: MV_REFERENCE_FRAME = 0;
    ref_frame = LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME;
    while ref_frame as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(cm, ref_frame);
        if (*(*cm).seq_params).order_hint_info.enable_order_hint != 0 && !buf.is_null() {
            let ref_order_hint: core::ffi::c_int = (*buf).order_hint as core::ffi::c_int;
            (*cm).ref_frame_sign_bias[ref_frame as usize] = if get_relative_dist(
                &raw mut (*(*cm).seq_params).order_hint_info,
                ref_order_hint,
                (*cm).current_frame.order_hint as core::ffi::c_int,
            ) <= 0 as core::ffi::c_int
            {
                0 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            };
        } else {
            (*cm).ref_frame_sign_bias[ref_frame as usize] = 0 as core::ffi::c_int;
        }
        ref_frame += 1;
    }
}
pub const MAX_OFFSET_WIDTH: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_OFFSET_HEIGHT: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_block_position(
    mut cm: *mut AV1_COMMON,
    mut mi_r: *mut core::ffi::c_int,
    mut mi_c: *mut core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut mv: MV,
    mut sign_bias: core::ffi::c_int,
) -> core::ffi::c_int {
    let base_blk_row: core::ffi::c_int = (blk_row >> 3 as core::ffi::c_int)
        << 3 as core::ffi::c_int;
    let base_blk_col: core::ffi::c_int = (blk_col >> 3 as core::ffi::c_int)
        << 3 as core::ffi::c_int;
    let row_offset: core::ffi::c_int = if mv.row as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        mv.row as core::ffi::c_int >> 4 as core::ffi::c_int + MI_SIZE_LOG2
    } else {
        -(-(mv.row as core::ffi::c_int) >> 4 as core::ffi::c_int + MI_SIZE_LOG2)
    };
    let col_offset: core::ffi::c_int = if mv.col as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        mv.col as core::ffi::c_int >> 4 as core::ffi::c_int + MI_SIZE_LOG2
    } else {
        -(-(mv.col as core::ffi::c_int) >> 4 as core::ffi::c_int + MI_SIZE_LOG2)
    };
    let row: core::ffi::c_int = if sign_bias == 1 as core::ffi::c_int {
        blk_row - row_offset
    } else {
        blk_row + row_offset
    };
    let col: core::ffi::c_int = if sign_bias == 1 as core::ffi::c_int {
        blk_col - col_offset
    } else {
        blk_col + col_offset
    };
    if row < 0 as core::ffi::c_int
        || row >= (*cm).mi_params.mi_rows >> 1 as core::ffi::c_int
        || col < 0 as core::ffi::c_int
        || col >= (*cm).mi_params.mi_cols >> 1 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if row < base_blk_row - (MAX_OFFSET_HEIGHT >> 3 as core::ffi::c_int)
        || row
            >= base_blk_row + 8 as core::ffi::c_int
                + (MAX_OFFSET_HEIGHT >> 3 as core::ffi::c_int)
        || col < base_blk_col - (MAX_OFFSET_WIDTH >> 3 as core::ffi::c_int)
        || col
            >= base_blk_col + 8 as core::ffi::c_int
                + (MAX_OFFSET_WIDTH >> 3 as core::ffi::c_int)
    {
        return 0 as core::ffi::c_int;
    }
    *mi_r = row;
    *mi_c = col;
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn motion_field_projection(
    mut cm: *mut AV1_COMMON,
    mut start_frame: MV_REFERENCE_FRAME,
    mut dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut tpl_mvs_base: *mut TPL_MV_REF = (*cm).tpl_mvs;
    let mut ref_offset: [core::ffi::c_int; 8] = [0 as core::ffi::c_int; 8];
    let start_frame_buf: *const RefCntBuffer = get_ref_frame_buf(cm, start_frame);
    if start_frame_buf.is_null() {
        return 0 as core::ffi::c_int;
    }
    if (*start_frame_buf).frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int
        || (*start_frame_buf).frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int
    {
        return 0 as core::ffi::c_int;
    }
    if (*start_frame_buf).mi_rows != (*cm).mi_params.mi_rows
        || (*start_frame_buf).mi_cols != (*cm).mi_params.mi_cols
    {
        return 0 as core::ffi::c_int;
    }
    let start_frame_order_hint: core::ffi::c_int = (*start_frame_buf).order_hint
        as core::ffi::c_int;
    let ref_order_hints: *const core::ffi::c_uint = &*((*start_frame_buf)
        .ref_order_hints)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const core::ffi::c_uint;
    let cur_order_hint: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    let mut start_to_current_frame_offset: core::ffi::c_int = get_relative_dist(
        &raw mut (*(*cm).seq_params).order_hint_info,
        start_frame_order_hint,
        cur_order_hint,
    );
    let mut rf: MV_REFERENCE_FRAME = LAST_FRAME as core::ffi::c_int
        as MV_REFERENCE_FRAME;
    while rf as core::ffi::c_int <= INTER_REFS_PER_FRAME as core::ffi::c_int {
        ref_offset[rf as usize] = get_relative_dist(
            &raw mut (*(*cm).seq_params).order_hint_info,
            start_frame_order_hint,
            *ref_order_hints
                .offset(
                    (rf as core::ffi::c_int - LAST_FRAME as core::ffi::c_int) as isize,
                ) as core::ffi::c_int,
        );
        rf += 1;
    }
    if dir == 2 as core::ffi::c_int {
        start_to_current_frame_offset = -start_to_current_frame_offset;
    }
    let mut mv_ref_base: *mut MV_REF = (*start_frame_buf).mvs;
    let mvs_rows: core::ffi::c_int = (*cm).mi_params.mi_rows + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    let mvs_cols: core::ffi::c_int = (*cm).mi_params.mi_cols + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int;
    let mut blk_row: core::ffi::c_int = 0 as core::ffi::c_int;
    while blk_row < mvs_rows {
        let mut blk_col: core::ffi::c_int = 0 as core::ffi::c_int;
        while blk_col < mvs_cols {
            let mut mv_ref: *mut MV_REF = &raw mut *mv_ref_base
                .offset((blk_row * mvs_cols + blk_col) as isize) as *mut MV_REF;
            let mut fwd_mv: MV = (*mv_ref).mv.as_mv;
            if (*mv_ref).ref_frame as core::ffi::c_int > INTRA_FRAME as core::ffi::c_int
            {
                let mut this_mv: int_mv = int_mv { as_int: 0 };
                let mut mi_r: core::ffi::c_int = 0;
                let mut mi_c: core::ffi::c_int = 0;
                let ref_frame_offset: core::ffi::c_int = ref_offset[(*mv_ref).ref_frame
                    as usize];
                let mut pos_valid: core::ffi::c_int = (abs(ref_frame_offset)
                    <= MAX_FRAME_DISTANCE && ref_frame_offset > 0 as core::ffi::c_int
                    && abs(start_to_current_frame_offset) <= MAX_FRAME_DISTANCE)
                    as core::ffi::c_int;
                if pos_valid != 0 {
                    av1_get_mv_projection(
                        &raw mut this_mv.as_mv,
                        fwd_mv,
                        start_to_current_frame_offset,
                        ref_frame_offset,
                    );
                    pos_valid = get_block_position(
                        cm,
                        &raw mut mi_r,
                        &raw mut mi_c,
                        blk_row,
                        blk_col,
                        this_mv.as_mv,
                        dir >> 1 as core::ffi::c_int,
                    );
                }
                if pos_valid != 0 {
                    let mi_offset: core::ffi::c_int = mi_r
                        * ((*cm).mi_params.mi_stride >> 1 as core::ffi::c_int) + mi_c;
                    (*tpl_mvs_base.offset(mi_offset as isize)).mfmv0.as_mv.row = fwd_mv
                        .row;
                    (*tpl_mvs_base.offset(mi_offset as isize)).mfmv0.as_mv.col = fwd_mv
                        .col;
                    (*tpl_mvs_base.offset(mi_offset as isize)).ref_frame_offset = ref_frame_offset
                        as uint8_t;
                }
            }
            blk_col += 1;
        }
        blk_row += 1;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calculate_ref_frame_side(mut cm: *mut AV1_COMMON) {
    let order_hint_info: *const OrderHintInfo = &raw mut (*(*cm).seq_params).order_hint_info;
    memset(
        ((*cm).ref_frame_side).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 8]>() as size_t,
    );
    if (*order_hint_info).enable_order_hint == 0 {
        return;
    }
    let cur_order_hint: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    let mut ref_frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while ref_frame <= ALTREF_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            ref_frame as MV_REFERENCE_FRAME,
        );
        let mut order_hint: core::ffi::c_int = 0 as core::ffi::c_int;
        if !buf.is_null() {
            order_hint = (*buf).order_hint as core::ffi::c_int;
        }
        if get_relative_dist(order_hint_info, order_hint, cur_order_hint)
            > 0 as core::ffi::c_int
        {
            (*cm).ref_frame_side[ref_frame as usize] = 1 as int8_t;
        } else if order_hint == cur_order_hint {
            (*cm).ref_frame_side[ref_frame as usize] = -(1 as core::ffi::c_int)
                as int8_t;
        }
        ref_frame += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_motion_field(mut cm: *mut AV1_COMMON) {
    let order_hint_info: *const OrderHintInfo = &raw mut (*(*cm).seq_params).order_hint_info;
    if (*order_hint_info).enable_order_hint == 0 {
        return;
    }
    let mut tpl_mvs_base: *mut TPL_MV_REF = (*cm).tpl_mvs;
    let mut size: core::ffi::c_int = ((*cm).mi_params.mi_rows + MAX_MIB_SIZE
        >> 1 as core::ffi::c_int) * ((*cm).mi_params.mi_stride >> 1 as core::ffi::c_int);
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < size {
        (*tpl_mvs_base.offset(idx as isize)).mfmv0.as_int = INVALID_MV as uint32_t;
        (*tpl_mvs_base.offset(idx as isize)).ref_frame_offset = 0 as uint8_t;
        idx += 1;
    }
    let cur_order_hint: core::ffi::c_int = (*(*cm).cur_frame).order_hint
        as core::ffi::c_int;
    let mut ref_buf: [*const RefCntBuffer; 7] = [0 as *const RefCntBuffer; 7];
    let mut ref_order_hint: [core::ffi::c_int; 7] = [0; 7];
    let mut ref_frame: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
    while ref_frame <= ALTREF_FRAME as core::ffi::c_int {
        let ref_idx: core::ffi::c_int = ref_frame
            - LAST_FRAME as core::ffi::c_int as core::ffi::c_int;
        let buf: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            ref_frame as MV_REFERENCE_FRAME,
        );
        let mut order_hint: core::ffi::c_int = 0 as core::ffi::c_int;
        if !buf.is_null() {
            order_hint = (*buf).order_hint as core::ffi::c_int;
        }
        ref_buf[ref_idx as usize] = buf;
        ref_order_hint[ref_idx as usize] = order_hint;
        ref_frame += 1;
    }
    let mut ref_stamp: core::ffi::c_int = MFMV_STACK_SIZE - 1 as core::ffi::c_int;
    if !(ref_buf[(LAST_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
        as usize])
        .is_null()
    {
        let alt_of_lst_order_hint: core::ffi::c_int = (*ref_buf[(LAST_FRAME
            as core::ffi::c_int - LAST_FRAME as core::ffi::c_int) as usize])
            .ref_order_hints[(ALTREF_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] as core::ffi::c_int;
        let is_lst_overlay: core::ffi::c_int = (alt_of_lst_order_hint
            == ref_order_hint[(GOLDEN_FRAME as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize]) as core::ffi::c_int;
        if is_lst_overlay == 0 {
            motion_field_projection(
                cm,
                LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
                2 as core::ffi::c_int,
            );
        }
        ref_stamp -= 1;
    }
    if get_relative_dist(
        order_hint_info,
        ref_order_hint[(BWDREF_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize],
        cur_order_hint,
    ) > 0 as core::ffi::c_int
    {
        if motion_field_projection(
            cm,
            BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            0 as core::ffi::c_int,
        ) != 0
        {
            ref_stamp -= 1;
        }
    }
    if get_relative_dist(
        order_hint_info,
        ref_order_hint[(ALTREF2_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize],
        cur_order_hint,
    ) > 0 as core::ffi::c_int
    {
        if motion_field_projection(
            cm,
            ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            0 as core::ffi::c_int,
        ) != 0
        {
            ref_stamp -= 1;
        }
    }
    if get_relative_dist(
        order_hint_info,
        ref_order_hint[(ALTREF_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize],
        cur_order_hint,
    ) > 0 as core::ffi::c_int && ref_stamp >= 0 as core::ffi::c_int
    {
        if motion_field_projection(
            cm,
            ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            0 as core::ffi::c_int,
        ) != 0
        {
            ref_stamp -= 1;
        }
    }
    if ref_stamp >= 0 as core::ffi::c_int {
        motion_field_projection(
            cm,
            LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
            2 as core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn record_samples(
    mut mbmi: *const MB_MODE_INFO,
    mut pts: *mut core::ffi::c_int,
    mut pts_inref: *mut core::ffi::c_int,
    mut row_offset: core::ffi::c_int,
    mut sign_r: core::ffi::c_int,
    mut col_offset: core::ffi::c_int,
    mut sign_c: core::ffi::c_int,
) {
    let bw: core::ffi::c_int = block_size_wide[(*mbmi).bsize as usize]
        as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[(*mbmi).bsize as usize]
        as core::ffi::c_int;
    let x: core::ffi::c_int = col_offset * MI_SIZE + sign_c * bw / 2 as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let y: core::ffi::c_int = row_offset * MI_SIZE + sign_r * bh / 2 as core::ffi::c_int
        - 1 as core::ffi::c_int;
    *pts.offset(0 as core::ffi::c_int as isize) = x * 8 as core::ffi::c_int;
    *pts.offset(1 as core::ffi::c_int as isize) = y * 8 as core::ffi::c_int;
    *pts_inref.offset(0 as core::ffi::c_int as isize) = *pts
        .offset(0 as core::ffi::c_int as isize)
        + (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv.col as core::ffi::c_int;
    *pts_inref.offset(1 as core::ffi::c_int as isize) = *pts
        .offset(1 as core::ffi::c_int as isize)
        + (*mbmi).mv[0 as core::ffi::c_int as usize].as_mv.row as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_selectSamples(
    mut mv: *mut MV,
    mut pts: *mut core::ffi::c_int,
    mut pts_inref: *mut core::ffi::c_int,
    mut len: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
) -> uint8_t {
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let thresh: core::ffi::c_int = clamp(
        if bw > bh { bw } else { bh },
        16 as core::ffi::c_int,
        112 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let mut ret: uint8_t = 0 as uint8_t;
    if len <= (1 as core::ffi::c_int) << 3 as core::ffi::c_int {} else {
        __assert_fail(
            b"len <= LEAST_SQUARES_SAMPLES_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1098 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 63],
                [core::ffi::c_char; 63],
            >(*b"uint8_t av1_selectSamples(MV *, int *, int *, int, BLOCK_SIZE)\0"))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < len {
        let diff: core::ffi::c_int = abs(
            *pts_inref.offset((2 as core::ffi::c_int * i) as isize)
                - *pts.offset((2 as core::ffi::c_int * i) as isize)
                - (*mv).col as core::ffi::c_int,
        ) as core::ffi::c_int
            + abs(
                *pts_inref
                    .offset((2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize)
                    - *pts
                        .offset(
                            (2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize,
                        ) - (*mv).row as core::ffi::c_int,
            ) as core::ffi::c_int;
        if !(diff > thresh) {
            if ret as core::ffi::c_int != i {
                memcpy(
                    pts
                        .offset(
                            (2 as core::ffi::c_int * ret as core::ffi::c_int) as isize,
                        ) as *mut core::ffi::c_void,
                    pts.offset((2 as core::ffi::c_int * i) as isize)
                        as *const core::ffi::c_void,
                    (2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_int>() as size_t,
                        ),
                );
                memcpy(
                    pts_inref
                        .offset(
                            (2 as core::ffi::c_int * ret as core::ffi::c_int) as isize,
                        ) as *mut core::ffi::c_void,
                    pts_inref.offset((2 as core::ffi::c_int * i) as isize)
                        as *const core::ffi::c_void,
                    (2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_int>() as size_t,
                        ),
                );
            }
            ret = ret.wrapping_add(1);
        }
        i += 1;
    }
    return (if ret as core::ffi::c_int > 1 as core::ffi::c_int {
        ret as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_findSamples(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut pts: *mut core::ffi::c_int,
    mut pts_inref: *mut core::ffi::c_int,
) -> uint8_t {
    let mbmi0: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let ref_frame: core::ffi::c_int = (*mbmi0).ref_frame[0 as core::ffi::c_int as usize]
        as core::ffi::c_int;
    let up_available: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let left_available: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    let mut np: uint8_t = 0 as uint8_t;
    let mut do_tl: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut do_tr: core::ffi::c_int = 1 as core::ffi::c_int;
    let mi_stride: core::ffi::c_int = (*xd).mi_stride;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    if up_available != 0 {
        let mi_row_offset: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
            .offset((mi_row_offset * mi_stride) as isize);
        let mut superblock_width: uint8_t = mi_size_wide[(*mbmi).bsize as usize];
        if (*xd).width as core::ffi::c_int <= superblock_width as core::ffi::c_int {
            let col_offset: core::ffi::c_int = -mi_col
                % superblock_width as core::ffi::c_int;
            if col_offset < 0 as core::ffi::c_int {
                do_tl = 0 as core::ffi::c_int;
            }
            if col_offset + superblock_width as core::ffi::c_int
                > (*xd).width as core::ffi::c_int
            {
                do_tr = 0 as core::ffi::c_int;
            }
            if (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == ref_frame
                && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    == NONE_FRAME as core::ffi::c_int
            {
                record_samples(
                    mbmi,
                    pts,
                    pts_inref,
                    0 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    col_offset,
                    1 as core::ffi::c_int,
                );
                pts = pts.offset(2 as core::ffi::c_int as isize);
                pts_inref = pts_inref.offset(2 as core::ffi::c_int as isize);
                np = np.wrapping_add(1);
                if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                    return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
                }
            }
        } else {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i
                < (if ((*xd).width as core::ffi::c_int)
                    < (*cm).mi_params.mi_cols - mi_col
                {
                    (*xd).width as core::ffi::c_int
                } else {
                    (*cm).mi_params.mi_cols - mi_col
                })
            {
                mbmi = *((*xd).mi).offset((i + mi_row_offset * mi_stride) as isize);
                superblock_width = mi_size_wide[(*mbmi).bsize as usize];
                if (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == ref_frame
                    && (*mbmi).ref_frame[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int == NONE_FRAME as core::ffi::c_int
                {
                    record_samples(
                        mbmi,
                        pts,
                        pts_inref,
                        0 as core::ffi::c_int,
                        -(1 as core::ffi::c_int),
                        i,
                        1 as core::ffi::c_int,
                    );
                    pts = pts.offset(2 as core::ffi::c_int as isize);
                    pts_inref = pts_inref.offset(2 as core::ffi::c_int as isize);
                    np = np.wrapping_add(1);
                    if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                        return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
                    }
                }
                i += superblock_width as core::ffi::c_int;
            }
        }
    }
    if np as core::ffi::c_int <= (1 as core::ffi::c_int) << 3 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"np <= LEAST_SQUARES_SAMPLES_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1168 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 73],
                [core::ffi::c_char; 73],
            >(
                *b"uint8_t av1_findSamples(const AV1_COMMON *, MACROBLOCKD *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    if left_available != 0 {
        let mi_col_offset: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mut mbmi_0: *const MB_MODE_INFO = *((*xd).mi).offset(mi_col_offset as isize);
        let mut superblock_height: uint8_t = mi_size_high[(*mbmi_0).bsize as usize];
        if (*xd).height as core::ffi::c_int <= superblock_height as core::ffi::c_int {
            let row_offset: core::ffi::c_int = -mi_row
                % superblock_height as core::ffi::c_int;
            if row_offset < 0 as core::ffi::c_int {
                do_tl = 0 as core::ffi::c_int;
            }
            if (*mbmi_0).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == ref_frame
                && (*mbmi_0).ref_frame[1 as core::ffi::c_int as usize]
                    as core::ffi::c_int == NONE_FRAME as core::ffi::c_int
            {
                record_samples(
                    mbmi_0,
                    pts,
                    pts_inref,
                    row_offset,
                    1 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                );
                pts = pts.offset(2 as core::ffi::c_int as isize);
                pts_inref = pts_inref.offset(2 as core::ffi::c_int as isize);
                np = np.wrapping_add(1);
                if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                    return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
                }
            }
        } else {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0
                < (if ((*xd).height as core::ffi::c_int)
                    < (*cm).mi_params.mi_rows - mi_row
                {
                    (*xd).height as core::ffi::c_int
                } else {
                    (*cm).mi_params.mi_rows - mi_row
                })
            {
                mbmi_0 = *((*xd).mi).offset((mi_col_offset + i_0 * mi_stride) as isize);
                superblock_height = mi_size_high[(*mbmi_0).bsize as usize];
                if (*mbmi_0).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int == ref_frame
                    && (*mbmi_0).ref_frame[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int == NONE_FRAME as core::ffi::c_int
                {
                    record_samples(
                        mbmi_0,
                        pts,
                        pts_inref,
                        i_0,
                        1 as core::ffi::c_int,
                        0 as core::ffi::c_int,
                        -(1 as core::ffi::c_int),
                    );
                    pts = pts.offset(2 as core::ffi::c_int as isize);
                    pts_inref = pts_inref.offset(2 as core::ffi::c_int as isize);
                    np = np.wrapping_add(1);
                    if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                        return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
                    }
                }
                i_0 += superblock_height as core::ffi::c_int;
            }
        }
    }
    if np as core::ffi::c_int <= (1 as core::ffi::c_int) << 3 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"np <= LEAST_SQUARES_SAMPLES_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1207 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 73],
                [core::ffi::c_char; 73],
            >(
                *b"uint8_t av1_findSamples(const AV1_COMMON *, MACROBLOCKD *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    if do_tl != 0 && left_available != 0 && up_available != 0 {
        let mi_row_offset_0: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mi_col_offset_0: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mut mbmi_1: *mut MB_MODE_INFO = *((*xd).mi)
            .offset((mi_col_offset_0 + mi_row_offset_0 * mi_stride) as isize);
        if (*mbmi_1).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_frame
            && (*mbmi_1).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == NONE_FRAME as core::ffi::c_int
        {
            record_samples(
                mbmi_1,
                pts,
                pts_inref,
                0 as core::ffi::c_int,
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                -(1 as core::ffi::c_int),
            );
            pts = pts.offset(2 as core::ffi::c_int as isize);
            pts_inref = pts_inref.offset(2 as core::ffi::c_int as isize);
            np = np.wrapping_add(1);
            if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
            }
        }
    }
    if np as core::ffi::c_int <= (1 as core::ffi::c_int) << 3 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"np <= LEAST_SQUARES_SAMPLES_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1222 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 73],
                [core::ffi::c_char; 73],
            >(
                *b"uint8_t av1_findSamples(const AV1_COMMON *, MACROBLOCKD *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    if do_tr != 0
        && has_top_right(
            cm,
            xd,
            mi_row,
            mi_col,
            (if (*xd).width as core::ffi::c_int > (*xd).height as core::ffi::c_int {
                (*xd).width as core::ffi::c_int
            } else {
                (*xd).height as core::ffi::c_int
            }),
        ) != 0
    {
        let trb_pos: POSITION = {
            let mut init = position {
                row: -(1 as core::ffi::c_int),
                col: (*xd).width as core::ffi::c_int,
            };
            init
        };
        let tile: *const TileInfo = &raw mut (*xd).tile;
        if is_inside(tile, mi_col, mi_row, &trb_pos) != 0 {
            let mi_row_offset_1: core::ffi::c_int = -(1 as core::ffi::c_int);
            let mi_col_offset_1: core::ffi::c_int = (*xd).width as core::ffi::c_int;
            let mut mbmi_2: *const MB_MODE_INFO = *((*xd).mi)
                .offset((mi_col_offset_1 + mi_row_offset_1 * mi_stride) as isize);
            if (*mbmi_2).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == ref_frame
                && (*mbmi_2).ref_frame[1 as core::ffi::c_int as usize]
                    as core::ffi::c_int == NONE_FRAME as core::ffi::c_int
            {
                record_samples(
                    mbmi_2,
                    pts,
                    pts_inref,
                    0 as core::ffi::c_int,
                    -(1 as core::ffi::c_int),
                    (*xd).width as core::ffi::c_int,
                    1 as core::ffi::c_int,
                );
                np = np.wrapping_add(1);
                if np as core::ffi::c_int >= LEAST_SQUARES_SAMPLES_MAX {
                    return LEAST_SQUARES_SAMPLES_MAX as uint8_t;
                }
            }
        }
    }
    if np as core::ffi::c_int <= (1 as core::ffi::c_int) << 3 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"np <= LEAST_SQUARES_SAMPLES_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1241 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 73],
                [core::ffi::c_char; 73],
            >(
                *b"uint8_t av1_findSamples(const AV1_COMMON *, MACROBLOCKD *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    return np;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_skip_mode_allowed(mut cm: *mut AV1_COMMON) {
    let order_hint_info: *const OrderHintInfo = &raw mut (*(*cm).seq_params).order_hint_info;
    let skip_mode_info: *mut SkipModeInfo = &raw mut (*cm).current_frame.skip_mode_info;
    (*skip_mode_info).skip_mode_allowed = 0 as core::ffi::c_int;
    (*skip_mode_info).ref_frame_idx_0 = INVALID_IDX;
    (*skip_mode_info).ref_frame_idx_1 = INVALID_IDX;
    if (*order_hint_info).enable_order_hint == 0 || frame_is_intra_only(cm) != 0
        || (*cm).current_frame.reference_mode as core::ffi::c_int
            == SINGLE_REFERENCE as core::ffi::c_int
    {
        return;
    }
    let cur_order_hint: core::ffi::c_int = (*cm).current_frame.order_hint
        as core::ffi::c_int;
    let mut ref_order_hints: [core::ffi::c_int; 2] = [-(1 as core::ffi::c_int), INT_MAX];
    let mut ref_idx: [core::ffi::c_int; 2] = [INVALID_IDX, INVALID_IDX];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < INTER_REFS_PER_FRAME as core::ffi::c_int {
        let buf: *const RefCntBuffer = get_ref_frame_buf(
            cm,
            (LAST_FRAME as core::ffi::c_int + i) as MV_REFERENCE_FRAME,
        );
        if !buf.is_null() {
            let ref_order_hint: core::ffi::c_int = (*buf).order_hint as core::ffi::c_int;
            if get_relative_dist(order_hint_info, ref_order_hint, cur_order_hint)
                < 0 as core::ffi::c_int
            {
                if ref_order_hints[0 as core::ffi::c_int as usize]
                    == -(1 as core::ffi::c_int)
                    || get_relative_dist(
                        order_hint_info,
                        ref_order_hint,
                        ref_order_hints[0 as core::ffi::c_int as usize],
                    ) > 0 as core::ffi::c_int
                {
                    ref_order_hints[0 as core::ffi::c_int as usize] = ref_order_hint;
                    ref_idx[0 as core::ffi::c_int as usize] = i;
                }
            } else if get_relative_dist(order_hint_info, ref_order_hint, cur_order_hint)
                > 0 as core::ffi::c_int
            {
                if ref_order_hints[1 as core::ffi::c_int as usize] == INT_MAX
                    || get_relative_dist(
                        order_hint_info,
                        ref_order_hint,
                        ref_order_hints[1 as core::ffi::c_int as usize],
                    ) < 0 as core::ffi::c_int
                {
                    ref_order_hints[1 as core::ffi::c_int as usize] = ref_order_hint;
                    ref_idx[1 as core::ffi::c_int as usize] = i;
                }
            }
        }
        i += 1;
    }
    if ref_idx[0 as core::ffi::c_int as usize] != INVALID_IDX
        && ref_idx[1 as core::ffi::c_int as usize] != INVALID_IDX
    {
        (*skip_mode_info).skip_mode_allowed = 1 as core::ffi::c_int;
        (*skip_mode_info).ref_frame_idx_0 = if ref_idx[0 as core::ffi::c_int as usize]
            < ref_idx[1 as core::ffi::c_int as usize]
        {
            ref_idx[0 as core::ffi::c_int as usize]
        } else {
            ref_idx[1 as core::ffi::c_int as usize]
        };
        (*skip_mode_info).ref_frame_idx_1 = if ref_idx[0 as core::ffi::c_int as usize]
            > ref_idx[1 as core::ffi::c_int as usize]
        {
            ref_idx[0 as core::ffi::c_int as usize]
        } else {
            ref_idx[1 as core::ffi::c_int as usize]
        };
    } else if ref_idx[0 as core::ffi::c_int as usize] != INVALID_IDX
        && ref_idx[1 as core::ffi::c_int as usize] == INVALID_IDX
    {
        ref_order_hints[1 as core::ffi::c_int as usize] = -(1 as core::ffi::c_int);
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < INTER_REFS_PER_FRAME as core::ffi::c_int {
            let buf_0: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                (LAST_FRAME as core::ffi::c_int + i_0) as MV_REFERENCE_FRAME,
            );
            if !buf_0.is_null() {
                let ref_order_hint_0: core::ffi::c_int = (*buf_0).order_hint
                    as core::ffi::c_int;
                if ref_order_hints[0 as core::ffi::c_int as usize]
                    != -(1 as core::ffi::c_int)
                    && get_relative_dist(
                        order_hint_info,
                        ref_order_hint_0,
                        ref_order_hints[0 as core::ffi::c_int as usize],
                    ) < 0 as core::ffi::c_int
                    && (ref_order_hints[1 as core::ffi::c_int as usize]
                        == -(1 as core::ffi::c_int)
                        || get_relative_dist(
                            order_hint_info,
                            ref_order_hint_0,
                            ref_order_hints[1 as core::ffi::c_int as usize],
                        ) > 0 as core::ffi::c_int)
                {
                    ref_order_hints[1 as core::ffi::c_int as usize] = ref_order_hint_0;
                    ref_idx[1 as core::ffi::c_int as usize] = i_0;
                }
            }
            i_0 += 1;
        }
        if ref_order_hints[1 as core::ffi::c_int as usize] != -(1 as core::ffi::c_int) {
            (*skip_mode_info).skip_mode_allowed = 1 as core::ffi::c_int;
            (*skip_mode_info).ref_frame_idx_0 = if ref_idx[0 as core::ffi::c_int
                as usize] < ref_idx[1 as core::ffi::c_int as usize]
            {
                ref_idx[0 as core::ffi::c_int as usize]
            } else {
                ref_idx[1 as core::ffi::c_int as usize]
            };
            (*skip_mode_info).ref_frame_idx_1 = if ref_idx[0 as core::ffi::c_int
                as usize] > ref_idx[1 as core::ffi::c_int as usize]
            {
                ref_idx[0 as core::ffi::c_int as usize]
            } else {
                ref_idx[1 as core::ffi::c_int as usize]
            };
        }
    }
}
unsafe extern "C-unwind" fn compare_ref_frame_info(
    mut arg_a: *const core::ffi::c_void,
    mut arg_b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let mut info_a: *const REF_FRAME_INFO = arg_a as *mut REF_FRAME_INFO;
    let mut info_b: *const REF_FRAME_INFO = arg_b as *mut REF_FRAME_INFO;
    let sort_idx_diff: core::ffi::c_int = (*info_a).sort_idx - (*info_b).sort_idx;
    if sort_idx_diff != 0 as core::ffi::c_int {
        return sort_idx_diff;
    }
    return (*info_a).map_idx - (*info_b).map_idx;
}
#[inline]
unsafe extern "C-unwind" fn set_ref_frame_info(
    mut remapped_ref_idx: *mut core::ffi::c_int,
    mut frame_idx: core::ffi::c_int,
    mut ref_info: *mut REF_FRAME_INFO,
) {
    if frame_idx >= 0 as core::ffi::c_int
        && frame_idx < INTER_REFS_PER_FRAME as core::ffi::c_int
    {} else {
        __assert_fail(
            b"frame_idx >= 0 && frame_idx < INTER_REFS_PER_FRAME\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1341 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 54],
                [core::ffi::c_char; 54],
            >(*b"void set_ref_frame_info(int *, int, REF_FRAME_INFO *)\0"))
                .as_ptr(),
        );
    };
    *remapped_ref_idx.offset(frame_idx as isize) = (*ref_info).map_idx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_frame_refs(
    cm: *mut AV1_COMMON,
    mut remapped_ref_idx: *mut core::ffi::c_int,
    mut lst_map_idx: core::ffi::c_int,
    mut gld_map_idx: core::ffi::c_int,
) {
    let mut lst_frame_sort_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut gld_frame_sort_idx: core::ffi::c_int = -(1 as core::ffi::c_int);
    if (*(*cm).seq_params).order_hint_info.enable_order_hint != 0 {} else {
        __assert_fail(
            b"cm->seq_params->order_hint_info.enable_order_hint\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1351 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 60],
                [core::ffi::c_char; 60],
            >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                .as_ptr(),
        );
    };
    if (*(*cm).seq_params).order_hint_info.order_hint_bits_minus_1
        >= 0 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"cm->seq_params->order_hint_info.order_hint_bits_minus_1 >= 0\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1352 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 60],
                [core::ffi::c_char; 60],
            >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                .as_ptr(),
        );
    };
    let cur_order_hint: core::ffi::c_int = (*cm).current_frame.order_hint
        as core::ffi::c_int;
    let cur_frame_sort_idx: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*(*cm).seq_params).order_hint_info.order_hint_bits_minus_1;
    let mut ref_frame_info: [REF_FRAME_INFO; 8] = [REF_FRAME_INFO {
        map_idx: 0,
        buf: 0 as *mut RefCntBuffer,
        sort_idx: 0,
    }; 8];
    let mut ref_flag_list: [core::ffi::c_int; 7] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < REF_FRAMES as core::ffi::c_int {
        let map_idx: core::ffi::c_int = i;
        ref_frame_info[i as usize].map_idx = map_idx;
        ref_frame_info[i as usize].sort_idx = -(1 as core::ffi::c_int);
        let buf: *mut RefCntBuffer = (*cm).ref_frame_map[map_idx as usize];
        ref_frame_info[i as usize].buf = buf;
        if !buf.is_null() {
            if (*buf).ref_count > 0 as core::ffi::c_int {} else {
                __assert_fail(
                    b"buf->ref_count > 0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1371 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 60],
                        [core::ffi::c_char; 60],
                    >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                        .as_ptr(),
                );
            };
            let offset: core::ffi::c_int = (*buf).order_hint as core::ffi::c_int;
            ref_frame_info[i as usize].sort_idx = if offset == -(1 as core::ffi::c_int) {
                -(1 as core::ffi::c_int)
            } else {
                cur_frame_sort_idx
                    + get_relative_dist(
                        &raw mut (*(*cm).seq_params).order_hint_info,
                        offset,
                        cur_order_hint,
                    )
            };
            if ref_frame_info[i as usize].sort_idx >= -(1 as core::ffi::c_int) {} else {
                __assert_fail(
                    b"ref_frame_info[i].sort_idx >= -1\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1379 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 60],
                        [core::ffi::c_char; 60],
                    >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                        .as_ptr(),
                );
            };
            if map_idx == lst_map_idx {
                lst_frame_sort_idx = ref_frame_info[i as usize].sort_idx;
            }
            if map_idx == gld_map_idx {
                gld_frame_sort_idx = ref_frame_info[i as usize].sort_idx;
            }
        }
        i += 1;
    }
    if lst_frame_sort_idx == -(1 as core::ffi::c_int)
        || lst_frame_sort_idx >= cur_frame_sort_idx
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_CORRUPT_FRAME,
            b"Inter frame requests a look-ahead frame as LAST\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if gld_frame_sort_idx == -(1 as core::ffi::c_int)
        || gld_frame_sort_idx >= cur_frame_sort_idx
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_CORRUPT_FRAME,
            b"Inter frame requests a look-ahead frame as GOLDEN\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    qsort(
        ref_frame_info.as_mut_ptr() as *mut core::ffi::c_void,
        REF_FRAMES as core::ffi::c_int as size_t,
        ::core::mem::size_of::<REF_FRAME_INFO>() as size_t,
        Some(
            compare_ref_frame_info
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    let mut fwd_start_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut fwd_end_idx: core::ffi::c_int = REF_FRAMES as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < REF_FRAMES as core::ffi::c_int {
        if ref_frame_info[i_0 as usize].sort_idx == -(1 as core::ffi::c_int) {
            fwd_start_idx += 1;
        } else if ref_frame_info[i_0 as usize].sort_idx >= cur_frame_sort_idx {
            fwd_end_idx = i_0 - 1 as core::ffi::c_int;
            break;
        }
        i_0 += 1;
    }
    let mut bwd_start_idx: core::ffi::c_int = fwd_end_idx + 1 as core::ffi::c_int;
    let mut bwd_end_idx: core::ffi::c_int = REF_FRAMES as core::ffi::c_int
        - 1 as core::ffi::c_int;
    if bwd_start_idx <= bwd_end_idx {
        set_ref_frame_info(
            remapped_ref_idx,
            ALTREF_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
            &raw mut *ref_frame_info.as_mut_ptr().offset(bwd_end_idx as isize),
        );
        ref_flag_list[(ALTREF_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
            as usize] = 1 as core::ffi::c_int;
        bwd_end_idx -= 1;
    }
    if bwd_start_idx <= bwd_end_idx {
        set_ref_frame_info(
            remapped_ref_idx,
            BWDREF_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
            &raw mut *ref_frame_info.as_mut_ptr().offset(bwd_start_idx as isize),
        );
        ref_flag_list[(BWDREF_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
            as usize] = 1 as core::ffi::c_int;
        bwd_start_idx += 1;
    }
    if bwd_start_idx <= bwd_end_idx {
        set_ref_frame_info(
            remapped_ref_idx,
            ALTREF2_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
            &raw mut *ref_frame_info.as_mut_ptr().offset(bwd_start_idx as isize),
        );
        ref_flag_list[(ALTREF2_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] = 1 as core::ffi::c_int;
    }
    let mut i_1: core::ffi::c_int = fwd_start_idx;
    while i_1 <= fwd_end_idx {
        if ref_frame_info[i_1 as usize].map_idx == lst_map_idx {
            set_ref_frame_info(
                remapped_ref_idx,
                LAST_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
                &raw mut *ref_frame_info.as_mut_ptr().offset(i_1 as isize),
            );
            ref_flag_list[(LAST_FRAME as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = 1 as core::ffi::c_int;
        }
        if ref_frame_info[i_1 as usize].map_idx == gld_map_idx {
            set_ref_frame_info(
                remapped_ref_idx,
                GOLDEN_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
                &raw mut *ref_frame_info.as_mut_ptr().offset(i_1 as isize),
            );
            ref_flag_list[(GOLDEN_FRAME as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = 1 as core::ffi::c_int;
        }
        i_1 += 1;
    }
    if ref_flag_list[(LAST_FRAME as core::ffi::c_int - LAST_FRAME as core::ffi::c_int)
        as usize] == 1 as core::ffi::c_int
        && ref_flag_list[(GOLDEN_FRAME as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] == 1 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"ref_flag_list[LAST_FRAME - LAST_FRAME] == 1 && ref_flag_list[GOLDEN_FRAME - LAST_FRAME] == 1\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                as *const core::ffi::c_char,
            1464 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 60],
                [core::ffi::c_char; 60],
            >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                .as_ptr(),
        );
    };
    static mut ref_frame_list: [MV_REFERENCE_FRAME; 5] = [
        LAST2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        LAST3_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        BWDREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF2_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        ALTREF_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
    ];
    let mut ref_idx: core::ffi::c_int = 0;
    ref_idx = 0 as core::ffi::c_int;
    while ref_idx < INTER_REFS_PER_FRAME as core::ffi::c_int - 2 as core::ffi::c_int {
        let ref_frame: MV_REFERENCE_FRAME = ref_frame_list[ref_idx as usize];
        if !(ref_flag_list[(ref_frame as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] == 1 as core::ffi::c_int)
        {
            while fwd_start_idx <= fwd_end_idx
                && (ref_frame_info[fwd_end_idx as usize].map_idx == lst_map_idx
                    || ref_frame_info[fwd_end_idx as usize].map_idx == gld_map_idx)
            {
                fwd_end_idx -= 1;
            }
            if fwd_start_idx > fwd_end_idx {
                break;
            }
            set_ref_frame_info(
                remapped_ref_idx,
                ref_frame as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
                &raw mut *ref_frame_info.as_mut_ptr().offset(fwd_end_idx as isize),
            );
            ref_flag_list[(ref_frame as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = 1 as core::ffi::c_int;
            fwd_end_idx -= 1;
        }
        ref_idx += 1;
    }
    while ref_idx < INTER_REFS_PER_FRAME as core::ffi::c_int - 2 as core::ffi::c_int {
        let ref_frame_0: MV_REFERENCE_FRAME = ref_frame_list[ref_idx as usize];
        if !(ref_flag_list[(ref_frame_0 as core::ffi::c_int
            - LAST_FRAME as core::ffi::c_int) as usize] == 1 as core::ffi::c_int)
        {
            set_ref_frame_info(
                remapped_ref_idx,
                ref_frame_0 as core::ffi::c_int - LAST_FRAME as core::ffi::c_int,
                &raw mut *ref_frame_info.as_mut_ptr().offset(fwd_start_idx as isize),
            );
            ref_flag_list[(ref_frame_0 as core::ffi::c_int
                - LAST_FRAME as core::ffi::c_int) as usize] = 1 as core::ffi::c_int;
        }
        ref_idx += 1;
    }
    let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_2 < INTER_REFS_PER_FRAME as core::ffi::c_int {
        if ref_flag_list[i_2 as usize] == 1 as core::ffi::c_int {} else {
            __assert_fail(
                b"ref_flag_list[i] == 1\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/mvref_common.c\0" as *const u8
                    as *const core::ffi::c_char,
                1508 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 60],
                    [core::ffi::c_char; 60],
                >(*b"void av1_set_frame_refs(AV1_COMMON *const, int *, int, int)\0"))
                    .as_ptr(),
            );
        };
        i_2 += 1;
    }
}
pub const LEAST_SQUARES_SAMPLES_MAX_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const LEAST_SQUARES_SAMPLES_MAX: core::ffi::c_int = (1 as core::ffi::c_int)
    << LEAST_SQUARES_SAMPLES_MAX_BITS;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;

extern "C" {
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
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
