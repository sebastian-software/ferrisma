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

// Mechanically translated from upstream/aom/av1/common/idct.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type aom_internal_error_info;
    static mut av1_highbd_inv_txfm_add: Option<
        unsafe extern "C-unwind" fn(
            *const tran_low_t,
            *mut uint8_t,
            core::ffi::c_int,
            *const TxfmParam,
        ) -> (),
    >;
    fn av1_highbd_iwht4x4_16_add_c(
        input: *const tran_low_t,
        dest: *mut uint8_t,
        dest_stride: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_highbd_iwht4x4_1_add_c(
        input: *const tran_low_t,
        dest: *mut uint8_t,
        dest_stride: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_16x16_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_16x32_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_16x4_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_16x64_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_16x8_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_32x16_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_32x32_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_32x64_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_32x8_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_4x16_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_4x4_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_4x8_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_64x16_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_64x32_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_64x64_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_8x16_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_8x32_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_8x4_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
    fn av1_inv_txfm2d_add_8x8_c(
        input: *const int32_t,
        output: *mut uint16_t,
        stride: core::ffi::c_int,
        tx_type: TX_TYPE,
        bd: core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
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
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const EXT_TX_SET_TYPES: C2RustUnnamed_12 = 6;
pub const EXT_TX_SET_ALL16: C2RustUnnamed_12 = 5;
pub const EXT_TX_SET_DTT9_IDTX_1DDCT: C2RustUnnamed_12 = 4;
pub const EXT_TX_SET_DTT4_IDTX_1DDCT: C2RustUnnamed_12 = 3;
pub const EXT_TX_SET_DTT4_IDTX: C2RustUnnamed_12 = 2;
pub const EXT_TX_SET_DCT_IDTX: C2RustUnnamed_12 = 1;
pub const EXT_TX_SET_DCTONLY: C2RustUnnamed_12 = 0;
pub type TxSetType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txfm_param {
    pub tx_type: TX_TYPE,
    pub tx_size: TX_SIZE,
    pub lossless: core::ffi::c_int,
    pub bd: core::ffi::c_int,
    pub is_hbd: core::ffi::c_int,
    pub tx_set_type: TxSetType,
    pub eob: core::ffi::c_int,
}
pub type TxfmParam = txfm_param;
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
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_13 = 7;
pub const BWD_REFS: C2RustUnnamed_13 = 3;
pub const FWD_REFS: C2RustUnnamed_13 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_13 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_13 = 8;
pub const REF_FRAMES: C2RustUnnamed_13 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_13 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_13 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_13 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_13 = 4;
pub const LAST3_FRAME: C2RustUnnamed_13 = 3;
pub const LAST2_FRAME: C2RustUnnamed_13 = 2;
pub const LAST_FRAME: C2RustUnnamed_13 = 1;
pub const INTRA_FRAME: C2RustUnnamed_13 = 0;
pub const NONE_FRAME: C2RustUnnamed_13 = -1;
pub type MV_REFERENCE_FRAME = int8_t;
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
}
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
pub type ENTROPY_CONTEXT = core::ffi::c_char;
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
pub type FRAME_CONTEXT = frame_contexts;
pub type CANDIDATE_MV = candidate_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct candidate_mv {
    pub this_mv: int_mv,
    pub comp_mv: int_mv,
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
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
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
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_TX_SIZE_LOG2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_TX_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_TX_SIZE_LOG2;
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
static mut tx_size_2d: [core::ffi::c_int; 20] = [
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
    256 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    4096 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    128 as core::ffi::c_int,
    128 as core::ffi::c_int,
    512 as core::ffi::c_int,
    512 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    64 as core::ffi::c_int,
    64 as core::ffi::c_int,
    256 as core::ffi::c_int,
    256 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    0,
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
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
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
unsafe extern "C-unwind" fn cast_to_int32(mut input: *const tran_low_t) -> *const int32_t {
    return input as *const int32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_tx_scale(tx_size: TX_SIZE) -> core::ffi::c_int {
    let pels: core::ffi::c_int = tx_size_2d[tx_size as usize];
    return (pels > 256 as core::ffi::c_int) as core::ffi::c_int
        + (pels > 1024 as core::ffi::c_int) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_iwht4x4_add(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut eob: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    if eob > 1 as core::ffi::c_int {
        av1_highbd_iwht4x4_16_add_c(input, dest, stride, bd);
    } else {
        av1_highbd_iwht4x4_1_add_c(input, dest, stride, bd);
    };
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_4x4_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut eob: core::ffi::c_int = (*txfm_param).eob;
    let mut bd: core::ffi::c_int = (*txfm_param).bd;
    let mut lossless: core::ffi::c_int = (*txfm_param).lossless;
    let mut src: *const int32_t = cast_to_int32(input);
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    if lossless != 0 {
        av1_highbd_iwht4x4_add(input, dest, stride, eob, bd);
        return;
    }
    av1_inv_txfm2d_add_4x4_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        tx_type,
        bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_4x8_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_4x8_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_8x4_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_8x4_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_16x32_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_16x32_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_32x16_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_32x16_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_16x4_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_16x4_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_4x16_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_4x16_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_32x8_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_32x8_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_8x32_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_8x32_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_32x64_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_32x64_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_64x32_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_64x32_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_16x64_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_16x64_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_64x16_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_64x16_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_8x8_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut bd: core::ffi::c_int = (*txfm_param).bd;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_8x8_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        tx_type,
        bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_16x16_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut bd: core::ffi::c_int = (*txfm_param).bd;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_16x16_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        tx_type,
        bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_8x16_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_8x16_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_16x8_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_16x8_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        (*txfm_param).tx_type,
        (*txfm_param).bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_32x32_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let bd: core::ffi::c_int = (*txfm_param).bd;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_32x32_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        tx_type,
        bd,
    );
}
unsafe extern "C-unwind" fn highbd_inv_txfm_add_64x64_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let bd: core::ffi::c_int = (*txfm_param).bd;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut src: *const int32_t = cast_to_int32(input);
    av1_inv_txfm2d_add_64x64_c(
        src,
        ((dest as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        stride,
        tx_type,
        bd,
    );
}
unsafe extern "C-unwind" fn init_txfm_param(
    mut xd: *const MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut tx_type: TX_TYPE,
    mut eob: core::ffi::c_int,
    mut reduced_tx_set: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    (*txfm_param).tx_type = tx_type;
    (*txfm_param).tx_size = tx_size;
    (*txfm_param).eob = eob;
    (*txfm_param).lossless = (*xd)
        .lossless[(**((*xd).mi).offset(0 as core::ffi::c_int as isize)).segment_id()
        as usize];
    (*txfm_param).bd = (*xd).bd;
    (*txfm_param).is_hbd = is_cur_buf_hbd(xd);
    (*txfm_param).tx_set_type = av1_get_ext_tx_set_type(
        (*txfm_param).tx_size,
        is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)),
        reduced_tx_set,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_inv_txfm_add_c(
    mut input: *const tran_low_t,
    mut dest: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let tx_size: TX_SIZE = (*txfm_param).tx_size;
    match tx_size as core::ffi::c_int {
        3 => {
            highbd_inv_txfm_add_32x32_c(input, dest, stride, txfm_param);
        }
        2 => {
            highbd_inv_txfm_add_16x16_c(input, dest, stride, txfm_param);
        }
        1 => {
            highbd_inv_txfm_add_8x8_c(input, dest, stride, txfm_param);
        }
        5 => {
            highbd_inv_txfm_add_4x8_c(input, dest, stride, txfm_param);
        }
        6 => {
            highbd_inv_txfm_add_8x4_c(input, dest, stride, txfm_param);
        }
        7 => {
            highbd_inv_txfm_add_8x16_c(input, dest, stride, txfm_param);
        }
        8 => {
            highbd_inv_txfm_add_16x8_c(input, dest, stride, txfm_param);
        }
        9 => {
            highbd_inv_txfm_add_16x32_c(input, dest, stride, txfm_param);
        }
        10 => {
            highbd_inv_txfm_add_32x16_c(input, dest, stride, txfm_param);
        }
        4 => {
            highbd_inv_txfm_add_64x64_c(input, dest, stride, txfm_param);
        }
        11 => {
            highbd_inv_txfm_add_32x64_c(input, dest, stride, txfm_param);
        }
        12 => {
            highbd_inv_txfm_add_64x32_c(input, dest, stride, txfm_param);
        }
        17 => {
            highbd_inv_txfm_add_16x64_c(input, dest, stride, txfm_param);
        }
        18 => {
            highbd_inv_txfm_add_64x16_c(input, dest, stride, txfm_param);
        }
        0 => {
            highbd_inv_txfm_add_4x4_c(input, dest, stride, txfm_param);
        }
        14 => {
            highbd_inv_txfm_add_16x4_c(input, dest, stride, txfm_param);
        }
        13 => {
            highbd_inv_txfm_add_4x16_c(input, dest, stride, txfm_param);
        }
        15 => {
            highbd_inv_txfm_add_8x32_c(input, dest, stride, txfm_param);
        }
        16 => {
            highbd_inv_txfm_add_32x8_c(input, dest, stride, txfm_param);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm_add_c(
    mut dqcoeff: *const tran_low_t,
    mut dst: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut txfm_param: *const TxfmParam,
) {
    let tx_size: TX_SIZE = (*txfm_param).tx_size;
    let mut tmp: crate::platform::Aligned32<[uint16_t; 4096]> = crate::platform::Aligned32([0; 4096]);
    let mut tmp_stride: core::ffi::c_int = MAX_TX_SIZE;
    let mut w: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let mut h: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < h {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < w {
            tmp.0[(r * tmp_stride + c) as usize] = *dst.offset((r * stride + c) as isize)
                as uint16_t;
            c += 1;
        }
        r += 1;
    }
    av1_highbd_inv_txfm_add
        .expect(
            "non-null function pointer",
        )(
        dqcoeff,
        (tmp.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        tmp_stride,
        txfm_param,
    );
    let mut r_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while r_0 < h {
        let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_0 < w {
            *dst.offset((r_0 * stride + c_0) as isize) = tmp.0[(r_0 * tmp_stride + c_0)
                as usize] as uint8_t;
            c_0 += 1;
        }
        r_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inverse_transform_block(
    mut xd: *const MACROBLOCKD,
    mut dqcoeff: *const tran_low_t,
    mut plane: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut dst: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut eob: core::ffi::c_int,
    mut reduced_tx_set: core::ffi::c_int,
) {
    if eob == 0 {
        return;
    }
    let mut txfm_param: TxfmParam = txfm_param {
        tx_type: 0,
        tx_size: 0,
        lossless: 0,
        bd: 0,
        is_hbd: 0,
        tx_set_type: 0,
        eob: 0,
    };
    init_txfm_param(xd, plane, tx_size, tx_type, eob, reduced_tx_set, &raw mut txfm_param);
    if txfm_param.is_hbd != 0 {
        av1_highbd_inv_txfm_add
            .expect("non-null function pointer")(dqcoeff, dst, stride, &raw mut txfm_param);
    } else {
        av1_inv_txfm_add_c(dqcoeff, dst, stride, &raw mut txfm_param);
    };
}
