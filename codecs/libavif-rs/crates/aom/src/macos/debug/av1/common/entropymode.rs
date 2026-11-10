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

// Mechanically translated from upstream/aom/av1/common/entropymode.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    fn av1_clearall_segfeatures(seg: *mut segmentation);
    fn av1_default_coef_probs(cm: *mut AV1Common);
    fn av1_init_mv_probs(cm: *mut AV1Common);
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
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
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
pub type AV1_COMMON = AV1Common;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const PALETTE_MAX_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const CDF_PROB_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const CDF_PROB_TOP: core::ffi::c_int = (1 as core::ffi::c_int) << CDF_PROB_BITS;
pub const SPATIAL_PREDICTION_PROBS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NUM_PALETTE_NEIGHBORS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
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
static mut default_kf_y_mode_cdf: [[[aom_cdf_prob; 14]; 5]; 5] = [
    [
        [
            (CDF_PROB_TOP - 15588 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17027 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19338 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20218 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20682 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21110 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21825 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23244 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24189 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28165 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29093 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30466 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 12016 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18066 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19516 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20303 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20719 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21444 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21888 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23032 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24434 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28658 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30172 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31409 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10052 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10771 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22296 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22788 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23055 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23239 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24133 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25620 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26160 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29336 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29929 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31567 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 14091 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15406 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16442 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18808 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19136 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19546 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19998 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22096 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24746 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29585 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30958 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32462 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 12122 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13265 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15603 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16501 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18609 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20033 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22391 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25583 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26437 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30261 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31073 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32475 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 10023 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19585 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20848 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21440 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21832 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22760 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23089 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24023 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25381 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29014 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30482 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31436 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5983 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24099 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24560 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24886 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25066 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25795 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25913 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26423 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27610 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29905 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31276 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31794 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7444 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12781 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20177 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20728 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21077 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21607 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22170 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23405 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24469 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27915 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29090 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30492 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8537 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14689 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15432 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17087 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17408 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18172 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18408 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19825 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24649 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29153 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31096 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32210 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7543 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14231 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15496 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16195 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17905 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20717 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21984 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24516 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26001 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29675 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30981 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31994 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 12613 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13591 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21383 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22004 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22312 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22577 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23401 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25055 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25729 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29538 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30305 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32077 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 9687 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13470 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18506 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19230 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19604 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20147 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20695 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22062 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23219 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27743 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29211 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30907 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 6183 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6505 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26024 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26252 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26366 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26434 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27082 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28354 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28555 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30467 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30794 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32086 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10718 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11734 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14954 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17224 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17565 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17924 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18561 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21523 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23878 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28975 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30287 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32252 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 9194 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9858 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16501 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17263 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18424 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19171 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21563 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25961 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26561 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30072 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30737 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32463 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 12602 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14399 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15488 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18381 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18778 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19315 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19724 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21419 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25060 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29696 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30917 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32409 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8203 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13821 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14524 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17105 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17439 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18131 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18404 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19468 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25225 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29485 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31158 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32342 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8451 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9731 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15004 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17643 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18012 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18425 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19070 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21538 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24605 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29118 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30078 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32018 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7714 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9048 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9516 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16667 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16817 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16994 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17153 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18767 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26743 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30389 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31536 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32528 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8843 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10280 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11496 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15317 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16652 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17943 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19108 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22718 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25769 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29953 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30983 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32485 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 12578 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13671 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15979 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16834 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19075 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20913 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22989 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25449 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26219 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30214 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31150 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32477 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 9563 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13626 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15080 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15892 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17756 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20863 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22207 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24236 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25380 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29653 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31143 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32277 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8356 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8901 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17616 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18256 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19350 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20106 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22598 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25947 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26466 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29900 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30523 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32261 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10835 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11815 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13124 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16042 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17018 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18039 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18947 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22753 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24615 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29489 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30883 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32482 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7618 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8288 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9859 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10509 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15386 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18657 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22903 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28776 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29180 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31355 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31802 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32593 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_angle_delta_cdf: [[aom_cdf_prob; 8]; 8] = [
    [
        (CDF_PROB_TOP - 2180 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 5032 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7567 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22776 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26989 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30217 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2301 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 5608 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8801 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23487 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26974 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30330 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 3780 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11018 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13699 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19354 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23083 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31286 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4581 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11226 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15147 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17138 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21834 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28397 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1737 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10927 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14509 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19588 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22745 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28823 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2664 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10176 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12485 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17650 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21600 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30495 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15453 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20341 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22561 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28917 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 3605 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10428 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12459 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17676 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21244 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30655 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_if_y_mode_cdf: [[aom_cdf_prob; 14]; 4] = [
    [
        (CDF_PROB_TOP - 22801 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23489 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24293 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24756 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25601 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26123 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26606 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27418 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27945 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29228 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29685 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30349 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18673 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22631 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23318 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23950 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24649 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25527 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27364 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28152 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29701 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29984 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30852 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 19770 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20979 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23396 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23939 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24241 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24654 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25136 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27073 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27830 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29360 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29730 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30659 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20155 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21301 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22838 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23178 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23261 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23533 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23703 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24804 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25352 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26575 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27016 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28049 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_uv_mode_cdf: [[[aom_cdf_prob; 15]; 13]; 2] = [
    [
        [
            (CDF_PROB_TOP - 22631 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24152 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25378 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25661 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25986 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26520 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27055 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27923 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28244 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30059 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30941 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31961 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 9513 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26881 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26973 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27046 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27118 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27664 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27739 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27824 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28359 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29505 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29800 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31796 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 9845 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9915 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28663 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28704 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28757 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28780 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29198 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29822 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29854 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30764 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31777 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32029 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 13639 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13897 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14171 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25331 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25606 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25727 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25953 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27148 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28577 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30612 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31355 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32493 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 9764 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9835 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9930 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9954 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25386 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27053 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27958 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28148 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28243 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31101 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31744 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32363 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 11825 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13589 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13677 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13720 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15048 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29213 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29301 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29458 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29711 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31161 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31441 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32550 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 14175 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14399 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16608 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16821 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17718 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17775 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28551 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30200 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30245 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31837 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32342 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32667 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 12885 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13038 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14978 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15590 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15673 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15748 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16176 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29128 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29267 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30643 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31961 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32461 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 12026 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13661 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13874 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15305 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15490 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15726 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15995 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16273 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28443 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30388 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30767 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32416 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 19052 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19840 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20579 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20916 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21150 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21467 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21885 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22719 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23174 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28861 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30379 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32175 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 18627 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19649 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20974 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21219 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21492 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21816 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22199 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23119 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23527 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27053 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31397 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32148 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 17026 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19004 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19997 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20339 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20586 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21103 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21349 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21907 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22482 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25896 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26541 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31819 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 12124 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13759 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14959 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14992 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15007 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15051 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15078 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15166 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15255 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15753 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16039 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16606 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 10407 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11208 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12900 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13181 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13823 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14175 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14899 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15656 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15986 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20086 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20995 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22455 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24212 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 4532 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19780 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20057 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20215 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20428 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21071 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21199 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21451 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22099 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24228 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24693 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27032 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29472 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5273 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5379 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20177 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20270 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20385 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20439 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20949 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21695 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21774 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23138 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24256 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24703 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26679 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 6740 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7167 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7662 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14152 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14536 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14785 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15034 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16741 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18371 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21520 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22206 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23389 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24182 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 4987 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5368 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5928 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6068 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19114 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20315 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21857 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22253 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22411 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24911 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25380 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26027 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26376 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5370 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6889 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7247 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7393 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9498 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21114 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21402 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21753 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21981 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24780 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25386 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26517 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27176 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 4816 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 4961 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7204 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7326 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8765 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8930 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20169 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20682 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20803 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23188 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23763 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24455 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24940 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 6608 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6740 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8529 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9049 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9257 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9356 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9735 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18827 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19059 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22336 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23204 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23964 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24793 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5998 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7419 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7781 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8933 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9255 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9549 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9753 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10417 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18898 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22494 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23139 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24764 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25989 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10660 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11298 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12550 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12957 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13322 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13624 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14040 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15004 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15534 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20714 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21789 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23443 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24861 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10522 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11530 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12552 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12963 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13378 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13779 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14245 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15235 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15902 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20102 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22696 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23774 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25838 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 10099 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10691 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12639 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13049 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13386 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13665 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14125 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15163 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15636 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19676 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20474 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23519 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25208 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3144 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5087 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7382 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7504 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7593 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7690 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7801 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8064 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8232 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9248 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9875 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10521 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29048 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_partition_cdf: [[aom_cdf_prob; 11]; 20] = [
    [
        (CDF_PROB_TOP - 19132 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25510 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30392 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 13928 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19855 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28540 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 12522 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23679 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28629 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 9896 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18783 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25853 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 15597 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20929 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24571 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26706 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27664 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28821 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29601 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30571 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31902 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7925 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11043 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16785 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22470 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23971 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25043 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26651 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28701 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29834 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5414 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13269 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15111 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20488 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22360 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24500 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25537 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32117 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2662 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6362 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8614 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20860 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23053 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24778 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26436 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27829 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31171 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18462 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20920 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23124 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27647 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28227 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29049 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29519 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30178 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31544 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7689 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9060 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12056 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24992 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25660 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26182 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26951 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28041 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29052 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 6015 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9009 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10062 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24544 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25409 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26545 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27071 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27526 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32047 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1394 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2208 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2796 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28614 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29061 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29466 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29840 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30185 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31899 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20137 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21547 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23078 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29566 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29837 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30261 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30524 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30892 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31724 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 6732 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7490 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9497 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27944 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28250 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28515 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28969 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29630 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30104 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5945 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7663 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8348 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28683 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29117 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29749 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30064 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30298 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32238 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 870 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1212 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1487 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31198 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31394 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31574 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31743 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31881 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32332 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27899 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28219 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28529 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32484 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32539 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32619 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32639 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 6607 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6990 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8268 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32060 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32219 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32338 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32371 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 5429 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6676 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7122 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32027 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32227 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32531 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32582 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
    ],
    [
        (CDF_PROB_TOP - 711 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 966 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1172 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32448 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32538 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32617 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32664 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
        0,
        0,
    ],
];
static mut default_intra_ext_tx_cdf: [[[[aom_cdf_prob; 17]; 13]; 4]; 3] = [
    [
        [
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
    ],
    [
        [
            [
                (CDF_PROB_TOP - 1535 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8035 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9461 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12751 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23467 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27825 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 564 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3335 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9709 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10870 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18143 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28094 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 672 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3247 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3676 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11982 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19415 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23127 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 5279 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13885 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15487 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18044 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23527 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 30252 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4423 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6074 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7985 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10416 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 25693 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 29298 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1486 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4241 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9460 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10662 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16456 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27694 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 439 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 2838 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3522 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6737 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18058 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23754 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1190 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4233 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4855 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11670 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 20281 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 24377 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1045 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4312 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8647 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10159 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18644 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 29335 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 202 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3734 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4747 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7298 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 17127 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 24016 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 447 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4312 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6819 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8884 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16010 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23858 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 277 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4369 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5255 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8905 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16465 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 22271 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 3409 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5436 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10599 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15599 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19687 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 24040 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 1870 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13742 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14530 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16498 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23770 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27698 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 326 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8796 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14632 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15079 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19272 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27486 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 484 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7576 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7712 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14443 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19159 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 22591 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1126 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15340 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15895 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 17023 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 20896 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 30279 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 655 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4854 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5249 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5913 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 22099 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27138 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1299 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6458 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8885 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9290 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14851 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 25497 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 311 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5295 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5552 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6885 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 22672 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 883 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8059 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8270 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11258 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 17289 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 21549 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 741 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7580 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9318 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10345 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16688 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 29046 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 110 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7915 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9195 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16041 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23329 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 363 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7974 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9357 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 10673 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15629 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 24474 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 153 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 7647 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 8112 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9936 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15307 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19996 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 3511 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6332 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11165 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15335 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19323 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23594 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 4681 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 9362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14043 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23406 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 28087 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
    ],
    [
        [
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 1127 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12814 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 22772 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27483 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 145 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6761 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11980 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26667 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 362 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5887 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11678 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 16725 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 385 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 15213 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18587 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 30693 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 25 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 2914 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23134 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 27903 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 60 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 4470 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11749 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23991 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 37 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 3332 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 14511 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 21448 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 157 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6320 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13036 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 17439 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 119 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6719 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12906 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 29396 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 47 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5537 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12576 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 21499 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 269 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 6076 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 11258 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 23115 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 83 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5615 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12001 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 17228 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 1968 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 5556 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 12023 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 18547 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
        [
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                (CDF_PROB_TOP - 6554 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 13107 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 19661 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - 26214 as core::ffi::c_int) as aom_cdf_prob,
                (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                    as aom_cdf_prob,
                0 as core::ffi::c_int as aom_cdf_prob,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ],
    ],
];
static mut default_inter_ext_tx_cdf: [[[aom_cdf_prob; 17]; 4]; 4] = [
    [
        [
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 4458 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5560 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7695 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 9709 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13330 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14789 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17537 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20266 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21504 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22848 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23934 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25474 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27727 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28915 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30631 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1645 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 2573 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 4778 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5711 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 7807 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8622 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10522 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15357 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17674 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20408 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22517 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25010 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27116 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28856 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30749 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 2731 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5461 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13653 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19115 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27307 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30037 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 2731 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5461 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13653 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19115 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27307 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30037 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 770 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 2421 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5225 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 12907 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15819 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18927 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21561 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24089 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26595 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28526 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30529 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 2731 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 5461 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 13653 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19115 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27307 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30037 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 4167 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 1998 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 748 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
];
static mut default_cfl_sign_cdf: [aom_cdf_prob; 9] = [
    (CDF_PROB_TOP - 1418 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 2123 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 13340 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 18405 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 26972 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 28343 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 32294 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_cfl_alpha_cdf: [[aom_cdf_prob; 17]; 6] = [
    [
        (CDF_PROB_TOP - 7637 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20719 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31401 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32481 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32657 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32688 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32692 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32696 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32700 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32704 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32708 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32712 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32716 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32724 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14365 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23603 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28135 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31168 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32167 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32395 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32487 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32573 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32620 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32647 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32668 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32676 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32680 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32684 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11532 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22380 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28445 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31360 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32349 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32523 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32584 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32649 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32673 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32681 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32685 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32689 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32693 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32697 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26990 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31402 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32282 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32571 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32692 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32696 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32700 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32704 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32708 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32712 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32716 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32724 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32728 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32732 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 17248 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26058 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28904 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30608 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31305 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31877 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32126 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32321 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32394 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32464 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32516 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32560 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32593 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32622 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14738 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21678 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25779 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27901 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29024 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30302 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30980 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31843 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32413 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32520 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32594 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32622 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32656 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32660 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_switchable_interp_cdf: [[aom_cdf_prob; 4]; 16] = [
    [
        (CDF_PROB_TOP - 31935 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5568 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32719 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 422 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2938 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28244 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32608 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 31206 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31953 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4862 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32121 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 770 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1152 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20889 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25637 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 31910 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32724 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32712 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 305 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2247 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27403 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32636 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 31022 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32009 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2963 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32093 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 601 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 943 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14969 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21398 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_newmv_cdf: [[aom_cdf_prob; 3]; 6] = [
    [
        (CDF_PROB_TOP - 24035 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16630 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 15339 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 8386 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12222 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4676 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_zeromv_cdf: [[aom_cdf_prob; 3]; 2] = [
    [
        (CDF_PROB_TOP - 2175 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1054 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_refmv_cdf: [[aom_cdf_prob; 3]; 6] = [
    [
        (CDF_PROB_TOP - 23974 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24188 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 17848 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28622 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24312 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 19923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_drl_cdf: [[aom_cdf_prob; 3]; 3] = [
    [
        (CDF_PROB_TOP - 13104 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24560 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18945 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_inter_compound_mode_cdf: [[aom_cdf_prob; 9]; 8] = [
    [
        (CDF_PROB_TOP - 7760 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13823 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15808 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17641 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19156 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20666 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26891 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10730 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19452 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21145 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22749 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24039 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25131 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28724 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10664 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20221 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21588 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22906 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24295 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25387 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28436 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 13298 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16984 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20471 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24182 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25067 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25736 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26422 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18904 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23325 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25242 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27898 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28258 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30758 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10725 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17454 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20124 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22820 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24195 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25168 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26046 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 17125 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24273 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25814 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27492 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28214 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28704 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30592 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 13046 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23214 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24505 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25942 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27435 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28442 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29330 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_interintra_cdf: [[aom_cdf_prob; 3]; 4] = [
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26887 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27597 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 30237 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_interintra_mode_cdf: [[aom_cdf_prob; 5]; 4] = [
    [
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1875 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11082 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27332 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2473 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9996 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26388 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4238 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11537 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25926 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_wedge_interintra_cdf: [[aom_cdf_prob; 3]; 22] = [
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20036 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24957 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26704 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27530 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 29564 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 29444 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26872 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_compound_type_cdf: [[aom_cdf_prob; 3]; 22] = [
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 23431 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 13171 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11470 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9770 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9100 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 8233 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 6172 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11820 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7701 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_wedge_idx_cdf: [[aom_cdf_prob; 17]; 22] = [
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2438 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4440 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6599 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8663 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11005 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12874 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15751 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18094 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20359 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22362 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24127 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25702 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27752 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29450 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31171 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 806 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 3266 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6005 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6738 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7218 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7367 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7771 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14588 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16323 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17367 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18452 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19422 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22839 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26127 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29629 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2779 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 3738 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4683 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7213 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7775 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8017 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8655 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14357 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17939 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21332 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24520 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27470 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29456 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30529 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31656 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1684 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 3625 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 5675 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7108 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9302 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11274 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14429 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19163 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20961 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22884 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24471 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26719 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28714 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30877 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1142 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 3491 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6277 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7314 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8089 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8355 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9023 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15369 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16730 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18114 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19313 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22521 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26012 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29550 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2742 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4195 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 5727 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8035 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8980 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10146 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14124 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17270 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20533 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23434 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25972 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27944 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29570 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31416 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1727 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 3948 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6101 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7796 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9841 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12344 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15766 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18944 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20638 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22038 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23963 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25311 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26988 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28766 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31012 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 154 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 987 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1925 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2051 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2088 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2111 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2151 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23033 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23703 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24284 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24985 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25684 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27259 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28883 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30911 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1135 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1322 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 1493 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2635 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2696 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2737 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 2770 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21016 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22935 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25057 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27251 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29173 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30089 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30960 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31933 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2048 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 4096 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 6144 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8192 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10240 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12288 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14336 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20480 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26624 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28672 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30720 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_motion_mode_cdf: [[aom_cdf_prob; 4]; 22] = [
    [
        (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7651 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24760 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4738 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24765 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5391 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25528 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 19419 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26810 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5123 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23606 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11606 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24308 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26260 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29116 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20360 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28062 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 21679 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26830 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 29516 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30701 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28898 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30397 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 30878 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31335 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 32507 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32558 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21845 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28799 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31390 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26431 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30774 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28973 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31594 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 29742 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31203 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_obmc_cdf: [[aom_cdf_prob; 3]; 22] = [
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10437 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9371 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9301 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 17432 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14423 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 15142 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 25817 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22823 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22083 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 30128 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 31014 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 31560 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 32638 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 23664 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20901 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24008 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26879 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_intra_inter_cdf: [[aom_cdf_prob; 3]; 4] = [
    [
        (CDF_PROB_TOP - 806 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16662 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20186 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26538 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_comp_inter_cdf: [[aom_cdf_prob; 3]; 5] = [
    [
        (CDF_PROB_TOP - 26828 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24035 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12031 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10640 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2901 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_comp_ref_type_cdf: [[aom_cdf_prob; 3]; 5] = [
    [
        (CDF_PROB_TOP - 1198 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2070 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9166 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7499 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22475 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_uni_comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3] = [
    [
        [
            (CDF_PROB_TOP - 5284 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3865 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3128 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 23152 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 14173 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 15270 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 31774 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 25120 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 26710 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_single_ref_cdf: [[[aom_cdf_prob; 3]; 6]; 3] = [
    [
        [
            (CDF_PROB_TOP - 4897 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1555 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 4236 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 8650 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 904 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1444 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 16973 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 16751 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 19647 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 24773 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 11014 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 15087 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 29744 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 30279 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 31194 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 31895 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 26875 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 30304 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3] = [
    [
        [
            (CDF_PROB_TOP - 4946 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 9468 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1503 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 19891 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 22441 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 15160 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 30731 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 31059 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 27544 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_comp_bwdref_cdf: [[[aom_cdf_prob; 3]; 2]; 3] = [
    [
        [
            (CDF_PROB_TOP - 2235 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1423 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 17182 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 15175 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 30606 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 30489 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_palette_y_size_cdf: [[aom_cdf_prob; 8]; 7] = [
    [
        (CDF_PROB_TOP - 7952 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13000 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18149 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21478 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25527 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29241 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7139 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11421 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16195 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19544 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23666 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28073 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7788 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12741 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17325 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20500 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24315 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28530 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 8271 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14064 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18246 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21564 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25071 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28533 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12725 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19180 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21863 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24839 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27535 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9711 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14888 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16923 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21052 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25661 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27875 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14940 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 20797 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21678 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24186 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27033 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28999 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_palette_uv_size_cdf: [[aom_cdf_prob; 8]; 7] = [
    [
        (CDF_PROB_TOP - 8713 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 19979 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27128 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29609 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31331 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32272 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5839 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 15573 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 23581 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 26947 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29848 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 31700 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4426 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 11260 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17999 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21483 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25863 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29430 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 3228 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 9464 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 14993 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18089 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22523 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27420 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 3768 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8886 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 13091 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 17852 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22495 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27207 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 2464 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 8451 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 12861 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21632 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25525 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28555 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 1269 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 5435 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 10433 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18963 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 21700 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 25865 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_palette_y_mode_cdf: [[[aom_cdf_prob; 3]; 3]; 7] = [
    [
        [
            (CDF_PROB_TOP - 31676 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3419 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1261 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 31912 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 2859 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 980 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 31823 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3400 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 781 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 32030 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3561 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 904 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 32309 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7337 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1462 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 32265 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 4015 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 1521 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 32450 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 7946 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 129 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_palette_uv_mode_cdf: [[aom_cdf_prob; 3]; 2] = [
    [
        (CDF_PROB_TOP - 32461 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 21488 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_palette_y_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7] = [
    [
        [
            (CDF_PROB_TOP - 28710 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 10553 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 27036 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31603 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 27877 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30490 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 11532 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25697 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 6544 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30234 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 23018 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28072 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31915 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32385 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 25572 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28046 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30045 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 9478 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21590 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27256 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 7248 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26837 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29824 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 19167 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24486 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28349 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31400 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31825 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32250 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 24779 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26955 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28576 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30282 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 8669 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20364 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24073 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28093 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 4255 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27565 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29377 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31067 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 19864 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23674 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26716 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29530 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31646 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31893 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32147 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32426 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 23132 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25407 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26970 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28435 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30073 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 7443 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17242 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20717 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24762 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27982 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 6300 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24862 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26944 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28784 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30671 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 18916 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22895 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25267 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27435 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29652 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31270 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31550 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31808 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32059 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32353 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 23105 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25199 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26464 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27684 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28931 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30318 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 6950 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15447 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18952 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22681 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25567 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28563 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 7560 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23474 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25490 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27203 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28921 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30708 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 18544 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22373 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24457 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26195 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28119 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30045 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 31198 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31451 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31670 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31882 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32123 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32391 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 21689 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23883 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25163 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26352 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27506 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28827 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30195 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 6892 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15385 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17840 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21606 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24287 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26753 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29204 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5651 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23182 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25042 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26518 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27982 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29392 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30900 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 19349 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22578 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24418 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25994 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27524 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29031 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30448 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 31028 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31270 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31504 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31705 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31927 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32153 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32392 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_palette_uv_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7] = [
    [
        [
            (CDF_PROB_TOP - 29089 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 8713 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 29257 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31610 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 25257 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29145 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 12287 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27293 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 7033 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27960 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 20145 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25405 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 30608 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31639 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 24210 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27175 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29903 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 9888 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22386 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27214 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 5901 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26053 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29293 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 18318 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22152 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28333 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 30459 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31136 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31926 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 22980 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25479 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27781 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29986 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 8413 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21408 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24859 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28874 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 2257 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29449 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30594 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31598 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 19189 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21202 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25915 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28620 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 31844 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32044 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32281 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32518 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 22217 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24567 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26637 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28683 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30548 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 7307 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 16406 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 19636 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24632 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28424 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 4441 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25064 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26879 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28942 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30919 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 17210 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20528 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23319 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26750 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29582 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
        [
            (CDF_PROB_TOP - 30674 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30953 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31396 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31735 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32207 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 21239 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23168 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25044 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26962 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28705 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30506 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 6545 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15012 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18004 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21817 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25503 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28701 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 3448 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26295 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27437 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28704 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30126 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31442 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 15889 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 18323 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 21704 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24698 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26976 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29690 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 30988 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31204 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31479 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31734 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31983 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32325 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 21442 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23288 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 24758 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26246 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27649 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28980 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30563 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5863 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 14933 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 17552 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20668 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23683 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26411 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29273 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 3415 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25810 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26877 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 27990 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 29223 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30394 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31618 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 17965 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 20084 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22232 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 23974 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 26274 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 28402 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30390 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 31190 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31329 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31516 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31679 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 31825 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32026 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 32322 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
static mut default_txfm_partition_cdf: [[aom_cdf_prob; 3]; 21] = [
    [
        (CDF_PROB_TOP - 28581 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 23846 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20847 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24315 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18196 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12133 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18791 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10887 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11005 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27179 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20004 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 11281 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 26549 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 19308 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14224 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28015 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 21546 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14400 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28165 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22401 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16088 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_skip_txfm_cdfs: [[aom_cdf_prob; 3]; 3] = [
    [
        (CDF_PROB_TOP - 31671 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16515 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4576 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_skip_mode_cdfs: [[aom_cdf_prob; 3]; 3] = [
    [
        (CDF_PROB_TOP - 32621 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20708 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 8127 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_compound_idx_cdfs: [[aom_cdf_prob; 3]; 6] = [
    [
        (CDF_PROB_TOP - 18244 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12865 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7053 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 13259 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9334 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 4644 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_comp_group_idx_cdfs: [[aom_cdf_prob; 3]; 6] = [
    [
        (CDF_PROB_TOP - 26607 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22891 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18840 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 24594 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 19934 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22674 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_intrabc_cdf: [aom_cdf_prob; 3] = [
    (CDF_PROB_TOP - 30531 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_filter_intra_mode_cdf: [aom_cdf_prob; 6] = [
    (CDF_PROB_TOP - 8949 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 12776 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 17211 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 29558 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_filter_intra_cdfs: [[aom_cdf_prob; 3]; 22] = [
    [
        (CDF_PROB_TOP - 4621 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 6743 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 5893 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 7866 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12551 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 9394 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12408 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14301 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12756 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 22343 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 12770 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 10368 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 20229 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 18101 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 16384 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_switchable_restore_cdf: [aom_cdf_prob; 4] = [
    (CDF_PROB_TOP - 9413 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 22581 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_wiener_restore_cdf: [aom_cdf_prob; 3] = [
    (CDF_PROB_TOP - 11570 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_sgrproj_restore_cdf: [aom_cdf_prob; 3] = [
    (CDF_PROB_TOP - 16855 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_delta_q_cdf: [aom_cdf_prob; 5] = [
    (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_delta_lf_multi_cdf: [[aom_cdf_prob; 5]; 4] = [
    [
        (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_delta_lf_cdf: [aom_cdf_prob; 5] = [
    (CDF_PROB_TOP - 28160 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 32120 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - 32677 as core::ffi::c_int) as aom_cdf_prob,
    (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int)) as aom_cdf_prob,
    0 as core::ffi::c_int as aom_cdf_prob,
];
static mut default_segment_pred_cdf: [[aom_cdf_prob; 3]; 3] = [
    [
        (CDF_PROB_TOP - 128 as core::ffi::c_int * 128 as core::ffi::c_int)
            as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 128 as core::ffi::c_int * 128 as core::ffi::c_int)
            as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 128 as core::ffi::c_int * 128 as core::ffi::c_int)
            as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_spatial_pred_seg_tree_cdf: [[aom_cdf_prob; 9]; 3] = [
    [
        (CDF_PROB_TOP - 5622 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 7893 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 16093 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18233 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 27809 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28373 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32533 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 14274 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 18230 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 22557 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 24935 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 29980 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 30851 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32344 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
    [
        (CDF_PROB_TOP - 27527 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28487 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28723 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 28890 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32397 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32647 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - 32679 as core::ffi::c_int) as aom_cdf_prob,
        (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
            as aom_cdf_prob,
        0 as core::ffi::c_int as aom_cdf_prob,
    ],
];
static mut default_tx_size_cdf: [[[aom_cdf_prob; 4]; 3]; 4] = [
    [
        [
            (CDF_PROB_TOP - 19968 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 19968 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
        [
            (CDF_PROB_TOP - 24320 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
            0,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 12272 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30172 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 12272 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30172 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 18677 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 30848 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 12986 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15180 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 12986 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 15180 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 24302 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 25602 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
    [
        [
            (CDF_PROB_TOP - 5782 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11475 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 5782 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 11475 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
        [
            (CDF_PROB_TOP - 16803 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - 22759 as core::ffi::c_int) as aom_cdf_prob,
            (CDF_PROB_TOP - ((1 as core::ffi::c_int) << 15 as core::ffi::c_int))
                as aom_cdf_prob,
            0 as core::ffi::c_int as aom_cdf_prob,
        ],
    ],
];
#[no_mangle]
pub static mut av1_palette_color_index_context_lookup: [core::ffi::c_int; 9] = [
    -(1 as core::ffi::c_int),
    -(1 as core::ffi::c_int),
    0 as core::ffi::c_int,
    -(1 as core::ffi::c_int),
    -(1 as core::ffi::c_int),
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_palette_color_index_context(
    mut color_map: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut palette_size: core::ffi::c_int,
    mut color_order: *mut uint8_t,
    mut color_idx: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    if !(palette_size <= 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            896 as core::ffi::c_int,
            b"palette_size <= PALETTE_MAX_SIZE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(r > 0 as core::ffi::c_int || c > 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            897 as core::ffi::c_int,
            b"r > 0 || c > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut color_neighbors: [core::ffi::c_int; 3] = [0; 3];
    color_neighbors[0 as core::ffi::c_int as usize] = if c - 1 as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        *color_map.offset((r * stride + c - 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    color_neighbors[1 as core::ffi::c_int as usize] = if c - 1 as core::ffi::c_int
        >= 0 as core::ffi::c_int && r - 1 as core::ffi::c_int >= 0 as core::ffi::c_int
    {
        *color_map
            .offset(
                ((r - 1 as core::ffi::c_int) * stride + c - 1 as core::ffi::c_int)
                    as isize,
            ) as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    color_neighbors[2 as core::ffi::c_int as usize] = if r - 1 as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        *color_map.offset(((r - 1 as core::ffi::c_int) * stride + c) as isize)
            as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    let mut scores: [core::ffi::c_int; 18] = [0 as core::ffi::c_int; 18];
    let mut i: core::ffi::c_int = 0;
    static mut weights: [core::ffi::c_int; 3] = [
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
    ];
    i = 0 as core::ffi::c_int;
    while i < NUM_PALETTE_NEIGHBORS {
        if color_neighbors[i as usize] >= 0 as core::ffi::c_int {
            scores[color_neighbors[i as usize] as usize] += weights[i as usize];
        }
        i += 1;
    }
    let mut inverse_color_order: [core::ffi::c_int; 8] = [0; 8];
    i = 0 as core::ffi::c_int;
    while i < PALETTE_MAX_SIZE {
        *color_order.offset(i as isize) = i as uint8_t;
        inverse_color_order[i as usize] = i;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < NUM_PALETTE_NEIGHBORS {
        let mut max: core::ffi::c_int = scores[i as usize];
        let mut max_idx: core::ffi::c_int = i;
        let mut j: core::ffi::c_int = i + 1 as core::ffi::c_int;
        while j < palette_size {
            if scores[j as usize] > max {
                max = scores[j as usize];
                max_idx = j;
            }
            j += 1;
        }
        if max_idx != i {
            let max_score: core::ffi::c_int = scores[max_idx as usize];
            let max_color_order: uint8_t = *color_order.offset(max_idx as isize);
            let mut k: core::ffi::c_int = max_idx;
            while k > i {
                scores[k as usize] = scores[(k - 1 as core::ffi::c_int) as usize];
                *color_order.offset(k as isize) = *color_order
                    .offset((k - 1 as core::ffi::c_int) as isize);
                inverse_color_order[*color_order.offset(k as isize) as usize] = k;
                k -= 1;
            }
            scores[i as usize] = max_score;
            *color_order.offset(i as isize) = max_color_order;
            inverse_color_order[*color_order.offset(i as isize) as usize] = i;
        }
        i += 1;
    }
    if !color_idx.is_null() {
        *color_idx = inverse_color_order[*color_map.offset((r * stride + c) as isize)
            as usize];
    }
    let mut color_index_ctx_hash: core::ffi::c_int = 0 as core::ffi::c_int;
    static mut hash_multipliers: [core::ffi::c_int; 3] = [
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        2 as core::ffi::c_int,
    ];
    i = 0 as core::ffi::c_int;
    while i < NUM_PALETTE_NEIGHBORS {
        color_index_ctx_hash += scores[i as usize] * hash_multipliers[i as usize];
        i += 1;
    }
    if !(color_index_ctx_hash > 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            959 as core::ffi::c_int,
            b"color_index_ctx_hash > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(color_index_ctx_hash <= 8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            960 as core::ffi::c_int,
            b"color_index_ctx_hash <= MAX_COLOR_CONTEXT_HASH\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let color_index_ctx: core::ffi::c_int = av1_palette_color_index_context_lookup[color_index_ctx_hash
        as usize];
    if !(color_index_ctx >= 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            965 as core::ffi::c_int,
            b"color_index_ctx >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(color_index_ctx < 5 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_get_palette_color_index_context\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            966 as core::ffi::c_int,
            b"color_index_ctx < PALETTE_COLOR_INDEX_CONTEXTS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return color_index_ctx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_mode_probs(mut fc: *mut FRAME_CONTEXT) {
    memcpy(
        ((*fc).palette_y_size_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_y_size_cdf).cast::<[aom_cdf_prob; 8]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 8]; 7]>() as size_t,
    );
    memcpy(
        ((*fc).palette_uv_size_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_uv_size_cdf).cast::<[aom_cdf_prob; 8]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 8]; 7]>() as size_t,
    );
    memcpy(
        ((*fc).palette_y_color_index_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_y_color_index_cdf).cast::<[[aom_cdf_prob; 9]; 5]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 9]; 5]; 7]>() as size_t,
    );
    memcpy(
        ((*fc).palette_uv_color_index_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_uv_color_index_cdf).cast::<[[aom_cdf_prob; 9]; 5]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 9]; 5]; 7]>() as size_t,
    );
    memcpy(
        ((*fc).kf_y_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_kf_y_mode_cdf).cast::<[[aom_cdf_prob; 14]; 5]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 14]; 5]; 5]>() as size_t,
    );
    memcpy(
        ((*fc).angle_delta_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_angle_delta_cdf).cast::<[aom_cdf_prob; 8]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 8]; 8]>() as size_t,
    );
    memcpy(
        ((*fc).comp_inter_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_comp_inter_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 5]>() as size_t,
    );
    memcpy(
        ((*fc).comp_ref_type_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_comp_ref_type_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 5]>() as size_t,
    );
    memcpy(
        ((*fc).uni_comp_ref_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_uni_comp_ref_cdf).cast::<[[aom_cdf_prob; 3]; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 3]; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).palette_y_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_y_mode_cdf).cast::<[[aom_cdf_prob; 3]; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 3]; 3]; 7]>() as size_t,
    );
    memcpy(
        ((*fc).palette_uv_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_palette_uv_mode_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 2]>() as size_t,
    );
    memcpy(
        ((*fc).comp_ref_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_comp_ref_cdf).cast::<[[aom_cdf_prob; 3]; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 3]; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).comp_bwdref_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_comp_bwdref_cdf).cast::<[[aom_cdf_prob; 3]; 2]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 3]; 2]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).single_ref_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_single_ref_cdf).cast::<[[aom_cdf_prob; 3]; 6]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 3]; 6]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).txfm_partition_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_txfm_partition_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 21]>() as size_t,
    );
    memcpy(
        ((*fc).compound_index_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_compound_idx_cdfs).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 6]>() as size_t,
    );
    memcpy(
        ((*fc).comp_group_idx_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_comp_group_idx_cdfs).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 6]>() as size_t,
    );
    memcpy(
        ((*fc).newmv_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_newmv_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 6]>() as size_t,
    );
    memcpy(
        ((*fc).zeromv_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_zeromv_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 2]>() as size_t,
    );
    memcpy(
        ((*fc).refmv_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_refmv_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 6]>() as size_t,
    );
    memcpy(
        ((*fc).drl_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_drl_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).motion_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_motion_mode_cdf).cast::<[aom_cdf_prob; 4]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 4]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).obmc_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_obmc_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).inter_compound_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_inter_compound_mode_cdf).cast::<[aom_cdf_prob; 9]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 9]; 8]>() as size_t,
    );
    memcpy(
        ((*fc).compound_type_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_compound_type_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).wedge_idx_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_wedge_idx_cdf).cast::<[aom_cdf_prob; 17]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 17]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).interintra_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_interintra_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).wedge_interintra_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_wedge_interintra_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).interintra_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_interintra_mode_cdf).cast::<[aom_cdf_prob; 5]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 5]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).seg.pred_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_segment_pred_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).filter_intra_cdfs).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_filter_intra_cdfs).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 22]>() as size_t,
    );
    memcpy(
        ((*fc).filter_intra_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_filter_intra_mode_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 6]>() as size_t,
    );
    memcpy(
        ((*fc).switchable_restore_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_switchable_restore_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 4]>() as size_t,
    );
    memcpy(
        ((*fc).wiener_restore_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_wiener_restore_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 3]>() as size_t,
    );
    memcpy(
        ((*fc).sgrproj_restore_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_sgrproj_restore_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 3]>() as size_t,
    );
    memcpy(
        ((*fc).y_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_if_y_mode_cdf).cast::<[aom_cdf_prob; 14]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 14]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).uv_mode_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_uv_mode_cdf).cast::<[[aom_cdf_prob; 15]; 13]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 15]; 13]; 2]>() as size_t,
    );
    memcpy(
        ((*fc).switchable_interp_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_switchable_interp_cdf).cast::<[aom_cdf_prob; 4]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 4]; 16]>() as size_t,
    );
    memcpy(
        ((*fc).partition_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_partition_cdf).cast::<[aom_cdf_prob; 11]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 11]; 20]>() as size_t,
    );
    memcpy(
        ((*fc).intra_ext_tx_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_intra_ext_tx_cdf).cast::<[[[aom_cdf_prob; 17]; 13]; 4]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[[aom_cdf_prob; 17]; 13]; 4]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).inter_ext_tx_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_inter_ext_tx_cdf).cast::<[[aom_cdf_prob; 17]; 4]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 17]; 4]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).skip_mode_cdfs).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_skip_mode_cdfs).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).skip_txfm_cdfs).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_skip_txfm_cdfs).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 3]>() as size_t,
    );
    memcpy(
        ((*fc).intra_inter_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_intra_inter_cdf).cast::<[aom_cdf_prob; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 3]; 4]>() as size_t,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < SPATIAL_PREDICTION_PROBS {
        memcpy(
            ((*fc).seg.spatial_pred_seg_cdf[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            (default_spatial_pred_seg_tree_cdf[i as usize]).as_ptr()
                as *const core::ffi::c_void,
            ::core::mem::size_of::<[aom_cdf_prob; 9]>() as size_t,
        );
        i += 1;
    }
    memcpy(
        ((*fc).tx_size_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_tx_size_cdf).cast::<[[aom_cdf_prob; 4]; 3]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[[aom_cdf_prob; 4]; 3]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).delta_q_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_delta_q_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 5]>() as size_t,
    );
    memcpy(
        ((*fc).delta_lf_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_delta_lf_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 5]>() as size_t,
    );
    memcpy(
        ((*fc).delta_lf_multi_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_delta_lf_multi_cdf).cast::<[aom_cdf_prob; 5]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 5]; 4]>() as size_t,
    );
    memcpy(
        ((*fc).cfl_sign_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_cfl_sign_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 9]>() as size_t,
    );
    memcpy(
        ((*fc).cfl_alpha_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_cfl_alpha_cdf).cast::<[aom_cdf_prob; 17]>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[[aom_cdf_prob; 17]; 6]>() as size_t,
    );
    memcpy(
        ((*fc).intrabc_cdf).as_mut_ptr() as *mut core::ffi::c_void,
        core::ptr::addr_of!(default_intrabc_cdf).cast::<aom_cdf_prob>() as *const core::ffi::c_void,
        ::core::mem::size_of::<[aom_cdf_prob; 3]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_default_ref_deltas(mut ref_deltas: *mut int8_t) {
    if ref_deltas.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_set_default_ref_deltas\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            1028 as core::ffi::c_int,
            b"ref_deltas != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *ref_deltas.offset(INTRA_FRAME as core::ffi::c_int as isize) = 1 as int8_t;
    *ref_deltas.offset(LAST_FRAME as core::ffi::c_int as isize) = 0 as int8_t;
    *ref_deltas.offset(LAST2_FRAME as core::ffi::c_int as isize) = *ref_deltas
        .offset(LAST_FRAME as core::ffi::c_int as isize);
    *ref_deltas.offset(LAST3_FRAME as core::ffi::c_int as isize) = *ref_deltas
        .offset(LAST_FRAME as core::ffi::c_int as isize);
    *ref_deltas.offset(BWDREF_FRAME as core::ffi::c_int as isize) = *ref_deltas
        .offset(LAST_FRAME as core::ffi::c_int as isize);
    *ref_deltas.offset(GOLDEN_FRAME as core::ffi::c_int as isize) = -(1
        as core::ffi::c_int) as int8_t;
    *ref_deltas.offset(ALTREF2_FRAME as core::ffi::c_int as isize) = -(1
        as core::ffi::c_int) as int8_t;
    *ref_deltas.offset(ALTREF_FRAME as core::ffi::c_int as isize) = -(1
        as core::ffi::c_int) as int8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_default_mode_deltas(mut mode_deltas: *mut int8_t) {
    if mode_deltas.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_set_default_mode_deltas\0"))
                .as_ptr(),
            b"entropymode.c\0" as *const u8 as *const core::ffi::c_char,
            1041 as core::ffi::c_int,
            b"mode_deltas != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    *mode_deltas.offset(0 as core::ffi::c_int as isize) = 0 as int8_t;
    *mode_deltas.offset(1 as core::ffi::c_int as isize) = 0 as int8_t;
}
unsafe extern "C-unwind" fn set_default_lf_deltas(mut lf: *mut loopfilter) {
    (*lf).mode_ref_delta_enabled = 1 as uint8_t;
    (*lf).mode_ref_delta_update = 1 as uint8_t;
    av1_set_default_ref_deltas(((*lf).ref_deltas).as_mut_ptr());
    av1_set_default_mode_deltas(((*lf).mode_deltas).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_frame_contexts(mut cm: *mut AV1_COMMON) {
    *(*cm).default_frame_context = *(*cm).fc;
    if (*cm).tiles.large_scale != 0 {
        let mut i: core::ffi::c_int = LAST_FRAME as core::ffi::c_int;
        while i <= ALTREF_FRAME as core::ffi::c_int {
            let buf: *mut RefCntBuffer = get_ref_frame_buf(cm, i as MV_REFERENCE_FRAME)
                as *mut RefCntBuffer;
            if !buf.is_null() {
                (*buf).frame_context = *(*cm).fc;
            }
            i += 1;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < (*(*cm).buffer_pool).num_frame_bufs as core::ffi::c_int {
            (*((*(*cm).buffer_pool).frame_bufs).offset(i_0 as isize)).frame_context = *(*cm)
                .fc;
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_past_independence(mut cm: *mut AV1_COMMON) {
    av1_clearall_segfeatures(&raw mut (*cm).seg);
    if !((*(*cm).cur_frame).seg_map).is_null() {
        memset(
            (*(*cm).cur_frame).seg_map as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((*(*cm).cur_frame).mi_rows * (*(*cm).cur_frame).mi_cols) as size_t,
        );
    }
    av1_set_default_ref_deltas(((*(*cm).cur_frame).ref_deltas).as_mut_ptr());
    av1_set_default_mode_deltas(((*(*cm).cur_frame).mode_deltas).as_mut_ptr());
    set_default_lf_deltas(&raw mut (*cm).lf);
    av1_default_coef_probs(cm as *mut AV1Common);
    av1_init_mode_probs((*cm).fc);
    av1_init_mv_probs(cm as *mut AV1Common);
    (*(*cm).fc).initialized = 1 as core::ffi::c_int;
    av1_setup_frame_contexts(cm);
}

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
