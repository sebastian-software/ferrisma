/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 *
 */

// Mechanically translated from upstream/aom/av1/common/restoration.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    fn aom_yv12_partial_coloc_copy_u_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_yv12_partial_coloc_copy_v_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_yv12_partial_coloc_copy_y_c(
        src_ybc: *const yv12_buffer_config,
        dst_ybc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_realloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        fb: *mut aom_codec_frame_buffer_t,
        cb: aom_get_frame_buffer_cb_fn_t,
        cb_priv: *mut core::ffi::c_void,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_highbd_wiener_convolve_add_src_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        filter_x: *const int16_t,
        x_step_q4: core::ffi::c_int,
        filter_y: *const int16_t,
        y_step_q4: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        conv_params: *const WienerConvolveParams,
        bd: core::ffi::c_int,
    );
    fn av1_wiener_convolve_add_src_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        filter_x: *const int16_t,
        x_step_q4: core::ffi::c_int,
        filter_y: *const int16_t,
        y_step_q4: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        conv_params: *const WienerConvolveParams,
    );
    fn av1_upscale_normative_rows(
        cm: *const AV1_COMMON,
        src: *const uint8_t,
        src_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        plane: core::ffi::c_int,
        rows: core::ffi::c_int,
    );
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type aom_metadata_array_t = aom_metadata_array;
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
pub type aom_metadata_insert_flags_t = aom_metadata_insert_flags;
pub type aom_metadata_insert_flags = core::ffi::c_uint;
pub const AOM_MIF_ANY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 18;
pub const AOM_MIF_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 17;
pub const AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 16;
pub const AOM_MIF_ANY_FRAME: aom_metadata_insert_flags = 2;
pub const AOM_MIF_KEY_FRAME: aom_metadata_insert_flags = 1;
pub const AOM_MIF_NON_KEY_FRAME: aom_metadata_insert_flags = 0;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type aom_color_range_t = aom_color_range;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
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
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
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
pub type aom_color_primaries_t = aom_color_primaries;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
}
pub type pthread_mutex_t = libc::pthread_mutex_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
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
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 40],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type uintptr_t = usize;
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
pub type jmp_buf = [core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type qm_val_t = uint8_t;
pub type pthread_cond_t = libc::pthread_cond_t;
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
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
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
pub struct WienerConvolveParams {
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
}
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
pub type AV1_COMMON = AV1Common;
pub type stripe_filter_fun = Option<
    unsafe extern "C-unwind" fn(
        *const RestorationUnitInfo,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *mut int32_t,
        core::ffi::c_int,
        *mut aom_internal_error_info,
    ) -> (),
>;
pub type copy_fun = Option<
    unsafe extern "C-unwind" fn(
        *const YV12_BUFFER_CONFIG,
        *mut YV12_BUFFER_CONFIG,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
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
pub type sync_write_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type sync_read_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn aom_memset16(
    mut dest: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
    mut length: size_t,
) -> *mut core::ffi::c_void {
    let mut i: size_t = 0;
    let mut dest16: *mut uint16_t = dest as *mut uint16_t;
    i = 0 as size_t;
    while i < length {
        let fresh0 = dest16;
        dest16 = dest16.offset(1);
        *fresh0 = val as uint16_t;
        i = i.wrapping_add(1);
    }
    return dest;
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
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
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
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const WIENER_ROUND0_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_conv_params_wiener(
    mut bd: core::ffi::c_int,
) -> WienerConvolveParams {
    let mut conv_params: WienerConvolveParams = WienerConvolveParams {
        round_0: 0,
        round_1: 0,
    };
    conv_params.round_0 = WIENER_ROUND0_BITS;
    conv_params.round_1 = 2 as core::ffi::c_int * FILTER_BITS - conv_params.round_0;
    let intbufrange: core::ffi::c_int = bd + FILTER_BITS - conv_params.round_0
        + 2 as core::ffi::c_int;
    if intbufrange > 16 as core::ffi::c_int {
        conv_params.round_0 += intbufrange - 16 as core::ffi::c_int;
        conv_params.round_1 -= intbufrange - 16 as core::ffi::c_int;
    }
    return conv_params;
}
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const AOM_RESTORATION_FRAME_BORDER: core::ffi::c_int = 32 as core::ffi::c_int;
pub const RESTORATION_PROC_UNIT_SIZE: core::ffi::c_int = 64 as core::ffi::c_int;
pub const RESTORATION_UNIT_OFFSET: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SGRPROJ_BORDER_VERT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SGRPROJ_BORDER_HORZ: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RESTORATION_BORDER_VERT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RESTORATION_BORDER_HORZ: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RESTORATION_BORDER: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RESTORATION_CTX_VERT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const RESTORATION_EXTRA_HORZ: core::ffi::c_int = 4 as core::ffi::c_int;
pub const RESTORATION_UNITSIZE_MAX: core::ffi::c_int = 256 as core::ffi::c_int;
pub const RESTORATION_UNITPELS_HORZ_MAX: core::ffi::c_int = RESTORATION_UNITSIZE_MAX
    * 3 as core::ffi::c_int / 2 as core::ffi::c_int
    + 2 as core::ffi::c_int * RESTORATION_BORDER_HORZ + 16 as core::ffi::c_int;
pub const RESTORATION_UNITPELS_VERT_MAX: core::ffi::c_int = RESTORATION_UNITSIZE_MAX
    * 3 as core::ffi::c_int / 2 as core::ffi::c_int
    + 2 as core::ffi::c_int * RESTORATION_BORDER_VERT + RESTORATION_UNIT_OFFSET;
pub const RESTORATION_UNITPELS_MAX: core::ffi::c_int = RESTORATION_UNITPELS_HORZ_MAX
    * RESTORATION_UNITPELS_VERT_MAX;
pub const SGRPROJ_PRJ_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SGRPROJ_RST_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
#[no_mangle]
pub static mut av1_sgr_params: [sgr_params_type; 16] = [
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [140 as core::ffi::c_int, 3236 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [112 as core::ffi::c_int, 2158 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [93 as core::ffi::c_int, 1618 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [80 as core::ffi::c_int, 1438 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [70 as core::ffi::c_int, 1295 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [58 as core::ffi::c_int, 1177 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [47 as core::ffi::c_int, 1079 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [37 as core::ffi::c_int, 996 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [30 as core::ffi::c_int, 925 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [25 as core::ffi::c_int, 863 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [0 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [-(1 as core::ffi::c_int), 2589 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [0 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [-(1 as core::ffi::c_int), 1618 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [0 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [-(1 as core::ffi::c_int), 1177 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [0 as core::ffi::c_int, 1 as core::ffi::c_int],
            s: [-(1 as core::ffi::c_int), 925 as core::ffi::c_int],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 0 as core::ffi::c_int],
            s: [56 as core::ffi::c_int, -(1 as core::ffi::c_int)],
        };
        init
    },
    {
        let mut init = sgr_params_type {
            r: [2 as core::ffi::c_int, 0 as core::ffi::c_int],
            s: [22 as core::ffi::c_int, -(1 as core::ffi::c_int)],
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_upsampled_plane_size(
    mut cm: *const AV1_COMMON,
    mut is_uv: core::ffi::c_int,
    mut plane_w: *mut core::ffi::c_int,
    mut plane_h: *mut core::ffi::c_int,
) {
    let mut ss_x: core::ffi::c_int = (is_uv != 0
        && (*(*cm).seq_params).subsampling_x != 0) as core::ffi::c_int;
    let mut ss_y: core::ffi::c_int = (is_uv != 0
        && (*(*cm).seq_params).subsampling_y != 0) as core::ffi::c_int;
    *plane_w = (*cm).superres_upscaled_width
        + ((1 as core::ffi::c_int) << ss_x >> 1 as core::ffi::c_int) >> ss_x;
    *plane_h = (*cm).height + ((1 as core::ffi::c_int) << ss_y >> 1 as core::ffi::c_int)
        >> ss_y;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lr_count_units(
    mut unit_size: core::ffi::c_int,
    mut plane_size: core::ffi::c_int,
) -> core::ffi::c_int {
    return if (plane_size + (unit_size >> 1 as core::ffi::c_int)) / unit_size
        > 1 as core::ffi::c_int
    {
        (plane_size + (unit_size >> 1 as core::ffi::c_int)) / unit_size
    } else {
        1 as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_restoration_struct(
    mut cm: *mut AV1_COMMON,
    mut rsi: *mut RestorationInfo,
    mut is_uv: core::ffi::c_int,
) {
    let mut plane_w: core::ffi::c_int = 0;
    let mut plane_h: core::ffi::c_int = 0;
    av1_get_upsampled_plane_size(cm, is_uv, &raw mut plane_w, &raw mut plane_h);
    let unit_size: core::ffi::c_int = (*rsi).restoration_unit_size;
    let horz_units: core::ffi::c_int = av1_lr_count_units(unit_size, plane_w)
        as core::ffi::c_int;
    let vert_units: core::ffi::c_int = av1_lr_count_units(unit_size, plane_h)
        as core::ffi::c_int;
    (*rsi).num_rest_units = horz_units * vert_units;
    (*rsi).horz_units = horz_units;
    (*rsi).vert_units = vert_units;
    aom_free((*rsi).unit_info as *mut core::ffi::c_void);
    (*rsi).unit_info = aom_memalign(
        16 as size_t,
        (::core::mem::size_of::<RestorationUnitInfo>() as size_t)
            .wrapping_mul((*rsi).num_rest_units as size_t),
    ) as *mut RestorationUnitInfo;
    if ((*rsi).unit_info).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate rsi->unit_info\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_free_restoration_struct(
    mut rst_info: *mut RestorationInfo,
) {
    aom_free((*rst_info).unit_info as *mut core::ffi::c_void);
    (*rst_info).unit_info = 0 as *mut RestorationUnitInfo;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_precal() {}
unsafe extern "C-unwind" fn extend_frame_lowbd(
    mut data: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: ptrdiff_t,
    mut border_horz: core::ffi::c_int,
    mut border_vert: core::ffi::c_int,
) {
    let mut data_p: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < height {
        data_p = data.offset((i as ptrdiff_t * stride) as isize);
        memset(
            data_p.offset(-(border_horz as isize)) as *mut core::ffi::c_void,
            *data_p.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            border_horz as size_t,
        );
        memset(
            data_p.offset(width as isize) as *mut core::ffi::c_void,
            *data_p.offset((width - 1 as core::ffi::c_int) as isize) as core::ffi::c_int,
            border_horz as size_t,
        );
        i += 1;
    }
    data_p = data.offset(-(border_horz as isize));
    i = -border_vert;
    while i < 0 as core::ffi::c_int {
        memcpy(
            data_p.offset((i as ptrdiff_t * stride) as isize) as *mut core::ffi::c_void,
            data_p as *const core::ffi::c_void,
            (width + 2 as core::ffi::c_int * border_horz) as size_t,
        );
        i += 1;
    }
    i = height;
    while i < height + border_vert {
        memcpy(
            data_p.offset((i as ptrdiff_t * stride) as isize) as *mut core::ffi::c_void,
            data_p
                .offset(
                    ((height - 1 as core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as *const core::ffi::c_void,
            (width + 2 as core::ffi::c_int * border_horz) as size_t,
        );
        i += 1;
    }
}
unsafe extern "C-unwind" fn extend_frame_highbd(
    mut data: *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: ptrdiff_t,
    mut border_horz: core::ffi::c_int,
    mut border_vert: core::ffi::c_int,
) {
    let mut data_p: *mut uint16_t = 0 as *mut uint16_t;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < height {
        data_p = data.offset((i as ptrdiff_t * stride) as isize);
        j = -border_horz;
        while j < 0 as core::ffi::c_int {
            *data_p.offset(j as isize) = *data_p.offset(0 as core::ffi::c_int as isize);
            j += 1;
        }
        j = width;
        while j < width + border_horz {
            *data_p.offset(j as isize) = *data_p
                .offset((width - 1 as core::ffi::c_int) as isize);
            j += 1;
        }
        i += 1;
    }
    data_p = data.offset(-(border_horz as isize));
    i = -border_vert;
    while i < 0 as core::ffi::c_int {
        memcpy(
            data_p.offset((i as ptrdiff_t * stride) as isize) as *mut core::ffi::c_void,
            data_p as *const core::ffi::c_void,
            ((width + 2 as core::ffi::c_int * border_horz) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        i += 1;
    }
    i = height;
    while i < height + border_vert {
        memcpy(
            data_p.offset((i as ptrdiff_t * stride) as isize) as *mut core::ffi::c_void,
            data_p
                .offset(
                    ((height - 1 as core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as *const core::ffi::c_void,
            ((width + 2 as core::ffi::c_int * border_horz) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        i += 1;
    }
}
unsafe extern "C-unwind" fn copy_rest_unit_highbd(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        memcpy(
            dst.offset((i * dst_stride) as isize) as *mut core::ffi::c_void,
            src.offset((i * src_stride) as isize) as *const core::ffi::c_void,
            (width as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_extend_frame(
    mut data: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut border_horz: core::ffi::c_int,
    mut border_vert: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
) {
    if highbd != 0 {
        extend_frame_highbd(
            ((data as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            width,
            height,
            stride as ptrdiff_t,
            border_horz,
            border_vert,
        );
        return;
    }
    extend_frame_lowbd(
        data,
        width,
        height,
        stride as ptrdiff_t,
        border_horz,
        border_vert,
    );
}
unsafe extern "C-unwind" fn copy_rest_unit_lowbd(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        memcpy(
            dst.offset((i * dst_stride) as isize) as *mut core::ffi::c_void,
            src.offset((i * src_stride) as isize) as *const core::ffi::c_void,
            width as size_t,
        );
        i += 1;
    }
}
unsafe extern "C-unwind" fn copy_rest_unit(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
) {
    if highbd != 0 {
        copy_rest_unit_highbd(
            width,
            height,
            ((src as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            src_stride,
            ((dst as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            dst_stride,
        );
        return;
    }
    copy_rest_unit_lowbd(width, height, src, src_stride, dst, dst_stride);
}
unsafe extern "C-unwind" fn get_stripe_boundary_info(
    mut limits: *const RestorationTileLimits,
    mut plane_w: core::ffi::c_int,
    mut plane_h: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    mut copy_above: *mut core::ffi::c_int,
    mut copy_below: *mut core::ffi::c_int,
) {
    *copy_above = 1 as core::ffi::c_int;
    *copy_below = 1 as core::ffi::c_int;
    let full_stripe_height: core::ffi::c_int = RESTORATION_PROC_UNIT_SIZE >> ss_y;
    let runit_offset: core::ffi::c_int = RESTORATION_UNIT_OFFSET >> ss_y;
    let first_stripe_in_plane: core::ffi::c_int = ((*limits).v_start
        == 0 as core::ffi::c_int) as core::ffi::c_int;
    let this_stripe_height: core::ffi::c_int = full_stripe_height
        - (if first_stripe_in_plane != 0 {
            runit_offset
        } else {
            0 as core::ffi::c_int
        });
    let last_stripe_in_plane: core::ffi::c_int = ((*limits).v_start + this_stripe_height
        >= plane_h) as core::ffi::c_int;
    if first_stripe_in_plane != 0 {
        *copy_above = 0 as core::ffi::c_int;
    }
    if last_stripe_in_plane != 0 {
        *copy_below = 0 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn setup_processing_stripe_boundary(
    mut limits: *const RestorationTileLimits,
    mut rsb: *const RestorationStripeBoundaries,
    mut rsb_row: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut data8: *mut uint8_t,
    mut data_stride: core::ffi::c_int,
    mut rlbs: *mut RestorationLineBuffers,
    mut copy_above: core::ffi::c_int,
    mut copy_below: core::ffi::c_int,
    mut opt: core::ffi::c_int,
) {
    let buf_stride: core::ffi::c_int = (*rsb).stripe_boundary_stride;
    let buf_x0_off: core::ffi::c_int = (*limits).h_start;
    let line_width: core::ffi::c_int = (*limits).h_end - (*limits).h_start
        + 2 as core::ffi::c_int * RESTORATION_EXTRA_HORZ;
    let line_size: core::ffi::c_int = line_width << use_highbd;
    let data_x0: core::ffi::c_int = (*limits).h_start - RESTORATION_EXTRA_HORZ;
    if opt == 0 {
        if copy_above != 0 {
            let mut data8_tl: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset(((*limits).v_start * data_stride) as isize);
            let mut i: core::ffi::c_int = -RESTORATION_BORDER;
            while i < 0 as core::ffi::c_int {
                let buf_row: core::ffi::c_int = rsb_row
                    + (if i + 2 as core::ffi::c_int > 0 as core::ffi::c_int {
                        i + 2 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    });
                let buf_off: core::ffi::c_int = buf_x0_off + buf_row * buf_stride;
                let mut buf: *const uint8_t = ((*rsb).stripe_boundary_above)
                    .offset((buf_off << use_highbd) as isize);
                let mut dst8: *mut uint8_t = data8_tl.offset((i * data_stride) as isize);
                memcpy(
                    ((*rlbs).tmp_save_above[(i + RESTORATION_BORDER) as usize])
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    (if use_highbd != 0 {
                        ((dst8 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8
                    }) as *const core::ffi::c_void,
                    line_size as size_t,
                );
                memcpy(
                    (if use_highbd != 0 {
                        ((dst8 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8
                    }) as *mut core::ffi::c_void,
                    buf as *const core::ffi::c_void,
                    line_size as size_t,
                );
                i += 1;
            }
        }
        if copy_below != 0 {
            let stripe_end: core::ffi::c_int = (*limits).v_start + h;
            let mut data8_bl: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset((stripe_end * data_stride) as isize);
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < RESTORATION_BORDER {
                let buf_row_0: core::ffi::c_int = rsb_row
                    + (if i_0 < 2 as core::ffi::c_int - 1 as core::ffi::c_int {
                        i_0
                    } else {
                        2 as core::ffi::c_int - 1 as core::ffi::c_int
                    });
                let buf_off_0: core::ffi::c_int = buf_x0_off + buf_row_0 * buf_stride;
                let mut src: *const uint8_t = ((*rsb).stripe_boundary_below)
                    .offset((buf_off_0 << use_highbd) as isize);
                let mut dst8_0: *mut uint8_t = data8_bl
                    .offset((i_0 * data_stride) as isize);
                memcpy(
                    ((*rlbs).tmp_save_below[i_0 as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    (if use_highbd != 0 {
                        ((dst8_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8_0
                    }) as *const core::ffi::c_void,
                    line_size as size_t,
                );
                memcpy(
                    (if use_highbd != 0 {
                        ((dst8_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8_0
                    }) as *mut core::ffi::c_void,
                    src as *const core::ffi::c_void,
                    line_size as size_t,
                );
                i_0 += 1;
            }
        }
    } else {
        if copy_above != 0 {
            let mut data8_tl_0: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset(((*limits).v_start * data_stride) as isize);
            let mut dst8_1: *mut uint8_t = data8_tl_0
                .offset((-RESTORATION_BORDER * data_stride) as isize);
            memcpy(
                ((*rlbs).tmp_save_above[0 as core::ffi::c_int as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                (if use_highbd != 0 {
                    ((dst8_1 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    dst8_1
                }) as *const core::ffi::c_void,
                line_size as size_t,
            );
            memcpy(
                (if use_highbd != 0 {
                    ((dst8_1 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    dst8_1
                }) as *mut core::ffi::c_void,
                (if use_highbd != 0 {
                    ((data8_tl_0
                        .offset(
                            ((-(3 as core::ffi::c_int) + 1 as core::ffi::c_int)
                                * data_stride) as isize,
                        ) as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    data8_tl_0
                        .offset(
                            ((-(3 as core::ffi::c_int) + 1 as core::ffi::c_int)
                                * data_stride) as isize,
                        )
                }) as *const core::ffi::c_void,
                line_size as size_t,
            );
        }
        if copy_below != 0 {
            let stripe_end_0: core::ffi::c_int = (*limits).v_start + h;
            let mut data8_bl_0: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset((stripe_end_0 * data_stride) as isize);
            let mut dst8_2: *mut uint8_t = data8_bl_0
                .offset((2 as core::ffi::c_int * data_stride) as isize);
            memcpy(
                ((*rlbs).tmp_save_below[2 as core::ffi::c_int as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                (if use_highbd != 0 {
                    ((dst8_2 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    dst8_2
                }) as *const core::ffi::c_void,
                line_size as size_t,
            );
            memcpy(
                (if use_highbd != 0 {
                    ((dst8_2 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    dst8_2
                }) as *mut core::ffi::c_void,
                (if use_highbd != 0 {
                    ((data8_bl_0
                        .offset(
                            ((2 as core::ffi::c_int - 1 as core::ffi::c_int)
                                * data_stride) as isize,
                        ) as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    data8_bl_0
                        .offset(
                            ((2 as core::ffi::c_int - 1 as core::ffi::c_int)
                                * data_stride) as isize,
                        )
                }) as *const core::ffi::c_void,
                line_size as size_t,
            );
        }
    };
}
unsafe extern "C-unwind" fn restore_processing_stripe_boundary(
    mut limits: *const RestorationTileLimits,
    mut rlbs: *const RestorationLineBuffers,
    mut use_highbd: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut data8: *mut uint8_t,
    mut data_stride: core::ffi::c_int,
    mut copy_above: core::ffi::c_int,
    mut copy_below: core::ffi::c_int,
    mut opt: core::ffi::c_int,
) {
    let line_width: core::ffi::c_int = (*limits).h_end - (*limits).h_start
        + 2 as core::ffi::c_int * RESTORATION_EXTRA_HORZ;
    let line_size: core::ffi::c_int = line_width << use_highbd;
    let data_x0: core::ffi::c_int = (*limits).h_start - RESTORATION_EXTRA_HORZ;
    if opt == 0 {
        if copy_above != 0 {
            let mut data8_tl: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset(((*limits).v_start * data_stride) as isize);
            let mut i: core::ffi::c_int = -RESTORATION_BORDER;
            while i < 0 as core::ffi::c_int {
                let mut dst8: *mut uint8_t = data8_tl.offset((i * data_stride) as isize);
                memcpy(
                    (if use_highbd != 0 {
                        ((dst8 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8
                    }) as *mut core::ffi::c_void,
                    ((*rlbs).tmp_save_above[(i + RESTORATION_BORDER) as usize]).as_ptr()
                        as *const core::ffi::c_void,
                    line_size as size_t,
                );
                i += 1;
            }
        }
        if copy_below != 0 {
            let stripe_bottom: core::ffi::c_int = (*limits).v_start + h;
            let mut data8_bl: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset((stripe_bottom * data_stride) as isize);
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < RESTORATION_BORDER {
                if stripe_bottom + i_0 >= (*limits).v_end + RESTORATION_BORDER {
                    break;
                }
                let mut dst8_0: *mut uint8_t = data8_bl
                    .offset((i_0 * data_stride) as isize);
                memcpy(
                    (if use_highbd != 0 {
                        ((dst8_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8_0
                    }) as *mut core::ffi::c_void,
                    ((*rlbs).tmp_save_below[i_0 as usize]).as_ptr()
                        as *const core::ffi::c_void,
                    line_size as size_t,
                );
                i_0 += 1;
            }
        }
    } else {
        if copy_above != 0 {
            let mut data8_tl_0: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset(((*limits).v_start * data_stride) as isize);
            let mut dst8_1: *mut uint8_t = data8_tl_0
                .offset((-RESTORATION_BORDER * data_stride) as isize);
            memcpy(
                (if use_highbd != 0 {
                    ((dst8_1 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                        as *mut uint8_t
                } else {
                    dst8_1
                }) as *mut core::ffi::c_void,
                ((*rlbs).tmp_save_above[0 as core::ffi::c_int as usize]).as_ptr()
                    as *const core::ffi::c_void,
                line_size as size_t,
            );
        }
        if copy_below != 0 {
            let stripe_bottom_0: core::ffi::c_int = (*limits).v_start + h;
            let mut data8_bl_0: *mut uint8_t = data8
                .offset(data_x0 as isize)
                .offset((stripe_bottom_0 * data_stride) as isize);
            if (stripe_bottom_0 + 2 as core::ffi::c_int)
                < (*limits).v_end + RESTORATION_BORDER
            {
                let mut dst8_2: *mut uint8_t = data8_bl_0
                    .offset((2 as core::ffi::c_int * data_stride) as isize);
                memcpy(
                    (if use_highbd != 0 {
                        ((dst8_2 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
                            as *mut uint8_t
                    } else {
                        dst8_2
                    }) as *mut core::ffi::c_void,
                    ((*rlbs).tmp_save_below[2 as core::ffi::c_int as usize]).as_ptr()
                        as *const core::ffi::c_void,
                    line_size as size_t,
                );
            }
        }
    };
}
unsafe extern "C-unwind" fn wiener_filter_stripe(
    mut rui: *const RestorationUnitInfo,
    mut stripe_width: core::ffi::c_int,
    mut stripe_height: core::ffi::c_int,
    mut procunit_width: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut bit_depth: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
) {
    let conv_params: WienerConvolveParams = get_conv_params_wiener(8 as core::ffi::c_int)
        as WienerConvolveParams;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < stripe_width {
        let mut w: core::ffi::c_int = if procunit_width
            < stripe_width - j + 15 as core::ffi::c_int & !(15 as core::ffi::c_int)
        {
            procunit_width
        } else {
            stripe_width - j + 15 as core::ffi::c_int & !(15 as core::ffi::c_int)
        };
        let mut src_p: *const uint8_t = src.offset(j as isize);
        let mut dst_p: *mut uint8_t = dst.offset(j as isize);
        av1_wiener_convolve_add_src_c(
            src_p,
            src_stride as ptrdiff_t,
            dst_p,
            dst_stride as ptrdiff_t,
            ((*rui).wiener_info.hfilter).as_ptr(),
            16 as core::ffi::c_int,
            ((*rui).wiener_info.vfilter).as_ptr(),
            16 as core::ffi::c_int,
            w,
            stripe_height,
            &conv_params,
        );
        j += procunit_width;
    }
}
unsafe extern "C-unwind" fn boxsum1(
    mut src: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut sqr: core::ffi::c_int,
    mut dst: *mut int32_t,
    mut dst_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut a: core::ffi::c_int = 0;
    let mut b: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    if sqr == 0 {
        j = 0 as core::ffi::c_int;
        while j < width {
            a = *src.offset(j as isize) as core::ffi::c_int;
            b = *src.offset((src_stride + j) as isize) as core::ffi::c_int;
            c = *src.offset((2 as core::ffi::c_int * src_stride + j) as isize)
                as core::ffi::c_int;
            *dst.offset(j as isize) = (a + b) as int32_t;
            i = 1 as core::ffi::c_int;
            while i < height - 2 as core::ffi::c_int {
                *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
                a = b;
                b = c;
                c = *src.offset(((i + 2 as core::ffi::c_int) * src_stride + j) as isize)
                    as core::ffi::c_int;
                i += 1;
            }
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
            *dst.offset(((i + 1 as core::ffi::c_int) * dst_stride + j) as isize) = (b
                + c) as int32_t;
            j += 1;
        }
    } else {
        j = 0 as core::ffi::c_int;
        while j < width {
            a = (*src.offset(j as isize) * *src.offset(j as isize)) as core::ffi::c_int;
            b = (*src.offset((src_stride + j) as isize)
                * *src.offset((src_stride + j) as isize)) as core::ffi::c_int;
            c = (*src.offset((2 as core::ffi::c_int * src_stride + j) as isize)
                * *src.offset((2 as core::ffi::c_int * src_stride + j) as isize))
                as core::ffi::c_int;
            *dst.offset(j as isize) = (a + b) as int32_t;
            i = 1 as core::ffi::c_int;
            while i < height - 2 as core::ffi::c_int {
                *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
                a = b;
                b = c;
                c = (*src.offset(((i + 2 as core::ffi::c_int) * src_stride + j) as isize)
                    * *src
                        .offset(((i + 2 as core::ffi::c_int) * src_stride + j) as isize))
                    as core::ffi::c_int;
                i += 1;
            }
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
            *dst.offset(((i + 1 as core::ffi::c_int) * dst_stride + j) as isize) = (b
                + c) as int32_t;
            j += 1;
        }
    }
    i = 0 as core::ffi::c_int;
    while i < height {
        a = *dst.offset((i * dst_stride) as isize) as core::ffi::c_int;
        b = *dst.offset((i * dst_stride + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        c = *dst.offset((i * dst_stride + 2 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        *dst.offset((i * dst_stride) as isize) = (a + b) as int32_t;
        j = 1 as core::ffi::c_int;
        while j < width - 2 as core::ffi::c_int {
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
            a = b;
            b = c;
            c = *dst.offset((i * dst_stride + (j + 2 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            j += 1;
        }
        *dst.offset((i * dst_stride + j) as isize) = (a + b + c) as int32_t;
        *dst.offset((i * dst_stride + (j + 1 as core::ffi::c_int)) as isize) = (b + c)
            as int32_t;
        i += 1;
    }
}
unsafe extern "C-unwind" fn boxsum2(
    mut src: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut sqr: core::ffi::c_int,
    mut dst: *mut int32_t,
    mut dst_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut a: core::ffi::c_int = 0;
    let mut b: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut d: core::ffi::c_int = 0;
    let mut e: core::ffi::c_int = 0;
    if sqr == 0 {
        j = 0 as core::ffi::c_int;
        while j < width {
            a = *src.offset(j as isize) as core::ffi::c_int;
            b = *src.offset((src_stride + j) as isize) as core::ffi::c_int;
            c = *src.offset((2 as core::ffi::c_int * src_stride + j) as isize)
                as core::ffi::c_int;
            d = *src.offset((3 as core::ffi::c_int * src_stride + j) as isize)
                as core::ffi::c_int;
            e = *src.offset((4 as core::ffi::c_int * src_stride + j) as isize)
                as core::ffi::c_int;
            *dst.offset(j as isize) = (a + b + c) as int32_t;
            *dst.offset((dst_stride + j) as isize) = (a + b + c + d) as int32_t;
            i = 2 as core::ffi::c_int;
            while i < height - 3 as core::ffi::c_int {
                *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e)
                    as int32_t;
                a = b;
                b = c;
                c = d;
                d = e;
                e = *src.offset(((i + 3 as core::ffi::c_int) * src_stride + j) as isize)
                    as core::ffi::c_int;
                i += 1;
            }
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e) as int32_t;
            *dst.offset(((i + 1 as core::ffi::c_int) * dst_stride + j) as isize) = (b + c
                + d + e) as int32_t;
            *dst.offset(((i + 2 as core::ffi::c_int) * dst_stride + j) as isize) = (c + d
                + e) as int32_t;
            j += 1;
        }
    } else {
        j = 0 as core::ffi::c_int;
        while j < width {
            a = (*src.offset(j as isize) * *src.offset(j as isize)) as core::ffi::c_int;
            b = (*src.offset((src_stride + j) as isize)
                * *src.offset((src_stride + j) as isize)) as core::ffi::c_int;
            c = (*src.offset((2 as core::ffi::c_int * src_stride + j) as isize)
                * *src.offset((2 as core::ffi::c_int * src_stride + j) as isize))
                as core::ffi::c_int;
            d = (*src.offset((3 as core::ffi::c_int * src_stride + j) as isize)
                * *src.offset((3 as core::ffi::c_int * src_stride + j) as isize))
                as core::ffi::c_int;
            e = (*src.offset((4 as core::ffi::c_int * src_stride + j) as isize)
                * *src.offset((4 as core::ffi::c_int * src_stride + j) as isize))
                as core::ffi::c_int;
            *dst.offset(j as isize) = (a + b + c) as int32_t;
            *dst.offset((dst_stride + j) as isize) = (a + b + c + d) as int32_t;
            i = 2 as core::ffi::c_int;
            while i < height - 3 as core::ffi::c_int {
                *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e)
                    as int32_t;
                a = b;
                b = c;
                c = d;
                d = e;
                e = (*src.offset(((i + 3 as core::ffi::c_int) * src_stride + j) as isize)
                    * *src
                        .offset(((i + 3 as core::ffi::c_int) * src_stride + j) as isize))
                    as core::ffi::c_int;
                i += 1;
            }
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e) as int32_t;
            *dst.offset(((i + 1 as core::ffi::c_int) * dst_stride + j) as isize) = (b + c
                + d + e) as int32_t;
            *dst.offset(((i + 2 as core::ffi::c_int) * dst_stride + j) as isize) = (c + d
                + e) as int32_t;
            j += 1;
        }
    }
    i = 0 as core::ffi::c_int;
    while i < height {
        a = *dst.offset((i * dst_stride) as isize) as core::ffi::c_int;
        b = *dst.offset((i * dst_stride + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        c = *dst.offset((i * dst_stride + 2 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        d = *dst.offset((i * dst_stride + 3 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        e = *dst.offset((i * dst_stride + 4 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        *dst.offset((i * dst_stride) as isize) = (a + b + c) as int32_t;
        *dst.offset((i * dst_stride + 1 as core::ffi::c_int) as isize) = (a + b + c + d)
            as int32_t;
        j = 2 as core::ffi::c_int;
        while j < width - 3 as core::ffi::c_int {
            *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e) as int32_t;
            a = b;
            b = c;
            c = d;
            d = e;
            e = *dst.offset((i * dst_stride + (j + 3 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            j += 1;
        }
        *dst.offset((i * dst_stride + j) as isize) = (a + b + c + d + e) as int32_t;
        *dst.offset((i * dst_stride + (j + 1 as core::ffi::c_int)) as isize) = (b + c + d
            + e) as int32_t;
        *dst.offset((i * dst_stride + (j + 2 as core::ffi::c_int)) as isize) = (c + d
            + e) as int32_t;
        i += 1;
    }
}
unsafe extern "C-unwind" fn boxsum(
    mut src: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut r: core::ffi::c_int,
    mut sqr: core::ffi::c_int,
    mut dst: *mut int32_t,
    mut dst_stride: core::ffi::c_int,
) {
    if r == 1 as core::ffi::c_int {
        boxsum1(src, width, height, src_stride, sqr, dst, dst_stride);
    } else if r == 2 as core::ffi::c_int {
        boxsum2(src, width, height, src_stride, sqr, dst, dst_stride);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_decode_xq(
    mut xqd: *const core::ffi::c_int,
    mut xq: *mut core::ffi::c_int,
    mut params: *const sgr_params_type,
) {
    if (*params).r[0 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        *xq.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *xq.offset(1 as core::ffi::c_int as isize) = ((1 as core::ffi::c_int)
            << SGRPROJ_PRJ_BITS) - *xqd.offset(1 as core::ffi::c_int as isize);
    } else if (*params).r[1 as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
        *xq.offset(0 as core::ffi::c_int as isize) = *xqd
            .offset(0 as core::ffi::c_int as isize);
        *xq.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
    } else {
        *xq.offset(0 as core::ffi::c_int as isize) = *xqd
            .offset(0 as core::ffi::c_int as isize);
        *xq.offset(1 as core::ffi::c_int as isize) = ((1 as core::ffi::c_int)
            << SGRPROJ_PRJ_BITS) - *xq.offset(0 as core::ffi::c_int as isize)
            - *xqd.offset(1 as core::ffi::c_int as isize);
    };
}
#[no_mangle]
pub static mut av1_x_by_xplus1: [int32_t; 256] = [
    1 as core::ffi::c_int,
    128 as core::ffi::c_int,
    171 as core::ffi::c_int,
    192 as core::ffi::c_int,
    205 as core::ffi::c_int,
    213 as core::ffi::c_int,
    219 as core::ffi::c_int,
    224 as core::ffi::c_int,
    228 as core::ffi::c_int,
    230 as core::ffi::c_int,
    233 as core::ffi::c_int,
    235 as core::ffi::c_int,
    236 as core::ffi::c_int,
    238 as core::ffi::c_int,
    239 as core::ffi::c_int,
    240 as core::ffi::c_int,
    241 as core::ffi::c_int,
    242 as core::ffi::c_int,
    243 as core::ffi::c_int,
    243 as core::ffi::c_int,
    244 as core::ffi::c_int,
    244 as core::ffi::c_int,
    245 as core::ffi::c_int,
    245 as core::ffi::c_int,
    246 as core::ffi::c_int,
    246 as core::ffi::c_int,
    247 as core::ffi::c_int,
    247 as core::ffi::c_int,
    247 as core::ffi::c_int,
    247 as core::ffi::c_int,
    248 as core::ffi::c_int,
    248 as core::ffi::c_int,
    248 as core::ffi::c_int,
    248 as core::ffi::c_int,
    249 as core::ffi::c_int,
    249 as core::ffi::c_int,
    249 as core::ffi::c_int,
    249 as core::ffi::c_int,
    249 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    250 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    251 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    252 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    253 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    254 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    255 as core::ffi::c_int,
    256 as core::ffi::c_int,
];
#[no_mangle]
pub static mut av1_one_by_x: [int32_t; 25] = [
    4096 as core::ffi::c_int,
    2048 as core::ffi::c_int,
    1365 as core::ffi::c_int,
    1024 as core::ffi::c_int,
    819 as core::ffi::c_int,
    683 as core::ffi::c_int,
    585 as core::ffi::c_int,
    512 as core::ffi::c_int,
    455 as core::ffi::c_int,
    410 as core::ffi::c_int,
    372 as core::ffi::c_int,
    341 as core::ffi::c_int,
    315 as core::ffi::c_int,
    293 as core::ffi::c_int,
    273 as core::ffi::c_int,
    256 as core::ffi::c_int,
    241 as core::ffi::c_int,
    228 as core::ffi::c_int,
    216 as core::ffi::c_int,
    205 as core::ffi::c_int,
    195 as core::ffi::c_int,
    186 as core::ffi::c_int,
    178 as core::ffi::c_int,
    171 as core::ffi::c_int,
    164 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn calculate_intermediate_result(
    mut dgd: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut dgd_stride: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut sgr_params_idx: core::ffi::c_int,
    mut radius_idx: core::ffi::c_int,
    mut pass: core::ffi::c_int,
    mut A: *mut int32_t,
    mut B: *mut int32_t,
) {
    let params: *const sgr_params_type = &*av1_sgr_params
        .as_ptr()
        .offset(sgr_params_idx as isize) as *const sgr_params_type;
    let r: core::ffi::c_int = (*params).r[radius_idx as usize];
    let width_ext: core::ffi::c_int = width
        + 2 as core::ffi::c_int * SGRPROJ_BORDER_HORZ;
    let height_ext: core::ffi::c_int = height
        + 2 as core::ffi::c_int * SGRPROJ_BORDER_VERT;
    let mut buf_stride: core::ffi::c_int = (width_ext + 3 as core::ffi::c_int
        & !(3 as core::ffi::c_int)) + 16 as core::ffi::c_int;
    let step: core::ffi::c_int = if pass == 0 as core::ffi::c_int {
        1 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    boxsum(
        dgd
            .offset(-((dgd_stride * SGRPROJ_BORDER_VERT) as isize))
            .offset(-(SGRPROJ_BORDER_HORZ as isize)),
        width_ext,
        height_ext,
        dgd_stride,
        r,
        0 as core::ffi::c_int,
        B,
        buf_stride,
    );
    boxsum(
        dgd
            .offset(-((dgd_stride * SGRPROJ_BORDER_VERT) as isize))
            .offset(-(SGRPROJ_BORDER_HORZ as isize)),
        width_ext,
        height_ext,
        dgd_stride,
        r,
        1 as core::ffi::c_int,
        A,
        buf_stride,
    );
    A = A.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    B = B.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    i = -(1 as core::ffi::c_int);
    while i < height + 1 as core::ffi::c_int {
        j = -(1 as core::ffi::c_int);
        while j < width + 1 as core::ffi::c_int {
            let k: core::ffi::c_int = i * buf_stride + j;
            let n: core::ffi::c_int = (2 as core::ffi::c_int * r + 1 as core::ffi::c_int)
                * (2 as core::ffi::c_int * r + 1 as core::ffi::c_int);
            let mut a: uint32_t = (*A.offset(k as isize)
                + ((1 as int32_t)
                    << 2 as core::ffi::c_int * (bit_depth - 8 as core::ffi::c_int)
                    >> 1 as core::ffi::c_int)
                >> 2 as core::ffi::c_int * (bit_depth - 8 as core::ffi::c_int))
                as uint32_t;
            let mut b: uint32_t = (*B.offset(k as isize)
                + ((1 as int32_t) << bit_depth - 8 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> bit_depth - 8 as core::ffi::c_int)
                as uint32_t;
            let mut p: uint32_t = if a.wrapping_mul(n as uint32_t) < b.wrapping_mul(b) {
                0 as uint32_t
            } else {
                a.wrapping_mul(n as uint32_t).wrapping_sub(b.wrapping_mul(b))
            };
            let s: uint32_t = (*params).s[radius_idx as usize] as uint32_t;
            let z: uint32_t = p
                .wrapping_mul(s)
                .wrapping_add(
                    ((1 as core::ffi::c_int) << 20 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as uint32_t,
                ) >> 20 as core::ffi::c_int;
            *A.offset(k as isize) = av1_x_by_xplus1[(if z < 255 as uint32_t {
                z
            } else {
                255 as uint32_t
            }) as usize];
            *B.offset(k as isize) = (((((1 as int32_t) << 8 as core::ffi::c_int)
                - *A.offset(k as isize)) as uint32_t)
                .wrapping_mul(*B.offset(k as isize) as uint32_t)
                .wrapping_mul(
                    av1_one_by_x[(n - 1 as core::ffi::c_int) as usize] as uint32_t,
                )
                .wrapping_add(
                    ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) as uint32_t,
                ) >> 12 as core::ffi::c_int) as int32_t;
            j += 1;
        }
        i += step;
    }
}
unsafe extern "C-unwind" fn selfguided_restoration_fast_internal(
    mut dgd: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut dgd_stride: core::ffi::c_int,
    mut dst: *mut int32_t,
    mut dst_stride: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut sgr_params_idx: core::ffi::c_int,
    mut radius_idx: core::ffi::c_int,
) {
    let params: *const sgr_params_type = &*av1_sgr_params
        .as_ptr()
        .offset(sgr_params_idx as isize) as *const sgr_params_type;
    let r: core::ffi::c_int = (*params).r[radius_idx as usize];
    let width_ext: core::ffi::c_int = width
        + 2 as core::ffi::c_int * SGRPROJ_BORDER_HORZ;
    let mut buf_stride: core::ffi::c_int = (width_ext + 3 as core::ffi::c_int
        & !(3 as core::ffi::c_int)) + 16 as core::ffi::c_int;
    let mut A_: [int32_t; 8100] = [0; 8100];
    let mut B_: [int32_t; 8100] = [0; 8100];
    let mut A: *mut int32_t = A_.as_mut_ptr();
    let mut B: *mut int32_t = B_.as_mut_ptr();
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    calculate_intermediate_result(
        dgd,
        width,
        height,
        dgd_stride,
        bit_depth,
        sgr_params_idx,
        radius_idx,
        1 as core::ffi::c_int,
        A,
        B,
    );
    A = A.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    B = B.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    i = 0 as core::ffi::c_int;
    while i < height {
        if i & 1 as core::ffi::c_int == 0 {
            j = 0 as core::ffi::c_int;
            while j < width {
                let k: core::ffi::c_int = i * buf_stride + j;
                let l: core::ffi::c_int = i * dgd_stride + j;
                let m: core::ffi::c_int = i * dst_stride + j;
                let nb: core::ffi::c_int = 5 as core::ffi::c_int;
                let a: int32_t = (*A.offset((k - buf_stride) as isize)
                    + *A.offset((k + buf_stride) as isize)) * 6 as int32_t
                    + (*A.offset((k - 1 as core::ffi::c_int - buf_stride) as isize)
                        + *A.offset((k - 1 as core::ffi::c_int + buf_stride) as isize)
                        + *A.offset((k + 1 as core::ffi::c_int - buf_stride) as isize)
                        + *A.offset((k + 1 as core::ffi::c_int + buf_stride) as isize))
                        * 5 as int32_t;
                let b: int32_t = (*B.offset((k - buf_stride) as isize)
                    + *B.offset((k + buf_stride) as isize)) * 6 as int32_t
                    + (*B.offset((k - 1 as core::ffi::c_int - buf_stride) as isize)
                        + *B.offset((k - 1 as core::ffi::c_int + buf_stride) as isize)
                        + *B.offset((k + 1 as core::ffi::c_int - buf_stride) as isize)
                        + *B.offset((k + 1 as core::ffi::c_int + buf_stride) as isize))
                        * 5 as int32_t;
                let v: int32_t = a * *dgd.offset(l as isize) + b;
                *dst.offset(m as isize) = v
                    + ((1 as int32_t)
                        << 8 as core::ffi::c_int + nb - 4 as core::ffi::c_int
                        >> 1 as core::ffi::c_int)
                    >> 8 as core::ffi::c_int + nb - 4 as core::ffi::c_int;
                j += 1;
            }
        } else {
            j = 0 as core::ffi::c_int;
            while j < width {
                let k_0: core::ffi::c_int = i * buf_stride + j;
                let l_0: core::ffi::c_int = i * dgd_stride + j;
                let m_0: core::ffi::c_int = i * dst_stride + j;
                let nb_0: core::ffi::c_int = 4 as core::ffi::c_int;
                let a_0: int32_t = *A.offset(k_0 as isize) * 6 as int32_t
                    + (*A.offset((k_0 - 1 as core::ffi::c_int) as isize)
                        + *A.offset((k_0 + 1 as core::ffi::c_int) as isize))
                        * 5 as int32_t;
                let b_0: int32_t = *B.offset(k_0 as isize) * 6 as int32_t
                    + (*B.offset((k_0 - 1 as core::ffi::c_int) as isize)
                        + *B.offset((k_0 + 1 as core::ffi::c_int) as isize))
                        * 5 as int32_t;
                let v_0: int32_t = a_0 * *dgd.offset(l_0 as isize) + b_0;
                *dst.offset(m_0 as isize) = v_0
                    + ((1 as int32_t)
                        << 8 as core::ffi::c_int + nb_0 - 4 as core::ffi::c_int
                        >> 1 as core::ffi::c_int)
                    >> 8 as core::ffi::c_int + nb_0 - 4 as core::ffi::c_int;
                j += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn selfguided_restoration_internal(
    mut dgd: *mut int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut dgd_stride: core::ffi::c_int,
    mut dst: *mut int32_t,
    mut dst_stride: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut sgr_params_idx: core::ffi::c_int,
    mut radius_idx: core::ffi::c_int,
) {
    let width_ext: core::ffi::c_int = width
        + 2 as core::ffi::c_int * SGRPROJ_BORDER_HORZ;
    let mut buf_stride: core::ffi::c_int = (width_ext + 3 as core::ffi::c_int
        & !(3 as core::ffi::c_int)) + 16 as core::ffi::c_int;
    let mut A_: [int32_t; 8100] = [0; 8100];
    let mut B_: [int32_t; 8100] = [0; 8100];
    let mut A: *mut int32_t = A_.as_mut_ptr();
    let mut B: *mut int32_t = B_.as_mut_ptr();
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    calculate_intermediate_result(
        dgd,
        width,
        height,
        dgd_stride,
        bit_depth,
        sgr_params_idx,
        radius_idx,
        0 as core::ffi::c_int,
        A,
        B,
    );
    A = A.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    B = B.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            let k: core::ffi::c_int = i * buf_stride + j;
            let l: core::ffi::c_int = i * dgd_stride + j;
            let m: core::ffi::c_int = i * dst_stride + j;
            let nb: core::ffi::c_int = 5 as core::ffi::c_int;
            let a: int32_t = (*A.offset(k as isize)
                + *A.offset((k - 1 as core::ffi::c_int) as isize)
                + *A.offset((k + 1 as core::ffi::c_int) as isize)
                + *A.offset((k - buf_stride) as isize)
                + *A.offset((k + buf_stride) as isize)) * 4 as int32_t
                + (*A.offset((k - 1 as core::ffi::c_int - buf_stride) as isize)
                    + *A.offset((k - 1 as core::ffi::c_int + buf_stride) as isize)
                    + *A.offset((k + 1 as core::ffi::c_int - buf_stride) as isize)
                    + *A.offset((k + 1 as core::ffi::c_int + buf_stride) as isize))
                    * 3 as int32_t;
            let b: int32_t = (*B.offset(k as isize)
                + *B.offset((k - 1 as core::ffi::c_int) as isize)
                + *B.offset((k + 1 as core::ffi::c_int) as isize)
                + *B.offset((k - buf_stride) as isize)
                + *B.offset((k + buf_stride) as isize)) * 4 as int32_t
                + (*B.offset((k - 1 as core::ffi::c_int - buf_stride) as isize)
                    + *B.offset((k - 1 as core::ffi::c_int + buf_stride) as isize)
                    + *B.offset((k + 1 as core::ffi::c_int - buf_stride) as isize)
                    + *B.offset((k + 1 as core::ffi::c_int + buf_stride) as isize))
                    * 3 as int32_t;
            let v: int32_t = a * *dgd.offset(l as isize) + b;
            *dst.offset(m as isize) = v
                + ((1 as int32_t) << 8 as core::ffi::c_int + nb - 4 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 8 as core::ffi::c_int + nb - 4 as core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_selfguided_restoration_c(
    mut dgd8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut dgd_stride: core::ffi::c_int,
    mut flt0: *mut int32_t,
    mut flt1: *mut int32_t,
    mut flt_stride: core::ffi::c_int,
    mut sgr_params_idx: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut dgd32_: [int32_t; 8100] = [0; 8100];
    let dgd32_stride: core::ffi::c_int = width
        + 2 as core::ffi::c_int * SGRPROJ_BORDER_HORZ;
    let mut dgd32: *mut int32_t = dgd32_
        .as_mut_ptr()
        .offset((dgd32_stride * SGRPROJ_BORDER_VERT) as isize)
        .offset(SGRPROJ_BORDER_HORZ as isize);
    if highbd != 0 {
        let mut dgd16: *const uint16_t = ((dgd8 as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut i: core::ffi::c_int = -SGRPROJ_BORDER_VERT;
        while i < height + SGRPROJ_BORDER_VERT {
            let mut j: core::ffi::c_int = -SGRPROJ_BORDER_HORZ;
            while j < width + SGRPROJ_BORDER_HORZ {
                *dgd32.offset((i * dgd32_stride + j) as isize) = *dgd16
                    .offset((i * dgd_stride + j) as isize) as int32_t;
                j += 1;
            }
            i += 1;
        }
    } else {
        let mut i_0: core::ffi::c_int = -SGRPROJ_BORDER_VERT;
        while i_0 < height + SGRPROJ_BORDER_VERT {
            let mut j_0: core::ffi::c_int = -SGRPROJ_BORDER_HORZ;
            while j_0 < width + SGRPROJ_BORDER_HORZ {
                *dgd32.offset((i_0 * dgd32_stride + j_0) as isize) = *dgd8
                    .offset((i_0 * dgd_stride + j_0) as isize) as int32_t;
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
    let params: *const sgr_params_type = &*av1_sgr_params
        .as_ptr()
        .offset(sgr_params_idx as isize) as *const sgr_params_type;
    if (*params).r[0 as core::ffi::c_int as usize] > 0 as core::ffi::c_int {
        selfguided_restoration_fast_internal(
            dgd32,
            width,
            height,
            dgd32_stride,
            flt0,
            flt_stride,
            bit_depth,
            sgr_params_idx,
            0 as core::ffi::c_int,
        );
    }
    if (*params).r[1 as core::ffi::c_int as usize] > 0 as core::ffi::c_int {
        selfguided_restoration_internal(
            dgd32,
            width,
            height,
            dgd32_stride,
            flt1,
            flt_stride,
            bit_depth,
            sgr_params_idx,
            1 as core::ffi::c_int,
        );
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_apply_selfguided_restoration_c(
    mut dat8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut eps: core::ffi::c_int,
    mut xqd: *const core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut bit_depth: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut flt0: *mut int32_t = tmpbuf;
    let mut flt1: *mut int32_t = flt0.offset(RESTORATION_UNITPELS_MAX as isize);
    let ret: core::ffi::c_int = av1_selfguided_restoration_c(
        dat8,
        width,
        height,
        stride,
        flt0,
        flt1,
        width,
        eps,
        bit_depth,
        highbd,
    ) as core::ffi::c_int;
    if ret != 0 as core::ffi::c_int {
        return ret;
    }
    let params: *const sgr_params_type = &*av1_sgr_params.as_ptr().offset(eps as isize)
        as *const sgr_params_type;
    let mut xq: [core::ffi::c_int; 2] = [0; 2];
    av1_decode_xq(xqd, xq.as_mut_ptr(), params);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < width {
            let k: core::ffi::c_int = i * width + j;
            let mut dst8ij: *mut uint8_t = dst8
                .offset((i * dst_stride) as isize)
                .offset(j as isize);
            let mut dat8ij: *const uint8_t = dat8
                .offset((i * stride) as isize)
                .offset(j as isize);
            let pre_u: uint16_t = (if highbd != 0 {
                *(((dat8ij as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t)
                    as core::ffi::c_int
            } else {
                *dat8ij as core::ffi::c_int
            }) as uint16_t;
            let u: int32_t = (pre_u as int32_t) << SGRPROJ_RST_BITS;
            let mut v: int32_t = u << SGRPROJ_PRJ_BITS;
            if (*params).r[0 as core::ffi::c_int as usize] > 0 as core::ffi::c_int {
                v = (v as core::ffi::c_int
                    + (xq[0 as core::ffi::c_int as usize]
                        * (*flt0.offset(k as isize) - u)) as core::ffi::c_int)
                    as int32_t;
            }
            if (*params).r[1 as core::ffi::c_int as usize] > 0 as core::ffi::c_int {
                v = (v as core::ffi::c_int
                    + (xq[1 as core::ffi::c_int as usize]
                        * (*flt1.offset(k as isize) - u)) as core::ffi::c_int)
                    as int32_t;
            }
            let w: int16_t = (v
                + ((1 as int32_t) << 7 as core::ffi::c_int + 4 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 7 as core::ffi::c_int + 4 as core::ffi::c_int) as int16_t;
            let out: uint16_t = clip_pixel_highbd(w as core::ffi::c_int, bit_depth)
                as uint16_t;
            if highbd != 0 {
                *(((dst8ij as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t) = out;
            } else {
                *dst8ij = out as uint8_t;
            }
            j += 1;
        }
        i += 1;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn sgrproj_filter_stripe(
    mut rui: *const RestorationUnitInfo,
    mut stripe_width: core::ffi::c_int,
    mut stripe_height: core::ffi::c_int,
    mut procunit_width: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut bit_depth: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < stripe_width {
        let mut w: core::ffi::c_int = if procunit_width < stripe_width - j {
            procunit_width
        } else {
            stripe_width - j
        };
        if av1_apply_selfguided_restoration_c(
            src.offset(j as isize),
            w,
            stripe_height,
            src_stride,
            (*rui).sgrproj_info.ep,
            ((*rui).sgrproj_info.xqd).as_ptr(),
            dst.offset(j as isize),
            dst_stride,
            tmpbuf,
            bit_depth,
            0 as core::ffi::c_int,
        ) != 0 as core::ffi::c_int
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_MEM_ERROR,
                b"Error allocating buffer in av1_apply_selfguided_restoration\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        j += procunit_width;
    }
}
unsafe extern "C-unwind" fn wiener_filter_stripe_highbd(
    mut rui: *const RestorationUnitInfo,
    mut stripe_width: core::ffi::c_int,
    mut stripe_height: core::ffi::c_int,
    mut procunit_width: core::ffi::c_int,
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut bit_depth: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
) {
    let conv_params: WienerConvolveParams = get_conv_params_wiener(bit_depth)
        as WienerConvolveParams;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < stripe_width {
        let mut w: core::ffi::c_int = if procunit_width
            < stripe_width - j + 15 as core::ffi::c_int & !(15 as core::ffi::c_int)
        {
            procunit_width
        } else {
            stripe_width - j + 15 as core::ffi::c_int & !(15 as core::ffi::c_int)
        };
        let mut src8_p: *const uint8_t = src8.offset(j as isize);
        let mut dst8_p: *mut uint8_t = dst8.offset(j as isize);
        av1_highbd_wiener_convolve_add_src_c(
            src8_p,
            src_stride as ptrdiff_t,
            dst8_p,
            dst_stride as ptrdiff_t,
            ((*rui).wiener_info.hfilter).as_ptr(),
            16 as core::ffi::c_int,
            ((*rui).wiener_info.vfilter).as_ptr(),
            16 as core::ffi::c_int,
            w,
            stripe_height,
            &conv_params,
            bit_depth,
        );
        j += procunit_width;
    }
}
unsafe extern "C-unwind" fn sgrproj_filter_stripe_highbd(
    mut rui: *const RestorationUnitInfo,
    mut stripe_width: core::ffi::c_int,
    mut stripe_height: core::ffi::c_int,
    mut procunit_width: core::ffi::c_int,
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut bit_depth: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < stripe_width {
        let mut w: core::ffi::c_int = if procunit_width < stripe_width - j {
            procunit_width
        } else {
            stripe_width - j
        };
        if av1_apply_selfguided_restoration_c(
            src8.offset(j as isize),
            w,
            stripe_height,
            src_stride,
            (*rui).sgrproj_info.ep,
            ((*rui).sgrproj_info.xqd).as_ptr(),
            dst8.offset(j as isize),
            dst_stride,
            tmpbuf,
            bit_depth,
            1 as core::ffi::c_int,
        ) != 0 as core::ffi::c_int
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_MEM_ERROR,
                b"Error allocating buffer in av1_apply_selfguided_restoration\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        j += procunit_width;
    }
}
static mut stripe_filters: [stripe_filter_fun; 4] = unsafe {
    [
        Some(
            wiener_filter_stripe
                as unsafe extern "C-unwind" fn(
                    *const RestorationUnitInfo,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut aom_internal_error_info,
                ) -> (),
        ),
        Some(
            sgrproj_filter_stripe
                as unsafe extern "C-unwind" fn(
                    *const RestorationUnitInfo,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut aom_internal_error_info,
                ) -> (),
        ),
        Some(
            wiener_filter_stripe_highbd
                as unsafe extern "C-unwind" fn(
                    *const RestorationUnitInfo,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut aom_internal_error_info,
                ) -> (),
        ),
        Some(
            sgrproj_filter_stripe_highbd
                as unsafe extern "C-unwind" fn(
                    *const RestorationUnitInfo,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut int32_t,
                    core::ffi::c_int,
                    *mut aom_internal_error_info,
                ) -> (),
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_filter_unit(
    mut limits: *const RestorationTileLimits,
    mut rui: *const RestorationUnitInfo,
    mut rsb: *const RestorationStripeBoundaries,
    mut rlbs: *mut RestorationLineBuffers,
    mut plane_w: core::ffi::c_int,
    mut plane_h: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut data8: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut tmpbuf: *mut int32_t,
    mut optimized_lr: core::ffi::c_int,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut unit_rtype: RestorationType = (*rui).restoration_type;
    let mut unit_h: core::ffi::c_int = (*limits).v_end - (*limits).v_start;
    let mut unit_w: core::ffi::c_int = (*limits).h_end - (*limits).h_start;
    let mut data8_tl: *mut uint8_t = data8
        .offset(((*limits).v_start as ptrdiff_t * stride as ptrdiff_t) as isize)
        .offset((*limits).h_start as isize);
    let mut dst8_tl: *mut uint8_t = dst8
        .offset(((*limits).v_start as ptrdiff_t * dst_stride as ptrdiff_t) as isize)
        .offset((*limits).h_start as isize);
    if unit_rtype as core::ffi::c_uint
        == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        copy_rest_unit(unit_w, unit_h, data8_tl, stride, dst8_tl, dst_stride, highbd);
        return;
    }
    let filter_idx: core::ffi::c_int = 2 as core::ffi::c_int * highbd
        + (unit_rtype as core::ffi::c_uint
            == RESTORE_SGRPROJ as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
    let stripe_filter: stripe_filter_fun = stripe_filters[filter_idx as usize];
    let procunit_width: core::ffi::c_int = RESTORATION_PROC_UNIT_SIZE >> ss_x;
    let mut remaining_stripes: RestorationTileLimits = *limits;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < unit_h {
        let mut copy_above: core::ffi::c_int = 0;
        let mut copy_below: core::ffi::c_int = 0;
        remaining_stripes.v_start = (*limits).v_start + i;
        get_stripe_boundary_info(
            &raw mut remaining_stripes,
            plane_w,
            plane_h,
            ss_y,
            &raw mut copy_above,
            &raw mut copy_below,
        );
        let full_stripe_height: core::ffi::c_int = RESTORATION_PROC_UNIT_SIZE >> ss_y;
        let runit_offset: core::ffi::c_int = RESTORATION_UNIT_OFFSET >> ss_y;
        let frame_stripe: core::ffi::c_int = (remaining_stripes.v_start + runit_offset)
            / full_stripe_height;
        let rsb_row: core::ffi::c_int = RESTORATION_CTX_VERT * frame_stripe;
        let nominal_stripe_height: core::ffi::c_int = full_stripe_height
            - (if frame_stripe == 0 as core::ffi::c_int {
                runit_offset
            } else {
                0 as core::ffi::c_int
            });
        let h: core::ffi::c_int = if nominal_stripe_height
            < remaining_stripes.v_end - remaining_stripes.v_start
        {
            nominal_stripe_height
        } else {
            remaining_stripes.v_end - remaining_stripes.v_start
        };
        setup_processing_stripe_boundary(
            &raw mut remaining_stripes,
            rsb,
            rsb_row,
            highbd,
            h,
            data8,
            stride,
            rlbs,
            copy_above,
            copy_below,
            optimized_lr,
        );
        stripe_filter
            .expect(
                "non-null function pointer",
            )(
            rui,
            unit_w,
            h,
            procunit_width,
            data8_tl.offset((i * stride) as isize),
            stride,
            dst8_tl.offset((i * dst_stride) as isize),
            dst_stride,
            tmpbuf,
            bit_depth,
            error_info,
        );
        restore_processing_stripe_boundary(
            &raw mut remaining_stripes,
            rlbs,
            highbd,
            h,
            data8,
            stride,
            copy_above,
            copy_below,
            optimized_lr,
        );
        i += h;
    }
}
unsafe extern "C-unwind" fn filter_frame_on_unit(
    mut limits: *const RestorationTileLimits,
    mut rest_unit_idx: core::ffi::c_int,
    mut priv_0: *mut core::ffi::c_void,
    mut tmpbuf: *mut int32_t,
    mut rlbs: *mut RestorationLineBuffers,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut ctxt: *mut FilterFrameCtxt = priv_0 as *mut FilterFrameCtxt;
    let mut rsi: *const RestorationInfo = (*ctxt).rsi;
    av1_loop_restoration_filter_unit(
        limits,
        &raw mut *((*rsi).unit_info).offset(rest_unit_idx as isize),
        &(*rsi).boundaries,
        rlbs,
        (*ctxt).plane_w,
        (*ctxt).plane_h,
        (*ctxt).ss_x,
        (*ctxt).ss_y,
        (*ctxt).highbd,
        (*ctxt).bit_depth,
        (*ctxt).data8,
        (*ctxt).data_stride,
        (*ctxt).dst8,
        (*ctxt).dst_stride,
        tmpbuf,
        (*rsi).optimized_lr,
        error_info,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_filter_frame_init(
    mut lr_ctxt: *mut AV1LrStruct,
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut optimized_lr: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) {
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let bit_depth: core::ffi::c_int = (*seq_params).bit_depth as core::ffi::c_int;
    let highbd: core::ffi::c_int = (*seq_params).use_highbitdepth as core::ffi::c_int;
    (*lr_ctxt).dst = &raw mut (*cm).rst_frame;
    let frame_width: core::ffi::c_int = (*frame)
        .c2rust_unnamed_1
        .crop_widths[0 as core::ffi::c_int as usize];
    let frame_height: core::ffi::c_int = (*frame)
        .c2rust_unnamed_2
        .crop_heights[0 as core::ffi::c_int as usize];
    if aom_realloc_frame_buffer(
        (*lr_ctxt).dst,
        frame_width,
        frame_height,
        (*seq_params).subsampling_x,
        (*seq_params).subsampling_y,
        highbd,
        AOM_RESTORATION_FRAME_BORDER,
        (*cm).features.byte_alignment,
        0 as *mut aom_codec_frame_buffer_t,
        None,
        NULL,
        false_0 != 0,
        0 as core::ffi::c_int,
    ) != AOM_CODEC_OK as core::ffi::c_int
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate restoration dst buffer\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*lr_ctxt).on_rest_unit = Some(
        filter_frame_on_unit
            as unsafe extern "C-unwind" fn(
                *const RestorationTileLimits,
                core::ffi::c_int,
                *mut core::ffi::c_void,
                *mut int32_t,
                *mut RestorationLineBuffers,
                *mut aom_internal_error_info,
            ) -> (),
    ) as rest_unit_visitor_t;
    (*lr_ctxt).frame = frame;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let mut rsi: *mut RestorationInfo = &raw mut *((*cm).rst_info)
            .as_mut_ptr()
            .offset(plane as isize) as *mut RestorationInfo;
        let mut rtype: RestorationType = (*rsi).frame_restoration_type;
        (*rsi).optimized_lr = optimized_lr;
        (*lr_ctxt).ctxt[plane as usize].rsi = rsi;
        if !(rtype as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let mut plane_w: core::ffi::c_int = 0;
            let mut plane_h: core::ffi::c_int = 0;
            av1_get_upsampled_plane_size(cm, is_uv, &raw mut plane_w, &raw mut plane_h);
            av1_extend_frame(
                (*frame).c2rust_unnamed_4.buffers[plane as usize],
                plane_w,
                plane_h,
                (*frame).c2rust_unnamed_3.strides[is_uv as usize],
                RESTORATION_BORDER,
                RESTORATION_BORDER,
                highbd,
            );
            let mut lr_plane_ctxt: *mut FilterFrameCtxt = &raw mut *((*lr_ctxt).ctxt)
                .as_mut_ptr()
                .offset(plane as isize) as *mut FilterFrameCtxt;
            (*lr_plane_ctxt).ss_x = (is_uv != 0 && (*seq_params).subsampling_x != 0)
                as core::ffi::c_int;
            (*lr_plane_ctxt).ss_y = (is_uv != 0 && (*seq_params).subsampling_y != 0)
                as core::ffi::c_int;
            (*lr_plane_ctxt).plane_w = plane_w;
            (*lr_plane_ctxt).plane_h = plane_h;
            (*lr_plane_ctxt).highbd = highbd;
            (*lr_plane_ctxt).bit_depth = bit_depth;
            (*lr_plane_ctxt).data8 = (*frame).c2rust_unnamed_4.buffers[plane as usize];
            (*lr_plane_ctxt).dst8 = (*(*lr_ctxt).dst)
                .c2rust_unnamed_4
                .buffers[plane as usize];
            (*lr_plane_ctxt).data_stride = (*frame)
                .c2rust_unnamed_3
                .strides[is_uv as usize];
            (*lr_plane_ctxt).dst_stride = (*(*lr_ctxt).dst)
                .c2rust_unnamed_3
                .strides[is_uv as usize];
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn loop_restoration_copy_planes(
    mut loop_rest_ctxt: *mut AV1LrStruct,
    mut cm: *mut AV1_COMMON,
    mut num_planes: core::ffi::c_int,
) {
    static mut copy_funs: [copy_fun; 3] = unsafe {
        [
            Some(
                aom_yv12_partial_coloc_copy_y_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                aom_yv12_partial_coloc_copy_u_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                aom_yv12_partial_coloc_copy_v_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
        ]
    };
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if !((*cm).rst_info[plane as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            let mut lr_plane_ctxt: *mut FilterFrameCtxt = &raw mut *((*loop_rest_ctxt).ctxt)
                .as_mut_ptr()
                .offset(plane as isize) as *mut FilterFrameCtxt;
            (copy_funs[plane as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*loop_rest_ctxt).dst,
                (*loop_rest_ctxt).frame,
                0 as core::ffi::c_int,
                (*lr_plane_ctxt).plane_w,
                0 as core::ffi::c_int,
                (*lr_plane_ctxt).plane_h,
            );
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn foreach_rest_unit_in_plane(
    mut cm: *const AV1Common,
    mut plane: core::ffi::c_int,
    mut on_rest_unit: rest_unit_visitor_t,
    mut priv_0: *mut core::ffi::c_void,
    mut tmpbuf: *mut int32_t,
    mut rlbs: *mut RestorationLineBuffers,
) {
    let mut rsi: *const RestorationInfo = &*((*cm).rst_info)
        .as_ptr()
        .offset(plane as isize) as *const RestorationInfo;
    let hnum_rest_units: core::ffi::c_int = (*rsi).horz_units;
    let vnum_rest_units: core::ffi::c_int = (*rsi).vert_units;
    let unit_size: core::ffi::c_int = (*rsi).restoration_unit_size;
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let ss_y: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_y != 0)
        as core::ffi::c_int;
    let ext_size: core::ffi::c_int = unit_size * 3 as core::ffi::c_int
        / 2 as core::ffi::c_int;
    let mut plane_w: core::ffi::c_int = 0;
    let mut plane_h: core::ffi::c_int = 0;
    av1_get_upsampled_plane_size(
        cm as *const AV1_COMMON,
        is_uv,
        &raw mut plane_w,
        &raw mut plane_h,
    );
    let mut y0: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while y0 < plane_h {
        let mut remaining_h: core::ffi::c_int = plane_h - y0;
        let mut h: core::ffi::c_int = if remaining_h < ext_size {
            remaining_h
        } else {
            unit_size
        };
        let mut limits: RestorationTileLimits = RestorationTileLimits {
            h_start: 0,
            h_end: 0,
            v_start: 0,
            v_end: 0,
        };
        limits.v_start = y0;
        limits.v_end = y0 + h;
        let voffset: core::ffi::c_int = RESTORATION_UNIT_OFFSET >> ss_y;
        limits.v_start = if 0 as core::ffi::c_int > limits.v_start - voffset {
            0 as core::ffi::c_int
        } else {
            limits.v_start - voffset
        };
        if limits.v_end < plane_h {
            limits.v_end -= voffset;
        }
        av1_foreach_rest_unit_in_row(
            &raw mut limits,
            plane_w,
            on_rest_unit,
            i,
            unit_size,
            hnum_rest_units,
            vnum_rest_units,
            plane,
            priv_0,
            tmpbuf,
            rlbs,
            Some(
                av1_lr_sync_read_dummy
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                av1_lr_sync_write_dummy
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            0 as *mut AV1LrSyncData,
            (*cm).error,
        );
        y0 += h;
        i += 1;
    }
}
unsafe extern "C-unwind" fn foreach_rest_unit_in_planes(
    mut lr_ctxt: *mut AV1LrStruct,
    mut cm: *mut AV1_COMMON,
    mut num_planes: core::ffi::c_int,
) {
    let mut ctxt: *mut FilterFrameCtxt = ((*lr_ctxt).ctxt).as_mut_ptr();
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if !((*cm).rst_info[plane as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            foreach_rest_unit_in_plane(
                cm,
                plane,
                (*lr_ctxt).on_rest_unit,
                &raw mut *ctxt.offset(plane as isize) as *mut FilterFrameCtxt
                    as *mut core::ffi::c_void,
                (*cm).rst_tmpbuf,
                (*cm).rlbs,
            );
        }
        plane += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_filter_frame(
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut optimized_lr: core::ffi::c_int,
    mut lr_ctxt: *mut core::ffi::c_void,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut loop_rest_ctxt: *mut AV1LrStruct = lr_ctxt as *mut AV1LrStruct;
    av1_loop_restoration_filter_frame_init(
        loop_rest_ctxt,
        frame,
        cm,
        optimized_lr,
        num_planes,
    );
    foreach_rest_unit_in_planes(loop_rest_ctxt, cm, num_planes);
    loop_restoration_copy_planes(loop_rest_ctxt, cm, num_planes);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_foreach_rest_unit_in_row(
    mut limits: *mut RestorationTileLimits,
    mut plane_w: core::ffi::c_int,
    mut on_rest_unit: rest_unit_visitor_t,
    mut row_number: core::ffi::c_int,
    mut unit_size: core::ffi::c_int,
    mut hnum_rest_units: core::ffi::c_int,
    mut vnum_rest_units: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut priv_0: *mut core::ffi::c_void,
    mut tmpbuf: *mut int32_t,
    mut rlbs: *mut RestorationLineBuffers,
    mut on_sync_read: sync_read_fn_t,
    mut on_sync_write: sync_write_fn_t,
    lr_sync: *mut AV1LrSyncData,
    mut error_info: *mut aom_internal_error_info,
) {
    let ext_size: core::ffi::c_int = unit_size * 3 as core::ffi::c_int
        / 2 as core::ffi::c_int;
    let mut x0: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while x0 < plane_w {
        let mut remaining_w: core::ffi::c_int = plane_w - x0;
        let mut w: core::ffi::c_int = if remaining_w < ext_size {
            remaining_w
        } else {
            unit_size
        };
        (*limits).h_start = x0;
        (*limits).h_end = x0 + w;
        let unit_idx: core::ffi::c_int = row_number * hnum_rest_units + j;
        on_sync_read
            .expect(
                "non-null function pointer",
            )(lr_sync as *mut core::ffi::c_void, row_number, j, plane);
        if (row_number + 1 as core::ffi::c_int) < vnum_rest_units {
            on_sync_read
                .expect(
                    "non-null function pointer",
                )(
                lr_sync as *mut core::ffi::c_void,
                row_number + 2 as core::ffi::c_int,
                j,
                plane,
            );
        }
        if !lr_sync.is_null() && (*lr_sync).num_workers > 1 as core::ffi::c_int {
            pthread_mutex_lock((*lr_sync).job_mutex as *mut pthread_mutex_t);
            let lr_mt_exit: bool = (*lr_sync).lr_mt_exit;
            pthread_mutex_unlock((*lr_sync).job_mutex as *mut pthread_mutex_t);
            if lr_mt_exit {
                return;
            }
        }
        on_rest_unit
            .expect(
                "non-null function pointer",
            )(limits, unit_idx, priv_0, tmpbuf, rlbs, error_info);
        on_sync_write
            .expect(
                "non-null function pointer",
            )(lr_sync as *mut core::ffi::c_void, row_number, j, hnum_rest_units, plane);
        x0 += w;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lr_sync_read_dummy(
    lr_sync: *mut core::ffi::c_void,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lr_sync_write_dummy(
    lr_sync: *mut core::ffi::c_void,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    sb_cols: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_corners_in_sb(
    mut cm: *const AV1Common,
    mut plane: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut rcol0: *mut core::ffi::c_int,
    mut rcol1: *mut core::ffi::c_int,
    mut rrow0: *mut core::ffi::c_int,
    mut rrow1: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    if bsize as core::ffi::c_int != (*(*cm).seq_params).sb_size as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let mi_row0: core::ffi::c_int = mi_row;
    let mi_col0: core::ffi::c_int = mi_col;
    let mi_row1: core::ffi::c_int = mi_row0
        + mi_size_high[bsize as usize] as core::ffi::c_int;
    let mi_col1: core::ffi::c_int = mi_col0
        + mi_size_wide[bsize as usize] as core::ffi::c_int;
    let mut rsi: *const RestorationInfo = &*((*cm).rst_info)
        .as_ptr()
        .offset(plane as isize) as *const RestorationInfo;
    let size: core::ffi::c_int = (*rsi).restoration_unit_size;
    let horz_units: core::ffi::c_int = (*rsi).horz_units;
    let vert_units: core::ffi::c_int = (*rsi).vert_units;
    let ss_x: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_x != 0)
        as core::ffi::c_int;
    let ss_y: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_y != 0)
        as core::ffi::c_int;
    let mi_size_x: core::ffi::c_int = MI_SIZE >> ss_x;
    let mi_size_y: core::ffi::c_int = MI_SIZE >> ss_y;
    let mi_to_num_x: core::ffi::c_int = if av1_superres_scaled(cm as *const AV1_COMMON)
        != 0
    {
        mi_size_x * (*cm).superres_scale_denominator as core::ffi::c_int
    } else {
        mi_size_x
    };
    let mi_to_num_y: core::ffi::c_int = mi_size_y;
    let denom_x: core::ffi::c_int = if av1_superres_scaled(cm as *const AV1_COMMON) != 0
    {
        size * SCALE_NUMERATOR
    } else {
        size
    };
    let denom_y: core::ffi::c_int = size;
    let rnd_x: core::ffi::c_int = denom_x - 1 as core::ffi::c_int;
    let rnd_y: core::ffi::c_int = denom_y - 1 as core::ffi::c_int;
    *rcol0 = (mi_col0 * mi_to_num_x + rnd_x) / denom_x;
    *rrow0 = (mi_row0 * mi_to_num_y + rnd_y) / denom_y;
    *rcol1 = if (mi_col1 * mi_to_num_x + rnd_x) / denom_x < horz_units {
        (mi_col1 * mi_to_num_x + rnd_x) / denom_x
    } else {
        horz_units
    };
    *rrow1 = if (mi_row1 * mi_to_num_y + rnd_y) / denom_y < vert_units {
        (mi_row1 * mi_to_num_y + rnd_y) / denom_y
    } else {
        vert_units
    };
    return (*rcol0 < *rcol1 && *rrow0 < *rrow1) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn extend_lines(
    mut buf: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut extend: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        if use_highbitdepth != 0 {
            let mut buf16: *mut uint16_t = buf as *mut uint16_t;
            aom_memset16(
                buf16.offset(-(extend as isize)) as *mut core::ffi::c_void,
                *buf16.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                extend as size_t,
            );
            aom_memset16(
                buf16.offset(width as isize) as *mut core::ffi::c_void,
                *buf16.offset((width - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                extend as size_t,
            );
        } else {
            memset(
                buf.offset(-(extend as isize)) as *mut core::ffi::c_void,
                *buf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                extend as size_t,
            );
            memset(
                buf.offset(width as isize) as *mut core::ffi::c_void,
                *buf.offset((width - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                extend as size_t,
            );
        }
        buf = buf.offset(stride as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn save_deblock_boundary_lines(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut cm: *const AV1_COMMON,
    mut plane: core::ffi::c_int,
    mut row: core::ffi::c_int,
    mut stripe: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
    mut is_above: core::ffi::c_int,
    mut boundaries: *mut RestorationStripeBoundaries,
) {
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut src_buf: *const uint8_t = if use_highbd != 0 {
        (((*frame).c2rust_unnamed_4.buffers[plane as usize] as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t as *mut uint8_t
    } else {
        (*frame).c2rust_unnamed_4.buffers[plane as usize]
    };
    let src_stride: core::ffi::c_int = (*frame).c2rust_unnamed_3.strides[is_uv as usize]
        << use_highbd;
    let mut src_rows: *const uint8_t = src_buf
        .offset((row as ptrdiff_t * src_stride as ptrdiff_t) as isize);
    let mut bdry_buf: *mut uint8_t = if is_above != 0 {
        (*boundaries).stripe_boundary_above
    } else {
        (*boundaries).stripe_boundary_below
    };
    let mut bdry_start: *mut uint8_t = bdry_buf
        .offset((RESTORATION_EXTRA_HORZ << use_highbd) as isize);
    let bdry_stride: core::ffi::c_int = (*boundaries).stripe_boundary_stride
        << use_highbd;
    let mut bdry_rows: *mut uint8_t = bdry_start
        .offset((RESTORATION_CTX_VERT * stripe * bdry_stride) as isize);
    let lines_to_save: core::ffi::c_int = if (2 as core::ffi::c_int)
        < (*frame).c2rust_unnamed_2.crop_heights[is_uv as usize] - row
    {
        2 as core::ffi::c_int
    } else {
        (*frame).c2rust_unnamed_2.crop_heights[is_uv as usize] - row
    };
    let mut upscaled_width: core::ffi::c_int = 0;
    let mut line_bytes: core::ffi::c_int = 0;
    if av1_superres_scaled(cm) != 0 {
        let ss_x: core::ffi::c_int = (is_uv != 0
            && (*(*cm).seq_params).subsampling_x != 0) as core::ffi::c_int;
        upscaled_width = (*cm).superres_upscaled_width + ss_x >> ss_x;
        line_bytes = upscaled_width << use_highbd;
        if use_highbd != 0 {
            av1_upscale_normative_rows(
                cm,
                (src_rows as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                (*frame).c2rust_unnamed_3.strides[is_uv as usize],
                (bdry_rows as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                (*boundaries).stripe_boundary_stride,
                plane,
                lines_to_save,
            );
        } else {
            av1_upscale_normative_rows(
                cm,
                src_rows,
                (*frame).c2rust_unnamed_3.strides[is_uv as usize],
                bdry_rows,
                (*boundaries).stripe_boundary_stride,
                plane,
                lines_to_save,
            );
        }
    } else {
        upscaled_width = (*frame).c2rust_unnamed_1.crop_widths[is_uv as usize];
        line_bytes = upscaled_width << use_highbd;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < lines_to_save {
            memcpy(
                bdry_rows.offset((i * bdry_stride) as isize) as *mut core::ffi::c_void,
                src_rows.offset((i * src_stride) as isize) as *const core::ffi::c_void,
                line_bytes as size_t,
            );
            i += 1;
        }
    }
    if lines_to_save == 1 as core::ffi::c_int {
        memcpy(
            bdry_rows.offset(bdry_stride as isize) as *mut core::ffi::c_void,
            bdry_rows as *const core::ffi::c_void,
            line_bytes as size_t,
        );
    }
    extend_lines(
        bdry_rows,
        upscaled_width,
        RESTORATION_CTX_VERT,
        bdry_stride,
        RESTORATION_EXTRA_HORZ,
        use_highbd,
    );
}
unsafe extern "C-unwind" fn save_cdef_boundary_lines(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut cm: *const AV1_COMMON,
    mut plane: core::ffi::c_int,
    mut row: core::ffi::c_int,
    mut stripe: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
    mut is_above: core::ffi::c_int,
    mut boundaries: *mut RestorationStripeBoundaries,
) {
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut src_buf: *const uint8_t = if use_highbd != 0 {
        (((*frame).c2rust_unnamed_4.buffers[plane as usize] as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t as *mut uint8_t
    } else {
        (*frame).c2rust_unnamed_4.buffers[plane as usize]
    };
    let src_stride: core::ffi::c_int = (*frame).c2rust_unnamed_3.strides[is_uv as usize]
        << use_highbd;
    let mut src_rows: *const uint8_t = src_buf
        .offset((row as ptrdiff_t * src_stride as ptrdiff_t) as isize);
    let mut bdry_buf: *mut uint8_t = if is_above != 0 {
        (*boundaries).stripe_boundary_above
    } else {
        (*boundaries).stripe_boundary_below
    };
    let mut bdry_start: *mut uint8_t = bdry_buf
        .offset((RESTORATION_EXTRA_HORZ << use_highbd) as isize);
    let bdry_stride: core::ffi::c_int = (*boundaries).stripe_boundary_stride
        << use_highbd;
    let mut bdry_rows: *mut uint8_t = bdry_start
        .offset((RESTORATION_CTX_VERT * stripe * bdry_stride) as isize);
    let src_width: core::ffi::c_int = (*frame)
        .c2rust_unnamed_1
        .crop_widths[is_uv as usize];
    let ss_x: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_x != 0)
        as core::ffi::c_int;
    let upscaled_width: core::ffi::c_int = if av1_superres_scaled(cm) != 0 {
        (*cm).superres_upscaled_width + ss_x >> ss_x
    } else {
        src_width
    };
    let line_bytes: core::ffi::c_int = upscaled_width << use_highbd;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < RESTORATION_CTX_VERT {
        memcpy(
            bdry_rows.offset((i * bdry_stride) as isize) as *mut core::ffi::c_void,
            src_rows as *const core::ffi::c_void,
            line_bytes as size_t,
        );
        i += 1;
    }
    extend_lines(
        bdry_rows,
        upscaled_width,
        RESTORATION_CTX_VERT,
        bdry_stride,
        RESTORATION_EXTRA_HORZ,
        use_highbd,
    );
}
unsafe extern "C-unwind" fn save_boundary_lines(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut use_highbd: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut cm: *mut AV1_COMMON,
    mut after_cdef: core::ffi::c_int,
) {
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let ss_y: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_y != 0)
        as core::ffi::c_int;
    let stripe_height: core::ffi::c_int = RESTORATION_PROC_UNIT_SIZE >> ss_y;
    let stripe_off: core::ffi::c_int = RESTORATION_UNIT_OFFSET >> ss_y;
    let mut plane_w: core::ffi::c_int = 0;
    let mut plane_h: core::ffi::c_int = 0;
    av1_get_upsampled_plane_size(cm, is_uv, &raw mut plane_w, &raw mut plane_h);
    let mut boundaries: *mut RestorationStripeBoundaries = &raw mut (*((*cm).rst_info)
        .as_mut_ptr()
        .offset(plane as isize))
        .boundaries;
    let plane_height: core::ffi::c_int = (*cm).height
        + ((1 as core::ffi::c_int) << ss_y >> 1 as core::ffi::c_int) >> ss_y;
    let mut stripe_idx: core::ffi::c_int = 0;
    stripe_idx = 0 as core::ffi::c_int;
    loop {
        let rel_y0: core::ffi::c_int = if 0 as core::ffi::c_int
            > stripe_idx * stripe_height - stripe_off
        {
            0 as core::ffi::c_int
        } else {
            stripe_idx * stripe_height - stripe_off
        };
        let y0: core::ffi::c_int = rel_y0;
        if y0 >= plane_h {
            break;
        }
        let rel_y1: core::ffi::c_int = (stripe_idx + 1 as core::ffi::c_int)
            * stripe_height - stripe_off;
        let y1: core::ffi::c_int = if rel_y1 < plane_h { rel_y1 } else { plane_h };
        let use_deblock_above: core::ffi::c_int = (stripe_idx > 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let use_deblock_below: core::ffi::c_int = (y1 < plane_height)
            as core::ffi::c_int;
        if after_cdef == 0 {
            if use_deblock_above != 0 {
                save_deblock_boundary_lines(
                    frame,
                    cm,
                    plane,
                    y0 - RESTORATION_CTX_VERT,
                    stripe_idx,
                    use_highbd,
                    1 as core::ffi::c_int,
                    boundaries,
                );
            }
            if use_deblock_below != 0 {
                save_deblock_boundary_lines(
                    frame,
                    cm,
                    plane,
                    y1,
                    stripe_idx,
                    use_highbd,
                    0 as core::ffi::c_int,
                    boundaries,
                );
            }
        } else {
            if use_deblock_above == 0 {
                save_cdef_boundary_lines(
                    frame,
                    cm,
                    plane,
                    y0,
                    stripe_idx,
                    use_highbd,
                    1 as core::ffi::c_int,
                    boundaries,
                );
            }
            if use_deblock_below == 0 {
                save_cdef_boundary_lines(
                    frame,
                    cm,
                    plane,
                    y1 - 1 as core::ffi::c_int,
                    stripe_idx,
                    use_highbd,
                    0 as core::ffi::c_int,
                    boundaries,
                );
            }
        }
        stripe_idx += 1;
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_save_boundary_lines(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut after_cdef: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let use_highbd: core::ffi::c_int = (*(*cm).seq_params).use_highbitdepth
        as core::ffi::c_int;
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < num_planes {
        save_boundary_lines(frame, use_highbd, p, cm, after_cdef);
        p += 1;
    }
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
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
