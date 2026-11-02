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

// Mechanically translated from upstream/aom/av1/common/pred_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
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
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed = 7;
pub const BWD_REFS: C2RustUnnamed = 3;
pub const FWD_REFS: C2RustUnnamed = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed = 7;
pub const EXTREF_FRAME: C2RustUnnamed = 8;
pub const REF_FRAMES: C2RustUnnamed = 8;
pub const ALTREF_FRAME: C2RustUnnamed = 7;
pub const ALTREF2_FRAME: C2RustUnnamed = 6;
pub const BWDREF_FRAME: C2RustUnnamed = 5;
pub const GOLDEN_FRAME: C2RustUnnamed = 4;
pub const LAST3_FRAME: C2RustUnnamed = 3;
pub const LAST2_FRAME: C2RustUnnamed = 2;
pub const LAST_FRAME: C2RustUnnamed = 1;
pub const INTRA_FRAME: C2RustUnnamed = 0;
pub const NONE_FRAME: C2RustUnnamed = -1;
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
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub c2rust_unnamed_0: C2RustUnnamed_8,
    pub c2rust_unnamed_1: C2RustUnnamed_6,
    pub c2rust_unnamed_2: C2RustUnnamed_4,
    pub c2rust_unnamed_3: C2RustUnnamed_2,
    pub c2rust_unnamed_4: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MIN_SB_SIZE_LOG2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PALETTE_MAX_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
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
unsafe extern "C-unwind" fn get_ref_filter_type(
    mut ref_mbmi: *const MB_MODE_INFO,
    mut xd: *const MACROBLOCKD,
    mut dir: core::ffi::c_int,
    mut ref_frame: MV_REFERENCE_FRAME,
) -> InterpFilter {
    return (if (*ref_mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        == ref_frame as core::ffi::c_int
        || (*ref_mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_frame as core::ffi::c_int
    {
        av1_extract_interp_filter(
            (*ref_mbmi).interp_filters,
            dir & 0x1 as core::ffi::c_int,
        ) as core::ffi::c_int
    } else {
        SWITCHABLE_FILTERS as core::ffi::c_int
    }) as InterpFilter;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_switchable_interp(
    mut xd: *const MACROBLOCKD,
    mut dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let ctx_offset: core::ffi::c_int = ((*mbmi).ref_frame[1 as core::ffi::c_int as usize]
        as core::ffi::c_int > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int
        * (SWITCHABLE_FILTERS as core::ffi::c_int as core::ffi::c_int
            + 1 as core::ffi::c_int);
    if !(dir == 0 as core::ffi::c_int || dir == 1 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"av1_get_pred_context_switchable_interp\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            34 as core::ffi::c_int,
            b"dir == 0 || dir == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ref_frame: MV_REFERENCE_FRAME = (*mbmi)
        .ref_frame[0 as core::ffi::c_int as usize];
    let mut filter_type_ctx: core::ffi::c_int = ctx_offset
        + (dir & 0x1 as core::ffi::c_int)
            * ((SWITCHABLE_FILTERS as core::ffi::c_int + 1 as core::ffi::c_int)
                * 2 as core::ffi::c_int);
    let mut left_type: core::ffi::c_int = SWITCHABLE_FILTERS as core::ffi::c_int;
    let mut above_type: core::ffi::c_int = SWITCHABLE_FILTERS as core::ffi::c_int;
    if (*xd).left_available {
        left_type = get_ref_filter_type(
            *((*xd).mi).offset(-(1 as core::ffi::c_int) as isize),
            xd,
            dir,
            ref_frame,
        ) as core::ffi::c_int;
    }
    if (*xd).up_available {
        above_type = get_ref_filter_type(
            *((*xd).mi).offset(-(*xd).mi_stride as isize),
            xd,
            dir,
            ref_frame,
        ) as core::ffi::c_int;
    }
    if left_type == above_type {
        filter_type_ctx += left_type;
    } else if left_type == SWITCHABLE_FILTERS as core::ffi::c_int {
        if !(above_type != SWITCHABLE_FILTERS as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 39],
                    [core::ffi::c_char; 39],
                >(*b"av1_get_pred_context_switchable_interp\0"))
                    .as_ptr(),
                b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
                54 as core::ffi::c_int,
                b"above_type != SWITCHABLE_FILTERS\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        filter_type_ctx += above_type;
    } else if above_type == SWITCHABLE_FILTERS as core::ffi::c_int {
        if !(left_type != SWITCHABLE_FILTERS as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 39],
                    [core::ffi::c_char; 39],
                >(*b"av1_get_pred_context_switchable_interp\0"))
                    .as_ptr(),
                b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
                57 as core::ffi::c_int,
                b"left_type != SWITCHABLE_FILTERS\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        filter_type_ctx += left_type;
    } else {
        filter_type_ctx += SWITCHABLE_FILTERS as core::ffi::c_int;
    }
    return filter_type_ctx;
}
unsafe extern "C-unwind" fn palette_add_to_cache(
    mut cache: *mut uint16_t,
    mut n: *mut core::ffi::c_int,
    mut val: uint16_t,
) {
    if *n > 0 as core::ffi::c_int
        && val as core::ffi::c_int
            == *cache.offset((*n - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
    {
        return;
    }
    let fresh4 = *n;
    *n = *n + 1;
    *cache.offset(fresh4 as isize) = val;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_palette_cache(
    xd: *const MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut cache: *mut uint16_t,
) -> core::ffi::c_int {
    let row: core::ffi::c_int = -(*xd).mb_to_top_edge >> 3 as core::ffi::c_int;
    let above_mi: *const MB_MODE_INFO = if row
        % ((1 as core::ffi::c_int) << MIN_SB_SIZE_LOG2) != 0
    {
        (*xd).above_mbmi
    } else {
        0 as *mut MB_MODE_INFO
    };
    let left_mi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let mut above_n: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut left_n: core::ffi::c_int = 0 as core::ffi::c_int;
    if !above_mi.is_null() {
        above_n = (*above_mi)
            .palette_mode_info
            .palette_size[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
            as core::ffi::c_int;
    }
    if !left_mi.is_null() {
        left_n = (*left_mi)
            .palette_mode_info
            .palette_size[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
            as core::ffi::c_int;
    }
    if above_n == 0 as core::ffi::c_int && left_n == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut above_idx: core::ffi::c_int = plane * PALETTE_MAX_SIZE;
    let mut left_idx: core::ffi::c_int = plane * PALETTE_MAX_SIZE;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut above_colors: *const uint16_t = if !above_mi.is_null() {
        ((*above_mi).palette_mode_info.palette_colors).as_ptr()
    } else {
        0 as *const uint16_t
    };
    let mut left_colors: *const uint16_t = if !left_mi.is_null() {
        ((*left_mi).palette_mode_info.palette_colors).as_ptr()
    } else {
        0 as *const uint16_t
    };
    while above_n > 0 as core::ffi::c_int && left_n > 0 as core::ffi::c_int {
        let mut v_above: uint16_t = *above_colors.offset(above_idx as isize);
        let mut v_left: uint16_t = *left_colors.offset(left_idx as isize);
        if (v_left as core::ffi::c_int) < v_above as core::ffi::c_int {
            palette_add_to_cache(cache, &raw mut n, v_left);
            left_idx += 1;
            left_n -= 1;
        } else {
            palette_add_to_cache(cache, &raw mut n, v_above);
            above_idx += 1;
            above_n -= 1;
            if v_left as core::ffi::c_int == v_above as core::ffi::c_int {
                left_idx += 1;
                left_n -= 1;
            }
        }
    }
    loop {
        let fresh0 = above_n;
        above_n = above_n - 1;
        if !(fresh0 > 0 as core::ffi::c_int) {
            break;
        }
        let fresh1 = above_idx;
        above_idx = above_idx + 1;
        let mut val: uint16_t = *above_colors.offset(fresh1 as isize);
        palette_add_to_cache(cache, &raw mut n, val);
    }
    loop {
        let fresh2 = left_n;
        left_n = left_n - 1;
        if !(fresh2 > 0 as core::ffi::c_int) {
            break;
        }
        let fresh3 = left_idx;
        left_idx = left_idx + 1;
        let mut val_0: uint16_t = *left_colors.offset(fresh3 as isize);
        palette_add_to_cache(cache, &raw mut n, val_0);
    }
    if !(n <= 2 as core::ffi::c_int * 8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_get_palette_cache\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            113 as core::ffi::c_int,
            b"n <= 2 * PALETTE_MAX_SIZE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return n;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_intra_inter_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let has_above: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let has_left: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    if has_above != 0 && has_left != 0 {
        let above_intra: core::ffi::c_int = (is_inter_block(above_mbmi) == 0)
            as core::ffi::c_int;
        let left_intra: core::ffi::c_int = (is_inter_block(left_mbmi) == 0)
            as core::ffi::c_int;
        return if left_intra != 0 && above_intra != 0 {
            3 as core::ffi::c_int
        } else {
            (left_intra != 0 || above_intra != 0) as core::ffi::c_int
        };
    } else if has_above != 0 || has_left != 0 {
        return 2 as core::ffi::c_int
            * (is_inter_block((if has_above != 0 { above_mbmi } else { left_mbmi }))
                == 0) as core::ffi::c_int
    } else {
        return 0 as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_reference_mode_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut ctx: core::ffi::c_int = 0;
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let has_above: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let has_left: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    if has_above != 0 && has_left != 0 {
        if has_second_ref(above_mbmi) == 0 && has_second_ref(left_mbmi) == 0 {
            ctx = ((*above_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                && (*above_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int)
                as core::ffi::c_int
                ^ ((*left_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                    && (*left_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int)
                    as core::ffi::c_int;
        } else if has_second_ref(above_mbmi) == 0 {
            ctx = 2 as core::ffi::c_int
                + ((*above_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                    && (*above_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int
                    || is_inter_block(above_mbmi) == 0) as core::ffi::c_int;
        } else if has_second_ref(left_mbmi) == 0 {
            ctx = 2 as core::ffi::c_int
                + ((*left_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                    && (*left_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                        as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int
                    || is_inter_block(left_mbmi) == 0) as core::ffi::c_int;
        } else {
            ctx = 4 as core::ffi::c_int;
        }
    } else if has_above != 0 || has_left != 0 {
        let mut edge_mbmi: *const MB_MODE_INFO = if has_above != 0 {
            above_mbmi
        } else {
            left_mbmi
        };
        if has_second_ref(edge_mbmi) == 0 {
            ctx = ((*edge_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                && (*edge_mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int <= ALTREF_FRAME as core::ffi::c_int)
                as core::ffi::c_int;
        } else {
            ctx = 3 as core::ffi::c_int;
        }
    } else {
        ctx = 1 as core::ffi::c_int;
    }
    if !(ctx >= 0 as core::ffi::c_int && ctx < 5 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"av1_get_reference_mode_context\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            183 as core::ffi::c_int,
            b"ctx >= 0 && ctx < COMP_INTER_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return ctx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_comp_reference_type_context(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let mut pred_context: core::ffi::c_int = 0;
    let above_mbmi: *const MB_MODE_INFO = (*xd).above_mbmi;
    let left_mbmi: *const MB_MODE_INFO = (*xd).left_mbmi;
    let above_in_image: core::ffi::c_int = (*xd).up_available as core::ffi::c_int;
    let left_in_image: core::ffi::c_int = (*xd).left_available as core::ffi::c_int;
    if above_in_image != 0 && left_in_image != 0 {
        let above_intra: core::ffi::c_int = (is_inter_block(above_mbmi) == 0)
            as core::ffi::c_int;
        let left_intra: core::ffi::c_int = (is_inter_block(left_mbmi) == 0)
            as core::ffi::c_int;
        if above_intra != 0 && left_intra != 0 {
            pred_context = 2 as core::ffi::c_int;
        } else if above_intra != 0 || left_intra != 0 {
            let mut inter_mbmi: *const MB_MODE_INFO = if above_intra != 0 {
                left_mbmi
            } else {
                above_mbmi
            };
            if has_second_ref(inter_mbmi) == 0 {
                pred_context = 2 as core::ffi::c_int;
            } else {
                pred_context = 1 as core::ffi::c_int
                    + 2 as core::ffi::c_int * has_uni_comp_refs(inter_mbmi);
            }
        } else {
            let a_sg: core::ffi::c_int = (has_second_ref(above_mbmi) == 0)
                as core::ffi::c_int;
            let l_sg: core::ffi::c_int = (has_second_ref(left_mbmi) == 0)
                as core::ffi::c_int;
            let frfa: MV_REFERENCE_FRAME = (*above_mbmi)
                .ref_frame[0 as core::ffi::c_int as usize];
            let frfl: MV_REFERENCE_FRAME = (*left_mbmi)
                .ref_frame[0 as core::ffi::c_int as usize];
            if a_sg != 0 && l_sg != 0 {
                pred_context = 1 as core::ffi::c_int
                    + 2 as core::ffi::c_int
                        * ((frfa as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                            && frfa as core::ffi::c_int
                                <= ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int
                            ^ (frfl as core::ffi::c_int
                                >= BWDREF_FRAME as core::ffi::c_int
                                && frfl as core::ffi::c_int
                                    <= ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int
                            == 0) as core::ffi::c_int;
            } else if l_sg != 0 || a_sg != 0 {
                let uni_rfc: core::ffi::c_int = if a_sg != 0 {
                    has_uni_comp_refs(left_mbmi) as core::ffi::c_int
                } else {
                    has_uni_comp_refs(above_mbmi) as core::ffi::c_int
                };
                if uni_rfc == 0 {
                    pred_context = 1 as core::ffi::c_int;
                } else {
                    pred_context = 3 as core::ffi::c_int
                        + ((frfa as core::ffi::c_int >= BWDREF_FRAME as core::ffi::c_int
                            && frfa as core::ffi::c_int
                                <= ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int
                            ^ (frfl as core::ffi::c_int
                                >= BWDREF_FRAME as core::ffi::c_int
                                && frfl as core::ffi::c_int
                                    <= ALTREF_FRAME as core::ffi::c_int) as core::ffi::c_int
                            == 0) as core::ffi::c_int;
                }
            } else {
                let a_uni_rfc: core::ffi::c_int = has_uni_comp_refs(above_mbmi)
                    as core::ffi::c_int;
                let l_uni_rfc: core::ffi::c_int = has_uni_comp_refs(left_mbmi)
                    as core::ffi::c_int;
                if a_uni_rfc == 0 && l_uni_rfc == 0 {
                    pred_context = 0 as core::ffi::c_int;
                } else if a_uni_rfc == 0 || l_uni_rfc == 0 {
                    pred_context = 2 as core::ffi::c_int;
                } else {
                    pred_context = 3 as core::ffi::c_int
                        + ((frfa as core::ffi::c_int == BWDREF_FRAME as core::ffi::c_int)
                            as core::ffi::c_int
                            ^ (frfl as core::ffi::c_int
                                == BWDREF_FRAME as core::ffi::c_int) as core::ffi::c_int
                            == 0) as core::ffi::c_int;
                }
            }
        }
    } else if above_in_image != 0 || left_in_image != 0 {
        let mut edge_mbmi: *const MB_MODE_INFO = if above_in_image != 0 {
            above_mbmi
        } else {
            left_mbmi
        };
        if is_inter_block(edge_mbmi) == 0 {
            pred_context = 2 as core::ffi::c_int;
        } else if has_second_ref(edge_mbmi) == 0 {
            pred_context = 2 as core::ffi::c_int;
        } else {
            pred_context = 4 as core::ffi::c_int * has_uni_comp_refs(edge_mbmi);
        }
    } else {
        pred_context = 2 as core::ffi::c_int;
    }
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 5 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_comp_reference_type_context\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            253 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < COMP_REF_TYPE_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_uni_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let frf_count: core::ffi::c_int = *ref_counts
        .offset(LAST_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(LAST2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(LAST3_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(GOLDEN_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let brf_count: core::ffi::c_int = *ref_counts
        .offset(BWDREF_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(ALTREF2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(ALTREF_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if frf_count == brf_count {
        1 as core::ffi::c_int
    } else if frf_count < brf_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_pred_context_uni_comp_ref_p\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            278 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < UNI_COMP_REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_uni_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let last2_count: core::ffi::c_int = *ref_counts
        .offset(LAST2_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let last3_or_gld_count: core::ffi::c_int = *ref_counts
        .offset(LAST3_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(GOLDEN_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if last2_count == last3_or_gld_count {
        1 as core::ffi::c_int
    } else if last2_count < last3_or_gld_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_get_pred_context_uni_comp_ref_p1\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            303 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < UNI_COMP_REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_uni_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let last3_count: core::ffi::c_int = *ref_counts
        .offset(LAST3_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let gld_count: core::ffi::c_int = *ref_counts
        .offset(GOLDEN_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if last3_count == gld_count {
        1 as core::ffi::c_int
    } else if last3_count < gld_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_get_pred_context_uni_comp_ref_p2\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            326 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < UNI_COMP_REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
unsafe extern "C-unwind" fn get_pred_context_ll2_or_l3gld(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let last_last2_count: core::ffi::c_int = *ref_counts
        .offset(LAST_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(LAST2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let last3_gld_count: core::ffi::c_int = *ref_counts
        .offset(LAST3_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(GOLDEN_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if last_last2_count == last3_gld_count {
        1 as core::ffi::c_int
    } else if last_last2_count < last3_gld_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"get_pred_context_ll2_or_l3gld\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            347 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
unsafe extern "C-unwind" fn get_pred_context_last_or_last2(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let last_count: core::ffi::c_int = *ref_counts
        .offset(LAST_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let last2_count: core::ffi::c_int = *ref_counts
        .offset(LAST2_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if last_count == last2_count {
        1 as core::ffi::c_int
    } else if last_count < last2_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"get_pred_context_last_or_last2\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            363 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
unsafe extern "C-unwind" fn get_pred_context_last3_or_gld(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let last3_count: core::ffi::c_int = *ref_counts
        .offset(LAST3_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let gld_count: core::ffi::c_int = *ref_counts
        .offset(GOLDEN_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if last3_count == gld_count {
        1 as core::ffi::c_int
    } else if last3_count < gld_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"get_pred_context_last3_or_gld\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            379 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
unsafe extern "C-unwind" fn get_pred_context_brfarf2_or_arf(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let brfarf2_count: core::ffi::c_int = *ref_counts
        .offset(BWDREF_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(ALTREF2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let arf_count: core::ffi::c_int = *ref_counts
        .offset(ALTREF_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if brfarf2_count == arf_count {
        1 as core::ffi::c_int
    } else if brfarf2_count < arf_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"get_pred_context_brfarf2_or_arf\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            396 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
unsafe extern "C-unwind" fn get_pred_context_brf_or_arf2(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let brf_count: core::ffi::c_int = *ref_counts
        .offset(BWDREF_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let arf2_count: core::ffi::c_int = *ref_counts
        .offset(ALTREF2_FRAME as core::ffi::c_int as isize) as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if brf_count == arf2_count {
        1 as core::ffi::c_int
    } else if brf_count < arf2_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"get_pred_context_brf_or_arf2\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            412 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_comp_ref_p(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_ll2_or_l3gld(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_comp_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_last_or_last2(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_comp_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_last3_or_gld(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_comp_bwdref_p(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_brfarf2_or_arf(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_comp_bwdref_p1(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_brf_or_arf2(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p1(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    let ref_counts: *const uint8_t = &*((*xd).neighbors_ref_counts)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const uint8_t;
    let fwd_count: core::ffi::c_int = *ref_counts
        .offset(LAST_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(LAST2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(LAST3_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(GOLDEN_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let bwd_count: core::ffi::c_int = *ref_counts
        .offset(BWDREF_FRAME as core::ffi::c_int as isize) as core::ffi::c_int
        + *ref_counts.offset(ALTREF2_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int
        + *ref_counts.offset(ALTREF_FRAME as core::ffi::c_int as isize)
            as core::ffi::c_int;
    let pred_context: core::ffi::c_int = if fwd_count == bwd_count {
        1 as core::ffi::c_int
    } else if fwd_count < bwd_count {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    if !(pred_context >= 0 as core::ffi::c_int && pred_context < 3 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_get_pred_context_single_ref_p1\0"))
                .as_ptr(),
            b"pred_common.c\0" as *const u8 as *const core::ffi::c_char,
            468 as core::ffi::c_int,
            b"pred_context >= 0 && pred_context < REF_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return pred_context;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p2(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_brfarf2_or_arf(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p3(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_ll2_or_l3gld(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p4(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_last_or_last2(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p5(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_last3_or_gld(xd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_pred_context_single_ref_p6(
    mut xd: *const MACROBLOCKD,
) -> core::ffi::c_int {
    return get_pred_context_brf_or_arf2(xd);
}
