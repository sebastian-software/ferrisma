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

// Mechanically translated from upstream/aom/av1/decoder/decoder.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_img_metadata_array_free(arr: *mut aom_metadata_array_t);
    fn aom_free_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG) -> core::ffi::c_int;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_qm_init(quant_params: *mut CommonQuantParams, num_planes: core::ffi::c_int);
    fn aom_invalidate_pyramid(pyr: *mut ImagePyramid);
    fn av1_invalidate_corner_list(corners: *mut CornerList);
    fn av1_loop_restoration_precal();
    fn av1_rtcd();
    fn aom_dsp_rtcd();
    fn aom_yv12_copy_frame_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        num_planes: core::ffi::c_int,
    );
    fn aom_scale_rtcd();
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
    fn av1_remove_common(cm: *mut AV1Common);
    fn aom_set_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_loop_filter_init(cm: *mut AV1Common);
    fn av1_init_wedge_masks();
    fn av1_init_intra_predictors();
    fn av1_free_mc_tmp_buf(thread_data: *mut ThreadData);
    fn av1_loop_filter_dealloc(lf_sync: *mut AV1LfSync);
    fn av1_loop_restoration_dealloc(lr_sync: *mut AV1LrSync);
    fn av1_dec_row_mt_dealloc(dec_row_mt_sync: *mut AV1DecRowMTSync);
    fn aom_decode_frame_from_obus(
        pbi: *mut AV1Decoder,
        data: *const uint8_t,
        data_end: *const uint8_t,
        p_data_end: *mut *const uint8_t,
    ) -> core::ffi::c_int;
}
pub type __darwin_size_t = usize;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
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
pub type PLANE_TYPE = uint8_t;
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
pub type ImagePyramid = image_pyramid;
pub type CornerList = corner_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CB_BUFFER {
    pub __aom_align_dqcoeff: [crate::platform::Align32; 0],

    pub dqcoeff: [[tran_low_t; 16384]; 3],
    pub eob_data: [[eob_info; 1024]; 3],
    pub __aom_align_color_index_map: [crate::platform::Align16; 0],
    pub color_index_map: [[uint8_t; 16384]; 2],
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
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorkerInterface {
    pub init: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub reset: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub sync: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub launch: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub execute: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub end: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
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
pub struct AV1Decoder {
    pub dcb: DecoderCodingBlock,
    pub __aom_align_common: [crate::platform::Align32; 0],
    pub common: AV1_COMMON,
    pub lf_worker: AVxWorker,
    pub lf_row_sync: AV1LfSync,
    pub lr_row_sync: AV1LrSync,
    pub lr_ctxt: AV1LrStruct,
    pub cdef_sync: AV1CdefSync,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub tile_workers: *mut AVxWorker,
    pub num_workers: core::ffi::c_int,
    pub thread_data: *mut DecWorkerData,
    pub td: ThreadData,
    pub tile_data: *mut TileDataDec,
    pub allocated_tiles: core::ffi::c_int,
    pub tile_buffers: [[TileBufferDec; 64]; 64],
    pub tile_mt_info: AV1DecTileMT,
    pub output_all_layers: core::ffi::c_int,
    pub output_frames: [*mut RefCntBuffer; 4],
    pub num_output_frames: size_t,
    pub decoding_first_frame: core::ffi::c_int,
    pub allow_lowbitdepth: core::ffi::c_int,
    pub max_threads: core::ffi::c_int,
    pub inv_tile_order: core::ffi::c_int,
    pub need_resync: core::ffi::c_int,
    pub reset_decoder_state: core::ffi::c_int,
    pub tile_size_bytes: core::ffi::c_int,
    pub tile_col_size_bytes: core::ffi::c_int,
    pub dec_tile_row: core::ffi::c_int,
    pub dec_tile_col: core::ffi::c_int,
    pub sequence_header_ready: core::ffi::c_int,
    pub sequence_header_changed: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub current_operating_point: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub seen_frame_header: core::ffi::c_int,
    pub next_start_tile: core::ffi::c_int,
    pub camera_frame_header_ready: core::ffi::c_int,
    pub frame_header_size: size_t,
    pub obu_size_hdr: DataBuffer,
    pub output_frame_width_in_tiles_minus_1: core::ffi::c_int,
    pub output_frame_height_in_tiles_minus_1: core::ffi::c_int,
    pub tile_count_minus_1: core::ffi::c_int,
    pub coded_tile_data_size: uint32_t,
    pub ext_tile_debug: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub ext_refs: EXTERNAL_REFERENCES,
    pub tile_list_outbuf: YV12_BUFFER_CONFIG,
    pub cb_buffer_base: *mut CB_BUFFER,
    pub cb_buffer_alloc_size: core::ffi::c_int,
    pub allocated_row_mt_sync_rows: core::ffi::c_int,
    pub row_mt_mutex_: *mut pthread_mutex_t,
    pub row_mt_cond_: *mut pthread_cond_t,
    pub frame_row_mt_info: AV1DecRowMTInfo,
    pub metadata: *mut aom_metadata_array_t,
    pub context_update_tile_id: core::ffi::c_int,
    pub skip_loop_filter: core::ffi::c_int,
    pub skip_film_grain: core::ffi::c_int,
    pub is_annexb: core::ffi::c_int,
    pub valid_for_referencing: [core::ffi::c_int; 8],
    pub is_fwd_kf_present: core::ffi::c_int,
    pub is_arf_frame_present: core::ffi::c_int,
    pub num_tile_groups: core::ffi::c_int,
    pub sframe_info: aom_s_frame_info,
    pub seq_params: SequenceHeader,
    pub buffer_removal_time_present: bool,
    pub error: aom_internal_error_info,
    pub number_temporal_layers: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_s_frame_info {
    pub is_s_frame: core::ffi::c_int,
    pub is_s_frame_at_altref: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTInfo {
    pub tile_rows_start: core::ffi::c_int,
    pub tile_rows_end: core::ffi::c_int,
    pub tile_cols_start: core::ffi::c_int,
    pub tile_cols_end: core::ffi::c_int,
    pub start_tile: core::ffi::c_int,
    pub end_tile: core::ffi::c_int,
    pub mi_rows_to_decode: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub row_mt_exit: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXTERNAL_REFERENCES {
    pub refs: [YV12_BUFFER_CONFIG; 128],
    pub num: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DataBuffer {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type AV1DecTileMT = AV1DecTileMTData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecTileMTData {
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut TileJobsDec,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub alloc_tile_rows: core::ffi::c_int,
    pub alloc_tile_cols: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileJobsDec {
    pub tile_buffer: *mut TileBufferDec,
    pub tile_data: *mut TileDataDec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileDataDec {
    pub tile_info: TileInfo,
    pub bit_reader: aom_reader,
    pub __aom_align_tctx: [crate::platform::Align16; 0],
    pub tctx: FRAME_CONTEXT,
    pub dec_row_mt_sync: AV1DecRowMTSync,
}
pub type AV1DecRowMTSync = AV1DecRowMTSyncData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub allocated_sb_rows: core::ffi::c_int,
    pub cur_sb_col: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub intrabc_extra_top_right_sb_delay: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_dec {
    pub buf: *const core::ffi::c_uchar,
    pub tell_offs: int32_t,
    pub end: *const core::ffi::c_uchar,
    pub bptr: *const core::ffi::c_uchar,
    pub dif: od_ec_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
}
pub type od_ec_window = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileBufferDec {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadData {
    pub dcb: DecoderCodingBlock,
    pub cb_buffer_base: CB_BUFFER,
    pub bit_reader: *mut aom_reader,
    pub mc_buf: [*mut uint8_t; 2],
    pub seg_mask: *mut uint8_t,
    pub mc_buf_size: int32_t,
    pub mc_buf_use_highbd: core::ffi::c_int,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub read_coeffs_tx_intra_block_visit: decode_block_visitor_fn_t,
    pub predict_and_recon_intra_block_visit: decode_block_visitor_fn_t,
    pub read_coeffs_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub inverse_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub predict_inter_block_visit: predict_inter_block_visitor_fn_t,
    pub cfl_store_inter_block_visit: cfl_store_inter_block_visitor_fn_t,
}
pub type cfl_store_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
>;
pub type predict_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut DecoderCodingBlock, BLOCK_SIZE) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderCodingBlock {
    pub __aom_align_xd: [crate::platform::Align32; 0],

    pub xd: MACROBLOCKD,
    pub corrupted: core::ffi::c_int,
    pub mc_buf: [*mut uint8_t; 2],
    pub dqcoeff_block: [*mut tran_low_t; 3],
    pub cb_offset: [uint16_t; 3],
    pub eob_data: [*mut eob_info; 3],
    pub txb_offset: [uint16_t; 3],
    pub ref_mv_count: [uint8_t; 29],
}
pub type decode_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *mut DecoderCodingBlock,
        *mut aom_reader,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        TX_SIZE,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecWorkerData {
    pub td: *mut ThreadData,
    pub data_end: *const uint8_t,
    pub error_info: aom_internal_error_info,
}
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1LfSync = AV1LfSyncData;
pub type palette_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut MACROBLOCKD, core::ffi::c_int, *mut aom_reader) -> (),
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MAX_NUM_SPATIAL_LAYERS: core::ffi::c_int = 4 as core::ffi::c_int;
unsafe extern "C-unwind" fn lock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_lock(&raw mut (*pool).pool_mutex);
}
unsafe extern "C-unwind" fn unlock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_unlock(&raw mut (*pool).pool_mutex);
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame(
    mut cm: *mut AV1_COMMON,
    mut index: core::ffi::c_int,
) -> *mut YV12_BUFFER_CONFIG {
    if index < 0 as core::ffi::c_int || index >= REF_FRAMES as core::ffi::c_int {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    if ((*cm).ref_frame_map[index as usize]).is_null() {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    return &raw mut (**((*cm).ref_frame_map).as_mut_ptr().offset(index as isize)).buf;
}
#[inline]
unsafe extern "C-unwind" fn get_free_fb(mut cm: *mut AV1_COMMON) -> core::ffi::c_int {
    let frame_bufs: *mut RefCntBuffer = (*(*cm).buffer_pool).frame_bufs;
    let mut i: core::ffi::c_int = 0;
    lock_buffer_pool((*cm).buffer_pool);
    let num_frame_bufs: core::ffi::c_int = (*(*cm).buffer_pool).num_frame_bufs
        as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < num_frame_bufs {
        if (*frame_bufs.offset(i as isize)).ref_count == 0 as core::ffi::c_int {
            break;
        }
        i += 1;
    }
    if i != num_frame_bufs {
        if (*frame_bufs.offset(i as isize)).buf.use_external_reference_buffers != 0 {
            let mut ybf: *mut YV12_BUFFER_CONFIG = &raw mut (*frame_bufs.offset(i as isize))
                .buf;
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.y_buffer = (*ybf)
                .store_buf_adr[0 as core::ffi::c_int as usize];
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.u_buffer = (*ybf)
                .store_buf_adr[1 as core::ffi::c_int as usize];
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.v_buffer = (*ybf)
                .store_buf_adr[2 as core::ffi::c_int as usize];
            (*ybf).use_external_reference_buffers = 0 as core::ffi::c_int;
        }
        (*frame_bufs.offset(i as isize)).ref_count = 1 as core::ffi::c_int;
    } else {
        if !(0 as core::ffi::c_int != 0
            && !(b"Ran out of free frame buffers. Likely a reference leak.\0"
                as *const u8 as *const core::ffi::c_char)
                .is_null()) as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"get_free_fb\0"))
                    .as_ptr(),
                b"av1_common_int.h\0" as *const u8 as *const core::ffi::c_char,
                1131 as core::ffi::c_int,
                b"0 && \"Ran out of free frame buffers. Likely a reference leak.\"\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        i = INVALID_IDX;
    }
    unlock_buffer_pool((*cm).buffer_pool);
    return i;
}
#[inline]
unsafe extern "C-unwind" fn assign_cur_frame_new_fb(cm: *mut AV1_COMMON) -> *mut RefCntBuffer {
    if !((*cm).cur_frame).is_null() {
        (*(*cm).cur_frame).ref_count -= 1;
        (*cm).cur_frame = 0 as *mut RefCntBuffer;
    }
    let new_fb_idx: core::ffi::c_int = get_free_fb(cm) as core::ffi::c_int;
    if new_fb_idx == INVALID_IDX {
        return 0 as *mut RefCntBuffer;
    }
    (*cm).cur_frame = &raw mut *((*(*cm).buffer_pool).frame_bufs).offset(new_fb_idx as isize)
        as *mut RefCntBuffer;
    aom_invalidate_pyramid((*(*cm).cur_frame).buf.y_pyramid as *mut ImagePyramid);
    av1_invalidate_corner_list((*(*cm).cur_frame).buf.corners as *mut CornerList);
    memset(
        &raw mut (*(*cm).cur_frame).interp_filter_selected as *mut [core::ffi::c_int; 4]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_int; 4]>() as size_t,
    );
    return (*cm).cur_frame;
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
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
#[inline]
unsafe extern "C-unwind" fn calc_mi_size(mut len: core::ffi::c_int) -> core::ffi::c_int {
    return len
        + (((1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int);
}
#[inline]
unsafe extern "C-unwind" fn decrease_ref_count(buf: *mut RefCntBuffer, pool: *mut BufferPool) {
    if !buf.is_null() {
        (*buf).ref_count -= 1;
        if !((*buf).ref_count >= 0 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"decrease_ref_count\0"))
                    .as_ptr(),
                b"decoder.h\0" as *const u8 as *const core::ffi::c_char,
                473 as core::ffi::c_int,
                b"buf->ref_count >= 0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (*buf).ref_count == 0 as core::ffi::c_int
            && !((*buf).raw_frame_buffer.data).is_null()
        {
            ((*pool).release_fb_cb)
                .expect(
                    "non-null function pointer",
                )((*pool).cb_priv, &raw mut (*buf).raw_frame_buffer);
            (*buf).raw_frame_buffer.data = 0 as *mut uint8_t;
            (*buf).raw_frame_buffer.size = 0 as size_t;
            (*buf).raw_frame_buffer.priv_0 = NULL;
        }
    }
}
unsafe extern "C-unwind" fn initialize_dec() {
    av1_rtcd();
    aom_dsp_rtcd();
    aom_scale_rtcd();
    av1_init_intra_predictors();
    av1_init_wedge_masks();
}
unsafe extern "C-unwind" fn dec_set_mb_mi(
    mut mi_params: *mut CommonModeInfoParams,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut min_partition_size: BLOCK_SIZE,
) {
    let aligned_width: core::ffi::c_int = width
        + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
    let aligned_height: core::ffi::c_int = height
        + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
    (*mi_params).mi_cols = aligned_width >> MI_SIZE_LOG2;
    (*mi_params).mi_rows = aligned_height >> MI_SIZE_LOG2;
    (*mi_params).mi_stride = calc_mi_size((*mi_params).mi_cols);
    (*mi_params).mb_cols = (*mi_params).mi_cols
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 2 as core::ffi::c_int;
    (*mi_params).mb_rows = (*mi_params).mi_rows
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
        >> 2 as core::ffi::c_int;
    (*mi_params).MBs = (*mi_params).mb_rows * (*mi_params).mb_cols;
    (*mi_params).mi_alloc_bsize = BLOCK_4X4;
    (*mi_params).mi_alloc_stride = (*mi_params).mi_stride;
    if !(mi_size_wide[(*mi_params).mi_alloc_bsize as usize] as core::ffi::c_int
        == mi_size_high[(*mi_params).mi_alloc_bsize as usize] as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"dec_set_mb_mi\0"))
                .as_ptr(),
            b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
            69 as core::ffi::c_int,
            b"mi_size_wide[mi_params->mi_alloc_bsize] == mi_size_high[mi_params->mi_alloc_bsize]\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
}
unsafe extern "C-unwind" fn dec_setup_mi(mut mi_params: *mut CommonModeInfoParams) {
    let mi_grid_size: core::ffi::c_int = (*mi_params).mi_stride
        * calc_mi_size((*mi_params).mi_rows) as core::ffi::c_int;
    memset(
        (*mi_params).mi_grid_base as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (mi_grid_size as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut MB_MODE_INFO>() as size_t),
    );
}
unsafe extern "C-unwind" fn dec_free_mi(mut mi_params: *mut CommonModeInfoParams) {
    aom_free((*mi_params).mi_alloc as *mut core::ffi::c_void);
    (*mi_params).mi_alloc = 0 as *mut MB_MODE_INFO;
    (*mi_params).mi_alloc_size = 0 as core::ffi::c_int;
    aom_free((*mi_params).mi_grid_base as *mut core::ffi::c_void);
    (*mi_params).mi_grid_base = 0 as *mut *mut MB_MODE_INFO;
    (*mi_params).mi_grid_size = 0 as core::ffi::c_int;
    aom_free((*mi_params).tx_type_map as *mut core::ffi::c_void);
    (*mi_params).tx_type_map = 0 as *mut TX_TYPE;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_decoder_create(pool: *mut BufferPool) -> *mut AV1Decoder {
    let pbi: *mut AV1Decoder = aom_memalign(
        32 as size_t,
        ::core::mem::size_of::<AV1Decoder>() as size_t,
    ) as *mut AV1Decoder;
    if pbi.is_null() {
        return 0 as *mut AV1Decoder;
    }
    memset(
        pbi as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AV1Decoder>() as size_t,
    );
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    (*cm).seq_params = &raw mut (*pbi).seq_params;
    (*cm).error = &raw mut (*pbi).error;
    loop {
match crate::jump::value_scope((((*pbi).error.jmp).as_mut_ptr()).cast(), || {
    (*pbi).error.setjmp = 1 as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_decoder_create\0"))
                .as_ptr(),
            b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
            111 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*cm).fc = aom_memalign(
        32 as size_t,
        ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
    ) as *mut FRAME_CONTEXT;
    if ((*cm).fc).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate cm->fc at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decoder.c\0" as *const u8
                as *const core::ffi::c_char,
            111 as core::ffi::c_int,
        );
    }
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"av1_decoder_create\0"))
                .as_ptr(),
            b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
            114 as core::ffi::c_int,
            b"((cm)->error)->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*cm).default_frame_context = aom_memalign(
        32 as size_t,
        ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
    ) as *mut FRAME_CONTEXT;
    if ((*cm).default_frame_context).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate cm->default_frame_context at %s:%d\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/decoder.c\0" as *const u8
                as *const core::ffi::c_char,
            114 as core::ffi::c_int,
        );
    }
    memset(
        (*cm).fc as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
    );
    memset(
        (*cm).default_frame_context as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<FRAME_CONTEXT>() as size_t,
    );
    (*pbi).need_resync = 1 as core::ffi::c_int;
    initialize_dec();
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < REF_FRAMES as core::ffi::c_int {
        (*cm).ref_frame_map[i as usize] = 0 as *mut RefCntBuffer;
        i += 1;
    }
    (*cm).current_frame.frame_number = 0 as core::ffi::c_uint;
    (*pbi).decoding_first_frame = 1 as core::ffi::c_int;
    (*pbi).common.buffer_pool = pool;
    (*(*cm).seq_params).bit_depth = AOM_BITS_8;
    (*cm).mi_params.free_mi = Some(
        dec_free_mi as unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> (),
    ) as Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>;
    (*cm).mi_params.setup_mi = Some(
        dec_setup_mi as unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> (),
    ) as Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>;
    (*cm).mi_params.set_mb_mi = Some(
        dec_set_mb_mi
            as unsafe extern "C-unwind" fn(
                *mut CommonModeInfoParams,
                core::ffi::c_int,
                core::ffi::c_int,
                BLOCK_SIZE,
            ) -> (),
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *mut CommonModeInfoParams,
                core::ffi::c_int,
                core::ffi::c_int,
                BLOCK_SIZE,
            ) -> (),
        >;
    av1_loop_filter_init(cm as *mut AV1Common);
    av1_qm_init(&raw mut (*cm).quant_params, av1_num_planes(cm));
    av1_loop_restoration_precal();
    (*pbi).error.setjmp = 0 as core::ffi::c_int;
    ((*aom_get_worker_interface()).init)
        .expect("non-null function pointer")(&raw mut (*pbi).lf_worker);
    (*pbi).lf_worker.thread_name = b"aom lf worker\0" as *const u8
        as *const core::ffi::c_char;
    return pbi as *mut AV1Decoder;}) {
Some(value) => return value,
None => {
        (*pbi).error.setjmp = 0 as core::ffi::c_int;
        av1_decoder_remove(pbi);
        return 0 as *mut AV1Decoder;}
}
}}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dealloc_dec_jobs(mut tile_mt_info: *mut AV1DecTileMTData) {
    if !tile_mt_info.is_null() {
        if !((*tile_mt_info).job_mutex).is_null() {
            pthread_mutex_destroy((*tile_mt_info).job_mutex as *mut pthread_mutex_t);
            aom_free((*tile_mt_info).job_mutex as *mut core::ffi::c_void);
        }
        aom_free((*tile_mt_info).job_queue as *mut core::ffi::c_void);
        memset(
            tile_mt_info as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1DecTileMTData>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dec_free_cb_buf(mut pbi: *mut AV1Decoder) {
    aom_free((*pbi).cb_buffer_base as *mut core::ffi::c_void);
    (*pbi).cb_buffer_base = 0 as *mut CB_BUFFER;
    (*pbi).cb_buffer_alloc_size = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_decoder_remove(mut pbi: *mut AV1Decoder) {
    let mut i: core::ffi::c_int = 0;
    if pbi.is_null() {
        return;
    }
    aom_free_frame_buffer(&raw mut (*pbi).tile_list_outbuf);
    ((*aom_get_worker_interface()).end)
        .expect("non-null function pointer")(&raw mut (*pbi).lf_worker);
    aom_free((*pbi).lf_worker.data1);
    if !((*pbi).thread_data).is_null() {
        let mut worker_idx: core::ffi::c_int = 1 as core::ffi::c_int;
        while worker_idx < (*pbi).num_workers {
            let thread_data: *mut DecWorkerData = ((*pbi).thread_data)
                .offset(worker_idx as isize);
            if !((*thread_data).td).is_null() {
                av1_free_mc_tmp_buf((*thread_data).td);
                aom_free((*thread_data).td as *mut core::ffi::c_void);
            }
            worker_idx += 1;
        }
        aom_free((*pbi).thread_data as *mut core::ffi::c_void);
    }
    aom_free((*pbi).dcb.xd.seg_mask as *mut core::ffi::c_void);
    i = 0 as core::ffi::c_int;
    while i < (*pbi).num_workers {
        let worker: *mut AVxWorker = &raw mut *((*pbi).tile_workers).offset(i as isize)
            as *mut AVxWorker;
        ((*aom_get_worker_interface()).end).expect("non-null function pointer")(worker);
        i += 1;
    }
    if !((*pbi).row_mt_mutex_).is_null() {
        pthread_mutex_destroy((*pbi).row_mt_mutex_ as *mut pthread_mutex_t);
        aom_free((*pbi).row_mt_mutex_ as *mut core::ffi::c_void);
    }
    if !((*pbi).row_mt_cond_).is_null() {
        pthread_cond_destroy((*pbi).row_mt_cond_ as *mut pthread_cond_t);
        aom_free((*pbi).row_mt_cond_ as *mut core::ffi::c_void);
    }
    i = 0 as core::ffi::c_int;
    while i < (*pbi).allocated_tiles {
        let tile_data: *mut TileDataDec = ((*pbi).tile_data).offset(i as isize);
        av1_dec_row_mt_dealloc(&raw mut (*tile_data).dec_row_mt_sync);
        i += 1;
    }
    aom_free((*pbi).tile_data as *mut core::ffi::c_void);
    aom_free((*pbi).tile_workers as *mut core::ffi::c_void);
    if (*pbi).num_workers > 0 as core::ffi::c_int {
        av1_loop_filter_dealloc(&raw mut (*pbi).lf_row_sync);
        av1_loop_restoration_dealloc(&raw mut (*pbi).lr_row_sync);
        av1_dealloc_dec_jobs(&raw mut (*pbi).tile_mt_info);
    }
    av1_dec_free_cb_buf(pbi);
    av1_free_mc_tmp_buf(&raw mut (*pbi).td);
    aom_img_metadata_array_free((*pbi).metadata);
    av1_remove_common(&raw mut (*pbi).common);
    aom_free(pbi as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_visit_palette(
    pbi: *mut AV1Decoder,
    xd: *mut MACROBLOCKD,
    mut r: *mut aom_reader,
    mut visit: palette_visitor_fn_t,
) {
    if is_inter_block(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) == 0 {
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane
            < (if (2 as core::ffi::c_int) < av1_num_planes(&raw mut (*pbi).common) {
                2 as core::ffi::c_int
            } else {
                av1_num_planes(&raw mut (*pbi).common)
            })
        {
            if plane == 0 as core::ffi::c_int
                || (*xd).is_chroma_ref as core::ffi::c_int != 0
            {
                if (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
                    .palette_mode_info
                    .palette_size[plane as usize] != 0
                {
                    visit.expect("non-null function pointer")(xd, plane, r);
                }
            } else {
                if !((**((*xd).mi).offset(0 as core::ffi::c_int as isize))
                    .palette_mode_info
                    .palette_size[plane as usize] as core::ffi::c_int
                    == 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 18],
                            [core::ffi::c_char; 18],
                        >(*b"av1_visit_palette\0"))
                            .as_ptr(),
                        b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
                        252 as core::ffi::c_int,
                        b"xd->mi[0]->palette_mode_info.palette_size[plane] == 0\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            plane += 1;
        }
    }
}
unsafe extern "C-unwind" fn equal_dimensions(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> core::ffi::c_int {
    return ((*a).c2rust_unnamed_0.c2rust_unnamed.y_height
        == (*b).c2rust_unnamed_0.c2rust_unnamed.y_height
        && (*a).c2rust_unnamed.c2rust_unnamed.y_width
            == (*b).c2rust_unnamed.c2rust_unnamed.y_width
        && (*a).c2rust_unnamed_0.c2rust_unnamed.uv_height
            == (*b).c2rust_unnamed_0.c2rust_unnamed.uv_height
        && (*a).c2rust_unnamed.c2rust_unnamed.uv_width
            == (*b).c2rust_unnamed.c2rust_unnamed.uv_width) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_copy_reference_dec(
    mut pbi: *mut AV1Decoder,
    mut idx: core::ffi::c_int,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> aom_codec_err_t {
    let mut cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let cfg: *const YV12_BUFFER_CONFIG = get_ref_frame(cm, idx);
    if cfg.is_null() {
        aom_set_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_ERROR,
            b"No reference frame\0" as *const u8 as *const core::ffi::c_char,
        );
        return (*pbi).error.error_code;
    }
    if equal_dimensions(cfg, sd) == 0 {
        aom_set_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const core::ffi::c_char,
        );
        return (*pbi).error.error_code;
    }
    aom_yv12_copy_frame_c(
        cfg as *const yv12_buffer_config,
        sd as *mut yv12_buffer_config,
        num_planes,
    );
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn equal_dimensions_and_border(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> core::ffi::c_int {
    return ((*a).c2rust_unnamed_0.c2rust_unnamed.y_height
        == (*b).c2rust_unnamed_0.c2rust_unnamed.y_height
        && (*a).c2rust_unnamed.c2rust_unnamed.y_width
            == (*b).c2rust_unnamed.c2rust_unnamed.y_width
        && (*a).c2rust_unnamed_0.c2rust_unnamed.uv_height
            == (*b).c2rust_unnamed_0.c2rust_unnamed.uv_height
        && (*a).c2rust_unnamed.c2rust_unnamed.uv_width
            == (*b).c2rust_unnamed.c2rust_unnamed.uv_width
        && (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride
            == (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride
        && (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride
            == (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride
        && (*a).border == (*b).border
        && (*a).flags & YV12_FLAG_HIGHBITDEPTH == (*b).flags & YV12_FLAG_HIGHBITDEPTH)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_reference_dec(
    mut cm: *mut AV1_COMMON,
    mut idx: core::ffi::c_int,
    mut use_external_ref: core::ffi::c_int,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> aom_codec_err_t {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if ((*(*cm).error).setjmp == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_set_reference_dec\0"))
                .as_ptr(),
            b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
            299 as core::ffi::c_int,
            b"cm->error->setjmp\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut ref_buf: *mut YV12_BUFFER_CONFIG = get_ref_frame(cm, idx);
    if ref_buf.is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_ERROR,
            b"No reference frame\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if use_external_ref == 0 {
        if equal_dimensions(ref_buf, sd) == 0 {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_ERROR,
                b"Incorrect buffer dimensions\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        aom_yv12_copy_frame_c(sd, ref_buf as *mut yv12_buffer_config, num_planes);
    } else {
        if equal_dimensions_and_border(ref_buf, sd) == 0 {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_ERROR,
                b"Incorrect buffer dimensions\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        (*ref_buf).store_buf_adr[0 as core::ffi::c_int as usize] = (*ref_buf)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer;
        (*ref_buf).store_buf_adr[1 as core::ffi::c_int as usize] = (*ref_buf)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer;
        (*ref_buf).store_buf_adr[2 as core::ffi::c_int as usize] = (*ref_buf)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer;
        (*ref_buf).c2rust_unnamed_4.c2rust_unnamed.y_buffer = (*sd)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer;
        (*ref_buf).c2rust_unnamed_4.c2rust_unnamed.u_buffer = (*sd)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer;
        (*ref_buf).c2rust_unnamed_4.c2rust_unnamed.v_buffer = (*sd)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer;
        (*ref_buf).use_external_reference_buffers = 1 as core::ffi::c_int;
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_copy_new_frame_dec(
    mut cm: *mut AV1_COMMON,
    mut new_frame: *mut YV12_BUFFER_CONFIG,
    mut sd: *mut YV12_BUFFER_CONFIG,
) -> aom_codec_err_t {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if equal_dimensions_and_border(new_frame, sd) == 0 {
        aom_set_error(
            (*cm).error,
            AOM_CODEC_ERROR,
            b"Incorrect buffer dimensions\0" as *const u8 as *const core::ffi::c_char,
        );
        return (*(*cm).error).error_code;
    }
    aom_yv12_copy_frame_c(new_frame, sd as *mut yv12_buffer_config, num_planes);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn release_current_frame(mut pbi: *mut AV1Decoder) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let pool: *mut BufferPool = (*cm).buffer_pool;
    (*(*cm).cur_frame).buf.corrupted = 1 as core::ffi::c_int;
    lock_buffer_pool(pool);
    decrease_ref_count((*cm).cur_frame, pool);
    unlock_buffer_pool(pool);
    (*cm).cur_frame = 0 as *mut RefCntBuffer;
}
unsafe extern "C-unwind" fn update_frame_buffers(
    mut pbi: *mut AV1Decoder,
    mut frame_decoded: core::ffi::c_int,
) {
    let mut ref_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mask: core::ffi::c_int = 0;
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let pool: *mut BufferPool = (*cm).buffer_pool;
    if frame_decoded != 0 {
        lock_buffer_pool(pool);
        if (*pbi).camera_frame_header_ready == 0 {
            mask = (*cm).current_frame.refresh_frame_flags;
            while mask != 0 {
                if mask & 1 as core::ffi::c_int != 0 {
                    decrease_ref_count((*cm).ref_frame_map[ref_index as usize], pool);
                    (*cm).ref_frame_map[ref_index as usize] = (*cm).cur_frame;
                    (*(*cm).cur_frame).ref_count += 1;
                }
                ref_index += 1;
                mask >>= 1 as core::ffi::c_int;
            }
        }
        if (*cm).show_existing_frame != 0 || (*cm).show_frame != 0 {
            if (*pbi).output_all_layers != 0 {
                if (*pbi).num_output_frames >= MAX_NUM_SPATIAL_LAYERS as size_t {
                    (*(*cm).cur_frame).buf.corrupted = 1 as core::ffi::c_int;
                    decrease_ref_count((*cm).cur_frame, pool);
                    (*pbi).error.error_code = AOM_CODEC_UNSUP_BITSTREAM;
                } else {
                    (*pbi).output_frames[(*pbi).num_output_frames as usize] = (*cm)
                        .cur_frame;
                    (*pbi).num_output_frames = ((*pbi).num_output_frames)
                        .wrapping_add(1);
                }
            } else {
                if !((*pbi).num_output_frames == 0 as size_t
                    || (*pbi).num_output_frames == 1 as size_t) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 21],
                            [core::ffi::c_char; 21],
                        >(*b"update_frame_buffers\0"))
                            .as_ptr(),
                        b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
                        404 as core::ffi::c_int,
                        b"pbi->num_output_frames == 0 || pbi->num_output_frames == 1\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if (*pbi).num_output_frames > 0 as size_t {
                    decrease_ref_count(
                        (*pbi).output_frames[0 as core::ffi::c_int as usize],
                        pool,
                    );
                }
                (*pbi).output_frames[0 as core::ffi::c_int as usize] = (*cm).cur_frame;
                (*pbi).num_output_frames = 1 as size_t;
            }
        } else {
            decrease_ref_count((*cm).cur_frame, pool);
        }
        unlock_buffer_pool(pool);
    } else {
        lock_buffer_pool(pool);
        decrease_ref_count((*cm).cur_frame, pool);
        unlock_buffer_pool(pool);
    }
    (*cm).cur_frame = 0 as *mut RefCntBuffer;
    if (*pbi).camera_frame_header_ready == 0 {
        ref_index = 0 as core::ffi::c_int;
        while ref_index < INTER_REFS_PER_FRAME as core::ffi::c_int {
            (*cm).remapped_ref_idx[ref_index as usize] = INVALID_IDX;
            ref_index += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_receive_compressed_data(
    mut pbi: *mut AV1Decoder,
    mut size: size_t,
    mut psource: *mut *const uint8_t,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut source: *const uint8_t = *psource;
    (*pbi).error.error_code = AOM_CODEC_OK;
    (*pbi).error.has_detail = 0 as core::ffi::c_int;
    if size == 0 as size_t {
        let mut ref_buf: *mut RefCntBuffer = get_ref_frame_buf(
            cm,
            LAST_FRAME as core::ffi::c_int as MV_REFERENCE_FRAME,
        );
        if !ref_buf.is_null() {
            (*ref_buf).buf.corrupted = 1 as core::ffi::c_int;
        }
    }
    if (assign_cur_frame_new_fb(cm)).is_null() {
        (*pbi).error.error_code = AOM_CODEC_MEM_ERROR;
        return 1 as core::ffi::c_int;
    }
    loop {
match crate::jump::value_scope((((*pbi).error.jmp).as_mut_ptr()).cast(), || {
    (*pbi).error.setjmp = 1 as core::ffi::c_int;
    let mut frame_decoded: core::ffi::c_int = aom_decode_frame_from_obus(
        pbi as *mut AV1Decoder,
        source,
        source.offset(size as isize),
        psource,
    );
    if frame_decoded < 0 as core::ffi::c_int {
        if !((*pbi).error.error_code as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"av1_receive_compressed_data\0"))
                    .as_ptr(),
                b"decoder.c\0" as *const u8 as *const core::ffi::c_char,
                483 as core::ffi::c_int,
                b"pbi->error.error_code != AOM_CODEC_OK\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        release_current_frame(pbi);
        (*pbi).error.setjmp = 0 as core::ffi::c_int;
        return 1 as core::ffi::c_int;
    }
    update_frame_buffers(pbi, frame_decoded);
    if frame_decoded != 0 {
        (*pbi).decoding_first_frame = 0 as core::ffi::c_int;
    }
    if (*pbi).error.error_code as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*pbi).error.setjmp = 0 as core::ffi::c_int;
        return 1 as core::ffi::c_int;
    }
    if (*cm).show_existing_frame == 0 {
        if (*cm).seg.enabled != 0 {
            if !((*cm).prev_frame).is_null()
                && (*cm).mi_params.mi_rows == (*(*cm).prev_frame).mi_rows
                && (*cm).mi_params.mi_cols == (*(*cm).prev_frame).mi_cols
            {
                (*cm).last_frame_seg_map = (*(*cm).prev_frame).seg_map;
            } else {
                (*cm).last_frame_seg_map = 0 as *mut uint8_t;
            }
        }
    }
    (*pbi).error.setjmp = 0 as core::ffi::c_int;
    return 0 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
            as *const AVxWorkerInterface;
        let mut i: core::ffi::c_int = 0;
        (*pbi).error.setjmp = 0 as core::ffi::c_int;
        ((*winterface).sync).expect("non-null function pointer")(&raw mut (*pbi).lf_worker);
        i = 0 as core::ffi::c_int;
        while i < (*pbi).num_workers {
            ((*winterface).sync)
                .expect(
                    "non-null function pointer",
                )(&raw mut *((*pbi).tile_workers).offset(i as isize));
            i += 1;
        }
        release_current_frame(pbi);
        return -(1 as core::ffi::c_int);}
}
}}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_raw_frame(
    mut pbi: *mut AV1Decoder,
    mut index: size_t,
    mut sd: *mut *mut YV12_BUFFER_CONFIG,
    mut grain_params: *mut *mut aom_film_grain_t,
) -> core::ffi::c_int {
    if index >= (*pbi).num_output_frames {
        return -(1 as core::ffi::c_int);
    }
    *sd = &raw mut (**((*pbi).output_frames).as_mut_ptr().offset(index as isize)).buf;
    *grain_params = &raw mut (**((*pbi).output_frames).as_mut_ptr().offset(index as isize))
        .film_grain_params;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_frame_to_show(
    mut pbi: *mut AV1Decoder,
    mut frame: *mut YV12_BUFFER_CONFIG,
) -> core::ffi::c_int {
    if (*pbi).num_output_frames == 0 as size_t {
        return -(1 as core::ffi::c_int);
    }
    *frame = (*(*pbi)
        .output_frames[((*pbi).num_output_frames).wrapping_sub(1 as size_t) as usize])
        .buf;
    return 0 as core::ffi::c_int;
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
